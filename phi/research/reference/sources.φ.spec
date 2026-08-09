# sources.φ Specification

## 1. Overview

`sources.φ` is the canonical catalog of all data sources in omegaflow. Each source block
defines one data source: its origin (URL), fetch properties (ttl, force, body), spatial
reference (frame), and the fields it produces (extracts).

The Archivar reads this file, fetches data, and makes it available as oscillators in the
ICRS field. The Mathematikerin (WebGPU fragment shader) evaluates the field at each pixel.

## 2. Data Flow

```
API-Live ──→ Archivar fetch ──→ parse ──→ extracted HashMap ──→ field samples
                                    ↑
API-Live ──→ CI pipeline ──→ CDN Release ──→ Archivar fetch ──┘
```

### Live Data (TTL < 300 or template URL)

The Archivar fetches directly from the API. The API response contains raw field names
(`latitude`, `WSPD`, `proton_speed`). The `field` directive maps each raw API key to a
Quellentreu output name.

### CDN Data (TTL ≥ 300, `releases/download` URL)

The CI pipeline (`migrate_live_to_cdn.py --mode regen-cdn`, running every 5 minutes)
fetches from the API, normalizes keys via `KEY_MAP` (`latitude`→`lat`, `temperature`→`val`),
and uploads the result as a JSON asset into the CDN release. The Archivar fetches the
static CDN file.

### {latest} Resolution

CDN URLs in sources.φ use `{latest}` as a timestamp placeholder:
```
url https://github.com/omegaflow/sources/releases/download/ndbc.noaa.gov/46083_{latest}.json
```
The Archivar resolves `{latest}` to the most recent timestamped asset in the release.

### Package (Offline Start)

`omegaflow-data-{iso8601_utc}.tar.gz` contains a snapshot of all CDN assets in `phi/data/`:
```
phi/data/ndbc.noaa.gov/46083_20260806T120000Z.json
phi/data/api.wheretheiss.at/25544_20260806T120000Z.json
```
Extract and run `cargo run` — no network access required.

## 3. CI Pipeline

### Refresh Workflow (`refresh-protected-data.yml`)

Runs every 5 minutes. `migrate_live_to_cdn.py --mode regen-cdn`:
1. Selects CDN sources with TTL ≥ 300
2. Checks if the CDN asset is older than TTL seconds
3. If stale: fetches from original API → normalizes via `KEY_MAP` → uploads as `{source_id}_{iso8601_utc}.json`
4. Deletes old timestamped assets for the same source

### Ephemeris Workflow (`generate-ephemerides.yml`)

Runs daily at 03:00 UTC. `generate_ephemerides.py`:
1. Downloads SPICE kernels from NAIF (de440.bsp, jup365.bsp, sat441.bsp, mar099s.bsp, nep097.bsp)
2. Generates Chebyshev polynomial binaries for 23 bodies (planets + moons)
3. Queries JPL Horizons REST API for 17 small bodies/spacecraft
4. Uploads as `{body}_{timestamp}.bin` into the `ssd.jpl.nasa.gov` release

### KEY_MAP (`flatten_cdn.py:206`)

The CDN pipeline normalizes raw API keys to canonical short forms:
```
latitude→lat  longitude→lon  temperature→val  time→t  altitude→alt
value→val  magnitude→val  depth→alt  RA→ra  DEC→dec
timestamp→t  date→t  epoch→t
```

For LIVE sources, sources.φ uses the raw API key as the field path.
For CDN sources, sources.φ uses the CDN-normalized key as the field path.
The parser does exact 1:1 key matching — no normalization at runtime.

## 4. Archivar Fetch Mechanics

### TTL/Φ — Refetch Interval

`origin_stale()` (main.rs:888): `now - last_fetch >= ttl / Φ`

Φ = 1.618 (golden ratio). For TTL 300: 300/1.618 ≈ 185s actual refetch interval.
The Φ factor prevents oscillation between fetch and TTL expiry.

### Min-TTL Priority Heap

`fetch_priority()` (main.rs:3683): `(ttl.log2() / Φ)`

Sources with shorter TTL are prioritized in the bounded worker pool (2³ workers).

### Φ Usage Summary

| Location | Formula | Purpose |
|---|---|---|
| origin_stale | `ttl / Φ` | Refetch interval |
| fetch_priority | `ttl.log2() / Φ` | Priority score |
| connect_t | `ttl / Φ³` | Exponential backoff (max 5s) |
| max_t | `ttl / Φ²` | Maximum fetch interval (max 120s) |
| law_bounds | `Φ * v, Φ * a` | Motion law buffer |
| presence_gate | `extent * Φ + range` | Spatial search radius |
| station ema | `Φ² * ema_interval` | Station data interval |

## 5. Parser Mechanics

### Keyword-Based, Position-Independent

The parser (`load_sources()`) processes keywords individually. State accumulates
across all keywords in the block. `flush!()` is called when the next `url` keyword
is encountered:

```
url https://...     ← flush!() previous block, reset all state, active=true
ttl 10              ← cur_ttl = 10
force em            ← cur_force = "em"
on earth 52.5 13.4  ← cur_body = "earth", cur_lat/cur_lon set
field WSPD wind_speed_m_s  ← cur_extracts.push(...)
                      ← next url → flush!() → SourceConfig created
```

### Exact Key Matching

The parser does 1:1 key matching in JSON: `jnum(json, "WSPD")` → value.
No auto-detect. No KEY_MAP at runtime. No pattern recognition.
What is declared in `field` is extracted. What is not declared, does not exist.

### Ephemeris Binary

`format ephemeris_binary` sources are processed specially:
1. Archivar fetches binary file from CDN
2. `parse_ephemeris_binary()` parses Chebyshev polynomials
3. `BodyEphemeris` stored in `archive.body_ephemerides`
4. Other sources resolve body positions via `body_barycenter_position()`

## 6. Block Format

### Specification

```
url https://github.com/omegaflow/sources/releases/download/{netloc}/{source_id}_{latest}.{ext}
ttl {seconds}
force {force}
body {body_name}
{frame directive}
format {json|text|ephemeris_binary}
tau {seconds}             ← optional
tau_key {key}             ← optional
map|cmap {arr_path}
lat_key|lon_key|ra_key|dec_key {key}
alt_key|epoch_key|val_key {key}
field {key_path} {physical_quantity}_{si_unit}
path|last|count {key_path} {physical_quantity}_{si_unit}
geojson|rows|tail|field_in {args}
pos {lat_key} {lon_key} [{alt_key}] [{scale}]
header {Key} "Value"      ← optional
stations {url}            ← optional
```

### Rules

- `url` is the first line of every block. It is the block's identity and anchor.
- No `source <name>` lines.
- Exactly 1× `ttl`, 1× `force`, 1× `body` per block.
- The `body` directive is REQUIRED for every source. No defaults. Earth is not special.
  Sources with `on <body>` or `at <body>` implicitly set `body` from the frame.
- Exactly 0-1 frame directives: `on`, `at`, `pos`, or template URL (no directive needed,
  position from `{lat}`/`{lon}` placeholders).
- Blank line separates blocks. No blank lines within a block.

### Parameter Reference

url|url <URL>|Identity, anchor. Template vars: {lat}, {lon}, {grid}, {year}, {latest}
ttl|ttl <s>|Archivar refetch interval. Phenomenon change rate
force|force <f>|Field medium: em, gravity, acoustic, seismic-body, seismic-surface, thermal, diffusion, advective. Composites: space-separated
body|body <name>|Body name. REQUIRED for every source. Set implicitly by on/at
on|on <body> <lat> <lon> [alt]|Fixed surface point. Implicitly sets body
at|at <body> <scale>|Barycenter frame. Implicitly sets body
pos|pos <lat_key> <lon_key> [alt_key] [scale]|Position from data fields. Needs explicit body
format|format <f>|json (default), text, ephemeris_binary
tau|tau <s>|Oscillator decay constant
tau_key|tau_key <k>|Tau from data field
map|map <path>|Terrestrial position extract. Array at path in JSON
cmap|cmap <path>|Celestial position extract (RA/Dec)
field|field <key> <name>|Scalar extract. key = JSON path, name = Quellentreu
path|path <key> <name>|Like field, JPath semantics
last|last <key> <name>|Last value of JSON array
field_in|field_in <key> <name>|Column extract (columnar CDN data)
rows|rows|Row-based text data
tail|tail|Last line only
geojson|geojson events <m> <min> <names>|GeoJSON event feed
lat_key|lat_key <k>|Latitude field name (attached to map)
lon_key|lon_key <k>|Longitude field name (attached to map)
ra_key|ra_key <k>|Right ascension (attached to cmap)
dec_key|dec_key <k>|Declination (attached to cmap)
alt_key|alt_key <k>|Altitude (attached to map)
epoch_key|epoch_key <k>|Timestamp (attached to map/cmap)
val_key|val_key <k>|Value field override
vel_key|vel_key <k>|Velocity (motion law)
trk_key|trk_key <k>|Track/heading (motion law)
vr_key|vr_key <k>|Vertical rate (motion law)
plx_key|plx_key <k>|Parallax (mas)
pmra_key|pmra_key <k>|Proper motion RA (mas/yr)
pmdec_key|pmdec_key <k>|Proper motion Dec (mas/yr)
radvel_key|radvel_key <k> <scale>|Radial velocity
dist_key|dist_key <k> <scale>|Distance key + scale factor
z_key|z_key <k>|Redshift (Hubble flow)
header|header <K> "V"|HTTP request header
stations|stations <url>|Station list API endpoint
stations_path|stations_path <p>|Station list path in JSON
stations_lat|stations_lat <k>|Station latitude field
stations_lon|stations_lon <k>|Station longitude field
stations_id|stations_id <k>|Station ID field
verify|verify false|Skip SSL verification (rare)
method|method POST|HTTP method override

## 7. Quellentreu Naming Rule

### Principle

The output name of an extract is WHAT is measured — not WHO measured it,
not WHERE it was measured, not with what instrument. No sphere prefixes.
No provider names. No station IDs. The name describes the physical quantity
and its unit.

### Construction

Output name = `{physical_quantity}_{si_unit}`

### Rules

- snake_case only — letters, digits, underscores
- SI unit suffix for dimensionful values: `_m_s`, `_deg`, `_km`, `_c`, `_k`, `_hpa`, `_nt`
- No suffix for dimensionless values: counts, codes, flags, indices
- Temperature: `_c` for medium (air/water/soil), `_k` for radiation source
- No prefix: no `buoy_`, no `iss_`, no `ndbc_`, no `solar_`, no `atmosphere_`
- No provider name: no `openmeteo`, no `noaa`, no `swpc`, no `worldbank`
- Invalid characters `(){}[]-` are forbidden — only `[a-z0-9_]`

### Examples

| API Key | Physical Quantity | Unit | Output Name |
|---|---|---|---|
| WSPD | wind speed | m/s | wind_speed_m_s |
| ATMP | air temperature | °C | air_temperature_c |
| PRES | barometric pressure | hPa | barometric_pressure_hpa |
| WVHT | wave height | m | wave_height_m |
| latitude | latitude | deg | latitude_deg |
| altitude | altitude | km | altitude_km |
| proton_speed | proton speed | km/s | proton_speed_km_s |
| flux | radio flux | sfu | radio_flux_sfu |
| ssn | sunspot number | — | sunspot_number |
| flight | callsign | — | callsign |

## 8. Force Reference

| Force | ID | Description |
|---|---|---|
| em | 1 | Electromagnetic (light, radio, radiation) |
| gravity | 2 | Gravity (tides, orbital dynamics) |
| acoustic | 3 | Acoustics (sound, waves) |
| seismic-body | 4 | Body-wave seismics (P/S waves) |
| seismic-surface | 5 | Surface-wave seismics (Rayleigh/Love) |
| thermal | 6 | Thermal (temperature, heat) |
| diffusion | 7 | Diffusion (species spread, particles) |
| advective | 8 | Advection (current, wind, flow) |

Forces can be combined space-separated: `em gravity` for an EM observation of a
gravitational phenomenon. The parser processes all space-separated tokens.

## 9. Ephemeris System

### SPICE Kernel Bodies (23)

Major planets and moons with ephemeris data from JPL NAIF SPICE kernels:
sun, mercury, venus, earth, moon, mars, jupiter, saturn, uranus, neptune, pluto,
io, europa, ganymede, callisto, enceladus, rhea, dione, tethys, titan,
phobos, deimos, triton.

Generated weekly by `generate-ephemerides.py` from SPICE kernel files.

### Horizons API Bodies (17)

Small bodies, dwarf planets, comets, and spacecraft tracked by JPL Horizons:
ceres, vesta, eris, haumea, makemake, encke, apophis, bennu,
iss, voyager1, voyager2, new_horizons, parker_solar_probe, solar_orbiter,
jwst, juno, atlas_3i.

Generated daily by `generate-ephemerides.py` via JPL Horizons REST API.

### Binary Format

Chebyshev polynomial coefficients. 32-day granules with 25-sample fit per granule.
Parsed by `parse_ephemeris_binary()`. Evaluated at query time by
`chebyshev_evaluate()` (Clenshaw recurrence).

## 10. CDN Structure

### Repository

`omegaflow/sources` — GitHub repository with per-domain release tags.

### Release Tags

One release per API domain (netloc): `ndbc.noaa.gov`, `services.swpc.noaa.gov`,
`api.wheretheiss.at`, `api.worldbank.org`, `ssd.jpl.nasa.gov`, etc.

### Asset Naming

`{source_identifier}_{iso8601_utc}.{ext}`

- `{source_identifier}`: what the API names itself (station ID, endpoint segment, NORAD ID)
- `{iso8601_utc}`: `20260806T120000Z` (UTC, no offset)
- `{ext}`: `json` for data, `bin` for Chebyshev ephemerides

Examples:
```
ndbc.noaa.gov/46083_20260806T120000Z.json
services.swpc.noaa.gov/rtsw_wind_1m_20260806T120000Z.json
api.wheretheiss.at/25544_20260806T120000Z.json
ssd.jpl.nasa.gov/enceladus_20260806T120000Z.bin
```

### Immutability

Assets are never overwritten. Two fetches produce two distinct files.
This enables scientific reproducibility, diff-based change detection, and
temporal queries.

## 11. Parameter Reference (CSV)

The table below is the canonical parameter schema. Validators parse it as TSV.
Required columns: keyword, syntax, cardinality, description.

keyword	syntax	cardinality	description
url	url <URL>	1	Identity, anchor. Template vars: {lat}, {lon}, {grid}, {year}, {latest}
ttl	ttl <s>	1	Archivar refetch interval. Phenomenon change rate
force	force <f>	1	Field medium. Composites: space-separated
body	body <name>	1	Body name. Set implicitly by on/at
on	on <body> <lat> <lon> [alt]	0-1	Fixed surface point. Implicitly sets body
at	at <body> <scale>	0-1	Barycenter frame. Implicitly sets body
pos	pos <lat_key> <lon_key> [alt_key] [scale]	0-1	Data-carried position. Needs explicit body
format	format <f>	0-1	json (default), text, ephemeris_binary
tau	tau <s>	0-1	Oscillator decay constant
tau_key	tau_key <k>	0-1	Tau from data field
map	map <path>	0-N	Terrestrial position extract
cmap	cmap <path>	0-N	Celestial position extract (RA/Dec)
field	field <key> <name>	0-N	Scalar extract. key=JSON path, name=Quellentreu
path	path <key> <name>	0-N	Scalar extract, JPath semantics
last	last <key> <name>	0-N	Last value of JSON array
count	count <key> <name>	0-N	Count array elements
field_in	field_in <key> <name>	0-N	Column extract (columnar CDN data)
rows	rows	0-1	Row-based text data
tail	tail	0-1	Last line only
geojson	geojson events <m> <min> <names>	0-1	GeoJSON event feed
lat_key	lat_key <k>	0-N	Latitude field (attached to map)
lon_key	lon_key <k>	0-N	Longitude field (attached to map)
ra_key	ra_key <k>	0-N	Right ascension (attached to cmap)
dec_key	dec_key <k>	0-N	Declination (attached to cmap)
alt_key	alt_key <k>	0-1	Altitude (attached to map)
epoch_key	epoch_key <k>	0-1	Timestamp (attached to map/cmap)
val_key	val_key <k>	0-1	Value override
vel_key	vel_key <k>	0-1	Velocity (motion law)
trk_key	trk_key <k>	0-1	Track/heading (motion law)
vr_key	vr_key <k>	0-1	Vertical rate (motion law)
plx_key	plx_key <k>	0-1	Parallax (mas)
pmra_key	pmra_key <k>	0-1	Proper motion RA (mas/yr)
pmdec_key	pmdec_key <k>	0-1	Proper motion Dec (mas/yr)
radvel_key	radvel_key <k> <scale>	0-1	Radial velocity
dist_key	dist_key <k> <scale>	0-1	Distance + scale factor
z_key	z_key <k>	0-1	Redshift (Hubble flow)
header	header <K> "V"	0-N	HTTP request header
stations	stations <url>	0-1	Station list API
stations_path	stations_path <p>	0-1	Station list path in JSON
stations_lat	stations_lat <k>	0-1	Station latitude field
stations_lon	stations_lon <k>	0-1	Station longitude field
stations_id	stations_id <k>	0-1	Station ID field
method	method POST	0-1	HTTP method override
verify	verify false	0-1	Skip SSL verification

## 12. Validation

`verify_sources.py` checks:

1. Every block starts with `url`
2. Exactly 1× ttl, 1× force, 1× body (or on/at setting body) per block
3. 0-1 frame directives
4. No duplicate parameters
5. All output names follow Quellentreu rule (no prefixes, snake_case, SI units)
6. All force values in canonical set
7. No characters `(){}[]-` in any name
8. Blank line between blocks, no blank lines within blocks
9. No `source <name>` lines
10. `cargo run` → 0 refused

CI runs `verify_sources.py` before any CDN refresh operation.
