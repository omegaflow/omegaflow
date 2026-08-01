# Auth-APIs für omegaflow — vollständige Liste

Alle in den Forschungsdateien identifizierten APIs, die **Authentifizierung** erfordern.
Gruppiert nach Daten-Mehrwert für die 8-Kräfte-Punktwolke. Basis: Archiv (Schreibtisch).

## Legende

- **Auth-Typ**: Token / Key / Login / Registrierung / Bearer
- **Kosten**: frei = kostenlose Registrierung
- **Force**: welcher der 8 Forces die Daten zugeordnet wären
- **Status**: fehlt komplett / erweitert vorhanden / schon da

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
| **OpenWeatherMap Lightning** | `api.openweathermap.org/data/2.5/` | Key | frei-begrenzt | Acoustic | erweitert | https://home.openweathermap.org/users/sign_up |
| **SAWS Südafrika** | `api.weathersa.co.za` | Registrierung | frei | Thermal | fehlt | https://api.weathersa.co.za/ |
| **NOAA NCDC/CDO** | `www.ncdc.noaa.gov/cdo-web/api/v2/` | Token | frei | Thermal | fehlt | https://www.ncdc.noaa.gov/cdo-web/token |
| **NASA NEO Asteroids** | `api.nasa.gov/neo/rest/v1/` | Key | frei | Gravity | erweitert | https://api.nasa.gov/ (auch: https://api.nasa.gov/#signUp) |
| **NASA Insight Mars** | `api.nasa.gov/insight_weather` | Key | frei | Thermal | fehlt | https://api.nasa.gov/ |
| **NASA Mars Photos** | `api.nasa.gov/mars-photos` | Key | frei | EM | fehlt | https://api.nasa.gov/ |
| **TNS Transient** | `www.wis-tns.org/api/` | Key | frei | EM | erweitert | https://www.wis-tns.org/user/register |
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

## D. Niedrig — viel Aufwand, wenig Mehrwert

| API | Endpoint | Auth | Warum niedrig | Registrierung |
|---|---|---|---|---|
| **CNEMC China AQI** | `air.cnemc.cn:18007` | POST-only | GET-only Ingestor | — |
| **BOM v1 Weather** | `api.weather.bom.gov.au/v1/` | urheberrechtl. | FTP-Mirror reicht | — |
| **BOM Space Weather** | `sws-data.sws.bom.gov.au` | Key | UV als Text verfügbar | https://sws-data.sws.bom.gov.au/ |
| **ThingSpeak** | `thingspeak.com` | Key | unstrukturiert | https://thingspeak.com/account/create |
| **EarthNetworks** | `api.earthnetworks.com` | Key | paid | https://www.earthnetworks.com/ |

---

## Empfehlung: Einmal-Registrierung, max. Quellen-Freischaltung

1. **NASA Earthdata-Login** (1 Account) → öffnet: GRACE-FO, SMAP, IONEX, GES DISC, AppEEARS, MODIS GPP = 6+ Gravity/Thermal-Quellen
   → https://urs.earthdata.nasa.gov/users/new
2. **NASA API-Key** (api.nasa.gov, 1 Key) → DONKI, NEO, Insight, Mars Photos
   → https://api.nasa.gov/#signUp
3. **NASA ADS-Token** (1 Min) → Astro-Literatur
   → https://ui.adsabs.harvard.edu/user/settings/token

## Nächste Schritte

- Keys/Tokens in GitHub-Actions-Secrets ablegen (workflow erweitern)
- Für jede Priorität-A-API Sources in `sources.φ` anlegen
- Auth-Header/Query-Param im Fetch-System unterstützen
