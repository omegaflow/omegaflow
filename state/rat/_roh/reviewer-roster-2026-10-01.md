# Reviewer-Roster — gemessen 2026-10-01 (aufgeräumt)

Zweck: die wenigen **gemessenen** Stimmen für die Pre-Review-Leiter — kein Katalog.

## Recherche-Trio (Operator-Wort 2026-10-02): GLM · Claude · Kimi im Chat

„glm 5.3 max mit deep search ist echt gut das sollten wir intensiver nutzen" und
„glm claude und kimi im chat liefern die besten recherchergebnisse" — Operator
(Session, Future 169). Konsequenz für die Leiter:

- **Die drei Chat-UIs sind der erste Kanal für Tiefen-Recherche** (Quellen-/Routen-/
  Format-Fragen) — in dieser Ordnung:
  1. **`chat.z.ai`** — GLM-5.3 (nicht `glm-5.3-flash`) **+ Deep Think Max + Deep Search/
     Tiefensuche**. Gemessen 2026-10-01 ✅.
  2. **`claude.ai`** — Sonnet 5.5, Aufwand Extra hoch. ✅ (vollständigste Stimme im Benchmark).
     **Fallback wenn `claude.ai` am 5-h-Nachrichtenlimit ist: `arena.ai` Search-Direct**
     — `https://arena.ai/search/direct?model_a=claude-sonnet-5-search` (Operator-Wort
     2026-10-02: „für sonnet 5.5 search geht auch immer arena"). Hier **pinnt der URL-Param**
     (Modell-Button zeigt `claude-sonnet-5-search`); auf `/text/direct` pinnt `?model_a=`
     dagegen **nicht** (Default Max Router) — dort Modell per Selector.
  3. **Kimi K3** — erreichbar **nur über `tryingopen.com`** (Operator-Wort 2026-10-02:
     auf `kimi.ai` geht K3 **nicht**, dort läuft nur ein schnelles ~2.6). **Prompt-Limit
     4000 Zeichen**; „Provider may log" → nur haus-freie Recherche.
- Die `opencode`-API-Flotte läuft als **Breite/Parität** daneben, nicht als Ersatz;
  der API-Kanal bleibt für Masse/Reproduzierbarkeit, die drei UIs für die scharfe
  Recherche.
- **Route:** laufender Chat `chat.z.ai/c/<id>` ist bereits GLM-5.3 (Button-Id
  `model-selector-glm-5_3-button`); die Modellwahl ist **im laufenden Chat disabled** —
  Max/Search-Aufwand **je neuem Chat** wählen. Lange Prompts hängen transient, kurze
  laufen (Messung 2026-10-01).
- **Vorbehalt:** einzelne Kanäle „Provider may log" — für Haus-/Grundsatzfragen
  Nicht-Logger bevorzugen; Haus-Daten (`state/`, `.secrets.local`) gehen nie an einen
  zweiten Cloud-Dienst.


## opencode (reproduzierbar; ein Agent-Lauf ~41k Kontext = TPM-Untergrenze)

| Modell | Familie | Befund |
|---|---|---|
| `nvidia/z-ai/glm-5.3` | zai/GLM | ✅ PONG |
| `google/gemini-3.8-flash` · `gemini-3.5-flash` | Google | ✅ PONG (2.5er pensioniert) |
| `deepseek/deepseek-flash` · `deepseek-v4-pro` | DeepSeek | ✅ PONG |
| `nvidia/nvidia/nemotron-3-ultra-550b-a55b` · `-super-120b-a12b` | NVIDIA | ✅ PONG |
| `openrouter/qwen/qwen3.8-27b:free` | Qwen | ✅ PONG |
| `nvidia/moonshotai/kimi-k3` | Moonshot | ✅ PONG im Retry |

**Provider-Stand:** 7 von 8 tragen — `nvidia`, `deepseek`, `google`, `openrouter`,
`kenari`, `kilo`, `cloudflare-workers-ai` (nach Key-Fix: `CLOUDFLARE_WORKERS_TOKEN`
statt des toten `CLOUDFLARE_API_TOKEN`; `@cf/qwen/…` ✅, `@cf/openai/gpt-oss-…` ✗
Schema-Brücke). Kaputt bleibt `groq` (TPM 8k < 41k). „:free" ist keine Garantie —
Free-Tiers rate-limiten (429).

## API (`state/stimmen/stimme.sh`)

zai `glm-4.7-flash` ✅ · zai `glm-4.5-flash` ✅ · gemini `gemini-2.5-flash` ✅ ·
mistral `mistral-small-latest` 429.

## UI (chrome-devtools, je Akt Operator-Wort)

`claude.ai` (Sonnet 5.5, Aufwand Extra hoch) ✅ · `kimi.ai` — schnell, aber **kein K3**
(nur ~2.6; K3 nur über `tryingopen.com`, **4000-Zeichen-Limit**) · `chat.z.ai` (GLM-5.3,
Deep Think Max) ✅ — **UI-Alternative** zur GLM-Familie (gleiche
Familie wie opencode `glm-5.3`; langer Prompt hing transient, kurzer läuft) ·
`arena.ai` **Direct mit gepinntem Modell** ✅ (`/text/direct?model_a=<id>`, z. B.
`grok-4.20-beta-0309-reasoning` → **PONG**; nennt das Modell). **2026-10-02 korrigiert:**
auf `/text/direct` pinnt der URL-Param `?model_a=` **nicht zuverlässig** (Default = Max
Router) — Modell über den Selector wählen (Sonnet-5.5-Option = `claude-sonnet-5-5-high`).
**Sonnet-Search-Route:** `https://arena.ai/search/direct?model_a=claude-sonnet-5-search`
— hier pinnt der Param (Button `claude-sonnet-5-search`).
**130 Modelle** gemessen
(Stand 2026-10-01); benannte Frontier-Familien: **xAI/Grok** (4.2-beta, 4.3, 4.5,
4.6-high/medium/low) · **OpenAI/GPT** (5.1–5.5, gpt-oss) · **Anthropic/Sonnet**
(sonnet-5-5-high, haiku-4-5) · **Google/Gemini** (3.x, 2.5) — **kein Opus** in der
Liste. Die anonymen Modi (Max/Battle) zählen **nicht**. Eingabe = Composer tippen +
Senden klicken (CDP), **exakt wie ein Mensch** — **keine** Bridge, kein Token-Minting,
kein reCAPTCHA-Bypass (die `arena2api`-Bridge bleibt ausgeschlossen). Je Akt
Operator-Wort (ToS-Risiko), Route nicht immer stabil.

## Frontier ohne Login (2026-10-01, gemessen)

Zwei Web-Chat-Routen tragen ohne API-Key: `chat.together.ai/?model=<id>` (GLM-5.3,
Kimi K3, DeepSeek V4 Pro/Flash …) und `tryingopen.com` (GLM 5.3/Kimi K3 → „Answered by
Z.ai"; 20/h-Limit). **Kimi K3 nur hier** (nicht auf `kimi.ai`), **Prompts ≤ 4000 Zeichen**;
„Provider may log". Rezept und Provider-Liste: `state/stimmen/2026-10-01_frontier-kanaele-ohne-login.md`.
**Routing:** erste GLM-5.3-Route = `chat.z.ai` (UI); der `nvidia`-Endpunkt hängt (240 s/200 s leer),
Together-API-Key ungültig (401, `opencode auth login together` nötig). Together/tryingopen sind
Parität 2/3; die hängenden UI-Routen entfallen.

## Benchmark — Nützlichkeit der Stimmen (2026-10-01)

**Aufgabe (aktuelle Review-Frage):** die `fam`/Null-Frage aus dem enso-gic-Prompt —
„Was kontrolliert das Plug-in-Runden-Maximum `fam` unter der realen DGP, und was ist
die korrekte Konstruktion?" **Ground-Truth (Baum-verifiziert, GIC-Papier §3.2/§6):**
`fam` kontrolliert **kein benanntes Niveau**; real Größenordnung **10⁻¹**; Konstruktion
= Westfall–Young max-T / Block-Bootstrap, Dedup auf 6, B≥10⁴, GPD-Tail, TE-CI separat.
**Vorbehalt:** Der Prompt **nannte** den Defekt — der Test misst also
**Konstruktionsqualität bei bekanntem Defekt**, nicht blinde Entdeckung.

| Stimme (Kanal) | Score /5 | Befund |
|---|---|---|
| **claude** (UI, Anthropic) | **5** | WY max-T, rank-gauss, Dedup per Index-Hash, Seed=hash(round/station/year), B=9999, GPD-Tail, BCa-CI, real 10⁻¹ |
| **kimi** (UI, Moonshot) | **5** | multivariate Block-Bootstrap, max-T, Dedup, B=10⁴, GEV, Bias-Korrektur, real 10⁻¹ (10–50 %) |
| **glm-5.3** (opencode) | 4.5 | Dedup, Joint-max-T mit counter-based Seeds, IAAFT/Shifts, B≥999/10⁴, real ~5–15 % |
| **gemini-2.5-flash** (API) | 4 | AAFT/IAAFT, B 1000–5000, Dedup, independent seeds, real 50–90 % |
| **nemotron-ultra** (opencode) | 4 | WY, Block-Perm, Dedup, B=10⁴ — **unterschätzt** aber die reale DGP (nennt „9–12 %/3–4 %" statt 10⁻¹) |
| **glm-4.7-flash** (API) | 3.5 | WY, Dedup, Seed-Riss — Niveau „undefined", verfehlt die Größenordnung |
| **glm-4.5-flash** (API) | 2.5 | Block-Bootstrap ja, aber „lag-0/1 als 6 distinct (no pooling)" **falsch** und Niveau 5–10 % unterschätzt |

**Ergebnis:** die Stimmen-Klasse ist **nützlich für Konstruktion-bei-bekanntem-Defekt**;
die UI-Stimmen (claude, kimi) sind am vollständigsten, `glm-5.3` (opencode) ist die
billigste vollständige. Schwächste: `glm-4.5-flash` (falscher Dedup-Rat),
`nemotron-ultra` (unterschätzt die reale Fehlerrate). **Der schärfere Test fehlt noch:**
blinde Entdeckung — der Prompt darf den Defekt **nicht** nennen. Als Auftrag an die
nächste River-Session: dieselbe Frage ohne „Named open defect"-Absatz, Ground-Truth
geheim halten, Score = wer den Riss **selbst findet**.

## Befund — Einhellige Zustimmung ist kein Beweis (gemessen 2026-10-05)

Fünf Thesen zur Gesetz/Modell-Trennung des Archivar-Datenlayers wurden dem
Recherche-Trio gleichzeitig vorgelegt (`chat.z.ai` GLM-5.3, `claude.ai` Sonnet 5.5,
`tryingopen.com` Kimi K3). Alle drei stimmten allen fünf Thesen zu — einhellig und
flüssig. Der Baum widersprach zwei davon: die Certainty-Formel ist
`e^(−max(0,|Δt|−d/c)/ttl)`, nicht `e^(−|Δt|/ttl)`, und die Gewissheit atmet durch
`ttl`, nicht durch `τ` (`src/archivar/spatial.rs:743`). Keine der drei Stimmen fand
den Unterschied. Der Rat (Mycelium) nannte es das Netz-Echo: die Gesetz/Modell-
Dichotomie hat ein Trainingsdaten-Nachbarschaftsband, und flüssige Zustimmung genau
dort ist Gradient, nicht Messung. **Regel:** ein einhelliges Stimmen-Verdikt über
eine abstrakte Dichotomie wird als `unverifiziert` getragen, nie als Befund; die
Messung am Baum entscheidet.

## Nicht weiterverfolgt

Broker (UnoRouter paid, NaraRouter domain-gated, Top Tools AI, ISH, ChatWave 530,
Empero maintenance) sind für die Leiter **nicht nötig** — je eine Familie,
Account/Geld/ToS, kein Verifizierer. Die **Arena-Bridge** (`arena2api`, reCAPTCHA-
Bypass) bleibt ausgeschlossen; **Arena selbst** fährt als UI-Route (Direct, s.o.).
Ein freier Opus-API-Key existiert nicht; Opus frei gibt es nur als Chat
(`ish.chat`, Sign-in).
