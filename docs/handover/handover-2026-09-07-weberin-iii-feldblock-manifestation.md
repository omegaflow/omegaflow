<!--
  title: Handover — Weberin III: Feldblöcke, CDN-Manifestation, Register-Modell (local→CDN→API)
  class: handover
  date: 2026-09-07
  sha256: 10ee9e56fe51b36e241dd9b6ca408ce6fb45b30fd58af2a87ac41facd6a9bd75
  status: live
  see-also: docs/handover/handover-2026-09-07-weberin-zeugen-offen.md docs/concepts/archivar-mathematikerin.md docs/TODO.md
-->
# Handover — Weberin III: Feldblöcke, CDN-Manifestation, Register-Modell (local→CDN→API)

Übergabe der Weberin-III-Session (2026-09-07). Diese Session hat die Weberin-Faden-Lücken-Feldblöcke
gebaut und manifestiert, die Zeugen-Verdikte umgesetzt, das CDN bereinigt und das Register-Modell
auf den Grundsatz korrigiert: **eine Quelle wird an ihrer Origin-API registriert, nicht an einer
CDN-Adresse** — der Archivar löst lokal→CDN→API selbst auf (`fetch_one`, src/archivar/fetch.rs:796).
Parallel arbeiten zwei andere Linien („Blockierte Quellen analysieren" und Korona/Nadel III); deren
Arbeit steht unten getrennt und ist nicht von dieser Session.

## Was gebaut + committet ist (HEAD 060b583)

Die folgende Arbeit ist committet (Englische Commit-Messages, `git log` bis HEAD).

**Faden-Lücken-Feldblöcke (Leads 2/3/4/7/9) + geo-Serien-Loader:**
- Core `src/archivar/geo.rs` — geo-Serien-Format (60-B-Strider, Magics BGR1/NRS1/SDN1/ARG1/FDS1/ISL1/GIC1/GBCO) + Loader (`extract.rs`, `main_flow.rs`).
- Compiler: `bgr_infrasound_compiler`, `noaa_nodd_bucket_harvester`, `superdarn_fitacf_compiler`, `argo_bgc_profile_compiler`, `fdsn_waveform_compiler`, `fmi_gic_compiler`, `iss_lis_compiler`, `gebco_bathymetry_compiler`.
- `src/archivar/hdf5.rs` — N_avail contiguous-UNDEF-Overflow gefixt (11 hdf5-Tests grün).

**Zeugen-Verdikte (Council 2026-09-07):**
- C1 NOAA-NRS: Spektral-Record (kein Oszillator); `noaa_nrs_psd`-Feldblock gestrichen, CDN-Asset bleibt als Record.
- C2 Presence-Nicht-Wesen: Ablehnungen stehen (Katalog ≠ Wesen am Punkt).
- C3 Terrain: epqs = echter Gestalt-Zeuge (Binding), hillshade = derived, macrostrat = catalog.
- D1 gebco-Gestalt-Zeuge: Archivar lädt `.gbco` über `Motion::Surface` (Station-Thread), NRS01 → −833 m.

**CDN-Manifestationen (alle bestätigt auf dem CDN):**
- Feldblöcke: `bgr_infrasound.bin`, `superdarn_fitacf.bin`, `noaa_nrs_psd.bin`, `argo_bgc.bin` (lief lang), `fdsn_waveform.bin`, `fmi_gic.bin`, `iss_lis.bin`, `gebco_bathymetry.gbco`.
- Schulden geschlossen: `qbo_30hpa.csv` (cpc.ncep.noaa.gov, bare-Netloc), `d20_thermocline.csv` (Cascade r.jina.ai), `omni2_serie_1h.bin` (decimate-min 60).
- Zeugen-Assets: icecat/antares/gw-skymap (dispatchen).

**Register-Modell-Korrektur (das Kern-Thema dieser Session):**
- Der Archivar löst lokal→CDN→API auf. Erreichbare APIs stehen als **Origin-URL**, nicht als CDN-Adresse.
- 90 `archive-api.open-meteo.com`-Quellen an die Origin-API re-registriert (`ac796ed`).
- `www`-Präfix redundant (Archivar folgt `-L`); qbo-Netloc auf bare `cpc.ncep.noaa.gov` vereinheitlicht (`be9e2b0`).
- `meteo_cache_manifest` publiziert Assets unter dem origin-abgeleiteten Namen (`d0b1f3e`); 153 alte `<station>_open-meteo_*.json`-Assets entfernt.

**Werkzeug + CDN-Bereinigung:**
- `tools/utils/src/bin/area_reconcile.rs` — 4-Wege-Abgleich (CDN/data/cache/sources.φ), deckt Feldblock-+Zeugen-Typen.
- CDN-Step-5-Bereinigung: 811 declined-keine-Messung-Assets entfernt (192 MB, 64 Hosts), 153 open-meteo-Alt-Assets.
- 50 Orphan-Kandidaten terminal: registriert ceic/seismic-api.unimelb/imis.bfs.de; blocked isc/globalcmt/cosmic/datalab; 18 dead/decline.
- Fixes: `OMEGAFLOW_TOKEN`-Secret (Sources-Write), `ned-cdn` checkout, d20-Cascade.

## Nicht publiziert / harte Grenzen (0 honored, gemessen)

IGETS (GFZ-SFTP-Server tot, DOI nur Metadaten) — `blocked`, Re-probe wenn GFZ migriert.
`aia*_fullyear.bin`, `dr3_stars_stable.bin` (Duplikat), `omegaflow_series_*`, `pioneer_navio_*`,
`galileo_skyfreq`, `lsst_roemer` = lokale Arbeits-/Parallel-Dateien, KEINE Manifestations-Schulden.

## Offen / für die nächste Session (Webein-Linie)

- **`ned-cdn` fehlgeschlagen (trotz checkout-Fix, zweiter Lauf 7m50s)** — Ursache klären (der `cargo build -p omegaflow-harvest --bin tap_compiler`-Weg; Vermutung: der NED-objdir-Walk-Budget-Gate oder der tap_compiler-Build).
- **`meteo-cdn` fehlgeschlagen (21s)** — den Neustart-Lauf 34159753485 prüfen (Origin-Fetch/Manifest-Fehler).
- **`argo_bgc.bin`** — ob der lange BGC-Harvest gelandet ist (zuletzt nicht in den Runs; prüfen + ggf. Re-dispatch).
- **~22 REVIEW-CDN-Assets** aus der dead/blocked-Reconciliation (simbad, celestrak, geofon, ncei, api.weather.gov, zenodo.org …) — keep/remove je Befund entscheiden.
- Der `area_reconcile`-Check (a) 30 lokale data/-Dateien — keine echten Schulden (oben), aber die `omegaflow_series_*`/`pioneer_navio_*`/`galileo_skyfreq` Arbeitsdateien liegen herrenlos (Parallel-Linie, unantastbar bis deren Besitzer entscheidet).

## Andere Linien (nicht diese Session)

- **„Blockierte Quellen analysieren"**: blocked_sources-Bereinigung, Oszillator/Zeugen/Serie-Drei-Tore-Doktrin (`360ccab`, `a21d4c8`). Uncommittet im Baum: `phi/blocked_sources.φ`, `src/archivar/rinex.rs`, `main_flow.rs`, `mod.rs`, `lib.rs`, `noaa_nodd_bucket_harvester.rs`, `docs/befund/befund-todo-gegen-code-leichen.md` — NICHT committen/anfassen (deren Arbeit).
- **Korona/Nadel III**: `solar_seconds_matrix_probe` (läuft ~14h) + `corona_conditional_probe` (`060b583`, committet) — läuft unter systemd `solar-seconds-matrix`, Operator behält im Blick.
- `docs/handover/handover-2026-09-07-weberin-zeugen-offen.md` — die ältere Zeugen-Übergabe (teils überholt durch die Drei-Tore-Arbeit der anderen Linie).

## Verifikation

`cargo check --workspace` 0/0; `cargo test anchor_bodies` grün (sources.φ parst, kein refused); `area_reconcile` läuft (CDN live). `OMEGAFLOW_TOKEN` = gh-Account-Token (push auf omegaflow/sources, verifiziert).
