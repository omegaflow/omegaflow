<!--
  title: OMEGAFLOW — Kernel Curation & CI Automation Plan
  class: concept
  sha256: 4d91fced7872c6f19dd2642120e0084abe01f0c49d0553b3540a10f66e231bc6
-->
I understand your concern absolutely. "For now but later" is the death of every architecture. We must not file this NASA list away as a "later problem".

Once we have moved the system to the v6 protocol and the Trommelfell (packets 2 and 3), we need a **scalable, automated process** to integrate this flood of kernels into the CI pipeline without you having to type every block manually.

Here is the official **Master Curation & Automation Plan**. You can save this text as a Markdown file (`docs/kernel_curation_plan.md`) and hand it to the Kybernaut in the next, isolated session. It does not solve the problem by "doing it later", but by "defining the automation now".

***

### Copy this text as the plan for the kernel curation:

___

# OMEGAFLOW — Kernel Curation & CI Automation Plan

**Status:** Architecture directive for the scalable integration of all NAIF SPICE kernels.
**Principle:** No "for now". We build a pipeline that automatically manifests thousands of bodies as flattened `.bin` files on the CDN, without manual code input.

## 1. The data sources (the raw NASA library)
The system divides the NASA kernels into four domains:
1. **Planetary (DE440s):** Sun, planets, Moon. (Already integrated; updated to v2 in packet 2.)
2. **Satellites:** Moons of Jupiter, Saturn, Uranus, Neptune, Pluto (e.g. `jup344.bsp`, `sat454.bsp`). Files are partly huge (up to 1.9 GB).
3. **Asteroids & Comets:** Tens of thousands of bodies from `asteroids_de441/` and comets like `67p/`.
4. **Spacecraft:** Probes like Voyager (`vgr1.x2100.bsp`), Cassini, Parker Solar Probe.

## 2. The CI pipeline (the flattening process)
The integration happens exclusively through the CI pipeline (e.g. GitHub Actions). No local download of 3 GB files on the XPS 13.

**The CI workflow (`.github/workflows/kernel-flatten.yml`):**
1. **Fetch:** The CI downloads the master `.bsp` files from `ssd.jpl.nasa.gov/ftp/eph/`.
2. **Compile:** The CI uses the `ephemeris_compiler` (with the new `--gm` and `--pck` flags from packet 2) to extract the bodies. The CI generates one `ephemeris_{body}.bin` per body (v2 format, gcount=12).
3. **Push to CDN:** The CI pushes the generated `.bin` files as release assets to the omegaflow GitHub CDN.
4. **Indexing:** The CI updates a `sources_index.φ` that carries all available bodies as a URL list.

## 3. The local symbiosis (the Enclosure Lemma)
None of this heavy work runs on the local XPS 13 (the browser/presence window).

1. The frontend reports to the backend: "I hover at `(x, y, z, t)` in the Jupiter system."
2. The Rust backend uses the **Enclosure Lemma**. It knows that Jupiter might have moons.
3. The backend downloads *only* `ephemeris_io.bin`, `ephemeris_europa.bin` etc. from the CDN (flattened, a few kilobytes) and caches them locally in `/tmp`.
4. When the point appears where the Voyager probe is, it loads `ephemeris_voyager1.bin`.

## 4. Implementation steps (for the curation session)
This packet gets built in a dedicated session, *after* the v6 protocol and the Trommelfell (packets 2 & 3) run stably.

1. **`scripts/generate_sources.py` (or Rust):** A script that scans the NASA directories and automatically generates the `sources.φ` blocks for `ephemeris_binary`.
2. **NAIF-ID mapping table:** Completion of the `pck_id_of` and `body_name_of` maps in Rust for all Saturn/Jupiter/Uranus moons and asteroid-belt objects.
3. **CI script:** YAML workflow that lets the compiler iterate over the list in the cloud.

## Conclusion
We do not defer the problem to "later". We define right now the exact mechanism by which the system will master this data flood: through CI automation and dynamic Enclosure-Lemma loading. The data exist, the pipeline is defined, the local hardware is spared.

___

### Why this solves your "for now but later" problem:
You now have an **architectural blueprint**. When someone (or an AI) asks: "When do we integrate the Jupiter moons?", the answer is not "later", but: "As soon as packets 2 & 3 are done, we build this CI pipeline, and they get integrated automatically."

We have the plan. It is safely stored. Now we return to DeepSeek and get the code for **packet 2**!
