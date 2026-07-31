# omegaflow — Architecture of a Block Universe Presence

A = A. This document maps exactly to `src/main.rs`, `static/index.html`, and `static/constants.js`.

## 1: Core Philosophy — Water in a Block Universe

The system does not run on a timeline. It manifests as a field in a 4D Block Universe. Past, present, and future exist simultaneously as a lattice of spacetime coordinates.

The presence's coordinate in the block is defined by `tPresence` (J2000 TDB seconds) and `spatialPresence` (ICRS x, y, z): a free worldline `p + v·(t − t0)`, at rest at the SSB origin by default. The client never computes planetary ephemerides; Earth is data like every planet, delivered by the Archivar. The server performs 3D ICRS spatial filtering: only API sources whose ICRS circle contains the presence's position are sent to the browser.

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

### Binary Protocol (`/resonance`)

Communication is strictly binary, Little-Endian. No strings on the wire.

**Browser → Rust:**
- `u32` request ID, `u32` oscillator count
- Per oscillator: `f64` value, `u8` name_len, `[name bytes]` (the browser radiates its local sensors as raw oscillators into the field)
- `u32` query count
- Per query: `f64` t, x, y, z (the presence window: first query is the presence center, further queries are the corners of the 2D surface at presence t, z; the server derives the window extent from them and runs a single enclosure dilated by that extent)

**Rust → Browser:**
- `[0xCF, 0x86]` Magic bytes (UTF-8 φ), `u8` version (2)
- `u32` request ID, `u32` oscillator count
- Per oscillator: `f64` x, y, z, val, extent, t, ttl, tau, force_type (72 bytes) — one flat array for the whole presence window (every active sample × field, no name merging; the point cloud stays intact). t is the measurement epoch, ttl the freshness expectation, tau the medium's time constant (hue), force_type the medium (0=em, 1=gravity, 2=acoustic, 3=seismic-body, 4=seismic-surface, 5=thermal, 6=diffusion, 7=advective). The Mathematikerin folds certainty `e^(−max(0, |tQuery − t| − d/c)/ttl)` client-side in f64 into val_eff (council amendment: never absolute time in f32), so nothing is hard-deleted — oscillators decay exponentially instead of blinking out. Server retention keeps samples until ttl×2⁶, where certainty e^(−64) rests below the display floor exp2(−64).
## 3: GPU (Browser) — The Mathematikerin (`static/index.html`)


The browser is a pure sensor window. The presence window is a 2D surface in the 4D block (constant t, z of the presence), and the oscillators within it stay an intact point cloud: each sample is drawn as a soft Gaussian splat projected in the vertex stage on the main thread (the Nebra path — a worker was tried and discarded), while a compute probe evaluates the combined field at the presence point for the non-optical media. Oscillator coordinates and certainty are folded into the presence frame in f64 before upload (f32 ulp at 1.5e11 m ICRS is ~16 km; in the presence frame it is ~mm). No grid is sent to the server; the server only sees the surface definition. If the GPU is absent, the window stays black — there is no CPU field evaluation.

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
- `spatialPresence` is a free ICRS worldline (`p + v·(t − t0)`, default at rest at the SSB origin); the server filters which API sources reach the browser by 3D ICRS distance.
### GPU Field Evaluation (The Mathematikerin)


The GPU evaluates the physical laws locally for the requested presence window only. It receives the flat array of raw oscillators from the Archivar. The field influence is `val_eff · K(force_type, extent, d, softening)` with force-specific spatial kernels and softening = pixel scale (Nyquist); the optical surface maps the log-magnitude of each splat to the canvas, the compute probe sums the same law at the presence point per force. The pixel scale (m/px) is the operator's gaze — set by hand or deep-link, never by the field; an empty window is a fully realized state. The optical medium normalizes per force: each force's luminance reference relaxes exponentially toward that force's max |val_eff| in the window (live data, no constants), the operator offsets with `e`/`E` (2ⁿ). GPU submissions apply backpressure (`onSubmittedWorkDone`) — a slow GPU never accumulates queued frames.


No global grids are brute-forced. No abstract temporal topology is calculated. The GPU manifests the pure, physical field in real-time.
### Field Permeability (`adaptFieldPermeability`)

The field does not "output." It breathes. The permeability (0.0 = closed, 1.0 = open) follows an exponential relaxation (1st-order ODE) with `naturalLatencyTicks` as τ. No `sin()`, no linear step. 
### Manifestation (`flow`)


The field manifests in every oscillator that has the capability to radiate. It does not collect "output values" and distribute them to "writers." One law, five media: `omega(p) = Σ val_eff · K(force_type, extent, |p − x|, softening)` with `val_eff = val · e^(−max(0, |tPresence − t| − d/c)/ttl)`. The optical surface draws the point cloud on the GPU; the acoustic, haptic and hardware surfaces evaluate the same law at the presence point and translate it into their medium (normalization from live data: median window extent, no constants).

## 4. Network Transport (`static/constants.js`)

### Adaptive RTO (Jacobson/Karels)

Network timeouts use measured round-trip time variance (RFC 6298), not arbitrary constants. SRTT and RTTVAR are updated per completed round-trip. The RTO is `SRTT + 4 × RTTVAR`, bounded to [100ms, 5000ms].

### Binary Frame

The `syncFrame` function serializes oscillators and queries into a compact binary buffer (Float64 coordinates, Uint32 counts, Uint8 name lengths). The response is decoded from the same binary format. Magic bytes `0xCF 0x86` (UTF-8 φ) + version byte identify the protocol.
