#!/bin/bash
# Run the three questions over the 44 measured free carriers. Output stays in this dir.
set -u
D="$(cd "$(dirname "$0")" && pwd)"
q="$D/formular.txt"
out="$D/out"
mkdir -p "$out"
: > "$out/index.tsv"
run() {
  m="$1"; safe=$(printf '%s' "$m" | tr '/:@ ' '____')
  jq -n --arg m "$m" --rawfile p "$q" '{model:$m,messages:[{role:"user",content:$p}],temperature:0.2,max_tokens:2200}' \
  | curl -s -m 200 http://127.0.0.1:1621/v1/chat/completions -H 'Content-Type: application/json' -d @- > "$out/$safe.raw" 2>/dev/null
  c=$(jq -r '.choices[0].message.content // empty' "$out/$safe.raw" 2>/dev/null)
  fr=$(jq -r '.choices[0].finish_reason // "?"' "$out/$safe.raw" 2>/dev/null)
  ct=$(jq -r '.usage.completion_tokens // 0' "$out/$safe.raw" 2>/dev/null)
  err=$(jq -r '.error.message // empty' "$out/$safe.raw" 2>/dev/null)
  printf '%s\n' "$c" > "$out/$safe.txt"
  n=$(grep -cE 'Verdikt:' "$out/$safe.txt" 2>/dev/null || true)
  if [ -n "$c" ]; then st=OK; else st=LEER; fi
  printf '%s\t%s\tverdikte=%s\tout=%s\tlen=%s\tfinish=%s\terr=%s\n' "$m" "$st" "$n" "$ct" "${#c}" "$fr" "$err" >> "$out/index.tsv"
}
export -f run; export q out
grep -v '^[[:space:]]*$' "$D/free44.txt" | xargs -P 6 -I{} bash -c 'run "$@"' _ {}
sort -t$'\t' -k2,2 -k3,3r "$out/index.tsv"
