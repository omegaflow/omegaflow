import { test } from "node:test";
import assert from "node:assert/strict";
import { parseColorLut, colorLutIndex, colorForCiLut, COLOR_LUT_LEN } from "./constants.js";

// The transport fixture mirrors the /color_lut wire: [lo:f32][hi:f32] then
// 256 × [r,g,b,a] f32, little-endian (Rust color_lut_wire()).
function lutBytes(lo, hi, fn) {
  const buf = new ArrayBuffer(8 + COLOR_LUT_LEN * 16);
  const dv = new DataView(buf);
  dv.setFloat32(0, lo, true);
  dv.setFloat32(4, hi, true);
  for (let i = 0; i < COLOR_LUT_LEN; i++) {
    const rgb = fn(i);
    dv.setFloat32(8 + i * 16, rgb[0], true);
    dv.setFloat32(8 + i * 16 + 4, rgb[1], true);
    dv.setFloat32(8 + i * 16 + 8, rgb[2], true);
    dv.setFloat32(8 + i * 16 + 12, 1.0, true);
  }
  return new Uint8Array(buf);
}

test("parseColorLut: reads the bounds and the 256 RGBA texels", () => {
  const bytes = lutBytes(-0.12, 5.1, (i) => [i / 255, 0, 1]);
  const { lo, hi, lut } = parseColorLut(bytes);
  assert.equal(lo, Math.fround(-0.12));
  assert.equal(hi, Math.fround(5.1));
  assert.equal(lut.length, COLOR_LUT_LEN * 4);
  assert.equal(lut[0], 0);
  assert.equal(lut[255 * 4], 1);
  assert.equal(lut[255 * 4 + 3], 1);
});
test("colorLutIndex: zero is the white sentinel, the ends clamp", () => {
  assert.equal(colorLutIndex(0.0, -0.12, 5.1), -1);
  assert.equal(colorLutIndex(-1.0, -0.12, 5.1), 0);
  assert.equal(colorLutIndex(6.0, -0.12, 5.1), 255);
  assert.equal(colorLutIndex(2.49, -0.12, 5.1), 128);
});

test("colorForCiLut: zero renders white, every other ci reads its texel", () => {
  const bytes = lutBytes(-0.12, 5.1, (i) => [i / 255, 0.5, 1]);
  const { lo, hi, lut } = parseColorLut(bytes);
  assert.deepEqual(colorForCiLut(0.0, lo, hi, lut), [1, 1, 1]);
  const ci = 2.49;
  const idx = colorLutIndex(ci, lo, hi);
  assert.ok(idx >= 0 && idx < COLOR_LUT_LEN, `index ${idx} outside the LUT`);
  assert.deepEqual(colorForCiLut(ci, lo, hi, lut), [
    lut[idx * 4],
    lut[idx * 4 + 1],
    lut[idx * 4 + 2],
  ]);
});
