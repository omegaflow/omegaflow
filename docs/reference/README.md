# Reference

Code-verified reference for omegaflow internals.

## Internal reference (code-backed)

| File | Content | Code source |
|------|---------|-------------|
| `BINARY_PROTOCOL.md` | v6: 19-byte WS header (response_epoch), 168-byte/21-f64 records, query frame, JS→GPU repacking | `src/main.rs` resonance, `static/constants.js` |
| `CONSTANTS.md` | Φ, C_LIGHT, J2000, PARSEC_M, HUBBLE_H0, etc. | `src/main.rs` |
| `FORCE_SYSTEM.md` | 9 force channels, 7 kernel shapes, BodyProperties, body channels | `src/main.rs`, `static/index.html` |
| `EXTRACT_TYPES.md` | Extract enum variants and fields | `src/main.rs` |
| `URL_TEMPLATES.md` | URL substitution variables | `src/main.rs` |

## External standards (official documents)

| File | Content | Source |
|------|---------|--------|
| `NIST_SP330_tables.md` | SI base units, derived units, prefixes | NIST SP 330 (SI Brochure, 9th ed, 2019) |
| `NIST_SP811_units.md` | Unit naming, API normalization, non-SI conversions | NIST SP 811 (SI Usage Guide, 2008) |
| `ucum-essence.xml` | Complete UCUM v2.2 machine-readable unit registry | ucum-org/ucum, 2024 |

## Binding (omegaflow-specific)

| File | Content | Source |
|------|---------|--------|
| `../concepts/SI_UNITS.md` | Unit system overview (superseded by SOURCES_V2_SPEC „Force-Unit Registry") | |
| `../concepts/SOURCES_V2_SPEC.md` | Controlling spec for sources.φ syntax — force names, IDs, units registry | `src/main.rs` parser |
