<!--
  title: Handover — NIM-Spezialmodelle (Stand 2026-09-13)
  session: NIM-Spezialmodelle
  class: handover
  date: 2026-09-13
  sha256: fc6a3c3fb0bdd6c2c808713728bd20925fe17386f362fc51f5836961b5990a2b
  status: live
-->
# Handover — NIM-Spezialmodelle (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Messung — NIM-Katalog gegen Kontingent (2026-09-13)

- `integrate.api.nvidia.com/v1/models` trägt 82 Namen; die Chat-Sonde (je ein Ruf)
  erreicht 20 davon — 14 allgemeine Chat-Modelle, 6 Spezialzweck-Modelle.
  55 Namen antworten `404 Function ... not found for account` (Kontingent, nicht
  Katalog); 3 hängen ohne Antwort (`google/gemma-4-31b-it`,
  `meta/llama-guard-4-12b`, `meta/llama-3.2-90b-vision-instruct`); 2 antworten
  `500` (`nvidia/ai-synthetic-video-detector`,
  `nvidia/llama-3.1-nemoguard-8b-topic-control`).
- Tragend (Chat, 14): `deepseek-ai/deepseek-v4-flash-0731`,
  `deepseek-ai/deepseek-v4-pro-0813`, `meta/llama-3.2-11b-vision-instruct`,
  `meta/muse-glimmer-30b`, `mistralai/mistral-nemotron`, `moonshotai/kimi-k3`,
  `nvidia/ising-calibration-1.5-31b`, `nvidia/nemotron-3.5-lightning-30b-a3b`,
  `nvidia/nemotron-3-nano-omni-30b-a3b-reasoning`,
  `nvidia/nemotron-3-super-120b-a12b`, `nvidia/nemotron-3-ultra-550b-a55b`,
  `openai/gpt-oss-20b`, `poolside/laguna-xs-2.1`, `z-ai/glm-5.3-flash`.
- Tragend (Spezialzweck, 6): `nvidia/llama-3.1-nemoguard-8b-content-safety`,
  `nvidia/llama-3.1-nemotron-safety-guard-8b-v3`,
  `nvidia/nemotron-3.5-content-safety`, `nvidia/riva-translate-4b-instruct-v1.1`,
  `nvidia/riva-translate-4b-instruct-v2`, `nvidia/nemotron-parse-2.0`.
  Keines der sechs trägt Tool-Ruf (gemessen: `auto`-Tool-Wahl wird abgelehnt).
- **Die Embedding-/Rerank-Linie fehlt vollständig** (`404`): `nv-embedqa-*`,
  `bge-m3`, `arctic-embed-l`, `nemotron-3-embed-1b`, Reranker. Die semantische
  Nähe im Punktfeld ist über NIM damit nicht bedienbar — der Befund, der die
  Frage am stärksten begrenzt.
- opencode: 13 der 14 Chat-Modelle stehen im models.dev-Katalog und sind wählbar;
  `nvidia/ising-calibration-1.5-31b` fehlte und ist in
  `~/.config/opencode/opencode.jsonc` unter `provider.nvidia.models` eingetragen
  (Restart nötig). Der Katalog kennt außerdem `nvidia/ising-calibration-1.5-31b`
  nicht, ebenso nicht `nemoguard-8b-content-safety`, `nemotron-3.5-content-safety`,
  `riva-translate-4b-instruct-v2`, `nemotron-parse-2.0`.

## Offen — Andockung der Spezialzweck-Modelle (Entscheid)

- Frage an den Rat: Sollen `nvidia/nemotron-parse-2.0` (Parse-Stufe des Archivar)
  und/oder `nvidia/riva-translate-4b-instruct-v1.1|-v2` (Übersetzung
  fremdsprachiger Quellen) über den erlaubten curl-Weg angedockt werden? Die
  Stack-Regel (Rust std + curl, Python verboten) hält; die bestehenden Proben
  tragen keinen LLM-Ruf.
- Bedingung: die sechs Spezialzweck-Modelle tragen keinen Tool-Ruf — die Andockung
  ist eine reine Ruf-Stufe (Quelle → Text → Register), kein Agent.
- Ungeprüft: Kontextgrenzen und Kosten der sechs Modelle.

## Abschluss

- Dieses Register steht für die Besprechung im Projekt; kein Commit, kein Push —
  wartet auf das Consent-Wort. Der Baum trägt fremde uncommittete Arbeit
  (bau18-/ernte-folge13-Linie), unberührt.
