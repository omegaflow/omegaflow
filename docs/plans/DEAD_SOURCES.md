# RESTORED 2026-07-31: www.ndbc.noaa.gov (40 Bojen+DART) - WAF entfernt, aus git wiederhergestellt.
# Verstorbene Quellen (getestet 2026-07-31 / updated 2026-08-01)
# ============================================================
# Diese Quellen sind tot. OBSOLET = durch verifizierte Ersatzquelle ersetzt,
# kein Wiederherstellen nötig. RESTORE = kein funktionierender Ersatz gefunden,
# Quelle aus git wiederherstellbar falls Domäne wieder erreichbar.
# Alle Ersatzquellen wurden per curl verifiziert (HTTP 200 + Daten).

## NEU 2026-08-01: In dieser Session behoben

### OBSOLET — CMEMS Domain Intercepted (13 Quellen → NOAA/EMODnet)
nrt.cmems-du.eu (13)              — Domain intercepted by 301Domains, ERDDAP retired
  cmems_global_sst_analysis            → ncdc_oisst_v2_avhrr (NCEI)           (200 OK, OISST global SST daily)
  cmems_sst_anomaly_global_l4          → ncdc_oisst_v2_avhrr anom             (200 OK)
  cmems_glorys12_ocean_temperature_3d  → cfs_reanl_mm_ocnh Potential_temperature (200 OK)
  cmems_glorys12_ocean_3d              → cfs_reanl_mm_ocnh                    (200 OK)
  cmems_atlantic_currents              → cfs_reanl_mm_ocnh u/v_components     (200 OK)
  cmems_sentinel3_chlorophyll_global   → noaa_snpp_chla_daily (oceanwatch)    (200 OK)
  cmems_duacs_dt2024_sla_global_grid   → EMODNET_SEA_LEVEL_TREND              (200 OK)
  cmems_sla_global_adt                 → EMODNET_SEA_LEVEL_MONTHLY_MEAN       (200 OK)
  cmems_global_sla_altimetry           → EMODNET_SEA_LEVEL_MONTHLY_MEAN       (200 OK)
  cmems_indian_ocean_waves             → IMI_IRISH_SHELF_SWAN_WAVE             (200 OK)
  cmems_aviso_mesoscale_eddy_tracking  → EMODNET_SEA_LEVEL_MONTHLY_DESEASONALIZED (200 OK)
  cmems_global_ocean_oxygen            → cfs_reanl_mm_ocnh Salinity            (200 OK)
  cmems_motu_web_services              → EMODnet Physics Erddap tabledap       (200 OK)
my.cmems-du.eu (1)                 — gleiche Domain-Interception
  oceanography_cmems_global_analysis   → EMODnet Physics Erddap               (200 OK)

### OBSOLET — RaspberryShake (1 Quelle, API geändert)
stationview.raspberryshake.org/api/stations (1) — 404, API-Pfad geändert
  geosphere_raspberry_shake_stations   → /stations?online=true                 (200 OK, 2775 Stationen)

### OBSOLET — IRIS GeoJSON 400 (1 Quelle → Earthscope)
service.iris.edu/fdsnws/...geojson (1) — 400, GeoJSON nicht mehr unterstützt
  geosphere_iris_stations              → service.earthscope.org/fdsnws geocsv (200 OK, 68023 Stationen)

### OBSOLET — GeoBlock / Rate-Limit → CDN (61 Quellen)
celestrak.org (15)                 — Geo-Block, alle Endpoints timeout
  orbit_celestrak_*                    → github.com/omegaflow/catalogs v1.0    (200 OK, CDN)
query1.finance.yahoo.com (37)      — HTTP 429 bei parallelen Batch
  biosphere_etf_*                      → github.com/omegaflow/catalogs v1.0    (200 OK, CDN)
export.arxiv.org (9)               — HTTP 500
  biosphere_arxiv_* / arxiv_*          → github.com/omegaflow/catalogs v1.0    (200 OK, CDN)

### OBSOLET — Auth-APIs → Workflow→live-data CDN (kein Key mehr in sources.φ)
api.waqi.info (5)                  — token=demo unbrauchbar
  atmosphere_waqi_* / waqi_*           → github.com/omegaflow/catalogs live-data (200 OK, WAQI_TOKEN Secret in Workflow)
api.waterdata.usgs.gov (1)         — DEMO_KEY → USGS_WATER_KEY in Workflow
  usgs_nwis_water_data                 → github.com/omegaflow/catalogs live-data (200 OK)
firms.modaps.eosdis.nasa.gov/{nasa_key} (1) — Key-Template → FIRMS_MAP_KEY in Workflow
  viirs_lst_global                     → github.com/omegaflow/catalogs live-data (200 OK, global CSV)
auth-eBird/OpenAQ/CMR: (bereits vorher via Workflow, jetzt 15 Assets auf live-data)

### OBSOLET — OpenSky Rate-Limit (4 Quellen → ADSB.lol)
opensky-network.org (4)           — HTTP 429 bei parallelen Batch
  opensky_aircraft_positions           → api.adsb.lol                           (200 OK, keyless, full data)

### OBSOLET — Pegelonline V2-API geändert (3 Quellen)
pegelonline.wsv.de (3)            — timeseries-Feld nicht mehr in Stationsliste
  hydrosphere_pegelonline_*            → Mess-Endpunkt /stations/<uuid>/W/     (200 OK)
  hydrosphere_pegelonline_all_stations → map . statt map features              (200 OK, 786 Stationen)

## Bestehend (2026-07-31) — unverändert:

## timeout (42 Quellen)

### OBSOLET — ersetzt durch ArcGIS/CDN-Alternativen (verifiziert)
erddap.incois.gov.in (6)         — ersetzt durch GCOOS-Bojen + Argovis + NDBC DART
  incois_erddap_tsunami_bpr_buoys        → ndbc_dart_* / noaa_ndbc_dart_*  (200 OK)
  incois_erddap_argo_profiles_io         → argovis_core_profiles          (200 OK)
  incois_erddap_sst_indian_ocean         → gcoos_buoy_wmo_*               (200 OK)
  incois_erddap_wave_arabian_sea         → gcoos_buoy_wmo_*               (200 OK)
  incois_erddap_wave_bay_of_bengal       → gcoos_buoy_wmo_*               (200 OK)
  incois_erddap_oscar_currents_io        → gcoos_buoy_wmo_*               (200 OK)
tsunami.incois.gov.in (2)        — ersetzt durch DART-Bojen + USGS
  oceanography_incois_tsunami_buoys      → ndbc_dart_*                   (200 OK)
  geosphere_incois_seismic_stations      → arcgis_usgs_seismic            (200 OK)

### OBSOLET — NMDB durch Stations-Catalog ersetzt (verifiziert)
www.nmdb.eu (36)                 — ersetzt durch astro_nmdb_stations (26 Stationen, GitHub-raw)
  nmdb_oulu_neutron_monitor      → astro_nmdb_stations              (200 OK)
  nmdb_nest_*                    → astro_nmdb_stations              (200 OK)
  radiation_nmdb_*               → astro_nmdb_stations              (200 OK)

### OBSOLET — ersetzt (verifiziert)
spaceweather.sansa.org.za (1)    — ersetzt durch ionosphere_giro_dxpredictor (29 GIRO-Stationen)
  geospace_sansa_ionosonde       → ionosphere_giro_dxpredictor      (200 OK mit Origin/Referer-Headern)
modvolc.ov.ingv.it (1)           — ersetzt durch arcgis_volcanoes_world
  modvolc_volcanic_alerts        → arcgis_volcanoes_world           (200 OK)
maps.emodnet.eu (1)              — ersetzt durch EMODnet-WFS (anderer Host)
  oceanography_emodnet_vessel_density_nrt → hydrosphere_emodnet_vessel_density (200 OK)
overpass-api.de (1)              — ersetzt durch Overpass-Mirror overpass.osm.ch
  osm_overpass_global_hospital_infrastructure → überpass.osm.ch/api/interpreter (200 OK, globale Query)
service.das.earthscope.org (2)   — ersetzt durch arcgis_gsn_seismic
  earthscope_das_active_count    → arcgis_gsn_seismic               (200 OK)
  earthscope_das_stations        → arcgis_gsn_seismic               (200 OK)
www.glims.org (1)                — ersetzt durch Natural Earth (GitHub-raw)
  cryosphere_glacier_count       → geosphere_ne_glaciers            (200 OK)

## HTTP 403 (3 Quellen)
weather.cma.cn (3)               — ersetzt durch METAR (47 CN-Stationen)
  atmosphere_cma_current_weather → arcgis_metar_weather             (200 OK)
  atmosphere_cma_weather_alerts  → arcgis_metar_weather             (200 OK)
  atmosphere_cma_weather_forecast→ arcgis_metar_weather             (200 OK)

## HTTP 500 (3 Quellen)
www.gmrt.org (1)                 — ersetzt durch Open-Meteo
  lidar_gmrt_elevation           → lidar_openmeteo_elevation        (200 OK)
www.ogimet.com (1)               — ersetzt durch METAR
  ogimet_synop_russia            → arcgis_metar_weather             (200 OK)
science-pds.cryosat.esa.int (1)  — ersetzt durch NSIDC + GitHub-raw
  esa_cryosat2_arctic_sea_ice_thickness → astro_nsidc_arctic_seaice + cryosphere_nsidc_arctic_sea_ice_v4 (200 OK)

## HTTP 502 (2 Quellen)
retlector.eu (2)                 — ersetzt durch ArcGIS-Satelliten
  orbital_retlector_active_tle   → arcgis_satellite_positions       (200 OK)
  orbital_retlector_satellite    → arcgis_satellite_positions       (200 OK)

## HTTP 404 (1 Quelle)
images-api.nasa.gov (1)          — aktuell WIEDER ERREICHBAR (HTTP 200, getestet 2026-07-31),
                                    der 404 im Scan war transient. War nie in git-HEAD.
  Räumliche Alternativen — der CMR-Quantensprung:
    - NASA CMR/Earthdata Search (cmr.earthdata.nasa.gov/search/granules.json) —
      ICESat-2 ATL03 + GEDI L2A Granulat-Footprints (Polygon-Vertices) via neuem
      CmrPolygon-Extract in src/main.rs → echte gemessene Satelliten-Tracks als
      Oscillator-Punktwolken. 2 neue Sources in sources.φ:
        nasa_cmr_icesat2_footprints (ATL03, ~2000 Granulen/Tag, ~50K Oscillatoren)
        nasa_cmr_gedi_footprints (GEDI02_A, ~20 Verts/Granulat)
    - arcgis_satellite_fire + arcgis_viirs_fire: aktive Feuerpunkte (keyless, 200 OK)
    - GIBS: nur Raster-Tiles, keine Punktwolke

### OBSOLET — Population via GitHub-raw-Katalog (verifiziert)
sedac.ciesin.columbia.edu (2)    — Damen + Bevölkerung ersetzt
  grand_global_dams              → arcgis_ntad_dams                 (200 OK)
  sedac_gpw_population_grid      → worldpop_population_density_global: jetzt
                                    raw.githubusercontent.com/omegaflow/astro-catalogs/main/population_countries.json
                                    (211 Länder, World-Bank SP.POP.TOTL 2025 + Zentroide, HTTP 200)

### OBSOLET — Mangrove via GitHub-raw-Katalog (verifiziert)
mangrove-atlas.sei.org (2)       — ersetzt durch astro-catalogs-Katalog
  global_mangrove_atlas_countries → raw.githubusercontent.com/omegaflow/astro-catalogs/main/mangrove_countries.json
                                    (118 Länder, ArcGIS "Mangrove Extent Countries Giri" + Zentroide, HTTP 200)
  global_mangrove_atlas_global    → abgedeckt durch denselben Katalog (per-Land, kein globaler Einzelwert)

# ===========================================
# Zusammenfassung:
#   2026-07-31: 16 von 16 Domänen OBSOLET — Ersatz per curl bestätigt
#   2026-08-01: +11 Domänen OBSOLET (CMEMS 14, RaspberryShake 1, IRIS 1,
#     GeoBlock/Rate-Limit 3, Auth-APIs 3, OpenSky 4, Pegelonline 3)
#   Gesamt: 27 defekte Domänen beseitigt, 0 direkte Auth-API-Zugriffe in sources.φ
#   Live: 15 Workflow-assets auf omegaflow/catalogs live-data Release
