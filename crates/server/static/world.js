const C = 299792458.0;

export const live = {};
export const pulse = { ws: null, pending: new Map(), seq: 0 };

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
