#!/usr/bin/env python3
"""Retry the screens that failed the vision step because their PNG (exported at
scale 2) exceeded the conservative context allowance when base64-encoded.

For each failed screen this downscales the PNG to 440px wide (sips, macOS) and
re-runs `market-swarm describe --import`. Results are merged into the DB, so the
final report can be regenerated from evidence afterwards.

Usage:
  python3 tools/retry_failed.py --failures output/audit/failures.json \
      --png-dir output/audit/png --workers 4
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor, as_completed


def retry_one(args: tuple) -> tuple[str, bool, str]:
    sid, png, binary, db, mode, prompt = args
    # Downscale to 440px wide (scale-1 equivalent) so base64 stays well under
    # the model's context allowance.
    subprocess.run(["sips", "--resampleWidth", "440", png, "--out", png],
                   capture_output=True, text=True)
    argv = [binary, "--db", db, "describe", png, "--import", "--mode", mode]
    if prompt:
        argv += ["--prompt", prompt]
    last = ""
    for attempt in range(3):
        r = subprocess.run(argv, capture_output=True, text=True)
        if r.returncode == 0:
            try:
                json.loads(r.stdout)
                return sid, True, ""
            except json.JSONDecodeError as e:
                last = f"bad json: {e}"
        else:
            last = (r.stderr or "describe failed").strip()[-200:]
        time.sleep(3 * (attempt + 1))
    return sid, False, last or "failed after retries"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--failures", default="output/audit/failures.json")
    parser.add_argument("--png-dir", default="output/audit/png")
    parser.add_argument("--binary", default="./target/debug/market-swarm")
    parser.add_argument("--db", default="state/swarm.sqlite")
    parser.add_argument("--mode", default="openrouter")
    parser.add_argument("--prompt", default=(
        "Describe esta pantalla de app: qué muestra, todo el texto visible, y "
        "cualquier problema de UX o de conversión (claridad, jerarquía, CTAs, precio)."))
    parser.add_argument("--workers", type=int, default=4)
    args = parser.parse_args()

    with open(args.failures, encoding="utf-8") as fh:
        failed = json.load(fh)
    print(f"retry: {len(failed)} pantallas", flush=True)

    jobs = [(r["id"], os.path.join(args.png_dir, f"{r['id']}.png"),
             args.binary, args.db, args.mode, args.prompt) for r in failed]
    ok, err = 0, []
    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        futures = [pool.submit(retry_one, j) for j in jobs]
        for fut in as_completed(futures):
            sid, success, msg = fut.result()
            if success:
                ok += 1
            else:
                err.append({"id": sid, "error": msg})
    print(f"\nreintentadas OK: {ok}/{len(failed)}", flush=True)
    if err:
        with open(args.failures, "w", encoding="utf-8") as fh:
            json.dump(err, fh, ensure_ascii=False, indent=2)
        print(f"aún fallan {len(err)} -> {args.failures}", flush=True)
        for e in err[:10]:
            print("  ", e["id"], "->", e["error"][:100], flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
