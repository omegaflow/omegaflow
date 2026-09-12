export const PROTOCOL_VERSION = 9;
export const KINETIC_TAG = 10;
export const RECORD_BYTES = 208;
export const FRAME_HEADER = 19;

const enc = new TextEncoder();

export function syncFrame(id, inputs, queries, presence) {
  let size = 8 + inputs.length * 17 + queries.length * 32 + 11 * 8;
  const inputNames = [];
  for (const inp of inputs) {
    const name = enc.encode(inp.name);
    inputNames.push(name);
    size += name.length;
  }
  const buf = new ArrayBuffer(size);
  const dv = new DataView(buf);
  let o = 0;
  dv.setUint32(o, id, true);
  o += 4;
  dv.setUint32(o, inputs.length, true);
  o += 4;
  for (let i = 0; i < inputs.length; i++) {
    const name = inputNames[i];
    dv.setFloat64(o, inputs[i].value, true);
    o += 8;
    dv.setUint8(o, name.length);
    o += 1;
    for (let k = 0; k < name.length; k++) {
      dv.setUint8(o + k, name[k]);
    }
    o += name.length;
    dv.setFloat64(o, inputs[i].tau, true);
    o += 8;
  }
  dv.setUint32(o, queries.length, true);
  o += 4;
  for (const q of queries) {
    dv.setFloat64(o, q[0], true);
    o += 8;
    dv.setFloat64(o, q[1], true);
    o += 8;
    dv.setFloat64(o, q[2], true);
    o += 8;
    dv.setFloat64(o, q[3], true);
    o += 8;
  }
  dv.setFloat64(o, presence.x, true);
  o += 8;
  dv.setFloat64(o, presence.y, true);
  o += 8;
  dv.setFloat64(o, presence.z, true);
  o += 8;
  dv.setFloat64(o, presence.t, true);
  o += 8;
  dv.setFloat64(o, presence.range, true);
  o += 8;
  dv.setFloat64(o, presence.cacheInterval, true);
  o += 8;
  dv.setFloat64(o, presence.vx, true);
  o += 8;
  dv.setFloat64(o, presence.vy, true);
  o += 8;
  dv.setFloat64(o, presence.vz, true);
  o += 8;
  dv.setFloat64(o, presence.thrustT, true);
  o += 8;
  dv.setFloat64(o, presence.gridStep, true);
  o += 8;
  return buf;
}

export function parseRecords(bytes) {
  if (bytes.byteLength < FRAME_HEADER) {
    throw new Error("frame short");
  }
  const dv = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  if (dv.getUint8(0) !== 0xcf || dv.getUint8(1) !== 0x86) {
    throw new Error("frame magic");
  }
  if (dv.getUint8(2) !== PROTOCOL_VERSION) {
    throw new Error("frame version");
  }
  const epoch = dv.getFloat64(3, true);
  const id = dv.getUint32(11, true);
  const count = dv.getUint32(15, true);
  const field = new Float32Array(count * 12);
  const meta = new Float32Array(count * 16);
  for (let j = 0; j < count; j++) {
    const b = FRAME_HEADER + j * RECORD_BYTES;
    const x = dv.getFloat64(b, true);
    const y = dv.getFloat64(b + 8, true);
    const z = dv.getFloat64(b + 16, true);
    const val = dv.getFloat64(b + 24, true);
    const t = dv.getFloat64(b + 32, true);
    const ttl = dv.getFloat64(b + 40, true);
    const tau = dv.getFloat64(b + 48, true);
    const extent = dv.getFloat64(b + 56, true);
    const kernelId = dv.getFloat64(b + 64, true);
    const forceType = dv.getFloat64(b + 72, true);
    const absorption = dv.getFloat64(b + 80, true);
    const advection = dv.getFloat64(b + 88, true);
    const vx = dv.getFloat64(b + 96, true);
    const vy = dv.getFloat64(b + 104, true);
    const vz = dv.getFloat64(b + 112, true);
    const poleX = dv.getFloat64(b + 120, true);
    const poleY = dv.getFloat64(b + 128, true);
    const poleZ = dv.getFloat64(b + 136, true);
    const j2 = dv.getFloat64(b + 144, true);
    const j4 = dv.getFloat64(b + 152, true);
    const rEq = dv.getFloat64(b + 160, true);
    const colorIndex = dv.getFloat64(b + 168, true);
    const freq = dv.getFloat64(b + 176, true);
    const binWidth = dv.getFloat64(b + 184, true);
    const phase = dv.getFloat64(b + 192, true);
    const presence = dv.getFloat64(b + 200, true);

    const f = j * 12;
    field[f] = x;
    field[f + 1] = y;
    field[f + 2] = z;
    field[f + 3] = val;
    field[f + 4] = t;
    field[f + 5] = ttl;
    field[f + 6] = forceType;
    field[f + 7] = absorption;
    field[f + 8] = advection;
    field[f + 9] = vx;
    field[f + 10] = vy;
    field[f + 11] = vz;

    const m = j * 16;
    meta[m] = extent;
    meta[m + 1] = tau;
    meta[m + 2] = kernelId;
    meta[m + 3] = forceType === 0 ? poleX : 0;
    meta[m + 4] = poleX;
    meta[m + 5] = poleY;
    meta[m + 6] = poleZ;
    meta[m + 7] = j2;
    meta[m + 8] = j4;
    meta[m + 9] = rEq;
    meta[m + 10] = colorIndex;
    meta[m + 11] = freq;
    meta[m + 12] = binWidth;
    meta[m + 13] = phase;
    meta[m + 14] = presence;
    meta[m + 15] = 0;
  }
  return { epoch, id, count, field, meta };
}

export function parseKinetic(bytes) {
  const dv = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const omega = new Float32Array(bytes.buffer, bytes.byteOffset + 3, 9);
  const aperture = dv.getFloat32(3 + 9 * 4, true);
  return { omega, aperture };
}
