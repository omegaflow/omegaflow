# Physical Constants

`src/main.rs` (const block)

| Constant | Value | Use |
|----------|-------|-----|
| Φ | 1.618033988749895 | golden ratio — caching, fetch timing, windowing |
| J2000_EPOCH | 2451545.0 | J2000.0 Julian Date |
| UNIX_J2000_OFFSET | 946728000.0 | UNIX epoch → J2000 offset (seconds) |
| PARSEC_M | 3.085677581e16 | parsec in meters |
| C_LIGHT | 299792458.0 | speed of light (m/s) |
| HUBBLE_H0 | 70000.0 / (PARSEC_M * 1e6) | Hubble constant in s⁻¹ |
| MAS_YR_TO_RAD_S | 4.84813681109536e-9 / 31557600.0 | mas/yr → rad/s conversion |
| AU | 1.495978707e11 | astronomical unit (meters) |
| GAUSS_K | 0.01720209895 | Gaussian gravitational constant |
| ECLIPTIC_OBLIQUITY | 0.409092804 | ecliptic obliquity at J2000 (radians) |
| CHEBYSHEV_N | 18 | Chebyshev polynomial degree + 1 |
