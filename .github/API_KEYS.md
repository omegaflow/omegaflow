# API Keys für omegaflow

Keys werden ausschließlich als **GitHub-Secrets** hinterlegt und **niemals** in
Repo, Docker-Image oder Laufzeit-Environment des Servers geschrieben.

Der Workflow `.github/workflows/refresh-protected-data.yml` liest die Secrets,
holt die geschützten Daten von den APIs und veröffentlicht sie als Release-Assets
auf domain-basierten Releases im Repo `omegaflow/sources`.

Der Rust-Server (Archivar) lädt nur diese öffentlichen Release-Assets — er sieht
keinen API-Key. Fehlt ein Secret, wird die zugehörige Datenquelle schlicht
übersprungen (kein Secret hinterlegt).

## Secrets anlegen

GitHub → Settings → Secrets and variables → Actions → New repository secret.

| GitHub-Secret | Platzhalter in sources.φ | Datenfile (Release) | Registrierung |
|---|---|---|---|
| `EBIRD_API_KEY` | — (via Release) | `biosphere_ebird_recent.json` | https://ebird.org/api/keygen (Cornell-Account) |
| `OPENAQ_API_KEY` | — | `atmosphere_openaq_pm25.json`, `atmosphere_openaq_pm10.json` | https://explore.openaq.org/register |
| `FIRMS_MAP_KEY` | — | `geosphere_firms_viirs_global_nrt.csv`, `geosphere_firms_modis_global_nrt.csv` | https://firms.modaps.eosdis.nasa.gov/api/area/ → „Get MAP_KEY" (mit Earthdata-Login) |
| `EARTHDATA_EDL_TOKEN` | — | `astro_nasa_cmr_*.json` via `cmr.earthdata.nasa.gov` Release | https://urs.earthdata.nasa.gov → Profil → „Generate Token" |
| `CATALOGS_REPO` *(optional)* | — | — | `omegaflow/sources` (Standard) |
| `OMEGAFLOW_TOKEN` | — | — | PAT mit `Contents-write` auf `omegaflow/sources`, sonst fällt der Workflow auf `GITHUB_TOKEN` zurück (nur eigenes Repo) |

## Ablauf

1. Secrets oben im GitHub-Repo setzen.
2. `Actions` → `Refresh Protected Data` → `Run workflow` (oder auf den 3-Stunden-Cron warten).
3. Der Workflow erstellt/aktualisiert die domain-basierten Releases auf `omegaflow/sources`.
4. Der Server lädt die Release-Assets als normale Sources (siehe `phi/sources.φ`).

## Lokale Entwicklung

Für lokale Tests genügt `cargo run`; die Release-Sources laufen auch ohne Secrets,
sobald einmal ein Release gepublished wurde. Falls eine geschützte Quelle nicht
gefüllt ist, liefert sie einfach keine Samples.
