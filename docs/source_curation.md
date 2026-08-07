# Source Curation: Complete Working Protocol

Self-contained. Interpretable by a session with zero prior context. Read this
before touching `phi/` source files.

## What this document is

The omegaflow Archivar loads two source-config files (see AGENTS.md for the
architecture). This document explains the source-curation workflow: how source
blocks are tested, classified, and added or rejected. It is the continuation
point for an ongoing effort to recover and verify data sources.

## The two source files

| File | Contents | Loading |
|------|----------|---------|
| `phi/sources_live.φ` | Live-API sources. The COMPLETE list of every source the Archivar can fetch directly. | Loaded by Archivar |
| `phi/sources_cdn.φ` | Only CDN-mirrored paths (URLs under `github.com/omegaflow/sources/releases/download/...`). | Loaded by Archivar |

Rule: `sources_live.φ` holds ALL sources (live AND CDN paths). `sources_cdn.φ`
holds only CDN paths. No source appears in both. The Archivar reads both files
and merges.

## Source block format

A block in the φ files:
```
url https://example.com/api/data?param={today}
ttl 3600
force em
on earth 48.2 16.4
map results
lat_key latitude
lon_key longitude
  field value my_measurement
```

Mandatory per block: `url`, `ttl`, `force`. The Archivar refuses blocks missing
force or frame at load.

### Force values (physical propagation mechanism)
`em`, `gravity`, `acoustic`, `seismic-body`, `seismic-surface`, `thermal`,
`diffusion`, `advective`, `biotic`.

### The Force Gate Principle (from AGENTS.md)
`force` declares the physical propagation of the MEASURED QUANTITY itself, not
the delivery medium. A stock price over HTTP has an EM carrier but no physical
force. Litmus test: could a non-human organism evolve a sensory organ for this
measurement? Categories automatically declined:
- Bare counts without individual event records (e.g. `count .` on a number)
- Station lists / registries / catalogs (metadata, no measurement)
- Model forecasts and reanalysis (GFS, ECMWF, CMEMS model analysis, open-meteo
  forecast) — no organism senses a model's output
- Reference constants (CODATA, PDG particle masses)
- Aggregate climate indices (AMO, MJO) — single scalar, no position
- Text warnings/alerts (SIGMET, TAF, storm warnings)
- Derived satellite products (tree-cover loss, DETER deforestation)
- Geographic infrastructure (rivers, boundaries, terrain DEM)

## Classification categories

When a source block is tested, it receives ONE of these dispositions, recorded
in `phi/dead_sources.φ` (for rejected) or added to `phi/sources_live.φ` (for
accepted):

| Category | Meaning | Where recorded |
|----------|---------|----------------|
| (accepted) | Reachable + correct force + parser can extract | `phi/sources_live.φ` |
| `dead` | Endpoint gone (404/500/DNS), maybe with fix note | `phi/dead_sources.φ` |
| `parser-def` | Endpoint alive but the Archivar parser cannot consume the format yet | `phi/dead_sources.φ` |
| `decline` | Force Gate: no physical force | `phi/dead_sources.φ` |
| `key-needed` | Free-registration API key required (not paid) | `phi/dead_sources.φ` |

Entry format in `phi/dead_sources.φ`:
```
dead 404
url <full-url-with-templates>
note <what failed, research hint>

parser-def votable-tap
url <url>
note <why parser-def>
```

## The curation workflow (per block)

1. Take a block from the pending list.
2. Fill URL templates with concrete values (`{today}`, `{year}`, `{lat}`, ...).
3. Test reachability: `curl -s -m 15 -o /dev/null -w '%{http_code}' -L <url>`.
   Also try the Jina proxy route when direct fails: prefix `https://r.jina.ai/`.
4. If reachable (200/206), inspect the response structure (JSON keys / CSV
   columns) to verify field paths are correct. A 200 HTML page is NOT data —
   many sites return an SPA shell instead of JSON.
5. Apply the Force Gate: does the measurement physically exist?
6. Classify and record (add to `sources_live.φ` if accepted, else
   `dead_sources.φ`).

Manual verification is required. `cargo check` only validates syntax.

## The pending list (current work)

The file `phi/recovery/pre_cdn_history/UNTESTED_blocks.φ` is the authoritative
list of source blocks not yet tested. Each has `url` + block lines. None of
these URLs appears in `dead_sources.φ`, `sources_live.φ`, or `sources_cdn.φ`.

Tracking mechanism:
1. Read `UNTESTED_blocks.φ`.
2. Diff each URL against `dead_sources.φ` + `sources_live.φ` + `sources_cdn.φ`.
3. Blocks not in any of those are still open. Test them with the workflow above.
4. When a block is tested, its disposition appears in one of the three files,
   so the next diff automatically shrinks the open set.
5. `phi/recovery/pre_cdn_history/UNTESTED_index.txt` lists remaining blocks by
   API domain for orientation.

## Why this work exists: the CDN-switch loss

Historically, all sources lived in a single `sources.φ` with direct API URLs.
A CDN migration replaced many URLs with `github.com/omegaflow/sources` asset
paths, and during merging thousands of original URLs and their extract
parameters were lost or replaced by guessed/fabricated endpoints.

The complete pre-CDN history is preserved in `phi/recovery/pre_cdn_history/`:
- 7 full `sources.φ` versions from git history (before CDN switch)
- `ALL_lost_blocks_richest.φ` — 5701 lost blocks with their richest extract
  parameters merged across history (fields, keys, frames)
- `lost_urls.txt` — all 5764 lost URLs
- `NEW_unchecked_blocks.φ` — arena-research blocks not present in history
- `UNTESTED_blocks.φ` — the still-open subset to curate

When curating, prefer the version with the richest extract (most `field` /
`path` / `*_key` lines) from `ALL_lost_blocks_richest.φ`. Compare the pending
block's field count against the historical version; if history is richer, use
history's extract. Some lost endpoints are now dead (servers moved or
restructured) — verify each.

## The Archivar parser (what it can consume)

The parser (in `src/main.rs`) was restored to full intelligence in commits
`188dd76` + `2cc6d5e`. It now supports 23 `Extract` variants including:
`Hapi` (HAPI JSON with data[]+parameters[]), `XmlCount` (XML tag counting),
`KeplerMap` (orbital elements → ICRS), `Vectors` (state vectors), `Ephemeris`.
This means previously-`parser-def` groups (e.g. VOTable TAP catalogs, HAPI
magnetometer feeds) may now be curatable.

Known good patterns:
- HAPI magnetometer (BGS InterMagnet, ESA Swarm): `format=json`, response
  `data[]` rows + `parameters[]`. Verified endpoints:
  `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=NGK/best-avail/PT1M/xyzf&start={yesterday}T00:00:00Z&stop={yesterday}T23:59:00Z&format=json`
  `https://vires.services/hapi/data?dataset=SW_OPER_MAGA_LR_1B&parameters=Latitude,Longitude,B_NEC&time.min=2026-08-03T00:00:00Z&time.max=2026-08-03T23:59:59Z`
- ERDDAP grid datasets: constraint syntax must match the current server (many
  batch URLs use outdated `[(last)]`/index forms that now 400).

## Secrets available

`phi/.secrets.local` (gitignored) holds working keys, loaded by `load_env()`.
`{SECRET_NAME}` in a URL template is replaced from env vars by the Archivar.
Notable: `FIRMS_MAP_KEY` (NASA fires), `OPENAQ_API_KEY`, `OCEANNETWORKS_TOKEN`,
`EARTHDATA_EDL_TOKEN`, `NASA_API_KEY`, `SPACETRACK_USER/PASS`, `CMEMS_USER/PASS`.

## Repository rules to honor

- Name = Implementation. TODO headings are identifiers, no numeric IDs.
- A commit closes/narrows/opens a TODO item; completed items are removed.
- `cargo check` zero errors AND zero warnings.
- Council decisions live in code, AGENTS.md, or TODO.md — no separate docs.
- The session is the atom: finish a complete, testable artifact each session.
