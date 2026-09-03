#!/usr/bin/env bash
# The reflection-loop gate. The key resolves from ~/.local/share/opencode/auth.json
# (deepseek entry) or the OMEGAFLOW_LLM_KEY env — no key on this command line.
set -uo pipefail

BIN="${OMEGAFLOW_GATE_BIN:-$(dirname "$0")/../target/release/llm_interceptor}"
PORT="${OMEGAFLOW_LLM_PORT:-4100}"
UPSTREAM="${OMEGAFLOW_LLM_UPSTREAM:-https://api.deepseek.com}"

exec env OMEGAFLOW_LLM_PORT="$PORT" OMEGAFLOW_LLM_UPSTREAM="$UPSTREAM" "$BIN"
