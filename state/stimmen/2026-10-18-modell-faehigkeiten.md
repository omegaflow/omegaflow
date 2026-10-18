# Was an freien Modellen steuerbar ist (gemessen 2026-10-18)

**Quelle:** `models.dev/api.json` (der Katalog, den OpenCode liest), 5,8 MB, 232 Provider.
**Umfang:** 58 freie Modelle — 40 × `opencode/*-free` + 18 × `openrouter/*:free`.
**Kosten aller 58:** `0/0` (input/output) — vom Katalog ausgewiesen.

## Die Felder und was sie bedeuten

| Feld | Bedeutung | steuerbar? |
|---|---|---|
| `reasoning` | denkt das Modell? | ja — **wie** ist der Hebel |
| `reasoning_options` | `toggle` (an/aus) · `effort` (Stufen) · `budget_tokens` (Zahl) | ja |
| `tool_call` | kann Werkzeuge rufen (Shell, Suche) | nein (Fähigkeit) |
| `attachment` | kann Dateien/Bilder anhängen | nein (Fähigkeit) |
| `temperature` | Sampling-Temperatur akzeptiert | ja |
| `modalities.input` | `text` · `image` · `video` · `audio` · `pdf` | nein (Fähigkeit) |
| `limit.context` | Kontextfenster | nein (Grenze) |
| `limit.output` | maximale Antwortlänge | ja (bis zur Grenze) |
| `cost` | Preis | nein |

**Wichtig — zwei Dinge, die *keine* Modell-Einstellung sind:**
- **`search`** ist ein **Werkzeug** (`websearch`/`websearch.provider` in OpenCode, `frei_search` im Haus), keine Modelleigenschaft. Jedes Modell mit `tool_call: Y` kann suchen, wenn der Harness es ihm gibt.
- **`image`** steht in `modalities.input` + `attachment` — es ist eine **Fähigkeit**, kein Schalter.

**„Default"** ist kein Feld: es ist der Lauf **ohne** Variante. OpenCode baut die Varianten aus `reasoning_options`; ein Modell mit `effort: low/high/max` hat genau diese drei plus Default (Provider-Standard).

## Die Verteilung über die 58 freien Modelle

| Merkmal | Anzahl |
|---|---|
| `tool_call: Y` | **57** von 58 (nur `nemotron-3.5-content-safety:free` nicht) |
| `reasoning: Y` | **56** von 58 |
| … davon `toggle` | 21 |
| … davon `effort` (Stufen) | 22 |
| … davon `budget_tokens` | 4 |
| **ohne** Reasoning | 2 |
| `attachment: Y` | **25** |
| `temperature: Y` | **55** |
| Bild-Eingang | **25** |
| Kontext | min 65 536 · **median 262 144** · max **1 048 576** |
| Ausgabe | min 8 192 · max **524 288** |

## Die breitesten freien Modelle (effort + 1M Kontext + Anhang)

| Modell | effort | Kontext | Ausgabe | Eingang |
|---|---|---|---|---|
| `opencode/fledge-alpha-free` | low/high/max | 1 048 576 | 131 072 | text+bild |
| `opencode/muse-spark-1.2-contributor-free` | minimal/low/medium/high/xhigh | 1 048 576 | 131 072 | text+bild+video+pdf+audio |
| `opencode/space-bunny-free` | low/medium/high/xhigh/max | 1 048 576 | **524 288** | text+bild+video |
| `opencode/nemotron-3.5-super-vl-free` | none/minimal/low/medium/high/xhigh | 1 048 576 | 131 072 | text+bild |
| `opencode/muse-spark-1.3-contributor-free` | minimal/low/medium/high/xhigh | 1 048 576 | 131 072 | text+bild+video+pdf+audio |
| `openrouter/thinkingmachines/inkling:free` | none…max | 1 048 576 | 262 144 | text+bild+audio |

## Der Riss: Katalog ≠ Angebot

`models.dev` führt **40** `opencode/*-free`. Der **laufende Provider bietet nur 10 Modelle an, davon 9 frei**:

```
nemotron-3.5-super-vl · beam · longcat-2.5-preview · space-bunny
mimo-v2.6-flash · deepseek-v4.1-flash · glm-5.3-flash
nemotron-3.5-lightning · nemotron-3-ultra
```

31 der 40 sind **im Katalog, aber nicht im Angebot**. Wer nach dem Katalog plant, plant mit Modellen, die es nicht gibt.

## Konsequenz für den Schwarm

- **`effort`-Steuerung ist der wichtigste Hebel** — 22 Modelle können ihn. Bei kleinen Budgets ist `low` Pflicht (Reasoning frisst sonst die Antwort, gemessen bei llm7/OpenRouter).
- **`toggle`-Modelle** (21) kennen kein `low` — nur an/aus. Für Rat-Fragen heißt das: Reasoning ganz an oder ganz aus, kein Zwischenmaß.
- **Nur 25 von 58 können Anhänge** — der Rest ist Text-only. Für Bild-/PDF-Recherche bleibt ein Viertel.
- **Der Median-Kontext ist 262k**, nicht 1M. Die 1M-Modelle sind die Ausnahme, nicht die Regel.
