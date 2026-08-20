# Force System

`src/main.rs` (`force_id_of`, `kernel_extent`, `body_channels`), `docs/concepts/SOURCES_V2_SPEC.md` (registry source), `static/index.html` (`field_spatial`).

## Force Identity (9 channels, ID 0–8)

| ID | Name | Propagation speed (WGSL `PROPAGATION_SPEED`) |
|----|------|----------------------------------------------|
| 0 | em | C_LIGHT |
| 1 | gravity | C_LIGHT |
| 2 | acoustic | 343 m/s (air) |
| 3 | seismic-body | 6000 m/s |
| 4 | seismic-surface | 3000 m/s |
| 5 | thermal | 0.3 m/s (diffusive) |
| 6 | diffusion | 0.05 m/s (diffusive) |
| 7 | advective | per-source advection slot when > 0, else 1 m/s |
| 8 | electric | C_LIGHT |

The speed table lives in the shader; advective uses the per-source slot (`field[j*3+2].x`).

## Kernel Shapes (7, keyed by `kernel_id` — decoupled from force)

`field_spatial` in `static/index.html`:

| kernel_id | Shape |
|-----------|-------|
| 0, 6 | `1.0 / max(d², 1.0)` — pure Nebra, guard d > 1 m, no softening |
| 1 | `exp(-d²/(2·max(e²,s²))) / max(d²+s², s²)` |
| 2 | `exp(-d²/(2·max(e²,s²))) / (d + sqrt(s²))` |
| 3 | `erfc(d / max(σ·√2, s))` |
| 4 | `exp(-d / max(σ, s))` |
| 5 | Patch-Levy: `(1-α)·exp(-d²/(2·max(e²,s²))) + α·max(e²,s²)^(β/2) / max(d²+s², s²)^(β/2)`, β = 1.6 |

`e²`/`s²` carry a 1e-30 floor — f32-underflow shield, not softening. Kernel 0/6 + gravity
additionally gets the zonal harmonic term
`1 − J2·(r_eq/d)²·P2(cosθ) − J4·(r_eq/d)⁴·P4(cosθ)` with `cosθ = dot(d̂, pole)`
(0 honored when j2/j4 absent).

## BodyProperties

`src/main.rs` struct — 22 fields; the last eight are `Option` (nut series
`Option<Vec<[f64;3]>>`, nutation `Option<Vec<NutationRecord>>`):

```
α0_deg, dα0_dt_deg_per_century, δ0_deg, dδ0_dt_deg_per_century,
w0_deg, dw_dt_deg_per_day, radius_m, flattening,
gaussian_inverse_square, gaussian_inverse, erfc, exponential_decay, patch_levy,
gm, j2, j4, radii_b, radii_c, nut_ra, nut_dec, nutation
```

Kernel extents derive via `kernel_extent(kernel_id, props, tau)`: kernel 0/6 → ∞;
kernels 1–5 → `sqrt(2 · param · tau)` with `param` the matching BodyProperties field
(0 = absent → extent 0). The first eight fields come from the ephemeris binary
(stype-1 v2, gcount=12); flattening is derived from R_a/R_c in v2.

## Ephemeris Binary Sections

`ephemeris_{body}.bin` (magic `0xCF 0x86 0x01 0x00`): header (granule count +
degree, Chebyshev position granules in meters) followed by sections:

| stype | Inhalt |
|-------|--------|
| 1 | Body-Parameter v2 (gcount=12): α0/dα0/δ0/dδ0/w0/dw0, Radii a/b/c, J2, J4, GM (m³/s²) |
| 2 | Kernel-Parameter (gcount=0, 5 Doubles, ungenutzt) |
| 3 | Rotationsmatrizen: je Record [t0_jd f64 + 9 f64] (stype-3-Abtastung aus dem vollen Orientierungsmodell) |
| 4 | **Nutationsreihe (additiv, K02)**: je Record [mid_jd, half_jd, RA-Koeffizienten, DEC-Koeffizienten, PM-Koeffizienten] — Chebyshev-Fit des Deltas (volles Modell − lineares Modell). Volles Modell = Binary-PCK (moon_pa_de440, Typ 2, Precedence binary > text) bzw. Text-PCK mit NUT_PREC-Reihen. Runtime addiert die Delta-Evaluation in `orientation_angles_at` (pole, gravity, Rotation). |

Absent properties sind 0.0 — die neutrale Konstante des festen Strides.
Nutation absent = Körper ohne Binary-PCK und ohne NUT_PREC-Reihen (0 honored).

## Asteroid Catalog Channels (K03)

`catalog_dastcom` (φ-Format): DASTCOM5-Asteroidenkatalog (92-B-Stride).
Nur Körper mit gemessenem GM manifestieren — GM-Gate, alle anderen sind absent
(0 honored). Massen-Kanal: val = GM (m³/s²); Radius-Kanal: val = Radius (m);
beide kernel 0, force 1, τ = ∞, extent = Hill-Radius a·(GM/3GM_sun)^(1/3).
Position/Velocity via Kepler-Evaluation zum Query-Zeitpunkt (die Zwei-Körper-
Physik des Katalogs; die Elemente sind osculating, die TTL-Frische regelt den
Takt gegen das n-body-Wahre).

## Body Channels

`body_channels()` emits exactly one channel:
- `{body}.mass` — val = gm, kernel 0, gravity, τ = ∞ (only when gm measured)

The `{body}.radius` channel died in Atom 7 — the form belongs to the anchor,
not the measurement. A planet is only its mass (GM). Gravity renders as a pure
field law (no multipole moments); the wire slots pole/j2/j4/r_eq carry 0.0 for
force_type 1 (0 honored). The occlusion (ephemeris radius barriers) died in
Atom 8 — replaced by the membrane diode threshold; field absorption is pending.
Rotation (erfc) and gi_sq channels were killed — NaN singularity
at the source center and a val=1.0 fabrication.
