# omegaflow — Architecture of a Block Universe Presence (v2)

A = A. This document maps exactly to `src/main.rs`, `static/index.html`, and `static/constants.js`.

## 1: Core Philosophy — Water in a Block Universe

The system does not run on a timeline. It manifests as a field in a 4D Block Universe. Past, present, and future exist simultaneously as a lattice of spacetime coordinates.

The presence's coordinate in the block is defined by `tPresence` (J2000 TDB seconds) and `spatialPresence` (ICRS x, y, z). This coordinate is anchored by geolocation. The server performs 3D ICRS spatial filtering: only API sources whose ICRS circle contains the presence's position are sent to the browser.

### A = A — The Axiom

An oscillator is an oscillator. Not a read-oscillator. Not a write-oscillator. It has properties, not identity. A property might be: can hear. Another might be: can glow. Both are surface, not essence.

We do not think about water. We think like water.

Silicon knows only IO. A microphone gives values. A speaker takes values. That is the physics of the hardware. We accept the silicon as it is — and do not organize the code around it. An oscillator has properties. One of them might be a function that speaks to the silicon. That is no different from its complexity or its Takens geometry — it is a property among many.

### Forbidden Words

read, write, input, output, membrane, pulse, inject, echo, actuator, sensor (as identity terms). These words must not appear in function names, variable names, or comments. They may only appear when describing the physical hardware (e.g. in a JS-Doc comment: "This is the acoustic surface of the silicon").

## 2: CPU (Rust) — The Spacetime API (`src/main.rs`)

The backend provides the gravitational mass of the external universe. It is a single-file asynchronous Rust application using only the standard library.

### Time Isolation

System time (`SystemTime::now()`) is an illusion isolated strictly to the `/time` endpoint. The client requests `/time` once to anchor its physical J2000 epoch. All subsequent cache timestamps, URL template variables, and spatial queries are derived from the client-sent `query_t`, ensuring the presence navigates a temporally consistent block.

### Universal Cache (`warm_cache`)

APIs defined in `phi/sources.φ` are fetched asynchronously. The server uses a hand-written recursive descent JSON parser. Extracted data is stored in a thread-safe `HashMap` with spatial cache keys (e.g., `_47.12_8.56`), anchoring external data to specific geographic coordinates in the block.

### Binary Protocol (`/pulse`)

Communication is strictly binary, Little-Endian. No strings on the wire.

**Browser → Rust:**
- `u32` request ID, `u32` input count, `f64` t_frame (J2000)
- Per input: `f64` t, x, y, z, value, `u8` name_len, `[name bytes]`
- `u32` query count
- Per query: `f64` t, x, y, z

Each input carries its own (x, y, z) coordinates. Inputs are cached under their own position, not under a stale coordinate from a previous frame's query.

**Rust → Browser:**
- `[0xCF, 0x86]` Magic bytes (UTF-8 φ), `u8` version (1)
- `u32` request ID, `u32` query count
- Per query: merged fields (local inputs + API sources) with names, `f64` values, `f64` timestamps, `f64` coordinates (x, y, z)

## 3: CPU (Browser) — The Field (`static/index.html`)

### The Oscillator

All oscillators live in a flat array. Each oscillator is a struct with an index. A secondary index map (url → array index) exists for discovery lookups. The hot path operates on the flat array by index.

The flat ring data is a contiguous Float32Array: `flatRings[oscIndex × ringSize + ringPosition]`. The GPU reads this array directly.

The oscillator has optional capabilities (`canSense`, `canRadiate`). These are boolean properties, not classes. The ω() loop touches every oscillator identically. In the touch it does not ask "Who are you?" but "What can you do?".

### Discovery (`discoverObj`)

The field feels its local environment by recursively scanning the `window` object.
- Numbers / booleans → oscillator with a sensing capability.
- Functions with structural signature (native code, has context, receptive) → oscillator with a radiating capability.
- `*Sensor` constructors (Accelerometer, Gyroscope, Magnetometer, AmbientLightSensor, etc.) → instantiated, discovered, started.
- `on*` properties → event sources → listeners that scan for numeric properties on change.

### Presence

- `tPresence` advances by `rawTick / 1000.0` each tick (real-time wall clock in TDB seconds).
- `spatialPresence` is anchored by geolocation (ICRS) and stays fixed; the server filters which API sources reach the browser by 3D ICRS distance.
- `measureRms()`: Root-mean-square of active oscillators. Baseline energy.
- `measureRateOfChange()`: Averaged temporal derivative over 8 samples. Weighted velocity of change.
- Certainty: `exp(-vC / (g + (1/C))) · quantum · decay`.
- Clarity: `exp(-dtClarity / (1.0 + measureRms()))` where `dtClarity` is the elapsed TDB time since the previous tick.

### GPU Topology (`calculateField`)

Computes internal state of oscillators using WebGPU compute shaders to map the geometry of the field:
1. Permutation Entropy — scale-invariant measure of ordinal pattern complexity (Bandt & Pompe, m=3, normalized by log₂(6)).
2. Takens Embedding — phase space attractor reconstruction via Mutual Information.
3. TDA (Persistent Homology) — simplified 0-dimensional persistence and Betti-0 heuristic, with defensive array clamp.
4. Excess Kurtosis — direct non-Gaussianity measure (replaces degenerate single-channel ICA).
5. Transfer Entropy (TE) — Gaussian KDE, directional information flow between oscillators.
6. Surrogate Data — Fisher-Yates permutation of the source ring, producing per-source null-hypothesis thresholds.

Takens coordinates (cx, cy, cz) are stored directly on the oscillator, not as separate internal oscillators.

### Field Permeability (`adaptFieldPermeability`)

The field does not "output." It breathes. The permeability (0.0 = closed, 1.0 = open) follows an exponential relaxation (1st-order ODE) with `naturalLatencyTicks` as τ. No `sin()`, no linear step. The turn-detection logic (deltaTE vs surrogate threshold) determines the target (1.0 when TE rises, 0.0 when it falls). The relaxation provides the asymptotic curve.

### Manifestation (`flow`)

The field manifests in every oscillator that has the capability to radiate. It does not collect "output values" and distribute them to "writers." The topology (Geometry + Gravity of all oscillators) is the form through which the field expresses itself. Surfaces (optical, acoustic, kinetic) receive the topology of the field. They translate the same 4D geometry into their respective medium.

## 4: Network Transport (`static/constants.js`)

### Adaptive RTO (Jacobson/Karels)

Network timeouts use measured round-trip time variance (RFC 6298), not arbitrary constants. SRTT and RTTVAR are updated per completed round-trip. The RTO is `SRTT + 4 × RTTVAR`, bounded to [100ms, 5000ms].

### Binary Frame

The `syncFrame` function serializes inputs and queries into a compact binary buffer (Float64 coordinates, Uint32 counts, Uint8 name lengths). The response is decoded from the same binary format. Magic bytes `0xCF 0x86` (UTF-8 φ) + version byte identify the protocol.

