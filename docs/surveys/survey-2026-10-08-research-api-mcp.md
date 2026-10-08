<!--
  title: Survey — Research-APIs & MCPs (Consensus · Elicit · SciSpace · Perplexity)
  class: survey
  date: 2026-10-08
  sha256: 1a0e0b4009c543a63dee3e8063c39c8f1683db81a7a538e67d6095ed0468fdac
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
**`--consensus` läuft live** (10 Treffer am Baum). **`--perplexity` läuft live** (`1569a26d8`):
der 403 war der **umgezogene Endpunkt** — Sonar ist jetzt die **Agent-API** (`POST
api.perplexity.ai/v1/agent`, `preset`+`input` statt `model`+`messages`); Live-Antwort mit
`[web:n]`-Citations. **Elicit `descoped`** (Operator-Wort): API nur Pro/Scale/Enterprise =
kommerziell. **SciSpace:** `api.scispace.com/api/v1/*` → HTTP 403 `{"message":"Missing
cookies"}` — eine **cookie-interne Web-API, keine Key-API**; keine öffentlichen REST-Docs
gefunden; MCP nur gebrokert (Claude/ChatGPT) → **kein Arm** (`pending`/descoped).

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
5. **SciSpace-MCP — nichts nachzubauen (gemessen):** der SciSpace-MCP ist ausschließlich über
   die Claude-/ChatGPT-Connector-Verzeichnisse verfügbar (kein öffentlicher MCP-URL). Die
   Web-API `api.scispace.com/api/v1/*` antwortet **HTTP 403 `{"message":"Missing cookies"}`** —
   sie ist **cookie-/session-gebunden** (die Web-App-Auth), **keine Key-API**; keine
   öffentlichen REST-Docs auffindbar. Ein Nachbau wäre Reverse-Engineering des gebrokerten
   Connectors (ToS-Risiko). → **kein `archive_search`-Arm, kein `SCISPACE_API_KEY`** (`pending`).

## Träger

Punkte 1–3 stehen als offene Punkte in `handover-2026-10-07-mycelium-folge267.md`; die
Keys legt der Operator an (per-Akt), die Arme baut die Linie.
