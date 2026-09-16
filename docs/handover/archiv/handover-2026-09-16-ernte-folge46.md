<!--
  title: Handover — Ernte-Folge 46 (Stand 2026-09-16)
  session: Ernte-Folge 46
  class: handover
  date: 2026-09-16
  sha256: 78a89c5cd215141c8198d8c36750a6161eabbf5e055df8bf1f42e4a3cc51928a
  status: live
-->
# Handover — Ernte-Folge 46 (2026-09-16)

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

## sky1-Zeugen-Arm — der gebaute Arm ist aus dem Zeugen-Register unerreichbar

- Der sky1-Arm (`extract.rs` `format == "sky1"`-Zweig, `main_flow.rs` sky1-Arm) + Einheiten (`units.rs` `m-2.s-1.tev-1`, `tev`; Gate-Test in `tests.rs`) sind gebaut, `cargo check --workspace` 0/0; die drei Zeugen (VTSCat, H.E.S.S. DL3 DR1, MAGIC) stehen in `phi/witnesses.φ` (aus `blocked_sources.φ` entfernt). **Gemessener Gap:** `archive.sources` wird nur aus `phi/sources.φ` gespeist (`parse.rs:4`); `witnesses.φ` wird nur für die netloc-Gewichtung gelesen (`port.rs:2450`, `frames.rs:101`). Der Arm prüft `archive.sources[i].format == "sky1"` — kein Zeugen-Eintrag erreicht ihn. Nächster Schritt: den Zeugen-Pfad verdrahten (Witness-Loader in `parse.rs`, der `witnesses.φ`-Zeilen als Feld-Quellen lädt, **oder** CDN-Asset-Zeilen in `sources.φ` für die `.sky1`-Assets), dann `gh workflow run vtscat-cdn.yml` + `dl3-skymap-cdn.yml` nach Push. (Schritt: `parse.rs`-Zeugen-Zweig oder `sources.φ`-Asset-Zeile — Route entscheiden.)

## Rosetta ODF — IFMS-AGC-Reader gebaut, Fixtur fehlt

- `src/archivar/ifms_agc.rs` (neu) parst `<header>`-XML + `<active_table>` + die Datenzeilen (Spalten aus der `//`-Kommentarzeile, nicht hartcodiert); `rosetta_odf_compiler.rs` darauf verdrahtet (ersetzt `parse_odf`); `cargo check -p omegaflow-harvest --bin rosetta_odf_compiler` 0/0. Die Prüf-Messung lief an `/tmp/opencode/r32.RAW` (nicht committet). Offen: echte `.RAW`-Fixtur unter `src/archivar/kernels/` (wie `odf07155.dat`) + `--ci-mode`-Workflow-Lauf. Die handover-benannte Datei liegt unter `RO-X-RSI-1-2-3-EAR2-0061-V1.0` (nicht ESC1-0446); gemessen 2026-09-16: `…/IFMS/AG1/R32ICL1L1A_AG1_072831317_02.RAW` 343 978 B, `_00` 605 756 B. (Schritt: Fixtur + `planetary-odf-cdn.yml`.)

## IONEX — Route entscheiden

- CDN-GIM1-Asset `ionex_tec_gim.bin` = HTTP 404 (nie manifestiert). Rohe CDDIS-INX-Route: `main_flow.rs:1354-1364` setzt `Bearer` nur für `fmt == "rinex"`, gunzippt nur für rinex (`:1382`) → der ionex-Pfad bekommt die Login-Seite bzw. gzip, null Kanäle; `parse_gim_bin` (`src/archivar/ionex.rs`) ist nicht verdrahtet. Nächster Schritt: Route entscheiden (GIM1-CDN vs. roh+Token+gzip) und `build_ionex_channels`/`main_flow` ionex-Zweig anpassen. (Schritt: `main_flow.rs` ionex-Zweig.)

## CDN-Manifestation (gated — nach Push + Consent)

- `gh workflow run`: `vtscat-cdn.yml`, `dl3-skymap-cdn.yml` (nach Zeugen-Pfad), `cors-cdn.yml`, `nh-rex-tnf-cdn.yml`, `flac-cdn.yml`, `ionex-cdn.yml` (nach Route), `bison-cdn.yml`; `juno_odf` via `planetary-odf-cdn.yml`, BGC-Argo-NetCDF, EMSO, folge40-Bündel. Unveränderte Ziele manuell (celestrak-eop, goes, himawari, …) + Gaia DR3 XP Voll-Survey.

## EMSO — Manifestationspfad fehlt

- `OBSEA_seabed_station_TS_L1c` in `sources.φ` (`.json`, ttl 3600) lebt (2026-09-16 gemessen 200, 30 496 B); es gibt **keinen** `*emso*`-Compiler/Workflow → CDN-Anbindung ausstehend. emodnet HFRADAR: `maxTime` erneut gemessen (2026-07-31, unverändert eingefroren; `ledger.φ` trägt es). mercator AQI bleibt `dead frozen` (2023-11-07, `dead_sources.φ`). (Schritt: EMSO-Compiler/Workflow bauen.)

## Register-Rest (aus post.md gefaltet)

- MPC-Shard UnnObs, GHRC-DAAC, Survey §1 (26 Pendings); Register-Digest-Rest (Fink/ALeRCE, TDAT/FITS, Akteure, Holdings, Orphan-Verdicts). (Schritt: `sources.φ` + CDN / je Punkt messen.)

## KASCADE-Grande / Voyager ODR

- KASCADE-Grande: `blocked account` — Konto (Operator) + Event-Tabellen-Format nach einem Job. Voyager ODR: `sample_count`-Semantik (Spec ≤ 299999 vs. gemessene Record-1-Werte ~4,29e9, Offset 52) + Serie über die 968 RSS-Dateien; der Reader-Arm liegt als Post an bau. (Schritt: Messung + bau-Reader.)

## CI-Gate

- Fremde fmt-Diffs (`commit_gate.rs:540`, `aia_ladder_probe`/`trishuli_gauge_probe`, `archive_search.rs`) + DRS-FITS-Compile-Bruch (`drs_fits_compiler.rs:93`) — Post an die Linien steht. Lokale fmt-Läufe strukturell verweigert. (Schritt: die Linien fixen ihre Dateien.)

## Benchmark

- Reader-Klasse (`parser-def`): der flash-Bau/pro-Verifikation-Doppellauf steht weiter offen. Diese Session fuhr fünf Taucher parallel (2× grind-pro, 3× grind-flash) + 1× council — kein Doppellauf als Benchmark, nur die Routine-Klasse ist geschlossen. (Schritt: `session_burn`-Doppellauf.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
