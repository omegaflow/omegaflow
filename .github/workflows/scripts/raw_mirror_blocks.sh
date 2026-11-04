#!/usr/bin/env bash
# raw_mirror_blocks.sh <before> <sha>
#
# The generic register -> manifest actor for the compiler-less raw_mirror
# blocks. It reads phi/sources.φ and mirrors the changed blocks that carry the
# explicit manifestation line "harvest raw_mirror" through raw_mirror. The
# manifestation line selects the actor; a bare format line does not.
#
# The four gates it runs through:
#   Lizenz-Tor         - a block without a terms line stays unmirrored
#   Fingerabdruck-Tor  - raw_mirror refuses a body that does not hash to the
#                        registered sha256 (house fingerprint) or md5 (the
#                        source gate Zenodo publishes) for dataset-scale assets
#   Format-Tor         - the arm equals the manifestation line (raw_mirror)
#   Registrierungs-Tor - the block stands in the register with url/origin/sha256
set -euo pipefail

before="${1:?raw_mirror_blocks.sh <before> <sha>}"
sha="${2:?raw_mirror_blocks.sh <before> <sha>}"
register="phi/sources.φ"

if [ "$before" = "0000000000000000000000000000000000000000" ]; then
  before="${sha}^"
fi

changed_file="$(mktemp)"
records_file="$(mktemp)"
cleanup() { rm -f "$changed_file" "$records_file"; }
trap cleanup EXIT

# the changed new-file line numbers of the register (zero-context hunks); a
# missing parent leaves the change set empty rather than aborting the actor
git diff --unified=0 "$before" "$sha" -- "$register" 2>/dev/null \
  | awk "/^@@/ { for (i = 1; i <= NF; i++) if (\$i ~ /^\+[0-9]/) { t = substr(\$i, 2); split(t, r, \",\"); n = r[1] + 0; c = (r[2] == \"\" ? 1 : r[2] + 0); for (j = 0; j < c; j++) print n + j } }" \
  | sort -n -u > "$changed_file" || true

declare -A changed=()
mapfile -t changed_lines < "$changed_file"
for n in "${changed_lines[@]}"; do
  changed["$n"]=1
done

gate_bad=0
block_url=""
block_origin=""
block_sha=""
block_md5=""
block_terms=""
block_has=0
block_hit=0

reset_block() {
  block_url=""
  block_origin=""
  block_sha=""
  block_md5=""
  block_terms=""
  block_has=0
  block_hit=0
}

emit_block() {
  if [ "$block_has" = "1" ] && [ "$block_hit" = "1" ]; then
    if [ -z "$block_url" ] || [ -z "$block_origin" ]; then
      echo "raw_mirror: harvest raw_mirror without url/origin - Registrierungs-Tor: stays unmirrored" >&2
      gate_bad=1
    elif [ -z "$block_sha" ] && [ -z "$block_md5" ]; then
      echo "raw_mirror: $block_url: harvest raw_mirror without sha256/md5 - Fingerabdruck-Tor: stays unmirrored" >&2
      gate_bad=1
    elif [ -z "$block_terms" ]; then
      echo "raw_mirror: $block_url: harvest raw_mirror without terms - Lizenz-Tor: stays unmirrored" >&2
      gate_bad=1
    elif [ -n "$block_sha" ]; then
      printf "%s|%s|sha256|%s\n" "$block_url" "$block_origin" "$block_sha"
    else
      printf "%s|%s|md5|%s\n" "$block_url" "$block_origin" "$block_md5"
    fi
  fi
  reset_block
}

mapfile -t register_lines < "$register"
nr=0
for line in "${register_lines[@]}"; do
  nr=$((nr + 1))
  if [ -z "${line//[[:space:]]/}" ]; then
    emit_block
    continue
  fi
  if [ -n "${changed[$nr]:-}" ]; then
    block_hit=1
  fi
  first="${line%% *}"
  rest="${line#* }"
  case "$first" in
    harvest)
      if [ "$rest" = "raw_mirror" ]; then
        block_has=1
      fi
      ;;
    url) block_url="$rest" ;;
    origin) block_origin="$rest" ;;
    sha256) block_sha="$rest" ;;
    md5) block_md5="$rest" ;;
    terms) block_terms="$rest" ;;
  esac
done > "$records_file"
emit_block >> "$records_file"

if [ ! -s "$records_file" ]; then
  if [ "$gate_bad" = "0" ]; then
    echo "raw_mirror: no changed harvest raw_mirror block - nothing to mirror"
    exit 0
  fi
fi
if [ "$gate_bad" != "0" ]; then
  echo "raw_mirror: an unbacked manifest line refused the mirror (see above)" >&2
  exit 1
fi

cargo build --release -p omegaflow-harvest --bin raw_mirror
mirror_bin="target/release/raw_mirror"
bad=0
mapfile -t records < "$records_file"
for record in "${records[@]}"; do
  IFS="|" read -r url origin flag val <<< "$record"
  if [ -z "$url" ]; then
    continue
  fi
  echo "raw_mirror: $url"
  "$mirror_bin" --ci-mode --url "$url" --origin "$origin" --"$flag" "$val" \
    || { echo "raw_mirror: $url returned void" >&2; bad=1; }
done
exit "$bad"
