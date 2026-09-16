<!--
  title: Handover — Ernte-Folge 51 (Stand 2026-09-16)
  session: Ernte-Folge 51
  class: handover
  date: 2026-09-16
  sha256: 18020156e319f5b5b28c136d37fe8cc8e021a9d4c8ad819b3fdb71dcfdf74227
  status: live
-->
# Handover — Ernte-Folge 51 (2026-09-16)

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

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## sky1-Zeugen — VTSCat-Compile schreibt nichts (Parser-Gap, härtester undatiert)

- Gemessen 2026-09-16: `vtscat-cdn.yml` run 35137986016 success, aber das Log
  endet „no VTSCat source carries a measured dnde — the asset stays unwritten
  (0 honored)"; alle 360 ECSVs „no measured dnde at 1 TeV — source skipped", viele
  referenzierte YAML-Pfade 404 auf `raw.githubusercontent.com/.../main/...`.
  `vtscat_flux.sky1` fehlt am CDN (Release `github.com`), das Issue
  „vtscat-cdn: vtscat_flux.sky1 returned void" ist bereits offen.
  `hess_dl3`/`magic_dl3`: `dl3-skymap-cdn.yml` (hess) success, aber das
  `opendata.magic.pic.es`-Release fehlt; alle drei Zeugen-URLs `--verdict` = HTTP 404.
  (Schritt: `src/archivar/vtscat.rs` ECSV-Reader + 1-TeV-Auswertung und die
  YAML-Pfadauflösung diagnostizieren — warum trägt keine ECSV einen dnde@1TeV?;
  danach die 3 `sources.φ`-Zeilen (format sky1, Muster `test_parse_sky1_cdn_asset_block`,
  Einheiten `m-2.s-1.tev-1`/`tev`) registrieren, wenn die Assets manifestieren.)

## DEMETER/CDPP — Registrierung braucht die Konsumenten-Verdrahtung

- Der Key öffnet (gemessen, in dieses Register gefaltet); `demeter_compiler.rs`,
  `demeter-cdn.yml`, `src/archivar/demeter.rs` stehen. Der Epochen-Bug in
  `demeter_compiler.rs` `unix_to_ym` (fehlender `+719468`-Shift gegenüber
  `days_from_civil`) ist in dieser Session gefixt (`cargo check` 0/0) — die 77
  bestehenden Shards `demeter_isl_0034…` tragen daher ein −1970-Jahr und brauchen
  eine Re-Aggregation. Die `sources.φ`-Registrierung bleibt blockiert:
  `series_parse_bin`/`series_component_name` (`extract.rs`) und das
  `main_flow`-Routing tragen keinen `demeter_isl`-Arm → ein Eintrag wäre ein toter,
  wertloser Kanal; ein einzelner kanonischer CDN-Dateiname fehlt (Monats-Shards).
  (Schritt: `demeter_isl`-Konsument in `extract.rs` verdrahten + `field`-Namen,
  dann Shard-`url`-Zeilen registrieren; neue Orders = Dritt-Akt → Post an `entscheid`.)

## CDN-Manifestation — Batch dispatcht, zwei Nachzügler

- Der Batch (vtscat, dl3-skymap, emso, planetary-odf, ionex, cors, nh-rex-tnf,
  flac, bison, mpcobs-shard, argo-bgc + folge40-Bündel) ist 2026-09-16 dispatcht,
  meist success; die MPC-UnnObs-Shards (2, `mpcobs-unnobs-1228354383-1261580867.bin`
  1 076 633 450 B + `…-1261580867-1412649780.bin` 212 977 450 B) sind gemessen und
  als `url`-Zeilen neben `phi/sources.φ:1899` registriert. Offen:
  `planetary-odf-cdn.yml` (cancelled) und `argo-bgc-cdn.yml` (pending) erneut
  dispatchen. (Schritt: `gh workflow run planetary-odf-cdn.yml argo-bgc-cdn.yml`.)

## Zenodo sha256 — Quaoar/TNBFits (Zenodo-Backend 504)

- `multimoon-1.0.zip` registriert. Quaoar (`zenodo.21185812`, 572 467 032 B) und
  TNBFits (`zenodo.10620251`, 14 232 134 885 B) sha256 `pending`: Zenodo HTTP 504
  service-weit, TNBFits zusätzlich 403 (Netz-Block). (Schritt: bei Rückkehr
  streamend `curl -sL <url> | sha256sum` + md5-Abgleich — Quaoar 420a1e94…,
  TNBFits a1aada71…; TNBFits 13,25 GiB > 2-GiB-Asset-Limit → CI-Stream + Granulat;
  dann Register.)

## GHRC — Familie pending, kein Produkt blocked account

- Der EDL-Token öffnet `ghrcw-protected`; ISS LIS/OTD registriert. Offen: GOES GLM L1B
  (`C3457558852-GHRC_DAAC`), TRMM LIS (`C1983762329-GHRC_DAAC`) — Compiler + CDN
  ungebaut; vorher die öffentliche NOAA-AWS-Route messen. (Schritt: GLM/TRMM-Route
  messen, Compiler bauen, registrieren.)

## KASCADE-Grande — Format erst nach einem SSO-Job

- DataShop ist Keycloak-SSO/JS, kein URL/POST-Endpoint; `blocked_sources.φ:63` korrekt.
  (Schritt: mit dem `omegaflow`-Konto einen Minimal-Job fahren, ASCII-Download,
  Spaltenlayout messen — `archive_search --playwright --headed` oder Operator.)

## Fink — Photometrie als benanntes Pending

- Der Konus lebt (HTTP 200, POST-only, `witnesses.φ:17`); die em-Photometrie
  (`r:psfFlux/r:psfFluxErr/r:midpointMjdTai`) ist via POST-columns abrufbar, der
  Compiler fordert nur ra/dec. (Schritt: `skydirection_compiler`-Scope um die
  Flux-Spalten erweitern — benanntes Pending, kein Format-Gap.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
