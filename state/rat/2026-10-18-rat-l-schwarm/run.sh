#!/bin/bash
set -u
D="$(cd "$(dirname "$0")" && pwd)"
q="$D/frage.txt"
list="/home/johannes/projects/omegaflow/state/rat/2026-10-18-schwarm-44/free44.txt"
out="$D/out"
mkdir -p "$out"; : > "$out/index.tsv"
run() {
  m="$1"; safe=$(printf '%s' "$m" | tr '/:@ ' '____')
  jq -n --arg m "$m" --rawfile p "$q" '{model:$m,messages:[{role:"user",content:$p}],temperature:0.2,max_tokens:1200}' \
  | curl -s -m 120 http://127.0.0.1:1621/v1/chat/completions -H 'Content-Type: application/json' -d @- > "$out/$safe.raw" 2>/dev/null
  c=$(jq -r '.choices[0].message.content // empty' "$out/$safe.raw" 2>/dev/null)
  fr=$(jq -r '.choices[0].finish_reason // "?"' "$out/$safe.raw" 2>/dev/null)
  err=$(jq -r '.error.message // empty' "$out/$safe.raw" 2>/dev/null)
  printf '%s\n' "$c" > "$out/$safe.txt"
  n=$(grep -cE 'Verdikt:' "$out/$safe.txt" 2>/dev/null || true)
  [ -n "$c" ] && st=OK || st=LEER
  printf '%s\t%s\tverdikte=%s\tlen=%s\tfinish=%s\terr=%s\n' "$m" "$st" "$n" "${#c}" "$fr" "$err" >> "$out/index.tsv"
}
export -f run; export q out
grep -v '^[[:space:]]*$' "$list" | xargs -P 6 -I{} bash -c 'run "$@"' _ {}
sort -t$'\t' -k2,2 -k3,3r "$out/index.tsv"
