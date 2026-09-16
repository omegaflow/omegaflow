<!--
  title: Handover — Bau-Folge 49 (Stand 2026-09-16)
  session: Bau-Folge 49
  class: handover
  date: 2026-09-16
  sha256: c43fe19bb3b9da4d4ffe73bf0af237238d54f987d90d4ed24b3d2d64d9a023b2
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

- Gemessen (te-gate run 35092997862 @ bf2423a5, `docs/zustand/external-state.md`):
  residual/ksg 19,51 % @ a=0,9, phase/ksg 8,52 %, restricted/binned 10,71 % (Boden 8 %);
  block/binned +2,47pp, block/ksg +2,75pp (Grenze 2pp); shift_sweep grün (binned/ksg ≤ 6,87 %).
  Panik-Sites `src/mathematikerin/te.rs:3185` (8 %-Boden) / `:3206` (2pp-Anstieg).
  (Schritt: den Null-Erzeuger/Estimator in `te.rs` gegen die a-Achse messen, Issue
  „the n=1000 FPR gate carries a finding" lesen; Fix ist ein Architektur-Atom.)

## Spektral-Achse — `parse.rs`-Rest-Literale

- `SPECTRAL_NO_BAND` (`src/archivar/spectral.rs:6`); 33 Schreibstellen in 8 Modulen
  ersetzt (copernicus, cors, geo, noaa_nodd, parse:883/891, relay, rinex, spatial).
  Offen: `parse.rs` FieldConfig-Literale `:369,393,413,432,463,535,555,575,738,798,1167`
  (+ bin_width-Paare) tragen weiter rohes `freq: 0.0, bin_width: 0.0`.
  (Schritt: dieselbe No-Band-Semantik prüfen, dann `crate::spectral::SPECTRAL_NO_BAND` setzen.)

## Star-Katalog — CDN-Run lesen

- `gaia-cdn.yml` dispatcht (run 35113568241) — rekompiliert 44 B inkl. `rv`.
  Offen: Lauf lesen, CDN `dr3_stars.bin` auf 44 B verifizieren.
  (Schritt: `gh run view 35113568241`, kein Polling.)

## DRS-FITS — CDN-Dispatch

- Der `drs_fits_compiler.rs`-Blocker ist am HEAD weg (gemessen). Offen:
  `.github/workflows/drs-fits-cdn.yml` noch nicht dispatcht; Index-Zeit-Drift bei
  Reihen-Lücken (24-B-Stride ohne per-Row-Zeit), Anker `at earth`.
  (Schritt: Workflow mit gemessenem Granulat dispatchen.)

## Katalog-Konsumenten — Feld-Semantik offen

- `bidsleep` verdrahtet (`extract.rs:54` + Komponenten `:223`, `main_flow.rs:2087`);
  `rinex`/`dcom5`/`nexrad` waren am HEAD bereits verdrahtet (Handover-Claim stale,
  gemessen 2026-09-16).
- Offen: VLASS (kein src-Modul, phi ohne Distanz/Parallaxe → 3D-Platzierung unbestimmt),
  `catalog_dcom5` (CometRec ohne GM), `des_y6` (einzige Größe `sigma_m`), `ossos`
  (Kepler ohne GM), `twomrs` (kein Distanzfeld), `las` (Frame/Anker fehlt),
  `nexrad_level2` (44-B-Bin ohne Standort).
  (Schritt: je Quelle die Feld-Semantik messen oder als `descoped`/`pending` benennen.)

## ODR — Serie/Granulat und Semantik

- `voyager-odr-cdn.yml` existiert (Einzeldatei, URL-Input); der Reihen-Leser-Arm
  (`extract.rs:51` `voyager_odr`/`galileo_odr`, `tests.rs:8048`) ist gebaut + getestet.
- Offen: Granulat für die 484-Datei-Serie (13,61 GB > GitHub-2-GB-Limit) —
  Architektur-Entscheid; `sample_count`-Semantik (Spec ≤ 299999 gegen gemessene
  Record-1-Werte ~4,29e9, Offset 52 bestätigt, `blocked_sources.φ:34`); Galileo
  12-bit kein Record im PPI-Archiv (real-record-unverifiziert); `year_full` 00–89
  bleibt `None` (`galileo_odr.rs:141`); Voyager Decimation>1 unverifiziert (alle
  6000 CDN-Records Code 7 = Ratio 1).
  (Schritt: Granulat wählen + Workflow erweitern; die Semantik-Punkte messen oder
  als pending benennen.)

## HRV/Puls-Oszillator-Bindung

- Physischer ESP32-Träger (on hold) + End-zu-End-Test. (Schritt: `src/archivar/hrv.rs`.)

## smail_recv — leerer Body bei verschachteltem MIME

- Rekursiver MIME-Abstieg (`collect_text`, `tools/service/src/bin/smail_recv.rs:209`)
  gebaut + getestet; Root cause (leerer Sotgiu-Reply) unbestätigt.
  (Schritt: bei erneutem leerem Body die Worker-`message.raw`-Quelle messen —
  `cloudflare/email_worker.js`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
