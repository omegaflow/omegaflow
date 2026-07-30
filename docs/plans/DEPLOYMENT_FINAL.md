# DEPLOYMENT FINAL — Nyquist, Kausalität, 8-Kräfte-Separation

**Konsolidiert aus: NIQUIST.md, DEPLOYMENT-DOKUMENT.md, DEPLOYMENT_REVISED.md**
**Korrigiert nach Council + User-Feedback: Keine Force-Hardware-Identität, `omegaNow` für alle Oberflächen**

---

## Bereits implementiert (Stand Juli 2026)

| Feature | Datei | Status |
|---------|-------|--------|
| `force advective` (ID 7) | `src/main.rs` | ✅ In `force_constants()` |
| VOTable parser | `src/main.rs` | ✅ |
| CSV parser | `src/main.rs` | ✅ |
| POST-Unterstützung | `src/main.rs` | ✅ |
| `lon_sign` für Map-Extract | `src/main.rs` | ✅ |
| Nächste-Nachbar-Resolver | `src/main.rs` | ✅ |
| Queue-Priority | `src/main.rs` | ✅ |
| 2127 Sources | `phi/sources.φ` | ✅ |

---

## Noch zu implementieren

### Phase 1: Rust — Kausalität + Auto-Frame (~40 LOC)

#### 1a. `force_constants_by_id()` (ID → v_or_d + is_diff)

```rust
fn force_constants_by_id(id: f64) -> Option<(f64, bool)> {
    match id as u8 {
        0 => Some((C_LIGHT, false)),
        1 => Some((C_LIGHT, false)),
        2 => Some((V_SOUND_288, false)),
        3 => Some((V_P_GRANITE, false)),
        4 => Some((V_S_GRANITE, false)),
        5 => Some((ALPHA_AIR, true)),
        6 => Some((D_AIR, true)),
        7 => Some((10.0, false)),
        _ => None,
    }
}
```

#### 1b. Kausalitäts-Vorfilter in `enclose_family()`

Vor `let p = smp.motion.at(t2, smp.epoch)` einfügen:
```rust
if let Some((v_or_d, is_diff)) = force_constants_by_id(smp.force_type) {
    if smp.tau > 0.0 && age > smp.tau * 64.0 { continue; }
    if is_diff {
        if 2.0 * v_or_d * age < dist2_p0f { continue; }
    } else {
        let max_causal_dist = v_or_d * age;
        if dist2_p0f > max_causal_dist * max_causal_dist { continue; }
    }
}
```

#### 1c. Auto-Frame aus `lat_key`

In `load_sources()` / `flush!()`:
```rust
let has_data_position = cur_pos.is_some()
    || !cur_lat_str.is_empty()
    || cur_extracts.iter().any(|e| { ... });
```

---

### Phase 2: Frontend — WGSL Shader (~100 LOC)

#### 2a. `field_spatial()` mit Perceptual Capacity

```wgsl
fn field_spatial(d2: f32, d_mag: f32, extent: f32, force_type: u32, global_scale: f32) -> f32 {
    let perceptual_extent = max(extent, global_scale);
    let e2 = perceptual_extent * perceptual_extent;
    let s2 = global_scale * global_scale;
    // ... force-type-specific kernels mit e2 statt extent*extent
}
```

#### 2b. `presence_probe()` — 8 omegas (inkl. advective)

```wgsl
var omegas: array<f32, 8>;  // 0=EM, 1=Gravity, 2=Acoustic, 3=Seismic-body, 4=Seismic-surface, 5=Thermal, 6=Diffusion, 7=Advective
// ...
if (force_type < 8u) { omegas[force_type] += val_eff * sk; }
```

#### 2c. Probe-Buffer 16 → 32 Bytes (8 × f32)

#### 2d. `omegaNow = sum(probedOmegas)` (bleibt)

---

### Phase 3: Frontend — Oberflächen (~30 LOC)

#### 3a. Nyquist-Block löschen

`windowMedianExtent()` + Nyquist-Zoom aus `manifestWindow()` entfernen.

#### 3b. Audio-Surface

```javascript
const lum = Math.tanh(Math.abs(omegaNow) * Math.pow(windowMedianExtent(), 2));
```

#### 3c. Haptic-Surface

```javascript
const lum = Math.tanh(Math.abs(omegaNow) * Math.pow(windowMedianExtent(), 2));
```

#### 3d. Hardware (Serial/USB/BT/HID)

```javascript
let peak = Math.abs(omegaNow);
```

---

## Architektur-Prinzipien (final)

1. **A = A:** Kräfte sind Eigenschaften des Feldes, nicht Routing-Labels für Hardware
2. **Superposition:** `omegaNow = Σ probedOmegas[0..7]` — algebraische Summe mit Vorzeichen
3. **Oberflächen-Agnostik:** Jede Oberfläche manifestiert `Math.abs(omegaNow)` — die totale Feldstärke
4. **Kein Nyquist:** Skala ist strikt manuell
5. **Perceptual Capacity:** `max(extent, global_scale)` entkoppelt Wahrnehmung von physischer Skala
6. **Kausalität:** Kein Sample erreicht die Presence bevor sein Signal im Lichtkegel ankommt
