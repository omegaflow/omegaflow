# The repository

2258 files, 62.5 MB. What each area is, and what it carries.

| Area | Files | Size | What |
|---|---:|---:|---|
| `src/` | 460 | 34.2 MB | the Archivar core (408), the Mathematikerin (49), the wasm bridge |
| `tools/` | 1191 | 16.1 MB | 969 binaries: harvest 542, measure 390, utils 180, register 48, membrane 14, science 6, service 6, app 3 |
| `.github/` | 515 | 1.2 MB | 511 workflows: 392 cdn, 18 probe, 82 other, 7 check, 4 te, 3 watch, 3 build, 1 firmware, 1 pages |
| `phi/` | 16 | 7.3 MB | the registers: instruments 3.1 MB, digests 2.4 MB, sources 1.4 MB, field model, licences |
| `docs/` | 13 | 3.5 MB | this index, the specs, two references, the publish log, the landing image |
| `static/` | 17 | 0.1 MB | the membrane page and the landing page |
| `firmware/` | 15 | 0.1 MB | the sensor board: radiatorium and its library |
| `bin/` | 4 | — | the four scripts the workflows call |
| `workers/` | 4 | — | the r2 verifier |
| `LICENSES/` | 1 | — | the licence texts |

The largest files: the lunar kernel `moon_pa_de440_200625.bpc` at 12.3 MB, two
LAS fixtures at 3.8 and 3.6 MB, a LAZ fixture at 0.7 MB, the three registers,
the landing animation at 1.7 MB, the CDN reconciliation at 0.8 MB.

## The shape

One Rust core, one binary per job, one workflow per dataset. The 392 cdn
workflows do not start on a push; they run on a schedule or by dispatch, so a
commit does not fan out into the harvest fleet.

## What is not here

The data itself lives in a private release repository. The register
`phi/sources.φ` names what is published: 1627 blocks, every one with an admitted
licence.

## Open

- 13681 assets in the data repository have no entry in the register; their
  licence is unknown and none of them is manifest.

## Rules

- `COMMITS.md` — the form of a commit, checked by `commit-message`.
- A relative link in this tree must resolve; `docs-links` fails the run when one
  does not.
