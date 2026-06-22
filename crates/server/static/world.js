const C = 299792458.0;
const EPOCH_J2000 = 2451545.0;
const J2000_YEAR = 2000.0;
const TROPICAL_YEAR = 365.24219;

export const live = {};
export const pulse = { ws: null, pending: new Map(), seq: 0 };

let localPck = null;
let localRecords = null;
let localWmm = null;

function drain(p, t, x, y, z, result) {
    if ('mid' in p && 'rad' in p) {
        if (Math.abs(t - p.mid) > p.rad) return;
    }
    for (const key in p) {
        if (key === 'mid' || key === 'rad' || key === 'nc' || key === 'coeffs') continue;
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

export async function get(t, x, y, z) {
    if (t === _last_t && x === _last_x && y === _last_y && z === _last_z) {
        return _last_result;
    }

    let needFetch = false;
    if (!localRecords) {
        needFetch = true;
    } else {
        let covered = false;
        for (const r of localRecords) {
            if ('mid' in r && 'rad' in r) {
                if (Math.abs(t - r.mid) <= r.rad) { covered = true; break; }
            }
        }
        if (!covered) needFetch = true;
    }

    let egm96Value = null;

    if (needFetch || !localWmm || !localPck) {
        const buf = new ArrayBuffer(33);
        const dv = new DataView(buf);
        dv.setFloat64(0, t, true);
        dv.setFloat64(8, x, true);
        dv.setFloat64(16, y, true);
        dv.setFloat64(24, z, true);
        let flags = 0;
        if (localWmm) flags |= 1;
        if (localPck) flags |= 2;
        dv.setUint8(32, flags);
        const id = ++pulse.seq;
        const promise = new Promise((resolve, reject) => {
            pulse.pending.set(id, { resolve, reject });
        });
        const frame = new Uint8Array(37);
        new DataView(frame.buffer).setUint32(33, id, true);
        frame.set(new Uint8Array(buf), 0);
        pulse.ws.send(frame);
        const buffer = await promise;
        const parsed = parsePayload(new Uint8Array(buffer));

        if (parsed.records && parsed.records.length > 0) localRecords = parsed.records;
        if (parsed.wmm) localWmm = parsed.wmm;
        if (parsed.pck) localPck = parsed.pck;
        if (parsed.egm96) egm96Value = parsed.egm96;
    }

    const result = {};

    const items = [];
    if (localRecords) items.push(...localRecords);
    if (localWmm) items.push(localWmm);
    if (egm96Value) items.push(egm96Value);

    for (const p of items) {
        drain(p, t, x, y, z, result);
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
    result.certainty = Math.exp(-dt_eff * g)
                     * Math.exp(-v_c / (g + 1e-15))
                     * c_q
                     * decay
                     * epig;

    for (const key in live) {
        result[key] = live[key];
    }

    _last_t = t; _last_x = x; _last_y = y; _last_z = z;
    _last_result = result;

    return result;
}

function matVec(m, v) {
    return [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2]
    ];
}

function parsePayload(bytes) {
    const dv = new DataView(bytes.buffer);
    const td = new TextDecoder();
    let o = 0;

    if (bytes.length < 4 || bytes[0] !== 73 || bytes[1] !== 83 || bytes[2] !== 2) {
        return {};
    }
    o = 3;
    const objCount = dv.getUint32(o, true); o += 4;

    const payload = { egm96: null, pck: null, records: [], wmm: null };

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
            if ('g' in base && 'h' in base) payload.wmm = base;
            else if ('ra' in base && 'dec' in base) payload.pck = base;
            else if ('value' in base && 'min' in base) payload.egm96 = base;
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
                payload.records.push(p);
            }
        }
    }

    return payload;
}
