<!--
  title: Handover — Ernte-Folge 45 (Stand 2026-09-16)
  session: Ernte-Folge 45
  class: handover
  date: 2026-09-16
  sha256: 208dd24cd974412588b76d37f1992ac553475756a5c017ccb38a33f944af49b8
  status: live
-->
# Handover — Ernte-Folge 45 (2026-09-16)

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

## Rosetta ODF — IFMS-AGC-Text-Reader (der härteste offene Punkt)

- Die `rosetta_odf`-TRK-2-34-Prämisse ist **verworfen** (gemessen 2026-09-16, Note in `phi/sources.φ` bei der `rosetta_odf`-Zeile): die PSA-.RAW ist `<header>`-XML + `<body_Gain>` + 3601 Zeilen à 429 B ASCII, `RECORD_TYPE = UNDEFINED`; `parse_odf` (36-Byte-Gate, `len % 36 = 9`) liest nichts. Nächster Schritt: IFMS-AGC-Text-Reader (XML-Header + 429-B-Festbreite → Serie) nach `flac.rs`/`vtscat.rs`-Muster + Prüf-Messung des handover-benannten `R32ICL1L1A_AG1_072831317_02.RAW` (343 978/605 756 B — unter dem gemessenen DATASET_ID nicht vorhanden, Suche offen). (Schritt: `src/archivar/`-Reader + `rosetta_odf_compiler` auf `…/RSI/<DATASET_ID>/DATA/LEVEL1A/CLOSED_LOOP/IFMS/{AG1,AG2,DP1,DP2}/` zeigen.)

## sky1-Format-Arm + Registrierung (VTSCat / H.E.S.S. / MAGIC)

- Drei Compiler sind gebaut und `cargo check --bin` 0/0: `vtscat_compiler.rs`, `dl3_skymap_compiler.rs` (+ `.github/workflows/{vtscat,dl3-skymap}-cdn.yml`). Sie emittieren SKY1 (`omegaflow::skymap`, `KIND_GAMMA`). **Kein `sky1`-Handler** in `main_flow.rs`/`extract.rs` → die Assets sind unlesbar. Zudem fehlen die Einheiten in `allowed_units_for_force(0)`: `m-2.s-1.TeV-1` (VTSCat dnde) und `tev` (H.E.S.S./MAGIC Summen-Energie). Die Einträge stehen `pending` in `phi/blocked_sources.φ:60-75`. Nächster Schritt: sky1-Arm + Einheiten (mit Gate-Test in `test_allowed_units_for_force`) + `sources.φ`-Registrierung + Workflow-Lauf. (Schritt: `main_flow.rs` sky1-Zweig + `units.rs`.)

## BiSON — Registrierung + Reader-Arm

- `bison_compiler.rs` + `.github/workflows/bison-cdn.yml` + CDN-Asset (`ssd.jpl.nasa.gov/bison_pmode.bin`, ~350 MB, BSV1, 21 892 536 Records, gemessen 200) existieren seit `8399b401`. Offen: kein `phi/sources.φ`-Eintrag, und `bison_velocity::parse_bin` ist nicht in `extract.rs`/`main_flow.rs` verdrahtet. Die post.md-URL `/opendata/…` ist 404 — der Compiler nutzt korrekt `/downloads/data/…`. Nächster Schritt: `sources.φ`-Eintrag (force 2 acoustic, `m/s`) + Reader-Arm. (Schritt: `extract.rs`/`main_flow.rs` bison-Zweig.)

## IONEX — CDN-Route

- `ionex_compiler.rs` + `.github/workflows/ionex-cdn.yml` gebaut (`cargo check --bin` 0/0), CDN-Tag `cddis.nasa.gov`, Asset `ionex_tec_gim.bin` (GIM1). Der `sources.φ`-Eintrag (`:1394`) zeigt aber auf die rohe CDDIS-INX (`format ionex`, verdrahtet über `build_ionex_channels`); `main_flow.rs:1358` setzt `Bearer` selbst — die vermeintliche Register-Korrektur ist hinfällig. Offen: entscheiden, ob die CDN-GIM1-Route die rohe Route ersetzt, dann Workflow-Lauf. (Schritt: Route messen/entscheiden.)

## CDN-Manifestation (gated — nach Push + Consent)

- `gh workflow run`: `cors-cdn.yml`, `nh-rex-tnf-cdn.yml`, `flac-cdn.yml`, `ionex-cdn.yml`, `vtscat-cdn.yml`, `dl3-skymap-cdn.yml`, `bison-cdn.yml`; `juno_odf` via `planetary-odf-cdn.yml`, BGC-Argo-NetCDF, EMSO, folge40-Bündel. Unveränderte Ziele manuell (celestrak-eop, goes, himawari, …) + Gaia DR3 XP Voll-Survey.

## Sensor-Welle — CDN/Recheck

- EMSO `OBSEA_seabed_station_TS_L1c` steht in `sources.φ` (`.json`, ttl 3600; URL gemessen 200, 30 496 B). Offen: CDN-Manifestation (Compiler/Workflow). emodnet HFRADAR: `maxTime` erneut messen (Familie steht, `ledger.φ`). mercator AQI bleibt `dead frozen` (2023-11-07) in `dead_sources.φ`.

## Register-Rest aus dem entscheid-Handover (Post an ernte)

- MPC-Shard UnnObs, GHRC-DAAC, Survey §1 (26 Pendings); Register-Digest-Rest (Fink/ALeRCE, TDAT/FITS, Akteure, Holdings, Orphan-Verdicts). (Schritt: `sources.φ` + CDN / je Punkt messen.)

## KASCADE-Grande / Voyager ODR

- KASCADE-Grande: `blocked account` — Konto (Operator) + Event-Tabellen-Format nach einem Job. Voyager ODR: `sample_count`-Semantik (Spec ≤ 299999 vs. gemessene Record-1-Werte ~4,29e9, Offset 52) + Serie über die 968 RSS-Dateien; der Reader-Arm liegt als Post an bau. (Schritt: Messung + bau-Reader.)

## CI-Gate

- Fremde fmt-Diffs (`commit_gate.rs:540`, `aia_ladder_probe`/`trishuli_gauge_probe`, `archive_search.rs`) + DRS-FITS-Compile-Bruch (`drs_fits_compiler.rs:93`) — Post an die Linien steht. Lokale fmt-Läufe strukturell verweigert. (Schritt: die Linien fixen ihre Dateien.)

## Benchmark

- Reader-Klasse (`parser-def`): der saubere flash-Bau/pro-Verifikation-Doppellauf steht weiter offen, erst nach Entkopplung der geteilten Registrierungspunkte. (Schritt: `session_burn`-Doppellauf nach Entkopplung.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
