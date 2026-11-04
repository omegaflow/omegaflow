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


## File names

Each class has one shape. A new file that breaks it is renamed in the same
commit that adds it.

| Class | Shape | Example |
|---|---|---|
| Rust source and binaries | snake_case | `fdsn_waveform_compiler.rs` |
| Registers and tables | snake_case | `field_model_definition.tsv` |
| Shell scripts | snake_case | `cdn_put`, `source_branch_guard` |
| Workflows | kebab-case | `fdsn-waveform-cdn.yml` |
| Markdown below the root | kebab-case | `auftrag-2026-11-02-sources-ci.md` |
| Root markdown | uppercase | `README.md`, `CHANGELOG.md` |
| External standards | their own name | `naif0012.tls`, `pck00011.tpc` |

An external file keeps the name its publisher gave it: the version lives in the
name and a rename would cut the provenance.
