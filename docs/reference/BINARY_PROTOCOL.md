# Binary Protocol

`src/main.rs:2684-2701`

## WebSocket Frame

| Offset | Size | Value |
|--------|------|-------|
| 0 | 2 | `0xCF 0x86` (magic) |
| 2 | 1 | `0x02` (protocol version) |
| 3 | 4 | `id` u32 LE — presence window identifier |
| 7 | 4 | `count` u32 LE — number of oscillator records |
| 11 | count × 80 | oscillator records |

## Oscillator Record (80 bytes, f64 LE)

| Offset | Bytes | Field | Source |
|--------|-------|-------|--------|
| 0 | 8 | x | ICRS X relative to presence (meters) |
| 8 | 8 | y | ICRS Y relative to presence (meters) |
| 16 | 8 | z | ICRS Z relative to presence (meters) |
| 24 | 8 | val | field value in SI units |
| 32 | 8 | extent | spatial extent (meters) |
| 40 | 8 | epoch | observation epoch (UNIX seconds) |
| 48 | 8 | ttl | time-to-live (seconds) |
| 56 | 8 | tau | decay constant (seconds) |
| 64 | 8 | force_type | force type ID (f64, 0-8) |
| 72 | 8 | absorption | absorption coefficient (0-1) |

## JavaScript → GPU repacking

`static/constants.js:70-104`

- `field`: Float32Array(oscCount × 8) = [x_rel, y_rel, z_rel, val, t, ttl, 0, 0]
- `meta`: Float32Array(oscCount × 4) = [extent, tau, force_type, absorption]

## WGSL unpacking

`static/index.html:305-311`

```
field[id*2]     = vec4f(x, y, z, val)
field[id*2+1]   = vec4f(t, ttl, _, _)
props[id]       = vec4f(extent, tau, force_type, absorption)
```

`force_type` read as `u32(mt.z)`, `absorption` as `f32(mt.w)`.
