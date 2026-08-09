# Session Handover — 2026-08-09 (Thread-Safety & Organization)

## What this session did

This session fixed the thread-safety problems that killed two previous sessions,
reorganized the recovery gold sources, and hardened the opencode configuration.

### Problem diagnosed

Two tools flood the context window and permanently freeze the session:
1. **`read` on directories** — 123-entry listing = 123 token vectors through the attention matrix
2. **`glob` with broad patterns** — 100+ results, same effect

### Fixes applied

**`opencode.json`:**
- `tool_output.max_lines`: 500 → **150**
- `tool_output.max_bytes`: 20480 → **10240**
- `experimental.batch_tool`: true → **false** (no parallel tool-call flood)
- `plan` agent model: Flash → **Pro** (user decision — Flash cannot plan the reconstruction)
- `general` agent model: Flash → **Pro** (these are the API-testing sub-agents that produced garbage in the batch session)
- `explore` agent: stays Flash (search-only, returns to Pro build agent)
- All sub-agents now have explicit `mode: "subagent"`

**`AGENTS.md` — new "Session Hygiene — Thread Safety" section (7 rules):**
1. **Never `read` a directory** → use `glob` with specific patterns
2. **Never `glob` without constraints** → must include extension or specific prefix
3. **Never `ls` in bash** → use `glob` instead
4. **Prefer `grep` → `read`** with offset+limit, never read whole files >80 lines
5. **Max 3 bash calls per session** → bundle with `&&`, absolute paths, no `cd`
6. **Split large reads** → files >100 lines: read in chunks with offset+limit
7. **Tool output caps enforced** → truncated output = signal to narrow query

### Directory reorganization

**`phi/research/`** — 4 subdirectories + 6 root φ files (10 entries, navigable):
```
phi/research/
  pre-cdn-history/    # 16 files — git-history gold (7 snapshots, ALL_lost_blocks_richest.φ, UNTESTED_blocks.φ, NEW_unchecked_blocks.φ)
  api-domains/        # 60 files — domain research, API gaps/potential, 21 Arena batches, gold φ
  git-extracts/       # 34 files — individual φ + commit messages from git archaeology
  reference/          # 4 files — source format spec + archive copies
  broken-2026-08-09.φ # evidence (fabricated force assignments)
  candidate-staging.φ, cdn-published.φ, dead-reachability.φ, live-verified.φ, pre-cdn-1924-blocks.φ
```

**`phi/recovery/`** moved to `/home/johannes/projects/archive/omegaflow-phi-recovery/` —
contains batch-session artifacts (dump_fields.csv, agent outputs, main.rs.bak, all_urls_dedup.txt).

### Document path updates

- `TODO.md`: Inventory section rewritten, all `phi/recovery/` → `phi/research/` or archive path
- `docs/source_curation.md`: all `phi/recovery/pre_cdn_history/` → `phi/research/pre-cdn-history/`
- `AGENTS.md`: source curation path updated, thread-safety section added

### Changes NOT yet committed

This session worked in Plan Mode (read-only) then Build Mode. The changes on disk
are staged for Commit 2 (Inventory & Order):

```
phi/research/          ← new (git add)
phi/recovery/          ← deleted (moved to archive)
opencode.json          ← modified
AGENTS.md              ← modified
TODO.md                ← modified
docs/source_curation.md ← modified
HANDOFF.md             ← modified (this file)
```

## Remaining cleanup (not done — next session)

1. **`scripts/migrate_to_v2.py.bak`** → move to `scripts/ARCHIVED/` (handover said "90% done")
2. **`HANDOFF.md`** — keep or archive? (it's the running handover)
3. **`SESSION_HANDOFF.md`** — older session handover, could go to archive
4. **`.gitignore`** — verify `migrate_to_v2.py.bak` would be blocked (`.py.bak` extension bypasses `*.py` rule)
5. **`cargo check`** — must be 0/0 before commit
6. **`git add` + `git commit`** — Commit 2 finalization

## Next steps (unchanged from prior handover)

After Commit 2 is finalized:

**Commit 0** — SI Reference & Books:
- `docs/reference/`: NIST SP 330, SP 811 PDFs + UCUM essence XML
- `docs/concepts/SI_UNITS.md`: omegaflow-binding SI table
- `phi/forces.φ`: force name, id, extent, velocity, absorption
- `phi/units.φ`: unit canonical string, allowed forces, SI base, conversion

**Commit 1** — main.rs Reconstruction:
- Git diff against pre-mess state (commits 188dd76 + 2cc6d5e)
- ONE parser, ONE format (4-token: `field <key> <force> <unit>`)
- CLI: `--fetch`, `--dump`, `--crawl`, `--verify` (no `--ci-mode`)
- Heart tests: ~15 existing + ~25 new fixture tests
- Reference: `main.rs.bak` at `/home/johannes/projects/archive/omegaflow-phi-recovery/_archive/main.rs.bak`

**Commit 3** — Archivar crawls & verifies:
- Deduplicate all URLs from gold sources in `phi/research/`
- Live-test each URL individually (no batch processing, no dump_fields.csv)
- Output: `verified_blocks.φ`, `partial_blocks.φ`, `failed_urls.φ`, `unknown_units.csv`
- Never write `sources.φ` directly — verified blocks are operator-reviewed candidates

## Three architectural gates (unchanged)

These prevent the batch-session disaster from recurring:
- **Gate 1** (Parse-time): refuse blocks without per-field force, force+unit mismatch, frame/key inconsistency
- **Gate 2** (Verify-time): field key must exist in live API response, unit must be recognized
- **Gate 3** (Operator-review): Force Gate human check, unknown units research

## Context for the next LLM

1. Read `AGENTS.md` first — contains the 7 thread-safety rules (CRITICAL)
2. Read `docs/source_curation.md` — curation protocol
3. Read `TODO.md` — current inventory and pending work
4. The gold sources for Commit 3 are in `phi/research/` (4 subdirectories)
5. The archive is at `/home/johannes/projects/archive/omegaflow-phi-recovery/`
6. `cargo check` must produce zero errors AND zero warnings
7. Never read a directory — use `glob` with extension/prefix patterns
8. Max 3 bash calls per session, bundled with `&&`
