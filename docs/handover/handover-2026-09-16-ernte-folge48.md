<!--
  title: Handover — Ernte-Folge 48 (Stand 2026-09-16)
  session: Ernte-Folge 48
  class: handover
  date: 2026-09-16
  sha256: 51654c3af2e493688b6fea756dc5b5e9fbdf14a717d3b2327bc7e476893afb3b
  status: live
-->
# Handover — Ernte-Folge 48 (2026-09-16)

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

## MPC-Shard UnnObs — der Decoder parst die moderne Packform nicht (härtester undatierter Punkt)

- Der CI-Lauf `mpcobs-shard-cdn.yml` (run 34788941489) schrieb **einen** Shard
  `mpcobs-unnobs-0-1.bin` (1 289 610 900 B > 1-GiB-Budget): `decode_packed_number`
  (`tools/harvest/src/bin/mpcobs_compiler.rs:20-31`) parst die moderne gepackte
  provisorische Bezeichnung (`K09A0…`, base62-Zyklus) nicht → alle Nummern 0 → kein
  Split. Der kaputte Shard wurde vom CDN-Release `minorplanetcenter.net` entfernt
  (2026-09-16, eigener Zustand); die `sources.φ`-Registrierung bleibt zurückgehalten.
  (Schritt: Decoder um die moderne Packform + Fixtur aus der echten UnnObs-Zeile
  erweitern; `gh workflow run mpcobs-shard-cdn.yml` re-dispatchen; Shard-Zahl/-Größen
  messen — gemessen 25 792 218 Records, 164 218 skipped, 2 102 471 316 B
  dekomprimiert; dann die url-Zeilen eintragen.)

## Zenodo sha256 — Quaoar/TNBFits (Zenodo-Backend 504)

- `multimoon-1.0.zip` registriert (`sources.φ`, format reference, sha256 `07b26a4f…`;
  Software, kein Messdatensatz — Rat bestätigt). Quaoar (`zenodo.21185812`,
  572 467 032 B) und TNBFits (`zenodo.10620251`, 14 232 134 885 B) sha256 `pending`:
  Zenodo antwortet service-weit HTTP 504 (2026-09-16); TNBFits zusätzlich 403
  (Netz-Block). (Schritt: bei Zenodo-Rückkehr streamend `curl -sL <url> | sha256sum`
  + md5-Abgleich — Quaoar 420a1e94…, TNBFits a1aada71…; TNBFits 13,25 GiB >
  2-GiB-Asset-Limit → CI-Stream + Granulat; dann Register.)

## GHRC — Familie pending, kein Produkt blocked account

- Der EDL-Token (`EARTHDATA_EDL_TOKEN`, `.secrets.local`) öffnet `ghrcw-protected`
  (401 anonym → 200); ISS LIS/OTD registriert (`sources.φ:6543`). Offen: GOES GLM L1B
  (`C3457558852-GHRC_DAAC`), TRMM LIS (`C1983762329-GHRC_DAAC`) — Compiler + CDN
  ungebaut; vorher die öffentliche NOAA-AWS-Route messen. (Schritt: GLM/TRMM-Route
  messen, Compiler bauen, registrieren.)

## AMS-02 — Operator-Wort (Verdrahtung eröffnen oder streichen)

- Der Descoped-Eintrag (`declined_sources.φ:231`, open no-consumer) ist amendiert:
  die Klausel „kein Compiler" ist überholt — ein TDAT/FITS-Serializer-Compiler steht
  (`src/archivar/ams02.rs` + `tools/harvest/src/bin/ams02_compiler.rs`, **uncommittet**,
  generisches TDAT-Archiv, kein force/Medium); der Kern (kein CR-Medium, kein
  Konsument) hält unverändert. Eine Feld-/Force-Registrierung bleibt verweigert.
  Post an `entscheid` steht. (Schritt: Operator-Wort — eröffnen oder streichen.)

## KASCADE-Grande — Format erst nach einem SSO-Job

- Domäne `kcdc.iap.kit.edu` (der Handover-Punkt trug `ikp`). DataShop ist
  Keycloak-SSO/JS, kein URL/POST-Endpoint; `blocked_sources.φ:63` korrekt. (Schritt:
  mit dem `omegaflow`-Konto einen Minimal-Job fahren, ASCII-Download, Spaltenlayout
  messen — `archive_search --playwright --headed` oder Operator.)

## Fink — Photometrie als benanntes Pending

- Der Konus lebt (HTTP 200, POST-only, `witnesses.φ:17`); die em-Photometrie
  (`r:psfFlux/r:psfFluxErr/r:midpointMjdTai`) ist via POST-columns abrufbar, der
  Compiler fordert nur ra/dec. (Schritt: `skydirection_compiler`-Scope um die
  Flux-Spalten erweitern — benanntes Pending, kein Format-Gap.)

## CDN-Manifestation + sky1-Zeugen (gated — nach Push)

- Nach Push dispatchen: `vtscat-cdn.yml`, `dl3-skymap-cdn.yml`, `emso-cdn.yml`,
  `planetary-odf-cdn.yml`, `ionex-cdn.yml`, `cors-cdn.yml`, `nh-rex-tnf-cdn.yml`,
  `flac-cdn.yml`, `bison-cdn.yml`; BGC-Argo + folge40-Bündel; unveränderte manuell.
  Dann `archive_search --verdict` auf die 3 sky1-Zeugen (`vtscat_flux`, `hess_dl3`,
  `magic_dl3`). (Schritt: nach Push dispatchen, kein Poll.)

## CI-Format-Gate

- Die 8 ernte-eigenen harvest-Compiler formatiert (`rustfmt --check` 0, `cargo check`
  0/0): gedi_l2a, icesat2_atl03, noaa_cdo, swot_l2_lr_ssh, vlass_tap, dl3_skymap,
  emso, vtscat. Fremd bleiben `src/gate/commit_gate.rs:540`,
  `tools/measure/.../{aia_ladder_probe,trishuli_gauge_probe}.rs`,
  `tools/utils/.../archive_search.rs` + baumweite weitere. (Schritt: fremde Linien
  fixen ihre Dateien; external-state `CI-Status` nach HEAD-Wechsel messen.)

## Benchmark

- 14 Taucher, alle flash-tier; kein pro/max nötig (mehrfach benannt). Die
  Routine-Klassen sind geschlossen — kein Re-Run. (Schritt: —)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
