# API keys

Keys live as GitHub secrets and never in the repository, an image or the
server environment.

The harvest workflows read the secrets, fetch the protected data and publish it
as release assets on per-domain releases in the data repository. The Rust
server (Archivar) reads only those release assets and never sees a key. A
missing secret means the source is skipped, not filled.

## Where a secret goes

GitHub → Settings → Secrets and variables → Actions → New repository secret.

| Secret | Data file | Registration |
|---|---|---|
| `EBIRD_API_KEY` | `biosphere_ebird_recent.json` | https://ebird.org/api/keygen |
| `OPENAQ_API_KEY` | `atmosphere_openaq_pm25.json`, `atmosphere_openaq_pm10.json` | https://explore.openaq.org/register |
| `FIRMS_MAP_KEY` | `geosphere_firms_viirs_global_nrt.csv`, `geosphere_firms_modis_global_nrt.csv` | https://firms.modaps.eosdis.nasa.gov/api/area/ |
| `EARTHDATA_EDL_TOKEN` | `astro_nasa_cmr_*.json` via `cmr.earthdata.nasa.gov` | https://urs.earthdata.nasa.gov |
| `OMEGAFLOW_TOKEN` | release uploads to the data repository | a classic PAT with `public_repo` |

## The flow

1. Set the secrets above.
2. Actions → the harvest workflow for the source → Run workflow, or wait for the
   schedule.
3. The workflow creates or updates the per-domain releases.
4. The server reads the release assets as ordinary sources (`phi/sources.φ`).

## Local work

`cargo run` works without secrets. A protected source simply delivers no
samples until its release exists.
