<!--
  title: Handover — Bau-Folge 48 (Stand 2026-09-16)
  session: Bau-Folge 48
  class: handover
  date: 2026-09-16
  sha256: e31e9a2608d3abd5857cce150f5c7f2b00844c1dc545c4c801255a5830038eea
  status: live
-->
# Handover — Bau-Folge 48 (2026-09-16)

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

## Parser-Magic — Rest (härtester undatiert)

- Gap 12: Category/Group-Vererbung — in `src/` und `tools/harvest` per `sgrep`
  nicht lokalisierbar. Erste Messung: PDS4-Parser-Pfad in `tools/harvest`
  suchen. (Schritt: `sgrep "category|group" --root tools/harvest`, dann den
  PDS4-Parser-Pfad öffnen.)

## Spektral-Achse — Pad-Schreibstellen benennen (Council 2026-09-16)

- Der Rat (Architektur-Verdikt, gemessen): der 0.0-Sentinel für `freq`/`bin_width`
  ist keine Fabrikation — der Wire-Pad bleibt, kein Option-Plumbing, kein
  Gate-Fixture; die Lesestellen tragen die Semantik (`omega.rs` Band-Gate,
  `spectral.rs` `band_overlap`/`color_for_ci`). Die Klammer in `AGENTS.md` und
  `docs/concepts/archivar-mathematikerin.md` ist berichtigt: `freq`/`bin_width`
  tragen die benannte Paar-Semantik, statt „0 ist ein realer Wert".
- Offen: die stummen 0.0-Literale an den Schreibstellen benennen — `parse.rs:883,891`
  und die Modul-Stellen (copernicus, rinex, noaa_nodd, relay, radio, ztf, spatial
  `Sample`, geo, cors, maxi, bl_narrowband, rixs) — eine benannte Konstante
  (z. B. `SPECTRAL_NO_BAND`, Name = Implementation). (Schritt: Konstante in
  `src/spectral.rs` + je Modul ein Ein-Token-Hunk, nur eigene Hunks.)

## Katalog-Konsumenten (Reader gebaut, Consumer fehlt)

- VLASS `catalog_vlass_tap_component`/`_source` (VLST/VLCT, 16-B-Header,
  40/48-B-Records, freq 3,0 GHz) → `catalog_*`-Pfad. (Schritt: Modul + Muster
  `catalog_dastcom` `main_flow.rs:1069`, Einfügepunkt `main_flow.rs:1514`.)
- `catalog_dcom5` registriert, kein Modul. (Schritt: Modul + Reader.)
- Ungenutzte Module — gemessener Ist (2026-09-16): `quakeml::parse_quakeml` wird
  aufgerufen (`extract.rs:3248`, `Extract::QuakeMlEvents`); `gaia_sso`/`mpcorb`
  in `src/weberin.rs:5-6`. Ohne main_flow-Consumer: `ossos`, `des_y6`, `twomrs`,
  `las`, `rinex`, `nexrad`; `bidsleep` → sauberster Punkt ist ein Arm in
  `extract.rs:4-53` (`"bidsleep" => bidsleep::parse_bin(bytes)`), nicht ein
  Katalog-Block. (Schritt: je Modul Glue `build_*_samples` in `spatial.rs` nach
  Muster `build_asteroid_samples:171` / `build_star_samples:316`, dann
  `main_flow.rs`-Block.)

## Epochen-Anker — CI-Workflow-Notizen stale

- `.github/workflows/{noaa-cdo,gedi,icesat2,swot}-cdn.yml` tragen veraltete
  Strides/Feldnamen (`date_unix`, „56 B = 7 × f64"); die Compiler tragen jetzt
  `date_tdb` bzw. `anchor_tdb`. (Schritt: Workflow-Notes an die neuen Records
  anpassen.)

## ODR — Rest

- Voyager ODR: CDN-Asset `voyager_odr.bin` + Serie über die 968 `.ODR`-Dateien.
  Galileo 12-bit-Packing (Layout gemessen, kein Bestand, `eight_bit`-Flag).
  Galileo `year_full` 00–89 ungemessen. Decimation>1 nur aus der Spec-Formel.
  Counts sind unitless (kein Doppler/Spannung im Bundle). (Schritt: CDN-Dispatch
  + Label-Serie über `INDEX.TAB`.)

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
  ohne per-Row-Zeit), Anker `at earth`. `tools/harvest/src/bin/drs_fits_compiler.rs:93`
  (`{epoch:.3f}` = ungültiger Format-Trait) blockt `cargo check -p omegaflow-harvest`
  (Post-Zeile an die DRS-FITS-Linie). (Schritt: CDN-Dispatch
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

## Empfehlung für die nächste Session (gemessen 2026-09-16)

- **Handover zuerst gegen den Baum prüfen, bevor irgendetwas anderes getan wird.**
  Die offenen Punkte werden gegen HEAD gemessen (nicht gegen den Handover-Text
  geglaubt). Diese Session hat genau so gearbeitet: die Punkte Gap 1 + Gap 8
  standen als offen im Handover, waren am HEAD aber bereits geschlossen
  (Commit `f8a673ee`) — die Messung vor der Arbeit hat einen ganzen unnötigen
  Code-Atom verhindert.
- **Benchmark-Klassen sind geschlossen — kein erneuter Benchmark nötig.** Der
  nächste Planungs-Pass zitiert diese Zeile, statt die Klasse neu zu fahren; nur
  eine gemessen falsche oder unvollständige `flash`-Antwort öffnet sie wieder
  (dann den Grund als neue Zeile). Gemessen: Parser-Gap-Messung (Gap 1 + Gap 8)
  `flash` genügt — kein Code-Bedarf, die Staleness war die Messung;
  Katalog-Konsumenten-Recon `flash` (`general`); Rat (Wire-Pad `freq`/`bin_width`)
  `council` (pro/max) gerechtfertigt — Architektur-Verdikt, kein `flash`-Job.
- **Agenten-Zuordnung (flash-first, gemäß Kosten-Leiter)** — die nächste Session nimmt:
  - Gap 12 (erste Messung, PDS4-Pfad in `tools/harvest`) → `grind-flash`.
  - Spektral-Pad-Schreibstellen benennen (je Modul ein Ein-Token-Hunk) → `grind-flash`.
  - Katalog-Konsumenten verdrahten (mechanisches Glue) → `grind-flash`; VLASS-Curation,
    falls Urteil nötig → `grind-pro`.
  - Epochen-Anker CI-Notes, mirror-research-Umfang → `grind-flash` / `general` (flash).
  - ODR-Roh-Sample-Dekodierung (RSC-11-6/PDS3) → `grind-max` (härtestes Atom, Urteil
    und Schreiben in einem Kontext).
  - Star-Katalog-/DRS-FITS-CDN-Dispatch, TE-Gate-Lauf lesen → die Session selbst
    (`gh workflow run` / `gh run view`), kein Agent.
  - HRV/ESP32, Format-Gate → operator-gebunden bzw. fremde Linie.
  - Architektur-/Abschluss-Entscheidungen → `council` (pro/max).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
