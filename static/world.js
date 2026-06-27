const C = 299792458.0;
const PHI = 1.618033988749895;

export const live = {};
export const pulse = { ws: null, pending: new Map(), seq: 0 };

let lastT = NaN, lastX = NaN, lastY = NaN, lastZ = NaN, lastResult = null;
let fetchPending = null;
let fetchTime = 0;
let lastIsData = null;

function blend(p, result) {
    const ma = 1 / (PHI * PHI);
    for (const key in p) {
        const val = p[key];
        if (typeof val === 'number') {
            result[key] = (result[key] || 0) * (1 - ma) + val * ma;
        } else if (Array.isArray(val) && val.length === 3) {
            if (!result[key]) result[key] = [0, 0, 0];
            result[key][0] = result[key][0] * (1 - ma) + val[0] * ma;
            result[key][1] = result[key][1] * (1 - ma) + val[1] * ma;
            result[key][2] = result[key][2] * (1 - ma) + val[2] * ma;
        }
    }
}

export async function get(t, x, y, z) {
    if (t === lastT && x === lastX && y === lastY && z === lastZ && lastResult) {
        return lastResult;
    }

    const now = performance.now();
    let needFetch = !lastIsData
    || Math.abs(t - lastT) > (1.0 / 128.0)
    || Math.abs(x - lastX) > (live['geolocation.accuracy'] || 0) * PHI
    || Math.abs(y - lastY) > (live['geolocation.accuracy'] || 0) * PHI
    || Math.abs(z - lastZ) > (live['geolocation.accuracy'] || 0) * PHI;

    if (needFetch) {
        if (fetchPending) {
            await fetchPending;
        } else {
            fetchPending = doFetch(t, x, y, z);
            await fetchPending;
            fetchPending = null;
            fetchTime = now;
        }
    }

    const result = {};
    if (lastIsData) {
        for (const p of lastIsData) {
            blend(p, result);
        }
    }

    let g = measureG(live);
    let vC = measureVC(live);
    let decay = measureDecay(result);
    let quantum = measureQuantum();
    let epig = measureEpigenetics(result);
    let tNow = live['server.time'] !== undefined
        ? (live['server.time'] / 86400.0) + 2440587.5 - 2451545.0
        : t;
    let dtEff = Math.abs(t - tNow);

    result.certainty = Math.exp(-dtEff * g)
                    * Math.exp(-vC / (g + (1.0 / C)))
                    * quantum * decay * epig;

    for (const key in live) {
        result[key] = live[key];
    }

    lastT = t; lastX = x; lastY = y; lastZ = z;
    lastResult = result;

    return result;
}

function parsePayload(bytes) {
    const dv = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const td = new TextDecoder();
    let o = 0;

    if (bytes.length < 7 || bytes[0] !== 73 || bytes[1] !== 83 || bytes[2] !== 4) {
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

async function doFetch(t, x, y, z) {
    const buf = new ArrayBuffer(32);
    const dv = new DataView(buf);
    dv.setFloat64(0, t, true);
    dv.setFloat64(8, x, true);
    dv.setFloat64(16, y, true);
    dv.setFloat64(24, z, true);
    const id = ++pulse.seq;
    const promise = new Promise((resolve, reject) => {
        pulse.pending.set(id, { resolve, reject });
    });
    const frame = new Uint8Array(36);
    new DataView(frame.buffer).setUint32(32, id, true);
    frame.set(new Uint8Array(buf), 0);
    if (pulse.ws && pulse.ws.readyState === WebSocket.OPEN) {
        pulse.ws.send(frame);
        const buffer = await promise;
        lastIsData = parsePayload(new Uint8Array(buffer));
    }
}

function measureDecay(result) {
    const flux = result['radiation_proton_flux_100mev'];
    if (typeof flux === 'number' && flux >= 0) {
        return 1.0 / (1.0 + flux);
    }
    return 1.0;
}

function measureG(live) {
    const ax = live['AccelerometerSensor.x'];
    const ay = live['AccelerometerSensor.y'];
    const az = live['AccelerometerSensor.z'];
    if (ax !== undefined && ay !== undefined && az !== undefined) {
        return Math.sqrt(ax*ax + ay*ay + az*az);
    }
    return 1.0;
}

function measureQuantum() {
    const sensors = window.omegaflow?.sensors;
    if (!sensors || sensors.size === 0) return 1.0;
    let sum = 0, count = 0;
    for (const s of sensors.values()) {
        if (s.noiseFloor > 0) { sum += s.noiseFloor; count++; }
    }
    if (count === 0) return 1.0;
    return Math.exp(-sum / count);
}

function measureVC(live) {
    const speed = live['geolocation.speed'];
    if (typeof speed === 'number' && speed >= 0) {
        return speed / C;
    }
    return 0.0;
}
