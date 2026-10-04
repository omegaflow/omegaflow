import fs from "node:fs";
import path from "node:path";
import { createRequire } from "node:module";

const args = process.argv.slice(2);
const arg = (name, def) => {
  const i = args.indexOf(name);
  return i >= 0 && i + 1 < args.length ? args[i + 1] : def;
};

const pkgDir = arg("--pkg", "pkg");
const stars = arg("--stars", "data/ssd.jpl.nasa.gov/dr3_stars.bin");
const epoch = Number(arg("--epoch", "2000.0"));
const now = Number(arg("--now", "8.443618e8"));
const nativeDump = arg("--native", "/tmp/opencode/native_after.bin");

const require = createRequire(import.meta.url);
const wasm = require(path.resolve(pkgDir, "omegaflow.js"));
const starBytes = fs.readFileSync(stars);
const lookup = new wasm.MembraneLookup(starBytes, epoch);
const wasmFlat = lookup.query(0, 0, 0, now, 1, 0, 0);

const nativeBuf = fs.readFileSync(nativeDump);
const nativeFlat = new Float64Array(
  nativeBuf.buffer,
  nativeBuf.byteOffset,
  nativeBuf.byteLength / 8,
);

const WIRE = 26;
const REL = Math.pow(2, -40);
const FLOOR = {
  0: 1e-3,
  1: 1e-3,
  2: 1e-3,
  3: 1e-40,
  5: 1e-3,
  6: 1e-3,
  12: 1e-3,
  13: 1e-3,
  14: 1e-3,
};

if (nativeFlat.length !== wasmFlat.length) {
  console.log(
    `RISS | shape: native ${nativeFlat.length} values vs wasm ${wasmFlat.length}`,
  );
  process.exit(1);
}

const slots = nativeFlat.length / WIRE;
let structure = 0;
let physical = 0;
let worstI = -1;
let worstRel = 0;
let worstAbs = 0;

for (let i = 0; i < nativeFlat.length; i++) {
  const slot = i % WIRE;
  const a = nativeFlat[i];
  const b = wasmFlat[i];
  const floor = FLOOR[slot];
  if (floor === undefined) {
    if (!Object.is(a, b) && a !== b) structure++;
    continue;
  }
  const abs = Math.abs(a - b);
  const lim = REL * Math.max(Math.abs(a), Math.abs(b)) + floor;
  if (abs > lim) {
    physical++;
    const rel = abs / (Math.max(Math.abs(a), Math.abs(b)) + floor);
    if (rel > worstRel) {
      worstRel = rel;
      worstAbs = abs;
      worstI = i;
    }
  }
}

const verdict =
  structure === 0 && physical === 0
    ? "holds"
    : "RISS";
console.log(
  `${verdict} | records ${slots} | values ${nativeFlat.length} | structure-mismatch ${structure} | physical-beyond-tol ${physical}`,
);
if (worstI >= 0) {
  console.log(
    `worst | index ${worstI} slot ${worstI % WIRE} record ${(worstI / WIRE) | 0} | abs ${worstAbs} | rel ${worstRel}`,
  );
}
process.exit(verdict === "holds" ? 0 : 1);
