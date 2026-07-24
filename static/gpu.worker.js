const fieldShader = `
struct VP { presence: vec4f, surface: vec4f };
@group(0) @binding(0) var<storage, read> field: array<vec4f>;
@group(0) @binding(1) var<storage, read> aperture: array<f32>;
@group(0) @binding(2) var<uniform> vp: VP;
struct V { @builtin(position) p: vec4f, @location(0) u: vec2f };
@vertex fn vs(@builtin(vertex_index) i: u32) -> V {
    var p = array<vec2f, 3>(vec2f(-1.0, -1.0), vec2f(3.0, -1.0), vec2f(-1.0, 3.0));
    var o: V;
    o.p = vec4f(p[i], 0.0, 1.0);
    o.u = vec2f(p[i].x * 0.5 + 0.5, 0.5 - p[i].y * 0.5);
    return o;
}
@fragment fn fs(i: V) -> @location(0) vec4f {
    let count = u32(vp.surface.z);
    let scale = vp.presence.w;
    let w = vp.surface.x;
    let h = vp.surface.y;
    let phi = vec3f(
        vp.presence.x + (i.u.x - 0.5) * w * scale,
        vp.presence.y - (i.u.y - 0.5) * h * scale,
        vp.presence.z);
    var omega = 0.0f;
    for (var j = 0u; j < count; j = j + 1u) {
        let m = field[j];
        let d = m.xyz - phi;
        let a = aperture[j];
        omega = omega + m.w / (dot(d, d) + a * a);
    }
    let aw = abs(omega);
    if (aw < exp2(-64.0)) { discard; }
    let t2 = clamp((log2(aw) + 64.0) / 64.0, 0.0, 1.0);
    let c1 = mix(vec3f(0.0, 0.0, 0.0), vec3f(0.0, 0.3, 0.8), clamp(t2 * 4.0, 0.0, 1.0));
    let c2 = mix(c1, vec3f(0.2, 0.8, 1.0), clamp((t2 - 0.25) * 4.0, 0.0, 1.0));
    let c3 = mix(c2, vec3f(1.0, 0.7, 0.1), clamp((t2 - 0.5) * 4.0, 0.0, 1.0));
    let c4 = mix(c3, vec3f(1.0, 1.0, 1.0), clamp((t2 - 0.75) * 4.0, 0.0, 1.0));
    return vec4f(c4, 1.0);
}`;
let gpu = null;
let surface = null;
let fieldPipeline = null;
let fieldLayout = null;
let vpBuf = null;
let fieldBuf = null;
let apertureBuf = null;
let fieldBind = null;
let capacity = 0;
let canvas = null;
async function initField(offscreen) {
    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter) { postMessage({ type: 'lost' }); return; }
    gpu = await adapter.requestDevice();
    canvas = offscreen;
    gpu.lost.then(() => { gpu = null; postMessage({ type: 'lost' }); });
    surface = canvas.getContext('webgpu');
    const format = navigator.gpu.getPreferredCanvasFormat();
    surface.configure({ device: gpu, format, alphaMode: 'opaque' });
    const module = gpu.createShaderModule({ code: fieldShader });
    fieldLayout = gpu.createBindGroupLayout({ entries: [
        { binding: 0, visibility: GPUShaderStage.FRAGMENT, buffer: { type: 'read-only-storage' } },
        { binding: 1, visibility: GPUShaderStage.FRAGMENT, buffer: { type: 'read-only-storage' } },
        { binding: 2, visibility: GPUShaderStage.FRAGMENT, buffer: { type: 'uniform' } }
    ] });
    const pipelineLayout = gpu.createPipelineLayout({ bindGroupLayouts: [fieldLayout] });
    fieldPipeline = gpu.createRenderPipeline({
        layout: pipelineLayout,
        vertex: { module, entryPoint: 'vs' },
        fragment: { module, entryPoint: 'fs', targets: [{ format }] },
        primitive: { topology: 'triangle-list' }
    });
    vpBuf = gpu.createBuffer({ size: 32, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST });
    postMessage({ type: 'ready' });
}
function ensureCapacity(n) {
    if (capacity > 0 && n <= capacity) return;
    let c = 256;
    while (c < n) c <<= 1;
    capacity = c;
    if (fieldBuf) fieldBuf.destroy();
    if (apertureBuf) apertureBuf.destroy();
    fieldBuf = gpu.createBuffer({ size: capacity * 16, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST });
    apertureBuf = gpu.createBuffer({ size: capacity * 4, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST });
    fieldBind = gpu.createBindGroup({ layout: fieldLayout, entries: [
        { binding: 0, resource: { buffer: fieldBuf } },
        { binding: 1, resource: { buffer: apertureBuf } },
        { binding: 2, resource: { buffer: vpBuf } }
    ] });
}
let gpuBusy = false;
function evaluateField(d) {
    if (!gpu || !surface || !fieldBind) return;
    const count = Math.min(d.count, capacity);
    if (canvas.width !== d.w || canvas.height !== d.h) { canvas.width = d.w; canvas.height = d.h; }
    gpu.queue.writeBuffer(fieldBuf, 0, new Float32Array(d.xyzval, 0, count * 4));
    gpu.queue.writeBuffer(apertureBuf, 0, new Float32Array(d.ap, 0, count));
    gpu.queue.writeBuffer(vpBuf, 0, new Float32Array([d.cx, d.cy, d.cz, d.step, d.w, d.h, count, 0]));
    const encoder = gpu.createCommandEncoder();
    const pass = encoder.beginRenderPass({ colorAttachments: [{ view: surface.getCurrentTexture().createView(), clearValue: { r: 0, g: 0, b: 0, a: 1 }, loadOp: 'clear', storeOp: 'store' }] });
    pass.setPipeline(fieldPipeline);
    pass.setBindGroup(0, fieldBind);
    pass.draw(3);
    pass.end();
    gpu.queue.submit([encoder.finish()]);
    gpuBusy = true;
    gpu.queue.onSubmittedWorkDone().then(() => { gpuBusy = false; drainFrames(); });
}
let evaluating = false;
let pendingFrame = null;
function drainFrames() {
    if (evaluating) return;
    evaluating = true;
    while (pendingFrame && !gpuBusy) {
        const f = pendingFrame;
        pendingFrame = null;
        if (gpu) {
            try {
                ensureCapacity(f.count);
                evaluateField(f);
            } catch (err) {
                gpu = null;
                postMessage({ type: 'lost' });
            }
        } else {
            pendingFrame = null;
        }
    }
    evaluating = false;
}
self.onmessage = async (e) => {
    const d = e.data;
    if (d.type === 'init') { await initField(d.canvas); return; }
    if (d.type !== 'frame') return;
    pendingFrame = d;
    drainFrames();
};
