#!/bin/sh
# commit_msg.sh — a commit touching an architecture path needs a RAT-ID
# (Rat R-2026-10-18-d/e: verdict B, commit-msg hook).
#
# Architecture paths (measured, 2026-10-18): tools/gate/** .opencode/** AGENTS.md
# opencode.json docs/concepts/**. bin/** and .github/workflows/** stay build-free.
# Validity: syntax R-YYYY-MM-DD-x (Kimi: a register check would block new RATs).
set -u
msg="${1:?usage: commit_msg.sh <commit-msg-file>}"
arch='^(tools/gate/|\.opencode/|AGENTS\.md$|opencode\.json$|docs/concepts/)'
hit=$(git diff --cached --name-only --diff-filter=ACM | grep -E "$arch" || true)
[ -n "$hit" ] || exit 0
grep -qE 'R-[0-9]{4}-[0-9]{2}-[0-9]{2}-[a-z]' "$msg" && exit 0
echo "commit-msg: architecture path staged without a RAT-ID:" >&2
printf '%s\n' "$hit" | sed 's/^/  /' >&2
echo "  cite the verdict id as R-YYYY-MM-DD-x in the commit message" >&2
echo "  (index: state/stimmen/_rat-register.md; rule: R-2026-10-18-d/e)" >&2
exit 1
