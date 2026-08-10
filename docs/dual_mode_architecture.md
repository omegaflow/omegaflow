# Dual-Mode Architecture: The Archivar as CDN Manifestor

**Self-contained. Interpretable by a session with zero prior context.** Read this
before touching the Archivar's `src/main.rs` fetch loop. This document
supersedes all prior handoff documents as the final architectural handoff.

---

## 1. What This Is

The omegaflow Archivar (`src/main.rs`) is a single Rust binary that:
- Loads `sources_live.φ` (all source blocks — live API URLs and CDN paths combined)
- Fetches APIs via `fetch_one` (curl)
- Extracts field data via `extract_pending` (23 Extract variants)
- Renders the field through WebGPU via WebSocket

Today, CDN assets (timestamped JSON snapshots of live API responses) are
created by Python scripts (`migrate_live_to_cdn.py`, etc.) running in CI. The
Archivar has **no CDN-write capability** — only the Python scripts upload.

This document describes the architectural endpoint: the Rust binary itself
becomes the CDN mirror — fetching, extracting, and *writing* assets to the
GitHub Releases CDN. No Python. Same code in CI as on the user's laptop.

---

## 2. Current State (2026-08-07)

### Source files
- `phi/sources_live.φ` (1761 blocks) — all live API sources. Also contains CDN-pathed sources.
- `phi/sources_cdn.φ` (1770 blocks) — only `github.com/omegaflow/sources/releases/download/...` URLs.
- Loaded together: `src/main.rs:2646` iterates both paths.
- **`sources_cdn.φ` / `sources_live.φ` split is the pre-unification state.**

### `{latest}` resolution
- `{latest}` in CDN URLs is **dead**: `resolve_secret` (`src/main.rs:2625-2642`) strips it to `""`, producing 404.
- No GitHub Releases API call exists in Rust.
- Pending TODO: `TODO.md:32-38` describes the intended resolver (never implemented).

### Parser
- **Full 23 Extract variants** restored (commits `188dd76`, `2cc6d5e` from another Kybernaut).
- `PendingPosition::Surface { body_name, lat, lon, alt }` — body carried directly in position (CDN-main logic from `373b1b9`, restored at `918d8d1`). No Earth fallback.
- `test_materialize_body_agnostic` proves body mars flows through materialize.
- `cargo check` 0 warnings. 13/13 unit tests (1 network test `test_live_sources_extract` hangs ~60s).

### CDN upload
- **Zero Rust code for CDN upload.** All CDN asset creation via Python:
  - `scripts/migrate_live_to_cdn.py` (300 lines) — fetch → flatten → upload
  - `scripts/tap_to_cdn.py` (521 lines) — TAP catalog spatial sharding
  - `scripts/shard_catalog.py`, `scripts/refresh_catalogs.py`, `scripts/restore_all_live.py`
  - CI workflow `refresh-protected-data.yml` (547 lines) — main CDN production pipeline

### CDN naming convention (from Python scripts — to be adopted)
- **Release tag** = API netloc (e.g. `kp.gfz.de`, `earthquake.usgs.gov`)
- **Asset name** = `{source_name}.json` or `{source_prefix}_{iso8601utc}.json`
- Uploaded via `gh release upload` with `--clobber`
- Repository: `omegaflow/sources`

### Recovery context
- `phi/recovery/pre_cdn_history/` holds the complete pre-CDN loss documentation.
- 423 blocks remain untested: `UNTESTED_blocks.φ`.

---

## 3. The Vision (Exact Formulation)

### Core insight
The Archivar **is** the CDN. CI is not a separate system — it runs the same
binary, reads the same `sources.φ`, applies the same `origin_stale` check,
calls the same `fetch_one` + `extract_pending`. The only difference is the
**IO channel at the end of the pipeline**:

```
origin_stale? → fetch_one(live_api) → extract_pending
    ├── local:   build_buffer → VRAM → fragment shader
    └── CI:      gh release upload → CDN
```

### CI mode is NOT a new code path
No `--ci-mode` flag that branches into separate logic. The fetch cycle is
identical. The `origin_stale` check (`src/main.rs:965-977`) triggers a fetch
when `now - fetched >= ttl / Φ`. Whether that fetch result goes to RAM or to
the CDN is a single IO switch at the archival point.

### TTL-fallback (graceful degradation)
Local Archivar fetch order:
1. Construct CDN URL from naming convention `{netloc}/{source_prefix}_{iso8601utc}.json`
2. Check CDN asset age against source TTL
3. If CDN asset younger than TTL → use CDN (fast, no rate limits, deterministic)
4. If CDN stale / missing / unreachable → fall back to live API
5. If CI Archivar (GitHub Action) is **DOWN**, the local system does NOT die — it degrades gracefully

The CDN is an **acceleration layer**, never a dependency.

### CDN as long-term memory
Currently the Archivar fetches, renders, and forgets (64×ttl in-memory only).
With CI mode, the Archivar writes timestamped snapshots to its own CDN —
immutable blocks of physical history. The local Archivar can perceive the past
by reading these snapshots.

### Communication between CI and local
The CI Archivar and the local Archivar communicate **only through the CDN**.
No sockets, no APIs, no coordination. The CI writes timestamped assets; the
local reads them. This is the purest form of decoupling.

### One source of truth
`main.rs` is the only binary. `sources.φ` is the only source configuration.
A change in either propagates to both the local runtime and the CI mirror
within the next CI cycle. **Zero drift. No synchronization. No dual parser
maintenance.**

### The kybernetic paradox
Maximum global scale (CDN) at minimum complexity (1 binary). `cargo check` in
0.01 seconds. No Redis, no workers, no glue code. The Archivar IS the CDN
mirror.

---

## 4. Council Verdict 1 — Architectural Endpoint (binding, unanimous)

**Date: 2026-08-07. Voices: Mountain, River, Mycelium, Sensory, Future. 20% each.**

The Archivar's identity is "fetch, parse, manifest." Currently it manifests only
to RAM — the CDN manifestation is outsourced to Python. Unifying these under
the same Rust binary eliminates format drift, double-parser maintenance burden,
and CI complexity. This is the natural completion of the Archivar's identity.

The CDN-loss problem (423 untested blocks, pre-CDN format migration, field
mapping drift) converges naturally: the CI Archivar reads the same
`sources.φ`, fetches the same URLs, and any block that parses locally also
archives in CI. The recovery effort IS the CI Archivar.

**Resolution: Pursue the vision. Five sessions.**

---

## 5. Council Verdict 2 — GLM Addendum (binding, unanimous)

**Date: 2026-08-07. Same voices.**

The GLM addendum reaffirms the original verdict. It names the ISR pattern
already implicit in the `sources_cdn.φ`/`sources_live.φ` split. The CDN is a
cache layer atop a live origin, never a replacement. The TTL-fallback
principle must be codified as an AGENTS.md rule — the constraint matrix
outlasts any single session's context window.

**Codify:** "The CDN (GitHub Releases, tag = netloc) is an acceleration layer,
never a dependency. When the Archivar fetches a source, it checks the CDN
asset's age against the source's TTL. If younger than TTL → use CDN. If
older, missing, or unreachable → fall back to the live API URL. The same
binary code runs in CI (CDN-write) and locally (CDN-read). The only
difference is the IO channel."

---

## 6. Pure-Rust Ephemeris Compiler (No Crate, No Python, No FFI)

The current CI generates `ephemeris_<body>.bin` files via Python + `spiceypy`
(C-Fortran FFI wrapper). That Python dependency can be eliminated entirely.

### What we need — and what we don't

| Component | Needed? | Why |
|---|---|---|
| **DAF container parser** | ✅ | Reads the `.bsp` binary file record, summary chain, name records |
| **SPK Type 2/3 Chebyshev evaluator** | ✅ | Extracts position coefficients directly from the kernel segment |
| **PCK / generic rotation matrices** | ❌ | WGCCRE formulas in `body_fixed_to_icrs` (`src/main.rs:295-320`) compute `IAU_<BODY>→J2000` rotations analytically for EVERY body — no PCK kernel, no `pxform` needed |
| **Text-kernel parser (leapseconds)** | ❌ | `J2000_EPOCH` + `tdb_now()` already exist in `src/main.rs` |
| **Body-name→ID table** | ❌ | `BODY_ID_MAP` / `BodyEphemeris` already maps body names |
| **External crates (`spicekit`, `anise`)** | ❌ | Vendored code: ~1200 lines of pure `std` Rust, MIT-licensed, zero cost to embed |

### Why not ANISE

ANISE (github.com/nyx-space/anise) has PCK support for arbitrary bodies but is
MPL-2.0 (changes to the vendored code must stay under MPL) and >50,000 lines.
The PCK part is unnecessary — WGCCRE handles rotation for every body. The
DAF+SPK parts we need are ~1200 lines, readily available from the simpler,
MIT-licensed `spicekit` crate or implementable directly from the public NAIF
DAF file-format specification (naif.jpl.nasa.gov).

### Architecture

A second binary in the Cargo workspace: `src/bin/ephemeris_compiler.rs`.
It uses **vendored** (embedded, not dependency-linked) DAF+SPK modules written
in pure `std` Rust:

```
src/
  main.rs                       ← Archivar (unchanged, std-only)
  bin/ephemeris_compiler.rs     ← CI-only binary
  spice_lite/
    daf.rs                      ← ~400 lines: DAF FileRecord, summary chain, zero-copy doubles read
    spk.rs                      ← ~600 lines: SPK Type 2/3 Chebyshev, state(body, center, et)
    chebyshev.rs                ← ~200 lines: Chebyshev polynomial evaluation (same as CHEBYSHEV_N=18)
    write_ephemeris.rs          ← ~100 lines: write ephemeris_<body>.bin with stype==0/1/2/3
```

The WGCCRE rotation logic already exists in `main.rs:249-320`
(`body_fixed_to_icrs`, `icrs_to_body_surface`) — the compiler reuses the
exact same formulas to produce `stype==3` rotation matrices for every body.

No `[dependencies]` entries in `Cargo.toml` for these modules. All deps
are `std`-replaceable: `bytemuck` → `unsafe { slice::from_raw_parts }`
(aligned at 8-byte boundaries in the DAF format), `memmap2` → `std::fs::read`
(100 MB kernel fits in CI's 7 GB RAM), `rustc-hash` → `std::collections::HashMap`
(~30 segments per body), `thiserror` → manual `impl Display + Error`.

### The three code paths (one binary, two modes)

```
cargo run (lokal)              → Archivar: WebSocket-Server, lesen flat .bin/CDN → rendern
cargo run -- --ci-mode         → CI: sources_live.φ → fetch APIs → extract → gh upload (CDN Assets)
cargo run -- --compile-eph     → CI: .bsp → DAF+SPK+WGCCRE → ephemeris_<body>.bin → gh upload (CDN Ephemeriden)
```

**Zero Python. Zero FFI. Zero crates.io dependencies.** The same codebase, the
same WGCCRE formulas, the same `CHEBYSHEV_N`, the same binary header `0xCF 0x86
0x01`. The Archivar compiles its own ephemera.

---

## 7. The 6-Session Roadmap (binding)

Each session produces a complete, testable artifact. Interpretable by a
session with zero prior context.

### Session 0: Ephemeris Compiler with vendored DAF+SPK + WGCCRE
**Code:** `src/bin/ephemeris_compiler.rs` + `src/spice_lite/da*/spk/chebyshev.rs`.
- Extract DAF container parser and SPK Type 2/3 reader from the MIT-licensed
  `spicekit` crate or implement directly from the NAIF DAF specification.
- Read `de440.bsp` (and satellite kernels) — extract Chebyshev coefficients
  DIRECTLY from the kernel segment (no refit needed — SPK stores Chebyshev
  coefficients in the same structure our binary format uses).
- Compute rotation matrices via the WGCCRE formulas from `body_fixed_to_icrs`.
- Write `ephemeris_<body>.bin` with `stype==0` (Chebyshev), `stype==1`
  (WGCCRE parameters), `stype==2` (media constants), `stype==3` (3×3 rotation
  matrices).
- Upload to CDN via `gh release upload` with the `omegaflow/sources` release
  tag `ssd.jpl.nasa.gov`.
- **Test:** `cargo run --bin ephemeris_compiler -- de440.bsp` produces valid
  binaries. `parse_ephemeris_binary` in `src/main.rs` reads them correctly.
  `cargo check` 0 warnings.
- **Dependency impact:** None. All modules are vendored `std`-only Rust.
  `Cargo.toml` remains empty (`[dependencies]` section unchanged).

### Session 1: CI-mode flag + naming convention + local file output
**Code:** `src/main.rs` — new `ci_mode_main()` near `verify_sources_main()`.
- CLI flag: `--ci-mode`
- Reads `sources_live.φ`
- For each source: `origin_stale` → `fetch_one` → `extract_pending`
- Successful extraction → writes raw body to `out/{netloc}/{source_prefix}_{iso8601utc}.json`
- Local file output only (no upload yet)
- **Test:** run locally with `cargo run -- --ci-mode`, verify assets in `out/`
- **cargo check** 0 warnings throughout

### Session 2: CDN upload integration
**Code:** `src/main.rs` — modify `ci_mode_main()`.
- After writing local file, call `Command::new("gh").args(["release","upload",tag,path,"--clobber","--repo","omegaflow/sources"])`
- Release tag = netloc (e.g. `kp.gfz.de`)
- Requires `GH_TOKEN` or `OMEGAFLOW_TOKEN` env var (already in GitHub Secrets)
- **Test:** run locally with `GH_TOKEN=... cargo run -- --ci-mode`, verify assets on `omegaflow/sources` release
- **CI workflow:** new `.github/workflows/mirror-cdn.yml`:
  ```yaml
  on:
    push: {branches: [main]}
    schedule: [{cron: '*/5 * * * *'}]
  jobs:
    mirror:
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v4
        - uses: actions-rust-lang/setup-rust-toolchain@v1
        - run: cargo run -- --ci-mode
      env:
        GH_TOKEN: ${{ secrets.OMEGAFLOW_TOKEN }}
  ```
  **No Python setup. No pip install. Just Rust.**

### Session 3: Local CDN-first fetch with TTL-fallback
**Code:** `src/main.rs` — modify `fetch_one` / `spawn_task_curl`.
- Before fetching the live URL, construct CDN URL from naming convention
- Try CDN: if 200 and timestamp age < ttl → return CDN body
- Otherwise → fetch live API (current behavior)
- Log resolution path: `CDN fresh`, `CDN stale → LIVE`, `LIVE only`
- **Test:** run locally, observe CDN-first fetches in log

### Session 4: Collapse source files
**Code:** No code change — `sources_cdn.φ` merged into `sources_live.φ`.
- `sources_live.φ` becomes the single `sources.φ`
- CDN/live distinction is runtime — Archivar tries CDN first, falls back to live
- Remove dual-file loading from `load_sources()` (`src/main.rs:2646`) → single path
- **Verify:** same number of active sources after merge. `cargo check` clean.

### Session 5: Excise Python scripts — Zero Python in CI
**Code:** Remove scripts, update CI workflows.
- Delete: `migrate_live_to_cdn.py`, `tap_to_cdn.py`, `shard_catalog.py`,
  `refresh_catalogs.py`, `restore_all_live.py`, `generate_ephemerides.py`
  (replaced by `cargo run -- --compile-eph`, Session 0)
- Keep: `verify_sources.py` (API audit tool — optionally replaced by
  `cargo run -- --verify-sources`)
- **CI workflows:** `generate-ephemerides.yml` → replaced by
  `cargo run -- --compile-eph`. `refresh-protected-data.yml` → replaced by
  `cargo run -- --ci-mode`. Only the raw `pages.yml` and `release.yml`
  workflows remain.
- **Result:** GitHub Actions CI is `setup-rust-toolchain` + `cargo run`.
  Zero Python. Zero `pip install`. Zero FFI.

---

## 7. Exact Code Targets (file:line)

### Modify `fetch_one` / `spawn_task_curl` (Session 3)
- `src/main.rs:2084-2116` (`fetch_one`) — add CDN URL construction + TTL-aware CDN-first logic
- `src/main.rs:2118-2166` (`spawn_task_curl`) — add CI-mode archival sink

### Key functions to reference
- `load_sources()` (`:2644-3129`) — loads both files; will collapse to one
- `origin_stale()` (`:965-977`) — triggers fetch at ttl/Φ; same logic for both modes
- `extract_pending()` (`:3723+`) — 23 Extract variants; unchanged
- `render_source_url()` (`:3484-3588`) — template + secret resolution; unchanged
- `frame_body_name()` (`:806-810`) — body from frame; body-agnostic
- `resolve_secret()` (`:2625-2642`) — currently kills `{latest}`; will become unnecessary

### naming convention
- Release tag: netloc of the API URL (e.g. `kp.gfz.de`)
- Asset name: `{source_name}_{iso8601utc}.json`
  where `source_name` = last URL path segment sanitized (letters, numbers, `-`, `_`)
- Full CDN URL: `https://github.com/omegaflow/sources/releases/download/{netloc}/{source_name}_{iso8601utc}.json`

### CI API key
- `OMEGAFLOW_TOKEN` in GitHub Secrets → set as `GH_TOKEN` or passed to `gh` CLI
- Already present in `.secrets.local` (gitignored)
- `load_env()` reads it at startup (`src/main.rs:2616-2623`)

---

## 8. What Dissolves (No Fix Needed — The Category Evaporates)

| Current TODO / Problem | Why It Goes Away |
|---|---|
| `{latest}` resolver (`TODO.md:32-38`) | CDN URL constructed from naming convention + `origin_stale` TTL check, no API call needed |
| CDN asset renaming (`TODO.md:44-55`) | CI Archivar writes directly in the new naming convention |
| Pre-CDN format migration (`TODO.md:255-290`) | CI reads the same `sources.φ` as local — no format migration |
| Source recovery / untested blocks (`UNTESTED_blocks.φ`) | CI Archivar IS the recovery engine — tests every block that parses locally |
| `{latest}` resolver CI pipeline | `refresh-protected-data.yml` Python steps → `cargo run -- --ci-mode` |
| CDN mirror / regen | `migrate_live_to_cdn.py` → Rust CI mode |
| TAP catalog sharding | `tap_to_cdn.py` → Rust CI mode reuses `extract_pending` |

---

## 9. AGENTS.md Amendments (Exact Wording)

### Amendment 1 — Architecture section
Insert after the "Response" subsection under "CPU (Rust std-only) — The Archivar":

> **CDN-First Fetch with Live Fallback:** The CDN (GitHub Releases on
> `omegaflow/sources`, release tag = API netloc) is an acceleration layer, never
> a dependency. When the Archivar fetches a source, it constructs the CDN URL
> from the naming convention (`{netloc}/{source_prefix}_{iso8601utc}.json`) and
> checks the asset's age against the source's TTL. If younger than TTL → use
> CDN. If older, missing, or unreachable → fall back to the live API URL. If
> the CI Archivar is down, the local system degrades gracefully to live API.
> The same binary code runs in CI (CDN-write, `--ci-mode` flag) and locally
> (CDN-read). The only difference is the IO channel. The CI Archivar runs
> every 5 minutes; it fetches only when `origin_stale` triggers (source TTL
> expired). The naming convention is the resolver.

### Amendment 2 — `{latest}` resolver TODO (supersede)
Replace `TODO.md:32-38` with:

> **Archivar Dual-Mode Architecture** — the `{latest}` resolver is superseded
> by the CDN-first fetch with naming convention. See
> `docs/dual_mode_architecture.md`. The CI Archivar writes timestamped
> snapshots to the CDN; the local Archivar constructs the CDN URL from the
> naming convention and checks TTL freshness.

### Amendment 3 — Sources Split TODO (supersede)
Replace `TODO.md:13-18` with:

> **Collapse sources_cdn.φ into sources_live.φ** — after the CI Archivar
> populates the CDN, the two-file split becomes a single `sources.φ`. The
> CDN/live distinction is a runtime decision: the Archivar tries CDN first
> for every source, falls back to live API."

### Amendment 4 — New rule
Insert in Architecture section:

> **The Archivar is the Manifestator of the CDN.** The Archivar manifests data
> into its own infrastructure. CI mode writes to the CDN (GitHub Releases).
> Local mode reads from the CDN. The naming convention is
> `{api_netloc}/{source_prefix}_{iso8601utc}.json`. The CDN is the Archivar's
> memory — no external catalog, no separate pipeline.

---

## 10. Exact Entry Point for Session 1

### What to do
Implement `--ci-mode` as a new CLI flag in `main()`. It loads `sources_live.φ`,
iterates all sources, checks `origin_stale`, fetches live APIs via `fetch_one`,
extracts via `extract_pending`, and writes the raw response body to
`out/{netloc}/{source_prefix}_{iso8601utc}.json`.

### Starting file
`src/main.rs`

### Key reference
- The existing `verify_sources_main()` pattern at `src/main.rs:5668` — same
  fetch + extract structure, but outputs files instead of counting.
- `fetch_one` at `:2084` — already handles curl with timeouts.
- `extract_pending` at `:3723` — unchanged.
- The naming convention: netloc from URL, source_prefix as last path segment
  sanitized, timestamp as ISO 8601 UTC.

### First skeleton
```rust
fn ci_mode_main() -> i32 {
    let srcs = load_sources();
    let now = tdb_now();
    for s in srcs.iter() {
        if s.url.starts_with("https://github.com/omegaflow/sources") {
            continue;
        }
        // origin_stale check (copy from warm_cache pattern)
        // fetch via fetch_one
        // extract via extract_pending → confirms data quality
        // write body to out/{netloc}/{name}_{utc_ts}.json
    }
    // TODO in session 2: gh release upload
    0
}
```

### Verification
1. `cargo check` must be 0 errors AND 0 warnings.
2. Run `cargo run -- --ci-mode` locally. Verify `out/` contains timestamped
   JSON files with correct netloc directories.
3. `cargo test --bin omegaflow -- --skip test_live_sources_extract` must pass
   13/13.

### What NOT to do
- Do NOT upload to CDN yet (that's session 2).
- Do NOT modify the local fetch loop (session 3).
- Do NOT touch `sources_cdn.φ` / `sources_live.φ` split (session 4).
- Do NOT delete Python scripts (session 5).
