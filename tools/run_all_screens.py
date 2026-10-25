#!/usr/bin/env python3
"""Batch UX audit of every phone screen in a pen.dev design.

Pipeline:
  1. export  each screen .pen -> PNG        (`pen --export`, needs `pen login`)
  2. describe each PNG -> vision+OCR JSON    (`market-swarm describe --import`,
                                             needs OPENROUTER_API_KEY)
  3. aggregate per-screen clarity/issues/CTAs/price mentions -> report.json + .csv
  4. optional: run the swarm on the aggregate for a product-level
     download/pay recommendation (`--swarm`)

The free VL model is rate-limited, so the describe phase is retried with
backoff and per-screen failures are recorded instead of aborting the batch.

Usage:
  python3 tools/run_all_screens.py --screens-dir output/screens --dry-run
  python3 tools/run_all_screens.py --screens-dir output/screens --swarm
"""

from __future__ import annotations

import argparse
import csv
import json
import os
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime, timezone

DEFAULT_PROMPT = (
    "Describe esta pantalla de app: qué muestra, todo el texto visible, y "
    "cualquier problema de UX o de conversión (claridad, jerarquía, CTAs, precio)."
)


def run_cmd(argv: list[str], check: bool = True) -> subprocess.CompletedProcess:
    result = subprocess.run(argv, capture_output=True, text=True)
    if check and result.returncode != 0:
        raise SystemExit(
            f"command failed ({result.returncode}): {' '.join(argv)}\n"
            f"stderr: {result.stderr.strip()}"
        )
    return result


def load_manifest(screens_dir: str) -> list[dict]:
    path = os.path.join(screens_dir, "manifest.json")
    if os.path.exists(path):
        with open(path, encoding="utf-8") as fh:
            manifest = json.load(fh)
        for item in manifest:
            item["file"] = os.path.join(screens_dir, item["file"])
        return manifest
    return [
        {"id": os.path.splitext(f)[0].split("-", 1)[0], "name": f,
         "file": os.path.join(screens_dir, f)}
        for f in sorted(os.listdir(screens_dir)) if f.endswith(".pen")
    ]


def export_one(args: tuple) -> tuple[str, str | None, str]:
    pen, screen, out_png, scale = args
    for attempt in range(2):
        r = subprocess.run(
            [pen, "--in", screen["file"], "--export", out_png,
             "--export-type", "png", "--export-scale", str(scale)],
            capture_output=True, text=True)
        if r.returncode == 0:
            return screen["id"], out_png, ""
        time.sleep(2 * (attempt + 1))
    return screen["id"], None, (r.stderr or "export failed").strip()


def describe_one(args: tuple) -> tuple[str, dict]:
    binary, db, mode, prompt, out_png = args
    argv = [binary, "--db", db, "describe", out_png, "--import", "--mode", mode]
    if prompt:
        argv += ["--prompt", prompt]
    last = ""
    for attempt in range(3):
        r = subprocess.run(argv, capture_output=True, text=True)
        if r.returncode == 0:
            try:
                return out_png, json.loads(r.stdout)
            except json.JSONDecodeError as e:
                last = f"bad json: {e}"
        else:
            last = (r.stderr or "describe failed").strip()[-300:]
        time.sleep(3 * (attempt + 1))
    return out_png, {"error": last or "describe failed after retries"}


def aggregate(manifest: list[dict], results: dict, errors: dict) -> tuple[list[dict], dict]:
    rows = []
    for screen in manifest:
        sid = screen["id"]
        if sid in errors:
            rows.append({"id": sid, "name": screen["name"], "clarity_0_10": None,
                         "screenshot_kind": "", "first_impression": "",
                         "ctas": "", "price_mentions": "", "issues": "",
                         "evidence_id": "", "error": errors[sid]})
            continue
        res = results.get(sid)
        if not res:
            continue
        if res.get("error"):
            rows.append({"id": sid, "name": screen["name"], "clarity_0_10": None,
                         "screenshot_kind": "", "first_impression": "",
                         "ctas": "", "price_mentions": "", "issues": "",
                         "evidence_id": "", "error": res["error"]})
            continue
        desc = res.get("description", {})
        rows.append({
            "id": sid, "name": screen["name"],
            "clarity_0_10": desc.get("clarity_0_10"),
            "screenshot_kind": desc.get("screenshot_kind", ""),
            "first_impression": desc.get("first_impression", ""),
            "ctas": "; ".join(desc.get("ctas", [])),
            "price_mentions": "; ".join(desc.get("price_mentions", [])),
            "issues": " | ".join(desc.get("issues", [])),
            "evidence_id": res.get("evidence_id", ""),
        })
    rows.sort(key=lambda r: (r["clarity_0_10"] is None, r["clarity_0_10"] or 0))
    clarities = [r["clarity_0_10"] for r in rows
                 if isinstance(r["clarity_0_10"], (int, float))]
    ok = sum(1 for r in rows if "error" not in r)
    stats = {
        "total_screens": len(manifest),
        "ok": ok,
        "failed": sum(1 for r in rows if "error" in r),
        "avg_clarity": round(sum(clarities) / len(clarities), 2) if clarities else None,
        "with_issues": sum(1 for r in rows if r["issues"]),
        "with_ctas": sum(1 for r in rows if r["ctas"]),
        "with_price": sum(1 for r in rows if r["price_mentions"]),
    }
    return rows, stats


def write_report(rows: list[dict], stats: dict, out_dir: str) -> None:
    os.makedirs(out_dir, exist_ok=True)
    with open(os.path.join(out_dir, "report.json"), "w", encoding="utf-8") as fh:
        json.dump({"stats": stats, "screens": rows}, fh, ensure_ascii=False, indent=2)
    with open(os.path.join(out_dir, "report.csv"), "w", newline="",
              encoding="utf-8") as fh:
        writer = csv.DictWriter(fh, fieldnames=[
            "id", "name", "clarity_0_10", "screenshot_kind", "ctas",
            "price_mentions", "issues", "first_impression", "evidence_id", "error"])
        writer.writeheader()
        writer.writerows(rows)
    print(f"\n== AUDIT ==\n{json.dumps(stats, ensure_ascii=False, indent=2)}")
    print(f"report: {out_dir}/report.json , {out_dir}/report.csv")


def build_aggregate_evidence(rows: list[dict]) -> dict:
    lines = [f"- [{r['clarity_0_10'] if r['clarity_0_10'] is not None else '?'}/10] "
             f"{r['name']} :: {r['first_impression']} "
             f"CTAs={r['ctas'] or '—'} precios={r['price_mentions'] or '—'} "
             f"problemas={r['issues'] or '—'}" for r in rows if "error" not in r]
    now = datetime.now(timezone.utc).isoformat(timespec="seconds").replace("+00:00", "Z")
    return {
        "id": "vision-aggregate",
        "source": "vision audit of all screens",
        "provider": "vision_ocr",
        "kind": "simulated",
        "published_at": None,
        "collected_at": now,
        "observed_at": now,
        "segment": None,
        "population": None,
        "content": "Auditoría visual de todas las pantallas:\n" + "\n".join(lines),
        "limitations": ["Synthetic vision/OCR description; not independent human observation."],
        "synthetic_fixture": True,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--screens-dir", default="output/screens")
    parser.add_argument("--out-dir", default="output/audit")
    parser.add_argument("--pen", default=os.environ.get("PEN", "pen"))
    parser.add_argument("--export-scale", type=int, default=2)
    parser.add_argument("--binary", default=os.environ.get(
        "MARKETSWARM_BIN", "./target/debug/market-swarm"))
    parser.add_argument("--db", default="state/swarm.sqlite")
    parser.add_argument("--mode", default="openrouter",
                        choices=["openrouter", "local", "offline"])
    parser.add_argument("--prompt", default=DEFAULT_PROMPT)
    parser.add_argument("--export-workers", type=int, default=8)
    parser.add_argument("--describe-workers", type=int, default=3)
    parser.add_argument("--limit", type=int, default=0)
    parser.add_argument("--skip-export", action="store_true")
    parser.add_argument("--swarm", action="store_true")
    parser.add_argument("--question", default="¿Descargarían y pagarían por esta app?")
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()

    manifest = load_manifest(args.screens_dir)
    if args.limit:
        manifest = manifest[: args.limit]
    print(f"screens to audit: {len(manifest)}", flush=True)

    png_dir = os.path.join(args.out_dir, "png")
    os.makedirs(png_dir, exist_ok=True)
    pngs: dict[str, str] = {}
    errors: dict[str, str] = {}

    if not args.skip_export:
        print("[1/3] exportando PNGs (pen --export)…", flush=True)
        jobs = [(args.pen, s, os.path.join(png_dir, f"{s['id']}.png"),
                 args.export_scale) for s in manifest]
        if args.dry_run:
            pngs = {s["id"]: os.path.join(png_dir, f"{s['id']}.png") for s in manifest}
            print(f"  (dry-run) {len(jobs)} exports", flush=True)
        else:
            done = 0
            with ThreadPoolExecutor(max_workers=args.export_workers) as pool:
                futures = [pool.submit(export_one, j) for j in jobs]
                for fut in as_completed(futures):
                    sid, png, err = fut.result()
                    done += 1
                    if png:
                        pngs[sid] = png
                    else:
                        errors[sid] = err
                    if done % 25 == 0:
                        print(f"  exportados {done}/{len(manifest)}", flush=True)
    else:
        pngs = {s["id"]: os.path.join(png_dir, f"{s['id']}.png") for s in manifest}

    print(f"[2/3] visión + OCR ({len(pngs)} pantallas)…", flush=True)
    results: dict[str, dict] = {}
    if args.dry_run:
        print(f"  (dry-run) {len(pngs)} describe calls", flush=True)
    else:
        jobs = [(args.binary, args.db, args.mode, args.prompt, png)
                for png in pngs.values()]
        done = 0
        with ThreadPoolExecutor(max_workers=args.describe_workers) as pool:
            futures = [pool.submit(describe_one, j) for j in jobs]
            for fut in as_completed(futures):
                png, res = fut.result()
                done += 1
                sid = os.path.splitext(os.path.basename(png))[0]
                results[sid] = res
                if done % 20 == 0:
                    print(f"  analizadas {done}/{len(pngs)}", flush=True)

    rows, stats = aggregate(manifest, results, errors)
    if not args.dry_run:
        write_report(rows, stats, args.out_dir)
        if stats["failed"]:
            with open(os.path.join(args.out_dir, "failures.json"), "w",
                      encoding="utf-8") as fh:
                json.dump([r for r in rows if "error" in r], fh,
                          ensure_ascii=False, indent=2)
            print(f"fallidas: {stats['failed']} -> {args.out_dir}/failures.json")
    else:
        print(f"  (dry-run) aggregate {len(rows)} rows", flush=True)

    if args.swarm:
        print("[3/3] swarm sobre el agregado…", flush=True)
        evidence = build_aggregate_evidence(rows)
        import_path = os.path.join(args.out_dir, "aggregate-evidence.json")
        if args.dry_run:
            print(f"  (dry-run) import {import_path} then run swarm", flush=True)
        else:
            with open(import_path, "w", encoding="utf-8") as fh:
                json.dump([evidence], fh, ensure_ascii=False, indent=2)
            run_cmd([args.binary, "--db", args.db, "import", import_path])
            request = {
                "question": args.question,
                "as_of": datetime.now(timezone.utc).isoformat(timespec="seconds")
                         .replace("+00:00", "Z"),
                "modules": ["product_discovery", "sentiment", "recommendations",
                            "pricing", "willingness_to_pay", "social_simulation",
                            "experimentation"],
                "evidence_ids": [evidence["id"]],
                "context": {"app": "Mantralia", "audit": stats},
                "limits": {"concurrency": 3, "timeout_seconds": 180,
                           "max_output_tokens": 8192, "max_cost_usd": 1.0,
                           "reasoning_enabled": False},
            }
            req_path = os.path.join(args.out_dir, "swarm-request.json")
            with open(req_path, "w", encoding="utf-8") as fh:
                json.dump(request, fh, ensure_ascii=False, indent=2)
            out = run_cmd([args.binary, "--db", args.db, "run", req_path,
                           "--mode", args.mode, "--output", args.out_dir])
            print(json.loads(out.stdout))
    else:
        print("[3/3] swarm omitido (usa --swarm para añadirlo)", flush=True)

    return 0


if __name__ == "__main__":
    sys.exit(main())
