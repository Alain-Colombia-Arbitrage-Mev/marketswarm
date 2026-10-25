#!/usr/bin/env python3
"""Generate a Word/PDF report for the Gabor-Granger study (reads the latest run)."""
import json
import os
from datetime import datetime, timezone

from docx import Document

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
GABOR = os.path.join(BASE, "output", "gabor")

runs = [d for d in os.listdir(GABOR) if os.path.isdir(os.path.join(GABOR, d))]
runs.sort(key=lambda d: os.path.getmtime(os.path.join(GABOR, d)))
report = json.load(open(os.path.join(GABOR, runs[-1], "report.json"), encoding="utf-8"))

wtp = report["numerical"]["wtp"]
study = report["request"]["study"]
offer = study["offer"]
segment = study["segment"]
currency = study["currency"]
period = study["billing_period"]
participants = len({r["participant_id"] for r in study["responses"]})
prices = [p["price_cents"] for p in wtp["curve"]]
price_str = " · ".join(f"${p/100:.2f}" for p in prices)

doc = Document()
doc.add_heading(f"Estudio Gabor-Granger — {offer}", level=0)
p = doc.add_paragraph()
now = datetime.now(timezone.utc).strftime("%Y-%m-%d")
p.add_run(f"{now} · Piloto sintético · {wtp['evidence_kind']} · "
          f"conclusión: {wtp['conclusion']}").italic = True

doc.add_heading("Parámetros", level=1)
meta = [
    ("Oferta", offer),
    ("Segmento", segment),
    ("Moneda / período", f"{currency} / {period}"),
    ("Participantes", f"{participants} (panel artificial)"),
    ("Precios evaluados", price_str),
    ("Método", "Gabor-Granger (declaración sí/no por precio)"),
]
t = doc.add_table(rows=len(meta), cols=2)
t.style = "Table Grid"
for i, (k, v) in enumerate(meta):
    t.rows[i].cells[0].text = k
    t.rows[i].cells[1].text = v

doc.add_heading("Disposición a pagar por precio", level=1)
tbl = doc.add_table(rows=1, cols=5)
tbl.style = "Table Grid"
for i, h in enumerate(["Precio", "Aceptación", "Sí/No", "IC 95%", "Ingreso/usuario"]):
    tbl.rows[0].cells[i].text = h
    for par in tbl.rows[0].cells[i].paragraphs:
        for run in par.runs:
            run.bold = True

for pnt in wtp["curve"]:
    row = tbl.add_row().cells
    row[0].text = f"${pnt['price_cents']/100:.2f}"
    row[1].text = f"{pnt['acceptance']*100:.1f}%"
    row[2].text = f"{pnt['accepted']}/{pnt['participants']}"
    row[3].text = f"{pnt['wilson_95'][0]*100:.0f}%–{pnt['wilson_95'][1]*100:.0f}%"
    row[4].text = f"${pnt['gross_per_exposed_cents']/100:.2f}"

best = max(wtp["curve"], key=lambda x: x["gross_per_exposed_cents"])
doc.add_heading("Precio recomendado (piloto sintético)", level=1)
doc.add_paragraph(
    f"El precio que maximiza el ingreso por usuario expuesto es "
    f"${best['price_cents']/100:.2f}/mes "
    f"(ingreso esperado ~${best['gross_per_exposed_cents']/100:.2f}/usuario).")

doc.add_heading("Limitaciones", level=1)
for l in wtp["limitations"]:
    doc.add_paragraph(l, style="List Bullet")

out = os.path.join(GABOR, "Gabor-Granger-Mantralia.docx")
doc.save(out)
print("DOCX:", out)
print("segmento:", segment, "| participantes:", participants, "| precios:", prices)
