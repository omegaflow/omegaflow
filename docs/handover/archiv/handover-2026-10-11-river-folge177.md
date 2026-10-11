<!--
  title: Handover — River-Folge 177 (2026-10-11)
  session: River-Folge 177
  class: handover
  date: 2026-10-11
  sha256: 0198fd5da1ad479fd10b16ba95f15c04f8ac5cd93dfe7c1861174823b7a389f7
  status: live
-->
# Handover — River-Folge 177 (2026-10-11)

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
`docs/handover/archiv/handover-2026-10-11-river-folge176.md` §Operator-Wort-Register.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der komplette Bauplan (P0–P10); P10.2a als gebaut nachgeführt (`:215`).
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens.
- `state/stimmen/2026-10-10-river-window-graph-panel-rat.md` (Rat, 5 Stimmen: Δ = coarsest `median_dt`, Verdrahtung, zwei Ohren, Summary-Graph), `…-roster.md` (Max-Roster), `…-research.txt` (`archive_search --all`).
- `state/stimmen/2026-10-10-river-d2-pcmci-rat.md` / `…-roster.md` / `…-research.txt`; `state/stimmen/2026-10-10-river-c-skalenspektrum-rat.md` / `…-roster.md` / `…-research.txt`.
- `state/stimmen/2026-10-11-river-panel-aufloesung-rat.md` — River 177, Rat zur Auflösungspolitik von `load_field_across_sources` (Registerfix kanonisch; `first-load-wins` → gemessene Eigenschafts-Auswahl).
- `state/stimmen/2026-10-10-river-p10.2a-token-semantik.md`; `state/stimmen/2026-10-10-river-advective-conserved.md`; `state/stimmen/2026-10-10-river-p92-rat.md`; `state/stimmen/2026-10-10-river-te-joint-grid-research.md` / `…-rat-roster.md`.
- `phi/pipeline/descriptors/vlies_panel.te` — der Panel-Deskriptor (RTSW/SWPC-Block, `scale auto`, `stage2 mci`, `fdr bh 0.05`); kein `phi/canon.φ`-Akt (eine `.te` steht außerhalb des Kanons, gemessen).
- `src/mathematikerin/mci.rs` — `mci_window_graph` (`WindowGraph { links, pending }`, `PendingEdge`), `n_min = bins^(2+dim)`-Gate; `mci_window_links` bleibt der Wrapper.
- `src/mathematikerin/pc.rs` — `pc_stable_skeleton_screened` (Commit `10e0d1323`): gibt den ParCorr-verworfenen Satz `(i, j, p)` als `screened` zurück; `pc_stable_skeleton` bleibt der Wrapper.
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
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keiner — eigener Bau-Schritt (die verdiktete Kanalmenge joint-fähig machen).
- **Lage:** (gemessen 2026-10-11, River 176/177) Lauf `38091393819` **all-green** (14/14 Jobs; Artefakt `field-te-panel-vlies.txt`, ID 11683769322). Ausgabe: `panel vlies_panel — 5 channels | Δ 86400s (from omni_imf_bz_gsm_nt, coarsest of 5)` und `panel pending — bin window carries no overlap between the arms`. Δ ist **gemessen** (`median_dt`). Ursache (gemessen via Code, `field_te_query.rs:1237-1266`): `field_sources` liefert Kandidaten in Register-Reihenfolge, `load_field_across_sources` nimmt den **ersten**, der lädt (first-load-wins); `phi/sources.φ` registriert `omni_imf_bz_gsm_nt` dreifach (`:738` Live-HAPI ohne `format`-Direktive → lädt nicht, `:1895` Tages-Bin `omni2_serie.bin` 86400, `:1910` 1h-Bin `omni2_serie_1h.bin` mit **Frame 86400 deklariert**) — der Tages-Bin steht vor dem 1h-Bin und gewinnt. Rat (`state/stimmen/2026-10-11-river-panel-aufloesung-rat.md`): kanonisch ist der **Registerfix**; `first-load-wins` ist als Ordnungs-Abhängigkeit nicht haltbar und wird nach der Messung durch eine **gemessene** Eigenschaft (feinster **gleichartiger** Zeuge, benannt) ersetzt; bei verschiedener Aggregation `panel pending` mit beiden Zeugen (Riss).
- **Blockade:** `phi/sources.φ` fremd-modifiziert im Arbeitsbaum (Open-LiDAR-Block ab `:3117`, uncommittet) — die OMNI-Zeilen sind nicht committierbar, ohne den Fremd-Hunk zu sweepen.
- **Braucht:** (Rat-Schritt 1) die Überlappungs-Korrelation Tages- vs Stunden-Bz auf gemeinsamen Stunden messen (entscheidet, ob die Registrierungen dieselbe Größe sind) — als neuer Druck im `field_te_query`-Panel, CI-Lauf; (Rat-Schritt 2, sobald `phi/sources.φ` frei) `:1910` `86400` → `3600` + Drei-Registrierung entflechten; (Rat-Schritt 3, nach 1.) `load_field_across_sources` → gemessene Eigenschafts-Auswahl; danach `field-te-query` `panel-vlies` neu dispatchen.

### window-graph — die zwei Ohren + Summary-Graph
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keiner — eigener Bau-Schritt.
- **Lage:** (gemessen 2026-10-11, River 177) Schritt (1) **steht** (Commit `10e0d1323`): `pc_stable_skeleton_screened(series, test, alpha, max_cond)` in `src/mathematikerin/pc.rs` gibt `(edges, screened)` zurück, `screened` = die verworfenen ParCorr-Kanten `(i, j, p)` mit `p > alpha`; der zuvor **leere** Screened-Satz ist nun befüllt (`screened.push((i,j,p))` im Entfernen-Zweig), `cargo check` (core) grün.
- **Blockade:** `src/mathematikerin/mci.rs` kollidiert mit der parallelen Session (uncommittet im Arbeitsbaum).
- **Braucht:** sobald `mci.rs` frei ist — (2) `ScreenedLink { driver, target, lag, p_pc }` in `WindowGraph`, TE auf der screeneten Menge, Divergenz = `riss`; (3) `field_te_query.rs` `screened`/`riss` drucken; (4) danach `summary_graph_from_rungs()` (Quelle `matrix-vlies`-Rung-Zeilen, Assaad-artig, kein Resampling) als eigener Schritt.
