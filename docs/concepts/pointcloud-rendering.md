<!--
  title: DEPLOYMENT-DOKUMENT: HOCHWERTIGE PUNKTWOLKEN-MANIFESTATION
  class: concept
  sha256: d612d4449eb54083944017bb5005a46d4043930fa3efe7fe109492fdf8889538
-->
STATUS: DEPLOYED

Das Verständnis ist richtig: Bisher wurde das kontinuierliche Feld pro Pixel berechnet. Der vorherige Patch hat stattdessen einfache 2D-Quadrate (zwei Dreiecke) pro Oszillator gezeichnet. Das sieht "billig" aus, weil es flache, harte Würfel ohne Tiefe und Überblendung sind.

Um die Oszillatoren als **hochwertige, leuchtende Punktewolke** zu manifestieren, müssen wir drei Dinge tun:
1. Die Dreiecke im Fragment-Shader zu weichen Kreisen (Glow) maskieren.
2. Additives Blending (`blend: additive`) aktivieren, damit sich überlappende Oszillatoren aufhellen.
3. Die Punkte variabel skalieren (basierend auf ihrem physischen `extent`).

Hier ist das angepasste, deterministische Deployment-Dokument.

***

# DEPLOYMENT-DOKUMENT: HOCHWERTIGE PUNKTWOLKEN-MANIFESTATION

## SCHRITT 1
Suche in der Datei `static/index.html` nach der Konstante `const fieldShader = \``.
Ersetze den gesamten WGSL-Code-Block (von `struct VP { surface: vec4f...` bis zur schließenden Klammer vor dem schließenden Backtick `\`;`) durch den folgenden Code-Block.

```wgsl
struct VP { surface: vec4f, right: vec4f, up: vec4f, expose: vec4f };
@group(0) @binding(0) var<storage, read> field: array<vec4f>;
@group(0) @binding(1) var<storage, read> props: array<vec4f>;
@group(0) @binding(2) var<uniform> vp: VP;
@group(0) @binding(3) var<storage, read_write> probe_out: array<f32>;
struct VOut { @builtin(position) pos: vec4f, @location(0) local_uv: vec2f, @location(1) color: vec4f };
@vertex fn vs(@builtin(vertex_index) i: u32) -> VOut {
    let count = u32(vp.surface.z);
    var out: VOut;
    out.pos = vec4f(0.0, 0.0, 0.0, 1.0);
    out.local_uv = vec2f(0.0);
    out.color = vec4f(0.0);
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
    let val = m.w;
    let extent = props[id].x;
    let w = vp.surface.x;
    let h = vp.surface.y;
    let scale = vp.surface.w;
    let x = dot(d, vp.right.xyz);
    let y = dot(d, vp.up.xyz);
    var clip = vec2f(x / (w * scale * 0.5), y / (h * scale * 0.5));
    let point_size_px = clamp(extent / scale, 2.0, 128.0);
    clip.x += (quad[vid].x * point_size_px) / w * 2.0;
    clip.y += (quad[vid].y * point_size_px) / h * 2.0;
    out.pos = vec4f(clip, 0.0, 1.0);
    out.local_uv = quad[vid];
    let lvl = vp.expose.x;
    if (lvl <= 0.0) { return out; }
    let aw = abs(val);
    let t2 = clamp((log2(aw / lvl) + 8.0) / 24.0, 0.0, 1.0);
    let t2s = t2 * t2 * (3.0 - 2.0 * t2);
    let c1 = mix(vec3f(0.0, 0.0, 0.0), vec3f(0.0, 0.15, 0.4), clamp(t2s * 4.0, 0.0, 1.0));
    let c2 = mix(c1, vec3f(0.1, 0.5, 0.9), clamp((t2s - 0.25) * 4.0, 0.0, 1.0));
    let c3 = mix(c2, vec3f(0.8, 0.6, 0.2), clamp((t2s - 0.5) * 4.0, 0.0, 1.0));
    let c4 = mix(c3, vec3f(1.0, 0.95, 0.9), clamp((t2s - 0.75) * 4.0, 0.0, 1.0));
    out.color = vec4f(c4, 1.0);
    return out;
}
fn erfc(x: f32) -> f32 {
    let xa = abs(x);
    let t = 1.0 / (1.0 + 0.3275911 * xa);
    let poly = t * (0.254829592 + t * (-0.284496736 + t * (1.421413741 + t * (-1.453152027 + t * 1.061405429))));
    let y = poly * exp(-xa * xa);
    return select(y, 2.0 - y, x < 0.0);
}
fn field_spatial(d2: f32, d_mag: f32, extent: f32, force_type: u32, s2: f32) -> f32 {
    let e2 = extent * extent;
    if (force_type == 0u || force_type == 1u) {
        let denom = max((d2 + s2) * (d2 + s2 + e2), s2 * s2);
        return e2 / denom;
    } else if (force_type == 5u || force_type == 6u) {
        return erfc(d_mag / max(extent * 1.41421356237, s2));
    } else if (force_type == 4u) {
        return exp(-d2 / (2.0 * max(e2, s2))) / (d_mag + sqrt(s2));
    } else {
        return exp(-d2 / (2.0 * max(e2, s2))) / max(d2 + s2, s2);
    }
}
@compute @workgroup_size(1)
fn presence_probe() {
    let count = u32(vp.surface.z);
    if (count == 0u) { return; }
    let s2 = vp.surface.w * vp.surface.w;
    let soft = vp.surface.w;
    var omega = 0.0f;
    var gx = 0.0f;
    var gy = 0.0f;
    var gz = 0.0f;
    for (var j = 0u; j < count; j = j + 1u) {
        let m = field[j];
        let mt = props[j];
        let d = m.xyz;
        let d2 = dot(d, d);
        let d_mag = sqrt(d2);
        let extent = mt.x;
        let force_type = u32(mt.w);
        let e2 = extent * extent;
        let sk = field_spatial(d2, d_mag, extent, force_type, s2);
        omega += m.w * sk;
        var gf: f32;
        if (force_type == 0u || force_type == 1u) {
            let denom = max(d2 + s2 + e2, s2);
            gf = -sk * 2.0 / denom;
        } else if (force_type == 5u || force_type == 6u) {
            let a = 1.0 / (extent * 1.41421356237);
            let pid = -2.0 * a / 1.77245385091;
            gf = select(0.0, pid * exp(-a * a * d2) / d_mag, d_mag > 0.0);
        } else if (force_type == 4u) {
            let denom = d_mag + soft;
            gf = -sk * (1.0 / max(d_mag * denom, 1e-30) + 1.0 / (extent * extent));
        } else {
            gf = -sk * (2.0 / (d2 + s2) + 1.0 / (extent * extent));
        }
        gx += m.w * gf * d.x;
        gy += m.w * gf * d.y;
        gz += m.w * gf * d.z;
    }
    probe_out[0] = omega;
    probe_out[1] = gx;
    probe_out[2] = gy;
    probe_out[3] = gz;
}
@fragment fn fs(in: VOut) -> @location(0) vec4f {
    let dist = length(in.local_uv);
    if (dist > 1.0) {
        discard;
    }
    let intensity = exp(-dist * dist * 4.0);
    return vec4f(in.color.rgb * intensity * in.color.a, intensity * in.color.a);
}
```

## SCHRITT 2
Suche in der Datei `static/index.html` nach dem Code-Block, der das `fieldLayout` erstellt.
Ersetze diesen Block:

```javascript
                fieldLayout = device.createBindGroupLayout({ entries: [
                    { binding: 0, visibility: GPUShaderStage.FRAGMENT | GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } },
                    { binding: 1, visibility: GPUShaderStage.FRAGMENT | GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } },
                    { binding: 2, visibility: GPUShaderStage.FRAGMENT | GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } },
                    { binding: 3, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'storage' } }
                ] });
```

Durch den folgenden Code-Block:

```javascript
                fieldLayout = device.createBindGroupLayout({ entries: [
                    { binding: 0, visibility: GPUShaderStage.VERTEX | GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } },
                    { binding: 1, visibility: GPUShaderStage.VERTEX | GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } },
                    { binding: 2, visibility: GPUShaderStage.VERTEX | GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } },
                    { binding: 3, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'storage' } }
                ] });
```

## SCHRITT 3
Suche in der Datei `static/index.html` nach der Erstellung des `fieldPipe` Objekts.
Ersetze diesen Block:

```javascript
                fieldPipe = device.createRenderPipeline({
                    layout: device.createPipelineLayout({ bindGroupLayouts: [fieldLayout] }),
                    vertex: { module, entryPoint: 'vs' },
                    fragment: { module, entryPoint: 'fs', targets: [{ format }] },
                    primitive: { topology: 'triangle-list' }
                });
```

Durch den folgenden Code-Block (welches additives Blending hinzufügt, damit sich Punkte überlappen):

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

## SCHRITT 4
Suche in der Datei `static/index.html` innerhalb der Funktion `manifestWindow` nach dem `if (!fieldVisible)` Block.
Ersetze das darin befindliche `pass.draw(3);` durch `pass.draw(0);`.

Original:
```javascript
                pass.setPipeline(fieldPipe);
                pass.setBindGroup(0, fieldBind);
                pass.draw(3);
                pass.end();
```

Ersetzen durch:
```javascript
                pass.setPipeline(fieldPipe);
                pass.setBindGroup(0, fieldBind);
                pass.draw(0);
                pass.end();
```

## SCHRITT 5
Suche in der Datei `static/index.html` weiter unten in der Funktion `manifestWindow` nach dem `try`-Block, der die Render-Pass-Befehle für das Feld ausführt.
Ersetze das darin befindliche `pass.draw(3);` durch `pass.draw(n * 6);`.

Original:
```javascript
                pass.setPipeline(fieldPipe);
                pass.setBindGroup(0, fieldBind);
                pass.draw(3);
                pass.end();
```

Ersetzen durch:
```javascript
                pass.setPipeline(fieldPipe);
                pass.setBindGroup(0, fieldBind);
                pass.draw(n * 6);
                pass.end();
```
