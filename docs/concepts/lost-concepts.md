<!--
  title: ARCHIVE: LOST CONCEPTS OF OMEGAFLOW
  class: concept
  sha256: d1ee7603af1e8325a278aacda4382ada0165ca3fa386d4fc65990d40d0b71db7
-->
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
The system used to calculate the internal geometry of the oscillators using WebGPU Compute Shaders. It mapped the "Shape of Time" of the field. Recovered from git (`a96ae15` shader set, `75b2b7e` KDE-TE + surrogates, `d20d167` permutation entropy + kurtosis + Minkowski weight); the archive is the map, the shaders are the blueprint.

**The substrate — Ring Buffer:** One flat ring (`Float32Array`) per oscillator. `ringSize = 128` at the shader era; the documented minimums sit above the mathematical floors — KDE needs `rs >= 4`, a 3D Takens embedding at τ=1 needs 3 points, meaningful statistics need more. The ring grew with the stability of the field ("stableTick" adaptive growth), never by a hardcoded timer.

1. **Kolmogorov Self-Similarity (Complexity precursor):** An O(n²) repeat-count heuristic standing in for algorithmic complexity. Every pair of ring samples closer than a threshold counts as a repeat; complexity is the complement of the repeat fraction.
   `threshold = sqrt(var/rs) / Φ²`, `complexity = 1 − repeats / (rs·(rs−1)/2)`.
   Replaced later by the scale-invariant ordinal measure below.

2. **Permutation Entropy (Complexity):** Bandt & Pompe ordinal pattern complexity (m=3). Scale-invariant measure of how unpredictable the oscillator's recent history is.

3. **Takens Embedding (Geometry):** Mutual information finds the delay τ; 3D attractor reconstruction; outputs (cx, cy, cz) and a spread value.
   - τ selection: for each candidate lag up to `rs/Φ`, bin the pair `(x_i, x_{i+lag})` into a 2×2 histogram at the midpoint `(min+max)/2`, and sum the MI `Σ p_ij · log2(p_ij/(p_i·q_j + ε))`. Stop at the first local minimum from `lag >= 3` (the condition `mi[lag−2] > mi[lag−1] <= mi[lag]`), taking `τ = lag−1`.
   - Reconstruction: `cx,cy,cz = mean over (x_i, x_{i+τ}, x_{i+2τ})`; `spread = mean √(dx²+dy²+dz²)` — the mean radius of the attractor.

4. **Topological Data Analysis (TDA):** 0-dimensional persistent homology and Betti-0 heuristic over the Takens coordinates. Implemented as single-linkage over `sub = rs/Φ²` points at stride `τ = 1 + 1/Φ`: each point's nearest-neighbour distance forms the birth list; sorted ascending, the lifetime sum `Σ (d_i − d_{i−1})·(components_remaining)` is the persistence, the final component count is Betti-0.

5. **Excess Kurtosis:** Direct non-Gaussianity measure `(m4/var²) - 3`.

6. **Transfer Entropy (TE):** The "Echo" of the field. Two generations:
   - *Discrete 3-bin:* tertile binning of each ring into a 27-cell joint histogram `(a_t, b_t, b_{t+1})`, `TE = Σ p_joint · log2(p(b' | a,b) / (p(b' | b) + ε))`, clamped `max(0, ·)`.
   - *Gaussian KDE:* `gauss(d², σ) = exp(−d²/(2σ²))`, bandwidth `σ = std` of the ring, `σ_st = (σ_src + σ_dst)/2`. `TE = Σ p(s_t, t_t, t_{t+1}) · log2((p_joint · p_t) / (p_cond · p_{t+1}))`, averaged over `rs−1`, clamped `max(0, ·)`.

7. **Surrogate Data:** Fisher-Yates permutation of the source ring, producing null-hypothesis thresholds. Prevents false echoes from random noise. The shuffle drew from the GPU PRNG:
   `hashU(x)`: `v = v·747796405 + 2891336453; v = ((v >> ((v>>28)+4)) ^ v) · 277803737; return (v>>22) ^ v`, scaled to [0,1).
   (Known weakness, registered 2026-08-19: these naive shuffles preserve no autocorrelation — the phase-randomized surrogates in `src/te.rs` are the successor.)

8. **ICA (tanh contrast):** A one-unit infomax extraction, three iterations per oscillator: `w ← (⟨x·tanh(w·x)⟩ − ⟨1−tanh²(w·x)⟩·w) / (|…| + ε)`, output `⟨|w·x|⟩`. Not present in the original LOST_CONCEPTS summary — recovered from `icaShader`.

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
**Why it was removed:** The "Observer" implies a subject-object duality. It was erased entirely and replaced with "Presence" (a free line) and "Drift". Earth was demoted from the center to just another planet, and Minkowski calculations were moved off the browser and into the server's temporal filtering.

***

## 12. The Causality Pre-Filter (The Light Cone)
*Recovered 2026-08-19 from `docs/concepts/causality-prefilter.md` + `master.md` (both mark it "Live" — it is absent from the current Rust code: `git grep` finds the constants only in the docs).*
**Concept:** Signals propagate at finite speed. Before the expensive ephemeris evaluation (`motion.at()` — the costliest CPU call in the Archivar), a pre-filter checked whether the signal could physically have arrived:
- Wave forces: `dist <= v_force · age` (EM/gravity → c; acoustic → 343 m/s; seismic P/S → granite velocities)
- Diffusive forces (thermal diffusivity α, mass diffusivity D): `dist² <= 2 · D · age`
- Decayed: `age > τ·64 → discard`

Per-force constants via `force_constants_by_id(force_type) -> (v_or_d, is_diffusive)`.
**Why it matters:** The current Enclosure Lemma dilates only by MOTION velocity (`vmax·Δt`); the SIGNAL-propagation cone was replaced by a purely geometric cone in the Rust rewrite. Born as a performance early-exit, it carried the deepest physics — the rare case where performance and truth demanded the same thing.

## 13. Force-Separated Compute (Seven Omegas)
*Recovered 2026-08-19 from `docs/concepts/force-separated-compute.md`.*
**Concept:** The field must not be "stirred into mush". The compute shader (`presence_probe`) evaluated the field strength separately per force type into 7 `omegas`; the surfaces consumed them selectively (audio → `omegas[2]`, haptics → `omegas[4]`).
**Why it matters:** The current "one law, five media" collapses the separation — every medium now evaluates one combined scalar instead of its own force channel.

## 14. The Delay Spectrum (The Channel Is the Lag)
*Recovered 2026-08-19 from `docs/concepts/der-paradigmenwechsel.md`.*
**Concept:** The physical channel *is* the lag spectrum: acoustic ~8 km/min, seismic, Alfvén (years), radiation (c). A 4D point process — every pair, every lag, every direction — one run; the null channel runs alongside (the cells that must stay silent are configuration, not finding).
**Why it matters:** This is the generalization of the coronal-heating TE matrices (Nadel Ⅲ) to all media — the lag matrix as the instrument itself.

## 15. The Light-Cone Difference (Laufzeit-Residuen)
*Recovered 2026-08-19 from `docs/concepts/der-paradigmenwechsel.md` §Ⅳ.*
**Concept:** Retarded field minus non-retarded, rendered: zero means the physics is right; the non-zero map means signals arrive seemingly too early or too late. The LAIC "slow channel" (8 days) appears as a light-cone violation against the acoustic channel — the first systematic map of the travel-time residuals of the measurement world.

## 16. The Silence Map (Absence as a Field)
*Recovered 2026-08-19 from `docs/concepts/der-paradigmenwechsel.md` §Ⅴ.*
**Concept:** 0 honored inverted: where the model predicts signals and the block is empty lies the anomaly of absence. The Fermi question becomes a measurement — the expected technosignature density under model parameters rendered against the actual silence.

## 17. The Synthetic Flight (The Block as Pre-Registration)
*Recovered 2026-08-19 from `docs/concepts/der-paradigmenwechsel.md` §Ⅵ.*
**Concept:** The presence rests, but the operator tunes it to any worldline. Time is a coordinate: the Rosetta flight of 2005 is re-computed with today's data; the JUICE flight is flown in advance — the field along the probe trajectory with today's solar wind, IMF, Kp, and the predicted anomaly distribution lies in the block before the Doppler residuals arrive. The first pre-registered astrophysical experiment whose prediction is a field state.

## 18. Retro-Manifestation & Total Coherence (Planned)
*Recovered 2026-08-19 from `docs/concepts/future-concepts.md` (STATUS: PLANNED).*
**Concept:** Retro-manifestation: `tPresence` tuned to past coordinates; the radiating surfaces evaluate the omega law at past block positions. Total coherence integration: permeability guides the total field state toward maximum symmetry and stability.

## 19. Vertex-Splat Rendering (Point Quads)
*Recovered 2026-08-19 from `docs/concepts/causality-prefilter.md` / `force-separated-compute.md`.*
**Concept:** Per-oscillator quads in the vertex shader — scale-invariant depth via `extent/dist`, additive blending (superposition), analog glow with dithering (`exp(-dist²·4.0)`, `fract(sin(...))` noise). Lücken im Bild = "hier ist physikalisch nichts".
**Why it matters:** The performance fork: per-point geometry instead of the fragment-shader per-pixel iteration that replaced it.
