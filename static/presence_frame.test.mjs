import { test } from "node:test";
import assert from "node:assert/strict";
import { encodeFrame, consented, makeWriter } from "./presence_frame.js";

const OMEGA = [1, 2, 3, 4, 5, 6, 7, 8, 9];

test("encodeFrame: Σω·aperture as tagged f32 little-endian", () => {
  const bytes = encodeFrame(OMEGA, 1);
  assert.ok(bytes instanceof Uint8Array);
  assert.equal(bytes.length, 6);
  assert.deepEqual(Array.from(bytes), [0x02, 0x01, 0x00, 0x00, 0x34, 0x42]);
});

test("encodeFrame: aperture 0 yields the tag then four zero bytes", () => {
  assert.deepEqual(Array.from(encodeFrame(OMEGA, 0)), [0x02, 0x01, 0, 0, 0, 0]);
});

test("encodeFrame: half aperture keeps the f32 shape", () => {
  const bytes = encodeFrame([1, 0, 0, 0, 0, 0, 0, 0, 0], 0.5);
  assert.deepEqual(Array.from(bytes), [0x02, 0x01, 0x00, 0x00, 0x00, 0x3f]);
});

test("encodeFrame: absent (non-finite) still returns null", () => {
  assert.equal(encodeFrame([1, 2, NaN, 4, 5, 6, 7, 8, 9], 1), null);
  assert.equal(encodeFrame(OMEGA, Infinity), null);
});

test("encodeFrame: non-finite omega is absent, not a value", () => {
  assert.equal(encodeFrame([1, 2, NaN, 4, 5, 6, 7, 8, 9], 1), null);
  assert.equal(encodeFrame([1, 2, Infinity, 4, 5, 6, 7, 8, 9], 1), null);
  assert.equal(encodeFrame([1, 2, -Infinity, 4, 5, 6, 7, 8, 9], 1), null);
});

test("encodeFrame: non-finite aperture is absent", () => {
  assert.equal(encodeFrame(OMEGA, NaN), null);
  assert.equal(encodeFrame(OMEGA, Infinity), null);
});

test("encodeFrame: missing omega is absent", () => {
  assert.equal(encodeFrame(null, 1), null);
  assert.equal(encodeFrame(undefined, 1), null);
  assert.equal(encodeFrame([1, 2, 3], 1), null);
});

test("consented: reads the api's consent gate", () => {
  assert.equal(consented({ consent: () => true }), true);
  assert.equal(consented({ consent: () => false }), false);
});

test("consented: absent api or gate is false", () => {
  assert.equal(consented(null), false);
  assert.equal(consented(undefined), false);
  assert.equal(consented({}), false);
  assert.equal(consented({ consent: 0 }), false);
});

test("makeWriter: consented writes one 6-byte frame to the simulated CDC", () => {
  const writes = [];
  const port = { write(bytes) { writes.push(bytes); } };
  makeWriter(port, true).onFrame(OMEGA, 1);
  assert.equal(writes.length, 1);
  assert.equal(writes[0].length, 6);
  assert.deepEqual(Array.from(writes[0]), Array.from(encodeFrame(OMEGA, 1)));
});

test("makeWriter: withheld consent writes nothing", () => {
  const writes = [];
  const port = { write(bytes) { writes.push(bytes); } };
  makeWriter(port, false).onFrame(OMEGA, 1);
  assert.equal(writes.length, 0);
});

test("makeWriter: absent writer writes nothing", () => {
  assert.doesNotThrow(() => makeWriter(null, true).onFrame(OMEGA, 1));
});

test("makeWriter: a non-finite frame is not written", () => {
  const writes = [];
  const port = { write(bytes) { writes.push(bytes); } };
  makeWriter(port, true).onFrame([1, 2, NaN, 4, 5, 6, 7, 8, 9], 1);
  assert.equal(writes.length, 0);
});

test("makeWriter: consent callback is read per frame", () => {
  const writes = [];
  const port = { write(bytes) { writes.push(bytes); } };
  let allow = false;
  const frame = makeWriter(port, () => allow);
  frame.onFrame(OMEGA, 1);
  allow = true;
  frame.onFrame(OMEGA, 1);
  assert.equal(writes.length, 1);
});
