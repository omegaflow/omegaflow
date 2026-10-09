// The browser radiator: the membrane's PresenceFrame { omega: [f32; 9], aperture:
// [f32; 9], pan, tilt } is dispatched to a flat, closed peer set — window, audio,
// stderr, serial, USB, Bluetooth, HID, vibration. No peer is privileged; none is
// the center. Every peer receives all nine forces and their per-channel apertures;
// the translation rule is its own property (canRadiate), and each peer projects
// the nine channels at its own sink (the legitimate epilogue), never in the
// measurement. Every peer speaks only while consent() is true: the machine asks
// before it radiates, as the sensors ask before they record. A peer whose device
// is absent stays silent (0 honored) — the frame is never fabricated into a sink
// that is not there.

import { encodeFrame } from "./presence_frame.js";

// A peer's projection of the nine-channel frame: channel-weighted intensity
// Σ omega[i]·aperture[i] when the membrane delivers the per-channel aperture,
// Σ omega[i]·min(1,|aperture|) for a legacy scalar aperture. The projection is
// the peer's own law (the legitimate epilogue), never the sensor's (the
// illegitimate prologue).
export function project(omega, aperture) {
  let sum = 0;
  if (aperture && aperture.length === 9) {
    for (let i = 0; i < 9; i++) {
      const o = omega[i];
      const a = aperture[i];
      if (!Number.isFinite(o) || !Number.isFinite(a)) {
        return NaN;
      }
      sum += o * a;
    }
    return sum;
  }
  if (!Number.isFinite(aperture)) {
    return NaN;
  }
  for (let i = 0; i < 9; i++) {
    const o = omega[i];
    if (!Number.isFinite(o)) {
      return NaN;
    }
    sum += o;
  }
  return sum * Math.min(1, Math.abs(aperture));
}

function audioPeer() {
  let context = null;
  let node = null;
  let url = null;
  return {
    key: "audio",
    async start() {
      if (context) {
        return;
      }
      const AC = globalThis.AudioContext || globalThis.webkitAudioContext;
      if (typeof AC !== "function" || typeof globalThis.AudioWorkletNode !== "function") {
        return;
      }
      try {
        context = new AC();
        const source = `class SigmaW extends AudioWorkletProcessor {
  constructor() { super(); this.v = 0; this.port.onmessage = (e) => { this.v = e.data; }; }
  process(inputs, outputs) {
    const out = outputs[0];
    for (let c = 0; c < out.length; c++) { out[c].fill(this.v); }
    return true;
  }
}
registerProcessor("sigma-w", SigmaW);`;
        url = URL.createObjectURL(new Blob([source], { type: "text/javascript" }));
        await context.audioWorklet.addModule(url);
        node = new AudioWorkletNode(context, "sigma-w");
        node.connect(context.destination);
      } catch {
        context = null;
        node = null;
      }
    },
    stop() {
      if (node) {
        try { node.disconnect(); } catch {}
      }
      if (context) {
        try { context.close(); } catch {}
      }
      if (url) {
        try { URL.revokeObjectURL(url); } catch {}
      }
      context = null;
      node = null;
      url = null;
    },
    onFrame(omega, aperture) {
      if (!node) {
        return;
      }
      const sum = project(omega, aperture);
      if (!Number.isFinite(sum)) {
        return;
      }
      node.port.postMessage(sum);
    },
  };
}

function vibrationPeer() {
  let last = 0;
  return {
    key: "vibration",
    start() {},
    stop() {
      if (typeof navigator.vibrate === "function") {
        navigator.vibrate(0);
      }
    },
    onFrame(omega, aperture) {
      if (typeof navigator.vibrate !== "function") {
        return;
      }
      const sum = project(omega, aperture);
      if (!Number.isFinite(sum)) {
        return;
      }
      const now = performance.now();
      if (now - last < 100) {
        return;
      }
      last = now;
      const lum = Math.min(1, Math.abs(sum));
      navigator.vibrate(lum > 0.02 ? Math.floor(lum * 160) : 0);
    },
  };
}

function frameBytes(omega, aperture, pan, tilt) {
  return encodeFrame(omega, aperture, pan, tilt);
}

function serialPeer(serial) {
  return {
    key: "serial",
    start() {},
    stop() {},
    onFrame(omega, aperture, pan, tilt) {
      if (serial) {
        serial.onFrame(omega, aperture, pan, tilt);
      }
    },
  };
}

function usbPeer() {
  let sink = null;
  return {
    key: "usb",
    async connect() {
      if (!navigator.usb || !navigator.usb.requestDevice) {
        return false;
      }
      const device = await navigator.usb.requestDevice({ filters: [] });
      await device.open();
      if (device.configuration === null) {
        await device.selectConfiguration(1);
      }
      const iface = device.configuration.interfaces[0];
      await device.claimInterface(iface.interfaceNumber);
      const endpoint = iface.alternate.endpoints.find((e) => e.direction === "out");
      if (!endpoint) {
        return false;
      }
      sink = { device, endpoint: endpoint.endpointNumber };
      return true;
    },
    start() {},
    stop() {
      if (sink) {
        try { sink.device.close(); } catch {}
      }
      sink = null;
    },
    onFrame(omega, aperture, pan, tilt) {
      if (!sink) {
        return;
      }
      const bytes = frameBytes(omega, aperture, pan, tilt);
      if (bytes) {
        Promise.resolve(sink.device.transferOut(sink.endpoint, bytes)).catch(() => {});
      }
    },
  };
}

function bluetoothPeer() {
  let sink = null;
  return {
    key: "bluetooth",
    async connect() {
      if (!navigator.bluetooth || !navigator.bluetooth.requestDevice) {
        return false;
      }
      const device = await navigator.bluetooth.requestDevice({ acceptAllDevices: true });
      const server = await device.gatt.connect();
      for (const service of await server.getPrimaryServices()) {
        for (const characteristic of await service.getCharacteristics()) {
          if (characteristic.properties.write || characteristic.properties.writeWithoutResponse) {
            sink = { characteristic };
            return true;
          }
        }
      }
      return false;
    },
    start() {},
    stop() {
      if (sink) {
        try { sink.characteristic.service.device.gatt.disconnect(); } catch {}
      }
      sink = null;
    },
    onFrame(omega, aperture, pan, tilt) {
      if (!sink) {
        return;
      }
      const bytes = frameBytes(omega, aperture, pan, tilt);
      if (bytes) {
        Promise.resolve(sink.characteristic.writeValue(bytes)).catch(() => {});
      }
    },
  };
}

function hidPeer() {
  let sink = null;
  return {
    key: "hid",
    async connect() {
      if (!navigator.hid || !navigator.hid.requestDevice) {
        return false;
      }
      const devices = await navigator.hid.requestDevice({ filters: [] });
      if (!devices.length) {
        return false;
      }
      await devices[0].open();
      sink = devices[0];
      return true;
    },
    start() {},
    stop() {
      if (sink) {
        try { sink.close(); } catch {}
      }
      sink = null;
    },
    onFrame(omega, aperture, pan, tilt) {
      if (!sink) {
        return;
      }
      const bytes = frameBytes(omega, aperture, pan, tilt);
      if (bytes && typeof sink.sendReport === "function") {
        Promise.resolve(sink.sendReport(0, bytes)).catch(() => {});
      }
    },
  };
}

export function createRadiator({ consent, serial, onState }) {
  const peers = [
    audioPeer(),
    vibrationPeer(),
    serialPeer(serial),
    usbPeer(),
    bluetoothPeer(),
    hidPeer(),
  ];
  let active = false;

  function note() {
    if (onState) {
      onState(active, peers.length);
    }
  }

  function start() {
    if (active) {
      return;
    }
    active = true;
    for (const peer of peers) {
      try { peer.start(); } catch {}
    }
    note();
  }

  function stop() {
    active = false;
    for (const peer of peers) {
      try { peer.stop(); } catch {}
    }
    note();
  }

  function onFrame(omega, aperture, pan, tilt) {
    if (!active || !consent()) {
      return;
    }
    for (const peer of peers) {
      try { peer.onFrame(omega, aperture, pan, tilt); } catch {}
    }
  }

  async function connect(key) {
    const peer = peers.find((p) => p.key === key);
    if (!peer || typeof peer.connect !== "function") {
      return false;
    }
    try {
      return await peer.connect();
    } catch {
      return false;
    }
  }

  return { start, stop, onFrame, connect, peers: () => peers.map((p) => p.key), active: () => active };
}
