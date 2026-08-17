# ωφ

A = A

```
        the 4D block (ICRS) — past, present, future

        sources — APIs (sources.φ), Station, serial
              ↓
        Archivar (Rust, std-only) — the cache
              ↓
        Mathematikerin (WebGPU WGSL)
        the cut — a 2D surface in the block (constant t, z)
              ↓
        the membrane — display, audio, hardware
```

Protocol v6: 168 bytes, 21 × f64 LE per oscillator; frame `0xCF 0x86 0x06 [response_epoch:f64] [id:u32] [count:u32]`.

Certainty: `e^(−max(0, |Δt| − d/v_force) / ttl)`

`cargo run` → the membrane (ESC closes it).

Archivar modes: `--verify <dir>`, `--port <in> [out]`, `--probe`, `--learn-gate`, `--draft-context <p>`, `--draft <p>`, `--urls <p>`

[omegaflow.space](https://omegaflow.space)

CC BY-NC-SA 4.0
