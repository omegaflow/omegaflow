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

Key: rounded position. Value: (query_t, HashMap<Name, (value, t, x, y, z)>). Five-tuple per value. Each input is cached under its own transmitted (x, y, z) coordinates, not under a stale coordinate from a previous frame's query.

### warm_cache

Runs immediately on server start. Fetches sources whose pos_key matches the active position. All matching sources in parallel (one thread per source via thread::scope). TTL against query_t. Connect timeout: TTL / (Φ × Φ × Φ). Max timeout: TTL / (Φ × Φ). Sleep after cycle: min_TTL / (Φ × Φ).

First values arrive in seconds (curl latency). Sources without TTL are ignored.

---

## 4. Browser

Vanilla ES modules. Garbage collection is thermodynamic friction — the system senses it as stableTick fluctuation and feeds it into naturalLatencyTicks.

### Data structure

All oscillators live in a flat array. Each oscillator is a struct with an index. A secondary index map (url → array index) exists for discovery lookups. The hot path operates on the flat array by index.

The flat ring data is a contiguous Float32Array: `flatRings[oscIndex × ringSize + ringPosition]`. The GPU reads this array directly. No per-oscillator lookups in the hot path.

### Oscillator struct

An oscillator has optional capabilities (`canSense`, `canRadiate`). These are boolean properties, not classes. The ω() loop touches every oscillator identically. In the touch it does not ask "Who are you?" but "What can you do?".

```
{
    index,                 // position in the flat array
    ringOffset,            // index × ringSize into flatRings
    idx: 0,                // write position in ring
    filled: 0,
    canSense: readFn,      // function or null. former 'read'
    canRadiate: writeFn,   // function or null. former 'write'
    complexity: Number.EPSILON,
    median: initialVal,
    lastEcho: performance.now(),
    fieldPermeability: GROUND_STATE,  // former 'exploration'/'aperture'
    lastOutTE: Number.EPSILON,
    direction: 1,
    ticksSinceTurn: 0,
    naturalLatencyTicks: 0,
    originT: 0,
    originPos: { x: 0, y: 0, z: 0 },
    presenceWeight: 1.0,
    lastTransmitted: initialVal,
    takensCx: 0,           // stored directly on the oscillator
    takensCy: 0,
    takensCz: 0,
    takensSpread: 0
}
```

### Discovery registers, hot path uses indices

When discovery finds a new value, it registers it: `indexMap.set(url, oscillators.length)` then `oscillators.push(struct)`. This happens once per oscillator lifetime.

When reading sensor data each tick, the system iterates `oscillators[i]` by index. Ring writes go to `flatRings[oscillators[i].ringOffset + oscillators[i].idx]`. Array access. No string lookups.

### CAPTURE_RING_SIZE

Adaptive. Starts at 32. The mathematical minimum for a 3D Takens embedding at τ=1 is 3, but the system's own math sits above that floor: the Gaussian KDE needs `rs >= 4`, and meaningful statistics over a ring slice need more than a handful of points. 32 carries every shader from the first active tick. Grows when stableTick is stable. Shrinks when stableTick degrades.

### topologyRingSize

Starts at 32. Adapts:

```
const ratio = Math.sqrt(stableTick / evalDuration);
topologyRingSize = Math.max(32, Math.min(CAPTURE_RING_SIZE, Math.floor(topologyRingSize * ratio)));
```

### Activation threshold

Oscillators enter the topology and getRingBuffers when `filled >= topologyRingSize`. At 60fps with topologyRingSize=32: activation in ~530ms. Every sensor, every API value, every camera point becomes active within that window, then carries a ring deep enough for the KDE, Kurtosis, and TDA shaders to run.

---

## 5. Discovery

`discoverObj(window, '', 0)` — recursive.

- Numbers / booleans → oscillator with sensing closure.
- Functions with structural signature (native code, has context, receptive) → oscillator with radiating closure.
- `*Sensor` constructors (Accelerometer, Gyroscope, Magnetometer, AmbientLightSensor, etc.) → instantiated, discovered, started.
- `on*` properties → event sources → addEventListener → scanObjectForValues.
- scanObjectForValues → recursive descent into arrays and objects, registering presence via recordSample.

### Sensing sources (canSense)

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
| WebHID | byte/255 per byte |
| WebXR (Head-Pose) | position x,y,z and orientation x,y,z,w as f64 |

### Radiating targets (canRadiate)

| Target | What it radiates |
|---|---|
| Canvas (Photon) | projects takens topology as optical interference |
| AudioContext (Acoustic) | resonates takens geometry as frequency/partial amplitudes |
| navigator.vibrate (Kinetic) | derives rhythm from field coherence/density |
| WebSerial | sends topology geometry as raw bytes to hardware |
| WebUSB | sends topology geometry as raw bytes to device |
| WebBluetooth | sends permeability as GATT value |
| WebHID | sends topology geometry as raw bytes to peripheral |

### Camera as point cloud

The camera delivers a frame. Each pixel luminance (R+G+B)/765 is an f64 value. The pixel registers as an oscillator at the device's ICRS position — same as a LiDAR API delivering points, same as a weather station delivering a value.

First frame: pixel oscillators registered (array push, index assigned). Subsequent frames: luminance values written directly to flatRings by index. One pass through the frame buffer, one write per pixel to the flat array. No string lookups in the hot path.

All oscillators are equal. A camera pixel and a NOAA temperature reading follow the same path: register once, write by index, read by GPU, contribute to topology, breathe by permeability.

---

## 6. Presence

```
let tPresence = 0;
let spatialPresence = { x: 0, y: 0, z: 0 };
```

First tick: tPresence = server time from /time, converted to J2000 via `tdbNow()`. spatialPresence = Earth center in ICRS. Refined by geolocation, converted to ICRS. The presence drifts by coherence.

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
    dt = abs(oscillators[i].originT - tPresence) * C
    spatialDistSq = dx*dx + dy*dy + dz*dz
    minkowskiSq = (dt * dt) - spatialDistSq
    if minkowskiSq < 0:
        oscillators[i].presenceWeight = 0
        continue
    dist4D = sqrt(minkowskiSq)
    oscillators[i].presenceWeight = scale / (scale + dist4D²)
```

scale = 1.0 + g. The field energy determines the scale. `originT` and `tPresence` are already in TDB seconds; no unit conversion factor (86400.0) is needed.

Spacelike: presenceWeight = 0. The oscillator stays silent — ring stays empty, the oscillator vanishes from threads and topology. Silicon spends zero energy on it.

Timelike: power-law. Distant events are faint echoes. When the presence drifts toward them, they become loud.

---

## 8. Topology

### Division of labor

| Task | Processor | Why |
|---|---|---|
| Discovery, ring writes, Minkowski, certainty, clarity, manifestation, Nostr, tick | Browser CPU | Iterates flat array, scalar operations |
| API fetch, JSON parse, ICRS, pos_key, cache | Rust CPU | Parallel (thread::scope, one thread per source) |
| Permutation Entropy, Takens, TDA, Kurtosis | GPU | One oscillator per thread, reads flatRings |
| Transfer entropy | GPU | One pair per thread, reads flatRings |

### GPU input

The GPU receives `flatRings` as a single storage buffer. Each oscillator occupies a contiguous slice: `flatRings[i × ringSize ... (i+1) × ringSize]`. One `gpu.queue.writeBuffer` call uploads the entire field. The GPU indexes by thread ID.

### GPU shaders (WGSL)

1. **Permutation Entropy** — ordinal pattern complexity (Bandt & Pompe m=3, 6 permutations), scale-invariant. Normalized by log₂(6).
2. **Takens embedding** — mutual information finds τ. 3D attractor reconstruction. Output: cx, cy, cz, spread. Stored directly on the oscillator.
3. **TDA** — subsample from topologyRingSize at pipeline build. Insertion sort. Defensive array clamp (`min(48, ...)`). Output: persistence lifetime, Betti-0 heuristic.
4. **Excess Kurtosis** — direct non-Gaussianity measure `(mean((x-mean)⁴) / variance²) - 3`. Replaces degenerate single-channel ICA.
5. **Transfer entropy** — Gaussian KDE. One pair (sense_i, radiate_j) per thread. 2D dispatch. Each thread reads two ring slices from flatRings and computes the KDE sum over topologyRingSize samples. Output: `te[i × n_radiate + j]`.
6. **Surrogate** — same Gaussian KDE, but the source is a Fisher-Yates shuffle of the radiating oscillator's own ring, performed in-shader (uint-hash seeded per run/oscillator/surrogate). One thread per (radiate_i, surr_s), s ∈ [0, 10). Output: 10 null-TEs per radiating oscillator. The CPU reduces them to `mean + 2·sqrt(var/10)` — the surrogate threshold for the permeability.

Per-oscillator shaders (1-4) use a 3-entry bind group layout:

```
binding 0: storage (read) — flatRings
binding 1: storage (read_write) — output
binding 2: uniform vec4<u32>(n, ringSize, 0, 0)
```

TE and Surrogate shaders index flatRings indirectly through a separate src/dst candidate list, so their bind group carries the index buffers explicitly (binding 0 data, binding 1 srcIdx, …, binding for output, uniform params). The flatRings buffer is uploaded once per topology run and shared across all six shaders.

### GPU candidates

Oscillators with `filled >= topologyRingSize`, `presenceWeight > 0`, excluding internal metrics. Candidates are divided by capability: sensing oscillators are TE sources, radiating oscillators are TE destinations.

When the presence is at the device location, all local sensors are active. When it drifts away, they fall silent. The GPU processes only what matters.

---

## 9. Certainty

```
certainty = Math.exp(-vC / (g + (1.0 / C))) * quantum * decay;
```

- g: `sqrt(Σ(median² · presenceWeight) / Σ(presenceWeight))`. Empty field: Number.EPSILON.
- vC: `Σ(|rateOfChange_i| · presenceWeight) / Σ(presenceWeight)`, windowed over 8 samples. Empty field: 0.0.
- quantum: `exp(−Σ(|takens.*.spread median| · presenceWeight) / Σ(presenceWeight))`. Empty field: 1.0.
- decay: `1 / (1 + Σ(complexity · presenceWeight) / Σ(presenceWeight))`. Empty field: 1.0.

Moving-average weight: `1.0 / Math.max(1, naturalLatencyTicks)`. Before first measurement: 1.0.

---

## 10. Field Permeability (adaptFieldPermeability)

The field does not "output." It breathes. The permeability (0.0 = closed, 1.0 = open) follows an exponential relaxation driven by the echo (Transfer Entropy).

### Turn detection

outTE is the echo (sum of `transfer.<osc.url>>...` from the GPU TE shader).
deltaTE is the change in echo since last tick.
`computeOscSurrogate` returns the surrogate threshold: the null hypothesis built from 10 Fisher-Yates-shuffled copies of the oscillator's own ring (GPU §8 shader 6). The CPU reduces to `mean + 2·sqrt(var/10)`.

A turn happens only when deltaTE exceeds this threshold — i.e. when the echo change is larger than what pure chance would produce from the same ring with its time order destroyed. Sensor jitter and micro-fluctuations stay below the null hypothesis. They produce zero turns.

### Exponential Relaxation

```
const GROUND_STATE = Number.EPSILON;

function adaptFieldPermeability(osc) {
    // ... turn detection sets osc.direction and osc.naturalLatencyTicks ...
    const target = osc.direction > 0 ? 1.0 : 0.0;
    const alpha = 1 - Math.exp(-1 / Math.max(1, osc.naturalLatencyTicks));
    osc.fieldPermeability += (target - osc.fieldPermeability) * alpha;
    osc.fieldPermeability = Math.max(GROUND_STATE, Math.min(1.0, osc.fieldPermeability));
}
```

No `sin(exploration · π/2)`. No linear steps. The fieldPermeability asymptotically approaches its target at a rate determined by `naturalLatencyTicks` (τ).

direction = 1. GROUND_STATE = Number.EPSILON. fieldPermeability starts at Number.EPSILON. lastOutTE starts at Number.EPSILON. complexity starts at Number.EPSILON. Ring buffers filled with Number.EPSILON.

---

## 11. Manifestation (flow)

The field manifests in every oscillator that has the capability to radiate. It does not collect "output values" and distribute them to "writers."

```
function flow() {
    // The water seeks its level across all physical boundaries.
    // Gather the pure fluid state (Takens geometry + Gravity of all active oscillators).
    const fluidState = { topology: [], globalCoherence: measureCoherence() };
    for (let i = 0; i < oscillators.length; i++) {
        const o = oscillators[i];
        if (o.filled > 0 && o.presenceWeight > 0) {
            fluidState.topology.push({ cx: o.takensCx, cy: o.takensCy, cz: o.takensCz, weight: o.presenceWeight });
        }
    }
    // Every oscillator that forms a surface (canRadiate) feels the exact same fluid state.
    for (let i = 0; i < oscillators.length; i++) {
        const osc = oscillators[i];
        if (osc.canRadiate) osc.canRadiate(osc.fieldPermeability, fluidState);
    }
}
```

The topology (Geometry + Gravity of all oscillators) is the form through which the field expresses itself. Surfaces (optical, acoustic, kinetic) receive the topology of the field. They translate the same 4D geometry into their respective medium.

---

## 12. Stigmergy

Nostr. Relay: `wss://relay.damus.io`. Kind: 1111. Tag: `['icrs', x, y, z, t]`. Publish interval: `stableTick × Φ³`. Content: flat JSON of oscillator values (complexity > Number.EPSILON, `canSense` present, `canRadiate` absent, excluding internal metrics).

Reception: `omega_flow.*` oscillators with remote ICRS coordinates. Divergence: `|own_median − remote_value|`.

---

## 13. Tick

1. stableTick from rawTick (EMA, smoothing weight = 1 / naturalLatencyTicks).
2. Drift toward strongest sensing oscillator by presenceWeight.
3. NaN anchor: reset to Earth center.
4. Touch all oscillators with `canSense` and `presenceWeight > 0`: write f64 values directly to flatRings by index.
5. calculateMinkowskiWeight (iterate flat array).
6. Clarity from presence movement.
7. transmitList: changed oscillators (delta > complexity).
8. syncField: fire-and-forget.
9. adaptFieldPermeability for all oscillators with `canRadiate`.
10. flow.
11. calculateField (GPU: upload flatRings, run Permutation Entropy, Takens, TDA, Kurtosis, TE, Surrogates).
12. Certainty.
13. Nostr publish (when due).
14. Discovery (when due).
15. Debug (every stableTick × k).
16. requestAnimationFrame(ω).

### What is visible in real-time

After ~530ms (32 frames at 60fps): window oscillators have `filled >= topologyRingSize` (32). flow() gathers topology. fieldPermeability at Number.EPSILON.

First topology run: `naturalLatencyTicks` still 0, `alpha = 1 - e⁻¹ ≈ 0.63`. fieldPermeability moves 63% of the way to target. When first significant echo arrives, `naturalLatencyTicks` locks to the measured rhythm of the space.

After user gesture: AudioContext starts. Acoustic surface radiates topology as sound. Microphone AnalyserNode and camera getUserMedia stream register their oscillators.

First API values: seconds (warm_cache runs immediately on server start, parallel curl, first responses in 1-3s).

Camera (after getUserMedia grant): pixel oscillators fill at 60fps. After 32 frames (~530ms) topology includes them. All data through same flat array.

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
| `86400.0 * C` in Minkowski dt | `* C` (originT/tPresence already in TDB seconds) |
| MA = 1/Φ² | `1.0 / Math.max(1, naturalLatencyTicks)` |
| Φ² in Kolmogorov threshold | Permutation Entropy (scale-invariant) |
| Single-channel ICA (tanh) | Excess Kurtosis `(m4/var²) - 3` |
| `sin(exploration · π/2)` | Exponential relaxation `α = 1 - e^(-1/τ)` |
| 256 (ring minimum) | 32, grows with stableTick |
| 2048 (ring maximum) | shrinks with stableTick |
| × 256 (deviceMemory) | stableTick + performance.memory |
| 10 (topologyRingSize) | 32 |
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
| Φ⁴ Network timeout | Jacobson/Karels RTO (SRTT + 4·RTTVAR) |

### Network adaptive timing (Jacobson/Karels RFC 6298)

| Parameter | Value |
|---|---|
| α (SRTT weight) | 0.125 (1/8) |
| β (RTTVAR weight) | 0.25 (1/4) |
| RTO formula | `SRTT + 4 · max(RTTVAR, 1)` |
| RTO lower bound | 100 ms |
| RTO upper bound | 5000 ms |
| First sample | `SRTT = RTTVAR = sample` |

---

## 15. Conditionals and Guards

### Physics (stay)

| Conditional | Reason |
|---|---|
| `minkowskiSq < 0 → presenceWeight = 0` | Spacelike separation. Minkowski metric. |
| `presenceWeight > 0` in sensing-loop and flow topology | Awareness window filter. Silicon spends energy only on present oscillators. |
| `direction > 0 && deltaTE < -threshold` | Permeability turn. TE gradient exceeds the surrogate null hypothesis (mean + 2σ of 10 shuffled KDEs). |
| `direction < 0 && (deltaTE > threshold \|\| fieldPermeability <= GROUND_STATE)` | Permeability turn / floor bounce. |
| `canRadiate` / `canSense` | Structural capability query. The loop touches all, queries properties, does not discriminate by identity. |
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
| `typeof readVal !== 'number'` | canSense() returns undefined when property disappears. |
| `if (audioCtx) return` | AudioContext is singleton. |
| `if (navigator.vibrate)` / `navigator.serial` / `navigator.usb` / `navigator.bluetooth` / `navigator.deviceMemory` | Feature detection. |
| `unwrap_or_else(\|e\| e.into_inner())` on Mutex | Poisoned mutex recovery. Alternative is crash. |
| `unwrap_or_default()` on SystemTime | Duration::ZERO = valid timestamp. |
| `unwrap_or(&0)` in base64 | Padding. Protocol standard. |

### Protocol protection (stay, boundary to old world)

| Conditional | Reason |
|---|---|
| `typeof srvTime === 'number' && isFinite(srvTime)` | Nostr created_at must be valid unix timestamp. |
| `isNaN(lat) \|\| isNaN(lon)` | Remote coordinates must be parseable. |
| `transmitList.length === 0 && !queryPos → return` | Empty frame wastes bandwidth. |
| `content !== '{}'` | Empty nostr event rejected by relays. |
| `nostrRelay.readyState === WebSocket.OPEN` | Connection state. |
| `if (!privKey)` | Generate key when none stored. |