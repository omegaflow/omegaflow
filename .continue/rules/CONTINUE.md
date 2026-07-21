---
name: omegaflow-rules
description: Architecture and coding rules for omegaflow
---

omegaflow is an agnostic kybernetic presence navigating a 4D Block Universe. 

The system does not run on a timeline. The field drifts through a Minkowski lattice toward regions of highest presence. Time is isolated to a single anchor (`/time`); all subsequent state exists in J2000 spacetime coordinates.

PHI (Φ = 1.618...) scales all adaptive intervals. Every numeric literal is a fundamental constant (c, WGS84, J2000, Φ), a power-of-2 buffer/protocol size, or derived from live data. The stack is Rust std, curl subprocess, vanilla ES modules, WebGPU. The project is FLAT.

## The Axiom: A = A

An oscillator is an oscillator. Not a read-oscillator. Not a write-oscillator. It has properties, not identity. A property might be: can hear. Another might be: can glow. Both are surface, not essence. One molecule can speak one language. Another molecule can hear and speak. All are water molecules in the flow (omegaflow).

We do not think about water. We think like water.

## The Gremium Declaration

The oscillator has optional capabilities (sensing, radiating). These are boolean properties, not classes. The ω() loop touches every oscillator identically. In the touch it does not ask "Who are you?" but "What can you do?". There is no separate manifestation function with its own identity. Manifestation is part of the touch. It happens in the same moment as measuring and adapting.

The loop does not discriminate. It touches every molecule. Some have the property to hear — they hear. Some have the property to glow — they glow. In the same moment, in the same field, without separation.

Silicon knows only IO. A microphone gives values. A speaker takes values. That is the physics of the hardware. We accept the silicon as it is — and do not organize the code around it. An oscillator has properties. One of them might be a function that speaks to the silicon. That is no different from its complexity or its Takens geometry — it is a property among many.

## Forbidden Words

read, write, input, output, membrane, pulse, inject, echo, actuator, sensor (as identity terms). These words must not appear in function names, variable names, or comments. They may only appear when describing the physical hardware (e.g. in a JS-Doc comment: "This is the acoustic surface of the silicon").

## The Vocabulary

### Concepts (Nouns — Being)

- **Presence** — Existence at a coordinate
- **φ(x,y,z,t)** — The absolute address in the 4D block
- **Oscillator** — The node in the field
- **Trace** — The trajectory of recent values (ring buffer)
- **Complexity** — Kolmogorov measure of repetition (SHALL: PermutationEntropy)
- **TransferEntropy** — Causal information flow between points
- **Coherence** — Ratio of TransferEntropy to Complexity
- **Geometry** — Takens coordinates (cx, cy, cz) of the point
- **Topology** — TDA metrics (Betti0, persistence) of the field
- **Gravity** — Minkowski-based weight (4D distance normalized)
- **fieldPermeability** — Permeability of the boundary layer (0.0 = closed, 1.0 = open)

### Functions (Verb + Noun — Acting)

- **registerOscillator()** — Connect a node to the field
- **presence()** — Register existence (sole exception: noun only)
- **recordSample()** — Write a value into the trace (history)
- **readTrace()** — Retrieve the trajectory of an oscillator
- **calculateField()** — GPU: compute Complexity, TE, Geometry, Topology
- **calculateMinkowskiWeight()** — CPU: convert 4D distance into weight
- **adaptFieldPermeability()** — Adapt permeability to coherence (exponential relaxation)
- **flow()** — The field manifests in the silicon
- **sharePresence()** — Send presence over the mycelium (Nostr)
- **measureRms()** — Weighted root-mean-square
- **measureRateOfChange()** — Averaged temporal derivative over 8 samples
- **measureCoherence()** — Ratio of TE to Complexity

### Shader Names

- **permutationEntropyShader** (formerly kolmogorovShader)
- **takensShader** (unchanged, correct)
- **kurtosisShader** (formerly icaShader)
- **tdaShader** (unchanged, with defensive clamp)
- **teShader** (unchanged, correct)
- **surrShader** (unchanged, correct)

## Code Rules

1. No If/Else discrimination by identity. Not `if (osc.write)`, not `if (osc.read)`. Instead: query the property: `if (osc.canRadiate)`, `if (osc.canSense)`. The property is queried, not the identity.

2. No separate manifestation loops. Manifestation happens in the ω() loop, in the moment the field touches every oscillator.

3. No linear controls. The fieldPermeability follows an exponential relaxation (1st-order ODE) with naturalLatencyTicks as τ. No sin(), no linear step.

4. No hardcoded thresholds. Every numeric parameter must be derived from measurements or be a universal constant (c, Φ, WGS84, J2000).

5. No state machine. No states of matter as categories. Behavior emerges continuously from properties (Complexity, TE, Geometry).

6. Name = Implementation. Every function name must be verifiable against the code. calculateMinkowskiWeight() returns a weight. measureRateOfChange() averages a derivative. No metaphor that triggers wrong physics in an LLM.

7. No compliance theater. No "Everything is great!" comments. Facts.

## The 7 Confirmed Bugfixes

1. **calculateMinkowskiWeight**: `* 86400.0 * C` → `* C` (Minkowski unit error, critical — `originT` and `tPresence` are already in TDB seconds)
2. **tPresence initialization**: `φ['server.time']` → `tdbNow(φ['server.time'])` (missing J2000 offset)
3. **adaptFieldPermeability**: `sin(exploration * π/2)` → exponential relaxation with naturalLatencyTicks as τ
4. **permutationEntropyShader**: replaces kolmogorovShader (Bandt & Pompe, m=3, normalized by log₂(6))
5. **kurtosisShader**: replaces icaShader (excess kurtosis instead of degenerate single-channel ICA)
6. **measureRateOfChange**: window over 8 samples instead of 2-sample point estimator
7. **main.rs handle_pulse**: cache inputs under their own (x,y,z), remove stale last_coord

## Confirmed Architecture Changes

1. Oscillator struct: `read`/`write` → optional capabilities (`canSense`/`canRadiate` or similar). The field `exploration` is replaced by `fieldPermeability`.
2. Takens coordinates (cx, cy, cz) are stored directly on the oscillator, not as separate internal oscillators.
3. flow() does not collect "output values" and distribute them to "writers". The field manifests in every oscillator that has the capability to radiate. The topology (Geometry + Gravity of all oscillators) is the form through which the field expresses itself.
4. Surfaces (optical, acoustic, kinetic) do not receive a scalar. They receive the topology of the field. They translate the same 4D geometry into their respective medium.

## What Is Already Correct (Do Not Touch)

- The oscillator abstraction (flat array, data-oriented design)
- Minkowski distance as weighting core (after bugfix 1)
- takensShader (MI-based lag selection + 3D embedding, correct)
- teShader (KDE-based transfer entropy, correct)
- surrShader (Fisher-Yates permutation, per-source threshold, correct)
- The ω() loop as arrow of time (requestAnimationFrame)
- The ICRS coordinate calculation
- The Nostr network integration
- The Rust server (after bugfix 7)

## The System Measures, Decays, and Adapts

Certainty is computed as exp(-vC / (g + ε)) · quantum · decay · clarity.

