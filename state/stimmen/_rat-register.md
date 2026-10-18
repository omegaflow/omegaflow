# Rat-Register — der Index (seit 2026-10-17)

**Was das ist.** Der **Index** aller Rat-Fragen — ein **Fahrplan**, kein Archiv. Je Frage eine Zeile;
sie sagt **wo das Verdikt liegt** und **wer es bekommt** — nicht, was entschieden wurde.
**Das Verdikt selbst** steht in seiner Datei (`state/stimmen/<datum>-rat-<thema>.md`), mit
**`## An <line>`-Blöcken** am Ende.
**Der Weg** (Sitze, Routen, `reasoning_effort`, Ablage): `_rat-anweisung.md`.
**Wer fährt:** der Operator (`/rat`). Kein automatischer Lauf.

**Der Adressat wird GEZÄHLT, nicht getippt** — aus den `## An`-Blöcken der Verdikt-Datei (Fugu:
*„die Zelle `→ Route` ist `channel_ref` in neuem Gewand — ein Name, wo eine Kraft verlangt ist"*).
**Kein Verdikt ohne Adresse** (GLM).

---

## Die Zeilen

| ID | Datum | Thema | Sitze | Stand | Verdikt-Datei | Adressaten | Quittung |
|---|---|---|---|---|---|---|---|
| R-2026-10-15-a | 2026-10-15 | China-Planetendaten | 13 Frontier | entschieden | *(Operator-Queue, descoped)* | — | — |
| R-2026-10-15-b | 2026-10-15 | Netz-Fusion + Platzierung | 13 Frontier | entschieden | `2026-10-15-netzfusion-rat-verdikt.md` | (Linien) | — |
| R-2026-10-15-c | 2026-10-15 | Venus-Bild-Form | 13 Frontier | entschieden | `2026-10-15-venus-bild-rat-verdikt.md` | (Linien) | — |
| R-2026-10-15-d | 2026-10-15 | Frühwarnkette F2–F7 | 11 unabhängig | entschieden | `2026-10-15-rat-fragenpaket-verdikt.md` | F2→River · F3→Sensory · F4→Mycelium · F5→River · F6→Mountain · F7→River | — |
| R-2026-10-17-a | 2026-10-17 | R1 Bindung · R2 exposom · R3 Concurrency · R4 Orphan | 4 API + 2 UI | entschieden | `2026-10-17-rat-offene-fragen.md` | R1→River · R2/R3→Mycelium · R4→Sensory | — |
| R-2026-10-17-b | 2026-10-17 | Acht Fäden + WGLC | 5 | entschieden | `2026-10-17-rat-acht-faeden.md` | →Mountain | — |
| R-2026-10-17-c | 2026-10-17 | M1 MODVOLC · M2 dropped-gate | 4 API + 2 UI | entschieden | `2026-10-17-rat-modvolc-dropped-gate.md` | M1→Sensory · M2→Mycelium | — |
| R-2026-10-17-d | 2026-10-17 | DE-2 Ion↔Neutral TE | 4 API + 2 UI | entschieden | `2026-10-17-rat-de2-ion-neutral.md` | →Mountain | — |
| R-2026-10-17-e | 2026-10-17 | Die Axiome (channel_ref-Ersatz) | 8 (6 API + 2 UI) | entschieden | `2026-10-17-rat-axiome2.md` | →AGENTS (Leitstelle trägt ein) | — |
| R-2026-10-17-f | 2026-10-17 | Die Beisitzer-Auswahl | 5 | entschieden | `2026-10-17-rat-beisitzer.md` | →Leitstelle (KPI-Regel) | — |
| R-2026-10-17-g | 2026-10-17 | **Architektur: Form + Routing** | GLM · Sonnet · Kimi · Fugu · Lumo | **entschieden** | `2026-10-17-rat-architektur.md` | →Leitstelle (Register-Bau) | — |
| R-2026-10-17-h | 2026-10-17 | **Start-Prompt: Marker statt Kämmen** | GLM · Mistral · Fugu · Lumo | entschieden | `2026-10-17-rat-startprompt.md` | →Leitstelle (`command/*.md`) | — |
| R-2026-10-17-i | 2026-10-17 | **Offen-Stand-Zähler**: Bullets vs `###`, 1-KiB-Kappe, Ledger-Schließungen | Fugu · Lumo · GLM-5.3 · Mistral 4 | **entschieden** (4/4 A) | `2026-10-17-rat-offen-stand.md` | →mycelium | — |
| R-2026-10-17-j | 2026-10-17 | **`K` nach der 1981–83-WDC-Ernte identifizierbar?** | Fugu · Lumo · DeepSeek V4 Pro · Kimi K3 · Mistral 4 | **entschieden** (5/5 A) | `2026-10-17-rat-k-identifizierbar.md` | →mountain · →river | — |
| R-2026-10-17-k | 2026-10-17 | **Native Fläche statt Bastellösungen?** (API/CLI/Features vs. Eigenbau) | Fugu · Lumo · GLM-5.3 · Mistral 4 | **entschieden** (4/4 C) | `2026-10-17-rat-native-flaeche.md` | →future | — |
| R-2026-10-18-a | 2026-10-18 | **Quittung im Token-Ledger** — Empfänger quittiert im eigenen Ledger statt im fremden (`xfer ack`, `--addressed`-Sicht) | *offen — future + Operator* | **offen** | `2026-10-18-xfer-quittung-formular.md` | →future | — |
| R-2026-10-18-b | 2026-10-18 | **GRS80/γ₀-Register** — skalare Referenzkonstanten ohne Parser-Arm: neue Direktive vs. Block vs. local-only; `N` bleibt pending | *offen — Operator wählt* | **offen** | `2026-10-18-grs80-skalar-formular.md` | →future | — |

**Stand-Vokabular:** `offen · entschieden · geroutet · quittiert · verworfen`.

**Die sechs alten Formulare** (China, Netzfusion, Venus, Platzierung, MODVOLC, grid-form) sind
**Zeilen** geworden; die Dateien bleiben als **Detail-Belege** (der Index zeigt auf sie).
