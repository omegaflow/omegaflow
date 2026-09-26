<!--
  title: Handover — Mycelium-Folge 174 (2026-09-26)
  session: Mycelium-Folge 174
  class: handover
  date: 2026-09-26
  sha256: 0f934300c4fb80eb82423976c6d8f21320b8f753685d96ed91cf789fa82b536a
  status: live
-->
# Handover — Mycelium-Folge 174 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `operator-gebunden` |
`blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-26-mycelium-folge173.md`.

## Operator-Wort-Register

- Wort | 2026-09-26 | „all" — session-weiter Consent (`mycelium_go`), Delegation an alle Taucher.
- Wort | 2026-09-27 | „DEMETER ist Sensory" → DEMETER aus der Mycelium-Übergabe entfernt (Sensory führt ihn).
- Wort | 2026-09-27 | „du machst GOSAT" → GOSAT bleibt Mycelium-Punkt; Re-pack committet `cb339a5c9`, Rest-Arme offen.
- Frage | 2026-09-27 | „wofür brauchen wir modis?" — LST-Thermalfeld gemessen (`sources.φ:15847-15872`); `modis-cdn`-Dispatch wartet auf Entscheidung (Grenze RAM/2-GiB).

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### modis-cdn — Sharding gebaut, RAM-/2-GiB-Cap-Riss
- **Status:** operator-gebunden | **Bindung:** eigen
- **Trigger:** Operator-Wort (feinerer Granule-Batch-Split vs. begrenztes Zeitfenster).
- **Lage:** (gemessen 2026-09-27 via grind-flash) `modis-cdn.yml` geshardet: 3 Produkte × 27 Jahre = 81 Shard-Jobs (`max-parallel 8`, `timeout-minutes 350`, Resume je Shard), actionlint 0 Fehler, CMR-Pfad live (daily 2000 = 266 Granule). Riss: Compiler akkumuliert alle Records im RAM (`modis_lst_cmg_compiler.rs:505`), 1 Granule = 688 MB, daily-Jahr ≈ 252 GB gegen 16 GB Runner-RAM und 2-GiB-Release-Asset-Limit (`modis_lst_cmg_daily.bin` schon 1,72 GiB).
- **Blockade:** RAM-/Asset-Grenze; Entscheidung fehlt.
- **Braucht:** Operator-Entscheidung; zudem Shard-Asset-Registrierung bzw. Descope der kanonischen `modis_lst_cmg_{daily,8day,monthly}.bin` (`phi/sources.φ:15565/15574/15583`).

#### Pre-CDN params — TIRM/TLON-Koordinaten-Riss (MCQG gelöst)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `source_keyer`-Re-Run grün (keine MCQG-Riss-Wiederholung).
- **Lage:** (gemessen 2026-09-26 via `source_keyer`-Re-Run) MCQG-Riss gelöst → `source magnetosphere_mcq_hapi` (keine Kollision); VALL no-op. Neu: TIRM `coords unresolved` (`on earth -22.22 114.1` vs supermag LRM `-21 115`, >0.05) und TLON `coords unresolved` (`on earth 45.408 16.659` matcht LON `45.4081 16.659201` UND P01 `45.41 16.66`).
- **Blockade:** keine.
- **Braucht:** TIRM/TLON-`on earth` gegen die echte Station messen (supermag_stations.φ oder Queue-Koordinate korrigieren).

#### IRIS/EarthScope EMC netCDF-4 — volume-Extract (Rat-Vorlage steht)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Architektur-Wort `volume`.
- **Lage:** (gemessen 2026-09-26 via `sread`/`sgrep`) netCDF-4 ist HDF5 (`0x89 48 44 46`). Der `Volume`-Container steht bereits (`src/archivar/volume.rs:151` — dims `[d,la,lo]` u32, Achsen `Axis::{Uniform,Explicit}`, `data: Vec<f32>`, Trilinear `sample_at:167`, `cell:162` Zeilen-Major, Bin `write_bin:191`/`read_bin:233`, Magic `0xCF 0x86 0x0D 0x01`), der Konsument steht (`format "volume"` `main_flow.rs:3187` → `archive.volumes:359` → `Buffer.volumes` `spatial.rs:48` → GPU `ensure_volumes`/`upload_volumes` `omega.rs:749`/`:813`). Der GPU-Sample ist presence-geodätisch `(lat_deg, lon_deg, depth_km)` mit `depth_km = -h/1000.0` (`motion.rs:425`, positiv nach unten) — EMC-Tiefe km-positiv-unten passt mit `depth_scale 1.0`. Was fehlt: `Extract` (`types.rs:136`) ohne `Volume`-Variante; `build_netcdf4_channels` (`channels.rs:423`) kennt nur `ProfileMap`; `FetchResult` (`fetch.rs:1138`) ohne Volumes-Slot; Grammatik-Arm `"volume"` fehlt (`parse.rs:297` = `profile`-Muster, `"format"` frei `:1318`). EarthScope-ToS: Attribution + Non-human-Visitor (Fetch bleibt Browser-Brücke/Operator-Profil; hier nur der Extrakt).
- **Blockade:** Architektur-Akt (neue `volume`-Direktive) — Rat/Operator-Wort `volume`.
- **Braucht:** Wort `volume`; danach bauen: (1) `Extract::Volume { value_key, lat_key, lon_key, depth_key, depth_scale, name }` in `types.rs` nach `ProfileMap:208` (kein `fields`-Vec — s. Vertrags-Frage d); (2) Grammatik-Arm `"volume"` in `parse.rs` neben `"profile":297`; (3) `build_netcdf4_volume(src, bytes) -> Option<(String, Volume)>` in `channels.rs` — liest `read_f64_dataset`/`read_f32_dataset` (`hdf5.rs:2787`), `dims` (`:2820`), `_FillValue` (`attr_f64:2816`), Achsen-Re-Order auf `[depth,lat,lon]` + Monotonie-Gate (`volume.rs:70`); (4) Arm `format "volume_netcdf"` neben `main_flow.rs:3187` (CDN-Stamp + `held.push((name, volume))`), kein `FetchResult`-Umbau.
  - Rat-Fragen (das Wort entscheidet): (a) Achsen-Ordnung kanonisch `[depth,lat,lon]` vs. Quelle `(lat,lon,depth)` — `cell:162`/GPU `upload_volumes:838` nehmen `[depth,lat,lon]`, Re-Order im Bau-Schritt; (b) statisches Gitter (ein `Volume`/Quelle) vs. Serie (4D — eigener späterer Arm); (c) `read_f32_dataset` nativ vs. `read_f64`→cast; (d) **Vertrag:** GPU-Head trägt nur `[data_off,dims×3,kinds×3,offs×3]` (10 u32, `omega.rs:780`), Uniform `[depth,lat,lon,valid,count,pad,pad,pad]` (`:838`) — kein `force`/`kernel`/`tau`; deklariert-aber-unverbraucht wäre Fabrikation → bare Gitter (`pending`) oder Head-/Uniform-Vertrag erweitern (Atom); (e) `_FillValue`/NaN im Gitter vs. `read_bin` lehnt non-finite ab (`volume.rs:325`) — Fill-Maske oder Domänen-Maskierung.

#### Voyager 1/2 closed-loop Doppler — descoped (kein öffentlicher Cruise-Endpunkt)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** — (abgeschlossen; neu nur bei neuer Fundstelle).
- **Lage:** (gemessen 2026-09-27 via research-max/`archive_search`) kein öffentlicher Cruise-ODF/ATDF: PDS/SPDF `cruise/` 404, `radio_science_rss` nur Okkultation; NSSDC nur Encounter (PSCM-00003/4 „ready for offline distribution", UNIVAC-Binär); Wayback-nssdcftp nie Tracking; `--all`/openalex/zenodo/datacite ohne Datensatz; ODR→ODF nicht konvertierbar.
- **Blockade:** keine (befundet).
- **Braucht:** descope-Eintrag `phi/blocked_sources.φ`/`dead_sources.φ` (Register-Duty); keine weitere Suche.

#### src.pas / esc.pithia.eu — MASER-Route + `epncore-spatial`-Gap
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** `epncore-spatial`-Parser-Arm (parser-def→mountain) ODER src.pas-Backend erholt.
- **Lage:** (gemessen 2026-09-27 via research-max) src.pas tot (PostgreSQL `:5432` refused, `/tap/tables` 500, `:8081/data/*` 404, kein Wayback). Ersatzroute nutzbar: `voparis-tap-maser.obspm.fr/tap` 200, `epn_core`-Klasse mit echtem H5-Sample (72 225 925 B, magic hdf5) — nicht dieselben Dateien, aber dieselbe Datenklasse. Parser-Gap `epncore-spatial`: `c1min/c1max/c2min/c2max/c3min/c3max`, `s_region` (STC-S), `c1/c2/c3_resol` — Region, kein lat/lon-Skalar; Anker `types.rs:136`/`Sample:41`.
- **Blockade:** Parser-Arm fehlt (parser-def); src.pas-Backend dritter.
- **Braucht:** `epncore-spatial`-Gap in `phi/blocked_sources.φ` → mountain-Linie; MASER-`epn_core`-Sample registrieren; LOFAR LTA (`lta.lofar.eu`, account-gated) als `blocked account`-Kandidat.

#### GOSAT-GW GWT3F_L1B — Arme + CDN-Workflow gebaut, Dispatch offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gosat-cdn`-Dispatch (Repo-Secrets `GOSAT_GW_MAIL`/`GOSAT_GW_PASS`).
- **Lage:** (gemessen 2026-09-27 via grind-pro) Re-pack + alle Arme committet (`fae4a5081`): `geo.rs`, `extract.rs`, `main_flow.rs`, `tests.rs`, `phi/sources.φ`-Block; `cargo check` 0/0. comp-Formel `(product−1)·3+band` (1..6). `gosat-cdn.yml` gebaut (Muster `gll-rss-odr`: idempotent, `--ci-mode`). Asset `gosat_tanso3.bin` nicht manifestiert.
- **Blockade:** `GOSAT_GW_MAIL`/`GOSAT_GW_PASS` als Repo-Secrets (ungemessen).
- **Braucht:** `gh workflow run gosat-cdn.yml -f product=GWT3F_L1B -f start=… -f end=…`; danach `sha256` in `phi/sources.φ` und `blocked_sources.φ:325` → released.

#### EMODNET HFRADAR NADR — Termin-Re-Messung
- **Status:** termin | **Bindung:** termin:2026-10-19
- **Trigger:** 2026-10-19.
- **Lage:** (gemessen 2026-09-24 via `external-state.md:43`) Asset registriert; Re-Messung offen.
- **Braucht:** `archive_search --sniff` bei Fälligkeit.

### Dritter

#### BepiColombo bc_mpo_more — Termin
- **Status:** termin | **Bindung:** termin:2027-04-01
- **Trigger:** 2027-04-01 (Science-Phase-Beginn).
- **Lage:** (gemessen 2026-09-26) PSA (Bentley) + PI (Iess): MORE-Cruise nicht öffentlich, Freigabe April 2027; Ticket `YYM-342-97327`.
- **Braucht:** Wiedervorlage 04/2027, dann MORE-`data_raw`/`calibration_raw`.

#### SSDC / Limadou — wartend auf CSES-02-Prozedur
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur `limadou.ssdc.asi.it`.
- **Lage:** (gemessen 2026-09-26 via `sfetch`) `query.php` → SSDC-CAS-Login (Auth-Redirect), neue Prozedur noch nicht live. Sotgiu (`mail_ledger.φ:79`): „wait a few weeks".
- **Blockade:** Portal-Umbau (dritter).
- **Braucht:** neue Prozedur abwarten; beim Trigger `query.php` re-messen.

#### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** 2026-09-28 (DEMETER) / 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-26 via `--verdict`) keine Erholung bei `regards.cnes.fr` (403) / `pithia.cbk.waw.pl` (500/TAP down) / `api.lasair.lsst.ac.uk` (404); Ersatzrouten gefunden (esc.pithia.eu, lasair-ztf).
- **Braucht:** `archive_search --verdict <url>`; bei Erholung den `*-cdn.yml`-Lauf dispatchen.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | `mariner-occlt-cdn` Lauf `36272916043` gelesen: `mariner_occlt.bin` liegt (1 375 496 B, sha256 4aa487cb…) | nächster Schritt: verbleibende Survey-Marker prüfen.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | Thomas SAMPLE_CONTACT (MPI-FKF/LAB_A) sagte am 2026-09-17 LAB_A-`I(q,t)`-Daten zu („few days"), danach kein Eingang | wartend auf Mail-Eingang (kein Nachfassen).
- `docs/surveys/survey-2026-09-26-secrets-inventar.md` | Dispositionen committet; Namens-Disposition trägt die Future-Übergabe.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | ausstehend nur Wiedervorlage 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending weiter tot; nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | Step 4 (CI-Dedupe) konkretisiert in `docs/auftrag/archiv/auftrag-saubere-datenbank.md:71-105` (Klassen-Zensus über 315 Workflows, Duplikat-Messung); offen: Step 5 (CDN-kanonisch, destruktiv → Operator-Wort) | nächster Schritt: Klassen-Zensus messen.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Migrationsplan-Vorlage steht (`:79-141`); stoppt am Operator-Wort | nächster Schritt: Operator-Wort zum Layout `knowledge/`+`backups/`.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | NOAA-NRS passive-bioacoustic entschieden (2026-09-27: Roh-Audio registriert `phi/sources.φ:9051`, Produkte `decline spectral-series` `phi/declined_sources.φ:4017`) | offen nur §7 Roh-Korpora/Scratch-Disposition | nächster Schritt: Operator-Wort.

## Abschluss

Commit-Wort (`/commit`) steht aus.
