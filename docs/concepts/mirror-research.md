<!--
  title: OMEGAFLOW MIRROR SOURCES — completed research
  class: concept
  sha256: 219948fa8ca841597e2930ec3215a41222991c3cf877977c884796bd0f6e63a5
-->
# OMEGAFLOW MIRROR SOURCES — completed research

## Purpose
Answers: do the 25 GitHub mirror sources in the MIRRORS table of the workflow
need to be switched to their "real original domains"?

## Short answer
Only 1 of 25 sources could be switched (ourairports.com). The other 24
stay on GitHub raw — either because the GitHub repo IS the original
(16 cases) or because the originals are not machine-readable/blocked (8 cases).

## Methodology
Each original domain was tested:
1. Direct HTTPS call (from local IP, Germany)
2. Via r.jina.ai proxy (US IP, simulates GitHub Actions runner)
3. Check for machine-readable formats (JSON/CSV, not HTML)

## Results table

Legend:
- ✅ Reachable + machine-readable → switchable to original
- ⚠️ Reachable but HTML/landing page → NOT switchable
- ❌ Blocked / 404 / 500 → GitHub mirror mandatory
- 🏠 GitHub repo IS the original source → NOT switchable

| Source | Original domain | Status | Reason |
|--------|-----------------|--------|-------|
| Natural Earth (9 files) | naturalearthdata.com | ❌ 403 | Cloudflare blocks. Only GitHub reachable |
| Natural Earth CDN | naciscdn.org | ❌ 403 | Same Cloudflare protection |
| GEM Active Faults | globalquakemodel.org | ⚠️ 200 | Only landing page, no GeoJSON data |
| GEM Harmonized | github.com/cossatot | 🏠 | Repo is the original (author's fork) |
| WRI Power Plants | datasets.wri.org | ❌ 404 | Platform migrated, page does not exist |
| WRI API | api.resourcewatch.org | ❌ 404 | API v1 removed |
| PB2002 Plates | peterbird.name | ⚠️ 200 | Only HTML homepage |
| PDG Particle | pdgapi.lbl.gov | ⚠️ 200 | Only HTML table, no CSV |
| TidyTuesday Meteorites | rfordatascience/tidytuesday | 🏠 | Community dataset, no external original |
| GeoNuclearData | cristianst85/GeoNuclearData | ❌ 404 | File path in the repo changed |
| Impact Craters | arijguest/3D-meteorite-viewer | 🏠 | Personal repo (no external original) |
| Meteorite Strikes | FreeCodeCamp/ProjectReferenceData | 🏠 | FCC exercise data |
| OURAirports | ourairports.com | ✅ | **Switched** in commit 45a8640 |
| D3 Celestial | ofrohn/d3-celestial | 🏠 | Library repo |
| JSON Airports | jbrooksuk/JSON-Airports | 🏠 | Personal repo |
| Periodic Table | Bowserinator/Periodic-Table-JSON | 🏠 | Personal repo |
| NASA Meteorites | data.nasa.gov | ❌ 500 | Server fault, needs key |
| eBird | api.ebird.org | ❌ 403 | Needs token (present in the secret) |
| Tectonic Plates | fraxen/tectonicplates | 🏠 | PB2002 processing, repo is the original |
| ISC Seismology | isc.ac.uk | ❌ 404 | FDSN endpoint moved |

## Consequence for release names

Releases are named `{fetch_domain}`, where `fetch_domain` is the ACTUAL URL
from which the workflow fetches (the release tag IS the domain name):

| Fetch source | Release tag |
|-------------|-------------|
| `raw.githubusercontent.com/nvkelso/natural-earth-vector` | `raw.githubusercontent.com/nvkelso/natural-earth-vector` |
| `raw.githubusercontent.com/wri/global-power-plant-database` | `raw.githubusercontent.com/wri/global-power-plant-database` |
| `github.com/cossatot/gem-global-active-faults` | `github.com/cossatot/gem-global-active-faults` |
| `ourairports.com` | `ourairports.com` |

## Already implemented

- Commit 45a8640: OURAirports switched from `raw.githubusercontent.com/davidmegginson/ourairports-data`
  to `ourairports.com/data/airports.csv`
- Workflow `refresh-protected-data.yml` MIRRORS table updated

## Relevant for the domain migration

These 25 sources are part of the ~2.300 CDN sources that need to be switched
to domain releases. Their release names follow the above table —
NOT `naturalearthdata.com` or `globalquakemodel.org` (those would be assumptions,
no verified fetch URLs).
