# Kosten, Kontingente, Rollenbesetzung (gemessen 2026-10-18)

**Anlass:** Operator-Wunsch — netto messen, was welches Modell kostet, welche Kontingente
effektiv tragen, ob GLM-5.3-Flash ein Partner für die Leitstelle ist, und ob getrennte Keys sinnvoll sind.
**PII-Entscheidung des Operators:** „PII-fähig" = **(a) Vertragsdeckung** — das Modell *soll* keine
Personendaten sehen, aber wenn doch, ist es vertraglich gedeckt. → GLM-5.3-Flash scheidet nicht aus.

## 1 — Zwei Korrekturen an früheren Aussagen

1. **„DeepInfra ist nicht autorisiert"** war ein **Modell**-Fehler, kein Key-Fehler: Das Log zeigt
   `deepinfra/…`-Calls, die mit **402** (Quota) antworten — nicht 401. Die 402 kamen von der
   Modell-Sperre (`User is not authorized to access this resource`), nicht vom Konto.
2. **„GLM-5.3 über Mistral"** ist **unbelegt**: `mistral.json` enthält **0** GLM-IDs.
   Der Satz in `_rat-anweisung.md:24` ist eine **Negativmessung** (Default kappt), kein Katalognachweis.

## 2 — Effektive Nettokosten (gemessen aus `llm-gateway-provenance.jsonl` × Katalog)

| Modell | Req | in | out | $/1M in | $/1M out | Netto $ | $/Antwort |
|---|---|---|---|---|---|---|---|
| ovh/Qwen3.6-27B | 10 | 368 | 6 764 | 0,47 | 3,19 | **0,02175** | 0,00218 |
| ovh/Qwen3.5-397B-A17B | 9 | 342 | 3 051 | 0,71 | 4,25 | 0,01321 | 0,00147 |
| ovh/Qwen3.8-27B | 6 | 20 608 | 346 | 0,47 | 3,19 | 0,01079 | 0,00180 |
| deepseek/deepseek-v4-pro | 1 | 114 | 170 | 1,11 | 3,33 | 0,00069 | 0,00069 |
| ovh/Llama-3_3-70B | 4 | 244 | 20 | 0,74 | 0,74 | 0,00020 | 0,00005 |
| 7× `kilo/*:free` | 37 | 1 754 | 6 818 | 0 | 0 | **0,00** | 0,00 |

**Summe gemessen: $0,0470.** Davon **52 % der Tokens in freien Modellen = $0.**

**Der Kostentreiber ist `out`, nicht `in`:** Bei Qwen3.6 sind **98,5 %** der Kosten Ausgabe-Tokens.

**`preis ungemessen`: 118 von 133 Alias-Gruppen**, davon **32 mit Tokens** — u. a. groq (1 214 Tok),
ollama (947), pollinations (1 296), **alle 12 erfolgreichen `mistral/*`-Läufe**. Für diese gibt es
**keine** Nettokosten-Aussage; das ist der größte Mess-Riss der Kostenfrage.

## 3 — Was die Kontingente tragen

| Anbieter | Rest | Preis/Frage | **Fragen** | bei 3 Runden/Tag |
|---|---|---|---|---|
| **Mistral** (Large 4) | 7,27 € (von 8,50 €) | 0,0043 € | **≈ 1 690** | **≈ 1,5 Jahre** |
| Mistral (20 € Credits) | 20,00 € | 0,0043 € | ≈ 4 651 | — |
| **DeepInfra** (V4-Pro) | $8,49 | $0,00793 | **≈ 1 070** | ≈ 1 Jahr |
| DeepInfra (Top-up) | $91,05 | $0,00793 | ≈ 11 482 | — |

**Nicht entscheidbar:** ob die **$2 814,88 „Total cost"** bezahlt oder gewährt sind — aus den
abgelesenen Zahlen nicht ableitbar. Ausdrücklich offen.

## 4 — GLM-5.3-Flash: der Preis ist der Punkt

| Modell | Route | $/1M in | $/1M out | Kontext | Reasoning |
|---|---|---|---|---|---|
| **GLM-5.3-Flash** | DeepInfra | **0,15** | **0,50** | 1 048 576 | effort **ungemessen** |
| GLM-5.3-Flash | OpenRouter | 0,0261 | 0,90 | 1 048 576 | `low/high/max` |
| **DeepSeek V4.1 Flash** | DeepInfra | 0,20 | 0,60 | 1 048 576 | ungemessen |
| GLM-5.3 voll | DeepInfra | 0,90 | 4,00 | 1 048 576 | `low` **live ✓** |
| GLM-5.3 voll | OpenRouter | 0,079 | 4,80 | 1 048 576 | `effort` |

**GLM-5.3-Flash ist billiger als DeepSeek V4.1 Flash** (out 0,50 vs 0,60) — aber sein
`reasoning_effort` ist **ungemessen**. Bei GLM ist genau das der Knackpunkt: **`#none` kappt**
(gemessen bei Kimi: `#none` wählte falsch), und **GLM voll `#low` trug** ($0,01284, richtig).
Ein Flash ohne gemessene Effort-Steuerung ist ein Risiko, kein Schnäppchen.

## 5 — Keys und Anbieter-Einstellungen

- **Getrennte Keys je Aufgabe: im Code nicht vorgesehen.** Eine `Route` kennt genau ein `key_env`
  und eine `auth_id` (`llm_gateway.rs:9–15`); Auflösung Env → `.secrets.local` → `auth.json`,
  **ein** String (`key_for`, `:230–244`).
- **Mehrere Keys je Anbieter: nicht unterstützt** (`auth_key`, `:213–228` nimmt den ersten Treffer).
  Rotation müsste man von Hand bauen.
- **DeepInfra 200 concurrent / 8 GPU je Typ: nicht im Code entscheidbar.** Der Gateway spricht nur
  `api.deepinfra.com`; GPU-Limits sind **Deployments**, nicht API — das ist die offizielle Lesart,
  im Haus-Code aber nicht belegt.
- **Rate-Limits sind real:** Mistral 13× **429**, SambaNova 8× 429, zai 18× 429 im Log.

## 6 — Die Rollenbesetzung (Messgrundlage, nicht Beschluss)

| Rolle | Kandidat | Begründung aus der Messung |
|---|---|---|
| **Leitstelle (Future)** | **GLM-5.3-Flash** oder **DeepSeek V4.1 Flash** (DeepInfra) | beide 1M Kontext, 0,15/0,50 vs 0,20/0,60 · **aber**: `effort` bei beiden ungemessen |
| **Linien** | **DeepSeek V4 Pro `#low`** | der einzige live belegte tragende Reasoning-Sitz ($0,01284, richtig gewählt) |
| **Schwarm / Volumen** | **freie Träger** (44 gemessen) | $0, 52 % der Tokens |
| **Rat-Struktur** | **Mistral Large 4** | 1 690 Fragen im Restkontingent; Key lief am 2026-10-17 ab → **neu ausstellen** |

**Die drei offenen Messungen, die diese Tabelle erst belastbar machen:**
1. `reasoning_effort` bei **GLM-5.3-Flash** und **DeepSeek V4.1 Flash** — die Fork-Frage mit `#low` fahren.
2. **Nettokosten je Antwort** für die 32 ungemessenen Alias-Gruppen (Preise aus der Anbieter-API).
3. **Mistral-Key neu ausstellen** — heute 401 zwischen 13 429ern; ohne ihn kein Rat-Sitz.

## 7 — PII (Entscheidung des Operators)

„PII-fähig" = **Vertragsdeckung**: Das Modell *soll* keine Personendaten sehen; sieht es doch welche,
trägt der Vertrag. Damit ist GLM-5.3-Flash über Mistral **kein Ausschlussgrund** — die Wahl
entscheidet der Preis und die gemessene Effort-Steuerung, nicht die Datenklasse.

## Belege

`zustand/llm-gateway-provenance.jsonl` (226 Zeilen) · `zustand/llm-gateway-catalog/*.json` ·
`2026-10-18-modell-faehigkeiten.md` · `2026-10-18-rat-gate-rat-id.md:130–147` ·
`llm_gateway.rs:9–15,213–244` · `_rat-anweisung.md:20–30,127–135`
