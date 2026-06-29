# SPECS

## 1: PROTOCOL (IS v6)

### Request (Browser → Rust)
One binary WebSocket frame containing inputs and queries.
```
u32 request_id
u32 input_count
[per input:]
  f64 t          (8 bytes)
  f64 x          (8 bytes)
  f64 y          (8 bytes)
  f64 z          (8 bytes)
  f64 value      (8 bytes)
  u8 name_len, [name_bytes]
u32 query_count
[per query:]
  f64 t, f64 x, f64 y, f64 z
```

### Response (Rust → Browser)
```
"IS"           (2 bytes magic)
u8 version     (6)
u32 request_id
u32 query_count
[per query:]
  u32 obj_count
  [per obj:] (field names + values)
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

### Universal Cache
The server caches coefficients by their universal existence.
- Key: Rasterized string `"lat_lon"` (e.g., `"48.12_11.56"`), derived from API resolution or sensor density.
- Value: `(timestamp, HashMap<Name, Value>)`.
- Contains API data AND local sensor data with absolute equality.

### `handle_pulse` (The Frame Processor)
1. Reads `input_count`. For each input: ECEF → geodetic → rasterized key. Inserts into universal cache.
2. Reads `query_count`. For each query: ECEF → geodetic → rasterized key. Registers key in `active_tiles`. Performs O(1) HashMap lookup.
3. Writes found values directly to binary output.
- *No weaving (φ), no distance calculation, and no evaluation take place here.*

### `warm_cache` (The API Fetcher)
- Runs asynchronously in the background.
- Fetches APIs for all regions (`active_tiles`) requested by queries.
- The `ecef_to_geodetic` conversion and URL rendering (`{lat}`) happen **exclusively** here, at the HTTP boundary to human servers.
- Three source types:
  - Fixed station (`lat`/`lon` set in `sources.φ`): cache key from rounded coords
  - Tile-based (`{lat}`/`{lon}` in URL): cache key = tile string
  - Global (no geo): cache key = URL
- `GeojsonEvents` extractor: caches each earthquake at its own coordinates

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
| `geojson` | `geojson events <mag_key> <min_mag> <o1> <o2>` | Caches each event at its own coordinates |

### `sources.φ` format
```
source <name>
ttl <seconds>
url <url_with_{templates}>
lat <f64>       (optional — fixed station)
lon <f64>       (optional — fixed station)
header <Name> "value"
field <key> <name>
```

Sorted by TTL ascending, then alphabetically.

## 3: CPU (JS)

Entry: `static/index.html`, `static/world.js`. Vanilla ES modules.

### `world.js`
- `get(inputs, queries)` → builds v6 binary frame, sends non-blocking
- `parseBatchPayload(bytes)` → reads objects, weaves into `result`
- `weave(p, result)` → moving average blend with `1/φ²`
- Certainty factors: `measureG()`, `measureVC()`, `measureDecay()`, `measureQuantum()`

### `index.html` — Sensor Discovery
- `discoverSensors()` / `discoverObj()` → walks `Object.getOwnPropertyNames(window)`, depth 3
- Circular set guard, `WeakSet` for visited objects
- Sensors: numbers/booleans. Actuators: functions taking arguments.
- `on*` properties → event sources → `addEventListener`

### `index.html` — Input Collection
- Sensors (Accelerometer, GPS, Battery, Gamepads, etc.) no longer write directly to `is`.
- They call `pushInput(name, value)`, which stamps them with the current `(t, x, y, z)` of the `io` and pushes them into the `inputBuffer`.

### `index.html` — Heartbeat & Batch
- Collects queries in `queryBuffer`.
- Sends the entire `inputBuffer` and `queryBuffer` as a batch once per cycle (`tickTime * φ`).
- Only the response writes to the global `is` object.

### `index.html` — Probe State Machine
- Phases: `waiting` → `probing` → `resolving`
- `startProbing(ts)` → pulse all actuators, snapshot sensors
- `checkProbing(ts)` → if resonators: resolve. Else: decay `pulse *= φ`
- `startResolving(ts, batch)` → binary split via `splitBatch()` (`φ` ratio)
- `checkResolving(ts)` → if single actuator resonates: store in `resonanceMap`
- `resonanceMap`: `Map<actuatorPath, Map<sensorPath, {pulseTone, magnitude, divergence}>>`
- `express()` → fires actuators above `μ + σ/φ` threshold
- Silent threshold: `pulse > sensors.size * φ`

### `index.html` — Interoception
- `navigator.hardwareConcurrency` → `pushInput('system.cpu', ...)`
- `performance.memory` → `pushInput('system.memory', ...)`
- `fetch('/time')` round-trip → `pushInput('system.latency', ...)`

### `index.html` — Nostr (Stigmergy)
- Connects `wss://relay.damus.io`
- Subscribes `kind: 39603`
- Publishes `kind: 39603`: `content` = flat JSON of `is` values, `geo` tag = `lat,lon`
- Publish interval: `tickTime * φ³`
- On receive: packs into `inputBuffer` via `pushInput('omega_flow.*', ...)` with ECEF stamp

## 4: GPU (WGSL)

### Ring Buffer
`ringSize = 128`. `processSensorReading()` fills `signalBuffers`.

### Shaders

#### Kolmogorov Complexity (`kolmogorovShader`)
- `workgroup_size(64)`
- Input: `data[n * 128]`, `params(n, ringSize)`
- Output: `complexity[n]` = `1 - repeats/total`
- Threshold: `sqrt(variance/ringSize) / φ²`

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
- Dynamic threshold in JS: `μ + σ/φ`

#### TDA: Persistent Homology (`tdaShader`)
- `workgroup_size(64)`
- 48-point subsample, `τ = 1 + 1/φ`
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

`is/sources.φ`. TTL range: 10s (ISS position) to 31536000s (Gaia star catalog).

### Geo Templates
```
{lat} {lon} {lat_min} {lat_max} {lon_min} {lon_max}
{today} {yesterday} {tomorrow} {hour_ago} {year} {month} {day}
```

### Fixed Stations
Tide stations (301 NOAA), geomagnetic observatories, and other fixed-location sources declare `lat`/`lon` in `sources.φ`. No runtime resolution.

## 6: DEVICES

### Browser-accessible
- Generic Sensor API: Accelerometer, Gyroscope, Magnetometer, AmbientLightSensor
- Geolocation: lat/lon/alt/accuracy/heading/speed
- Battery: charging/level/time
- Gamepad: axes/buttons
- Web Audio: `PannerNode` HRTF

### Smartwatch (Wearable API)
- Heart rate, HRV, SpO2, steps, stress, sleep stages
- Web Bluetooth / Garmin Connect API / Apple HealthKit bridge
- Pushes via `pushInput('wearable.*', value)` with ECEF stamp from phone GPS

### XR (WebXR Device API)
- Hand tracking: joints, pinch, grasp
- Eye tracking: gaze origin, gaze direction
- Spatial sensors: pose, acceleration from headset IMU
- World sensing: plane detection, mesh, depth
- Pushes via `pushInput('xr.*', value)` with ECEF stamp from headset position

### ESP32-S3 (omegaflow sense)
Specification: `docs/omegaflow_sense_hardware.yaml`

Core sensors: Telluric currents, Biophotons, 50/60Hz flicker, PM2.5, VOC/Temp/Press/Humid, EMF/Schumann, Bioacoustics.

## 7: CONSTANTS

```
C   = 299792458.0
φ = 1.618033988749895
a   = 6378137.0      (WGS84 semi-major)
f   = 1/298.257223563 (WGS84 flattening)
```
