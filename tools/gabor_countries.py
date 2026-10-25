#!/usr/bin/env python3
"""Gabor-Granger per country (Mexico, Brazil, USA) — synthetic pilot.

For each market it generates a synthetic panel with country-calibrated price
ladders and WTP thresholds, runs market-swarm's deterministic WTP evaluation,
and reports the demand curve plus the highest payable price (acceptance >= 30%)
and the revenue-optimal price.
"""
import json
import math
import os
import random
import subprocess
from datetime import datetime, timedelta, timezone

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BINARY = os.environ.get("MARKETSWARM_BIN", os.path.join(BASE, "target", "debug", "market-swarm"))
DB = os.path.join(BASE, "state", "swarm.sqlite")
OUT = os.path.join(BASE, "output", "gabor")

OFFER = "Mantralia Premium"
COUNTRIES = [
    {"code": "mx", "name": "México", "currency": "MXN",
     "prices": [4900, 7900, 9900, 14900, 19900], "median": 9900, "sigma": 0.55},
    {"code": "br", "name": "Brasil", "currency": "BRL",
     "prices": [1990, 2990, 3990, 4990, 6990], "median": 3990, "sigma": 0.55},
    {"code": "us", "name": "Estados Unidos", "currency": "USD",
     "prices": [499, 699, 999, 1299, 1599], "median": 999, "sigma": 0.55},
]
N = 200
SEED = 7
MIN_ACCEPTANCE = 0.30  # "highest payable" = highest price with >= 30% acceptance


def run_country(cfg, seed):
    random.seed(seed)
    now = datetime.now(timezone.utc)
    registered = (now - timedelta(days=2)).isoformat(timespec="seconds")
    recorded = (now - timedelta(hours=1)).isoformat(timespec="seconds")
    closes = (now + timedelta(days=5)).isoformat(timespec="seconds")
    as_of = now.isoformat(timespec="seconds")

    segment = f"usuarios en {cfg['name']} interesados en meditación/mandalas (sintético)"
    evid = f"mantralia-gg-{cfg['code']}"

    responses = []
    for i in range(N):
        thresh = max(cfg["prices"][0] // 2,
                     int(round(random.lognormvariate(math.log(cfg["median"]), cfg["sigma"]))))
        pid = f"{cfg['code']}-{i:03d}"
        for p in cfg["prices"]:
            responses.append({
                "participant_id": pid, "price_cents": p,
                "outcome": "yes" if thresh >= p else "no",
                "evidence_id": evid, "recorded_at": recorded,
            })

    evidence = [{
        "id": evid, "source": f"synthetic gabor-granger panel {cfg['code']}",
        "provider": "synthetic", "kind": "simulated", "published_at": None,
        "collected_at": now.isoformat(timespec="seconds"),
        "observed_at": now.isoformat(timespec="seconds"),
        "segment": segment, "population": f"panel sintético {cfg['name']}",
        "content": f"Respuestas sintéticas Gabor-Granger de {N} personas en {cfg['name']}.",
        "limitations": ["Datos 100% sintéticos; no representan demanda real."],
        "synthetic_fixture": True,
    }]

    study = {
        "offer": OFFER, "segment": segment, "currency": cfg["currency"],
        "billing_period": "monthly", "kind": "simulated",
        "protocol": {"method": "gabor_granger", "registered_at": registered,
                     "closes_at": closes, "minimum_per_price": 20,
                     "maximum_half_width": 0.08,
                     "population": f"panel sintético {cfg['name']}",
                     "allocation": "illustrative", "maturity_days": 0},
        "responses": responses,
    }
    request = {
        "question": f"Disposición a pagar por {OFFER} en {cfg['name']} (sintético).",
        "as_of": as_of, "modules": ["willingness_to_pay"], "evidence_ids": [evid],
        "context": {"offer": OFFER, "currency": cfg["currency"],
                    "billing_period": "monthly"},
        "limits": {"concurrency": 1, "timeout_seconds": 60,
                   "max_output_tokens": 1024, "max_cost_usd": 1.0},
        "study": study,
    }

    cdir = os.path.join(OUT, cfg["code"])
    os.makedirs(cdir, exist_ok=True)
    with open(os.path.join(cdir, "evidence.json"), "w", encoding="utf-8") as fh:
        json.dump(evidence, fh, ensure_ascii=False, indent=2)
    with open(os.path.join(cdir, "request.json"), "w", encoding="utf-8") as fh:
        json.dump(request, fh, ensure_ascii=False, indent=2)

    subprocess.run([BINARY, "--db", DB, "import", os.path.join(cdir, "evidence.json")],
                   check=True, capture_output=True)
    subprocess.run([BINARY, "--db", DB, "run", os.path.join(cdir, "request.json"),
                    "--mode", "offline", "--output", cdir], check=True,
                   capture_output=True)

    runs = [d for d in os.listdir(cdir) if os.path.isdir(os.path.join(cdir, d))]
    runs.sort(key=lambda d: os.path.getmtime(os.path.join(cdir, d)))
    report = json.load(open(os.path.join(cdir, runs[-1], "report.json"),
                            encoding="utf-8"))
    return report["numerical"]["wtp"]


def summarize(cfg, wtp):
    curve = wtp["curve"]
    viable = [p for p in curve if p["acceptance"] >= MIN_ACCEPTANCE]
    highest = max(viable, key=lambda p: p["price_cents"]) if viable else None
    optimal = max(curve, key=lambda p: p["gross_per_exposed_cents"])
    return {
        "country": cfg["name"], "currency": cfg["currency"],
        "curve": [{"price_cents": p["price_cents"], "acceptance": p["acceptance"],
                   "accepted": p["accepted"], "n": p["participants"]}
                  for p in curve],
        "highest_payable_cents": highest["price_cents"] if highest else None,
        "highest_payable_acceptance": highest["acceptance"] if highest else None,
        "optimal_cents": optimal["price_cents"],
        "optimal_acceptance": optimal["acceptance"],
    }


summary = []
for cfg in COUNTRIES:
    wtp = run_country(cfg, SEED)
    s = summarize(cfg, wtp)
    summary.append(s)
    sym = {"USD": "$", "MXN": "$", "BRL": "R$"}[cfg["currency"]]
    print(f"\n=== {cfg['name']} ({cfg['currency']}) ===")
    for p in wtp["curve"]:
        print(f"  {sym}{p['price_cents']/100:7.2f}  aceptación {p['acceptance']*100:5.1f}%  "
              f"({p['accepted']}/{p['participants']})")
    print(f"  → precio MÁS ALTO pagable (≥{MIN_ACCEPTANCE*100:.0f}%): "
          f"{sym}{s['highest_payable_cents']/100:.2f}")
    print(f"  → precio óptimo por ingreso: {sym}{s['optimal_cents']/100:.2f}")

with open(os.path.join(OUT, "countries-summary.json"), "w", encoding="utf-8") as fh:
    json.dump(summary, fh, ensure_ascii=False, indent=2)
print(f"\nresumen guardado en output/gabor/countries-summary.json")
