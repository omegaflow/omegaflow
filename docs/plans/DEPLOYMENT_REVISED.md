Hier ist das konsolidierte, bereinigte und absolut deterministische Deployment-Dokument. Es vereint alle drei Dokumente zu einer einzigen, nahtlosen Architektur-Anweisung. 

Die Nyquist-Skalen-Manipulation wurde entfernt, die Kräfte-Separation (inklusive der 8. Kraft `advective`) ist vollständig integriert, und die perzeptive Kapazität (`max(extent, global_scale)`) entkoppelt die Wahrnehmung endgültig von der physischen Skala.

***

# FINALES KONSOLIDIERTES DEPLOYMENT-DOKUMENT

## DATEI 1: src/main.rs

### SCHRITT 1
Suche in der Datei `src/main.rs` nach der Funktion `fn force_constants(force: &str) -> Option<(f64, f64, bool, u8)>`.
Füge direkt unter dem Eintrag für `"diffusion"` den neuen Eintrag für `"advective"` hinzu:

**Original:**
```rust
        "diffusion" => Some((D_AIR, 1.0 / D_AIR, true, 6)),
        _ => None,
```
**Ersetzen durch:**
```rust
        "diffusion" => Some((D_AIR, 1.0 / D_AIR, true, 6)),
        "advective" => Some((10.0, 100.0, false, 7)),
        _ => None,
```

### SCHRITT 2
Suche die Funktion `fn force_type_of(force: &str) -> f64` und füge dort ebenfalls den Eintrag hinzu:

**Original:**
```rust
        "diffusion" => Some((D_AIR, 1.0 / D_AIR, true, 6)),
        _ => None,
    }
}
```
**Ersetzen durch:**
```rust
        "diffusion" => Some((D_AIR, 1.0 / D_AIR, true, 6)),
        "advective" => Some((10.0, 100.0, false, 7)),
        _ => None,
    }
}
```

### SCHRITT 3
Füge direkt unterhalb der schließenden Klammer `}` der Funktion `fn force_type_of` die neue Umkehrfunktion inklusive der 8. Kraft ein:

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

### SCHRITT 4
Suche in der Datei `src/main.rs` nach der Funktion `fn enclose_family`. 
Finde innerhalb dieser Funktion den folgenden Code-Block:

```rust
    for samples in visit {
        for smp in samples {
            let age = (t2 - smp.epoch).abs();
            let reach = smp.extent + smp.vmax * age + 0.5 * smp.amax * age * age + pad;
            let dx = smp.p0f[0] - qf[0];
            let dy = smp.p0f[1] - qf[1];
            let dz = smp.p0f[2] - qf[2];
            if dx * dx + dy * dy + dz * dz > reach * reach {
                continue;
            }
            let p = smp.motion.at(t2, smp.epoch);
            let ddx = p[0] - q[0];
            let ddy = p[1] - q[1];
            let ddz = p[2] - q[2];
            let exact = smp.extent + pad;
            let dist2 = ddx * ddx + ddy * ddy + ddz * ddz;
            if dist2 > exact * exact {
                continue;
            }
```

Ersetze diesen Code-Block vollständig durch den Kausalitäts-gefilterten Block:

```rust
    for samples in visit {
        for smp in samples {
            let age = (t2 - smp.epoch).abs();
            let reach = smp.extent + smp.vmax * age + 0.5 * smp.amax * age * age + pad;
            let dx = smp.p0f[0] - qf[0];
            let dy = smp.p0f[1] - qf[1];
            let dz = smp.p0f[2] - qf[2];
            let dist2_p0f = dx * dx + dy * dy + dz * dz;
            
            if dist2_p0f > reach * reach {
                continue;
            }

            if let Some((v_or_d, is_diff)) = force_constants_by_id(smp.force_type) {
                if smp.tau > 0.0 && age > smp.tau * 64.0 {
                    continue; 
                }

                if is_diff {
                    if 2.0 * v_or_d * age < dist2_p0f {
                        continue;
                    }
                } else {
                    let max_causal_dist = v_or_d * age;
                    if dist2_p0f > max_causal_dist * max_causal_dist {
                        continue;
                    }
                }
            }

            let p = smp.motion.at(t2, smp.epoch);
            let ddx = p[0] - q[0];
            let ddy = p[1] - q[1];
            let ddz = p[2] - q[2];
            let exact = smp.extent + pad;
            let dist2 = ddx * ddx + ddy * ddy + ddz * ddz;
            
            if dist2 > exact * exact {
                continue;
            }
```

### SCHRITT 5
Suche in der Datei `src/main.rs` innerhalb des Makros `flush!()` nach der Zeile `let has_data_position = cur_pos.is_some()`.
Ersetze diese Zeile:

```rust
                    let has_data_position = cur_pos.is_some()
                        || cur_extracts.iter().any(|e| {
```

Durch:

```rust
                    let has_data_position = cur_pos.is_some()
                        || !cur_lat_str.is_empty()
                        || cur_extracts.iter().any(|e| {
```

---

## DATEI 2: static/index.html

### SCHRITT 6
Suche in der Datei `static/index.html` nach der Konstante `const fieldShader = \``.
Ersetze den gesamten WGSL-Code-Block (von `struct VP { surface: vec4f...` bis zur schließenden Klammer vor dem schließenden Backtick `\`;`) durch den folgenden Code. Dieser Shader vereint die Punktwolken-Projektion, die 8-Kräfte-Separation, die perzeptive Kapazität und das analoge Rauschen in reiner Form:

```wgsl
struct VP { surface: vec4f, right: vec4f, up: vec4f, expose: vec4f };
@group(0) @binding(0) var<storage, read> field: array<vec4f>;
@group(0) @binding(1) var<storage, read> props: array<vec4f>;
@group(0) @binding(2) var<uniform> vp: VP;
@group(0) @binding(3) var<storage, read_write> probe_out: array<f32>;

struct VOut { @builtin(position) pos: vec4f, @location(0) uv: vec2f, @location(1) color: vec3f };

@vertex fn vs(@builtin(vertex_index) i: u32) -> VOut {
    let count = u32(vp.surface.z);
    var out: VOut;
    out.pos = vec4f(0.0, 0.0, 0.0, 1.0);
    out.uv = vec2f(0.0);
    out.color = vec3f(0.0);
    if (count == 0u) { return out; }

    let id = i / 6u;
    if (id >= count) { return out; }
    let vid = i % 6u;

    var quad = array<vec2f, 6>(
        vec2f(-1.0, -1.0), vec2f(1.0, -1.0), vec2f(1.0, 1.0),
        vec2f(-1.0, -1.0), vec2f(1.0, 1.0), vec2f(-1.0, 1.0)
    );

    let m = field[id];
    let d = m.xyz;
    let raw_val = m.w;
    let mt = props[id];
    let extent = mt.x;
    let retarded_dt = mt.y;
    let tau = mt.z;

    let val = raw_val * exp(-retarded_dt / max(tau, 1e-9));

    let w = vp.surface.x;
    let h = vp.surface.y;
    let scale = vp.surface.w;

    let x = dot(d, vp.right.xyz);
    let y = dot(d, vp.up.xyz);
    var clip = vec2f(x / (w * scale * 0.5), y / (h * scale * 0.5));

    let dist = max(length(d), scale);
    let point_size_px = max((extent / dist), 1.0) * 2.0;
    clip.x += (quad[vid].x * point_size_px) / w * 2.0;
    clip.y += (quad[vid].y * point_size_px) / h * 2.0;
    
    out.pos = vec4f(clip, 0.0, 1.0);
    out.uv = quad[vid];

    let lvl = vp.expose.x;
    if (lvl <= 0.0) { return out; }
    let aw = abs(val);
    let t2 = clamp((log2(aw / lvl) + 8.0) / 24.0, 0.0, 1.0);
    let t2s = t2 * t2 * (3.0 - 2.0 * t2);
    let c1 = mix(vec3f(0.0, 0.0, 0.0), vec3f(0.0, 0.15, 0.4), clamp(t2s * 4.0, 0.0, 1.0));
    let c2 = mix(c1, vec3f(0.1, 0.5, 0.9), clamp((t2s - 0.25) * 4.0, 0.0, 1.0));
    let c3 = mix(c2, vec3f(0.8, 0.6, 0.2), clamp((t2s - 0.5) * 4.0, 0.0, 1.0));
    let c4 = mix(c3, vec3f(1.0, 0.95, 0.9), clamp((t2s - 0.75) * 4.0, 0.0, 1.0));
    out.color = c4;
    return out;
}

fn erfc(x: f32) -> f32 {
    let xa = abs(x);
    let t = 1.0 / (1.0 + 0.3275911 * xa);
    let poly = t * (0.254829592 + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    let y = poly * exp(-xa * xa);
    return select(y, 2.0 - y, x < 0.0);
}

fn field_spatial(d2: f32, d_mag: f32, extent: f32, force_type: u32, global_scale: f32) -> f32 {
    // Perzeptive Kapazität: Das Maximum aus physischer Ausdehnung und globaler Skala
    let perceptual_extent = max(extent, global_scale);
    let e2 = perceptual_extent * perceptual_extent;
    let s2 = global_scale * global_scale; // Fallback für extrem kleine Skalen
    
    if (force_type == 0u || force_type == 1u) {
        let denom = max((d2 + s2) * (d2 + s2 + e2), s2 * s2);
        return e2 / denom;
    } else if (force_type == 5u || force_type == 6u) {
        return erfc(d_mag / max(perceptual_extent * 1.41421356237, s2));
    } else if (force_type == 4u) {
        return exp(-d2 / (2.0 * max(e2, s2))) / (d_mag + sqrt(s2));
    } else {
        return exp(-d2 / (2.0 * max(e2, s2))) / max(d2 + s2, s2);
    }
}

@compute @workgroup_size(1)
fn presence_probe() {
    let count = u32(vp.surface.z);
    if (count == 0u) { 
        for (var i: u32 = 0u; i < 8u; i = i + 1u) { probe_out[i] = 0.0f; }
        return; 
    }
    let soft = vp.surface.w;
    
    var omegas: array<f32, 8>;
    for (var i: u32 = 0u; i < 8u; i = i + 1u) { omegas[i] = 0.0f; }

    for (var j = 0u; j < count; j = j + 1u) {
        let m = field[j];
        let mt = props[j];
        let d = m.xyz;
        let d2 = dot(d, d);
        let d_mag = sqrt(d2);
        let extent = mt.x;
        let retarded_dt = mt.y;
        let tau = mt.z;
        let force_type = u32(mt.w);
        
        let val_eff = m.w * exp(-retarded_dt / max(tau, 1e-9));
        let sk = field_spatial(d2, d_mag, extent, force_type, soft);
        
        if (force_type < 8u) {
            omegas[force_type] += val_eff * sk;
        }
    }
    
    for (var i: u32 = 0u; i < 8u; i = i + 1u) {
        probe_out[i] = omegas[i];
    }
}

@fragment fn fs(in: VOut) -> @location(0) vec4f {
    let dist = length(in.uv);
    if (dist > 1.0) { discard; }
    
    let intensity = exp(-dist * dist * 4.0);
    let noise = fract(sin(dot(in.pos.xy, vec2f(12.9898, 78.233))) * 43758.5453);
    let analog_intensity = intensity * (0.85 + noise * 0.15);
    
    return vec4f(in.color * analog_intensity, analog_intensity);
}
```

### SCHRITT 7
Suche in der Datei `static/index.html` nach dem Code-Block, der das `fieldLayout` erstellt.
Ersetze diesen Block durch (Vertex-Sichtbarkeit für Buffer 0 und 1):

```javascript
                fieldLayout = device.createBindGroupLayout({ entries: [
                    { binding: 0, visibility: GPUShaderStage.VERTEX | GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } },
                    { binding: 1, visibility: GPUShaderStage.VERTEX | GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } },
                    { binding: 2, visibility: GPUShaderStage.VERTEX | GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } },
                    { binding: 3, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'storage' } }
                ] });
```

### SCHRITT 8
Suche in der Datei `static/index.html` nach der Erstellung des `fieldPipe` Objekts.
Ersetze diesen Block durch (mit aktiviertem Additivem Blending):

```javascript
                fieldPipe = device.createRenderPipeline({
                    layout: device.createPipelineLayout({ bindGroupLayouts: [fieldLayout] }),
                    vertex: { module, entryPoint: 'vs' },
                    fragment: {
                        module, entryPoint: 'fs',
                        targets: [{
                            format,
                            blend: {
                                color: { srcFactor: 'one', dstFactor: 'one', operation: 'add' },
                                alpha: { srcFactor: 'one', dstFactor: 'one', operation: 'add' }
                            }
                        }]
                    },
                    primitive: { topology: 'triangle-list' }
                });
```

### SCHRITT 9
Suche in `static/index.html` nach der Erstellung des `probeBuf` und `probeRead`.
Ersetze die Größen von 16 auf 32 Bytes (für 8 f32 Werte):

```javascript
                probeBuf = device.createBuffer({ size: 32, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC });
                probeRead = device.createBuffer({ size: 32, usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST });
```

### SCHRITT 10
Ersetze die alten Skalar-Variablen. Suche nach der Zeile:

```javascript
        let probePending = false, probedOmega = 0, probedGradient = [0, 0, 0];
```

Ersetze sie durch:

```javascript
        let probePending = false;
        let probedOmegas = [0, 0, 0, 0, 0, 0, 0, 0];
        let probedGradient = [0, 0, 0]; 
```

### SCHRITT 11
Suche in der Funktion `probePresence()` nach der Zeile `await probeRead.mapAsync(GPUMapMode.READ);` und dem darauf folgenden Block.
Ersetze:

```javascript
                const data = new Float32Array(probeRead.getMappedRange());
                probedOmega = data[0];
                probedGradient = [data[1], data[2], data[3]];
                probeRead.unmap();
```

Durch:

```javascript
                const data = new Float32Array(probeRead.getMappedRange());
                probedOmegas = [data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]];
                probeRead.unmap();
```

### SCHRITT 12
Suche in der Funktion `manifestWindow` nach der Schleife, die die `xyzval` und `meta` Arrays füllt: `for (let i = 0; i < n; i++) { ... }`.
Finde darin diese Zeilen:

```javascript
                const retarded_dt = Math.max(0.0, Math.abs(fdt) - d / c_js);
                const val_eff = fv * Math.exp(-retarded_dt / Math.max(o ? o.ttl : 1.0, 1e-9));
                xyzval[i * 4] = dx;
                xyzval[i * 4 + 1] = dy;
                xyzval[i * 4 + 2] = dz;
                xyzval[i * 4 + 3] = val_eff;
                meta[i * 4] = fext;
                meta[i * 4 + 1] = 0;
                meta[i * 4 + 2] = 0;
                meta[i * 4 + 3] = fforce;
```

Ersetze sie durch (Übergabe von rohen Werten und `tau` an die GPU):

```javascript
                const retarded_dt = Math.max(0.0, Math.abs(fdt) - d / c_js);
                const tau = o ? o.tau : 1.0;
                xyzval[i * 4] = dx;
                xyzval[i * 4 + 1] = dy;
                xyzval[i * 4 + 2] = dz;
                xyzval[i * 4 + 3] = fv;
                meta[i * 4] = fext;
                meta[i * 4 + 1] = retarded_dt;
                meta[i * 4 + 2] = tau;
                meta[i * 4 + 3] = fforce;
```

### SCHRITT 13 (Nyquist-Relaxation entfernen)
Suche in der Funktion `manifestWindow(ts)` nach dem folgenden Block und **lösche diesen komplett**:

```javascript
            const medExt = windowMedianExtent();
            if (rawWindowResponse.length > 0 && medExt > 0 && isFinite(medExt) && prevSyncTs > 0 && (ts - lastManualZoomTs) > Φ * 1000) {
                const nyquistTarget = (medExt * 2) / Math.max(RX, RY);
                if (nyquistTarget > 0 && isFinite(nyquistTarget)) {
                    const dtFrame = (ts - prevSyncTs) / 1000;
                    const relax = 1.0 - Math.pow(2, -dtFrame / (Φ * 4));
                    gridStep += (nyquistTarget - gridStep) * relax;
                }
            }
```

### SCHRITT 14
Suche in der Datei `static/index.html` innerhalb der Funktion `manifestWindow` nach dem `if (!fieldVisible)` Block.
Ersetze das darin befindliche `pass.draw(3);` durch `pass.draw(0);`.

### SCHRITT 15
Suche weiter unten in der Funktion `manifestWindow` nach dem `try`-Block, der die Render-Pass-Befehle für das Feld ausführt.
Ersetze das darin befindliche `pass.draw(3);` durch `pass.draw(n * 6);`.

### SCHRITT 16
Suche in der `startAudio()` Funktion den Block `surfaces.push({ manifest: () => { ... } });`.
Ersetze den Inhalt der `manifest` Funktion durch:

```javascript
                    manifest: () => {
                        try {
                            const lum = Math.tanh(Math.abs(probedOmegas[2]) * Math.pow(windowMedianExtent(), 2));
                            audioFilter.frequency.setTargetAtTime(Math.max(audioCtx.sampleRate / 2048, audioCtx.sampleRate * lum / 16), audioCtx.currentTime, 0.05);
                            audioGain.gain.setTargetAtTime(lum * 0.5, audioCtx.currentTime, 0.05);
                        } catch {}
                    }
```

### SCHRITT 17
Suche den Block für `navigator.vibrate`.
Ersetze den Inhalt durch:

```javascript
                    manifest: () => {
                        const lum = Math.tanh(Math.abs(probedOmegas[4]) * Math.pow(windowMedianExtent(), 2));
                        const pulse = Math.floor(lum * stableTick) & 1023;
                        if (pulse === 0 && lum > 0) navigator.vibrate(Math.floor(lum * 255));
                    }
```

### SCHRITT 18
Suche die `surfaces.push` Blöcke für Serial, USB, Bluetooth und HID.
Ersetze in jedem dieser Blöcke die Zeile `let peak = 0;` und die alte `for`-Schleife, die `peak` berechnet hat, durch:

```javascript
                        let peak = 0;
                        for (let i = 0; i < 8; i++) peak = Math.max(peak, Math.abs(probedOmegas[i]));
                        if (peak <= 0) return;
```

### SCHRITT 19
Suche in der `ω(ts)` Funktion nach der Zeile `const omegaNow = probedOmega;`.
Ersetze sie durch:

```javascript
            const omegaNow = probedOmegas[0] + probedOmegas[1] + probedOmegas[2] + probedOmegas[3] + probedOmegas[4] + probedOmegas[5] + probedOmegas[6] + probedOmegas[7];
```

Suche weiter unten im Debug-Block nach der Zeile:

```javascript
                    `  Window omega: ${probedOmega.toExponential(2)} (exposure ${(exposureBoost > 0 ? '+' : '') + exposureBoost}dB)\n\n` +
```

Ersetze sie durch:

```javascript
                    `  Window omega:\n` +
                    `    EM:       ${probedOmegas[0].toExponential(2)}\n` +
                    `    Gravity:  ${probedOmegas[1].toExponential(2)}\n` +
                    `    Acoustic: ${probedOmegas[2].toExponential(2)}\n` +
                    `    Seismic:  ${probedOmegas[4].toExponential(2)}\n` +
                    `    Thermal:  ${probedOmegas[5].toExponential(2)}\n\n` +
```

---

## DATEI 3: phi/sources.φ

### SCHRITT 20
Ändere die Klassifizierung der Wind- und Strömungs-Quellen von `em` auf `advective`.

1. Suche den Block `source atmosphere_open_meteo_current`.
   Ändere die Zeile `force em` zu `force advective`.

2. Suche den Block `source hydrosphere_marine_ocean`.
   Ändere die Zeile `force em` zu `force advective`.

3. Suche den Block `source atmosphere_wind_heights`.
   Ändere die Zeile `force em` zu `force advective`.

4. Suche den Block `source atmosphere_jetstream_250hpa`.
   Ändere die Zeile `force em` zu `force advective`.

*(Gleiches gilt für jegliche anderen Quellen wie `noaa_global_forecast_system` oder `ocean_currents_oscar`, falls du sie hinzufügst. Alle Wind- und Wasserströmungs-Daten erhalten nun die korrekte Physik).*
