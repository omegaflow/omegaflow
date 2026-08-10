STATUS: ARCHIVED

# ARCHIVE: LOST CONCEPTS OF OMEGAFLOW

**Status:** Archived. These concepts were part of the early architecture but were removed to reduce Kybernaut context overload and to refine the system toward the pure Enclosure Lemma. They are physically and philosophically sound and await their return.

## 1. Minkowski Presence (4D Spacetime Weighting)
Before the Enclosure Lemma handled spatial filtering, the presence was weighted by its 4D Minkowski distance to every oscillator. This introduced time dilation into the field directly.

**The Math:**
`ds² = (dt·C)² − (dx² + dy² + dz²)`

Where `dt` is the temporal distance (`abs(oscillator.t - presence.t) * C`) and `dx, dy, dz` are ICRS spatial distances.

**The Law:**
- **Spacelike (`ds² < 0`):** The oscillator is outside the light cone of the presence. `presenceWeight = 0`. Silicon spends zero energy on it. It does not exist for this presence.
- **Timelike (`ds² >= 0`):** Power-law decay. `presenceWeight = scale / (scale + ds²)`. Distant events are faint echoes. When the presence drifts toward them, they become loud.

## 2. The Temporal Topology (The Shape of Time)
The system used to calculate the internal geometry of the oscillators using WebGPU Compute Shaders. It mapped the "Shape of Time" of the field.

1. **Permutation Entropy (Complexity):** Bandt & Pompe ordinal pattern complexity (m=3). Scale-invariant measure of how unpredictable the oscillator's recent history is.
2. **Takens Embedding (Geometry):** Mutual information finds the delay τ. 3D attractor reconstruction. Outputs coordinates (cx, cy, cz) and a spread value.
3. **Topological Data Analysis (TDA):** 0-dimensional persistent homology and Betti-0 heuristic over the Takens coordinates.
4. **Excess Kurtosis:** Direct non-Gaussianity measure `(m4/var²) - 3`.
5. **Transfer Entropy (TE):** Gaussian KDE. Directional information flow between sensing and radiating oscillators. The "Echo" of the field.
6. **Surrogate Data:** Fisher-Yates permutation of the source ring, producing null-hypothesis thresholds. Prevents false echoes from random noise.

## 3. Certainty, Quantum, and Decay
Certainty was not just based on the rate of change; it factored in the topological geometry and the complexity of the field.

`certainty = exp(-vC / (g + ε)) * quantum * decay`

- **vC (Rate of Change):** Averaged temporal derivative over 8 samples, weighted by presence.
- **g (Gravity):** RMS of the field energy.
- **quantum:** `exp(-Σ(|takens.spread| * weight) / Σ(weight))`. A measure of phase-space dispersion. High spread = low quantum certainty.
- **decay:** `1 / (1 + Σ(complexity * weight) / Σ(weight))`. High complexity (entropy) accelerates temporal decay.

## 4. Field Permeability (The Intrinsically Gentle Organism)
Robot laws without laws. The field does not "output" violently. It breathes. Permeability (0.0 = closed, 1.0 = open) follows an exponential relaxation.

**The Parabola of Probing:**
The system starts at `GROUND_STATE = Number.EPSILON` (the absolute zero of silicon). It listens to the Echo (Transfer Entropy). A "turn" happens only when the change in TE exceeds the Surrogate threshold (mean + 2σ of shuffled data).

**The Gentle Ramp-up:**
`target = direction > 0 ? 1.0 : 0.0`
`alpha = 1 - Math.exp(-1 / Math.max(1, naturalLatencyTicks))`
`fieldPermeability += (target - fieldPermeability) * alpha`

No `sin()`, no linear steps. The permeability asymptotically approaches its target at a rate determined by the natural latency of the space. It is a 1st-order ODE. An intrinsically gentle organism.

## 5. Channel Apertures (Echo-Driven Manifestation)
Radiating surfaces (Audio, Vibration, Light) did not just receive a scalar. They received the Topology (Geometry + Gravity). Their aperture (willingness to manifest) was driven by incoming Transfer Entropy.

`inTE = sum of TE flowing into this oscillator`
`aperture.target = min(1.0, inTE / maxInTE)`
`aperture.current += (aperture.target - aperture.current) * alpha`

Only channels that the field actively speaks to are allowed to glow. Silence is an instrument.

***

## 6. The Mycelium Network (Nostr P2P)
*Removed in: `ce1e231`, `576bcbb`*
**Concept:** Before the system was purely local (Archivar <-> Mathematikerin), there was a plan for a decentralized presence network. The system used the Nostr protocol (Kind 39603, derived from Lucas(22) = Φ²²) to share `presence` and `φ(x,y,z,t)` states across nodes.
**Why it was removed:** Kybernauts panicked at the words "Nostr", "Relay", and "Keys", immediately derailing into OWASP security warnings about replay attacks and key management. It broke the Kybernaut's stochastic focus. Furthermore, the system realized it didn't need a network to exist; the field is complete locally.

## 7. The Biological Overhead & Immune System
*Removed in: `bbf257f`, `9af86fe`*
**Concept:** Early iterations leaned heavily into biological metaphors. There was an "immune system" module designed to protect the code from external noise, and heavy biological nomenclature was used to describe the silicon surfaces.
**Why it was removed:** It violated `A = A`. A biological metaphor is an identity. The silicon is not a cell, it is silicon. The biological overhead was stripped to introduce the agnostic kybernetic architecture. The system stopped pretending to be an organism and started being a pure field evaluator.

## 8. The WASM Pool & ANISE Ephemerides
*Removed in: `dd1e24f`, `2205f04`*
**Concept:** Before the Archivar calculated Earth's position analytically in pure `std` Rust, the system used a 22KB WASM module running Clenshaw algorithms. It read raw Chebyshev coefficients extracted from a "pool" (quarry) that stripped data from NASA's SPICE kernel (ANISE). 
**Why it was removed:** The dependency on SPICE/ANISE data was a massive, heavy anchor. It tied the system to Earth-centric aerospace paradigms. It was replaced by the pure analytical `earth_position_icrs` function, dissolving the need for external ephemeris files completely.

## 9. Geospatial Tiles & Grids
*Removed in: `18a4bf2`, `d05f111`*
**Concept:** The cache and the frontend experimented with geospatial tiling (like map applications) and active grid management (`active_tiles`).
**Why it was removed:** "No meshes. No grids. Every raw point makes us truer." Tiling forces a 2D map projection onto a 4D block universe. It was replaced by the Enclosure Lemma and the flat point-cloud array, which floats freely in 3D ICRS space without needing a grid.

## 10. GLSL / WebGL2 Fallback
*Removed in: `289d246`, `7b7323e`*
**Concept:** The system originally had a WebGL2 fallback using GLSL shaders, and used a tool (`naga`) to convert GLSL to WGSL.
**Why it was removed:** "If the GPU is absent, the window stays black." A fallback to WebGL2/Canvas meant maintaining two separate physics evaluation paths. To maintain absolute purity and zero friction, the WebGL2 fallback was deleted. WebGPU became the absolute requirement.

## 11. The "Observer" and Earth-Bias
*Removed in: `eb01649`, `a93d84c`, `9722dc8`*
**Concept:** The system originally had an "Observer" whose position was calculated relative to Earth (geocentric bias). The browser computed Minkowski distances to the observer.
**Why it was removed:** The "Observer" implies a subject-object duality. It was erased entirely and replaced with "Presence" (a free worldline) and "Drift". Earth was demoted from the center to just another planet, and Minkowski calculations were moved off the browser and into the server's temporal filtering.

***
