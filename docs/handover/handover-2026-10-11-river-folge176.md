<!--
  title: Handover — River-Folge 176 (2026-10-11)
  session: River-Folge 176
  class: handover
  date: 2026-10-11
  sha256: 3c372341decccad2722c19af8c8dd73aafdef20e57fabd1d05e691160b3066cd
  status: live
-->
# Handover — River-Folge 176 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** abgearbeitet; jeder wird **aufgeschlüsselt** geführt
(**Trigger** / **Lage** / **Blockade** / **Braucht**).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Nicht gebaut — die P10.2a-Token-Semantik ist nicht entschieden; ein geratener Bau wäre Fabrikation (A=A). bitte archive search all rat und roster" | 2026-10-10 | Operator (Session, River 165)
„wenn du den rat befragst befrage bitte auch die wissenschaft mit archive serahc alll und die ui und oenweight frontier voices" | 2026-10-09 | Operator (Session, River 145)
„warum nur duck … ich möchte dass du alle frontier chats befragst" | 2026-10-07 | Operator (Session, River 127)
„ja ich meine alle blöcke müssen korrekt sein dafür haben wir doch die wissenschaft" | 2026-10-09 | Operator (Session, River 160) — **P10-Wort**: jeder `field`-Block wird wissenschaftlich geprüft und korrekt etikettiert (Quantity | Mechanism | Medium)
„ja natürlich sonst wartest du doch bis zum st. nimmerleinstag" + „ja bitte ihr müsst das jetzt echt mal in den griff bekommen" | 2026-10-10 | Operator (Session, River 170) — den abgebrochenen `ci-gate` neu anstoßen; den CI-Burst in den Griff bekommen
„mach A" | 2026-10-10 | Operator (Session, River 170) — `ci-gate.yml`-`subset` von dem geteilten t420 auf GitHub-hosted `ubuntu-24.04-arm` mit per-SHA-Concurrency umziehen
„beides" | 2026-10-10 | Operator (Session, River 170) — (1) die zwei fremden ci-gate-Roten heilen; (2) die `observer`→`receiver`-Frage als Rat-Frage aufsetzen; Audit „wo wurde der Observer-Bias wieder eingeschleust"
„C (Skalenspektrum je Paar) ist der nächste begrenzte Schritt — Architekturfrage → Linse der fünf Stimmen, bevor gebaut; danach D (Summary-Graph/PCMCI, Runge 2105.10381/1702.07077). Die Seats: B ohne Skalenannotation wäre die stille Fabrikation. bitte --all rat und max roster" | 2026-10-10 | Operator (Session, River 172) — C vor dem Bau durch die fünf Stimmen + Max-Roster; `--all`-Recherche voran
„Nächster Schritt: D2 durch die fünf Stimmen (Rat), dann Skeleton/MCI bauen. bitte --all max roster und rat" | 2026-10-10 | Operator (Session, River 172) — D2 (Summary-Graph/PCMCI) durch Rat + Max-Roster, `--all` voran
„bitte --all max roster und rat" | 2026-10-10 | Operator (Session, River 174) — die Window-Graph-CI-Verdrahtung und die Panel-Skala Δ (D2-Rat-Verdikt Punkt 5) durch `--all`-Recherche + Rat (5 Stimmen) + Max-Roster
„z.ai immer glm 5.3 max gibt es auch in tryong open genauso wie qwen" | 2026-10-10 | Operator (Session, River 174) — Z.ai stets **GLM-5.3 Max**; `tryingopen` führt **GLM 5.3 Max** und **Qwen**

Verbatim: `state/operator-gespraeche/2026-10-10-river.md` und
`state/operator-gespraeche/2026-10-09-river.md`. Fortgeschrieben aus
`docs/handover/archiv/handover-2026-10-11-river-folge175.md` §Operator-Wort-Register.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der komplette Bauplan (P0–P10); P10.2a als gebaut nachgeführt (`:215`).
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens.
- `state/stimmen/2026-10-10-river-window-graph-panel-rat.md` (Rat, 5 Stimmen: Δ = coarsest `median_dt`, Verdrahtung, zwei Ohren, Summary-Graph), `…-roster.md` (Max-Roster), `…-research.txt` (`archive_search --all`).
- `state/stimmen/2026-10-10-river-d2-pcmci-rat.md` / `…-roster.md` / `…-research.txt`; `state/stimmen/2026-10-10-river-c-skalenspektrum-rat.md` / `…-roster.md` / `…-research.txt`.
- `state/stimmen/2026-10-10-river-p10.2a-token-semantik.md`; `state/stimmen/2026-10-10-river-advective-conserved.md`; `state/stimmen/2026-10-10-river-p92-rat.md`; `state/stimmen/2026-10-10-river-te-joint-grid-research.md` / `…-rat-roster.md`.
- `phi/pipeline/descriptors/vlies_panel.te` — der Panel-Deskriptor (RTSW/SWPC-Block, `scale auto`, `stage2 mci`, `fdr bh 0.05`); kein `phi/canon.φ`-Akt (eine `.te` steht außerhalb des Kanons, gemessen).
- `src/mathematikerin/mci.rs` — `mci_window_graph` (`WindowGraph { links, pending }`, `PendingEdge`), `n_min = bins^(2+dim)`-Gate; `mci_window_links` bleibt der Wrapper.
- `tools/measure/src/bin/field_te_query.rs` — `run_summary_panel` (Δ-Herkunft `(from <channel>, coarsest of <N>)` / `(operator-declared)`, `n_min`-Kanten), `parse_descriptor` Form `panel <label>` + `scale auto|<s>` + `stage2 mci`.
- `.github/workflows/field-te-query.yml` — Job `panel-vlies` (Artefakt `field-te-panel-vlies.txt`).
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` — Asservatenkammer (11 offen), Träger der newell-omni-Messung; Mountain-303 per `## An river` an River als Träger bestätigt (2026-10-10).
- `docs/blatt/blatt-gic-breitenband-familien.md` (`status: unsealed`) — Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md`, `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md`, `docs/paper/gic-causal-driver.md`, `docs/paper/flyby-path-2-addendum-2026-09-29.md`, `docs/concepts/remove-bias.md`.

## Offen (aufgeschlüsselt)

### Flyby-Kette — recon bleibt
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** `ephemeris-juice-recon` Wiedervorlage 2026-11-01 (ESA/ESOC publiziert die Post-Flyby-SPK). Wahrheit: `state/zustand/wartend.φ:34`.
- **Lage:** (gemessen 2026-10-08, River 138) RTSW gefüllt; kp `def` freigegeben; `omni2_bz` absent → `pending`; `δ = 0,168 km` steht, Δ + σ_recon brauchen die Post-Flyby-Rekonstruktion (`ephemeris_juice_recon.bin` 404).
- **Blockade:** ESA/ESOC-SPK + 1-σ-Kovarianz absent.
- **Braucht:** am 2026-11-01 `archive_search --verdict` auf den ESOC-recon-Pfad; dann `flyby_ephemeris_gate --recon <recon.bin> --sigma-recon <km>`.

### panel-vlies — das Gitter ist nicht joint-fähig
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner — eigener Bau-Schritt (die verdiktete Kanalmenge joint-fähig machen).
- **Lage:** (gemessen 2026-10-11, River 176) Lauf `38091393819` **all-green** (14/14 Jobs; `unit` grün = `cargo test --release -p omegaflow-measure --bin field_te_query`; `panel-vlies` grün; Artefakt `field-te-panel-vlies.txt`, ID 11683769322). Ausgabe: `panel vlies_panel — 5 channels | Δ 86400s (from omni_imf_bz_gsm_nt, coarsest of 5)` und `panel pending — bin window carries no overlap between the arms`. Δ ist **gemessen** (`median_dt`, keine 86400-Konstante im Code), kein Fallback. **Riss:** Rat + Deskriptor (`vlies_panel.te:5`) sagen „86400 is never the graph grid" und setzen 86400 als Notwert einer ungemessenen Rate — hier ist 86400 die *gemessene* native Rate von `omni_imf_bz_gsm_nt`; die verdiktete 5-Kanal-Menge überlappt auf dem Tagesgitter nicht (`align_many`, `field_te_query.rs:4539`). Ursache gemessen: `phi/sources.φ` registriert `omni_imf_bz_gsm_nt` **dreifach** — `:738` (Frame 3600 s, stündlich), `:1895` und `:1910` (86400 s, täglich); der Loader greift die Tages-Registrierung.
- **Blockade:** kein lokaler Testlauf erlaubt (Batterie läuft in CI); `phi/sources.φ` ist aktuell fremd-modifiziert (parallele Session), nicht anfassbar.
- **Braucht:** die Mehrfach-Registrierung von `omni_imf_bz_gsm_nt` prüfen (`load_matrix_arm` — welche Frame-Registrierung gewinnt; `phi/sources.φ:738` vs `:1895/:1910`) und auf die native Rate (3600 s) führen; dann die Kanalmenge auf überlappende Fenster mit `n ≥ n_min = bins^(2+dim)` prüfen und `field-te-query` `panel-vlies` neu dispatcht.

### window-graph — die zwei Ohren + Summary-Graph
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner — eigener Bau-Schritt.
- **Lage:** (gemessen 2026-10-11, River 176) Schritt (1) **gebaut und grün**: `pc_stable_skeleton_screened(series, test, alpha, max_cond) -> (edges, screened: Vec<(usize,usize,f64)>)` in `src/mathematikerin/pc.rs` (die verworfenen ParCorr-Kanten mit `p > alpha`, sortiert; `pc_stable_skeleton` bleibt der Wrapper), Unit-Test `confounder_screened_pair` verifiziert die verworfene (1,2)-Kante. `cargo check` (core) grün. **Nicht committet:** `src/mathematikerin/*` ist von einer parallelen Session aktiv gehalten (`mci.rs`/`parcorr.rs`/`receiver.rs`/`root.rs` u. a. uncommittet im Arbeitsbaum) — ein Commit von `pc.rs` würde fremde Hunks sweepen (Operator-Wort 2026-09-30, Commit als Letzter).
- **Blockade:** `pc.rs` fremd-gehalten im Arbeitsbaum; Schritt (2) `mci.rs` kollidiert direkt mit der parallelen Session (die selbst `mci.rs` schreibt).
- **Braucht:** sobald `src/mathematikerin/*` frei ist — (1) `git commit src/mathematikerin/pc.rs` (der Screened-Satz, liegt fertig im Arbeitsbaum); (2) `mci.rs` — `ScreenedLink { driver, target, lag, p_pc }` in `WindowGraph`, TE auf der screeneten Menge, Divergenz = `riss`; (3) `field_te_query.rs` — `screened`/`riss` drucken; (4) danach `summary_graph_from_rungs()` (Quelle `matrix-vlies`-Rung-Zeilen, Assaad-artig, kein Resampling) als eigener Schritt.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.

## Burn: open 0.0000 · close 0.1489 · cap 0.20 — Grund: der gemessene Delta der River-`line`-Summe über die Sitzung (`session_burn`: $0.6988 → $0.8477) trägt die parallel laufenden Linien-Sessions mit; die eigene Sitzung ist nicht isolierbar (die `line`-Zählung ist noch offen). Ein-Pass: panel-vlies-Lauf `38091393819` verifiziert (all-green; Δ-Herkunft + pending gemessen; Riss = Tages-Registrierung von `omni_imf_bz_gsm_nt`), `pc_stable_skeleton_screened` gebaut und `cargo check` grün, aber nicht committet (Arbeitsbaum fremd-gehalten); deepseek-flash, kein pro/max, kein Diver.
