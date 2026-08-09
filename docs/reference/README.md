# Reference

Code-verified reference for omegaflow internals.

## Internal reference (code-backed)

| File | Content | Code source |
|------|---------|-------------|
| `BINARY_PROTOCOL.md` | 80-byte oscillator record, WebSocket framing, JS→GPU repacking | `src/main.rs:2684-2701` |
| `CONSTANTS.md` | Φ, C_LIGHT, J2000, PARSEC_M, HUBBLE_H0, etc. | `src/main.rs:10-16` |
| `FORCE_SYSTEM.md` | Force IDs, extents, velocities, WGSL kernels, BodyProperties | `src/main.rs:1990-2034` |
| `EXTRACT_TYPES.md` | Extract enum variants and fields | `src/main.rs:1900-1988` |
| `URL_TEMPLATES.md` | URL substitution variables | `src/main.rs:3266-3309` |

## External standards (official documents)

| File | Content | Source |
|------|---------|--------|
| `NIST_SP330_tables.md` | SI base units, derived units, prefixes | NIST SP 330 (SI Brochure, 9th ed, 2019) |
| `NIST_SP811_units.md` | Unit naming, API normalization, non-SI conversions | NIST SP 811 (SI Usage Guide, 2008) |
| `ucum-essence.xml` | Complete UCUM v2.2 machine-readable unit registry | ucum-org/ucum, 2024 |

## Binding (omegaflow-specific)

| File | Content | Source |
|------|---------|--------|
| `../phi/forces.φ` | Force names, IDs, extent, velocity | `src/main.rs:1990-2034` |
| `../phi/units.φ` | Unit→force bindings | NIST SP 330/811 + UCUM (unit identity) + physics (force binding) |
| `../concepts/SI_UNITS.md` | Unit system overview, force-unit matrix | |
| `../concepts/SOURCES_V2_SPEC.md` | Controlling spec for sources.φ syntax | |
