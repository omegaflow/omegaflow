# SI Units — omegaflow Binding

> SUPERSEDED as controlling source by `docs/concepts/SOURCES_V2_SPEC.md` §2/§10.
> `phi/units.φ` and `phi/forces.φ` do not exist — the force/unit registry lives
> in the spec and the parser (`src/main.rs`). The unit-per-force matrix below
> remains a useful physical reference. SI conversion in the parser is open work
> (TODO P02).

## Architecture

| Layer | Document | Content |
|-------|----------|---------|
| **Identity** | `docs/reference/NIST_SP330_tables.md` | SI base + derived units (official BIPM tables) |
| **Identity** | `docs/reference/NIST_SP811_units.md` | API unit normalization, non-SI conversions |
| **Identity** | `docs/reference/ucum-essence.xml` | UCUM v2.2 complete registry |
| **Binding** | `docs/concepts/SOURCES_V2_SPEC.md` §2 | Force names, IDs, units — the growing registry |
| **Parser** | `src/main.rs` | force/unit handling (SI conversion open work: TODO P02) |

No layer is a filter. A unit outside the registry is resolved by consulting
the identity layers (NIST → UCUM) and applying physics reasoning to determine
the force. It is never silently dropped.

## Force-Unit Matrix

| Force | ID | Physical quantities carried | Example units |
|-------|----|----------------------------|---------------|
| em | 0 | radiant flux, magnetic field, photon energy, length (ranging) | W, W/m², T, nT, eV, Jy, Hz, m, km |
| gravity | 1 | acceleration, mass, orbital position, length (gravimetry), magnetic anomaly | m/s², gal, mGal, kg, au, pc, T, nT |
| acoustic | 2 | sound pressure, sound level, wave height, precipitation, frequency | Pa, dB, m, mm, Hz |
| seismic-body | 3 | displacement, acceleration, stress (P-waves) | m, mm, m/s², gal, Pa, Hz |
| seismic-surface | 4 | surface displacement, stress (S/Rayleigh/Love waves) | m, mm, cm, Pa, m/s |
| thermal | 5 | temperature, heat flux, heat energy | K, °C, W/m², W, J |
| diffusion | 6 | concentration, salinity, turbidity, humidity, partial pressure | ppm, ppb, mg/m³, PSU, NTU, %, hPa |
| advective | 7 | wind/current velocity, discharge, dynamic pressure, water level | m/s, km/h, knot, m³/s, Pa, m |
| electric | 8 | E-field, bioelectric potential, current, conductivity, resistivity | V/m, V, A, S/m, Ω·m |

## Fallback Protocol

```
API unit string → NIST_SP811_units.md (identify + normalize)
                → NIST_SP330_tables.md (classify physical quantity)
                → SOURCES_V2_SPEC §2 (look up force binding)
                → if missing: physics reasoning (propagation mechanism)
                → if still unclear: operator review flag
```

## References

- `docs/concepts/SOURCES_V2_SPEC.md` — force registry + units (controlling spec)
- `docs/reference/NIST_SP330_tables.md` — SI base and derived units
- `docs/reference/NIST_SP811_units.md` — unit normalization and conversions
- `docs/reference/ucum-essence.xml` — UCUM v2.2 complete registry
- `docs/reference/ucum-essence.xml`: https://raw.githubusercontent.com/ucum-org/ucum/main/ucum-essence.xml
- NIST SP 330: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.330-2019.pdf
- NIST SP 811: https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication811e2008.pdf
