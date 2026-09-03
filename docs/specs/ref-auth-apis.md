<!--
  title: Auth-APIs für omegaflow — vollständige Liste
  class: ref
  date: 2026-09-03
  sha256: 8a2aebc8ba5d4fc21179fbd846761a66ffbc29577797432dcdd7455003d8d495
  status: live
-->
# Auth-APIs für omegaflow — vollständige Liste

Alle in den Forschungsdateien identifizierten APIs, die **Authentifizierung** erfordern.
Gruppiert nach Daten-Mehrwert für die 9-Kräfte-Punktwolke. Basis: Register.

**Stand 2026-09-03.** Dieses Blatt ist die **Registrierungs-/Lizenz-Sicht**: welche
Accounts existieren, welche Keys wo liegen, welche Registrierungen eine
CDN-Redistribution erlauben. Das verbindliche **Port-/Verdikt-Register** je Quelle
lebt in `phi/sources.φ` (live), `phi/dead_sources.φ` (decline/dead, mit note) und
`phi/pipeline/ledger.φ` — dieses Blatt wird dort als `ref-auth-apis.md §E` zitiert,
es ist nicht selbst das Verdikt-Register. Wo eine Zeile hier einen Einzel-Endpoint-
Befund trägt, stammt er aus `phi/dead_sources.φ`.

## Legende

- **Auth-Typ**: Token / Key / Login / Registrierung / Bearer
- **Kosten**: frei = kostenlose Registrierung
- **Force**: welcher der 9 Forces die Daten zugeordnet wären
- **Status**: fehlt (nicht portiert) / erweitert / schon da (live in sources.φ) /
  decline (Register-Verdikt) / dead (Endpoint weg)

---

## Secrets-Matrix

Keys werden **niemals** in dieses Dokument oder das Repo geschrieben. Sie leben nur in:

1. **Lokal**: `.secrets.local` (gitignored) — Platzhalter für alle unten gelisteten Secrets
2. **GitHub Actions**: `Settings → Secrets and variables → Actions` (Workflow `health-check.yml`)

Die Verbuchung ist Stand 2026-08-14 (Kern-Keys eingelöst) und wurde am 2026-09-03
gegen `.secrets.local` nachgeprüft: alle unten benannten Stubs sind dort tatsächlich
vorhanden.

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

Ein vorhandener Key **bedeutet kein Port-Verdikt**: einige besorgte Accounts führen
auf Quellen, die das Register als `decline` (kein Messwert, Modell, projekt-gebunden)
oder `dead` (Endpoint weg) beurteilt hat — siehe Status-Spalten unten und
`phi/dead_sources.φ`.

### ❌ Entfernt (ToS verbieten Redistribution via CDN)

| GitHub-Secret | API | Ersatz |
|---|---|---|
| `WAQI_TOKEN` | WAQI (aqicn.org) | OpenAQ (PM2.5/PM10, Open Data) |
| `OWM_API_KEY` | OpenWeatherMap | Open-Meteo (keyless) |
| `ALPHAVANTAGE_KEY` | AlphaVantage | — (kein Äquivalent nötig) |
| `FRED_API_KEY` | FRED St. Louis Fed | — (kein Äquivalent nötig) |

CMEMS_USER/CMEMS_PASS liegen noch in `.secrets.local`, die Quelle selbst ist aber
retired (ERDDAP → NOAA/EMODnet umgestellt).

### ❌ Decline (Register-Verdikt, `phi/dead_sources.φ`)

| Secret | API | Verdikt |
|---|---|---|
| `ARBIMON_KEY` | Arbimon (Bioakustik) | projekt-gebunden (acoustic ja, Datenmodell passt nicht) |
| `WILDLIFE_INSIGHTS_KEY` | Wildlife Insights (Kamera-Fallen) | projekt-gebunden |
| `TNG_KEY` | IllustrisTNG (Galaxien) | Simulation, keine Messung |
| `PLANTNET_KEY` | PlantNet | Klassifikator per Bild, kein Sensor (`no-physical-force`) |
| `IUCN_TOKEN` | IUCN Red List | Arten-Katalog, keine Messung am Punkt (`presence-catalog`) |
| `SI_API_KEY` | Smithsonian | Sammlungs-Metadaten, kein Messwert (`no-physical-force`) |
| `GFW_USER/PASS` | Global Fishing Watch | ToS — Redistribution eingeschränkt |
| `TRANSIT511_KEY` | 511.org Transit | stops/operators = Registry; VehicleMonitoring = 401 Premium |
| `EIA_API_KEY` | EIA | Endpoints timeout / kein Live-Messwert |
| `CDS_API_KEY` | Copernicus CDS (ERA5) | Reanalyse-Modell, kein Messwert (`model-forecast`); `decline method` (HTTP 202) |

---

## Register-Anker: live / decline / dead je Quelle

Gemessen aus `phi/sources.φ` (live), `phi/dead_sources.φ` (Verdikt), 2026-09-03.
Dies ersetzt den früheren statischen Status-Kommentar: die Verdikt-Begründung je
Endpoint liegt in `phi/dead_sources.φ` und ist dort nachschlagbar.

**Live in sources.φ (schon da / integriert):** Open-Meteo (Punkt-/Archiv-Stationen),
NOAA NCDC/CDO (thermal, P09-Fanout), OpenAQ (diffusion, P09-Fanout, v3/sensors),
TNS/WIS (em, Vollkatalog), PurpleAir (diffusion), NASA DONKI CME/FLR (em),
frost.met.no (thermal/advective), Lasair ZTF (em). Siehe auch §E.1.

**dead (Endpoint weg, `dead_sources.φ`):** NASA-ADS-Subendpoints (`/v1/export/bibtex`),
PODAAC/NSIDC (S3-`scheme-unsupported` + DNS), GES-DISC (`404`, GLDAS = Modell),
EIA (`timeout`), SCISAT `databace.scisat.ca` (`404`), IAEA `nucleus.iaea.org/ILIS` (`404`),
Insight SEIS (`404`), IRSA-TAP/Scan, CEDA-CCI-SST-Browser, ICOS-meta. Live-Probe
2026-09-03 (aus §A/B/C nachgemessen): **JSOC** `jsoc.stanford.edu` (`dead timeout`,
curl 000 auf allen Pfaden) und **SAWS** `api.weathersa.co.za` (`dead origin-down`,
502/521) — beide in `phi/dead_sources.φ` verbucht.

**decline (Verdikt, `dead_sources.φ`):** Copernicus CDS/ERA5 (`model-forecast`),
Global Fishing Watch (`redistribution`), Global Forest Watch `data-api` (`derived-product`),
Smithsonian (`no-physical-force`), PlantNet (`no-physical-force`), IUCN (`presence-catalog`),
IllustrisTNG (`simulation`), NASA NEO (`derived-orbit-fit`), NASA Mars-Photos (`imagery`),
OpenTopography-API (`method`, 405), 511.org (`registry-premium-gzip`), arbimon/Wildlife
(`projekt-gebunden`).

---

## A. Höchster Mehrwert — Daten fehlen komplett

> Hinweis: „fehlt komplett" = Quelle nicht portiert. Ein vorhandener `.secrets.local`-Stub
> dazu ist vermerkt. Das Fehlen ist `pending` (Port-Dienst), kein fertiger Zustand.

| API | Endpoint | Auth | Kosten | Force | Status | Registrierung |
|---|---|---|---|---|---|---|
| **NASA ADS** | `api.adsabs.harvard.edu/v1/search/query` | Bearer-Token | frei | EM | fehlt (Key vorhanden; einzelne /v1-Subendpoints dead 404) | https://ui.adsabs.harvard.edu/user/settings/token |
| **Space-Track.org** | `www.space-track.org/basicspacedata/query/class/satcat` | Login | frei | EM | fehlt (Credentials vorhanden; Live 200, Query 401-auth) | https://www.space-track.org/auth/login |
| **SuperMAG** | `supermag.jhuapl.edu` | Login-Name | frei | EM | fehlt (~300 Stat., User vorhanden; Live 200) | https://supermag.jhuapl.edu/info/signup.php |
| **GRACE-FO / SWOT (PODAAC)** | `podaac.jpl.nasa.gov` S3-Bucket | Earthdata | frei | Gravity | fehlt (Earthdata vorhanden; S3-Scheme ungetragen) | https://urs.earthdata.nasa.gov/users/new |
| **SMAP Bodenfeuchte** | `nsidc.org/data/smap` | Earthdata | frei | Gravity | fehlt (Earthdata vorhanden; Live 200) | https://urs.earthdata.nasa.gov/users/new |
| **CDDIS IONEX** | `cddis.nasa.gov` | Earthdata | frei | EM | fehlt (Ionosphäre; Earthdata vorhanden; Live 200 → Earthdata-gated) | https://urs.earthdata.nasa.gov/users/new |
| **GES DISC (GPM/MODIS)** | `gesdisc.eosdis.nasa.gov` | Earthdata | frei | EM/Thermal | fehlt (griddap dead 404; GLDAS = Modell-decline) | https://urs.earthdata.nasa.gov/users/new |
| **NASA AppEEARS** | `appeears.earthdatacloud.nasa.gov` | Earthdata | frei | Thermal | fehlt (Earthdata vorhanden; Live 200) | https://urs.earthdata.nasa.gov/users/new |

## B. Mittel — erweitert vorhandene Daten

| API | Endpoint | Auth | Kosten | Force | Status | Registrierung |
|---|---|---|---|---|---|---|
| **MarineTraffic AIS** | `services.marinetraffic.com/api/` | Key | teils paid | Advective | fehlt (Live 401 key-gated; Root 404) | https://www.marinetraffic.com/en/ais-api-services |
| **Open-Meteo** | `api.open-meteo.com` | Keyless | frei | Thermal/Advective | schon da (Punkt-Stationen; ERA5-Modell-Subset decline) | — |
| **NOAA NCDC/CDO** | `www.ncdc.noaa.gov/cdo-web/api/v2/` | Token | frei | Thermal | integriert (P09-Fanout, live in sources.φ) | https://www.ncdc.noaa.gov/cdo-web/token |
| **NASA Insight Mars** | `api.nasa.gov/insight_weather` | Key | frei | Thermal | fehlt (SEIS dead 404; API Live 200, Port offen) | https://api.nasa.gov/ |
| **TNS Transient** | `www.wis-tns.org/api/` + CSV-Staging | Key + User-Marker | frei | EM | integriert (Vollkatalog, csv_zip, live) | https://www.wis-tns.org/user/register |
| **NASA DONKI** | `api.nasa.gov/DONKI/*` | Key | frei | EM | schon da (CME/FLR live; NEO/Mars-Photos decline) | https://api.nasa.gov/ |

> **Entfernt aus B/C nach Register-Verdikt (Ersatz/Erklärung in `dead_sources.φ`):**
> Copernicus CDS/ERA5 (`model-forecast`), Global Fishing Watch (`redistribution`),
> NASA NEO (`derived-orbit-fit`), NASA Mars-Photos (`imagery`), Zenodo (Endpoints dead 404,
> kein Messwert-Kanal), EIA (`timeout`/kein Messwert), **JSOC** (`dead timeout`, Live-Probe
> 2026-09-03), **SAWS** (`dead origin-down` 502/521, Live-Probe 2026-09-03). Früher als
> „Mittel" geführt, heute Verdikt-beschieden — nicht mehr Wunschliste, sondern
> `phi/dead_sources.φ`.

## C. Nische — spezifischer Mehrwert

| API | Endpoint | Auth | Kosten | Force | Status | Registrierung |
|---|---|---|---|---|---|---|
| **Materials Project** | `api.materialsproject.org` | Key | frei | Diffusion | fehlt (Key vorhanden; Live 200 → docs) | https://materialsproject.org/register |
| **AirNow EPA** | `www.airnowapi.org` | Key | frei | Diffusion | fehlt (Key vorhanden; Live 401 key-gated) | https://docs.airnowapi.org/account/request/ |
| **OpenAQ** | `api.openaq.org/v3` | Key | frei | Diffusion | schon da (P09-Fanout, live; v2 → 410, v3/sensors) | https://api.openaq.org/register/ |
| **IAEA WISER (GNIP)** | `nucleus.iaea.org/wiser` | Registrierung | frei | Diffusion | fehlt Bulk (ILIS-API dead 404; WISER 403 SSO-gated) | https://nucleus.iaea.org/wiser/ |
| **PurpleAir** | `api.purpleair.com/v1/` | Key | frei | Diffusion | schon da (live, `X-API-Key`-Header) | https://develop.purpleair.com/ |
| **Ocean Networks Canada** | `data.oceannetworks.ca` | Token | frei | Advective | fehlt (Token vorhanden; Live 200; einzelne ERDDAP-URLs dead 400) | https://data.oceannetworks.ca/DataSearch |
| **Swiss OpenTransport** | `api.opentransportdata.swiss` | Key | frei | Advective | fehlt (Root 404; Port-Pfad unverifiziert) | https://opentransportdata.swiss/en/dataset/ |
| **OpenTopography** | `portal.opentopography.org` | Demo-Key | frei | EM | erweitert (DEM30; Portal Live 200; API-Endpoints 405 method) | https://opentopography.org/ |

> **Entfernt aus C nach Register-Verdikt (siehe `phi/dead_sources.φ` + Decline-Tabelle):**
> AlphaVantage, FRED (ToS-removed / kein Messwert), PlantNet, IUCN, Smithsonian,
> Global Forest Watch, IllustrisTNG (decline), SCISAT (`databace` dead 404),
> 511.org (decline), Arbimon/Wildlife Insights (projekt-gebunden).

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
Das Urteil ist im Register vollzogen: die Hosts stehen in
`phi/dead_sources.φ` / `phi/pipeline/blocked_sources.φ` als
`decline redistribution` / `decline no-physical-force` (mit Verdikt-Note)
und in `phi/pipeline/interesting_domains.φ`.

| Host | Secret (Stub) | Auth | Lizenz | Registrierung |
|---|---|---|---|---|
| archive.opensearch.ceda.ac.uk | `CEDA_USER`/`CEDA_PASS` | Login (OpenID) | OGL/CC (UK) | https://archive.ceda.ac.uk/ (Token-API: services.ceda.ac.uk/api/token/create/) |
| data.icos-cp.eu | `ICOS_USER`/`ICOS_PASS` | Login | CC-BY 4.0 | https://data.icos-cp.eu/ (cpauth.icos-cp.eu — Account vorhanden) |
| gracedb.ligo.org | ~~`GRACEDB_TOKEN`~~ | ~~Auth~~ | offen (Alerts) | ~~https://gracedb.ligo.org/~~ — refused: Private-Events verlangt LVC/MOU-Gruppenmitgliedschaft, kein Self-Service; public superevents offen in sources.φ. |
| mast.stsci.edu | `MAST_TOKEN` | Token | Public Domain | https://mast.stsci.edu/ (TESS/HST/JWST-Photometrie, em — Token vorhanden) |
| toar-data.fz-juelich.de | `TOAR_USER`/`TOAR_PASS` | Login | offen | https://toar-data.fz-juelich.de/ (API v2 vorhanden — Registrierung pending 2026-08-17) |

**§E.1 — integriert (live in `phi/sources.φ`):**

| Host | Secret | Quellen-Block | Was IS |
|---|---|---|---|
| frost.met.no | `FROST_BASIC_AUTH` | `frost_*` (air_temperature, wind_speed) | thermal (C) + advective (m/s), Stations-Fanout, Basic-Auth-Header. |
| lasair-ztf.lsst.ac.uk | `LASAIR_TOKEN` | `lasair_ztf_*` | ZTF-Transienten (em), SQL-SELECT mit token, gmag-Fluss (`flux_from_mag`), 24h-Fenster über `{jd_start}`, Himmelskugel. |

> Zeilenverweise entfallen absichtlich: `phi/sources.φ` wächst; die Quellen-Blöcke
> sind über ihren Namen adressiert, nicht über eine brüchige Zeilennummer.

**Geklärt — offen ohne Key (aus key-needed entfernt, Stand 2026-08-17):**

| Host | Befund |
|---|---|
| alerce.online | ZTF-Database-API (`api.alerce.online/ztf/v1`) laut ALeRCE-Service-Doku „without needing any authentication" — offen, kein Token. `ALERCE_TOKEN`-Stub entfernt. |
| data.cosmic.ucar.edu | GNSS-RO/-R/Suominet laut UCAR „no need for a login to access these data"; curl 200 auf allen drei Pfaden. `CDAAC_USER`/`CDAAC_PASS`-Stubs entfernt. |
| climate-themetoffice.hub.arcgis.com | Met-Office-Climate-Data-Portal: „You do not need to log in to download the data"; ArcGIS-REST-FeatureServer + OGC-API-Records-Search-API, offen. `METOFFICE_CLIMATE_KEY`-Stub entfernt. |
| irsa.ipac.caltech.edu | Kein API-Token. Öffentliche Daten (WISE/NEOWISE/2MASS/Spitzer/SPHEREx/Euclid/ZTF-DR24) offen auf AWS S3 Open Data (anonymous). Proprietäres ZTF via IPAC-SSO. Verbleibt `decline redundant` — ZTF über Lasair/ALeRCE. `IRSA_AUTH`→`IRSA_USER`/`IRSA_PASS`. |
| marinecadastre.gov | Kein Live-API — AIS-Bulk-Downloads (CSV/GDB) auf Azure-Blob, Public Domain; AccessAIS-Bestellservice derzeit pausiert. `MARINECADASTRE_KEY`-Stub entfernt. |
| sios-svalbard.org | SIOS-User-Account nur für Working-Group/Committee/Katalog-Edit/SESS — Datenzugriff offen (Station-Data-REST `/rest/stations/data.json` curl 200). `SIOS_TOKEN`-Stub entfernt. |

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

Stand 2026-09-03 — die Accounts der Schritte 1–3 sind **bereits eingelöst** (siehe
Secrets-Matrix); die Schritte benennen, welche vorhandenen Accounts welche offenen
Quellen in §A noch freischalten könnten.

1. **NASA Earthdata-Login** (1 Account, vorhanden) → öffnet: GRACE-FO, SMAP, IONEX,
   GES DISC, AppEEARS, MODIS GPP = 6+ Gravity/Thermal-Quellen (§A; Port offen)
   → https://urs.earthdata.nasa.gov/users/new
2. **NASA API-Key** (api.nasa.gov, 1 Key, vorhanden) → DONKI live; Insight (§A)
   → https://api.nasa.gov/#signUp
3. **NASA ADS-Token** (vorhanden) → ein `/v1`-Subendpoint (export/bibtex) ist dead 404;
   der Such-Pfad `v1/search/query` ist der Port-Kandidat → Verifikation in einer
   Port-Session nötig.

## Nächste Schritte

1. **Register-Stand (2026-09-03):** Dieses Blatt ist auf das Register angeglichen.
   Offene, redistributions-konforme Registrierungen warten in §A/§C auf Port;
   Verdikt-beschiedene Quellen stehen in `phi/dead_sources.φ`, nicht mehr hier als
   Wunsch.

2. Werte in `.secrets.local` eintragen (gitignored, keine Keys committen)
3. **Neue Quellen einbauen** (nächste Sessions) — aus §A (Earthdata-Bündel),
   Space-Track (EM-Katalog), SuperMAG (EM ~300 Stat.).
4. Workflow `health-check.yml` erweitern — jede API als optionalen Step (überspringt wenn Secret leer)
5. Auth-Header/Query-Param im Fetch-System unterstützen (PurpleAir braucht `X-API-Key`-Header, FROST Basic-Auth)
