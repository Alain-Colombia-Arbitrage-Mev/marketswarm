# Mantralia — Correcciones de UX

*Generado 2026-10-25 a partir de la auditoría de 250 pantallas (pen.dev + market-swarm + visión/OCR Nemotron VL).*

## Resumen

- **Pantallas analizadas:** 250
- **Claridad promedio:** 8.41 / 10
- **Con issues:** 238
- **Con CTA claro:** 230
- **Con mención de precio/plan:** 1 ⚠️

## Hallazgo transversal: falta monetización

Solo **1 de 249** pantallas muestra precio/plan (`Configuración / Mi plan`, que dice “Plan gratuito”). No hay paywall, ni precios, ni planes de pago visibles. Antes de corregir pantallas individuales, conviene **diseñar el pricing** (sugerido por el estudio Gabor-Granger sintético: **US$5.99–7.99/mes**; por país: MX$99, R$39.90, US$9.99 como precio más alto pagable).

## Sentimiento de la UI (patrones negativos recurrentes)

Sin sentimiento real de usuarios (la app no está lanzada), el swarm derivó estos 5 patrones de experiencia negativa de la auditoría de las 249 pantallas:

1. **No hay botón explícito “Guardar/Continuar”** tras escribir; solo “Terminar sin escribir” con una línea roja que se lee como borrar/advertencia.
2. **Dos CTAs casi idénticos** (“Registrar mi experiencia” vs “También puedo guardar esta práctica”).
3. **Área de texto sin indicar** si es editable, si hay límite de caracteres o cómo se confirma la entrada.
4. **Aviso “NO ES UN DIAGNÓSTICO” demasiado pequeño** y mal ubicado (debería dar confianza, no fricción).
5. **Estado redundante**: la casilla dice “Desactivado” aunque ya aparece desmarcada.

**Recomendaciones:** A/B test de un botón primario “Guardar/Continuar” vs “Terminar sin escribir”; diferenciar visual y textualmente los CTAs de registrar vs guardar; rediseñar el aviso de no-diagnóstico (tamaño + ubicación).

---

## 🔴 Críticas (claridad 5–6) (4)

### Códigos / 03 Contador activo — claridad 5 ✅ ya corregida

- El contador muestra "0 DE 45" sin explicar qué se cuenta ni cómo avanzar.
- "TOCA AL REPETIR" es una instrucción ambigua dentro del círculo sin contexto de qué repetir.
- El botón secundario "Deshacer la última" está ubicado por encima del primario, lo que puede debilitar la jerarquía de conversión.
- No hay indicador de progreso visual claro más allá del número cero.
- El aviso "sin resultados garantizados" puede generar fricción o desconfianza al momento de registrar.
- Falta contexto sobre qué significa el número 888 y en qué consiste la "secuencia de hoy".
- *CTAs actuales:* Deshacer la última; Terminar y registrar

### Astrología / astro-moon — claridad 6 ✅ ya corregida

- La imagen principal es casi totalmente negra y no aporta información visual útil
- El subtítulo "No es el calendario lunar actual" genera confusión al definir el producto por lo que no es
- Las cuatro tarjetas de fases no tienen indicadores visuales claros de ser botones seleccionables
- El CTA principal "Escribir una intención" solo tiene sentido para la fase Nueva, pero se presenta como acción global
- Falta de precio o planes visibles, por lo que no es un paywall claro si el propósito es conversión
- *CTAs actuales:* Escribir una intención

### Horas espejo / 17:17 / Abundancia — claridad 6 ✅ ya corregida

- Área central muestra patrón de tablero de ajedrez en lugar de imagen o video real (contenido faltante)
- Falta etiqueta de texto visible en el botón principal de reproducción
- Jerarquía visual interrumpida por el contenido ausente en el centro
- No se menciona precio ni plan de suscripción en esta pantalla
- *CTAs actuales:* Guardar mi próximo paso; Comenzar

### Meditación / Guía activa · AMOR PROPIO — claridad 6 ✅ ya corregida

- El botón "Notar" está resaltado en morado mientras los demás son oscuros, sin dejar claro si es estado seleccionado o acción primaria.
- La etiqueta "AHORA · NOTAR" es pequeña y ambigua; no explica qué hace "Notar".
- Los tres botones superiores (Notar/Acompañar/Integrar) tienen jerarquía visual similar sin distinguir acción principal.
- El botón de reproducción central sugiere audio, pero la pantalla dice "GUÍA ESCRITA", creando confusión sobre formato.
- "Terminar práctica" está muy visible al final, lo que puede fomentar el abandono antes de completar los 8 minutos.
- Los botones inferiores "Ver la guía" y "Sonido" tienen bajo contraste y compiten con el botón de terminar.
- *CTAs actuales:* Notar; Acompañar; Integrar; Ver la guía; Sonido; Terminar práctica

## 🟠 Altas (claridad 7) (15)

### Astrología / astro-chart — claridad 7 ✅ ya corregida

- La imagen principal muestra un patrón de cuadros (placeholder) en lugar de la carta natal real, interrumpiendo la experiencia visual.
- El aviso "Ejemplo ilustrativo · No corresponde a tus datos" puede generar confusión sobre si la información mostrada es personal o genérica.
- No hay mención de precios, planes o pasos de suscripción visibles en esta pantalla.
- La jerarquía visual se ve afectada por la ausencia del contenido principal (la carta natal).
- *CTAs actuales:* ←; Explorar los planetas

### Acompañamiento / Ánimo bajo / 03 Integrar — claridad 7 ✅ ya corregida

- No hay un botón primario claro de "Guardar" o "Continuar" tras seleccionar una opción; solo aparece "Terminar sin guardar" al final.
- Los tres botones de radio aparecen sin seleccionar por defecto, lo que obliga al usuario a elegir antes de avanzar pero no indica qué pasa después.
- El campo "Escribir a una persona de confianza." no distingue visualmente si es un input editable o texto estático.
- La línea roja divisoria antes de "Terminar sin guardar" sugiere una acción destructiva o de eliminación, lo cual puede generar confusión en un contexto de bienestar.
- Falta retroalimentación visual sobre qué ocurre al pulsar cada opción de radio.
- *CTAs actuales:* Terminar sin guardar

### Terapias respiratorias / 08 Cierre · Cómo estás ahora — claridad 7 ✅ ya corregida

- No hay botón primario de guardar/continuar visible para confirmar la respuesta
- "Salir el registro" tiene redacción gramaticalmente extraña en español (debería ser "Salir del registro" o "Salir")
- El campo opcional muestra la pregunta pero no se ve un área de entrada de texto clara
- El botón seleccionado "Más calmado" no indica qué acción sigue después de elegirlo
- La flecha de regreso podría generar duda sobre si guarda o descarta la respuesta
- *CTAs actuales:* Más calmado; Igual; Más activado; Con molestia; Salir el registro; ←

### Silva / 02 Preparar visualización — claridad 7 ✅ ya corregida

- No hay un botón principal para iniciar la visualización después de definir la situación
- El campo de texto parece estático; no está claro si es editable o solo informativo
- El CTA "Editar mi escena" está aislado al final sin una acción primaria de "comenzar"
- La línea roja sobre el CTA inferior puede interpretarse como error o división no intencional
- *CTAs actuales:* ←; Editar mi escena

### Horas espejo / 07:07 / Enfoque — claridad 7 ✅ ya corregida

- El gran área central con patrón de tablero de ajedrez parece una imagen faltante o placeholder roto, lo que genera confusión visual.
- No queda claro si "07:07" es la hora actual o el temporizador, ni la relación con "03:00".
- El texto inferior tiene bajo contraste y tamaño pequeño, dificultando su lectura rápida.
- Falta una explicación inmediata de qué significa "HORAS ESPEJO" para un usuario nuevo.
- *CTAs actuales:* Guardar mi próximo paso; Comenzar; reproducir (botón central)

### Referencia / Splash técnico — claridad 7 ✅ ya corregida

- No hay botón ni llamada a la acción visible.
- El mensaje "Preparando tus prácticas." no incluye indicador de progreso ni tiempo estimado de carga.
- Falta información adicional sobre beneficios, funcionalidades o pasos siguientes para el usuario nuevo.

### Tarot / 02 Formular pregunta — claridad 7 ✅ ya corregida

- No hay botón principal de continuación o avance después de seleccionar la pregunta
- No se indica si la función "Una carta para reflexionar" es gratuita o de pago
- El único CTA visible es "Cambiar mi pregunta", lo que deja sin acción clara para continuar
- La tarjeta seleccionada tiene check pero no explica qué sigue ni cuál es el siguiente paso
- *CTAs actuales:* Cambiar mi pregunta

### Peticiones / Estado · Campos por completar — claridad 7 ✅ ya corregida

- No hay botón visible de enviar, guardar o continuar para completar el formulario
- Tres campos de texto vacíos pueden generar fricción o abandono por alto esfuerzo
- La etiqueta "Opcional" está integrada en el título del segundo campo sin distinción visual clara
- No hay contadores de caracteres ni indicadores de validación en los campos de texto
- El único elemento accionable inferior es un enlace de retroceso, lo que deja sin ruta clara de conversión
- *CTAs actuales:* Volver a elegir intención

### Horas espejo / mirror-day — claridad 7 ✅ ya corregida

- Las 4 tarjetas muestran patrones de tablero de ajedrez en lugar de imágenes reales, lo que sugiere contenido faltante, error de carga o diseño incompleto
- No hay indicador visual claro de que las tarjetas sean seleccionables o navegables (no hay flechas, estados activos ni bordes resaltados)
- El CTA "Ver las 24 horas del día" implica que esta pantalla es solo una vista previa limitada (Día 1) y no la experiencia completa
- Falta contexto inicial para usuarios nuevos sobre qué significa "HORAS ESPEJO" y cuál es el propósito de elegir una hora
- *CTAs actuales:* Ver las 24 horas del día

### Perfil / Perfil — claridad 7 ✅ ya corregida

- El botón "Editar mi perfil" tiene bajo contraste sobre el fondo oscuro de la tarjeta.
- No hay un CTA principal claro para iniciar una práctica o acción clave.
- Las estadísticas (sesiones, minutos, favoritos) carecen de contexto o metas asociadas.
- La barra inferior muestra "Perfil" activo aunque el usuario ya está en esa pantalla.
- No se ve ninguna opción para cerrar sesión.
- La jerarquía visual compite entre el encabezado grande y la tarjeta de perfil oscura.
- *CTAs actuales:* Editar mi perfil; Mi diario; Mi plan; Configuración

### Referencia / Reproductor · Variante anterior — claridad 7

- El tiempo restante muestra '-14:42' con signo negativo, lo cual es confuso
- El encabezado 'DEJA DE SER TÚ' no está claramente vinculado a la sesión actual
- Falta un CTA claro para iniciar o finalizar la meditación más allá del botón de pausa
- La barra de progreso tiene bajo contraste sobre el fondo gris
- El botón 'Guardar sesión' durante la reproducción puede generar duda sobre si detendrá la meditación
- *CTAs actuales:* Ajustar sonido; Guardar sesión; Observar sin cambiar; Pausa; Retroceder 15 s; Avanzar 15 s

### Meditación / Guía activa · ENFOQUE — claridad 7

- "AHORA · ELEGIR" parece un estado sin acción clara asociada
- Los botones "Volver" y "Continuar" tienen estilo secundario o desactivado sin contexto que explique por qué
- "Continuar" puede confundir si la práctica aún no ha iniciado
- "Terminar práctica" está muy expuesto al final y puede tocarse por accidente
- No hay información de precio, plan o suscripción visible
- La jerarquía visual entre el botón primario "Elegir" y las acciones secundarias no está del todo resuelta
- *CTAs actuales:* Elegir; Volver; Continuar; Ver la guía; Sonido; Terminar práctica

### Bienestar / Resultado · Ánimo bajo — claridad 7

- La casilla indica "Opcional · Desactivado", lo que genera ambigüedad sobre si la función está deshabilitada o solo no seleccionada.
- No hay un CTA primario claro para avanzar tras ver los resultados.
- El enlace inferior "También quiero una pausa" es vago y no explica qué ocurre al tocarlo.
- No se menciona precio ni plan, por lo que la ruta de conversión no es evidente desde esta pantalla.
- *CTAs actuales:* Guardar esta orientación; También quiero una pausa

### Respiración / 02 Preparar ejercicio — claridad 7

- Falta un CTA principal visible para iniciar la sesión de respiración
- El enlace inferior "Usar respiración sin conteo" tiene bajo contraste visual y puede pasar desapercibido
- No está claro si la tarjeta central con la imagen de montañas es un botón interactivo o solo información informativa
- *CTAs actuales:* Usar respiración sin conteo

### Acompañamiento / Sobrecarga / 01 Preparar — claridad 7

- Falta un botón principal de inicio o continuar destacado
- La acción siguiente no es evidente por la jerarquía visual
- "Cambiar forma de practicar" aparece como enlace discreto al final, no como CTA de conversión
- Las tres zonas se listan sin indicador de selección activa ni progreso
- *CTAs actuales:* Cambiar forma de practicar

## 🟡 Medias (claridad 8) (97)

### Procrastinación / 03 El primer paso — claridad 8

- "Cambiar" carece de contexto (¿qué se cambia exactamente?).
- No hay un botón primario claro para iniciar o confirmar el primer paso.
- La línea roja inferior no tiene etiqueta ni propósito evidente.
- Las sugerencias muestran "5 min" sin indicar si son estimaciones, temporizadores o estados.
- No hay indicador visual de selección ni progreso entre las sugerencias.
- *CTAs actuales:* Cambiar; Escribir solo el índice; Reunir los datos del trimestre

### Bienestar / 07 Carga percibida — claridad 8

- No hay botón visible de "Siguiente" o "Continuar" tras seleccionar una opción
- No se indica el progreso (ej. pregunta X de Y)
- Todos los botones de radio están desmarcados por defecto, sin selección predeterminada
- La línea roja inferior puede confundirse con un estado de error o campo activo
- El enlace "Omitir esta pregunta" está aislado al final y podría pasarse desapercibido
- *CTAs actuales:* Omitir esta pregunta; Nada; Un poco; Bastante; Mucho

### Configuración / Editar perfil — claridad 8

- El botón "Cancelar" tiene menor jerarquía visual que el primario, lo cual es correcto, pero podría pasar desapercibido.
- No hay indicador de caracteres restantes ni límite visible en el campo de intención personal.
- No se explica qué sucede tras pulsar "Guardar perfil" (¿redirección, confirmación, recarga?).
- El campo opcional carece de ejemplos visuales adicionales más allá del placeholder.
- *CTAs actuales:* Guardar perfil; Cancelar

### Bienestar / PHQ-4 · Pregunta 3 — claridad 8

- No hay botón visible de "Siguiente" o "Continuar" tras seleccionar una opción; no está claro si avanza automáticamente.
- La tarjeta del síntoma se ve como etiqueta estática y no como estado seleccionado, lo que puede generar confusión.
- El enlace "Omitir esta pregunta" está al final y podría pasarse desapercibido o fomentar respuestas incompletas.
- La línea roja divisoria bajo el texto de pie puede interpretarse erróneamente como error o campo obligatorio.
- *CTAs actuales:* Para nada; Varios días; Más de la mitad de los días; Casi todos los días; Omitir esta pregunta

### Prácticas / Método Silva · Azul — claridad 8

- La sección "Sigue explorando" tiene un subrayado rojo pero no muestra contenido debajo, lo que parece una sección vacía o incompleta
- No hay un botón CTA claro para iniciar la práctica; solo la tarjeta funciona como entrada
- La línea roja bajo "Sigue explorando" puede interpretarse como error de diseño o elemento roto
- No está claro si "PRÁCTICA INICIAL" es la única práctica disponible o si hay más por descubrir
- *CTAs actuales:* VISUALIZACIÓN GUIADA; ¿Qué harías con más confianza?; Inicio; Prácticas; Mi diario; Perfil

### Acompañamiento / Sobrecarga / 03 Integrar — claridad 8

- No hay un botón primario visible para guardar o continuar tras seleccionar una opción, solo "Terminar sin guardar", lo que puede generar confusión sobre cómo avanzar.
- La tarjeta de "Mi siguiente paso" parece un campo de texto de solo lectura o ejemplo, no está claro si es editable.
- Los botones de radio aparecen sin seleccionar y no hay indicación visual del siguiente paso tras elegirlos.
- La línea roja divisoria inferior es visualmente pesada y puede interpretarse erróneamente como zona de eliminación o error.
- *CTAs actuales:* Terminar sin guardar

### Peticiones / 03 Formular petición — claridad 8

- No se visualiza ningún CTA (guardar, continuar, enviar) para avanzar tras completar la petición.
- Las tarjetas de texto parecen de solo lectura; no está claro si son editables o solo resumen.
- La línea roja inferior parece un divisor sin propósito visual claro o un elemento de UI incompleto.
- Falta indicación de progreso o paso actual dentro del flujo.

### Configuración / Recordatorios — claridad 8

- El toggle de 'Horas espejo' está desactivado pero se muestran los chips '11:11' y '22:22' sin dejar claro si están activos o son solo ejemplos.
- No hay botón directo para abrir los ajustes del sistema desde la tarjeta de permiso de notificaciones, solo un enlace informativo.
- El estado del toggle 'Mi práctica diaria' no se relaciona visualmente con la hora '08:00' mostrada debajo, lo que puede generar confusión sobre qué está activo.
- *CTAs actuales:* Guardar recordatorios; Cómo activar los avisos; Hora de la práctica; 11:11; 22:22

### Silva avanzado / 07 Espejo de la mente — claridad 8

- La pestaña "Posibilidad" se ve seleccionada visualmente (relleno morado) pero la tarjeta muestra "Situación presente", lo que genera confusión de estado.
- No hay indicador de progreso ni de cuántos pasos faltan en el ejercicio.
- El área de texto no tiene contador de caracteres ni validación visible.
- El botón "Ver la otra escena" puede resultar confuso si aún no se ha creado una escena alternativa.
- No hay mensaje de confirmación tras guardar ni indicación de qué ocurre después.
- *CTAs actuales:* Presente; Posibilidad; Guardar escena y continuar; Ver la otra escena

### Códigos / 02 Configurar repetición — claridad 8

- El subtítulo menciona "repetir el código" sin aclarar qué es el código, lo cual puede generar confusión.
- Ambas opciones (45 repeticiones y Contador manual) muestran marca de verificación al mismo tiempo, sin dejar claro si son mutuamente excluyentes o complementarias.
- No hay un botón principal visible para iniciar o comenzar la repetición; solo aparece "Editar secuencia" al final.
- El descargo legal/ético está en letra pequeña al final y podría pasar desapercibido.
- *CTAs actuales:* Editar secuencia

### Programa / Recorrido · 4 semanas — claridad 8

- El texto descriptivo se corta al final: "Cuatro semanas" queda incompleto.
- Las semanas 2, 3 y 4 muestran candados sin indicar precio, requisito de desbloqueo ni plan de suscripción.
- "Toca para explorar" está alineado a la derecha sin indicar claramente si aplica a toda la sección o solo a un elemento.
- Existe posible confusión entre "2 de 28 prácticas completadas" y "2 / 7 días" (rachas vs días de la semana).
- *CTAs actuales:* ←; compartir; ▶ Atender al presente; ▶ Aprende a meditar; Toca para explorar; Inicio; Prácticas; Mi diario; Perfil

### Respiración / 04 Resultado — claridad 8

- Falta un botón explícito de "Continuar" o "Enviar" tras seleccionar una opción
- No queda claro si "Repetir otro día" guarda la respuesta seleccionada o solo cierra la pantalla
- La navegación hacia adelante no es obvia después de marcar la experiencia
- *CTAs actuales:* Repetir otro día

### Procrastinación / 02 Qué la hace difícil — claridad 8

- No hay un botón visible de "Continuar" o "Siguiente" tras seleccionar una opción
- La línea roja en la parte inferior parece un artefacto de diseño o elemento incompleto
- La primera opción aparece preseleccionada sin indicación clara de cómo avanzar al siguiente paso
- Falta retroalimentación visual sobre qué ocurre después de elegir una tarjeta

### Programa / Semana 3 · Azul — claridad 8

- El indicador '0 de 7 prácticas' no especifica si el cero representa prácticas completadas o pendientes, lo que puede generar confusión.
- No hay un botón principal o CTA destacado para iniciar la primera práctica; el usuario debe tocar cada ítem de la lista individualmente.
- Una línea roja delgada aparece al final de la lista sin propósito visual claro, puede parecer un error de diseño o artefacto.
- *CTAs actuales:* Inicio; Prácticas; Mi diario; Perfil

### Astrología / astro-unknown — claridad 8

- El placeholder de cuadros oscuros ocupa mucho espacio visual sin mostrar contenido real.
- Solo hay un CTA principal para usuarios sin hora exacta; no hay opción visible para quienes sí conocen su hora de nacimiento.
- La jerarquía entre las dos tarjetas blancas es similar, lo que puede diluir el foco en la acción principal.
- *CTAs actuales:* Explorar sin hora exacta; ←

### Terapias respiratorias / Estado · Primera visita — claridad 8

- Línea roja en la parte inferior que parece un error de renderizado o artefacto visual
- No hay un botón CTA principal destacado; las tarjetas son las únicas opciones interactivas
- Falta información de precios o planes si la app es de pago
- El disclaimer médico está en texto pequeño al final y podría pasar desapercibido
- La primera tarjeta está resaltada visualmente sin que quede claro si es por selección del usuario o por defecto
- *CTAs actuales:* Hacer una pausa; Preparar mi descanso; Atender al presente

### Welcome / pensantes — claridad 8

- "Omitir" está pequeño y poco visible en la esquina superior derecha
- "Explorar sin cuenta" es muy discreto y puede pasar desapercibido
- No se muestra información de precios ni planes, lo que genera incertidumbre si hay contenido de pago
- El fondo a cuadros puede resultar visualmente cargado aunque no impide la lectura
- *CTAs actuales:* Buscar una meditación; Explorar sin cuenta; Omitir

### Horas espejo / 06 Recordatorios elegidos — claridad 8

- Ambos avisos inician desactivados sin un CTA principal de confirmar o guardar
- El enlace "Seguir sin avisos" al final puede confundirse como la acción principal en lugar de una opción de saltar
- La línea divisoria roja rompe visualmente con el tono suave del resto de la interfaz
- No hay indicación visual de cuál de los dos avisos es prioritario o recomendado
- *CTAs actuales:* Toca para activar; Seguir sin avisos

### Silva avanzado / 05 Cuenta 3 · 2 · 1 — claridad 8

- El gran rectángulo blanco central está vacío, lo que genera ambigüedad sobre si es un reproductor de video, una imagen que no carga o un espacio reservado.
- No se indica el número actual ni el progreso dentro del ejercicio de conteo, a pesar de que el botón dice "Siguiente número".
- Falta una indicación visual de cuánto dura el ejercicio o cuántos pasos quedan.
- *CTAs actuales:* Siguiente número; Salir del ejercicio

### Programa / Semana 4 · Azul — claridad 8

- No hay un botón principal de inicio claro; los CTAs están dispersos como filas individuales
- El indicador "0 de 7 prácticas" puede confundir si todas las prácticas ya están disponibles
- Falta una recomendación visual de cuál práctica hacer hoy primero
- La línea roja inferior no tiene un propósito visual claro
- *CTAs actuales:* Elegir una intención; Visualizar una situación; Ensayar una respuesta; Sentir y describir; Dar un paso pequeño; Repetir lo que ayuda; Cerrar y seguir; Prácticas

### Terapias respiratorias / 02 Detalle · Coherencia 5 · 5 — claridad 8

- No hay botón principal visible para iniciar la sesión de respiración (falta CTA de conversión)
- Línea roja en la parte inferior indica posible contenido truncado o diseño incompleto
- No se muestra estado del temporizador ni indicador de progreso activo

### Deja de ser tú / 07 Creencias y posibilidades — claridad 8

- Los botones superiores "Presente" y "Posibilidad" carecen de contexto sobre su función exacta o relación con el campo de texto
- El placeholder "Una frase que limite mis opciones" puede confundirse con contenido ya escrito en lugar de instrucción
- No hay contador de caracteres ni indicación de longitud requerida para la respuesta
- Los dos botones inferiores tienen el mismo peso visual, sin distinguir acción primaria de secundaria
- "Ver la otra escena" es ambiguo: no está claro si avanza, retrocede o salta el ejercicio actual
- *CTAs actuales:* Presente; Posibilidad; Guardar escena y continuar; Ver la otra escena

### Códigos / 04 Integración — claridad 8

- No hay un botón explícito de "Guardar" o "Continuar"; la progresión depende de seleccionar una opción de radiobutton sin indicación de avance automático.
- El enlace "Terminar sin escribir" está aislado al final con una línea roja, lo que puede percibirse como una vía de escape más que una acción secundaria clara.
- Las dos opciones principales tienen subtítulos muy similares sobre guardar/registrar, lo que puede generar confusión sobre la diferencia real entre ambas.
- El área de texto muestra contenido ya escrito sin indicar si es editable, si hay límite de caracteres o cómo se confirma la entrada.
- *CTAs actuales:* Registrar mi experiencia; También puedo guardar esta práctica; Terminar sin escribir

### Deja de ser tú / 06 Elige otra respuesta — claridad 8

- No se indica si los campos de texto son obligatorios o opcionales
- No hay contador de caracteres, límite de longitud ni validación visible
- Los dos inputs tienen el mismo estilo visual aunque solicitan tipos de respuesta distintos (patrón vs conducta)
- El botón "Continuar" no muestra estado activo/inactivo ni retroalimentación de validación
- La acción secundaria "Volver a la biblioteca" tiene peso visual similar a la acción principal
- No queda claro qué sucede tras pulsar Continuar ni si se guarda el progreso automáticamente
- *CTAs actuales:* Continuar; Volver a la biblioteca

### Horas espejo / Mandalas 08–11 — claridad 8

- Las tarjetas muestran patrones de cuadros, lo que indica imágenes faltantes o placeholders rotos.
- No hay indicador visual claro de que las tarjetas de hora sean botones táctiles.
- Falta un indicador de progreso a pesar de decir "Día 1".
- El CTA principal (seleccionar una hora) no está etiquetado como acción explícita; el botón inferior es secundario pero visualmente dominante.
- *CTAs actuales:* Ver las 24 horas del día

### Welcome / permiso — claridad 8

- No se muestra precio ni plan de suscripción en esta pantalla
- El enlace "Explorar sin cuenta" tiene baja jerarquía visual y puede pasar desapercibido
- "Omitir" es pequeño y está en esquina, con bajo contraste
- No queda claro qué ocurre tras pulsar el botón principal ni si requiere registro inmediato
- *CTAs actuales:* Explorar mi prosperidad; Explorar sin cuenta; Omitir

### Deja de ser tú / 08 Cultiva una emoción — claridad 8

- El botón "Duración" no indica si es informativo o editable, lo que genera ambigüedad.
- No hay indicador visual de progreso de la pista entre 00:00 y 03:00.
- Falta contexto de precio o plan (no aplica si es contenido gratuito, pero no se aclara).
- *CTAs actuales:* Desplegar menú; Más opciones; Lista/reproducción; Anterior; Reproducir; Siguiente; Repetir; Guía; Duración; Voz guía; Reproducir siguiente

### Silva / 03 Visualización activa — claridad 8

- Los tres botones "Situar", "Imaginar" y "Concretar" tienen jerarquía visual similar pero solo uno está activo, lo que puede confundir si son pasos secuenciales o modos alternativos.
- El botón grande de reproducción (play) sugiere audio, pero el contenido se etiqueta como "GUÍA ESCRITA", creando ambigüedad sobre el formato.
- "AHORA · SITUAR" mezcla estado y acción sin claridad visual.
- "Terminar práctica" está al fondo y podría tocarse por error al navegar.
- No hay indicación de precio ni planes, por lo que no es una pantalla de conversión de pago.
- *CTAs actuales:* Situar; Imaginar; Concretar; Ver la guía; Sonido; Terminar práctica; X

### Horas espejo / 22:22 / Gratitud — claridad 8

- El cuadrado central con patrón de tablero de ajedrez parece un asset faltante o placeholder sin propósito claro, lo que confunde visualmente.
- El texto "Comenzar · Guía y contemplación" es pequeño y ambiguo: no está claro si es un botón clickeable o solo una etiqueta.
- No hay información de precio ni plan visible, por lo que no se puede evaluar conversión de pago.
- El botón "Guardar mi próximo paso" está en la parte inferior con bajo contraste jerárquico respecto al botón de reproducción central.
- *CTAs actuales:* X; Comenzar; Guardar mi próximo paso; ▶ (reproducir); 🔊 (volumen); 📖 (libro)

### Horas espejo / 00:00 / Descanso — claridad 8

- El patrón de ajedrez en el centro parece una imagen faltante o placeholder sin propósito claro.
- Hay dos temporizadores (00:00 y 03:00) sin etiquetar cuál es tiempo transcurrido vs. tiempo total, lo que genera ambigüedad.
- El texto "Comenzar · Guía y contemplación" es pequeño y no está claro si es un botón clickeable o solo una etiqueta.
- No hay indicación visual de qué ocurre al presionar el botón de reproducción morado.
- La jerarquía entre "00:00" y "03:00" no es inmediatamente obvia en cinco segundos.
- *CTAs actuales:* Guardar mi próximo paso; Comenzar · Guía y contemplación; X

### Horas espejo / 12:12 / Pausa — claridad 8

- El patrón de tablero de ajedrez en el centro es ambiguo: puede parecer un error de carga o imagen faltante en lugar de un espejo intencional.
- La instrucción 'Haz una pausa' aparece dos veces (como encabezado y en el cuerpo), lo que genera redundancia.
- El botón 'X' de cierre es pequeño y poco contrastado.
- No hay indicación clara de qué representa el cuadrado central ni si el usuario debe interactuar con él.
- *CTAs actuales:* Guardar mi próximo paso; reproducir (play); cerrar (X)

### Horas espejo / Mandalas 16–19 — claridad 8

- Las tarjetas muestran patrones de cuadros (placeholders) en lugar de imágenes reales, lo que puede parecer contenido roto o sin cargar.
- No hay indicador visual claro de que las tarjetas de hora sean botones táctiles (falta affordance de interacción).
- El único CTA explícito está al final; la selección de una hora específica no está etiquetada como acción principal.
- *CTAs actuales:* Ver las 24 horas del día; 16:16; 17:17; 18:18; 19:19

### Horas espejo / Mandalas 04–07 — claridad 8

- Los patrones de ajedrez en las tarjetas parecen placeholders de imagen faltantes en lugar de diseño intencional
- No hay indicador visual claro de qué tarjeta está seleccionada
- El término "HORAS ESPEJO" puede no ser inmediatamente claro para usuarios nuevos
- No se muestra información de precio o plan si aplica
- La jerarquía entre las cuatro tarjetas y el botón inferior no está del todo definida
- *CTAs actuales:* Ver las 24 horas del día; 04:04; 05:05; 06:06; 07:07

### Horas espejo / Estado · Avisos bloqueados — claridad 8

- No hay botón directo para abrir Ajustes y habilitar notificaciones, solo se menciona el texto "Ajustes".
- El único CTA prominente es "Seguir sin avisos", lo que desincentiva activar los permisos.
- Las tarjetas de recordatorio (11:11 y 22:22) son informativas pero no tienen acción clara asociada.
- Falta un botón primario claro para resolver el problema de los avisos bloqueados.
- *CTAs actuales:* Seguir sin avisos; ←

### Práctica / five-minute — claridad 8

- El área central con patrón de ajedrez es un placeholder sin indicar qué contenido se mostrará (video, audio o animación).
- No se ven controles de reproducción (pausa, adelante, atrás) ni indicador de progreso durante la sesión.
- No hay alternativa visible al usuario si no desea comenzar la pausa inmediatamente.
- *CTAs actuales:* Comenzar mi pausa

### Horas espejo / Mandalas 12–15 — claridad 8

- Las tarjetas muestran imágenes de marcador de posición (tablero de ajedrez) en lugar de contenido real, lo que da aspecto de pantalla incompleta o rota.
- No hay indicador visual de qué tarjeta está seleccionada ni botón de "Continuar" tras elegir una hora.
- El botón de retroceso en el Día 1 de un onboarding puede generar confusión sobre si se puede salir del flujo.
- La jerarquía entre elegir una tarjeta y el botón inferior no está del todo clara: no se sabe si ambas acciones son necesarias o alternativas.
- *CTAs actuales:* Ver las 24 horas del día; 12:12; 13:13; 14:14; 15:15

### Silva avanzado / 09 Un gesto para volver — claridad 8

- El área del gesto muestra un patrón de tablero vacío sin indicación de si es una vista de cámara, ilustración o estado de carga, lo que genera ambigüedad.
- El contador "0 de 3" no explica qué ocurre tras completar los tres ensayos ni si es obligatorio.
- El botón secundario "Continuar sin el gesto" permite saltarse la funcionalidad principal, lo que puede reducir la adopción del gesto.
- No hay retroalimentación visual ni instrucción sobre cómo realizar el gesto dentro del área vacía.
- *CTAs actuales:* Registrar este ensayo; Continuar sin el gesto

### Configuración / Eliminar cuenta — claridad 8

- El botón primario (morado) corresponde a la acción segura "Conservar mi cuenta", mientras que la acción destructiva "Solicitar eliminación" es secundaria y visualmente menos prominente, lo que invierte la jerarquía esperada en flujos de eliminación.
- El campo de texto ya muestra "ELIMINAR" escrito; no está claro si es valor prellenado, placeholder o texto del usuario.
- No se menciona precio ni plan, aunque se advierte que la suscripción se cancela por separado desde la tienda, lo que puede generar dudas sobre reembolsos o acceso restante.
- *CTAs actuales:* Conservar mi cuenta; Solicitar eliminación; Atrás

### Silva / 04 Llevar a la acción — claridad 8

- No hay un botón principal visible para guardar o continuar después de escribir la acción; solo aparece "Terminar sin escribir" al fondo, lo que puede confundir la acción principal.
- Las dos tarjetas iniciales tienen propósitos similares pero no está claro si son mutuamente excluyentes o complementarias.
- El campo de texto no indica límite de caracteres ni formato esperado.
- El enlace "Terminar sin escribir" está visualmente separado por una línea roja, lo que puede llamar más la atención que el flujo de guardado.
- *CTAs actuales:* Registrar mi experiencia; También puedo guardar esta práctica; Terminar sin escribir

### Afirmaciones / Estado · Frase vacía — claridad 8

- No se ve un botón principal de guardar o enviar la frase escrita
- El área de texto está vacía sin indicador de caracteres ni placeholder visible dentro del cuadro
- Dos CTAs idénticos ("Usar esta frase como punto de partida") pueden generar duda sobre cuál elegir
- El texto de ayuda en rojo debajo del campo vacío podría pasar desapercibido
- No hay retroalimentación visual clara sobre si se debe escribir primero o seleccionar una sugerencia
- *CTAs actuales:* Volver a las frases; Usar esta frase como punto de partida

### Configuración / Mi plan — claridad 8

- No se muestran precios ni nombres de planes de pago en esta pantalla, solo se invita a "Consultar opciones".
- La tarjeta del plan gratuito ocupa mucho espacio visual sin ofrecer ruta directa de upgrade.
- El texto legal sobre gestión en la tienda puede generar fricción si el usuario espera cancelar desde la app.
- *CTAs actuales:* Consultar opciones; Restaurar compras; Volver a mi perfil

### Astrología / astro-transits — claridad 8

- El gran rectángulo oscuro con patrón de ajedrez parece una imagen faltante o placeholder sin carga, lo que genera confusión visual.
- No hay indicación clara de si el contenido es gratuito o de pago.
- Las dos tarjetas informativas no tienen botones o acciones asociadas visibles.
- El único CTA principal está al final y su propósito (guardar en diario) no está vinculado explícitamente a una lectura generada.
- *CTAs actuales:* Llevarlo a mi diario; ←

### Silva avanzado / 06 Tu pantalla mental — claridad 8

- No se indica qué ocurre tras pulsar "Continuar" ni si hay límite de caracteres en los campos
- El encabezado "SILVA · ADAPTACIÓN" no explica su propósito sin contexto adicional
- No hay indicador de progreso ni borrador automático visible
- Dos campos de texto abiertos pueden generar fricción sin una propuesta de valor clara
- *CTAs actuales:* Continuar; Volver a la biblioteca

### TDAH / 07 Cerrar bucles abiertos — claridad 8

- El CTA inferior "Revisarlo mañana" es ambiguo: no queda claro si aplaza todas las tareas, cierra la pantalla o pospone la revisión.
- Falta un botón primario explícito para guardar o finalizar el cierre del día tras marcar las tareas.
- La tarjeta de "Lavadora" tiene fondo distinto al estar marcada, lo cual da feedback visual pero no explica qué ocurre con el resto de ítems al salir.
- No hay indicación de consecuencias al marcar o desmarcar ítems (si se mueven, se archivan o se reprograman).
- *CTAs actuales:* Revisarlo mañana

### Silva avanzado / 16 Mapa del método Silva — claridad 8

- No se indican precios ni si las formaciones son gratuitas o de pago.
- Los botones de formación no explican su contenido ni diferencia entre sí sin hacer clic.
- El texto de pie menciona fuentes originales pero no incluye enlaces visibles.
- La flecha de retroceso y el icono de información son pequeños y podrían pasar desapercibidos.
- *CTAs actuales:* Técnicas y fundamentos; Life System · formación original; Intuition System · formación original; Volver a las prácticas

### Sesión / 02 Reproductor inmersivo · Azul — claridad 8

- El tiempo restante muestra -14:42 (valor negativo), lo cual es confuso y probablemente un error de UI
- La jerarquía visual combina el título de la sesión con los controles del reproductor sin una separación visual clara
- El botón "Registrar cómo me siento" está al final de la pantalla y puede pasar desapercibido
- No hay indicación clara del progreso general del programa más allá del contador 01/03
- *CTAs actuales:* Ajustar sonido; Guardar sesión; Registrar cómo me siento; Pausa; Retroceder 15 s; Avanzar 15 s

### Deja de ser tú / 16 Mapa de las enseñanzas — claridad 8

- No se muestran precios ni planes de pago aunque parece ser una pantalla de selección de contenidos de pago
- Los términos "acceso oficial" e "información oficial" son vagos y no explican qué recibe el usuario exactamente
- El descargo sobre la colección Advanced (retiro oficial, respiración intensa, audios no incluidos) puede generar confusión o fricción en la conversión
- Los tres botones principales tienen el mismo estilo visual, sin distinguir claramente cuál es la acción recomendada o primaria
- *CTAs actuales:* Programa del libro · cuatro semanas; Colección avanzada · acceso oficial; Caleidoscopio · información oficial; Volver a las prácticas

### Procrastinación / 10 Mi patrón de postergación — claridad 8

- No hay un CTA principal claro para actuar sobre el insight (solo "ver detalle").
- El consejo sobre los martes no tiene botón asociado para aplicar la sugerencia.
- Los días S y D aparecen desactivados sin explicación visual.
- La métrica "4 min hasta empezar" carece de contexto comparativo.
- No se menciona precio ni plan, por lo que no es una pantalla de conversión de pago.
- *CTAs actuales:* Ver el detalle por tarea

### Mi diario / Historial · Azul — claridad 8

- "3 momentos para ti" muestra solo 3 días marcados de 7 sin explicar qué significan los círculos vacíos frente a los marcados con tilde
- "40 min" en la tarjeta semanal no especifica si es tiempo total, promedio o de la sesión actual
- La transición "Inquieta → En calma" no tiene suficiente contexto sobre qué representa
- No hay un CTA principal claro para iniciar una nueva meditación o crear una entrada
- El título de pantalla "TU RECORRIDO" difiere de la pestaña activa "Mi diario" en la navegación inferior
- El punto de tiempo a la izquierda de la entrada de hoy parece parte de una línea de tiempo, pero solo se muestra un punto sin continuidad visual clara
- *CTAs actuales:* Ver todo; Inquieta; En calma; Inicio; Prácticas; Mi diario; Perfil

### Astrología / astro-profile — claridad 8

- El texto "Ejemplo editable" junto a la fecha puede confundir al usuario sobre si el campo es editable o solo ilustrativo.
- No hay un botón explícito de "Continuar" o "Siguiente" para enviar el formulario; solo aparece el CTA de ejemplo al final.
- La opción "No sé mi hora de nacimiento" se presenta como tarjeta pero no está claro si es seleccionable o solo informativa.
- El aviso de privacidad y el límite del prototipo están en letra pequeña y pueden pasar desapercibidos.
- *CTAs actuales:* Explorar carta de ejemplo

### Horas espejo / 11:11 / Intención — claridad 8

- El gran cuadrado con patrón de ajedrez sugiere una imagen faltante o estado de carga roto, lo que genera confusión visual.
- El texto "Comenzar · Guía y contemplación" es pequeño y su interactividad no está clara.
- No hay información de precio o plan visible, por lo que no se puede evaluar conversión de pago.
- El temporizador "03:00" y el encabezado "3 MIN" son redundantes.
- *CTAs actuales:* Guardar mi próximo paso; Comenzar; X

### Silva avanzado / 03 Una situación concreta — claridad 8

- No se indica si los campos son obligatorios o opcionales
- No hay indicador de progreso (paso X de Y)
- El texto de ejemplo dentro de los campos podría confundirse con valores ya precargados
- No se muestra estado de validación ni errores si los campos quedan vacíos
- No queda claro qué ocurre al pulsar "Continuar" ni si el botón está desactivado hasta completar los campos
- *CTAs actuales:* Continuar; Volver a la biblioteca

### Terapias respiratorias / 09 Mi registro respiratorio — claridad 8

- "5 · 5 tu ritmo" no explica qué miden los dos números ni su relación
- No hay un botón de acción principal destacado (solo un enlace de texto al final)
- La flecha de retroceso y el encabezado son visualmente pequeños
- No hay botón claro para iniciar una nueva sesión desde esta pantalla
- *CTAs actuales:* Ver el mes completo

### Procrastinación / 09 Plan para la próxima vez — claridad 8

- No hay un botón principal visible para continuar o guardar el plan.
- El texto "Solo el recordatorio" separado por una línea roja es ambiguo: puede interpretarse como opción para omitir o como acción destructiva.
- Los dos enlaces "Cambiar" no especifican qué se edita (hora, lugar o ambos).
- La tarjeta "SI PASA ESTO, HAGO ESTO" no deja claro si es editable o solo un ejemplo informativo.
- *CTAs actuales:* Cambiar; Cambiar; Solo el recordatorio

### Afirmaciones / 05 Llevar a la acción — claridad 8

- No hay botón visible para guardar, continuar o avanzar después de escribir la acción.
- La única acción destacada al final es "Terminar sin escribir", lo que puede desincentivar la finalización del flujo.
- La casilla "Guardar en mis favoritas" ya aparece marcada, sin que quede claro si es predeterminada o seleccionada por el usuario.
- La línea roja decorativa sobre "Terminar sin escribir" genera confusión visual al parecer un error o división no intencional.
- *CTAs actuales:* Terminar sin escribir; Guardar en mis favoritas

### TDAH / 08 Registro del día — claridad 8

- No hay botón visible de guardar o continuar para enviar el registro
- No queda claro si se puede seleccionar más de una opción en "¿Qué pasó hoy?"
- El enlace "Saltar el registro" está muy visible al final y podría causar abandono accidental
- Falta indicador de progreso o paso del flujo
- La línea divisoria roja rompe la armonía visual del tema lavanda
- *CTAs actuales:* Saltar el registro; Me costó arrancar; Me dispersé; El reloj ayudó; Buen día

### Horas espejo / mirror-month — claridad 8

- El texto promete "24 horas espejo" pero la grilla muestra 30 botones numerados, lo que puede confundir al usuario sobre el alcance real del contenido.
- Solo el botón "1" está resaltado; no queda claro si los demás días están disponibles, bloqueados o son solo decorativos.
- Falta indicación de progreso o estado (completado/bloqueado) para los días no seleccionados.
- *CTAs actuales:* Explorar el día 1

### Inicio / Categorías de prácticas guiadas — claridad 8

- Las cuatro tarjetas de categoría muestran imágenes de marcador de posición (damero) en lugar de ilustraciones reales, lo que debilita el atractivo visual y la identificación rápida.
- El enlace "Ver todas" está alineado a la derecha y podría pasar desapercibido si el usuario escanea de izquierda a derecha.
- No se menciona ningún precio o plan de suscripción en esta pantalla, por lo que no es posible evaluar la conversión de pago desde aquí.
- *CTAs actuales:* Ver todas; Elegir; Elegir cómo me siento →; Inicio; Prácticas; Mi diario; Perfil

### Horas espejo / 05 Integrar el mandala — claridad 8

- El gran área con patrón de tablero indica una imagen o recurso faltante, lo que genera confusión visual.
- No hay un botón principal claro para guardar o continuar el flujo principal; solo aparece un enlace secundario al final.
- La línea roja divisoria carece de contexto visual claro y puede parecer un error de diseño.
- La jerarquía entre la acción principal y el enlace inferior no está resuelta, lo que puede afectar la conversión.
- *CTAs actuales:* Elegir un recordatorio

### Horas espejo / Mandalas 20–23 — claridad 8

- Las zonas superiores de las tarjetas muestran patrones de tablero de ajedrez, lo que indica imágenes faltantes o placeholders no cargados.
- No hay indicador visual claro de que las tarjetas sean botones táctiles (falta affordance de interacción).
- No hay indicador de progreso del onboarding a pesar de decir "Día 1".
- No se muestra precio ni plan, por lo que no es posible evaluar si es pantalla de conversión de pago.
- *CTAs actuales:* Ver las 24 horas del día

### Terapias respiratorias / 06 Sesión · 4 · 7 · 8 — claridad 8

- Redundancia entre el número "7" del círculo central y el texto "Retención actual · 7 s" de la tarjeta.
- Falta un botón principal claro para avanzar al siguiente ciclo o continuar el ejercicio.
- El enlace inferior "Practicar sin retención" puede generar confusión sobre si se debe omitir la retención actual.
- "EN CURSO" en la parte superior es pequeño y su contexto no es inmediato.
- *CTAs actuales:* ←; Cambiar a una guía sin retención; Practicar sin retención

### Welcome / universo — claridad 8

- "Omitir" es ambiguo: no deja claro qué se salta el usuario
- No hay mención de precios ni planes si esta pantalla fuera un paywall
- El enlace "Explorar sin cuenta" tiene baja jerarquía visual respecto al botón principal
- El patrón de fondo puede restar legibilidad en pantallas pequeñas
- *CTAs actuales:* Escribir mi petición; Explorar sin cuenta; Omitir

### Onboarding / 04 Resultado deseado — claridad 8

- Flujo de 6 pasos sin indicación de cuánto falta realmente más allá del contador numérico
- No se menciona precio, plan ni qué ocurre tras pulsar Continuar (¿suscripción? ¿contenido gratuito?)
- El estado seleccionado de la primera tarjeta es sutil; las demás tarjetas blancas se ven muy similares y podrían confundir
- Botón de retroceso en un flujo de 6 pasos puede hacer perder progreso sin aviso
- Falta un CTA secundario como "Omitir" o "Más tarde" para usuarios que no quieran elegir ahora
- *CTAs actuales:* Continuar →

### Deja de ser tú / 04 Cuerpo y espacio — claridad 8

- Placeholder de patrón de ajedrez en lugar de imagen real del contenido
- Botones secundarios "Guía", "Duración" y "Voz guía" con función poco evidente sin contexto adicional
- Botón de reproducción pequeño en la tarjeta siguiente puede competir visualmente con el botón principal
- Ausencia de información de precio o plan (no es un paywall visible)
- *CTAs actuales:* Reproducir; Guía; Duración; Voz guía; Reproducir siguiente

### Onboarding / 01 Bienvenida — claridad 8

- El indicador "1 DE 6" sugiere un flujo de onboarding largo que podría generar abandono.
- El patrón de tablero detrás de la tarjeta central puede confundirse con un estado de carga o placeholder.
- No hay botón de "saltar" visible, aunque el pie aclara que la personalización es opcional.
- El enlace "Puedes explorar directamente" en el pie compite visualmente con el CTA principal.
- *CTAs actuales:* Personalizar mis sugerencias; explorar directamente

### Tarot / 04 Registrar reflexión — claridad 8

- No hay un botón claro de "Guardar" o "Continuar" visible; solo aparece "Terminar sin escribir" al final.
- El campo de texto ya contiene texto escrito, lo que puede confundir al usuario sobre si es un placeholder o contenido real.
- Las dos tarjetas superiores parecen opciones mutuamente excluyentes, pero no queda claro qué acción se dispara tras elegirlas.
- El CTA "Terminar sin escribir" es contradictorio si el usuario ya escribió algo en el campo de texto.
- *CTAs actuales:* Terminar sin escribir

### Deja de ser tú / 12 Mañana y noche — claridad 8

- No queda claro si los campos de texto son editables o solo son ejemplos/placeholders
- No hay indicador de progreso ni de cuántos pasos tiene el flujo de onboarding
- El botón "Continuar" no muestra estado de validación (habilitado/deshabilitado) si los campos están vacíos
- La navegación superior "DEJA DE SER TÚ · ADAPTACIÓN" es pequeña y podría pasar desapercibida
- No se explica qué sucede al pulsar "Volver a la biblioteca" ni si se pierden los datos ingresados
- *CTAs actuales:* Continuar; Volver a la biblioteca; ←; (i)

### Inicio / Todas las categorías — claridad 8

- El subtítulo del encabezado está cortado a la derecha ("...algo que desec")
- No hay un botón principal de acción explícito (solo selección de categorías)
- Existe posible duplicación o confusión entre "Respiración" y "Respiración guiada"
- No se muestra información de precios, planes ni estado de suscripción
- La jerarquía visual es plana: todas las categorías tienen el mismo peso visual excepto la seleccionada
- *CTAs actuales:* X; Meditación; Control de estrés; Peticiones al universo; Frecuencias; Método Silva; Códigos sagrados; Tarot; Horas espejo; Observador; Afirmaciones; Respiración; Astrología; Deja de ser tú; Hábitos; Procrastinación; Respiración guiada; Bienestar emocional; Atención y TDAH

### Astrología / astro-home — claridad 8

- Las pestañas inferiores tienen muy bajo contraste (texto gris claro sobre fondo lavanda), lo que dificulta su legibilidad y su estado activo/inactivo.
- El banner central muestra un patrón de tablero de ajedrez que parece un placeholder de imagen o estado de carga, no un contenido real.
- No hay un botón de acción principal destacado; todas las tarjetas tienen el mismo peso visual, lo que diluye la jerarquía de conversión.
- No se menciona ningún precio, plan de suscripción o prueba gratuita, por lo que el modelo de negocio no es claro desde esta pantalla.
- *CTAs actuales:* Descubre tu carta natal.; Mi carta natal; Signos y planetas; Luna y ciclos; Vínculos y diario; Planetas; Tránsitos; Mi diario; Guía

### Bienestar / Apoyo · Preparar una consulta — claridad 8

- No hay un botón principal claro para iniciar el proceso (ej. "Comenzar evaluación" o "Solicitar apoyo").
- Los pasos son informativos pero no conducen a una acción obvia siguiente.
- El enlace "Necesito ayuda urgente" está separado por una línea roja pero se presenta como texto plano, no como botón destacado.
- El selector de país está al final y podría pasar desapercibido.
- No se menciona precio ni plan, lo cual puede generar incertidumbre si el usuario espera saber costos.
- *CTAs actuales:* Cambiar país; Necesito ayuda urgente

### Acompañamiento / Preocupación / 02 Práctica — claridad 8

- La imagen de fondo es muy oscura y borrosa, lo que dificulta distinguir colores reales
- Los círculos numerados 1, 2, 3 no muestran progreso visual (todos vacíos)
- El CTA "He encontrado uno" usa singular aunque la meta es encontrar tres colores
- No queda claro qué ocurre tras pulsar "He encontrado uno" (¿se suma al contador? ¿se valida el color?)
- El texto "Sin prisa" es pequeño y puede restar claridad sobre si hay límite de tiempo
- La jerarquía entre las pílulas superiores y el ejercicio principal no es del todo evidente
- *CTAs actuales:* He encontrado uno; Pausar o elegir otro apoyo; X (cerrar)

### Prácticas / Práctica del observador · Azul — claridad 8

- "Sigue explorando" parece un enlace sin destino claro ni indicación visual de interacción más allá del subrayado rojo.
- No hay CTA principal visible para iniciar la práctica de 8 minutos (por ejemplo, "Empezar" o "Reproducir").
- No se muestra información de precio, plan o suscripción, por lo que no es posible evaluar elementos de conversión de pago.
- El contexto de navegación es limitado: solo se ve "CONTEMPLACIÓN" sin indicador de progreso ni breadcrumb claro.
- *CTAs actuales:* Sigue explorando; Inicio; Prácticas; Mi diario; Perfil

### Prácticas / Códigos sagrados · Azul — claridad 8

- La sección "Sigue explorando" tiene un subrayado rojo pero no muestra contenido debajo, lo que genera expectativa vacía.
- No hay un botón CTA visible para iniciar el ritual de 6 minutos dentro de la tarjeta principal.
- El texto principal funciona más como titular que como botón accionable, reduciendo la claridad de conversión.

### Peticiones / 01 Estado presente — claridad 8

- La insignia "3" en el botón es redundante con el texto "3 rituales de intención"
- No se menciona precio, plan ni opción de suscripción visible
- El texto descriptivo dentro de las tarjetas es pequeño y de bajo contraste
- La etiqueta "A tu ritmo" se superpone sobre la imagen de trigo
- No hay un CTA principal único destacado más allá de los botones superiores
- *CTAs actuales:* Peticiones; 3 rituales de intención; Inicio; Prácticas; Mi diario; Perfil

### Silva avanzado / 01 José Silva — claridad 8

- El texto del pie de página es muy pequeño y de bajo contraste
- No se indica si el contenido es gratuito o de pago
- El uso de voseo ("Ensayá") puede resultar ajeno para usuarios de otras variantes del español
- No hay indicador de progreso ni estado (completado/bloqueado) en las tarjetas
- *CTAs actuales:* Prepara tu visualización; Imagina cómo actuar; Ensayá el primer paso; Explora una solución; Todas las prácticas

### Afirmaciones / 04 Repetición activa — claridad 8

- El contador dice "0 de 5 repeticiones" pero el botón "1" está resaltado, lo que genera inconsistencia visual
- No queda claro qué ocurre al completar las 5 repeticiones
- El botón de cierre (X) es pequeño y puede pasar desapercibido
- No hay información de precio o plan visible en esta pantalla
- *CTAs actuales:* X; Ya la repetí; Deshacer; Editar frase

### Acompañamiento / Preocupación / 01 Preparar — claridad 8

- No hay un botón claro para iniciar la práctica principal; el único CTA visible es secundario ("Cambiar forma de practicar").
- El texto de la tarjeta central se superpone a una imagen oscura con detalles de telaraña, lo que puede reducir ligeramente la legibilidad.
- La jerarquía visual enfatiza el título pero no indica claramente qué acción debe tomar el usuario para comenzar.
- *CTAs actuales:* Cambiar forma de practicar; ←

### Deja de ser tú / 13 Regresa con calma — claridad 8

- El título "Termina la práctica despacio" puede leerse como instrucción al usuario en lugar de nombre de pantalla
- Los tres botones inferiores tienen el mismo peso visual, lo que diluye la jerarquía entre continuar y terminar
- No queda claro qué ocurre tras pulsar "Terminar práctica" (¿cierra la app, vuelve al inicio, muestra resumen?)
- La flecha de retroceso en la parte superior puede generar confusión durante una práctica activa
- *CTAs actuales:* Escuchar este paso; Continuar · paso 2; Terminar práctica

### Bienestar / Resultado · Sin señales elevadas — claridad 8

- La casilla aparece como "Desactivado", lo que puede generar confusión sobre si se puede guardar la orientación.
- No hay un CTA principal único tras los resultados; compiten varias acciones (guardar, práctica, hablar con alguien, salir).
- El aviso "NO ES UN DIAGNÓSTICO" es visualmente secundario respecto a la pregunta principal.
- La tarjeta de práctica con fondo oscuro puede sentirse como contenido insertado o publicidad entre el resultado y las acciones finales.
- *CTAs actuales:* Guardar esta orientación; Quiero hablar con alguien; Elige lo que necesitas hoy

### Observador / 03 Práctica activa — claridad 8

- El texto "AHORA · ESCUCHAR" puede leerse como estado o como acción, generando leve ambigüedad.
- Los dos iconos de actualizar a ambos lados del botón de play parecen redundantes y no está claro su función distinta.
- El botón "Terminar práctica" está al final y podría pulsarse por error al navegar.
- No hay indicación visual de cuánto audio queda más allá de la barra numérica pequeña.
- *CTAs actuales:* Escuchar; Notar; Volver; Ver la guía; Sonido; Terminar práctica

### Acceso / 02 Crea tu cuenta — claridad 8

- El texto superpuesto sobre la imagen de fondo tiene bajo contraste y es difícil de leer
- No se indican requisitos de contraseña ni campo de confirmación
- Los términos y la privacidad no se distinguen claramente como enlaces clickeables
- El botón "Explorar sin cuenta" tiene menor jerarquía visual que el CTA principal
- *CTAs actuales:* Crear mi cuenta; Iniciar sesión; Explorar sin cuenta

### Acompañamiento / Sobrecarga / 02 Práctica — claridad 8

- El temporizador 00:45 no indica si cuenta tiempo transcurrido o restante, ni su propósito exacto.
- Tres botones finales tienen tamaño visual similar, lo que diluye la jerarquía del CTA principal "Comenzar descanso".
- El texto "Puedes omitir esta zona" es pequeño y puede pasar desapercibido.
- La imagen de la hoja es decorativa pero no comunica claramente la acción de descansar las manos.
- No hay indicación de qué ocurre tras presionar "Comenzar descanso" ni cuánto durará el descanso.
- *CTAs actuales:* X; Manos; Hombros; Mandíbula; Omitir esta zona; Comenzar descanso; Pausar o elegir otro apoyo

### Acompañamiento / Enfoque / 02 Práctica — claridad 8

- El botón 'Me distraje · vuelvo' aparece antes de iniciar el temporizador, lo cual es confuso en el flujo de usuario.
- No queda claro si el '01:00' es una cuenta regresiva o solo una meta de tiempo.
- La instrucción pide mirar 'un detalle' pero la imagen no resalta ningún detalle específico.
- No hay indicación visual de qué ocurre al finalizar el minuto.
- La jerarquía de botones es ambigua: el CTA principal está en medio de opciones secundarias sin un camino lineal claro.
- *CTAs actuales:* X; Me distraje · vuelvo; Comenzar un minuto; Pausar o elegir otro apoyo

### Acompañamiento / Ánimo bajo / 01 Preparar — claridad 8

- No hay un CTA principal con botón destacado, solo un enlace de texto al final.
- La jerarquía visual prioriza la imagen decorativa sobre el contenido accionable.
- No se muestra precio ni plan, por lo que no hay contexto de conversión o suscripción.
- La flecha de retroceso en la parte superior puede generar confusión sobre si el usuario puede avanzar o solo regresar.
- *CTAs actuales:* Cambiar forma de practicar

### Silva avanzado / 04 Relajación progresiva — claridad 8

- No hay indicación de precio, plan o si el contenido es gratuito o de pago.
- El botón de flecha hacia abajo superior izquierda no tiene etiqueta y su función no es evidente (¿colapsar?, ¿menú?).
- El icono de aleatorio/repetir a la derecha puede ser ambiguo sin tooltip.
- La sección "SIGUIENTE EN TU RECORRIDO" compite visualmente con la sesión actual en curso.
- *CTAs actuales:* Guía; Duración; Voz guía; Play (reproducción central); Play (siguiente contenido)

### Deja de ser tú / 01 Deja de ser tú — claridad 8

- Los botones de reproducción no tienen etiqueta de texto accesible, solo ícono
- No hay indicador de progreso ni estado de completado de las tarjetas
- El texto del pie de página es pequeño y de bajo contraste
- No se menciona precio, plan ni suscripción en esta pantalla
- Las cuatro tarjetas tienen estilo visual muy similar, lo que puede dificultar distinguir su propósito a primera vista
- *CTAs actuales:* ←; i; Todas las prácticas; botones de reproducción (▶) en cada tarjeta

### Bienestar / Resultado · Sobrecarga — claridad 8

- La casilla dice "Desactivado" aunque está desmarcada, lo que genera ambigüedad sobre si la función está bloqueada o solo no seleccionada.
- El único CTA de conversión es un enlace de texto al final, sin botón primario destacado.
- El aviso "NO ES UN DIAGNÓSTICO" tiene jerarquía visual menor que el título principal, aunque es información legal importante.
- La superposición de texto sobre la imagen del bosque puede tener contraste variable según el dispositivo.
- *CTAs actuales:* Quiero buscar apoyo; Guardar esta orientación; ←

### Horas espejo / mirror-entry — claridad 8

- El botón del medio "Mi reflexión" no tiene flecha direccional como los laterales, lo que puede generar duda sobre su función de navegación.
- No se explica qué significa "HORAS ESPEJO" para un usuario nuevo.
- La imagen central es decorativa pero su propósito no está explicitado.
- El texto de pie de página "Voz dirigida · Intención y acción" es pequeño y de bajo contraste, fácil de pasar por alto.
- No hay indicador de progreso visual más allá del texto "Día 1".
- *CTAs actuales:* ← Hora; Mi reflexión; Hora →; Escuchar afirmación; flecha de retroceso superior

### Frecuencias / 03 Escucha activa — claridad 8

- El botón "Permanecer" es ambiguo: no deja claro qué se permanece ni qué sucede al tocarlo.
- El estado del reproductor no es evidente: el temporizador marca 00:00 y hay un botón de play grande, pero no se indica si el audio ya está sonando o está listo para iniciar.
- "AHORA · ESCUCHAR" aparece como etiqueta pequeña y podría confundirse con un botón o estado sin interacción clara.
- No hay información de precio, plan ni indicación de si el contenido es gratuito o de pago.
- "Terminar práctica" está al final y puede pasar desapercibido o tocarse por error al navegar.
- *CTAs actuales:* Escuchar; Permanecer; Cerrar; Guía de escucha; Sonido; Terminar práctica

### Prácticas / Tarot · Azul — claridad 8

- La sección "Sigue explorando" tiene subrayado rojo pero no muestra contenido ni opciones debajo, lo que genera expectativa vacía.
- No hay un botón CTA claro para iniciar la lectura de 5 minutos a pesar de que se menciona el tiempo y el formato.
- El subrayado rojo bajo "Sigue explorando" puede interpretarse como estado activo o error en lugar de encabezado de sección.
- La navegación inferior marca "Prácticas" como activa, pero la pantalla parece un detalle de práctica específica en lugar de un listado, lo que puede confundir al usuario sobre su ubicación en la app.
- *CTAs actuales:* Sigue explorando; Inicio; Prácticas; Mi diario; Perfil

### Mi diario / Estado · Primera visita — claridad 8

- No hay un botón de acción principal visible (como "Escribir nota" o "+") para iniciar la conversión.
- La imagen sugiere escribir una frase, pero no hay campo de texto ni botón evidente para hacerlo.
- La línea roja sobre la navegación inferior no tiene propósito visual claro.
- La jerarquía explica el beneficio, pero el camino hacia la acción principal es ambiguo.
- *CTAs actuales:* Inicio; Prácticas; Mi diario; Perfil

### Deja de ser tú / 05 Observar sin seguir — claridad 8

- El botón "Volver al foco" tiene un ícono de mira que podría no ser inmediatamente comprensible para todos los usuarios.
- La sección "SIGUIENTE EN TU RECORRIDO" compite visualmente con la práctica actual y podría distraer antes de terminar el ejercicio actual.
- No hay indicación clara de progreso dentro de la práctica más allá del temporizador de audio.
- No se muestra precio ni plan, por lo que no aplica a evaluación de conversión de pago en esta pantalla.
- *CTAs actuales:* Reproducir; Guía; Duración; Volver al foco; Prepara otra respuesta

### Deja de ser tú / 11 Lleva la intención contigo — claridad 8

- No se muestra precio ni plan de suscripción en esta pantalla
- El menú de tres puntos superior derecho carece de etiqueta visible
- La sección "SIGUIENTE EN TU RECORRIDO" puede competir visualmente con el CTA principal de reproducción
- El botón "Duración" muestra una etiqueta pero no el valor numérico real de la duración
- No hay indicación clara de si el contenido es gratuito o requiere pago
- *CTAs actuales:* Guía; Duración; Voz guía; Cierra la práctica; Reproducir (play central)

### Meditación / 05 Guía activa · Calma — claridad 8

- "AHORA · LLEGAR" es ambiguo: no está claro si es estado, etiqueta o botón accionable.
- Los tres botones segmentados (Llegar/Observar/Integrar) carecen de contexto sobre su función exacta.
- No hay información de precio, plan o estado de suscripción visible en la pantalla.
- El botón de cierre (X) es pequeño y podría pasar desapercibido.
- La barra de progreso está en 0:00 sin indicador de carga o estado de audio.
- *CTAs actuales:* Llegar; Observar; Integrar; Ver la guía; Sonido; Terminar práctica; X

### Bienestar / Resultado · Ansiedad — claridad 8

- El disclaimer 'NO ES UN DIAGNÓSTICO' es pequeño y puede pasar desapercibido
- La casilla indica 'Desactivado' en lugar de un estado visual claro de marcado/desmarcado
- 'También quiero una pausa' no tiene formato de botón claro; su función es ambigua
- No hay un CTA principal destacado para continuar tras ver los resultados
- El texto sobre la imagen de fondo ('3 colores · 2 sonidos · 1 punto de apoyo') tiene bajo contraste
- *CTAs actuales:* Guardar esta orientación; También quiero una pausa

### Bienestar / Resultado · Experiencia mixta — claridad 8

- El aviso "NO ES UN DIAGNÓSTICO" está en tamaño pequeño en la parte superior
- La palabra "cribados" puede ser jerga técnica poco clara para algunos usuarios
- La casilla indica "Desactivado" aunque ya aparece desmarcada, lo cual es redundante
- No hay un botón principal destacado al final, solo un enlace de texto plano
- La imagen de cueva oscura puede resultar visualmente pesada en un contexto de bienestar emocional
- *CTAs actuales:* Guardar esta orientación; Elegir una pausa

### Bienestar / Estado · Respuestas incompletas — claridad 8

- Hay dos CTAs con texto muy similar ("Elegir una práctica por mi cuenta" dentro de una tarjeta y "Elegir una práctica" al final), lo que puede generar confusión sobre cuál es la acción principal.
- La tarjeta inferior menciona que se puede salir sin guardar, pero no hay un botón explícito de "salir" visible, solo el CTA final.
- La imagen del bosque no tiene una relación visual obvia con el contexto del cuestionario, lo que puede distraer.
- La jerarquía visual no resalta claramente una acción primaria sobre las demás.
- *CTAs actuales:* Completar lo que falta; Elegir una práctica por mi cuenta; Elegir una práctica

## 🟢 Menores (claridad 9–10, con issues) (122)

### Configuración / Cuenta — claridad 9

- No se muestran precios ni detalles del plan aunque existe la opción "Mi plan y compras"; el usuario debe navegar para ver costos.
- El botón "Eliminar cuenta" está en color rojo al final sin una confirmación visible, lo que podría generar toques accidentales o ansiedad.
- La tarjeta "Tu perfil de Mantralia" no muestra foto ni nombre real del usuario, solo un ícono genérico.
- El encabezado "PERFIL · TU EXPERIENCIA" es pequeño y podría pasar desapercibido.
- *CTAs actuales:* Cerrar sesión; Eliminar cuenta; Datos personales; Mi plan y compras; Privacidad; Tu perfil de Mantralia

### TDAH / Información · Esto no es un diagnóstico — claridad 9

- No hay botón de CTA principal visible, solo un enlace de texto al final
- La línea divisoria roja inferior puede resultar visualmente abrupta
- Falta un botón explícito de cierre o aceptación del descargo
- *CTAs actuales:* Volver a los apoyos

### Observador / 04 Registrar experiencia — claridad 9

- Los textos 'Registrar mi experiencia' y 'También puedo guardar esta práctica' aparecen como subtítulos dentro de las tarjetas de selección, lo que puede confundirse con botones de acción independientes en lugar de descripciones del resultado al elegir esa opción.
- No hay un botón explícito de 'Guardar' o 'Continuar' visible tras escribir la nota; el único cierre visible es 'Terminar sin escribir', lo que genera ambigüedad sobre cómo avanzar si el usuario desea conservar su registro.
- La línea divisoria roja sobre 'Terminar sin escribir' es visualmente pesada y puede interpretarse como una acción de eliminación o advertencia sin contexto adicional.
- El área de texto no muestra límite de caracteres ni indicador visual de final, lo que puede dejar al usuario sin referencia sobre la extensión esperada de la nota.
- *CTAs actuales:* Registrar mi experiencia; También puedo guardar esta práctica; Terminar sin escribir

### Procrastinación / 03 El primer paso — claridad 9

- No hay un botón principal claro para iniciar o completar el primer paso; las tarjetas son informativas pero no tienen estado de acción evidente.
- El enlace "Cambiar" está a la derecha dentro de una pastilla y puede pasar desapercibido.
- Las sugerencias muestran "5 min" pero no está claro si son seleccionables o cómo se inician.
- Una línea roja en la parte inferior parece un artefacto visual o divisor incompleto.
- *CTAs actuales:* Cambiar

### Horas espejo / 03 Escribir mi intención — claridad 9

- No hay un botón principal visible para iniciar o continuar después de escribir la intención.
- El enlace "Cambiar de hora" está aislado al final con una línea roja, lo que puede generar ambigüedad sobre su función.
- La flecha de retroceso es pequeña y podría pasar desapercibida.
- No se indica límite de caracteres ni botón de envío para el campo de texto libre.
- *CTAs actuales:* Cambiar de hora

### Procrastinación / 07 Pausa entre ciclos — claridad 9

- No hay botón para continuar o extender la pausa, solo para terminarla; el temporizador muestra 03:12 mientras el encabezado dice 5 MIN, lo que puede generar duda si cuenta hacia arriba o hacia abajo.
- *CTAs actuales:* Terminar por hoy

### Astrología / astro-signs — claridad 9

- El botón inferior "Explorar los planetas" no está claramente vinculado a la selección de un signo; no queda obvio si es obligatorio elegir un signo antes de continuar.
- No hay indicación visual de precio, plan o suscripción en esta pantalla.
- La tarjeta de Aries está resaltada, pero no hay una etiqueta explícita de "Seleccionado" que confirme el estado al usuario.
- *CTAs actuales:* Explorar los planetas

### Bienestar / Elección · Práctica sin etiqueta — claridad 9

- No hay un botón explícito de "Continuar" o "Siguiente" tras seleccionar una práctica, por lo que no queda claro cómo avanzar.
- La línea roja divisoria antes de "Prefiero buscar apoyo" puede leerse visualmente como un separador de error o acción destructiva, aunque solo separa secciones.
- El enlace final está aislado sin indicar a dónde conduce exactamente.
- No se mencionan precios ni planes (no aplica si no es una pantalla de pago).
- *CTAs actuales:* Un gesto amable para hoy; Una referencia a tu alrededor; Descansar el cuerpo unos minutos; Concentrarme en una tarea; Prefiero buscar apoyo

### Welcome / cuidado — claridad 9

- "Omitir" y "Explorar sin cuenta" tienen bajo contraste visual y tamaño pequeño, lo que puede hacerlos pasar desapercibidos
- No hay indicación visual de qué contenido o práctica se desbloqueará al presionar "Elegir algo para mí"
- La pantalla tiene mucho espacio vacío, lo que puede generar sensación de falta de información adicional
- *CTAs actuales:* Elegir algo para mí; Omitir; Explorar sin cuenta

### Sesión / Ajustes de voz y ambiente — claridad 9

- No hay botón explícito de guardar o confirmar cambios; solo aparece "Cancelar cambios", lo que genera ambigüedad sobre cómo aplicar la selección.
- La flecha de retroceso podría interpretarse como guardar al salir, pero no está claro.
- Los deslizadores de volumen son delgados y pueden dificultar un ajuste preciso.
- *CTAs actuales:* Cancelar cambios

### Respiración / Rama · Ritmo natural — claridad 9

- No hay botón de pausa visible, solo "Terminar esta pausa"
- La flecha de retroceso superior puede generar confusión sobre si sale de la sesión activa
- "Sin conteo" se repite en la cabecera y debajo del temporizador
- El CTA de terminar está ubicado en la parte inferior, lo que podría tocarse por error durante el ejercicio
- *CTAs actuales:* Terminar esta pausa; Ajustar las señales de la guía

### Respiración / Rama · Cierre sin conteo — claridad 9

- No hay un botón explícito de "Continuar" o "Siguiente" tras seleccionar una opción, lo que puede generar incertidumbre sobre cómo avanzar.
- La línea roja al fondo puede interpretarse visualmente como una acción destructiva o de error, lo que choca con el tono calmado de la app.
- La opción "Repetir otro día" está aislada y sin contenedor, lo que reduce su visibilidad como CTA principal.
- *CTAs actuales:* Repetir otro día

### Bienestar / Apoyo · Ayuda inmediata — claridad 9

- El botón "Llamar al 106" está solo al final; no hay botón directo para llamar al 123 de emergencias
- El texto de descargo es pequeño y de bajo contraste, fácil de pasar por alto
- No está claro si "Cambiar ubicación..." es un enlace táctil o solo texto informativo
- La acción principal (llamar al 106) queda separada al final del contenido
- *CTAs actuales:* Llamar al 106; Cambiar ubicación para ver otros recursos
- *Precio/plan mencionado:* Gratuito

### Silva avanzado / 15 Tu práctica, en perspectiva — claridad 9

- No hay precio visible (no aplica, es pantalla inicial)
- El estado vacío con "0" puede sentirse desalentador sin contexto adicional
- El icono de información no muestra contenido al instante, requiere interacción
- No hay indicación clara de qué son las "prácticas" antes de empezar
- *CTAs actuales:* Elegir mi primera práctica; ←; i

### Astrología / astro-learn — claridad 9

- No se indican precios ni planes si el contenido es de pago
- Las tarjetas de temas no tienen CTA individual visible, solo el botón inferior
- No hay indicador de progreso ni contexto de en qué paso del onboarding se encuentra el usuario
- El subtítulo usa "sus signos" (posesivo en tercera persona) lo cual puede resultar ligeramente impersonal
- *CTAs actuales:* Abrir la guía; ←

### Acceso / 04 Revisa tu correo — claridad 9

- El correo mostrado es un placeholder genérico (nombre@correo.com) en lugar del correo real del usuario.
- No hay temporizador ni indicación de cuándo estará disponible el reenvío del correo.
- El enlace "Explorar sin cuenta" tiene menor jerarquía visual y podría pasar desapercibido.
- *CTAs actuales:* Ya confirmé mi correo; Reenviar correo; Explorar sin cuenta

### Configuración / Ayuda — claridad 9

- No hay barra de búsqueda para encontrar ayuda rápidamente
- El enlace "Redactar una consulta" tiene bajo contraste visual respecto a un botón primario
- No se especifica si las tarjetas llevan a artículos, videos o formularios
- No hay datos de contacto (correo o teléfono) visibles
- El botón "Volver a configuración" puede resultar confuso si el usuario solo quiere salir de la ayuda
- *CTAs actuales:* Cómo empezar una práctica; Audio y recordatorios; Mis datos y privacidad; Fuentes de las prácticas; Redactar una consulta; Volver a configuración

### Meditación / 03 Detalle · Calma — claridad 9

- No hay botón visible de inicio/reproducción para comenzar la sesión de audio
- La línea roja inferior parece un artefacto de recorte sin propósito claro
- No se muestra información de precio o plan aunque el contenido pueda ser de pago
- El texto "Puedes pausar" es informativo, no una CTA accionable

### TDAH / Estado · Día de poca energía — claridad 9

- Inconsistencia entre "Dos minutos" en el título y "un minuto" en el subtítulo de la primera opción.
- El enlace inferior "Dejarlo por hoy sin apuntarlo" puede pasar desapercibido o leerse como acción de descarte.
- No hay botón de confirmación explícito; se asume que tocar una tarjeta avanza el flujo.
- La línea roja inferior es un elemento visual inusual sin propósito claro.
- *CTAs actuales:* Dos minutos y paro; Algo mecánico; Parar del todo; Dejarlo por hoy sin apuntarlo; ←

### Configuración / Idioma — claridad 9

- No se ve botón de cancelar ni indicación de qué ocurre si se retrocede sin guardar
- La tarjeta informativa "Voces y contenido" puede pasar desapercibida al ubicarse entre las opciones y el botón de guardar
- El botón "Guardar idioma" está anclado abajo; en pantallas más pequeñas podría requerir desplazamiento
- No hay confirmación visual de guardado ni estado de éxito tras pulsar el botón
- *CTAs actuales:* Guardar idioma; Español; English; Português

### TDAH / 05 El tiempo a la vista — claridad 9

- Inconsistencia leve: el titular dice "doce minutos" pero el temporizador marca 12:40 (12 min 40 seg).
- "Aviso a mitad" es ambiguo: no especifica si es a mitad de la tarea total o del tiempo restante.
- Falta un botón principal visible para pausar o reiniciar el temporizador; solo aparece la opción de terminar antes de tiempo al final.
- La línea divisoria roja es sutil y podría pasar desapercibida como separador visual.
- *CTAs actuales:* Terminar antes de tiempo; ←

### Silva avanzado / 11 Una pregunta al descanso — claridad 9

- El cuadro de ajedrez indica una imagen faltante o sin cargar, lo que genera ambigüedad visual.
- No hay indicador de progreso ni contexto del paso actual dentro de un flujo.
- El botón secundario "Volver a la biblioteca" puede causar pérdida de datos sin confirmación de salida.
- Falta contador de caracteres o guía de longitud esperada en el campo de texto.
- *CTAs actuales:* Guardar mi pregunta; Volver a la biblioteca

### Bienestar / Sesión · Aumentó el malestar — claridad 9

- La casilla junto a "Terminar la práctica" puede generar confusión sobre si es un botón o un interruptor.
- "Guardar solo si tú lo eliges" es ambiguo: no especifica qué se guarda.
- No hay un CTA explícito para reanudar la práctica, solo para terminarla.
- El enlace "Necesito ayuda urgente" es solo texto azul y podría pasar desapercibido en una situación de crisis.
- La línea roja divisoria carece de contexto visual y puede interpretarse como error o separador sin función clara.
- *CTAs actuales:* Abrir los ojos y mirar alrededor; Terminar la práctica; Buscar apoyo; Necesito ayuda urgente

### Onboarding / 06 Plan adaptable — claridad 9

- No hay un CTA explícito de "Comenzar" o "Finalizar onboarding" en la parte superior, solo el indicador "6 DE 6".
- El botón de reproducción dentro de la tarjeta de hoy puede competir visualmente con el botón principal inferior.
- No se muestran precios ni planes, por lo que si esta pantalla tiene intención de conversión de pago, falta información de precios.
- El botón inferior dice "Explorar mis prácticas", lo cual sugiere navegación libre en lugar de comprometerse con el plan mostrado.
- *CTAs actuales:* Explorar mis prácticas; ▶ (reproducir práctica de hoy); ← (volver)

### Respiración / 03B Exhalar — claridad 9

- La línea divisoria roja cerca del fondo resulta visualmente llamativa sin una función obvia.
- La opción de salida "Volver a mi respiración natural" está como enlace de texto bajo una línea, con menor jerarquía que los controles principales.
- "Sin retenciones" puede no ser inmediatamente claro para usuarios nuevos (indica que no hay retención de aliento).
- *CTAs actuales:* Ajustar las señales de la guía; Volver a mi respiración natural

### Tarot / 03 Carta y reflexión — claridad 9

- El patrón de tablero de ajedrez sobre la carta puede interpretarse como imagen faltante o estado de carga.
- No hay CTA principal para guardar, compartir o registrar la reflexión tras leerla.
- La línea roja inferior carece de contexto y puede confundirse con un indicador de error.
- No hay indicador de progreso ni navegación clara hacia otras cartas o pasos siguientes.
- *CTAs actuales:* Volver a mi pregunta

### Frecuencias / 04 Registro — claridad 9

- No hay botón visible para guardar o continuar después de escribir en el campo de notas; solo aparece "Terminar sin escribir" al final, lo que puede generar incertidumbre sobre si el texto se guarda.
- El CTA de finalización está visualmente separado del flujo de escritura por una línea y texto al pie, lo que debilita su conexión con la acción principal.
- Aunque el campo de notas se presenta como opcional, no hay confirmación visual de qué ocurre al omitirlo o al escribir y salir.
- *CTAs actuales:* Terminar sin escribir

### Silva avanzado / 08 Tres escenas — claridad 9

- No se indica si los campos de texto son obligatorios o opcionales
- No hay contador de caracteres ni límite de longitud visible en los campos de entrada
- El botón "Continuar" no muestra estado visual (activo/desactivado) según se completen los campos
- El botón de información (i) no comunica su propósito sin interactuar con él
- No hay indicador de progreso (por ejemplo, "Paso 1 de 3") aunque hay tres secciones numeradas
- *CTAs actuales:* Continuar; Volver a la biblioteca

### Onboarding / 02 Energía — claridad 9

- La flecha de retroceso es pequeña y de bajo contraste, podría pasar desapercibida.
- No hay opción visible para omitir o saltar el onboarding.
- La barra de progreso es delgada y de bajo contraste, lo que reduce la percepción de avance.
- La distinción visual entre la opción seleccionada y las no seleccionadas es sutil (solo cambia el ícono y aparece un check).
- *CTAs actuales:* Continuar

### Astrología / astro-bonds — claridad 9

- No se ven opciones para cambiar los signos seleccionados (Aries/Libra) en esta pantalla
- No hay indicación de costo o si la función es gratuita o de pago
- El botón de retroceso está muy cerca del título de sección, lo que puede generar confusión sobre si "ASTROLOGÍA" es navegable
- *CTAs actuales:* ←; Explorar nuestras preguntas

### Hábitos / 06 Preparar una señal — claridad 9

- El CTA "Ver mi plan" no deja claro si finaliza el onboarding o solo muestra una vista previa
- No hay botón visible para omitir o saltar el paso
- El estado seleccionado de la primera opción es sutil (fondo lavanda muy claro vs blanco)
- *CTAs actuales:* Ver mi plan; Dejar una nota visible; Preparar mi espacio; Pedir apoyo a alguien; Mi propia señal

### Acompañamiento / Enfoque / 03 Integrar — claridad 9

- No hay un botón primario claro de "Guardar" o "Continuar" tras seleccionar una opción; solo aparece "Terminar sin guardar" al final, lo que genera ambigüedad sobre cómo avanzar o guardar los cambios.
- Los tres botones de radio aparecen sin selección marcada, sin indicar cuál está activo o si es obligatorio elegir uno.
- El campo "Mi siguiente paso · Opcional" se presenta como texto dentro de una tarjeta blanca, lo que podría confundirse con una etiqueta estática en lugar de un área de entrada editable.
- *CTAs actuales:* Terminar sin guardar

### Bienestar / 09 Impacto cotidiano — claridad 9

- No hay botón visible de "Siguiente" o "Continuar" tras seleccionar una opción, lo que deja incierta la acción de avance.
- El enlace "Omitir esta pregunta" está al final sin jerarquía visual de acción principal.
- No hay indicador de progreso (cuántas preguntas faltan).
- El ícono superior derecho no tiene etiqueta y su función es ambigua.
- El texto de advertencia es pequeño y puede pasar desapercibido.
- *CTAs actuales:* Omitir esta pregunta

### Sesión / 04 Diario · Azul — claridad 9

- No se ve un botón principal de guardar, continuar o finalizar (problema de conversión).
- El contador de caracteres solo aparece en el primer campo, no en el segundo.
- Dos líneas divisorias rojas al final sin etiqueta ni propósito claro.
- Espacio vacío grande en la parte inferior que puede sugerir contenido faltante o corte de pantalla.
- *CTAs actuales:* ←

### Bienestar / PHQ-4 · Pregunta 2 — claridad 9

- No hay botón visible de "Siguiente" o "Continuar" tras seleccionar una opción, lo que puede generar incertidumbre sobre cómo avanzar.
- El enlace "Omitir esta pregunta" está muy visible al final y subrayado en rojo, lo que puede incentivar el abandono de la pregunta o parecer una acción destructiva.
- Ninguna opción está preseleccionada por defecto.
- *CTAs actuales:* Omitir esta pregunta

### Deja de ser tú / 14 Haz visible tu intención — claridad 9

- El botón "Terminar sin guardar" tiene la misma prominencia visual que "Guardar mi práctica", lo que puede provocar descartes accidentales.
- No hay contador de caracteres ni indicación de longitud máxima en los campos de texto.
- No se muestra confirmación antes de salir sin guardar si se presiona el botón de retroceso superior.
- *CTAs actuales:* Escuchar afirmación; Guardar mi práctica; Terminar sin guardar

### Terapias respiratorias / 01 Biblioteca de terapias — claridad 9

- Línea roja al final sin contexto aparente (posible artefacto de recorte)
- No se menciona precio ni plan de suscripción en esta vista
- El formato "5 + 5" requiere leer la descripción para entender que son segundos de inhalar/exhalar
- *CTAs actuales:* EMPEZAR POR AQUÍ; Respiración tranquila · 5 + 5; Preparar el descanso · 4 · 7 · 8; Respiración cuadrada; Suspiro fisiológico; Nasal alterna

### Terapias respiratorias / 07 Rescate · Suspiro fisiológico — claridad 9

- No hay un botón principal visible para iniciar la respiración guiada (solo se enumeran los pasos).
- El CTA de fondo "Añadirlo a mis accesos rápidos" está separado por una línea roja y compite visualmente con el propósito principal de la pantalla.
- No se muestra temporizador ni indicador visual de ritmo de respiración en esta vista.
- *CTAs actuales:* Añadirlo a mis accesos rápidos

### Hábitos / 02 Identificar el momento — claridad 9

- La opción seleccionada solo se distingue por una tilde y un fondo ligeramente distinto, sin resaltado fuerte.
- El botón "Continuar" está habilitado aunque no queda claro si es obligatorio seleccionar una opción.
- La nota de privacidad al pie puede generar duda o fricción al mencionar que las respuestas no se guardan hasta elegirlas.
- La opción "Otra situación" no muestra campo de texto inmediato, lo que puede requerir un paso adicional no visible.
- *CTAs actuales:* Continuar; Tengo un momento libre; Algo me abruma; Termina mi día; Otra situación

### Deja de ser tú / 02 Prepara tu práctica — claridad 9

- No se indica precio ni si el contenido es gratuito o de pago
- La selección de opciones se distingue solo por un borde blanco sutil, podría ser más evidente
- El botón de información (i) no explica su propósito sin tocarlo
- No hay indicación de qué ocurre tras pulsar "Guardar ajustes y elegir práctica"
- *CTAs actuales:* 1 min; 3 min; 5 min; Guía escrita; Voz latina · Valentina; Guardar ajustes y elegir práctica

### Sesión / Transcripción accesible — claridad 9

- La tarjeta "Seguir el audio" muestra una marca de verificación pero no está claro si es un interruptor activable o solo un estado informativo.
- No hay botón visible de reproducción/pausa de audio a pesar de que el texto menciona pausar la sesión.
- La línea roja inferior parece un divisor sin propósito funcional claro.
- Los controles de tamaño de letra están pequeños y al fondo, podrían pasar desapercibidos.
- *CTAs actuales:* Seguir el audio; A-; A+; Volver al reproductor; ←

### Configuración / Audio y guía — claridad 9

- El interruptor "Leer las instrucciones" está desactivado sin indicador visual de estado activo más allá del texto descriptivo.
- La fila "Escuchar una muestra" combina ícono de reproducción y flecha de navegación, lo que puede generar ligera ambigüedad entre reproducir y abrir detalle.
- El deslizador de volumen no muestra etiquetas de mínimo/máximo, solo el porcentaje actual.
- *CTAs actuales:* Guardar audio; Escuchar una muestra

### Terapias respiratorias / Estado · Molestia durante la práctica — claridad 9

- No hay un botón explícito para reanudar el ejercicio actual tras la pausa; solo aparece "Explorar otras prácticas" al final, lo que puede generar fricción si el usuario se recupera rápido.
- La flecha de retroceso superior no deja claro si sale de la práctica por completo o solo de esta pantalla de ayuda.
- La línea divisoria roja es visualmente abrupta y no tiene una función obvia.
- La caja de alternativas sugiere otras prácticas pero no incluye botones directos para seleccionarlas.
- *CTAs actuales:* Explorar otras prácticas

### Hábitos / 05 Probar otra respuesta — claridad 9

- El texto de privacidad al pie es pequeño y puede pasar desapercibido.
- No hay indicación visual de qué ocurre tras pulsar "Continuar" ni cuántos pasos quedan exactamente más allá del contador.
- La selección actual está marcada, pero no hay retroalimentación explícita al tocar las otras opciones (comportamiento estándar de radio, pero no se ve el estado activo al pulsar).
- *CTAs actuales:* Continuar; Dejar el teléfono un minuto; Mirar por la ventana; Abrir solo lo que necesito; Escribir mi alternativa

### Configuración / Accesibilidad — claridad 9

- La tarjeta "Observa una sensación y continúa a tu ritmo." parece texto instructivo sin una acción asociada clara, lo que puede generar confusión sobre su propósito.
- Ambos interruptores (Reducir movimiento, Más contraste) vienen desactivados por defecto; no hay indicación visual de beneficios si se activan.
- No hay mensaje de confirmación ni retroalimentación tras presionar "Guardar accesibilidad" (esperable en captura estática, pero relevante para UX).
- La ruta superior "PERFIL · TU EXPERIENCIA" es genérica y no especifica exactamente en qué sección del perfil se encuentra el usuario.
- *CTAs actuales:* Guardar accesibilidad; Estándar; Grande

### TDAH / 01 Apoyos para la atención — claridad 9

- No hay un botón de acción principal explícito; la navegación depende de tocar las tarjetas
- La primera tarjeta tiene un fondo distinto sin indicador claro de selección o estado activo
- El descargo médico está en texto pequeño y puede pasar desapercibido
- Falta jerarquía visual entre el enlace inferior y las tarjetas principales
- *CTAs actuales:* Me voy a otra cosa sin darme cuenta; Empezar me cuesta más que hacerlo; Pierdo la noción del tiempo; Qué hace y qué no hace esta sección

### Bienestar / PHQ-4 · Pregunta 1 — claridad 9

- No hay botón visible de "Siguiente" o "Continuar" tras seleccionar una respuesta, lo que deja ambiguo cómo avanza el usuario.
- Las opciones de radio button no muestran estado seleccionado claramente hasta que se elige.
- El enlace "Omitir esta pregunta" compite visualmente con la acción principal esperada (enviar respuesta).
- No hay indicación clara de cuánto tiempo tomará completar las 4 preguntas más allá del contador "1 DE 4".
- *CTAs actuales:* Omitir esta pregunta

### Hábitos / 07 Mi plan de cambio — claridad 9

- Posible error tipográfico: "Revisa" en lugar de "Revisa"
- Los iconos de lápiz indican edición pero no hay etiqueta ni retroalimentación visual clara de su función
- La sección "PARA RECUPERAR · Tiempo para mí" es ambigua: no está claro si es un campo editable o solo una etiqueta informativa
- Falta un botón explícito de guardar o confirmar cambios aparte del CTA principal
- *CTAs actuales:* Ensayar durante 1 minuto

### Meditación / Guía activa · DESCANSO — claridad 9

- El texto "AHORA · LLEGAR" es ambiguo: parece un estado pero coincide con el nombre del botón principal "Llegar".
- Tres botones horizontales similares (Llegar, Descansar, Cerrar) sin diferenciar claramente su función principal más allá del color.
- La barra de progreso es muy delgada y no indica visualmente la posición actual de reproducción.
- "Terminar práctica" está aislado en la parte inferior y podría tocarse por error al intentar interactuar con controles cercanos.
- No hay mención de precio ni plan, por lo que no se sabe si el contenido es gratuito o de pago.
- *CTAs actuales:* Llegar; Descansar; Cerrar; Ver la guía; Sonido; Terminar práctica; Reproducir

### Procrastinación / 05 Arranque de cinco minutos — claridad 9

- El enlace inferior combina dos acciones en una sola línea ("Ya empecé" y "Pasar al ciclo largo"), lo que puede generar ambigüedad sobre qué ocurre al tocarlo.
- No se menciona precio ni plan de suscripción en esta pantalla, aunque el texto inicial sugiere que habrá una decisión de continuidad después de los cinco minutos.
- La tarjeta de interrupción "Se me ocurrió otra cosa" es útil, pero compite visualmente con la instrucción principal del índice.
- *CTAs actuales:* Se me ocurrió otra cosa; Ya empecé · Pasar al ciclo largo; ← (retroceso)

### Hábitos / 04 Elegir mi motivo — claridad 9

- El subtítulo inicial se corta ligeramente en el borde izquierdo (se ve una letra suelta "p").
- No hay indicación de cuánto falta exactamente más allá de "4 DE 6".
- El aviso de que las respuestas no se guardan hasta elegirlas puede generar duda menor sobre si el avance es seguro.
- *CTAs actuales:* Continuar; ←

### Silva avanzado / 02 Diseña tu sesión — claridad 9

- El estado seleccionado solo se indica con un borde blanco sutil, lo cual puede pasar desapercibido
- Hay mucho espacio vacío entre las opciones y el CTA inferior
- El botón de información (i) no explica su propósito sin tocarlo
- No hay indicación visual clara de cuál es la opción predeterminada si el usuario no elige nada
- *CTAs actuales:* Guardar ajustes y elegir práctica; 1 min; 3 min; 5 min; Guía escrita; Lectura del dispositivo

### Procrastinación / 08 Lo que sí hiciste — claridad 9

- El texto "CIERRE DE SESIÓN" en el encabezado es ambiguo: puede interpretarse como cerrar sesión de la app en lugar de finalizar una sesión de trabajo.
- No hay un botón explícito de guardar o confirmar después de seleccionar el estado de la tarea.
- El campo "Escribir el apartado de resultados." aparece como texto placeholder dentro de una tarjeta sin indicar claramente que es editable.
- El enlace "Seguir con otro ciclo" está separado por una línea roja y su jerarquía visual es débil respecto a las acciones principales.
- *CTAs actuales:* Empezada; A medias; Terminada; Atascada; Seguir con otro ciclo; ←

### Procrastinación / 04 Preparar el entorno — claridad 9

- No hay un botón principal explícito de "Comenzar" o "Iniciar" tras elegir la duración; no queda claro si seleccionar un tiempo inicia automáticamente el temporizador.
- El enlace "Saltar la preparación" está separado por una línea roja delgada que puede confundirse con un divisor decorativo en lugar de un botón accionable.
- El toggle "Una sola ventana abierta" viene desactivado por defecto, lo cual contradice el objetivo de enfoque de la pantalla.
- Los toggles solo registran intención del usuario, no verifican que realmente haya apartado el móvil o cerrado ventanas.
- *CTAs actuales:* 5 min; 10 min; 25 min; Saltar la preparación; ←

### Frecuencias / 02 Preparar paisaje sonoro — claridad 9

- No hay un botón explícito de "Continuar" o "Siguiente"; el único CTA visible es el enlace de texto "Ajustar mezcla" al final.
- La tarjeta seleccionada se distingue solo por un fondo lavanda más claro, lo cual puede ser sutil.
- El texto menciona ajustar el volumen, pero no se muestra ningún control de volumen en pantalla.
- La duración muestra 12 minutos seleccionada y menciona 5 o 20 como alternativas, pero no indica cómo cambiarlas.
- *CTAs actuales:* Ajustar mezcla

### Favoritos / Primera visita — claridad 9

- Estado vacío sin contenido real de rituales mostrados
- Solo un CTA principal, puede sentirse como callejón sin salida si el usuario no quiere explorar
- La imagen decorativa no muestra contenido real de la app
- Título "Tus rituales favoritos" sugiere existencia de contenido que aún no hay
- No se explica qué ocurre tras pulsar "Explorar prácticas"
- *CTAs actuales:* Explorar prácticas; Perfil; Prácticas; Inicio; Mi diario

### Hábitos / 09 Revisar sin juzgar — claridad 9

- La primera opción parece seleccionada por color de fondo, pero no está claro si es un estado de selección exclusiva o solo resaltado visual.
- El enlace "Terminar sin guardar" está cerca del botón principal y podría provocar pérdida accidental de datos.
- No hay un botón explícito de "Siguiente" aparte de guardar, lo que puede generar duda sobre si se debe elegir una opción antes de continuar.
- *CTAs actuales:* Guardar mi plan y revisión; Terminar sin guardar; Me resultó posible; Necesito un paso más pequeño; Hoy me costó

### TDAH / 02 Elegir el obstáculo — claridad 9

- No se ve un botón explícito de "Continuar" o "Siguiente" para avanzar tras elegir una opción
- No está claro si se puede seleccionar más de una opción o solo una
- La línea roja al final no tiene propósito visible ni etiqueta
- La jerarquía indica selección pero no da una acción clara de conversión inmediata
- *CTAs actuales:* Me voy sin darme cuenta; Empezar cuesta más que hacerlo; Se me va el tiempo sin notarlo; Dejo muchas cosas a medias

### Afirmaciones / 01 Necesidad — claridad 9

- No hay botón visible de "Continuar" o "Siguiente" tras elegir una opción
- La tarjeta seleccionada (Confianza) no indica claramente qué acción sigue
- La línea roja inferior parece un elemento de diseño sin función obvia
- Falta retroceso visual claro al tocar las tarjetas no seleccionadas
- *CTAs actuales:* ←; Confianza; Amor propio; Abundancia; Límites

### Procrastinación / Estado · No pude empezar — claridad 9

- "SIN ARRANCAR" puede resultar ambiguo sin contexto previo
- No hay un botón primario destacado al final, solo un enlace de texto "Dejarlo por hoy"
- El estado seleccionado de la primera tarjeta es sutil (solo cambio de fondo)
- *CTAs actuales:* Bájalo a dos minutos; Cambia el paso por otro; Aplázalo con fecha; Dejarlo por hoy; ←

### Acceso / 01 Inicia sesión — claridad 9

- El icono de Apple aparece como un cuadrado oscuro genérico en lugar del logo oficial, lo que puede generar confusión o parecer un error de carga.
- "Explorar sin cuenta" está al final como texto plano sin contorno de botón, reduciendo su visibilidad como CTA.
- No se muestran requisitos de contraseña ni enlaces a términos de servicio o política de privacidad cerca del formulario.
- El botón de cerrar (X) en la esquina superior derecha podría descartar todo el flujo de login si se presiona por error.
- *CTAs actuales:* Continuar con Apple; Continuar con Google; Iniciar sesión; Crear cuenta; Explorar sin cuenta; ¿Olvidaste tu contraseña?

### Sesión / Audio no disponible — claridad 9

- La línea roja sobre "Volver a mis prácticas" parece un divisor visual sin función clara
- No hay un botón explícito de cerrar o descartar el error aparte de regresar
- Ambas tarjetas principales tienen peso visual similar aunque una es la alternativa recomendada
- *CTAs actuales:* Continuar con la guía escrita; Reintentar el audio; Volver a mis prácticas

### Afirmaciones / 02 Ajustar frase — claridad 9

- No hay botón visible de "Continuar" o "Siguiente" para avanzar tras elegir una frase
- La línea roja inferior parece un artefacto de diseño sin función clara
- El área "Mi versión personal" no indica visualmente si es editable
- La nota al pie sugiere editar frases pero no hay botón de edición visible junto a la tarjeta personalizada
- *CTAs actuales:* Puedo dar un paso pequeño; Estoy aprendiendo a confiar en mí; Puedo mostrar mi trabajo

### Configuración / Configuración — claridad 9

- No se ve opción de cerrar sesión ni número de versión de la app
- No hay botón de guardado explícito; no queda claro si los cambios se aplican automáticamente
- La fila "Idioma" muestra "Español" pero no indica si es el idioma actual o si se puede cambiar desde ahí
- El encabezado "PERFIL · TU EXPERIENCIA" puede generar confusión sobre el contexto de navegación
- *CTAs actuales:* Recordatorios; Audio y guía; Accesibilidad; Idioma; Privacidad; Cuenta; Ayuda

### Favoritos / Prácticas guardadas — claridad 9

- Los íconos de marcador resultan redundantes en una pantalla ya llamada "favoritos"
- No hay acción visible para eliminar o gestionar prácticas de la lista de favoritos
- El botón de retroceso superior dice "Perfil" aunque el usuario ya está en esa pestaña
- No se muestran precios ni planes de suscripción en esta pantalla
- *CTAs actuales:* Reproducir (botón púrpura); Marcar (ícono de marcador); Inicio; Prácticas; Mi diario; Perfil

### Procrastinación / 01 Nombrar lo aplazado — claridad 9

- No hay un botón principal explícito (tipo "Continuar" o "Seleccionar") para avanzar tras elegir una tarea.
- El campo de texto ya contiene "Terminar el informe trimestral", pero no está claro si es editable, una sugerencia o una selección predeterminada.
- La línea roja en la parte inferior parece un elemento visual sin propósito claro o un error de maquetación.
- *CTAs actuales:* Llamar a Hacienda; Pedir cita médica; Responder a Marta; Terminar el informe trimestral

### Entrada / 01 Cómo te sientes hoy — claridad 9

- El botón principal inferior "Ver prácticas para mí" se ve deshabilitado o secundario, lo que puede generar duda sobre si es necesario seleccionar una emoción primero.
- Los íconos del círculo no tienen etiquetas, por lo que su significado puede ser ambiguo para algunos usuarios.
- El ícono de información (i) no tiene contexto ni tooltip visible.
- No hay indicación clara de qué ocurre tras tocar un símbolo o emoción.
- La opción "Explorar por mi cuenta" está al final con menor jerarquía visual y podría pasarse por alto.
- *CTAs actuales:* Ver prácticas para mí; Explorar por mi cuenta; En calma; Con energía; Con estrés; Triste; Con cansancio; No sé aún

### Hábitos / 01 Reconocer el hábito — claridad 9

- La tarjeta "Otro hábito" no muestra campo de texto ni explica qué ocurre al tocarla (¿abre un input?).
- El botón "Continuar" parece activo aunque no queda claro si la selección es obligatoria o si ya viene preseleccionada por defecto.
- El aviso final "Tus respuestas no se guardan hasta que tú lo elijas" es ligeramente ambiguo y podría generar duda sobre cuándo se confirma la elección.
- *CTAs actuales:* Continuar; Mirar el móvil sin querer; Posponer lo importante; Responder por impulso; Alargar la noche; Actuar en automático; Otro hábito

### Deja de ser tú / 09 Ensaya tu nuevo día — claridad 9

- No se indica si los campos son obligatorios o opcionales
- No hay contador de caracteres ni límite visible en los campos de texto
- El botón "Continuar" no muestra estado (habilitado/deshabilitado) según si hay contenido escrito
- Ambos botones inferiores tienen el mismo peso visual púrpura, aunque "Volver a la biblioteca" es una acción secundaria
- No hay indicador de guardado automático o borrador
- *CTAs actuales:* Continuar; Volver a la biblioteca; Atrás; Información

### TDAH / 06 Volver sin castigo — claridad 9

- No hay un botón principal de avance claro; solo aparece "DE VUELTA" y un enlace al final
- La línea roja divisoria puede confundirse con un separador visual en lugar de preceder a una acción
- La tarjeta con el corazón podría pasar desapercibida al no tener el mismo énfasis visual que los pasos numerados
- *CTAs actuales:* DE VUELTA; Cambiar a una tarea más corta

### Hábitos / 03 Entender la necesidad — claridad 9

- El texto "Puedes cuidar esa necesidad de otra manera." es pequeño y puede pasar desapercibido.
- No hay indicación visual de qué ocurre tras pulsar "Continuar" más allá del avance del flujo.
- El aviso de privacidad al pie puede generar duda leve sobre si las respuestas se están guardando en tiempo real.
- *CTAs actuales:* Continuar; Distraerme un poco; Descansar; Sentir alivio; Todavía no lo sé

### TDAH / 10 Patrón por franja horaria — claridad 9

- La barra de Tarde aparece vacía/gris sin valor numérico, lo que puede confundir si significa cero minutos o simplemente menos tiempo
- El gráfico no muestra valores exactos en las barras, solo altura relativa
- "22 min tramo medio" es ambiguo: no especifica si es promedio diario, por franja o en qué periodo
- No hay un botón de acción principal destacado; el único CTA es un enlace de texto al final
- La flecha de retroceso y el encabezado son visualmente pequeños respecto al contenido principal
- *CTAs actuales:* Ver el detalle por tarea

### Peticiones / 04 Preparar ritual — claridad 9

- La línea roja inferior no tiene propósito visual claro y puede confundir como separador o error de diseño.
- No hay un CTA principal visible para avanzar al siguiente paso; solo aparece "Editar mi petición".
- El fondo de tablero de ajedrez en la tarjeta de la petición puede interpretarse como imagen faltante o placeholder en lugar de elemento decorativo intencional.
- *CTAs actuales:* Editar mi petición; ←

### Horas espejo / 01 Las 24 horas — claridad 9

- No hay botón visible de confirmación, 'Siguiente' o 'Guardar' tras seleccionar una hora
- La línea roja inferior no tiene propósito claro ni etiqueta
- No se indica qué ocurre después de elegir la hora espejo (falta siguiente paso)

### Deja de ser tú / 03 Reconoce el patrón — claridad 9

- No hay indicador de progreso (paso X de Y)
- Los campos muestran texto de ejemplo que podría confundirse con contenido ya ingresado
- No se especifica límite de caracteres ni validación obligatoria antes de continuar
- El botón "Continuar" no indica visualmente si está habilitado o deshabilitado
- *CTAs actuales:* Continuar; Volver a la biblioteca

### Programa / Semana 2 · Azul — claridad 9

- "0 de 7 prácticas" puede generar ambigüedad sobre si el usuario ya comenzó o cuántas ha completado realmente
- No hay un botón principal de inicio o "Empezar" visible; el usuario debe tocar cada ítem individualmente
- La línea roja cerca de la parte inferior parece un artefacto de diseño sin propósito claro
- No se menciona precio, plan ni estado de suscripción, por lo que no está claro si el contenido es gratuito o de pago
- *CTAs actuales:* Nombrar el patrón; Reconocer el disparador; Escuchar la emoción; Revisar lo que me exijo; Escribir lo que repites; Elegir qué quiero cambiar; Integrar y registrar; Inicio; Prácticas; Mi diario; Perfil

### Sesión / 03 Completada · Azul — claridad 9

- Dos líneas rojas horizontales en la parte inferior sin propósito o etiqueta visible
- No hay botón claro de 'continuar' o 'finalizar' después de seleccionar el estado de ánimo
- La métrica '3 / 28 prácticas' puede confundirse con 'Día 3 completado' de la tarjeta central
- *CTAs actuales:* Más en calma; Igual; Con inquietud

### Procrastinación / 06 Ciclo de enfoque — claridad 9

- No se ve un botón claro de pausa o reinicio del temporizador en la zona principal.
- La flecha de retroceso superior es pequeña y de bajo contraste visual.
- La acción final "Terminar y registrar lo hecho" está aislada al final bajo una línea roja, lo que puede pasar desapercibida o generar duda sobre si interrumpe el ciclo.
- No está claro si desde esta pantalla se pueden editar o agregar las distracciones anotadas antes de finalizar el ciclo.
- *CTAs actuales:* ←; Terminar y registrar lo hecho

### Hábitos / 10 Plan guardado — claridad 9

- El enlace "Editar mi plan" tiene baja jerarquía visual y está al final, lo que reduce su visibilidad.
- No hay un indicador visual explícito de éxito más allá del tilde y el texto de guardado.
- El botón de retroceso superior lleva la etiqueta "CAMBIAR UN HÁBITO", lo que puede generar ambigüedad sobre si navega atrás o abre una edición.
- *CTAs actuales:* Volver a practicar; Editar mi plan

### TDAH / 04 Ajustar el entorno — claridad 9

- El botón principal de guardado aparece como texto enlazado al final separado por una línea roja, sin jerarquía visual de botón tradicional
- No hay un botón de "Siguiente" o "Continuar" evidente para avanzar
- La tercera opción viene desactivada por defecto sin explicación de por qué difiere de las otras dos activadas
- El encabezado "ENTORNO" junto al botón de regreso puede generar confusión sobre si es un título de sección o una etiqueta de navegación
- *CTAs actuales:* Guardar como mi entorno habitual

### Peticiones / 06 Compromiso — claridad 9

- "Activar si quiero" parece un enlace de texto sin botón claro, su affordance es ambigua
- No hay un botón primario visible para guardar con el recordatorio activado; la única acción destacada al final es "Guardar sin recordatorio"
- La hora del recordatorio (09:30) se muestra pero no hay forma obvia de editarla desde esta pantalla
- El enlace final puede pasar desapercibido por estar al fondo y sin contenedor destacado
- *CTAs actuales:* Activar si quiero; Guardar sin recordatorio

### Terapias respiratorias / 04 Sesión · Coherencia 5 · 5 — claridad 9

- El botón principal de detener está al final como texto plano, sin contenedor destacado, lo que puede reducir su visibilidad como CTA principal
- La leyenda "5 + 5" en el encabezado no explica inmediatamente si se refiere a segundos de inhalar/exhalar
- La línea roja inferior carece de contexto visual claro
- *CTAs actuales:* Detener y volver a mi ritmo; Cambiar las señales del ritmo

### Mi diario / Detalle · Registro guardado — claridad 9

- La línea roja bajo "Esta nota es para ti..." puede interpretarse erróneamente como campo obligatorio o error en lugar de divisor decorativo.
- El texto dice que se pueden cambiar las palabras, pero no hay botón de edición visible, lo que debilita la affordance de edición.
- No hay indicador visual claro de que la nota ya se guardó más allá del texto "GUARDADO" en el encabezado.
- *CTAs actuales:* ←; Volver a mi diario

### Configuración / Privacidad — claridad 9

- La tarjeta "Nota de mi práctica" no tiene interruptor ni flecha, por lo que no queda claro si es solo informativa o si al tocarla abre algo.
- El botón "Guardar privacidad" puede resultar redundante si los cambios se guardan automáticamente al activar el toggle.
- No hay indicación visual de qué cambios están pendientes de guardar antes de pulsar el botón inferior.
- *CTAs actuales:* Guardar privacidad; Revisar mis datos; Información de privacidad; Eliminar cuenta

### Onboarding / 05 Rutina — claridad 9

- La opción "Después de apagar la alarma" se presenta como una tarjeta similar a las demás pero incluye texto explicativo, lo que puede generar ambigüedad sobre si es seleccionable o solo informativa.
- No hay indicación visual de qué opciones de momento del día están seleccionadas, a diferencia de la duración base donde la de 8 min está claramente resaltada.
- El indicador "5 DE 6" muestra que falta un paso más, pero no se explica qué viene después ni si hay costo asociado.
- *CTAs actuales:* Crear mi semana; 3 MIN; 8 MIN; 15 MIN; AL DESPERTAR; ANTES DE TRABAJAR; AL PAUSAR; ANTES DE DORMIR; ←

### Welcome / cinco — claridad 9

- No se muestra precio ni plan de suscripción en esta pantalla, lo que puede generar incertidumbre sobre costos futuros.
- "Omitir" es pequeño y está en la esquina superior derecha, con bajo peso visual respecto al CTA principal.
- "Explorar sin cuenta" tiene bajo contraste y podría pasar desapercibido.
- No hay indicación de prueba gratuita, duración ni qué incluye la meditación guiada.
- *CTAs actuales:* Elegir mi primera meditación; Explorar sin cuenta; Omitir

### Bienestar / 08 Contexto del enfoque — claridad 9

- No hay botón visible de "Siguiente" o "Continuar" para avanzar tras seleccionar una opción
- No se indica el progreso (cuántas preguntas quedan)
- La imagen de la rosa es puramente decorativa y no aporta contexto funcional
- El enlace "Omitir esta pregunta" está al final sin un CTA primario claro junto a las opciones
- *CTAs actuales:* Omitir esta pregunta

### Meditación / 06 Integrar — claridad 9

- No hay un botón principal explícito para confirmar o enviar la opción seleccionada
- El área de texto bajo "Un paso para después" parece un campo editable pero muestra una sugerencia fija, lo que genera ambigüedad sobre si es editable
- El enlace "Guardar sin escribir" puede confundir: no queda claro si guarda la respuesta seleccionada o la descarta
- La línea roja divisoria inferior no tiene un propósito visual evidente
- *CTAs actuales:* Guardar sin escribir

### Onboarding / 03 Patrones — claridad 9

- Dos opciones ya aparecen preseleccionadas con checkmarks sin que quede claro si el usuario las eligió o son valores por defecto
- La opción "NO ESTOY SEGURO" puede generar abandono o indecisión en el flujo
- No hay retroalimentación visual inmediata al seleccionar/deseleccionar más allá del check
- El indicador "3 DE 6" no explica cuánto falta ni el propósito total del onboarding
- No se muestra qué sucede tras pulsar Continuar ni si hay paso de pago o suscripción próximo
- *CTAs actuales:* Continuar →

### Hábitos / 08 Pausa y ensayo activo — claridad 9

- El temporizador muestra 01:00 pero no está claro si ya está corriendo o espera a que se presione "Comenzar".
- El texto "Pausa guiada en pantalla" es pequeño y puede generar confusión sobre su función.
- No hay indicación clara de qué ocurre tras presionar "Terminar y revisar" ni cuánto dura el resto del flujo (falta contexto de los pasos 2 y 3).
- *CTAs actuales:* Terminar y revisar; Comenzar; Pausa guiada en pantalla

### Programa / Semana 1 · Azul — claridad 9

- No hay un botón CTA principal grande para iniciar la sesión actual; el play es pequeño y está a la derecha de "Atender al presente".
- Los candados en D4-D7 generan fricción visual aunque sean esperados en un plan progresivo.
- El indicador "≈ 15 min al día" está pequeño y puede pasar desapercibido.
- El ícono de compartir en la esquina superior derecha compite visualmente con el título y no aporta contexto inmediato.
- *CTAs actuales:* D1; D2; D3; D4; D5; D6; D7; Atender al presente; Inicio; Prácticas; Mi diario; Perfil

### Acceso / 03 Recupera tu acceso — claridad 9

- El placeholder "nombre@correo.com" puede confundirse con texto ya escrito en lugar de ejemplo
- No hay indicador visual de campo obligatorio ni mensajes de validación/error
- El enlace "Explorar sin cuenta" tiene baja jerarquía visual y podría pasar desapercibido
- No se muestra retroalimentación de estado (carga, éxito o error) tras pulsar el botón principal
- *CTAs actuales:* Enviar enlace; Volver a iniciar sesión; Explorar sin cuenta

### Meditación / Detalle · ENFOQUE — claridad 9

- No hay un botón CTA principal visible para iniciar la sesión de 8 minutos
- La tarjeta central repite el título pero no indica claramente si es interactiva o cómo iniciar el ejercicio
- La línea roja inferior parece un elemento de diseño sin propósito claro
- No hay indicador de progreso ni estado actual entre los tres pasos

### Astrología / astro-planets — claridad 9

- Solo se muestran 5 planetas sin indicador visual de que hay más contenido por desplazarse
- No se incluyen iconos o glifos astrológicos junto a los nombres de los planetas
- El botón de retroceso es pequeño y podría pasar desapercibido
- No hay contexto de precios, planes ni llamada a conversión visible en esta pantalla
- *CTAs actuales:* Ver todos los símbolos; ←

### Bienestar / PHQ-4 · Pregunta 4 — claridad 9

- No hay botón visible de "Siguiente" o "Continuar" tras seleccionar una opción; no está claro cómo avanza el flujo.
- El indicador de progreso muestra cuatro líneas pero no es evidente cuáles están completadas.
- El ícono superior derecho no tiene etiqueta ni función obvia.
- La línea roja divisoria antes de "Omitir esta pregunta" puede interpretarse como un elemento de error o advertencia en lugar de un separador decorativo.
- *CTAs actuales:* Omitir esta pregunta

### Acompañamiento / Preocupación / 03 Integrar — claridad 9

- No hay un botón principal visible para guardar y continuar; solo aparece "Terminar sin guardar", lo que deja ambiguo cómo avanzar guardando el registro.
- El campo de texto opcional no indica claramente que es editable ni tiene un botón de envío asociado.
- La opción "Terminar sin guardar" puede generar ansiedad de pérdida de datos si se toca por error.
- *CTAs actuales:* Terminar sin guardar

### Deja de ser tú / 15 Observa tu recorrido — claridad 9

- La etiqueta "TÚ ÚLTIMO PASO" parece descontextualizada sobre un estado vacío sin indicador de progreso.
- El botón destructivo "Borrar mis registros" tiene la misma prominencia visual que el botón principal de acción.
- Hay cierta redundancia entre la tarjeta del "0" y la tarjeta de estado vacío que repite que no hay prácticas guardadas.
- No hay un CTA directo para iniciar una práctica desde esta pantalla; solo se ofrece volver a elegir otra.
- *CTAs actuales:* Elegir otra práctica; Borrar mis registros

### Silva avanzado / 12 Lo que recuerdo — claridad 9

- El texto de sugerencia dentro de los campos podría confundirse con contenido ya escrito si el usuario no lee con atención.
- No hay indicador de progreso ni contador de caracteres en los campos de texto.
- El botón de retroceso superior y el botón "Volver a la biblioteca" inferior pueden ser redundantes.
- No se indica si los cambios se guardan automáticamente o si hay riesgo de perder la entrada al salir.
- *CTAs actuales:* Continuar con mi reflexión; Volver a la biblioteca; flecha de retroceso; ícono de información

### Respiración / 03 Inhalar — claridad 9

- No hay botón de pausa visible en la pantalla principal
- El enlace inferior "Volver a mi respiración natural" es ambiguo: no está claro si detiene el ejercicio, sale de la sesión o reinicia
- "Sin retenciones" puede requerir interpretación para usuarios no familiarizados con técnicas de respiración
- La tarjeta de ajustes de sonido/vibración compite visualmente con el foco principal del ejercicio
- *CTAs actuales:* Ajustar las señales de la guía; Volver a mi respiración natural

### Silva avanzado / 10 Laboratorio mental — claridad 9

- No se indica límite o contador de caracteres en los campos de texto
- La imagen del eclipse es decorativa pero su relación con el ejercicio no se explica
- El botón "Volver a la biblioteca" puede hacer perder progreso sin confirmación
- Los dos campos de texto tienen apariencia idéntica, solo diferenciados por sus etiquetas
- *CTAs actuales:* Continuar con esta idea; Volver a la biblioteca

### Astrología / astro-sign — claridad 9

- No se explica qué ocurre tras pulsar "Escribir mi respuesta" (¿guardar diario, avanzar, suscribirse?).
- No hay opción visible para saltar o posponer la actividad.
- La palabra "Iniciar" en el título puede generar expectativa de inicio de un servicio en lugar de una pregunta de reflexión.
- La imagen hero ocupa mucho espacio visual sin aportar información funcional clara.
- *CTAs actuales:* Escribir mi respuesta; ←

### Silva avanzado / 14 De imaginar a hacer — claridad 9

- El placeholder del primer campo termina con puntos suspensivos ("Hoy, a las..., voy a..."), lo que puede confundirse con texto ya escrito o incompleto.
- No hay contador de caracteres ni indicación de longitud máxima en los campos de texto.
- El botón "Terminar sin guardar" está ubicado justo debajo del primario, lo que podría generar toques accidentales y pérdida de datos.
- No se explica qué ocurre después de guardar ni si hay una confirmación de éxito.
- *CTAs actuales:* Guardar mi intención y mi paso; Terminar sin guardar

### Procrastinación / Estado · Tarea sin nombre — claridad 9

- No se ve un botón explícito de enviar o continuar; puede depender de escritura automática o estar fuera de pantalla.
- El placeholder en rojo puede interpretarse como error antes de que el usuario escriba.
- La línea roja inferior parece un divisor sin propósito claro o UI incompleta.
- *CTAs actuales:* Escribe qué estás aplazando.

### Terapias respiratorias / 05 Detalle · 4 · 7 · 8 — claridad 9

- La línea roja delgada sobre el CTA inferior puede interpretarse como un separador de error o división visual innecesaria.
- No hay un botón principal de "comenzar" o "iniciar respiración" visible; solo aparece la alternativa sin retención al final.
- La flecha de retroceso es pequeña y podría pasar desapercibida.
- *CTAs actuales:* Elegir una guía sin retención

### Bienestar / 01 Entender y elegir — claridad 9

- El botón secundario "Prefiero una práctica" tiene menos jerarquía visual que el botón morado principal, lo que puede reducir su visibilidad como alternativa.
- No hay indicador de progreso ni contexto de cuántos pasos tiene el flujo de bienestar.
- El aviso "Mayores de 18" aparece sin explicación adicional, lo que podría generar dudas o fricción.
- El botón final "Necesito apoyo ahora" no especifica qué tipo de apoyo se ofrece (humano, de crisis, técnico, etc.).
- No se mencionan precios ni planes en esta pantalla.
- *CTAs actuales:* Comenzar cuestionario; Prefiero una práctica; Necesito apoyo ahora

### Sesión / Pausa — claridad 9

- No hay un botón explícito de "Reanudar" o "Seguir"; la acción principal está superpuesta sobre la imagen de fondo
- "Tiempo detenido" puede resultar ambiguo sin contexto adicional
- El enlace "Terminar por hoy" tiene menor jerarquía visual que las tarjetas principales
- La flecha de retroceso junto a "SESIÓN EN PAUSA" no deja claro si es navegación o solo una etiqueta de estado
- *CTAs actuales:* Continúa cuando estés listo.; Ajustar voz y ambiente; Leer la guía; Terminar por hoy

### Acompañamiento / Enfoque / 01 Preparar — claridad 9

- No hay un botón principal evidente para iniciar los 60 segundos de práctica; la acción parece implícita.
- El enlace inferior "Cambiar forma de practicar" tiene bajo contraste y tamaño pequeño, lo que reduce su visibilidad como CTA.
- La superposición de texto sobre la imagen de la flor podría dificultar ligeramente la lectura en pantallas pequeñas.
- *CTAs actuales:* Cambiar forma de practicar

### Sesión / 01 Preparar · Azul — claridad 9

- No hay un botón principal de reproducción o inicio visible en la pantalla; no está claro si la meditación comienza automáticamente.
- El botón "Mi voz" es ambiguo: puede interpretarse como grabar la propia voz o simplemente escuchar sin guía externa, sin contexto adicional.
- El enlace "Cambiar" tiene menor peso visual que los botones principales y podría pasar desapercibido.
- El enlace de descarga está al final y podría no notarse de inmediato.
- *CTAs actuales:* Voz guía; Mi voz; Sin voz; Cambiar; Descargar para escuchar sin conexión

### Meditación / 01 Elegir necesidad — claridad 9

- Las tarjetas no tienen etiquetas de botón explícitas (ej. "Seleccionar" o "Empezar"), se basa en la affordance de tocar la tarjeta completa.
- "PARA EMPEZAR · 8 MINUTOS" es ambiguo: no está claro si es un botón activable, una etiqueta informativa o si aplica a todas las opciones por igual.
- No hay indicador visual de selección activa entre las cuatro opciones.
- La línea roja inferior no tiene propósito visual claro y puede confundir.
- La flecha de retroceso en una pantalla de selección puede generar duda sobre si se puede regresar en un flujo de onboarding.
- *CTAs actuales:* Dejar de darle vueltas; Recuperar enfoque; Descansar; Tratarme con amabilidad; PARA EMPEZAR

### Deja de ser tú / 10 Atención por zonas — claridad 9

- La marca de verificación aparece tanto en la opción seleccionada de la lista como en el botón "Marcar como observada", lo que puede generar redundancia o confusión sobre qué acción ya se realizó.
- El progreso muestra 1/5 sin dejar claro si se refiere a zonas completadas o pasos del recorrido.
- El botón "Finalizar recorrido" está disponible desde el primer paso, lo que podría permitir salir prematuro del ejercicio.
- *CTAs actuales:* Escuchar esta zona; Marcar como observada; Finalizar recorrido

### Meditación / 04 Personalizar sesión — claridad 9

- No hay botón visible de inicio o confirmación para comenzar la meditación tras elegir opciones
- La línea roja inferior sugiere contenido cortado o sección incompleta
- No se explica explícitamente que los tiempos del desglose suman los 8 minutos seleccionados
- *CTAs actuales:* 3 min; 8 min; 15 min; Voz guía + lluvia suave

### Acompañamiento / Ánimo bajo / 02 Práctica — claridad 9

- El botón "Ya la leí · elegir un gesto" combina dos acciones en una sola etiqueta, lo que puede generar confusión sobre qué ocurrirá al pulsarlo.
- No queda claro si los tres botones intermedios (Reconocer, Acompañar, Elegir un gesto) son mutuamente excluyentes ni a qué flujo llevan cada uno.
- La imagen de la ventana no tiene una relación visual obvia con el mensaje de amabilidad, lo que puede debilitar la conexión emocional.
- El texto instructivo "Léela despacio. Puedes cambiar las palabras." tiene bajo contraste y tamaño reducido, riesgo de pasar desapercibido.
- No hay indicación de progreso ni de cuántos pasos tiene este ejercicio.
- *CTAs actuales:* X; Reconocer; Acompañar; Elegir un gesto; Ya la leí · elegir un gesto; Pausar o elegir otro apoyo

### Bienestar / Entrada desde Prácticas — claridad 9

- El indicador "4 min" está alineado a la derecha sin dejar claro si se refiere al enlace principal o a toda la tarjeta.
- No hay menciones de precio, planes ni opciones de suscripción en esta pantalla.
- La sección "También puedes..." presenta dos tarjetas con igual peso visual que podrían diluir la atención del CTA principal.
- La flecha de retroceso y el encabezado "BIENESTAR" indican que es una subpantalla, no el inicio raíz de la app.
- *CTAs actuales:* Explorar cómo estoy →; Respirar un momento; Encontrar apoyo; Inicio; Prácticas; Mi diario; Perfil

### Prácticas / Frecuencias · Azul — claridad 9

- La tarjeta destacada muestra un patrón de tablero de ajedrez en lugar de una imagen real, lo que puede sugerir una imagen faltante o estado de carga
- No se menciona precio ni plan de suscripción en esta pantalla
- El botón principal usa primera persona ("Dedicarme") lo cual es menos directo que un imperativo
- La flecha de retroceso en la parte superior puede generar confusión si el usuario llegó desde un enlace directo
- *CTAs actuales:* Dedicarme 12 minutos; Reproducir Sonidos para descansar; Reproducir La noche, en calma; Inicio; Prácticas; Mi diario; Perfil

### Sesión / Salir antes de terminar — claridad 9

- La marca de verificación en la primera opción puede interpretarse como selección activa aunque sea solo una alternativa.
- La jerarquía visual entre las dos opciones es similar; no hay un CTA primario claramente destacado por color o tamaño.
- El texto "Sesión previa: 8 minutos" dentro de la tarjeta de tiempo actual puede generar confusión sobre a qué sesión se refiere.
- *CTAs actuales:* Guardar como práctica parcial; Salir sin guardar; Volver a la sesión

### Meditación / 02 Biblioteca por necesidad — claridad 9

- La primera tarjeta tiene un overlay oscuro que dificulta leer el título y la descripción.
- El botón "Opciones de la guía" es ambiguo; no se especifica qué opciones contiene.
- No hay indicación visual clara de qué contenido es gratuito o de pago.
- La jerarquía visual es desigual entre la tarjeta destacada oscura y las tarjetas blancas estándar.
- *CTAs actuales:* Opciones de la guía; Volver a prácticas

### Bienestar / Resultado · Enfoque — claridad 9

- La casilla indica "Desactivado" aunque solo está desmarcada, lo que puede generar confusión sobre si la función está disponible.
- No hay un botón principal claro para iniciar la práctica; el único CTA destacado es un enlace de texto al final.
- El aviso "NO ES UN DIAGNÓSTICO" y la mención a TDAH aparecen en texto pequeño, pudiendo pasar desapercibidos o generar preocupación innecesaria.
- *CTAs actuales:* Quiero buscar apoyo; Guardar esta orientación

### Silva avanzado / 13 Vuelve a tu entorno — claridad 9

- No se indica el total de pasos ni el progreso del ejercicio (solo se ve el número 1).
- El CTA "Seguir volviendo" es ambiguo: no deja claro si continúa el ejercicio o regresa a una pantalla anterior.
- Los dos botones inferiores compiten visualmente aunque el primario está resaltado con color sólido.
- *CTAs actuales:* Seguir volviendo; Salir del ejercicio

### Mi plan / Prácticas recomendadas — claridad 9

- No hay un botón principal explícito para iniciar una meditación específica; las tarjetas son presumiblemente tocables pero no está señalizado
- El chip "3 experiencias" con insignia "3" no deja claro si es un filtro activo o un contador de contenido
- No se muestra información de precios, planes o suscripción, por lo que el modelo de negocio es ambiguo
- La navegación inferior marca "Prácticas" como activo aunque el usuario ya está en esa pantalla, lo que genera redundancia visual
- *CTAs actuales:* Meditación; 3 experiencias; Inicio; Prácticas; Mi diario; Perfil

### Afirmaciones / 03 Preparar repetición — claridad 9

- No hay un botón principal claro para iniciar o continuar tras elegir una opción
- El enlace "Editar mi afirmación" está separado visualmente de la tarjeta de la afirmación
- "Sin prisa" en la línea inferior puede ser ambiguo (¿función o solo texto tranquilizador?)
- La selección actual solo se indica con una tilde, sin contraste fuerte frente a las otras opciones
- *CTAs actuales:* Voz guía; Mi voz; En silencio; Editar mi afirmación

### Bienestar / 10 Preferencia de práctica — claridad 9

- Las tres tarjetas de opciones parecen seleccionables pero no tienen un estado de botón explícito ni indicador visual de interacción.
- No hay un botón principal de "Continuar" o "Siguiente" visible tras elegir una opción.
- El ícono superior derecho (ojo tachado o similar) no tiene etiqueta y su función es ambigua.
- La línea roja sobre el enlace inferior puede interpretarse erróneamente como un separador de error o advertencia.
- *CTAs actuales:* Elegir una práctica sin resultado

### Bienestar / 02 Elegir una pausa breve — claridad 9

- La cuarta tarjeta muestra un marcador de posición roto (tablero de ajedrez) en lugar de imagen.
- No hay un botón explícito de 'continuar' o 'seleccionar' tras elegir una tarjeta; no queda claro si tocar una tarjeta avanza automáticamente.
- El texto superior 'ELIGE CÓMO ACOMPAÑARTE' es pequeño y gris, con baja jerarquía visual.
- El CTA final está separado y puede pasar desapercibido frente a las tarjetas principales.
- *CTAs actuales:* Necesito otro tipo de apoyo →
