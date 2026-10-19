<!--
  title: Auth-APIs für omegaflow — vollständige Liste
  class: ref
  date: 2026-09-05
  sha256: 83c2776b6636e82a1ed7089171e300854d6dbb8c12266e6d9fa87c50ba4bd3dc
  status: live
-->
# Auth-APIs für omegaflow — vollständige Liste

Alle in den Forschungsdateien identifizierten APIs, die **Authentifizierung** erfordern.
Gruppiert nach Daten-Mehrwert für die 9-Kräfte-Punktwolke. Basis: Register.

**Stand 2026-09-03, Auth-Nachmessung 2026-10-19.** Dieses Blatt ist die **Registrierungs-/Lizenz-Sicht**: welche
Accounts existieren, welche Keys wo liegen, welche Registrierungen eine
CDN-Redistribution erlauben. Das verbindliche **Port-/Verdikt-Register** je Quelle
lebt in `phi/sources.φ` (live), `phi/sources.φ` (decline/dead, mit note) und
`phi/pipeline/ledger.φ` — dieses Blatt wird dort als `ref-auth-apis.md §E` zitiert,
es ist nicht selbst das Verdikt-Register. Wo eine Zeile hier einen Einzel-Endpoint-
Befund trägt, stammt er aus `phi/sources.φ`.

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

`NASA_API_KEY`, `NASA_ADS_TOKEN`, `BRAVE_API_KEY`, `SPACETRACK_USER/PASS`, `SUPERMAG_USER`
(API nutzt nur `user=`-Param, kein Passwort nötig), `CDS_API_KEY`, `JSOC_EMAIL`,
`NOAA_CDO_TOKEN`, `TNS_API_KEY` (integriert, Vollkatalog), `ZENODO_TOKEN`,
`MATERIALS_KEY`, `EIA_API_KEY`, `PLANTNET_KEY`, `AIRNOW_KEY`, `GFW_USER/PASS`,
`OCEANNETWORKS_TOKEN`, `TRANSPORTDATA_KEY` (tedp_*), `SI_API_KEY`, `IUCN_TOKEN`,
`MOVEBANK_USER/PASS/TOKEN`, `CMEMS_USER/PASS`, `GBIF_USER/PASS` (verifiziert,
3,9 Mrd. Vorkommen), `PURPLEAIR_KEY` (verifiziert, globale PM-Sensoren),
`TRANSIT511_KEY` (verifiziert, aber decline — Premium-Echtzeit + gzip + Registry).

`archive_search --github` liest seit 2026-09-15 `GH_SEARCH_TOKEN` (read-only,
fine-grained ohne Permissions) statt `OMEGAFLOW_TOKEN`; der Write-Token bleibt bei
der CI. `BRAVE_API_KEY` bedient `archive_search --brave`.

**Token-Rollen (gemessen 2026-09-17):**
- `OMEGAFLOW_TOKEN` — der **CI-Write**-PAT (classic, `repo`+`workflow`+`read:org`);
  die Workflows reichen ihn als Env-Var `GH_TOKEN` an `gh` weiter
  (`GH_TOKEN: ${{ secrets.OMEGAFLOW_TOKEN }}`). Seit 2026-09-17 ein **eigener**
  PAT (`omegaflow-ci-write`), getrennt vom lokalen `GH_TOKEN` (`omegaflow-write`) —
  die CI-Flotte und die lokalen Sessions haben damit getrennte 5000/h-Buckets.
  Der CI-PAT liegt auch in `.secrets.local` (`OMEGAFLOW_TOKEN`) — sein Bucket und
  seine Schreibrechte sind lokal messbar, ohne den Wert zu drucken:
  `GH_TOKEN="$(sed -n 's/^OMEGAFLOW_TOKEN=//p' .secrets.local)" gh api /rate_limit`
  bzw. `gh api /repos/omegaflow/sources --jq .permissions`. Ein CI-`403` ist damit
  gegen den aktuellen Token prüfbar — kein Operator-Punkt.
- `GH_SEARCH_TOKEN` — der **Read**-PAT (`.secrets.local`, fine-grained ohne
  Permissions), genutzt von `archive_search --github` und `ci_manage list|view`.
- `GH_TOKEN` — lokal (`.secrets.local` + gh-Keyring) der klassische PAT
  (`repo, workflow`), genutzt von `gh` und `ci_manage cancel|rerun`.
- Die Actions-Secrets `GH_TOKEN`/`GH_SEARCH_TOKEN` (von keinem Workflow genutzt)
  wurden 2026-09-17 entfernt.

Ein vorhandener Key **bedeutet kein Port-Verdikt**: einige besorgte Accounts führen
auf Quellen, die das Register als `decline` (kein Messwert, Modell, projekt-gebunden)
oder `dead` (Endpoint weg) beurteilt hat — siehe Status-Spalten unten und
`phi/sources.φ`.

### ❌ Entfernt (ToS verbieten Redistribution via CDN)

| GitHub-Secret | API | Ersatz |
|---|---|---|
| `WAQI_TOKEN` | WAQI (aqicn.org) | OpenAQ (PM2.5/PM10, Open Data) |
| `OWM_API_KEY` | OpenWeatherMap | Open-Meteo (keyless) |
| `ALPHAVANTAGE_KEY` | AlphaVantage | — (kein Äquivalent nötig) |
| `FRED_API_KEY` | FRED St. Louis Fed | — (kein Äquivalent nötig) |

CMEMS_USER/CMEMS_PASS liegen noch in `.secrets.local`, die Quelle selbst ist aber
retired (ERDDAP → NOAA/EMODnet umgestellt).

### ❌ Decline (Register-Verdikt, `phi/sources.φ`)

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

Gemessen aus `phi/sources.φ` (live), `phi/sources.φ` (Verdikt), 2026-09-03;
Auth-Nachmessung 2026-10-19 (nur HTTP-Status, keine Secret-Werte).
Dies ersetzt den früheren statischen Status-Kommentar: die Verdikt-Begründung je
Endpoint liegt in `phi/sources.φ` und ist dort nachschlagbar.

**Live in sources.φ (schon da / integriert):** Open-Meteo (Punkt-/Archiv-Stationen),
NOAA NCDC/CDO (thermal, P09-Fanout), OpenAQ (diffusion, P09-Fanout, v3/sensors),
TNS/WIS (em, Vollkatalog), PurpleAir (diffusion), NASA DONKI CME/FLR (em),
frost.met.no (thermal/advective), Ocean Networks Canada (`onc_*`, advective),
CDDIS IONEX (`ionex_tec_gim`, EM), GES DISC (`cosmic2_gnssro_l2b`, EM/Thermal),
PODAAC GRACE-FO/SWOT (Gravity; Earthdata-Header + S3-Credentials). Die ZTF-Richtungs-Transienten (ALeRCE, Lasair) sind pending — Richtung ohne Distanz, siehe §E.2 und TODO. Siehe auch §E.1.

**dead (Endpoint weg, `sources.φ`):** NASA-ADS-Subendpoints (`/v1/export/bibtex`),
EIA (`timeout`), SCISAT `databace.scisat.ca` (`404`), IAEA `nucleus.iaea.org/ILIS` (`404`),
Insight SEIS (`404`), IRSA-TAP/Scan, CEDA-CCI-SST-Browser, ICOS-meta. Live-Probe
2026-09-03 (aus §A/B/C nachgemessen): **JSOC** `jsoc.stanford.edu` (`dead timeout`,
curl 000 auf allen Pfaden) und **SAWS** `api.weathersa.co.za` (`dead origin-down`,
502/521) — beide in `phi/sources.φ` verbucht.
Auth-Nachmessung 2026-10-19, **nicht im Register**: **AirNow** (`410 Gone`,
Web-Service zum 2026-10-01 eingestellt) und **NASA Insight** (`api.nasa.gov/insight_weather`
timeout 000, auch mit `DEMO_KEY`). Frühere dead-Befunde zu PODAAC-S3 und GES-DISC
sind damit überholt: beide antworten 2026-10-19 mit Token (s. §A).

**decline (Verdikt, `sources.φ`):** Copernicus CDS/ERA5 (`model-forecast`),
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
| **NASA ADS** | `api.adsabs.harvard.edu/v1/search/query` | Bearer-Token | frei | EM | **live** (gemessen 2026-10-19: /v1/search/query = 200 mit Bearer; die 404 betreffen nur /v1/ und /v1/export/bibtex/; Port noch offen) | https://ui.adsabs.harvard.edu/user/settings/token |
| **Space-Track.org** | `www.space-track.org/basicspacedata/query/class/satcat` | Login | frei | EM | **live** (gemessen 2026-10-19: ajaxauth-Login 200 + satcat-Query 200 mit Credentials; Port noch offen) | https://www.space-track.org/auth/login |
| **SuperMAG** | `supermag.jhuapl.edu/services/data-api.php` | `logon=<username>` (passwortlos) | frei | EM | **funktioniert** (gemessen 10.09.2026: inventory/indices/data-api liefern echte Daten; User `omegaflow`; der frühere „geht nicht"-Befund war der falsche Parameter `user`/`username` statt `logon`) | https://supermag.jhuapl.edu/mag/ |
| **GRACE-FO / SWOT (PODAAC)** | `podaac.jpl.nasa.gov` S3-Bucket | Earthdata | frei | Gravity | **live** (gemessen 2026-10-19: s3credentials = 200, protected GRACE-FO-Datei = 303 auf signierte CloudFront-URL; der frühere 401 ‚client id missing' ist weg; Port noch offen) | https://urs.earthdata.nasa.gov/users/new |
| **SMAP Bodenfeuchte** | `nsidc.org/data/smap` | Earthdata | frei | Gravity | **live** (gemessen 2026-10-19: Portal 200, NSIDC-`s3credentials` = 200 mit EDL-Token; konkreter Bucket-Pfad 404, on-prem n5eil01u.ecs.nsidc.org DNS-tot → Port über S3 noch offen) | https://urs.earthdata.nasa.gov/users/new |
| **CDDIS IONEX** | `cddis.nasa.gov` | Earthdata | frei | EM | **live** (gemessen 2026-10-19: IONEX-Datei mit Token = 200; der frühere 404/client_id-Block ist weg; Register-Block `ionex_tec_gim` vorhanden) | https://urs.earthdata.nasa.gov/users/new |
| **GES DISC (GPM/MODIS)** | `gesdisc.eosdis.nasa.gov` | Earthdata | frei | EM/Thermal | **live** (gemessen 2026-10-19: `data.gesdisc.earthdata.nasa.gov` Verzeichnis + OPeNDAP-Base mit Token = 200; Register-Block `cosmic2_gnssro_l2b` vorhanden; GPM/MODIS-Port noch offen) | https://urs.earthdata.nasa.gov/users/new |
| **NASA AppEEARS** | `appeears.earthdatacloud.nasa.gov` | Earthdata | frei | Thermal | **blocked account** (gemessen 2026-10-19: /api/ 200, /api/product mit Token 403 — Zugriff auf die App-Ebene verweigert, kein Key-/Netzproblem) | https://urs.earthdata.nasa.gov/users/new |

## B. Mittel — erweitert vorhandene Daten

| API | Endpoint | Auth | Kosten | Force | Status | Registrierung |
|---|---|---|---|---|---|---|
| **MarineTraffic AIS** | `services.marinetraffic.com/api/` | Key | teils paid | Advective | **blocked key** (gemessen 2026-10-19: API = 400 `INVALID KEY`; kein Secret) | https://www.marinetraffic.com/en/ais-api-services |
| **Open-Meteo** | `api.open-meteo.com` | Keyless | frei | Thermal/Advective | schon da (Punkt-Stationen; ERA5-Modell-Subset decline) | — |
| **NOAA NCDC/CDO** | `www.ncdc.noaa.gov/cdo-web/api/v2/` | Token | frei | Thermal | integriert (P09-Fanout, live in sources.φ) | https://www.ncdc.noaa.gov/cdo-web/token |
| **NASA Insight Mars** | `api.nasa.gov/insight_weather` | Key | frei | Thermal | **dead** (gemessen 2026-10-19: timeout 000, auch mit `DEMO_KEY`; api.nasa.gov-Root 200, aber /insight_weather antwortet nicht) | https://api.nasa.gov/ |
| **TNS Transient** | `www.wis-tns.org/api/` + CSV-Staging | Key + User-Marker | frei | EM | integriert (Vollkatalog, csv_zip, live) | https://www.wis-tns.org/user/register |
| **NASA DONKI** | `api.nasa.gov/DONKI/*` | Key | frei | EM | schon da (CME/FLR live; NEO/Mars-Photos decline) | https://api.nasa.gov/ |

> **Entfernt aus B/C nach Register-Verdikt (Ersatz/Erklärung in `sources.φ`):**
> Copernicus CDS/ERA5 (`model-forecast`), Global Fishing Watch (`redistribution`),
> NASA NEO (`derived-orbit-fit`), NASA Mars-Photos (`imagery`), Zenodo (Endpoints dead 404,
> kein Messwert-Kanal), EIA (`timeout`/kein Messwert), **JSOC** (`dead timeout`, Live-Probe
> 2026-09-03), **SAWS** (`dead origin-down` 502/521, Live-Probe 2026-09-03). Früher als
> „Mittel" geführt, heute Verdikt-beschieden — nicht mehr Wunschliste, sondern
> `phi/sources.φ`.

## C. Nische — spezifischer Mehrwert

| API | Endpoint | Auth | Kosten | Force | Status | Registrierung |
|---|---|---|---|---|---|---|
| **Materials Project** | `api.materialsproject.org` | Key | frei | Diffusion | **live** (gemessen 2026-10-19: `/materials/summary` = 200 mit `MATERIALS_KEY`/`MP_API_KEY`; Port noch offen) | https://materialsproject.org/register |
| **AirNow EPA** | `www.airnowapi.org` | Key | frei | Diffusion | **dead** (gemessen 2026-10-19: HTTP 410, „web service has been retired as of October 1, 2026"; Key vorhanden, aber Endpoint weg) | https://docs.airnowapi.org/account/request/ |
| **OpenAQ** | `api.openaq.org/v3` | Key | frei | Diffusion | schon da (P09-Fanout, live; v2 → 410, v3/sensors) | https://api.openaq.org/register/ |
| **IAEA WISER (GNIP)** | `nucleus.iaea.org/wiser` | Registrierung | frei | Diffusion | **blocked account** (gemessen 2026-10-19: 302 → `websso.iaea.org`; kein Account, ILIS-API bleibt dead 404) | https://nucleus.iaea.org/wiser/ |
| **PurpleAir** | `api.purpleair.com/v1/` | Key | frei | Diffusion | schon da (live, `X-API-Key`-Header) | https://develop.purpleair.com/ |
| **Ocean Networks Canada** | `data.oceannetworks.ca` | Token | frei | Advective | **live** (gemessen 2026-10-19: `/api/locations` = 200 mit Token; mehrere `onc_*`-Blöcke im Register) | https://data.oceannetworks.ca/DataSearch |
| **Swiss OpenTransport** | `api.opentransportdata.swiss` | Key | frei | Advective | **blocked key** (gemessen 2026-10-19: `/ojp20` mit `tedp_ojp20` = 403 `Requested endpoint is forbidden`, Root `/` + `/api/` = 404; Port-Pfad/Berechtigung offen) | https://opentransportdata.swiss/en/dataset/ |
| **OpenTopography** | `portal.opentopography.org` | Demo-Key | frei | EM | erweitert (DEM30; Portal Live 200; API-Endpoints 405 method) | https://opentopography.org/ |

> **Entfernt aus C nach Register-Verdikt (siehe `phi/sources.φ` + Decline-Tabelle):**
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

Stand 2026-08-17, nachgeprüft 2026-09-13 — aus `phi/sources.φ` (key-needed)
gegen `.secrets.local` abgeglichen. Nachgeprüft: CEDA/ICOS/MAST tragen ihre
Secrets inzwischen (nicht mehr leer); TOAR ist descoped (redundant, s. u.).

**Grundsatz CDN-Redistribution:** Alle Messwerte werden auf das CDN (GitHub
Releases) manifestiert und öffentlich redistribuiert. Es kommen NUR APIs infrage,
deren Lizenz/ToS Redistribution erlauben (Public Domain, CC0, CC-BY, OGL/NLOD).
Kommerzielle/proprietäre APIs und solche mit Redistributionsverbot sind entfernt.
Das Urteil ist im Register vollzogen: die Hosts stehen in
`phi/sources.φ` / `phi/sources.φ` als
`decline redistribution` / `decline no-physical-force` (mit Verdikt-Note)
und in `phi/pipeline/interesting_domains.φ`.

| Host | Secret (Stub) | Auth | Lizenz | Registrierung |
|---|---|---|---|---|
| archive.opensearch.ceda.ac.uk | `CEDA_USER`/`CEDA_PASS` | Login (OpenID) | OGL/CC (UK) | https://archive.ceda.ac.uk/ (Token-API: services.ceda.ac.uk/api/token/create/) — Account + Token vorhanden (`.secrets.local` gefüllt, gemessen 2026-09-13) |
| data.icos-cp.eu | `ICOS_USER`/`ICOS_PASS` | Login | CC-BY 4.0 | https://data.icos-cp.eu/ (cpauth.icos-cp.eu — Account vorhanden) |
| gracedb.ligo.org | ~~`GRACEDB_TOKEN`~~ | ~~Auth~~ | offen (Alerts) | ~~https://gracedb.ligo.org/~~ — refused: Private-Events verlangt LVC/MOU-Gruppenmitgliedschaft, kein Self-Service; public superevents offen in sources.φ. |
| mast.stsci.edu | `MAST_TOKEN` | Token | Public Domain | https://mast.stsci.edu/ (TESS/HST/JWST-Photometrie, em — Token vorhanden) |
| toar-data.fz-juelich.de | `TOAR_USER`/`TOAR_PASS` (leer) | Login (Vollzugang) | CC-BY 4.0 | https://toar-data.fz-juelich.de/ (API v2 gemessen 10.09.: **anonym nutzbar**, Limit 5 Zeitreihen, `/database_statistics/` offen = 65,6 Mrd. Datensätze / 24.385 Stationen). **Registrierung nicht nötig** — TOAR ist die Aggregation derselben WMO-Stationsdaten, die WOUDC liefert (`atmosphere_woudc_total_ozone` integriert; Verdikt 2026-08-19 in `sources.φ`). Die parallele Anfrage lief weiter: Schröder-Antwort 2026-08-25, Projektbeschreibung/CC-BY angefragt. |

**§E.1 — integriert (live in `phi/sources.φ`):**

| Host | Secret | Quellen-Block | Was IS |
|---|---|---|---|
| frost.met.no | `FROST_BASIC_AUTH` | `frost_*` (air_temperature, wind_speed) | thermal (C) + advective (m/s), Stations-Fanout, Basic-Auth-Header. |
| lasair-ztf.lsst.ac.uk | `LASAIR_TOKEN` | ZTF-Transienten (em) | SQL-SELECT mit token, gmag-Fluss (`flux_from_mag`), 24h-Fenster über `{jd_start}`. **pending**: Richtung-only ohne Distanz (sources.φ, parser-def 2026-09-05). Token bleibt genutzt von lsst_anomaly_probe. |

> Zeilenverweise entfallen absichtlich: `phi/sources.φ` wächst; die Quellen-Blöcke
> sind über ihren Namen adressiert, nicht über eine brüchige Zeilennummer.

**§E.2 — Astronomische Transienten-Broker (LSST/Rubin community brokers):**

Stand 2026-09-05 — aus dem Review *An overview of astronomical transient brokers
in Rubin era* (CAOSP 55/2, pp95–105) plus Live-Verifikation der Zugangswege.
omegaflow nutzt die Broker im **historischen REST-Modus** (paginiert abfragbare
Daten), nicht den Echtzeit-Kafka-Stream. Kein Stream-Stub liegt in
`.secrets.local` (der Datenweg ist REST; Stream-Keys werden nicht genutzt).
URLs/Auth-Pfade sind NUR hier, nie in `.secrets.local`. (gemessen):

| # | Broker | Art | UI / API (gemessen 2026-09-10) | Auth | Status |
|---|---|---|---|---|---|
| 1 | **ALeRCE** | full | `science.alerce.online` · `api.alerce.online/ztf/v1/` (Doku `alerceapi.readthedocs.io`) | anonym | ✓ 200 |
| 2 | **AMPEL** | full | `ampelproject.github.io` · `ampel-ztf.zeuthen.desy.de/api/live/` | teils Token | ✓ 200 |
| 3 | **ANTARES** | full | `antares.noirlab.edu` · `api.antares.noirlab.edu/v2/` (**v2**, nicht v1; `/v2/helloworld/` → Hello World) | anonym | ✓ 200 |
| 4 | **Babamul** | full | `babamul.caltech.edu` · `babamul.caltech.edu/api/` („Greetings from BOOM!"; neu, arXiv:2511.00164) | OpenAPI 401 | ✓ 200 |
| 5 | **Fink** | full | `fink-broker.org` · `lsst.fink-portal.org` · `api.lsst.fink-portal.org/api/v1/` | anonym | ✓ 200 |
| 6 | **Lasair** | full | `lasair.lsst.ac.uk` · `api.lasair.lsst.ac.uk/` | Token (`LASAIR_LSST_TOKEN`/`LASAIR_TOKEN`) | ✓ 200 |
| 7 | **Pitt-Google** | full | `pitt-broker.readthedocs.io` · GCP (`mwvgroup.github.io/pittgoogle-client`) | GCP-Service-Account | ✓ Doku |
| 8 | **SNAPS** | down | `snaps.nau.edu` (Solar System; Trilling et al. 2023) | ? | ✓ 200 |
| 9 | **POI Broker** | down | `poibroker.uantof.cl` (variable stars) | ? | ✓ 200 |

**Die neun** (offizielle Rubin-Seite `rubinobservatory.org/for-scientists/data-products/alerts-and-brokers`, gemessen 2026-09-10): **7 full-stream** (ALeRCE, AMPEL, ANTARES, Babamul, Fink, Lasair, Pitt-Google) + **2 down-stream** (SNAPS, POI Broker). Die **Alerts sind world public, kein proprietäres Fenster** — die Broker-Daten sind offen, nur die RSP (Bilder/Kataloge) braucht Datenrechte. Korrekturen: ANTARES ist **v2**; **Babamul** ist wieder da (`babamul.caltech.edu`, arXiv:2511.00164 — die alte Zeile „nie entwickelt" war überholt); MOMENT/SNAD sind **nicht** in der offiziellen Neuner-Liste (MOMENT DNS-tot).

Die neun ZTF-Broker (Paper 2506.14744 „Technosignature Searches with Real-time Alert Brokers", arXiv:2506.14744). Korrekturen 2026-09-10: ANTARES ist **v2**; **Babamul** ist wieder da (`babamul.caltech.edu`, der öffentliche LSST-Broker aus arXiv:2511.00164 — die alte Zeile „nie entwickelt" war überholt); Lasair läuft auf `api.lasair.lsst.ac.uk`.

**§E.2.1 — Zugangs-/Auth-Pfad je Broker (Lese- und Stream-Weg getrennt):**

| Broker | Lese-Zugang (REST, genutzt) | Stream-Zugang (Kafka, NICHT genutzt — kein .secrets.local-Stub) |
|---|---|---|
| **ANTARES** | `https://api.antares.noirlab.edu/v1/` anonym, keyless | Key+Secret per E-Mail via `https://antares.noirlab.edu/support` |
| **Fink** | `https://api.ztf.fink-portal.org` / `https://api.lsst.fink-portal.org` anonym | Google-Formular `https://forms.gle/2td4jysT4e9pkf889` → E-Mail-Credentials |
| **AMPEL** | `https://ampel-ztf.zeuthen.desy.de/api/live/docs` (2 Endpunkte offen) | Bearer-Token: GitHub-Org `https://github.com/AmpelProject` + `https://ampel-ztf.zeuthen.desy.de/live/dashboard/tokens` |
| **Pitt-Google** | `https://mwvgroup.github.io/pittgoogle-client/` (GCP-Datasets) | GCP Service-Account (kein Broker-Konto) `https://mwvgroup.github.io/pittgoogle-client/one-time-setup/authentication.html` |
| **Lasair** | Token (Pflicht): `LASAIR_TOKEN`/`LASAIR_LSST_TOKEN` in .secrets.local; Registrierung `https://lasair-ztf.lsst.ac.uk/register/`, Token "My Profile" | Kafka `lasair_consumer` — ungenutzt |

ALeRCE (live) und die ANTARES-/Fink-Lese-REST-APIs brauchen **keinen** Key
(anonym). AMPEL: nur `/stock/ztf/{id}`-Pfade sind offen,
Rest 403 ohne Token; Root leitet auf Doku, `/live/dashboard` auf toten Port 8500,
das echte Dashboard liegt unter `/live/dashboard/tokens`. Pitt-Google hat kein
Login — Zugang ausschließlich über eigenes GCP-Projekt (Service-Account).

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

Stand 2026-09-03, Auth-Nachmessung 2026-10-19 — die Accounts der Schritte 1–3 sind
**bereits eingelöst** (siehe Secrets-Matrix); die Schritte benennen, welche vorhandenen
Accounts welche offenen Quellen in §A noch freischalten könnten.

1. **NASA Earthdata-Login** (1 Account, vorhanden) → Auth 2026-10-19 live für GRACE-FO,
   SMAP (S3), IONEX und GES DISC; AppEEARS bleibt account-blocked (403 App-Berechtigung).
   Ports weiterhin offen → https://urs.earthdata.nasa.gov/users/new
2. **NASA API-Key** (api.nasa.gov, 1 Key, vorhanden) → DONKI live; Insight ist
   gemessen 2026-10-19 dead (timeout) → https://api.nasa.gov/#signUp
3. **NASA ADS-Token** (vorhanden) → `v1/search/query` gemessen 2026-10-19 = 200,
   live; nur die Subendpoints (`/v1/`, `/v1/export/bibtex/`) sind 404. Port-Kandidat
   bestätigt, Port selbst noch offen.

## Nächste Schritte

1. **Register-Stand (2026-09-03):** Dieses Blatt ist auf das Register angeglichen.
   Offene, redistributions-konforme Registrierungen warten in §A/§C auf Port;
   Verdikt-beschiedene Quellen stehen in `phi/sources.φ`, nicht mehr hier als
   Wunsch.

2. Werte in `.secrets.local` eintragen (gitignored, keine Keys committen)
3. **Neue Quellen einbauen** (nächste Sessions) — aus §A (Earthdata-Bündel),
   Space-Track (EM-Katalog), SuperMAG (EM ~300 Stat.).
4. Workflow `health-check.yml` erweitern — jede API als optionalen Step (überspringt wenn Secret leer)
5. Auth-Header/Query-Param im Fetch-System unterstützen (PurpleAir braucht `X-API-Key`-Header, FROST Basic-Auth)
