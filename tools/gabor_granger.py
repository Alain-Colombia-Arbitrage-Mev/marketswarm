#!/usr/bin/env python3
"""Synthetic Gabor-Granger pilot for Mantralia Premium.

Generates a panel of artificial respondents, each with a willingness-to-pay
threshold, records their yes/no at each price point, and runs market-swarm's
deterministic WTP evaluation (offline). The result is clearly synthetic: it is
a pricing *hypothesis*, not validated human demand.
"""
import json
import math
import os
import random
import subprocess
import sys
from datetime import datetime, timedelta, timezone

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BINARY = os.environ.get("MARKETSWARM_BIN", os.path.join(BASE, "target", "debug", "market-swarm"))
DB = os.path.join(BASE, "state", "swarm.sqlite")
OUT = os.path.join(BASE, "output", "gabor")

OFFER = "Mantralia Premium"
SEGMENT = ("usuarios globales y LATAM hispanohablantes interesados en meditación, "
           "mandalas y espiritualidad (sintético)")
CURRENCY = "USD"
PERIOD = "monthly"
PRICES = [399, 599, 799, 999, 1299, 1599]  # cents
N_PARTICIPANTS = 200
SEED = 42

random.seed(SEED)
now = datetime.now(timezone.utc)
registered = (now - timedelta(days=2)).isoformat(timespec="seconds")
recorded = (now - timedelta(hours=1)).isoformat(timespec="seconds")
closes = (now + timedelta(days=5)).isoformat(timespec="seconds")
as_of = now.isoformat(timespec="seconds")

responses = []
thresholds = []
for i in range(N_PARTICIPANTS):
    thresh = max(199, int(round(random.lognormvariate(math.log(899), 0.55))))
    thresholds.append(thresh)
    pid = f"gg-{i:03d}"
    for p in PRICES:
        responses.append({
            "participant_id": pid,
            "price_cents": p,
            "outcome": "yes" if thresh >= p else "no",
            "evidence_id": "mantralia-gg-survey-v2",
            "recorded_at": recorded,
        })

evidence = [{
    "id": "mantralia-gg-survey-v2",
    "source": "synthetic gabor-granger panel",
    "provider": "synthetic",
    "kind": "simulated",
    "published_at": None,
    "collected_at": now.isoformat(timespec="seconds"),
    "observed_at": now.isoformat(timespec="seconds"),
    "segment": SEGMENT,
    "population": "panel sintético global + LATAM hispanohablante",
    "content": ("Respuestas sintéticas de disposición a pagar (Gabor-Granger) de un "
                "panel artificial de 200 personas con umbrales de pago simulados."),
    "limitations": ["Datos 100% sintéticos; no representan demanda humana real."],
    "synthetic_fixture": True,
}]

study = {
    "offer": OFFER,
    "segment": SEGMENT,
    "currency": CURRENCY,
    "billing_period": PERIOD,
    "kind": "simulated",
    "protocol": {
        "method": "gabor_granger",
        "registered_at": registered,
        "closes_at": closes,
        "minimum_per_price": 20,
        "maximum_half_width": 0.08,
        "population": "panel sintético global + LATAM hispanohablante",
        "allocation": "illustrative",
        "maturity_days": 0,
    },
    "responses": responses,
}

request = {
    "question": ("Estimación sintética de disposición a pagar por Mantralia Premium "
                 "(Gabor-Granger). No validar con datos reales."),
    "as_of": as_of,
    "modules": ["willingness_to_pay"],
    "evidence_ids": ["mantralia-gg-survey-v2"],
    "context": {"offer": OFFER, "currency": CURRENCY, "billing_period": PERIOD},
    "limits": {"concurrency": 1, "timeout_seconds": 60, "max_output_tokens": 1024,
               "max_cost_usd": 1.0},
    "study": study,
}

os.makedirs(OUT, exist_ok=True)
with open(os.path.join(OUT, "evidence.json"), "w", encoding="utf-8") as fh:
    json.dump(evidence, fh, ensure_ascii=False, indent=2)
with open(os.path.join(OUT, "request.json"), "w", encoding="utf-8") as fh:
    json.dump(request, fh, ensure_ascii=False, indent=2)

subprocess.run([BINARY, "--db", DB, "import", os.path.join(OUT, "evidence.json")],
               check=True)
subprocess.run([BINARY, "--db", DB, "run", os.path.join(OUT, "request.json"),
                "--mode", "offline", "--output", OUT], check=True,
               capture_output=True)

# Read the produced report (find the newest run dir).
runs = [d for d in os.listdir(OUT) if os.path.isdir(os.path.join(OUT, d))]
runs.sort(key=lambda d: os.path.getmtime(os.path.join(OUT, d)))
report = json.load(open(os.path.join(OUT, runs[-1], "report.json"), encoding="utf-8"))
wtp = report["numerical"]["wtp"]

print("\n=== GABOR-GRANGER (sintético) ===")
print("oferta:", OFFER, "| segmento:", SEGMENT)
print("participantes:", N_PARTICIPANTS, "| precios:", PRICES)
print("conclusión:", wtp["conclusion"], "| evidencia:", wtp["evidence_kind"])
print("\n% aceptación por precio:")
for p in wtp["curve"]:
    price = p["price_cents"] / 100
    print(f"  ${price:6.2f}  aceptación {p['acceptance']*100:5.1f}%  "
          f"({p['accepted']}/{p['participants']})  Wilson95 {p['wilson_95'][0]:.2f}-{p['wilson_95'][1]:.2f}")

best = max(wtp["curve"], key=lambda p: p["gross_per_exposed_cents"])
print(f"\nPrecio que maximiza ingreso por usuario expuesto: "
      f"${best['price_cents']/100:.2f} "
      f"(ingreso esperado ${best['gross_per_exposed_cents']/100:.2f}/usuario)")
print("\nLIMITACIONES:")
for l in wtp["limitations"]:
    print("  -", l)
