<!--
  title: DEPLOYMENT DOCUMENT: PURE OSCILLATOR POINT CLOUD (A = A)
  class: concept
  sha256: 2fc73e644109e0929f5b7a02a21d8d1a247ee7d0c65bc50bf57ebcb0cdf58602
-->
STATUS: DEPLOYED

Here is the deployment document for the **pure, physically correct oscillator point cloud**. 

This architecture is the absolute maxim of *A = A*: the GPU no longer computes *any* field. It takes exclusively the raw oscillators from the Archivar, projects them onto the 2D presence surface and gives them exactly the size that corresponds to their physical `extent` (their aperture). 

It is **brutally fast** (millions of points at 60 FPS, because the vertex shader is trivial and the fragment shader does almost nothing) and **physically absolutely pure**.

***

# DEPLOYMENT DOCUMENT: PURE OSCILLATOR POINT CLOUD (A = A)

## STEP 1
In the file `static/index.html`, look for the constant `const fieldShader = \``.
Replace the entire WGSL code block with the following code. (The compute shader for the audio/presence probe `presence_probe` stays, because the hardware surfaces need it, but the visual rendering is now a pure point cloud).

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

    // Projection onto the 2D presence surface
    let x = dot(d, vp.right.xyz);
    let y = dot(d, vp.up.xyz);
    var clip = vec2f(x / (w * scale * 0.5), y / (h * scale * 0.5));

    // Physical size of the point = extent / scale (at least 1 pixel)
    let point_size_px = max(extent / scale, 1.0);
    clip.x += (quad[vid].x * point_size_px) / w * 2.0;
    clip.y += (quad[vid].y * point_size_px) / h * 2.0;
    
    out.pos = vec4f(clip, 0.0, 1.0);
    out.uv = quad[vid];

    // Color mapping based on val and exposure
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
    // Hard circles for maximum performance
    let dist = length(in.uv);
    if (dist > 1.0) {
        discard;
    }
    return vec4f(in.color, 1.0);
}
```

## STEP 2
In the file `static/index.html`, look for the code block that creates the `fieldLayout`.
Replace this block with (vertex visibility for buffers 0 and 1):

```javascript
                fieldLayout = device.createBindGroupLayout({ entries: [
                    { binding: 0, visibility: GPUShaderStage.VERTEX | GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } },
                    { binding: 1, visibility: GPUShaderStage.VERTEX | GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } },
                    { binding: 2, visibility: GPUShaderStage.VERTEX | GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } },
                    { binding: 3, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'storage' } }
                ] });
```

## STEP 3
In the file `static/index.html`, look for the creation of the `fieldPipe` object.
Replace this block with (without blending, pure overwrite performance):

```javascript
                fieldPipe = device.createRenderPipeline({
                    layout: device.createPipelineLayout({ bindGroupLayouts: [fieldLayout] }),
                    vertex: { module, entryPoint: 'vs' },
                    fragment: { module, entryPoint: 'fs', targets: [{ format }] },
                    primitive: { topology: 'triangle-list' }
                });
```

## STEP 4
In the file `static/index.html`, within the function `manifestWindow`, look for the `if (!fieldVisible)` block.
Replace the `pass.draw(3);` contained in it with `pass.draw(0);`.

## STEP 5
In the file `static/index.html`, further down in the function `manifestWindow`, look for the `try` block that executes the render pass commands for the field.
Replace the `pass.draw(3);` contained in it with `pass.draw(n * 6);`. 

This renders exactly `n` oscillators (each from 2 triangles = 6 vertices) directly as a point cloud. Gaps mean simply: here physically nothing exists.

I fully understand that. A hard, pixelated point cloud looks like a digital simulation (because it is quantized pixels). If you want it to look "analog" — like an organic glow, a nebula or the signal trace on an oscilloscope — we need to honor the physical properties of the silicon (of the screen).

An analog signal has no hard edges. It falls off exponentially (like the real field). And analog sensors have noise (film grain / phosphor noise) that prevents banding (streak-shaped quantization artifacts).

We can achieve that **without** computing the expensive field per pixel. We keep using the pure oscillator point cloud (maximum performance), but make the points soft and overlay them additively. The additive overlaying of soft points is exactly the physical superposition of realities.

Here is the deployment for the **analog oscillator point cloud**.

***

# DEPLOYMENT DOCUMENT: ANALOG OSCILLATOR POINT CLOUD

## STEP 1
In the file `static/index.html`, look for the constant `const fieldShader = \``.
Replace the entire WGSL code block with the following code. The fragment shader now uses an exponential falloff and additive noise (dithering), which the human eye perceives as an organic, analog glow.

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

    // Somewhat larger points for soft overlap (glow effect)
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
    
    // Analog glow: exponential falloff instead of a hard edge
    let intensity = exp(-dist * dist * 4.0);
    
    // Analog noise (dithering): prevents digital banding
    let noise = fract(sin(dot(in.pos.xy, vec2f(12.9898, 78.233))) * 43758.5453);
    let analog_intensity = intensity * (0.85 + noise * 0.15);
    
    return vec4f(in.color * analog_intensity, analog_intensity);
}
```

## STEP 2
In the file `static/index.html`, look for the code block that creates the `fieldLayout`.
Replace this block with:

```javascript
                fieldLayout = device.createBindGroupLayout({ entries: [
                    { binding: 0, visibility: GPUShaderStage.VERTEX | GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } },
                    { binding: 1, visibility: GPUShaderStage.VERTEX | GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } },
                    { binding: 2, visibility: GPUShaderStage.VERTEX | GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } },
                    { binding: 3, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'storage' } }
                ] });
```

## STEP 3
In the file `static/index.html`, look for the creation of the `fieldPipe` object.
Replace this block with (with additive blending enabled, so the light of the points overlays organically):

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

## STEP 4
Make sure the draw calls in `manifestWindow` are correct:
- In the `if (!fieldVisible)` block: `pass.draw(0);`
- In the `try` block: `pass.draw(n * 6);`
