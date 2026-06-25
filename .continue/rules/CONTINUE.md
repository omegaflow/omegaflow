---
name: omegaflow-rules
description: Architecture and coding rules for omegaflow
---

# omegaflow — Project Guide

## CORE PRINCIPLES (ALWAYS APPLY)

1. Put only code or configuration into files. Reasoning and explanations go in the chat response.
2. Keep the project structure FLAT. All files stay in their current directories.
3. Use only the Rust standard library, curl subprocess, and manual string parsing. Keep Cargo.toml empty.
4. Use vanilla ES modules exclusively. Stick to the browser-native stack.
5. Treat placeholders like {nasa_key} as runtime variables. Leave them for render_url().
6. Output only the changed lines with surrounding context when editing.
7. Write dense Rust with minimal comments. Use single-letter variables where clear.
8. Use workgroup_size(64) and snake_case naming in WGSL shaders.
9. Follow the is/sources.is DSL syntax: source, ttl, url, field, first, last, count, last_row, vector, last_obj, geojson, path, sum, header, format.

---

## 1. Overview

Real-time, measurement-driven system.
Answers: is(t, x, y, z) — state of the universe at any coordinate.

No theoretical models. No predictions. No catalogs.
Fetches live measurements from scientific APIs (NOAA, NASA, USGS).
Combines with local sensor data from observer's device.
Collapses into a single certainty value.

## 2. Key Technologies

- Server: Rust (edition 2024), standard library only
- Client: Vanilla JS ES modules, WebGPU (WGSL shaders), WebSocket
- HTTP: All fetching via curl CLI subprocess
- JSON: Hand-rolled string parsing
- Hardware: ESP32-S3 via WebSerial (planned)

## 3. Architecture

```
Observer (Browser)         Server (Rust)
+------------------+       +---------------------+
| index.html       |  WS   | src/main.rs         |
|  Sensor Discovery|<----->|  HTTP + WebSocket   |
|  Actuator Probing|       |  weave()            |
|  WebGPU Topology |------>|  sources.is parser  |
|  Immunity Tracker|<------|  immunity.is store  |
+------------------+  33B  +---------------------+
| world.js         |
|  get(t,x,y,z)    |
|  Certainty (GPU) |
|  IS protocol     |
+------------------+
```

## 4. Project Structure (FLAT — no crates/)

```
omegaflow/
  src/main.rs          # Entire backend
  static/index.html    # Client app (inline JS)
  static/world.js      # Certainty, IS protocol, GPU
  is/sources.is        # API source definitions (DSL)
  is/immunity.is       # Actuator safety records
  docs/                # Roadmap, foundation, specs
  Cargo.toml           # Zero dependencies
```

## 5. Key Files

### src/main.rs (~400 lines)

| Function | Purpose |
|---|---|
| main() | TCP listener, thread per connection |
| handle_observer() | Routes HTTP vs WebSocket |
| handle_pulse() | WS lifecycle, binary frames |
| weave() | Core: fetch sources, return IS binary |
| load_sources() | Parses is/sources.is |
| ecef_to_geodetic() | ECEF to lat/lon/alt |
| render_url() | Template: {lat}, {lon}, {today} |
| jnum(), jarr_*() | JSON extractors (string-based) |

### static/index.html (~500 lines inline JS)

| System | Purpose |
|---|---|
| discoverSensors() | Dynamic window/navigator walk |
| tick(ts) | Main adaptive loop |
| startBroad/Narrowing() | Actuator probing state machine |
| runSignalTopology() | WebGPU Kolmogorov complexity |

### static/world.js (~250 lines)

| Export | Purpose |
|---|---|
| get(t,x,y,z) | Fetch data, evaluate certainty |
| live | All current sensor values |
| pulse | WebSocket state |

### is/sources.is (Custom DSL)

```
source <name> [on_earth]
  ttl <seconds>
  url <template_url>
  field <json_key> <output_name>
  first <json_key> <output_name>
  last <json_key> <output_name>
  count <json_key> <output_name>
  last_row <column> <output_name>
  vector <out_x> <out_y> <out_z>
  last_obj <fk> <fv> <ek> <output>
  geojson <name> <dist> <mag_key> <min> <out...>
```

Templates: {lat}, {lon}, {today}, {tomorrow},
{lat_min}, {lat_max}, {lon_min}, {lon_max}

## 6. Coding Conventions

### Rust
- Dense, single-letter variables
- Standard library only
- All HTTP via curl subprocess
- JSON parsed with custom string functions
- Everything in one file: src/main.rs

### JavaScript
- ES modules, browser-native
- PHI (1.618...) used as adaptive scaling constant
- Physics-inspired naming (certainty, weave, pulse)

### IS Binary Protocol

```
Header: "IS" + version(1B) + obj_count(4B LE)
Per object:
  field_count(1B)
  Per field:
    name_len(1B) + name + type(1B)
    0=f64, 1=u32, 2=f64 array
  record_count(4B LE)
  If records > 0:
    field descriptors + values
```

## 7. Certainty Formula

```
certainty = exp(-dt * g)
          * exp(-v_c / (g + eps))
          * quantum * decay * epig
```

| Factor | Source |
|---|---|
| g | Accelerometer magnitude |
| v_c | GPS speed / c |
| decay | 1/(1 + cosmic_protons) |
| quantum | exp(-avg(noiseFloor)) |
| dt | abs(t - server_time) |
| epig | Hardcoded 1.0 (Phase 7) |

## 8. Key Concepts

### Dynamic Topological Discovery (S33)
Client discovers all properties of window/navigator.
Numbers/booleans = sensors.
Functions = actuators.
Any device is supported automatically.

### Immunity System
Probed actuators that cause no response
get pokeValue multiplied by PHI.
Exceeds 1e15 = marked dead.
Persisted in is/immunity.is.

### Resonance Map
Map of actuator-to-sensor causal links.
Records threshold, magnitude, latency.

### PHI Scaling
All adaptive intervals scale by 1.618...
Fast when things change.
Exponential backoff when stable.

## 9. Common Tasks

### Debug WebSocket
1. DevTools -> Network -> WS
2. Filter /pulse
3. Outgoing: 37 bytes (33 + 4 ID)
4. Incoming: starts with "IS"

### Check Active Sources
```bash
cat is/sources.is
curl localhost:8080/immunity
curl localhost:8080/time
```

### Inspect Live Values
```javascript
window.omegaflow.live
window.omegaflow.sensors
window.omegaflow.actuators
window.omegaflow.resonanceMap
```

## 10. Key Constants

- Speed of light: 299792458.0 (world.js)
- Golden ratio: 1.618033988749895 (index.html)
- WGS84 a: 6378137.0, f: 1/298.257223563
- J2000 offset: 2440587.5 - 2451545.0

## 11. Deployment

- URL: omegaflow.space
- GitHub: github.com/omegaflow/omegaflow
- License: CC BY-NC-SA 4.0
- Docker: multi-stage Rust build
- Fly.io: region fra, port 8080, HTTPS
