import { test } from "node:test";
import assert from "node:assert/strict";
import { createSensorium } from "./sensorium.js";

class FakeSensor {
  static instances = [];
  constructor() {
    this.listeners = [];
    this.started = false;
    this.x = 1.5;
    this.y = -2.5;
    this.z = 0.5;
    FakeSensor.instances.push(this);
  }
  addEventListener(type, fn) {
    if (type === "reading") {
      this.listeners.push(fn);
    }
  }
  removeEventListener(type, fn) {
    this.listeners = this.listeners.filter((f) => f !== fn);
  }
  start() {
    this.started = true;
  }
  stop() {
    this.started = false;
  }
  read() {
    this.listeners.forEach((fn) => fn());
  }
}

function withFakeAccelerometer(run) {
  const previous = globalThis.Accelerometer;
  globalThis.Accelerometer = FakeSensor;
  FakeSensor.instances = [];
  try {
    run();
  } finally {
    if (previous === undefined) {
      delete globalThis.Accelerometer;
    } else {
      globalThis.Accelerometer = previous;
    }
  }
}

function harness(consent) {
  const readings = [];
  const sensorium = createSensorium({
    record: (name, value, tau) => readings.push({ name, value, tau }),
    consent: () => consent.value,
  });
  return { sensorium, readings };
}

test("the sensorium records a reading only while consented", () => {
  withFakeAccelerometer(() => {
    const consent = { value: true };
    const h = harness(consent);
    h.sensorium.start();
    FakeSensor.instances[0].read();
    assert.deepEqual(h.readings, [
      { name: "accelerometer.x", value: 1.5, tau: 0 },
      { name: "accelerometer.y", value: -2.5, tau: 0 },
      { name: "accelerometer.z", value: 0.5, tau: 0 },
    ]);
  });
});

test("a revoked consent silences the sensor in the same breath", () => {
  withFakeAccelerometer(() => {
    const consent = { value: true };
    const h = harness(consent);
    h.sensorium.start();
    FakeSensor.instances[0].read();
    consent.value = false;
    FakeSensor.instances[0].read();
    assert.equal(h.readings.length, 3);
  });
});

test("a stopped sensorium records nothing", () => {
  withFakeAccelerometer(() => {
    const consent = { value: true };
    const h = harness(consent);
    h.sensorium.start();
    h.sensorium.stop();
    FakeSensor.instances[0].read();
    assert.equal(h.readings.length, 0);
  });
});

test("a non-finite reading stays absent, never a fabricated zero", () => {
  withFakeAccelerometer(() => {
    const consent = { value: true };
    const h = harness(consent);
    h.sensorium.start();
    const sensor = FakeSensor.instances[0];
    sensor.x = NaN;
    sensor.y = Infinity;
    sensor.z = 3.5;
    sensor.read();
    assert.deepEqual(h.readings, [{ name: "accelerometer.z", value: 3.5, tau: 0 }]);
  });
});

test("the sensorium reports its size through onState", () => {
  withFakeAccelerometer(() => {
    const consent = { value: true };
    const sizes = [];
    const sensorium = createSensorium({
      record: () => {},
      consent: () => consent.value,
      onState: (count) => sizes.push(count),
    });
    sensorium.start();
    sensorium.stop();
    assert.deepEqual(sizes, [1, 0]);
  });
});
