# TODO

AGENTS.md is the primary constraint matrix. Git is the history. This file contains only pending work.

---

## Touchpad / Touch Control

Spec: `docs/concepts/INTUITIVE TOUCHPAD- & TOUCH-STEUERUNG.md`. Diagonal pinch → zoom. 2-finger horizontal → time. 2-finger vertical → spatial forward/back. `pointers.size < 3` condition in tThrust reset → `< 2`.
Code: `static/index.html` — pointer event handlers, tThrust reset condition, gesture state machine.

Action: Implement 2-finger gesture handling: track initial separation and center for pinch zoom, horizontal delta for time thrust, vertical delta for spatial thrust. Change tThrust reset from `pointers.size < 3` to `< 2` so single-finger lift doesn't reset temporal velocity.
Verification: Two-finger pinch zooms in/out (scale changes). Two-finger horizontal swipe changes time. Two-finger vertical swipe moves forward/back. Single-finger lift does not reset tThrust. `cargo check` clean. Open browser at `127.0.0.1:1111`, confirm gestures work.

---

## Archivar `{latest}` Resolver

`sources.φ` URLs may carry `{latest}` suffix. The Archivar must resolve this to the most recent timestamped asset from the CDN release. GitHub Releases API: `GET /repos/omegaflow/sources/releases/tags/{netloc}` → filter assets by the name prefix (everything before `_{latest}`) → select asset with highest alphanumeric sort → substitute URL.
Code: New function `resolve_latest(url: &str) -> String` in `src/main.rs`. Uses `Command::new("curl")` + pure-Rust string parsing. No serde. No external HTTP library.

Action: Implement `resolve_latest()` that: extracts netloc and base name from URL, curls GitHub API, filters and sorts assets by name (timestamps sort alphanumerically), returns substituted URL. Call from the fetch loop before `spawn_task_curl`. Cache resolved URLs per source per ttl window.
Verification: Source with `{latest}` in URL resolves and fetches successfully. Non-`{latest}` URLs pass through unchanged. Cache hit returns previous resolution. `cargo check` clean. Watch server log for `Resolved {latest}: ...` lines during fetch cycle.

---

## CDN Asset Renaming

CDN assets carry old sphere-prefix names. Must be renamed to immutable format.
Rule: Release tag = netloc. Asset = `{api_delivered_filename}_{iso8601_utc}.json`. No `--clobber`.
Code: `.github/workflows/rename-cdn-assets.yml`, `scripts/migrate_live_to_cdn.py`.

Action: CI workflow reads `phi/live_url_map.json`, original filename = last path segment of API URL, `gh release upload` with timestamped name. `migrate_live_to_cdn.py`: `asset_name = f"{delivered_filename}_{iso_utc}.json"`.
Verification: CI run on a fresh release creates assets with `_{iso8601_utc}.json` suffix. No duplicate uploads. Assets are immutable.

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
