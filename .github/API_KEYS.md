# API Keys für omegaflow

Keys werden ausschließlich als **GitHub-Secrets** hinterlegt und **niemals** in
Repo, Docker-Image oder Laufzeit-Environment des Servers geschrieben.

Der Workflow `.github/workflows/refresh-protected-data.yml` liest die Secrets,
holt die geschützten Daten von den APIs und veröffentlicht sie als Assets des
GitHub-Release-Tags `live-data` im Repo `omegaflow/catalogs`
(URLs: `https://github.com/omegaflow/catalogs/releases/download/live-data/<file>`).

Der Rust-Server (Archivar) lädt nur diese öffentlichen Release-Assets — er sieht
keinen API-Key. Fehlt ein Secret, wird die zugehörige Datenquelle schlicht
übersprungen (0 honored).

## Secrets anlegen

GitHub → Settings → Secrets and variables → Actions → New repository secret.

| GitHub-Secret | Platzhalter in sources.φ | Datenfile (Release) | Registrierung |
|---|---|---|---|
| `EBIRD_API_KEY` | — (via Release) | `ebird_recent.json` | https://ebird.org/api/keygen (Cornell-Account) |
| `OPENAQ_API_KEY` | — | `openaq_pm25.json`, `openaq_pm10.json` | https://explore.openaq.org/register |
| `FIRMS_MAP_KEY` | — | `firms_viirs_nrt_world.csv`, `firms_modis_nrt_world.csv` | https://firms.modaps.eosdis.nasa.gov/api/area/ → „Get MAP_KEY" (mit Earthdata-Login) |
| `EARTHDATA_EDL_TOKEN` | — | `cmr_GEDI02_A.json`, `cmr_ATL06.json`, `cmr_ATL03.json`, `cmr_SWOT_L2_LR_SSH_D.json`, `cmr_VNP14IMG_NRT.json` | https://urs.earthdata.nasa.gov → Profil → „Generate Token" |
| `CMEMS_USER` | — | `cmems_oxygen.json` | https://data.marine.copernicus.eu/register |
| `CMEMS_PASS` | — | (s.o.) | s.o. → persönlicher Token als Passwort |
| `CATALOGS_REPO` *(optional)* | — | — | `omegaflow/catalogs` (Standard) |
| `CATALOGS_TOKEN` | — | — | PAT mit `repo`/Contents-write auf `omegaflow/catalogs`, sonst fällt der Workflow auf `GITHUB_TOKEN` zurück (nur eigenes Repo) |

## Earthdata-Login (NASA)

Du hast bereits einen Earthdata-Login. Daraus erhältst du zwei Secrets:

1. **EARTHDATA_EDL_TOKEN** — CMR-Granuledaten (GEDI, ICESat-2 ATL06/ATL03, SWOT, VIIRS-Feuer):
   https://urs.earthdata.nasa.gov → im Profil unter „Developer Tools" → „Generate Token".
2. **FIRMS_MAP_KEY** — aktive Feuer (VIIRS/MODIS NRT):
   https://firms.modaps.eosdis.nasa.gov/api/area/ → „Get MAP_KEY" (mit dem Earthdata-Login einloggen).

## Ablauf

1. Secrets oben im GitHub-Repo setzen.
2. `Actions` → `Refresh Protected Data` → `Run workflow` (oder auf den 3-Stunden-Cron warten).
3. Der Workflow erstellt/aktualisiert das Release `live-data` auf `omegaflow/catalogs`.
4. Der Server lädt die Release-Assets als normale Sources (siehe `phi/sources.φ`).

## Lokale Entwicklung

Für lokale Tests genügt `cargo run`; die Release-Sources laufen auch ohne Secrets,
sobald einmal ein Release gepublished wurde. Falls eine geschützte Quelle nicht
gefüllt ist, liefert sie einfach keine Samples.
