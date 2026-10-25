# Auditoría UX/UI

`market-swarm ux audit` lee un diseño pen.dev (`.pen`, JSON) y produce un
informe para cambiar pantallas. No hace falta exportar 249 PNG ni un modelo
para medir contraste, inventario, CTAs ni transparencia de precio.

Los doce módulos de investigación (`run`) no cambian. Esta auditoría es otro
comando, con sus propios especialistas.

## Qué mide sin modelo

| Especialista | Qué decide | Tipo de hallazgo |
|---|---|---|
| `visual_design` | Paleta resuelta (variables y tema) y contraste WCAG 2.x sobre rellenos sólidos | Observación. El ratio es un cálculo, no una impresión |
| `information_architecture` | Pantallas, nombres repetidos, etiquetas que se repiten, pasos sin regreso | Observación, o inferencia si dice que algo «puede ser navegación» |
| `cta_copy` | CTAs por verbo o por nombre de control, duplicados cercanos, salida como única acción | Observación |
| `accessibility` | Texto bajo 12 px, controles bajo 44×44 px, iconos sin rótulo, avisos pequeños | Observación |
| `onboarding_paywall` | Precio de pago, pantallas de paywall, onboarding largo | Observación del archivo. «Gratis» no cuenta como precio |
| `competitive_benchmark` | Cruza el diseño con evidencia ya importada (`fetch` / `search` + lectura) | Inferencia citada. Sin evidencia, no inventa un ranking |
| `synthetic_panel` | Cinco personas regla (Sofía, Carlos, Miguel, Raúl, Valentina) | Simulación. No son usuarios reales |
| `critic` | Recalcula los ratios y comprueba citas y etiquetas | Observación sobre el propio informe |
| `synthesis` | Backlog impacto × esfuerzo | El cambio recomendado, con la evidencia del hallazgo |

Bandas: **hacer ahora** (impacto ≥ 4 y esfuerzo ≤ 2), **planificar** (impacto alto
y más esfuerzo), **pulir**, **diferir**. Las opiniones sintéticas no entran en
ese backlog.

## Cómo correrlo

```sh
cargo build --locked
cargo run --locked -- ux specialists
cargo run --locked -- ux audit examples/ux.meditation.pen \
  --out output/ux \
  --as-of 2026-10-25T00:00:00Z \
  --category meditation \
  --import
```

- `--out` escribe `report.json` y `report.txt`. El JSON también sale por stdout.
- `--as-of` es el corte de la evidencia importada (`observed_at` y
  `collected_at`). `--category` se guarda como segmento. Para reimportar sin
  chocar hay que repetir el mismo archivo, el mismo `--as-of` y la misma
  categoría: el id ya existe y el cuerpo no cambió. Otra categoría es otro
  registro y el import se niega a sobrescribirlo.
- `--import` guarda el inventario y cada pantalla. `kind` es `simulated` y
  `synthetic_fixture` es `false`: es el archivo leído, no una encuesta ni un pago.
  El panel usa `synthetic_fixture=true` y el proveedor `synthetic_panel`.
- `--evidence-ids id1,id2` cita documentos que ya están en la base. El
  especialista competitivo cita el fragmento; no añade descargas ni estrellas.
- `--screenshots dir` enlaza PNG/JPEG por id o nombre de pantalla y, si puede,
  lee el tamaño en píxeles. No interpreta la imagen. Si el tamaño no es el del
  frame ni un múltiplo 1–4, lo marca.
- `--mode openrouter` (o `local`) añade `ux_critic` y `ux_synthesis` neuronales.
  Si fallan, el informe determinista sigue. El modo por defecto es `offline`.

`POST /ux/audit` hace el mismo cálculo en el servidor local, con el mismo token
que el resto de la API. El cuerpo es
`{"pen": {…}, "question": "…", "category": "…", "as_of": "…", "evidence": []}`.
Archivos grandes van por el CLI: el servicio limita el cuerpo.

## Informe

`report.txt` abre con el aviso del panel y luego los cuatro bloques de cambio.
Cada línea trae pantalla, frame, nodo, enunciado, cambio propuesto, ids de
evidencia y si es observación o inferencia. `report.json` añade el arreglo
`contrast` (todos los pares medidos), `screens`, `palette`, `variables` y los
agentes.

Para seguir con una investigación de mercado, usa los ids importados en un
`run` normal. No mezcles el panel sintético con un estudio de disposición a
pagar: el validador de WTP no trata `simulated` como pago observado, y el
contenido del panel dice que no lo es.

## Límites

- No se ejecutan nodos `script` ni shaders. Las librerías importadas en
  `imports` no se cargan.
- Un degradado, una imagen o un mesh no tienen un ratio. El informe los cuenta
  como no medidos.
- Tema por defecto: el primer valor de cada eje. Un frame puede cambiar el tema
  con `theme`; los hijos heredan.
- Instancias `ref` se expanden, incluidos `descendants`. Un ciclo o un id
  desconocido aborta la auditoría.
- Los componentes con `reusable: true` no son pantallas. Las pantallas son los
  frames (o refs) de primer nivel, y los frames dentro de un grupo de primer nivel.
- CTA, rol (onboarding, paywall, inicio…) y navegación son heurísticos.
- Una ausencia de precio vale para el texto de este archivo, no para la app
  publicada.
- `describe` / `POST /vision` siguen disponibles para OCR. Esa salida es
  sintética y no pisa el contraste medido.

## Python

`tools/README.md` dice qué script sigue haciendo falta. El orquestador llama a
`ux audit` y solo exporta visión o lanza el swarm de mercado si se lo pides:

```sh
python3 tools/orchestrator.py \
  --backend pen --design diseno.pen \
  --question "¿Qué se cambia primero?" \
  --audit-mode offline --dry-run
```
