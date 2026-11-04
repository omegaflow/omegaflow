# Contributing

## Before you change anything

Read `COMMITS.md`. It is short and it holds: an area, a subject of at most 72
characters, a blank line, then a body that carries the why.

## The path of a change

1. Measure. What is the state, with which command, on which date?
2. Build. The change and its test in the same commit.
3. Close. `RUSTFLAGS="-D warnings" cargo check --workspace --all-targets` runs
   without a finding.
4. Commit. English, one sentence per thing, the number with its command.
5. Push. No green run, no claim.

## What a commit must not do

- invent values: `0.0`, `unwrap_or_default`, invented tolerances;
- switch a check off instead of fixing the cause;
- leave a test gap behind: every package in the workspace runs in CI.

## Questions and disagreement

Open an issue. Architecture and ethics are decided by the maintainers.
