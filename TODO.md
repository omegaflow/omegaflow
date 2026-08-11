# TODO

AGENTS.md is the primary constraint matrix. Git is the history.

## Stand — 2026-08-11

Körper sind Oszillatoren. τ-Gate in `anchor()`. `kernel_for_force` als Tabelle. 3 Höhe-0.0 eliminiert. 3 Eigenbewegung-0.0 eliminiert. 4 Epoche-"jetzt" eliminiert. `/crash` entfernt. `unwrap_or_default` ×2 eliminiert. Exzentrizitäts-Clamp entfernt. Probe-Koordinaten per `--lat`/`--lon`. Casts bereinigt. Force 8 in WGSL. Audio-Radiator safe. Device-τ lebt im Transit. Diverse Konstanten benannt. 16 Tests, 0/0/0.

---

### Zentrismus (7)

**Z02** SSB einziger privilegierter Jump-Target
```
index.html:1259 → const jumps = { '0': { target: 'ssb', grid: Math.pow(2, 54) } };
```
Key '0' → Grid 2⁵⁴. Alle anderen Körper teilen Keys 1–9 mit Grid 2³³.

**Z03** Sonne (NAIF 10) spezialbehandelt im Ephemeriden-Compiler
```
ephemeris_compiler.rs:658 → if granules.is_empty() && target_id != 10 {
```

**Z04** Ephemeris-Kanäle hardcoded auf gravity
```
main.rs:4637 → kernel: 0, force: 1, tau: 86400.0 * 365.0,
```

**Z08** Device-Daten verworfen ohne Körper-Ephemeriden
```
main.rs:2954 → if let (Some(lat), Some(lon), Some(alt)) = (st_lat, st_lon, st_alt) {
```

**Z09** Audio-Radiator ignoriert τ=0 → Device-Oszillatoren stumm
```
main.rs:2381 → if tau_u32 == 0 { continue; }
```

**Z05** Probe-Koordinaten 35N/139E (Tokyo) hardcoded
```
main.rs:5921 → .replace("{lat}", "35.000000").replace("{lon}", "139.000000")
```

**Z08** Device-Daten verworfen ohne Körper-Ephemeriden
```
main.rs:2960 → if eph_map.get(&body_name).and_then(|e| e.props.as_ref()).is_some()
```

---

### Bypass (1)

**B08** `unsafe set_var` in `load_env`
```
main.rs:3200 → unsafe { std::env::set_var(key, val); }
```
Rust-inhärent. Kein safe-Äquivalent für `set_var`. Einmaliger Startup-Call.

---

### Hack (4)

**H10** E > 0.999 still geklammert
```
main.rs:5282 → let e = e_val.clamp(0.0, KEPLER_ECCENTRICITY_MAX);
```

**H11** Hot-Path-Clones: `all.push(s.clone())` jeden Tick
```
main.rs:7245 → all.push(s.clone());
```

**H12** `query_hash` klont Zellen-Referenzen
```
main.rs:643 → out.push(samples);
```

**H15** Exposure-Median ignoriert Force 8
```
index.html:604 → if (ft >= 0 && ft < 7)
```
`ft_vals[8]` gesammelt aber nie verwendet. Force 8 bekommt Exposure über kernel_id=5.

---

### Fabrikation (28)

**F01–F24** 24 `unwrap_or`-Varianten (vollständig):

String/Path:
- `main.rs:1531` → `.unwrap_or("")` — `jlast` Pfad-Split
- `main.rs:1532` → `.unwrap_or(key)` — `jlast` final key
- `main.rs:1567` → `.unwrap_or(("", key))` — `jfirst`
- `main.rs:1651` → `strip_prefix('#').unwrap_or(trimmed)` — CSV header
- `main.rs:1673` → `.unwrap_or(false)` — text_last_col alpha
- `main.rs:1703` → `.unwrap_or(r.len())` — regex end-scan
- `main.rs:1705` → `r.find(suffix).unwrap_or(r.len())` — regex suffix
- `main.rs:2703` → `.unwrap_or("")` — `/crash` body parse
- `main.rs:3919` → `s.lines().next().unwrap_or("")` — parse_path
- `main.rs:5163` → `strip_prefix('#').unwrap_or(t)` — Rows header
- `main.rs:5837` → `strip_prefix("www.").unwrap_or(netloc)` — netloc
- `main.rs:5843` → `.unwrap_or(url)` — source name ×3
- `main.rs:6032` → `.unwrap_or(sub.len())` — template value
- `main.rs:6266` → `strip_prefix('#').unwrap_or(trimmed)` — CSV header 2
- `main.rs:6757` → `.map(|x| x == "φ").unwrap_or(false)` — source filter
- `main.rs:7272` → `.unwrap_or(r.len())` — horizons parse
- `bsp_reader/daf.rs:229` → `.unwrap_or("")` — summary name

Default-Insertion:
- `main.rs:2726` → `unwrap_or_else(|_| cfg.index_html.clone())` — index.html
- `main.rs:2835` → `unwrap_or_else(|_| cfg.constants_js.clone())` — constants.js
- `main.rs:4385` → `unwrap_or(&[])` — universal auto-extract
- `main.rs:7957` → `.unwrap_or_else(|| "absent".into())` — diagnostic
- `horizons_compiler.rs:580` → `.unwrap_or_default()` — header parse
- `ephemeris_compiler.rs:662` → `.unwrap_or(Ordering::Equal)` — sort
- `ephemeris_compiler.rs:663` → `.unwrap_or(Ordering::Equal)` — sort
- `horizons_compiler.rs:604` → `.unwrap_or(Ordering::Less)` — sort

**F33** Geschwindigkeit `[0.0, 0.0, 0.0]` — CelestialPolygon
```
main.rs:5161 → v: [0.0, 0.0, 0.0],
```

**F35** State-Vector hardcoded gravity
```
main.rs:4637 → kernel: 0, force: 1, tau: 86400.0 * 365.0,
```

**F36** Gleiche Messung, verschiedene Kraft
```
main.rs:4637 → Extract::Ephemeris → force: 1 (gravity)
main.rs:4715 → Extract::Vectors    → force: 0 (em)
```

**F39** Zero-BodyProperties für körperlose Körper
```
horizons_compiler.rs:707 → BodyProps { a0_deg: 0.0, radius_m: 0.0, flattening: 0.0, … }
```

**F40** NAIF-ID unbekannt → "unknown"
```
ephemeris_compiler.rs:604 → _ => "unknown"
```

**F43** Kernel-Parameter null-initialisiert
```
main.rs:132 → gaussian_inverse_square: 0.0, …
```

---

### Daten zweiter Klasse (5)

**D04** Force 8 ohne Luminanz-Referenz
```
index.html:604 → if (ft >= 0 && ft < 7)
```

**D05** Gleiche Messung, verschiedene Kraft
```
main.rs:4637 vs 4715 → (siehe F36)
```

**D06** Kernel-IDs auf 6 geklammert
```
index.html:346 → let k_id = min(u32(mt.z), 6u);
```

**D07** Kein Oszillator-Cap in Rust
```
index.html:534 → while (c * 32 > maxBuf) c >>= 1;
```

**D08** `/device` scannt nur Device-Quellen
```
main.rs:2746 → if matches!(osc.source, OscillatorSource::Device)
```

---

### Bias (6)

**B01** 4 Kräfte mit 0.0 m/s
```
index.html:259 → (c, c, 343.0, 6000.0, 3000.0, 0.0, 0.0, 0.0)
```
Thermal (5), Diffusion (6), Advective (7) → 0.0 m/s.

**B02** `kernel_for_force` asymmetrisch
```
main.rs:2044 → match force { 0|1 => 0, 5|6 => 3, 4 => 1, 8 => 5, _ => 1 }
```

**B03** Exposure pro Kernel, nicht pro Force
```
index.html:350 → var lvl = vp.expose_lo[min(k_id, 3u)];
index.html:351 → if (k_id >= 4u && k_id < 7u) { lvl = vp.expose_hi[k_id - 4u]; }
```

**B04** Fragment-Shader ignoriert force_type
```
index.html:434 → fn fs(in: VOut) → @location(0) vec4f {
    let intensity = 0.02 / (dist * dist + 0.02);
```
`K(force_type, extent, d, softening)` fehlt.

**B05** ISS bekommt spezielles Datenfenster
```
horizons_compiler.rs:727 → let months = if *name == "iss" { 0.9 } else { 1.0 };
```

**B06** Hardcodierte Body-Listen in Compilern
```
horizons_compiler.rs:19 → BODIES_WITH_MEDIA (23 Einträge)
ephemeris_compiler.rs:20 → BODIES_WITH_MEDIA, all_target_ids
ephemeris_compiler.rs:579 → body_id_to_name Tabelle
```

---

## Residual

### Source Curation
- `archeology/sources/sources_gold_359-domains.φ` — 27K lines, 359 domains, OLD grammar
- EVERY source block needs migration: `force em` → `field <key> <name> <kernel> <force> <unit>`
- Arena batches (`archeology/arena/`) contain API proposals not yet in φ files
- `archeology/sources/sources_new_untested_14k_new-unchecked.φ` — 873 unchecked blocks
- `archeology/sources/sources_new_untested_candidate-staging.φ` — staging candidates
- `archeology/foundation/gaps.md` — domain coverage gaps
- `archeology/foundation/collection.md` — curated collection state

### Code Hygiene
- 3-part `field` form hardcodes `tau: 0.0` with no gate — main.rs:3735
- Tau-Gate inkonsistent: 5-token/6-token prüfen `v > 0.0`, 9-token prüft nicht

### Validation
- `--verify` CLI exists (tests URL reachability), no sources loaded yet
- Old sources: `pos` and `field_in` tokens → parser ignores via `_ => {}`

### CI Pipeline
- `mirror-cdn.yml`: cron disabled
- `generate-ephemerides.yml`: Python/spiceypy → needs Rust rewrite
- `refresh-protected-data.yml`: Python inline scripts → needs Rust rewrite

### Feature Backlog
- Advective per-Oszillator: wind speed in `tm.w` (channel wired, data source needed)
- OPeNDAP Integration
- New Extract Types: Kepler, HorizonsVec, Flatten
- `field_in` nested support
- Command Palette (⌘K)
- Minkowski 4D Weighting

### Deferred
- Temporal Topology (TDA, Takens, Transfer Entropy)
- Field Permeability

### Rejected
- Unknown-Force soft fallback → parser rejects unknown force
- Default τ values → gate closes if not declared
- World Bank Indicators → forceless, DROP
- Yahoo Finance → forceless, DROP
