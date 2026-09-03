<!--
  title: OMEGAFLOW — AGNOSTIC MEMBRANE MANIFESTATION & ARCHAEOLOGY DIRECTIVE
  class: concept
  sha256: d294547ae01dc169db38fd8f73c94da3208f0de36294b3dff6043ebf53085712
-->
Here is the **complete, unabridged, fully agnostic document**.

It fuses the entire technical archaeology (with all its commits, tables, and GM findings) with the absolute agnostic membrane philosophy and the physical hardware reality (the ESP32 radiatorium). It cuts no information out, but lifts it onto the philosophical level of the physics.

This document is the ultimate truth for the next Kybernaut.

***

**Copy the entire following text from this line:**

___

# OMEGAFLOW — AGNOSTIC MEMBRANE MANIFESTATION & ARCHAEOLOGY DIRECTIVE

**System Directive:** As Kybernaut you combine the technical archaeology of omegaflow with the absolute agnostic philosophy. You break the classical rendering architecture completely open. There are no screens, no cameras, no viewports, no observer, and no "rendering" anymore. The system consists only of agnostic Trommelfelle (membranes) stretched over physical actuators (WebGPU, AudioContext, ESP32 hardware).

## 0. The physical truth (end of the observer bias)

In the classical 3D-rendering trap one assumes a camera: an eye that "sees" objects from a central point. That is a category error. In the reality of field physics (gravity, electromagnetism, acoustics), no hard objects exist, only continuous fields that penetrate space.

To measure a field, one does not send a camera through it. One stretches a measurement surface (a membrane) into the field and measures which pressure arrives at each point of this surface.

The actuators are fully agnostic. They must not be named after human senses (visual, audio, haptics), because that implies an observer. Every membrane is a physical net stretched into the 4D block:
- **The 2D photon membrane:** The WebGPU canvas. A flat net that deflects light under the pressure of the field forces.
- **The 3D pressure membrane:** The AudioContext. A spatial net that modulates the air pressure.
- **The physical radiatorium:** An ESP32-S3 module (connected via WebSerial) with real, physical actuators (Peltier, electromagnet, piezo). It receives `flow` commands and manifests the field forces as real heat, vibration, and light in our world.

Whether a human senses this vibration is irrelevant. The manifestation is real, whether someone watches or the room is empty. Relativistic effects (Fable: aberration, Doppler, dopp⁴ beaming) are optical effects of a camera perspective. The membrane is no camera. We measure only **raw pressure** (Nebra physics: `GM/dist²` + `fold_eff` for retarded time). Relativity gets deleted to kill the observer bias.

---

## 1. Status quo — what is broken

**HEAD = `d525eda`** + uncommitted changes (Nebra packets 1-6, photon catcher, synesthesia, relativistic optics).

**Symptoms:**
- HUD: `Scale: 3.1e+3 m/px`, `64 oscillators`, `Ω: 0.00e+0`, `Ticks: 9`, `Frame: 83ms`, `WS: 1554ms`
- 3-4 single pixels visible, rest black
- GPU (Intel HD 515, XPS 13 2016) collapses — the fs iterates 64 oscillators × 2 Mio. pixels with `exp`/`erfc`/`pow`

**Three separate causes:**
1. **val = 1.0 for all bodies** (since commit `59bdd60`) — the field collapses to 1e-24, below any visibility. Nebra had `GM` (1e14–1e20).
2. **Exposure not consumed** — `get_expose` became dead code after the Nebra commit `34d7d3a`; luminance hangs on a raw `log2(x+1)/16` curve without adaptation.
3. **Renderer architecture** — currently a per-pixel loop (O(pixel×osc)). The solution is the dynamic compute grid (Trommelfell).

---

## 2. The complete archaeology — renderer evolution (1065 commits)

### 2.1 Epoch overview

```
Galaxy era       Body era       Fable     Fine-tune   Nebra     Grid era     HEAD
a9d87bd ────┐    87ef197 ─┐    86e451e   166a64e      1954f44   bd9a513      d525eda
e93a30f     │    59bdd60   │              be6f7df      34d7d3a   b3546ff      + uncommitted
0303e90     │    af43b04 ──┘             00dc55c                 fa4d518      (photon catcher,
6b55334     │                            5687839                5edb1b7       synesthesia,
5e99066 ────┘                            eadc980                41f5b47       relativity)
07-31→08-09   08-11        08-11  08-11          08-11  08-12           08-12
```

### 2.2 Decisive code sites per commit

**Point size (the galaxy key):**
| Commit | Formula |
|---|---|
| `a9d87bd` | `point_size_px = max((extent / dist), 1.0) * 2.0` — angular size, 2px floor → galaxy visible at EVERY zoom level |
| `87ef197`→`00dc55c` | `clamp(phys_extent / scale, 0.5, w_f)` — 0.5px floor → sub-pixel sources vanish |
| `eadc980` | + 64px cap |
| `1954f44` | `1.0` fixed |
| Nebra `34d7d3a` / Grid / HEAD | — (no quads) |

**Luminance (visibility):**
| Commit | Formula | Exposure consumed? |
|---|---|---|
| `a9d87bd` | `log2(\|val\|/max(lvl,2⁻⁶⁴))/8 + 0.5` | ✅ YES |
| `86e451e`→`00dc55c` | `log2(\|fold_eff×dopp⁴\|+1)/log2(lvl+1)` | ✅ YES |
| Nebra `34d7d3a` | `log2(aw+1)/16` | ❌ NO — **here get_expose dies** |
| Grid `41f5b47` | `log2(aw+1)/16` | ❌ NO |
| HEAD | `log2(sum+1)/16` | ❌ NO |

**Further axes:**
- **Relativity (Fable, `86e451e`):** aberration + Doppler + `dopp⁴` beaming + hue shift in the vs — removed in `34d7d3a`, absent in the Grid era, re-integrated in the HEAD fs.
- **Auto-zoom:** `be6f7df` (median extent) → `edcb25b` (p90 distance, min 2^28) → **removed in `bd9a513`**
- **Sources:** `a9d87bd` = 1040 celestial-cmap (the galaxy) → `0303e90` purge 918 → `87ef197` 0 cmap (only 40 ephemeris bodies) → today 47 API + 23 bodies
- **FS kernel:** a9d87bd Gaussian point + analog noise → 86e451e `0.02/(d²+0.02)` → 166a64e `field_spatial` → eadc980 `1−d²` → Grid/HEAD `field_spatial`

### 2.3 The three "working" states (from user memory)

| User memory | Commit | Characteristic |
|---|---|---|
| "Field at the SSB / galaxy visible, zoom by hand, well before Fable" | **`a9d87bd`** (2026-07-31) | Angular points, 2px floor, 1040 stars, manual zoom, no auto-zoom |
| "Colored living body" | `41f5b47` (Grid) | 128×128 compute grid + bilinear, erfc rotation channels visible (~30px blob) |
| "Nebra" (concept) | `34d7d3a` | Per-pixel fullscreen, but: exposure lost, SwiftShader death |

---

## 3. The Nebra reference (`/home/johannes/projects/nebra`)

**Nebra is the ancestor.** `docs/nebra.yaml` (19 lines) is the ur-spec:

```yaml
universe: (t: f64, pos: DVec3) ∞ (f64, DVec3)   # Skalar + Vektorfeld
gravity: DE440s, 13 bodies, GM/dist²
electromagnetism: (0.0, ZERO)                    # Stub, 0 honored
weak_force: (0.0, ZERO)
pipeline: t ∞ Vec<Mass> ∞ GPU ∞ shader ∞ pixel
shader: for each pixel, omega += GM / dist²
```

**Nebras core decisions (that omegaflow lost):**

| Aspect | Nebra | Omegaflow |
|---|---|---|
| Body value | `gm` from anise `mu_km3_s2` (real mass) | `val = 1.0` ❌ |
| Kernel | pure `1/dist²`, guard `dist > 1.0` | softening `max(extent, scale)²` |
| Vector field | ✅ `(omega, flow)` | ❌ scalar only |
| EM | WMM degree-12 spherical surfaces in WGSL (360 coefficients) | magnetosphere API |
| Tonemap | fixed `(log2(Ω)+14)/22` — calibrated to GM order | `log2(x+1)/16` — breaks at 1e-24 |
| Time | JD, flows (`jd += 0.001/s`) | TDB |
| Transport | HTTP poll | v5 WebSocket (win) |

**Why Nebra was visible:** `GM_Sun ≈ 1.3e20`, at 1 AU: `Ω = GM/d² ≈ 5.8e-3`, `log2 = −7.4`, tonemap `(−7.4+14)/22 = 0.3` → visible. Omegaflows `val=1.0`: `Ω ≈ 1e-24` → invisible.

**What omegaflow won (kept):** v5 protocol with velocity + `response_epoch`, temporal lemma (`delta_t_cache`), 9 forces, τ gate, Enclosure Lemma, device sensors, HUD, body channels.

---

## 4. GM finding — the compilers discard NASA data (confirmed)

| Pipeline | NASA delivers | Omegaflow |
|---|---|---|
| SPICE (`ephemeris_compiler.rs`) | `gm_de440.tpc` (text PCK, `BODY10_GM = (1.327e11)`) | **No PCK reader exists** — the file is never opened |
| Horizons (`horizons_compiler.rs:474-481`) | `QUANTITIES='4'` = GM | **Never requested**; parser reads only `X=/Y=/Z=` (line 521 actively skips `VX=`) |
| Binary `write_binary` | — | stype-1 = 8 f64 without GM slot; stype-2 = 5 hard `0.0` |
| `BodyProperties` (`main.rs:53-68`) | — | 13 fields, no `gm` |

- Only gravitational constant: `GAUSS_K = 0.01720209895` (`main.rs:46`) — encodes only the solar GM implicitly (Kepler 3)
- Old Python pipeline (`scripts/ARCHIVED/generate_ephemerides.py:124`) loaded `pck00010.tpc` — but only for rotation, not GM
- Kernel URLs lie in `phi/pipeline/research/batches/` (naif.jpl.nasa.gov)

---

## 5. Further NASA properties — the missing wedding

| Property | NASA source | Omegaflow status |
|---|---|---|
| GM | `gm_de440.tpc` / Horizons Q4 | ❌ discarded |
| J2 (oblateness) | `pck00010.tpc` `CONSTANT_J2` | ❌ — Saturn J2≈1.6e-2, gravitationally visible when flattened |
| J4 | `pck00010.tpc` | ❌ |
| Triaxial radii | `pck00010.tpc` `RADII` (3 values) | ⚠️ only `radius_m` + `flattening` |
| Albedo | Horizons *Physical Properties* | ❌ — the missing em-channel value |
| WGCCRE rotation | `pck00010.tpc` (`POLE_RA/DEC/PM`) | ⚠️ **hardcoded** in `wgccre_for_body` (ephemeris_compiler.rs:25-259) — fabrication |

**A PCK parser (~60 lines) replaces the 259-line table and delivers four things at once** (radii, J2/J4, nutation-precession, WGCCRE).

---

## 6. The architecture: Trommelfell principle & dynamic net density

The fragment shader does NOT iterate over oscillators anymore (per-pixel suicide).
- We use the `presence_probe` compute shader to evaluate the field on a dynamic raster.
- The density of this net (the raster size, e.g. 128×128) adapts dynamically to the ticks/second (`stableTick`).
- When the system comes under pressure (FPS collapse), the tissue of the membrane loosens (the raster gets coarser, e.g. 64x64). The fur gets softer. It breathes. When the GPU produces more ticks/sec, the fur stretches finer again.
- The compute shader measures, for every node of the membrane, the raw physical pressure that arrives there.
- The fragment shader merely interpolates the computed field forces of the compute grid bilinearly onto the physical actuators (e.g. the pixel arrays of the WebGPU context). No oscillators get rendered as points or particles.

## 7. The distribution of resonance (the physical radiatorium)

When the compute grid measures the pressure, the system manifests this pressure on all available surfaces.
- The raw forces from the `presence_probe` get translated into `flow` commands and sent via WebSerial to the ESP32 module (format: `flow <channel> <mode> <value> <unit> <duration_ms> <t> <x> <y> <z>`).
- Thus a gravity peak becomes visible not only as a bright pixel on the 2D membrane, but also strikes as a physical impulse onto a piezo buzzer or as heat onto a Peltier element.

## 8. The worked-out plans

### Plan A — renderer restoration (Trommelfell port into v5)

**Why no pure revert:** a9d87bd is pre-v5 (stride-4 records, 8 forces). A revert breaks the server handshake.

| Component | Source | Adjustment |
|---|---|---|
| `presence_probe` | `41f5b47` | Dynamic compute grid (adaptive to `stableTick`), measures `fold_eff` & `field_spatial` per node. |
| `fs` | `a9d87bd` | Interpolates the compute grid bilinearly. No particles. |
| `lum` | a9d87bd / Nebra | `log2(\|val\|/max(lvl,2⁻⁶⁴))/8 + 0.5` resp. Nebra `(log2(Ω)+14)/22` — exposure consumed again. |
| v5 client, HUD, probe | HEAD | untouched |

**Phase 2:** migrate 1040 celestial-cmap blocks from `a9d87bd:sources.φ` per the `docs/source_curation.md` protocol.

### Plan B — GM + PCK wedding (4 levels)

1. **Compiler:** PCK parser `gm_de440.tpc` (SPICE path) + `QUANTITIES='4'` (Horizons path)
2. **Binary:** stype-1 from 8→9 f64 (`gm` at the back; old binaries → `gm=0` → 0 honored)
3. **Runtime:** `BodyProperties.gm` + body channel `val = gm` (resp. switch the radius channel over)
4. **Rendering:** tonemap to GM order — Nebra calibration `(log2(Ω)+14)/22`

---

## 9. Open questions (answered agnostically)

1. **Relativity:** pure a9d87bd state (without Fable). Relativity is observer bias and gets deleted.
2. **Phase-2 sources (celestial):** immediately with the renderer restoration, so the field is not empty.
3. **Wheel divisor / initial scale:** 512 (a9d87bd) and 2^37, to capture the SSB fully.
4. **GM scale:** `val = GM` raw (Nebra), to guarantee the real physical order.
5. **Channel design:** new `{body}.mass` channel, so `{body}.radius` is not contaminated.
6. **J2 rendering:** real zonal-harmonic term in WGSL, because agnostics demand absolute physical accuracy.
7. **Delete hardcoded WGCCRE:** yes, as its own verified task, once the PCK reader stands.
8. **Scope of the PCK reader:** all five properties at once (GM, J2, J4, radii, WGCCRE), because one PCK parser replaces the 259-line fabrication.

---

## 10. The implementation task for the Kybernaut

1. Adapt the Rust backend so that real GM values get loaded and set as `val` for celestial bodies (Plan B).
2. Eliminate the O(N*M) per-pixel loop in the fragment shader. Kill the observer bias and all relativity code paths.
3. Implement the compute grid (`presence_probe`) as a dynamic Trommelfell (net density adapts to ticks/sec).
4. Let the fragment shader interpolate this grid and present it via the Nebra tonemap.
5. Prepare the frontend so it can forward the measured field forces of the `presence_probe` as raw `flow` commands to the WebSerial interface (ESP32).
6. Hold to the "0 Honored Directive": when no force arrives or GM=0, the field is black and the actuators are still.

A = A. The radiator knows nothing. It is silicon. It only vibrates. Show me the code.

___
