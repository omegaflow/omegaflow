# Rat-Anweisung — wie der Rat zu fahren ist (gemessen 2026-10-17)

**Wann lesen:** nur beim Fahren einer Rat-Frage — nicht in jedem Turn.
**Warum hier:** jede Future-Linie, die einen Rat baut, öffnet `state/stimmen/` (wo das Formular liegt)
und findet diese Datei daneben. Kein Raten mehr.

---

## 1 — Die Sitze und ihre Routen (gemessen an den APIs, 2026-10-17)

**API-Sitze** (direkt gegen die Anbieter, Keys in `~/projects/omegaflow/.secrets.local`):

| Sitz | Route | Modell-ID | Key |
|---|---|---|---|
| GLM-5.3 | `api.deepinfra.com/v1/openai/chat/completions` | `zai-org/GLM-5.3` | `DEEPINFRA_API_KEY` |
| Qwen3.8-Max | `api.deepinfra.com/v1/openai/chat/completions` | `Qwen/Qwen3.8-Max` | `DEEPINFRA_API_KEY` |
| DeepSeek V4 Pro | **`api.deepseek.com/v1/chat/completions`** (direkt) | `deepseek-v4-pro` | `DEEPSEEK_API_KEY` |
| Mistral Large 4 | `api.mistral.ai/v1/chat/completions` | `mistral-large-4` | `MISTRAL_API_KEY` |
| **Opus 5.5** (Flagship, Reasoning) | `api.deepinfra.com/v1/openai/chat/completions` | `anthropic/claude-opus-5-5` | `DEEPINFRA_API_KEY` |
| **Kimi K3** | `api.deepinfra.com/v1/openai/chat/completions` | `moonshotai/Kimi-K3` | `DEEPINFRA_API_KEY` |

**UI-Sitze** (Browser-Bridge `browser.*`, dieselben Tabs wie das OpenCode-Panel):

| Sitz | URL | Modell im Wähler | Fallen |
|---|---|---|---|
| **Sakana Fugu Max** | `chat.sakana.ai` | **„Fugu Max"** (Effort High) | **nicht „Namazu"** wählen — das ist ein **anderer** Eintrag (`sakana-namazu`, japanisch) |
| Lumo 2.0 Max | `lumo.proton.me/guest` | „Lumo 2.0 Max" (anonym) | Senden per **Enter** (kein Send-Button) |

## 2 — Der Reasoning-Level (der größte Fehler des Tages)

**Kein einziger Rat-Lauf setzte je einen Level** — alle fuhren Default. Folge: **abgeschnittene
Antworten** (`finish: length`) bei GLM, DeepSeek, Kimi; Qwen-Timeout.

| Modell | Parameter | Werte | Default | **so fahren** |
|---|---|---|---|---|
| DeepSeek V4 Pro | `reasoning_effort` | low/high/max | high | **low** |
| Kimi K3 | `reasoning_effort` | none/low/high/max | **max** | **low** |
| Opus 5.5 | `reasoning_effort` | low… | — | **low** |
| Qwen3.8-Max | `reasoning_effort` | — | — | **low** |
| GLM-5.3 · Mistral | (reasoning-Modell) | — | — | wie geliefert |

**`low` liefert vollständig. `high`/`max` kappt (ins Reasoning). `none` dünnt** (Kimi `none`:
645 out-tok ohne Verdikt-Kopf — zu wenig für eine Urteilsfrage). **Also `low`.**
Body-Feld: `"reasoning_effort":"low"` (OpenAI-Form, direkt neben `max_tokens`).

**`max_tokens` hoch genug:** die Antworten der acht Sitze lagen bei ~1–5 k out. **6000** ist sicher.

## 3 — Der Aufruf (Muster)

```sh
DK=$(grep "^DEEPINFRA_API_KEY=" ~/projects/omegaflow/.secrets.local|cut -d= -f2-|tr -d '"')
Q=$(cat /tmp/opencode/<formular>.txt)
curl -s https://api.deepinfra.com/v1/openai/chat/completions \
  -H "Content-Type: application/json" -H "Authorization: Bearer $DK" \
  -d "$(jaq -n --arg m "Qwen/Qwen3.8-Max" --arg q "$Q" \
      '{model:$m,messages:[{role:"user",content:$q}],temperature:0.2,max_tokens:6000,reasoning_effort:"low"}')" \
  > x-Qwen.json
```

**Nur die Frage senden** — nicht den Haus-Kontext. Das Formular (3–4 kB) ist der ganze Input.
Das hält die Kosten klein (~1.100 in-Tokens) und ist der Sinn der API-Route.

## 4 — Die Ablage

- **Fragebogen:** `state/stimmen/<datum>-<thema>-formular.md` (oder `/tmp/opencode/<thema>.txt`).
- **Rohantwort je Sitz:** `state/stimmen/<datum>-<thema>-<sitz>.txt`.
- **Sammlung (Bilanz + Risse):** `state/stimmen/<datum>-rat-<thema>.md`.
- **Zwei UI-Antworten** aus dem Browser: `document.body.innerText`, ab Antwortbeginn bis zum
  Footer („Generated content may be inaccurate" / „Namazu High" / „Lumo kann Fehler machen").
- **`finish` je Sitz prüfen.** `length` = abgeschnitten → mit `low` wiederholen; `reasoning_content`
  ist der Rettungsweg, wenn `content` leer ist.

## 5 — Kosten je Frage (DeepInfra/Mistral, gemessen — nur die Frage)

| Sitz | in | out | **Frage** |
|---|---|---|---|
| Kimi K3 | $2,85/M | $14,25/M | ~$0,04 |
| Opus 5.5 | $4,00/M | $20,00/M | ~$0,054 (low) |
| GLM / Qwen / DeepSeek / Mistral | ~$0,2–1,7/M | ~$1–5/M | ~$0,02–0,08 |

Der **Cap** der Session ist **0,15 USD/Burn** — vier API-Sitze ≈ **$0,15–0,18**; **Opus 5.5 + Kimi
zusammen ≈ $0,09**.

---

**Und die Sitz-Namen-Falle, die 20 Minuten kostete:** Sakana = **Fugu Max** (`fugu-max`, Login
nötig, „Orchestrates diverse models"); „Namazu" (`sakana-namazu`) ist ein **anderer** Modell-Eintrag
im selben Wähler. Fugu **orchestriert** — es routet auf andere Modelle (auch Namazu).
