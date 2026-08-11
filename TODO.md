# TODO

AGENTS.md is the primary constraint matrix. Git is the history. Pending work only.

## Status — 2026-08-11

**Done:**
- Commit 2: phi/research/ reorganization, thread-safety rules, scripts to ARCHIVED
- Commit 0: docs/reference/ (code-verified, NIST SP 330/811, UCUM)
- Force→Kernel: 7 kernels replace 8 forces, per-field
- Mutex→Channel: 0 locks, 0 RwLock, 0 Condvar, mpsc::channel
- Protocol v3: 96-byte records (12 × f64)
- τ per Field: `FieldConfig.tau`, τ-Gate (tau absent or 0.0 → gate closes via `continue` in fetch loop; 3-part field form has open gate: sets tau 0.0 without check)
- Fetch pipeline wired: Lemma gate per-field, `fetched_oscillators → all → build_buffer`
- Multi-Radiator: Tcp (Screen) + Audio (PCM stdout) + Stderr (field stats, not Haptic/Hardware)
- Force-Farben WGSL: VOut.force_type, hue per kernel+force
- fold_eff fix: v=0 forces → temporal decay only (no d/c)
- `max(1)` and `reach_ttl` eliminated from source
- 0 comments in all source files (bsp_reader doc comments removed 2026-08-11)
- Overflow Protocol restored to AGENTS.md System Directive
- Archaeology consolidated: `archeology/` (56 φ sources, 370+ research docs), `docs/omegaflow_archeology.zip`
- GitHub cleanup: 22 issues closed, ~80 failed runs deleted, mirror-cdn cron disabled
- Probe auto-curation: `probe_classify` (23 force rules → DROP or field decl), field output in `walk_json_probe` (Num + Str arms) and `probe_csv`

## Source Curation (next)

### Gold Source Migration
- `archeology/sources/sources_gold_359-domains.φ` — 27K lines, 359 domains, OLD grammar
- EVERY source block needs migration: `force em` → `field <key> <name> <kernel> <force> <unit>`
- Arena batches (`archeology/arena/`) contain API proposals not yet in φ files

### Untested Blocks
- `archeology/sources/sources_new_untested_14k_new-unchecked.φ` — 873 unchecked blocks
- `archeology/sources/sources_new_untested_candidate-staging.φ` — staging candidates

### Gap Analysis
- `archeology/foundation/gaps.md` — domain coverage gaps
- `archeology/foundation/collection.md` — curated collection state

### Residual Defaults
- `unwrap_or(0.0)` 3× in `load_sources_from` `on` handler: lat, lon, alt (:6666-6668)
- `_ => 0.0` in `kernel_extent` wildcard arm (:2088)
- `unwrap_or(0)` at :904
- 3-part `field` form hardcodes `tau: 0.0` with no gate (:3735)
- `kernel_for_force` has arm `8 => 5` referencing non-existent force id 8 (:2038-2046)

### Validation
- `--verify` CLI exists (tests URL reachability), no sources loaded yet
- Old sources: `pos` and `field_in` tokens → parser ignores via `_ => {}`; `force` token IS parsed (used for 3-part field form)

## CI Pipeline

- `mirror-cdn.yml`: cron disabled, uses `--verify` manually
- `generate-ephemerides.yml`: Python/spiceypy → needs Rust rewrite
- `refresh-protected-data.yml`: Python inline scripts → needs Rust rewrite

## Feature Backlog

- Advective per-Oszillator: wind speed in `tm.w` (channel wired, data source needed)
- OPeNDAP Integration
- New Extract Types: Kepler, HorizonsVec, Flatten
- `field_in` nested support
- Command Palette (⌘K)
- Minkowski 4D Weighting

## Deferred

- Temporal Topology (TDA, Takens, Transfer Entropy) — needs AGENTS.md amendment
- Field Permeability — needs AGENTS.md amendment
- 10 Free-API-Keys registration
- Workflow Secret Wiring

## Rejected

- Unknown-Force soft fallback → parser rejects unknown force
- Default τ values → gate closes if not declared
- World Bank Indicators → forceless, DROP
- Yahoo Finance → forceless, DROP
