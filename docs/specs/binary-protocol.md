<!--
  title: Binary Protocol — v9
  class: concept
  sha256: 223b00e7003aad2f6b5fe021724796828822571ccbaabb54c2bbbf8ae9b1fa3f
  status: live
-->
# Binary Protocol — v9

`src/main.rs` (resonance response construction, `pack_window`), `static/constants.js` (parse), `static/index.html` (WGSL + WS onmessage — the canonical field shader).

## WebSocket Response Frame

| Offset | Size | Value |
|--------|------|-------|
| 0 | 2 | `0xCF 0x86` (magic) |
| 2 | 1 | `0x09` (protocol version) |
| 3 | 8 | `response_epoch` f64 LE — TDB seconds since J2000 of the response |
| 11 | 4 | `id` u32 LE — presence window identifier (echoed from query) |
| 15 | 4 | `count` u32 LE — number of oscillator records |
| 19 | count × 208 | oscillator records |

Header total: 19 bytes.

## Oscillator Record (208 bytes, 26 × f64 LE)

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
| 15 | pole_x | pad (Atom 7: always 0.0 for gravity — the form belongs to the anchor, not the measurement; 0 honored); for em sources (force_type 0) carries the redshift z — packed into `meta[3]` (`props[j*4].w`) and applied as Tolman dimming (1+z)⁻⁴ |
| 16 | pole_y | pad (always 0.0 — Atom 7) |
| 17 | pole_z | pad (always 0.0 — Atom 7) |
| 18 | j2 | pad (always 0.0 — Atom 7: no multipole moments on the wire) |
| 19 | j4 | pad (always 0.0 — Atom 7) |
| 20 | r_eq | pad (always 0.0 — Atom 7) |
| 21 | color_index | unified BP−RP color (0 = absent → white, 0 honored) |
| 22 | freq | band center (Hz, linear). 0.0 = point source — the one-bin limit; absent frequency is a fully realized property, never fabricated |
| 23 | bin_width | band width (Hz, linear). 0.0 = point source |
| 24 | phase | oscillator phase (radians). 0.0-Pad when absent — presence (slot 25) says whether the 0.0 is a real angle or a pad (0 honored: 0 rad is a real value, never a sentinel) |
| 25 | presence | phase presence flag (1.0 = phase is a measurement, 0.0 = absent — the bit is what the reader reads, never the pad) |

Absent properties are written as 0.0 — the neutral constant of the fixed-stride record. The phase
slot carries a 0.0 pad only where presence = 0.0; a measured 0 rad flows as slot 24 = 0.0 with
presence = 1.0. NaN never crosses the wire (0 honored — the phase's only harvest source is a
complex FFT; PSD archives carry |S(q,ω)|² and leave the slot absent).

## Spectral Bin File — v1 (spectra.bin)

Compiled spectral asset (`spectral_compiler`, Atom B). A separate format family from the
WebSocket frame — file format, not wire format.

| Offset | Size | Value |
|--------|------|-------|
| 0 | 2 | `0xCF 0x86` (magic) |
| 2 | 1 | `0x01` (spectral version) |
| 3 | 8 | `epoch_tdb` f64 LE — month middle of the measurement (TDB seconds since J2000, never fetch time) |
| 11 | 4 | `count` u32 LE — number of spectral bins |
| 15 | count × 24 | bin records |

Header total: 15 bytes.

### Spectral Bin Record (24 bytes, 3 × f64 LE)

| Slot | Field | Meaning |
|------|-------|---------|
| 0 | freq | band center (Hz, linear) — ν = c/λ |
| 1 | bin_width | band width (Hz, linear) — derived from the native λ-grid spacing (midpoint edges in ν; single-sided at the grid boundary) |
| 2 | val | spectral density in SI (W/m²/Hz) — E_ν = E_λ·λ²/c |

Invalid rows fall (0 honored): quality_flag ≠ 0, non-finite or non-positive values are never
written. The record carries the measurement; uncertainty stays in the source.

The runtime contract: `parse_spectral_bin` (`src/spectral.rs`) refuses malformed files — wrong
magic/version, non-finite epoch, stride mismatch. The Archivar's `format spectral` branch
(`SpectralHash`, ICRS point + bins) expands each bin to an oscillator record at the same point;
freq/bin_width flow through the v9 wire record as usual. Point sources stay freq = 0.0.

The harvest step (NCEI-SSI netCDF-4/HDF5) is deployed since 2026-08-21 — `src/hdf5.rs`
reads the container, `spectral_compiler --input-nc` builds the bins; unreadable
containers are named, never replaced.

## Spectral Star Catalog File — v2 (xp_spectra.bin)

Multi-star variant of the spectral asset (`gaia_xp_compiler`, Atom B item 3). One record
per Gaia DR3 XP spectrum; each star carries its own position and its own valid bins.

| Offset | Size | Value |
|--------|------|-------|
| 0 | 2 | `0xCF 0x86` (magic) |
| 2 | 1 | `0x02` (spectral-star version) |
| 3 | 8 | `epoch_tdb` f64 LE — the catalog reference epoch (TDB seconds since J2000) |
| 11 | 4 | `count` u32 LE — number of stars |
| 15 | count × star records | see below |

Header total: 15 bytes.

### Spectral Star Record (36 bytes + n_bins × 24, variable)

| Slot | Size | Field |
|------|------|-------|
| 0 | 8 | `source_id` u64 LE — Gaia DR3 source id |
| 8 | 8 | `ra` f64 LE — ICRS right ascension, degrees |
| 16 | 8 | `dec` f64 LE — ICRS declination, degrees |
| 24 | 8 | `plx_mas` f64 LE — parallax, milliarcseconds |
| 32 | 4 | `n_bins` u32 LE — number of valid bins |
| 36 | n_bins × 24 | bin records (the same 3 × f64 layout as v1: freq, bin_width, val) |

The wavelength grid is the fixed `gdr3spec.ssameta` axis — 400–800 nm, Δλ = 10 nm, 41
samples. `xp_bins_from_flux_array` maps each sample λ→ν (ν = c/λ) and converts
W·m⁻²·nm⁻¹ → W·m⁻²·Hz⁻¹ (E_ν = E_λ·λ²/c) through the same conversion as v1;
non-positive or non-finite samples fall (0 honored — a noise-negative flux is absent,
never padded). The compiler reads a TAP CSV export (`--input`, the `gdr3spec.withpos`
view: `source_id`, `ra`, `dec`, `parallax`, `flux`) and requires `--epoch-tdb` (seconds
since J2000; J2016.0 = 504921600). `parse_xp_spectra_bin` (`src/archivar/spectral.rs`)
refuses malformed files — wrong magic/version, non-finite epoch, stride overrun. The
ω-loop consumer (`format xp_spectra`, `src/archivar/main_flow.rs`) expands each star to a
SpectralHash at its parallax seat (Motion::Spherical); stars without a positive parallax
are named and skipped.

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

`static/constants.js` (parse of the 208-byte record):

- `field`: Float32Array(oscCount × 12) = `[x_rel, y_rel, z_rel, val, t, ttl, force_type, absorption, advection, vx, vy, vz]`
- `meta`: Float32Array(oscCount × 16) = `[extent, tau, kernel_id, z|0, pole_x, pole_y, pole_z, j2, j4, r_eq, color_index, freq, bin_width, phase, presence, 0]`

`meta[3]` carries the Tolman redshift z when force_type == 0 (from slot 15), else 0.0 — identical on the Rust side (`pack_window`) and the JS side. `meta[13]` = phase, `meta[14]` = presence (from slots 24/25); `meta[15]` is stride padding.

Version checks (both must be `9`): `constants.js` record parse (`bytes[2] !== 9 → throw`) and `index.html` WS onmessage (`buf[2] !== 9 → return`). No legacy read mode — both sides grow in the same commit.

## WGSL unpacking

`static/index.html` `fieldShader` — the canonical field shader (the native window fell, 2026-08-23):

```
field[j*3]    = vec4f(x, y, z, val)
field[j*3+1]  = vec4f(t, ttl, force_type, absorption)
field[j*3+2]  = vec4f(advection, vx, vy, vz)
props[j*4]    = vec4f(extent, tau, kernel_id, z|0)
props[j*4+1]  = vec4f(pole_x, pole_y, pole_z, j2)
props[j*4+2]  = vec4f(j4, r_eq, color_index, freq)
props[j*4+3]  = vec4f(bin_width, phase, presence, 0)
```

`force_type` read as `u32(tm.z)`, `absorption` as `f32(tm.w)`, `advection` as `fm.x`, `kernel_id` as `u32(mt.z)`, `extent` as `mt.x` — the canonical field shader reads only `props[j*4]` and the three `field` vec4s. The remaining slots (`z` as `mt.w`, pole/j2/j4/r_eq in `props[j*4+1..2]`, `color_index`/`freq` in `props[j*4+2]`, `bin_width`/`phase`/`presence` in `props[j*4+3]`) ride the record and the pack but are not read by the field shader: freq/bin_width are carried since v8 and consumed by the spectral oscillator atoms (see `docs/specs/spectral-oscillator.md`), phase/presence are the Atom D bit, and pole/j2/j4/r_eq are pad (Atom 7 — the form belongs to the anchor, no multipole moments on the wire).

Slot identity is verified by `golden_pack_slots_against_wgsl_access` (mathematikerin tests) — the golden test of the `pack_window` slot layout. The WGSL sources validate offline via naga (`field_wgsl_validates_offline`).
