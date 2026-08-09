# Session Handover — 2026-08-09

## Status

**Commit 2** (`c915cb8`): Inventory & Order — phi/research/ reorganization, thread-safety rules, scripts to ARCHIVED. Done.

**`biotic → electric`** (`d0e9b4c`): force_id_of, force_extent, force_constants_by_id updated. Done.

**Commit 0** (`efc72c7`): code-verified reference docs — BINARY_PROTOCOL, CONstANTS, KERNEL_SYSTEM (was FORCE_SYSTEM), EXTRACT_TYPES, URL_TEMPLATES, NIST SP 330/811 extracts, UCUM essence. Done.

**Architecture decision (Council unanimous): Kernel replaces Force**
- Grammar: `field <key> <unit> <kernel>` — 3 tokens, no `force` token
- 7 kernels: inverse-square, gaussian-inverse-square, gaussian-inverse, erfc, exponential-decay, patch-levy, inverse-linear
- Kernel specified per field, not per source block
- `phi/forces.φ` deleted, `phi/units.φ` deleted (NIST + UCUM are the binding unit references)
- `phi/sources.φ` = 0 bytes — nothing to migrate

## Next steps

### Commit 1 — main.rs Reconstruction (next)

**Scope**: Force → Kernel + Parser Rewrite. Single session, one commit.

**5 files touched**:
1. `src/main.rs` — kernel_id_of, kernel_extent, parser (3-token field), CLI, v2 removal
2. `static/index.html` — WGSL kernel switch (7 branches), 9→7 arrays, expose rewriting
3. `static/constants.js` — rename force_type → kernel_id
4. `phi/forces.φ` — deleted
5. `phi/units.φ` — deleted

**Deliverables**:
- Grammar: `field <key> <unit> <kernel>` (3 tokens, no force)
- `flush_v2!` macro removed
- `--ci-mode` removed
- CLI: `--fetch`, `--dump`, `--crawl`, `--verify`
- ~15 existing tests + ~25 new
- Zero "force" references in src/main.rs, static/
- Zero "v2" references in src/main.rs
- `cargo check` 0/0, all tests green
- Reference: `main.rs.bak` at `/home/johannes/projects/archive/omegaflow-phi-recovery/_archive/main.rs.bak`

### Commit 3 — Archivar Crawls & Verifies

- Deduplicate URLs from gold sources in `phi/research/`
- Live-test each URL individually
- Output: `verified_blocks.φ`, `partial_blocks.φ`, `failed_urls.φ`, `unknown_units.csv`

## Remaining cleanup

- `HANDOFF.md` — keep (running handover, this file)

## Context for next LLM

1. Read `AGENTS.md` first — 7 thread-safety rules (CRITICAL)
2. Read `TODO.md` — current inventory and pending work
3. `phi/sources.φ` is 0 bytes — start fresh
4. Gold sources for Commit 3 are in `phi/research/`
5. Archive at `/home/johannes/projects/archive/omegaflow-phi-recovery/`
6. `main.rs.bak` in archive
7. `cargo check` must be 0/0
8. Never read a directory — use `glob` with extension/prefix patterns
9. Max 3 bash calls per session, bundled with `&&`
10. No "for now", no "Phase 2", no backward compatibility
