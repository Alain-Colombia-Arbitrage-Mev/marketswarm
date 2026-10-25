#!/usr/bin/env python3
"""Batch-fix the worst screens with the pen.dev agent, then re-audit each.

For each of the N lowest-clarity screens it:
  1. builds a fix prompt from the audit's issues,
  2. runs `pen --in ... --out ... --prompt ...` (agent configurable),
  3. exports the fixed .pen to PNG,
  4. re-runs `market-swarm describe` to measure the new clarity.

Usage:
  python3 tools/fix_screens.py --limit 10 --agent codex
  python3 tools/fix_screens.py --limit 10 --agent openrouter --model "pi;openrouter;anthropic/claude-sonnet-5.5"
"""
import argparse
import glob
import json
import os
import subprocess
import sys

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FIXED = os.path.join(BASE, "output", "fixed")


def load_worst(n: int) -> list[dict]:
    r = json.load(open(os.path.join(BASE, "output", "audit", "report.json"),
                       encoding="utf-8"))
    screens = [s for s in r["screens"]
               if isinstance(s.get("clarity_0_10"), (int, float))]
    screens.sort(key=lambda s: s["clarity_0_10"])
    return screens[:n]


def fix_prompt(s: dict) -> str:
    issues = s["issues"].replace(" | ", "\n- ")
    return (f"Corrige esta pantalla '{s['name']}' manteniendo el estilo azul "
            f"oscuro e Inter. Problemas detectados de claridad/jerarquía/CTA:\n"
            f"- {issues}\n\n"
            f"Solo corrige esos problemas, no cambies el resto del diseño.")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--limit", type=int, default=10)
    ap.add_argument("--agent", default="codex")
    ap.add_argument("--model", default=None)
    ap.add_argument("--pen", default=os.environ.get("PEN", "pen"))
    ap.add_argument("--binary", default=os.environ.get(
        "MARKETSWARM_BIN", os.path.join(BASE, "target", "debug", "market-swarm")))
    ap.add_argument("--db", default=os.path.join(BASE, "state", "swarm.sqlite"))
    ap.add_argument("--mode", default="openrouter")
    args = ap.parse_args()

    os.makedirs(FIXED, exist_ok=True)
    worst = load_worst(args.limit)
    print(f"corrigiendo {len(worst)} pantallas (agent={args.agent})", flush=True)

    results = []
    for s in worst:
        if os.path.exists(os.path.join(FIXED, f"{s['id']}-v2.pen")):
            print(f"  = {s['name'][:42]}: ya corregida, omitida", flush=True)
            continue
        srcs = glob.glob(os.path.join(BASE, "output", "screens",
                                      f"{s['id']}-*.pen"))
        if not srcs:
            results.append({"id": s["id"], "name": s["name"],
                            "before": s["clarity_0_10"], "after": None,
                            "error": "no .pen file"})
            continue
        dst = os.path.join(FIXED, f"{s['id']}-v2.pen")
        cmd = [args.pen, "--in", srcs[0], "--out", dst,
               "--prompt", fix_prompt(s), "--agent", args.agent]
        if args.model:
            cmd += ["--model", args.model]
        r = subprocess.run(cmd, capture_output=True, text=True)
        if r.returncode != 0 or not os.path.exists(dst):
            results.append({"id": s["id"], "name": s["name"],
                            "before": s["clarity_0_10"], "after": None,
                            "error": (r.stderr or "fix failed").strip()[-200:]})
            print(f"  ✗ {s['name'][:42]}: {s['clarity_0_10']} -> ERROR", flush=True)
            continue

        png = os.path.join(FIXED, f"{s['id']}-v2.png")
        subprocess.run([args.pen, "--in", dst, "--export", png,
                        "--export-type", "png", "--export-scale", "1"],
                       capture_output=True, text=True)
        after = None
        d = subprocess.run([args.binary, "--db", args.db, "describe", png,
                            "--mode", args.mode], capture_output=True, text=True)
        if d.returncode == 0:
            try:
                after = json.loads(d.stdout).get("clarity_0_10")
            except Exception:
                after = None
        results.append({"id": s["id"], "name": s["name"],
                        "before": s["clarity_0_10"], "after": after})
        print(f"  {s['name'][:42]}: {s['clarity_0_10']} -> {after}", flush=True)

    with open(os.path.join(FIXED, "before-after.json"), "w", encoding="utf-8") as fh:
        json.dump(results, fh, ensure_ascii=False, indent=2)

    improved = [r for r in results
                if isinstance(r["after"], (int, float)) and r["after"] > r["before"]]
    print(f"\nmejoradas: {len(improved)}/{len(results)}", flush=True)
    print(f"reporte: output/fixed/before-after.json", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
