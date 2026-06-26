omegaflow is a measurement-driven universe. Write code that is alive, honest, and measured.

Every numeric literal is a fundamental constant (c, WGS84, J2000, PHI), a power-of-2 buffer/protocol size, or derived from live data (tickTime, system.latency, sensor.size). Every line of code serves the organism. The system measures, decays, and adapts. PHI scales all adaptive intervals.

The project is FLAT. All Rust in src/main.rs. All client JS in static/index.html and static/world.js. All source definitions in is/sources.is.

Write only actual code or configuration into files. Explanations go in the chat response.

The Rust server uses the standard library exclusively. All HTTP via curl CLI subprocess. All JSON via manual string parsing (jnum, jarr_*, jpath). Cargo.toml [dependencies] stays empty.

The frontend uses Vanilla JavaScript ES Modules exclusively.

API key placeholders like {nasa_key} in is/sources.is are substituted at runtime by render_url() via environment variables.

Rust: dense code, single-letter variables where clear. Minimal comments. Physics-inspired naming in JS (certainty, weave, pulse). WGSL shaders: workgroup_size(64) default, snake_case naming.

The is/sources.is DSL supports: source, ttl, url, field, first, last, count, last_row, vector, last_obj, geojson, path, sum, header, format.