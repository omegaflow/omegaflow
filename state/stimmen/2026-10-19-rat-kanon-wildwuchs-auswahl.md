# Schwarm-Auswahl — R-2026-10-19-a (gezogen 2026-10-19)

**Regel (Operator-Wort 2026-10-19, zweite Fassung):** Drei Blöcke zu fünf — **5 Schwarm (Maschine) ·
5 UI ohne Login (Maschine, davon 4 + Fugu) · 5 Frontier mit Login (Operator)**. Gesamt 15 Stimmen.
**Kein Paid** — DeepSeek/Mistral-API bleiben unberührt; Mistral Vibe ist ein UI-Sitz, kein API-Aufruf.

## Schwarm: 5 aus 13 (Zufall, `shuf -n 5`)

Quelle: `state/zustand/llm-gateway-alive.tsv`, frisch geschrieben 2026-10-19 09:54:04 (13 Sitze).

| # | Sitz |
|---|---|
| 1 | `kenari/nemotron-3-nano-omni-30b-a3b-reasoning:free` |
| 2 | `openrouter/nvidia/nemotron-3-ultra-550b-a55b:free` |
| 3 | `kilo/nvidia/nemotron-3-ultra-550b-a55b:free` |
| 4 | `ovh/Qwen3Guard-Gen-0.6B` |
| 5 | `kilo/xingchen-agi/xing4.0-29b-a4b:free` |

**Ersatz-Regel (Operator-Wort 2026-10-19):** Ein untauglicher Sitz wird **ersetzt**, aus derselben
gezogenen Liste, mit derselben Methode (`shuf` ohne Zurücklegen über die verbleibenden Sitze) —
und der Ersatz wird **hier veröffentlicht**, damit „ersetzt" nicht „ausgesucht" heißt.

**Bekannter Untauglicher:** Sitz 4 `ovh/Qwen3Guard-Gen-0.6B` ist ein Sicherheitsklassifikator,
kein Urteilsträger. Er läuft trotzdem **einmal** mit (Messung statt Annahme). Liefert er kein
Verdikt, ersetzt ihn der erste Nächste aus der Restliste.

## Block 2 — UI ohne Login, Maschine fährt (4 + Fugu)

| # | Sitz | Composer (Roster) | Max je Sitz |
|---|---|---|---|
| 1 | Perplexity | `div[contenteditable]` (Enter) | Modell-Selektor; Suche immer an |
| 2 | Duck.ai | `textarea[placeholder="Alles privat fragen"]` | **Begründungsmodus** (Enter sendet **nicht** — Senden-Button) |
| 3 | Qwen | `textarea[placeholder*="Qwen fragen"]` | **Web-Suche** an + **Denken** + Modell `Qwen3.8-Max` |
| 4 | MiniMax | `div[contenteditable]` („Enter message…") | „Search"; Modell MaxHermes/MaxClaw |
| 5 | Sakana Fugu | `textarea[placeholder="Ask anything"]` | Namazu, rechnet von sich; **langsam, zuerst stellen** |

**Zugang (korrigiert 2026-10-19):** Die Login-Tabelle im Roster ist **falsch** — sie führt Duck,
MiniMax und Kimi nicht als Login-Seats. `curl` gemessen: `duck.ai` 200 · `agent.minimax.io` 200 ·
`kimi.ai` **302 (Redirect, Konto)**. Duck.ai, MiniMax und Kimi sind **eingeloggt über das
Operator-Profil** (derselbe lebende Chrome wie alle anderen Seats) — das ist kein offener Zugang,
sondern der Operator-Browser, den die Maschine bedient. Mistral **Vibe** steht in keiner
Bedien-Tabelle des Rosters → hier **nicht** gesetzt, weil unbelegt; das Gate (Composer/Senden/Max)
ist Pflicht vor dem Senden.

## Block 3 — Frontier mit Login, **Operator** fährt

| # | Sitz | Warum Frontier | Max je Sitz |
|---|---|---|---|
| 1 | Claude | ProseMirror, 5-h-Limit | Sonnet 5.5 Extra hoch |
| 2 | Kimi | Login-Wall (Google/Telefon), K3 | — |
| 3 | Gemini Notebook | Dokument-Bindung, kein freier Composer | — |
| 4 | SciSpace | Sign-up-Wall, Research-Task | — |
| 5 | *(offen — Operator wählt den fünften aus `ui-seats.md`)* | | |

**Offene Stelle:** `ui-seats.md` führt nur vier echte Login-Seats. Der fünfte wird **nicht erfunden**
— der Operator nennt ihn beim Fahren.

**Kein Paid.** Wird das Feld uneinig und die Frage verlangt Autorität, ist der nächste Schritt Ask
an den Operator — nicht ein stiller Paid-Aufruf (`bin/rat_guard`).
