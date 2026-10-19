# Die freien Träger — gemessen 2026-10-18

Zwei Proben: **278** Modelle der Gateway-Frei-Präfixe + **121** der als bezahlt geführten
(16 openrouter `:free`, 105 unorouter). Ein Request je Modell, 0,6 s Pause, keine Retries.

## Tragen (HTTP 200 **mit** Inhalt)

- `cf/@cf/aisingapore/gemma-sea-lion-v4-27b-it`
- `cf/@cf/daily-co/phonellm-mini-alpha-2`
- `cf/@cf/google/gemma-7b-it-lora`
- `cf/@cf/ibm-granite/granite-4.0-h-micro`
- `cf/@cf/meta/llama-3.1-8b-instruct-fp8`
- `cf/@cf/meta/llama-3.2-1b-instruct`
- `cf/@cf/meta/llama-3.3-70b-instruct-fp8-fast`
- `cf/@cf/meta/llama-4-scout-17b-16e-instruct`
- `cf/@cf/meta-llama/llama-2-7b-chat-hf-lora`
- `cf/@cf/mistral/mistral-7b-instruct-v0.2-lora`
- `cf/@cf/openai/gpt-oss-120b`
- `cf/@cf/openai/gpt-oss-20b`
- `cf/@cf/qwen/qwen2.5-coder-32b-instruct`
- `cf/@cf/qwen/qwen3-30b-a3b-fp8`
- `cf/@cf/qwen/qwq-32b`
- `cf/@cf/zai-org/glm-4.7-flash`
- `groq/allam-2-7b`
- `groq/qwen/qwen3.8-27b`
- `kenari/agnes-2-5-flash:free`
- `kenari/agnes-3-0-flash:free`
- `kenari/beam-1-0:free`
- `kenari/deepseek-v4-1-flash:free`
- `kenari/dots-3-note-preview:free`
- `kenari/hy3:free`
- `kenari/nemotron-3-nano-omni-30b-a3b-reasoning:free`
- `kenari/nemotron-3-super-120b-a12b:free`
- `kenari/nemotron-3-ultra-550b-a55b:free`
- `kenari/space-bunny:free`
- `kenari/step-5-preview:free`
- `kilo/kilo-auto/small`
- `llm7/codestral-latest`
- `llm7/DeepSeek-V4-Flash-0731`
- `llm7/minimax-m2.7`
- `llm7/mistral-Nemo-Instruct-2407`
- `ollama/gemma4:31b`
- `openrouter/nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free`
- `openrouter/nvidia/nemotron-3-ultra-550b-a55b:free`
- `openrouter/poolside/laguna-s-2.1:free`
- `openrouter/xingchen-agi/xing4.0-29b-a4b:free`
- `ovh/Meta-Llama-3_3-70B-Instruct`
- `ovh/Mistral-Nemo-Instruct-2407`
- `ovh/Mistral-Small-3.2-24B-Instruct-2506`
- `ovh/Qwen2.5-VL-72B-Instruct`
- `pollinations/openai-fast`

## Ausfall nach Grund

| Grund | Anzahl |
|---|---|
| HTTP 402 | 191 |
| HTTP 000 | 71 |
| HTTP 400 | 40 |
| HTTP 200 | 28 |
| HTTP 429 | 14 |
| HTTP 403 | 9 |
| HTTP 503 | 2 |

**Wichtig:** 402 kommt vom **Anbieter** (kein Guthaben), nicht vom Haus-Gateway.
**unorouter** ist von hier nicht erreichbar (70× 000) — tot, nicht bezahlt.
**openrouter**: 4 von 16 `:free` tragen, 8 antworten leer (Reasoning frisst das Budget).

## OpenCode-Provider (nicht im Gateway)

9 frei live; 1 gemessen (Mechanik-Test bestanden): `opencode/deepseek-v4.1-flash-free`.
