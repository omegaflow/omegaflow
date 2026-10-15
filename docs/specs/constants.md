<!--
  title: Physical Constants
  class: concept
  sha256: cc51ce87d6ed9ec831d69d4ae253398ecbf93167d29aeecb60b7def5d9037d6b
  status: live
-->
# Physical Constants

`src/archivar/types.rs` (const block)

| Constant | Value | Use |
|----------|-------|-----|
| Φ | 1.618033988749895 | golden ratio — caching, fetch timing, windowing |
| J2000_EPOCH | 2451545.0 | J2000.0 Julian Date |
| J2000_UNIX_OFFSET | 946728000.0 | J2000 (TDB seconds) → UNIX epoch offset (seconds) |
| PARSEC_M | 3.085677581e16 | parsec in meters |
| C_LIGHT | 299792458.0 | speed of light (m/s) |
| HUBBLE_H0 | 70000.0 / (PARSEC_M * 1e6) | Hubble constant in s⁻¹ |
| MAS_YR_TO_RAD_S | 4.84813681109536e-9 / 31557600.0 | mas/yr → rad/s conversion |
| AU | 1.495978707e11 | astronomical unit (meters) |
| GAUSS_K | 0.01720209895 | Gaussian gravitational constant |
| ECLIPTIC_OBLIQUITY | 0.409092804 | ecliptic obliquity at J2000 (radians) |
| CHEBYSHEV_N | 18 | Chebyshev polynomial degree + 1 |
