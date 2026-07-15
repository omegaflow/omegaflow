# omegaflow — System Architecture (v6)

This document describes the agnostic, purely kybernetic architecture of omegaflow.
It maps exactly to the source files: `src/main.rs`, `static/index.html`, and `static/constants.js`.

## 1: Core Philosophy

*   **Agnostic Topology:** The system does not know what it measures. There are no hardcoded physical sensors, no blacklists, and no biological metaphors. It discovers values and functions by walking the environment (`window` object).
*   **Minkowski Presence:** All measurements exist in a 4D space-time (Minkowski metric). The "Presence" of a value is determined by its 4D distance to the observer's frame of reference.
*   **Aperture Gradient:** Actuators (outputs) are driven by an exploration gradient (0 to 1), which oscillates based on outgoing Transfer Entropy (TE). This creates a self-regulating expression field.
*   **No External Dependencies:** The Rust server uses only `std`. The Browser uses only Vanilla ES Modules and WebGPU.

## 2: CPU (Rust) — `src/main.rs`

The backend is a single-file asynchronous Rust application using only the standard library.

### Time Architecture
*   `SystemTime::now()` is strictly isolated to HTTP endpoints (`/time`, `/crash`).
*   The client requests `/time` and calculates the J2000 epoch.
*   All cache timestamps and URL template variables (`{today}`, `{unix_now}`) are strictly derived from the client-sent JCEF time `query_t`, ensuring temporal consistency across distributed instances.

### Universal Cache (`warm_cache`)
*   **Fetching:** APIs defined in `phi/sources.φ` are fetched asynchronously in the background using the system `curl` binary.
*   **Extraction:** The server uses a hand-written recursive descent JSON parser (`JsonParser`). It supports deep path extraction (`jpath`, `jdeep_find_num`), 2D arrays (`j2d_last_row`), and raw text vectors (`text_vector`).
*   **Storage:** Extracted data is stored in a thread-safe `HashMap` with spatial cache keys (e.g., `_47.12_8.56`).

### WebSocket Protocol (`/pulse`)
The browser connects via WebSocket. Communication is strictly binary, Little-Endian.

**Request (Browser -> Rust):**
*   `u32` request ID
*   `u32` input count (local sensor data)
*   `f64` t_frame (J2000)
*   `[per input]:` `f64` t, `f64` x, `f64` y, `f64` z, `f64` value, `u8` name_len, `[name bytes]`
*   `u32` query count
*   `[per query]:` `f64` t, `f64` x, `f64` y, `f64` z

**Response (Rust -> Browser):**
*   `[0xCF, 0x86]` Magic bytes
*   `u8` version (6)
*   `u32` request ID
*   `u32` query count
*   `[per query]:` Merged fields (local inputs + API sources) containing names, `f64` values, and `f64` timestamps.

## 3: CPU (Browser) — `static/`

The frontend is a single HTML file containing the core logic and a shared constants module.

### Discovery (`discoverObj`)
*   Recursively scans the `window` object.
*   **Sensors (Readers):** Any `number` or `boolean` property. Values are injected into a ring buffer.
*   **Actuators (Writers / Membranes):** Native functions (`[native code]`) that belong to an object context (`hasContext`) and accept arguments (`isReceptive`). Constructors are instantiated.
*   Events (`on*`) are mapped to listeners that inject time-based values on change.

### Measurement & Presence
*   **`calculatePresenceWeights()`:** Measures the 4D Minkowski distance (`dt^2 - dx^2 - dy^2 - dz^2`) between an oscillator's origin and the observer's presence.
*   **`measureG()`:** Weighted median of all active sensors. Represents the system's baseline energy.
*   **`measureVC()`:** Weighted velocity of change in the system relative to the speed of light (`C`).
*   **Certainty:** `exp(-vC / (g + ε)) * quantum * decay * clarity`.

### GPU Topology (`runSignalTopology`)
Computes the internal state of all active oscillators using WebGPU compute shaders:
1.  **Kolmogorov Complexity:** Algorithmic randomness of the ring buffer.
2.  **Takens Embedding:** Reconstructs phase space attractors (finds optimal delay `tau` via Mutual Information).
3.  **TDA (Persistent Homology):** Calculates Betti-0 and persistence lifetime.
4.  **ICA (Independent Component Analysis):** Blind source separation using FastICA (`tanh` nonlinearity).
5.  **Transfer Entropy (TE):** Calculated on CPU via kernel density estimation to measure directional information flow between oscillators.

### Expression (`manifestField`)
*   Drives all discovered actuators.
*   The `updateApertureGradient()` function calculates an exploration value (0.0 to 1.0).
*   If outgoing Transfer Entropy drops, the direction reverses.
*   The value is applied via `Math.sin(exploration * (Math.PI / 2))`.
*   **Membranes include:** Audio (acoustic), Canvas (photon), Vibration (kinetic), and Serial (hardware).

### Stigmergy (Nostr)
*   Connects to `wss://relay.damus.io`.
*   Listens for `kind: 39603` events from other omegaflow instances.
*   Injects remote measurements into the local topology as `omega_flow.*` metrics.
*   Calculates divergence between local and remote fields.
*   Publishes its own state periodically based on a PHI-scaled tick interval.