#!/usr/bin/env bash
# register_release_set.sh <netloc> [<prefix>] [<register>]
#
# Prints the asset names registered for <netloc> in the source register, one per
# line, sorted and unique. An optional <prefix> keeps only the assets carrying
# that prefix — the scoped binding for a release written by several workflows.
# The register is the source of truth: a workflow that manifests a release reads
# its expected set here instead of maintaining its own list
# (survey-2026-09-03-orphan-verdicts Step 5).
set -euo pipefail
netloc="${1:?usage: register_release_set.sh <netloc> [<prefix>] [<register>]}"
prefix="${2:-}"
register="${3:-phi/sources.φ}"
esc="${netloc//./\\.}"
grep -oE "releases/download/${esc}/[^ \"]+" "$register" | xargs -r -n1 basename | sort -u \
  | { if [ -n "$prefix" ]; then grep "^${prefix}"; else cat; fi; }
