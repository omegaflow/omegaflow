export const C = 299792458.0;
export const Φ = 1.618033988749895;
export const UNIX_J2000_OFFSET = 946728000.0;
export function tdbNow(unixSecs) { return unixSecs - UNIX_J2000_OFFSET; }
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
export async function syncFrame(inputs, queries) {
    inputs = inputs || [];
    queries = queries || [];
    if (inputs.length === 0 && queries.length === 0) return [];
    let inputBytes = 0;
    for (const inp of inputs) inputBytes += 9 + inp.name.length;
    const buf = new ArrayBuffer(8 + inputBytes + 4 + queries.length * 32);
    const dv = new DataView(buf);
    const id = ++transport.seq;
    dv.setUint32(0, id, true);
    dv.setUint32(4, inputs.length, true);
    let off = 8;
    for (const inp of inputs) {
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
    const startTime = performance.now();
    if (!transport.socket || transport.socket.readyState !== WebSocket.OPEN) return [];
    const promise = new Promise((resolve, reject) => {
        const timeoutDuration = getRto();
        const timeout = setTimeout(() => {
            if (transport.pending.has(id)) { transport.pending.delete(id); reject(new Error("Frame timeout")); }
        }, timeoutDuration);
        transport.pending.set(id, { resolve, reject, timeout, startTime });
    });
        transport.socket.send(new Uint8Array(buf));
    const buffer = await promise;
    const bytes = new Uint8Array(buffer);
    const dvRes = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    if (bytes.length < 13 || bytes[0] !== 0xCF || bytes[1] !== 0x86 || bytes[2] !== 1) return [];
    let o = 3;
    o += 4;
    const pointCount = dvRes.getUint32(o, true); o += 4;
    const result = [];
    for (let pi = 0; pi < pointCount; pi++) {
        if (o + 4 > bytes.length) break;
        const objCount = dvRes.getUint32(o, true); o += 4;
        for (let oi = 0; oi < objCount; oi++) {
            if (o >= bytes.length) break;
            const sfCount = bytes[o++];
            for (let s = 0; s < sfCount; s++) {
                const nl = bytes[o++];
                let name = '';
                for (let i = 0; i < nl; i++) name += String.fromCharCode(bytes[o++]);
                o++;
                if (o + 40 > bytes.length) break;
                const val = dvRes.getFloat64(o, true); o += 8;
                const t = dvRes.getFloat64(o, true); o += 8;
                const x = dvRes.getFloat64(o, true); o += 8;
                const y = dvRes.getFloat64(o, true); o += 8;
                const z = dvRes.getFloat64(o, true); o += 8;
                result.push({ name, val, t, x, y, z });
            }
        }
    }
    return result;
}
