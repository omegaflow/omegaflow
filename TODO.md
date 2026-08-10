# TODO

AGENTS.md is the primary constraint matrix. Git is the history. Pending work only.

## Status — 2026-08-10

**Done:**
- Commit 2: phi/research/ reorganization, thread-safety rules, scripts to ARCHIVED
- Commit 0: docs/reference/ (code-verified, NIST SP 330/811, UCUM)
- Force→Kernel: 7 kernels replace 8 forces, per-field, grammar: `field <key> <name> <kernel> <force> <unit>` (5 tokens)
- Mutex→Channel: 0 locks, 0 RwLock, 0 Condvar, mpsc::channel
- Protocol v3: 88-byte records (11 × f64), force_type 11th field
- All defaults eliminated: no `unwrap_or(0.0)`, no `_ => 0`, no `max(1)`, no `reach_ttl`
- τ per Field: `FieldConfig.tau`, τ-Gate (tau missing → `return vec![]`)
- Fetch pipeline wired: Lemma gate per-field, `fetched_samples → all → build_buffer`
- Multi-Radiator: Screen + Speaker (PCM stdout) + Haptic (stderr) + Hardware (stderr)
- Force-Farben WGSL: VOut.force_type, hue per kernel+force
- fold_eff fix: v=0 forces → temporal decay only (no d/c)
- 0 comments in source files
- Overflow Protocol restored to AGENTS.md System Directive
- Archaeology consolidated: `archeology/` (56 φ sources, 370+ research docs), `docs/omegaflow_archeology.zip`
- GitHub cleanup: 22 issues closed, ~80 failed runs deleted, mirror-cdn cron disabled

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

### Validation
- `--verify` CLI exists (tests URL reachability), no sources loaded yet
- Old sources use `force`, `pos`, `field_in` tokens → parser ignores them (grammar mismatch)

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
