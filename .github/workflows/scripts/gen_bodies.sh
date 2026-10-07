#!/usr/bin/env bash
# gen_bodies.sh [--check <file>] — derive the membrane BODIES array from the
# pages-deploy stage lines.
#
# The set is measured from the workflow's `stage <tag> ephemeris_de440_<body>.bin
# <sha>` lines; the order is the declared visibility rank (the operator gate
# first: the sun, then earth and moon; the star catalog is not a body and streams
# in last). Without --check it prints the canonical `const BODIES = [...]` line;
# with --check <file> it fails unless <file> carries exactly that line — the
# generated-manifest gate, so the hand copy in static/membrane.html can no longer
# drift from the staged bodies.
set -euo pipefail
workflow=.github/workflows/pages-deploy.yml
rank=(sun earth moon)
seen=$(grep -oE 'ephemeris_de440_[a-z]+\.bin' "$workflow" \
  | sed -E 's/^ephemeris_de440_(.*)\.bin$/\1/' | sort -u | tr '\n' ' ')
if [ -z "${seen// /}" ]; then
  echo "no ephemeris_de440_<body>.bin stage line in $workflow" >&2
  exit 2
fi
for body in $seen; do
  ranked=0
  for r in "${rank[@]}"; do
    if [ "$body" = "$r" ]; then ranked=1; fi
  done
  if [ "$ranked" != 1 ]; then
    echo "body '$body' has no visibility rank in gen_bodies.sh" >&2
    exit 2
  fi
done
literal=""
for r in "${rank[@]}"; do
  case " $seen " in
    *" $r "*) if [ -n "$literal" ]; then literal="$literal, "; fi
              literal="${literal}\"$r\"" ;;
  esac
done
line="const BODIES = [$literal];"
if [ "${1:-}" = "--check" ]; then
  file="${2:?usage: gen_bodies.sh [--check <file>]}"
  if ! grep -qxF "$line" "$file"; then
    echo "BODIES drift in $file — expected:" >&2
    echo "  $line" >&2
    exit 1
  fi
  echo "BODIES manifest in sync: $line"
else
  printf '%s\n' "$line"
fi
