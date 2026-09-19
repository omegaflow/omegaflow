<!--
  title: Handover — Bau-Folge 89 (Stand 2026-09-19)
  session: Bau-Folge 89
  class: handover
  date: 2026-09-19
  sha256: 2c6be83cfa6b1117ef539b61af05a410cc7f28f5d85456d0d52d3354f31aa176
  status: live
-->
# Handover — Bau-Folge 89 (2026-09-19)

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

- **HEAD** `b374736c` (research folge89) — ernte (folge90/91) und research
  (folge88/89) haben während der Session committet. Fremd uncommittet:
  `tools/harvest/src/bin/cassini_odf_compiler.rs` (ernte), die
  `handover-2026-09-16-*`-Moves — unangetastet.
- **Postfach** — letzte Ledger-Zeile: CSES-Limadou-Antwort (Dr. Sotgiu,
  2026-09-16): Zugang wird nach Website-Umbau neu geregelt → `wartend` bis die
  neuen Instruktionen erscheinen (operator-gebunden, nicht Bau). `post.md` trug
  zwei „An bau"-Zeilen (ernte: hdf5/icesat2; research: multi_force_te_probe) —
  gefaltet, gelöscht.
- **CI** — `te-gate` `35427414837` @`fe6bdb2b` noch `in_progress` (~4,5 h, alter
  Stand). `ci-check` `35427572159` @`fe6bdb2b`: 1377 grün / 7 rot (die Basis vor
  diesem Fix, s. unten). `health-check` `35436939441` failure (fremd).

## TE-Gate-Fix — gebaut (uncommittet), CI-Messung ausstehend · `wartend`

Der per-Lag-Commit `fe6bdb2b` ließ 7 Tests rot: 6 TE-Gates
(`gate_fpr_autocorrelation_{restricted,arx,block-ksg}_null_binned_n_surr_200`,
`flare_envelope_conditional_keeps_true_coupling`,
`binned_null_does_not_leak_on_multi_driver_confound`,
`synthetic_dag_recovers_known_direction`) + den Relay-Test
`weberin_verdicts::live_verdicts_drops_only_the_expired_lines`. Gebaut:

- **Fix A (Kern):** der ARX-Null verlor im per-Lag-Commit die Kondensor-Lag-Historie
  (`c.series[t - c.lag]`); `arx_null_cond_series` (te.rs) dedupliziert nach Serie,
  der Fit expandiert jede distinkte Serie auf `0..=max_lag` — Full-History-Null,
  Schätzer behält per-Lag. Regressionen `synthetic_dag`/`binned_null` kehren auf
  die Basis-Werte zurück.
- **Fix C (Council, angenommen):** die feste 2pp-Anstiegsdecke war fabriziert
  (1.19σ des Binomialrauschens); ersetzt durch `fpr_rise_sigma_test` mit
  `FPR_RISE_Z = 3.0` (3σ der Binomialdifferenz der gemessenen Zellen) + Kalibrier-Gate
  `gate_fpr_rise_calibration` (Binomial-MC, H0-Trefferrate ≈ 0.27 % zweiseitig;
  folge86-Leck 9→18 % bleibt rot).
- **n_surr 10→256** in flare/DAG/multi-driver-leak.
- **Relay-Test-Fixture** korrigiert (apophis abgelaufen, ceres/vesta lebendig).
- **TE-API-Bruch nachgezogen:** `multi_force_te_probe.rs`,
  `corona_conditional_probe.rs`, `placebo_pair_eeg_probe.rs` auf `LaggedCond`
  (E0308 seit `fe6bdb2b`).
- `cargo check --all-targets` (core) + `cargo check -p omegaflow-measure
  --all-targets`: 0 Fehler, 0 Warnungen.

**Nächster Schritt:** `/commit` + Push; dann liest der push-getriggerte `ci-check`
die 7 Gates, und `gh workflow run te-gate.yml` misst die n=1000-Kurven +
`coupling_scan_te_over_thr_n1000` (flare-Power-Frage) + `fpr_bins_sweep_n2000`.
Ergebnis einmal beim nächsten Pass (`ci_manage view`), nie erwartet. · `wartend`

## Offen

- **Betti-0 `betti0_persistence`** — Fasy-Bootstrap-Gate + Null-Verteilungs-Quantil
  in CI (`measure-gates`, `betti0_probe.rs`); 0.5-Schwelle unkalibriert; `--port`
  force-Gate-Fixture. (Schritt:
  `docs/handover/archiv/handover-2026-09-18-entscheid-folge52.md`
  §Gremium+Wissenschaft.) · `pending`
- **Eine-Quelle-Ideal `ttl`** — `VERDICT_STALE_S` (weberin_verdicts.rs:5) spiegelt
  die Register-`ttl` + `BIN_TTL_S`; `ttl` durch den `LoopCtx` reichen. (Schritt:
  `LoopCtx`-Definition lesen, `ttl`-Feld + Durchreichen in den Verdict-/Compiler-Pfad;
  Council bei der Wire-Frage.) · `pending`
- **`register_lookup --open` Binary veraltet** — Quelle implementiert `run_open`
  (`b67f5cae`), PATH- und `target/release`-Binary geben die `--live`-Usage.
  (Schritt: Workflow ermitteln, der die Tools-Binaries baut/publiziert.) · `pending`
- **`hdf5.rs` `gather_messages` Guard** (Post, ernte) — der eager `parse_fetch`
  traversiert den ganzen Objektgraphen (v1-Cont-Loop `hdf5.rs:652`, v2 `:715`);
  GEDI L2A hängt daran (Run `35351460528`, 180 min ohne Ausgabe). (Schritt:
  `sgrep "gather_messages" src/archivar/hdf5.rs` — globaler Range-Guard im
  `Hdf5WindowReader` + Längen-Kappe.) · `pending`
- **`icesat2_atl03_compiler.rs` `--limit`/`--skip`** (Post, ernte) —
  `granules.truncate(limit)` nimmt immer die ersten N; 215 Granules à ~100 min,
  der ci_watchdog killt bei 2× Median 71 s. (Schritt:
  `sgrep "truncate" tools/harvest/src/bin/icesat2_atl03_compiler.rs`.) · `pending`

## Wartestellungen (kein Auswahlpunkt)

- **`te-gate` `35427414837`** — in_progress, alter Stand. (Schritt: `ci_manage view` einmal.) · `wartend`
- **FUGIN-Bulk-Manifestation** — 270 Cubes. (Schritt: `ci_manage view` einmal.) · `wartend`
- **planetary-odf-cdn** — `35351411938` pending. (Schritt: `ci_manage view` einmal.) · `wartend`
- **Scanned-/bild-only-PDFs → `vision`-OCR** — kein Bau nötig. · `pending`

## Benchmark

- TE-Atom: `grind-max` (pro/max) — härtestes Atom (Urteil UND Schreiben in einem
  Kontext): per-Lag-Null-Fix + Council-umgesetzter σ-Test. Routine (Relay-Fixture,
  drei Probe-Bins): `grind-flash`. Kein Doppel-Lauf; die flash/pro-Klasse ist für
  dieses Atom nicht doppelt gemessen. Burn: `session_burn`.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session (uncommittet):** `src/mathematikerin/te.rs`,
  `src/gate/commit_gate.rs`, `src/gate/commit_gate_vocab.json`,
  `src/archivar/weberin_verdicts.rs`,
  `tools/measure/src/bin/{multi_force_te_probe,corona_conditional_probe,placebo_pair_eeg_probe}.rs`,
  `docs/handover/handover-2026-09-19-bau-folge89.md` (neu), Move
  `handover-2026-09-19-bau-folge88.md` → `archiv/`, `docs/handover/post.md`
  (Faltung der zwei „An bau"-Zeilen).
- **Fremd (nicht anfassen):** `tools/harvest/src/bin/cassini_odf_compiler.rs` (ernte),
  die `handover-2026-09-16-*`-Moves. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
