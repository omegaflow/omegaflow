<!--
  title: Survey — FMHY/Awesome-Mining: Research-Landschaft (Seats + Suchwege)
  class: survey
  date: 2026-10-08
  sha256: f23ea3992a67bab98a926a032a8c3c1d2ecfe1b3736f1725dff98625f2f20093
  status: live
  see-also: docs/concepts/ui-seats.md AGENTS.md docs/SOURCE_PORT.md
-->
# Survey — FMHY/Awesome-Mining: Research-Landschaft (Seats + Suchwege)

Auftrag (Operator-Wort 2026-10-08): die zwei kuratierten Listen (FMHY `fmhy.net/ai`,
`felladrin/awesome-ai-web-search`) **nicht verpuffen lassen** — im science-/research-
Bereich liegt großes Potential. Diese Survey misst die Kandidaten und benennt je
Kandidat den konkreten nächsten Schritt.

## Quellen (gemessen 2026-10-08)

- FMHY AI-Rohliste `https://raw.githubusercontent.com/fmhy/edit/main/docs/ai.md`
  (462 Z.; gerendert `https://fmhy.net/ai`, HTTP 200, 176673 B).
- `https://raw.githubusercontent.com/felladrin/awesome-ai-web-search/main/README.md`
  (172 Z.) — FMHY listet sie als **ersten Eintrag** von `#specialized-chatbots`.
- Reachability je Kandidat: `curl -sS -o /dev/null -w '%{http_code}' --max-time 18 <url>`.

## Befund

Die Listen liefern **Research-Chatbots/Seats** und **Agent-Such-APIs** — aber **kein
neuer Science-Daten-Arm**: die awesome-Liste erklärt akademische Paper-APIs explizit
als out-of-scope (`README.md:147`), FMHY liefert nur Chatbot-UIs. Unser `archive_search`
deckt die Wissenschaftslandschaft (arxiv · ads · crossref · openalex · semanticscholar ·
core · pubmed · europepmc · doaj · zenodo · datacite · nasa-PDB · heasarc · supermag …)
bereits breiter ab als beide Listen zusammen.

## Gemessene Kandidaten (Auszug; HTTP + Wert)

| Name | URL | Klasse | HTTP | Login? | Wert |
|---|---|---|---|---|---|
| Jina Reader | s.jina.ai / r.jina.ai | b (Reader-API) | 200 | **keyless** | **hoch** |
| SearXNG | searxng.org | b (Metasuche) | 200 | keyless/self-host | **hoch** |
| AI2 Playground (Olmo) | playground.allenai.org | a (Seat) | 200 | **nein** | **hoch** |
| NVIDIA NIM | build.nvidia.com/models | a/b | 200 | **nein** | **hoch** |
| Elicit | elicit.com | a/c (Lit-Review) | 200 | Konto | hoch |
| Consensus | consensus.app | a/c (Paper-Engine) | 200 | Konto | mittel |
| alphaXiv | alphaxiv.org | a | 200 | API-Key (MCP) | bereits `--alphaxiv` |
| Sci-Bot (Sci-Hub) | sci-bot.ru | a | 200 | nein | mittel — **Grauzone (UrhG), Rat** |
| Bohrium | bohrium.com | a | 200 | Sign-Up | niedrig |
| Firecrawl | firecrawl.dev | b | 200 | Key | mittel (überlappt `--playwright`) |
| Perplexity | perplexity.ai | b (Answer-Engine + Deep Research) | 403 Cloudflare (Bridge: Challenge) | Google-Login optional | **Kandidat — Auth-Route** |
| SciSpace | scispace.com | a/c (Paper-Chat) | 202 mit Browser-UA | Konto | **Kandidat — Auth-Route** |
| Sakana Chat | chat.sakana.ai | a (Research-Lab-Chat) | 200 mit Browser-UA | Konto | **Kandidat — Auth-Route** |
| NotebookLM | notebooklm.google.com | a | 301 | Google | mittel (Auth-Route) |
| LongCat 2.0 | longcat.ai/chat | a (Seat) | 200 | Sign-Up | mittel |
| Tencent Hy3 | aistudio.tencent.com | a (Seat) | 200 | Sign-Up | mittel |
| MiMo Studio | aistudio.xiaomimimo.com | a (Seat) | **000** | — | pending (nicht erreichbar) |
| Chat Motif | chat.motiftech.io/chat | a (Seat) | 200 | nein | mittel |

**Ausschluss-Kriterium (Operator-Wort 2026-10-08): nur `kommerziell` (pay-only, keine
Free-/Auth-Route) und `illegal` schließen aus. Auth ist KEIN Ausschluss** — Login,
API-Key und Cloudflare-Challenge laufen über die **Auth-Route** (Operator-Account bzw.
Browser-Bridge/Operator-Profil); ein 403-Bot-Block ist ein Reachability-Zustand, kein
Verdikt. Gemessen 2026-10-08: Perplexity (Cloudflare-Challenge, Bridge), SciSpace
(202 mit Browser-UA), Sakana (200 mit Browser-UA) — alle **Auth-Kandidaten**, nicht „raus".

**Geringer Wert / redundant (kein Ausschluss, nur kein erster Schritt):** die keyed
General-Such-APIs (Desearch · Querit · Context.dev · Tako · Olostep · JigsawStack —
`--tavily/--exa/--linkup` vorhanden), AI-Leaderboards (LMArena · Vals · SWE-bench ·
MathArena · Artificial Analysis), `paper2gal`, `hyperspace`.

## Top-Schritte (nicht verpuffen — je Kandidat ein konkreter Schritt)

1. **`archive_search --jina <url>`** (Jina Reader, keyless) — **GEBAUT** (`0e644437d`, `tools/utils/src/bin/archive_search/jina.rs` + Wiring). Gemessen end-to-end: `./target/debug/archive_search --jina https://example.com` → HTTP 200, liefert „Title / URL Source / Markdown Content" (1465 B). Saubere Text-Extraktion jenseits von `--playwright`/`--sniff`. Nächster Schritt: `tools-latest`-Release bestätigen (Lauf), dann nur noch nutzen.
2. **`archive_search --searxng <query>`** (keyless Metasuche): gemessen — `searx.be`
   `?format=json` liefert **HTML** (JSON-Format nicht freigeschaltet); braucht eine Instanz
   mit `format: json` (self-host oder freigegebene Instanz). **Arm bauen + Instanz festlegen**.
3. **AI2 Playground als UI-Seat** (kein Sign-Up, Olmo — neue Zuchtlinie): Gretchenfrage
   (Fähigkeit 4/4 + Tempo), dann in `docs/concepts/ui-seats.md` + Roster.
4. **Elicit + Consensus als Research-Seats/Quellen**: Literatur-Review-Antworten gegen
   `--openalex`/`--semanticscholar` gegenprüfen; prüfen, ob Consensus einen API-Endpoint hat.
5. **Sci-Bot (Sci-Hub)**: Grauzone (UrhG/DMCA) — **Rat-Verdikt vor jeder Berührung**, nie
   blind wiren.
6. **NVIDIA NIM** (keyless Modelle/Endpoint): als Seat **und** mögliche API-Quelle messen.
7. **Perplexity · Sakana — 4/4-Seats (Gretchenfrage gemessen 2026-10-08, Profil-Browser):**
   Perplexity `PHIL: absent,null-echt,pending,nein,b,fabrication` (4/4, „Recherchiert 3 Sek.");
   Sakana Chat 4/4 (Tempo `pending`, kein Turn-Stempel). Beide in `ui-seats.md` aufgenommen.
   **SciSpace** = Login-Wall (Sign-up-Dialog vor dem Chat → `pending`), **Gemini Notebook** =
   Dokument-Bot ohne freien Composer → beide Auth-Route-/Dokument-Kandidaten, kein Ausschluss.

## Träger

Diese Survey trägt die obigen Schritte; Punkte 1–2 (die zwei Arme) und 3 (Seat) stehen als
offene Punkte in `handover-2026-10-07-mycelium-folge267.md`. Kein „Regal": der nächste
Schritt ist je Zeile benannt.
