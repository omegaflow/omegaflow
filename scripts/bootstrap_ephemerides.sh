#!/bin/bash
# Omegaflow ephemeris bootstrap — one-time dev-proof compile (K01 verification
# step). The permanent path is the CI flattener; this script recovers a machine
# locally when /tmp caches are gone. Moons are not included (multi-GB SPKs —
# they arrive with the flattener).
set -euo pipefail

REPO=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
WORK=/tmp/opencode/eph
mkdir -p "$WORK"

echo "== text kernels (runtime files; the server refetches these itself too) =="
curl -sS -m 120 -o /tmp/omegaflow_kernel_gm_de440.txt   https://naif.jpl.nasa.gov/pub/naif/generic_kernels/pck/gm_de440.tpc
curl -sS -m 120 -o /tmp/omegaflow_kernel_pck00010.txt   https://naif.jpl.nasa.gov/pub/naif/generic_kernels/pck/pck00010.tpc
curl -sS -m 120 -o /tmp/omegaflow_kernel_geophysical.txt https://naif.jpl.nasa.gov/pub/naif/generic_kernels/pck/geophysical.ker
curl -sS -m 120 -o /tmp/omegaflow_kernel_naif0012.txt   https://naif.jpl.nasa.gov/pub/naif/generic_kernels/lsk/naif0012.tls

echo "== planets SPK (~32 MB) =="
[ -f "$WORK/de440s.bsp" ] || curl -sS -m 300 -o "$WORK/de440s.bsp" https://naif.jpl.nasa.gov/pub/naif/generic_kernels/spk/planets/de440s.bsp

echo "== compile SPICE bodies (v2, meters) =="
cargo build --release --bin ephemeris_compiler
(cd "$WORK" && "$REPO/target/release/ephemeris_compiler" de440s.bsp \
  --gm /tmp/omegaflow_kernel_gm_de440.txt \
  --pck /tmp/omegaflow_kernel_pck00010.txt \
  --pck /tmp/omegaflow_kernel_geophysical.txt)

echo "== compile Horizons bodies (network-bound, several minutes) =="
cargo build --release --bin horizons_compiler
(cd "$WORK" && "$REPO/target/release/horizons_compiler")

echo "== install into the runtime cache =="
for f in "$WORK"/ephemeris_*.bin; do
  b=$(basename "$f")
  cp "$f" "/tmp/omegaflow_eph_${b#ephemeris_}"
done

echo "done: $(ls /tmp/omegaflow_eph_*.bin 2>/dev/null | wc -l) binaries under /tmp/omegaflow_eph_*.bin"
echo "kernel files: $(ls /tmp/omegaflow_kernel_* 2>/dev/null | wc -l) under /tmp/omegaflow_kernel_*"
echo "start the server (cargo run) — the runtime caches have a 7-day TTL (kernels) / 1-day (ephemerides)."
