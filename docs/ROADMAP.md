# SPECS

## 1: PROTOCOL (φ v6)

### Request (Browser → Rust)
One binary WebSocket frame containing inputs, queries, and stigmergy signals.
```
u32 request_id
u32 input_count
[per input:]
  f64 t, x, y, z, value
  u8 name_len, [name_bytes]
u32 query_count
[per query:]
  f64 t, x, y, z
u8 signal_count
[per signal:]
  u8 type (0=pulse, 1=resonant)
  u8 path_len, [path_bytes]
```

All channels are binary. No JSON on the WebSocket.

### Response (Rust → Browser)
```
φ              (2 bytes magic, 0xCF 0x86)
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
- Key: Rasterized string `"lat_lon"`, derived from API resolution or sensor density.
- Value: `(timestamp, HashMap<Name, Value>)`.
- Contains API data AND local sensor data with absolute equality.

### `handle_pulse` (The Frame Processor)
1. Reads `input_count`. For each input: ECEF → geodetic → rasterized key. Inserts into universal cache.
2. Reads `query_count`. For each query: ECEF → geodetic → rasterized key. Registers key in `active_tiles`. Performs O(1) HashMap lookup.
3. Reads `signal_count`. Routes pulse/resonant signals to dormant tracker.
4. Writes found values directly to binary output.
- *No weaving (φ), no distance calculation, and no evaluation take place here.*
- *tile_key results are cached. Repeated coordinates reuse the cached key without recomputing ECEF.*

### `warm_cache` (The API Fetcher)
- Runs asynchronously in the background.
- Fetches APIs for all regions (`active_tiles`).
- ECEF conversion and URL rendering happen exclusively here.
- Connect timeout: `TTL / Φ³`. Max timeout: `TTL / Φ²`.
- Sleep when idle: `min_TTL / Φ`. Sleep after cycle: `min_TTL / Φ²`.
- Thread fallback when `available_parallelism()` fails: 1.
- Single `SystemTime::now()` call per `render_url` invocation.

### Dormant Tracking
- One function serializes and writes. Not two.
- Function is named `φ_obj`, not `is_obj`.

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

## 3: CPU (JS)

Entry: `static/index.html`, `static/world.js`. Vanilla ES modules.

### `world.js` — The Shared Universe
Exports the constants Φ, C, WGS84_A, WGS84_F, MA, and the function `j2000()`.
`index.html` imports these. No redeclaration.

- `get(inputs, queries)` → builds v6 binary frame (including signal section), sends non-blocking
- Binary parsing weaves directly into `result` during parse. No intermediate object array.
- `weave(p, result)` → moving average blend with `MA = 1/Φ²` (precomputed once)
- Certainty factors: `measureG()`, `measureVC()`, `measureDecay()`, `measureQuantum()`
- `j2000(unixSecs)` → single source of truth for J2000 conversion
- `certainty` is returned and flows back into the organism's behavior

### `index.html` — Constants
Imports Φ, C, WGS84, MA, j2000 from `world.js`. Does not declare them.

### `index.html` — Sensor & Actuator Discovery
- `discoverSensors()` / `discoverObj()` → walks `Object.getOwnPropertyNames(window)`, depth 3
- Circular set guard, `WeakSet` for visited objects
- Sensors: numbers/booleans
- Actuators: functions taking arguments AND expression channels
- Expression channels discovered and connected: Canvas, Web Audio (oscillator → panner → destination), Vibration, CSS properties
- `on*` properties → event sources → `addEventListener`

### `index.html` — Input Collection
- All sensors call `pushInput(name, value)`, stamped with current `(t, x, y, z)`
- ECEF conversion runs only when geolocation values change, not every tick

### `index.html` — Heartbeat & Batch
- Sends `inputBuffer` and `queryBuffer` as a batch once per cycle
- Pulse and resonant signals sent as binary opcodes in the same frame
- Tick rate scales with certainty: `nextDelay = stableTick / certainty`

### `index.html` — Certainty
- `certainty` influences tick rate, probe intensity, and expression threshold
- High certainty → faster ticks, deeper probing, stronger expression
- Low certainty → slower ticks, restful state

### `index.html` — Probe State Machine
- Phases: `waiting` → `probing` → `resolving`
- `splitBatch` ratio varies: sometimes Φ, sometimes derived from quantum entropy
- Curiosity decision uses `quantum_entropy_anu` instead of `Math.random()`
- Silent threshold: `pulse > sensors.size * Φ`

### `index.html` — Complexity
- JS computes `s.median` and push-threshold (fast approximation)
- GPU computes true Kolmogorov complexity and writes `s.complexity`
- JS does not overwrite GPU complexity

### `index.html` — Expression
- `express()` fires all actuator types: functions, Canvas pixels, Audio frequencies, Vibration pulses
- Screen is an active IO channel, not a black void

### `index.html` — Gamepad Hash
- Hash uses Φ-based mixing, not `*31` and `Math.pow(2,16)`

### `index.html` — Interoception
- `navigator.hardwareConcurrency` → `pushInput('system.cpu', ...)`
- `performance.memory` → `pushInput('system.memory', ...)`
- `fetch('/time')` round-trip → `pushInput('system.latency', ...)`

### `index.html` — Nostr (Stigmergy)
- Connects `wss://relay.damus.io`
- Subscribes `kind: 39603`
- Publishes `kind: 39603`: `content` = flat JSON of `φ` values, `geo` tag = `lat,lon`
- Publish interval: `tickTime * Φ³`
- On receive: compares received values with own `φ` values. Inter-node resonance measured as divergence between local and remote certainty.

## 4: GPU (WGSL)

### Ring Buffer
`ringSize = 128`. `processSensorReading()` fills `signalBuffers`.

### Shaders

#### Kolmogorov Complexity (`kolmogorovShader`)
- `workgroup_size(64)`
- Output: `complexity[n]` = `1 - repeats/total`
- Threshold: `sqrt(variance/ringSize) / Φ²`

#### Takens Embedding (`takensShader`)
- `workgroup_size(64)`
- Mutual Information finds optimal `τ` (first local minimum)
- 3D attractor reconstruction: `x[t], x[t+τ], x[t+2τ]`
- Output per signal: `cx, cy, cz, spread`

#### Transfer Entropy (`teShader`)
- `workgroup_size(8, 8)`
- 3-bin histogram
- Output: `te[a*n+b] = max(0, te_val)`
- Dynamic threshold in JS: `μ + σ/Φ`

#### TDA: Persistent Homology (`tdaShader`)
- `workgroup_size(64)`
- 48-point subsample, `τ = 1 + 1/Φ`
- Output: persistence lifetime, Betti-0

#### ICA: Blind Source Separation (`icaShader`)
- `workgroup_size(64)`
- FastICA: `tanh` non-linearity
- Separates mixed signals into independent sources
- Output per signal: independent component magnitude

### GPU Bindings
All pipelines share 3-entry bind group layout:
```
binding 0: storage (read)  — input data
binding 1: storage (write) — output
binding 2: uniform         — params vec4(n, ringSize, 0, 0)
```

## 5: SOURCES

`φ/sources.φ`. TTL range: 10s (ISS position) to 31536000s (Gaia star catalog).

### Geo Templates
```
{lat} {lon} {lat_min} {lat_max} {lon_min} {lon_max}
{today} {yesterday} {tomorrow} {hour_ago} {year} {month} {day}
```

## 6: DEVICES

### Browser-accessible
- Generic Sensor API: Accelerometer, Gyroscope, Magnetometer, AmbientLightSensor
- Geolocation: lat/lon/alt/accuracy/heading/speed
- Battery: charging/level/time
- Gamepad: axes/buttons
- Web Audio: Oscillator → PannerNode HRTF → destination (expression)
- Canvas: pixel output (expression)
- Vibration API (expression)

### ESP32-S3 (omegaflow sense)
Specification: `docs/omegaflow_sense_hardware.yaml`

## 7: CONSTANTS

```
C   = 299792458.0
Φ   = 1.618033988749895
MA  = 1/Φ²            (precomputed moving-average weight)
a   = 6378137.0       (WGS84 semi-major)
f   = 1/298.257223563 (WGS84 flattening)
```
