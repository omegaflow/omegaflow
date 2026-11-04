// The sensorium: the operator's own devices as oscillators in the field.
//
// Every sensor speaks only while consent() is true — the machine asks before it
// records, and a revoked consent stops every reader in the same breath. A device
// the browser does not carry stays absent (0 honored): the sensor is never
// fabricated, a missing API is a silent peer, not a zero.
//
// Names are chosen so the Archivar's sensor_config maps them to a force: motion
// and gyro (seismic), gravity (gravity), magnetometer and light and camera (em),
// microphone (acoustic), battery (electric), gamepad and orientation as events.
// The sensorium never invents a value: a non-finite reading is dropped.

const GENERIC = [
  ["Accelerometer", "accelerometer", (s) => ({ "accelerometer.x": s.x, "accelerometer.y": s.y, "accelerometer.z": s.z })],
  ["Gyroscope", "gyroscope", (s) => ({ "gyroscope.x": s.x, "gyroscope.y": s.y, "gyroscope.z": s.z })],
  ["Magnetometer", "magnetometer", (s) => ({ "magnetometer.x": s.x, "magnetometer.y": s.y, "magnetometer.z": s.z })],
  ["LinearAccelerationSensor", "linear_acceleration", (s) => ({ "linear_acceleration.x": s.x, "linear_acceleration.y": s.y, "linear_acceleration.z": s.z })],
  ["GravitySensor", "gravity", (s) => ({ "gravity.x": s.x, "gravity.y": s.y, "gravity.z": s.z })],
  ["AmbientLightSensor", "light", (s) => ({ "light.illuminance": s.illuminance })],
  ["AbsoluteOrientationSensor", "orientation", (s) => (s.quaternion ? { "event.orientation.x": s.quaternion[0], "event.orientation.y": s.quaternion[1], "event.orientation.z": s.quaternion[2], "event.orientation.w": s.quaternion[3] } : null)],
];

export function createSensorium({ record, consent, onState }) {
  const cleanups = new Map();
  let running = false;
  let count = 0;

  function emit(name, value) {
    if (!running || !consent() || !Number.isFinite(value)) {
      return;
    }
    record(name, value, 0);
  }

  function emitAll(map) {
    if (!map) {
      return;
    }
    for (const name of Object.keys(map)) {
      emit(name, map[name]);
    }
  }

  function note() {
    if (onState) {
      onState(count, running);
    }
  }

  function add(key, cleanup) {
    if (cleanups.has(key)) {
      return;
    }
    cleanups.set(key, cleanup);
    count += 1;
    note();
  }

  function release(key) {
    const cleanup = cleanups.get(key);
    if (cleanup) {
      cleanups.delete(key);
      try {
        cleanup();
      } catch {}
      count -= 1;
    }
  }

  function startGeneric(name, key, map) {
    const Ctor = globalThis[name];
    if (typeof Ctor !== "function" || cleanups.has(key)) {
      return;
    }
    try {
      const sensor = new Ctor({ frequency: 10 });
      const onReading = () => emitAll(map(sensor));
      sensor.addEventListener("reading", onReading);
      sensor.start();
      add(key, () => {
        sensor.removeEventListener("reading", onReading);
        try { sensor.stop(); } catch {}
      });
    } catch {}
  }

  function startMotion() {
    if (typeof globalThis.DeviceMotionEvent === "undefined" || cleanups.has("motion")) {
      return;
    }
    const onMotion = (event) => {
      const a = event.accelerationIncludingGravity || event.acceleration;
      if (a) {
        emit("accelerometer.x", a.x);
        emit("accelerometer.y", a.y);
        emit("accelerometer.z", a.z);
      }
      const r = event.rotationRate;
      if (r) {
        const d = Math.PI / 180;
        emit("gyroscope.x", r.alpha * d);
        emit("gyroscope.y", r.beta * d);
        emit("gyroscope.z", r.gamma * d);
      }
    };
    globalThis.addEventListener("devicemotion", onMotion);
    add("motion", () => globalThis.removeEventListener("devicemotion", onMotion));
  }

  function startOrientation() {
    if (typeof globalThis.DeviceOrientationEvent === "undefined" || cleanups.has("orientation_event")) {
      return;
    }
    const onOrientation = (event) => {
      if (event.alpha == null) {
        return;
      }
      const d = Math.PI / 180;
      emit("event.orientation.alpha", event.alpha * d);
      emit("event.orientation.beta", event.beta * d);
      emit("event.orientation.gamma", event.gamma * d);
    };
    globalThis.addEventListener("deviceorientation", onOrientation);
    add("orientation_event", () => globalThis.removeEventListener("deviceorientation", onOrientation));
  }

  async function startMicrophone() {
    if (!navigator.mediaDevices || !navigator.mediaDevices.getUserMedia || cleanups.has("microphone")) {
      return;
    }
    let context = null;
    let stream = null;
    let timer = null;
    try {
      const AC = globalThis.AudioContext || globalThis.webkitAudioContext;
      context = new AC();
      stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      if (!running || !consent()) {
        stream.getTracks().forEach((t) => t.stop());
        try { context.close(); } catch {}
        return;
      }
      const analyser = context.createAnalyser();
      analyser.fftSize = 512;
      context.createMediaStreamSource(stream).connect(analyser);
      const bins = new Uint8Array(analyser.frequencyBinCount);
      const bands = 8;
      const step = Math.max(1, Math.floor(bins.length / bands));
      timer = setInterval(() => {
        analyser.getByteFrequencyData(bins);
        let sum = 0;
        for (let b = 0; b < bands; b++) {
          let peak = 0;
          for (let i = b * step; i < (b + 1) * step && i < bins.length; i++) {
            if (bins[i] > peak) {
              peak = bins[i];
            }
          }
          emit("microphone.freq_" + b, peak / 255);
          sum += peak;
        }
        emit("microphone.amplitude", sum / (bands * 255));
      }, 100);
      add("microphone", () => {
        clearInterval(timer);
        stream.getTracks().forEach((t) => t.stop());
        try { context.close(); } catch {}
      });
    } catch {
      if (stream) {
        stream.getTracks().forEach((t) => t.stop());
      }
      if (context) {
        try { context.close(); } catch {}
      }
    }
  }

  async function startCamera() {
    if (!navigator.mediaDevices || !navigator.mediaDevices.getUserMedia || cleanups.has("camera")) {
      return;
    }
    let stream = null;
    let timer = null;
    try {
      stream = await navigator.mediaDevices.getUserMedia({ video: true });
      if (!running || !consent()) {
        stream.getTracks().forEach((t) => t.stop());
        return;
      }
      const video = document.createElement("video");
      video.srcObject = stream;
      video.muted = true;
      video.playsInline = true;
      await video.play();
      const surface = document.createElement("canvas");
      surface.width = 32;
      surface.height = 24;
      const c2d = surface.getContext("2d", { willReadFrequently: true });
      timer = setInterval(() => {
        if (!video.videoWidth) {
          return;
        }
        c2d.drawImage(video, 0, 0, surface.width, surface.height);
        const data = c2d.getImageData(0, 0, surface.width, surface.height).data;
        let sum = 0;
        for (let i = 0; i < data.length; i += 4) {
          sum += (data[i] + data[i + 1] + data[i + 2]) / 765;
        }
        emit("camera.brightness", sum / (surface.width * surface.height));
      }, 500);
      add("camera", () => {
        clearInterval(timer);
        stream.getTracks().forEach((t) => t.stop());
      });
    } catch {
      if (stream) {
        stream.getTracks().forEach((t) => t.stop());
      }
    }
  }

  async function startBattery() {
    if (typeof navigator.getBattery !== "function" || cleanups.has("battery")) {
      return;
    }
    try {
      const battery = await navigator.getBattery();
      if (!running || !consent()) {
        return;
      }
      const push = () => {
        emit("battery.level", battery.level);
        emit("battery.charging", battery.charging ? 1 : 0);
      };
      const events = ["levelchange", "chargingchange"];
      events.forEach((e) => battery.addEventListener(e, push));
      push();
      add("battery", () => events.forEach((e) => battery.removeEventListener(e, push)));
    } catch {}
  }

  function startGeolocation() {
    if (!navigator.geolocation || cleanups.has("geolocation")) {
      return;
    }
    const id = navigator.geolocation.watchPosition((position) => {
      const c = position.coords;
      emit("lat", c.latitude);
      emit("lon", c.longitude);
      emit("alt", c.altitude);
    }, () => {}, { enableHighAccuracy: true });
    add("geolocation", () => navigator.geolocation.clearWatch(id));
  }

  function startGamepad() {
    if (typeof navigator.getGamepads !== "function" || cleanups.has("gamepad")) {
      return;
    }
    const timer = setInterval(() => {
      const pads = navigator.getGamepads ? navigator.getGamepads() : [];
      for (let p = 0; p < pads.length; p++) {
        const gp = pads[p];
        if (!gp) {
          continue;
        }
        for (let a = 0; a < gp.axes.length; a++) {
          emit("event.gamepad." + p + ".axis." + a, gp.axes[a]);
        }
        for (let b = 0; b < gp.buttons.length; b++) {
          emit("event.gamepad." + p + ".button." + b, gp.buttons[b].value);
        }
      }
    }, 50);
    add("gamepad", () => clearInterval(timer));
  }

  async function startXR() {
    if (!navigator.xr || !navigator.xr.requestSession || cleanups.has("xr")) {
      return;
    }
    try {
      const session = await navigator.xr.requestSession("inline");
      if (!running || !consent()) {
        session.end();
        return;
      }
      const refSpace = await session.requestReferenceSpace("local");
      const onFrame = (time, frame) => {
        const pose = frame.getViewerPose(refSpace);
        if (pose) {
          const p = pose.transform.position;
          const o = pose.transform.orientation;
          emit("event.xr.headpose.x", p.x);
          emit("event.xr.headpose.y", p.y);
          emit("event.xr.headpose.z", p.z);
          emit("event.xr.orientation.x", o.x);
          emit("event.xr.orientation.y", o.y);
          emit("event.xr.orientation.z", o.z);
          emit("event.xr.orientation.w", o.w);
        }
        session.requestAnimationFrame(onFrame);
      };
      session.requestAnimationFrame(onFrame);
      add("xr", () => session.end());
    } catch {}
  }

  function start() {
    if (running) {
      return;
    }
    running = true;
    for (const [name, key, map] of GENERIC) {
      startGeneric(name, key, map);
    }
    startMotion();
    startOrientation();
    startGeolocation();
    startMicrophone();
    startCamera();
    startBattery();
    startGamepad();
    startXR();
  }

  function stop() {
    running = false;
    for (const key of [...cleanups.keys()]) {
      release(key);
    }
    count = 0;
    note();
  }

  return { start, stop, active: () => running, size: () => count };
}
