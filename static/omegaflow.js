// Hand-written wasm loader. The module exports plain `extern "C"` functions
// (see src/wasm.rs); there is no binding generator between Rust and the page.

let exports = null;

function adopt(instance) {
  exports = instance.exports;
  return exports;
}

export default async function init(input) {
  const url =
    input instanceof URL
      ? input
      : new URL(input ?? "omegaflow_bg.wasm", import.meta.url);
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`omegaflow wasm: HTTP ${response.status}`);
  }
  const bytes = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(bytes, {});
  return adopt(instance);
}

// For hosts without a usable fetch (the Node parity gate), where the caller
// already holds the module bytes.
export function initSync(bytes) {
  const module = new WebAssembly.Module(bytes);
  return adopt(new WebAssembly.Instance(module, {}));
}

function heapU8() {
  return new Uint8Array(exports.memory.buffer);
}

function heapF64() {
  return new Float64Array(exports.memory.buffer);
}

function toBytes(value) {
  if (value instanceof Uint8Array) return value;
  if (ArrayBuffer.isView(value)) {
    return new Uint8Array(value.buffer, value.byteOffset, value.byteLength);
  }
  return new Uint8Array(value);
}

function copyIn(bytes) {
  const ptr = exports.omegaflow_alloc(bytes.length);
  heapU8().set(bytes, ptr);
  return ptr;
}

export class MembraneLookup {
  constructor(stars, catalogEpochYr) {
    if (!exports) {
      throw new Error("omegaflow wasm is not instantiated");
    }
    const bytes = toBytes(stars);
    const ptr = copyIn(bytes);
    this.ptr = exports.omegaflow_lookup_new(ptr, bytes.length, catalogEpochYr);
    exports.omegaflow_dealloc(ptr, bytes.length);
  }

  add_stars(stars, catalogEpochYr) {
    const bytes = toBytes(stars);
    const ptr = copyIn(bytes);
    exports.omegaflow_lookup_add_stars(this.ptr, ptr, bytes.length, catalogEpochYr);
    exports.omegaflow_dealloc(ptr, bytes.length);
  }

  load_ephemeris(name, bytes) {
    const nameBytes = new TextEncoder().encode(name);
    const data = toBytes(bytes);
    const namePtr = copyIn(nameBytes);
    const dataPtr = copyIn(data);
    const ok = exports.omegaflow_lookup_load_ephemeris(
      this.ptr,
      namePtr,
      nameBytes.length,
      dataPtr,
      data.length,
    );
    exports.omegaflow_dealloc(namePtr, nameBytes.length);
    exports.omegaflow_dealloc(dataPtr, data.length);
    return ok === 1;
  }

  query(cx, cy, cz, t2, fx, fy, fz) {
    const len = exports.omegaflow_lookup_query(
      this.ptr,
      cx,
      cy,
      cz,
      t2,
      fx,
      fy,
      fz,
    );
    if (len === 0) return new Float64Array(0);
    const ptr = exports.omegaflow_lookup_result(this.ptr);
    return heapF64().slice(ptr / 8, ptr / 8 + len);
  }

  free() {
    if (this.ptr) {
      exports.omegaflow_lookup_free(this.ptr);
      this.ptr = 0;
    }
  }
}
