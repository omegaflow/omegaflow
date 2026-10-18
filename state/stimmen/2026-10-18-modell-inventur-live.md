# Modell-Inventur — konfiguriert vs. live (2026-10-18)

**Anlass:** „Wir müssen exakt wissen, welche Modelle es wirklich gibt, sonst ist alles Raterei."
Zwei Messungen: (1) **konfiguriert** — `opencode models`; (2) **live** — der Gateway holt je Anbieter
`GET <base>/models` (`llm_gateway.rs:2139–2186`) und protokolliert Code + Anzahl.

## 1 — Live: was die Anbieter wirklich antworten

Aus dem Gateway-stderr (`/proc/<pid>/fd/2`, nach einem frischen `/v1/models`-Call):

| Anbieter | HTTP | IDs |
|---|---|---|
| openrouter | 200 | 471 |
| kilo | 200 | 402 |
| unorouter | 200 | 270–271 |
| hf | 200 | 136 |
| kenari | 200 | 111 |
| nim | 200 | 81 |
| cf | 200 | 71 |
| llm7 | 200 | 67 |
| google | 200 | 63 |
| ovh | 200 | 22 |
| ollama | 200 | 22 |
| zai | 200 | 11 |
| groq | 200 | 11 |
| sambanova | 200 | 6 |
| inception | 200 | 3 |
| pollinations | 200 | 1 |
| horde | 404 | 0 |
| **deepinfra** | **401** | **0** |
| **deepseek** | **401** | **0** |
| **mistral** | **401** | **0** |

**Alle drei bezahlten APIs liefern auf `/models` 401.** Das heißt: ihre Modellliste ist live
**nicht lesbar**, und der Gateway meldet sie als „0 IDs". Das ist der Grund, warum GLM über Mistral
im Live-Bild fehlte — nicht, weil es das Modell nicht gibt.

## 2 — Konfiguriert: was `opencode models` wirklich kennt

| Provider | Modelle |
|---|---|
| **deepinfra** | **56** |
| **mistral** | **18** |
| **opencode** (frei) | **10** |
| **deepseek** | **2** |

### Mistral (18)
`codestral-2508` · `codestral-latest` · `ministral-14b-2512` · `ministral-3b-2512` · `ministral-3b-latest` ·
`ministral-8b-2512` · `ministral-8b-latest` · `mistral-embed` · `mistral-large-2512` · **`mistral-large-4`** ·
`mistral-large-latest` · `mistral-medium-2604` · `mistral-medium-latest` · `mistral-small-2603` ·
`mistral-small-latest` · `voxtral-small-2507` · `voxtral-small-latest` · **`zai-glm-5-3`**

→ **Mistral führt GLM — aber `zai-glm-5-3` (voll), nicht `GLM-5.3-Flash`.**
Die Behauptung in `_rat-anweisung.md:24` ist damit **bestätigt** (nicht widerlegt, wie ich zuerst sagte);
der **Katalog** `mistral.json` enthält die ID nur nicht — Katalog ≠ Konfiguration.

### DeepSeek (2)
`deepseek/deepseek-flash` · **`deepseek/deepseek-v4-pro`**

### OpenCode frei (10)
`beam-free` · `big-pickle` · **`deepseek-v4.1-flash-free`** · **`glm-5.3-flash-free`** ·
`longcat-2.5-preview-free` · `mimo-v2.6-flash-free` · `nemotron-3-ultra-free` ·
`nemotron-3.5-lightning-free` · `nemotron-3.5-super-vl-free` · `space-bunny-free`

→ Du hattest **5.3**, nicht 5.2: es ist **`opencode/glm-5.3-flash-free`**.

### DeepInfra — GLM- und DeepSeek-Reihe (Auszug aus 56)
`zai-org/GLM-4.6` · `zai-org/GLM-4.7` · **`zai-org/GLM-5.2`** · **`zai-org/GLM-5.3`** · **`zai-org/GLM-5.3-Flash`** ·
`deepseek-ai/DeepSeek-V4-Flash` · `…-V4-Flash-0731` · `…-V4-Flash-Vision-Exp` · `…-V4-Pro` · `…-V4-Pro-0813` ·
**`…-V4.1-Flash`** · **`…-V4.1-Flash-1017`**

## 3 — Der Widerspruch, der zählt

`deepseek/deepseek-v4-pro` **antwortet auf Chat** (R-h-Lauf, `finish=stop`, 1193 out-tok) — aber
`GET /models` gibt **401**. Dasselbe Muster bei deepinfra (Chat 402) und mistral (401 expired).

**Also:** Der Live-`/models`-Abruf des Gateways ist für die drei bezahlten Routen **kaputt oder
falsch authentifiziert**, obwohl Chat teilweise funktioniert. Die **konfigurierte Liste ist heute
die belastbare Quelle**, nicht `/v1/models`.

Das ist ein eigener Riss: `catalog()` und `chat_completions()` benutzen offenbar **nicht denselben
Key-/Base-Pfad**. Zu prüfen in `key_for` (`:230–244`) und `effective_base`.

## 4 — Was daraus für die Rollen folgt (Fakten, keine Wahl)

| Kandidat | Wo es **wirklich** existiert | Live-Chat |
|---|---|---|
| **GLM-5.3-Flash** | `deepinfra/zai-org/GLM-5.3-Flash` · `opencode/glm-5.3-flash-free` | deepinfra: 402 · opencode: eigener Provider |
| **GLM-5.3 voll** | `deepinfra/…GLM-5.3` · `mistral/zai-glm-5-3` | DI: live `#low` getragen · Mistral: Key expired |
| **DeepSeek V4.1 Flash** | `deepinfra/…DeepSeek-V4.1-Flash(-1017)` · `opencode/deepseek-v4.1-flash-free` | DI: 402 · opencode: eigener Provider |
| **DeepSeek V4 Pro** | `deepseek/deepseek-v4-pro` · `deepinfra/…V4-Pro` | deepseek: **funktioniert** |
| **Mistral Large 4** | `mistral/mistral-large-4` | Key expired (401) |

## 5 — Die drei offenen Messungen (unverändert)

1. **Keys:** deepinfra, deepseek, mistral liefern auf `/models` 401. Mindestens **Mistral** ist
   nachweislich abgelaufen (Chat: `{"detail":"Your API key expired on 2026-10-17."}`). Die anderen
   beiden: Chat teils ok, `/models` 401 → **Key-/Pfad-Riss**, nicht zwingend abgelaufen.
2. **Effort:** `reasoning_effort` für GLM-5.3-Flash (DI) und DeepSeek-V4.1-Flash (DI) ist ungemessen.
3. **Preis:** Für die 32 Modelle mit Tokens, aber ohne Katalogpreis, fehlt die Netto-Aussage.

## Belege

`opencode models` (56/18/10/2) · Gateway-stderr `/proc/2986373/fd/2` (Live-Codes) ·
`llm_gateway.rs:2139–2186` · `2026-10-18-kosten-kontingente.md`

## 6 — Mistral-Konsole (Operator-Ablesung 2026-10-18): zwei Kanäle, drei neue Modelle

Die Konsole zählt **getrennt nach Kanal** — das erklärt, warum die API-Liste und das Live-Bild
nicht zusammenpassten:

**Kanal „Completion" (= API, abrechenbar, zählt aufs Kontingent):**
`mistral-large-2512` **136** · `mistral-small-2603` **90** · `mistral-large-4` **32** ·
`mistral-small-latest` 10 · `mistral-medium-3-5` 3 · `codestral-2508` 2 · `codestral-latest` 1 ·
`ministral-3b/8b/14b-*` je 1 · `mistral-code-fim-latest` 1 · `mistral-code-latest` 1 ·
`voxtral-small-2507/-latest` je 1 · **`zai-glm-5-3` 1** → **Σ ≈ 285 abrechenbare Calls = 1,23 €**

**Kanal „Vibe Work" / „Connectors" (Mistral-Produkt, NICHT abrechenbar, Abo inklusive):**
**`glm-5-latest-short` 137** · `ministral-8b-2512` 81 · `mistral-moderation-2603-internal` 60 ·
`web_search` 60 · (`mistral-small-latest` 3 über Vibe Code)

**Drei Befunde:**

1. **`zai-glm-5-3` ist API-real:** 1 Completion-Call, abrechenbar. GLM-5.3 **voll** ist über die
   Mistral-API **wirklich verfügbar** (nicht Flash — Flash taucht nirgends auf).
2. **`glm-5-latest-short` existiert — aber nicht in unserer API-Liste.** Es läuft über **Vibe Work**
   (Mistral-eigenes Produkt), **nicht abrechenbar**, 137 Calls. Das ist ein **Produkt-Kanal**, kein
   Gateway-Kanal: mit dem API-Key nicht erreichbar, in `opencode models` nicht vorhanden.
   → Ein **GLM-5-Modell, das im Abo inklusive ist** — als **UI-/Produkt-Sitz** vergleichbar mit
   Fugu/Lumo, nicht als API-Sitz.
3. **Die Kostenrechnung kalibriert sich selbst:** ≈ 285 abrechenbare Calls = 1,23 € →
   **≈ 0,0043 € je Call** — exakt die gerechneten 0,0043 €/Frage für Mistral Large 4.
   Die Kontingent-Rechnung trägt also.

**Konsistenz mit dem Key-Befund:** Die Konsole zeigt API-Nutzung (abrechenbar), aber der Gateway
bekommt seit 2026-10-17 **401** — der Key ist abgelaufen, das Konto nicht. Neueustrierung genügt.
