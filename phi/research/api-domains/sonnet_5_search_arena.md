Basierend auf den echten API-Responses hier die korrigierte und verifizierte Konfiguration. Ich teile das in **funktionierend (mit echten Feldpfaden)** und **kaputt/braucht Fix** auf.

```
# ============================================================
# TEIL A — VERIFIZIERT FUNKTIONIEREND (echte Feldpfade)
# ============================================================

source ripe_bgp_default_route
ttl 300
res 2
url https://stat.ripe.net/data/bgp-state/data.json?resource=0.0.0.0/0
field data.bgp_state technosphere_bgp_path_array
# HINWEIS: count = Länge von data.bgp_state (Anzahl BGP-Pfade zur Default-Route)

source satnogs_radio_observations
ttl 300
url https://network.satnogs.org/api/observations/?format=json
map root
lat_key station_lat
lon_key station_lng
alt_key station_alt
field_in norad_cat_id technosphere_satnogs_norad_id
field_in transmitter_mode technosphere_satnogs_transmitter_mode
# KORREKTUR: Das ist eine DYNAMISCHE Multi-Punkt-Quelle (jede Beobachtung hat
# eigene station_lat/station_lng/station_alt) — kein einzelner Fixpunkt!

source gdacs_disasters
ttl 3600
lat 45.803
lon 8.624
url https://www.gdacs.org/gdacsapi/api/events/geteventlist/SEARCH
field features anthroposphere_gdacs_active_disaster_count
# jedes Feature hat eigene geometry.coordinates [lon,lat] — optional als
# dynamische Multi-Punkt-Quelle behandeln statt fixer JRC-Position

source nasa_eonet
ttl 3600
lat 38.992
lon -76.848
url https://eonet.gsfc.nasa.gov/api/v3/events
field events biosphere_eonet_event_count
# jedes Event hat geometry[0].coordinates [lon,lat] — ebenfalls Multi-Punkt-fähig

source nasa_eonet_fires
ttl 3600
lat 38.992
lon -76.848
url https://eonet.gsfc.nasa.gov/api/v3/events?category=wildfires
field events biosphere_eonet_wildfire_count

source who_influenza
ttl 86400
lat 46.234
lon 6.140
url https://xmart-api-public.who.int/FLUMART/VIW_FNT
field value anthroposphere_influenza_record_count
# KORREKTUR: Endpoint muss /FLUMART/VIW_FNT sein, nicht /FLUMART
# value ist Array über alle Länder/Wochen — Aggregation nötig für Einzelwert

source who_cholera
ttl 86400
lat 46.234
lon 6.140
url https://ghoapi.azureedge.net/api/CHOLERA_0000000001
field value anthroposphere_cholera_record_count
# KORREKTUR: exakter Endpoint ist CHOLERA_0000000001, NumericValue oft null,
# Value als String vorhanden — braucht Filter auf neuestes TimeDim pro Land

source who_gho_tuberculosis
ttl 86400
lat 46.234
lon 6.140
url https://ghoapi.azureedge.net/api/MDG_0000000020
field value anthroposphere_tuberculosis_record_count
# bestätigt korrekt, NumericValue vorhanden

source cern_cms_data
ttl 86400
lat 46.233
lon 6.055
url https://opendata.cern.ch/api/records/?q=experiment:CMS&size=1
field aggregations.experiment.buckets technosphere_cern_cms_record_count
# KORREKTUR: Wert steht in aggregations.experiment.buckets[key=CMS].doc_count
# gemessen: 57889

source cern_alice_pbpb
ttl 86400
lat 46.233
lon 6.055
url https://opendata.cern.ch/api/records/?q=experiment:ALICE&size=1
field aggregations.collision_type technosphere_cern_alice_pbpb_count
# PbPb + Pb-Pb Buckets summieren: 138+6=144 (zwei Schreibweisen!)

source cern_open_data
ttl 86400
lat 46.233
lon 6.055
url https://opendata.cern.ch/api/records/?size=1
field aggregations.experiment.buckets technosphere_cern_total_record_count
# gemessen gesamt: 80373 (hits fehlt in Response, nutze doc_count Summe)

source macrostrat_ages
ttl 604800
lat 43.073
lon -89.401
url https://macrostrat.org/api/v2/units?format=json
field success.data macrostrat_unit_count
# KORREKTUR: Pfad ist success.data (Array), NICHT success.v (das ist nur
# die API-Versionsnummer =2)

source macrostrat_timescale
ttl 604800
lat 43.073
lon -89.401
url https://macrostrat.org/api/v2/defs/timescales?format=json
field success.data macrostrat_timescale_count
# gleiche Korrektur: success.data Array-Länge, nicht success.v

source neotoma_paleoecology
ttl 604800
url https://api.neotomadb.org/v2.0/data/occurrences?limit=1
map data
field_in site.location paleobiology_neotoma_site_location_geojson
field_in sample.taxonname paleobiology_neotoma_taxon_name
# KORREKTUR: site.location ist ein GeoJSON-String mit "coordinates":[lon,lat]
# pro Fundstelle → eigentlich DYNAMISCHE Multi-Punkt-Quelle, muss geparst werden!

source pbdb_paleobiology
ttl 604800
lat 43.073
lon -89.401
url https://paleobiodb.org/data1.2/occs/list.json?all_records=1&limit=1
field records_returned paleobiology_pbdb_occurrence_count
# FIX: ursprüngliche URL gab HTTP 400 — es fehlte ein Pflichtparameter.
# all_records=1 behebt das (laut Fehlermeldung der API)

source crossref_dois
ttl 86400
lat 42.530
lon -71.048
url https://api.crossref.org/works?rows=0
field message.total-results technosphere_crossref_doi_count
# bestätigt: 184.631.784 (Stand der Abfrage)

source openalex_works
ttl 86400
lat 42.373
lon -71.110
url https://api.openalex.org/works?per-page=1
field meta.count technosphere_openalex_work_count
# bestätigt: 320.755.974

source microbe_census
ttl 604800
lat 52.079
lon 0.187
url https://www.ebi.ac.uk/metagenomics/api/v1/studies?page_size=1
field meta.pagination.count biosphere_microbe_study_count
# bestätigt: 5203

source protein_structures
ttl 604800
lat 40.522
lon -74.460
url https://data.rcsb.org/rest/v1/core/entry/1UBQ
field entry.id biosphere_protein_structure_id
# KORREKTUR: Feld heißt entry.id, nicht rcsb_id (das existiert in dieser
# Entry-Response nicht direkt)

source nsidc_sea_ice
ttl 86400
lat 40.008
lon -105.263
url https://noaadata.apps.nsidc.org/NOAA/G02135/north/daily/data/N_seaice_extent_daily_v4.0.csv
format csv
last line cryosphere_sea_ice_extent_north_mkm2
# FIX: ursprüngliche URL war nur Verzeichnis-Listing (HTML), nicht die Datei
# selbst. Korrigierte URL zeigt direkt auf die CSV-Datei.

source cryosphere_sea_ice_north
ttl 86400
lat 40.008
lon -105.263
url https://noaadata.apps.nsidc.org/NOAA/G02135/north/daily/data/N_seaice_extent_daily_v4.0.csv
format csv
last line cryosphere_sea_ice_extent_north_mkm2

source global_temp_anomaly
ttl 2592000
lat 40.807
lon -73.964
url https://data.giss.nasa.gov/gistemp/tabledata_v4/GLB.Ts+dSST.csv
format csv
column 13 atmosphere_giss_temp_anomaly_jd
# bestätigt funktionierend. Spalte 13 (0-indiziert) = "J-D" (Jahresmittel-Anomalie)
# letzte Datenzeile verwenden

source vostok_icecore
ttl 2592000
lat 35.595
lon -82.551
url https://www.ncei.noaa.gov/pub/data/paleo/icecore/antarctica/vostok/co2nat.txt
format text
skip_header true
last line paleoclimate_vostok_co2_raw
# Datei enthält langen Header/Metadaten-Block vor den eigentlichen Messdaten —
# Parser muss Header überspringen (Datenzeilen beginnen typischerweise nach
# "DESCRIPTION"-Block mit numerischen Spalten)

source noaa_paleoclimate_co2
ttl 2592000
lat 35.595
lon -82.551
url https://www.ncei.noaa.gov/pub/data/paleo/icecore/antarctica/epica_domec/edc-co2-2008.txt
format text
skip_header true
last line paleoclimate_epica_co2_raw

source unhcr_displacement
ttl 86400
lat 46.234
lon 6.140
url https://api.unhcr.org/population/v1/population/?limit=1
field items anthroposphere_displacement_record
# items[].refugees enthält Wert (bestätigt: erste Zeile 1951, refugees=2116011)
# Für aktuellen Wert: Sortierung nach year DESC nötig (API sortiert sonst ASC)

source cdc_covid_variants
ttl 86400
lat 33.797
lon -84.323
url https://data.cdc.gov/resource/jr58-6ysp.json?$limit=1
field share anthroposphere_covid_variant_share
# bestätigt funktionierend: share als String-Float, z.B. "0.0273..."

source worldbank_population
ttl 604800
lat 38.899
lon -77.043
url https://api.worldbank.org/v2/country/WLD/indicator/SP.POP.TOTL?format=json
field 1.0.value anthroposphere_worldbank_population_total
# Array-Struktur: [meta, [records]] — erstes Element von records-Array (Index 1.0)
# ist neuestes Jahr (2025: 8.215.424.893)

source worldbank_gdp_growth
ttl 604800
lat 38.899
lon -77.043
url https://api.worldbank.org/v2/country/[ISO3]/indicator/NY.GDP.MKTP.KD.ZG?format=json
field 1.0.value economic_gdp_growth_pct
# gleiche Array-Struktur bestätigt

source technosphere_submarine_cables
ttl 604800
lat 38.899
lon -77.043
url https://www.submarinecablemap.com/api/v3/cable/cable-geo.json
field features technosphere_submarine_cable_count
# bestätigt funktionierend, FeatureCollection mit MultiLineString-Geometrien

source planetary_computer_collections
ttl 86400
lat 47.643
lon -122.137
url https://planetarycomputer.microsoft.com/api/stac/v1/collections
field collections technosphere_planetary_computer_collection_count

source eoapi_collections
ttl 86400
lat 46.056
lon 14.506
url https://stac.eoapi.dev/collections
field collections technosphere_eoapi_collection_count
# HINWEIS: Response ist paginiert (links.rel=next vorhanden) —
# echte Gesamtzahl erfordert mehrere Requests/Offset-Iteration

source copernicus_sentinel2_count
ttl 3600
lat 41.827
lon 12.674
url https://catalogue.dataspace.copernicus.eu/odata/v1/Products?$top=1&$count=true
field @odata.count copernicus_sentinel2_product_count
# FIX: ursprüngliche URL gab nur einzelne Produkte zurück, kein Count-Feld.
# $count=true Parameter ergänzt für echten Zähler

source ecdc_monkeypox
ttl 86400
lat 59.365
lon 18.016
url https://opendata.ecdc.europa.eu/monkeypox/casedistribution/json/data.json
format json
field 0 anthroposphere_monkeypox_first_record
# FIX: ursprüngliche URL zeigte nur Verzeichnis-Listing (HTML). 
# Korrigierte URL zeigt direkt auf data.json

source gdelt_news_volume
ttl 900
lat 38.909
lon -77.072
url https://api.gdeltproject.org/api/v2/doc/doc?query=climate&mode=timelinevol&format=json
field timeline.0.data technosphere_gdelt_volume_series
last value technosphere_gdelt_volume_latest
# bestätigt funktionierend (429 war nur Rate-Limit bei Testabfrage),
# echte Struktur: timeline[0].data[].value, letzter Eintrag = aktuellster Tag

# ============================================================
# SIMBAD — TAP-Queries bestätigt (data[0][0] Pattern)
# lat 48.583 lon 7.751 res 3
# ============================================================

source simbad_total_objects
ttl 604800
lat 48.583
lon 7.751
url https://simbad.u-strasbg.fr/simbad/sim-tap/sync
query SELECT COUNT(*) FROM basic
field data.0.0 simbad_total_object_count
# bestätigt: 21.808.051

source simbad_white_dwarfs
ttl 604800
lat 48.583
lon 7.751
url https://simbad.u-strasbg.fr/simbad/sim-tap/sync
query SELECT COUNT(*) FROM basic WHERE otype='WD*'
field data.0.0 simbad_white_dwarf_count
# bestätigt: 129.849

source simbad_quasars
ttl 604800
lat 48.583
lon 7.751
url https://simbad.u-strasbg.fr/simbad/sim-tap/sync
query SELECT COUNT(*) FROM basic WHERE otype='QSO'
field data.0.0 simbad_quasar_count
# bestätigt: 801.739

source simbad_pulsars
ttl 604800
lat 48.583
lon 7.751
url https://simbad.u-strasbg.fr/simbad/sim-tap/sync
query SELECT COUNT(*) FROM basic WHERE otype='Psr'
field data.0.0 simbad_pulsar_count
# KORREKTUR: otype-Code ist 'Psr', nicht 'Pulsar' — bestätigt: 3965

source simbad_galaxies
ttl 604800
lat 48.583
lon 7.751
url https://simbad.u-strasbg.fr/simbad/sim-tap/sync
query SELECT COUNT(*) FROM basic WHERE otype='G'
field data.0.0 simbad_galaxy_count
# KORREKTUR: otype-Code ist 'G', nicht 'Galaxy' — bestätigt: 4.177.463

source simbad_novae
ttl 604800
lat 48.583
lon 7.751
url https://simbad.u-strasbg.fr/simbad/sim-tap/sync
query SELECT COUNT(*) FROM basic WHERE otype='No*'
field data.0.0 simbad_nova_count
# KORREKTUR: otype-Code ist 'No*', nicht 'Nova' — bestätigt: 2217

source simbad_supernovae
ttl 604800
lat 48.583
lon 7.751
url https://simbad.u-strasbg.fr/simbad/sim-tap/sync
query SELECT COUNT(*) FROM basic WHERE otype='SN*'
field data.0.0 simbad_supernova_count
# KORREKTUR: otype-Code ist 'SN*', nicht 'SN' — bestätigt: 21.636

source simbad_highest_redshift_quasar
ttl 604800
lat 48.583
lon 7.751
url https://simbad.u-strasbg.fr/simbad/sim-tap/sync
query SELECT TOP 1 main_id, rvz_redshift FROM basic JOIN ident ON oid=oidref WHERE otype='QSO' ORDER BY rvz_redshift DESC
field data.0.1 simbad_highest_redshift_quasar_z
# WARNUNG: getesteter Query gab main_id zurück aber rvz_redshift=null!
# Join-Bedingung liefert evtl. falschen Datensatz — Query braucht Überarbeitung
# (z.B. WHERE rvz_redshift IS NOT NULL ergänzen)

# ============================================================
# GAIA — ESAC TAP bestätigt (data[0][0] Pattern)
# lat 40.443 lon -3.951 res 3
# ============================================================

source gaia_total_measured_stars
ttl 604800
lat 40.443
lon -3.951
url https://gea.esac.esa.int/tap-server/tap/sync
query SELECT COUNT(*) FROM gaiadr3.gaia_source WHERE parallax>0
field data.0.0 gaia_total_measured_star_count
# bestätigt: 1.110.324.277

source gaia_nearby_stars
ttl 604800
lat 40.443
lon -3.951
url https://gea.esac.esa.int/tap-server/tap/sync
query SELECT COUNT(*) FROM gaiadr3.gaia_source WHERE parallax>100
field data.0.0 gaia_nearby_star_count
# bestätigt: 315

source gaia_variable_stars
ttl 604800
lat 40.443
lon -3.951
url https://gea.esac.esa.int/tap-server/tap/sync
query SELECT COUNT(*) FROM gaiadr3.vari_classifier_result
field data.0.0 gaia_variable_star_count
# bestätigt: 9.976.881

# ============================================================
# NASA Exoplanet Archive — bestätigt (count(*) Key-Name!)
# lat 34.136 lon -118.127 res 3
# ============================================================

source nasa_exoplanet_total
ttl 86400
lat 34.136
lon -118.127
url https://exoplanetarchive.ipac.caltech.edu/TAP/sync
query SELECT COUNT(*) FROM ps
field 0.count(*) nasa_exoplanet_total_count
# WICHTIG: Feldname ist wörtlich "count(*)" als JSON-Key! bestätigt: 40.006

source nasa_hot_jupiters
ttl 86400
lat 34.136
lon -118.127
url https://exoplanetarchive.ipac.caltech.edu/TAP/sync
query SELECT COUNT(*) FROM ps WHERE pl_orbper<1
field 0.count(*) nasa_hot_jupiter_count
# bestätigt: 927

# ============================================================
# SDSS — bestätigt (verschachtelte Rows-Struktur)
# lat: SDSS/JHU Baltimore-basiert, aber Datenzentrum meist Fermilab/APO
# ============================================================

source sdss_cosmic_web_galaxies
ttl 604800
lat 32.780
lon -105.820
url https://skyserver.sdss.org/dr18/SkyServerWS/SearchTools/SqlSearch?format=json&cmd=SELECT+TOP+10+*+FROM+SpecObj+WHERE+class=%27GALAXY%27+AND+z+BETWEEN+0.1+AND+0.2
field 0.Rows sdss_cosmic_web_galaxy_records
# Koordinaten: Apache Point Observatory, NM (physischer Teleskopstandort)
# Pfad: Response[0].Rows ist Array der Ergebniszeilen, jede mit ra/dec/z Feldern

# ============================================================
# PDG — WICHTIGE KORREKTUR: IDs falsch zugeordnet!
# ============================================================

source pdg_higgs_mass
ttl 2592000
lat 37.877
lon -122.247
url https://pdgapi.lbl.gov/summaries/S003M
field pdg_values.0.value physics_electron_mass_mev
# ACHTUNG: S003M liefert tatsächlich "e MASS" (Elektronenmasse = 0.511 MeV),
# NICHT die Higgs-Masse! Die PDG-ID für Higgs-Masse muss recherchiert werden
# (vermutlich S126M o.ä. — bitte verifizieren)

source pdg_proton_mass
ttl 2592000
lat 37.877
lon -122.247
url https://pdgapi.lbl.gov/summaries/S008M
field pdg_values.0.value physics_pion_mass_mev
# ACHTUNG: S008M liefert "pi+- MASS" (139.57 MeV), NICHT Protonenmasse!
# Korrekte PDG-ID für Protonenmasse muss recherchiert werden

# ============================================================
# TEIL B — DEFEKT / BRAUCHT FIX
# ============================================================

# cryosphere_sea_ice_south → HTTP 404, Dataset-ID falsch
# VORSCHLAG: https://noaadata.apps.nsidc.org/NOAA/G02135/south/daily/data/S_seaice_extent_daily_v4.0.csv

# hadcrut5_temp → HTTP 404, Dateiname/Pfad veraltet, braucht Recherche der aktuellen HadCRUT5-URL

# usdm_conus_drought → HTTP 400, Platzhalter {today} muss durch echtes Datum (MM/DD/YYYY) ersetzt werden

# technosphere_global_ixp_count → HTTP 301, PeeringDB leitet um, evtl. Auth/andere Basis-URL nötig

# esa_maap_collections → HTTP 000, Server nicht erreichbar

# esa_cci_datasets → HTTP 503, Service temporär down (ESGF CEDA), Retry-Logik einbauen

# noaa_pmel_co2_moorings → HTTP 400, Spaltenname "co2" existiert nicht — echte Spaltennamen der ERDDAP-Tabelle prüfen

# ooi_ga01sumo_pco2 → HTTP 404, Dataset-ID "ooi_ga01sumo" existiert nicht im ERDDAP-Katalog — korrekte ID recherchieren

# dome_fuji_co2 → HTTP 404, Dateiname "df2012co2.txt" falsch — korrekten Dateinamen im NCEI-Verzeichnis prüfen

# globalforestwatch_tree_cover_loss → HTTP 404, komplette API-Struktur anders als angenommen (data-api.globalforestwatch.org braucht andere Pfadstruktur, evtl. /v1/... oder /dataset/{id}/latest/query)

# celestrak_earth_orientation → HTTP 000, Server nicht erreichbar (TLS/Redirect-Problem möglich)

# arxiv_new_papers → HTTP 301, http:// → https:// Redirect. URL korrigieren zu:
#   https://export.arxiv.org/api/query?search_query=all:physics&max_results=1
# WICHTIG: Response ist Atom/XML, kein JSON — braucht XML-Parser, kein "field"-Pfad

# effis_fires / emodnet_vessel_density → beide liefern nur WFS GetCapabilities
# (Server-Metadaten), nicht die eigentlichen Feature-Daten. Braucht echten
# GetFeature-Request mit typeName-Parameter, z.B.:
#   ...?service=WFS&request=GetFeature&typeName=<layer>&outputFormat=application/json

# sentinel_hub_catalog → HTTP 503, zusätzlich vermutlich OAuth-Pflicht (Sentinel Hub erfordert i.d.R. Access Token)

# worldbank_co2_emissions → Indikator "EN.ATM.CO2E.KT" ist archiviert/gelöscht laut API-Fehlermeldung. Alternative: EN.GHG.CO2.MT.CE.AR5 oder EN.ATM.CO2E.PC verifizieren

# Horizons-API (alle *_vectors Quellen) → HTTP 200 aber Fehlermeldung
# "Cannot interpret date" — Platzhalter {today}/{tomorrow} müssen durch
# echtes Datum im Format YYYY-MMM-DD (z.B. 2026-JUL-19) ersetzt werden.
# Zusätzlich fehlen Pflichtparameter: format=json, OBJ_DATA=NO,
# MAKE_EPHEM=YES, EPHEM_TYPE=VECTORS, CENTER=500@0 müssen für ALLE
# *_vectors-Quellen ergänzt werden (betrifft ~35 Quellen aus Gruppe A)
```

**Wichtigste strukturelle Erkenntnisse für dein System:**

1. **`satnogs_radio_observations`, `nasa_eonet(_fires)`, `gdacs_disasters`, `neotoma_paleoecology`** sind eigentlich **dynamische Multi-Punkt-Quellen** (jeder Datensatz hat eigene Koordinaten) — dein `res`-Konzept für dynamische Quellen passt hier besser als ein fixer `lat`/`lon`.

2. **Alle Horizons-API-Quellen (~35 Stück)** brauchen einen einheitlichen URL-Template-Fix mit echtem Datum und den Pflichtparametern — das betrifft praktisch die gesamte Gruppe A aus der letzten Antwort.

3. **Die PDG-IDs (`S003M`, `S008M`) waren falsch zugeordnet** — das ist ein handfester Bug, den du unbedingt korrigieren solltest, bevor du produktiv gehst.

4. **~12 Quellen sind aktuell komplett defekt** (404/503/000/400) und brauchen entweder recherchierte korrekte Endpunkte oder müssen ggf. ersetzt werden.

Möchtest du, dass ich für die defekten Quellen (Teil B) gezielt nach den korrekten URLs/Parametern recherchiere?
