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

The pending list lives in `archeology/sources/sources_*_untested_*` (the
pre-cdn-history tree was archived there):

- `sources_new_untested_14k_new-unchecked.φ` — 873 blocks
- `sources_astro_untested_30-astro.φ` — 30 blocks
- `sources_exotic_untested_neutrino-ligo.φ` — 16 blocks (no force → Gate)
- `sources_earth_untested_stac-sentinel.φ` — 3 blocks

None of these URLs appears in `phi/dead_sources.φ` or `phi/sources.φ`.

Tracking mechanism:
1. Read the four untested files above.
2. Diff each URL against `phi/dead_sources.φ` + `phi/sources.φ`.
3. Blocks not in either are still open. Test them with the workflow above.
4. When a block is tested, its disposition appears in one of the two files,
   so the next diff automatically shrinks the open set.
5. The former `UNTESTED_index.txt` was not archived — reconstruct the per-domain
   index from the four files (open work).

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
- `FAIL <url> no samples (empty-response (JSON parsed but all containers
  empty))` — the API legitimately returned no data (`features:[]`, `[]`,
  empty object). This is NOT a source defect; the source is correct and will
  produce samples when data exists. Only verify the extract against a
  data-bearing response (e.g. a wider time window).
- `FAIL <url> no samples (data-present (container array has rows but extract
  yielded nothing))` — the data IS there but the extract directives do not
  match. This is the actionable defect: fix the block (compare against the
   golden version in `archeology/sources/sources_recovery_cdn-merged_60k_lost-blocks.φ`
   joined with the richest parameter sets from `archeology/sources/sources_gold_pre-cdn_*`
   and `sources_recovery_pre-cdn_*`).
- `FAIL <url> no samples (data-present (keys exist ...))` / `(JSON has
  content but declared keys absent)` — data exists but keys/containers are
  wrong. Fix the block.
- `FAIL <url> no samples (data-present (non-JSON body: HTML/XML/text))` —
  the API returned a maintenance/error page or an XML/VOTable response the
  parser cannot consume (may be `parser-def` or a transient service outage).

The `diagnose_no_samples` function (commit `2094638`) performs this
classification; `json_has_content` counts only non-empty arrays as data
(meta numbers like `count:0` or `api:2.7` are not rows). Unit-tested in
`test_diagnose_no_samples`.

The verifier skips source classes that need dedicated fetch paths (2026-08-15):
`kernel_text` (data files, not field sources), `csv_zip` (byte path + inflate),
`fanout` (two-stage station fetch) — each skip is logged. Header-carrying
sources are fetched with `render_headers` (secrets substituted), so keyed
sources (PurpleAir X-API-Key, NOAA token) extract with their real headers.
- `source refused (pos without body directive)` — block uses data-carried
  `pos` but declares no `body`. Fix the SOURCE (add `body <body>` matching the
  source's world, or a proper frame), never invent a body in the parser.
- `source refused (no reference frame)` — block has no frame at all.
- `warning: map extract with non-surface frame` — map extract needs a
  terrestrial surface frame; the block uses `at sun`/barycenter. The golden
  sources used `wgs84` (→ `on <body>`); migration mislabeled them.
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

### 2026-08-07 session results (93 -> 194 ok of 200)

The extraction test now reports **194 ok, 6 fail** (was 29 ok / 171 fail at
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

The remaining FAILs are NOT source defects (commit `e19b47f` fixed the
DONKI key, the waterservices bBox blocks, and added presence-window
substitution to the test; the test now uses a Houston TX window so USGS
bBox queries resolve):

- 5 USGS earthquake feeds legitimately empty on 2026-08-07 (no M5.5+/
  pager/aftershock-forecast products, no M2+ near Houston in the last
  hour). Verified: `minmagnitude=6&orderby=magnitude` and `4.5_week`
  feeds return features; the extract directives (map features +
  geometry.coordinates.N + properties.mag) match. They produce samples
  when events occur.
- DONKI/FLR: `{NASA_API_KEY}` now resolves (resolve_secret matches
  case-insensitively), block extracts the latest X-class flare via
  `regex classType":"X...` (verified X1.1 -> 1.1). 2026-08-06 had no
  flares (`[]`) — legitimately empty.
- gracedb was returning 503 "scheduled maintenance" during the run
  (transient service outage, not a source defect); `last superevents.far`
  is correct.

Parser improvements landed in `e19b47f`:
- `resolve_secret` matches `{template}` case-insensitively against the
  UPPERCASE env vars (`{nasa_key}` -> `NASA_API_KEY`, `{firms_map_key}` ->
  `FIRMS_MAP_KEY`).
- `test_live_sources_extract` substitutes `{lon_min}/{lon_max}/{lat_min}/
  {lat_max}/{grid}/{nearest_station}` and uses a Houston TX window so
  USGS bBox presence-window queries actually exercise extraction.

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
  Golden archives: `phi/research/pre-cdn-1924-blocks.φ` (the reference),
  `archeology/sources/sources_recovery_cdn-merged_60k_lost-blocks.φ` (5701 lost blocks
  with richest extract params), `sources_3_kopie.md`, `sources_4_kopie.md`,
  `sources_new.φ`, `sources_backup_20260719.φ` (copied from
  `~/Schreibtisch/Archiv` into the archive).
- `load_sources()` reads both φ files and merges (~3505 sources).
- Extraction test: **194 ok / 6 fail** of the first 200 non-CDN live sources
  (was 29 ok / 171 fail at the start of the day). See the session-results
  section above for what was fixed and why the 7 remaining FAILs are
  data-availability/test-coverage artifacts, not source defects.

### NEXT SESSION — EXPLICIT TASK (imperative, not context)

Your assignment: **drive the parser-verified source repair to zero FAILs and
curate the untested backlog.**

The 107-FAIL inventory from the previous handoff is RESOLVED (see session
results above). Current state: 194 ok / 6 fail of 200. The 6 remaining are
verified data-availability artifacts (empty USGS quake feeds, DONKI needs
`NASA_API_KEY` in `phi/.secrets.local`, and 2 USGS waterservices bBox
templates that only the runtime presence window can substitute).

DO THIS, IN ORDER:

1. Run `cargo test test_live_sources_extract -- --nocapture` and capture the
   full output. Confirm it is at/near 194 ok / 6 fail.
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
4. Then open the four files under `archeology/sources/sources_*_untested_*`.
   For each block, apply the per-block curation workflow (fill templates -> curl ->
   structure -> Force Gate -> classify). Add accepted blocks to `phi/sources.φ`,
   rejected ones to `phi/dead_sources.φ`.
5. `cargo check` must stay at zero errors AND zero warnings throughout.
6. Commit per logical unit (TODO.md updated in the same commit).

This is the task. The context below is reference material you read as needed.

## Why this work exists: the CDN-switch loss

Historically, all sources lived in a single `sources.φ` with direct API URLs.
A CDN migration replaced many URLs with `github.com/omegaflow/sources` asset
paths, and during merging thousands of original URLs and their extract
parameters were lost or replaced by guessed/fabricated endpoints.

The pre-CDN history is archived in `archeology/sources/`:
- Full block versions from before the CDN switch: 5× gold, 12× main_cdn-merged,
  11× main_pre-cdn, 8× recovery_pre-cdn, 7× recovery_cdn-merged
- `sources_recovery_cdn-merged_60k_lost-blocks.φ` — 5701 lost URLs
  (extract parameters must be joined from the gold/recovery files)
- `sources_gold_pre-cdn_27k_359-domains.φ` (2572 blocks) and
  `sources_recovery_pre-cdn_25k_211-domains.φ` (1924 blocks) — the richest
  parameter sources (old `force` grammar, migrate per the Force Gate protocol)
- `sources_*_untested_*` — the still-open subset to curate

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
