import { test } from "node:test";
import assert from "node:assert/strict";
import { createRadiator } from "./radiator.js";

const OMEGA = [1, 2, 3, 4, 5, 6, 7, 8, 9];

test("the radiator peer set is flat and closed", () => {
  const radiator = createRadiator({ consent: () => true, serial: null });
  assert.deepEqual(radiator.peers(), ["audio", "vibration", "serial", "usb", "bluetooth", "hid"]);
});

test("one frame is dispatched to every peer equally", () => {
  let serialFrames = 0;
  const serial = { onFrame() { serialFrames += 1; } };
  const radiator = createRadiator({ consent: () => true, serial });
  radiator.start();
  radiator.onFrame(OMEGA, 1, 0.5, 0.25);
  assert.equal(serialFrames, 1);
});

test("a withheld consent silences every peer", () => {
  let serialFrames = 0;
  const serial = { onFrame() { serialFrames += 1; } };
  const radiator = createRadiator({ consent: () => false, serial });
  radiator.start();
  radiator.onFrame(OMEGA, 1);
  assert.equal(serialFrames, 0);
});

test("a stopped radiator dispatches nothing", () => {
  let serialFrames = 0;
  const serial = { onFrame() { serialFrames += 1; } };
  const radiator = createRadiator({ consent: () => true, serial });
  radiator.start();
  radiator.stop();
  radiator.onFrame(OMEGA, 1);
  assert.equal(serialFrames, 0);
});

test("the audio peer carries the raw Σω sequence, never a synthesized partial", async () => {
  const previousAC = globalThis.AudioContext;
  const previousAWN = globalThis.AudioWorkletNode;
  const previousCreate = URL.createObjectURL;
  const previousRevoke = URL.revokeObjectURL;
  const posted = [];
  class FakeWorklet {
    constructor() {
      this.port = { postMessage: (value) => posted.push(value) };
    }
    connect() {}
    disconnect() {}
  }
  class FakeAudioContext {
    constructor() {
      this.audioWorklet = { addModule: async () => {} };
      this.destination = {};
    }
    close() {}
  }
  globalThis.AudioContext = FakeAudioContext;
  globalThis.AudioWorkletNode = FakeWorklet;
  URL.createObjectURL = () => "blob:test";
  URL.revokeObjectURL = () => {};
  try {
    const radiator = createRadiator({ consent: () => true, serial: null });
    radiator.start();
    await new Promise((resolve) => setTimeout(resolve, 0));
    radiator.onFrame([1, 1, 1, 1, 1, 1, 1, 1, 1], 1);
    assert.deepEqual(posted, [9]);
    radiator.onFrame([1, 1, 1, 1, 1, 1, 1, 1, 1], 0.5);
    assert.deepEqual(posted, [9, 4.5]);
    radiator.stop();
  } finally {
    if (previousAC === undefined) {
      delete globalThis.AudioContext;
    } else {
      globalThis.AudioContext = previousAC;
    }
    if (previousAWN === undefined) {
      delete globalThis.AudioWorkletNode;
    } else {
      globalThis.AudioWorkletNode = previousAWN;
    }
    URL.createObjectURL = previousCreate;
    URL.revokeObjectURL = previousRevoke;
  }
});
