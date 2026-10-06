#!/usr/bin/env bash
# register_release_set.sh <netloc> [<register>]
#
# Prints the asset names registered for <netloc> in the source register, one per
# line, sorted and unique. The register is the source of truth: a workflow that
# manifests a release reads its expected set here instead of maintaining its own
# list (survey-2026-09-03-orphan-verdicts Step 5).
set -euo pipefail
netloc="${1:?usage: register_release_set.sh <netloc> [<register>]}"
register="${2:-phi/sources.φ}"
esc="${netloc//./\\.}"
grep -oE "releases/download/${esc}/[^ \"]+" "$register" | xargs -r -n1 basename | sort -u
