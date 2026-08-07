# TODO

AGENTS.md is the primary constraint matrix. Git is the history. This file contains only pending work.

## Autonomous Biodiversity Sensing

Council decision 2026-08-07: biodiversity observation APIs (GBIF, iNaturalist, OBIS) measure human database records, not physical field quantities — forceless under the Force Gate. Genuine biophysical biodiversity measurements (camera traps, eDNA sequencers, bioacoustic monitors, chlorophyll fluorescence) carry force. Sources exist for acoustic (xeno-canto, passive acoustic monitoring) and satellite-derived (chlorophyll, NDVI). Open them when available.

---

## Council Agent Versioned (2026-08-07)

`.opencode/agent/council.md` and `.opencode/command/council.md` are now versioned (skip-worktree). Verify a fresh clone invokes the council agent with the same behavior.

---

## Collapse CDN/Live split → single `sources.φ`

SUPERSEDES "Sources Split: CDN / Live" — council verdict 2026-08-07. After
the CI Archivar populates the CDN (session 4 of the dual-mode roadmap),
`sources_cdn.φ` and `sources_live.φ` collapse into a single `sources.φ`. The
CDN/live distinction becomes a runtime decision: the Archivar tries CDN first
for every source, falls back to live API. See `docs/dual_mode_architecture.md`
for the full roadmap.
Code: `phi/sources_live.φ`, `src/main.rs:2644`

---

## Touchpad / Touch Control

Spec: `docs/concepts/INTUITIVE TOUCHPAD- & TOUCH-STEUERUNG.md`. Diagonal pinch → zoom. 2-finger horizontal → time. 2-finger vertical → spatial forward/back. `pointers.size < 3` condition in tThrust reset → `< 2`.
Code: `static/index.html` — pointer event handlers, tThrust reset condition, gesture state machine.

Action: Implement 2-finger gesture handling: track initial separation and center for pinch zoom, horizontal delta for time thrust, vertical delta for spatial thrust. Change tThrust reset from `pointers.size < 3` to `< 2` so single-finger lift doesn't reset temporal velocity.
Verification: Two-finger pinch zooms in/out (scale changes). Two-finger horizontal swipe changes time. Two-finger vertical swipe moves forward/back. Single-finger lift does not reset tThrust. `cargo check` clean. Open browser at `127.0.0.1:1111`, confirm gestures work.

---

## Archivar Dual-Mode Architecture — 5-Session Roadmap

SUPERSEDES "Archivar `{latest}` Resolver" and "CDN Asset Renaming". Council
verdict 2026-08-07 (unanimous, 5 voices). The `{latest}` resolver and CDN
renaming are unnecessary when the CI Archivar writes timestamped snapshots to
the CDN — the naming convention IS the resolver. The full architecture is
described in `docs/dual_mode_architecture.md`.

### Session 1: CI-mode flag + naming convention + local file output — DONE
Commit: upcoming. `src/main.rs:5791-5894` — `ci_mode_main()`, `extract_netloc`,
`source_name_from_url`, `utc_iso8601_now`, `file_fresh`. CLI dispatch at
`main():5896-5903`. Uses `{name}.json` (no timestamp) for Session 1 freshness.
Verified: `cargo check` 0 errors 0 warnings, 14/14 tests, `--ci-mode` writes
to `out/{netloc}/{name}.json`. `out/` added to `.gitignore`.
Next session entry point: extend `ci_mode_main()` to call `gh release upload`.

### Session 2: CDN upload integration — DONE
Commit: upcoming. `src/main.rs:5795-5859` — `gh release create` + `gh release
upload --clobber` after each successful write. Token guard: skips if neither
`GH_TOKEN` nor `OMEGAFLOW_TOKEN` set. Upload counter in log line.
`.github/workflows/mirror-cdn.yml` — `cargo run -- --ci-mode` on push +
`*/5 * * * *` schedule. Verified: `cargo check` 0/0, 14/14 tests, local run
without token skips uploads gracefully.
Next session entry point: modify `fetch_one` for CDN-first TTL-fallback.

### Session 3: Local CDN-first fetch with TTL-fallback — DONE
Commit: upcoming. `src/main.rs:2203-2258` — `fetch_raw` (pure curl), `fetch_one`
(CDN-first wrapper: try `releases/download/{netloc}/{name}.json`, fall through
to `fetch_raw`). `ci_mode_main()` uses `fetch_raw` directly (CI is the writer).
`warm_cache` consumer: CDN pre-scan at `:5567-5585` (sequential CDN tries per
chunk before spawn loop), pre-filled tasks skip `spawn_task_curl` and are
processed at `:5588-5650`. No TTL comparison needed — CI cadence (TTL/Φ)
guarantees CDN freshness. `cargo check` 0/0, 14/14 tests.
Next session entry point: collapse `sources_cdn.φ` into `sources_live.φ`.

### Session 4: Collapse source files — DONE
Code: `phi/sources_cdn.φ` + `phi/sources_live.φ` → `phi/sources.φ` (3510 blocks
= 1770+1740). `load_sources()` at `src/main.rs:2792` reads single
`phi/sources.φ`. `main()` eprintln updated. `cargo check` 0/0, 14/14 tests.
Next session entry point: excise Python CDN scripts, only Rust CI remains.

### Session 5: Excise Python CDN scripts
Delete: `migrate_live_to_cdn.py`, `tap_to_cdn.py`, `shard_catalog.py`,
`refresh_catalogs.py`, `restore_all_live.py`. Keep `generate_ephemerides.py`
(SPICE) and `verify_sources.py` (audit). CI workflow only runs
`cargo run -- --ci-mode` + ephemeris generation.

### CI Script URL-First Migration

`tap_to_cdn.py`, `refresh_catalogs.py`, `restore_all_live.py` parse with `source <name>` lines as block anchors. The file is url-first now.
Code: `scripts/tap_to_cdn.py`, `scripts/refresh_catalogs.py`, `scripts/restore_all_live.py`.

Action: `tap_to_cdn.py` and `refresh_catalogs.py`: Replace `re.split(r"\n(?=source )", content)` with `content.strip().split('\n\n')`. Derive name from URL path. `restore_all_live.py`: Remove `_current_names_from_urls()` `{latest}` strip logic (no longer needed after URL repair).
Verification: Run each script against current `phi/sources.φ`. No parse errors. Correct source names extracted. `grep -c '{latest}' phi/sources.φ` → 0 (URLs already repaired or resolver handles them).

---

## New Extract Types: Kepler, Horizons, Flatten

All touch `src/main.rs` `Extract` enum and materialize pipeline.

### Kepler Exoplanet Map Parser

Kepler/NASA Exoplanet Archive data. New extract type or extends existing `Rows`/`Map` with orbital parameter mapping.
Code: `src/main.rs` — new extract variant in `Extract` enum, parsing in `load_sources()`, materialization in `materialize()`.

Action: Add Kepler-specific extract handling: parse orbital elements (period, semi-major axis, eccentricity, inclination) from table rows. Map to ICRS position via orbital mechanics (Kepler's equation). Output as oscillators with motion law derived from orbital parameters.
Verification: Kepler source block parses without error. Oscillators carry correct ICRS positions. `cargo check` clean. Verify a known exoplanet (e.g., Kepler-22b) appears at reasonable ICRS coordinates.

### Vectors/Horizons Text Parser

JPL Horizons `vec` output format. Space-fixed state vectors. New extract type.
Code: `src/main.rs` — new extract variant parsing Horizons text format, extracting `[x, y, z, vx, vy, vz]` from `$$SOE`/`$$EOE` blocks.

Action: Add `Extract::HorizonsVec` variant. Parse the Horizons text format: find `$$SOE`, read state vector lines (X/Y/Z + VX/VY/VZ in km and km/s), parse epoch from header. Scale to meters. Output as `Motion::Linear { p, v }`.
Verification: Horizons `vec` format source parses without error. State vectors converted to meters. `cargo check` clean.

### Flatten Extract Type

Array-of-arrays JSON responses where each inner array is a row (no key mapping). Flatten without field name mapping.
Code: `src/main.rs` — new `Extract::Flatten` variant.

Action: Add `Extract::Flatten` extract type that takes a flat array-of-arrays and maps by index position (0→field_0, 1→field_1, ...) or by declared field name order.
Verification: Source using `extract flatten` parses nested arrays into oscillator fields. `cargo check` clean.

---

## Field Infrastructure

All touch `src/main.rs` field parsing and bounding logic.

### `field_in` Nested Support

`field_in` (line 3025) currently annotates only the last extract declared. A nested path like `field_in temperature data.main.value` should traverse into nested extracts.
Code: `src/main.rs` — `load_sources()` field_in handling, nested extract lookup.

Action: Extend `field_in` parsing to support `.`-delimited nested paths. When an extract declares an inner sub-extract (e.g., `map` inside `map`), `field_in` can target a field within a named inner extract. Parse syntax: `field_in <key> <extract_name>.<field_name>` or `field_in <key> <nested.path>`.
Verification: Source with nested JSON structure parses fields from inner maps correctly. Existing flat `field_in` continues working. `cargo check` clean.

### Extent Per Force-Type Verifiability

`force_extent()` returns hardcoded spatial extent values per force type. These should be verifiable against live data — do oscillators of a given force type actually spread within the declared extent?
Code: `src/main.rs` — `force_extent()` function, no runtime verification exists.

Action: Add verification: during fetch, compare per-source oscillator spatial spread against `force_extent()` value. If actual spread exceeds declared extent by > 2×, emit warning. Add `validate_extent` flag to source config (opt-in). This is a query-properties task — the extent emerges from the data, not the declaration.
Verification: Warning emitted when actual oscillator spread exceeds declared extent. No change to rendering — verification only. `cargo check` clean.

### Window/Temporal `from`/`until` Bounding

The presence window currently has no temporal or spatial aperture limits. The sensor should be able to bound what it sees in time and space.
Code: `src/main.rs` — sense_buffer, presence frame, WebSocket response. `static/index.html` — window state.

Action: Add `t_from` / `t_until` temporal bound parameters to the presence frame (deep-linkable via `#x,y,z,t,tFrom,tUntil` or separate controls). Add `x_from/x_until/y_from/y_until` spatial bounds (frustum crop). Oscillators outside temporal/spatial window are excluded from the response array. Bounds are properties of the sensor (query properties emerge from the observer), not the oscillators.
Verification: Deep-link with temporal bounds excludes oscillators outside the range. Spatial bounds exclude oscillators outside the frustum. Bounds are reset on `s` (halt) or explicit clear. `cargo check` clean. Render confirms bounded window.

---

## Command Palette (⌘K)

Keyboard-driven command palette for operator actions: deep-link navigation, force visibility toggle, window scale, temporal velocity control.
Code: `static/index.html` — new overlay element, keystroke handler, command registry.

Action: Implement ⌘K palette overlay with fuzzy-search command list. Commands: `goto <body>`, `scale <value>`, `thrust <value>`, `halt`, `toggle <force>`, `jump <time>`. Each command maps to existing omegaflow state mutations. Palette closes on Escape or command execution.
Verification: ⌘K opens palette. Typing filters commands. Enter executes. Escape closes. Commands modify omegaflow state correctly. No regression in existing keyboard shortcuts. `cargo check` clean.

---

## Minkowski 4D Weighting

WGSL kernel refinement. Touches `static/index.html` WGSL shader.

The current temporal fold uses `e^(−max(0, |Δt| − d/c) / ttl)` — Euclidean space + absolute time. A proper Minkowski kernel replaces the `max(0, |Δt| − d/c)` with proper-time interval: `Δs² = d² − c²·Δt²`. Spacelike-separated oscillators (Δs² > 0) are weighted differently than timelike-separated (Δs² < 0).
Code: `static/index.html` — WGSL `fold_eff` function, vertex shader temporal weighting.

Action: Modify the WGSL temporal fold to compute proper-time interval `Δs² = d² − c²·Δt²` (in the presence frame). Timelike separation (`Δs² < 0`): weight by `exp(−√(−Δs²) / (c·ttl))`. Spacelike separation (`Δs² > 0`): weight falls off as `exp(−√(Δs²) / (c·ttl))`. This is a refinement of the existing fold — the Lorentz structure is already implicit in `d/c` light-travel retardation. The new kernel makes the physics explicit.
Verification: Known test case: oscillator at 299792 km distance, 1 second ago — Δs² ≈ 0 (lightlike), weight ≈ 1. Oscillator at same point, 1 second ago — timelike, normal decay. Oscillator at 600000 km distance, 1 second ago — spacelike, extra decay from spacelike interval. `cargo check` clean. Render window shows visibly different weighting for distant vs. recent oscillators.

---

## Data Pipeline: Audit + TAP + Priority-A

### 96 Audit Findings

`scripts/api_audit.py` output (`phi/api_audit.jsonl`) identifies 96 issues across sources: key-missing, not-json, fetch-fail.
Code: `phi/sources.φ` (source block repairs), `scripts/api_audit.py` (audit runner).

Action: Run `api_audit.py` → categorize findings → fix `key-missing` (wrong field names in field_in declarations) → inspect `not-json` (API returning non-JSON, possibly auth-gated) → repair `fetch-fail` (URLs that 404/timeout). Audit findings include the 395 map-frame fixes — fold into this work.
Verification: `api_audit.py` run after fixes → zero `key-missing`, `not-json` and `fetch-fail` tracked as known issues. Source count does not decrease. `cargo check` clean.

### Live TAP Catalogs → CDN Mirror

135 TAP catalog sources with `ttl=86400`. Static catalogs. Mirror to CDN and serve from cache.
Code: `phi/sources.φ` (source blocks), CI workflow `refresh-catalogs.yml`.

Action: Add TAP catalog sources to CDN mirror pipeline. Each catalog fetched once per day → uploaded to CDN release → Archivar fetches from CDN (not live TAP endpoint). Update `sources.φ` URLs to CDN paths.
Verification: CI workflow fetches TAP catalogs, uploads to CDN release. Archivar resolves CDN URLs. Source blocks active and producing oscillators.

### Priority-A Sources

Auth APIs from `docs/plans/AUTH_APIS.md` section A (highest value): NASA ADS, Space-Track, SuperMAG, GRACE-FO, SMAP, CDDIS IONEX, GES DISC, NASA AppEEARS. These require API keys and source blocks.
Code: `phi/sources.φ` (new source blocks), CI workflow `refresh-protected-data.yml` (fetch steps).

Action: For each Priority-A API: create source block in `sources.φ` with correct frame, force, extracts. Add fetch step to `refresh-protected-data.yml`. Register API keys in `.secrets.local` and GitHub Secrets. Use `{SECRET_NAME}` substitution.
Verification: Each source block parses without error. CI workflow step runs (skips gracefully if secret missing). Oscillators appear in render window for each new source.

---

## Secrets Wiring (operational — not code)

### Missing Secrets

`docs/plans/AUTH_APIS.md` lists 25 total secrets to obtain. 7 are already present. 8 additional are needed for sources currently blocked.
Code: `.secrets.local` (gitignored), GitHub Actions Secrets.

Action: Obtain API keys for: NASA_API_KEY, NASA_ADS_TOKEN, SPACETRACK_USER/PASS, SUPERMAG_USER/PASS, CDS_API_KEY, JSOC_EMAIL, NOAA_CDO_TOKEN, TNS_API_KEY. Register via URLs in `AUTH_APIS.md`. Enter values in `.secrets.local`. Add to GitHub Actions Secrets.
Verification: `.secrets.local` contains non-empty values for all 8 keys. GitHub Actions Secrets populated.

### Workflow Secret Wiring

`refresh-protected-data.yml` must use the secrets from above for authenticated API fetches.
Code: `.github/workflows/refresh-protected-data.yml`.

Action: Add workflow steps for each new secret-bearing API. Each step: `if: secrets.SECRET_NAME != ''`, fetch with auth header/param, upload artifact to CDN release. Skip gracefully when secret not set.
Verification: CI run with secrets set → all authenticated sources fetch. CI run without secrets → steps skip, no failure. CDN assets created for fetched sources.

---

## Biosphere Verification

### verify_sources.py — Timeouts

iNaturalist blocks: Jina timeout (30s) for 200-geojson records. Direct API returns HTTP 200. Either increase timeout or make Jina fallback toggleable.

### verify_sources.py — Format Recognition

Five source types need format recognition:
- openlittermap.com (2 blocks): JSON not detected as valid format
- stac.openlandmap.org (1 block): JSON not detected as valid format
- www.sciencebase.gov (1 block): field mapping mismatch
- meta.icos-cp.eu (2 blocks): SPARQL response format not recognized

### OBIS grid/2

Polygon geometry (FeatureCollection). Polygon features require centroid extraction. The occurrence endpoint with geometry parameter returns valid point data.

### Mangrove CDN Pipeline

Workflow step must execute → assets created. Release `data-gis.unep-wcmc.org` needs initial population. 11 layers: `gmw_v3_{year}_vec_centroids_{timestamp}.json`.

### FIRMS CDN

Blocks in `sources.φ` carry `{latest}` suffix → 404 without resolver. Covered by `{latest}` resolver above.

---

## Sources Inventory Reclassification

Operator correction: "auth required" was applied too broadly. The only obstacle is PAID. Free keys and free registrations are NOT obstacles. `.secrets.local` already contains working keys for nearly every source previously marked "auth required."

Reclassification of the 2,189 pre_cdn.φ source blocks:
- IMPLEMENT-NOW: ~85% (keyless or key in .secrets.local)
- ACQUIRE-KEY: ~3% (10 free-registration APIs, operator work)
- DEFER: ~10% (parser support, TLE extract, TAP protocol, live URL verification)
- DROP: ~2% (MarineTraffic, SentinelHub/Planet/GEE, LeoLabs, CTBTO, DSN Now, MBARI MARS)

Code: `phi/recovery/pre_cdn.φ` (inventory), `phi/sources_cdn.φ`, `phi/sources_live.φ`

---

## Sources Live Schema Drift Canonicalization

`sources_live.φ` carries inconsistent field naming patterns across source blocks. Example: `field properties.mag mag` vs `field properties.mag event_magnitude` for identical USGS earthquake magnitude fields. Schema drift compounds with each added block.

Rule: force + frame are already canonicalized. Field keywords must follow: `field <json_path> <canonical_name>` where canonical_name is force-prefixed and describes the physical quantity, not the source-specific abbreviation.

Action: One-pass canonicalization of `sources_live.φ` field keywords. Apply the same naming discipline used in `sources_cdn.φ`. Incremental — run after each session that adds new live blocks, not a blocking prerequisite.
Verification: `grep -oP 'field \S+ \K\S+' phi/sources_live.φ | sort | uniq -c | sort -rn` shows consistent naming (no duplicate patterns for the same physical quantity). `cargo check` clean.

---

## Pre-CDN Format Migration: Acoustic Force

Migrate remaining acoustic-force source blocks from `pre_cdn.φ` old format to url-first format in `sources_cdn.φ` (ttl≥86400) or `sources_live.φ` (ttl<86400).

Scope: 105 acoustic blocks. NDBC buoys (39, CSV format, keyless), NOAA CO-OPS (130 blocks across acoustic/thermal split), GCOOS buoys (30, ArcGIS, keyless), METAR stations, rain radar, aviation weather.

Format conversion: `source <name>` → remove, `wgs84 <lat> <lon>` → `on earth <lat> <lon>`, `ecliptic 1` → `at sun 1.0`, add `body earth` where missing. Canonical field keywords. No overlapping blocks with existing `sources_cdn.φ`/`sources_live.φ` — deduplicate.

Verification: All migrated blocks parse without error. `cargo run` loads from both files. `cargo check` clean. Acoustic oscillators appear in render window.

---

## Pre-CDN Format Migration: Diffusion Force

Migrate remaining diffusion-force source blocks from `pre_cdn.φ` to canonical format.

Scope: 186 diffusion blocks. USGS water quality (21, USGS_WATER_KEY in .secrets.local), AERONET aerosol (18, DEFER — needs AERONET parser research first, move to DEFER), BOM NSW weather, Open-Meteo air quality, ArcGIS PM2.5, NOAA electron/proton flux, GOES SEP proton flux.

Approach: IMPLEMENT-NOW blocks first (~140). DEFER blocks to separate TODO.

Verification: Diffusion oscillators appear in render window. `cargo check` clean.

---

## Pre-CDN Format Migration: Gravity Force — BGS Magnetic

Migrate the British Geological Survey magnetic data blocks from `pre_cdn.φ`.

Scope: 162 blocks from `imag-data.bgs.ac.uk`. All keyless. Force `gravity`. Each block is a single magnetic observatory or model grid point with lat/lon and field component values.

Format: url-first with `on earth` frame, `force gravity`. CDN candidates (ttl≥86400 implied by static nature). Canonical field keywords: `field <path> gravity_bgs_<component>`.

Verification: All 162 blocks parse. `cargo check` clean. Magnetic field oscillators appear in render window with force `gravity`.

---

## Pre-CDN Format Migration: Gravity Force — JPL SSD / NEO

Migrate JPL Solar System Dynamics and NEO close-approach blocks.

Scope: 78 SSD blocks + 26 NEO blocks. Keyless. Force `gravity`. Position via orbital elements or state vectors. May require `Extract::HorizonsVec` or `Extract::Kepler` (existing TODOs). Simple blocks (CAD NEO) are rows with `ra_key`/`dec_key`.

Approach: Simple CAD NEO blocks first (IMPLEMENT-NOW). Ephemeris blocks DEFER until extract types exist.

Verification: NEO oscillators appear at correct ICRS positions. `cargo check` clean.

---

## Pre-CDN Reclassification: World Bank Indicators DROP

The 228 World Bank indicator blocks in `pre_cdn.φ` (GDP, birth rate, forest percentage, etc.) are economic statistics — not direct physical measurements. They have no force. The prior verdict of IMPLEMENT-NOW with `force em` is revoked under the Force Gate Principle: the measured quantity does not propagate through the block universe under any declared force. All 228 are DROP.
Code: `phi/recovery/pre_cdn.φ` (inventory reference only — no migration occurs).

---

## Pre-CDN Reclassification: Yahoo Finance DROP

The 37 Yahoo Finance ETF/stock price blocks in `pre_cdn.φ` are symbolic abstractions (prices) — not physical measurements. They have no force. The prior verdict of IMPLEMENT-NOW is revoked. All 37 are DROP.
Code: `phi/recovery/pre_cdn.φ` (inventory reference only — no migration occurs).

---

## Cleanup: Excise Force-Assigned World Bank Violations

Three World Bank blocks were migrated to `sources_cdn.φ` with falsely assigned forces: `worldbank_forest_area` (force diffusion, economic statistic), `worldbank_pm25_exposure` (force diffusion, modeled estimate), `worldbank_co2_emissions` (force em, national accounting number). These violate the Force Gate Principle.
Action: Remove the three blocks from `phi/sources_cdn.φ`. Verify `cargo check` clean. DONE.
Code: `phi/sources_cdn.φ`

---

## Pre-CDN Format Migration: EM Force — TAP Astronomy

Migrate TAP/VizieR astronomical catalog source blocks.

Scope: 79 TAP VizieR blocks, 51 HEASARC blocks. ASTROQUERY TAP protocol. CDS_API_KEY in `.secrets.local`. Force `em`.

Status: TAP response format (VOTable XML) needs parser support. Blocks are IMPLEMENT-NOW for key access but DEFER for parser. Add as `format universal` with `rows` extract where CDS API returns simple JSON. TAP-specific VOTable → DEFER.

Verification: JSON-returning CDS blocks parse. VOTable blocks skip gracefully with format warning. `cargo check` clean.

---

## Pre-CDN Format Migration: EM Force — PDG Particle Data

Migrate Particle Data Group fundamental physics constants.

Scope: 38 blocks from `pdgapi.lbl.gov`. Keyless. Force `em`. Each block is a single particle mass, width, or decay constant. Static data, ttl≥604800 → `sources_cdn.φ`. Frame: `on earth` at LBNL coordinates (37.877, -122.247).

Approach: url-first format, `force em`, canonical `field pdg_values.0.value <name>`. Single-session implementation.

Verification: All 38 blocks parse. `cargo check` clean. PDG mass values confirmed against published PDG review.

---

## Pre-CDN Format Migration: Seismic Force Deduplication

Migrate remaining seismic-body and seismic-surface blocks, deduplicating against already-active sources.

Scope: 46 seismic-body + 67 seismic-surface = 113 blocks. USGS (48 — mostly duplicates of active), EMSC, EarthScope, INGV, KOERI, historical yearly quake catalogs, volcano activity.

Work: Compare each pre_cdn.φ seismic block against sources_cdn.φ and sources_live.φ. Blocks with identical URL → drop (already migrated). Blocks with different URL but same data → merge (canonical URL). Blocks with new data → migrate.

Verification: No duplicate URLs between CDN and live files. No duplicate data streams. `cargo check` clean.

---

## Pre-CDN Format Migration: Thermal Force — Historical Climate

Migrate remaining thermal force blocks.

Scope: 73 thermal blocks. NCEI climate (NOAA_CDO_TOKEN in .secrets.local), NSIDC sea ice (EARTHDATA_EDL_TOKEN), FIRMS fire (FIRMS_MAP_KEY in .secrets.local), Archive Open-Meteo (25, keyless, CDN candidates), ArcGIS METAR.

CDN candidates: all blocks with ttl≥86400 are static archives → mirror to CDN.

Verification: Thermal oscillators appear. `cargo check` clean.

---

## Key Acquisition: Free Registration Blitz

Operator action (not code). Register for 10 free-API keys not yet in `.secrets.local`.

APIs: ENTSO-E, Electricity Maps, TNS (wis-tns.org), Lasair (lasair.roe.ac.uk), Movebank, OpenTopography (portal.opentopography.org), CAMS ADS (ads.atmosphere.copernicus.eu), FRED (api.stlouisfed.org), Global Fishing Watch, Meteostat free tier.

Process: Visit each registration URL from `docs/plans/AUTH_APIS.md`. Obtain key. Enter in `.secrets.local` (gitignored). Enter in GitHub Actions Secrets.

Verification: `.secrets.local` contains non-empty values for all 10 keys. `grep -c '=$' .secrets.local` returns 0 (no empty values for these keys).

---

## Deferred — Require AGENTS.md Amendment

### Temporal Topology (TDA, Takens, Transfer Entropy)

PREREQUISITE: AGENTS.md amendment. `fieldPermeability = exponential relaxation (naturalLatencyTicks as τ)` must be replaced or extended. Clarification needed: does "topology analysis belong to the Mathematikerin" mean GPU-side computation of TDA/TE kernels, or does it require a new architectural layer?
Scoped as: WGSL compute shader implementing Takens delay embedding on the temporal oscillator stream. Transfer entropy between force-type channels. Persistent homology on the point cloud across time.
Not implementable in one session without architectural decisions.

### Field Permeability without TE (`tanh(vC/g)`)

PREREQUISITE: AGENTS.md amendment. The current `fieldPermeability = exponential relaxation` would be replaced by `tanh(vC/g)` where v is the propagation velocity for the force type, C is a coherence scalar, and g is the local field gradient. This changes the fundamental evaluation formula.
Scoped as: WGSL shader modification to compute tanh-based permeability kernel. Requires defining the coherence scalar C and the local gradient g from the oscillator field.
Not implementable until AGENTS.md is amended.

### Visionary

- **Water Metaphor:** Visualization concept. Could be approached as a rendering refinement when the presence window is mapped to fluid dynamics visualization. Deferred until rendering pipeline is stable.
- **Total Coherence:** New scalar metric. Does not emerge from oscillator properties — introduces new abstraction layer. Requires AGENTS.md amendment or reformulation.
- **Temporal Manifestation / Retro-Active:** Changes fundamental time model. Requires block universe physics rewrite. Deferred indefinitely.
- **Global Station Web via Nostr:** Protocol integration. Requires station-to-station communication architecture. Separate infrastructure project. Unclear how it emerges from oscillator `canRadiate`/`canSense` properties without new protocol machinery.

### ESP32 Mantis-Shrimp Firmware

Not part of this repository. `docs/omegaflow_sense_hardware.yaml` specifies hardware design. Firmware is a separate project. OUT-OF-SCOPE.

---

## Rejected — AGENTS.md conflict

### Unknown-Force Soft Fallback to `em`

REJECTED. AGENTS.md: "Frameless and forceless sources are refused at load." The code at line 2635 implements this: `force_id_of(&cur_force).is_none()` → source refused with `eprintln!`. Soft fallback to `em` would silently assign electromagnetic propagation physics to an unknown force type. A = A: an acoustic oscillator is not an electromagnetic oscillator. The physics is incorrect.
If a force type is needed, it must be added to `force_id_of()` and `force_extent()` explicitly, with defined propagation constants. This constraint is architectural, not negotiable. |

## CDN-Switch Loss Analysis

`Archiv/sources.sorted.φ` (2412 urls) was the pre-CDN original. Comparison with current live+cdn (3514 urls) found 1524 urls absent. Analysis:
- DELIBERATELY REMOVED (council decisions): worldbank(230), yahoo(37), pdg(38), heasarc/inspirehep metadata, cmems model, celestrak TLE
- TRUE LOSSES to recover: simbad(84, em catalogues - original used wrong col main_type, correct is otype), ssd.jpl.nasa.gov(48, gravity Horizons), gea.esac.esa.int(50, Gaia em), overpass.openstreetmap.fr(235), nmdb.eu(41 - but HTTP-only, likely dead), api.geonet.org.nz(12)
- BGS-HAPI: 92 observatories present in live but had dead hardcoded timestamps (fixed to {yesterday}..{yesterday}T23:59). 2 missing (mcg, nag). Original used {today} template which fails (1405 - best-avail has latency).

Action: recover SIMBAD (curate with otype), JPL-SSD (Horizons, parser-def), Gaia. Verify each.
Code: `phi/recovery/research/arena/` inventory, `phi/sources_live.φ`

## Remaining Curation Backlog (post-recovery comparison)

`ALL_lost_blocks_richest.φ` (5701 recovered) vs arena blocks still unchecked (1033):
- 873 blocks NOT in lost history = truly new, completely unchecked
  (saved: `phi/recovery/pre_cdn_history/NEW_unchecked_blocks.φ`, 13658 lines)
  Largest: tapvizier 249, ncei 47, heasarc 39, coastwatch 20, pangaea 18,
  celestrak 16, cdaweb 15, cmems 13, mast 11, irsa 10, inaturalist 10,
  gml 10...
- 160 blocks ARE in lost history (known): 127 equal richness, 28 arena
  richer, 5 arena POORER (noaa_gml_co2_global_network 7vs10, nsidc_seaice
  arctic/antarctic 5vs6, swpc_solar_wind_dscovr 5vs7, nist_codata 4vs5)
- Lost endpoint example: co2_surface-flask_ccgg_text.txt (rich, 8 fields)
  now 404 (GML restructured) - not all recovered endpoints still live.

Action: curate 873 new blocks (reachability + structure + force-gate
one-by-one). For 160 known, compare arena vs lost richest version and
use the richer. Every source tested individually.
Code: `phi/recovery/pre_cdn_history/NEW_unchecked_blocks.φ`, `sources_live.φ`

## NEXT SESSION ENTRY POINT: Untested Blocks

Read `docs/source_curation.md` first — it is the full session handoff with
the parser status, the parser-as-verifier method, and the session results.

STATUS (2026-08-07, post key-fix): extraction test = **194 ok / 6 fail** of
the first 200 non-CDN live sources (was 29 ok / 171 fail). Commit `e19b47f`
fixed DONKI (case-insensitive secret resolution + regex X-class extract),
converted the two USGS waterservices fixed-bbox blocks to presence-window
templates, and gave the test a Houston-TX window + `{lon_min}`..`{lat_max}`
substitution. The 6 remaining FAILs are verified service/data availability:
5 USGS quake feeds legitimately empty on 2026-08-07 (extract directives
verified against data-bearing feeds), gracedb 503 scheduled maintenance.
Previous 107-FAIL inventory RESOLVED — decisive fix was reverting the
`field_in`->`field` migration error (commit 9c16f8a).

TASK: run `cargo test test_live_sources_extract -- --nocapture` (~150s).
Confirm 194 ok / 6 fail, then raise the test limit (`let mut limit =
200usize;` in `test_live_sources_extract`) to the next chunk and repeat,
fixing every new `FAIL <url> no samples` with the generic recipes in
source_curation.md (positionless -> `last <dot.path>` at frame; GeoJSON ->
`map features` + `geometry.coordinates.N` + `on earth`/`body earth`; fixed
station -> `last <container>.<field>`; metadata/text-warning -> decline).

Then continue curation with `phi/recovery/pre_cdn_history/UNTESTED_blocks.φ`
(423 blocks). PROGRESS TRACKING: when a block is tested, add it to
dead_sources.φ (dead/parser-def/decline/key-needed) or sources_live.φ
(verified). UNTESTED_index.txt shows remaining by domain.

Also open:
- 5 poorer blocks: co2_global_network(7vs10), nsidc_arctic/antarctic(5vs6),
  swpc_solar_wind_dscovr(5vs7), nist_codata(4vs5) - rebuild with richer
  extract from ALL_lost_blocks_richest.φ
- 93 HAPI blocks: consider switching manual path 1.0 extracts to native
  `hapi` Extract variant
