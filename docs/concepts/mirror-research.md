<!--
  title: OMEGAFLOW MIRROR-QUELLEN — abgeschlossene Recherche
  class: concept
  sha256: 6b6707920db5e6fbe16e8cbf8e7ff0213d145f0675372520b21b179ee7f67b10
-->
# OMEGAFLOW MIRROR-QUELLEN — abgeschlossene Recherche

## Zweck
Beantwortet: Müssen die 25 GitHub-Mirror-Quellen in der MIRRORS-Tabelle des Workflows
auf ihre "echten Original-Domains" umgestellt werden?

## Kurzantwort
Nur 1 von 25 Quellen konnte umgestellt werden (ourairports.com). Die anderen 24
müssen auf GitHub raw bleiben — entweder weil das GitHub-Repo DAS Original ist
(16 Fälle) oder weil die Originale nicht maschinenlesbar/blockiert sind (8 Fälle).

## Methodik
Jede Original-Domain wurde getestet:
1. Direkter HTTPS-Aufruf (von lokaler IP, Deutschland)
2. Über r.jina.ai-Proxy (US-IP, simuliert GitHub-Actions-Runner)
3. Prüfung auf maschinenlesbare Formate (JSON/CSV, nicht HTML)

## Ergebnistabelle

Legende:
- ✅ Erreichbar + maschinenlesbar → umstellbar auf Original
- ⚠️ Erreichbar aber HTML/Landingpage → NICHT umstellbar
- ❌ Blockiert / 404 / 500 → GitHub-Mirror zwingend notwendig
- 🏠 GitHub-Repo IST die Original-Quelle → NICHT umstellbar

| Quelle | Original-Domain | Status | Grund |
|--------|-----------------|--------|-------|
| Natural Earth (9 files) | naturalearthdata.com | ❌ 403 | Cloudflare blockt. Nur GitHub erreichbar |
| Natural Earth CDN | naciscdn.org | ❌ 403 | Selber Cloudflare-Schutz |
| GEM Active Faults | globalquakemodel.org | ⚠️ 200 | Nur Landingpage, keine GeoJSON-Daten |
| GEM Harmonized | github.com/cossatot | 🏠 | Repo ist Original (Fork des Autors) |
| WRI Power Plants | datasets.wri.org | ❌ 404 | Plattform migriert, Seite existiert nicht |
| WRI API | api.resourcewatch.org | ❌ 404 | API v1 gelöscht |
| PB2002 Plates | peterbird.name | ⚠️ 200 | Nur HTML-Homepage |
| PDG Particle | pdgapi.lbl.gov | ⚠️ 200 | Nur HTML-Tabelle, kein CSV |
| TidyTuesday Meteorites | rfordatascience/tidytuesday | 🏠 | Community-Datensatz, kein externes Original |
| GeoNuclearData | cristianst85/GeoNuclearData | ❌ 404 | File-Pfad im Repo geändert |
| Impact Craters | arijguest/3D-meteorite-viewer | 🏠 | Persönliches Repo (kein externes Original) |
| Meteorite Strikes | FreeCodeCamp/ProjectReferenceData | 🏠 | FCC-Übungsdaten |
| OURAirports | ourairports.com | ✅ | **Umgestellt** in Commit 45a8640 |
| D3 Celestial | ofrohn/d3-celestial | 🏠 | Bibliotheks-Repo |
| JSON Airports | jbrooksuk/JSON-Airports | 🏠 | Persönliches Repo |
| Periodic Table | Bowserinator/Periodic-Table-JSON | 🏠 | Persönliches Repo |
| NASA Meteorites | data.nasa.gov | ❌ 500 | Server-Fehler, braucht Key |
| eBird | api.ebird.org | ❌ 403 | Braucht Token (vorhanden im Secret) |
| Tectonic Plates | fraxen/tectonicplates | 🏠 | PB2002-Bearbeitung, Repo ist Original |
| ISC Seismology | isc.ac.uk | ❌ 404 | FDSN-Endpunkt umgezogen |

## Konsequenz für Release-Namen

Releases heißen `{fetch_domain}`, wobei `fetch_domain` die ACTUAL URL
ist, von der der Workflow fetched (der Release-Tag IST der Domain-Name):

| Fetch-Quelle | Release-Tag |
|-------------|-------------|
| `raw.githubusercontent.com/nvkelso/natural-earth-vector` | `raw.githubusercontent.com/nvkelso/natural-earth-vector` |
| `raw.githubusercontent.com/wri/global-power-plant-database` | `raw.githubusercontent.com/wri/global-power-plant-database` |
| `github.com/cossatot/gem-global-active-faults` | `github.com/cossatot/gem-global-active-faults` |
| `ourairports.com` | `ourairports.com` |

## Bereits umgesetzt

- Commit 45a8640: OURAirports von `raw.githubusercontent.com/davidmegginson/ourairports-data`
  auf `ourairports.com/data/airports.csv` umgestellt
- Workflow `refresh-protected-data.yml` MIRRORS-Tabelle aktualisiert

## Für die Domain-Migration relevant

Diese 25 Quellen sind Teil der ~2.300 CDN-Quellen, die auf Domain-Releases
umgestellt werden müssen. Ihre Release-Namen folgen der o.g. Tabelle —
NICHT `naturalearthdata.com` oder `globalquakemodel.org` (das wären Annahmen,
keine belegten Fetch-URLs).
