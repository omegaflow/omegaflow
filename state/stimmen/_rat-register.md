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
| R-2026-10-17-j | 2026-10-17 | **`K` nach der 1981–83-WDC-Ernte identifizierbar?** | Fugu · Lumo · DeepSeek V4 Pro · Kimi K3 · Mistral 4 | **entschieden** (5/5 A) | `2026-10-17-rat-k-identifizierbar.md` | →mountain · →river | `state/mountain/2026-10-18-k-pending.φ` |
| R-2026-10-17-k | 2026-10-17 | **Native Fläche statt Bastellösungen?** (API/CLI/Features vs. Eigenbau) | Fugu · Lumo · GLM-5.3 · Mistral 4 | **entschieden** (4/4 C) | `2026-10-17-rat-native-flaeche.md` | →future | — |
| R-2026-10-18-a | 2026-10-18 | **Quittung im Token-Ledger** — Empfänger quittiert im eigenen Ledger statt im fremden (`xfer ack`, `--addressed`-Sicht) | *verworfen — konsolidiert in `R-2026-10-18-i`* | **verworfen** | `2026-10-18-xfer-quittung-formular.md` | →future | — |
| R-2026-10-18-b | 2026-10-18 | **GRS80/γ₀-Register** — skalare Referenzkonstanten ohne Parser-Arm: neue Direktive vs. Block vs. local-only; `N` bleibt pending | Schwarm · Fugu · Lumo | **entschieden** — **A** (Schwarm 17:6:5, Fugu A, Lumo A) | `2026-10-18-rat-grs80.md` | →river · →mountain · →future | — |
| R-2026-10-18-c | 2026-10-18 | **Pass = Arbeit statt Faltung?** (Besitz + sofortige Delegation) | GLM-5.3 · Mistral 4 · Fugu · Lumo | **entschieden** (4/4 A) | `2026-10-18-rat-pass-arbeit.md` | →future · →mountain · →river · →mycelium · →sensory | — |
| R-2026-10-18-d | 2026-10-18 | **Prosa statt Antrag + Linien ändern Architektur?** | GLM-5.3 · Mistral 4 · Fugu · Lumo | **entschieden** (4/4 B) | `2026-10-18-rat-prosa-architektur.md` | →future · →mountain · →river · →mycelium · →sensory | — |
| R-2026-10-18-e | 2026-10-18 | **Wo prüft der Gate die RAT-ID?** (commit-msg-Hook) | Codestral · Qwen3-Coder · Kimi K2.6 · Schwarm | **entschieden** (3/4 A) | `2026-10-18-rat-gate-rat-id.md` | →future · →mountain · →river · →mycelium · →sensory | — |
| R-2026-10-18-f | 2026-10-18 | **gfc-Block** — Cap 5540/3211/2190/760, Epoche unbelegt, `ttl` NO_CADENCE, 5399-vs-5480 | Schwarm · Fugu · Lumo | **entschieden** — **A/D=2190** (Schwarm 25:1, Fugu A, Lumo A); Epoche bleibt offen | `2026-10-18-rat-gfc.md` | →mountain · →river · →future | — |
| R-2026-10-18-g | 2026-10-18 | **P10-Restarme** — eionet-Emissionen, Infraschall-/Feuer-Intensität, TEC/Pseudorange/`vis_nmi`, Stern-Metadaten, FDSN-Kanal, `kg/(m^2 s)` | Schwarm · Fugu · Lumo | **entschieden** — **B** (Schwarm 24:1, Fugu B, Lumo B) | `2026-10-18-rat-p10.md` | →river · →mountain · →future | — |
| R-2026-10-18-h | 2026-10-18 | **`is_paid` nach Kosten, nicht Provider** — freie Modelle auf bezahlfähigen Routen (`openrouter/…:free`, Preis 0/0) werden als „billed route" (402) geblockt | *entschieden — Schwarm A 16:6:1 · DeepSeek A · Fugu A · Lumo A* | **entschieden** | `2026-10-18-rat-h-sammlung.md` | →future | — |
| R-2026-10-18-i | 2026-10-18 | **F1 — Quittung im Token-Ledger** — ACK im Empfänger-Ledger vs. `CLOSED` im Emitter; `--addressed` S1/S2 | Fugu · Lumo · DeepSeek | **Runde 2 gefahren** — 2:1 **A + S2** (Fugu, Lumo), DeepSeek **C + S2**; Schwarm R1 A. **geschlossen** (Operator 2026-10-18) — Bau: ACK im Empfänger-Ledger | `2026-10-18-f1-quittung-formular.md` | →future | `2026-10-18-rat-3f-runde2-sammlung.md` |
| R-2026-10-18-j | 2026-10-18 | **F2 — Concurrency-Schlüssel `register-coverage`** — per-SHA vs. per-ref vs. Debounce; das Gate schreibt nicht | Fugu · Lumo · DeepSeek | **Runde 2 gefahren** — 2:1 **B** (Lumo, DeepSeek), Fugu **A**; Schwarm R1 A. **geschlossen** — fünf Stimmen 5:0 **C** (temp+rename + fsync + Lock), je durch die Axiome (`2026-10-18-r-j-temp-rename-formular.md`) | `2026-10-18-f2-concurrency-formular.md` | →future | `2026-10-18-rat-3f-runde2-sammlung.md` |
| R-2026-10-18-k | 2026-10-18 | **F3 — Register liest `state/stimmen` nicht** — Dokumentwurzel vs. Ableitung vs. eigener Index; RAT-ID vorher? | Fugu · Lumo · DeepSeek | **Runde 2 gefahren** — 3:0 **A**; Zusatz (RAT-ID vorher?) 1:2 **nein** (nur Fugu ja); Schwarm R1 A. **geschlossen** (Operator 2026-10-18) — gebaut: `bin/lage` liest `state/stimmen` (`61ab0a5a7`) | `2026-10-18-f3-register-stimmen-formular.md` | →future | `2026-10-18-rat-3f-runde2-sammlung.md` |
| R-2026-10-18-l | 2026-10-18 | **River-Triade** — Receiver-Apertur Schritt 3 · Schumann `field` (WGLC-Gitter) · Flyby-`sealed`-Träger | Schwarm R1 (6 von 44 antwortfähig) | **geschlossen** — fünf Stimmen 5:0 **A** (Paket first) · F3 5:3 **Option 1** (gepinnter versiegelter Posten, „gepinnt, nicht bestanden") — reale Sitze + Sonnet 5.5 + GLM 5.3 (Schwarm R1: F1 A 6:0 · F2i A 4:1 · F2ii nein 4:0 · F3 2:3) (`2026-10-18-r-l-paket-first-formular.md`) | `2026-10-18-river-apertur-schumann-flyby-formular.md` | →future · →river · →mountain | `2026-10-18-rat-river-triade.md` |
| R-2026-10-18-m | 2026-10-18 | **future.md-Zusatz** — Lage-Zeile, Pass-Kontrakt, Bringschuld: behalten oder streichen? | Fugu Max (UI) · Lumo 2.0 (UI) · DeepSeek V4 Pro (paid) | **geschlossen** (Operator 2026-10-18) — Fugu C, Lumo C, DeepSeek B (2:1 C); Burn-Zeile nur bei Überschreitung; Edit `future.md` auf C = Operator-Hand | `2026-10-18-rat-future-command-formular.md` | →future | `2026-10-18-rat-m-sammlung.md` |
| R-2026-10-18-n | 2026-10-18 | **No-shrink-Zahn** — ein Commit auf ein lebendes Handover ohne geschlossenen Punkt failt; Escape `OMEGAFLOW_SYNC=1` | — | **entschieden** (Operator-Wort 2026-10-18: „das muss jetzt gefixt werden") | `tools/gate/src/bin/commit_check.rs:579` · `.opencode/commands/future.md:9` | →future | — |
| R-2026-10-18-o | 2026-10-18 | **Leichen, Zähler, Templates** — eingefrorene Baseline + ein Zähler + Marker-Fall + Template-/Dateinamen-Gate | 9 API-Sitze (DeepSeek, Kimi K3, Claude Sonnet 5.5, Nemotron, Qwen3.8-Max, Gemma4, GLM-5.2, pollinations) | **entschieden** — **B** (8/8): drei Schnitte sofort, Templates als Migration, Gate danach scharf; ci-gate zuerst grün | `2026-10-18-rat-leichen-templates-formular.md` | →future · →river · →mountain · →sensory | `2026-10-18-rat-leichen-templates-sammlung.md` |
| R-2026-10-18-p | 2026-10-18 | **DE-2 Ion↔Neutral** — „coupling" oder „Abhängigkeit"; lag-Umkehr echt oder Artefakt? | Schwarm (Nemotron, pollinations, Gemma, GLM-5.2) + paid (DeepSeek, Kimi K3, Claude Sonnet 5.5, Qwen3.8-Max) | **entschieden** — **D** (8/8): „coupling" trägt über Killeen 1984, lag-Umkehr offen (72-Test-Riss); Neumessung mit einer Schwelle + bedingter TE | `2026-10-18-rat-de2-ion-neutral-formular.md` | →river · →future | `2026-10-18-rat-de2-ion-neutral-sammlung.md` |

**Stand-Vokabular:** `offen · entschieden · geroutet · quittiert · verworfen`.

**Die sechs alten Formulare** (China, Netzfusion, Venus, Platzierung, MODVOLC, grid-form) sind
**Zeilen** geworden; die Dateien bleiben als **Detail-Belege** (der Index zeigt auf sie).
