export const C = 299792458.0;
export const Φ = 1.618033988749895;
export const φ = {};
export const transport = { socket: null, pending: new Map(), seq: 0, tickTime: 16, rtt: 0, srtt: 0, rttvar: 0 };

export function updateRtt(sampleRtt) {
    if (transport.srtt === 0) { transport.srtt = sampleRtt; transport.rttvar = sampleRtt / 2; }
    else { transport.rttvar = 0.75 * transport.rttvar + 0.25 * Math.abs(sampleRtt - transport.srtt); transport.srtt = 0.875 * transport.srtt + 0.125 * sampleRtt; }
    transport.rtt = transport.srtt;
}

export function getRto() {
    if (transport.srtt === 0) return 5000;
    return Math.max(100, Math.min(transport.srtt + 4 * Math.max(transport.rttvar, 1), 5000));
}

export async function syncFrame(inputs, queries, presence) {
    inputs = inputs || [];
    queries = queries || [];
    const emptyResp = { field: new Float32Array(0), meta: new Float32Array(0), count: 0, response_epoch: 0 };
    if (inputs.length === 0 && queries.length === 0) return emptyResp;

    let inputBytes = 0;
    for (const inp of inputs) inputBytes += 17 + new TextEncoder().encode(inp.name).length;
    const buf = new ArrayBuffer(8 + inputBytes + 4 + queries.length * 32 + 48);
    const dv = new DataView(buf);
    const id = ++transport.seq;
    dv.setUint32(0, id, true);
    dv.setUint32(4, inputs.length, true);
    let off = 8;
    for (const inp of inputs) {
        dv.setFloat64(off, inp.value, true); off += 8;
        const nameBytes = new TextEncoder().encode(inp.name);
        dv.setUint8(off, nameBytes.length); off += 1;
        new Uint8Array(buf, off, nameBytes.length).set(nameBytes); off += nameBytes.length;
        dv.setFloat64(off, inp.tau || 0, true); off += 8;
    }
    dv.setUint32(off, queries.length, true); off += 4;
    for (const q of queries) {
        dv.setFloat64(off, q.t, true); off += 8;
        dv.setFloat64(off, q.x, true); off += 8;
        dv.setFloat64(off, q.y, true); off += 8;
        dv.setFloat64(off, q.z, true); off += 8;
    }
    dv.setFloat64(off, presence.x, true); off += 8;
    dv.setFloat64(off, presence.y, true); off += 8;
    dv.setFloat64(off, presence.z, true); off += 8;
    dv.setFloat64(off, presence.t, true); off += 8;
    dv.setFloat64(off, presence.range, true); off += 8;
    dv.setFloat64(off, presence.cache_interval || 0, true); off += 8;

    const startTime = performance.now();
    if (!transport.socket || transport.socket.readyState !== WebSocket.OPEN) return emptyResp;

    const promise = new Promise((resolve, reject) => {
        const timeoutDuration = getRto();
        const timeout = setTimeout(() => {
            if (transport.pending.has(id)) { transport.pending.delete(id); reject(new Error("frame timed out")); }
        }, timeoutDuration);
        transport.pending.set(id, { resolve, reject, timeout, startTime });
    });

    transport.socket.send(new Uint8Array(buf));
    const buffer = await promise;

    const bytes = new Uint8Array(buffer);
    const dvRes = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    if (bytes.length < 19 || bytes[0] !== 0xCF || bytes[1] !== 0x86) return emptyResp;
    if (bytes[2] !== 7) throw new Error('protocol mismatch');

    let o = 3;
    const response_epoch = dvRes.getFloat64(o, true); o += 8;
    o += 4;
    const oscCount = dvRes.getUint32(o, true); o += 4;

    const field = new Float32Array(oscCount * 12);
    const meta = new Float32Array(oscCount * 12);

    for (let i = 0; i < oscCount; i++) {
        if (o + 176 > bytes.length) break;
        const x = dvRes.getFloat64(o, true); o += 8;
        const y = dvRes.getFloat64(o, true); o += 8;
        const z = dvRes.getFloat64(o, true); o += 8;
        const val = dvRes.getFloat64(o, true); o += 8;
        const t = dvRes.getFloat64(o, true); o += 8;
        const ttl = dvRes.getFloat64(o, true); o += 8;
        const tau = dvRes.getFloat64(o, true); o += 8;
        const extent = dvRes.getFloat64(o, true); o += 8;
        const kernel_id = dvRes.getFloat64(o, true); o += 8;
        const force_type = dvRes.getFloat64(o, true); o += 8;
        const absorption = dvRes.getFloat64(o, true); o += 8;
        const advection = dvRes.getFloat64(o, true); o += 8;
        const vx = dvRes.getFloat64(o, true); o += 8;
        const vy = dvRes.getFloat64(o, true); o += 8;
        const vz = dvRes.getFloat64(o, true); o += 8;
        const pole_x = dvRes.getFloat64(o, true); o += 8;
        const pole_y = dvRes.getFloat64(o, true); o += 8;
        const pole_z = dvRes.getFloat64(o, true); o += 8;
        const j2 = dvRes.getFloat64(o, true); o += 8;
        const j4 = dvRes.getFloat64(o, true); o += 8;
        const r_eq = dvRes.getFloat64(o, true); o += 8;
        const color_index = dvRes.getFloat64(o, true); o += 8;

        const fOff = i * 12;
        if (presence) {
            field[fOff] = Math.fround(x - presence.x);
            field[fOff + 1] = Math.fround(y - presence.y);
            field[fOff + 2] = Math.fround(z - presence.z);
        } else {
            field[fOff] = x;
            field[fOff + 1] = y;
            field[fOff + 2] = z;
        }
        field[fOff + 3] = val;
        field[fOff + 4] = t;
        field[fOff + 5] = ttl;
        field[fOff + 6] = force_type;
        field[fOff + 7] = absorption;
        field[fOff + 8] = advection;
        field[fOff + 9] = vx;
        field[fOff + 10] = vy;
        field[fOff + 11] = vz;

        const mOff = i * 12;
        meta[mOff] = extent;
        meta[mOff + 1] = tau;
        meta[mOff + 2] = kernel_id;
        meta[mOff + 3] = 0;
        meta[mOff + 4] = pole_x;
        meta[mOff + 5] = pole_y;
        meta[mOff + 6] = pole_z;
        meta[mOff + 7] = j2;
        meta[mOff + 8] = j4;
        meta[mOff + 9] = r_eq;
        meta[mOff + 10] = color_index;
        meta[mOff + 11] = 0;
    }
    return { field, meta, count: oscCount, response_epoch };
}

