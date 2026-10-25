# Herramientas

La auditoría UX soportada está en Rust: `market-swarm ux audit`. Estos scripts
no son el motor. Se quedan donde hacen algo que el binario no hace.

| Script | Para qué sirve todavía |
|---|---|
| `orchestrator.py` | Bucle headless: `ux audit`, export PNG opcional con el CLI de pen.dev, y aplicar un prompt. `--dry-run` no llama a la red. |
| `pen_extract.py` | Parte un `.pen` grande en un archivo por pantalla para exportar PNG. No analiza. |
| `run_all_screens.py`, `retry_failed.py`, `fix_screens.py`, `finalize_audit.py` | Lote histórico de visión/OCR sobre las 249 pantallas de Mantralia. Hace falta `pen` y `OPENROUTER_API_KEY`. |
| `sentiment_ui.py` | Lanza el swarm de mercado (sentimiento, producto, recomendaciones) sobre evidencia de visión ya importada. |
| `sentiment_ai_humans.py` | Panel sintético que llama a un modelo por su cuenta. No es el panel de `ux audit` y no son usuarios reales. |
| `gabor_granger.py`, `gabor_countries.py` | Estudio de precio sintético, aparte de la auditoría de interfaz. |
| `gen_*_docx.py`, `gen_correcciones_md.py` | Pasan informes ya generados a Markdown o Word. |

`describe` y `POST /vision` siguen en el binario para OCR de capturas. Esa
lectura es sintética (`synthetic_fixture=true`) y no sustituye el contraste
medido sobre el `.pen`.
