<!--
  title: Survey — Die Weberin Faden-Matrix (thread matrix)
  class: survey
  date: 2026-09-07
  sha256: 8bc8cc92e5fabd4cdbce9989ea3e625855810ad0e18e9be57807259f13305573
  status: live
  see-also: docs/concepts/die-weberin.md docs/concepts/archivar-mathematikerin.md
-->

# Die Weberin — Faden-Matrix (thread matrix)

**Frame:** die-weberin §3 taxonomy (Kette: Body/Station/Direction worldlines) × §4 Schuss (witnesses per force medium). Statuses measured against `phi/sources.φ` (410 live/held blocks), `phi/blocked_sources.φ`, `phi/dead_sources.φ`, and the built bins in `tools/harvest/src/bin`. Force ids per `src/mathematikerin/force.rs` (em 0 … electric 8).

## I. Die Kette — warp threads

### I.1 Kette/Body — ephemeris worldlines (`Motion::Barycenter`)

| thread kind / actor-category | channel | data status | count-or-route |
|---|---|---|---|
| **Planets** (Mercury … Neptune, Earth) | body | in-register + built | 8 × `ephemeris_binary` |
| **Sun** | body | in-register + built | 1 (`ephemeris_sun`) |
| **Satellites** — Moon, Mars (2), Jupiter (9), Saturn (15), Uranus (5), Neptune (8), Pluto-system 5 (Charon/Hydra/Kerberos/Nix/Styx) | body | in-register + built | 45 × `ephemeris_binary` |
| **Dwarf/KBO/asteroid/comet** — Pluto, Ceres, Eris, Makemake, Haumea, Vesta, Bennu, Apophis, Encke + one 3I-named SPK file | body | in-register + built | 10 |
| **Spacecraft** — ISS, Juno, JWST, New Horizons, Parker Solar Probe, Solar Orbiter, Voyager 1/2 | body | in-register + built | 8 |
| **Wind** (L1 solar wind + orbit) | body | in-register + built | 1 × `orbit_bin` + `wind_waves` |
| **Small-body elements** (DASTCOM asteroid/comet catalog) | body-elements | in-register + built | `catalog_dastcom`, `dastcom_compiler` |
| **MPC orbits / observations** (independent second body-line) | body | in-register + built (Nachzug 2026-09-27: `mpcobs`/`mpcorb`, `phi/sources.φ`:2381/2395, `mpcobs_compiler`) | `docs/handover/archiv/handover-2026-09-09-mechanische-reste.md`, `mpcobs_compiler` built |

72 × `ephemeris_binary` + 1 `orbit_bin` = **73 registered body worldlines** (all `at <body>`, ttl 86400, CDN JPL-SPK, compiled by `ephemeris_compiler`/`wind_orbit_compiler`).

### I.2 Kette/Station — geodetic point worldlines (`Motion::Surface`)

| thread kind / actor-category | channel (force) | data status | count-or-route |
|---|---|---|---|
| **INTERMAGNET magnetometer network** | em (nT X/Y/Z + F) | in-register, fanout live | `fanout 154`, BGS GIN HAPI; + single ABK block |
| **INTERMAGNET dB/dt (1-h)** | electric/em | built (2 stations) | `abk_dbdt_1h`, `sod_dbdt_1h`; `intermagnet_dbdt_compiler` |
| **Pegel/flood gauges — UK EA** | gravity (stage m) | in-register, fanout live | `fanout 40`, flood-monitoring |
| **Tide/sea-level — NOAA CO-OPS** | gravity (m) | in-register, fanout live | `fanout 40`, water_level MLLW |
| **Sea-level — IOC** | gravity | in-register | stationlist service route |
| **River flow — USGS NWIS** | advective (cfs) | in-register (dynamic bbox) | `hydrosphere_river_flow_cfs` |
| **NDBC buoys** (wave height/period/dir acoustic, wind advective, water/air temp thermal, tide gravity) | acoustic + advective + thermal + gravity | in-register | 37 realtime txt stations (own blocks) + 36 historical stdmet + national ArcGIS sweep + 46092 |
| **Cabled ocean obs. — Ocean Networks Canada** (CTD conductivity/pressure/salinity/temp) | electric/advective/diffusion/thermal | in-register, fanout live | `fanout 8` × 4 channels |
| **Argo float** | thermal + diffusion | in-register (1 float, netcdf) | R1901843; BGC-Argo pending |
| **NOAA surface drifters** (SST + currents) | thermal + advective | in-register | erddap tabledap |
| **Weather — FROST met.no** (air temp / wind) | thermal / advective | in-register, fanout live | `fanout 40` each element |
| **Weather — NOAA GHCND** (TMAX) | thermal | in-register, fanout live | `fanout 50`, cdo-web |
| **METAR / BOM / Environment Canada / Barrow-permafrost / open-meteo point-archives** (gyirong, kollab, rasuwa + Himalaya; 30+ params) | thermal/diffusion/advective/acoustic/em (per register) | in-register | airports EDDF/EDDM/EGLL/KJFK/RJTT, ArcGIS sweeps, CDN meteo archive |
| **Mars surface stations — Mars2020 (Jezero), MSL (Gale)** | thermal + acoustic (Pa) | in-register | `on mars` 18.445/77.45 and −4.5895/137.4417 |
| **Air quality** — openAQ | diffusion | in-register, fanout live | `fanout 25` |
| **Air quality** — PurpleAir, sensor.community | diffusion | in-register (sweep/bbox) | pm2.5 / pm10+pm2.5 |
| **Aerosol — AERONET** (AOD) | em + diffusion | in-register | GSFC station |
| **Radiation environment — Safecast** | em (cpm) | in-register | bbox |
| **Physiology/operator sensors** (BIDSleep, MITDB, movement) | electric/advective (register labels) | in-register + built | own blocks; human-channel, pending |

Every `on earth`/`on mars` block is a `StationEntry {id, lat, lon}` → `Motion::Surface` → `body_fixed_to_icrs`. 220 fixed `on earth` + 2 `on mars` anchors; fanout families carry `stations`/`fanout` tokens (above).

### I.3 Kette/Direction — S² threads (`SkyDirection`, no distance)

| thread kind / actor-category | channel | data status | count-or-route |
|---|---|---|---|
| **dr3 stars / Tycho-2 / 2MASS** (density + crossmatch → SkyDirection) | direction | built | `vlies_density_compiler` (Stars, Twomass), `tycho2_compiler`; `direction_distance_join` verdict |
| **Broker transient loci — Lasair-ZTF** | direction (gmag w/ single-detection epoch only) | held (skydirection_compiler), positions-pending | blocked_sources.φ parser-def json (token) |
| **Broker — ANTARES loci (NOIRLab)** | direction (unbanded mag samples) | held, positions-pending | blocked_sources.φ parser-def json |
| **Broker — Fink-LSST cone** | direction (no photometry) | held, positions-pending | blocked_sources.φ parser-def json |
| **ALeRCE** | direction | unreachable (404 measured) | dead_sources.φ:4667; direction class held via Lasair/ANTARES/Fink |
| **TNS supernovae, ZTF/TESS lightcurves** | direction/em | in-register + built | `csv_zip`/`lightcurve`, `tns_compiler`, `ztf_lightcurves_compiler`, `tess_compiler` |
| **S² sense (the sphere itself)** | direction | built (LIVE) | `s2.rs`, `S2_WGSL`, `sky_reload`/`sky_tick` |
| **EOP / Earth orientation (IERS finals, UT1-UTC, polar motion)** | em (register label) | in-register + built | `finals` at earth |

## II. Der Schuss — weft witnesses per force medium

Status legend: **built** = compiler in tools/harvest → asset; **in-register** = live field/osc in `sources.φ`; **pending** = route/register duty identified; **not-published** = no open machine route in the system (0 honored, never fabricated).

### em (0)

| actor-category | data status | count-or-route |
|---|---|---|
| Optical/IR astro catalogs (SDSS QSO, NED z, RAVE, CARMENES, HECATE, GCVS/VSX, DENIS, MSX6C/AKARI/IRAS, exoplanet hosts, stellar teff) | in-register (+built where bins exist) | `at sun`, TAP/tapvizier + ssd jsons |
| 2MASS / Tycho-2 / DR3 star assets | built | `twomass_compiler`, `tycho2_compiler`, vlies feed |
| Radio surveys (NVSS, FIRST, CORNISH, ATNF pulsar, OH masers, MERLIN-family) | in-register + built | `radio_compiler` |
| FRB catalogs | in-register + built | `frbcat_flat`, `frb_a279`; `frb_compiler` |
| Gamma: Fermi-LAT 4FGL | built | `fermi_4fgl_compiler` |
| X-ray: Chandra CSC; X-ray binaries; magnetars | in-register | chandra_csc, lmxbdata, magnetar_flat |
| **VHE γ-ray: TeVCat** (published sources: HAWC/H.E.S.S./MAGIC/VERITAS/CTA detections) | built | tevcat_flat.json |
| VHE instruments individually (HAWC 2HWC, H.E.S.S./MAGIC/VERITAS/CTA event lists, LHAASO) | **gebaut** (HAWC `phi:12629`, LHAASO `phi:28529` — Nachzug 2026-10-16); H.E.S.S./MAGIC/VERITAS/CTA event lists bleiben not-published | hawc_compiler/lhaaso_compiler |
| Solar: GOES XRS/EUVS/X-ray, SDO AIA, SDO/EVE, GONG modes, HMI/WSO polar, F10.7 | in-register + built | goes/euvs/aia/eve/gong/hmi/wso/f107 compilers |
| Solar wind & IMF (OMNI, ACE, PSP, RTSW, GOES mag, DONKI CME/flare) | in-register + built | omni2_compiler; live hapi |
| Radio: Wind/WAVES; Breakthrough Listen L-band | in-register + built | wind_waves_compiler, bl_narrowband |
| Optical transient brokers as witnesses | held (built compiler), positions-pending | skydirection_compiler (Lasair/ANTARES/Fink) |
| Fireballs/bolides (JPL SSD fireball, GOES GLM) | in-register | em |
| Solar spectral irradiance (Mauna Kea) | built | spectral_compiler |
| **Particle provenance em (Cherenkov/shower)**: IceCube | built | `amon_compiler` (.amn1 alerts), `icecat_compiler` (S2E1+SKY1, ROOT_NEUTRINO); data-release portal 403 → blocked |
| ANTARES / KM3NeT ant20_01 events | built | `antares_vo_compiler` (S2E1 + SKY1, particle_root neutrino); VO route registered in blocked |
| Pierre Auger (UHECR) | built | `auger_compiler` (.pao1) |
| TA / KM3NeT detector / Super-K / JUNO catalogs | not-published in-system; **LHAASO ist gebaut** (`phi:28529`) | TA: nur Einzelereignis (Zenodo 8427755, kein Vollkatalog); KM3NeT/Super-K/JUNO no open route; KASCADE-Grande absent |
| CMB (Planck SMICA), dust extinction, cosmic flows | in-register + built | cmb_planck/cosmicflows/dust_map compilers |
| Pulsars, white dwarfs, binaries (sb9/WDS), KBO elements, comets (dcom5/cometels) | in-register + built | ssd jsons; mpcobs/kbo/dcom5/cometels compilers |

### gravity (1)

| actor-category | data status | count-or-route |
|---|---|---|
| GW scalar alerts (GraceDB `superevents.far`) | in-register | `gravity_wave_far` |
| **GW sky maps — LIGO/Virgo/KAGRA bayestar** | built (SKY1 kind gravity) | `gw_skymap_compiler`, `bayestar_compiler` (.be19); per-superevent asset, positions-pending |
| Tide/sea-level/stage gauges (ocean loading) | in-register | NOAA fanout 40, IOC, UK fanout 40, NDBC TIDE |
| Ground gravimeters (superconducting/relative networks) | in-register + built (Nachzug 2026-09-27: IGETS, `igets.bin`, `phi/sources.φ`:9955, `igets_compiler`) | harvester leads |
| Astrophysical gravity (sb9 orbits, exoplanet hosts, CBET, corot, pastel/polarbase) | in-register | at sun |
| INPOP25c asteroid masses (gravity catalog route) | pending | die-weberin §1 |

### acoustic (2)

| actor-category | data status | count-or-route |
|---|---|---|
| Ocean surface waves (NDBC buoy WVHT/DPD/APD/MWD) | in-register | 37+36 buoy blocks + national sweep |
| Mars atmospheric pressure (Mars2020/MSL) | in-register | on mars |
| Solar p-modes (GONG, BiSON) | built | gong_compiler/gong_series, bison/basu/shift |
| Air pressure fields (register acoustic label: METAR/OM pressure) | in-register | weather stations |
| **Infrasound (CTBTO IMS)** | BGR-Netz in-register + built (Nachzug 2026-09-27: `bgr_infrasound.bin`, `phi/sources.φ`:9751); die CTBTO-IMS-Route bleibt blocked account (vDEC) | harvester-leads |
| **Hydroacoustics / ocean noise** (NOAA NRS, ONC Hydrophon, PANGAEA PALAOA) | **gebaut** (`phi:11859` flac, `:10355` onc_hydrophone — Nachzug 2026-10-16); **Riss:** die Register-`origin` (Pfad-Stil) liefert 404, nur das GCS-`storage/v1`-JSON-Listing ist 200 | harvester-leads |

### seismic (3 body / 4 surface)

| actor-category | data status | count-or-route |
|---|---|---|
| Earthquake event feeds (JMA EEW/quake, p2pquake, vedur.is, GeoNet, USGS all_day + regional FDSN, seismicportal, INGV, ArcGIS Worldwide/ROMPLUS) | in-register | seismic-body / seismic-surface live |
| Archival quake catalog | built | `seismic_quakes_compiler` |
| Seismic station *networks* as station worldlines (IRIS/FDSN broadband) | Roh-Fetch-Block **steht** (`phi:8250` EarthScope FDSN `station/1`, `terms CC-BY-4.0`, Nachzug 2026-10-16); weitere Netze (GEOFON GE, RaspberryShake AM) offen | fdsn_station_compiler |

### thermal (5)

| actor-category | data status | count-or-route |
|---|---|---|
| Weather / land / permafrost stations (GHCND, FROST, METAR, BOM, Env-Canada, Barrow, Himalaya open-meteo) | in-register | fanout 40/50 + point blocks |
| Active-fire thermal remote sensing (FIRMS MODIS/VIIRS) | in-register | csv sweep |
| SST / water temp (drifters, NDBC, Argo, OOI, WQP) | in-register | erddap/arcgis |
| CMB temperature sky, stellar teff catalogs | in-register + built | planck smica, RAVE/pastel |

### diffusion (6)

| actor-category | data status | count-or-route |
|---|---|---|
| Air quality PM (openAQ, PurpleAir, sensor.community) | in-register | fanout 25 + sweeps |
| Trace gases (CO2/CH4/N2O/SF6 NOAA GML, ozone WOUDC) | in-register | Mauna Loa anchors |
| Aerosol AOD (AERONET), total column | in-register | GSFC |
| Ocean salinity / pCO2 / DO (WQP, ocean ArcGIS, PMEL CO2, OOI) | in-register | diffusion (salinity/CO2) |
| **BGC-Argo (O₂/pH/nitrate/chlorophyll)** | in-register (Nachzug 2026-09-27: Argovis `bgcargoplus`, `phi/sources.φ`:1250) | harvester-leads |

### advective (7)

| actor-category | data status | count-or-route |
|---|---|---|
| Wind (weather stations, NDBC, wind-profile 80/120/180 m, QBO) | in-register | + `qbo_compiler` |
| Solar wind bulk (OMNI, ACE, PSP, RTSW) | in-register + built | omni2_compiler |
| Ocean currents (drifters ve/vn), river discharge (USGS) | in-register | erddap / bbox |
| Aircraft ADS-B groundspeed | in-register | adsb |
| Stellar radial velocities (HECATE, RAVE/SDSS cz) | in-register | at sun, km/s |
| **HF-radar surface-current grids (IOOS HFRNet)** | in-register + built (Nachzug 2026-09-27: `hfrnet_rtv.bin`, `phi/sources.φ`:1428, `hfrnet_compiler`) | harvester-leads |

### electric (8)

| actor-category | data status | count-or-route |
|---|---|---|
| Swarm (EFI potential, FAC, radial ionospheric current; F em; N_ion diffusion; T_e thermal) | in-register | vires hapi |
| DEMETER ionospheric e-field | in-register + built | demeter_harvest/compiler |
| Solar Orbiter RPW e-field | in-register + built | live hapi + rpw_efield.bin |
| Heart/physiology channel (BIDSleep HRV — register electric label) | in-register | human sensor |
| **Geomagnetically induced currents (GIC)** | in-register + built — `fmi_gic.bin` (`sources.φ`:6907); Nachzug 2026-09-17 | leads |
| **Lightning networks (WWLLN / ground)** | declined — `decline commercial`, UW-copyright (Verdikt `5b980d570`, 2026-10-12 re-gehomt); keine Registerzeile | — |
| **SuperDARN polar radar** | in-register + built (Nachzug 2026-09-27: `superdarn_rawacf.bin`/`superdarn_fitacf.bin`, `phi/sources.φ`:9814/11487, `superdarn_*_compiler`) | leads |
| INTERMAGNET dB/dt (induction driver for GIC) | built (2 stations) | abk/sod 1-h bins |

**Gaps — thread categories with NO actor yet (Stand 2026-10-16, gemessen):** die restricted **CTBTO-IMS**-Route (roh vDEC blocked account; **offener Weg** = IRIS `fdsnws/station/1/query?network=IM`, 200 → IMS-Stationen inkl. Hydroakustik); die VHE/ν/CR-Teleskope **ohne** offenen Vollkatalog (TA, Super-K, JUNO, KM3NeT, KASCADE-Grande) — `pending`/`not-published`, nie null-erfunden. **Seither geschlossen (Nachzug 2026-10-16):** **hydrophone/ocean-noise** (NOAA-NRS, ONC, PANGAEA — `phi:11859`/`:10355`/`:10412`), **seismic station networks** (EarthScope `phi:8250`), **HAWC 2HWC + LHAASO** (`phi:12629`/`:28529`). **Geschlossen (Nachzug 2026-09-27):** ground **gravimeters** (IGETS), **infrasound** (BGR), **GIC**, **lightning** (WWLLN), **SuperDARN**, **HF-radar** (HFRNet), **BGC-Argo** (Argovis).
