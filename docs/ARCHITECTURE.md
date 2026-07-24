# omegaflow — Architecture of a Block Universe Presence

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

## 2: CPU (Rust) — The Archivar (`src/main.rs`)

The backend provides the raw mass of the external universe. It is a single-file asynchronous Rust application using only the standard library. It acts as the Archivar: it fetches, parses, and caches raw spacetime data. It performs no field calculations.

### Time Isolation

System time (`SystemTime::now()`) is an illusion isolated strictly to the `/time` endpoint. The client requests `/time` once to anchor its physical J2000 epoch. All subsequent cache timestamps, URL template variables, and spatial queries are derived from the client-sent `query_t`, ensuring the presence navigates a temporally consistent block.

### Universal Spatial Cache (`warm_cache`)

APIs defined in `phi/sources.φ` are fetched asynchronously. The server uses a hand-written recursive descent JSON parser. Extracted data is stored in a thread-safe `RwLock<Arc<Buffer>>` spatial hash. Data is anchored to specific ICRS coordinates in the block. Fetches run through a bounded pool of 2³ workers fed by a min-ttl priority heap, gated by the presence window; every attempt (success or failure) is timestamped so a source is re-fetched at its own ttl/Φ pace — fresh data overlaps, failures never flood.

The cache uses the Enclosure Lemma: it radiates where oscillators were, and the sense flow queries where they could have been. Time is moved out of the cache key into the motion law and the search envelope.

### Binary Protocol (`/pulse`)

Communication is strictly binary, Little-Endian. No strings on the wire.

**Browser → Rust:**
- `u32` request ID, `u32` oscillator count
- Per oscillator: `f64` value, `u8` name_len, `[name bytes]` (the browser radiates its local sensors as raw oscillators into the field)
- `u32` query count
- Per query: `f64` t, x, y, z (the presence window: first query is the presence center, further queries are the corners of the 2D surface at presence t, z; the server derives the window extent from them and runs a single enclosure dilated by that extent)

**Rust → Browser:**
- `[0xCF, 0x86]` Magic bytes (UTF-8 φ), `u8` version (1)
- `u32` request ID, `u32` oscillator count
- Per oscillator: `f64` x, y, z, val, aperture — one flat array for the whole presence window (every active sample × field, no name merging; the point cloud stays intact).
## 3: GPU (Browser) — The Mathematikerin (`static/gpu.worker.js`, `static/index.html`)


The browser is a pure sensor window. The presence window is a 2D surface in the 4D block (constant t, z of the presence), and the native screen pixels are its point cloud: every pixel is one ICRS point `presence + (u − 0.5) · resolution · scale`, evaluated by a WebGPU fragment shader in `gpu.worker.js` on an OffscreenCanvas at native resolution. No grid is sent to the server; the server only sees the surface definition. If the GPU is absent, the window stays black — there is no CPU field evaluation.

### The Oscillator

All oscillators live in a flat array. The oscillator has optional capabilities (`canSense`, `canRadiate`). These are boolean properties, not classes. The ω() loop touches every oscillator identically. In the touch it does not ask "Who are you?" but "What can you do?".

### Discovery (`discoverObj`)

The field feels its local environment by recursively scanning the `window` object.
- Numbers / booleans → oscillator with a sensing capability.
- Functions with structural signature (native code, has context, receptive) → oscillator with a radiating capability.
- `*Sensor` constructors (Accelerometer, Gyroscope, Magnetometer, AmbientLightSensor, etc.) → instantiated, discovered, started.
- `on*` properties → event sources → listeners that scan for numeric properties on change.

### Presence

- `tPresence` advances by `rawTick / 1000.0` each tick (real-time wall clock in TDB seconds).
- `spatialPresence` is anchored by geolocation (ICRS) and stays fixed; the server filters which API sources reach the browser by 3D ICRS distance.
### GPU Field Evaluation (The Mathematikerin)


The GPU evaluates the physical laws locally for the requested presence window only. It receives the flat array of raw oscillators from the Archivar. A WebGPU fragment shader iterates over this array for every pixel of the window, calculating the field influence `val / (dist² + aperture²)` with the aperture as softening length, and maps the log-magnitude to the canvas. The pixel scale (m/px) relaxes exponentially toward the median aperture of the oscillators in the window (Nyquist: two pixels per aperture); an empty window zooms out by Φ. GPU submissions apply backpressure (`onSubmittedWorkDone`) — a slow GPU never accumulates queued frames.


No global grids are brute-forced. No abstract temporal topology is calculated. The GPU manifests the pure, physical field in real-time.
### Field Permeability (`adaptFieldPermeability`)

The field does not "output." It breathes. The permeability (0.0 = closed, 1.0 = open) follows an exponential relaxation (1st-order ODE) with `naturalLatencyTicks` as τ. No `sin()`, no linear step. 

### Manifestation (`flow`)

The field manifests in every oscillator that has the capability to radiate. It does not collect "output values" and distribute them to "writers." The physical field coefficients calculated by the GPU are the form through which the field expresses itself. Surfaces (optical, acoustic, kinetic) receive these raw coefficients. They translate the same 4D field into their respective medium.

## 4. Network Transport (`static/constants.js`)

### Adaptive RTO (Jacobson/Karels)

Network timeouts use measured round-trip time variance (RFC 6298), not arbitrary constants. SRTT and RTTVAR are updated per completed round-trip. The RTO is `SRTT + 4 × RTTVAR`, bounded to [100ms, 5000ms].

### Binary Frame

The `syncFrame` function serializes oscillators and queries into a compact binary buffer (Float64 coordinates, Uint32 counts, Uint8 name lengths). The response is decoded from the same binary format. Magic bytes `0xCF 0x86` (UTF-8 φ) + version byte identify the protocol.
