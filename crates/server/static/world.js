const C = 299792458.0;

export const live = {};
export const pulse = { ws: null, pending: new Map(), seq: 0 };

const _SHADER = `
@group(0) @binding(0) var<storage, read> data: array<f32>;
@group(0) @binding(1) var<storage, read_write> cert: array<f32>;
@group(0) @binding(2) var<uniform> params: vec4<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    let n = u32(params.z);
    if (i >= n) { return; }
    let base = i * 5u;
    let dt = abs(data[base]);
    let dx = data[base + 1u];
    let dy = data[base + 2u];
    let dz = data[base + 3u];
    let dist = length(vec3<f32>(dx, dy, dz));
    let g = params.x;
    let v_c = params.y;
    let epig = params.w;
    let c_q = data[base + 4u];
    cert[i] = exp(-dt * g) * exp(-v_c / (g + 0.0000001)) * c_q * epig;
}
`;

let _gpuDevice = null;
let _gpuPipeline = null;
let _gpuBindLayout = null;

async function _initGPU() {
    if (_gpuDevice) return;
    try {
        const gpu = window.omegaflow?.gpu;
        if (!gpu) return;
        _gpuDevice = gpu;
        const module = gpu.createShaderModule({ code: _SHADER });
        _gpuBindLayout = gpu.createBindGroupLayout({
            entries: [
                { binding: 0, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'storage' } },
                { binding: 1, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'storage' } },
                { binding: 2, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } },
            ]
        });
        const pipelineLayout = gpu.createPipelineLayout({ bindGroupLayouts: [_gpuBindLayout] });
        _gpuPipeline = gpu.createComputePipeline({
            layout: pipelineLayout,
            compute: { module, entryPoint: 'main' }
        });
    } catch { _gpuDevice = null; }
}

function _collectPoints(result, observerT, observerX, observerY, observerZ) {
    const keys = Object.keys(result).filter(k => typeof result[k] === 'number');
    const n = keys.length;
    if (n === 0) return null;
    const buf = new Float32Array(n * 5);
    let idx = 0;
    for (const key of keys) {
        const val = result[key];
        const parts = key.split('_');
        const suffix = parts[parts.length - 1];
        let dx = 0, dy = 0, dz = 0;
        if (suffix === 'x' && parts.length >= 2) {
            const base = parts.slice(0, -1).join('_');
            const ry = result[base + '_y'];
            const rz = result[base + '_z'];
            if (typeof ry === 'number' && typeof rz === 'number') {
                dx = val - observerX;
                dy = ry - observerY;
                dz = rz - observerZ;
            }
        } else if (suffix === 'y' || suffix === 'z') {
            continue;
        }
        buf[idx * 5] = 0;
        buf[idx * 5 + 1] = dx;
        buf[idx * 5 + 2] = dy;
        buf[idx * 5 + 3] = dz;
        buf[idx * 5 + 4] = val;
        idx++;
    }
    return { buf: buf.slice(0, idx * 5), count: idx };
}

async function _gpuEvaluate(points, g, v_c, epig) {
    if (!_gpuDevice) await _initGPU();
    if (!_gpuDevice || !_gpuPipeline) return null;
    try {
        const device = _gpuDevice;
        const n = points.count;
        const inBuf = device.createBuffer({
            size: points.buf.byteLength,
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
        });
        device.queue.writeBuffer(inBuf, 0, points.buf);
        const outBuf = device.createBuffer({
            size: n * 4,
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC
        });
        const paramData = new Float32Array(4);
        paramData[0] = g;
        paramData[1] = v_c;
        paramData[2] = n;
        paramData[3] = epig;
        const paramBuf = device.createBuffer({
            size: 16,
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST
        });
        device.queue.writeBuffer(paramBuf, 0, paramData);
        const readBuf = device.createBuffer({
            size: n * 4,
            usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST
        });
        const bindGroup = device.createBindGroup({
            layout: _gpuBindLayout,
            entries: [
                { binding: 0, resource: { buffer: inBuf } },
                { binding: 1, resource: { buffer: outBuf } },
                { binding: 2, resource: { buffer: paramBuf } },
            ]
        });
        const encoder = device.createCommandEncoder();
        const pass = encoder.beginComputePass();
        pass.setPipeline(_gpuPipeline);
        pass.setBindGroup(0, bindGroup);
        pass.dispatchWorkgroups(Math.ceil(n / 64));
        pass.end();
        encoder.copyBufferToBuffer(outBuf, 0, readBuf, 0, n * 4);
        device.queue.submit([encoder.finish()]);
        await readBuf.mapAsync(GPUMapMode.READ);
        const result = new Float32Array(readBuf.getMappedRange().slice(0));
        readBuf.unmap();
        inBuf.destroy();
        outBuf.destroy();
        paramBuf.destroy();
        readBuf.destroy();
        return result;
    } catch { return null; }
}

function drain(p, result) {
    for (const key in p) {
        const val = p[key];
        if (typeof val === 'number') {
            if (result[key] === undefined) result[key] = 0;
            result[key] += val;
        } else if (Array.isArray(val) && val.length === 3) {
            if (result[key] === undefined) result[key] = [0, 0, 0];
            result[key][0] += val[0];
            result[key][1] += val[1];
            result[key][2] += val[2];
        }
    }
}

let _last_t = NaN, _last_x = NaN, _last_y = NaN, _last_z = NaN, _last_result = null;
let _fetch_pending = null;
let _fetch_time = 0;
let _last_is_data = null;
const _FETCH_MAX_AGE_MS = 30000;

export async function get(t, x, y, z) {
    if (t === _last_t && x === _last_x && y === _last_y && z === _last_z && _last_result) {
        return _last_result;
    }

    const now = performance.now();
    let needFetch = !_last_is_data
        || (now - _fetch_time) > _FETCH_MAX_AGE_MS
        || Math.abs(t - _last_t) > 0.01
        || Math.abs(x - _last_x) > 1e3
        || Math.abs(y - _last_y) > 1e3
        || Math.abs(z - _last_z) > 1e3;

    if (needFetch) {
        if (_fetch_pending) {
            await _fetch_pending;
        } else {
            _fetch_pending = _doFetch(t, x, y, z);
            await _fetch_pending;
            _fetch_pending = null;
            _fetch_time = now;
        }
    }

    const result = {};
    if (_last_is_data) {
        for (const p of _last_is_data) {
            drain(p, result);
        }
    }

    let g = result.grav_time_dilation !== undefined ? result.grav_time_dilation : 1.0;
    let phi = Math.abs(result.gravity || 0);
    let v_c = Math.sqrt(2 * phi) / C;
    let t_now = live['server.time'] !== undefined
        ? (live['server.time'] / 86400.0) + 2440587.5 - 2451545.0
        : t;
    let dt_s = Math.abs(t - t_now) * 86400.0;
    let dt_eff = dt_s / 86400.0;
    let c_q = result.quantum !== undefined ? result.quantum : 1.0;
    let decay = result.decay_probability !== undefined ? result.decay_probability : 1.0;
    let epig = live['epigenetic_factor'] !== undefined ? live['epigenetic_factor'] : 1.0;

    const points = _collectPoints(result, t_now, x, y, z);
    if (points && points.count > 0) {
        const gpuResult = await _gpuEvaluate(points, g, v_c, epig);
        if (gpuResult) {
            result.certainty = gpuResult[0];
        } else {
            result.certainty = Math.exp(-dt_eff * g)
                             * Math.exp(-v_c / (g + 1e-15))
                             * c_q
                             * decay
                             * epig;
        }
    } else {
        result.certainty = Math.exp(-dt_eff * g)
                         * Math.exp(-v_c / (g + 1e-15))
                         * c_q
                         * decay
                         * epig;
    }

    for (const key in live) {
        result[key] = live[key];
    }

    _last_t = t; _last_x = x; _last_y = y; _last_z = z;
    _last_result = result;

    return result;
}

async function _doFetch(t, x, y, z) {
    const buf = new ArrayBuffer(33);
    const dv = new DataView(buf);
    dv.setFloat64(0, t, true);
    dv.setFloat64(8, x, true);
    dv.setFloat64(16, y, true);
    dv.setFloat64(24, z, true);
    dv.setUint8(32, 0);
    const id = ++pulse.seq;
    const promise = new Promise((resolve, reject) => {
        pulse.pending.set(id, { resolve, reject });
    });
    const frame = new Uint8Array(37);
    new DataView(frame.buffer).setUint32(33, id, true);
    frame.set(new Uint8Array(buf), 0);
    if (pulse.ws && pulse.ws.readyState === WebSocket.OPEN) {
        pulse.ws.send(frame);
        const buffer = await promise;
        _last_is_data = parsePayload(new Uint8Array(buffer));
    }
}

function parsePayload(bytes) {
    const dv = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const td = new TextDecoder();
    let o = 0;

    if (bytes.length < 7 || bytes[0] !== 73 || bytes[1] !== 83 || bytes[2] !== 2) {
        return [];
    }
    o = 3;
    const objCount = dv.getUint32(o, true); o += 4;

    const records = [];

    for (let oi = 0; oi < objCount; oi++) {
        const sfCount = bytes[o++];
        const staticFields = [];
        for (let s = 0; s < sfCount; s++) {
            const nl = bytes[o++];
            const name = td.decode(bytes.slice(o, o + nl)); o += nl;
            const typ = bytes[o++];
            staticFields.push({ name, typ });
        }

        const base = {};
        for (const f of staticFields) {
            if (f.typ === 0) { base[f.name] = dv.getFloat64(o, true); o += 8; }
            else if (f.typ === 1) { base[f.name] = dv.getUint32(o, true); o += 4; }
            else if (f.typ === 2) {
                const cnt = dv.getUint32(o, true); o += 4;
                const arr = new Float64Array(cnt);
                for (let i = 0; i < cnt; i++) { arr[i] = dv.getFloat64(o, true); o += 8; }
                base[f.name] = arr;
            }
        }

        const recCount = dv.getUint32(o, true); o += 4;

        if (recCount === 0) {
            records.push(base);
        } else {
            const rfCount = bytes[o++];
            const recordFields = [];
            for (let r = 0; r < rfCount; r++) {
                const nl = bytes[o++];
                const name = td.decode(bytes.slice(o, o + nl)); o += nl;
                const typ = bytes[o++];
                recordFields.push({ name, typ });
            }

            for (let ri = 0; ri < recCount; ri++) {
                const p = Object.assign({}, base);
                for (const f of recordFields) {
                    if (f.typ === 0) { p[f.name] = dv.getFloat64(o, true); o += 8; }
                    else if (f.typ === 1) { p[f.name] = dv.getUint32(o, true); o += 4; }
                    else if (f.typ === 2) {
                        const cnt = dv.getUint32(o, true); o += 4;
                        const arr = new Float64Array(cnt);
                        for (let i = 0; i < cnt; i++) { arr[i] = dv.getFloat64(o, true); o += 8; }
                        p[f.name] = arr;
                    }
                }
                records.push(p);
            }
        }
    }

    return records;
}
