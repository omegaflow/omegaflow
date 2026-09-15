# Security & responsible use

The `archive_search` tool (`tools/utils/src/bin/archive_search.rs`) is a local
research instrument. This note names its credentialed paths and the
responsible-use limits.

## Credentialed modes

- `--ads` reads `NASA_ADS_TOKEN`
- `--brave` reads `BRAVE_API_KEY`
- `--github` reads `GITHUB_SEARCH_TOKEN` (a read-only GitHub token; the write
  token `OMEGAFLOW_TOKEN` belongs to the CI publish path only)
- Earthdata hosts retry with `EARTHDATA_EDL_TOKEN` (or `EARTHDATA_USER` /
  `EARTHDATA_PASS`)

## Network exits

The fetch ladder tries a direct exit, then any live Proton tunnel (`proton-wg`),
then its SOCKS proxy. `--playwright` routes the browser through the same SOCKS
proxy when one is up. Rotation between exits is an operator action
(`bin/proton-wg.sh`, `bin/proton-exit.sh`), not part of the tool.

## Limits

- Credentials live in `.secrets.local` (gitignored) and are never committed.
- Operator-facing modes honor the double-ask before they run.
- `--mft` reads a raw device path and is for the operator's own volumes only.
