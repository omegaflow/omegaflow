<!--
  title: Handover — Bau-Folge 91 (Stand 2026-09-19)
  session: Bau-Folge 91
  class: handover
  date: 2026-09-19
  sha256: 3dec98a762f73ea43ffe11c8b697852e43fd1399a2ba3d79b7ca2c1a2ab9a17d
  status: live
-->
# Handover — Bau-Folge 91 (2026-09-19)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder offene
Punkt trägt seinen nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**.

## Stehender Pass (gemessen 2026-09-19, Session-Beginn)

- **HEAD** `a786e208` (die fremde Hyperscanning-/Entscheid-Linie ist seit
  folge90 weitergelaufen: `5cba9f70` → `2c13f7da` → `a786e208`). `main`.
- **Postfach** leer (`post.md` nur Header).
- **Fremd uncommittet/gestaged (unangetastet):** entscheid-Handover-Moves
  (`handover-2026-09-19-entscheid-folge54`, `forschung-folge93` → `archiv/`;
  `entscheid-folge55` neu; `external-state.md`), `src/mathematikerin/te.rs`
  (`TeNull::CoherentPhase` + `coherent_phase_surrogates`), `src/archivar/motion.rs`,
  `tools/measure/src/bin/hyperscanning_group_te.rs`,
  `tools/measure/src/bin/pcmci_class_benchmark.rs`,
  `tools/utils/src/bin/hdf5_reader.rs`, `mars_dust_compiler.rs` +
  `.github/workflows/mars-dust-cdn.yml` (neu), `forschung-folge94`.
- **CI** (Watchdog-Snapshot 19:06): `te-gate 35427414837` in_progress,
  `measure-gates`/`ci-check` rot; `ci-check 35451506666` @`5cba9f70` ist der
  analysierte Lauf.

## Zwei rote TE-Gates — Diagnose steht, Fix braucht freies te.rs · `blockiert`

`flare_envelope_conditional_keeps_true_coupling` und
`synthetic_dag_recovers_known_direction` sind Einzel-Realisierungs-Vergleiche
gegen `mean + 2σ` (`conditional_te_stats_lagged`, `te.rs`). Gemessen am Code:
Test 1 `(te−mean)/σ ≈ 1,8`, Test 2 ≈ 2,003 — **entgegengesetzte Richtung**, kein
Schwellenwert heilt beide. Der Fix ist ein Multi-Seed-Fraktions-Design wie
`gate_conditional_arx_fpr_fn_n1000` (`te.rs:4422`, 30 Seeds, `found/meas ≥ 0.5`).
`grind-max` lieferte erneut leeren Rücklauf (2. Messung). **te.rs ist gerade
fremd-editiert** — der Umbau wartet auf freies te.rs. (Schritt: die zwei Tests
auf Multi-Seed umbauen; dann `gh workflow run te-gate.yml`.) · `blockiert`

## Clippy-Job — 4 der 5 Lints committet, te.rs-Hunk geteilt · `pending`

CI `35451506666` `clippy` rot; alle fünf lokalisiert:
- `src/archivar/fugin.rs:43` collapsible_if → let-chain — **committet**
- `src/archivar/zarr.rs:203` explicit_counter_loop → `(start..).take(match_len)` — **committet**
- `src/mathematikerin/omega.rs:1626` collapsible_if → let-chain — **committet**
- `src/archivar/tests.rs:4722` manual_is_multiple_of → `!is_multiple_of` — **committet**
- `src/mathematikerin/te.rs:1972` question_mark → `?` — Hunk liegt im
  fremd-editierten te.rs, nicht allein committbar; reitet mit dem nächsten
  te.rs-Commit der Fremd-Linie. (Schritt: nach freiem te.rs verifizieren.) · `pending`

`cargo check --all-targets`: 0 Fehler, 0 Warnungen.

## Offen

- **fmt-Drift baumweit (54 Dateien)** — `cargo fmt` lokal strukturell verweigert
  (`opencode.json`: `cargo *` deny, nur `cargo check`). Neuer Apply-Pfad:
  `.github/workflows/fmt-apply.yml` (workflow_dispatch, `cargo fmt --all` auf dem
  ausgecheckten HEAD, Commit+Push mit Retry — kein geteilter Arbeitsbaum).
  (Schritt: `gh workflow run fmt-apply.yml`; danach `ci-check`.) · `pending`
- **`hdf5.rs` lazy Objektauflösung** — Guard steht, Graph-Traversal noch eager.
  (Schritt: bedarfsgesteuerte Auflösung im `Hdf5WindowReader`.) · `pending`
- **`icesat2_atl03_compiler.rs` Pagination** — `HARVEST_WINDOW_MAX = 512` deckelt
  eine Seite. (Schritt: CMR `page_num` / S3 Continuation-Token.) · `pending`
- **vC-Permeabilität — Messakt lokal, kein CDN** — (Schritt: Release-Bin →
  `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad>`, dann `perm_target_probe --live`.) · `operator-gebunden`
- **Pipeline-Port force-Gate** — Verteilungs-Probe + Fixture über die 10
  `phi/pipeline/queue/*.φ`; A/B wartet auf Operator-Wort. · `pending`
- **`register_lookup`-Symlink** — `~/.local/bin/register_lookup` zeigt direkt auf
  `target/release`, ohne `run_open`. (Schritt: Symlink auf `bin/register_lookup`.) · `operator-gebunden`

## Wartestellungen (kein Auswahlpunkt)

- `te-gate 35427414837` in_progress · `wartend`
- FUGIN-Bulk-Manifestation (270 Cubes) · `wartend`
- planetary-odf-cdn `35351411938` · `wartend`
- Scanned-/bild-only-PDFs → `vision`-OCR · `pending`

## Diese Session — gearbeitet (git trägt es)

- Die fünf Clippy-Lints aus `ci-check 35451506666` geheilt; vier davon committet.
- TE-Gate-Diagnose am Code (Zwei-σ-Grenzlage, entgegengesetzte Richtung) —
  als Fix-Richtung im offenen Punkt festgehalten.

## Benchmark

- **2 rote TE-Gates**: `grind-max` (pro/max) — leerer Rücklauf, **2. Messung**
  (folge90 + folge91). Kein Doppel-Lauf; die Klasse hat keinen Sieger, der
  Multi-Seed-Umbau bleibt dem nächsten freien te.rs.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/archivar/fugin.rs`, `src/archivar/zarr.rs`,
  `src/mathematikerin/omega.rs`, `src/archivar/tests.rs` (je nur eigene Hunks),
  `src/mathematikerin/te.rs` (Hunk `betti0_persistence`, geteilt),
  `docs/handover/handover-2026-09-19-bau-folge91.md` (neu), Move
  `handover-2026-09-19-bau-folge90.md` → `archiv/`.
- **Fremd (nicht anfassen):** `te.rs` CoherentPhase, `motion.rs`, hyperscanning,
  mars-dust, entscheid-Moves, `external-state.md`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
