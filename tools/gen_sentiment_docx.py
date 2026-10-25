#!/usr/bin/env python3
"""Generate a Word/PDF report for the synthetic UI sentiment analysis."""
import json
import os
from datetime import datetime, timezone

from docx import Document
from docx.shared import Pt

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SENT = os.path.join(BASE, "output", "sentiment")
data = json.load(open(os.path.join(SENT, "ai-humans.json"), encoding="utf-8"))
means = data["mean_by_attribute"]
dist = data["overall_distribution"]
grand = data["overall_score"]

ATTRS = [("utility", "Utilidad"), ("quality", "Calidad"),
         ("experience", "Experiencia"), ("trust", "Confianza"),
         ("support", "Soporte"), ("price", "Precio")]


def bar(v):
    filled = round(v * 2)
    return "█" * filled + "░" * (10 - filled)


doc = Document()
doc.add_heading("Mantralia — Sentimiento de la UI", level=0)
p = doc.add_paragraph()
now = datetime.now(timezone.utc).strftime("%Y-%m-%d")
p.add_run(f"{now} · Panel sintético de 8 humanos artificiales (GPT) · "
          "no es sentimiento real de usuarios").italic = True

doc.add_heading("Score general", level=1)
doc.add_paragraph(f"Sentimiento general: {grand}/5 "
                  f"({', '.join(f'{k}: {v}' for k, v in dist.items())}).")

doc.add_heading("Sentimiento por atributo", level=1)
tbl = doc.add_table(rows=1, cols=3)
tbl.style = "Table Grid"
for i, h in enumerate(["Atributo", "Score", ""]):
    tbl.rows[0].cells[i].text = h
    for par in tbl.rows[0].cells[i].paragraphs:
        for run in par.runs:
            run.bold = True
for key, label in ATTRS:
    row = tbl.add_row().cells
    row[0].text = label
    row[1].text = f"{means[key]}/5"
    row[2].text = bar(means[key])

doc.add_heading("Lectura", level=1)
doc.add_paragraph(
    "El contenido aporta valor (Utilidad 3.88), pero la Experiencia (2.12) y la "
    "Confianza (2.25) hunden el score general. La causa son los 5 patrones de "
    "fricción detectados en la auditoría de las 249 pantallas.")

doc.add_heading("Patrones negativos recurrentes", level=1)
patterns = [
    "No hay botón explícito “Guardar/Continuar” tras escribir; solo “Terminar sin escribir” con línea roja que se lee como borrar/advertencia.",
    "Dos CTAs casi idénticos (“Registrar mi experiencia” vs “También puedo guardar esta práctica”).",
    "Áreas de texto sin indicar si son editables, si hay límite de caracteres o cómo se confirma.",
    "Aviso “NO ES UN DIAGNÓSTICO” demasiado pequeño y mal ubicado.",
    "Estado redundante: la casilla dice “Desactivado” aunque ya aparece desmarcada.",
]
for x in patterns:
    doc.add_paragraph(x, style="List Bullet")

doc.add_heading("Plan de mejora", level=1)
plan = [
    "A/B test de un botón primario “Guardar/Continuar” vs “Terminar sin escribir” (medir completación).",
    "Diferenciar visual y textualmente los CTAs de “registrar” vs “guardar práctica”.",
    "Rediseñar el aviso “NO ES UN DIAGNÓSTICO” (tamaño + ubicación) para que dé confianza.",
    "Eliminar estados redundantes en formularios (casilla “Desactivado” + ya desmarcada).",
    "Añadir indicadores de progreso y confirmación en áreas de texto.",
    "Diseñar el paywall/pricing (hoy solo 1 pantalla menciona precio; Gabor-Granger sugiere US$5.99–7.99/mes).",
]
for x in plan:
    doc.add_paragraph(x, style="List Bullet")

doc.add_heading("Nota metodológica", level=1)
doc.add_paragraph(
    "Este análisis es una simulación sintética (8 personas artificiales puntuando la UI "
    "desde el resumen de auditoría). El sentimiento real requiere reseñas de usuarios, "
    "encuestas o datos de uso; no es una medición validada.")

out = os.path.join(SENT, "Sentimiento-UI-Mantralia.docx")
doc.save(out)
print("DOCX:", out)
