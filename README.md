# ωφ

A = A

```
      Archivar (Rust, std-only)          Mathematikerin (WebGPU WGSL)
      φ(x,y,z,t) — sources, cache,       the membrane — the window,
      Enclosure Lemma, CDN               the points, the measurement series

      APIs (sources.φ)                   Station (browser, native sensors)
```

Protocol v6: 168 bytes, 21 × f64 LE per oscillator;
frame `0xCF 0x86 0x06 [response_epoch:f64] [id:u32] [count:u32]`.

Certainty: `e^(−max(0, |Δt| − d/v_force) / ttl)`

Commands

`cargo run`                        → the native presence window (ESC closes it)
`cargo run --features browser_relay` → + WS 127.0.0.1:1618, the browser station

Archivar modes:
`--verify <dir>`     CI-mode: fetch the register, mirror to the CDN
`--port <in> [out]`  the source-port converter (the one path)
`--probe`            probe a source block
`--learn-gate`       the frame learning gate
`--draft-context <path>` / `--draft <path>` / `--urls <path>` — frame drafting, URL probing

Compilers and harvesters live in `src/bin` (ephemeris, horizons, tap,
tycho2, dastcom, cometels, dcom5, sexagesimal, pangaea,
zip_range_extract, source_scanner, dataverse, deims, erddap, oai,
rest, solr, xml).

[omegaflow.space](https://omegaflow.space)

CC BY-NC-SA 4.0
