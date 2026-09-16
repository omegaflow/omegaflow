<!--
  title: Handover — Ernte-Folge 43 (Stand 2026-09-16)
  session: Ernte-Folge 43
  class: handover
  date: 2026-09-16
  sha256: 2e7e083275f7e2df86a29673ef7829daa1b5300295a623160861ea457549ea09
  status: live
-->
# Handover — Ernte-Folge 43 (2026-09-16)

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

## Quellen-Inventur — Reader-Bauten (der härteste offene Punkt)

- Die vier `parser-def`-Gaps aus dem HEP-Batch brauchen je einen Reader:
  H.E.S.S./MAGIC/VERITAS (DL3-FITS-Event-Listen + IRFs → Sky-Map/Feld),
  KASCADE-Grande (`kcdc.iap.kit.edu` DataShop-Event-Tabellen), dazu der
  NRS-FLAC-Decoder und der NetCDF-`profile`-Arm (BGC-Argo). (Schritt: Bau-Atom
  je Format, Muster `geo.rs`/`fits.rs`; die Quellen stehen in
  `phi/blocked_sources.φ`.)
- `rosetta_odf_compiler`: der Pfad `DATA/LEVEL1A/CLOSED_LOOP/DSN/ODF/` ist 404;
  LEVEL1A trägt `IFMS/`, EAR2 (`…-EAR2-0061/0063`) `AG1/AG2/DP1/DP2`. (Schritt:
  Compiler-Pfad korrigieren, `tools/harvest/src/bin/rosetta_odf_compiler.rs`.)

## CORS — SNR entblockt; CDN offen

- `dbhz` in `convert_to_si` (`10^(x/10)`) + `allowed_units_for_force(0)`; der
  CORS-Block trägt `s1/s2/s5 … dbhz`. Der `cors_rinex_compiler` emittierte die
  SNR-Felder bereits generisch über alle Obs-Codes — kein Compiler-Bau nötig.
  Offen: CDN-Manifestation. (Schritt: `gh workflow run cors-cdn.yml` nach
  Push+Consent.)
- Baum-Befund: die CORS-`sources.φ`/`tests.rs`-Hunks hat die Bau-Linie in
  `3cba457e` mitgenommen (geteilter Index); `src/archivar/units.rs` trägt der
  Ernte-Commit.

## TNF + Voyager — CDN/Reader offen

- TNF registriert; offen: Workflow-Lauf `gh workflow run nh-rex-tnf-cdn.yml`
  nach Push+Consent (pdssbn-Erreichbarkeit).
- Voyager ODR: `sample_count`-Semantik (Spec ≤ 299999 vs. gemessene
  Record-1-Werte ~4,29e9, Offset 52 bestätigt) + Serie über die 968 RSS-Dateien.
  Der Reihen-Leser-Arm liegt als Post an bau (`post.md`). (Schritt: Messung +
  bau-Reader.)

## Sensor-Welle — registriert, Rest offen

- EMSO: lebendes Dataset `OBSEA_seabed_station_TS_L1c` (maxTime 2026-09-14);
  meteo eingefroren 2026-05-12 — in `ledger.φ`. Offen: sources.φ/CDN. (Schritt:
  Registrierung + `gh workflow run`.)
- mercator AQI: `max(DATE_TIME)` 2023-11-07 unverändert → beide Einträge nach
  `dead_sources.φ` (`dead frozen`). emodnet HFRADAR 2026-07-30 bestätigt
  (Familie steht). SondeHub ~1 Hz, IOOS-Glider ~27 min — in `ledger.φ`.
- Routen: `dachs.fai.kz/tap` (sync 500, PG-Backend refused), `vo.lmd.jussieu.fr`
  (https Timeout), `pithia.cbk.waw.pl` (https Zertifikat abgelaufen; http-Route
  maßgeblich) — notes in `ledger.φ`.

## Quellen-Inventur — disponiert

- HEP/Astro: LHAASO `decline catalog-register` (echter Katalog VizieR
  `J/ApJS/271/25`; `lhaaso_compiler` verwaist), Super-K `decline count`,
  Telescope Array `decline event-record`, VERITAS/H.E.S.S./KASCADE/MAGIC
  `blocked parser-def`.
- Geo/Zenodo: BGC-Argo Sprof registriert (`sources.φ`, NetCDF), Wohlmuth 1997
  `decline paper`, NRS FLAC `blocked parser-def flac`, Rosetta RSI EAR2-only
  bestätigt, HF-Radar unverändert.
- Offen aus dem Forschung-Post: Juno ODF, NH REX Pluto, Cassini PDS RSS raw
  (PDS, teils request-only), Lasair/SuperDARN. (Schritt: je Quelle
  Reachability + `sources.φ`-Block.)

## CDN-Manifestation (gated — nach Push + Consent)

- `nh-rex-tnf-cdn.yml`, `voyager-odr-cdn.yml`, `cors-cdn.yml`, `juno_odf` via
  `planetary-odf-cdn.yml`, BGC-Argo-NetCDF; folge40-Bündel. Unveränderte Ziele
  manuell (celestrak-eop, goes, himawari, …) + Gaia DR3 XP Voll-Survey.

## CI-Gate

- Fremde fmt-Diffs (`commit_gate.rs:540`, `aia_ladder_probe`/
  `trishuli_gauge_probe`, `archive_search.rs`) + DRS-FITS-Compile-Bruch
  (`drs_fits_compiler.rs:93`) — Post an die Linien steht (`post.md`). Lokale
  fmt-Läufe strukturell verweigert. (Schritt: die Linien fixen ihre Dateien.)

## Benchmark

- Flash-first, je Aufgabe der billigste tragende Vertreter: HEP-Batch
  (`grind-pro`, Force-Gate), Geo-Batch (`grind-flash`), Sensor-Welle (`general`),
  Ledger-Routen (`research-max`). Routinen-Klasse flash-gewonnen; die
  `parser-def`-Reader bleiben das pro/max-Atom. Burn via `session_burn`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
