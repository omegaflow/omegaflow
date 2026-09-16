<!--
  title: Handover — Bau-Folge 47 (Stand 2026-09-16)
  session: Bau-Folge 47
  class: handover
  date: 2026-09-16
  sha256: 26e25eb107900e02f2f08dac34fa88c960db28682f25debd9d66b6d06fc84e0c
  status: live
-->
# Handover — Bau-Folge 47 (2026-09-16)

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

## Parser-Magic-Gaps — härtester undatiert

- Gap 1: `Frame::Data` fehlt in `src/archivar/types.rs:282`; `frame_body_name`
  (369) ohne Data-Arm. (Schritt: types.rs — Data-Variante + Arm.)
- Gap 8: `flush!()`-Gate in `src/archivar/parse.rs:110,1361`. (Schritt:
  parse.rs — Gate messen und schließen.)
- Gap 12: Category/Group-Vererbung — in `src/` und `tools/harvest` per `sgrep`
  nicht lokalisierbar (Ernte markiert: Operator). (Schritt: erste Messung —
  PDS4-Parser-Pfad in harvest suchen.)

## Katalog-Konsumenten (Reader gebaut, Consumer fehlt)

- VLASS `catalog_vlass_tap_component`/`_source` (VLST/VLCT, 16-B-Header,
  40/48-B-Records, freq 3,0 GHz) → `catalog_*`-Pfad, nicht `series_parse_bin`.
  (Schritt: Modul + `catalog_*`-Muster `dastcom`/`tycho`.)
- `catalog_dcom5` registriert, kein Modul. (Schritt: Modul + Reader.)
- 10 deklarierte, nie aufgerufene Module in `src/archivar/mod.rs`: `ossos`,
  `gaia_sso`, `des_y6`, `mpcorb`, `las`, `twomrs`, `rinex`, `quakeml`,
  `nexrad`, `bidsleep`. (Schritt: je Modul Consumer in `main_flow.rs` verdrahten.)

## Epochen-Anker — CI-Workflow-Notizen stale

- `.github/workflows/{noaa-cdo,gedi,icesat2,swot}-cdn.yml` tragen veraltete
  Strides/Feldnamen (`date_unix`, „56 B = 7 × f64"); die Compiler tragen jetzt
  `date_tdb` bzw. `anchor_tdb`. (Schritt: Workflow-Notes an die neuen Records
  anpassen.)

## ODR — Rest

- Voyager ODR: CDN-Asset `voyager_odr.bin` + Serie über die 968 `.ODR`-Dateien.
  Galileo 12-bit-Packing (Layout gemessen, kein Bestand, `eight_bit`-Flag).
  Galileo `year_full` 00–89 ungemessen. Decimation>1 nur aus der Spec-Formel.
  Counts sind unitless (kein Doppler/Spannung im Bundle, Befund Bau31).
  (Schritt: CDN-Dispatch + Label-Serie über `INDEX.TAB`.)

## Star-Katalog — CI-Dispatch

- `tap_compiler` schreibt 44 B inkl. rv (gemessen); CDN `dr3_stars.bin` ist 40 B
  (stale, vor der rv-Spalte gebaut). (Schritt: `gh workflow run gaia-cdn.yml` —
  rekompiliert mit `rv:t.radial_velocity` + `--ci-mode`.)

## TE-Gate / CI

- `te-gate` run `35092997862` (Schritt 4, shift-null sweep); residual/ksg > 8 %
  → gemessenen FPR-Boden als Register-Zeile tragen. `ci-check` run `35093194650`
  test-Job. (Schritt: Lauf lesen, kein Polling.)

## DRS-FITS

- Arm steht; kein DRSF-Bin am CDN. Index-Zeit-Drift bei Reihen-Lücken (24-B-Stride
  ohne per-Row-Zeit), Anker `at earth`. (Schritt: CDN-Dispatch
  `.github/workflows/drs-fits-cdn.yml` mit gemessenem Granulat.)

## mirror-research

- „~2.300-Quellen" ist ungemessen; gemessen: 412 Quellen, 203 Live-Releases,
  67 Source-Netlocs, 200 Release-Netlocs. (Schritt: Umfang gegen
  `docs/specs/cdn-ziel-schema.md` §1 messen.)

## HRV/Puls-Oszillator-Bindung

- Physischer ESP32-Träger (on hold) + End-zu-End-Test. (Schritt:
  `src/archivar/hrv.rs`.)

## Format-Gate

- CI-`format` rot auf fremden Dateien (`src/gate/commit_gate.rs:540`,
  `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`,
  `tools/utils/src/bin/archive_search.rs`). (Schritt: fremde Linie.)

## Benchmark (gemessen 2026-09-16)

- Routine-Audit (registrierte-aber-inerte Serien-Formate), Compiler-Anker,
  Star-Stride, ODR-Verdrahtung: `flash` hätte gereicht — `pro`/`max` brachten
  kein anderes Ergebnis. ODR-Roh-Sample-Dekodierung (RSC-11-6/PDS3) ist das
  harte Atom: `max` gerechtfertigt (drei Kreuzverifikationen gegen die
  PDS3-Label-Zeiten).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
