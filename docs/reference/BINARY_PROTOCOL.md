# Binary Protocol — v8

`src/main.rs` (resonance response construction, `pack_window`), `static/constants.js` (parse), `static/index.html` (WGSL + WS onmessage), `src/main.rs` `MEMBRANE_WGSL` (native membrane).

## WebSocket Response Frame

| Offset | Size | Value |
|--------|------|-------|
| 0 | 2 | `0xCF 0x86` (magic) |
| 2 | 1 | `0x08` (protocol version) |
| 3 | 8 | `response_epoch` f64 LE — TDB seconds since J2000 of the response |
| 11 | 4 | `id` u32 LE — presence window identifier (echoed from query) |
| 15 | 4 | `count` u32 LE — number of oscillator records |
| 19 | count × 192 | oscillator records |

Header total: 19 bytes.

## Oscillator Record (192 bytes, 24 × f64 LE)

| Slot | Field | Meaning |
|------|-------|---------|
| 0 | x | ICRS X relative to presence (meters, f32-folded upstream) |
| 1 | y | ICRS Y relative to presence (meters) |
| 2 | z | ICRS Z relative to presence (meters) |
| 3 | val | field value in SI units (e.g. GM in m³/s² for mass channels) |
| 4 | epoch | observation epoch (TDB seconds since J2000) |
| 5 | ttl | time-to-live (seconds) |
| 6 | tau | decay constant (seconds; ∞ for mass/radius channels) |
| 7 | extent | spatial extent (meters; ∞ for kernel 0/6 channels) |
| 8 | kernel_id | kernel shape ID (0–6) |
| 9 | force_type | force type ID (0–8) |
| 10 | absorption | absorption coefficient (0–1) |
| 11 | advection | advective propagation speed (m/s, 0 = absent) |
| 12 | vx | oscillator velocity X (m/s) |
| 13 | vy | oscillator velocity Y (m/s) |
| 14 | vz | oscillator velocity Z (m/s) |
| 15 | pole_x | body pole axis X (ICRS unit vector, 0 = absent); for em sources (force_type 0) carries the redshift z — packed into `meta[3]` (`props[j*4].w`) and applied as Tolman dimming (1+z)⁻⁴ |
| 16 | pole_y | body pole axis Y |
| 17 | pole_z | body pole axis Z |
| 18 | j2 | zonal harmonic J2 (0 = absent) |
| 19 | j4 | zonal harmonic J4 (0 = absent) |
| 20 | r_eq | equatorial radius (meters, 0 = absent) |
| 21 | color_index | unified BP−RP color (0 = absent → white, 0 honored) |
| 22 | freq | band center (Hz, linear). 0.0 = point source — the one-bin limit; absent frequency is a fully realized property, never fabricated |
| 23 | bin_width | band width (Hz, linear). 0.0 = point source |

Absent properties are written as 0.0 — the neutral constant of the fixed-stride record.

## WebSocket Query Frame (browser → server)

`static/constants.js` `syncFrame`:

| Offset | Size | Value |
|--------|------|-------|
| 0 | 4 | `id` u32 LE |
| 4 | 4 | `input_count` u32 LE |
| 8 | inputs | station samples (17 bytes + UTF-8 name each) |
| … | 4 | `query_count` u32 LE |
| … | query_count × 32 | queries: t, x, y, z f64 LE each |
| … | 48 | presence: x, y, z, t, range, cache_interval — 6 × f64 LE |

No magic/version bytes in the query frame.

## JavaScript → GPU repacking

`static/constants.js` (parse of the 192-byte record):

- `field`: Float32Array(oscCount × 12) = `[x_rel, y_rel, z_rel, val, t, ttl, force_type, absorption, advection, vx, vy, vz]`
- `meta`: Float32Array(oscCount × 16) = `[extent, tau, kernel_id, z|0, pole_x, pole_y, pole_z, j2, j4, r_eq, color_index, freq, bin_width, 0, 0, 0]`

`meta[3]` carries the Tolman redshift z when force_type == 0 (from slot 15), else 0.0 — identical on the Rust side (`pack_window`) and the JS side. `meta[13..15]` are stride padding.

Version checks (both must be `8`): `constants.js` record parse (`bytes[2] !== 8 → throw`) and `index.html` WS onmessage (`buf[2] !== 8 → return`). No legacy read mode — both sides grow in the same commit.

## WGSL unpacking

`static/index.html` fieldShader and `src/main.rs` `MEMBRANE_WGSL` (identical access):

```
field[j*3]    = vec4f(x, y, z, val)
field[j*3+1]  = vec4f(t, ttl, force_type, absorption)
field[j*3+2]  = vec4f(advection, vx, vy, vz)
props[j*4]    = vec4f(extent, tau, kernel_id, z|0)
props[j*4+1]  = vec4f(pole_x, pole_y, pole_z, j2)
props[j*4+2]  = vec4f(j4, r_eq, color_index, freq)
props[j*4+3]  = vec4f(bin_width, 0, 0, 0)
```

`force_type` read as `u32(tm.z)`, `absorption` as `f32(tm.w)`, `advection` as `fm.x`, `kernel_id` as `u32(mt.z)`, `z` as `mt.w` (Tolman dimming), pole/j2/j4/r_eq from `mp`/`mg` (zonal harmonic term in `osc_field`), `freq` as `mg.w`, `bin_width` as `props[j*4+3].x`. The freq/bin_width slots are carried by the record and the pack since v8; band-selective rendering consumes them in the spectral oscillator atoms (see `docs/concepts/DER_SPEKTRALE_OSZILLATOR.md`).

Slot identity is verified by `golden_pack_slots_against_wgsl_access` (mathematikerin tests) — the pack layout test against the WGSL access pattern. The WGSL sources validate offline via naga (`membrane_wgsl_validates_offline`).
