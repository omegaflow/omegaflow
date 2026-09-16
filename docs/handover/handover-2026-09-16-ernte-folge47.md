<!--
  title: Handover — Ernte-Folge 47 (Stand 2026-09-16)
  session: Ernte-Folge 47
  class: handover
  date: 2026-09-16
  sha256: 84f393b49fbb6227ac634a4b6b5c27f489d4f402af07174c5d397e6b3c2d35d1
  status: live
-->
# Handover — Ernte-Folge 47 (2026-09-16)

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

## Register-Rest (aus post.md gefaltet) — der breiteste offene Punkt

- MPC-Shard UnnObs, GHRC-DAAC, Survey §1 (26 Pendings); Register-Digest-Rest
  (Fink/ALeRCE, TDAT/FITS, Akteure, Holdings, Orphan-Verdicts). (Schritt:
  `phi/sources.φ` + CDN je Punkt messen; `register_lookup <term>`.)

## CDN-Manifestation (gated — nach Push + Consent)

- `gh workflow run`: `vtscat-cdn.yml`, `dl3-skymap-cdn.yml` (die drei pending
  sky1-Zeugen-Assets), `emso-cdn.yml` (neu, `tools/harvest/src/bin/emso_compiler.rs`),
  `planetary-odf-cdn.yml` (rosetta-Fixtur), `ionex-cdn.yml`, `cors-cdn.yml`,
  `nh-rex-tnf-cdn.yml`, `flac-cdn.yml`, `bison-cdn.yml`; BGC-Argo-NetCDF,
  folge40-Bündel. Unveränderte Ziele manuell (celestrak-eop, goes, himawari, …)
  + Gaia DR3 XP Voll-Survey. (Schritt: nach Push dispatchen, kein Poll.)

## sky1-Arm — 6 Assets live registriert, 3 pending

- Der Arm ist jetzt erreichbar: `sources.φ` trägt 6 **live** sky1-Assets
  (lhaaso, hawc_2hwc, hawc_3hwc, icecat, antares, gw250207; gemessen 2026-09-16
  HTTP 200) + die 3 Zeugen-Assets (`vtscat_flux`, `hess_dl3`, `magic_dl3`) als
  CDN-Zeilen (404 bis die Workflows laufen). Gate-Test
  `test_parse_sky1_cdn_asset_block` (`src/archivar/tests.rs`). (Schritt:
  nach Push `vtscat-cdn.yml` + `dl3-skymap-cdn.yml`, dann
  `archive_search --verdict` auf die drei Assets.)

## IONEX — Route verdrahtet, Publikationsfenster offen

- Route roh+Token+gzip: `main_flow.rs` ionex-Zweig (URL-Render + Bearer + gunzip)
  + `ionex.rs::build_channels` (Höhe aus dem INX-Band-Header, kein fabrizierter
  Shell-Altitude). Offen: die Quelle nutzt `{yday}` = heute → vor der täglichen
  GIM-Publikation HTTP 404, der Arm wartet ttl 86400. (Schritt: `{yesterday}`-Pfad
  prüfen oder 1..7-Tage-Fallback wie `ionex-cdn.yml`.)

## Voyager ODR — Reader-Arm (Post an bau)

- `sample_count` gemessen (2026-09-16): signed 32-bit **BE** Zweierkomplement
  (`be32(52) as i32` = −299956; 299 999 = DRA-Counter-Max, keine zweite Feldart);
  484 `.ODR` + 484 `.LBL` = 968 Dateien, 1981-08-25T20:11Z–1981-08-26T07:59Z,
  ~14,68 GB; Reader-Arm liegt als Post an bau. (Schritt: bau baut die Serie über
  die 968 Dateien.)

## KASCADE-Grande

- Konto + Event-Tabellen-Format: das Format steht erst nach einem Job fest
  (`blocked_sources.φ`). (Schritt: Konto/Job, dann Format messen.)

## CI-Gate

- Fremde fmt-Diffs (`commit_gate.rs:540`, `aia_ladder_probe`/`trishuli_gauge_probe`,
  `archive_search.rs`) + DRS-FITS-Compile-Bruch (`drs_fits_compiler.rs:93`) —
  Post an die Linien steht. (Schritt: die Linien fixen ihre Dateien.)

## Benchmark

- Reader/Route-Klasse sky1 (parser-def): Doppellauf gemessen 2026-09-16 —
  grind-flash und research-max kommen unabhängig auf **Route B** (CDN-Zeilen in
  `sources.φ`), flash zusätzlich mit Gate-Test → **flash gewinnt**. Geschlossen,
  kein Re-Run. (Schritt: —)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
