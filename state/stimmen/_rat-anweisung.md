# Rat-Anweisung — wie der Rat zu fahren ist (gemessen 2026-10-17)

**Wann lesen:** nur beim Fahren einer Rat-Frage — nicht in jedem Turn.
**Wer entscheidet:** **der Operator** — welche Sitze, wann gefahren wird. Die Maschine bereitet vor.
Kein automatischer Lauf, kein Executable. Der Operator ist dabei.
**Warum hier:** jede Linie, die einen Rat baut, öffnet `state/stimmen/` (wo das Formular liegt) und
findet diese Datei daneben. **Der kanonische Weg steht hier — es gibt keinen anderen.**

---

## 1 — Die Sitze, Routen und Eigenschaften (gemessen an den APIs, 2026-10-17)

**API-Sitze** — direkt gegen die Anbieter, Keys in `~/projects/omegaflow/.secrets.local`:

| Sitz | Route | Modell-ID | Kontext | Vision | in/out per M |
|---|---|---|---|---|---|
| **Mistral Large 4** | `api.mistral.ai` | `mistral-large-4` | — | ✓ | **$0,50 / $1,50** |
| **GLM-5.3** | `api.deepinfra.com` | `zai-org/GLM-5.3` | 1 048 576 | — | $0,90 / $4,00 |
| **DeepSeek V4 Pro** | `api.deepinfra.com` | `deepseek-ai/DeepSeek-V4-Pro` | 1 048 576 | — | $1,30 / $2,60 |
| **Qwen3.8-Max** | `api.deepinfra.com` | `Qwen/Qwen3.8-Max` | 256 000 | — | $1,65 / $4,95 |
| **Kimi K3** | `api.deepinfra.com` | `moonshotai/Kimi-K3` | 1 048 576 | ✓ | $2,85 / $14,25 |
| **Opus 5.5** | `api.deepinfra.com` | `anthropic/claude-opus-5-5` | 1 000 000 | ✓ | $4,00 / $20,00 |

**Nicht über Mistral fahren:** Mistral führt zwar `zai-glm-5-3`, aber **ohne** `reasoning_effort` —
GLM kappt dort (`finish: length`, content leer). **GLM nur über DeepInfra.**
**DeepSeek direkt** (`api.deepseek.com`) ist **teurer** als DeepInfra (off-peak $0,66/$1,98 gegen
$1,30/$2,60 peak-ähnlich) — und dachte bei derselben Frage **3 406** reasoning-tokens (DeepInfra 1 233).

**UI-Sitze** (Browser-Bridge `browser.*`, dieselben Tabs wie das OpenCode-Panel) — **frei**:

| Sitz | URL | Modell im Wähler | Falle |
|---|---|---|---|
| **Sakana Fugu Max** | `chat.sakana.ai` | **„Fugu Max"** (Effort High) | **nicht „Namazu"** — das ist (`sakana-namazu`, „For Japanese") ein **anderer** Eintrag. Fugu **orchestriert** (routet auf andere Modelle, auch Namazu). Login nötig. |
| **Lumo 2.0 Max** | `lumo.proton.me/guest` | „Lumo 2.0 Max" (anonym) | Senden per **Enter** (kein Send-Button) — `browser.press` nach `focus()`. |

## 2 — Der Reasoning-Level (der größte Fehler: er wurde nie gesetzt)

**Kein früherer Rat-Lauf setzte je einen Level** — alle fuhren Default → **abgeschnittene Antworten**
(`finish: length`) bei GLM, DeepSeek, Kimi; Qwen-Timeout.

**Live gemessen (alle Stufen akzeptiert/abgelehnt):**

| Modell | erlaubte `reasoning_effort` | Default | wirken? | **so fahren** |
|---|---|---|---|---|
| **DeepSeek V4 Pro** | `none` `low` `medium` `high` `max` | `high` | **ja, stark** (27→2268 reasoning-tok) | `low` |
| **GLM-5.3** | `none` `low` `medium` `high` `max` | (max) | ja | `low` |
| **Qwen3.8-Max** | `none`…`max` | — | ja (schwach) | `low` |
| **Opus 5.5** | `none`…`max` | — | (n. einzeln gemessen) | `low` |
| **Kimi K3** | `none` `low` `high` `max` | **`max`** | ja | `low` |
| **Mistral Large 4** | **nur `none`, `high`** | — | — | **`none`** |

**`low` liefert vollständig. `high`/`max` kappt (Reasoning frisst das Budget). `none` dünnt** (Kimi
`none`: 645 out-tok, kein Verdikt-Kopf — zu wenig für eine Urteilsfrage). **Mistral kennt kein `low`.**
Body-Feld: `"reasoning_effort":"low"` (OpenAI-Form, neben `max_tokens`).
**`max_tokens` ≥ 6000** — die Antworten lagen bei 1–5 k out.

## 3 — Der Aufruf (Muster: nur die Frage, nicht der Haus-Kontext)

```sh
DK=$(grep "^DEEPINFRA_API_KEY=" ~/projects/omegaflow/.secrets.local|cut -d= -f2-|tr -d '"')
Q=$(cat /tmp/opencode/<formular>.txt)
curl -s https://api.deepinfra.com/v1/openai/chat/completions \
  -H "Content-Type: application/json" -H "Authorization: Bearer $DK" \
  -d "$(jaq -n --arg m "Qwen/Qwen3.8-Max" --arg q "$Q" \
      '{model:$m,messages:[{role:"user",content:$q}],temperature:0.2,max_tokens:6000,reasoning_effort:"low"}')" \
  > x-Qwen.json
```

## 3b — Der freie Schwarm geht VOR die Paid-Sitze (gemessen 2026-10-18)

**Regel:** Vor jeder Rat-Runde darf ein **Schwarm freier Modelle** die Frage fahren — als
**Riss-Generator**, nicht als Entscheider. Kosten $0, Laufzeit ~1 s je Modell.

```
1. Frage steht (Formular)
2. Freier Schwarm → Risse + Mindermeinung
3. Die Risse wandern INS Formular an die Paid-Sitze
4. Paid entscheidet und trägt (Urteil + Autorität)
5. Sammlung nennt BEIDES: Paid-Verdikt und Schwarm-Risse
```

**Warum:** Benchmark über drei abgeschlossene Runden (`2026-10-18-schwarm-benchmark/README.md`):
- **Sachfragen** (R-j, R-k): Schwarm = Paid-Rat, gleiche Verdikte, **schärfere Risse**.
- **Zielkonflikt** (R-i): Schwarm wägt anders (B statt A) — vertretbar, aber ohne Träger.
- In **3 von 3** Runden hat der Schwarm einen Einwand produziert, den **kein** Paid-Sitz hatte.

**Schwarm (gemessen antwortfähig):** `cf/@cf/openai/gpt-oss-120b` · `kenari/agnes-3-0-flash:free` ·
`pollinations/openai-fast` · `ollama/gemma4:31b`.
**Ausgeschlossen — HTTP 200, aber leer:** `groq/*` · `kilo/*` · `llm7/GLM-5.3-Flash`.
**Immer Inhalt prüfen:** HTTP-Erfolg ≠ Antwort.

**Grenze:** Der Schwarm **trägt nicht**. Sein Verdikt ist eine Mindermeinung, seine Risse sind
der Wert. Kein Register-Schreibakt aus einem Schwarm-Lauf.

## 4 — Die Ablage

- **Fragebogen:** `state/stimmen/<datum>-<thema>-formular.md` (oder `/tmp/opencode/<thema>.txt`).
- **Rohantwort je Sitz:** `state/stimmen/<datum>-<thema>-<sitz>.txt`.
- **Sammlung (Bilanz + Risse):** `state/stimmen/<datum>-rat-<thema>.md`.
- **UI-Antworten:** `document.body.innerText`, ab Antwortbeginn bis zum Footer („Generated content may
  be inaccurate" / „Lumo kann Fehler machen").
- **`finish` je Sitz prüfen.** `length` = abgeschnitten → mit `low` wiederholen; `reasoning_content`
  rettet, wenn `content` leer ist.

## 5 — Kosten je Frage (gemessen — nur die Frage, ~1,1 k in)

| Sitz | in | out | **Frage (≈2,5 k out)** |
|---|---|---|---|
| Mistral Large 4 | $0,50/M | $1,50/M | **~$0,004** |
| GLM-5.3 | $0,90/M | $4,00/M | ~$0,011 |
| DeepSeek V4 Pro | $1,30/M | $2,60/M | ~$0,008 |
| Qwen3.8-Max | $1,65/M | $4,95/M | ~$0,014 |
| Kimi K3 | $2,85/M | $14,25/M | ~$0,039 |
| Opus 5.5 | $4,00/M | $20,00/M | ~$0,054 |
| **Fugu Max · Lumo** | **frei** | **frei** | **$0** |

Der **Cap** ist 0,15 USD/Burn; **Mistral + GLM + DeepSeek + Qwen zusammen ≈ $0,04**.

---

**Der kanonische Weg, in einem Satz:** Frage ins Formular → Operator wählt die Sitze → je Sitz ein
direkter API-Aufruf (`reasoning_effort` gesetzt) bzw. der UI-Tab → Rohantwort + Sammlung nach
`state/stimmen/`. **Kein `rat_dispatch`, kein Gateway, kein anderer Weg.**

## 6 — Die KPI je Stimme (der Streich-Test) + die minimale Auswahl

**Nach jedem Rat messen, wer trug — der Streich-Test** (Kimi/Sonnet/Mistral, einig):

> **Eine Stimme ist tragend, wenn ihr Riss das Verdikt ändert oder stürzt, sobald man ihn streicht.**
> Sonst ist sie **bestätigend** (redundant). Kriterien: einzigartig (k=1 von n) **und** am Material
> prüfbar **und** wirksam (hat das Verdikt verändert).

**Regel:** Sitze, die **nur bestätigen**, sitzen die **nächste** Runde aus.

**Die minimale Auswahl — gemessen, aber Betreiber-Sache** (der Rat ist bei der eigenen Zusammensetzung
**befangen**: jeder hält sich für nötig — die drei API-Sitze warfen je **andere** raus):

| Rolle | Sitz | Kosten/Frage |
|---|---|---|
| Bestandsmessung (immer, frei) | **Fugu Max + Lumo 2.0** | **$0** |
| Tiefe | **Kimi K3** *oder* **Sonnet 5.5** | ~$0,014 |
| Struktur | **Mistral Large 4** | $0,004 |

**≈ $0,018** — das gemessene Minimum mit Streuung. **Opus 5.5 fällt** (4× teurer, kein Mehr-Tragen).
**Kimi vs. Sonnet:** beide trugen; Sonnet die Beweislast („ich habe die Zahl nicht nachgezählt"),
Kimi die Bestands-Homogenität. **Einer reicht.**
