# TODO

AGENTS.md is the primary constraint matrix. Git is the history. This file contains only pending work.

## Status — 2026-08-09

**Done:**
- Commit 2 (`c915cb8`): phi/research/ reorganization, thread-safety rules, scripts to ARCHIVED
- `biotic → electric` (`d0e9b4c`): force_id_of, force_extent, force_constants_by_id
- Commit 0 (`efc72c7`): docs/reference/ (7 code-verified files, NIST SP 330/811, UCUM)
- `phi/units.φ` and `phi/forces.φ`: created but become obsolete with kernel decision (see below)
- `out/*` emptied, `SESSION_HANDOFF.md` deleted

**Architecture decision (Council unanimous): Kernel replaces Force**
- Grammar: `field <key> <unit> <kernel>` — 3 tokens, no `force` token
- 7 kernel shapes: inverse-square, gaussian-inverse-square, gaussian-inverse, erfc, exponential-decay, patch-levy, inverse-linear
- Kernel specified per field, not per source block
- `force` token rejected by parser
- `phi/forces.φ` deleted, `phi/units.φ` deleted (NIST SP 330/811 + UCUM are binding)

**phi/sources.φ = 0 bytes.** Nothing to migrate.

## Commit 1 — main.rs Reconstruction (next atomic unit)

### Scope: Force → Kernel + Parser Rewrite

Single session, one commit. No phases.

**Grammar change:**
```
field <key> <force> <unit>          (old, 4 tokens, removed)
field <key> <unit> <kernel>         (new, 3 tokens)
```

**Cut list — zero "force" remaining:**
- `force_id_of`, `force_extent`, `force_type_val`, `force_constants_by_id`, `force_tau_of_id`, `force_extent_of_id` — all deleted
- `Sample.force_type` → `kernel_id`
- `SourceConfig.force: String` → removed
- `cur_forces` → removed
- `flush_v2!` macro → removed (v2 naming dies)
- All `src.force.split_whitespace()` loops → extent from field kernels
- `--ci-mode` → removed
- WGSL: `force_type` → `kernel_id`, `9u → 7u`, `expose_hi2` removed
- JS: `forceExposeF` → `kernelExpose`, `probedOmegas` 9→7
- `phi/forces.φ` → deleted
- `phi/units.φ` → deleted

**Add:**
- `kernel_id_of(name) -> Option<u8>` — 7-arm match
- `kernel_extent(id, body_props, ttl, tau) -> f64` — per-kernel extent formula

**CLI:**
- `--fetch` (auto-detect CDN↔local)
- `--dump`
- `--crawl`
- `--verify`
- `--ci-mode` removed

**Parser:**
- ONE parser, ONE format
- 3-token field: `field <key> <unit> <kernel>`
- `force` token → parser error
- Unknown kernel → source block refused
- No field without kernel → source block refused

**Tests:**
- ~15 existing (adapt to kernel_id)
- ~25 new fixture tests
- Reference: `main.rs.bak` at `/home/johannes/projects/archive/omegaflow-phi-recovery/_archive/main.rs.bak`

**Post-commit verification:**
- `cargo check` 0/0, all tests green
- `cargo run` → 127.0.0.1:1111 → non-black render window
- `grep -r "force" src/main.rs` returns zero matches
- `grep -r "force_type" static/` returns zero matches
- `grep -r "v2" src/main.rs` returns zero matches

### 5 files touched:
1. `src/main.rs` — all parser + kernel logic
2. `static/index.html` — WGSL shader + JS
3. `static/constants.js` — variable rename
4. `phi/forces.φ` — deleted
5. `phi/units.φ` — deleted

## Commit 3 — Archivar Crawls & Verifies

- Deduplicate all URLs from gold sources in `phi/research/`
- Live-test each URL individually (no batch)
- Output: `verified_blocks.φ`, `partial_blocks.φ`, `failed_urls.φ`, `unknown_units.csv`
- Never write `sources.φ` directly — verified blocks are operator-reviewed candidates

## Three Architectural Gates

- **Gate 1** (Parse-time): refuse blocks without per-field kernel, unknown kernel, field without unit
- **Gate 2** (Verify-time): field key must exist in live API response, unit must be recognized
- **Gate 3** (Operator-review): physical validity of kernel assignment

---

## Source Curation (LTS)

### Pending Blocks
- 423 UNTESTED blocks (`phi/research/pre-cdn-history/UNTESTED_blocks.φ`)
- 873 NEW_unchecked blocks (`phi/research/pre-cdn-history/NEW_unchecked_blocks.φ`)
- 96 Audit Findings (`scripts/api_audit.py`)
- 5 poorer blocks: co2_global_network, nsidc_arctic/antarctic, swpc_solar_wind_dscovr, nist_codata
- 93 HAPI blocks

### Pre-CDN Format Migration (after Commit 1)
- EM TAP astronomy (130): VOTable parser needed
- EM PDG (38): static, keyless, ttl≥604800
- Acoustic (105): NDBC buoys, NOAA CO-OPS, GCOOS, METAR
- Diffusion (186): USGS water quality, AERONET, Open-Meteo air quality
- Gravity BGS (162): magnetic observatories
- Gravity JPL-SSD (104): Horizons — needs parser support
- Seismic (113): USGS/EMSC/EarthScope dedup
- Thermal (73): NCEI climate, NSIDC sea ice, FIRMS fire

### Source Inventory Reclassification
2189 pre-cdn blocks → IMPLEMENT-NOW(85%) / ACQUIRE-KEY(3%) / DEFER(10%) / DROP(2%)

### CDN-Switch Loss Recovery
simbad(84), ssd.jpl.nasa.gov(48), gea.esac.esa.int(50), overpass.openstreetmap.fr(235)

---

## Feature Backlog

- OPeNDAP Integration for NASA EarthData
- New Extract Types: Kepler, HorizonsVec, Flatten
- `field_in` nested support
- Window/Temporal `from`/`until` bounding
- Command Palette (⌘K)
- Minkowski 4D Weighting (WGSL kernel refinement)
- 2-finger gesture handling (touchpad pinch/zoom)

---

## Deferred

### Require AGENTS.md Amendment
- Temporal Topology (TDA, Takens, Transfer Entropy)
- Field Permeability (`tanh(vC/g)`)

### Operator / Key Acquisition
- 10 Free-API-Keys registrieren
- 8 Secrets in `.secrets.local` + GitHub Actions
- Workflow Secret Wiring

### Out of Scope
- ESP32 Mantis-Shrimp Firmware (separate project)
- Global Station Web via Nostr (separate infrastructure)
- Water Metaphor / Total Coherence / Temporal Manifestation (visionary, no architecture)

---

## Rejected

- Unknown-Force soft fallback to `em` — AGENTS.md: forceless sources refused at load. A = A.
- World Bank Indicators (228 blocks) — economic statistics, no physical force. DROP.
- Yahoo Finance (37 blocks) — stock prices, symbolic abstractions. DROP.
