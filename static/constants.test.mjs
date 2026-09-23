import { test } from "node:test";
import assert from "node:assert/strict";
import { parseRecords } from "./constants.js";

// one 26 × f64 record (208 B), protocol v9 — the phase slot at byte 192,
// the presence flag at byte 200 (spectral-oscillator.md Atom D)
function frameWith(phase, presence, freq = 0, binWidth = 0) {
  const buf = new ArrayBuffer(19 + 208);
  const dv = new DataView(buf);
  dv.setUint8(0, 0xcf);
  dv.setUint8(1, 0x86);
  dv.setUint8(2, 9);
  dv.setFloat64(3, 1.0e9, true);
  dv.setUint32(11, 7, true);
  dv.setUint32(15, 1, true);
  const b = 19;
  dv.setFloat64(b, 1.0, true);
  dv.setFloat64(b + 8, 2.0, true);
  dv.setFloat64(b + 16, 3.0, true);
  dv.setFloat64(b + 24, 4.0, true);
  dv.setFloat64(b + 32, 1.0e9, true);
  dv.setFloat64(b + 40, 60.0, true);
  dv.setFloat64(b + 48, 6.0, true);
  dv.setFloat64(b + 56, 1.0, true);
  dv.setFloat64(b + 64, 0.0, true);
  dv.setFloat64(b + 72, 0.0, true);
  dv.setFloat64(b + 80, 0.5, true);
  dv.setFloat64(b + 88, 0.0, true);
  dv.setFloat64(b + 168, 0.0, true); // color_index
  dv.setFloat64(b + 176, freq, true);
  dv.setFloat64(b + 184, binWidth, true);
  dv.setFloat64(b + 192, phase, true);
  dv.setFloat64(b + 200, presence, true);
  return new Uint8Array(buf);
}

test("parseRecords: the phase slot and the presence flag ride the meta row", () => {
  const { meta } = parseRecords(frameWith(0.5, 1.0, 1.0e6, 0.0));
  assert.equal(meta[13], 0.5, "meta[13] is the phase (rad)");
  assert.equal(meta[14], 1.0, "meta[14] is the presence flag");
  assert.equal(meta[11], 1.0e6, "meta[11] is the freq");
  assert.equal(meta[12], 0.0, "meta[12] is the null-echt point-source bin_width");
});

test("parseRecords: 0 rad is a real angle — the flag disambiguates, never the pad", () => {
  const { meta } = parseRecords(frameWith(0.0, 1.0));
  assert.equal(meta[13], 0.0);
  assert.equal(meta[14], 1.0, "presence 1 marks a measured zero phase");
});

test("parseRecords: absent phase stays pad 0 with presence 0", () => {
  const { meta } = parseRecords(frameWith(0.0, 0.0));
  assert.equal(meta[14], 0.0, "the reader reads the flag, never the pad");
});
