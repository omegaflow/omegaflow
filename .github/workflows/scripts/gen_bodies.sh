#!/usr/bin/env bash
# gen_bodies.sh --write <file> — write the membrane ephemeris manifest from the
# pages-deploy stage lines.
#
# The set is measured from the workflow's `stage <tag> ephemeris_de440_<body>.bin
# <sha>` lines, in the workflow's declared order: the hull is the only admission
# criterion, no hand rank. The output is newline-separated body names staged as
# /membrane_bodies.txt; static/membrane.html reads it, so the page never writes a
# body name. Drift is impossible — the manifest is derived from the same stage
# lines the deploy fetches.
set -euo pipefail
workflow=.github/workflows/pages-deploy.yml
bodies=$(grep -oE 'ephemeris_de440_[a-z]+\.bin' "$workflow" \
  | sed -E 's/^ephemeris_de440_(.*)\.bin$/\1/' \
  | awk '!seen[$0]++')
if [ -z "$bodies" ]; then
  echo "no ephemeris_de440_<body>.bin stage line in $workflow" >&2
  exit 2
fi
if [ "${1:-}" = "--write" ]; then
  file="${2:?usage: gen_bodies.sh --write <file>}"
  printf '%s\n' "$bodies" > "$file"
  echo "wrote $file:"
  printf '%s\n' "$bodies"
else
  printf '%s\n' "$bodies"
fi
