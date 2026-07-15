# omegaflow — Architecture of a Block Universe Observer (v1)

A = A. This document maps exactly to `src/main.rs`, `static/index.html`, and `static/constants.js`.

## 1: Core Philosophy — The Block Universe

The system does not run on a timeline. It manifests as an observer navigating a 4D Minkowski Block Universe. Past, present, and future exist simultaneously as a lattice of spacetime coordinates. Time does not flow; the observer drifts.

The observer's coordinate in the block is defined by `tPresence` (J2000) and `spatialPresence` (ECEF x, y, z). This coordinate is not absolute. It drifts toward the regions of highest presence — the oscillators whose 4D Minkowski distance (`dt² - dx² - dy² - dz²`) to the observer is smallest. 

The system is agnostic. It does not know what it measures. It discovers values by walking the environment (`window` object) and pulling external spacetime data (`sources.φ`). It consists entirely of Oscillators (ring buffers that accumulate measurement) and Membranes (permeability regulation toward the environment). The Aperture (0.0 to 1.0) determines how permeable the membrane is.

## 2: CPU (Rust) — The Spacetime API (`src/main.rs`)

The backend provides the gravitational mass of the external universe. It is a single-file asynchronous Rust application using only the standard library.

### Time Isolation
System time (`SystemTime::now()`) is an illusion isolated strictly to the `/time` endpoint. The client requests `/time` once to anchor its physical J2000 epoch. All subsequent cache timestamps, URL template variables, and spatial queries are derived from the client-sent `query_t`, ensuring the observer navigates a temporally consistent block. 

### Universal Cache (`warm_cache`)
APIs defined in `phi/sources.φ` are fetched asynchronously. The server uses a hand-written recursive descent JSON parser (`JsonParser`). Extracted data is stored in a thread-safe `HashMap` with spatial cache keys (e.g., `_47.12_8.56`), anchoring external data to specific geographic coordinates in the block.

### Binary Protocol (`/pulse`)
Communication is strictly binary, Little-Endian. No strings on the wire.

**Browser → Rust:**
- `u32` request ID, `u32` input count, `f64` t_frame (J2000)
- Per input: `f64` t, x, y, z, value, `u8` name_len, `[name bytes]`
- `u32` query count
- Per query: `f64` t, x, y, z

**Rust → Browser:**
- `[0xCF, 0x86]` Magic bytes, `u8` version (1)
- `u32` request ID, `u32` query count
- Per query: merged fields (local inputs + API sources) with names, `f64` values, `f64` timestamps

## 3: CPU (Browser) — The Observer (`static/`)

### Discovery (`discoverObj`)
The observer feels its local environment by recursively scanning the `window` object.
- Oscillators: `number` or `boolean` properties → ring buffer.
- Membranes: native functions (`[native code]`) with object context that accept arguments.
- Events (`on*`) → listeners that inject time-based values on change.

### Presence & The Drift
- `calculatePresenceWeights()`: Measures the 4D Minkowski distance between an oscillator's origin and the observer's presence.
- The Drift: The observer's frame (`tPresence`, `spatialPresence`) drifts toward the strongest oscillators. The drift speed scales with PHI and the inverse of the 4D distance.
- `measureG()`: Weighted median of active oscillators. Baseline energy.
- `measureVC()`: Weighted velocity of change relative to C.
- Certainty: `exp(-vC / (g + ε)) · quantum · decay · clarity`.

### GPU Topology (`runSignalTopology`)
Computes internal state of oscillators using WebGPU compute shaders to map the geometry of the observer's internal field:
1. Kolmogorov Complexity — algorithmic randomness of the ring buffer.
2. Takens Embedding — phase space attractor reconstruction via Mutual Information.
3. TDA (Persistent Homology) — Betti-0 and persistence lifetime.
4. ICA (Independent Component Analysis) — FastICA blind source separation.
5. Transfer Entropy (TE) — kernel density estimation on CPU, directional information flow between oscillators.

### Permeability (`regulatePermeability`)
The observer does not "output." It breathes. 
The `updateApertureGradient()` function calculates an exploration value (0.0 to 1.0). When outgoing Transfer Entropy drops, the direction reverses. The value is applied via `Math.sin(exploration · (π/2))`.
Membranes: Audio (acoustic), Canvas (photon), Vibration (kinetic), Serial (hardware).

### Stigmergy (Nostr)
Connects to `wss://relay.damus.io`. Listens for `kind: 1111` events from other omegaflow instances. Injects remote measurements as `omega_flow.*` metrics with divergence calculation. Publishes its own state periodically based on a PHI-scaled tick interval.
