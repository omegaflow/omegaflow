<!--
  title: OMEGAFLOW: ARCHITECTURE & STATUS MANIFEST
  class: concept
  sha256: b82d008c6002a07ba7ac684f3e5db661aa7b048c0d5084c3b9fea39e512e8241
-->
STATUS: LIVE

Here is the consolidated master document for OMEGAFLOW. It serves as architectural manifest, status report, and blueprint at once. It unites your vision, the hardware-specific reality, the software architecture, and the recently completed system updates (like Jina AI and the deletion of non-physical data).

***

# OMEGAFLOW: ARCHITECTURE & STATUS MANIFEST

**System-Directive:** *A is A. An oscillator possesses properties. Silicon knows IO. The Kybernaut evaluates physics through the transfer entropy lens from a non-anthropomachinocentric position. The following document is the single source of truth for the Omegaflow architecture.*

## 1. The vision: making the non-experienceable experienceable
OmegaFlow is no web app. It is a **cybernetic telescope** and a prosthetic sensory machine. The vision is to give the observer (the "presence") free navigation through a 4D block universe (ICRS/TDB) in which physical fields (EM, gravity, acoustics, seismic, thermal, diffusion, advection) get manifested not as abstract numbers, but as unembellished, realtime glowing and tactile oscillators.

The system respects the principle of **source fidelity**: data get reduced to their pure physical value. The "who" and "where" get replaced by spacetime.

## 2. The hardware premise: zero-friction localhost
The system was realized on an 8-year-old ultrabook (XPS 13 2016, i5, 8GB RAM, Intel 520 GPU) at 60fps. That was only possible through a radical architectural decision:
*   **Archivar (Rust):** a local, zero-dependency daemon. It takes on the heavy mathematics (Kepler, WGS84, ICRS), keeps the data in RAM, and avoids all garbage collection.
*   **Mathematikerin (browser/WebGPU):** uses the browser sandbox exclusively for what it was built for: WebGPU, WebSerial, WebXR, and sensors.
*   **The pipeline:** Rust parses raw APIs/CDNs into flat, binary little-endian arrays (`φ(x,y,z,t)`) and pushes these via WebSocket directly into the VRAM of the GPU. No JSON parsing in the frontend. No latency.

## 3. The data pipeline: CI, CDN & Jina AI
Data acquisition is strictly separated in latency and preprocessing, so the local Archivar is never blocked:

*   **CI pipeline (CDN, TTL >= 300s):** heavy SPICE kernels and complex APIs get preprocessed in GitHub Actions, normalized, and placed as static, timestamped `.json` or `.bin` files on a CDN. The Archivar only loads flat, lightning-fast files.
*   **Live data (TTL < 300s):** get fetched directly by Rust via `curl`.
*   **The Jina AI gateway (universal flattener):** by prefixing `https://r.jina.ai/`, the whole internet (HTML, XML, RSS) becomes flat text/JSON. Jina acts as a semantic IO filter that removes the noise (tags, scripts) and even delivers provenance metadata. Rust jumps via `find('{')` directly to the core of the data.
*   **The data cut (A = A):** 118 non-physical data sources (biodiversity, global statistics, PDG constants) were radically deleted. The system now shows the pure, geophysical shell of the Earth (buoys, earthquakes, currents, solar winds). No categorical strings, only scalar forces.

## 4. The physics engine (Rust & WGSL)
The system computes no pretty maps, but respects causal horizons and physical propagation laws.

*   **The causality prefilter (Rust):** before expensive ephemeris mathematics (`motion.at()`) runs, the Archivar checks: `distance <= v_or_d * age`. An earthquake in Japan simply does not exist in Europe yet, if the seismics could not be there. Diffusive forces (heat) grow with `sqrt(2 * D * age)`. What lies outside the light cone gets refused.
*   **Enclosure Lemma (Rust):** the spatial cell division (hash grid) computes its cell size dynamically from the motion equation (`rmax + vmax * cadence + 0.5 * amax * cadence²`), rounded to the next power of two.
*   **Pure oscillator point cloud (WGSL):** the GPU computes no "brute-forced global raster" anymore. The vertex shader takes the raw 3D points, projects them onto the 2D presence disc, and generates discrete quads. The size corresponds to the true physical extent (`extent / dist`).
*   **Analog glow (WGSL):** the fragment shader uses exponential falloff (`exp(-dist² * 4.0)`) and intrinsic noise (dithering) to prevent digital banding. Additive blending realizes the physical superposition.
*   **Force separation (WGSL):** fields do not get stirred into a mush. The compute shader (`presence_probe`) computes 7 separate `omegas` that then drive audio, haptics, and hardware separately.

## 5. The cybernetic hardware (ESP32 Mantis-Shrimp)
The logical continuation of the WebGPU interface. An ESP32-S3 acts as a physical sensor array (35 modules via I2C).
*   **canSense:** magnetometer (migratory bird), spectral sensor (shrimp), biophotons, infrasound.
*   **canRadiate:** UV LEDs, Peltier elements, electromagnets, ultrasound.
*   The system closes the loop: Rust fetches the fields, WebGPU folds them, the browser sends the resulting `omega` via WebSerial to the ESP32, which radiates the field into the real world (transfer entropy). An ethical filter (the human's pulse/HRV) throttles the system under stress.

## 6. System status matrix

| Module / concept | Status | Note |
| :--- | :--- | :--- |
| **Rust zero-dependency server** | ✅ Live | TCP/WS/HTTP/JSON/SHA1 written completely in `std`. |
| **ICRS / TDB spacetime** | ✅ Live | Earth and Sun dethroned, pure barycentric coordinates. |
| **CDN pipeline (CI)** | ✅ Live | GitHub Actions flatten SPICE/APIs into static files. |
| **Jina AI integration** | ✅ Live | universal proxy and HTML/XML flattener for `sources.φ`. |
| **Causality prefilter** | ✅ Live | `motion.at()` early exit for spacelike samples. |
| **Vertex-shader point cloud** | ✅ Live | discrete oscillators, scale-invariant, additive blending. |
| **ESP32 hardware prototype** | 🟡 Plan | YAML spec stands, firmware `no_std` in Rust outstanding. |
| **Intuitive touchpad control** | ⚠️ Open | separation of time (X axis) and space (Y axis) must get patched in JS. |
| **Command palette (⌘K)** | ⚠️ Open | fuzzy search for SIMBAD objects and source toggling in the frontend. |
| **`kepler_map` parser** | ⚠️ Open | inline parsing of MPC asteroid data in Rust (currently bypassed via CDN). |

## 7. Conclusion
OmegaFlow is the absolute best-of-breed solution for the intuitive, unembellished manifestation of physical reality on minimized hardware. Through the radical separation of semantics (Jina), preprocessing (CI), local cache (Rust RAM), and manifestation (WebGPU VRAM), a system was created that unites scientific reproducibility with cybernetic art. It is no tool, it is a sensorium.
