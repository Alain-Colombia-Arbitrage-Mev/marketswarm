#!/usr/bin/env python3
"""Generate CORRECCIONES_MANTRALIA.md — the UX fix backlog from the audit."""
import json
import os
from datetime import datetime, timezone

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
report = json.load(open(os.path.join(BASE, "output", "audit", "report.json"),
                        encoding="utf-8"))
stats = report["stats"]
screens = report["screens"]

fixed_ids = {os.path.basename(f).split("-")[0]
             for f in __import__("glob").glob(os.path.join(BASE, "output", "fixed", "*-v2.pen"))}

def tier(c):
    if c is None:
        return 0
    if c <= 6:
        return 0
    if c == 7:
        return 1
    if c == 8:
        return 2
    return 3

TIERS = [
    ("🔴 Críticas (claridad 5–6)", lambda c: c is not None and c <= 6),
    ("🟠 Altas (claridad 7)", lambda c: c == 7),
    ("🟡 Medias (claridad 8)", lambda c: c == 8),
    ("🟢 Menores (claridad 9–10, con issues)", lambda c: c is not None and c >= 9),
]

with_issues = [s for s in screens if s.get("issues")]

lines = []
lines.append("# Mantralia — Correcciones de UX")
lines.append("")
now = datetime.now(timezone.utc).strftime("%Y-%m-%d")
lines.append(f"*Generado {now} a partir de la auditoría de {stats.get('total_screens')} pantallas "
             f"(pen.dev + market-swarm + visión/OCR Nemotron VL).*")
lines.append("")
lines.append("## Resumen")
lines.append("")
lines.append(f"- **Pantallas analizadas:** {stats.get('total_screens')}")
lines.append(f"- **Claridad promedio:** {stats.get('avg_clarity')} / 10")
lines.append(f"- **Con issues:** {stats.get('with_issues')}")
lines.append(f"- **Con CTA claro:** {stats.get('with_ctas')}")
lines.append(f"- **Con mención de precio/plan:** {stats.get('with_price')} ⚠️")
lines.append("")
lines.append("## Hallazgo transversal: falta monetización")
lines.append("")
lines.append("Solo **1 de 249** pantallas muestra precio/plan (`Configuración / Mi plan`, que dice "
             "“Plan gratuito”). No hay paywall, ni precios, ni planes de pago visibles. "
             "Antes de corregir pantallas individuales, conviene **diseñar el pricing** "
             "(sugerido por el estudio Gabor-Granger sintético: **US$5.99–7.99/mes**; "
             "por país: MX$99, R$39.90, US$9.99 como precio más alto pagable).")
lines.append("")
lines.append("## Sentimiento de la UI (patrones negativos recurrentes)")
lines.append("")
lines.append("Sin sentimiento real de usuarios (la app no está lanzada), el swarm derivó estos "
             "5 patrones de experiencia negativa de la auditoría de las 249 pantallas:")
lines.append("")
lines.append("1. **No hay botón explícito “Guardar/Continuar”** tras escribir; solo "
             "“Terminar sin escribir” con una línea roja que se lee como borrar/advertencia.")
lines.append("2. **Dos CTAs casi idénticos** (“Registrar mi experiencia” vs "
             "“También puedo guardar esta práctica”).")
lines.append("3. **Área de texto sin indicar** si es editable, si hay límite de caracteres "
             "o cómo se confirma la entrada.")
lines.append("4. **Aviso “NO ES UN DIAGNÓSTICO” demasiado pequeño** y mal ubicado "
             "(debería dar confianza, no fricción).")
lines.append("5. **Estado redundante**: la casilla dice “Desactivado” aunque ya aparece desmarcada.")
lines.append("")
lines.append("**Recomendaciones:** A/B test de un botón primario “Guardar/Continuar” vs "
             "“Terminar sin escribir”; diferenciar visual y textualmente los CTAs de registrar vs "
             "guardar; rediseñar el aviso de no-diagnóstico (tamaño + ubicación).")
lines.append("")
lines.append("---")
lines.append("")

for title, pred in TIERS:
    group = [s for s in with_issues if pred(s.get("clarity_0_10"))]
    group.sort(key=lambda s: (s["clarity_0_10"] is None, s["clarity_0_10"] or 0))
    if not group:
        continue
    lines.append(f"## {title} ({len(group)})")
    lines.append("")
    for s in group:
        mark = " ✅ ya corregida" if s["id"] in fixed_ids else ""
        lines.append(f"### {s['name']} — claridad {s['clarity_0_10']}{mark}")
        lines.append("")
        for issue in [i.strip() for i in s["issues"].split("|") if i.strip()]:
            lines.append(f"- {issue}")
        if s.get("ctas"):
            lines.append(f"- *CTAs actuales:* {s['ctas']}")
        if s.get("price_mentions"):
            lines.append(f"- *Precio/plan mencionado:* {s['price_mentions']}")
        lines.append("")

out = os.path.join(BASE, "docs", "CORRECCIONES_MANTRALIA.md")
os.makedirs(os.path.dirname(out), exist_ok=True)
with open(out, "w", encoding="utf-8") as fh:
    fh.write("\n".join(lines))
print("MD:", out)
print("pantallas con issues documentadas:", len(with_issues))
print("ya corregidas:", len(fixed_ids))
