const C = 299792458.0;
const PHI = 1.618033988749895;

export const is = {};
export const pulse = { ws: null, pending: new Map(), seq: 0 };

function weave(p, result) {
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

export async function get(inputs, queries) {
    if (!queries || queries.length === 0) return {};

    let inputBytes = 0;
    for (const inp of inputs) { inputBytes += 41 + inp.name.length; }
    const buf = new ArrayBuffer(8 + inputBytes + 4 + queries.length * 32);
    const dv = new DataView(buf);
    const id = ++pulse.seq;
    dv.setUint32(0, id, true);
    dv.setUint32(4, inputs.length, true);
    let off = 8;

    for (const inp of inputs) {
        dv.setFloat64(off, inp.t, true); off += 8;
        dv.setFloat64(off, inp.x, true); off += 8;
        dv.setFloat64(off, inp.y, true); off += 8;
        dv.setFloat64(off, inp.z, true); off += 8;
        dv.setFloat64(off, inp.value, true); off += 8;
        dv.setUint8(off, inp.name.length); off += 1;
        for (let i = 0; i < inp.name.length; i++) { dv.setUint8(off, inp.name.charCodeAt(i)); off++; }
    }

    dv.setUint32(off, queries.length, true); off += 4;
    for (const q of queries) {
        dv.setFloat64(off, q.t, true); off += 8;
        dv.setFloat64(off, q.x, true); off += 8;
        dv.setFloat64(off, q.y, true); off += 8;
        dv.setFloat64(off, q.z, true); off += 8;
    }

    const promise = new Promise((resolve, reject) => {
        pulse.pending.set(id, { resolve, reject });
    });
    if (pulse.ws && pulse.ws.readyState === WebSocket.OPEN) {
        pulse.ws.send(new Uint8Array(buf));
    }
    const buffer = await promise;
    const batchRecords = parseBatchPayload(new Uint8Array(buffer));

    const result = {};
    for (const p of batchRecords.flat()) { weave(p, result); }

    let g = measureG(result);
    let vC = measureVC(result);
    let decay = measureDecay(result);
    let quantum = measureQuantum();
    const tNow = result['server.time'] !== undefined
        ? (result['server.time'] / 86400.0) + 2440587.5 - 2451545.0
        : (queries[0] ? queries[0].t : Date.now() / 1000);
    const dtEff = Math.abs((queries[0] ? queries[0].t : tNow) - tNow);

    result.certainty = Math.exp(-dtEff * g)
                    * Math.exp(-vC / (g + (1.0 / C)))
                    * quantum * decay;

    return result;
}

function parseBatchPayload(bytes) {
    const dv = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const td = new TextDecoder();
    let o = 0;

    if (bytes.length < 13 || bytes[0] !== 73 || bytes[1] !== 83 || bytes[2] !== 6) {
        return [];
    }
    o = 3;
    o += 4;
    const pointCount = dv.getUint32(o, true); o += 4;
    
    const allRecords = [];

    for (let pi = 0; pi < pointCount; pi++) {
        if (o + 4 > bytes.length) break;
        const objCount = dv.getUint32(o, true); o += 4;
        const pointRecords = [];

        for (let oi = 0; oi < objCount; oi++) {
            if (o >= bytes.length) break;
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
                pointRecords.push(base);
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
                    pointRecords.push(p);
                }
            }
        }
        allRecords.push(pointRecords);
    }

    return allRecords;
}

function measureDecay(result) {
    const flux = result['radiation_proton_flux_100mev'];
    if (typeof flux === 'number' && flux >= 0) {
        return 1.0 / (1.0 + flux);
    }
    return 1.0;
}

function measureG(result) {
    const ax = result['AccelerometerSensor.x'];
    const ay = result['AccelerometerSensor.y'];
    const az = result['AccelerometerSensor.z'];
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
        if (s.complexity > 0) { sum += s.complexity; count++; }
    }
    if (count === 0) return 1.0;
    return Math.exp(-sum / count);
}

function measureVC(result) {
    const speed = result['geolocation.speed'];
    if (typeof speed === 'number' && speed >= 0) {
        return speed / C;
    }
    return 0.0;
}
