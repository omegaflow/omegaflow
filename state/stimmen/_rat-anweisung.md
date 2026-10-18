# Rat-Anweisung — wie der Rat zu fahren ist (gemessen 2026-10-17)

**Wann lesen:** nur beim Fahren einer Rat-Frage — nicht in jedem Turn.
**Wer entscheidet:** **der Operator** — welche Sitze, wann gefahren wird. Die Maschine bereitet vor.
Kein automatischer Lauf, kein Executable. Der Operator ist dabei.
**Warum hier:** jede Linie, die einen Rat baut, öffnet `state/stimmen/` (wo das Formular liegt) und
findet diese Datei daneben. **Der kanonische Weg steht hier — es gibt keinen anderen.**

---

## 1 — Die Sitze, Routen und Eigenschaften (gemessen an den APIs, 2026-10-17)

**Die Leiter (Operator-Wort, Commits `91682fc3d` · `039a6ee55` · `2266bf2ba` · `d72e2cdc9`):**
`free > Mistral inklusive > DeepSeek paid > Mistral paid`.
**DeepInfra nur für (a) PII, wo Mistral nicht ausreicht, und (b) Frontier (Kimi-Klasse) — nie Linienarbeit.**
PII läuft zuerst über Mistral.

| Rang | Sitz | Route / Modell | Grenzkosten |
|---|---|---|---|
| **1** | `opencode/deepseek-v4.1-flash-free#low` | OpenCode frei — Fork bestanden | **$0** |
| **2** | `opencode/glm-5.3-flash-free#low` — **nur Mechanik** | OpenCode frei | **$0** |
| **3** | `mistral/mistral-large-4` · `mistral/zai-glm-5-3` | **Mistral inklusive** (7,27 € Rest) | **€0** bis erschöpft |
| **4** | `deepseek/deepseek-v4-pro` | DeepSeek paid (≈$90) | voll |
| **5** | `mistral/*` nach Erschöpfung | Mistral paid (20 €) | voll |
| **—** | `deepinfra/…` | **reserviert** | nur PII + Frontier |

**Live-Beleg:** `2026-10-18-netto-rollen.md` (Leiter, Töpfe) · `2026-10-18-modell-inventur-live.md`
(Gateway-Neustart: die 401er waren der **eingefrorene Prozess** — danach mistral 54 / deepinfra 180 /
deepseek 2 live; `mistral/zai-glm-5-3` trägt `low`, `finish=stop`).
**Die alte 17.10.-Tabelle ist überholt** — sie führte alle Paid-Sitze über DeepInfra.

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

## 3c — Mechanik läuft auf einem freien Subagenten (gemessen 2026-10-18)

**Regel:** Offen-Stand lesen (Sessionanfang) und Session-Zustand (Ende) darf ein **freier
Subagent** erledigen — er ist billiger und war im Test **zahlentreuer als meine Anweisung**.

**Der Kanal entscheidet:**
- **Gateway (Chat-API)** = nur Text. Kein Modell dort kann Shell. Ein Auftrag „führe aus" endet
  in *„I cannot execute shell commands"* — korrekt und nutzlos.
- **OpenCode-Subagent** (`general` + `model:`) = hat die Shell. Das ist der Weg.

**Freie Modelle als OpenCode-Provider (nicht im Gateway):** `opencode/deepseek-v4.1-flash-free` ·
`opencode/glm-5.3-flash-free` · `opencode/mimo-v2.6-flash-free` · `opencode/beam-free` ·
`opencode/longcat-2.5-preview-free` · `opencode/space-bunny-free` ·
`opencode/nemotron-3.5-super-vl-free`.

**Grenze, gemessen:** ein „n Pfade offen" ohne Zeitstempel ist eine Momentaufnahme, kein Zustand —
die Zahl wuchs während des Tests 10→13→15. Wer sie berichtet, nennt den Zeitpunkt.

## 4 — Die Ablage

- **Fragebogen:** `state/stimmen/<datum>-<thema>-formular.md` (oder `/tmp/opencode/<thema>.txt`).
- **Rohantwort je Sitz:** `state/stimmen/<datum>-<thema>-<sitz>.txt`.
- **Sammlung (Bilanz + Risse):** `state/stimmen/<datum>-rat-<thema>.md`.
- **UI-Antworten:** `document.body.innerText`, ab Antwortbeginn bis zum Footer („Generated content may
  be inaccurate" / „Lumo kann Fehler machen").
- **`finish` je Sitz prüfen.** `length` = abgeschnitten → mit `low` wiederholen; `reasoning_content`
  rettet, wenn `content` leer ist.

## 5 — Kosten je Frage (live: Leiter statt Listenpreis)

Die Grenzkosten folgen der Leiter, nicht dem Listenpreis (`2026-10-18-netto-rollen.md`):

| Rang | Sitz | Frage (≈2,5 k out) |
|---|---|---|
| 1–2 | OpenCode frei (`…-free#low`) | **$0** |
| 3 | Mistral inklusive (`mistral-large-4`, `zai-glm-5-3`) | **€0** bis 7,27 € erschöpft (~1 690 Calls) |
| 4 | DeepSeek paid (`deepseek-v4-pro`) | ≈ $0,008 |
| 5 | Mistral paid | ≈ €0,0043 |
| — | DeepInfra | **reserviert** (PII/Frontier) |

**Cap 0,15 USD/Burn.** DeepInfra wird für Linienarbeit **nicht** angefasst.

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
