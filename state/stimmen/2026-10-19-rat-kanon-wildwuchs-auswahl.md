# Schwarm-Auswahl — R-2026-10-19-a (gezogen 2026-10-19)

**Regel (Operator-Wort 2026-10-19):** Schwarm-Budget schonen — **5 per Zufall** aus den lebenden
freien Sitzen, dann **5 UI-Chats** (2 Operator / 3 Maschine). Gesamt 10 Stimmen, kein Paid.

## Schwarm: 5 aus 13 (Zufall, `shuf -n 5`)

Quelle: `state/zustand/llm-gateway-alive.tsv`, frisch geschrieben 2026-10-19 09:54:04 (13 Sitze).

| # | Sitz |
|---|---|
| 1 | `kenari/nemotron-3-nano-omni-30b-a3b-reasoning:free` |
| 2 | `openrouter/nvidia/nemotron-3-ultra-550b-a55b:free` |
| 3 | `kilo/nvidia/nemotron-3-ultra-550b-a55b:free` |
| 4 | `ovh/Qwen3Guard-Gen-0.6B` |
| 5 | `kilo/xingchen-agi/xing4.0-29b-a4b:free` |

**Vorab-Riss:** Sitz 4 ist ein **Guard-Modell** (`Qwen3Guard-Gen-0.6B`, Sicherheitsklassifikator,
kein Generalist). Ist die Antwort kein Urteil, wird er im Lauf **als untauglich ausgewiesen** —
nicht ersetzt, nicht stillschweigend nachgezogen. Sonst liest sich der Zufall als Auswahl.

## UI: 5 aus dem Roster (`ui-seats.md`)

| # | Sitz | Wer | Max je Sitz |
|---|---|---|---|
| 1 | Perplexity | Operator | Deep Research |
| 2 | Claude | Operator | Sonnet 5.5 Extra hoch |
| 3 | DeepSeek Chat | Maschine | DeepThink |
| 4 | Qwen | Maschine | Denken = max |
| 5 | Gemini 3.1 Pro | Maschine | Grounding |

**Warum diese fünf:** Perplexity und Claude sind die zwei 4/4-laut-Roster-Sitze für Operator-Hand;
Maschinen-Seats brauchen einen benannten Composer + Max-Schalter (alle drei in `ui-seats.md` belegt).

**Kein Paid.** Wird der Schwarm oder die UI-Runde uneinig und die Frage verlangt Autorität, ist der
nächste Schritt Ask an den Operator — nicht ein stiller Paid-Aufruf (`bin/rat_guard`).
