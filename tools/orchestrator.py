#!/usr/bin/env python3
"""UX swarm loop: pen.dev/Figma -> vision+OCR -> market-swarm -> modifications.

This is the thin orchestrator (part "A") that reuses the native capabilities
added to `market-swarm` (part "B"):

  1. capture screenshots   pen CLI export  OR  MCP (Figma / pen.dev live)
  2. `market-swarm describe`   vision + OCR of the screenshots (VL model)
  3. `market-swarm run`        12-specialist swarm (WTP, download intent)
  4. apply variant          pen CLI modify  OR  MCP write tool

The loop: capture -> describe (evidence) -> research (report) -> apply.

Two backends:

  pen  (default)  uses the headless `@pen.dev/cli`: `pen --in x.pen --export shot.png`
                  and `pen --in x.pen --out y.pen --prompt "..."`. No desktop app or
                  MCP handshake needed. Requires `npm i -g @pen.dev/cli` + `pen login`
                  (or PEN_CLI_KEY).

  mcp            uses `market-swarm mcp` (native Rust MCP client) against any stdio
                  MCP server, e.g. pen.dev live (`npx -y @pen.dev/mcp-server`) or Figma
                  (`npx -y @modelcontextprotocol/server-figma --stdio`).

Only the Python standard library is used. Run with `--dry-run` to print the exact
commands without touching the network or the pen.dev API.
"""

from __future__ import annotations

import argparse
import base64
import json
import os
import subprocess
import sys
import tempfile
from datetime import datetime, timezone

DEFAULT_PROMPT = (
    "Describe each supplied app screenshot: what it shows, all visible text, "
    "and any UX or conversion problems."
)

UX_MODULES = [
    "product_discovery",
    "sentiment",
    "recommendations",
    "pricing",
    "willingness_to_pay",
    "social_simulation",
    "experimentation",
]


def run_cmd(argv: list[str], check: bool = True) -> subprocess.CompletedProcess:
    result = subprocess.run(argv, capture_output=True, text=True)
    if check and result.returncode != 0:
        raise SystemExit(
            f"command failed ({result.returncode}): {' '.join(argv)}\n"
            f"stderr: {result.stderr.strip()}"
        )
    return result


# --------------------------------------------------------------------------- #
# MCP backend helpers
# --------------------------------------------------------------------------- #
def mcp_call(binary: str, command: str, tool: str, args: dict) -> dict:
    argv = [
        binary, "mcp", "--command", command,
        "--action", "call", "--tool", tool,
        "--args", json.dumps(args),
    ]
    return json.loads(run_cmd(argv).stdout)


def extract_images(result: dict, out_dir: str) -> list[str]:
    os.makedirs(out_dir, exist_ok=True)
    paths: list[str] = []
    for i, block in enumerate(result.get("content", [])):
        if isinstance(block, dict) and block.get("type") == "image":
            data = block.get("data", "")
            mime = block.get("mimeType", "image/png")
            ext = {"image/png": "png", "image/jpeg": "jpg",
                   "image/webp": "webp", "image/gif": "gif"}.get(mime, "png")
            if data.startswith("data:"):
                data = data.split(",", 1)[1]
            raw = base64.b64decode(data)
            path = os.path.join(out_dir, f"screenshot-{i}.{ext}")
            with open(path, "wb") as handle:
                handle.write(raw)
            paths.append(path)
        elif isinstance(block, dict) and block.get("type") == "text":
            print(f"[mcp text] {block.get('text', '')[:500]}")
    return paths


# --------------------------------------------------------------------------- #
# pen.dev CLI backend helpers
# --------------------------------------------------------------------------- #
def pen_capture(pen: str, design: str, out_png: str, scale: int) -> list[str]:
    os.makedirs(os.path.dirname(out_png) or ".", exist_ok=True)
    run_cmd([
        pen, "--in", design, "--export", out_png,
        "--export-type", "png", "--export-scale", str(scale),
    ])
    return [out_png]


def pen_apply(pen: str, design: str, out_design: str, prompt: str) -> str:
    run_cmd([pen, "--in", design, "--out", out_design, "--prompt", prompt])
    return out_design


# --------------------------------------------------------------------------- #
# market-swarm steps
# --------------------------------------------------------------------------- #
def describe(binary: str, db: str, files: list[str], prompt: str, mode: str) -> dict:
    argv = [binary, "--db", db, "describe", *files, "--import", "--mode", mode]
    if prompt:
        argv += ["--prompt", prompt]
    return json.loads(run_cmd(argv).stdout)


def build_request(question: str, evidence_ids: list[str], context: dict,
                  as_of: str) -> dict:
    return {
        "question": question,
        "as_of": as_of,
        "modules": UX_MODULES,
        "evidence_ids": evidence_ids,
        "context": context,
        "limits": {
            "concurrency": 3,
            "timeout_seconds": 180,
            "max_output_tokens": 8192,
            "max_cost_usd": 1.0,
            "reasoning_enabled": False,
        },
    }


def run_swarm(binary: str, db: str, request: dict, mode: str, out_dir: str) -> dict:
    os.makedirs(out_dir, exist_ok=True)
    handle, path = tempfile.mkstemp(suffix=".json", prefix="ux-request-")
    try:
        with os.fdopen(handle, "w") as fh:
            json.dump(request, fh, indent=2)
        argv = [binary, "--db", db, "run", path, "--mode", mode, "--output", out_dir]
        return json.loads(run_cmd(argv).stdout)
    finally:
        os.unlink(path)


def synthesize_recommendations(report: dict) -> str:
    synthesis = report.get("synthesis", {}) or {}
    output = synthesis.get("output") or {}
    summary = output.get("summary", "")
    findings = "; ".join(f.get("statement", "") for f in output.get("findings", []))
    return f"{summary}\n{findings}".strip()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--backend", default="pen", choices=["pen", "mcp"])
    # pen.dev CLI backend
    parser.add_argument("--pen", default=os.environ.get("PEN", "pen"))
    parser.add_argument("--design", default=None, help="input .pen file (pen backend)")
    parser.add_argument("--export-scale", type=int, default=2)
    # MCP backend
    parser.add_argument("--mcp-command", default=os.environ.get(
        "DESIGN_MCP_COMMAND", "npx -y @pen.dev/mcp-server"))
    parser.add_argument("--screenshot-tool", default="get_screenshot")
    parser.add_argument("--screenshot-args", default='{}')
    parser.add_argument("--apply-tool", default=None)
    parser.add_argument("--apply-args", default='{}')
    # market-swarm
    parser.add_argument("--binary", default=os.environ.get(
        "MARKETSWARM_BIN", "./target/debug/market-swarm"))
    parser.add_argument("--db", default="state/swarm.sqlite")
    parser.add_argument("--mode", default="openrouter",
                        choices=["openrouter", "local", "offline"])
    parser.add_argument("--prompt", default=DEFAULT_PROMPT)
    parser.add_argument("--question", required=True)
    parser.add_argument("--app-name", default="app")
    parser.add_argument("--price-cents", type=int, default=500)
    parser.add_argument("--currency", default="USD")
    parser.add_argument("--billing-period", default="monthly")
    parser.add_argument("--audience", default="segmento por definir")
    parser.add_argument("--out-dir", default="output")
    parser.add_argument("--iterations", type=int, default=1)
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()

    if args.backend == "pen" and not args.design:
        parser.error("--design is required for the pen backend")

    as_of = datetime.now(timezone.utc).isoformat(timespec="seconds").replace("+00:00", "Z")
    design = args.design
    recommendations = ""

    for iteration in range(1, args.iterations + 1):
        print(f"\n=== iteration {iteration}/{args.iterations} ===")

        # 1) capture screenshots
        print(f"[1/4] capture (backend={args.backend})")
        if args.backend == "pen":
            out_png = os.path.join(args.out_dir, f"screenshot-{iteration}.png")
            if args.dry_run:
                images = [out_png]
                print(f"  (dry-run) {args.pen} --in {design} --export {out_png} "
                      f"--export-scale {args.export_scale}")
            else:
                images = pen_capture(args.pen, design, out_png, args.export_scale)
        else:
            screenshot_args = json.loads(args.screenshot_args)
            if args.dry_run:
                images = [os.path.join(args.out_dir, "screenshot-0.png")]
                print(f"  (dry-run) mcp call {args.screenshot_tool} "
                      f"{json.dumps(screenshot_args)}")
            else:
                shot = mcp_call(args.binary, args.mcp_command,
                                args.screenshot_tool, screenshot_args)
                images = extract_images(shot, args.out_dir)

        # 2) vision + OCR, imported as evidence
        print(f"[2/4] vision + OCR via 'describe' ({len(images)} images)")
        if args.dry_run:
            evidence_id = "vision-<dry-run>"
        else:
            desc = describe(args.binary, args.db, images, args.prompt, args.mode)
            evidence_id = desc.get("evidence_id", "")
            clarity = desc.get("description", {}).get("clarity_0_10")
            print(f"  evidence_id={evidence_id} clarity={clarity}")

        # 3) swarm research
        print(f"[3/4] swarm research ({','.join(UX_MODULES)})")
        context = {
            "app": args.app_name,
            "currency": args.currency,
            "billing_period": args.billing_period,
            "price_cents": args.price_cents,
            "audience": args.audience,
            "visual_evidence_id": evidence_id,
        }
        request = build_request(args.question, [evidence_id], context, as_of)
        if args.dry_run:
            print("  (dry-run) request:", json.dumps(request, indent=2)[:400])
        else:
            report = run_swarm(args.binary, args.db, request, args.mode, args.out_dir)
            recommendations = synthesize_recommendations(report)
            print(f"  run_id={report.get('run_id')} status={report.get('status')}")
            print(f"  report_json={report.get('report_json')}")

        # 4) apply the recommended variant
        if args.backend == "pen":
            print("[4/4] apply via pen CLI")
            stem = os.path.splitext(design)[0]
            out_design = f"{stem}-v{iteration}.pen"
            prompt = recommendations or "Improve clarity and conversion of this screen."
            if args.dry_run:
                print(f"  (dry-run) {args.pen} --in {design} --out {out_design} "
                      f"--prompt \"{prompt[:120]}...\"")
            else:
                design = pen_apply(args.pen, design, out_design, prompt)
                print(f"  new design: {design}")
        elif args.apply_tool:
            print(f"[4/4] apply via MCP tool '{args.apply_tool}'")
            if args.dry_run:
                print("  (dry-run) skipping apply call")
            else:
                args_json = json.dumps(json.loads(args.apply_args)).replace(
                    "{recommendations}", recommendations)
                applied = mcp_call(args.binary, args.mcp_command,
                                   args.apply_tool, json.loads(args_json))
                print(f"  applied: {json.dumps(applied)[:300]}")
        else:
            print("[4/4] apply skipped (no --apply-tool for mcp backend)")

    return 0


if __name__ == "__main__":
    sys.exit(main())
