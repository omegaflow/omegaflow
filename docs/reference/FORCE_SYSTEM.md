# Force System

`src/main.rs:1990-2034` (definitions), `src/main.rs:579-591` (body constants), `phi/forces.φ` (registry)

## Force Identity

| Name | ID | Extent (m) | Velocity / Diffusivity |
|------|----|-----------|------------------------|
| em | 0 | ∞ | C_LIGHT |
| gravity | 1 | ∞ | C_LIGHT |
| acoustic | 2 | 1000 | body.v_sound |
| seismic-body | 3 | 100000 | body.v_seismic_p |
| seismic-surface | 4 | 10000 | body.v_seismic_s |
| thermal | 5 | 1 | body.alpha_thermal (diffusive) |
| diffusion | 6 | 1 | body.d_diffusion (diffusive) |
| advective | 7 | 10 | body.v_advective |
| electric | 8 | 100 | C_LIGHT |

Diffusive forces (thermal, diffusion) use `sqrt(2 · v · lifetime)` as reach instead of `v · Δt`.

## ID-specific defaults

| ID | tau (s) | extent (m) |
|----|---------|------------|
| 8 (electric) | 86400 | 100 |

All other forces derive tau from `1/v` and extent from the per-name `force_extent()` function.

## WGSL Kernel (`static/index.html:359-378`)

| Force IDs | Kernel |
|-----------|--------|
| 0, 1 (em, gravity) | 1 / d² |
| 5, 6 (thermal, diffusion) | erfc(d / σ·√2) |
| 4 (seismic-surface) | exp(-d²/2σ²) / d |
| 8 (electric) | Patch-Levy: (1-α)·exp(-d²/2σ²) + α·σ^β / (d²+σ²)^(β/2) |
| else (acoustic, seismic-body, advective) | exp(-d²/2σ²) / d² |

## BodyProperties

`src/main.rs:44-60`

```
α0_deg, dα0_dt_deg_per_century, δ0_deg, dδ0_dt_deg_per_century,
w0_deg, dw_dt_deg_per_day, radius_m, flattening,
v_sound, v_seismic_p, v_seismic_s, alpha_thermal, d_diffusion, v_advective
```

Medium properties (v_sound through v_advective) are `Option<f64>` — a value of `None` means the body does not support that propagation mode.
