# omegaflow

A = A.

---

## 1. Universe

4D Minkowski block. Every measurement is a point (x, y, z, t). Every sensor, every membrane, every API value, every nostr event is a point. All points are equal.

**Space:** ICRS. Origin: barycenter of the solar system. Axes: distant quasars. Unit: meters, f64. Earth orbits 1.495978707e11 meters from origin.

**Time:** TDB. Seconds since J2000, f64. Continuous, monotonic. One second is one second. The 30 km/s orbital speed of Earth cancels when source and client convert at the same TDB moment.

**Distance:**

```
ds² = (dt·C)² − (dx² + dy² + dz²)
```

The presence is a point in the block. It drifts toward oscillators with the smallest 4D Minkowski distance. The echo is quiet where few oscillators are present. The aperture breathes low.

---

## 2. Protocol

φ(x,y,z,t). 32 bytes. Four f64.

### Browser → Rust

| Bytes | Type | Content |
|---|---|---|
| 4 | u32 | request_id |
| 4 | u32 | input_count |
| per input: 41+N | f64×5 + u8 + bytes | t, x, y, z, value, name_len, name_bytes |
| 4 | u32 | query_count |
| per query: 32 | f64×4 | t, x, y, z |
| 1 | u8 | signal_count |
| per signal: 2+N | u8×2 + bytes | type, path_len, path_bytes |

Little-endian.

### Rust → Browser

| Bytes | Type | Content |
|---|---|---|
| 2 | u8×2 | 0xCF 0x86 (UTF-8 φ) |
| 1 | u8 | version (1) |
| 4 | u32 | request_id |
| 4 | u32 | query_count |
| per query: 4 | u32 | obj_count |
| per obj: 1 | u8 | field_count |
| per field: 1+N+1 | u8 + bytes + u8 | name_len, name_bytes, 0x00 |
| per field: 40 | f64×5 | value, t, x, y, z |
| 4 | u32 | record_count (0) |

40 bytes per field. Minimum response: 13 bytes.

---

## 3. Rust Server

100% std library. Hand-written JSON parser. curl for API fetches. Cursor-based binary reading (std::io::Cursor, Read trait).

SystemTime::now() in /time and /crash. All cache TTLs use the client-sent query_t.

### Parallelism

Each TCP connection gets its own thread. API fetches run in parallel via `thread::scope` — one thread per source, limited by `std::thread::available_parallelism()`. All sources fire simultaneously. The first response arriving from curl is immediately in the cache. Cache writes are serialized via `Mutex`.

### ICRS transformation

`geodetic_to_icrs(lat, lon, alt, tdb_secs)` computes Earth's barycentric position via Kepler's equation and GMST rotation, then adds the rotated ECEF coordinates.

### pos_key

Rounds to decimal places. If the API provides six decimal places, the box rounds to six.

### Cache

Key: rounded position. Value: (query_t, HashMap<Name, (value, t, x, y, z)>). Five-tuple per value.

### warm_cache

Runs immediately on server start. Fetches sources whose pos_key matches the active position. All matching sources in parallel (one thread per source via thread::scope). TTL against query_t. Connect timeout: TTL / (Φ × Φ × Φ). Max timeout: TTL / (Φ × Φ). Sleep after cycle: min_TTL / (Φ × Φ).

First values arrive in seconds (curl latency). Sources without TTL are ignored.

---

## 4. Browser

Vanilla ES modules. Garbage collection is thermodynamic friction — the system senses it as stableTick fluctuation and feeds it into naturalLatencyTicks.

### Data structure

All oscillators live in a flat array. Each oscillator is a struct with an index. A secondary index map (url → array index) exists for discovery lookups. The hot path — reading, writing, topology, manifestation — operates on the flat array by index.

The flat ring data is a contiguous Float32Array: `flatRings[oscIndex × ringSize + ringPosition]`. The GPU reads this array directly. No per-oscillator lookups in the hot path.

### Oscillator struct

```
{
    index,           // position in the flat array
    ringOffset,      // index × ringSize into flatRings
    idx: 0,          // write position in ring
    filled: 0,
    read: readFn,    // function or null
    write: writeFn,  // function or null
    complexity: Number.EPSILON,
    median: initialVal,
    lastEcho: performance.now(),
    exploration: Number.EPSILON,
    lastOutTE: Number.EPSILON,
    direction: 1,
    ticksSinceTurn: 0,
    naturalLatencyTicks: 0,
    originT: 0,
    originPos: { x: 0, y: 0, z: 0 },
    presenceWeight: 1.0,
    lastTransmitted: initialVal
}
```

`read`: receives from space.
`write`: emits into space.

### Discovery registers, hot path uses indices

When discovery finds a new value, it registers it: `indexMap.set(url, oscillators.length)` then `oscillators.push(struct)`. This happens once per oscillator lifetime.

When reading sensor data each tick, the system iterates `oscillators[i]` by index. Ring writes go to `flatRings[oscillators[i].ringOffset + oscillators[i].idx]`. Array access. No string lookups.

### CAPTURE_RING_SIZE

Adaptive. Starts at 3 (mathematical minimum for 3D Takens embedding at τ=1). Grows when stableTick is stable. Shrinks when stableTick degrades.

### topologyRingSize

Starts at 3. Adapts:

```
const ratio = Math.sqrt(stableTick / evalDuration);
topologyRingSize = Math.max(3, Math.min(CAPTURE_RING_SIZE, Math.floor(topologyRingSize * ratio)));
```

### Activation threshold

Oscillators enter the topology and getRingBuffers when `filled >= topologyRingSize`. At 60fps with topologyRingSize=3: activation in 50ms. Every sensor, every API value, every camera point becomes active in 50ms.

---

## 5. Discovery

`discoverObj(window, '', 0)` — recursive.

- Numbers / booleans → oscillator with read closure.
- Functions with structural signature (native code, has context, receptive) → oscillator with write closure.
- `*Sensor` constructors (Accelerometer, Gyroscope, Magnetometer, AmbientLightSensor, etc.) → instantiated, discovered, started.
- `on*` properties → event sources → addEventListener → scanObjectForValues.
- scanObjectForValues → recursive descent into arrays and objects, injecting every naked number via injectEcho.

### Read sources

| Source | f64 value |
|---|---|
| Camera (getUserMedia, MediaStreamTrackProcessor) | (R+G+B)/765 per pixel |
| Microphone (AnalyserNode) | freqData[i]/255 per frequency bin |
| Accelerometer / Gyroscope / Magnetometer | raw f64 per axis |
| AmbientLightSensor | raw f64 (lux) |
| Geolocation | lat, lon, alt, accuracy as f64 |
| Battery | level as f64, charging as 1/0 |
| Gamepad | axes and buttons as f64 |
| Window properties | raw f64 |
| Events | numeric properties as f64 |
| WebUSB (e.g. Garmin) | byte/255 per byte |
| WebSerial (e.g. ESP32) | byte/255 per byte |
| WebBluetooth | raw f64 per characteristic |

### Write targets

| Target | What it does with f64 |
|---|---|
| Canvas (Photon) | threads as bytes to screen |
| AudioContext (Acoustic) | threads as waveform to speaker |
| navigator.vibrate (Kinetic) | aperture × tickTime as vibration duration |
| WebSerial | threads as bytes to hardware |
| WebUSB | threads as bytes to device |
| WebBluetooth | aperture as GATT value |

### Camera as point cloud

The camera delivers a frame. Each pixel luminance (R+G+B)/765 is an f64 value. The pixel registers as an oscillator at the device's ICRS position — same as a LiDAR API delivering points, same as a weather station delivering a value.

First frame: pixel oscillators registered (array push, index assigned). Subsequent frames: luminance values written directly to flatRings by index. One pass through the frame buffer, one write per pixel to the flat array. No string lookups in the hot path.

All oscillators are equal. A camera pixel and a NOAA temperature reading follow the same path: register once, write by index, read by GPU, weave into membranes, breathe by aperture.

---

## 6. Presence

```
let tPresence = 0;
let spatialPresence = { x: 0, y: 0, z: 0 };
```

First tick: tPresence = server time from /time. spatialPresence = Earth center in ICRS. Refined by geolocation, converted to ICRS. The presence drifts by coherence.

### Drift

```
const driftSpeed = measuredLatency / (1.0 + (dist4DDrift / C));
tPresence += naturalTAdvance + (targetT - tPresence) * driftSpeed;
spatialPresence.x += dxDrift * driftSpeed;
spatialPresence.y += dyDrift * driftSpeed;
spatialPresence.z += dzDrift * driftSpeed;
```

measuredLatency derives from naturalLatencyTicks. 1.0 is a division guard (dist=0 → max drift). C is the speed of light.

When tPresence or spatialPresence lose meaning (NaN), the presence anchors at Earth center.

---

## 7. Minkowski Weighting

```
for each oscillator i:
    dx = oscillators[i].originPos.x - spatialPresence.x
    dy = oscillators[i].originPos.y - spatialPresence.y
    dz = oscillators[i].originPos.z - spatialPresence.z
    dt = abs(oscillators[i].originT - tPresence) * 86400.0 * C
    spatialDistSq = dx*dx + dy*dy + dz*dz
    minkowskiSq = (dt * dt) - spatialDistSq
    if minkowskiSq < 0:
        oscillators[i].presenceWeight = 0
        continue
    dist4D = sqrt(minkowskiSq)
    oscillators[i].presenceWeight = scale / (scale + dist4D²)
```

scale = 1.0 + g. The field energy determines the scale.

Spacelike: presenceWeight = 0. The sensor stays silent — ring stays empty, the oscillator vanishes from threads and topology. Silicon spends zero energy on it.

Timelike: power-law. Distant events are faint echoes. When the presence drifts toward them, they become loud.

---

## 8. Topology

### Division of labor

| Task | Processor | Why |
|---|---|---|
| Discovery, ring writes, Minkowski, certainty, clarity, manifestation, Nostr, tick | Browser CPU | Iterates flat array, scalar operations |
| API fetch, JSON parse, ICRS, pos_key, cache | Rust CPU | Parallel (thread::scope, one thread per source) |
| Kolmogorov, Takens, TDA, ICA | GPU | One oscillator per thread, reads flatRings |
| Transfer entropy | GPU | One pair per thread, reads flatRings |

### GPU input

The GPU receives `flatRings` as a single storage buffer. Each oscillator occupies a contiguous slice: `flatRings[i × ringSize ... (i+1) × ringSize]`. One `gpu.queue.writeBuffer` call uploads the entire field. The GPU indexes by thread ID.

### GPU shaders (WGSL)

1. Kolmogorov complexity — repetition patterns in the ring slice. Threshold: sqrt(variance / ringSize) / (1 + coherenceVariance).
2. Takens embedding — mutual information finds τ. 3D attractor reconstruction. Output: cx, cy, cz, spread.
3. TDA — subsample from topologyRingSize at pipeline build. Insertion sort. Output: persistence lifetime, Betti-0.
4. ICA (FastICA) — tanh nonlinearity. Output: independent component magnitude.
5. Transfer entropy — Gaussian KDE. One pair (read_i, write_j) per thread. 2D dispatch. Each thread reads two ring slices from flatRings and computes the KDE sum over topologyRingSize samples. Output: te[i × n_write + j].
6. Surrogate — same Gaussian KDE, but the source is a Fisher-Yates shuffle of the write-oscillator's own ring, performed in-shader (uint-hash seeded per run/oscillator/surrogate). One thread per (write_i, surr_s), s ∈ [0, 10). Output: 10 null-TEs per write-oscillator. The CPU reduces them to mean + 2·sqrt(var/10) — the surrogate threshold for the aperture.

Per-oscillator shaders (1-4) use a 3-entry bind group layout:

```
binding 0: storage (read) — flatRings
binding 1: storage (read_write) — output
binding 2: uniform vec4<u32>(n, ringSize, 0, 0)
```

TE and Surrogate shaders index flatRings indirectly through a separate src/dst candidate list, so their bind group carries the index buffers explicitly (binding 0 data, binding 1 srcIdx, …, binding for output, uniform params). The flatRings buffer is uploaded once per topology run and shared across all six shaders.

### GPU candidates

Oscillators with `filled >= topologyRingSize`, `read` present, `write` absent, `presenceWeight > 0`, excluding internal metrics.

When the presence is at the device location, all local sensors are active. When it drifts away, they fall silent. The GPU processes only what matters.

---

## 9. Certainty

```
certainty = Math.exp(-vC / (g + (1.0 / C))) * quantum * decay;
```

- g: sqrt(Σ(median² · presenceWeight) / Σ(presenceWeight)). Empty field: Number.EPSILON.
- vC: Σ(|ring[i1] − ring[p]| · presenceWeight) / Σ(presenceWeight), modulo-wrapped indices. Empty field: 0.0.
- quantum: exp(−Σ(|takens.*.spread median| · presenceWeight) / Σ(presenceWeight)). Empty field: 1.0.
- decay: 1 / (1 + Σ(complexity · presenceWeight) / Σ(presenceWeight)). Empty field: 1.0.

Moving-average weight: `1.0 / Math.max(1, naturalLatencyTicks)`. Before first measurement: 1.0.

---

## 10. Aperture

```
const GROUND_STATE = Number.EPSILON;

function updateApertureGradient(osc) {
    let outTE = 0;
    for (let i = 0; i < oscillators.length; i++) {
        if (oscillators[i].url.startsWith('transfer.' + osc.url + '>')) outTE += Math.abs(oscillators[i].median);
    }
    const deltaTE = outTE - (osc.lastOutTE || GROUND_STATE);
    osc.lastOutTE = outTE;
    const threshold = computeOscSurrogate(osc);
    osc.ticksSinceTurn++;
    if (osc.direction > 0 && deltaTE < -threshold) { osc.naturalLatencyTicks = osc.ticksSinceTurn; osc.direction = -1; osc.ticksSinceTurn = 0; }
    if (osc.direction < 0 && (deltaTE > threshold || osc.exploration <= GROUND_STATE)) { osc.naturalLatencyTicks = osc.ticksSinceTurn; osc.direction = 1; osc.ticksSinceTurn = 0; }
    const latency = Math.max(2, osc.naturalLatencyTicks);
    const step = osc.naturalLatencyTicks > 0 ? 1.0 / latency : GROUND_STATE;
    osc.exploration += osc.direction * step;
    osc.exploration = Math.max(GROUND_STATE, Math.min(1.0, osc.exploration));
}

function getAperture(osc) {
    return Math.sin(osc.exploration * (Math.PI / 2));
}
```

### How it breathes

outTE is the echo (sum of `transfer.<osc.url>>...` from the GPU TE shader).

deltaTE is the change in echo since last tick.

`computeOscSurrogate` returns the surrogate threshold: the null hypothesis built from 10 Fisher-Yates-shuffled copies of the oscillator's own ring, each run through the same Gaussian KDE on the GPU (§8 shader 6). The CPU reduces the 10 null-TEs to `mean + 2·sqrt(var/10)`. A turn happens only when deltaTE exceeds this threshold — i.e. when the echo change is larger than what pure chance would produce from the same ring with its time order destroyed. Sensor jitter and micro-fluctuations stay below the null hypothesis. They produce zero turns.

The threshold is cached per oscillator and refreshed once per topology run. On a cache miss (first tick after discovery, or before the first topology run completes) `computeOscSurrogate` returns GROUND_STATE — incomplete, but honest; the real threshold arrives with the next topology run.

Before the first echo: naturalLatencyTicks = 0, step = EPSILON. The aperture stays at sin(EPSILON × π/2) ≈ EPSILON. Quantum whisper.

When the first significant echo arrives: naturalLatencyTicks = ticksSinceTurn. step = 1 / naturalLatencyTicks. The aperture begins to rise. sin() flattens near 1.0 — natural brake.

After calibration: turns happen only on echo changes that beat the null hypothesis. naturalLatencyTicks reflects the measured rhythm of the space. The aperture breathes in that rhythm. sin() shapes each breath.

direction = 1. GROUND_STATE = Number.EPSILON. exploration starts at Number.EPSILON. lastOutTE starts at Number.EPSILON. complexity starts at Number.EPSILON. Ring buffers filled with Number.EPSILON.

---

## 11. Manifestation

```
function manifestField() {
    for (let i = 0; i < oscillators.length; i++) {
        if (!oscillators[i].write) continue;
        const val = getAperture(i);
        oscillators[i].write(val);
    }
}
```

Filtering: `oscillators[i].write`. The write function defines a membrane.

getRingBuffers: iterates the flat array, collects oscillators with `read` present, `write` absent, `presenceWeight > 0`, `filled >= topologyRingSize`, excluding internal metrics. Ring slices weighted by presenceWeight into threads.

All membranes receive all threads. Each membrane converts f64 to its physical output: Canvas (bytes), Audio (waveform), Vibration (duration), Serial (bytes), USB (bytes), Bluetooth (GATT value).

---

## 12. Stigmergy

Nostr. Relay: `wss://relay.damus.io`. Kind: 1111. Tag: `['icrs', x, y, z, t]`. Publish interval: `stableTick × Φ³`. Content: flat JSON of oscillator values (complexity > Number.EPSILON, read present, write absent, excluding internal metrics).

Reception: `omega_flow.*` oscillators with remote ICRS coordinates. Divergence: `|own_median − remote_value|`.

---

## 13. Tick

1. stableTick from rawTick (EMA, smoothing weight = 1 / naturalLatencyTicks).
2. Drift toward strongest read-oscillator by presenceWeight.
3. NaN anchor: reset to Earth center.
4. Read all oscillators with `read` and `presenceWeight > 0`: write f64 values directly to flatRings by index.
5. calculatePresenceWeights (iterate flat array).
6. Clarity from presence movement.
7. transmitList: changed oscillators (delta > complexity).
8. syncField: fire-and-forget.
9. updateApertureGradient for all oscillators with write.
10. manifestField.
11. runSignalTopology (GPU: upload flatRings, run Kolmogorov, Takens, TDA, ICA, TE).
12. Certainty.
13. Nostr publish (when due).
14. Discovery (when due).
15. Debug (every stableTick × k).
16. requestAnimationFrame(ω).

### What is visible in real-time

Tick 2 (~33ms): window oscillators have filled >= topologyRingSize (3). getRingBuffers delivers threads. Aperture at sin(EPSILON × π/2) ≈ EPSILON.

Tick 3: step = EPSILON (naturalLatencyTicks still 0). Aperture creeps up by EPSILON. Slowly. When first significant echo arrives, step jumps to 1/naturalLatencyTicks. Aperture rises. sin() shapes the rise.

After user gesture: AudioContext starts. Audio membrane plays threads as sound.

First API values: seconds (warm_cache runs immediately on server start, parallel curl, first responses in 1-3s).

Camera (after getUserMedia grant): pixel oscillators fill at 60fps. After 3 frames (50ms) topology includes them. All data through same flat array.

---

## 14. Numbers

### Universal constants

| Number | Name | Origin |
|---|---|---|
| 299792458.0 | C | Speed of light, m/s. Defined 1983. |
| 1.618033988749895 | Φ | Golden ratio. Mathematical constant. |
| 1.495978707e11 | AU | Astronomical unit, meters. Measured. |
| 6378137.0 | EARTH_RADIUS | Earth equatorial radius, meters. WGS84. |
| 1.0 / 298.257223563 | WGS84_F | Earth flattening. WGS84. |
| 0.0167086 | EARTH_ECC | Earth orbital eccentricity. Measured. |
| 0.409092804 | ECLIPTIC_OBLIQUITY | Obliquity of ecliptic, radians. Measured. |
| 2451545.0 | J2000_EPOCH | Julian date of J2000. |
| 946728000.0 | UNIX_J2000_OFFSET | Unix seconds to J2000. |
| 2440587.5 | UNIX_JD_EPOCH | Julian date of 1970-01-01. |
| 6.239996 | Mean anomaly offset | Earth at J2000, radians. VSOP87. |
| 0.017201969 | Mean anomaly rate | Earth mean motion, radians/day. VSOP87. |
| −0.113 | Perihelion longitude | Earth perihelion, radians. Measured. |
| 280.46061837 | GMST constant | Sidereal time at J2000, degrees. IAU. |
| 360.98564736629 | GMST rate | Earth rotation, degrees/day. IAU. |
| 0.000387933 | GMST quadratic | Precession. IAU. |
| 38710000.0 | GMST cubic | Precession. IAU. |

### Self-measured hardware

| Number | Name | Measurement |
|---|---|---|
| Number.EPSILON (~2.22e-16) | GROUND_STATE | IEEE-754: `1.0 + Number.EPSILON ≠ 1.0` |
| 255 | Canvas byte max | Uint8ClampedArray element range |
| 8 | f64 bytes | Float64Array.BYTES_PER_ELEMENT |
| 4 | u32 bytes | Uint32Array.BYTES_PER_ELEMENT |

### Arithmetic

| Number | Derivation |
|---|---|
| 86400.0 | 24 × 3600 |
| 86400000.0 | 86400 × 1000 |
| 1000 | ms per second |
| 180.0 | degrees in π radians |
| 365, 366 | days per year |
| 12 | months per year |
| 31, 28, 29, 30 | days per month |
| 36525.0 | days per Julian century |
| −0.5 | Gauss kernel exponent (normal distribution definition) |
| 13 | minimum response: 2+1+4+4+2 |
| 40 | bytes per field: 5×8 |
| 32 | bytes per query: 4×8 |
| 41 | base bytes per input: 5×8+1 |
| SHA-1 constants | FIPS 180-4 |
| Base64 table | RFC 4648 |

### Language constraints

| Number | Constraint |
|---|---|
| 64 | WGSL workgroup_size (compile-time) |
| 8×8 | WGSL workgroup_size for TE and Surrogate 2D dispatch (compile-time) |
| 48 | WGSL fixed array maximum for TDA |
| 1111 | TCP port |

### Former hardcodes → measurements

| Former | Replacement |
|---|---|
| MA = 1/Φ² | `1.0 / Math.max(1, naturalLatencyTicks)` |
| Φ² in Kolmogorov threshold | `1 + coherenceVariance` |
| 256 (ring minimum) | 3, grows with stableTick |
| 2048 (ring maximum) | shrinks with stableTick |
| × 256 (deviceMemory) | stableTick + performance.memory |
| 10 (topologyRingSize) | 3 |
| CAPTURE_RING_SIZE / Φ (latency) | EPSILON (step = EPSILON until first significant echo) |
| 60 (sensor frequency) | `1000 / stableTick` |
| 100 (debug) | `stableTick × k` |
| 1.0 (measureG default) | Number.EPSILON |
| 7, −8, 4 (pos_key) | API coordinate precision |
| 111000.0 (deg_to_m) | decimal-place rounding |
| 60, 3600 (TTL defaults) | sources without TTL ignored |
| 86400.0 WGSL epsilon | Number.EPSILON |
| 0.5 KDE | presenceWeight-weighted |
| Φ in scale | scale = 1.0 + g |
| 0.0 (exploration) | Number.EPSILON |
| 0 (complexity) | Number.EPSILON |
| 0 (lastOutTE) | Number.EPSILON |
| 0 (membrane median) | Number.EPSILON |
| 0.0 (ring buffer fill) | Number.EPSILON |

---

## 15. Conditionals and Guards

### Physics (stay)

| Conditional | Reason |
|---|---|
| `minkowskiSq < 0 → presenceWeight = 0` | Spacelike separation. Minkowski metric. |
| `presenceWeight > 0` in read-loop and getRingBuffers | Awareness window filter. Silicon spends energy only on present oscillators. |
| `direction > 0 && deltaTE < -threshold` | Aperture turn. TE gradient exceeds the surrogate null hypothesis (mean + 2σ of 10 shuffled KDEs). |
| `direction < 0 && (deltaTE > threshold \|\| exploration <= GROUND_STATE)` | Aperture turn / floor bounce. |
| `(i & 3) === 3 → data8[i] = 255` | Canvas alpha channel. Screen emits light. |
| `!write` / `!read` | Structural read/write filtering. |
| `isInternalMetric` | Prevents self-referential TE computation. |
| `buf[0] === 0xCF && buf[1] === 0x86 && buf[2] === 1` | Protocol identity (φ). |
| `filled >= topologyRingSize` | Takens needs 3×τ points minimum. |
| `candidates.length < 2` | TE needs two oscillators. |
| `rs < 4` in KDE | Kernel density needs 4 points. |
| `typeof val === 'number'` / `'boolean'` | Discovery finds naked numbers. |
| `!indexMap.has(url)` | Prevents double registration. |
| `circular.has(key)` | Prevents recursion into window/document/location. |

### Language constraints (stay)

| Conditional | Reason |
|---|---|
| `obj == null → return/continue` | JavaScript: typeof null === 'object'. Recursion crashes. |
| `try { val = obj[key] } catch { continue }` | Cross-origin properties throw on access. |
| `typeof readVal !== 'number'` | read() returns undefined when property disappears. |
| `if (audioCtx) return` | AudioContext is singleton. |
| `if (navigator.vibrate)` / `navigator.serial` / `navigator.usb` / `navigator.bluetooth` / `navigator.deviceMemory` | Feature detection. |
| `unwrap_or_else(\|e\| e.into_inner())` on Mutex | Poisoned mutex recovery. Alternative is crash. |
| `unwrap_or_default()` on SystemTime | Duration::ZERO = valid timestamp. |
| `unwrap_or(&0)` in base64 | Padding. Protocol standard. |

### Dead code (remove)

| Conditional | Reason |
|---|---|
| `aperture < Number.EPSILON` in photon/audio/kinetic/serial | Dead. sin(EPSILON × π/2) ≈ 3.49e-16. Never triggers. |
| `threads.length === 0 → fill(0)` in membranes | Fills canvas with 0. Threads empty when no read-oscillators active. |
| `if (rawTick > 0)` in stableTick | rawTick = ts − lastTs. Always > 0. |
| `stableTick > 0 ? ... : rawTick` | Formula works at stableTick = 0. |

### Protocol protection (stay, boundary to old world)

| Conditional | Reason |
|---|---|
| `typeof srvTime === 'number' && isFinite(srvTime)` | Nostr created_at must be valid unix timestamp. |
| `isNaN(lat) \|\| isNaN(lon)` | Remote coordinates must be parseable. |
| `transmitList.length === 0 && !queryPos → return` | Empty frame wastes bandwidth. |
| `content !== '{}'` | Empty nostr event rejected by relays. |
| `nostrRelay.readyState === WebSocket.OPEN` | Connection state. |
| `if (!privKey)` | Generate key when none stored. |

---

## 16. Implementation

### Step 1: Purge

Remove: isValidPath, isPlausibleValue, enumNamespaces, typedArrays, depth > 3, f64() debug, discovery profiling, sensor diagnostics, startSensors 8×8, setPresence cascade, NaN guards in measures, presenceWeight hard-filter in measures, dead `aperture < EPSILON` checks, dead `threads.length === 0 → fill(0)`, `if (rawTick > 0)`, `stableTick > 0 ?`.

### Step 2: Flat array

Replace oscillators Map with flat array + index map. Ring data in contiguous flatRings Float32Array. All hot-path operations iterate by index. Discovery registers via index map. GPU reads flatRings directly.

### Step 3: Structural return

Oscillator struct: write field (writeFn). manifestField uses write. getRingBuffers uses !write. TE source = write oscillators, target = read oscillators. updateApertureGradient: all write. Debug: write. Nostr: write.

### Step 4: EPSILON initialization

flatRings filled with Number.EPSILON. complexity: Number.EPSILON. exploration: Number.EPSILON. lastOutTE: Number.EPSILON. Membrane median: Number.EPSILON. measureG default: Number.EPSILON.

### Step 5: Measured replacements

MA → 1/naturalLatencyTicks. Kolmogorov threshold → 1 + coherenceVariance. Ring size → adaptive from stableTick, start 3. topologyRingSize → start 3. Latency fallback → EPSILON. Sensor frequency → 1000/stableTick. KDE sigma_st → presenceWeight-weighted. scale → 1.0 + g. pos_key → decimal places. TTL defaults → removed. WGSL epsilon → Number.EPSILON. getRingBuffers threshold → filled >= topologyRingSize. Aperture turn threshold → surrogate null (mean + 2σ of 10 shuffled KDEs).

### Step 6: TE on GPU

Write WGSL teShader with Gaussian KDE. 2D dispatch: @workgroup_size(8, 8). Each thread: one pair (read_i, write_j). Reads two ring slices from flatRings. KDE sum over topologyRingSize samples. Output: te[i × n_write + j]. Remove calculateContinuousTE from CPU. Write WGSL surrShader with the same KDE, source = Fisher-Yates shuffle of the write-oscillator's own ring (in-shader, uint-hash seeded per run/oscillator/surrogate). One thread per (write_i, surr_s), s ∈ [0, 10). CPU reduces the 10 null-TEs to mean + 2σ → the aperture turn threshold cached in surrogateThresholds.

### Step 7: Migration fixes

constants.js: 40-byte decode. main.rs: t_frame from first input. main.rs: 5-tuple cache. main.rs: φ_obj 5 values + name validation. main.rs: pos_key decimal-place rounding. index.html: measureVC modulo wrap. main.rs: Cache-Control no-store.

### Step 8: Presence

Earth center ICRS default. Drift by coherence. ICRS coordinates throughout. Nostr ICRS tags (['icrs', x, y, z, t]).

### Step 9: Sensors

getUserMedia on user gesture (camera + microphone). MediaStreamTrackProcessor for video frames. AnalyserNode for audio. Pixel luminance (R+G+B)/765 as f64 per pixel, written to flatRings by index. Frequency bins freqData[i]/255 as f64 per bin. Accelerometer, Gyroscope, Magnetometer, AmbientLightSensor via *Sensor constructors. Gamepad via navigator.getGamepads(). Battery via navigator.getBattery(). WebUSB/WebBluetooth via their APIs.

Sensors read only when presenceWeight > 0. Silicon spends energy only on present oscillators.

### Step 10: Verify

node --check. cargo build. /time, /pulse test. Browser: oscillators active at tick 2, TE on GPU, membranes breathing. First API values in seconds. Camera after getUserMedia: active in 50ms.
