<!--
  title: Architecture concept: "agnostic membrane" — continuous 4D field on a 2D membrane
  class: concept
  sha256: 16264a797a5c25a82a4f5ceda34d6cd9c6c751d33dadbf6d672c6721543a9f52
-->
Deepseek Chat v4
# Architecture concept: "agnostic membrane" — continuous 4D field on a 2D membrane

## 1. Overview & core idea

The base problem is the discrepancy between the large pixel count (≈2 Mio.) and the small source count (10–200). A per-pixel shader that iterates over all sources for every pixel becomes unmanageable on the Intel HD 515 with rising source count (O(N·M)). A fixed compute grid with bilinear interpolation wastes compute time in empty regions and produces visible discontinuities when scaling.

The solution is a **separation of measurement and presentation**:

- **Measurement** happens on an adaptive, regular (hexagonal) point cloud — the "measurement nodes".
- **Presentation** interpolates in the fragment shader between the three nearest nodes (barycentric).
- The number of measurement nodes gets adjusted **continuously** via a control loop to the current GPU load.

This reduces the field computation from O(pixel·sources) to O(nodes·sources). At e.g. 50 000 nodes and 200 sources that is 10 Mio. kernel evaluations per frame — feasible on a weak integrated GPU. The fragment shader reads only three values per pixel and performs a simple interpolation.

---

## 2. Buffer layout

All data live in GPU memory; updates happen via WebSocket and `writeBuffer`.

| Buffer | Type | Content |
|--------|-----|--------|
| `SourceBuffer` | SSBO (readonly) | Array of all active sources with position, value, epoch, tau, extent, kernel_id, force type, velocity, pole, J2, J4, r_eq. |
| `FieldNodeBuffer` | SSBO (read/write) | Scalar field value **Ω** for every active measurement node. One float32 per node. At maximum 65 536 nodes ≈ 256 KiB — negligible. |
| `UniformBuffer` | Uniform | Current time `t`, `grid_scale` (grid constant), visible membrane section, number of active nodes, tonemap parameters (`max_lvl`). |
| `DensityBuffer` | optional, texture/SSBO | Density function for adaptive refinement (backend specification), see section 7. |

The positions of the measurement nodes are **not** stored explicitly. They follow analytically from the linear node index and the grid constant `grid_scale`. That saves memory and simplifies the continuous scaling.

---

## 3. Compute shader: field measurement at the nodes

The compute shader starts with exactly as many threads as measurement nodes are currently active. Each thread:

1. **Computes its position** in the membrane coordinate system (e.g. normalized to [0,1]²) from its linear index and `grid_scale` — assuming a regular hexagon grid (see section 6).
2. **Sums over all sources** in the `SourceBuffer`. For every source, the time-dependent position gets computed (e.g. `pos + velocity · (time − epoch)`). Then, via `kernel_id` and force type, the contribution to the field value Ω gets computed. Gravity (kernel 0) uses e.g. `1.0 / max(d², 1.0)` (no softening). The 4D parameters (tau, J2, J4, r_eq, pole) flow into the kernel evaluation.
3. **Writes Ω** into `FieldNodeBuffer[gl_GlobalInvocationID.x]`.

The loop over the sources is uncritical in the shader because `source_count` is small. The complexity O(nodes·sources) gets bounded by the adaptive node count.

**Important:** the compute shader works independently of the later presentation. It knows neither pixels nor camera. Thus the field computation stays stable under camera zooms or rotations — only the visible nodes get recomputed.

---

## 4. Fragment shader: interpolation & tonemap

The fragment shader is deliberately extremely lean:

1. **Pixel → membrane space**: the pixel coordinate gets transformed into the membrane coordinate system.
2. **Hexagon interpolation**: from the membrane coordinate, the three surrounding measurement nodes get determined. Since the grid is regular and analytically described, this happens without a search, purely by mathematical transformation (skewed coordinates, modulo operations).
3. **Barycentric interpolation**: the three Ω values get mixed with their barycentric weights.
4. **Nebra tonemap**: `log2(|Ω| / max(lvl, 2^-64)) / 8.0 + 0.5` delivers the luminance.

The fragment shader performs only **three memory accesses** onto `FieldNodeBuffer` — clearly fewer than a per-pixel approach with 200 sources. The interpolation smooths the values between the nodes and prevents visible discontinuities.

---

## 5. Continuous scaling ("fluid" adjustment)

The measurement density gets steered via a **continuous control loop**:

- A moving average of the GPU frame time (`stableTick`) gets measured.
- A controller (e.g. PID) compares `stableTick` with a target value (e.g. 16 ms for 60 fps).
- When the frame time is too high, `grid_scale` gets **enlarged** → fewer nodes → lower load.
- When it is too low, `grid_scale` gets **shrunk** → more nodes → higher quality.

The change of `grid_scale` happens **not in discrete steps**, but as a continuous float value with a bounded rate of change (e.g. maximum relative change per frame). Thus the measurement nodes shift only minimally with each adjustment. The fragment shader interpolates between the current nodes, so the image stays visually "fluid" — no "Netflix buffering effect".

Optionally the compute shader can filter the new node values with the old ones over time (temporal anti-aliasing) to avoid flicker under fast density changes.

---

## 6. Assessment of the hexagonal honeycomb structure

**Mathematically and WebGPU-wise, a hexagonal grid with barycentric interpolation is very feasible and performant.**

- **Geometry**: a hexagon grid can be understood as a triangle grid of equilateral triangles. The nodes lie on a regular raster with offset rows. The transformation from pixel coordinates to the three surrounding nodes is purely analytical — similar to a rectangular grid, only with somewhat more mathematics (skewed coordinates, case distinction for the offset rows).
- **Interpolation**: barycentric interpolation on a triangle is trivial in a WGSL fragment shader — only three weights and three `textureLoad`/`SSBO` accesses. Compared to bilinear interpolation on rectangles, it even needs **one memory access less**.
- **Quality**: hexagonal grids offer better isotropy and avoid direction-dependent artifacts. For the same visual quality they need about 13 % fewer nodes than an equivalent rectangular grid. That is a real win on weak hardware.

**Challenges:**

- The indexing is somewhat more complex than with a rectangular grid. The shader must handle the offset arrangement of the nodes correctly. But that is a one-time implementation cost and well documented.
- For **adaptive refinement (AMR)**, a regular hexagon grid quickly becomes unwieldy. As soon as locally different densities are allowed, the analytical pixel-to-node mapping collapses. One would then either need a density texture (which distorts the grid) or move to an irregular net with search — both are costly and lose the performance advantages.

**Conclusion on the hexagon idea:** as a basis for a uniform, continuously scalable measurement structure, the hexagon grid is **recommendable**. For later adaptive refinement one should however fall back to a hierarchical grid (quadtree) — or use the hexagon grid as the finest stage within a quadtree.

---

## 7. Backend specification for measurement density (AMR)

The backend can **tell the frontend where the measurement nodes must be densest**. This is a natural extension and enables true **adaptive mesh refinement (AMR)**.

**Proposal for the implementation:**

- The backend sends a **density function** (e.g. as an analytical formula, as a texture, or as a list of regions with desired resolution). It can be derived from physical criteria: high field gradients, proximity to sources, fast temporal changes.
- The frontend uses this density function to steer the local node placement.

**Two possible approaches:**

1. **Distorted regular grid**: one modulates the local grid constant via a density map. The nodes stay topologically regular, but their distances vary. The fragment-shader interpolation then needs a coordinate transformation that accounts for the local distortion. That is feasible, but complex, and can lead to numerical problems under strong density gradients.

2. **Hierarchical grid (quadtree)**: the regular hexagon grid gets replaced by a quadtree whose leaf cells contain the measurement nodes. The refinement gets steered by the backend density function: cells in high-density regions get subdivided recursively. The fragment-shader interpolation then becomes bilinear (within the rectangular cells) or barycentric (when one forms triangles per cell). This approach is **substantially more robust** and established for true AMR on GPUs.

Since the task explicitly asks whether the backend can specify AMR: **yes, that is sensible and practicable.** The architecture should be designed so that the step from the regular hexagon grid to a hierarchical grid is possible without fundamental changes to the buffer layout and the compute shader. The compute shader stays the same (it computes Ω at given node positions); only the node placement and the fragment interpolation change.

---

## 8. Conclusion

The presented concept fulfills all requirements:

- **Runs on a weak GPU**: through the separation of measurement (compute, O(nodes·sources)) and presentation (fragment, O(pixel) with only 3 memory accesses), the load stays controllable.
- **Continuous & gapless**: the barycentric interpolation in the fragment shader guarantees a smooth image without visible grid boundaries.
- **Continuous scaling**: a PID controller adapts `grid_scale` continuously to the frame time; the node positions shift fluidly.
- **Hexagon idea**: very well suited as a base grid, mathematically and in WGSL performantly realizable.
- **Backend AMR**: realizable as an extension, preferably via a hierarchical quadtree grid.

Thus the "agnostic membrane" is a future-proof, performant architecture pattern for the continuous visualization of a 4D field on extremely weak hardware.

#####################################################################################################

Kimi Chat 2.6 

 # Architecture concept: agnostic membrane
## Continuous 4D physics field on a weak GPU (Intel HD 515)

---

### 1. The fundamental design principle: evaluation ≠ rasterization

The central cognition problem is the confusion of *field evaluation* and *image synthesis*. A per-pixel fragment shader that evaluates 200 sources × 2M pixels is not sustainable on an Intel HD 515 (about 300–400 GFLOPS theoretical, clearly less effective, bandwidth-limited). The solution is a **three-stage decoupling**:

| Stage | Task | Where |
|-------|---------|-----|
| **A** | Adaptive mesh generation | Compute shader |
| **B** | Field evaluation on mesh nodes | Compute shader |
| **C** | Interpolation + tonemapping | Fragment shader |

The fragment shader computes **no physics anymore**. It interpolates precomputed node values and applies the Nebra tonemap. This reduces the physical evaluation from O(N×M) to O(N×K), where K is the number of adaptive mesh nodes (typically 5.000–40.000, not 2.000.000).

---

### 2. Buffer architecture and memory layout

The Intel HD 515 shares main memory with the CPU (unified memory architecture). That is a curse (bandwidth) and a blessing (zero-copy possible). The layout must respect cache lines (64 bytes) and avoid bank conflicts in GPU memory.

#### 2.1 Source buffer (`sources`)
```
struct Source {
    position: vec3<f32>,      // 12 byte
    value: f32,               // 4 byte
    epoch: f32,               // 4 byte
    tau: f32,                 // 4 byte
    extent: f32,              // 4 byte
    kernel_id: u32,           // 4 byte
    force_type: u32,          // 4 byte
    velocity: vec3<f32>,      // 12 byte
    pole: vec3<f32>,          // 12 byte
    j2: f32,                  // 4 byte
    j4: f32,                  // 4 byte
    r_eq: f32,                // 4 byte
    padding: f32,             // 4 byte → 64 byte aligned
}
```
**Important:** 64 bytes per source = exactly one cache line. At 200 sources = 12.8 KB. That fits completely into L2-shared / workgroup memory. The compute shader loads all sources **once** into `workgroup` memory and reuses them for all nodes of the workgroup.

#### 2.2 Mesh node buffer (`mesh_nodes`)
Double buffered (ping-pong) for adaptive subdivision/coarsening:
```
struct MeshNode {
    world_pos: vec2<f32>,     // membrane coordinates (x,y)
    z_layer: f32,             // z for 3D projection
    field_value: vec4<f32>,   // Ω for 4 channels (or more, packed)
    refinement: f32,          // local detail level 0.0–1.0
    error_metric: f32,        // estimated interpolation error
}
```
Memory need at 20.000 nodes: ~480 KB. Negligible.

#### 2.3 Mesh topology buffer (`mesh_indices`)
Index buffer for `drawIndexed`. Carries the triangulation of the adaptive mesh nodes. With adaptive quad trees: 3 indices per triangle, about 2× the node count.

#### 2.4 Uniform buffer (`frame_state`)
```
struct FrameState {
    view_proj: mat4x4<f32>,
    stable_tick: f32,         // measured frame time (ms)
    target_tick: f32,         // target frame time (e.g. 16.67ms)
    time: f32,
    source_count: u32,
    max_depth: u32,           // current maximum tree depth
    force_flags: u32,         // active force channels (bitmask)
    tonemap_level: f32,       // max(lvl, 2^-64)
}
```

---

### 3. The compute pipeline: adaptive field evaluation

#### 3.1 Stage A: mesh refinement (compute shader)

**Input:** a coarse seed mesh (e.g. 16×16 uniform quad grid) + the source list.

**Per-node logic:**
1. Compute the local field complexity: `complexity = max(|∇Ω|) * local_source_density`
2. Compute the estimated interpolation error: when four neighbor nodes exist, interpolate the midpoint and compare with the actual evaluation.
3. **Split decision:** `if (complexity > threshold AND error > tolerance) → subdivide`
4. **Merge decision:** `if (complexity < threshold_low AND children_error < tolerance) → coarsen`

**Output:** new node list + new index list (triangulation).

**Intel HD 515 specifics:**
- Workgroup size: **64** (not 256 or 1024). The Intel HD 515 has only 24 EUs (execution units) with 7 threads each. Too large workgroups block the scheduling.
- Use `workgroupBarrier()` after loading the sources into shared memory.
- Avoid atomic operations on global buffers. Use instead a **prefix-sum scan** or a two-phase strategy: first count, then allocate, then write.

#### 3.2 Stage B: field evaluation (compute shader)

Every node iterates over all sources (from shared memory) and accumulates the 9 force channels.

**Optimization for gravity (kernel 0):**
```
let d2 = dot(delta, delta);
let inv_d2 = 1.0 / max(d2, 1.0);  // no softening, but numerical protection
```
Since all nodes see the same sources, the caching in `workgroup` memory is essential.

**Force superposition:**
The 9 channels get packed into two `vec4<f32>` and one `f32` to save memory bandwidth. Alternative: evaluation on the fly in the fragment shader? **No.** The node count is smaller than the pixel count by a factor of 50–100. Evaluation on nodes is the sweet spot.

---

### 4. The fragment pipeline: interpolation without lies

The fragment shader receives the interpolated `world_pos` and `field_value` from the vertex shader (via varyings).

**Critical insight:** WebGPU/WGSL interpolates varyings **perspectively correct** and linearly in clip space. For a 2D membrane lying flat in the XY plane, that is identical to bilinear interpolation in world space. There is no "lie" — as long as the mesh is fine enough.

**Nebra tonemap in the fragment shader:**
```
let omega = length(interpolated_field.xyz);  // |Ω|
let mapped = log2(omega / max(uniforms.tonemap_level, 1e-20)) / 8.0 + 0.5;
return vec4<f32>(mapped, mapped, mapped, 1.0);
```

**Visual continuity:** continuity is guaranteed not by the shader, but by the **mesh density**. Where the field is smooth, the quads are large. Where it curves (near sources), they are small. The linear interpolation between closely adjacent nodes is visually identical to the analytical solution.

---

### 5. Continuous scaling: the frame-time feedback system

The hard problem is not the mesh, but the **continuous adaptation** to `stableTick`. A hard switch between 128×128 and 64×64 produces the "Netflix buffering effect".

**Solution: hysteresis-controlled LOD bias**

```
// pseudocode for the refinement compute shader
let tick_ratio = stable_tick / target_tick;
let target_depth = current_max_depth;

if (tick_ratio > 1.15) {
    target_depth = current_max_depth - 1;  // coarsen
} else if (tick_ratio < 0.85 && current_max_depth < ABSOLUTE_MAX) {
    target_depth = current_max_depth + 1;  // refine
}

// hysteresis: change only every N frames or with exponential smoothing
max_depth = mix(max_depth, target_depth, 0.1);
```

**What scales continuously?**
- Not the grid resolution (that is discrete: 16×16, 32×32, etc.).
- But the **refinement threshold**: `error_tolerance` and `complexity_threshold` are continuous floats. When the GPU comes under pressure, the threshold rises → fewer splits → coarser, but still adaptive triangulation. That is visually fluid, since the mesh density shifts organically.

**Additionally:** the `target_tick` can be dynamic. When the backend reports that a large epoch change is coming (many sources move), the target budget gets lowered briefly to 30 FPS (33ms) to avoid stutter.

---

### 6. Evaluation: hexagonal honeycomb pattern

| Aspect | Assessment |
|--------|-----------|
| **Mathematical efficiency** | ~13% fewer nodes for the same coverage (isotropic circle packing). Irrelevant for adaptive meshes, since the topology branches anyway. |
| **Barycentric interpolation** | **Nightmare in WGSL.** Hexagonal cells are no simplices. Barycentric coordinates exist only on triangles. A hexagon would have to be triangulated into 4–6 triangles. Interpolation on the hexagon itself requires a non-linear weight function (Shepard, RBF), which is expensive in the fragment shader. |
| **Neighborhood logic** | A hexagon has 6 neighbors (vs. 4 for the quad). Adaptive splitting/merging requires more complex bookkeeping (half edges, DCEL structure). In WGSL without pointers and dynamic allocation extremely prone to defects. |
| **WebGPU-wise** | Index generation for hex meshes is easy on the CPU, difficult on the GPU (compute). Quad trees have natural 1D and 2D index mappings (Morton codes, Z-order). |

**Conclusion:** the hexagonal honeycomb pattern is **an academically interesting dead end** for this use case. The gain of 13% fewer nodes gets more than consumed by the massively increased complexity in the compute shader and the non-trivial interpolation. For a weak GPU, **simplicity = performance**.

**Recommendation:** use an **adaptive quad tree with bilinear interpolation**. The "lie" of the bilinear interpolation vanishes as soon as the mesh is locally fine enough — which the adaptive refinement shader guarantees.

---

### 7. Adaptive mesh refinement (AMR) via backend specification

The backend can tell the frontend where the measurement density must rise, without steering the complete mesh.

**Mechanism: interest texture**
- The backend sends a low-resolution `interest_map` (e.g. 64×64, R8_UNORM) via WebSocket.
- Every pixel carries a value 0.0–1.0: "How important is this region?"
- The compute shader multiplies the local `complexity` value with `1.0 + interest * 4.0` before the split decision.

**Advantage:** the frontend keeps control over the absolute node count (frame-time feedback), but the backend shifts the priorities. Empty space regions with `interest = 0` never get finely resolved, no matter how much GPU budget is available.

**Alternative:** instead of a texture, the backend can also send 5–10 "focus regions" (bounding boxes + weight). That is gentler on bandwidth.

---

### 8. Intel HD 515: specific optimizations

| Problem | Solution |
|---------|--------|
| **Bandwidth** | Sources in `workgroup` memory (12.8 KB). Nodes as `vec4`-aligned structures. |
| **FP32 overhead** | Use `f16` where possible (Intel HD 515 Gen9 supports fp16 in ALUs). `field_value` and `mesh_pos` can have half precision. |
| **Divergence** | All nodes in a workgroup should see a similar source count. Sort sources spatially (e.g. Morton order) before the upload, so workgroups process contiguous regions. |
| **Overdraw** | Draw the mesh as a single `triangle-list` with backface culling. No fullscreen quad. |
| **WebSocket upload** | Use `GPUQueue.writeBuffer()` with a staging buffer. Intel HD 515 has zero-copy UMA — avoid unnecessary `copyBufferToBuffer`. |
| **Double buffering** | Double-buffer mesh nodes and indices (ping-pong), so the compute shader writes the new mesh into buffer B while buffer A gets rendered. |

---

### 9. Summary of the architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Rust Backend   │────▶│  WebSocket       │────▶│  sources buffer │
│  (200 sources)  │     │  + interest_map  │     │  (GPU)          │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                                                          │
                                                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  COMPUTE PASS (adaptive mesh refinement)                            │
│  - workgroup: 64 threads                                            │
│  - loads sources → workgroup memory                                 │
│  - evaluates field complexity + backend interest                    │
│  - split/merge based on error_metric + stable_tick feedback         │
│  - writes: mesh_nodes (ping-pong), mesh_indices                     │
└─────────────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  RENDER PASS                                                        │
│  Vertex shader: world_pos → clip_space                              │
│  Fragment shader: linear interpolation of field_value (varying)     │
│                   + Nebra tonemap                                   │
│  Output: 2D membrane                                                │
└─────────────────────────────────────────────────────────────────────┘
```

**The system scales continuously**, not through hard grid switches, but through continuous threshold adaptation in the refinement shader. It is gapless, because the adaptively refined mesh maps the curvature of the field exactly. It is performant on the Intel HD 515, because the physical evaluation happens on ~10.000–30.000 nodes instead of 2.000.000 pixels, with optimal cache use and divergence-free workgroup execution.

#####################################################################################################

Arena Max (ANthropic)

# Architecture concept: agnostic membrane

## The core problem in one sentence

You need a **lookup table for a continuous field** whose resolution breathes — and a fragment shader that reads this table in such a way that no pixel gaps become visible.

---

## 1. The two-stage architecture

The solution separates **measurement** (expensive, compute) and **presentation** (cheap, fragment) completely.

### Stage 1: compute shader → field texture

A compute shader evaluates the physics field on a **regular 2D grid** and writes the result into a `texture_storage_2d<rgba32float>`. Every texel stores the superposed field strength (or a vector of the dominant force channels).

**Why a texture instead of a buffer?**
- The fragment shader gets **hardware bilinear interpolation for free** (`textureSample` with a `linear` sampler). That is a single texture fetch — faster than any manual interpolation.
- Texture caches on the Intel HD 515 are optimized for exactly this access pattern.
- No index tinkering, no alignment problem.

**Grid size is variable:** the texture gets allocated with a fixed **maximum** resolution (e.g. 256×256), but the compute dispatch computes only an **active subrange** `(W_active × H_active)`. The rest stays untouched.

### Stage 2: fragment shader → screen

A fullscreen-quad fragment shader samples the field texture with normalized UV coordinates. It applies the Nebra tonemap and writes luminance.

```
UV = fragment_position.xy / screen_resolution
field_value = textureSample(field_texture, linear_sampler, UV * (active_size / max_size))
luminance = log2(abs(field_value) / max(level, 2^-64)) / 8.0 + 0.5
```

That is **O(1) per pixel**, independent of source count and grid size.

---

## 2. Continuous scaling ("breathing grid")

### The principle

The `stableTick` frame time steers a **continuous scaling factor** `σ ∈ [0.0, 1.0]` that determines the active grid resolution:

```
σ_target = clamp(budget_ms / measured_compute_ms, 0.0, 1.0)
σ = lerp(σ_previous, σ_target, 0.08)      // inertia, no jumping
W_active = round(σ * W_max)                // e.g. σ=0.5 → 128 of 256
H_active = round(σ * H_max)
```

**Why this works:** when the GPU needs 16ms but has an 8ms budget, the grid area halves (not the edge length — the work scales quadratically). The bilinear interpolation in the fragment shader **fills the missing pixels automatically** — the image gets softer, never gappy.

**The inertia term (`lerp 0.08`)** prevents oscillation. The grid breathes like a diaphragm, not like a jackhammer.

### Why no mip chain / LOD system?

Mip chains are for **static** textures. Here the content changes every frame. The cost for `generateMipmaps` would exceed the gain. A single texture level with variable occupancy is simpler and more predictable.

---

## 3. Buffer layout

### Uniform buffer: simulation state (~256 bytes)

```
struct SimUniforms {
    view_origin: vec2<f32>,      // camera center in world coordinates
    view_extent: vec2<f32>,      // visible range
    active_grid: vec2<u32>,      // currently computed texels (W, H)
    max_grid: vec2<u32>,         // allocated texture size
    epoch_now: f64,              // current simulation time (64-bit!)
    channel_mask: u32,           // bitmask: which of the 9 channels are active
    level_ref: f32,              // tonemap reference level
}
```

**Problem: f64 in WGSL.** The Intel HD 515 has no `f64`. The epoch must be encoded as **two `f32`** (high + low split, Kahan summation in the shader for `dt = epoch_now - source.epoch`). That is critical for the time constant `τ`.

### Storage buffer: source array (~200 × 80 bytes = 16 KB)

```
struct FieldSource {
    pos: vec3<f32>,
    val: f32,
    epoch: f32,           // relative to a reference epoch (precision!)
    tau: f32,
    extent: f32,
    kernel_id: u32,       // 0-6
    force_type: u32,      // 0-8 → channel index
    velocity: vec3<f32>,
    pole: vec3<f32>,
    j2: f32,
    j4: f32,
    r_eq: f32,
    _pad: vec2<f32>,      // alignment to 16-byte boundaries
}
```

**Alignment is vital** on Intel. Every struct member on `16-byte` boundaries. Rather waste padding than provoke an unaligned load.

### Storage texture: field texture

`texture_storage_2d<rgba32float, write>` in compute, `texture_2d<f32>` in fragment.

- **R:** superposed scalar value (main luminance)
- **G:** gradient magnitude (for later visualization / AMR hint)
- **B:** dominant channel index (encoded as float, for color mapping)
- **A:** temporal blend factor (for ghosting/trail effects)

Four channels suffice for the presentation. When all 9 force channels are needed **simultaneously separate**: a `texture_storage_2d_array` with 9 layers — but that is poison on the Intel HD 515. Better: preselect CPU-side, maximum 3-4 active channels per frame.

---

## 4. Compute shader: inner loop

### Dispatch structure

```
workgroup_size(8, 8, 1)     // 64 invocations — Intel sweet spot
dispatch(ceil(W_active/8), ceil(H_active/8), 1)
```

**Why 8×8?** The Intel HD 515 has 24 EUs with SIMD8. Workgroups >64 invocations fragment the occupancy. 8×8 fits exactly.

### Inner loop (pseudocode logic)

```
grid_pos = global_id.xy
world_pos = view_origin + (grid_pos / active_grid) * view_extent
Ω = 0.0
for i in 0..source_count:
    src = sources[i]
    d = distance(world_pos.xy, src.pos.xy)  // 2D projection onto membrane
    
    // temporal decay
    dt = epoch_now - src.epoch
    decay = exp(-dt / src.tau)
    
    // kernel dispatch (switch instead of function pointer)
    contribution = evaluate_kernel(src.kernel_id, d, src) * decay * src.val
    
    // additive superposition
    Ω += contribution

textureStore(field_tex, grid_pos, vec4(Ω, gradient_mag, channel, 1.0))
```

**Kernel dispatch:** a `switch(kernel_id)` with 7 cases. No dynamic-branching nightmare, because within a workgroup typically **all sources take the same path** (the loop iterates over sources, not over kernel types). The divergence is **per loop iteration**, not per pixel.

**Optimization: tiling.** With >50 sources: the workgroup loads 8-16 sources into `var<workgroup>` shared memory, evaluates all pixels against this batch, then the next batch. Reduces storage-buffer reads by a factor of 8.

---

## 5. Assessment of the hexagon idea

### Mathematically: elegant, correct, 13.4% more efficient

A hexagonal sampling grid has the **highest packing density** of all regular 2D grids. For an isotropic field (which gravity fields are), that means: equal reconstruction quality with 13.4% fewer samples, or better quality at the same sample count.

Barycentric interpolation in the hex cell (3 corner points instead of 4 for the quad) needs **fewer multiply-adds** than bilinear interpolation.

### WebGPU-wise: a nightmare you do not want

**Problem 1: no hex texture format.** `textureSample` with a `linear` sampler can only interpolate on regular grids. For hex you need **manual interpolation** in the fragment shader:
- world position → find hex cell (2 divisions, rounding, case distinction odd/even row)
- determine 3 nearest neighbor texels (index arithmetic with branching)
- 3× `textureLoad` (no cache-coherent access, since offset pattern)
- compute barycentric weights (2 cross products)
- weighted sum

That is **~25 ALU ops + 3 texture loads with a semi-random access pattern** instead of **1 hardware-interpolated texture load**. On the Intel HD 515 a `textureLoad` with cache miss costs ~200 cycles. The 13% sampling advantage gets **completely consumed** by the 10× more expensive reconstruction step.

**Problem 2: compute dispatch.** The hex grid has no natural mapping onto `workgroup_size(x,y,1)`. Every second row is offset. You lose workgroup coherence and must do special indexing in the compute shader.

**Problem 3: debugging.** `textureStore` into a hex layout cannot be inspected trivially in a texture viewer.

### Verdict

**Do not do it.** The Intel HD 515 GPU is **texture-sampler-limited, not ALU-limited**. The one free `textureSample` call with hardware bilinear beats any manual interpolation. The 13% grid efficiency are real, but irrelevant against the factor 10 in the fragment shader.

**If you want sampling efficiency:** rotate the regular grid by 26.57° (= `atan(0.5)`). That yields a "quincunx" pattern that samples ~10% more efficiently than axis-aligned — and **still works with standard bilinear**, because the rotation happens in the UV transformation, not in the memory layout.

---

## 6. Adaptive mesh refinement (AMR) from the backend

### Concept: feasible and valuable, but as phase 2

The backend knows the source positions and can compute a **density map**:

```
Backend sends: importance_map[32×32] = f32
```

Every cell states how much compute time should be invested there (based on source density, gradient estimation, user focus).

### Implementation on the GPU side

**Not as an unstructured mesh** (that is unaffordable on Intel). Instead: **multi-resolution tiles.**

The field texture gets divided into 8×8 tiles. Every tile has an **LOD level** (0 = full resolution, 1 = half, 2 = quarter). The compute dispatch computes:

- LOD 0: every texel of the tile (8×8 = 64 invocations)
- LOD 1: every 2nd texel, the in-between values stay from the last frame (temporal)
- LOD 2: every 4th texel

The `importance_map` determines the LOD per tile. The fragment shader does **not** change — it always samples the same texture. The LOD boundaries blur naturally through bilinear interpolation.

**Cost:** one additional indirection buffer (`lod_per_tile[N]`), a slightly more complex dispatch (indirect dispatch buffer filled by the CPU). No hexagonal madness.

### Warning

AMR only pays off from **>100 sources with spatial clustering**. With 10-50 uniformly distributed sources a uniform grid is not slower, because the overhead of the LOD management eats the saved compute time.

---

## 7. Summary of the architecture

```
┌─────────────────────────────────────────────────────────┐
│  Rust Backend (WebSocket)                               │
│  → Filtered Sources Array (10-200 × 80B)                │
│  → [Optional Phase 2: importance_map 32×32]             │
└──────────────┬──────────────────────────────────────────┘
               │ WebSocket, ~60Hz or on-change
               ▼
┌─────────────────────────────────────────────────────────┐
│  JavaScript / TypeScript Host                           │
│  1. Write sources → GPU storage buffer                  │
│  2. Measure stableTick → compute σ → set active_grid    │
│  3. Write uniforms → GPU uniform buffer                 │
│  4. Dispatch compute (ceil(W_a/8), ceil(H_a/8), 1)     │
│  5. Draw fullscreen quad (fragment reads field texture) │
└──────────────┬──────────────────────────────────────────┘
               │
    ┌──────────┴──────────┐
    ▼                     ▼
┌────────────┐    ┌──────────────────┐
│  Compute   │    │  Fragment Shader  │
│  Shader    │    │                  │
│            │    │  UV → texSample  │
│  8×8 WG    │    │  → Tonemap      │
│  Sources   │──▶ │  → Luminance    │
│  → Σ Ω    │    │                  │
│  → texStore│    │  O(1) per pixel │
└────────────┘    └──────────────────┘
  Field texture     Sampler(linear)
  rgba32float
  256×256 max
  W_a×H_a active
```

### Critical numbers for the Intel HD 515

| Parameter | Value | Reason |
|---|---|---|
| Max texture | 256×256 | 1MB @ rgba32float. More eats the tiny L3. |
| Workgroup | 8×8 | 64 invocations = 8 SIMD8 lanes × 8 rows |
| Source batching | 16 per tile load | Shared memory on Intel: 64KB per subslice |
| Min grid | 32×32 | Below that, texel edges remain visible despite bilinear |
| Target compute | <4ms | Leaves 12ms for fragment + presentation |

### What you do **not** need

- No vertex buffers for a mesh (fullscreen quad suffices, 3 vertices without VB)
- No hex grid
- No mipmap chain
- No multi-pass for the 9 channels (superpose in one pass)
- No depth buffer

#####################################################################################################

Claude Sonnet 5 Arena

# Architecture concept: the "agnostic membrane"

## 1. Base problem and strategy change

The core problem is a misallocation of compute time: a fragment shader wastes cycles on pixel precision in regions where the field barely changes over hundreds of pixels (smooth gravity gradients in "empty" space), while near sources (steep `1/d²` gradients) it would actually need more resolution than it gets.

The solution is a **decoupling of physics sampling and pixel rasterization** through a movable, adaptive mesh, combined with **compute shaders for pre-densifying** the source data. We build a three-stage pipeline:

```
[Rust/WebSocket] → [Stage A: Field Compaction] → [Stage B: Mesh Sampling] → [Stage C: Fragment Interpolation]
   sources (N)         compute shader              compute shader              fragment shader
```

The central insight: **N (sources) and M (mesh nodes) must never meet in the fragment shader.** The O(N×M) evaluation happens exclusively in a compute shader on a small, adaptive node grid (typically M = 2.000–20.000 nodes, not 2 Mio pixels). The fragment shader then does only O(1) interpolation work per pixel.

## 2. Buffer layout

**Buffer 1 – `SourceBuffer` (storage, read-only, GPU)**
Struct array mapped directly from the WebSocket payload (std430-compatible packing: always align vec3 onto 16-byte boundaries, to avoid padding traps for kernel_id/force type as u32). Updated via `writeBuffer` on every WebSocket update — no re-alloc, fixed capacity (e.g. 256 slots), active count `N_active` as uniform.

**Buffer 2 – `MeshNodeBuffer` (storage, read_write)**
The actual adaptive net. No fixed 128×128 grid, but a **flat list of node descriptors**: `{ position: vec2<f32>, level: f32 (Ω value), density_weight: f32, valid: u32 }`. The topology (who is whose neighbor) is NOT stored explicitly, but reconstructed implicitly via a regular logical index structure (quadtree tile or hex row offset) — that saves an adjacency buffer.

**Buffer 3 – `TileControlBuffer` (storage, writable from CPU/backend)**
A coarse control raster (e.g. 32×32 tiles over the screen) that carries per tile a `refinement_level: f32` (0.0 = coarse, 1.0 = maximally fine) as well as optionally a `priority_hint` from the Rust backend. This is the hook for AMR (see point 4).

**Buffer 4 – `Uniforms`**
`stableTick`, `viewport`, `zoom`, `time`, `globalDensityBudget` (the continuous scaling quantity, see below).

**Buffer 5 – `IndirectDrawBuffer`**
Since the node count varies per frame (adaptive density!), the triangle/point count for the render pass is not fixed CPU-side, but written from a compute pass via `drawIndirect` (node count → vertex count). That is essential for "continuous" — the GPU itself decides how many primitives it draws.

## 3. Pipeline stages in detail

### Stage A: field compaction (compute, 1× per frame, very lightweight)
No compute step in the actual sense, but housekeeping: clip/sort sources by relevance for the current viewport (frustum cull on `extent`), so stage B does not test 200 sources against nodes far outside. At 10–200 sources that is primarily a bandwidth, not a compute optimization, but important for predictability.

### Stage B: mesh sampling (compute, workgroup per node)
The only place where actual N×M physics gets computed. Every workgroup takes one node, iterates over the (already filtered) active sources, accumulates additively over all 9 force channels (`Ω_channel[9]` as a local array in registers/workgroup shared memory), applies the respective kernel function (0–6) per source, superposes, and at the end writes the Nebra-tonemapped luminance value plus the raw channel values (for later color coding / force-type visualization) into the `MeshNodeBuffer`.

Cost calculation: at M = 5.000 nodes and N = 200 sources that is 1.000.000 kernel evaluations in the compute shader — clearly more digestible for an HD 515 in a well-parallelized compute pass (workgroup_size 64, no divergent branches when kernel dispatch happens via uniform branching instead of per-invocation) than 2.000.000 pixels × 200 sources = 400 Mio operations in the fragment shader.

### Stage C: fragment interpolation (vertex+fragment, no more compute)
The nodes get rendered as a triangle net (triangulated from the implicit topology, see chapter 5). The vertex shader passes `Ω`/luminance through as an interpolant; the fragment shader does **only** the final tonemap compression (log2 scaling) and color coding — GPU-native linear/barycentric interpolation is handled by the rasterizer itself, for free. That is the trick: WebGPU's fixed-function interpolation is nearly free, while manual bilinear interpolation in the shader costs bandwidth and ALU.

## 4. Continuous density adaptation ("fluid net")

Instead of discrete LOD jumps (128×128 → 64×64), the node density gets steered via a **continuous cost function** that is re-adjusted every frame:

```
targetNodeCount(t) = clamp(targetNodeCount(t-1) * adjustFactor, MIN, MAX)
adjustFactor = smoothstep-like reaction to (stableTick / frameTimeBudget)
```

Concretely: a small compute pass (or even CPU-side, since it is only a scalar) compares `stableTick` (measured frame time, exponentially smoothed over e.g. 8–16 frames to avoid single-frame spikes) against a target budget (e.g. 12ms at a 60Hz target). The deviation steers a **continuous scalar `densityBudget ∈ [0,1]`** that does not set the grid resolution directly, but a parameter for the subdivision depth per tile in the `TileControlBuffer`.

The decisive trick against the "Netflix buffering stutter": **hysteresis + inertia**. `densityBudget` moves per frame only by a small fraction (`lerp(current, target, 0.05)`) toward the value demanded by `stableTick`. Thus no visible jumps arise in the mesh, but a slow "flowing" of the node density — exactly the desired fluid behavior. The node positions themselves do not get re-rolled; instead, existing nodes get subdivided locally on densification (midpoint split) resp. merged on thinning (neighbor merge), so no popping artifacts arise from complete re-meshing.

The subdivision depth itself is computed not globally, but **per tile** (from `TileControlBuffer`): `localLevel = baseLevel(densityBudget) + gradientBoost(|∇Ω|) + backendHint`. The gradient boost ensures that regions with steep field changes (near sources with small `extent`) automatically get resolved finer than empty space — that solves the "wasted compute time in empty regions" problem structurally, not just through raw node reduction.

## 5. Assessment: hexagonal honeycomb pattern

**Mathematically:** yes, unreservedly feasible. Hex sampling is indeed information-theoretically superior (the 13% number refers to the Nyquist-optimal sampling density for isotropic 2D signals — an advantage that fits particularly well for radially symmetric fields like gravity potentials, since aliasing from radially symmetric sources gets distributed more evenly with hexagonal sampling instead of concentrating along axis directions).

**WebGPU-wise:** here theory separates from practice on several points:

- **Rasterizer use:** a hexagon is no native primitive. One must triangulate it as 4 or 6 triangles (fan around the center). Thus one loses the biggest advantage of the proposal — one then computes with triangle barycentrics again, only with more triangles per "logical cell" than with the simple quad grid (2 triangles). The net saving on vertex count is real, but smaller than the 13% suggest, because the fan triangulation produces overhead.

- **Adjacency complexity:** the biggest practical hurdle is not the rendering, but the **adaptive refinement**. With a quad/triangle grid, midpoint subdivision (1 quad → 4 quads, or 1 triangle → 4 triangles) is a pattern solved for decades and GPU-friendly (cf. terrain LOD quadtrees). For a hex grid there is **no clean recursive subdivision scheme** — hexagons cannot be decomposed into smaller hexagons of the same shape (that is geometrically impossible; a hexagon decomposes into triangles or into a mixture of smaller hexagons and triangles/rhombi at the seams). For our core need — continuous, local densification — that is a show-stopper or at least considerable extra effort in special-case logic.

- **Indexing:** hex coordinate systems (axial/cube coordinates) are elegant in CPU code, but in WGSL compute shaders (where one wants to derive the neighborhood from a linear buffer index) more cumbersome than the trivial `row*width+col` scheme of quads. Every neighborhood query (for gradient boost, merge/split) needs more branching logic.

**Verdict:** the hex grid is mathematically appealing, but for an **adaptive** mesh on weak hardware a cost-benefit failure. The performance gain (13% fewer samples at equal Nyquist quality) gets mostly consumed by triangulation overhead, while the implementation complexity for adaptive subdivision rises massively. **Recommendation: not for v1.** Instead: a regular triangle grid from quad diagonals (not the quads themselves — triangles give the rasterizer directly real barycentric interpolation without the bilinear crutch), with classic quadtree-style midpoint subdivision. That delivers 90% of the hex advantage (smoother interpolation than pure axis bilinear) at a fraction of the complexity. Hex sampling could be evaluated as a later research iteration for a **static** background grid (e.g. the far "empty space" region that never gets subdivided anyway), where the adjacency problems do not arise because nothing is ever refined there.

## 6. AMR: backend-steered densification

Yes, and the concept is already laid out in the `TileControlBuffer` (chapter 2). The Rust backend knows the sources exactly (position, `extent`, force type, even predicted motion via `velocity`) and can therefore compute **analytically** in advance where high gradients are to be expected — something the frontend would otherwise only discover through sampling (reactive instead of predictive).

Concrete mechanism: on every WebSocket update the backend sends, additionally to the `SourceBuffer`, a compact `RefinementHint` stream — e.g. a list of `(tileX, tileY, priority)` triples for tiles that contain a source with small `extent` or a high `|val|/tau` ratio (fast temporal change = potentially sharp visual edge). The frontend writes these hints via `writeBuffer` into the `TileControlBuffer`, and the stage-B subdivision logic (chapter 4) accounts for them additively to the locally measured gradient.

Important for stability: the hint is a **bias, not a dictate** — the frontend keeps the final control over the total node count (`densityBudget` cap), so a backend that accidentally sends "everything is important" does not destroy the frame rate of the weak GPU. The relation is: the backend says *where* (relative priority), the frontend decides *how much* (absolute budget, `stableTick`-steered). This separation makes the system resilient against a "greedy" server and still lets it profit from domain-specific knowledge that purely image-based heuristics (gradient-based refinement alone) would not have — e.g. a source that just spawned and has not yet produced a visible gradient, but according to `velocity` will soon enter a region.

## 7. Summary of the core decisions

| Aspect | Decision | Reason |
|---|---|---|
| N×M evaluation | compute shader on mesh nodes, not fragment shader | reduces operations by a factor of ~50-400x |
| Interpolation | triangle barycentrics via hardware rasterizer | free fixed-function instead of manual bilinear |
| Mesh topology | regular triangle grid with quadtree subdivision | adjacency trivial, subdivision GPU-proven |
| Scaling | continuous `densityBudget` scalar with lerp inertia | prevents popping, real "flowing" |
| Hex grid | discarded for v1 | subdivision geometrically unsolved, effort > benefit |
| AMR | backend delivers priority hints, frontend keeps budget authority | use domain knowledge without giving up control |

#####################################################################################################

GLM 5.2 Chat

Here is the architecture concept for the "agnostic membrane", optimized for the extreme constraints of an Intel HD 515 GPU (Skylake architecture, extremely limited ALUs, texture bandwidth, and register file).

---

# Architecture concept: the agnostic membrane (WGSL)

## 1. Base paradigm: measurement vs. presentation (decoupling)
The most fatal mistake on weak hardware would be to couple physics (measurement) and pixel output (presentation) in the fragment shader. We separate this rigorously into a **compute pass (measurement)** and a **fragment pass (interpolation & tonemap)**.

The image does not get computed per pixel, but on a variable measurement net. The gaps between the measurement points get filled by native hardware interpolation in the fragment shader. Since the Intel HD 515 can do linear filtering in hardware extremely fast (free compared to ALU operations), we use that as the main driver.

## 2. Buffer & memory strategy
We avoid UBOs for the sources, since these have size limits and overflow at 200 sources with many floats. We use read-only-hardware (RoH) buffers:
*   `Src_Buffer` (storage buffer, read-only): carries the filtered sources (position, val, t, tau, etc.). Format: array of structs, strided for alignment.
*   `Field_Target` (texture2D, RGBA16Float): the render target for the compute shader. Stores the 9 force channels compressed (e.g. in 3 RGBA textures or an array-texture layer). Important: *this texture must be operated with `linear` sampling*.

## 3. Continuous scaling (fluid mesh scaling)
A hard raster stutters when scaling, because it requires topology changes. The solution for weak GPUs is **progressive blue-noise accumulation** (temporal refinement):
We keep a *fixed* maximum size of the compute grid (e.g. 256x256 points). In every frame the backend/WASM computes a fill level $K \in [0.0, 1.0]$ based on `stableTick`.
*   When the frame time is bad (e.g. 30ms), $K$ sinks to e.g. 0.2.
*   The compute shader gets dispatched with only `ceil(256 * K)` workgroups per dimension.
*   **The trick of continuity:** we use no quadratic sweeping, but a Halton or blue-noise sequence. The compute shader reads from a static look-up texture (blue noise) *which* grid points should be computed in this frame.
*   Before the compute pass we copy the `Field_Target` of the last frame into a `History_Target` (ping-pong). The compute shader writes only the $K$% new values into `Field_Target`; all other pixels keep the previous value.
*   **Result:** under bad performance the image does not clump up; instead the measurement points "trickle" like rain over the screen and smooth themselves temporally. No topology change, no stutter, continuously adjustable.

## 4. Compute shader vs. fragment shader
**Compute shader (the measurement):**
*   Reads the list of active points for this frame (via blue-noise LUT).
*   For every active point: iterates over the `Src_Buffer` array (max. 200).
*   Computes the 9 force channels (gravity, etc.) for every source. We do not branch (`switch` on `kernel_id`) per pixel, but group sources ideally in the backend, or accept light divergence in the workgroup.
*   Writes the result (superposition of the forces) into the `Field_Target`.

**Fragment shader (the membrane):**
*   Renders a fullscreen quad.
*   Reads from `Field_Target` via `textureSample` (linear interpolation).
*   The 9 channels get added to $\Omega$.
*   Applies the Nebra tonemap: `log2(abs(omega) / max(lvl, 2e-64)) / 8.0 + 0.5`.
*   Outputs the luminance.

## 5. Evaluation of the hexagon idea
**Verdict: feasible WebGPU-wise, but mathematically a nightmare on the Intel HD 515.**
*   *Theory:* hexagonal sampling is 13% more efficient, since it is the optimal 2D sphere packing (isotropic frequency response).
*   *Practice in WGSL:* WebGPU (and the Intel drivers) offer native hardware filtering (bilinear) **only for regular square grids**. If you used a hex raster, you would have to search the 3 nearest neighbors in the fragment shader and perform barycentric interpolation by hand (ALU).
*   *The problem:* a manual barycentric interpolation costs about 6-8 texture fetches and divisions per pixel. At 2 million pixels on an Intel HD 515 the GPU burns through. The 13% saving in compute gets more than annihilated by the 300% higher cost in fragment rendering.
*   *Conclusion:* discard. Use a square grid. WebGPU's hardware bilinear interpolation is the absolute bottleneck saver on low-end hardware.

## 6. Adaptive mesh refinement (AMR) through the backend
That is not only possible, but the absolutely correct path for the future of the pipeline. The backend knows where gravity falls off steeply (high gradients, near sources).
*   **Concept:** the backend sends, next to the sources, a 1D array of "regions" (bounding boxes) that require a high measurement density.
*   **Implementation in WebGPU:** instead of changing the resolution of the main texture (impossible without re-allocation), we use a **hierarchical texture system (clipmaps)**.
    *   Layer 0: 128x128 for the global membrane.
    *   Layer 1..N: 64x64 textures, computed at high resolution where the backend demands it.
*   In the compute shader we check whether the current pixel falls into a backend ROI. If yes, it gets written into the finer texture (and an offset table).
*   In the fragment shader, first the coarse base texture gets sampled, then checked whether a finer ROI texture exists at this UV position. If yes, its value gets used (resp. blended via lerp).
*   This prevents the "Netflix buffering" and uses the compute time exclusively where the Nebra tonemap would otherwise alias (flicker) through steep gradients.

## Summary of the pipeline
1. **WS update:** Rust sends sources (array) and optional AMR boxes.
2. **CPU/WASM:** reads `stableTick`, computes $K$ (density 0..1).
3. **Compute pass 1:** dispatches $K$% of the points per blue-noise distribution onto the 128x128 grid. Writes into `Field_Target`.
4. **Fragment pass:** fullscreen quad, samples `Field_Target` (bilinear), applies the Nebra tonemap. (Later extendable with AMR sampling.)

#####################################################################################################

Claude Sonnet 5 High Chat 

# Architecture concept: the agnostic membrane
### Continuous 4D field evaluation on weak hardware (Intel HD 515)

---

## 1. Base principle: decoupling physics and pixels

The core problem is a confusion of two independent resolutions: the **physical resolution** (how finely must I sample the field to reconstruct it correctly?) and the **display resolution** (how many pixels has the screen?). The per-pixel shader forces both onto the same value — that is the O(N·M) suicide.

The solution is a strict **two-pass pipeline** with a third, invisible domain in between:

```
Compute pass (physics)  →  Node buffer (Ω field)  →  Fragment pass (optics)
   O(N × G)                  intermediate store         O(pixel), no more N
```

`G` is the number of measurement nodes (typically 64² to 128², *not* 2 Mio.). Thus the physics load sinks from ~400 Mio. operations/frame (2M px × 200 sources) to ~3–16 Mio. — a factor of 25–130×, exactly in the range an HD 515 can handle. The fragment shader **never** sees the sources; it only knows the already superposed Ω field and interpolates.

---

## 2. Buffer layout

| Buffer | Type | Content | Update frequency |
|---|---|---|---|
| `SourceBuffer` | Storage buffer (read-only, compute) | Array `Source[≤200]`, 16-byte-aligned packing (pos, val, t, tau, extent, kernel_id, force_type, velocity, pole, J2, J4, r_eq) | per WS snapshot, double-buffered (ping-pong), so the compute pass never reads a half-written frame |
| `SimParams` | Uniform buffer | time, stableTick (EMA), Q (quality factor), viewport, camera transform, Nebra tonemap parameters (`lvl`) | per frame |
| `FieldTexture` | `texture_storage_2d<rgba16float>` with **double usage flag** (STORAGE_BINDING + TEXTURE_BINDING) | Ω in the R channel, optional gradient/age in G/B/A | per compute dispatch |
| `PriorityBuffer` | Storage buffer, r32float, fixed maximum size | refresh priority per node (see 3.2) | accumulating, persistent over frames |

Important: `FieldTexture` gets allocated **once** in maximum size and never recreated. A re-allocation (`createTexture`) is a stall candidate on Intel iGPUs — that would be the actual reason for the "Netflix buffering effect" you observe, not the bilinearity itself.

`rgba16float` instead of `r32float`, because 32-bit float textures in WebGPU are linearly sampleable only with the optional feature `float32-filterable` — not guaranteed on the Intel HD 515. `rgba16float` is filterable per core spec and delivers hardware-accelerated bilinear nearly free (one sampler tap instead of manual interpolation logic in the fragment shader).

---

## 3. Compute pass: sampling instead of raster

### 3.1 Base evaluation
One thread per node, not per pixel. Every thread iterates over `SourceBuffer`, evaluates the 9 force channels per `kernel_id`/`force_type`, and sums additively to Ω. That is structurally identical to the hard 128×128 grid you already have — the difference lies in section 3.2/3.3.

### 3.2 Continuous scaling — the "fluid" net
Instead of changing the grid *resolution* discretely (128→96→64, which inevitably pops), the topology stays fixed at maximum size. What varies instead is **how often** every node gets recomputed — a priority scheduler:

- Every node gets a priority from two terms: proximity to a source (derived from its `extent` field — that is already your AMR signal, see 5.) and volatility (`tau`, `velocity` — fast-moving/short-lived sources force more frequent updates).
- The global quality factor `Q ∈ [0,1]` gets computed from `stableTick` via EMA + hysteresis controller (Schmitt trigger, no bang-bang): Q sinks only when stableTick lies constantly over budget for e.g. 250 ms; rises only when constantly below for over 500 ms.
- Per frame, the `round(Q · G_max)` nodes with the highest priority get updated; all others keep their last value (temporal caching). Since the field develops physically smoothly (τ-steered), a value 1–3 frames old is visually irrelevant in calm regions.
- For non-integer budgets one uses stochastic scheduling: a node with a fractional refresh interval `K` gets updated with probability `1/K` per frame (hash from node ID + frame counter). That produces a real continuous expectation value instead of integer steps — "continuous" in the literal sense.

Thus there are **no** resolution jumps, no re-tessellation, no buffer reallocation — only a continuously breathing update rate on fixed geometry.

### 3.3 Near-field correction (optional, for hard singularities)
Gravity without softening (`1/max(d²,1)`) can vary more strongly near the camera than any grid can resolve. Recommendation: add sources within a small screen-space radius around the camera separately, exactly per pixel (typically 0–3 sources, no N-problem), while the grid delivers the smooth far-field part of all remaining sources. Splitting procedure, no grid refinement needed.

---

## 4. Fragment pass: interpolation

Full screen quad, one `textureSampleLevel` with a linear sampler on `FieldTexture`. That is all — the bilinearity is a hardware feature, costing practically nothing on Intel iGPUs. Afterwards apply the Nebra tonemap directly onto the sampled Ω value. No loop, no source reference in the fragment shader anymore.

The "gaps"/"lies" between the support points that you observe with the hard 128 grid are not an interpolation problem, but a **sampling problem**: bilinear can only be as good as the node density. Solve 3.2/3.3, not section 4.

---

## 5. Assessment: hexagonal honeycomb pattern

The 13% efficiency statement is correct — the hexagonal grid is the information-theoretically optimal 2D quantizer (lowest reconstruction error per sample at a given density). Purely computationally: barycentric interpolation from three support points is O(1) and no performance problem for 2 Mio. pixels — the axial hex coordinate transformation (pixel → (q,r) → next triangle → barycentrics) is ~20–30 ALU ops, which is irrelevant against the actual bottleneck (the N-iteration, which we have already eliminated).

**The problem is not performance, but compatibility with section 3.2.** A square grid subdivides cleanly (1 quad → 4 quads), the standard case for adaptive/LOD meshing since the terrain engines of the 2000s. A hexagon does *not* subdivide cleanly — at resolution boundaries, pentagon defects or T-junction-like seams inevitably arise, a non-trivial meshing problem known for decades. But exactly that is what you need for the continuous, location-dependent density of section 3.2/5(AMR).

Conclusion: hex is **no nightmare in WGSL** (the fragment-shader mathematics is feasible and cheap), but it is a **bad cost-benefit ratio for this project**: 13% constant factor against an already present 25–130× through the decoupling — and it complicates exactly the requirement (continuous adaptive density) that matters most. Recommendation: **v1 with quad grid**, hex as a possible v2 optimization only if the density ends up staying uniform after all (then the subdivision conflict disappears).

---

## 6. Assessment: backend-steered AMR

Camera movement/zoom is purely local at the client and may never wait for a WebSocket roundtrip — any backend specification for refinement would lag behind the user input. Client-side refinement (section 3.2) must therefore keep the authority for the frame-to-frame decision.

However: the protocol already delivers an implicit AMR signal with `extent` and `tau` — no new fields needed. Sensible would be a **rare, coarse** backend specification (not per frame): on larger scene changes (many new, tightly clustered sources) send along a compact hotspot hint that gives the priority scheduler a cold-start bias, instead of it having to settle itself over 1–2 seconds. So: backend as **slow prior**, client as **fast controller** — not the other way around.

---

## 7. Summary

| Requirement | Solution |
|---|---|
| Weak GPU, no O(N·M) | 2-pass pipeline: compute on G nodes, fragment only interpolation |
| Gapless image | hardware bilinear via `rgba16float` texture, near-field splitting at singularities |
| Continuous scaling | fixed topology, continuous priority/stochastic refresh scheduler instead of resolution steps |
| Hexagon honeycomb | mathematically/technically feasible, but conflicts with AMR — not recommended for v1 |
| Backend AMR | `extent`/`tau` suffice as signal; backend only as rare cold-start hint, never as frame authority |
