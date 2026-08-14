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

`NASA_API_KEY`, `NASA_ADS_TOKEN`, `SPACETRACK_USER/PASS`, `SUPERMAG_USER`,
`CDS_API_KEY`, `JSOC_EMAIL`, `NOAA_CDO_TOKEN`, `TNS_API_KEY` (integriert, Vollkatalog),
`ZENODO_TOKEN`, `MATERIALS_KEY`, `EIA_API_KEY`, `PLANTNET_KEY`, `AIRNOW_KEY`,
`GFW_USER/PASS`, `OCEANNETWORKS_TOKEN`, `TRANSPORTDATA_KEY` (tedp_*), `SI_API_KEY`,
`IUCN_TOKEN`, `MOVEBANK_USER/PASS/TOKEN`, `CMEMS_USER/PASS`.

### ❌ Entfernt (4 — ToS verbieten Redistribution via CDN)
 
| GitHub-Secret | API | Ersatz |
|---|---|---|
| `WAQI_TOKEN` | WAQI (aqicn.org) | OpenAQ (PM2.5/PM10, Open Data) |
| `OWM_API_KEY` | OpenWeatherMap | Open-Meteo (keyless) |
| `ALPHAVANTAGE_KEY` | AlphaVantage | — (kein Äquivalent nötig) |
| `FRED_API_KEY` | FRED St. Louis Fed | — (kein Äquivalent nötig) |

CMEMS_USER/CMEMS_PASS liegen noch in `.secrets.local`, die Quelle selbst ist aber
retired (ERDDAP → NOAA/EMODnet umgestellt).

### ❌ Noch fehlend (7 — in `.secrets.local` absent)

| Secret | API | Force | Registrierung / Hinweis |
|---|---|---|---|
| `GBIF_USER`/`GBIF_PASS` | GBIF (Organismen-Beobachtung) | em | https://www.gbif.org/user/profile — Account existiert (2026-08-01); API keyless (3 req/s), Login hebt auf 100 req/s |
| `SUPERMAG_PASS` | SuperMAG (Magnetometer) | em | nur `SUPERMAG_USER` in secrets; Passwort aus der Signup-Mail |
| `PURPLEAIR_KEY` | PurpleAir (PM-Sensoren) | diffusion | https://develop.purpleair.com/ — Key erstellt 2026-06-23, im Portal abrufen |
| `ARBIMON_KEY` | Arbimon (Bioakustik) | acoustic | https://arbimon.rfcx.org/ — E-Mail verifiziert 2026-08-08 |
| `WILDLIFE_INSIGHTS_KEY` | Wildlife Insights (Kamera-Fallen) | em | https://www.wildlifeinsights.org/ — Account approved 2026-08-12 |
| `TRANSIT511_KEY` | 511.org Transit | advective | https://511.org/open-data/token |
| `TNG_KEY` | IllustrisTNG (Galaxien) | — | decline per Force Gate (Simulation, keine Messung) |

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
| **Arbimon** | `arbimon.rfcx.org` API | Token | frei | Acoustic | E-Mail verifiziert 2026-08-08 | https://arbimon.rfcx.org/ |
| **Wildlife Insights** | `www.wildlifeinsights.org` API | Key | frei | EM | approved 2026-08-12 | https://www.wildlifeinsights.org/ |

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

1. **Noch fehlende Keys besorgen** (siehe Secrets-Matrix oben) — konkrete Wege:

   | Secret | Wie besorgen |
   |---|---|
   | `GBIF_USER`/`GBIF_PASS` | Account existiert (2026-08-01): https://www.gbif.org/user → Passwort setzen → `GBIF_USER=code@omegaflow.space` + Passwort in `.secrets.local` |
   | `SUPERMAG_PASS` | Signup-Mail (2026-08-01, robin.barnes@jhuapl.edu) → Passwort eintragen (neben `SUPERMAG_USER=omegaflow`) |
   | `PURPLEAIR_KEY` | https://develop.purpleair.com → „API Keys" (Key wurde 2026-06-23 erstellt) → in `.secrets.local` |
   | `ARBIMON_KEY` | https://arbimon.rfcx.org → Login (E-Mail verifiziert 2026-08-08) → Account → API-Token |
   | `WILDLIFE_INSIGHTS_KEY` | https://www.wildlifeinsights.org → Login (approved 2026-08-12) → Profil → API-Key |
   | `TRANSIT511_KEY` | https://511.org/open-data/token → Token anfordern |
   | `TNG_KEY` | überspringen — decline per Force Gate (Simulation) |

2. Werte in `.secrets.local` eintragen (gitignored, keine Keys committen)
3. Workflow `refresh-protected-data.yml` erweitern — jede API als optionalen Step (überspringt wenn Secret leer)
4. Für jede Priorität-A-API Sources in `sources.φ` anlegen
5. Auth-Header/Query-Param im Fetch-System unterstützen
