#!/usr/bin/env python3
"""Regenerate the final audit report from persisted evidence and run the swarm.

The batch runner stores each successful vision result as `Evidence` in SQLite
(provider `vision_ocr`). This rebuilds `report.csv` / `report.json` from the DB
(so retried screens are included), then chunks the aggregate into several
evidence records (under the 100k-char per-record limit) and runs the swarm.

Usage:
  python3 tools/finalize_audit.py --binary ./target/debug/market-swarm --swarm
"""

from __future__ import annotations

import argparse
import csv
import json
import os
import sqlite3
import subprocess
import sys
from datetime import datetime, timezone

FIELDS = ["id", "name", "clarity_0_10", "screenshot_kind", "ctas",
          "price_mentions", "issues", "first_impression", "evidence_id"]


def load_rows(db: str, manifest_path: str) -> list[dict]:
    manifest = {m["id"]: m["name"]
                for m in json.load(open(manifest_path, encoding="utf-8"))}
    conn = sqlite3.connect(db)
    rows = []
    for (body,) in conn.execute("SELECT body FROM evidence").fetchall():
        e = json.loads(body)
        if e.get("provider") != "vision_ocr":
            continue
        sid = os.path.splitext(os.path.basename(e.get("source", "")))[0]
        if sid not in manifest:
            continue
        try:
            desc = json.loads(e.get("content", "{}"))
        except Exception:
            continue
        rows.append({
            "id": sid, "name": manifest[sid],
            "clarity_0_10": desc.get("clarity_0_10"),
            "screenshot_kind": desc.get("screenshot_kind", ""),
            "ctas": "; ".join(desc.get("ctas", [])),
            "price_mentions": "; ".join(desc.get("price_mentions", [])),
            "issues": " | ".join(desc.get("issues", [])),
            "first_impression": desc.get("first_impression", ""),
            "evidence_id": e.get("id", ""),
        })
    conn.close()
    rows.sort(key=lambda r: (r["clarity_0_10"] is None, r["clarity_0_10"] or 0))
    return rows


def stats(rows: list[dict]) -> dict:
    clar = [r["clarity_0_10"] for r in rows
            if isinstance(r["clarity_0_10"], (int, float))]
    return {
        "total_screens": len(rows),
        "avg_clarity": round(sum(clar) / len(clar), 2) if clar else None,
        "with_issues": sum(1 for r in rows if r["issues"]),
        "with_ctas": sum(1 for r in rows if r["ctas"]),
        "with_price": sum(1 for r in rows if r["price_mentions"]),
    }


def chunk_evidence(rows: list[dict], max_chars: int = 80_000) -> list[dict]:
    now = datetime.now(timezone.utc).isoformat(timespec="seconds").replace("+00:00", "Z")
    lines = [f"- [{r['clarity_0_10']}/10] {r['name']} :: {r['first_impression']} "
             f"CTAs={r['ctas'] or '—'} precios={r['price_mentions'] or '—'} "
             f"problemas={r['issues'] or '—'}" for r in rows]
    chunks, current, size = [], [], 0
    for line in lines:
        if current and size + len(line) + 1 > max_chars:
            chunks.append(current)
            current, size = [], 0
        current.append(line)
        size += len(line) + 1
    if current:
        chunks.append(current)
    return [{
        "id": f"vision-aggregate-{i}",
        "source": "vision audit of all screens (chunked)",
        "provider": "vision_ocr",
        "kind": "simulated",
        "published_at": None,
        "collected_at": now,
        "observed_at": now,
        "segment": None,
        "population": None,
        "content": f"Auditoría visual (parte {i + 1}/{len(chunks)}):\n" + "\n".join(c),
        "limitations": ["Synthetic vision/OCR description; not independent human observation."],
        "synthetic_fixture": True,
    } for i, c in enumerate(chunks)]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--db", default="state/swarm.sqlite")
    parser.add_argument("--manifest", default="output/screens/manifest.json")
    parser.add_argument("--out-dir", default="output/audit")
    parser.add_argument("--binary", default=os.environ.get(
        "MARKETSWARM_BIN", "./target/debug/market-swarm"))
    parser.add_argument("--mode", default="openrouter")
    parser.add_argument("--question", default="¿Descargarían y pagarían por esta app?")
    parser.add_argument("--swarm", action="store_true")
    args = parser.parse_args()

    rows = load_rows(args.db, args.manifest)
    s = stats(rows)
    os.makedirs(args.out_dir, exist_ok=True)
    with open(os.path.join(args.out_dir, "report.json"), "w", encoding="utf-8") as fh:
        json.dump({"stats": s, "screens": rows}, fh, ensure_ascii=False, indent=2)
    with open(os.path.join(args.out_dir, "report.csv"), "w", newline="",
              encoding="utf-8") as fh:
        w = csv.DictWriter(fh, fieldnames=FIELDS)
        w.writeheader()
        w.writerows(rows)
    print(f"reporte regenerado: {s}", flush=True)
    print(f"  {args.out_dir}/report.csv ({len(rows)} filas)", flush=True)

    if not args.swarm:
        return 0

    evidence = chunk_evidence(rows)
    import_path = os.path.join(args.out_dir, "aggregate-evidence.json")
    with open(import_path, "w", encoding="utf-8") as fh:
        json.dump(evidence, fh, ensure_ascii=False, indent=2)
    subprocess.run([args.binary, "--db", args.db, "import", import_path],
                   check=True)
    request = {
        "question": args.question,
        "as_of": datetime.now(timezone.utc).isoformat(timespec="seconds")
                 .replace("+00:00", "Z"),
        "modules": ["product_discovery", "sentiment", "recommendations",
                    "pricing", "willingness_to_pay", "social_simulation",
                    "experimentation"],
        "evidence_ids": [e["id"] for e in evidence],
        "context": {"app": "Mantralia", "audit": s},
        "limits": {"concurrency": 3, "timeout_seconds": 180,
                   "max_output_tokens": 8192, "max_cost_usd": 1.0,
                   "reasoning_enabled": False},
    }
    req_path = os.path.join(args.out_dir, "swarm-request.json")
    with open(req_path, "w", encoding="utf-8") as fh:
        json.dump(request, fh, ensure_ascii=False, indent=2)
    print("corriendo swarm…", flush=True)
    out = subprocess.run([args.binary, "--db", args.db, "run", req_path,
                          "--mode", args.mode, "--output", args.out_dir],
                         capture_output=True, text=True)
    if out.returncode != 0:
        print(out.stderr, file=sys.stderr)
        return out.returncode
    print(out.stdout)
    return 0


if __name__ == "__main__":
    sys.exit(main())
