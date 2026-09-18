import { test } from "node:test";
import assert from "node:assert/strict";
import { parseVerdicts, VERDICT_TAG } from "./constants.js";

function frame(lines) {
  const enc = new TextEncoder();
  let size = 7;
  for (const l of lines) {
    size += 1 + enc.encode(l.name).length + 1 + 2 + 1 + (l.sep == null ? 0 : 8) + 8;
  }
  const buf = new ArrayBuffer(size);
  const dv = new DataView(buf);
  const bytes = new Uint8Array(buf);
  bytes[0] = 0xcf;
  bytes[1] = 0x86;
  bytes[2] = VERDICT_TAG;
  dv.setUint32(3, lines.length, true);
  let o = 7;
  for (const l of lines) {
    const name = enc.encode(l.name);
    dv.setUint8(o, name.length);
    o += 1;
    bytes.set(name, o);
    o += name.length;
    dv.setUint8(o, l.word);
    o += 1;
    dv.setUint8(o, l.ka);
    dv.setUint8(o + 1, l.kb);
    o += 2;
    if (l.sep == null) {
      dv.setUint8(o, 0);
      o += 1;
    } else {
      dv.setUint8(o, 1);
      o += 1;
      dv.setFloat64(o, l.sep, true);
      o += 8;
    }
    dv.setFloat64(o, l.weave, true);
    o += 8;
  }
  return bytes;
}

test("parseVerdicts: a riss keeps its name and both witness lines", () => {
  const bytes = frame([{ name: "apophis", word: 3, ka: 0, kb: 3, sep: 2.3e9, weave: 8.0e8 }]);
  const lines = parseVerdicts(bytes);
  assert.equal(lines.length, 1);
  assert.deepEqual(lines[0], {
    name: "apophis",
    word: "riss",
    knot: ["spk-ephemeris", "inpop-ephemeris"],
    sep: 2.3e9,
    weave: 8.0e8,
  });
});

test("parseVerdicts: an absent body carries no fabricated sep", () => {
  const bytes = frame([{ name: "vesta", word: 1, ka: 2, kb: 255, sep: null, weave: 8.0e8 }]);
  const lines = parseVerdicts(bytes);
  assert.equal(lines[0].word, "absent");
  assert.equal(lines[0].sep, null);
  assert.deepEqual(lines[0].knot, ["mpc-keplerian", null]);
});

test("parseVerdicts: a placed body reads placed", () => {
  const bytes = frame([{ name: "ceres", word: 0, ka: 255, kb: 255, sep: 2.5e4, weave: 8.0e8 }]);
  assert.equal(parseVerdicts(bytes)[0].word, "placed");
});

test("parseVerdicts: an empty frame reads no verdict", () => {
  assert.deepEqual(parseVerdicts(frame([])), []);
});
