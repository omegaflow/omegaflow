# Rat-Formular — das lebende Register (seit 2026-10-17)

**Was das ist.** **EIN** Register für alle Rat-Fragen — nicht sechs Formulare. Je Frage **eine
Zeile**: die Sitze, das Verdikt, die **Route an die Linie**, der **KPI** (wer trug). Rohantworten
und Sammlung liegen daneben (`state/stimmen/<datum>-…`).
**Der Weg** (Sitze, Routen, `reasoning_effort`, Ablage): `_rat-anweisung.md`.
**Wer fährt:** der Operator — `/rat` (Command), Sitze je Thema. Kein automatischer Lauf.

---

## Die Fragen

| # | Datum | Thema | Sitze | Stand | Verdikt | → Route | KPI (trug) |
|---|---|---|---|---|---|---|---|
| 1 | 2026-10-15 | China-Planetendaten | 13 Frontier | **entschieden** | `descoped` 1/2/3/4/6 · `pending` 5 · kein Send | Operator-Queue (geklärt) | — |
| 2 | 2026-10-15 | Netz-Fusion + Platzierung | 13 Frontier | **entschieden** | Min-Fusion geschlagen | Linien (Verdikt) | — |
| 3 | 2026-10-15 | Venus-Bild-Form | 13 Frontier | **entschieden** | Verdikt steht | — | — |
| 4 | 2026-10-15 | Frühwarnkette F2 · Aurora F3 · Schumann F4 · Null-Form F5 · Blocklänge F6 · Aktuator F7 | 11 unabhängig | **entschieden** | `rat-fragenpaket-verdikt.md` | F2→River · F3→Sensory · F4→Mycelium · F5→River · F6→Mountain · F7→River | — |
| 5 | 2026-10-17 | R1 Bindung · R2 exposom · R3 Concurrency · R4 Orphan | 4 API + 2 UI | **entschieden** | `2026-10-17-rat-offene-fragen.md` | R1→River · R2/R3→Mycelium · R4→Sensory | — |
| 6 | 2026-10-17 | Acht Fäden + WGLC | 5 | **entschieden** | `2026-10-17-rat-acht-faeden.md` | →Mountain | — |
| 7 | 2026-10-17 | M1 MODVOLC · M2 dropped-gate | 4 API + 2 UI | **entschieden** | `2026-10-17-rat-modvolc-dropped-gate.md` | M1→Sensory · M2→Mycelium | — |
| 8 | 2026-10-17 | DE-2 Ion↔Neutral TE | 4 API + 2 UI | **entschieden** | `2026-10-17-rat-de2-ion-neutral.md` | →Mountain | Fugu/Lumo schärfsten Riss |
| 9 | 2026-10-17 | Die Axiome (channel_ref-Ersatz) | 8 (6 API + 2 UI) | **entschieden** | `2026-10-17-rat-axiome2.md` | → AGENTS (Leitstelle trägt ein) | Opus·Kimi tief · Mistral·GLM·DeepSeek struktur · Fugu·Lumo messen |
| 10 | 2026-10-17 | Die minimale Beisitzer-Auswahl | — | **offen** | — | — | — |

*(Die Spalte KPI wird ab Frage 5 geführt — ab hier wird je Rat gemessen, wer trug.)*

---

## Das Verdikt je Frage (Kurzform)

- **#9 Axiome:** Der `channel_ref`-Ersatz ist die **Form der `field`-Messzeile** — `field <name>
  <name> <KERNEL> <KRAFT> <EINHEIT>` — nicht `witness` (28×), nicht `channel` (13×). 8/8.
- **#10 minimale Auswahl:** **Fugu + Lumo (frei) + Opus oder Kimi (Tiefe) + Mistral (billig)** ≈ $0,06.
  Regel: Sitze, die **nur bestätigen**, sitzen die nächste Runde aus.

## Die Routing-Regel

Ein Verdikt geht **an die Linie** per `bin/task_dispatch <linie> "<aufgabe>" <stelle>…` → `post/out/`
— die Linie liest ihren Dispatch beim nächsten Pass. **Die Spalte `→ Route` nennt das Ziel; der
Brief trägt die Aufgabe.** Bleibt der nächste Akt beim Operator (Send/Antrag/CDN), steht er in der
**Operator-Queue**, nicht als Linien-Route.
