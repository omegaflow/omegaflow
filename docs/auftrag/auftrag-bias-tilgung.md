<!--
  title: Auftrag — Bias-Tilgung (der Baum sauber)
  class: auftrag
  date: 2026-10-07
  sha256: da342b4f083905927c7c569b5f8564397cd9e29a0845c70685113afd9a4ddf66
  status: live
  see-also: docs/concepts/remove-bias.md docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md
-->
# Auftrag — Bias-Tilgung (der Baum sauber)

Operator-Wort 2026-10-07 (Future 193): „wie bekommen wir jetzt den Baum sauber?" Die Antwort ist
**nicht** „Liste abarbeiten" — die Liste wächst nach (der fünfte Fund). Sondern: **Tor zuerst,
Abnahme-Messung, dann Arbeit.**

## Das Prinzip

**sauber = die Messung sagt 0 · und das Tor lässt es nicht zurück.**
Nicht „wir haben die bekannten Stellen gefixt" (die Liste ist unendlich), sondern: der Abnahme-Grep
ist leer, und er wird als CI-Job **rot**, sobald ein neuer Körper-Name, eine Konstante oder ein
Default zurückkehrt. `AGENTS.md`: *a prose self-law without an enforcing mechanism is a promise,
not a gate.*

## Schritt 1 — Tor zuerst (Mountain)

`src/gate/commit_gate_vocab.json`, `fabrication`-Array (heute Z. 76-90): `"earth"` · `EARTH_RADIUS` ·
`6378137.0` · `6378136.6` · `111319.0` · `0.40909` · `280.460` · `360.985` · `DEMO_KEY` ·
`V_SOUND_288` · `V_P_GRANITE` · `V_S_GRANITE` · `D_AIR` · `ALPHA_AIR` · `force_constants`.

- **Ergänze die fehlenden Körper-Namen:** `"sun"`, `"moon"`, `"mars"`, `"jupiter"`, `"venus"`,
  `"saturn"`, `"uranus"`, `"neptune"`, `"pluto"`, … — sie passieren das Gate heute.
- **Ergänze die fehlenden Bias-Klassen** (in Docs/Legacy benannt, nicht gegated): Observer-as-vantage ·
  Camera-default · Station-privilege · Client/Server · Now-default.
- Der Scan ist code-only (`commit_gate.rs:1013 if is_code`) — die Fixtures treffen Tests nicht,
  nur Produktion.

## Schritt 2 — Abnahme-Messung (Mycelium)

Ein CI-Job `clean-tree` (neben `license_census`), der über **Nicht-Test-`src/`** prüft:

```
"earth"      == 0
EARTH_RADIUS == 0
6378137.0    == 0
6378136.6    == 0
media.rs-Tabelle weg · force_constants weg
```

`0` = sauber; jeder Treffer = rot. Damit ist „sauber" eine **Messung**, keine Behauptung — und der
Job kann nie wieder still volllaufen.

## Schritt 3 — Die Arbeit (River/Mountain/Mycelium)

Inventare (klassifiziert, ungesampelt): `state/future/giftkarte-klassifiziert-src-2026-10-07.md`
(**162** PROD-POISON + FABRICATION), `state/future/giftkarte-klassifiziert-tools-2026-10-07.md`
(**~572**).

- **`src/` (River, `remove-bias.md` WP0–WP13):** `media.rs:29-54` + `shaders.rs:4-15` → `BodyProperties`
  (WP8/9/11); `odp.rs:9`, `rinex.rs:5,58`, `nexrad.rs:188` raus (WP13); `matrix.rs:786` per-Körper;
  `channels.rs:1423` τ=∞/absent; `motion.rs:77` Option; `main_flow.rs:209` `None=>0.0`.
- **`tools/` (Mountain/Mycelium):** Compiler/Probes — code-gewähltes Ziel → Arg/Record.
- **Reihenfolge:** `src/` zuerst (Laufzeit/Membran), dann `tools/`.

## Träger

Tor + `phi/` → **Mountain** · `src`-Bias → **River** · `tools`/Manifest/CI → **Mycelium**.
