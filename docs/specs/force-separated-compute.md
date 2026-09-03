<!--
  title: force-separated-compute
  class: concept
  sha256: 28da3e79b4050d2285ef763bedd920a5f471bfa232e19434cf9b9066828a0f5f
-->
STATUS: DEPLOYED

Here are the physical and architectural explanations of the individual points of the deployment document. They serve as the theoretical foundation so that the implementation is not only code, but pure manifestation of the omegaflow axiom (*A = A*).

---

### 1. The causality pre-filter (Rust / `main.rs`)

**The idea:**
So far the system has filtered oscillators purely geometrically: if a source lay within the search radius (`reach`), it was admitted to the point cloud — completely independent of whether its signal could *have* reached the presence point at all.

**The physics:**
Information propagates with a finite speed (`v_or_d`). An acoustic signal (sound) is extremely slow (343 m/s). An earthquake in Japan measured 10 minutes ago physically does not exist in Europe yet. Light from the sun takes 8 minutes.
The filter implements the exact **light cone (causality cone)** of the presence. It checks: `distance <= v_or_d * age`. If the signal is not there yet, it is discarded. For diffusive forces (heat) the reach grows with `sqrt(2 * D * age)`.
Additionally it is checked whether the signal has already "died away" (`age > tau * 64.0`).

**The architecture:**
The check runs as an `early exit` *before* the call to `smp.motion.at(t2, smp.epoch)`. That function computes Keplerian orbits and WGS84 ephemerides — the most expensive CPU operation in the Archivar. By sorting out causally inadmissible samples beforehand with simple multiplications, we save massive CPU cycles. What does not lie in the causality cone does not exist for the silicon.

---

### 2. Auto-frame from `lat_key` / `lon_key` (Rust / `main.rs`)

**The idea:**
When an API (e.g. aircraft, earthquakes) delivers its coordinates dynamically in the JSON, the parser uses the directives `lat_key` and `lon_key` to define the column names. So far the parser refused the source because it thought the reference frame (`Frame`) was missing.

**The architecture:**
The axiom reads: *"Silicon knows IO."* When `lat_key` is defined, the information is there. The system does not need to ask "who" delivers the data; it suffices that the property (the coordinate) exists. The patch `|| !cur_lat_str.is_empty()` instructs the Archivar to set the frame to `Data` automatically when coordinate keys are present. The system becomes more agnostic and more robust toward new data sources.

---

### 3. Switch from the fragment shader to the vertex shader (frontend / `index.html`)

**The idea:**
The old rendering computed the field for *every single screen pixel* in the fragment shader (a full-screen quad). That is a "brute-forced global raster" — a direct violation of the omegaflow architecture, heating up the GPU unnecessarily across millions of pixels.

**The architecture:**
We now manifest **pure oscillators**. The vertex shader takes the raw 3D points from the Archivar, projects them onto the 2D presence surface and generates a small 2D quad per point. The GPU computes geometry only where data actually exists. Gaps in the image mean simply: *Here physically nothing exists.* That is the absolute maxim of *A = A* and enables millions of points at 60 FPS with minimal GPU load.

---

### 4. Additive blending & analog glow (frontend / `index.html`)

**The idea:**
A hard, pixelated point cloud looks like a cold digital simulation. To manifest the organic, "analog" behavior of a real oscilloscope or phosphor glow, we need to honor the physical properties of the silicon.

**The architecture:**
1. **Exponential falloff:** The fragment shader discards hard edges (`discard` outside the radius) and instead uses `exp(-dist * dist * 4.0)`. The field falls off softly, exactly like the real physical `omega` law.
2. **Additive blending:** Overlapping points add their colors (`blend: additive`). This corresponds to the physical **superposition** of realities. Where many oscillators are, it becomes bright.
3. **Analog noise (dithering):** A light, position-specific noise (`fract(sin(...))`) breaks the strict 8-bit quantization of the digital framebuffer. This prevents "banding" (streaky color gradients) and makes the field look to the human eye like an organic, analog glow.

---

### 5. Adapting the draw calls (`pass.draw(n * 6)`)

**The idea:**
The GPU needs to know how many triangles to draw.

**The architecture:**
Since we now draw a 2D quad per oscillator (consisting of exactly 2 triangles = 6 vertices), the draw call changes. Earlier it was `pass.draw(3)` (one large triangle for the whole screen). Now it is `pass.draw(n * 6)`, where `n` is the number of oscillators received from the Archivar. When the presence window is empty (`n = 0`), the GPU draws absolutely nothing (`pass.draw(0)`). That is the ultimate performance economy: the silicon consumes zero energy for idle time.
