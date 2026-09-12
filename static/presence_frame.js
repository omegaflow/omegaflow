// The radiatorium's write law, pure: the membrane's PresenceFrame (9 omegas +
// aperture) becomes one tagged frame — [0x02, 0x01, Σω·aperture as f32-LE] (6 B).
// No DOM, no window — the CDC interface enters as an argument, so the law is
// testable against a simulated port. Absent means null (0-Kanon): NaN/Inf is
// never a value.

const FRAME_TAG = 0x02;
const MASK_INTENSITY = 0x01;

export function encodeFrame(omega, aperture) {
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
  const bytes = new Uint8Array(6);
  bytes[0] = FRAME_TAG;
  bytes[1] = MASK_INTENSITY;
  new DataView(bytes.buffer).setFloat32(2, sum * aperture, true);
  return bytes;
}

export function consented(api) {
  return Boolean(api && api.consent && api.consent());
}

export function makeWriter(writer, consented) {
  return {
    onFrame(omega, aperture) {
      const port = typeof writer === "function" ? writer() : writer;
      const allowed = typeof consented === "function" ? consented() : Boolean(consented);
      if (!allowed || !port) {
        return;
      }
      const bytes = encodeFrame(omega, aperture);
      if (!bytes) {
        return;
      }
      Promise.resolve(port.write(bytes)).catch(() => {});
    },
  };
}
