const C = 299792458.0;
const PHI = 1.618033988749895;

export const live = {};
export const pulse = { ws: null, pending: new Map(), seq: 0 };

const _SHADER = `
@group(0) @binding(0) var<storage, read> data: array<f32>;
@group(0) @binding(1) var<storage, read_write> cert: array<f32>;
@group(0) @binding(2) var<uniform> params0: vec4<f32>;
@group(0) @binding(3) var<uniform> params1: vec4<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    let n = u32(params0.z);
    if (i >= n) { return; }
    let base = i * 5u;
    let dt = abs(data[base]);
    let g = params0.x;
    let v_c = params0.y;
    let epig = params0.w;
    let decay = params1.x;
    let quantum = params1.y;
    cert[i] = exp(-dt * g) * exp(-v_c / (g + 0.0000001)) * quantum * decay * epig;
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
                { binding: 3, visibility: GPUShaderStage.COMPUTE, buffer: { type: 'uniform' } },
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

async function _gpuEvaluate(points, g, v_c, epig, decay, quantum) {
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
        const paramData0 = new Float32Array(4);
        paramData0[0] = g;
        paramData0[1] = v_c;
        paramData0[2] = n;
        paramData0[3] = epig;
        const paramBuf0 = device.createBuffer({
            size: 16,
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST
        });
        device.queue.writeBuffer(paramBuf0, 0, paramData0);
        const paramData1 = new Float32Array(4);
        paramData1[0] = decay;
        paramData1[1] = quantum;
        const paramBuf1 = device.createBuffer({
            size: 16,
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST
        });
        device.queue.writeBuffer(paramBuf1, 0, paramData1);
        const readBuf = device.createBuffer({
            size: n * 4,
            usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST
        });
        const bindGroup = device.createBindGroup({
            layout: _gpuBindLayout,
            entries: [
                { binding: 0, resource: { buffer: inBuf } },
                { binding: 1, resource: { buffer: outBuf } },
                { binding: 2, resource: { buffer: paramBuf0 } },
                { binding: 3, resource: { buffer: paramBuf1 } },
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
        paramBuf0.destroy();
        paramBuf1.destroy();
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


function _measureG(live) {
    const ax = live['AccelerometerSensor.x'];
    const ay = live['AccelerometerSensor.y'];
    const az = live['AccelerometerSensor.z'];
    if (ax !== undefined && ay !== undefined && az !== undefined) {
        return Math.sqrt(ax*ax + ay*ay + az*az);
    }
    return 1.0;
}

function _measureVC(live) {
    const speed = live['geolocation.speed'];
    if (typeof speed === 'number' && speed >= 0) {
        return speed / C;
    }
    return 0.0;
}

function _measureDecay(result) {
    const flux = result['cosmic_protons_100mev'];
    if (typeof flux === 'number' && flux >= 0) {
        return 1.0 / (1.0 + flux);
    }
    return 1.0;
}

function _measureQuantum() {
    const sensors = window.omegaflow?.sensors;
    if (!sensors || sensors.size === 0) return 1.0;
    let sum = 0, count = 0;
    for (const s of sensors.values()) {
        if (s.noiseFloor > 0) { sum += s.noiseFloor; count++; }
    }
    if (count === 0) return 1.0;
    return Math.exp(-sum / count);
}

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

    let g = _measureG(live);
    let v_c = _measureVC(live);
    let decay = _measureDecay(result);
    let quantum = _measureQuantum();
    let epig = 1.0;
    let t_now = live['server.time'] !== undefined
        ? (live['server.time'] / 86400.0) + 2440587.5 - 2451545.0
        : t;
    let dt_eff = Math.abs(t - t_now);

    const points = _collectPoints(result, t_now, x, y, z);
    if (points && points.count > 0) {
        const gpuResult = await _gpuEvaluate(points, g, v_c, epig, decay, quantum);
        if (gpuResult) {
            result.certainty = gpuResult[0];
        } else {
            result.certainty = Math.exp(-dt_eff * g)
                             * Math.exp(-v_c / (g + 1e-15))
                             * quantum * decay * epig;
        }
    } else {
        result.certainty = Math.exp(-dt_eff * g)
                         * Math.exp(-v_c / (g + 1e-15))
                         * quantum * decay * epig;
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


