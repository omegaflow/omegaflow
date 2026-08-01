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

| API | Endpoint | Auth | Kosten | Force | Status |
|---|---|---|---|---|---|
| **NASA ADS** | `api.adsabs.harvard.edu/v1/search/query` | Bearer-Token | frei, 1 Min | EM | fehlt |
| **Space-Track.org** | `www.space-track.org/basicspacedata/query/class/satcat` | Login | frei | EM | fehlt (voller Sat-Katalog) |
| **SuperMAG** | `supermag.jhuapl.edu` | Login-Name | frei | EM | fehlt (~300 Stat.) |
| **GRACE-FO (PODAAC)** | `podaac.jpl.nasa.gov` S3-Bucket | Earthdata | frei | Gravity | fehlt |
| **SMAP Bodenfeuchte** | `nsidc.org/data/smap` | Earthdata | frei | Gravity | fehlt |
| **CDDIS IONEX** | `cddis.nasa.gov` | Earthdata | frei | EM | fehlt (Ionosphäre) |
| **GES DISC (GPM/MODIS)** | `gesdisc.eosdis.nasa.gov` | Earthdata | frei | EM/Thermal | fehlt |
| **NASA AppEEARS** | `appeears.earthdatacloud.nasa.gov` | Earthdata | frei | Thermal | fehlt |

## B. Mittel — erweitert vorhandene Daten

| API | Endpoint | Auth | Kosten | Force | Status |
|---|---|---|---|---|---|
| **MarineTraffic AIS** | `services.marinetraffic.com/api/` | Key | teils paid | Advective | fehlt Echtzeit |
| **Global Fishing Watch** | `globalfishingwatch.org/our-apis/` | Token | frei | Advective | fehlt |
| **Copernicus CDS/ADS** | `cds.climate.copernicus.eu/api` | CDS-Key | frei | Thermal | fehlt (ERA5) |
| **JSOC Sonne** | `jsoc.stanford.edu` | Registrierung | frei | EM/Thermal | fehlt (HMI) |
| **OpenWeatherMap Lightning** | `api.openweathermap.org/data/2.5/` | Key | frei-begrenzt | Acoustic | erweitert |
| **SAWS Südafrika** | `api.weathersa.co.za` | Registrierung | frei | Thermal | fehlt |
| **NOAA NCDC/CDO** | `www.ncdc.noaa.gov/cdo-web/api/v2/` | Token | frei | Thermal | fehlt |
| **NASA NEO Asteroids** | `api.nasa.gov/neo/rest/v1/` | Key | frei | Gravity | erweitert |
| **NASA Insight Mars** | `api.nasa.gov/insight_weather` | Key | frei | Thermal | fehlt |
| **NASA Mars Photos** | `api.nasa.gov/mars-photos` | Key | frei | EM | fehlt |
| **TNS Transient** | `www.wis-tns.org/api/` | Key | frei | EM | erweitert |
| **Zenodo** | `zenodo.org/api/records` | Token | frei | EM | erweitert |

## C. Nische — spezifischer Mehrwert

| API | Endpoint | Auth | Kosten | Force | Status |
|---|---|---|---|---|---|
| **Materials Project** | `api.materialsproject.org` | Key | frei | Diffusion | fehlt |
| **EIA Energie** | `api.eia.gov` | Key | frei | Thermal | erweitert |
| **AlphaVantage** | `alphavantage.co/query` | Key | frei | EM | fehlt |
| **FRED (St. Louis)** | `api.stlouisfed.org/fred` | Key | frei | EM | fehlt |
| **PlantNet** | `my-api.plantnet.org` | Key | frei | Diffusion | fehlt |
| **AirNow EPA** | `www.airnowapi.org` | Key | frei | Diffusion | fehlt |
| **OpenAQ** | `api.openaq.org/v3` | Key | frei | Diffusion | erweitert |
| **IAEA WISER (GNIP)** | `nucleus.iaea.org/wiser` | Registrierung | frei | Diffusion | fehlt Bulk |
| **IUCN Red List** | `apiv3.iucnredlist.org` | Token (demo) | frei | Diffusion | fehlt |
| **SCISAT** | `databace.scisat.ca` | Registrierung | frei | EM | fehlt |
| **Global Forest Watch** | `data-api.globalforestwatch.org` | Token | frei | Diffusion | fehlt |
| **Ocean Networks Canada** | `data.oceannetworks.ca` | Token | frei | Advective | fehlt |
| **Swiss OpenTransport** | `api.opentransportdata.swiss` | Key | frei | Advective | fehlt |
| **511.org Transit** | `api.511.org/transit/` | Key | frei | Advective | fehlt |
| **IllustrisTNG** | `tng-project.org/data/` | Key | frei | EM | fehlt (Galaxien) |
| **OpenTopography** | `portal.opentopography.org` | Demo-Key | frei | EM | erweitert |
| **Smithsonian Open Access** | `api.si.edu` | Key | frei | EM | fehlt |

## D. Niedrig — viel Aufwand, wenig Mehrwert

| API | Endpoint | Auth | Warum niedrig |
|---|---|---|---|
| **CNEMC China AQI** | `air.cnemc.cn:18007` | POST-only | GET-only Ingestor |
| **BOM v1 Weather** | `api.weather.bom.gov.au/v1/` | urheberrechtl. | FTP-Mirror reicht |
| **BOM Space Weather** | `sws-data.sws.bom.gov.au` | Key | UV als Text verfügbar |
| **ThingSpeak** | `thingspeak.com` | Key | unstrukturiert |
| **EarthNetworks** | `api.earthnetworks.com` | Key | paid |

---

## Empfehlung: Einmal-Registrierung, max. Quellen-Freischaltung

1. **NASA Earthdata-Login** (1 Account) → öffnet: GRACE-FO, SMAP, IONEX, GES DISC, AppEEARS, MODIS GPP = 6+ Gravity/Thermal-Quellen
2. **NASA API-Key** (api.nasa.gov, 1 Key) → DONKI, NEO, Insight, Mars Photos
3. **NASA ADS-Token** (1 Min) → Astro-Literatur

## Nächste Schritte

- Keys/Tokens in Konfiguration ablegen (env-Var oder phi-Konfig)
- Für jede Priorität-A-API Sources in `sources.φ` anlegen
- Auth-Header/Query-Param im Fetch-System unterstützen
