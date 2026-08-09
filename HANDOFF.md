# Session Handover — 2026-08-10

## State

All commits: 0 warnings, 0 errors, 16/16 tests. Zero locks. Zero defaults.
Zero fabrications. Zero comments in source files.

## Architecture

**Kernel system** — 7 spatial shapes replace Force. Per-field, not per-block.
**Channel concurrency** — 0 Mutex, 0 RwLock, 0 Condvar. mpsc::channel.
**Protocol v3** — 88 bytes (11 × f64). `force_type` 11th field.
**5-token grammar** — `field <key> <name> <kernel> <force> <unit>` mandatory.
**τ-Gate** — `pend.tau = None` → `return vec![]`. No oscillation without physics.

## Key Files

| File | Lines | Status |
|------|-------|--------|
| `src/main.rs` | ~6300 | 0 warnings, Rust std-only |
| `static/index.html` | ~1290 | WGSL + JS, force_type in VOut |
| `static/constants.js` | ~110 | JS parser, protocol v3 check |

## Completed

- Commit 2: phi/research/ reorganization
- biotic → electric fix
- Commit 0: reference docs
- Commit 1a-e: Force→Kernel + WGSL + constants.js + file deletes
- Commit 1f: Mutex→Channel (0 locks)
- Commit 3: --verify CLI mode
- Protocol v3 (88-byte records, force_type 11th f64)
- 5-token grammar: field <key> <name> <kernel> <force> <unit>
- Scalar Extract → FieldConfig (9 variants)
- All defaults eliminated (15+ unwrap_or, _ => 0, max(1), reach_ttl, SourceConfig.tau)
- force_id_of → Option<u8> (parser reject)
- kernel_id_of → Option<u8> (parser reject)
- τ per Field (FieldConfig.tau), not per block
- τ-Gate: tau missing → no oscillator
- P0+P1: fetch pipeline wired, Lemma gate per-field
- Force-Farben WGSL: VOut.force_type, hue += force_type * 0.125
- fold_eff fix: v=0 → temporal only (no d/c)
- 0 comments in src/main.rs, static/index.html, static/constants.js

## Pending

### Render-Time (WGSL)
- Fragment shader uses `in.color` from vertex (force-hue already applied) — complete
- Advective per-Oszillator: wind speed in `tm.w` (unused slot). Needs data source.
- `PROPAGATION_SPEED[7] = 0.0` — honest absence

### Multi-Radiator (Plan B)
- `trait Radiator` + `RadiatorRegistry`
- ScreenRadiator, SpeakerRadiator, HapticRadiator, HardwareRadiator
- async accept via channel

### Source Curation
- phi/sources.φ = 0 bytes
- phi/research/ — gold sources with old grammar (needs migration)
- `--verify` CLI exists but untested with real URLs

## Context for Next LLM

1. `phi/sources.φ` is 0 bytes — greenfield
2. All defaults are eliminated — no `unwrap_or(0.0)`, no `_ => 0`, no `max(1)`
3. τ is per-field (`FieldConfig.tau`), mandatory for non-EM/non-Gravity kernels
4. `kernel_id_of` and `force_id_of` return `Option<u8>` — parser rejects unknowns
5. `cargo check` must be 0/0 — warnings are forbidden
6. No comments in source code
7. No German in code
8. Session is the atom — planning and implementation in same context window
