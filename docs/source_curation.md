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
on mars 14.0 90.0
map results
lat_key latitude
lon_key longitude
  field value my_measurement
```
(`on mars` is one example; any body works — `on sirius 14.0 90.0` is the same
construct with a different body name.)

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

## SESSION HANDOFF (2026-08-07) — read this first

The Archivar parser was restored to FULL pre-CDN intelligence. This is the
single most important fact for the next session.

### Parser status (DONE)

- `src/main.rs` `extract_pending()` is the full pre-CDN 23-variant function
  (52,705 chars), restored verbatim in commits `188dd76` + `2cc6d5e`.
- All 23 `Extract` variants have extraction arms: Field, First, Last, Count,
  LastRow, LastObj, LastLine, ObjLast, GeojsonEvents, Path, Deep, Regex, Map,
  CelestialMap, Rows, Flatten, CmrPolygon, CelestialPolygon, KeplerMap, Hapi,
  XmlCount, Ephemeris, Vectors.
- All 8 formerly-scaffold directive arms are wired (`flatten`, `cmr_polygon`,
  `celestial_polygon`, `kepler_map`, `hapi`, `xml_count`, `ephemeris`,
  `vectors`).
- Helpers restored: `horizons_nums`, `ecliptic_to_field`,
  `flatten_geojson_coords`, `parse_horizons_ephemeris`, `tdb_to_jd`,
  `jfirst`, `jdeep_find_num`, `j2d_last_row`, `text_last_col`,
  `extract_regex_val` (hand-rolled std regex engine with char-class ranges),
  `parse_quoted_args`.
- `PendingPosition` uses the pre-CDN `Geodetic`/`GeodeticFlow` variants;
  the current-only `Surface`/`SurfaceFlow` variants were excised (dead).
- `cargo check`: zero errors AND zero warnings. `cargo test`: 13 passed.
- The `parse_json` entry point skips Jina metadata headers (commit `2bcbe79`).

### How to verify sources with the parser itself (the real verifier)

There is a diagnostic test `tests::test_live_sources_extract` in `src/main.rs`.
It loads BOTH φ files, substitutes URL templates with fixed dates (2026-08-07),
fetches each live URL via `fetch_one` (curl), runs `extract_pending`, and
counts sources whose extraction yields zero samples as FAIL.

Run: `cargo test test_live_sources_extract -- --nocapture`
Takes ~150s (network-bound, first 200 non-CDN sources). Output lines:
- `FAIL <url> no samples` — parser extracted nothing from a 200 response.
  This is the actionable list: the block's extract directives do not match
  the live response. Fix the block (compare against the golden version in
  `phi/recovery/pre_cdn_history/ALL_lost_blocks_richest.φ`).
- `source refused (pos without body directive)` — block uses data-carried
  `pos` but declares no `body`. Fix the SOURCE (add `body <body>` matching the
  source's world, or a proper frame), never invent a body in the parser.
- `source refused (no reference frame)` — block has no frame at all.
- `warning: map extract with non-surface frame` — map extract needs a
  terrestrial surface frame; the block uses `at sun`/barycenter. The golden
  sources used `wgs84` (→ `on <body>`); migration mislabeled them.

Diagnosis from 2026-08-07 run (200 live sources): 29 ok, 171 fail. Fail
reasons were dominated by `no samples` on raw.githubusercontent.com catalog
blocks (these are CDN candidates — the test should skip them like it skips
`github.com/omegaflow/sources`) and by the frame/refusal cases above. The
fixes are per-block, using the golden archive as reference.

### 2026-08-07 session results (93 -> 193 ok of 200)

The extraction test now reports **193 ok, 7 fail** (was 29 ok / 171 fail at
the start of the day). The single most impactful fix:

- **`field_in` vs `field` migration regression** (commit `9c16f8a`): the
  CDN-switch commit `fcd1468` renamed every `field_in` to `field`, but the
  parser only attaches `field_in` to `map`/`cmap`/`rows` extracts (the
  `field` arm pushes a top-level scalar extract). Map rows with empty field
  lists are all skipped -> 0 samples. Reverted across `sources_live.φ` and
  `sources_cdn.φ` (7130 line pairs). This fixed ~90 of the 107 FAILs by itself.

Remaining per-family fixes (commit `7cfe37e`):
- wmo buoys (`services6.arcgis.com`): live schema uses `_1_` fields
  (`sea_surface_temperature_1__degr`, `sea_water_speed_1__cm_s_1_`), NOT
  `_0_` as the older handoff claimed for a different host; stations
  42084/42091/42095/42098/42099 have NO `sea_water_speed` field at all.
  ArcGIS returns HTTP 200 WITH `{"error":{"code":400}}` on invalid outFields —
  that 200-with-error body yields zero features. Frames: `on earth 0 0`.
- swpc GOES flux feeds (`bb24b40`): positionless arrays (no lat/lon) —
  `map .` can never produce samples; use scalar `last flux` at the
  `at sun` frame. `solar_regions`/`ovation` carry real position (`lat_key
  latitude`/`0`, `lon_key longitude`/`1`). `alerts.json` is message strings
  only (no numeric measurement) -> declined.
- `api.weather.gov/stations`: `format=json` and `cursor=` params are
  unrecognized (400). Drop them.
- `tidesandcurrents`: `application=batch16` -> `omegaflow`; data is
  `{data:[{t,v,f}]}` at a fixed station — `map data` needs lat/lon, use
  scalar `last data.v` (dot path: parent `data` array, last element's `v`).
- `gracedb`: `last superevents.far` (dot path into the array).
- `NOAA_METAR` `f=json` variant: geometry is `{x,y}` not GeoJSON — `lat_key
  geometry.y`, `lon_key geometry.x`.

Declined / dead (documented in `phi/dead_sources.φ`): weather.gov alerts
(text warnings, Force Gate), volcano HANS routes (only `getUSVolcanoes`
exists and it is a static station catalog), blitzortung (binary/HTML),
emsc/tmd/dead NASA meteorite endpoint, heasarc integralburst (VOTable-only
TAP), ssd cad (positionless array-of-arrays, already CDN-mirrored), scedc
(text-rows only, no geodetic map), cdaweb/irsc (unreachable).

The 7 remaining FAILs are NOT source defects:
- 5 USGS earthquake feeds legitimately empty today (no M5.5+/pager/
  aftershock-forecast products, no quakes near Berlin in the last hour) —
  they produce samples when events occur.
- DONKI needs `NASA_API_KEY` in `phi/.secrets.local` (operational) and had
  no flares on the test date.
- 2 USGS `waterservices` bBox blocks use `{lon_min}...` presence-window
  templates the test cannot substitute (`bBox=,,,` -> 400); the Archivar
  substitutes them at runtime from the presence window (verified working
  with a real 1-degree window).

### The separate Python verifier (deprecated but kept)

`scripts/verify_sources.py` (tracked, default input `phi/sources_live.φ`)
is a JSON-path checker: it fetches each unique URL and compares declared
`field`/`path`/`lat_key`/`lon_key` keys against the live response. It handles
rate limits (per-host semaphore, jina fallback `https://r.jina.ai/`),
TAP/columnar metadata (`metadata`+`data`, `fields`+`data`), CSV/VOTable/HTML
headers, empty-container tolerance, and optional-field matching across array
elements. It is useful for bulk triage, but the parser test above is the
authoritative verifier. Command:
`python3 scripts/verify_sources.py phi/sources_live.φ --parallel 8 --host-max 2`

### Known API facts (learned, hard-won)

- HEASARC TAP: `FORMAT=csv`/`json` are NOT accepted for all catalogs — the
  integralburst query returned VOTable XML for both. Verify each catalog's
  accepted formats (`SELECT TOP 1 *` first); VOTable-only catalogs are
  `parser-def` (no VOTable row extract).
- ArcGIS wmo buoys (services6.arcgis.com/2DGR1sZBUvcPcd8Z): live schema is
  `sea_surface_temperature_1__degr` + `sea_water_speed_1__cm_s_1_` (index
  `_1_`), and some stations (42084/42091/42095/42098/42099) have NO
  `sea_water_speed` field. Query `outFields=*` to see a station's real
  schema before declaring outFields. An outFields naming a missing field
  returns HTTP 200 with `{"error":{"code":400}}` -> zero features.
  (Earlier handoff claimed `_0_` — that was a different host/era.)
- Open-Meteo split hosts: `archive-api.open-meteo.com`, `flood-api`,
  `marine-api`, `air-quality-api`, `ensemble-api` — never the
  `api.open-meteo.com/v1/<x>/v1/<x>` double-path form.
- BGS HAPI: requires `format=json`; station-specific `stop` dates (each
  station's last-data timestamp differs); `best-avail` has latency.
- tapvizier: broken queries reference columns not in the catalog
  (`AllWISE` etc.) — verify SELECT columns against `SELECT TOP 1 *`.
- NCEI `global-summary-of-the-*`: `countries=` 2-letter codes 400;
  `dataTypes=TEMP,DEWP,WDSP,P` truncated forms 400; daily data lags ~30 days.
- GBIF: `results` array is the map container (`map results`); `count` is
  top-level (`map .`). `stateProvince`/`occurrenceDate` are optional per
  record — absent in the first record is NOT a schema error.
- AviationWeather METAR/TAF/sigmet: flat arrays with `lat`/`lon` keys —
  do NOT rewrite to `geometry.coordinates`.

### Current source inventory (2026-08-07, end of day)

- `phi/sources_live.φ`: 1767 blocks. `phi/sources_cdn.φ`: 1770 blocks.
  Golden archives: `phi/recovery/pre_cdn.φ` (the reference),
  `phi/recovery/pre_cdn_history/ALL_lost_blocks_richest.φ` (5701 lost blocks
  with richest extract params), `sources_3_kopie.md`, `sources_4_kopie.md`,
  `sources_new.φ`, `sources_backup_20260719.φ` (copied from
  `~/Schreibtisch/Archiv` into `phi/recovery/`).
- `load_sources()` reads both φ files and merges (~3505 sources).
- Extraction test: **193 ok / 7 fail** of the first 200 non-CDN live sources
  (was 29 ok / 171 fail at the start of the day). See the session-results
  section above for what was fixed and why the 7 remaining FAILs are
  data-availability/test-coverage artifacts, not source defects.

### NEXT SESSION — EXPLICIT TASK (imperative, not context)

Your assignment: **drive the parser-verified source repair to zero FAILs and
curate the untested backlog.**

The 107-FAIL inventory from the previous handoff is RESOLVED (see session
results above). Current state: 193 ok / 7 fail of 200. The 7 remaining are
verified data-availability artifacts (empty USGS quake feeds, DONKI needs
`NASA_API_KEY` in `phi/.secrets.local`, and 2 USGS waterservices bBox
templates that only the runtime presence window can substitute).

DO THIS, IN ORDER:

1. Run `cargo test test_live_sources_extract -- --nocapture` and capture the
   full output. Confirm it is at/near 193 ok / 7 fail.
2. Raise the test's source limit (`let mut limit = 200usize;` in
   `test_live_sources_extract`) to the next chunk (e.g. 400) and iterate,
   fixing every new `FAIL <url> no samples` per the fix recipes in the
   session-results section above (they apply generically: positionless feeds
   -> scalar `last <dot.path>` at the frame; GeoJSON -> `map features` +
   `geometry.coordinates.N` + `on earth`/`body earth`; array-of-arrays with
   row positions -> index lat/lon keys; fixed-station scalar feeds -> `last
   <container>.<field>`; text-warning/metadata feeds -> decline under the
   Force Gate).
3. Also fix the load-time refusals: every `source refused (pos without body
   directive)` / `(no reference frame)` line in the test output names a block
   that loads zero samples. Add `body earth` / `on earth` / `at sun` as the
   source's world requires (never invent a body in the parser). Many are
   `raw.githubusercontent.com/omegaflow/catalogs` celestial catalogs that
   need `at sun 1` (barycentric) — or a working URL.
4. Then open `phi/recovery/pre_cdn_history/UNTESTED_blocks.φ`. For each block,
   apply the per-block curation workflow (fill templates -> curl -> structure ->
   Force Gate -> classify). Add accepted blocks to `phi/sources_live.φ`,
   rejected ones to `phi/dead_sources.φ`.
5. `cargo check` must stay at zero errors AND zero warnings throughout.
6. Commit per logical unit (TODO.md updated in the same commit).

This is the task. The context below is reference material you read as needed.

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
- Frame semantics (from parser `src/main.rs`): frames are body-agnostic —
  `on <body> <lat> <lon> [alt]` = fixed geodetic point on any body (e.g.
  `on mars`, `on sirius`, `on moon`); `at <body> <scale>` = barycentric frame
  of that body (e.g. `at sun 1.0`, `at sirius 1.0`); `body <body>` + `lat_key`/
  `pos` = DATA-CARRIED position (each record carries its own coords — GBIF,
  iNaturalist, USGS bBox). No body is privileged; the body name is just data.
  `body <planet>` in ephemeris/Horizons blocks declares the queried body (the
  COMMAND target), NOT a frame — keep it. `body {json}` on STAC/method=post
  blocks is a JSON POST payload, not a frame — keep it. Never mix `body` with
  `at`/`on` in the same block.
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
