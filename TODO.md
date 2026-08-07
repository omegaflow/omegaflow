# TODO

AGENTS.md is the primary constraint matrix. Git is the history. This file contains only pending work.

---

## Touchpad / Touch Control

Spec: `docs/concepts/INTUITIVE TOUCHPAD- & TOUCH-STEUERUNG.md`. Diagonal pinch → zoom. 2-finger horizontal → time. 2-finger vertical → spatial forward/back. `pointers.size < 3` condition in tThrust reset → `< 2`. Code: `static/index.html`.

---

## `{latest}` URL Resolution

`sources.φ` URLs may carry `{latest}` suffix. Resolve to the most recent timestamped asset from the CDN release. GitHub Releases API → latest alphanumeric asset for the name prefix. Pure Rust (Command::new("curl") + string parsing, no external HTTP lib). Call from fetch loop. Cache resolutions per source per ttl window. Code: `src/main.rs`.

---

## CDN Asset Renaming

CDN assets carry old sphere-prefix names. Rename to `{api_delivered_filename}_{iso8601_utc}.json`. Immutable. No --clobber. Release tag = netloc. Code: `.github/workflows/rename-cdn-assets.yml`, `scripts/migrate_live_to_cdn.py`.

## CI Script URL-First Migration

`tap_to_cdn.py`, `refresh_catalogs.py` still use `re.split(r"\n(?=source )")`. Switch to `content.strip().split('\n\n')`. Derive name from URL path. `restore_all_live.py`: remove `{latest}` strip logic. Code: `scripts/tap_to_cdn.py`, `scripts/refresh_catalogs.py`, `scripts/restore_all_live.py`.

---

## New Extract Types: Kepler, Horizons, Flatten

- **Kepler**: Orbital elements from table rows → ICRS via Kepler's equation. New Extract variant.
- **Horizons/Vectors**: Parse JPL Horizons `$$SOE`/`$$EOE` state vector format. `Motion::Linear { p, v }` from km→m scaled position/velocity.
- **Flatten**: Array-of-arrays JSON → map by index position or declared field order. No key mapping needed.
Code: `src/main.rs` Extract enum, `load_sources()`, `materialize()`.

---

## Field Infrastructure

- **`field_in` nested support**: Handle paths like `states.state[0].name`. Current implementation only flat arrays.
- **Extent per force-type verification**: `force_extent()` uses hardcoded values. Needs data verification against live body properties.
- **Window/temporal bounding**: `from`/`until` time constraints on queries. Not yet implemented.
Code: `src/main.rs`.

---

## Command Palette (⌘K)

SIMBAD TAP object search → presence jump. Source search from `phi/sources.φ`. Force-type filter. Client-side only (vanilla JS). Code: `static/index.html`. Spec: `docs/concepts/SEARCH_COMMAND-PALETTE.md`.

---

## `tanh(vC/g)` Field Permeability

Alternative to current exponential relaxation. PREREQUISITE: AGENTS.md C6 amendment. `v` = force-type propagation velocity, `C` = coherence scalar, `g` = local field gradient. Code: WGSL fragment shader.

---

## 96 Audit Findings (`phi/api_audit.jsonl`)

`key-missing`: declared keys don't match real data keys. `not-json`: flatten_cdn.py bug. `fetch-fail`: missing CDN asset → pipeline defect. Fix sources and pipeline.

## Live TAP Catalogs → CDN Mirror

135 live TAP sources with ttl=86400. Mirror to CDN instead of hitting live APIs. 69 Gaia `em gravity→em` fixes unverified. 395 map-frame fixes unverified. 2 NDBC buoys shifted ~260 km.

## Priority-A Sources

NASA ADS, Space-Track (full satellite catalog), SuperMAG (~300 stations), GRACE-FO, SMAP soil moisture, CDDIS IONEX, GES DISC, NASA AppEEARS. Source blocks + pipeline steps.

---

## Minkowski 4D Weighting

Unresolved design: at cosmic scales, light-cone filter makes Sun invisible (API `t` is fetch-time, not emission-time). Apply only to local sensors, or dynamic scale? `docs/concepts/MINKOWSKI_FIELD-PERMEABILITY.md`.

---

## Secrets (Operational — not code)

8 missing API keys (SPACETRACK_PASS, SUPERMAG_PASS, TNS_API_KEY, ALPHAVANTAGE_KEY, FRED_API_KEY, TRANSPORTDATA_KEY, TRANSIT511_KEY, TNG_KEY). Wire acquired secrets into `refresh-protected-data.yml`.

---

## Rejected — AGENTS.md C4 conflict

Soft fallback of unknown force types to `em`. C4: "Frameless and forceless sources are refused at load." Unknown force = refused. Must be added to `force_id_of()` and `force_extent()` explicitly.

