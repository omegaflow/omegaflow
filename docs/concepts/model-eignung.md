<!--
  title: Modelleignung für die Quellenarbeit
  class: concept
  date: 2026-10-21
  sha256: 724b5f560c477542a0919ad67e5cd9410e1c5eddfb090ceb0f252348c2a52a0a
  status: live
  see-also: rat/2026-10-21-rat-sources-schema.md docs/concepts/messzustaende-v3.md docs/surveys/survey-2026-10-21-sources-schema-archaeologie.md
-->
# Modelleignung für die Quellenarbeit

**Stand:** 2026-10-21, gemessen am lokalen Gateway (`127.0.0.1:1621`) gegen
DeepInfra/Mistral/DeepSeek. Kein Leaderboard; nur die Rollen, die wir
tatsächlich fahren.

## Rollen

| Rolle | Modell | Warum |
|---|---|---|
| **Autor** | Flash 1017 `high` | vollständiger Entwurf für $0,0019; billigster brauchbarer Autor |
| **Kritiker 1** | Kimi K3 `low` | scharf, billig; fand CHUR, `global`, `depth` |
| **Kritiker 2** | DeepSeek V4 Pro `none` | billig, anderer Blick; fand Frame/`spacecraft` |
| **Kritiker 3 (optional)** | Sonnet 5.5 `low` | starke Kurz-Kritik für $0,0081; Full-Design zu teuer |
| **Baumprüfung** | Session mit Repo-Zugriff | prüft Text gegen 2.856 Blöcke/7.458 Felder, nicht gegen Modellmeinung |
| **Nicht nötig** | Opus, GPT, GLM 5.3 | kein Unique Role; GLM-Route `HTTP/1` kaputt |

## Matrix (gemessen)

| Modell | Rolle | Setting | Budget | Ergebnis | Kosten |
|---|---|---|---:|---|---:|
| Flash 1017 | Autor | `high` | 8.000 | 6.154 tok, voll | $0,0019 |
| Flash 1017 | Gate-Audit | `high` | 6.000 | 4.867 tok, voll | $0,0015 |
| Flash 1017 | Gate-Audit | `max` | 6.000 | 5.009 tok, **nicht besser** | $0,0015 |
| Flash 1017 | Kurz-Kritik | `medium` | 1.500 | 590 tok, 503 Z. | $0,000202 |
| Flash 1017 | Kurz-Kritik | `high` | 1.500 | 1.074 tok | $0,000345 |
| Kimi K3 | Kritik | `low` | 1.200–1.800 | 545–730 tok, voll | $0,009–0,017 |
| Kimi K3 | Finalizer | **ohne** | 6.000 | 6.000 Reasoning, 0 Content | $0,09 |
| DeepSeek V4 Pro | Kritik | `none` | 1.200–1.800 | 352–467 tok, voll | $0,001–0,004 |
| DeepSeek V4 Pro | Kritik | `low` | 1.200 | 1.200 Reasoning, 0 Content | — |
| Sonnet 5.5 | Kurz-Kritik | `low` | 1.500 | 726 tok, 1.384 Z. | $0,0081 |
| Sonnet 5.5 | Kurz-Kritik | `high` | 1.500 | 1.421 tok, 990 Z. | $0,015 |
| Sonnet 5.5 | Design/Gate | `high` | 1.800/6.000 | `length`, leer | $0,025–0,062 |
| Opus 5.5 | Design | 16k | 16.000 | 8.009 tok, voll | $0,171 |
| GLM 5.3 | — | — | — | `HTTP/1`-Route | — |

## Reasoning-Semantik

- `low`/`medium`/`high` sind **modellspezifisch**; nie als generische Skala
  behandeln.
- `none` ist bei DeepSeek V4 Pro **gemessen** (`reasoning_tokens=0`, Content da),
  aber nicht dokumentiert; als Schalter behandeln, bei Modellwechsel prüfen.
- `max` bringt bei Flash 1017 **keinen** Gewinn; mehr Text, schlechtere
  Klassifikation.
- **Ohne** `reasoning_effort` verbrennen Kimi/Opus/Sonnet das Budget im
  Reasoning und liefern 0 Content. Das ist der teuerste Fehler.

## Sparregeln

1. `reasoning_effort` **immer explizit** setzen.
2. Budget **decken**: Flash 8k, Kimi 1.8k, DeepSeek 1.8k, Sonnet 1.5k (nur kurz).
3. **Full-Design nie** an Kritiker-Modelle; Sonnet/Opus sind dafür zu teuer.
4. `max` nicht verwenden.
5. Bei Modellwechsel/neuer Route neu messen; sonst diese Matrix nutzen.

## OpenCode

Die Varianten werden in `opencode.json` definiert (`providers.deepseek.models.
deepseek-flash.variants`): `low` → `reasoningEffort:"low"`, `mid` → `"medium"`,
`high` → `"high"`. Agenten: `plan` high, `build` low, `general` mid,
`explore` low. Commands können per Frontmatter überschreiben:
`model: deepseek/deepseek-flash#mid`.

## Prüfstand

- Letzte Messung: 2026-10-21.
- Offen/nicht gemessen: Kimi `low` im Full-Design; Sonnet `low`/`medium` im
  Full-Design; DeepSeek `none` im Full-Design. Diese Tests kosten mehr, als sie
  bei den aktuellen Preisen sparen — erst bei Modellwechsel wieder messen.
- GLM 5.3: Route `Unsupported HTTP/1 subversion` (2×); nicht Teil der Runde.

## Belege

- `rat/2026-10-21-rat-sources-schema.md`: Messungen, Kritikrunden,
  Reasoning-Matrix, GLM-Ausfall.
- `docs/surveys/survey-2026-10-21-sources-schema-archaeologie.md`,
  `...-wissenschaft.md`: der Baum- und Wissenschaftskontext.
