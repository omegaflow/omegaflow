<!--
  title: Handover — Bau-Folge 90 (Stand 2026-09-19)
  session: Bau-Folge 90
  class: handover
  date: 2026-09-19
  sha256: 234e707ba9e708df04011ef2a435c54322cc069e3191a38442857c3621614d0a
  status: live
-->
# Handover — Bau-Folge 90 (2026-09-19)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-19, Session-Beginn)

- **HEAD** `5cba9f70` (== origin/main) bei Session-Beginn; während der Session
  committete die fremde Hyperscanning-Linie weiter (`49801083`, `88fb27ad`).
  Fremd uncommittet: `docs/handover/handover-2026-09-19-{entscheid-folge53→54,
  forschung-folge92, hyperscanning-te}.md`, `.github/workflows/hyperscanning-te.yml`,
  `tools/measure/src/bin/hyperscanning_*.rs`, `tools/measure/src/eeglab.rs`,
  `docs/zustand/external-state.md`, die `handover-2026-09-16-*`-Moves — unangetastet.
- **Postfach** — `state/mail/mail_ledger.φ` letzte Zeile: CSES-Limadou (Dr. Sotgiu,
  2026-09-16): Zugang nach Website-Umbau neu → `wartend`, operator-gebunden (nicht
  Bau). `post.md` trug vier „An bau"-Zeilen — gefaltet, gelöscht (s. unten).
- **CI** — `ci-check` `35451506666` @`5cba9f70` **failure**: `test` 1392 grün /
  2 rot (die zwei TE-Gates unten), `clippy` 5 Lints, `format` baumweiter fmt-Drift.
  `ci-check` `35448988400` @`b374736c` hatte 7 rot — der folge89-Fix hat 7 → 2
  reduziert. `te-gate` `35451283398` @`1934c012` `pending` (kein Verdikt lesbar).
  `measure-gates` `35454958335` @`5cba9f70` failure (`silence_map_probe` exit 101,
  fremd). Jüngster `ci-check` `35456571554` @`49801083` `pending` (fremd).
  Am Session-Ende: eigener Commit `1b0c3303` gepusht (die fremde Linie hat ihn im
  Fast-Forward mitgenommen, HEAD `2c13f7da`); `measure-gates` `35457737916`
  @`2c13f7da` `pending` — dispatcht, misst die neuen Betti-0-Gates.

## Zwei rote TE-Gates — der folge89-Fix ist unvollständig · `pending`

Der per-Lag-Commit `1934c012` hat 5 der 7 roten Gates geheilt; **zwei bleiben rot**
(CI `35451506666`, Job `test`, wörtliche Fehler):

- `flare_envelope_conditional_keeps_true_coupling` — `cond 0.19935536790681893
  thr 0.20395465799700968` (wahre Kopplung 2.3 % unter der Schwelle).
- `synthetic_dag_recovers_known_direction` — `cond 0.13120800647171352
  thr 0.1311547901865559` (falsche Rückkante B→A 0.04 % über der Schwelle).

Beide marginal, beide im conditional-TE-Pfad (`transfer_entropy_conditional`,
`conditional_te_stats`, `flare_pair`/`flare_envelope`). Ein `grind-max`-Lauf in
dieser Session lieferte **kein Ergebnis** (kein Hunk, leerer Rücklauf) — der Punkt
steht unverändert.

**Nächster Schritt:** `ci_manage log 35451506666 --all` lesen; die zwei Tests in
`src/mathematikerin/te.rs` (`sgrep "flare_envelope_conditional"` /
`"synthetic_dag_recovers"`) lokalisieren; Diagnose des conditional-TE-Surrogats
(der folge89-Fix betraf den ARX-Null, nicht diesen Pfad) und die Schwelle aus der
gemessenen Null-Verteilung statt aus einer gewählten Zahl setzen; dann
`gh workflow run te-gate.yml`. Council bei der Estimator-Frage. · `pending`

## Offen

- **5 clippy-Lints + fmt-Drift in `src/`** — `ci-check` `35451506666` `clippy` rot:
  `src/archivar/fugin.rs:43`, `src/archivar/zarr.rs:203`,
  `src/mathematikerin/omega.rs:1626`, `src/mathematikerin/te.rs:1972`,
  `src/archivar/tests.rs:4722`; `format` rot baumweit (u. a.
  `tools/utils/src/bin/archive_search/*.rs`). (Schritt: Lints fixen (`grind-flash`),
  `cargo fmt`; fremde Linien nicht überschreiben.) · `pending`
- **`hdf5.rs` lazy Objektauflösung** — der Guard (diese Session) beendet den eager
  `parse_fetch`-Gang, aber `parse_fetch` traversiert weiter den ganzen
  Objektgraphen. (Schritt: bedarfsgesteuerte Auflösung im `Hdf5WindowReader`.) ·
  `pending`
- **`icesat2_atl03_compiler.rs` Pagination** — `--skip`/`--limit` (diese Session)
  trägt echte Scheiben, aber `HARVEST_WINDOW_MAX = 512` deckelt eine Seite; Tage
  mit >512 Granules brauchen echte Pagination (CMR `page_num` / S3
  Continuation-Token). (Schritt: `sgrep "list_page\|page_size" tools/harvest/src/bin/icesat2_atl03_compiler.rs`.) · `pending`
- **vC-Permeabilität — Messakt lokal, kein CDN** (Post entscheid/Operator-Wort
  2026-09-19) — die Permeabilität darf **nicht** auf CDN. Messakt:
  `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> ./target/release/omegaflow`, danach
  `perm_target_probe --live <pfad>` (lokal). Offen: (a) Fallback ohne
  Surrogat-Nullkontrolle, (b) Saturations-Skala v_c/g. (Schritt: Release-Bin
  vorhanden → Messakt fahren, Verteilung messen.) · `operator-gebunden`
- **Pipeline-Port force-Gate** (Post entscheid/Operator-Wort) — `force_type`-Verteilung
  + Fixture über die 10 `phi/pipeline/queue/*.φ`-Korpora (gitignored, ~3.683
  Blöcke); A/B (lokaler Lauf vs. CI) wartet auf Operator-Wort. (Schritt:
  Verteilungs-Probe + Fixture.) · `pending`
- **`register_lookup`-Symlink** — `bin/register_lookup` (Wrapper, diese Session)
  existiert; `~/.local/bin/register_lookup` zeigt aber direkt auf
  `target/release/register_lookup` (mtime 2026-09-15, ohne `run_open`). Kein
  CI-Workflow baut/publiziert das Bin (gemessen). (Schritt: Symlink auf
  `bin/register_lookup` repointen — lokaler Install-Akt.) · `operator-gebunden`

## Wartestellungen (kein Auswahlpunkt)

- **`te-gate` `35451283398`** @`1934c012` — `pending`. (Schritt: `ci_manage view` einmal.) · `wartend`
- **`ci-check` `35456571554`** @`49801083` — `pending` (fremd). (Schritt: `ci_manage view` einmal.) · `wartend`
- **FUGIN-Bulk-Manifestation** — 270 Cubes. (Schritt: `ci_manage view` einmal.) · `wartend`
- **planetary-odf-cdn** — `35351411938` pending. (Schritt: `ci_manage view` einmal.) · `wartend`
- **Scanned-/bild-only-PDFs → `vision`-OCR** — kein Bau nötig. · `pending`

## Diese Session — gearbeitet (git trägt es)

- **Betti-0-Kalibrierung** (Post 2, Operator-Wort „muss auf jeden Fall gemessen
  werden"): Kalibrier-Gates `betti0_calibration_fp_null_q95_stays_below_threshold`
  + `_fn_structured_q05_stays_above_threshold` (je 100 Realisierungen, Quantil
  gegen 0.5) und Fasy-Bootstrap-Gates `betti0_bootstrap_fp_ci_upper...` /
  `_fn_ci_lower...` (`src/mathematikerin/te.rs`). `measure-gates.yml`-Filter
  `betti0` fängt alle. `--port`-Fixture begründet **nicht** gebaut (force-agnostisch).
- **`hdf5.rs` Range-Guard** (`MAX_READ_BYTES`/`MAX_FETCH_BYTES`/`MAX_FETCHES` +
  benannte `Hdf5Note`-Varianten + zwei Tests) — GEDI-L2A-Hang adressiert.
- **`icesat2_atl03_compiler.rs` `--skip`** — echtes `K..K+N` statt immer 0..N;
  strikte Arg-Prüfung (kein stiller Default/Clamp).
- **`bin/register_lookup`** — Wrapper nach `bin/archive_search`-Muster.
- **ttl/LoopCtx** — Council-Verdikt (Weg B): die Konstante bleibt; die behauptete
  Duplikation ist eine gemessene Konflation (Register-ttl 86400 ≠ `VERDICT_STALE_S`
  604800). Kein Wire-Kontakt. Punkt geschlossen.

## Benchmark

- **Betti-0**: `grind-max` (pro/max) — Kalibrier-/Bootstrap-Konstruktion.
  **Ein Fehler gefunden und korrigiert** (Session-Lead): der Bootstrap-Index
  `gate_rng * 0.5 * m` deckte nur die halbe Serie (`gate_rng` liefert `[0,1]`, nicht
  `[0,2)`) — korrigiert auf `gate_rng * m`, `.min(m-1)`.
- **2 rote TE-Gates**: `grind-max` — **leerer Rücklauf**, kein Hunk. Kein
  Doppel-Lauf; der Punkt steht.
- Routine (`hdf5`, `icesat2`, `bin/register_lookup`, CI-Log): `grind-flash`/`general`.
  Architektur (ttl): `council`.
- Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session (uncommittet):** `src/mathematikerin/te.rs`,
  `src/archivar/hdf5.rs`, `tools/harvest/src/bin/icesat2_atl03_compiler.rs`,
  `bin/register_lookup` (neu), `docs/handover/handover-2026-09-19-bau-folge90.md`
  (neu), Move `handover-2026-09-19-bau-folge89.md` → `archiv/`, `docs/handover/post.md`
  (Faltung der vier „An bau"-Zeilen).
- **Fremd (nicht anfassen):** die Hyperscanning-TE-Dateien, `external-state.md`,
  `forschung-folge92`, `entscheid-folge53/54`, die `handover-2026-09-16-*`-Moves.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
