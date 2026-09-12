// WebSerial path to the ESP32 radiatorium (CDC-ACM). The device firmware
// (M02, no_std esp-hal) lives in firmware/radiatorium; this module carries the
// host half. Read: raw
// bytes are raw intensity (Σω) — no flow, no hsv/pwm/duration. Write: the
// membrane's PresenceFrame (9 omegas + aperture) translates to raw intensity
// (Σω · aperture, f32-LE, 4 B/frame) — the peer's own law, as SeismicOscillator.
// Both directions speak only while window.omegaflow.consent() is true.

import { encodeFrame, consented, makeWriter } from "./presence_frame.js";

const panel = document.createElement("div");
panel.style.cssText =
  "position:fixed;right:12px;bottom:10px;display:flex;gap:6px;align-items:center;" +
  "font:12px system-ui,monospace;color:#6b6256;z-index:9";
const field = "background:#0a0908;border:1px solid #2a2622;color:#c4b9a8;padding:2px 6px;font:inherit";
const nameInput = document.createElement("input");
nameInput.placeholder = "sensor name";
nameInput.style.cssText = field;
const tauInput = document.createElement("input");
tauInput.placeholder = "tau";
tauInput.value = "0";
tauInput.size = 3;
tauInput.style.cssText = field;
const button = document.createElement("button");
button.textContent = "open WebSerial";
button.style.cssText =
  "background:none;border:1px solid #3a352e;color:#e8ddc8;padding:2px 8px;font:inherit;cursor:pointer";
const stateEl = document.createElement("span");
panel.append(nameInput, tauInput, button, stateEl);
document.body.append(panel);

let port = null;
let reader = null;
let writer = null;
let reading = false;

const frame = makeWriter(
  () => writer,
  () => consented(window.omegaflow)
);

function note(text) {
  stateEl.textContent = text;
}

function emit(bytes) {
  const name = nameInput.value.trim();
  if (name === "" || !consented(window.omegaflow)) {
    return;
  }
  const tau = Number(tauInput.value);
  const t = Number.isFinite(tau) ? tau : 0;
  for (const b of bytes) {
    window.omegaflow.recordSample(name, b, t);
  }
}

async function readLoop() {
  reader = port.readable.getReader();
  reading = true;
  note("reading");
  try {
    while (reading) {
      const { value, done } = await reader.read();
      if (done) {
        break;
      }
      emit(value);
    }
  } catch {
    reading = false;
  } finally {
    reader.releaseLock();
    reader = null;
    reading = false;
    note("stream ended");
  }
}

async function toggle() {
  if (reading) {
    reading = false;
    try {
      await reader.cancel();
    } catch {}
    try {
      writer.releaseLock();
    } catch {}
    try {
      await port.close();
    } catch {}
    reader = null;
    writer = null;
    port = null;
    note("closed");
    return;
  }
  if (!("serial" in navigator)) {
    note("WebSerial absent");
    return;
  }
  if (!port) {
    try {
      port = await navigator.serial.requestPort();
      await port.open({ baudRate: 115200 });
      writer = port.writable.getWriter();
    } catch {
      port = null;
      writer = null;
      note("no port");
      return;
    }
  }
  readLoop();
}

export function onFrame(omega, aperture) {
  frame.onFrame(omega, aperture);
}

button.addEventListener("click", toggle);

if (!("serial" in navigator)) {
  button.disabled = true;
  note("WebSerial absent");
}
