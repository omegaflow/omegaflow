#!/bin/bash
set -u
D="$(cd "$(dirname "$0")" && pwd)"
G="http://127.0.0.1:1621/v1/chat/completions"
MODELS=(
  "cf/@cf/openai/gpt-oss-120b"
  "cf/@cf/meta/llama-3.3-70b-instruct-fp8-fast"
  "pollinations/openai-fast"
  "ollama/gemma4:31b"
)
run_one() {
  q="$1"; m="$2"
  qf="$D/frage-$q.txt"
  out="$D/out-$q"; mkdir -p "$out"
  safe=$(printf '%s' "$m" | tr '/:@ ' '____')
  jq -n --arg m "$m" --rawfile p "$qf" '{model:$m,messages:[{role:"user",content:$p}],temperature:0.2,max_tokens:3000}' \
    | curl -s -m 240 "$G" -H 'Content-Type: application/json' -d @- > "$out/$safe.raw" 2>/dev/null
  c=$(jq -r '.choices[0].message.content // .choices[0].message.reasoning_content // .choices[0].message.reasoning // empty' "$out/$safe.raw" 2>/dev/null)
  fr=$(jq -r '.choices[0].finish_reason // "?"' "$out/$safe.raw" 2>/dev/null)
  err=$(jq -r '.error.message // empty' "$out/$safe.raw" 2>/dev/null)
  printf '%s\n' "$c" > "$out/$safe.txt"
  n=$(grep -cE 'Verdikt' "$out/$safe.txt" 2>/dev/null || true)
  [ -n "$c" ] && st=OK || st=LEER
  printf '%s\t%s\t%s\tverdikte=%s\tlen=%s\tfinish=%s\terr=%s\n' "$q" "$m" "$st" "$n" "${#c}" "$fr" "$err"
}
export -f run_one
export D G
for q in b f g; do for m in "${MODELS[@]}"; do printf '%s\t%s\n' "$q" "$m"; done; done \
  | xargs -P 8 -n 2 bash -c 'run_one "$0" "$1"' \
  | sort > "$D/index.tsv"
cat "$D/index.tsv"
