const permutationEntropyShader = `@group(0) @binding(0) var<storage, read> data: array<f32>; @group(0) @binding(1) var<storage, read_write> complexity: array<f32>; @group(0) @binding(2) var<uniform> params: vec4<u32>; fn perm3(a: f32, b: f32, c: f32) -> u32 { if (a <= b && b <= c) { return 0u; } if (a <= c && c <= b) { return 1u; } if (b <= a && a <= c) { return 2u; } if (b <= c && c <= a) { return 3u; } if (c <= a && a <= b) { return 4u; } return 5u; } @compute @workgroup_size(64) fn main(@builtin(global_invocation_id) gid: vec3<u32>) { let s = gid.x; let n = u32(params.x); let rs = u32(params.y); if (s >= n) { return; } let base = s * rs; var counts: array<f32, 6>; for (var k = 0u; k < 6u; k = k + 1u) { counts[k] = 0.0; } let num_windows = rs - 2u; for (var i = 0u; i < num_windows; i = i + 1u) { let p = perm3(data[base + i], data[base + i + 1u], data[base + i + 2u]); counts[p] = counts[p] + 1.0; } let total = f32(num_windows); var pe: f32 = 0.0; for (var k = 0u; k < 6u; k = k + 1u) { if (counts[k] > 0.0) { let pk = counts[k] / total; pe = pe - pk * log2(pk); } } complexity[s] = pe / log2(6.0); }`;
const takensShader = `const PHI = f32(1.618033988749895); @group(0) @binding(0) var<storage, read> data: array<f32>; @group(0) @binding(1) var<storage, read_write> out: array<f32>; @group(0) @binding(2) var<uniform> params: vec4<u32>; @compute @workgroup_size(64) fn main(@builtin(global_invocation_id) gid: vec3<u32>) { let s = gid.x; let n = u32(params.x); let rs = u32(params.y); if (s >= n) { return; } let base = s * rs; var sum: f32 = 0.0; for (var i = 0u; i < rs; i++) { sum += data[base + i]; } let mean = sum / f32(rs); var best_tau: u32 = 1u; var mi_prev: f32 = data[base]; var mi_prev2: f32 = data[base]; let max_lag = u32(f32(rs) / PHI); for (var lag = 1u; lag <= max_lag; lag++) { var mn: f32 = data[base]; var mx: f32 = data[base]; for (var i = 0u; i < rs - lag; i++) { mn = min(mn, data[base + i]); mx = max(mx, data[base + i]); } let range = mx - mn + 1e-16; var h00: f32 = 0.0; var h01: f32 = 0.0; var h10: f32 = 0.0; var h11: f32 = 0.0; for (var i = 0u; i < rs - lag; i++) { let b1 = select(0u, 1u, data[base + i] > mn + range * 0.5); let b2 = select(0u, 1u, data[base + i + lag] > mn + range * 0.5); if (b1 == 0u && b2 == 0u) { h00 += 1.0; } else if (b1 == 0u && b2 == 1u) { h01 += 1.0; } else if (b1 == 1u && b2 == 0u) { h10 += 1.0; } else { h11 += 1.0; } } let total = f32(rs - lag); let p0 = (h00 + h01) / total; let p1 = (h10 + h11) / total; let q0 = (h00 + h10) / total; let q1 = (h01 + h11) / total; var mi: f32 = 0.0; let eps = 1e-16; if (h00 > 0.0) { mi += (h00/total) * log2((h00/total) / (p0*q0 + eps) + eps); } if (h01 > 0.0) { mi += (h01/total) * log2((h01/total) / (p0*q1 + eps) + eps); } if (h10 > 0.0) { mi += (h10/total) * log2((h10/total) / (p1*q0 + eps) + eps); } if (h11 > 0.0) { mi += (h11/total) * log2((h11/total) / (p1*q1 + eps) + eps); } if (lag >= 3u && mi_prev2 > mi_prev && mi_prev <= mi) { best_tau = lag - 1u; break; } mi_prev2 = mi_prev; mi_prev = mi; } if (2u * best_tau >= rs) { best_tau = 1u; } let max_pts = rs - 2u * best_tau; let out_base = s * 4u; var cx: f32 = 0.0; var cy: f32 = 0.0; var cz: f32 = 0.0; for (var i = 0u; i < max_pts; i++) { cx += data[base + i]; cy += data[base + i + best_tau]; cz += data[base + i + 2u * best_tau]; } cx = cx / f32(max_pts); cy = cy / f32(max_pts); cz = cz / f32(max_pts); var spread: f32 = 0.0; for (var i = 0u; i < max_pts; i++) { let dx = data[base + i] - cx; let dy = data[base + i + best_tau] - cy; let dz = data[base + i + 2u * best_tau] - cz; spread += sqrt(dx*dx + dy*dy + dz*dz); } spread = spread / f32(max_pts); out[out_base] = cx; out[out_base + 1u] = cy; out[out_base + 2u] = cz; out[out_base + 3u] = spread; }`;
const kurtosisShader = `@group(0) @binding(0) var<storage, read> data: array<f32>; @group(0) @binding(1) var<storage, read_write> out: array<f32>; @group(0) @binding(2) var<uniform> params: vec4<u32>; @compute @workgroup_size(64) fn main(@builtin(global_invocation_id) gid: vec3<u32>) { let s = gid.x; let n = u32(params.x); let rs = u32(params.y); if (s >= n) { return; } let base = s * rs; var mean: f32 = 0.0; for (var i = 0u; i < rs; i = i + 1u) { mean = mean + data[base + i]; } mean = mean / f32(rs); var m2: f32 = 0.0; var m4: f32 = 0.0; for (var i = 0u; i < rs; i = i + 1u) { let d = data[base + i] - mean; let d2 = d * d; m2 = m2 + d2; m4 = m4 + d2 * d2; } m2 = m2 / f32(rs); m4 = m4 / f32(rs); let variance = max(m2, 1e-16); out[s] = abs((m4 / (variance * variance)) - 3.0); }`;
const tdaShader = `const PHI = f32(1.618033988749895); @group(0) @binding(0) var<storage, read> data: array<f32>; @group(0) @binding(1) var<storage, read_write> out: array<f32>; @group(0) @binding(2) var<uniform> params: vec4<u32>; @compute @workgroup_size(64) fn main(@builtin(global_invocation_id) gid: vec3<u32>) { let s = gid.x; let n = u32(params.x); let rs = u32(params.y); if (s >= n) { return; } let base = s * rs; let sub = min(48u, u32(f32(rs) / 2.618033988749895)); let tau = u32(1.0 + 1.0 / PHI); var dists: array<f32, 48>; for (var i = 0u; i < sub; i++) { var min_d: f32 = 3.4e38; let p1 = base + i * tau; for (var j = 0u; j < sub; j++) { if (i == j) { continue; } let p2 = base + j * tau; let d = abs(data[p1] - data[p2]); if (d < min_d) { min_d = d; } } dists[i] = min_d; } for (var i = 1u; i < sub; i++) { let key = dists[i]; var j = i; while (j > 0u && dists[j - 1u] > key) { dists[j] = dists[j - 1u]; j = j - 1u; } dists[j] = key; } var life_sum: f32 = 0.0; var prev_d: f32 = 0.0; var comps: f32 = f32(sub); for (var i = 0u; i < sub; i++) { let d = dists[i]; life_sum += (d - prev_d) * comps; comps -= 1.0; prev_d = d; } var mean_d: f32 = 0.0; for (var i = 0u; i < sub; i++) { mean_d += dists[i]; } mean_d = mean_d / f32(sub); var betti0: f32 = 0.0; var above = false; for (var i = 0u; i < sub; i++) { if (dists[i] > mean_d && !above) { betti0 += 1.0; above = true; } else if (dists[i] <= mean_d && above) { above = false; } } let out_base = s * 2u; out[out_base] = life_sum / f32(sub); out[out_base + 1u] = betti0; }`;
const teShader = `
struct TeParams { srcW: u32, dstR: u32, rs: u32, _pad: u32 };
@group(0) @binding(0) var<storage, read> data: array<f32>;
@group(0) @binding(1) var<storage, read> srcIdx: array<u32>;
@group(0) @binding(2) var<storage, read> dstIdx: array<u32>;
@group(0) @binding(3) var<storage, read_write> teOut: array<f32>;
@group(0) @binding(4) var<uniform> params: TeParams;
fn gauss(d2: f32, sigma: f32) -> f32 { return exp(-0.5 * d2 / (sigma * sigma)); }
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x; let j = gid.y;
    let srcW = params.srcW; let dstR = params.dstR; let rs = params.rs;
    if (i >= srcW || j >= dstR) { return; }
    if (rs < 4u) { teOut[i * dstR + j] = 0.0; return; }
    let baseA = srcIdx[i] * rs;
    let baseB = dstIdx[j] * rs;
    var sumS: f32 = 0.0; var sumT: f32 = 0.0;
    for (var k = 0u; k < rs; k = k + 1u) { sumS = sumS + data[baseA + k]; sumT = sumT + data[baseB + k]; }
    let meanS = sumS / f32(rs); let meanT = sumT / f32(rs);
    var varS: f32 = 0.0; var varT: f32 = 0.0;
    for (var k = 0u; k < rs; k = k + 1u) { let dS = data[baseA + k] - meanS; let dT = data[baseB + k] - meanT; varS = varS + dS * dS; varT = varT + dT * dT; }
    let eps = 1.1920928955078125e-07;
    let sigmaS = max(sqrt(varS / f32(rs)), eps);
    let sigmaT = max(sqrt(varT / f32(rs)), eps);
    let sigmaSt = (sigmaS + sigmaT) * 0.5;
    let rsM1 = rs - 1u;
    var teVal: f32 = 0.0;
    for (var p = 0u; p < rsM1; p = p + 1u) {
        let sT = data[baseA + p]; let tT = data[baseB + p]; let tT1 = data[baseB + p + 1u];
        var kJoint: f32 = 0.0; var kCond: f32 = 0.0; var kT: f32 = 0.0; var kT1: f32 = 0.0;
        for (var q = 0u; q < rsM1; q = q + 1u) {
            let sTq = data[baseA + q]; let tTq = data[baseB + q]; let tTq1 = data[baseB + q + 1u];
            let dj2 = (sT - sTq) * (sT - sTq) + (tT - tTq) * (tT - tTq) + (tT1 - tTq1) * (tT1 - tTq1);
            kJoint = kJoint + gauss(dj2, sigmaSt);
            let dc2 = (sT - sTq) * (sT - sTq) + (tT - tTq) * (tT - tTq);
            kCond = kCond + gauss(dc2, sigmaSt);
            let dt2 = (tT - tTq) * (tT - tTq);
            kT = kT + gauss(dt2, sigmaT);
            let dt12 = (tT - tTq) * (tT - tTq) + (tT1 - tTq1) * (tT1 - tTq1);
            kT1 = kT1 + gauss(dt12, sigmaT);
        }
        if (kJoint > 0.0 && kCond > 0.0 && kT > 0.0 && kT1 > 0.0) {
            let pJ = kJoint / f32(rs); let pC = kCond / f32(rs); let pT = kT / f32(rs); let pT1 = kT1 / f32(rs);
            teVal = teVal + pJ * log2((pJ * pT) / (pC * pT1) + 1.1920928955078125e-07);
        }
    }
    let result = teVal / f32(rsM1);
    teOut[i * dstR + j] = select(0.0, result, result > 0.0);
}`;
let gpu = null;
let permutationEntropyPipeline = null, takensPipeline = null, tdaPipeline = null, kurtosisPipeline = null, tePipeline = null;
async function initGpu() {
    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter) { return; }
    const maxBufferSize = adapter.limits.maxBufferSize;
    gpu = await adapter.requestDevice();
    gpu.lost.then(() => { gpu = null; postMessage({ type: 'lost' }); });
    const layout = { entries: [ { binding: 0, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } }, { binding: 1, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'storage' } }, { binding: 2, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } } ] };
    const bl = gpu.createBindGroupLayout(layout); const pl = gpu.createPipelineLayout({ bindGroupLayouts: [bl] });
    permutationEntropyPipeline = gpu.createComputePipeline({ layout: pl, compute: { module: gpu.createShaderModule({ code: permutationEntropyShader }), entryPoint: 'main' } });
    takensPipeline = gpu.createComputePipeline({ layout: pl, compute: { module: gpu.createShaderModule({ code: takensShader }), entryPoint: 'main' } });
    kurtosisPipeline = gpu.createComputePipeline({ layout: pl, compute: { module: gpu.createShaderModule({ code: kurtosisShader }), entryPoint: 'main' } });
    tdaPipeline = gpu.createComputePipeline({ layout: pl, compute: { module: gpu.createShaderModule({ code: tdaShader }), entryPoint: 'main' } });
    const teLayout = { entries: [ { binding: 0, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } }, { binding: 1, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } }, { binding: 2, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'read-only-storage' } }, { binding: 3, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'storage' } }, { binding: 4, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } } ] };
    const teBl = gpu.createBindGroupLayout(teLayout);
    const tePl = gpu.createPipelineLayout({ bindGroupLayouts: [teBl] });
    tePipeline = gpu.createComputePipeline({ layout: tePl, compute: { module: gpu.createShaderModule({ code: teShader }), entryPoint: 'main' } });
    postMessage({ type: 'ready', maxBufferSize });
}
const initPromise = navigator.gpu ? initGpu().catch(() => {}) : Promise.resolve();
self.onmessage = async (e) => {
    const d = e.data;
    if (d.type !== 'compute') { return; }
    await initPromise;
    if (!gpu) { postMessage({ type: 'result', ok: false, error: 'no_device' }); return; }
    const n = d.n, ringSize = d.ringSize, srcW = d.srcW, dstR = d.dstR, hasTE = d.hasTE;
    let inBuf, paramBuf, kOutBuf, tOutBuf, tdaOutBuf, kurtosisOutBuf, readBuf;
    let srcIdxBuf, dstIdxBuf, teOutBuf, teParamBuf, teBg = null;
    try {
        inBuf = gpu.createBuffer({ size: d.flat.byteLength, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST }); gpu.queue.writeBuffer(inBuf, 0, new Float32Array(d.flat));
        paramBuf = gpu.createBuffer({ size: 16, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST }); gpu.queue.writeBuffer(paramBuf, 0, new Uint32Array([n, ringSize, 0, 0]));
        kOutBuf = gpu.createBuffer({ size: n * 4, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC });
        tOutBuf = gpu.createBuffer({ size: n * 4 * 4, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC });
        tdaOutBuf = gpu.createBuffer({ size: n * 2 * 4, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC });
        kurtosisOutBuf = gpu.createBuffer({ size: n * 4, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC });
        const bl = permutationEntropyPipeline.getBindGroupLayout(0);
        const kBg = gpu.createBindGroup({ layout: bl, entries: [{ binding: 0, resource: { buffer: inBuf } }, { binding: 1, resource: { buffer: kOutBuf } }, { binding: 2, resource: { buffer: paramBuf } }] });
        const tBg = gpu.createBindGroup({ layout: bl, entries: [{ binding: 0, resource: { buffer: inBuf } }, { binding: 1, resource: { buffer: tOutBuf } }, { binding: 2, resource: { buffer: paramBuf } }] });
        const tdaBg = gpu.createBindGroup({ layout: bl, entries: [{ binding: 0, resource: { buffer: inBuf } }, { binding: 1, resource: { buffer: tdaOutBuf } }, { binding: 2, resource: { buffer: paramBuf } }] });
        const kurtosisBg = gpu.createBindGroup({ layout: bl, entries: [{ binding: 0, resource: { buffer: inBuf } }, { binding: 1, resource: { buffer: kurtosisOutBuf } }, { binding: 2, resource: { buffer: paramBuf } }] });
        if (hasTE) {
            srcIdxBuf = gpu.createBuffer({ size: srcW * 4, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST }); gpu.queue.writeBuffer(srcIdxBuf, 0, new Uint32Array(d.srcList));
            dstIdxBuf = gpu.createBuffer({ size: dstR * 4, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST }); gpu.queue.writeBuffer(dstIdxBuf, 0, new Uint32Array(d.dstList));
            teOutBuf = gpu.createBuffer({ size: srcW * dstR * 4, usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC });
            teParamBuf = gpu.createBuffer({ size: 16, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST }); gpu.queue.writeBuffer(teParamBuf, 0, new Uint32Array([srcW, dstR, ringSize, 0]));
            teBg = gpu.createBindGroup({ layout: tePipeline.getBindGroupLayout(0), entries: [{ binding: 0, resource: { buffer: inBuf } }, { binding: 1, resource: { buffer: srcIdxBuf } }, { binding: 2, resource: { buffer: dstIdxBuf } }, { binding: 3, resource: { buffer: teOutBuf } }, { binding: 4, resource: { buffer: teParamBuf } }] });
        }
        const readSize = n * 4 + n * 4 * 4 + n * 2 * 4 + n * 4 + (hasTE ? srcW * dstR * 4 : 0);
        readBuf = gpu.createBuffer({ size: readSize, usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST });
        const enc = gpu.createCommandEncoder();
        const kPass = enc.beginComputePass(); kPass.setPipeline(permutationEntropyPipeline); kPass.setBindGroup(0, kBg); kPass.dispatchWorkgroups(Math.ceil(n / 64)); kPass.end();
        const tPass = enc.beginComputePass(); tPass.setPipeline(takensPipeline); tPass.setBindGroup(0, tBg); tPass.dispatchWorkgroups(Math.ceil(n / 64)); tPass.end();
        const tdaPass = enc.beginComputePass(); tdaPass.setPipeline(tdaPipeline); tdaPass.setBindGroup(0, tdaBg); tdaPass.dispatchWorkgroups(Math.ceil(n / 64)); tdaPass.end();
        const kurtosisPass = enc.beginComputePass(); kurtosisPass.setPipeline(kurtosisPipeline); kurtosisPass.setBindGroup(0, kurtosisBg); kurtosisPass.dispatchWorkgroups(Math.ceil(n / 64)); kurtosisPass.end();
        if (hasTE) { const tePass = enc.beginComputePass(); tePass.setPipeline(tePipeline); tePass.setBindGroup(0, teBg); tePass.dispatchWorkgroups(Math.ceil(srcW / 8), Math.ceil(dstR / 8), 1); tePass.end(); }
        let ro = 0;
        enc.copyBufferToBuffer(kOutBuf, 0, readBuf, ro, n * 4); ro += n * 4;
        enc.copyBufferToBuffer(tOutBuf, 0, readBuf, ro, n * 4 * 4); ro += n * 4 * 4;
        enc.copyBufferToBuffer(tdaOutBuf, 0, readBuf, ro, n * 2 * 4); ro += n * 2 * 4;
        enc.copyBufferToBuffer(kurtosisOutBuf, 0, readBuf, ro, n * 4); ro += n * 4;
        if (hasTE) { enc.copyBufferToBuffer(teOutBuf, 0, readBuf, ro, srcW * dstR * 4); ro += srcW * dstR * 4; }
        gpu.queue.submit([enc.finish()]);
        await readBuf.mapAsync(GPUMapMode.READ);
        const res = new Float32Array(readBuf.getMappedRange());
        let off = 0;
        const kRes = res.slice(off, off + n); off += n;
        const tRes = res.slice(off, off + n * 4); off += n * 4;
        const tdaRes = res.slice(off, off + n * 2); off += n * 2;
        const kurtosisRes = res.slice(off, off + n); off += n;
        const teRes = hasTE ? res.slice(off, off + srcW * dstR) : null;
        readBuf.unmap();
        const msg = { type: 'result', ok: true, kRes: kRes.buffer, tRes: tRes.buffer, tdaRes: tdaRes.buffer, kurtosisRes: kurtosisRes.buffer, teRes: teRes ? teRes.buffer : null };
        const transfers = [msg.kRes, msg.tRes, msg.tdaRes, msg.kurtosisRes];
        if (teRes) { transfers.push(msg.teRes); }
        postMessage(msg, transfers);
    } catch (err) {
        postMessage({ type: 'result', ok: false, error: err && err.message ? err.message : String(err) });
    } finally {
        if (inBuf) inBuf.destroy(); if (kOutBuf) kOutBuf.destroy(); if (tOutBuf) tOutBuf.destroy(); if (tdaOutBuf) tdaOutBuf.destroy(); if (kurtosisOutBuf) kurtosisOutBuf.destroy(); if (paramBuf) paramBuf.destroy(); if (readBuf) readBuf.destroy();
        if (srcIdxBuf) srcIdxBuf.destroy(); if (dstIdxBuf) dstIdxBuf.destroy(); if (teOutBuf) teOutBuf.destroy(); if (teParamBuf) teParamBuf.destroy();
    }
};
