# SPECS

## 1: PROTOCOL (IS v4)

### Request (Browser → Rust)
36 byte WebSocket binary frame:
```
f64 t          (8 bytes)
f64 x          (8 bytes)
f64 y          (8 bytes)
f64 z          (8 bytes)
u32 request_id (4 bytes)
```

### Response (Rust → Browser)
```
"IS"           (2 bytes magic)
u8 version     (4)
u32 obj_count
```

### Object
```
u8 field_count
[field_name: u8 len, utf8 bytes, 0x00 terminator]
[f64 value per field]
u32 record_count
```

## 2: CPU (RUST)

Entry: `src/main.rs`. Rust std. Flat hierarchy.

### Constants
```
PHI     = 1.618033988749895
WGS84_A = 6378137.0
WGS84_F = 1.0 / 298.257223563
```

### Bindings
- `sources.is` → parsed at boot into `Vec<SourceConfig>`
- WebSocket listener → `handle_pulse()` receives request, calls `weave()`, returns response
- `curl` subprocess → `fetch_with_headers()`, 8s timeout, 4s connect

### `weave(payload, archive) -> Vec<u8>`
1. Extract `(t, x, y, z)` from first 32 bytes
2. Calculate ECEF distance `r = sqrt(x²+y²+z²)`
3. `on_earth = r > 6e6 && r < 7.5e6`
4. If `on_earth`: convert ECEF → geodetic (WGS84), resolve nearest geo lookups
5. Deliver stigmergy cache (`stig_{lat:.1}_{lon:.1}`) as `omega_flow.*` fields if data exists and age < 60s
6. Iterate `archive.sources` (sorted by TTL ascending, then alphabetically):
   - Skip `nostr://` URLs
   - Render URL templates (`{lat}`, `{lon}`, etc.)
   - Check cache (`Mutex<HashMap>`): if age < `ttl`, use cached body
   - Else: fetch via curl, store in cache
   - Run extractors on body, emit `is_obj`
7. Write `obj_count` into output

### Extractors
| Keyword | Syntax | Output |
|---|---|---|
| `field` | `field <json_key> <name>` | `jnum()` |
| `first` | `first <arr_key> <name>` | `jarr_first()` |
| `last` | `last <arr_key> <name>` | `jarr_last()` |
| `count` | `count <arr_key> <name>` | `jarr_count()` |
| `sum` | `sum <key> <name>` | `jsum()` |
| `last_row` | `last_row <col> <name>` | `j2d_last_row()` or `text_last_col()` |
| `path` | `path <dotted.path> <name>` | `jpath()` |
| `vector` | `vector <nx> <ny> <nz>` | `text_vector()` |
| `last_obj` | `last_obj "fk" "fv" "ek" "name"` | `jobj_last_match()` |
| `geojson` | `geojson nearby <max_dist_m> <mag_key> <min_mag> <o1> <o2> <o3>` | Haversine filter |

### `sources.is` format
```
source <name>
ttl <seconds>
url <url_with_{templates}>
header <Name> "value"
field <key> <name>
```

Sorted by TTL ascending, then alphabetically.

### Cache
`Mutex<HashMap<String, (u64 timestamp, String body)>>`. Keyed by URL path + lat/lon.

### Stigmergy
`Mutex<HashMap<String, (u64 timestamp, String json)>>`. Keyed by `stig_{lat:.1}_{lon:.1}`. Fed by browser WebSocket text messages: `{"pulse":"lat|lon|json"}`.

## 3: CPU (JS)

Entry: `static/index.html`, `static/world.js`. Vanilla ES modules.

### `world.js`
- `get(t, x, y, z)` → sends 36-byte request, parses IS v4 response
- `parsePayload(bytes)` → reads objects, populates `is` object
- `weave(p, result)` → moving average blend with `1/PHI²`
- `doFetch()` → WebSocket binary frame, Promise-based with `request_id` tracking
- Certainty factors: `measureG()`, `measureVC()`, `measureDecay()`, `measureQuantum()`, `measureEpigenetics()`

### `index.html` — Sensor Discovery
- `discoverSensors()` / `discoverObj()` → walks `Object.getOwnPropertyNames(window)`, depth 3
- Circular set guard, `WeakSet` for visited objects
- Sensors: numbers/booleans. Actuators: functions taking arguments.
- `on*` properties → event sources → `addEventListener`

### `index.html` — Probe State Machine
- Phases: `waiting` → `probing` → `resolving`
- `startProbing(ts)` → pulse all actuators, snapshot sensors
- `checkProbing(ts)` → if resonators: resolve. Else: decay `pulse *= PHI`
- `startResolving(ts, batch)` → binary split via `splitBatch()` (`PHI` ratio)
- `checkResolving(ts)` → if single actuator resonates: store in `resonanceMap`
- `resonanceMap`: `Map<actuatorPath, Map<sensorPath, {pulseTone, magnitude, divergence}>>`
- `express()` → fires actuators above `μ + σ/PHI` threshold
- Silent threshold: `pulse > sensors.size * PHI`

### `index.html` — Interoception
- `navigator.hardwareConcurrency` → `system.cpu`
- `performance.memory` → `system.memory`, `system.memoryTotal`
- `fetch('/time')` round-trip → `system.latency`

### `index.html` — Nostr (Stigmergy)
- Connects `wss://relay.damus.io`
- Subscribes `kind: 39603`
- Publishes `kind: 39603`: `content` = flat JSON of `is` values, `geo` tag = `lat,lon`
- Publish interval: `tickTime * PHI³`
- On receive: forwards `{"pulse":"lat|lon|json"}` to Rust via pulse WebSocket

## 4: GPU (WGSL)

### Ring Buffer
`ringSize = 128`. `processSensorReading()` fills `signalBuffers`.

### Shaders

#### Kolmogorov Complexity (`kolmogorovShader`)
- `workgroup_size(64)`
- Input: `data[n * 128]`, `params(n, ringSize)`
- Output: `complexity[n]` = `1 - repeats/total`
- Threshold: `sqrt(variance/ringSize) / PHI²`

#### Takens Embedding (`takensShader`)
- `workgroup_size(64)`
- Mutual Information finds optimal `τ` (first local minimum)
- 3D attractor reconstruction: `x[t], x[t+τ], x[t+2τ]`
- Output per signal: `cx, cy, cz, spread`

#### Transfer Entropy (`teShader`)
- `workgroup_size(8, 8)`
- 3-bin histogram: `to_bin(v, min, max)` → 0, 1, or 2
- `P(bn+1 | bn, an)` vs `P(bn+1 | bn)` for all N² pairs
- Output: `te[a*n+b] = max(0, te_val)`
- Dynamic threshold in JS: `μ + σ/PHI`

#### TDA: Persistent Homology (`tdaShader`)
- `workgroup_size(64)`
- 48-point subsample, `τ = 1 + 1/PHI`
- Insertion sort of nearest-neighbor distances
- Union-Find parent tracking
- Output: persistence lifetime, Betti-0

#### ICA: Blind Source Separation (`icaShader`)
- `workgroup_size(64)`
- FastICA: `tanh` non-linearity, 3 iterations
- Weight update + normalization per iteration
- Output per signal: variance, amplitude
- Dynamic source count via variance cutoff in JS

### GPU Bindings
All pipelines share 3-entry bind group layout:
```
binding 0: storage (read)  — input data
binding 1: storage (write) — output
binding 2: uniform         — params vec4(n, ringSize, 0, 0)
```

## 5: SOURCES

`is/sources.is`. 229 sources. TTL range: 10s (ISS position) to 31536000s (Gaia star catalog).

### Geo Templates
```
{lat} {lon} {lat_min} {lat_max} {lon_min} {lon_max}
{today} {yesterday} {tomorrow} {hour_ago} {year} {month} {day}
{nearest_buoy} {nearest_tide_station} {nearest_geomag_station}
{nearest_airport} {nearest_site} {nearest_observatory}
```

### Geo Resolution
- USGS site lookup via `waterservices.usgs.gov`
- NDBC buoy lookup via `ndbcmapstations.json`
- Tide station via `tidesandcurrents.noaa.gov`
- Airport/radiosonde via `ourairports-data`
- Intermagnet, NMDB, AERONET, SURFRAD via `is/lookups.is`
- Country code via `bigdatacloud.net`
- Cached 24h in `Mutex<HashMap>`

## 6: DEVICES

### Browser-accessible
- Generic Sensor API: Accelerometer, Gyroscope, Magnetometer, AmbientLightSensor
- Geolocation: lat/lon/alt/accuracy/heading/speed
- Battery: charging/level/time
- Gamepad: axes/buttons
- Web Audio: `PannerNode` HRTF

### ESP32-S3 (omegaflow sense)
Specification: `docs/omegaflow_sense_hardware.yaml`

Core sensors: Telluric currents, Biophotons, 50/60Hz flicker, PM2.5, VOC/Temp/Press/Humid, EMF/Schumann, Bioacoustics.

## 7: CONSTANTS

```
C   = 299792458.0
PHI = 1.618033988749895
a   = 6378137.0      (WGS84 semi-major)
f   = 1/298.257223563 (WGS84 flattening)
```
