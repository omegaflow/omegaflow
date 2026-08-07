# API-Übersicht – Earth Observation, Astronomie & Space APIs

Zusammenfassung von 23 gescrapten Dokumentationsseiten, sortiert nach Themenbereich.

---

## 🛰️ Satellitenbild- / Earth-Observation-Plattformen

### Planet Data API
- **URL:** https://docs.planet.com/develop/apis/data/
- **Zweck:** Zugriff auf Planets kompletten Bildkatalog (Items & Assets)
- **Auth:** Planet-Account erforderlich
- **Kernkonzepte:** Item (Scene), Item Type (z. B. `PSScene`, `SkySatCollect`), Asset
- **Funktionen:** Quick Search, Saved Search, STAC-Support (Beta)
- **Pagination:** über `_links` (`_self`, `_first`, `_next`, `_prev`)
- **Rate Limits:** Activation 2 req/s · Download 5 req/s · Search 5 req/s · sonstige 10 req/s
- **Max. Payload:** 5 MB
- **Doku:** [Items & Assets](https://docs.planet.com/develop/apis/data/items/) · [Item Search](https://docs.planet.com/develop/apis/data/item-search/) · [API Reference](https://docs.planet.com/develop/apis/data/reference/)

### Sentinel Hub
- **URL:** https://docs.sentinel-hub.com/api/latest/
- **Zweck:** Multispektrale/multitemporale Satellitenbildverarbeitung in Echtzeit
- **Hinweis:** Startseite ist größtenteils JS-gerendert; Kern-Inhalt liegt in der „API Reference“ (`/reference/`)
- **Teil von:** Copernicus Data Space Ecosystem

### Copernicus Data Space Ecosystem (CDSE)
- **URL:** https://dataspace.copernicus.eu/
- **Zweck:** Zentraler, kostenloser Zugang zu allen Sentinel-Missionen + Copernicus-Services
- **Tools:** STAC Browser, openEO, Sentinel Hub, JupyterLab, Data Workspace, Traceability-Service
- **APIs:** gebündelt unter `/analyse/apis` (STAC, openEO, Sentinel Hub, Download/Processing)
- **Login:** `identity.dataspace.copernicus.eu` (OpenID Connect)

### openEO
- **URL:** https://openeo.org/
- **Zweck:** Offene **API-Spezifikation** (kein eigener Dienst!) zur einheitlichen Anbindung an EO-Cloud-Backends (z. B. CDSE, openEO Platform)
- **Clients:** JavaScript, Python, R, QGIS, Julia (WIP)
- **API-Referenz:** https://openeo.org/documentation/1.0/developers/api/reference.html
- **Prozess-Referenz:** https://openeo.org/documentation/1.0/processes.html
- **Auth:** https://openeo.org/documentation/1.0/authentication.html
- **Aktuelle Version:** API 1.3.0 / Processes 2.0.0 RC2 (Stand Feb. 2026); seit Mai 2026 OGC Community Standard

### eoAPI (eoapi.dev)
- **URL:** https://eoapi.dev/
- **Zweck:** Open-Source-Framework zum Selbst-Hosten eines vollständigen EO-API-Stacks
- **Komponenten:**
  - **pgSTAC** – optimiertes Postgres-Schema für STAC-Kataloge
  - **stac-fastapi** – OGC-Features-konforme REST-Such-API
  - **titiler-pgstac** – Raster-Tiling-Service
  - **tipg** – Vector-Tiling-Service (OGC Features/Tiles)
- **Einsatz u. a. bei:** NASA IMPACT VEDA-Plattform
- **GitHub:** https://github.com/developmentseed/eoAPI

### Microsoft Planetary Computer
- **URL:** https://planetarycomputer.microsoft.com/docs/concepts/api/
- **Hinweis:** Seite ist vollständig JS-gerendert – kein Text per Fetch extrahierbar
- **Bekannt (aus allgemeinem Wissen, nicht aus dem Scrape verifiziert):** STAC-API unter `planetarycomputer.microsoft.com/api/stac/v1`

### Google Earth Engine
- **URL:** https://developers.google.com/earth-engine
- **Zweck:** Petabyte-Datenkatalog + planetare Rechen-Infrastruktur für Geodaten
- **Zugangswege:** JavaScript Code Editor, Python-Bibliothek (open source), REST API
- **Doku:** https://developers.google.com/earth-engine/guides
- **Hinweis:** Non-commerciale Quota-Tiers eingeführt (Community-Tier als Standard)

### STAC Index
- **URL:** https://stacindex.org/
- **Zweck:** Zentrales Verzeichnis aller bekannten STAC-APIs & statischen Kataloge
- **Bereiche:** Catalogs · Ecosystem (Tools/Software) · Learning Resources
- **Bekannte Einträge:** CDSE-STAC, Microsoft Planetary Computer, Google Earth Engine STAC, earth-search.aws.element84.com, WorldPop STAC API u. v. m.
- **Maschinenlesbare Liste:** https://github.com/opengeos/stac-index-catalogs (täglich aktualisiertes CSV/JSON)

---

## 🌍 ESA MAAP-Ökosystem (3 zusammengehörige Seiten)

### ESA MAAP Portal
- **URL:** https://portal.maap.eo.esa.int/
- **Zweck:** Daten-Zugriff & -Discovery für ESA Earth Explorer / Heritage / Third-Party-Missionen
- **Komponenten:** Catalogue (Metadaten-API), Explorer (WMS/WMTS-Visualisierung), Product Algorithm Laboratory (PAL) für on-demand Processing
- **STAC-API-Doku:** https://catalog.maap.eo.esa.int/doc/stac.html
- **Data-Access-Beispiele:** https://catalog.maap.eo.esa.int/doc/data-access.html
- **Auth-Token:** https://portal.maap.eo.esa.int/ini/services/auth/token/

### ESA MAAP – Earth Online Übersicht
- **URL:** https://earth.esa.int/eogateway/tools/esa-maap
- **Zweck:** Erklärseite zum MAAP-Konzept, verlinkt auf Portal + Biomass-/EarthCARE-Collaborative-Environments
- **Besonderheit:** Streaming-Zugriff & On-the-fly-Subsetting ohne Voll-Download

### Multi-Mission Algorithm and Analysis Platform (SciMAAP)
- **URL:** https://scimaap.net/
- **Zweck:** Gemeinsames **ESA-NASA**-Projekt für Carbon-Dynamics-Forschung (BIOMASS, GEDI, NISAR)
- **Zugänge:** [NASA-Portal](https://maap-project.org/) · [ESA-Portal](https://portal.maap.eo.esa.int/)

> ⚠️ `earthdata.nasa.gov/about/maap` konnte wegen Bot-Schutz nicht abgerufen werden.

---

## 🪐 Astronomie & Planetendaten

### Solar System OpenData
- **URL:** https://api.le-systeme-solaire.net/en/
- **Zweck:** REST-API mit allen Daten zu Körpern im Sonnensystem (Planeten, Monde, Zwergplaneten, Asteroiden)
- **Endpunkte:**
  - `GET /rest/bodies/` – alle Körper
  - `GET /rest/bodies/{id}` – ein Körper
  - `GET /rest/knowncount/` – bekannte Objektzahlen je Typ
  - `GET /rest/positions` – Himmelsposition (RA/Dec/Az/Alt) für Beobachterstandort
- **Parameter:** `data`, `exclude`, `order`, `page`, `filter[]` (Operatoren: `eq`, `lt`, `gt`, `bt`, `cs` …)
- **Auth:** **Pflicht-Bearer-Token** (kostenlos, seit 09/2025) – `Authorization: Bearer <UUID>`
- **Swagger UI:** https://api.le-systeme-solaire.net/swagger/

### NASA Exoplanet Archive
- **URL:** https://exoplanetarchive.ipac.caltech.edu/
- **Zweck:** Tabellen zu bestätigten Exoplaneten, Kepler/K2/TESS-Kandidaten, Sternendaten
- **Zugriffswege:**
  - **TAP-Interface:** https://exoplanetarchive.ipac.caltech.edu/docs/TAP/usingTAP.html
  - **API:** https://exoplanetarchive.ipac.caltech.edu/docs/program_interfaces.html
  - Bulk Data Download, ExoFOP
- **Neu (Juni 2026):** API für den Transit- & Ephemeris-Service (automatisierte Beobachtungsplanung)
- **Stand:** 6.298 bestätigte Planeten (Stand 06/2026)

### JPL SSD/CNEOS API Service
- **URL:** https://ssd-api.jpl.nasa.gov/
- **Zweck:** Sammlung einzelner JSON-REST-APIs zu Asteroiden/Kometen/Himmelsmechanik
- **Verfügbare APIs:** Fireball, Horizons, Horizons File, Horizons Lookup, JD Date/Time Converter, NHATS, Periodic Orbits, **SB Close Approach (CAD)**, SB Identification, SB Mission Design, SB Observability, SB Radar, SB Satellites, **SBDB** (Small-Body DataBase), SBDB Query, Scout, **Sentry** (Impact-Risiko)
- **Fair-Use-Policy:** nur 1 Request gleichzeitig, kein Embedding in Websites, kein SLA
- **Methoden:** GET & POST, reine Datenabfrage (kein Update/Delete)

### MAST API (Mikulski Archive for Space Telescopes)
- **URL:** https://mast.stsci.edu/api/v0/
- **Zweck:** Programmatischer Zugriff auf den MAST-Datenportal-Katalog
- **Funktionsweise:** `?request=(MashupRequest JSON-Objekt)`, GET oder POST
- **Limit:** ~500.000 Records/Query (sonst Memory-Error)
- **Long Polling** für asynchrone Jobs
- **Tutorial:** Beispiel-Workflow von Query bis Download vorhanden

### ESASky
- **URL:** https://sky.esa.int/esasky/
- **Hinweis:** Reine JavaScript-Anwendung, kein Text per Fetch extrahierbar
- **Bekannt:** eigene REST-API zur programmatischen Abfrage von Missionsdaten existiert separat in der ESASky-Doku

### SDSS Data Release 19
- **URL:** https://www.sdss.org/dr19/
- **Zweck:** Aktueller Datenrelease der Sloan Digital Sky Survey (SDSS-V)
- **Neue Tools:** „Zora“ und „Valis“ – Web-UIs **und APIs** zum Interagieren mit Spektren/Datenprodukten
- **Data Access:** https://www.sdss.org/dr19/data_access/
- **Hinweis:** Keine klassische REST-API-Referenz direkt auf dieser Seite

### Illustris API
- **URL:** https://www.illustris-project.org/data/docs/api/
- **Zweck:** Sehr ausführlich dokumentierte REST-API für kosmologische Simulationsdaten (Illustris/IllustrisTNG)
- **Auth:** `api-key` als Query-Parameter (`?api_key=`) oder HTTP-Header (`api-key: ...`)
- **Wichtigste Endpunkte:**
  - `GET /api/` – Liste aller Simulationen
  - `GET /api/{sim}/snapshots/{num}/subhalos/?{search_query}` – Subhalo-Suche (Operatoren `__gt`, `__lt`, `__lte` …)
  - `GET /api/{sim}/snapshots/{num}/subhalos/{id}/cutout.hdf5?{cutout_query}` – HDF5-Datenauszug
  - `GET /api/{sim}/snapshots/{num}/subhalos/{id}/sublink/mpb.hdf5` – Merger-Trees (SubLink/LHaloTree)
  - `GET /api/{sim}/snapshots/{num}/subhalos/{id}/stellar_mocks/*` – synthetische Bilder/SEDs
- **Response-Codes:** 200, 302 (Redirect bei Downloads), 400, 401, 403, 404, 500
- **Clients:** Beispielcode für Python, IDL, MATLAB

---

## 🚀 Raumfahrt & Sonstiges

### SpaceX-API (r-spacex)
- **URL:** https://github.com/r-spacex/SpaceX-API
- ⚠️ **Repo wurde am 6. Juni 2026 archiviert (read-only)!**
- **Base-URL:** `https://api.spacexdata.com/v5/...`
- **Daten:** Launches, Rockets, Cores, Capsules, Starlink, Launchpads, Landingpads
- **Beispiel:** `GET /v5/launches/latest`
- **Lizenz:** Apache-2.0 · Nicht offiziell mit SpaceX verbunden

### Amentum
- **URL:** https://www.amentum.io/
- **Zweck:** Kommerzielle Umwelt-Intelligence-APIs
- **APIs:** Ocean, Aviation Radiation (AvRad), Atmosphere, Geomagnetic (GlobalMagnet), Gravity
- **Auch verfügbar als:** MCP-Server für AI-Agenten, CLI
- **Trial:** https://developer.amentum.io/register

### NASA Open APIs
- **URL:** https://api.nasa.gov/
- **Hinweis:** Seite ist JS-gerendert, zeigt nur das Key-Signup-Formular (api.data.gov)
- **Bekannte Kern-APIs** (allgemeines Wissen, nicht aus dem Scrape verifiziert): APOD, Mars Rover Photos, NeoWs, DONKI, EPIC, EONET, Exoplanet-API u. a.

---

## Zusammenfassende Hinweise

- Mehrere Seiten sind **vollständig JavaScript-gerendert** und liefern beim einfachen Fetch keinen Inhalt: Microsoft Planetary Computer, STAC Index (Startseite), JPL SSD-API (Terms-Gate), NASA api.nasa.gov, ESASky.
- **earthdata.nasa.gov/about/maap** war wegen Bot-Schutz nicht abrufbar.
- Drei der angefragten URLs gehören zum **gleichen ESA-MAAP-Ökosystem** (Portal, Earth-Online-Übersicht, SciMAAP) und überschneiden sich inhaltlich stark.
- Das **SpaceX-API-Repo ist seit Juni 2026 archiviert** – für produktive Nutzung ggf. nach Forks/Alternativen suchen.


http://api.brain-map.org/api/v2/data/query.json?criteria=model::SectionDataSet,rma::criteria,[failed
http://api.geonames.org/findNearbyPlaceNameJSON?lat={lat}&lng={lon}&username={geonames_user}
http://api.nasa.gov/neo/rest/v1/feed?start\_date=2025-06-22&end\_date=2025-06-22&detailed=false&api\_key=REDACTED
http://api.nasa.gov/neo/rest/v1/feed?start\_date=2025-06-23&end\_date=2025-06-23&detailed=false&api\_key=REDACTED
http://api.nasa.gov/neo/rest/v1/feed?start\_date=2025-06-24&end\_date=2025-06-24&detailed=false&api\_key=REDACTED
http://api.nasa.gov/neo/rest/v1/neo/3408650?api\_key=REDACTED
http://api.nasa.gov/neo/rest/v1/neo/3659801?api\_key=REDACTED
http://api.nasa.gov/neo/rest/v1/neo/3825168?api\_key=REDACTED
http://api.open-notify.org/astros.json
http://api.open-notify.org/howmany
http://api.open-notify.org/iss-pass.json
http://api.open-notify.org/iss-pass.json?lat={lat}&lon={lon}
http://api.open-notify.org/iss-pass.json?lat={lat}&lon={lon}&n=5
http://api.open-notify.org/iss-pass.json?lat=...&lon=...
http://api.open-notify.org/iss-pass?lat={lat}&lon={lon}
http://apiv3.iucnredlist.org/api/v3/getspeciesbylocation/{lat_min}/{lat_max}/{lon_min}/{lon_max}
http://cosmicrays.oulu.fi/phi/Pulu_1min.txt
http://cr0.izmiran.ru/mosc/main.txt
http://data.neonscience.org/api/v0/documents/NEON.DP1.10081\_20086\_20141.001.variables.20180306T000000Z.csv
http://export.arxiv.org/api/query?search_query=cat:cs.AI&...
http://export.arxiv.org/api/query?search_query=cat:cs.AI&start=0&max_results=1&sortBy=submittedDate&sortOrder=descending
http://fenixservices.fao.org/faostat/api/v1/en/data/QCL?area={country_code}
http://noosphere.princeton.edu/basket/gcp_data.txt
http://opendata.cern.ch/api/records/?type=dataset&experiment=ALICE&size=1
http://opendata.cern.ch/api/records/?type=dataset&experiment=ATLAS&size=1
http://shibe.online/
http://simbad.u-strasbg.fr/simbad/
http://sosrff.tsu.ru/?page_id=7
http://waterservices.usgs.gov/nwis/iv/format=json&sites=06730200¶meterCd=00060,00065,00010&siteStatus=active
http://worldtimeapi.org/api/timezone/Etc/UTC
http://worldtimeapi.org/api/timezone/UTC
http://www.blitzortung.org/Websocket/#1
http://www.dxcluster.info/
http://www.nmdb.eu/nest/draw_graph.php?formchk=1&stations[]={nearest_neutron_station}&tabchoice=revori&dtype=corr_for_efficiency&tresolution=60&force=1&yunits=0&date_choice=1&start_day={today}&start_hour=00&start_min=00&end_day={today}&end_hour=23&end_min=59&output=ascii
http://www.nmdb.eu/nest/dynamics.php?stations={nearest_neutron_station}&output=json&resolution=60&history=300
http://www.nmdb.eu/nmdb/qtipart.php?station=JUNG&start={yesterday}&end={today}&format=json&bins=3600

https://aeronet.gsfc.nasa.gov/cgi-bin/print_web_data_v3_new?site={nearest_site}...
https://aeronet.gsfc.nasa.gov/cgi-bin/print_web_data_v3_new?site={nearest_site}&year={year}&month={month}&day={day}&hour={hour}&hour2={hour}&AOD15=1&AVG=10&if_no_html=1
https://air-quality-api.open-meteo.com/v1/air-quality?latitude={lat}&longitude={lon}&current=pm10,pm2_5,carbon_monoxide,nitrogen_dioxide,sulphur_dioxide,ozone,dust,uv_index,uv_index_clear_sky,aerosol_optical_depth,ammonia
https://air-quality-api.open-meteo.com/v1/air-quality?latitude={lat}&longitude={lon}&current=pm10,pm2_5,european_aqi,carbon_monoxide,nitrogen_dioxide,sulphur_dioxide,ozone
https://air-quality-api.open-meteo.com/v1/air-quality?latitude={lat}&longitude={lon}&current=pm2_5,pm10,no2,so2,o3,co
https://api.acleddata.com/acled/read?event_date=2025-06-23&event_date_where=BETWEEN&limit=0
https://api.acleddata.com/acled/read?event_date=2025-06-23&event_date_where=BETWEEN&limit=1
https://api.acleddata.com/acled/read?event_date={today}&event_date_where=BETWEEN&limit=0
https://api.acleddata.com/acled/read?event_date={today}&event_date_where=>=&limit=0&key=REDACTED&email={acled_email}
https://api.acleddata.com/acled/read?...&key=REDACTED&email={acled_email}
https://api.acleddata.com/acled/read?limit=1&email=test&key=REDACTED
https://api.acleddata.com/acled/read?terms=accept&limit=1
https://api.adsabs.harvard.edu/v1/search/query?q=exoplanet+atmosphere&rows=1&sort=date+desc
https://api.blitzortung.org/v1/strikes/latest
https://api.blitzortung.org/v1/strikes/latest?lat={lat}&lon={lon}&radius=100
https://api.carbonintensity.org.uk/generation
https://api.census.gov/data/2019/pep/population?get=POP&for=zip+code+tabulation+area:{zip}
https://api.cloudflare.com/client/v4/radar/http/summary/protocol?dateRange=1h
https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum,solana&vs_currencies=usd
https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd&include_24hr_change=true
https://api.crossref.org/works?query.title=artificial+intelligence&rows=1
https://api.drand.sh/public/latest
https://api.ebeard.org/v2/data/obs/geo/recent?lat={lat}&lng={lon}&key=REDACTED
https://api.ebird.org/v2/data/obs/geo/recent?lat=40.0&lng=-105.0&key=REDACTED
https://api.ebird.org/v2/data/obs/geo/recent?lat={lat}&lng={lon}&key=REDACTED
https://api.ebird.org/v2/ref/hotspot/geo?lat={lat}&lng={lon}&dist=25&fmt=json
https://api.eia.gov/series/...
https://api.eia.gov/series/?api_key=REDACTED&series_id=EBA.US-ALL.D.H
https://api.eia.gov/v2/electricity/rto/region-data/data/?api_key=REDACTED
https://api.eia.gov/v2/electricity/rto/region-data/data/?frequency=hourly&data[0]=value&facets[respondent][]=US48&sort[0][column]=period&sort[0][direction]=desc&offset=0&length=1&api_key=REDACTED
https://api.electricitymap.org/v3/carbon-intensity/latest?lat={lat}&lon={lon}
https://api.energy-charts.info/price?country=DE
https://api.europeana.eu/record/v2/search.json?query=*&qf=place:{lat},{lon}&wskey=REDACTED
https://api.exchangerate-api.com/v4/latest/USD
https://api.fda.gov/drug/event.json?search=patient.drug.medicinalproduct:{drug_name}&limit=1
https://api.fda.gov/drug/event.json?search=receivedate:[{yesterday}+TO+{today}]&limit=1
https://api.fda.gov/drug/label.json?search=openfda.brand_name:{drug_name}&limit=1
https://api.gbif.org/v1/occurrence/search?decimalLatitude={lat}&decimalLongitude={lon}&radius=1&limit=0
https://api.gbif.org/v1/occurrence/search?taxon_key=REDACTED&decimalLatitude={lat_min},{lat_max}&decimalLongitude={lon_min},{lon_max}&limit=0
https://api.gbif.org/v1/occurrence/search?taxon_key=REDACTED&decimalLatitude={lat_min},{lat_max}&decimalLongitude={lon_min},{lon_max}&limit=1
https://api.github.com/search/repositories?q=language:python+ai&sort=updated&per_page=1
https://api.grid.iamkate.com/frequency/live
https://api.inaturalist.org/v1/observations
https://api.inaturalist.org/v1/observations?lat=40.0&lng=-105.0&radius=1&per_page=1
https://api.inaturalist.org/v1/observations?lat=52.5&lng=13.4&radius=1&quality_grade=research&per_page=1
https://api.inaturalist.org/v1/observations?lat={lat}&lines={lon}&radius=1&per_page=0
https://api.inaturalist.org/v1/observations?lat={lat}&lng={lon}&radius=1&per_page=0
https://api.inaturalist.org/v1/observations?lat={lat}&lng={lon}&radius=1&quality_grade=research&per_page=0
https://api.inaturalist.org/v1/observations?lat={lat}&lng={lon}&radius=1&quality_grade=research&per_page=1
https://api.inaturalist.org/v1/observations?lat=...&lng=...&radius=1
https://api.inaturalist.org/v1/observations?lat=X&lng=Y&radius=1
https://api.inaturalist.org/v1/observations?taxon_id=47170&lat={lat}&lng={lon}&radius=10&quality_grade=research
https://api.inaturalist.org/v1/observations?taxon_id=47170&lat={lat}&lng={lon}&radius=10&quality_grade=research&per_page=0
https://api.ioda.caida.org/v2/outages/asn/...
https://api.ioda.caida.org/v2/outages/asn/?from=-300&to=now
https://api.ioda.ioda.caida.org/v2/signals/raw?from={timestamp_5min_ago}&until=now&datasource=ping-slash24&entityType=country&entityCode={country_code}
https://api.ipify.org
https://api.irail.be/
https://api.irail.be/occupancy/
https://api.materialsproject.org/materials/{formula}/thermo
https://api.materialsproject.org/materials/summary?_limit=1&api_key=REDACTED
https://api.nasa.gov/DONKI/CME...
https://api.nasa.gov/DONKI/CME?startDate=2025-06-22&api_key=REDACTED
https://api.nasa.gov/DONKI/CME?startDate={today}&api_key=REDACTED
https://api.nasa.gov/DONKI/CME?startDate={yesterday}&api_key=REDACTED
https://api.nasa.gov/DONKI/FLR...
https://api.nasa.gov/DONKI/FLR?startDate=2025-06-22&api_key=REDACTED
https://api.nasa.gov/DONKI/FLR?startDate=...&api_key=REDACTED
https://api.nasa.gov/DONKI/FLR?startDate={today}&api_key=REDACTED
https://api.nasa.gov/DONKI/FLR?startDate={yesterday}&api_key=REDACTED
https://api.nasa.gov/DONKI/GST...
https://api.nasa.gov/DONKI/GST?startDate={yesterday}&api_key=REDACTED
https://api.nasa.gov/DONKI/HSS?startDate={yesterday}&api_key=REDACTED
https://api.nasa.gov/DONKI/IPS?startDate={yesterday}&api_key=REDACTED
https://api.nasa.gov/DONKI/MPC?startDate={yesterday}&api_key=REDACTED
https://api.nasa.gov/DONKI/RBE?startDate={yesterday}&api_key=REDACTED
https://api.nasa.gov/DONKI/SEP?startDate={yesterday}&api_key=REDACTED
https://api.nasa.gov/DONKI/WSAEnlil?...
https://api.nasa.gov/insight_weather/...
https://api.nasa.gov/insight_weather/?api_key=REDACTED&feedtype=json&ver=1.0
https://api.nasa.gov/mars-photos/api/v1/rovers/perseverance/latest_photos?api_key=REDACTED
https://api.nasa.gov/neo/rest/v1/feed...
https://api.nasa.gov/neo/rest/v1/feed?...
https://api.nasa.gov/neo/rest/v1/feed?start_date=...&end_date=...&api_key=REDACTED
https://api.nasa.gov/neo/rest/v1/feed?start_date={today}&api_key=REDACTED
https://api.nasa.gov/neo/rest/v1/feed?start_date={today}&end_date={today}&api_key=REDACTED
https://api.nasa.gov/neo/rest/v1/neo/browse?api_key=REDACTED
https://api.obis.org/occurrence?lat={lat}&lon={lon}&radius=50&size=0
https://api.openaq.org/v2/latest?coordinates={lat},{lon}&radius=1000&limit=1
https://api.openaq.org/v2/latest?coordinates=X,Y&radius=1000
https://api.openaq.org/v3/locations?coordinates=40.0,-105.0&radius=1000&limit=1
https://api.openaq.org/v3/locations?coordinates={lat},{lon}&radius=1000&limit=1
https://api.openaq.org/v3/locations?coordinates={lat},{lon}&radius=1000&limit=1&api_key=REDACTED
https://api.openaq.org/v3/locations/latest?coordinates={lat},{lon}&radius=1000&limit=1
https://api.openaq.org/v3/measurements?coordinates={lat},{lon}&radius=1000&limit=1
https://api.open-meteo.com/...
https://api.open-meteo.com/v1/elevation
https://api.open-meteo.com/v1/elevation?...
https://api.open-meteo.com/v1/elevation?latitude=52.5&longitude=13.4
https://api.open-meteo.com/v1/elevation?latitude={lat}&longitude={lon}
https://api.open-meteo.com/v1/elevation?latitude=X&longitude=Y
https://api.open-meteo.com/v1/forecast?...
https://api.open-meteo.com/v1/forecast?...&current=soil_temperature_0_to_10cm,soil_moisture_0_to_10cm
https://api.open-meteo.com/v1/forecast?latitude=...&longitude=...&current=temperature_2m,pressure,surface_pressure,wind_speed_10m
https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&current=snow_depth&timezone=auto
https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&current=temperature_2m,relative_humidity_2m,apparent_temperature,is_day,precipitation,rain,showers,snowfall,weather_code,cloud_cover,pressure_msl,surface_pressure,wind_speed_10m,wind_direction_10m,wind_gusts_10m&timezone=auto
https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&current=temperature_2m,wind_speed_10m,surface_pressure,relative_humidity_2m,precipitation&hourly=cloud_cover
https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&current=temperature_2m,wind_speed_10m,surface_pressure,relative_humidity_2m,precipitation,uv_index,is_day&hourly=cloud_cover,visibility
https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&hourly=soil_temperature_0cm,soil_moisture_0_to_1cm,et0_fao_evapotranspiration,vapor_pressure_deficit
https://api.protectedplanet.net/v3/protected_areas?with_geometry=true&latitude={lat}&longitude={lon}&token=REDACTED
https://api.purpleair.com/v1/sensors?fields=pm2.5_atm,pm10.0_atm,humidity,temperature,pressure&location_type=0&nwlng={lon_min}&nwlat={lat_max}&selng={lon_max}&selat={lat_min}
https://api.resourcewatch.org/v1/geostore?lat={lat
https://api.resourcewatch.org/v1/geostore?lat={lat}&lon={lon}
https://api.si.edu/openaccess/api/v1.0/content/search?q={topic}&api_key=REDACTED
https://api.stlouisfed.org/fred/series/observations?series_id={fred_series_id}&api_key=REDACTED&file_type=json&limit=1&sort_order=desc
https://api.stlouisfed.org/fred/series/observations?series_id=GDP&api_key=REDACTED&file_type=json&limit=1&sort_order=desc
https://api.tidesandcurrents.noaa.gov/api/prod/
https://api.tidesandcurrents.noaa.gov/api/prod/...
https://api.tidesandcurrents.noaa.gov/api/prod/datagetter?station={nearest_station}&product=currents&time_zone=gmt&units=metric&format=json&date=latest
https://api.tidesandcurrents.noaa.gov/api/prod/datagetter?station={nearest_station}&product=predictions&datum=MLLW&time_zone=gmt&units=metric&format=json&begin_date={today}&end_date={tomorrow}&interval=hilo
https://api.tidesandcurrents.noaa.gov/api/prod/datagetter?station={nearest_station}&product=predictions&datum=MLLW&time_zone=gmt&units=metric&format=json&date=latest
https://api.tidesandcurrents.noaa.gov/api/prod/datagetter?station={nearest_station}&product=water_level&datum=MLLW&time_zone=gmt&units=metric&format=json&date=latest
https://api.tidesandcurrents.noaa.gov/api/prod/datagetter?station={}&product=water_level&datum=MLLW&time_zone=gmt&units=metric&format=json&date=latest
https://api.tidesandcurrents.noaa.gov/api/prod/datagetter?station={station_id}&product=water_level&datum=MLLW&time_zone=gmt&units=metric&format=json&date=latest
https://api.tidesandcurrents.noaa.gov/mdapi/prod/webapi/stations.json
https://api.tidesandcurrents.noaa.gov/mdapi/prod/webapi/stations.json?type=waterlevels
https://api.weather.gov
https://api.weather.gov/alerts/active?point={lat},{lon}
https://api.wheretheiss.at/v1/satellites/20580
https://api.wheretheiss.at/v1/satellites/25544
https://api.wheretheiss.at/v1/satellites/48274
https://api.worldbank.org/v2/country/all/indicator/SP.POP.TOTL?format=json&date=latest&per_page=1
https://api.worldbank.org/v2/country/{country_code}/indicator/NY.GDP.MKTP.CD?format=json
https://api.worldbank.org/v2/country/{country_code}/indicator/SP.POP.TOTL?format=json
https://api.woudc.org/collections/totalozone/items...
https://api.woudc.org/collections/totalozone/items?f=json
https://api.woudc.org/collections/totalozone/items?f=json&limit=1
https://api.z.ai/api/coding/paas/v4
https://api.z.ai/api/paas/v4
https://aqs.epa.gov/data/api/dailyData/byBox?email={epa_email}&key=REDACTED&param=88101&bdate={today}&edate={today}&minlat={lat_min}&maxlat={lat_max}&minlon={lon_min}&maxlon={lon_max}
https://aqs.epa.gov/data/api/signup
https://archive.opensearch.ceda.ac.uk/opensearch/description.xml
https://argovis-api.colorado.edu/
https://argovis-api.colorado.edu/argo?date_range={}T00:00:00Z,{}T23:59:59Z&box=[-180,-90,180,90]
https://argovis-api.colorado.edu/argo?date_range={today}...
https://argovis-api.colorado.edu/argo?date_range={today}T00:00:00Z,{today}T23:59:59Z&box=[{lon_min},{lat_min},{lon_max},{lat_max}]
https://argovis-api.colorado.edu/profile?startDate={today}&lat_range={lat_min},{lat_max}&lon_range={lon_min},{lon_max}&data=temperature,salinity,pressure
https://argovis.colorado.edu/selection/api/profiles...
https://argovis.colorado.edu/selection/api/profiles?startDate={today}&lat_range={lat_min},{lat_max}&lon_range={lon_min},{lon_max}
https://atlas.ripe.net/api/v2/probes/
https://beacon.nist.gov/beacon/2.0/api/beacon/1.0/record/last
https://celestrak.com/NORAD/elements/gp.php?GROUP=...
https://celestrak.com/NORAD/elements/gp.php?GROUP=active&FORMAT=json
https://celestrak.com/NORAD/elements/gp.php?GROUP=gps-ops&FORMAT=json
https://celestrak.com/NORAD/elements/gp.php?GROUP=oneweb&FORMAT=json
https://celestrak.com/NORAD/elements/gp.php?GROUP=starlink&FORMAT=json
https://celestrak.org/NORAD/elements/gp.php?GROUP=active&FORMAT=json
https://celestrak.org/NORAD/elements/gp.php?GROUP=gps-ops&FORMAT=json
https://celestrak.org/NORAD/elements/gp.php?GROUP=iridium-33-debris&FORMAT=json
https://celestrak.org/NORAD/elements/gp.php?GROUP=spysat&FORMAT=json
https://celestrak.org/NORAD/elements/gp.php?GROUP=starlink&FORMAT=json
https://celestrak.org/NORAD/elements/gp.php?GROUP=weather&FORMAT=json
https://clinicaltrials.gov/api/v2/studies?query.cond={condition}&countTotal=true
https://cmr.earthdata.nasa.gov/search/
https://coastwatch.pfeg.noaa.gov/erddap/griddap/erdMH1chla8day.json?chlorophyll[(last
https://coastwatch.pfeg.noaa.gov/erddap/griddap/erdMH1chlamday.json?chlorophyll[(last
https://coastwatch.pfeg.noaa.gov/erddap/griddap/jplMURSST41.json?analysed_sst[(last
https://coastwatch.pfeg.noaa.gov/erddap/griddap/socat_v2022_clim.json?fco2_ave_unwtd[(last
https://coastwatch.pfeg.noaa.gov/erddap/tabledap/nsidcG02135South.csv?extent&time>=2024-01-01&orderByMax(%22time%22
https://coastwatch.pfeg.noaa.gov/erddap/tabledap/nsidcG02135South.csv?extent&time>=2024-01-01T00:00:00Z&time<=2024-01-02T00:00:00Z&orderByMax(%22time%22
https://coastwatch.pfeg.noaa.gov/erddap/tabledap/nsidcG02135South.csv?extent&time%3E={yesterday}&orderByMax(%22time%22
https://data-api.globalforestwatch.org/
https://data-api.globalforestwatch.org/...
https://data-api.globalforestwatch.org/dataset/umd_tree_cover_loss/latest/query?sql=SELECT+COUNT(*
https://data-api.globalforestwatch.org/ds/glad-tiles/gfw-tiles/glad-alerts/latest/json?geostore={geostore_id}
https://data-api.globalforestwatch.org/geostore
https://data.blitzortung.org/...
https://data.blitzortung.org/Data/Protected/strikes.json
https://data.blitzortung.org/strikes/{year}/{month}/{day}/...
https://data.neonscience.org/api/v0/data/DP1.10081.001/ABBY/2016-10
https://data.neonscience.org/api/v0/data/DP1.10081.001/ABBY/2017-05
https://data.neonscience.org/api/v0/data/DP1.10081.001/ABBY/2017-06
https://data.neonscience.org/api/v0/data/DP1.10081.001/ABBY/2017-10
https://data.neonscience.org/api/v0/data/DP1.10081.001/BARR/2017-08
https://data.neonscience.org/api/v0/data/DP1.10081.001/BART/2014-06
https://data.neonscience.org/api/v0/data/DP1.10081.001/BART/2014-07
https://data.neonscience.org/api/v0/data/DP1.10081.001/BART/2016-04
https://data.neonscience.org/api/v0/data/DP1.10081.001/BART/2016-07
https://data.neonscience.org/api/v0/data/DP1.10081.001/BART/2016-11
https://data.neonscience.org/api/v0/data/DP1.10081.001/BART/2017-05
https://data.neonscience.org/api/v0/data/DP1.10081.001/BART/2017-06
https://data.neonscience.org/api/v0/data/DP1.10081.001/BART/2017-10
https://data.neonscience.org/api/v0/data/DP1.10081.001/BLAN/2015-11
https://data.neonscience.org/api/v0/data/DP1.10081.001/BLAN/2016-03
https://data.neonscience.org/api/v0/data/DP1.10081.001/BLAN/2016-08
https://data.neonscience.org/api/v0/data/DP1.10081.001/BLAN/2016-11
https://data.neonscience.org/api/v0/data/DP1.10081.001/BLAN/2017-03
https://data.neonscience.org/api/v0/data/DP1.10081.001/BLAN/2017-07
https://data.neonscience.org/api/v0/data/DP1.10081.001/BLAN/2017-10
https://data.neonscience.org/api/v0/data/DP1.10081.001/BONA/2018-09
https://data.neonscience.org/api/v0/data/DP1.10081.001/CLBJ/2016-06
https://data.neonscience.org/api/v0/data/DP1.10081.001/CLBJ/2016-08
https://data.neonscience.org/api/v0/data/DP1.10081.001/CLBJ/2016-1
https://data.neonscience.org/api/v0/products/DP1.10081.001
https://data.neonscience.org/api/v0/releases/RELEASE-2021
https://data.neonscience.org/api/v0/releases/RELEASE-2022
https://data.neonscience.org/api/v0/releases/RELEASE-2023
https://data.neonscience.org/api/v0/releases/RELEASE-2024
https://data.neonscience.org/api/v0/releases/RELEASE-2025
https://data.neonscience.org/api/v0/releases/RELEASE-2026
https://dataverse.harvard.edu/api/search?q=UN+General+Assembly+votes&type=dataset
https://de.wikipedia.org/w/api.php?action=query&list=geosearch&gscoord=X|Y&gsradius=10000
https://defend.network/api/
https://dods.wh.gov/opendap/jg/acoustic/aggregated/acoustic.nc.json
https://earthquake.usgs.gov/fdsnws/event/1/
https://earthquake.usgs.gov/fdsnws/event/1/query
https://earthquake.usgs.gov/fdsnws/event/1/query?eventid=us7000sr28&format=geojson
https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson...
https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&latitude=40.0&longitude=-105.0&maxradius=2&minmagnitude=2.0&limit=5
https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&latitude={lat}&longitude={lon}&maxradius=2&minmagnitude=2.0&limit=5
https://earthquake.usgs.gov/fdsnws/event/1/query?format=geojson&starttime={hour_ago}&latitude={lat}&longitude={lon}&maxradiuskm=500&minmagnitude=2.0
https://earthquake.usgs.gov/fdsnws/event/1/query?format=json&starttime={hour_ago}&minmagnitude=2.0&eventtype=underwater
https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/prc_hicp_manr?lastTimePeriod=1
https://en.wikipedia.org/w/api.php?action=query&list=geosearch...
https://en.wikipedia.org/w/api.php?action=query&list=geosearch&gscoord={lat}|{lon}&gsradius=10000&format=json
https://en.wikipedia.org/w/api.php?action=query&list=geosearch&gscoord={lat}|{lon}&gsradius=10000&gslimit=500&format=json
https://en.wikipedia.org/w/api.php?action=query&list=geosearch&gscoord=X|Y&gsradius=10000
https://en.wiktionary.org/w/api.php?action=query&titles={word}&prop=extracts&format=json
https://eonet.gsfc.nasa.gov/api/v3/events
https://eonet.gsfc.nasa.gov/api/v3/events...
https://eonet.gsfc.nasa.gov/api/v3/events/EONET\_20558
https://eonet.gsfc.nasa.gov/api/v3/events/EONET\_20560
https://eonet.gsfc.nasa.gov/api/v3/events/EONET\_20562
https://eonet.gsfc.nasa.gov/api/v3/events/EONET\_20565
https://eonet.gsfc.nasa.gov/api/v3/events/EONET\_20606
https://eonet.gsfc.nasa.gov/api/v3/events?status=open&bbox={lon_min},{lat_min},{lon_max},{lat_max}
https://eonet.gsfc.nasa.gov/api/v3/events?status=open&limit=100
https://eonet.gsfc.nasa.gov/api/v3/events?status=open&limit=100&bbox={lon_min},{lat_min},{lon_max},{lat_max}
https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=nuccore&term={species_name}&retmode=json
https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esummary.fcgi?db=assembly&id=GCF_000001405.40&retmode=json
https://exoplanetarchive.ipac.caltech.edu/TAP/sync...
https://exoplanetarchive.ipac.caltech.edu/TAP/sync?query=...
https://exoplanetarchive.ipac.caltech.edu/TAP/sync?query=SELECT+TOP+10+pl_name,pl_orbper,pl_rade,pl_eqt+FROM+pscomppars+WHERE+pl_eqt+IS+NOT+NULL&format=json
https://exoplanetarchive.ipac.caltech.edu/TAP/sync?query=SELECT+TOP+1+*+FROM+ps+WHERE+pl_atm=1&format=json
https://exoplanetarchive.ipac.caltech.edu/TAP/sync?query=SELECT+TOP+20+...
https://exoplanetarchive.ipac.caltech.edu/TAP/sync?query=SELECT+TOP+20+pl_name,pl_orbper,pl_rade,pl_eqt+FROM+pscomppars+WHERE+pl_eqt+IS+NOT+NULL+ORDER+BY+pl_eqt+ASC&format=json
https://exoplanetarchive.ipac.caltech.edu/TAP/sync?query=SELECT+TOP+5+pl_name,pl_atm+FROM+pscomppars+WHERE+pl_atm+IS+NOT+NULL&format=json
https://firms.modaps.eosdis.nasa.gov/api/area/
https://firms.modaps.eosdis.nasa.gov/api/area/csv/...
https://firms.modaps.eosdis.nasa.gov/api/area/csv/{api_key=REDACTED
https://firms.modaps.eosdis.nasa.gov/api/area/csv/{firms_map_key=REDACTED
https://firms.modaps.eosdis.nasa.gov/api/area/csv/{nasa_key=REDACTED
https://firms.modaps.eosdis.nasa.gov/api/country/csv/YOUR_MAP_KEY=REDACTED
https://firms.modaps.eosdis.nasa.gov/api/map_key=REDACTED
https://fireballs.ndc.nasa.gov/api/meteors/last24h
https://gcn.nasa.gov/circulars?format=json&limit=10
https://gcn.nasa.gov/circulars.json?limit=10
https://gcn.nasa.gov/circulars.json?limit=10&subject=IceCube
https://gcn.nasa.gov/circulars.json?limit=5&subject=AMON
https://gcn.nasa.gov/circulars.json?limit=5&subject=Auger
https://gcn.nasa.gov/circulars.json?limit=5&subject=Fermi
https://gcn.nasa.gov/circulars.json?limit=5&subject=GW
https://gcn.nasa.gov/circulars.json?limit=5&subject=HAWC
https://gcn.nasa.gov/circulars.json?limit=5&subject=Super-Kamiokande
https://gcn.nasa.gov/circulars.json?limit=5&subject=TAROT
https://gcn.nasa.gov/circulars.json?limit=5&subject=XENON
https://gcn.nasa.gov/circulars.json?limit={}&subject={}
https://gea.esac.esa.int/archive/tap/sync...
https://gea.esac.esa.int/archive/tap/sync?query=SELECT+TOP+50+...
https://gea.esac.esa.int/archive/tap/sync?query=SELECT+TOP+50+source_id,ra,dec,phot_g_mean_mag,parallax+FROM+gaiadr3.gaia_source+WHERE+phot_g_mean_mag+<+3.0+ORDER+BY+phot_g_mean_mag&format=json
https://gea.esac.esa.int/tap-server/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=json&QUERY=SELECT+COUNT(*
https://gea.esac.esa.int/tap-server/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=json&QUERY=SELECT+TOP+10+source_id,ra,dec,phot_g_mean_mag,parallax+FROM+gaiadr3.gaia_source+WHERE+phot_g_mean_mag+<+3.0+ORDER+BY+phot_g_mean_mag
https://gea.esac.esa.int/tap-server/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=json&QUERY=SELECT+TOP+50+source_id,ra,dec,phot_g_mean_mag,parallax+FROM+gaiadr3.gaia_source+WHERE+phot_g_mean_mag+<+3.0+ORDER+BY+phot_g_mean_mag
https://geomag.usgs.gov/ws/data/?...
https://geomag.usgs.gov/ws/data/?id={nearest_station}&starttime={today}T00:00:00&endtime={today}T23:59:59&format=json&elements=X,Y,Z,F
https://geomag.usgs.gov/ws/observatories/
https://ghoapi.azureedge.net/api/Indicator
https://glottolog.org/resource/languoid/id/stan1293.json
https://gml.noaa.gov/erddap/tabledap/co2_mlo_surface_weekly.csv?time,value&orderByMax(%22time%22
https://gml.noaa.gov/webdata/ccgg/trends/cfc12/cfc12_mm_gl.txt
https://gml.noaa.gov/webdata/ccgg/trends/ch4/ch4_mm_gl.txt
https://gml.noaa.gov/webdata/ccgg/trends/co2/co2_weekly_mlo.txt
https://gml.noaa.gov/webdata/ccgg/trends/n2o/n2o_mm_gl.txt
https://gml.noaa.gov/webdata/ccgg/trends/sf6/sf6_mm_gl.txt
https://gracedb.ligo.org/api/
https://gracedb.ligo.org/api/superevents/?count=10&category=production
https://gracedb.ligo.org/api/superevents/?format=json&orderby=-created&count=5
https://gwosc.org/eventapi/json/allevents/
https://huggingface.co/api/models?sort=downloads&search=llama
https://huggingface.co/inference-api
https://imag-data.bgs.ac.uk/GIN_V1/GINServices?Request=GetData&format=JSON&observatoryIagaCode=**CODE**&samplesPerDay=1440&dataStartDate={today}&dataDuration=1
https://imag-data.bgs.ac.uk/GIN_V1/GINServices?Request=GetData&format=JSON&observatoryIagaCode={nearest_observatory}&samplesPerDay=minute&dataStartDate={today}&dataDuration=1
https://imag-data.bgs.ac.uk/GIN_V1/GINServices?Request=GetData&format=JSON&observatoryIagaCode=NGK&samplesPerDay=1440&dataStartDate={}&dataDuration=1
https://imag-data.bgs.ac.uk/GIN_V1/GINServices?Request=GetData&format=JSON&observatoryIagaCode=**STATION_CODE**&samplesPerDay=1440&dataStartDate={today}&dataDuration=1
https://intermagnet.github.io/data/k-index/
https://intermagnet.github.io/data/k-index/{nearest_observatory}/{today}.json
https://intermagnet.github.io/data/k-index/{station}/{date}.json
https://intermagnet.github.io/stations.json
https://ioda.caida.org/api/v1/outages
https://irwin.doi.gov/observer/incidents/5c3a9dc4-43ad-4b51-9110-554a2267d827
https://irwin.doi.gov/observer/incidents/860ef78f-f819-411e-b109-ce167510fc50
https://irwin.doi.gov/observer/incidents/8d86b276-309e-484d-844e-2f2f6419be72
https://irwin.doi.gov/observer/incidents/b109ff45-8c62-4301-aef9-77acc3736ac4
https://jsoc.stanford.edu/cgi-bin/ajax/jsoc_fetch?op=rs_list&ds=hmi.M_720s[{today}]&key=REDACTED&format=json
https://labs.waterdata.usgs.gov/api/nldi/linked-data/nwissite/...
https://labs.waterdata.usgs.gov/api/observations/latest?monitoringLocationId={nearest_site}&observedProperty=00060,00065,00010
https://lambda.gsfc.nasa.gov/data/map/dr5/skymaps/5yr/raw/wmap_5yr_wmap_v1.fits
https://marine-api.open-meteo.com/
https://marine-api.open-meteo.com/v1/marine?latitude={lat}&longitude={lon}&current=wave_height,wave_direction,wave_period
https://marinecadastre.gov/ais/api/v1/locations?bbox={lon_min},{lat_min},{lon_max},{lat_max}
https://marinecadastre.gov/ais/v1/messages...
https://marinecadastre.gov/ais/v1/messages?bbox={lon_min},{lat_min},{lon_max},{lat_max}
https://mempool.space/api/v1/fees/recommended
https://meri.digitraffic.fi/api/ais/v1/locations
https://minorplanetcenter.net/iau/MPCORB/MPCORB.DAT
https://musicbrainz.org/ws/2/recording/?query=artist:{artist}&fmt=json
https://my-api.plantnet.org/v2/identify/all?api-key=REDACTED
https://neo.ssa.esa.int/api/neo/close-approaches?date-min={today}&date-max={tomorrow}
https://neurovault.org/api/collections/?limit=1
https://nextstrain.org/charon?...
https://nextstrain.org/charon/getDataset...
https://nextstrain.org/charon/getDataset?prefix=/...
https://nextstrain.org/charon/getDataset?prefix=/ncov/open/global
https://nextstrain.org/charon/getDataset?prefix=/nextclade/sars-cov-2
https://nextstrain.org/charon/getDataset?prefix=/wuhan/{date}
https://noaadata.apps.nsidc.org/NOAA/G02135/north/daily/data/N_seaice_extent_daily_v3.0.csv
https://noaadata.apps.nsidc.org/NOAA/G02135/south/daily/data/S_seaice_extent_daily_v3.0.csv
https://nomads.ncep.noaa.gov/cgi-bin/filter_hrrr_2d_00z.pl?dir=%2Fhrrr.YYYYMMDD%2Fconus&file=hrrr.YYYYMMDD%2Fconus%2Fhrrr.YYYYMMDD00.t00z.wrfsf00.grib2&var=SMOKE&lev=surface&lat={lat}&lon={lon}
https://nomads.ncep.noaa.gov/cgi-bin/filter_wave_00z.pl?dir=%2Fwave.YYYYMMDD&file=wave_00z.grb2&var=HTSGW&lev=surface&lat={lat}&lon={lon}
https://nominatim.openstreetmap.org/reverse?lat={lat}&lon={lon}&format=json
https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=json
https://nrt.smap.jpl.nasa.gov/nrt/data/SPL4SMGP/20260627/smap_SPL4SMGP_{today}_0000_00000.h5
https://onionoo.torproject.org/summary?running=true
https://openinfra.io/api/v1/power/plants
https://openneuro.org/api/crn/datasets
https://openneuro.org/api/crn/datasets?limit=1
https://openquantumsafe.org/api/v1/algorithms
https://opensky-network.org/api/states/all
https://opensky-network.org/api/states/all...
https://opensky-network.org/api/states/all?lamin=40&lomin=-10&lamax=50&lomax=10
https://opensky-network.org/api/states/all?lamin={lat_min}&lomin={lon_min}&lamax={lat_max}&lomax={lon_max}
https://overpass-api.de/api/inter
https://overpass-api.de/api/interpreter...
https://overpass-api.de/api/interpreter?data=...
https://overpass-api.de/api/interpreter?data=[out:json];area[name=
https://overpass-api.de/api/interpreter?data=[out:json];(node[
https://overpass-api.de/api/interpreter?data=[out:json];node[
https://overpass-api.de/api/interpreter?data=[out:json][timeout:25];node[
https://overpass-api.de/api/interpreter?data=[out:json][timeout:60];node[
https://overpass-api.de/api/interpreter?data=[out:json];way[
https://paperswithcode.com/api/v1/papers/recent
https://p.ntrlst.rg/v1/bsrvtns
https://portal.opentopography.org/API/globaldem?demtype=SRTMGL3&south={lat_min}&north={lat_max}&west={lon_min}&east={lon_max}&outputFormat=json
https://power.larc.nasa.gov/api/temporal/daily/point?...
https://power.larc.nasa.gov/api/temporal/daily/point?...&community=AG&parameters=NDVI
https://power.larc.nasa.gov/api/temporal/daily/point?parameters=ALLSKY_SFC_SW_DWN,ALLSKY_SFC_UV_INDEX,WS50M,WD50M,T2MDEW&community=RE&longitude={lon}&latitude={lat}&format=JSON
https://power.larc.nasa.gov/api/temporal/daily/point?parameters=ALLSKY_SFC_SW_DWN,ALLSKY_SFC_UV_INDEX,WS50M,WD50M,T2MDEW,T2MWET&community=RE&longitude={lon}&latitude={lat}&format=JSON
https://power.larc.nasa.gov/api/temporal/daily/point?parameters=ALLSKY_SFC_SW_DWN,CLRSKY_SFC_SW_DWN,ALLSKY_SFC_UV_INDEX,WS50M,WD50M,T2MDEW,T2MWET&community=RE&longitude={lon}&latitude={lat}&format=JSON
https://power.larc.nasa.gov/api/temporal/daily/point?parameters=ALLSKY_SFC_SW_DWN&community=RE&longitude={lon}&latitude={lat}&format=JSON
https://power.larc.nasa.gov/api/temporal/daily/point?parameters=ALLSKY_SFC_SW_DWN,WS10M,T2M&community=RE&longitude={lon}&latitude={lat}&format=JSON
https://power.larc.nasa.gov/api/temporal/daily/point?parameters=...&latitude=...&longitude=...
https://poweroutage.us/api/states.json
https://p.pn-mt.cm/v1/frcst?...
https://p.pn-mt.cm/v1/lvtn
https://p.pn-mt.cm/v1/lvtn?...
https://pskreporter.info/cgi-bin/lastspots.cgi?last=24
https://p.tdsndcrrnts.n.gv/p/prd/
https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/{compound}/property/MolecularWeight,IUPACName/JSON
https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/{compound}/property/MolecularWeight,IUPACName,XLogP/JSON
https://qrng.anu.edu.au/API/jsonI.php?length=1&type=uint8
https://quantum-computing.ibm.com/api/backends
https://quantumrandomnumbergenerator.net/api/1.0/json
https://quickstats.nass.usda.gov/api/api_GET/?key=REDACTED&commodity_desc=CORN&statisticcat_desc=PRODUCTION&format=JSON
https://random.dog/woof.json
https://random-d.uk/api
https://randomfox.ca/floof/
https://rapid.ac.uk/data
https://rapid.ac.uk/data/
https://rapid.ac.uk/data_download.php
https://resonanceone.app/api/now
https://restcountries.com/v3.1/all
https://restcountries.com/v3.1/alpha/{country_code}
https://rest.ensembl.org/info/genomes?content-type=application/json
https://rest.uniprot.org/uniprotkb/search?query=organism_id:9606&format=json&size=1
https://ris-live.ripe.net/
https://ris.ripe.net/v1/peers
https://rthqk.sgs.gv/rthqks/fd/v1.0/smmry/ll_hr.gjsn
https://rucsoundings.noaa.gov/get_soundings.cgi
https://rucsoundings.noaa.gov/get_soundings.cgi?data_source=GSD&latest=latest&n_hrs=1.0&wban_stn={wban_station}&startSecs={unix_now}&endSecs={unix_now_plus_3600}&fcst_len=shortest&airport={station_id}&text=Ascii%20text%20%28GSD%20format%29
https://search.rcsb.org/rcsbsearch/v2/query?json={
https://service.iris.edu/fdsnws/
https://service.iris.edu/fdsnws/dataselect/1/query
https://service.iris.edu/fdsnws/dataselect/1/query...
https://service.iris.edu/fdsnws/dataselect/1/query?...
https://service.iris.edu/fdsnws/dataselect/1/query?start={yesterday}&end={today}&format=miniseed&lat={lat}&lon={lon}&maxradius=2
https://service.iris.edu/fdsnws/event/1/
https://service.iris.edu/fdsnws/event/1/query
https://service.iris.edu/fdsnws/event/1/query?format=geojson&lat=40&lon=-105&maxradius=2&orderby=time&limit=1
https://service.iris.edu/fdsnws/event/1/query?format=geojson&latitude={lat}&longitude={lon}&maxradius=2&minmagnitude=1.0
https://service.iris.edu/fdsnws/event/1/query?format=geojson&lat={lat}&lon={lon}&maxradius=2&orderby=time-asc&limit=1&starttime={yesterday}
https://service.iris.edu/fdsnws/event/1/query?format=geojson&lat={lat}&lon={lon}&maxradius=2&orderby=time&limit=1
https://service.iris.edu/fdsnws/event/1/query?format=json&starttime={hour_ago}&minmag=2.0
https://service.iris.edu/fdsnws/station/1/query?...
https://services.nvd.nist.gov/rest/json/cves/2.0?resultsPerPage=1
https://services.swpc.noaa.gov/images/animations/suvi/primary/304/latest.jpg
https://services.swpc.noaa.gov/json/
https://services.swpc.noaa.gov/json/...
https://services.swpc.noaa.gov/json/aurora_dashboard.json
https://services.swpc.noaa.gov/json/goes/primary/differential-protons-1-day.json
https://services.swpc.noaa.gov/json/goes/primary/euv-1-day.json
https://services.swpc.noaa.gov/json/goes/primary/integral-electrons-1-day.json
https://services.swpc.noaa.gov/json/goes/primary/integral-protons-1-day.json
https://services.swpc.noaa.gov/json/goes/primary/magnetometers-1-day.json
https://services.swpc.noaa.gov/json/goes/primary/xray-flares-7-day.json
https://services.swpc.noaa.gov/json/goes/primary/xrays-1-day.json
https://services.swpc.noaa.gov/json/ovation_aurora_latest.json
https://services.swpc.noaa.gov/json/planetary_k_index_1m.json
https://services.swpc.noaa.gov/json/rtsw/...
https://services.swpc.noaa.gov/json/rtsw/rtsw_tec_global.json
https://services.swpc.noaa.gov/json/solar-cycle/observed-solar-cycle-indices.json
https://services.swpc.noaa.gov/json/solar-cycle/solar-cycle-predicted.json
https://services.swpc.noaa.gov/json/solar_radio_burst.json
https://services.swpc.noaa.gov/json/solar-radio-flux.json
https://services.swpc.noaa.gov/json/solar_regions.json
https://services.swpc.noaa.gov/json/solar/sunspot_report.json
https://services.swpc.noaa.gov/json/solar-wind/plasma-7-day.json
https://services.swpc.noaa.gov/json/space-weather/alerts.json
https://services.swpc.noaa.gov/products/alerts/drap_5m.json
https://services.swpc.noaa.gov/products/alerts/drap.txt
https://services.swpc.noaa.gov/products/geospace/
https://services.swpc.noaa.gov/products/geospace/propagation-1-day.json
https://services.swpc.noaa.gov/products/kyoto-dst.json
https://services.swpc.noaa.gov/products/solar-wind/mag-7-day.json
https://services.swpc.noaa.gov/products/solar-wind/plasma-7-day.json
https://services.swpc.noaa.gov/products/solar-wink/plasma-7-day.json
https://services.swpc.noaa.gov/text/aurora-nowcast-map.txt
https://services.swpc.noaa.gov/text/us-tec-summary.txt
https://services.swpc.noaa.gov/text/us-tec.txt
https://simbad.u-strasbg.fr/simbad/sim-id?...
https://simbad.u-strasbg.fr/simbad/sim-tap/sync?request=doQuery&lang=adql&format=json&query=SELECT+TOP+10+basic.name,basic.ra,basic.dec,basic.otype+FROM+basic+WHERE+otype=
https://soa.smext.faa.gov/asws/api/airport/status/ATL?format=json
https://soa.smext.faa.gov/asws/api/airport/status/EWR?format=json
https://soa.smext.faa.gov/asws/api/airport/status/LAX?format=json
https://srvcs.swpc.n.gv/jsn/...
https://ssd-api.jpl.nasa.gov/cad.api?date-min={}&date-max={}
https://ssd-api.jpl.nasa.gov/cad.api?date-min={today}&date-max={tomorrow}
https://ssd-api.jpl.nasa.gov/fireball.api?limit={}
https://ssd-api.jpl.nasa.gov/fireball.api?limit=10
https://ssd-api.jpl.nasa.gov/sentry.api
https://ssd.jpl.nasa.gov/api/horizons.api
https://ssd.jpl.nasa.gov/api/horizons.api...
https://ssd.jpl.nasa.gov/api/horizons.api?fmt=json&COMMAND=%27599%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%2710%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27-170%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27199%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%272000001%3B%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%272000002%3B%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%272000004%3B%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%272101955%3B%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27{}%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%272026-06-22%27&STOP_TIME=%272026-06-23%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27299%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27301%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27499%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27599%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27699%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27799%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27899%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&API_HORIZONS,
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%279000028%3B%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%279003331%3B%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27999%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27999%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS端午?%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27999%27&OBJ_DATA=%27NO%&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%2799942%3B%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://ssd.jpl.nasa.gov/api/horizons.api?format=json&URL=https://ssd.jpl.nasa.gov/api/horizons.api?format=json&COMMAND=%27999%27&OBJ_DATA=%27NO%27&MAKE_EPHEM=%27YES%27&EPHEM_TYPE=%27VECTORS%27&CENTER=%27500%400%27&START_TIME=%27{today}%27&STOP_TIME=%27{tomorrow}%27&STEP_SIZE=%271%20d%27
https://stat.ripe.net/data/bgp-updates/data.json?resource=0.0.0.0/0&starttime={}&endtime={}
https://stat.ripe.net/data/bgp-updates/data.json?resource=0.0.0.0/0&starttime={hour_ago}&endtime={now}
https://stat.ripe.net/data/network-info/data.json...
https://stat.ripe.net/data/network-info/data.json?resource=8.8.8.8
https://stat.ripe.net/data/network-info/data.json?resource={ip}
https://stat.ripe.net/data/network-info/data.json?resource={observer_ip}
https://stats.oecd.org/SDMX-JSON/data/MEI_CLI/LOLIHR.MEI/all?startTime=latest&endTime=latest
https://sungeo.net/api/current
https://supermag.jhuapl.edu/mag/?start={today}T{hour}%3A{minute}&interval=PT10M&station={nearest_station}&fmt=json&content=all
https://supermag.jhuapl.edu/services/data.php
https://supermag.jhuapl.edu/services/data.php?start={today}T{hour}%3A{minute}&delta=10&station={nearest_station}&user={supermag_user}&fmt=json&content=magsmei
https://supermag.jhuapl.edu/services/indices.php
https://supermag.jhuapl.edu/services/indices.php?start={today}T{hour}%3A{minute}&delta=10&user={supermag_user}&fmt=json&indices=SME,SML,SMU
https://supermag.jhuapl.edu/services/inventory.php
https://supermag.jhuapl.edu/services/inventory.php?user={supermag_user}&fmt=json
https://timeapi.io/api/Time/current/zone?timeZone=UTC
https://tle.ivanstanojevic.me/api/v1/satellite/?search=25544
https://{transit_agency}/gtfs/vehicle_positions.json
https://transparency.entsoe.eu/api?documentType=A65&processType=A16&in_Domain={country_code}&securityToken=REDACTED
https://volcanoes.usgs.gov/vhp/updates.json
https://volcanoes.usgs.gov/vsc/api/so2/getLatestSo2?volc_code=HAVO
https://volcano.si.edu/news/WeeklyVolcanoRSS.jsp
https://waterservices.usgs.gov/nwis/iv/...
https://waterservices.usgs.gov/nwis/iv/?format=json&latitude=40.0&longitude=-105.0&parameterCd=00060
https://waterservices.usgs.gov/nwis/iv/?format=json&latitude=40.0&longitude=-105.0&parameterCd=00060&siteStatus=active
https://waterservices.usgs.gov/nwis/iv/?format=json&latitude=40&longitude=-105&parameterCd=00060
https://waterservices.usgs.gov/nwis/iv/?format=json&latitude={lat}&longitude={lon}&parameterCd=00060
https://waterservices.usgs.gov/nwis/iv/?format=json&latitude={lat}&longitude={lon}&parameterCd=00060,00065,00010&siteStatus=active
https://waterservices.usgs.gov/nwis/iv/?format=json&parameterCd=00060,00065,00010&bBox={lon_min},{lat_min},{lon_max},{lat_max}&siteStatus=active
https://waterservices.usgs.gov/nwis/iv/?format=json&parameterCd=00060&bBox={lon_min},{lat_min},{lon_max},{lat_max}
https://waterservices.usgs.gov/nwis/iv/?format=json&parameterCd=00060&siteStatus=active&latitude={lat}&longitude={lon}
https://waterservices.usgs.gov/nwis/iv/?format=json&sites=06730200&parameterCd=00060,00065,00010&siteStatus=active
https://waterservices.usgs.gov/nwis/iv/?format=json&sites={nearest_site}&parameterCd=00060,00065,00010&siteStatus=active
https://waterservices.usgs.gov/nwis/iv/?format=json&sites={nearest_site}&parameterCd=00095,00300,00400&siteStatus=active
https://waterservices.usgs.gov/nwis/iv/?format=json&sites={nearest_site}&parameterCd=72019&siteStatus=active
https://waterservices.usgs.gov/nwis/iv/?format=json&sites=&parameterCd=00060&siteStatus=active&latitude=40.0&longitude=-105.0&radius=25
https://web-api.tp.entsoe.eu/api?documentType=A65&in_Domain=10YDE-VE------2&out_Domain=10YDE-VE------2&periodStart=202606270000&periodEnd=202606280000
https://webbook.nist.gov/cgi/cbook.cgi?Name={compound}&Units=SI&cTG=on&cIR=on&cMS=on&cUV=on&cTHz=on&cMS=on&cESc=on
https://webbook.nist.gov/cgi/cbook.cgi?Name={compound}&Units=SI&cTG=on&cJSON=on
https://whale.fm/api/v1/sounds/recent
https://world.openagritechdata.org/api/v0/product/{barcode}.json
https://www.addgene.org/api/v1/plasmids.json?limit=1
https://www.airnowapi.org/aq/observation/latLong/current/...
https://www.airnowapi.org/aq/observation/latLong/current/?format=application/json&latitude=40.0&longitude=-105.0&distance=25&API_KEY=REDACTED
https://www.airnowapi.org/aq/observation/latLong/current/?format=application/json&latitude={lat}&longitude={lon}&distance=25&API_KEY=REDACTED
https://www.alphavantage.co/query?function=GLOBAL_QUOTE...
https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=SPY&apikey=REDACTED
https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={symbol}&apikey=REDACTED
https://www.crystallography.net/cod/result?formula={compound}&format=json
https://www.ebi.ac.uk/chembl/api/data/similar.json?smiles={compound}&limit=1
https://www.govtrack.us/api/v2/vote...
https://www.govtrack.us/api/v2/vote?created__gt={yesterday}&limit=0
https://www.imf.org/external/datamapper/api/v1/NGDP_R/WEOWRL
https://www.lido-ifremer.fr/data/acoustic/last24h.json
https://www.limaps.org/C1/2026/06/27/09/00.json
https://www.movebank.org/movebank/service/...
https://www.movebank.org/movebank/service/direct-read?...
https://www.movebank.org/movebank/service/direct-read?entity_type=event&study_id={study_id}&sensor_type_id=gps
https://www.movebank.org/movebank/service/direct-read?entity_type=study
https://www.movebank.org/movebank/service/public/json?entity_type=study&i_can_see_data=true
https://www.ncbi.nlm.nih.gov/geo/query/acc.cgi?acc=GSE1&targy=0&view=json
https://www.ndbc.noaa.gov/activestations.xml
https://www.ndbc.noaa.gov/data/realtime2/46042.txt
https://www.ndbc.noaa.gov/data/realtime2/{nearest_buoy}.txt
https://www.ndbc.noaa.gov/data/realtime2/{station}.txt
https://www.ndbc.noaa.gov/ndbcmapstations.json
https://www.nmdb.eu/nest/api/get_data.php?stations=OULU&start={hour_ago}&end={now}&output=json
https://www.nmdb.eu/nest/draw_graph.php?stations[]=DOMC&tabchoice=revori&dtype=corr_for_efficiency&tresolution=60&force=1&output=ascii
https://www.nmdb.eu/nest/draw_graph.php?stations[]=NEWK&tabchoice=revori&dtype=corr_for_efficiency&tresolution=60&force=1&output=ascii
https://www.nmdb.eu/nest/draw_graph.php?stations[]=OULU&tabchoice=revori&dtype=corr_for_efficiency&tresolution=60&force=1&yunits=0&date_choice=2&history=60&output=ascii
https://www.ocearch.org/tracker/ajax/find-sharks
https://www.ocearch.org/tracker/ajax/sharks/
https://www.pmel.noaa.gov/tao/jsdisplay/data/acoustic.json
https://www.reddit.com/dev/api/
https://www.reversebeacon.net/dxlast24h.json
https://www.seismicportal.eu/fdsnws/event/1/query?format=json&starttime={hour_ago}&lat={lat}&lon={lon}&maxradius=5&minmag=2.0
https://www.space-track.org/api/v1/satellite/catalog
https://www.space-track.org/basicspacedata/query/
https://www.space-track.org/basicspacedata/query/...
https://www.spaceweatherlive.com/includes/live-data.php
https://www.spaceweatherlive.com/includes/live-data.php?object=SolarFlare
https://www.top500.org/lists/2023/06/json/
https://www.xeno-canto.org/api/2/recordings...
https://www.xeno-canto.org/api/2/recordings?query=...
https://www.xeno-canto.org/api/2/recordings?query=lat:40,lon:-105
https://www.xeno-canto.org/api/2/recordings?query=lat:{lat},lon:{lon}
https://www.xeno-canto.org/api/2/recordings?query=loc:{lat},{lon}
https://www.xeno-canto.org/api/2/recordings?query=q:A&page=1
https://www.xeno-canto.org/api/2/recordings?query={search}
https://www.xeno-canto.org/api/2/recordings?query=sp:red%20eyed%20vireo
https://xeno-canto.org/api/2/recordings?query=lat:{lat}+lon:{lon}
https://xeno-canto.org/explore/api
```
