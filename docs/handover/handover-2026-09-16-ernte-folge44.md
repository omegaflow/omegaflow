<!--
  title: Handover — Ernte-Folge 44 (Stand 2026-09-16)
  session: Ernte-Folge 44
  class: handover
  date: 2026-09-16
  sha256: e8e235a1dabd72470767b84b87f92aa12314aadcd2840be95be421e4fdc1f97d
  status: live
-->
# Handover — Ernte-Folge 44 (2026-09-16)

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

## Rosetta ODF — Format-Gap (der härteste offene Punkt)

- `rosetta_odf_compiler`: der Pfad ist auf die gemessene Live-Struktur korrigiert
  (`DATA/LEVEL1A/CLOSED_LOOP/IFMS/{AG1,AG2,DP1,DP2}/`, `.RAW`-Dateien; das alte
  `DATA/LEVEL1A/CLOSED_LOOP/DSN/ODF/` + `.dat` ist 404). Die TRK-2-34-Prämisse
  ist unbestätigt: `odf::parse_odf` verlangt 36-Byte-Records
  (`bytes.len().is_multiple_of(36)`); die gemessene `.RAW` (343 978 B bzw.
  605 756 B) ist kein 36-Vielfaches und trägt am Dateiende XML (`</body_Gain>`).
  Erste Messung: `curl -r 0-255` + `xxd` auf `R32ICL1L1A_AG1_072831317_02.RAW`;
  ist die Datei nicht TRK-2-34, ist die `rosetta_odf`-Prämisse zu verwerfen.

## CDDIS IONEX — registriert, Compiler/CDN offen

- Der `blocked key-needed`-Eintrag war stale und ist nach `sources.φ` gewandert
  (`format ionex`, `field tec ionex_tec … tecu`, `header Authorization
  {EARTHDATA_EDL_TOKEN}`, Template `{year}/{yday}`). Gemessen 2026-09-16: ohne
  Token die EDL-Login-Seite (200, 9385 B), mit `EARTHDATA_EDL_TOKEN` die
  Verzeichnisliste (200, 96 126 B), konkretes Tagesfile
  `…/ionex/2026/001/IGS0OPSRAP_20260010000_01D_02H_GIM.INX.gz` (200, 184 487 B).
  Kein Nutzer-`client_id` nötig. Offen: CDN-Manifestation — es gibt keinen
  `ionex`-Compiler in `tools/harvest`. (Schritt: `ionex_compiler` nach
  ODF-Muster, dann Workflow.)

## Quellen-Inventur — Reader-Bauten (die vier parser-def + NetCDF-profile)

- **NRS-FLAC** — gebaut und registriert: `src/archivar/flac.rs` (voller Decoder
  `decode` → StreamInfo + PCM-Samples; STREAMINFO gemessen 2026-09-16 per
  `curl -r 0-127` an NRS01_20141016_154112.flac: 5000 Hz / 1 Kanal / 16 bps /
  72 000 000 Samples) + `write_series`/`parse_series` (`FLCS`-Bin),
  Compiler `tools/harvest/src/bin/flac_compiler.rs` (Epoche aus Dateinamen
  `YYYYMMDD_HHMMSS`), Workflow `flac-cdn.yml`, registriert in `sources.φ`
  (`format flac`, `field pcm nrs_hydrophone_pcm … acoustic count`; `count` in
  `allowed_units_for_force(2)`). `blocked`-Eintrag entfernt. Offen: der
  Workflow-Lauf nach Push+Consent + die NRS01-Standortkoordinaten (die Serie
  trägt `at earth`, kein lat/lon gemessen).
- **VERITAS VTSCat** — Reader gebaut: `src/archivar/vtscat.rs` (`parse_ecsv` +
  `parse_sexagesimal` + `yaml_sexagesimal`; gemessen an VER-000058-sed.ecsv).
  `blocked` → `pending`. Offen: Compiler (ECSV/YAML → Fluss-Karte) + Workflow.
- **H.E.S.S. DL3 DR1** — Reduktion gebaut: `src/archivar/dl3.rs` (`parse_events`
  + `reduce_grid`; gemessen an hess_dl3_dr1_obs_id_033798.fits, EVENTS-BINTABLE
  RA/DEC/ENERGY). `blocked` → `pending`. Offen: Compiler (TAR → Sky-Map) + Workflow.
- **MAGIC DL3** — derselbe Reader (`dl3.rs`), `blocked` → `pending`. Offen:
  Compiler + Workflow.
- **KASCADE-Grande** — `blocked` → `blocked account` (Login `/accounts/login`,
  „Data Format" `/information/dataFormat` anonym 0 B; kein URL-Endpoint). Offen:
  Konto (Operator) + das Event-Tabellen-Format nach einem Job.
- **BGC-Argo Sprof (NetCDF-4)** — der `profile`-Arm liest jetzt HDF5/NetCDF-4:
  `src/archivar/channels.rs` (`build_netcdf4_channels` + HDF5-Routing in
  `build_netcdf_channels`) + `src/archivar/hdf5.rs` (`attr_f64`/`dims`); gemessen
  an 1901614_Sprof.nc (HDF5-Signatur `89 48 44 46`). Die bestehenden
  `sources.φ`-Einträge (1901614/1901843) lesen damit echte Bytes statt `Hdf5`-void.

## CORS — CDN offen

- `dbhz` + `allowed_units_for_force(0)` stehen; der `cors_rinex_compiler`
  emittiert die SNR-Felder. Offen: CDN-Manifestation. (Schritt:
  `gh workflow run cors-cdn.yml` nach Push+Consent.)

## TNF + Voyager — CDN/Reader offen

- TNF registriert; offen: `gh workflow run nh-rex-tnf-cdn.yml` nach
  Push+Consent (pdssbn-Erreichbarkeit).
- Voyager ODR: `sample_count`-Semantik (Spec ≤ 299999 vs. gemessene
  Record-1-Werte ~4,29e9, Offset 52 bestätigt) + Serie über die 968 RSS-Dateien.
  Der Reihen-Leser-Arm liegt als Post an bau. (Schritt: Messung + bau-Reader.)

## Sensor-Welle — CDN offen

- EMSO `OBSEA_seabed_station_TS_L1c` ist in `sources.φ` (`.json`, ttl 3600;
  TEMP/PSAL/PRES/CNDC; exakte URL gemessen 2026-09-16: 200, 30 496 B). Offen:
  CDN-Manifestation. (Schritt: Compiler/Workflow.)
- emodnet HFRADAR: `maxTime` 2026-07-30 bestätigt (Familie steht, `ledger.φ`).
  Nächster Schritt: `maxTime` erneut messen. mercator AQI steht in
  `dead_sources.φ` (`dead frozen`, 2023-11-07); Routen-Notes in `ledger.φ`
  unverändert.

## CDN-Manifestation (gated — nach Push + Consent)

- `nh-rex-tnf-cdn.yml`, `voyager-odr-cdn.yml`, `cors-cdn.yml`, `flac-cdn.yml`
  (neu), `juno_odf` via
  `planetary-odf-cdn.yml`, BGC-Argo-NetCDF, EMSO; folge40-Bündel. Unveränderte
  Ziele manuell (celestrak-eop, goes, himawari, …) + Gaia DR3 XP Voll-Survey.

## CI-Gate

- Fremde fmt-Diffs (`commit_gate.rs:540`, `aia_ladder_probe`/
  `trishuli_gauge_probe`, `archive_search.rs`) + DRS-FITS-Compile-Bruch
  (`drs_fits_compiler.rs:93`) — Post an die Linien steht (`post.md`). Lokale
  fmt-Läufe strukturell verweigert. (Schritt: die Linien fixen ihre Dateien.)
- Gemessen 2026-09-16 (Ende Reader-Bau): `cargo check` 0/0 und
  `cargo check --tests` 0/0 (Core) sowie `cargo check -p omegaflow-harvest
  --tests` 0/0 — die Reader-Dateien (`flac.rs`, `vtscat.rs`, `dl3.rs`,
  `channels.rs`, `hdf5.rs`, `flac_compiler.rs`) sind warnungsfrei. Die fremden
  fmt-Diffs (`commit_gate.rs:540`, `aia_ladder_probe`/`trishuli_gauge_probe`,
  `archive_search.rs`) bleiben — Post an die Linien steht (`post.md`).

## Benchmark

- Reader-Klasse (`parser-def`: DL3-FITS, VTSCat, FLAC, NetCDF-profile) — **nicht**
  flash-first gelaufen: die fünf Arme teilen `mod.rs`/`main_flow.rs`/`extract.rs`,
  parallele flash-Bauten hätten sich im geteilten Baum überschrieben. Ein
  `grind-pro` baute alle Arme in einem Kontext, ein zweiter `grind-pro`
  verifizierte unabhängig und fand **8 reale Defekte** (FLAC-Serien-Roundtrip tot,
  VTSCat-Units verloren, EMSO-Force falsch, H.E.S.S.-Disposition stale,
  fabriziertes STREAMINFO) — der Bau allein trug nicht, die Verifikation war
  tragend. Register/Sensor/Inventur liefen `grind-flash`. Burn via `session_burn`.
- Offen: ein sauberer flash-Bau/pro-Verifikation-Doppellauf der Reader-Klasse,
  erst nach Entkopplung der geteilten Registrierungspunkte.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
