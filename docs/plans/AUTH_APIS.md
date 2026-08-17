# Auth-APIs für omegaflow — vollständige Liste

Alle in den Forschungsdateien identifizierten APIs, die **Authentifizierung** erfordern.
Gruppiert nach Daten-Mehrwert für die 8-Kräfte-Punktwolke. Basis: Archiv (Schreibtisch).

## Legende

- **Auth-Typ**: Token / Key / Login / Registrierung / Bearer
- **Kosten**: frei = kostenlose Registrierung
- **Force**: welcher der 8 Forces die Daten zugeordnet wären
- **Status**: fehlt komplett / erweitert vorhanden / schon da

---

## Secrets-Matrix

Keys werden **niemals** in dieses Dokument oder das Repo geschrieben. Sie leben nur in:

1. **Lokal**: `.secrets.local` (gitignored) — Platzhalter für alle unten gelisteten Secrets
2. **GitHub Actions**: `Settings → Secrets and variables → Actions` (Workflow `refresh-protected-data.yml`)

### ✅ Bereits besorgt (im Workflow aktiv, legal redistributable)
 
| GitHub-Secret | Lokal (`.secrets.local`) | API | Registrierung |
|---|---|---|---|
| `EBIRD_API_KEY` | ✅ | eBird (CC-BY-NC) | https://ebird.org/api/keygen |
| `OPENAQ_API_KEY` | ✅ | OpenAQ (Open Data) | https://explore.openaq.org/register |
| `FIRMS_MAP_KEY` | ✅ | FIRMS Feuer (Public Domain) | https://firms.modaps.eosdis.nasa.gov/api/area/ |
| `EARTHDATA_EDL_TOKEN` | ✅ | NASA Earthdata (PD) | https://urs.earthdata.nasa.gov → Profil → Generate Token |
| `USGS_WATER_KEY` | ✅ | USGS Water (PD) | https://api.waterdata.usgs.gov/ogcapi/ |
| `CATALOGS_REPO`/`OMEGAFLOW_TOKEN` | ✅ | Publish-Ziel | PAT mit Contents-write |

### ✅ In `.secrets.local` vorhanden (2026-08-14 — aus der ehemaligen „Noch zu besorgen"-Liste eingelöst)

`NASA_API_KEY`, `NASA_ADS_TOKEN`, `SPACETRACK_USER/PASS`, `SUPERMAG_USER`
(API nutzt nur `user=`-Param, kein Passwort nötig), `CDS_API_KEY`, `JSOC_EMAIL`,
`NOAA_CDO_TOKEN`, `TNS_API_KEY` (integriert, Vollkatalog), `ZENODO_TOKEN`,
`MATERIALS_KEY`, `EIA_API_KEY`, `PLANTNET_KEY`, `AIRNOW_KEY`, `GFW_USER/PASS`,
`OCEANNETWORKS_TOKEN`, `TRANSPORTDATA_KEY` (tedp_*), `SI_API_KEY`, `IUCN_TOKEN`,
`MOVEBANK_USER/PASS/TOKEN`, `CMEMS_USER/PASS`, `GBIF_USER/PASS` (verifiziert,
3,9 Mrd. Vorkommen), `PURPLEAIR_KEY` (verifiziert, globale PM-Sensoren),
`TRANSIT511_KEY` (verifiziert, aber decline — Premium-Echtzeit + gzip + Registry).

### ❌ Entfernt (4 — ToS verbieten Redistribution via CDN)
 
| GitHub-Secret | API | Ersatz |
|---|---|---|
| `WAQI_TOKEN` | WAQI (aqicn.org) | OpenAQ (PM2.5/PM10, Open Data) |
| `OWM_API_KEY` | OpenWeatherMap | Open-Meteo (keyless) |
| `ALPHAVANTAGE_KEY` | AlphaVantage | — (kein Äquivalent nötig) |
| `FRED_API_KEY` | FRED St. Louis Fed | — (kein Äquivalent nötig) |

CMEMS_USER/CMEMS_PASS liegen noch in `.secrets.local`, die Quelle selbst ist aber
retired (ERDDAP → NOAA/EMODnet umgestellt).

### ❌ Decline (projekt-gebunden — kein globales Feld)

| Secret | API | Grund |
|---|---|---|
| `ARBIMON_KEY` | Arbimon (Bioakustik) | Projekt-gebunden: Aufnahmen gehören zu einzelnen Projekt-Standorten, kein öffentlicher Entwickler-Token, kein „ein Fetch = globales Feld". Force Gate erfüllt (acoustic), aber Datenmodell passt nicht. |
| `WILDLIFE_INSIGHTS_KEY` | Wildlife Insights (Kamera-Fallen) | Projekt-gebunden: Kamera-Deployments einzelner Organisationen, Datenzugriff über manuellen Download (≤ 500k Records), kein globaler Punktwolken-Endpoint. |
| `TNG_KEY` | IllustrisTNG (Galaxien) | Simulation, keine Messung (Force Gate). |

---

## A. Höchster Mehrwert — Daten fehlen komplett

| API | Endpoint | Auth | Kosten | Force | Status | Registrierung |
|---|---|---|---|---|---|---|
| **NASA ADS** | `api.adsabs.harvard.edu/v1/search/query` | Bearer-Token | frei, 1 Min | EM | fehlt | https://ui.adsabs.harvard.edu/user/settings/token |
| **Space-Track.org** | `www.space-track.org/basicspacedata/query/class/satcat` | Login | frei | EM | fehlt (voller Sat-Katalog) | https://www.space-track.org/auth/login |
| **SuperMAG** | `supermag.jhuapl.edu` | Login-Name | frei | EM | fehlt (~300 Stat.) | https://supermag.jhuapl.edu/info/signup.php |
| **GRACE-FO (PODAAC)** | `podaac.jpl.nasa.gov` S3-Bucket | Earthdata | frei | Gravity | fehlt | https://urs.earthdata.nasa.gov/users/new |
| **SMAP Bodenfeuchte** | `nsidc.org/data/smap` | Earthdata | frei | Gravity | fehlt | https://urs.earthdata.nasa.gov/users/new |
| **CDDIS IONEX** | `cddis.nasa.gov` | Earthdata | frei | EM | fehlt (Ionosphäre) | https://urs.earthdata.nasa.gov/users/new |
| **GES DISC (GPM/MODIS)** | `gesdisc.eosdis.nasa.gov` | Earthdata | frei | EM/Thermal | fehlt | https://urs.earthdata.nasa.gov/users/new |
| **NASA AppEEARS** | `appeears.earthdatacloud.nasa.gov` | Earthdata | frei | Thermal | fehlt | https://urs.earthdata.nasa.gov/users/new |

## B. Mittel — erweitert vorhandene Daten

| API | Endpoint | Auth | Kosten | Force | Status | Registrierung |
|---|---|---|---|---|---|---|
| **MarineTraffic AIS** | `services.marinetraffic.com/api/` | Key | teils paid | Advective | fehlt Echtzeit | https://www.marinetraffic.com/en/ais-api-services |
| **Global Fishing Watch** | `globalfishingwatch.org/our-apis/` | Token | frei | Advective | fehlt | https://globalfishingwatch.org/our-apis/ |
| **Copernicus CDS/ADS** | `cds.climate.copernicus.eu/api` | CDS-Key | frei | Thermal | fehlt (ERA5) | https://cds.climate.copernicus.eu/user/register |
| **JSOC Sonne** | `jsoc.stanford.edu` | Registrierung | frei | EM/Thermal | fehlt (HMI) | https://jsoc.stanford.edu/ajax/register_account.html |
| **Open-Meteo** | `api.open-meteo.com` | Keyless | frei | Acoustic/Thermal | schon da (ersetzt OWM) | — |
| **SAWS Südafrika** | `api.weathersa.co.za` | Registrierung | frei | Thermal | fehlt | https://api.weathersa.co.za/ |
| **NOAA NCDC/CDO** | `www.ncdc.noaa.gov/cdo-web/api/v2/` | Token | frei | Thermal | fehlt | https://www.ncdc.noaa.gov/cdo-web/token |
| **NASA NEO Asteroids** | `api.nasa.gov/neo/rest/v1/` | Key | frei | Gravity | erweitert | https://api.nasa.gov/ (auch: https://api.nasa.gov/#signUp) |
| **NASA Insight Mars** | `api.nasa.gov/insight_weather` | Key | frei | Thermal | fehlt | https://api.nasa.gov/ |
| **NASA Mars Photos** | `api.nasa.gov/mars-photos` | Key | frei | EM | fehlt | https://api.nasa.gov/ |
| **TNS Transient** | `www.wis-tns.org/api/` + CSV-Staging | Key + User-Marker | frei | EM | integriert (Vollkatalog, csv_zip) | https://www.wis-tns.org/user/register |
| **Zenodo** | `zenodo.org/api/records` | Token | frei | EM | erweitert | https://zenodo.org/oauth/login |

## C. Nische — spezifischer Mehrwert

| API | Endpoint | Auth | Kosten | Force | Status | Registrierung |
|---|---|---|---|---|---|---|
| **Materials Project** | `api.materialsproject.org` | Key | frei | Diffusion | fehlt | https://materialsproject.org/register |
| **EIA Energie** | `api.eia.gov` | Key | frei | Thermal | erweitert | https://www.eia.gov/opendata/register.php |
| **AlphaVantage** | `alphavantage.co/query` | Key | frei | EM | fehlt | https://www.alphavantage.co/support/#api-key |
| **FRED (St. Louis)** | `api.stlouisfed.org/fred` | Key | frei | EM | fehlt | https://fred.stlouisfed.org/docs/api/api_key.html |
| **PlantNet** | `my-api.plantnet.org` | Key | frei | Diffusion | fehlt | https://my.plantnet.org/ |
| **AirNow EPA** | `www.airnowapi.org` | Key | frei | Diffusion | fehlt | https://docs.airnowapi.org/account/request/ |
| **OpenAQ** | `api.openaq.org/v3` | Key | frei | Diffusion | erweitert | https://api.openaq.org/register/ |
| **IAEA WISER (GNIP)** | `nucleus.iaea.org/wiser` | Registrierung | frei | Diffusion | fehlt Bulk | https://nucleus.iaea.org/wiser/ |
| **IUCN Red List** | `apiv3.iucnredlist.org` | Token (demo) | frei | Diffusion | fehlt | https://apiv3.iucnredlist.org/api/v3/token |
| **SCISAT** | `databace.scisat.ca` | Registrierung | frei | EM | fehlt | https://databace.scisat.ca/ |
| **Global Forest Watch** | `data-api.globalforestwatch.org` | Token | frei | Diffusion | fehlt | https://data.globalforestwatch.org/ |
| **Ocean Networks Canada** | `data.oceannetworks.ca` | Token | frei | Advective | fehlt | https://data.oceannetworks.ca/DataSearch |
| **Swiss OpenTransport** | `api.opentransportdata.swiss` | Key | frei | Advective | fehlt | https://opentransportdata.swiss/en/dataset/ |
| **511.org Transit** | `api.511.org/transit/` | Key | frei | Advective | fehlt | https://511.org/open-data/api |
| **IllustrisTNG** | `tng-project.org/data/` | Key | frei | EM | fehlt (Galaxien) | https://www.tng-project.org/register/ |
| **OpenTopography** | `portal.opentopography.org` | Demo-Key | frei | EM | erweitert | https://opentopography.org/ |
| **Smithsonian Open Access** | `api.si.edu` | Key | frei | EM | fehlt | https://api.data.gov/signup/ (SI-Key via api.data.gov) |
| **PurpleAir** | `api.purpleair.com/v1/` | Key | frei | Diffusion | Key erstellt 2026-06-23 | https://develop.purpleair.com/ |
| **Arbimon** | `arbimon.rfcx.org` API | Token | frei | Acoustic | ~~E-Mail verifiziert 2026-08-08~~ → decline projekt-gebunden | https://arbimon.rfcx.org/ |
| **Wildlife Insights** | `www.wildlifeinsights.org` API | Key | frei | EM | ~~approved 2026-08-12~~ → decline projekt-gebunden | https://www.wildlifeinsights.org/ |

## D. Niedrig — viel Aufwand, wenig Mehrwert

| API | Endpoint | Auth | Warum niedrig | Registrierung |
|---|---|---|---|---|
| **CNEMC China AQI** | `air.cnemc.cn:18007` | POST-only | GET-only Ingestor | — |
| **BOM v1 Weather** | `api.weather.bom.gov.au/v1/` | urheberrechtl. | FTP-Mirror reicht | — |
| **BOM Space Weather** | `sws-data.sws.bom.gov.au` | Key | UV als Text verfügbar | https://sws-data.sws.bom.gov.au/ |
| **ThingSpeak** | `thingspeak.com` | Key | unstrukturiert | https://thingspeak.com/account/create |
| **EarthNetworks** | `api.earthnetworks.com` | Key | paid | https://www.earthnetworks.com/ |

## E. Offene Registrierungen (key-needed ohne Secret)

Stand 2026-08-17 — aus `phi/blocked_sources.φ` (key-needed) gegen `.secrets.local`
abgeglichen. Leere Stubs liegen in `.secrets.local`.

**Grundsatz CDN-Redistribution:** Alle Messwerte werden auf das CDN (GitHub
Releases) manifestiert und öffentlich redistribuiert. Es kommen NUR APIs infrage,
deren Lizenz/ToS Redistribution erlauben (Public Domain, CC0, CC-BY, OGL/NLOD).
Kommerzielle/proprietäre APIs und solche mit Redistributionsverbot sind entfernt.
Das Urteil ist im Register vollzogen: die 19 Hosts stehen in
`phi/blocked_sources.φ` als `decline redistribution` / `decline no-physical-force`
(mit Verdikt-Note, Stand 2026-08-17) und in `phi/interesting_domains.φ` §5.

| Host | Secret (Stub) | Auth | Lizenz | Registrierung |
|---|---|---|---|---|
| archive.opensearch.ceda.ac.uk | `CEDA_USER`/`CEDA_PASS` | Login (OpenID) | OGL/CC (UK) | https://archive.ceda.ac.uk/ (Token-API: services.ceda.ac.uk/api/token/create/) |
| data.icos-cp.eu | `ICOS_USER`/`ICOS_PASS` | Login | CC-BY 4.0 | https://data.icos-cp.eu/ (cpauth.icos-cp.eu — Account vorhanden) |
| frost.met.no | `FROST_CLIENT_ID` | Client-ID | NLOD/CC | https://frost.met.no/ (Client registrieren) |
| gracedb.ligo.org | ~~`GRACEDB_TOKEN`~~ | ~~Auth~~ | offen (Alerts) | ~~https://gracedb.ligo.org/~~ — refused: Private-Events verlangt LVC/MOU-Gruppenmitgliedschaft, kein Self-Service; public superevents offen in sources.φ. |
| lasair-ztf.lsst.ac.uk | `LASAIR_TOKEN` | Token | offen | https://lasair-ztf.lsst.ac.uk/profile/ (API-Token angezeigt) |
| mast.stsci.edu | `MAST_TOKEN` | Token | Public Domain | https://mast.stsci.edu/ (TESS/HST/JWST-Photometrie, em — Token vorhanden) |
| toar-data.fz-juelich.de | `TOAR_USER`/`TOAR_PASS` | Login | offen | https://toar-data.fz-juelich.de/ (API v2 vorhanden — Registrierung pending 2026-08-17) |

**Geklärt — offen ohne Key (aus key-needed entfernt, Stand 2026-08-17):**

| Host | Befund |
|---|---|
| alerce.online | ZTF-Database-API (`api.alerce.online/ztf/v1`) laut ALeRCE-Service-Doku „without needing any authentication" — offen, kein Token. `ALERCE_TOKEN`-Stub entfernt. |
| data.cosmic.ucar.edu | GNSS-RO/-R/Suominet laut UCAR „no need for a login to access these data"; curl 200 auf allen drei Pfaden. `CDAAC_USER`/`CDAAC_PASS`-Stubs entfernt. |
| climate-themetoffice.hub.arcgis.com | Met-Office-Climate-Data-Portal: „You do not need to log in to download the data"; ArcGIS-REST-FeatureServer + OGC-API-Records-Search-API, offen. `METOFFICE_CLIMATE_KEY`-Stub entfernt. |
| irsa.ipac.caltech.edu | Kein API-Token. Öffentliche Daten (WISE/NEOWISE/2MASS/Spitzer/SPHEREx/Euclid/ZTF-DR24) offen auf AWS S3 Open Data (anonymous). Proprietäres ZTF via IPAC-SSO (`curl --user email:passwort`). Verbleibt `decline redundant` — ZTF über Lasair/ALeRCE. `IRSA_AUTH`→`IRSA_USER`/`IRSA_PASS`. |
| marinecadastre.gov | Kein Live-API — AIS-Bulk-Downloads (CSV/GDB) auf Azure-Blob, Public Domain, „Bulk data downloads are still available"; AccessAIS-Bestellservice derzeit pausiert. `MARINECADASTRE_KEY`-Stub entfernt. |
| sios-svalbard.org | SIOS-User-Account nur für Working-Group/Committee/Katalog-Edit/SESS — „You do not need a SIOS user account if you want to access/search on the Data Portal or download data". Datenzugriff offen (Station-Data-REST `/rest/stations/data.json` curl 200; Blue-Cloud ohne AAI). `SIOS_TOKEN`-Stub entfernt. |

### Entfernt — kommerziell / proprietär / Redistribution verboten

| Host | Grund |
|---|---|
| api.electricitymap.org | kommerziell (Daten proprietär) |
| api.maptiler.com | kommerzielle Tiles |
| api.sncf.com | kommerziell (ToS) |
| api.z.ai | kommerziell (AI-Modell, kein Messwert) |
| bgp.tools | kommerziell, Redistribution eingeschränkt |
| docs.sentinel-hub.com | kommerziell (Subscription) |
| dev.meteostat.net / meteostat.net | kommerziell (paid) |
| www.dxpredictor.com | kommerziell |
| www.reddit.com | kommerziell (ToS: keine Redistribution) |
| api.protectedplanet.net | WDPA — Redistribution eingeschränkt |
| api.resourcewatch.org | WRI — ToS/Redistribution unklar |
| data.blitzortung.org | Community-Lizenz, nicht-kommerziell, Redistribution eingeschränkt |
| gateway.api.globalfishingwatch.org | ToS — Redistribution eingeschränkt |
| data.lsst.cloud | LSST — proprietäre Frist |

### Entfernt — kein Messwert (Force-Gate)

| Host | Grund |
|---|---|
| crates.io | Paket-Registry |
| api.europeana.eu | Kultur-Metadaten |
| api.semanticscholar.org | Bibliographie |
| www.peeringdb.com | Netz-Infrastruktur |

### Open — kein Key nötig (403 = Bot-Block / User-Agent)

| Host | Anmerkung |
|---|---|
| api.sensor.community | offen; 403 = Bot-Block (UA/Header) |
| api.worldbank.org | offen; 403 = Bot-Block |
| climateknowledgeportal.worldbank.org | offen |
| volcano.si.edu | offen (GVP) |
| www.bom.gov.au | offen; 403 = Bot-Block/Geo |
| www.epa.gov | offen |
| www.ngdc.noaa.gov | offen |
| weather.cma.cn | offen |
| lisn.igp.gob.pe | offen |
| www3.mbari.org | offen |
| overpass-api.de | offen; 429/403 = Rate-Limit |
| overpass.kumi.systems | offen; Rate-Limit |

---

## Empfehlung: Einmal-Registrierung, max. Quellen-Freischaltung

1. **NASA Earthdata-Login** (1 Account) → öffnet: GRACE-FO, SMAP, IONEX, GES DISC, AppEEARS, MODIS GPP = 6+ Gravity/Thermal-Quellen
   → https://urs.earthdata.nasa.gov/users/new
2. **NASA API-Key** (api.nasa.gov, 1 Key) → DONKI, NEO, Insight, Mars Photos
   → https://api.nasa.gov/#signUp
3. **NASA ADS-Token** (1 Min) → Astro-Literatur
   → https://ui.adsabs.harvard.edu/user/settings/token

## Nächste Schritte

1. **Keys-Status (2026-08-17):** Kern-Keys sind besorgt. Offene, redistributions-konforme
   Registrierungen warten in Sektion E auf Einlösung; kommerzielle/proprietäre APIs
   sind ausgeschlossen (CDN-Redistribution).

2. Werte in `.secrets.local` eintragen (gitignored, keine Keys committen)
3. **Neue Quellen einbauen** (nächste Sessions):
   - PurpleAir (`diffusion`, globale PM-Sensoren, `X-API-Key`-Header) — verifiziert + live
   - GBIF (`em`, 3,9 Mrd. Vorkommen, Basic-Auth) — decline (Presence-Katalog)
   - Transit511 — decline (stops/operators = Registry; VehicleMonitoring = 401 Premium; gzip)
4. Workflow `refresh-protected-data.yml` erweitern — jede API als optionalen Step (überspringt wenn Secret leer)
5. Auth-Header/Query-Param im Fetch-System unterstützen (PurpleAir braucht `X-API-Key`-Header, GBIF Basic-Auth)
