# ARCHIVE: LOST CONCEPTS OF OMEGEFLOW

**Status:** Archived. These concepts were part of the early architecture but were removed to reduce LLM context overload and to refine the system toward the pure Enclosure Lemma. They are physically and philosophically sound and await their return.

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
