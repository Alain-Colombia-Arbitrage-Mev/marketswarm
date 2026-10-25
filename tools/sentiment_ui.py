#!/usr/bin/env python3
"""Sentiment analysis of the Mantralia UI using the swarm's sentiment module.

Runs the neural sentiment + product_discovery + recommendations specialists on
the aggregated vision/OCR audit evidence, and reports the sentiment breakdown
by attribute plus concrete improvement suggestions.
"""
import json
import os
import subprocess
import sys
from datetime import datetime, timezone

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BINARY = os.environ.get("MARKETSWARM_BIN", os.path.join(BASE, "target", "debug", "market-swarm"))
DB = os.path.join(BASE, "state", "swarm.sqlite")
OUT = os.path.join(BASE, "output", "sentiment")

EVIDENCE = ["vision-aggregate-0", "vision-aggregate-1"]

request = {
    "question": ("¿Cuál es el sentimiento de los usuarios ante la UI de Mantralia "
                 "y cómo se puede mejorar? Analiza por atributo (utilidad, calidad, "
                 "experiencia, confianza, soporte y precio) y propón mejoras concretas."),
    "as_of": datetime.now(timezone.utc).isoformat(timespec="seconds"),
    "modules": ["sentiment", "product_discovery", "recommendations"],
    "evidence_ids": EVIDENCE,
    "context": {"app": "Mantralia",
                "audit_source": "visión/OCR sintético de 249 pantallas (pen.dev)"},
    "limits": {"concurrency": 3, "timeout_seconds": 180,
               "max_output_tokens": 8192, "max_cost_usd": 1.0,
               "reasoning_enabled": False},
}

os.makedirs(OUT, exist_ok=True)
req_path = os.path.join(OUT, "request.json")
with open(req_path, "w", encoding="utf-8") as fh:
    json.dump(request, fh, ensure_ascii=False, indent=2)

r = subprocess.run([BINARY, "--db", DB, "run", req_path, "--mode", "openrouter",
                    "--output", OUT], capture_output=True, text=True)
if r.returncode != 0:
    print(r.stderr, file=sys.stderr)
    sys.exit(r.returncode)
print(r.stdout)
