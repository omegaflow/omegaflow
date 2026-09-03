<!--
  title: Struktur-Vorschlag — tools/src nach Funktion (harvest/measure/register/service/science/gate/utils)
  class: survey
  date: 2026-09-02
  status: live
  sha256: none
-->

# Struktur-Vorschlag: `src`/`tools` nach Funktion ordnen

Operator-Wort (2026-09-02): die Namen `src`, `tools/live`, `tools/work` sind nicht
selbsterklärend. Gemessen ist die funktionale Wirklichkeit; dieser Vorschlag
benennt die Ordner nach der Funktion (Name = Implementation).

## Ist-Zustand (gemessen)

- `src/` — das eine Core-Crate `omegaflow`: `archivar/` + `mathematikerin/` +
  die 6 Gate-Module (`axioms, friction, handover, llm_gate, state, tool_perm`).
  106 `.rs`. Bibliothek + `main.rs` (`archivar::main_flow`).
- `tools/work` (Crate `omegaflow-work`, 152 Bins) — funktional gemischt:
  - 74 Quellen-Kompiler + Harvester (`*_compiler`, crawl → `phi/bin`, CDN)
  - 55 Messung/Probe (TE, Doppler, Solar, Blatt)
  - 10 Register/Verwaltung (`register_verify`, `claim_verify`, …)
  - 6 Live-Dienst (`cds_watchdog`, `smail`, `llm_interceptor`, …)
  - 4 Paper/arxiv (`arxiv`, `paper_new`, `export_latex`)
  - 1 Hilfs-Format (`spk_split`)
- `tools/live` (Crate `omegaflow-live`, 9 Bins) — ebenfalls gemischt, nicht „live":
  - 1 Gate: `livefeed_gate`
  - 8 Reader/Utilities: `netcdf_reader`, `hdf5_reader`, `bench`, `omega_sh`,
    `sfetch`, `sgrep`, `source_scanner`, `zip_range_extract`

## Ziel-Baum (Vorschlag)

```
src/                    → core-Crate omegaflow  (Archivar, Mathematikerin, Gate-Module)
tools/
  harvest/              → 74 Kompiler + Harvester      (Crate omegaflow-harvest)
  measure/              → 55 Messung/Probe             (Crate omegaflow-measure)
  register/             → 10 Register/Verwaltung       (Crate omegaflow-register)
  service/              → 5 Live-Dienste/Watchdogs     (Crate omegaflow-service)
  science/              → 4 Paper/arxiv + export_latex (Crate omegaflow-science)
  gate/                 → 2 Gate-Bins                  (Crate omegaflow-gate)
  utils/                → 9 Reader/Utilities + spk_split (Crate omegaflow-utils)
```

Jedes Funktions-Crate ist ein eigenes Cargo-Paket mit flacher `src/bin/*.rs`
(Cargo-Autodiscovery bleibt). Alle Crate hängen an `omegaflow` (path).
`tools/live` und `tools/work` verschwinden.

## Abhängigkeits-Erkenntnis (gemessen)

- Die Extra-Deps des heutigen `live`-Crate (geotiff-reader, miniz_oxide, utm,
  ndarray, geotiff-writer) nutzt **nur `livefeed_gate`** → wandert mit ins
  `gate`-Crate.
- `zstd` (heute in work) nutzen nur `s1_sar_compiler` + `s1_raster_diff`.
- Die übrigen ~150 Bins brauchen nur `omegaflow` (std-only).
- `omegaflow`-Feature: `harvest/measure/register/service/science` brauchen
  KEIN `llm_gate`; nur `gate`-Crate setzt `features=["llm_gate"]`.

## Bin→Crate-Zuordnung

### harvest (74)
aia_compiler bia_efield_compiler bidsleep_compiler bison_basu_compiler
bison_compiler bison_shift_compiler cmb_planck_compiler meteo_archive_harvest
cometels_compiler cosmicflows_compiler crystal_compiler d20_compiler
dastcom_compiler dataverse_harvester dcom5_compiler deims_harvester
demeter_compiler demeter_harvest infrared_anomaly_compiler ephemeris_compiler
ephemeris_horizons_check erddap_harvester euvs_compiler eve_compiler
excess_exclude_compiler f107_compiler fermi_4fgl_compiler frb_compiler
goes_r_xrs_compiler goes_xrs_compiler gong_compiler gong_series_compiler
hmi_polar_compiler horizons_compiler infrared_excess_compiler jwst_equilibrium_compiler
jwst_spectra_compiler kbo_compiler meteo_harvest mitdb_compiler mitdb_rr_compiler
movement_monitoring_compiler mpcobs_compiler oai_harvester omni2_compiler
pangaea_harvester phonocardiogram_compiler pioneer11_odf_compiler
pioneer_atdf_compiler pioneer_doppler_compiler pioneer_telemetry_compiler
qbo_compiler radio_compiler rest_harvester rpw_compiler s1_post_capture
s1_sar_compiler seismic_quakes_compiler sexagesimal_compiler solr_harvester
sparql_harvester spectral_compiler srd62_compiler tap_compiler tess_compiler
tic_compiler tns_compiler twomass_compiler tycho2_compiler wind_orbit_compiler
wind_waves_compiler wso_polar_compiler xml_harvester ztf_lightcurves_compiler

### measure (55)
aia_ladder_probe bigbang_echo_probe bison_basu_probe bison_cycle_probe
bz_blatt_probe bz_retro_probe code_drift_te corona_event_probe
corona_ladder_probe corona_lag_probe cross_te_screen dark_flow_probe
dark_matter_probe session_te session_token_te flyby_probe frb_blatt_probe gong_cycle_probe
hmi_cycle_probe jwst_biosignature_scanner kbo_residue_probe rixs_cuprate_probe
laic_probe pioneer_link_correction_probe long_window_probe mitdb_selfcheck_probe
mitdb_sweep_probe mitdb_te_probe mseed_measure nobel_probe_corona
pioneer11_negative_fuzzy_probe pioneer11_odf_residuum pioneer11_tx_sequence_probe
pioneer_atdf_residuum pioneer_doppler_moyer pioneer_doppler_moyer_navio
pioneer_doppler_residuum pioneer_drift_te_probe pioneer_navio_clean
pioneer_navio_diagnose pioneer_odp pioneer_residue_diagnose
pioneer_text_correlation s1_raster_diff signal_cone_audit_probe solar_cycle_probe
solar_causal_graph_probe te_fn_probe te_ground_truth te_pair_probe te_rng_fix_probe
te_surrogate_amp_probe wso_cycle_probe wso_hmi_consistency ztf_anomaly_probe

### register (10)
claim_reader claim_verify commit_words home_scan number_audit pending_extract
reference_verify register_verify relations_consistency_probe
repo_surveillance

### service (5)
cds_watchdog mail_watchdog notes_notify smail smail_recv

### science (4)
arxiv arxiv_submit export_latex paper_new

### gate (2)
livefeed_gate llm_interceptor

### utils (9)
bench hdf5_reader netcdf_reader omega_sh sfetch sgrep source_scanner
spk_split zip_range_extract

## Blast-Radius (gemessen)

- 86 Referenzen auf `omegaflow-work`/`omegaflow-live` (Skripte, CI, docs).
- 49 eindeutige `--bin <name>`-Namen in 8 CI-Workflows
  (physionet-cdn, kernel-flatten, meteo-cdn, paper-check, s1-sar-cdn,
  probe-sweep, laic-cdn, demeter-cdn).
- AGENTS.md + README erwähnen `tools/live`/`tools/work` und die
  `cargo run -p`-Aufrufe.
- Cargo-Workspace `members` in Root-Cargo.toml.

## Umsetzungsschritte (Reihenfolge)

1. `git mv` jeder Bin in ihr Funktions-Crate (`tools/<fkt>/src/bin/<name>.rs`).
2. Pro Funktions-Crate eine `Cargo.toml` (name = `omegaflow-<fkt>`,
   `license-file`, dep `omegaflow`; nur `gate` mit `features=["llm_gate"]`;
   `harvest` zusätzlich `zstd`).
3. Root-Cargo.toml: `members` auf die neuen Crate; `tools/live`,`tools/work`
   entfernen.
4. Alle `-p omegaflow-work|omegaflow-live` und `--bin`-Referenzen in
   Skripten/CI/docs auf `-p omegaflow-<fkt>` umschreiben.
5. AGENTS.md/README-Struktur-Absatz auf den Ziel-Baum aktualisieren.
6. `cargo check` (alle Features, alle Crate) 0/0; Gate-Tests grün.

## Gremium-Entscheidungen (Council, 2026-09-02)

Beide Entscheidungen einstimmig (Mountain, River, Mycelium, Sensory, Future).

**1. `src/` bleibt — dokumentiert als Konvention.** Kein Umbenennen nach `core/`.
`src/` *ist*, was es benennt: das Quell-Verzeichnis des einen Core-Crates
(Archivar + Mathematikerin + Gate-Module). Cargo-Konvention (`src/lib.rs`,
`src/main.rs` = autolib/autobin) ist der Default; `core/` erzwingt
`[lib] path`/`[[bin]] path` — ein stiller Kampf, den jede künftige Sitzung
erinnern müsste. Die Konvention wird ausdrücklich dokumentiert (AGENTS.md/
README: „`src/` = das eine Core-Crate"), nicht still angenommen.

**2. `science` bleibt eigenes Crate `omegaflow-science`.** Kein Aufgehen in
`measure`. Ein Paper ist keine Sonde: `export_latex`/`paper_new`/`arxiv`
emittieren die Messung nach außen; `measure` sondiert das Feld. Veröffentlichen
als Messen zu benennen wäre eine Fabrikation. Die Docs-Struktur spiegelt es
schon (`docs/paper/` class: paper gegen die Proben); die Code-Struktur folgt ihr.

## Offene Entscheidung (geklärt)

- `src/` bleibt (Gremium).
- `science` bleibt eigenes Crate (Gremium).

## Ziel-Baum (final, nach Gremium)

```
src/                    → core-Crate omegaflow (Archivar, Mathematikerin, Gate-Module)
tools/
  harvest/              → 74 Kompiler + Harvester      (Crate omegaflow-harvest)
  measure/              → 55 Messung/Probe             (Crate omegaflow-measure)
  register/             → 10 Register/Verwaltung       (Crate omegaflow-register)
  service/              → 5 Live-Dienste/Watchdogs     (Crate omegaflow-service)
  science/              → 4 Paper/arxiv                (Crate omegaflow-science)
  gate/                 → 2 Gate-Bins                  (Crate omegaflow-gate)
  utils/                → 9 Reader/Utilities + spk_split (Crate omegaflow-utils)
```
