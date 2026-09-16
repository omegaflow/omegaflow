<!--
  title: Handover — Bau-Folge 49 (Stand 2026-09-16)
  session: Bau-Folge 49
  class: handover
  date: 2026-09-16
  sha256: 36228b4dd0747489982935f53ee257f3a8b981e0f201b4c2149ca04768f7ec65
  status: live
-->
# Handover — Bau-Folge 49 (2026-09-16)

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
Der Planungs-Pass nennt **einen schweren und fünf leichte** offene Punkte (der
schwere ist der erste offene Abschnitt, die leichten sind mechanisch
schließbar); die Session arbeitet beide ab.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## TE-Gate n=1000 FPR — der Null leckt Autokorrelation (härtester undatiert)

- Gemessen (te-gate run 35092997862 @ bf2423a5, Register `docs/zustand/external-state.md`):
  residual/ksg 19,51 % @ a=0,9, phase/ksg 8,52 %, restricted/binned 10,71 % (Boden 8 %);
  block/binned +2,47pp, block/ksg +2,75pp (Grenze 2pp). shift_sweep grün (binned/ksg ≤ 6,87 %).
  Panik-Sites `src/mathematikerin/te.rs:3185` (8 %-Boden) / `:3206` (2pp-Anstieg).
  (Schritt: den Null-Erzeuger/Estimator in `te.rs` gegen die a-Achse messen, Issue
  „the n=1000 FPR gate carries a finding" lesen; Fix ist ein Architektur-Atom.)

## ODR — Rest (Voyager-Serie, Galileo)

- Voyager-Serie gebaut (`src/archivar/voyager_odr.rs` Mehrdatei-Erbschaft + Record-Lücken-Gate,
  `tools/harvest/src/bin/voyager_odr_compiler.rs --index/--base`), CDN-Asset `voyager_odr.bin`
  existiert. Offen: kein ODR-CDN-Workflow — 484 `.ODR` = 13,61 GB überschreitet das
  GitHub-2-GB-Asset-Limit; Granulat (je Datei / Teilserie / Einzel-Asset) ist ein
  Architektur-Entscheid. (Schritt: Workflow nach Muster `drs-fits-cdn.yml`, Granulat wählen.)
- Galileo 12-bit-Dekoder gebaut (`src/archivar/galileo_odr.rs`, SIS-belegt RSC11_11 Fig. 4),
  aber kein 12-bit-Record im PPI-Archiv (27 Dateien quer aller Volumes alle 8-bit) —
  real-record-unverifiziert. `year_full` 00–89 existiert nicht im Archiv (bleibt `None`,
  `galileo_odr.rs:141`). Voyager Decimation>1 unverifiziert (alle 6000 CDN-Records Code 7 = Ratio 1).
  `sample_count`-Semantik offen (`blocked_sources.φ:34`). (Schritt: Archiv-Erweiterung oder
  als benanntes pending belassen.)

## Katalog-Konsumenten — Feld-Semantik offen

- `bidsleep` verdrahtet (`extract.rs:54` + Komponenten `:223`, `main_flow.rs:2087`).
  `rinex`/`dcom5`/`nexrad` waren am HEAD bereits verdrahtet (Handover-Claim stale,
  gemessen 2026-09-16).
- Offen (Feld-/Distanz-Semantik, kein mechanisches Glue): VLASS (kein src-Modul,
  phi ohne Distanz/Parallaxe → 3D-Platzierung unbestimmt), `catalog_dcom5` (CometRec
  ohne GM), `des_y6` (einzige Größe `sigma_m`), `ossos` (Kepler ohne GM), `twomrs`
  (kein Distanzfeld), `las` (Frame/Anker fehlt), `nexrad_level2` (44-B-Bin ohne Standort).
  (Schritt: je Quelle die Feld-Semantik messen oder als `descoped`/`pending` benennen.)

## Spektral-Achse — Rest-Literale

- `SPECTRAL_NO_BAND` in `src/archivar/spectral.rs:7`; 33 Schreibstellen in 8 Modulen
  ersetzt (copernicus, cors, geo, noaa_nodd, parse:883/891, relay, rinex, spatial).
  Offen: dieselben No-Band-Pads in den `FieldConfig`-Literalen von `parse.rs` außerhalb
  883/891 (`:369,393,413,432,463,535,555,575,738,798,1167` + bin_width-Paare) bleiben
  stumm. (Schritt: prüfen, ob sie dieselbe Semantik tragen, dann ersetzen.)

## Star-Katalog — CDN-Run lesen

- `gaia-cdn.yml` dispatcht (run 35113568241) — rekompiliert 44 B inkl. `rv`.
  Offen: Lauf lesen, CDN `dr3_stars.bin` auf 44 B verifizieren.
  (Schritt: `gh run view 35113568241`, kein Polling.)

## DRS-FITS — CDN-Dispatch

- Der `cargo check -p omegaflow-harvest`-Blocker (`drs_fits_compiler.rs:93`) ist am HEAD
  weg (gemessen). Offen: `.github/workflows/drs-fits-cdn.yml` noch nicht dispatcht;
  Index-Zeit-Drift bei Reihen-Lücken (24-B-Stride ohne per-Row-Zeit), Anker `at earth`.
  (Schritt: Workflow mit gemessenem Granulat dispatchen.)

## HRV/Puls-Oszillator-Bindung

- Physischer ESP32-Träger (on hold) + End-zu-End-Test. (Schritt: `src/archivar/hrv.rs`.)

## Descoped (gemessen 2026-09-16)

- Parser-Magic Gap 12 (Category/Group-Vererbung): descoped — es existiert kein
  PDS4-Label-Parser im Baum (`sgrep "category|group"`/`Discipline_Area`/`pds4` in
  `src/` + `tools/harvest` leer; einziger XML-Parser `quakeml.rs`), keine Quelle in
  `phi/sources.φ` und kein Consumer verlangt ihn. Nie gebaut, nicht gebraucht.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
