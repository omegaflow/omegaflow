#!/usr/bin/env bash
set -euo pipefail

repo="${1:?repo}"
title="${2:?title}"
body="${3:?body}"
label="${4:-health}"

if gh issue list --repo "$repo" --state open --label "$label" --json title --jq '.[].title' | grep -Fqx "$title"; then
  echo "issue already open: $title"
  exit 0
fi

gh issue create --repo "$repo" --title "$title" --body "$body" --label "$label"
