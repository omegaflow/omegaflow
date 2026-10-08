<!--
  title: Survey — Research-APIs & MCPs (Consensus · Elicit · SciSpace · Perplexity)
  class: survey
  date: 2026-10-08
  sha256: bfb40b6eada7264454d8f6d40aec4d991d2faf7e525089a9e2888a01deb80f5b
  status: live
  see-also: docs/surveys/survey-2026-10-08-fmhy-research-landscape.md docs/concepts/ui-seats.md docs/concepts/tools-map.md
-->
# Survey — Research-APIs & MCPs

Auftrag (Operator-Wort 2026-10-08): die FMHY-Leads nicht verpuffen lassen, Fokus
**APIs/MCPs** im science/research-Bereich. Alle vier Kandidaten (Consensus · Elicit ·
SciSpace · Perplexity) haben inzwischen einen Account (registriert/eingeloggt).

**Kriterium:** nur `kommerziell` (pay-only) und `illegal` schließen aus; **Auth ist kein
Ausschluss** — die Auth-Route ist der Weg.

## Gemessene Endpunkte (2026-10-08)

| Dienst | API | MCP | Auth | Plan |
|---|---|---|---|---|
| **Consensus** | `GET https://api.consensus.app/v1/search` | `https://mcp.consensus.app/mcp` | `x-api-key` (REST) / Consensus-Konto (MCP) | self-serve Key; Paid-Pläne (Pro/Deep/Teams) inkl. API-Calls; Enterprise |
| **Perplexity** | `https://api.perplexity.ai` (Agent + Search API) | `https://api.perplexity.ai/mcp` (remote, Streamable HTTP) / lokal `@perplexity-ai/mcp-server` | `Authorization: Bearer <PERPLEXITY_API_KEY>` | paid (console.perplexity.ai) |
| **Elicit** | REST-API (docs.elicit.com) | Elicit MCP Server (GA 2026-07-15) | API-Key | Free-Tier + Pro (Pro **kostet** — Operator-Notiz) |
| **SciSpace** | SciSpace API | SciSpace MCP (Claude-Connector) | API-Key | Free + Pro |
| alphaXiv | — | MCP (`--alphaxiv`) | API-Key (MCP) | frei |

HTTP gemessen: `api.consensus.app/v1/search` = **401** (Key nötig, erreichbar),
`mcp.consensus.app/mcp` = **401** (Auth), Consensus-MCP-README = 200, Perplexity-MCP-README = 200.

**Status 2026-10-08:** `CONSENSUS_API_KEY` und `PERPLEXITY_API_KEY` liegen in `.secrets.local`;
**`--consensus` läuft live** (10 Treffer am Baum); `--perplexity`-Arm als Nächstes.
**Elicit `descoped`** (Operator-Wort): API nur auf Pro/Scale/Enterprise = kommerziell.
**SciSpace:** kein self-serve Key, MCP nur gebrokert (Claude/ChatGPT).

## Consensus — der stärkste Arm (220–400M Papers, Volltext)

- `GET https://api.consensus.app/v1/search?query=…&year_min=…&study_types=rct,meta-analysis`
  mit Header `x-api-key: $CONSENSUS_API_KEY`.
- Ergebnis je Paper: `title · authors · publish_year · doi · journal_name · citation_count ·
  study_type · sample_size · sjr_best_quartile · takeaway · abstract · url`; Pagination
  (`page`, `page_size`, `next_page`).
- Filter: `year_min/max`, `study_types`, `human`, `controlled`, `sample_size_min`,
  `exclude_preprints`, `sjr_min/max`, `citation_min`, `medical_mode`, `domain`, `country`,
  `open_access`.
- Opt-in: `include_semantic_score=true`, `include_full_text_chunks=true` (Paid).
- MCP-Server `https://mcp.consensus.app/mcp` (Claude/ChatGPT/Cursor/Gemini Spark …).

## Top-Schritte

1. **`archive_search --consensus <query>`** — REST-Arm (`api.consensus.app/v1/search`,
   `x-api-key` aus `.secrets.local`, Muster `--tavily`/`--exa`); Ausgabe `title/doi/year/
   citations/study_type/takeaway`. **Braucht:** `CONSENSUS_API_KEY` (self-serve im
   Consensus-Konto) im `.secrets.local` (Operator), dann Arm bauen.
2. **`archive_search --perplexity <query>`** — Perplexity Search/Agent API (`PERPLEXITY_API_KEY`).
3. **Elicit API/MCP** und **SciSpace API/MCP** — Endpunkte + Auth aus `docs.elicit.com`
   bzw. SciSpace-Connector-Doku ziehen; Keys (Elicit/SciSpace) anlegen.
4. **MCP-Harness (opencode ist der Harness):** `opencode.json` hat einen `mcp`-Block mit
   `type: "remote"` + `{env:VAR}`-Interpolation. **Verdrahtet 2026-10-08:** `consensus`
   (`https://mcp.consensus.app/mcp`, Header `x-api-key: {env:CONSENSUS_API_KEY}`) und
   `perplexity` (`https://api.perplexity.ai/mcp`, `Authorization: Bearer
   {env:PERPLEXITY_API_KEY}`) — kein Secret im getrackten Config. **Braucht:** die zwei Env-Vars
   im Shell exportieren (`set -a; source .secrets.local; set +a`) und opencode **neu starten**;
   dann haben alle Agenten (auch der Taucher) die MCP-Tools direkt.
5. **SciSpace-MCP — nichts nachzubauen:** der SciSpace-MCP ist ausschließlich über die
   Claude-/ChatGPT-Connector-Verzeichnisse verfügbar (kein öffentlicher MCP-URL); ein
   Nachbau wäre Reverse-Engineering des gebrokeren Connectors (ToS-Risiko). Der REST-Host
   `api.scispace.com` antwortet (202/302) — **SciSpace als `archive_search`-Arm wiren**, sobald
   Key/Docs vorliegen. Kein `SCISPACE_API_KEY` verfügbar.

## Träger

Punkte 1–3 stehen als offene Punkte in `handover-2026-10-07-mycelium-folge267.md`; die
Keys legt der Operator an (per-Akt), die Arme baut die Linie.
