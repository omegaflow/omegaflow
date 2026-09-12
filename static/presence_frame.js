// The radiatorium's write law, pure: the membrane's PresenceFrame (9 omegas +
// aperture + optional pan/tilt pulse widths) becomes one tagged frame —
// [0x02, mask, Σω·aperture as f32-LE, pan f32-LE?, tilt f32-LE?]. No DOM, no
// window — the CDC interface enters as an argument, so the law is testable
// against a simulated port. Absent means null (0-Kanon): NaN/Inf is never a
// value.

const FRAME_TAG = 0x02;
const MASK_INTENSITY = 0x01;
const MASK_PAN = 0x02;
const MASK_TILT = 0x04;

export function encodeFrame(omega, aperture, pan, tilt) {
  if (!omega || !Number.isFinite(aperture)) {
    return null;
  }
  let sum = 0;
  for (let i = 0; i < 9; i++) {
    const value = omega[i];
    if (!Number.isFinite(value)) {
      return null;
    }
    sum += value;
  }
  const hasPan = Number.isFinite(pan);
  const hasTilt = Number.isFinite(tilt);
  const mask = MASK_INTENSITY | (hasPan ? MASK_PAN : 0) | (hasTilt ? MASK_TILT : 0);
  const bytes = new Uint8Array(6 + (hasPan ? 4 : 0) + (hasTilt ? 4 : 0));
  const dv = new DataView(bytes.buffer);
  bytes[0] = FRAME_TAG;
  bytes[1] = mask;
  dv.setFloat32(2, sum * aperture, true);
  let o = 6;
  if (hasPan) {
    dv.setFloat32(o, pan, true);
    o += 4;
  }
  if (hasTilt) {
    dv.setFloat32(o, tilt, true);
  }
  return bytes;
}

export function consented(api) {
  return Boolean(api && api.consent && api.consent());
}

export function makeWriter(writer, consented) {
  return {
    onFrame(omega, aperture, pan, tilt) {
      const port = typeof writer === "function" ? writer() : writer;
      const allowed = typeof consented === "function" ? consented() : Boolean(consented);
      if (!allowed || !port) {
        return;
      }
      const bytes = encodeFrame(omega, aperture, pan, tilt);
      if (!bytes) {
        return;
      }
      Promise.resolve(port.write(bytes)).catch(() => {});
    },
  };
}
