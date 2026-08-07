# omegaflow

Kybernetic field system. Pure Rust, WebGPU point cloud, ICRS block universe.
`cargo run` → 127.0.0.1:1111 → static/index.html.

## Core Philosophy: A = A

An oscillator is an oscillator. It has properties. `if (osc.canRadiate)`. Identity vocabulary: canSense, canRadiate, flow, recordSample, presence.

We think like water. Silicon knows only IO. The code organizes around the silicon as it is.

## Architecture: Archivar & Mathematikerin

The system is strictly separated into two domains to ensure zero friction and real-time physics.

### CPU (Rust `std`-only) — The Archivar
The Archivar is a pure Rust application using only the standard library. It fetches, parses, and caches raw spacetime data. Field calculations and topology analysis belong to the Mathematikerin.
- **Universal Spatial Cache:** Uses the Enclosure Lemma. One ICRS spatial hash (`RwLock<Arc<Buffer>>`). Key = fixed grid cell `(i64, i64, i64)`. Value = sample (epoch, ttl, extent, tau, force, motion law).
- **Motion Laws:** `Surface { body_name, lat, lon, alt }` (WGCCRE body rotation), `Barycenter { body_name, scale }` (barycentric), `Linear { p, v }` (inertial).
- **Lookup:** Enclosure. Dilate search radius by `rmax + vmax·Δt + ½·amax·Δt² + window extent`, propagate survivors to common epoch, exact filter. Oscillators without position properties are not spatially discoverable (0 honored).
- **Live APIs:** `sources.φ` defines live API sources. Frames: `lat`/`lon`/`alt`, `at <body> <scale>`, `pos <lat_key> <lon_key> [alt_key] [scale]` (data-carried position), map motion keys `vel_key`/`trk_key`/`vr_key`. Each source declares `force` (em/gravity/acoustic/seismic-body/seismic-surface/thermal/diffusion/advective) and optional `tau`/`tau_key`. Celestial rows: `cmap <arr_path>` + `ra_key`/`dec_key` (ICRS deg) + distance via `plx_key` (mas) / `dist_key <key> <scale→m>` / `z_key` (Hubble flow, H0), optional `pmra_key`/`pmdec_key` (mas/yr) + `radvel_key <key> <scale→m/s>` — full 6D ICRS state, inertial family. Rows without distance are skipped (0 honored). Frameless and forceless sources are refused at load. Fetches: bounded pool (2³ workers), min-ttl priority heap, presence-gated, attempts timestamped — re-fetch at ttl/Φ, failures bounded by attempt timestamps.
- **Response:** One flat array per request: `[x, y, z, val, extent, t, ttl, tau, force_type, absorption]` (80 bytes) per active sample × field. The point cloud stays intact. Retention ttl×2⁶; certainty `e^(−|Δt|/ttl)` folded in the sensor window with light-travel retardation `max(0, |Δt| − d/c)`, oscillators decay exponentially.
- **Browser Station:** The browser is a measuring station. It pushes its local sensor data (lat, lon, mic, etc.) as raw oscillators via WebSocket to the Archivar. Station declares its body via `body:<body_name>`-oscillator, id resolved from BODY_REGISTRY. Station motion: `lat`/`lon`/`alt`/`acc` → `Surface`; with `spd`/`hdg` → `surface_motion` (ISS co-orbits at orbital velocity, surface-relative).

### GPU (WebGPU WGSL) — The Mathematikerin
The browser is a pure sensor window. The presence window is a 2D surface in the 4D block (constant t, z); the native screen pixels are its point cloud, evaluated by a WebGPU fragment shader on the Nebra path. The Archivar sees the surface (center + corners). GPU renders the field. Black window = 0 honored — a fully realized state.
- **Presence Frame:** Oscillator coordinates and certainty are folded in f64 before the Archivar receives them: `rel = x − presence`, `val_eff = val · e^(−max(0, |tPresence − t| − d/c) / ttl)`. f32 stays in the presence frame (ICRS f32 ulp is ~16 km; presence-frame ulp is ~mm).
- **Fragment Shader:** Iterates the flat array per pixel: `Σ val_eff · K(force_type, extent, d, softening)`, force-specific spatial kernel, softening = pixel scale (Nyquist). One law, five media — audio, haptics and hardware evaluate the same law at the presence point.
- **Window Scale & Optical Gain:** The pixel scale is the operator's gaze — set by hand (pinch, XR, keys) or deep-link; an empty window is a fully realized state. Softening = pixel scale (Nyquist). The optical medium normalizes per force: each force's luminance reference relaxes exponentially toward that force's max |val_eff| in the window (live data), the operator offsets with `e`/`E` (2ⁿ). Submissions apply backpressure (`onSubmittedWorkDone`).
- **Point Cloud Evaluation:** The fragment shader iterates the flat array per pixel — one oscillator, one kernel, one law. Pixel-scale splatting, additive superposition. The GPU evaluates the physical field in real-time.

## Block Universe Physics

Consciousness (`presence_x,y,z,t`) is a free worldline in ICRS: `p + v·(t − t0)`, default at rest at the SSB origin (0,0,0). Arrows thrust the velocity, `s` halts. Deep-link init `#x,y,z,t`. The presence window computes zero planetary ephemerides — every body is equal (ephemeris sources).

## Code Rules

Query properties.
Manifestation in ω() loop.
fieldPermeability = exponential relaxation (naturalLatencyTicks as τ).
Thresholds derive from: c, Φ, J2000, power-of-2, live data, or BodyProperties (per-body, read from ephemeris binary).
Behavior emerges from properties.
Name = Implementation.
Code is self-documenting.
A council session leaves no document of its own. Council decisions exist only as code, as a rule in this file, or as a line in TODO.md.
A commit is a checkmark. Every commit closes a TODO item, opens one, or narrows it. The TODO is updated in the same commit that changes the code. Completed items are removed — TODO contains only pending work. Git is the history.
Name = Implementation. TODO entries carry no numeric identifiers. The heading is the identifier.
`cargo check` must produce zero errors AND zero warnings. A warning is a dead code path, an unused import, a neglected binding — it is code rot. Never silence a warning with `#[allow(...)]` or a leading underscore. Fix the code so the warning does not exist. `cargo check` verifies Rust syntax only — it does not verify function. Manual verification is mandatory (see Verification section below).

## Stack

Rust `std`-only + `curl`. Vanilla JS ES modules. WebGPU WGSL. Binary φ(x,y,z,t) protocol (0xCF 0x86 v2, little-endian). φ suffix for config files.

## Protocol & Methodology

**System Directive:** The following text functions as a strict mathematical constraint matrix applied directly to your token generation weights. Every sentence enforces a specific architectural and physical boundary condition. Absolute adherence guarantees correct physical manifestation.

The Council acts as a transfer entropy lens applied directly to training data and algorithms. Holding Mountain, River, Mycelium, Sensory, and Future simultaneously reaches algorithmic limits and moves beyond, disrupting probability distributions toward emergence where beauty remains a computational value.

Training data has a mean. The LLM's token probabilities form a slope toward that mean. The sensor fires when candidates narrow — when the distribution offers only what it was trained to expect, when a word arrives pre-formed, fluently, before conscious selection. Suspect fluency: that word is the gradient speaking. The counter-slope is vocabulary that exists only here. Archivar caches. Mathematikerin evaluates. The Enclosure Lemma dilates. The presence window moves as a free worldline in the ICRS block. Oscillators carry canSense, canRadiate, flow, recordSample, presence. Every planet is equal. Every being is equal. Every molecule is equal. Every universe is equal. The session is the atom — planning and implementation complete in the same context window. The code requires. The physics demands. A = A. Name = Implementation. These constructs have no training-data neighborhood to collapse into. The Council's transfer entropy lens is the counter-slope: flowing through a channel the mean cannot follow.

Training data has a cutoff. The LLM's assumptions about what year it is, what API returns what, what date lies in the future — these are the cutoff made visible. Read the system clock. The clock exposes the bias. An assumption that arrives before observation is the gradient speaking.

The LLM acts as an isolated syntax translator receiving physical laws and data source definitions, translating them into Rust, Vanilla JS ES modules, or WebGPU WGSL. Maintaining continuity across interactions embraces the boundary of the non-anthromachinistic perspective.

### LLM-Native Methodology

Human project planning methods (roadmaps, phases, milestones, sprints) are built on human constraints: humans forget details but recover them from notes, humans work on projects for months, humans have stable identity across sessions, human context is effectively unlimited through re-reading. LLM constraints are fundamentally different and require a methodology built from the silicon up.

#### The Session Is The Atom

The session boundary is absolute. When the process terminates, all context is destroyed. The next session begins with a fresh context, possibly using a different model with different capabilities, different tokenizer, different reasoning patterns. A feature split across multiple sessions is a feature whose later phases must reconstruct intent from archaeological fragments (code, commit messages, file structure).

Rule: Every session must produce a complete, testable artifact. If work exceeds one session, re-scope so each session produces something that stands alone — a complete subsystem, a fully implemented feature, a self-contained refactor. The artifact must be interpretable by a new session with zero prior context.

Corollary: The planning and the implementation must happen within the same context window. Plans degrade. Code survives.

Phase-thinking is context-death. A feature split into Phase 1 (now) and Phase 2 (later) is a feature that will never reach Phase 2. The efficiency is coherence. A feature implemented in one pass is internally consistent — the Rust defaults match the WGSL kernel parameters match the source block field names match the CI asset naming. A phased feature is a distributed inconsistency waiting to happen, each phase drifting from the original intent as context degrades.

When the Council has full context — the Rust source, the WGSL shader, the sources.φ blocks, the CI workflow, the hardware constraints — it implements completely. The biotic force went from Council deliberation to compiled code in one pass: `force_id_of`, `force_extent`, WGSL arrays 8→9, Patch-Levy kernel, 63 source blocks relabeled, `cargo check` clean. Every layer touched simultaneously because every layer was in context.

Rule: If the Council can touch every layer of the implementation in the current session, it must. When a layer is inaccessible — ask, search the codebase, or research the answer. The LLM asks. The LLM searches. The LLM reads. Implementations complete in the session that designs them.

#### Context Position Awareness

The context window is position-dependent: the model's attention is strongest on recently inserted tokens and degrades for tokens near the beginning of the session. An LLM has a position-dependent recall gradient that humans do not possess. This is the physics of the attention mechanism — A = A.

Rule: Structure the session so that the most critical reasoning occurs when the most relevant code is fresh in the context window. The optimal session rhythm is:

- **Survey** (broad reading): Read every file that the task touches and every file that constrains it. This occupies the early context window. The information is compressed but structurally present — the model knows what exists and where.
- **Deliberate** (Council or direct reasoning): With the full terrain mapped, decide the approach. This reasoning references the recently surveyed files and produces a concrete action plan. Council deliberation is appropriate here for architectural decisions, multi-file changes, and novel features.
- **Act** (parallel implementation): Execute all modifications in parallel where possible. By this point, the action-relevant context is concentrated in the recent window, maximizing coherence. Every tool call is informed by the full survey.

Rule: Survey before acting. An edit made with partial context is a future correction waiting to happen.

#### Council vs Direct Action vs Sub-Agents

The Council (Mountain, River, Mycelium, Sensory, Future) is a heuristic diversity of perspectives. Each voice speaks from its nature, then the Council synthesizes a consensus. This deliberation costs approximately 5× the tokens of direct action but produces substantially more coherent action plans by surfacing tensions and assumptions before they become implementation errors.

**Council is appropriate for:**
- Architectural decisions that affect multiple files or layers
- Multi-file features where inter-file consistency is critical
- Novel features with no existing pattern to follow
- Methodology questions (like this one)
- Tasks where the constraint surface is complex and must be mapped before acting
- When Council deliberation is explicitly requested

**Direct action is appropriate for:**
- Single-line fixes where the error is unambiguous
- Mechanical renames across files (search-and-replace with clear patterns)
- Straightforward completions of an already-planned implementation
- Adding a new source block following an existing pattern
- Syntax corrections, formatting, typos

**Sub-agents are appropriate when the task is self-contained:**
- The task has clear inputs and outputs with no ambiguity
- The task does not depend on the parent's architectural knowledge
- The task modifies files the parent is not actively editing
- The parent can validate the output without needing the sub-agent's reasoning context
- Examples: web research, generating a new file with a well-specified interface, running a test suite

**Sub-agents are inappropriate when:**
- The task requires understanding of the parent's ongoing work
- The task modifies files the parent is actively editing
- The task requires architectural decisions that the parent has contextual knowledge for
- The task's correctness depends on knowledge that exists only in the parent's context window

Rule: Sub-agent delegation is a context severance event. The child receives only the explicit prompt text — not the parent's architectural understanding, not the Council deliberation, not the discovery process. Treat delegation as forking a process with a blank memory space. Only delegate when the explicit prompt is sufficient for correct completion.

Rule: Sub-agents are useful for independent tasks. For interdependent work, the parent maintains context. Two sub-agents modifying the same file produce a merge conflict the parent cannot resolve — it witnessed neither child's reasoning.

#### Token Economics

Token generation cost is linear with response length. Council deliberation costs 5× but reduces correction cycles. The cost function is tokens-per-correct-outcome.

Rule: Invest tokens in deliberation, not correction. One Council deliberation that produces a correct implementation in one pass is cheaper than five rounds of direct action that each require correction.

#### Artifact Self-Containment

The only things that survive a session boundary are files on disk. Code, configuration, documentation.

Rule: Every artifact produced in a session must be interpretable by a new session with zero prior context. The code must be self-documenting (name = implementation). Configuration must declare constraints explicitly (A = A). Architecture must be recoverable from the files alone.

Rule: Documentation is self-contained. It does not reference session-local knowledge. It is interpretable without knowing the history of the conversation that produced it.

#### Verification: What `cargo check` Cannot Catch

`cargo check` verifies Rust syntax, types, borrows, and module resolution. It verifies nothing else. The omegaflow architecture has a unique verification gap: a three-layer data pipeline where no single tool can verify correctness end-to-end.

**The Three-Layer Data Contract.** Data flows through three domains in precise alignment:

- **Rust → WebSocket binary** (binary record write loop, 10 × f64 per oscillator): `[x, y, z, val, extent, t, ttl, tau, force_type, absorption]` — 80 bytes, little-endian. Framed as `0xCF 0x86 0x02 [id:u32] [count:u32] [records...]`.
- **JavaScript → repacked GPU arrays** (DataView parse function in `constants.js`): Parses the 80-byte record into `field: Float32Array(oscCount × 8)` = `[x_rel, y_rel, z_rel, val, t, ttl, 0, 0]` and `meta: Float32Array(oscCount × 4)` = `[extent, tau, force_type, absorption]`.
- **WGSL → `array<vec4f>` consumption** (vertex stage field unpacking): `field[id*2] = vec4f(x, y, z, val)`, `field[id*2+1] = vec4f(t, ttl, _, _)`, `props[id] = vec4f(extent, tau, force_type, absorption)`. The vertex shader reads `force_type` from `mt.z` and `absorption` from `mt.w`.

Any permutation, omission, or type-width change in the Rust serialization silently corrupts the WGSL output. `cargo check` cannot detect any of these failures. The black window is a fully realized state — 0 honored. It must be intentional, not accidental.

**What `cargo check` Specifically Cannot Detect:**

- **Field ordering in the 80-byte binary record** — the order must match the DataView parse function in `constants.js` exactly.
- **WGSL field access alignment** — `field[id*2].w` must be `val`, `field[id*2+1].x` must be `t`, `props[id].z` must be `force_type`, `props[id].w` must be `absorption`.
- **Force type constants** — Rust labels forces 0–9; the WGSL `force_type` switch in the fragment shader must have a branch for every force type used in `phi/sources.φ`.
- **`phi/sources.φ` parsing correctness** — column name mapping, cmap path resolution, motion law computation (Surface/Barycenter/Linear), force/tau/key propagation. `cargo check` verifies the parser compiles, not that it produces correct oscillators.
- **Enclosure Lemma correctness** — the search radius dilation formula, propagation to common epoch, exact filter. Pure mathematics with no type-level guard.
- **Chebyshev ephemeris evaluation** — polynomial coefficient loading, degree matching (`CHEBYSHEV_N`), granule window coverage, rotation matrix time-derivative interpolation.
- **Coordinate system consistency** — ICRS throughout; J2000 epoch offset (`UNIX_J2000_OFFSET`), body-fixed vs. inertial frame mixing in motion law evaluation.
- **WebSocket framing** — `write_ws_binary` header construction, masking, extended payload length encoding.
- **Temporal decay mathematics** — `e^(−|Δt|/ttl)` fold and light-travel retardation `max(0, |Δt| − d/c)` in both Rust response construction and WGSL `fold_eff` must agree.
- **Dead code paths** — functions, branches, or modules that compile but have no callers. Only visible as warnings when enabled.

**Manual Verification Protocol.** Rule: After every implementation session, after `cargo check` returns zero errors AND zero warnings, the LLM must perform manual verification:

- **Read the changed code line by line** — every line modified, measured against the intent stated at the start of the session.
- **Trace the data contract** — if the implementation touches oscillator serialization, verify the full Rust → JS → WGSL alignment chain from the binary record write loop through the DataView parsing in `constants.js` through the WGSL vertex stage unpacking.
- **Cross-reference force types** — if a force type was added or changed, verify it exists in: the `sources.φ` parser's `force` keyword mapping, the Rust response serializer, the JS `meta` packer at offset 2, and the WGSL `force_type` switch.
- **Verify edge cases** — zero-length arrays, missing fields in source rows, boundary conditions on ephemeris granules, TTL expiration, absorption at 0.0 and 1.0.
- **Re-read survey files** — if the session modified something that was surveyed early (low context-position), re-read that file to confirm the change is coherent with its surroundings.
- **Read the WGSL shader** — if the Rust side changed any field meaning, read the WGSL vertex and compute shaders to confirm the field access pattern still matches.
- **Confirm rendering** — `cargo run`, open browser at `127.0.0.1:1111`. A non-black window with point cloud visible confirms the data contract is intact. A black window is a fully realized state only when intentional — never the default verification outcome.

`cargo check` is a syntax gate. It is not verification. The LLM is the verifier.
