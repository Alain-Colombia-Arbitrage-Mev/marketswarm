#!/usr/bin/env python3
"""Extract frames from a .pen file into a standalone .pen.

Prefer `market-swarm ux audit design.pen` to analyze a design. This script only
splits frames so the pen.dev CLI can export one screen to PNG.

pen.dev `.pen` files are JSON. A large "product map" can hold hundreds of
screens as top-level `children`. This isolates one or more screens (by exact id
or name substring) into their own .pen so the UX swarm loop tests a single
screen instead of a 249-screen map.

Usage:
  python3 tools/pen_extract.py --in design.pen --list
  python3 tools/pen_extract.py --in design.pen --out screen.pen --id imeOI
  python3 tools/pen_extract.py --in design.pen --out screens.pen --name "Mi plan"
  python3 tools/pen_extract.py --in design.pen --all-screens output/screens/
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys


def frames_by(root: dict) -> list[dict]:
    return [c for c in root.get("children", []) if isinstance(c, dict)]


def list_frames(root: dict) -> None:
    for c in frames_by(root):
        print(f"{c.get('id','?'):10s}  {c.get('width','?')}x{c.get('height','?')}  "
              f"{c.get('name','(sin nombre)')}")


def extract(root: dict, ids: set[str] | None, name: str | None) -> list[dict]:
    picked = []
    for c in frames_by(root):
        if ids and c.get("id") in ids:
            picked.append(c)
        elif name and name.lower() in (c.get("name") or "").lower():
            picked.append(c)
    return picked


def safe_slug(name: str, frame_id: str) -> str:
    slug = re.sub(r"[^0-9A-Za-z]+", "-", name or "").strip("-").lower()
    return f"{frame_id}-{slug}" if slug else frame_id


def clone_with(root: dict, children: list[dict]) -> dict:
    out = {k: v for k, v in root.items() if k != "children"}
    out["children"] = children
    return out


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--in", dest="src", required=True, help="input .pen")
    parser.add_argument("--out", dest="out", help="output .pen (default stdout)")
    parser.add_argument("--id", action="append", default=[],
                        help="exact frame id; repeatable")
    parser.add_argument("--name", default=None, help="substring match on name")
    parser.add_argument("--list", action="store_true",
                        help="list top-level frames and exit")
    parser.add_argument("--all-screens", default=None, metavar="DIR",
                        help="extract every phone screen (440x956) into DIR "
                             "and write DIR/manifest.json")
    args = parser.parse_args()

    root = json.load(open(args.src, encoding="utf-8"))

    if args.list:
        list_frames(root)
        return 0

    if args.all_screens:
        os.makedirs(args.all_screens, exist_ok=True)
        screens = [c for c in frames_by(root)
                   if c.get("width") == 440 and c.get("height") == 956]
        manifest = []
        for c in screens:
            fname = f"{safe_slug(c.get('name'), c.get('id'))}.pen"
            path = os.path.join(args.all_screens, fname)
            with open(path, "w", encoding="utf-8") as fh:
                json.dump(clone_with(root, [c]), fh, ensure_ascii=False, indent=2)
            manifest.append({
                "id": c.get("id"),
                "name": c.get("name"),
                "file": fname,
                "width": c.get("width"),
                "height": c.get("height"),
            })
        manifest.sort(key=lambda m: m["file"])
        with open(os.path.join(args.all_screens, "manifest.json"), "w",
                  encoding="utf-8") as fh:
            json.dump(manifest, fh, ensure_ascii=False, indent=2)
        print(f"extracted {len(screens)} phone screens into {args.all_screens}")
        return 0

    picked = extract(root, set(args.id) or None, args.name)
    if not picked:
        print("No frames matched; use --list to see ids/names.", file=sys.stderr)
        return 1

    text = json.dumps(clone_with(root, picked), ensure_ascii=False, indent=2)
    if args.out:
        with open(args.out, "w", encoding="utf-8") as fh:
            fh.write(text)
        print(f"wrote {args.out} with {len(picked)} frame(s):",
              ", ".join(f.get('name', f.get('id')) for f in picked))
    else:
        sys.stdout.write(text)
    return 0


if __name__ == "__main__":
    sys.exit(main())
