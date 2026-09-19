<!--
  title: Handover — Bau-Folge 92 (Stand 2026-09-19)
  session: Bau-Folge 92
  class: handover
  date: 2026-09-19
  sha256: 7a52f78dd25c9bbc9990e2658a4d038683a008c7832985d7984569d26ff2d0e1
  status: live
-->
# Handover — Bau-Folge 92 (2026-09-19)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder offene
Punkt trägt seinen nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt.

Das Handover wird **vor allem anderen gegen den Baum gehalten**.

## Stehender Pass (gemessen 2026-09-19, Session-Beginn)

- **HEAD** Session-Beginn `3d2e6adb`; während der Session von fremden Linien auf
  `7358143e` (ernte folge96) gezogen (== `origin/main`). folge91 (`a786e208`) war
  über `566d8ea4` (fremde CoherentPhase-Arbeit, committet) auf `3d2e6adb` gezogen.
  `git_safety` Snapshot `refs/safety/1789844617`.
- **Postfach** leer (`post.md` nur Header; jetzt + die drei neuen entscheid-Zeilen).
- **CI** (Watchdog 20:21): `te-gate 35451283398` in_progress; `ci-check`/`measure-gates`
  rot. `ci-check 35456865096` @`88fb27ad`: Job `format` rot (fmt --check) — die
  baumweite fmt-Drift ist die gemessene ci-check-Ursache.
- **Arbeitsbaum** fremd uncommittet (nicht angefasst): die drei
  `handover-2026-09-16-*`-Renames (entscheid/forschung → `archiv/`).

## TE-Gates — Umbau gebaut, CI-Verifikation ausstehend · `wartend`

Die zwei roten Einzel-Realisierungs-Gates sind auf Multi-Seed-Fraktion umgebaut
(`src/mathematikerin/te.rs`): `flare_envelope_conditional_keeps_true_coupling`
(30 Seeds, `found/meas ≥ 0.5`) und `synthetic_dag_recovers_known_direction`
(30 Seeds, wahre Kante A->B `≥ 0.5`, falsche Gegenkante B->A `≤ 0.5`), Vorbild
`gate_conditional_arx_fpr_fn_n1000`. **Korrektur am folge91-Schritt:** die zwei
Tests sind *nicht* ignoriert und stehen *nicht* in `te-gate.yml` — sie laufen im
`test`-Job von `ci-check` (`cargo test --release`), das auf `src/**`-Push
automatisch startet. (Auslöser: der `ci-check`-Lauf am Push; Ergebnis via
Watchdog-Snapshot / `ci_manage list`, nie `gh run`.) · `wartend`

## Offen

- **fmt-Drift baumweit (54 Dateien)** — `fmt-apply.yml` steht (`c74017a3`).
  (Schritt: nach dem Push `gh workflow run fmt-apply.yml`, dann `ci-check`.) · `pending`
- **`hdf5.rs` lazy Objektauflösung** — Guard steht, Graph-Traversal noch eager.
  (Schritt: bedarfsgesteuerte Auflösung im `Hdf5WindowReader`.) · `pending`
- **`icesat2_atl03_compiler.rs` Pagination** — `HARVEST_WINDOW_MAX = 512` deckelt
  eine Seite. (Schritt: CMR `page_num` / S3 Continuation-Token.) · `pending`

## Wartestellungen (kein Auswahlpunkt)

- `te-gate 35451283398` in_progress · `wartend`
- FUGIN-Bulk-Manifestation (270 Cubes) · `wartend`
- planetary-odf-cdn `35351411938` · `wartend`
- Scanned-/bild-only-PDFs → `vision`-OCR · `pending`

## An entscheid gegeben (post.md)

Die drei operator-gebundenen Punkte — vC-Permeabilität, Pipeline-Port force-Gate,
`register_lookup`-Symlink — sind als `An entscheid:`-Zeilen in `post.md` gesetzt
und aus diesem Register entfernt; sie gehören der entscheid-Linie.

## Diese Session — gearbeitet (git trägt es)

- Die zwei roten TE-Gates auf Multi-Seed-Fraktion umgebaut;
  `cargo check --all-targets`: 0 Fehler, 0 Warnungen.
- Clippy-Hunk `te.rs` question_mark verifiziert: bereits geheilt in `566d8ea4`
  (`te.rs:2018` trägt die `?`-Form) — kein Edit nötig.
- Die drei operator-gebundenen Punkte an entscheid gepostet.

## Benchmark

- TE-Gate-Umbau: `grind-max` (pro/max) hatte auf dieser Klasse zwei leere Rückläufe
  (folge90 + folge91). Dieser Umbau lief direkt in der Bau-Session (build/DeepSeek);
  kein dritter max-Dispatch. Die Klasse hat noch keinen Sieger; der CI-Lauf am Push
  entscheidet über die Fraktions-Schwellen.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/mathematikerin/te.rs` (zwei Test-Hunks),
  `docs/handover/post.md` (drei Zeilen + Header-sha),
  `docs/handover/handover-2026-09-19-bau-folge92.md` (neu), Move
  `handover-2026-09-19-bau-folge91.md` → `archiv/`.
- **Fremd (nicht anfassen):** die `handover-2026-09-16-*`-Renames.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
