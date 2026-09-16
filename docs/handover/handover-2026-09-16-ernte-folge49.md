<!--
  title: Handover — Ernte-Folge 49 (Stand 2026-09-16)
  session: Ernte-Folge 49
  class: handover
  date: 2026-09-16
  sha256: 90a99de89491287a4450b1e0cc742afc90f4673c78d66bd4f1bd371d2860a0af
  status: live
-->
# Handover — Ernte-Folge 49 (2026-09-16)

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

- `decode_packed_number` (`tools/harvest/src/bin/mpcobs_compiler.rs:20-31`) parst die
  moderne gepackte provisorische Bezeichnung (`K09A0…`, base62-Zyklus) nicht → alle
  Nummern 0 → kein Split (der CI-Lauf schrieb einen 1,2-GiB-Shard; vom Release
  entfernt). (Schritt: Decoder + Fixtur aus der echten UnnObs-Zeile erweitern;
  `gh workflow run mpcobs-shard-cdn.yml` re-dispatchen; Shard-Zahl/-Größen messen —
  25 792 218 Records, 164 218 skipped, 2 102 471 316 B dekomprimiert; dann die
  url-Zeilen eintragen.)

## Zenodo sha256 — Quaoar/TNBFits (Zenodo-Backend 504)

- `multimoon-1.0.zip` ist registriert. Quaoar (`zenodo.21185812`, 572 467 032 B) und
  TNBFits (`zenodo.10620251`, 14 232 134 885 B) sha256 `pending`: Zenodo HTTP 504
  service-weit, TNBFits zusätzlich 403 (Netz-Block). (Schritt: bei Rückkehr streamend
  `curl -sL <url> | sha256sum` + md5-Abgleich — Quaoar 420a1e94…, TNBFits a1aada71…;
  TNBFits 13,25 GiB > 2-GiB-Asset-Limit → CI-Stream + Granulat; dann Register.)

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

## CDN-Manifestation + sky1-Zeugen (gated — nach Push)

- Nach Push dispatchen: `vtscat-cdn.yml`, `dl3-skymap-cdn.yml`, `emso-cdn.yml`,
  `planetary-odf-cdn.yml`, `ionex-cdn.yml`, `cors-cdn.yml`, `nh-rex-tnf-cdn.yml`,
  `flac-cdn.yml`, `bison-cdn.yml`; BGC-Argo + folge40-Bündel; unveränderte manuell.
  Dann `archive_search --verdict` auf die 3 sky1-Zeugen (`vtscat_flux`, `hess_dl3`,
  `magic_dl3`). (Schritt: nach Push dispatchen, kein Poll.)

## CI-Format-Gate

- Die 8 ernte-eigenen harvest-Compiler sind formatiert. Fremd bleiben
  `src/gate/commit_gate.rs:540`, `tools/measure/.../{aia_ladder_probe,trishuli_gauge_probe}.rs`,
  `tools/utils/.../archive_search.rs` + baumweite weitere. (Schritt: fremde Linien
  fixen ihre Dateien; external-state `CI-Status` nach HEAD-Wechsel messen.)

## Benchmark

- AMS-02-Strike: flash genügte (2 Dateien, 2 Zeilen, `cargo check` 0/0) — kein pro/max.
  Die Routine-Klasse bleibt geschlossen. (Schritt: —)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
