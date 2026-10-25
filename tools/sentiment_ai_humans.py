#!/usr/bin/env python3
"""Synthetic sentiment panel: artificial humans score the Mantralia UI.

Eight personas each evaluate the UI (from the audit summary) on six attributes
(1-5) via OpenRouter, then the results are aggregated into a numeric sentiment
score per attribute and an overall distribution. Clearly synthetic: it is a
simulated panel, not real user sentiment.
"""
import json
import os
import statistics
import sys
import time
import urllib.request
from collections import Counter

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MODEL = os.environ.get("SENTIMENT_MODEL", "openai/gpt-5.6-sol")
URL = "https://openrouter.ai/api/v1/chat/completions"

# Read the API key from .env.
key = None
with open(os.path.join(BASE, ".env")) as fh:
    for line in fh:
        if line.startswith("OPENROUTER_API_KEY="):
            key = line.split("=", 1)[1].strip()

report = json.load(open(os.path.join(BASE, "output", "audit", "report.json"),
                        encoding="utf-8"))
stats = report["stats"]
screens = [s for s in report["screens"]
           if isinstance(s.get("clarity_0_10"), (int, float))]
screens.sort(key=lambda s: s["clarity_0_10"])
worst_names = "; ".join(s["name"] for s in screens[:6])

summary = (
    f"Mantralia es una app de meditación/mandalas (tema azul oscuro, tipografía Inter). "
    f"Auditoría de 249 pantallas: claridad media {stats['avg_clarity']}/10, "
    f"{stats['with_issues']} pantallas con problemas detectados, {stats['with_ctas']} con CTA claro, "
    f"y solo {stats['with_price']} menciona precio (no hay paywall). "
    f"Problemas recurrentes: sin botón 'Guardar/Continuar' tras escribir; CTAs duplicados "
    f"('Registrar mi experiencia' vs 'También puedo guardar'); áreas de texto sin indicadores; "
    f"aviso 'NO ES UN DIAGNÓSTICO' pequeño; estados redundantes. "
    f"Pantallas con peor claridad: {worst_names}."
)

PERSONAS = [
    ("Ana, 28, medita a diario", "usuaria frecuente, exige una experiencia fluida y profunda"),
    ("Carlos, 45, escéptico con apps de pago", "muy sensible al precio y al valor percibido"),
    ("Lucía, 22, interesada en tarot y astrología", "valora el contenido espiritual y simbólico"),
    ("Miguel, 35, con ansiedad, busca bienestar y TDAH", "valora claridad, confianza y no sentirse abrumado"),
    ("Sofía, 40, principiante en meditación", "necesita un onboarding simple y guiado"),
    ("Jorge, 52, usuario ocasional", "baja tolerancia a la fricción y a pantallas confusas"),
    ("Valentina, 30, diseñadora UX", "critica la jerarquía visual y la coherencia de CTAs"),
    ("Raúl, 60, poco técnico", "valora simplicidad, legibilidad y botones obvios"),
]


def ask(name, desc):
    prompt = (
        f"Eres {name} ({desc}). Estás evaluando la UI de una app llamada Mantralia. "
        f"Resumen de la UI: {summary}\n"
        f"Desde TU perspectiva personal, puntúa de 1 (muy negativo) a 5 (muy positivo) "
        f"tu sentimiento sobre cada atributo, y da tu sentimiento general. "
        f"Responde SOLO con un objeto JSON con exactamente estos campos: "
        f'{{"utility":1-5,"quality":1-5,"experience":1-5,"trust":1-5,'
        f'"support":1-5,"price":1-5,"overall":"positivo|neutral|negativo",'
        f'"comment":"una frase corta"}}'
    )
    body = {
        "model": MODEL,
        "messages": [{"role": "user", "content": prompt}],
        "response_format": {"type": "json_object"},
        "temperature": 0.4,
    }
    req = urllib.request.Request(
        URL,
        data=json.dumps(body).encode(),
        headers={
            "Authorization": f"Bearer {key}",
            "Content-Type": "application/json",
            "X-OpenRouter-Title": "Mantralia sentiment panel",
        },
    )
    with urllib.request.urlopen(req, timeout=120) as resp:
        data = json.loads(resp.read().decode())
    content = data["choices"][0]["message"]["content"]
    return json.loads(content)


results = []
for name, desc in PERSONAS:
    for attempt in range(3):
        try:
            r = ask(name, desc)
            r["persona"] = name
            results.append(r)
            print(f"  {name}: {r.get('overall')} (exp {r.get('experience')})", flush=True)
            break
        except Exception as e:
            print(f"  ✗ {name}: {str(e)[:80]}", flush=True)
            time.sleep(3 * (attempt + 1))

attrs = ["utility", "quality", "experience", "trust", "support", "price"]
means = {a: round(statistics.mean(r[a] for r in results), 2) for a in attrs}
overall = Counter(r["overall"] for r in results)
total = len(results)
grand = round(statistics.mean(means.values()), 2)

out = {
    "model": MODEL,
    "participants": total,
    "mean_by_attribute": means,
    "overall_distribution": {k: v for k, v in overall.items()},
    "overall_score": grand,
    "responses": results,
    "synthetic": True,
}
os.makedirs(os.path.join(BASE, "output", "sentiment"), exist_ok=True)
with open(os.path.join(BASE, "output", "sentiment", "ai-humans.json"), "w",
          encoding="utf-8") as fh:
    json.dump(out, fh, ensure_ascii=False, indent=2)

print("\n=== SENTIMIENTO SINTÉTICO (humanos artificiales) ===")
for a in attrs:
    print(f"  {a:12s}: {means[a]}/5")
print(f"  {'GENERAL':12s}: {grand}/5")
print(f"  distribución: {dict(overall)}")
print(f"\nGuardado en output/sentiment/ai-humans.json")
