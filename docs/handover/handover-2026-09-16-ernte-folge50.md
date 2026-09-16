<!--
  title: Handover — Ernte-Folge 50 (Stand 2026-09-16)
  session: Ernte-Folge 50
  class: handover
  date: 2026-09-16
  sha256: fa2a9c447992e3c092620499fb0344d24ac82abae55ce9bc70c2a196704602b9
  status: live
-->
# Handover — Ernte-Folge 50 (2026-09-16)

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

## MPC-UnnObs-Shards — Decoder gefixt, Messung + Register offen

- Der Shard-Key liest jetzt die Bezeichnung statt des leeren Nummernfelds: `designation_key`
  = erste 4 Bytes der 7-Byte-Bezeichnung big-endian; `enum ShardKey { Number, Designation }`
  per `--shard-key designation` gewählt (nie aus der 0 geschlossen); `record_bytes`
  unverändert, `number` bleibt 0 (0 honored — Nummern beginnen bei 1); Fixtur aus der echten
  Zeile `     I73O00A* A1873 …`, 3 Tests; `cargo check` 0/0. `mpcobs-shard-cdn.yml` trägt
  `--shard-key designation`. Gemessen: UnnObs 25 956 436 Zeilen, 2 102 471 316 B; alle
  Nummernfelder leer (`sgrep -c "     K"` = 25 847 946). Nach dem Push: `mpcobs-shard-cdn.yml`
  dispatchen, aus dem Run-Log die Shard-Zahl/-Größen lesen (Budget 2³⁰ B → erwartet ~2 Shards),
  dann die `url`-Zeilen neben `phi/sources.φ:1899` eintragen. (Schritt: `gh run view <id>` →
  Zeile `mpcobs: … B in S shards` → url-Zeilen; kein Poll.)

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
  `flac-cdn.yml`, `bison-cdn.yml`, `mpcobs-shard-cdn.yml`; BGC-Argo + folge40-Bündel;
  unveränderte manuell. Dann `archive_search --verdict` auf die 3 sky1-Zeugen
  (`vtscat_flux`, `hess_dl3`, `magic_dl3`). (Schritt: nach Push dispatchen, kein Poll.)

## CI-Format-Gate

- Die ernte-eigenen Compiler sind formatiert. Fremd bleiben `src/gate/commit_gate.rs:540`,
  `tools/measure/.../{aia_ladder_probe,trishuli_gauge_probe}.rs`,
  `tools/utils/.../archive_search.rs` sowie die harvest-Compiler
  `swot_l2_lr_ssh_compiler.rs` + `vlass_tap_compiler.rs` (gemessen im format-Job,
  `ci-check` run 35091175017). (Schritt: fremde Linien fixen ihre Dateien; external-state
  `CI-Status` nach HEAD-Wechsel messen.)

## Benchmark

- MPC-UnnObs-Shard-Key: das Architektur-Urteil trug der Council (pro/max) — Kandidat (a),
  roher Bezeichnungs-Präfix, `number` bleibt 0, Modus pro Lauf benannt; die Umsetzung trug
  grind-flash (mechanisch, `cargo check` 0/0). Kein Doppel-Lauf — das Urteil war der
  harte Teil, die Ausführung Routine.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
