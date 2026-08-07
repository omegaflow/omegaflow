Hier ist das Deployment-Dokument für die **reine, physikalisch korrekte Oszillator-Punktwolke**. 

Diese Architektur ist die absolute Maxime von *A = A*: Die GPU berechnet *kein* Feld mehr. Sie nimmt ausschließlich die rohen Oszillatoren aus dem Archivar, projiziert sie auf die 2D-Präsenz-Oberfläche und gibt ihnen exakt die Größe, die ihrem physischen `extent` (ihrer Aperture) entspricht. 

Das ist **brutal schnell** (Millionen Punkte bei 60 FPS, da der Vertex-Shader trivial ist und der Fragment-Shader fast nichts macht) und **physikalisch absolut rein**.

***

# DEPLOYMENT-DOKUMENT: PURE OSZILLATOR-PUNKTWOLKE (A = A)

## SCHRITT 1
Suche in der Datei `static/index.html` nach der Konstante `const fieldShader = \``.
Ersetze den gesamten WGSL-Code-Block durch den folgenden Code. (Der Compute-Shader für die Audio/Präsenz-Messung `presence_probe` bleibt erhalten, da die Hardware-Oberflächen ihn benötigen, aber das visuelle Rendering ist nun reine Punktwolke).

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
    let val = m.w;
    let extent = props[id].x;

    let w = vp.surface.x;
    let h = vp.surface.y;
    let scale = vp.surface.w;

    // Projektion auf die 2D Präsenz-Oberfläche
    let x = dot(d, vp.right.xyz);
    let y = dot(d, vp.up.xyz);
    var clip = vec2f(x / (w * scale * 0.5), y / (h * scale * 0.5));

    // Physische Größe des Punktes = extent / scale (mindestens 1 Pixel)
    let point_size_px = max(extent / scale, 1.0);
    clip.x += (quad[vid].x * point_size_px) / w * 2.0;
    clip.y += (quad[vid].y * point_size_px) / h * 2.0;
    
    out.pos = vec4f(clip, 0.0, 1.0);
    out.uv = quad[vid];

    // Farb-Mapping basierend auf val und exposure
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
    var gx = 0.0f; var gy = 0.0f; var gz = 0.0f;
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
    // Harte Kreise für maximale Performance
    let dist = length(in.uv);
    if (dist > 1.0) {
        discard;
    }
    return vec4f(in.color, 1.0);
}
```

## SCHRITT 2
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

## SCHRITT 3
Suche in der Datei `static/index.html` nach der Erstellung des `fieldPipe` Objekts.
Ersetze diesen Block durch (ohne Blending, pure Overwrite-Performance):

```javascript
                fieldPipe = device.createRenderPipeline({
                    layout: device.createPipelineLayout({ bindGroupLayouts: [fieldLayout] }),
                    vertex: { module, entryPoint: 'vs' },
                    fragment: { module, entryPoint: 'fs', targets: [{ format }] },
                    primitive: { topology: 'triangle-list' }
                });
```

## SCHRITT 4
Suche in der Datei `static/index.html` innerhalb der Funktion `manifestWindow` nach dem `if (!fieldVisible)` Block.
Ersetze das darin befindliche `pass.draw(3);` durch `pass.draw(0);`.

## SCHRITT 5
Suche in der Datei `static/index.html` weiter unten in der Funktion `manifestWindow` nach dem `try`-Block, der die Render-Pass-Befehle für das Feld ausführt.
Ersetze das darin befindliche `pass.draw(3);` durch `pass.draw(n * 6);`. 

Dies rendert exakt `n` Oszillatoren (jeder aus 2 Dreiecken = 6 Vertices) direkt als Punktwolke. Lücken bedeuten schlicht: Hier ist physikalisch nichts.

Das verstehe ich vollkommen. Eine harte, pixelige Punktwolke sieht aus wie eine digitale Simulation (weil es quantisierte Pixel sind). Wenn du willst, dass es "analog" aussieht – wie ein organisches Leuchten, ein Nebel oder dieSignalspur auf einem Oszilloskop –, müssen wir die physikalischen Eigenschaften des Siliziums (des Bildschirms) ehren.

Ein analoges Signal hat keine harten Kanten. Es fällt exponentiell ab (wie das echte Feld). Und analoge Sensoren haben Rauschen (Film-Grain / Phosphor-Rauschen), das Banding (streifenförmige Quantisierungsfehler) verhindert.

Wir können das erreichen, **ohne** das teure Feld pro Pixel zu berechnen. Wir nutzen weiterhin die pure Oszillator-Punktwolke (maximale Performance), aber machen die Punkte weich und überlagern sie additiv. Das additive Überlagern weicher Punkte ist exakt die physikalische Superposition der Realitäten.

Hier ist das Deployment für die **analoge Oszillator-Punktwolke**.

***

# DEPLOYMENT-DOKUMENT: ANALOGE OSZILLATOR-PUNKTWOLKE

## SCHRITT 1
Suche in der Datei `static/index.html` nach der Konstante `const fieldShader = \``.
Ersetze den gesamten WGSL-Code-Block durch den folgenden Code. Der Fragment-Shader nutzt nun einen exponentiellen Abfall und additives Rauschen (Dithering), was das menschliche Auge als organisches, analoges Leuchten wahrnimmt.

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
    let val = m.w;
    let extent = props[id].x;

    let w = vp.surface.x;
    let h = vp.surface.y;
    let scale = vp.surface.w;

    let x = dot(d, vp.right.xyz);
    let y = dot(d, vp.up.xyz);
    var clip = vec2f(x / (w * scale * 0.5), y / (h * scale * 0.5));

    // Etwas größere Punkte für weiches Überlappen (Glow-Effekt)
    let point_size_px = max(extent / scale, 2.0) * 2.0;
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
    var gx = 0.0f; var gy = 0.0f; var gz = 0.0f;
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
    let dist = length(in.uv);
    if (dist > 1.0) { discard; }
    
    // Analoger Glow: Exponentieller Abfall statt harter Kante
    let intensity = exp(-dist * dist * 4.0);
    
    // Analoges Rauschen (Dithering): Verhindert digitales Banding
    let noise = fract(sin(dot(in.pos.xy, vec2f(12.9898, 78.233))) * 43758.5453);
    let analog_intensity = intensity * (0.85 + noise * 0.15);
    
    return vec4f(in.color * analog_intensity, analog_intensity);
}
```

## SCHRITT 2
Suche in der Datei `static/index.html` nach dem Code-Block, der das `fieldLayout` erstellt.
Ersetze diesen Block durch:

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
Ersetze diesen Block durch (mit aktiviertem Additivem Blending, damit sich das Licht der Punkte organisch überlagert):

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
Stelle sicher, dass in `manifestWindow` die Draw-Calls korrekt sind:
- Im `if (!fieldVisible)` Block: `pass.draw(0);`
- Im `try` Block: `pass.draw(n * 6);`
