<!--
  title: Handover — Entscheid-Folge 64 (Free-Modelle: Non-Tool ausgeblendet, unorouter-Probe rate-limitiert) (Stand 2026-09-20)
  session: Entscheid-Folge 64
  class: handover
  date: 2026-09-20
  sha256: 5f3ef4288e5243f2ed86e012a3071772931e94524e456a96ec372f648d93fe40
  status: live
-->
# Handover — Entscheid-Folge 64 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen nächsten
Schritt in derselben Zeile. Wartestellungen (`wartend`) sind kein Auswahlpunkt,
sondern nennen nur ihren Auslöser. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-20, Entscheid-Folge 64)

- **HEAD** `66579fc3` („bau folge108: disposition six stale parser-gap ledger
  entries …") **== `origin/main`**, Arbeitsbaum sauber; `git_safety --snapshot` →
  `refs/safety/1789924200`.
- **CI** — Watchdog-Snapshot 2026-09-20T18:10: aktiv `tools-build` `35521838130`,
  `ps1-cdn` `35521569959`, `ci-check` `35520538766`, `health-check` `35519449450`;
  rot (attempt 1) die `ci-check`-Kette `35517957987`/`35517136467`/`35516232478`.
- **Postfach** — neuester Ledger-Eingang `1789918147` (GitHub-Dritt-App „ISH Chat",
  `read:user`/`user:email` — Sicherheitsereignis, keine Handlung); kein neuer
  Eingang. `post.md` trug eine entscheid-Zeile (bau: Queue-Korpora Re-Lauf) → in
  dieses Handover gefaltet, Zeile gelöscht.

## Free-Modelle — Non-Tool ausgeblendet, unorouter-Probe rate-limitiert (2026-09-20)

Gemessen an `~/.cache/opencode/models.json` (models.dev `tool_call`-Metadaten) und
live gegen `https://api.unorouter.com/v1`.

- **20 Non-Tool-Modelle aus den Whitelists entfernt** (ausgeblendet): google 5
  (`gemini-3.1-flash-tts-preview`, `gemini-embedding-2`,
  `gemini-3.5-transcribe`/`-live`, `gemini-robotics-er-2-preview`), groq 6
  (`whisper-large-v3`/`-turbo`, `canopylabs/orpheus-*` ×2,
  `meta-llama/llama-prompt-guard-2-*` ×2), cloudflare 9
  (`gemma-sea-lion-v4-27b-it`, `deepseek-r1-distill-qwen-32b`,
  `llama-3.1-8b-instruct-fp8`, `llama-3.2-11b-vision-instruct`, `llama-3.2-1b`/`-3b`,
  `llama-guard-3-8b`, `qwen2.5-coder-32b-instruct`, `qwq-32b`). Whitelists jetzt:
  unorouter 16, google 12, groq 4, cloudflare 11; übrige unverändert; mistral
  deaktiviert.
- **unorouter +3**: `glm-5.2:free` (models.dev `tool_call:true`) + 2 live gemessen
  TOOL (`agnes-2.0-flash:free`, `codestral-latest:free`); Whitelist 13 → 16. **5
  registry-gelistete free-Modelle nicht aktiviert**: `deepseek-v4-pro:free`,
  `qwen3.5-397b-a17b:free`, `minimax-m2.7:free`, `gpt-5.4:free`, `gpt-5.5:free` →
  HTTP 503 „All providers for model … are busy" = kein lebender Provider (Katalog ≠
  Verfügbarkeit).
- **Live-Probe-Befund**: `allam-2-7b:free` → HTTP 400 „tool calling is not
  supported" = NO-TOOL; `agnes-2.0-flash:free`/`codestral-latest:free` = TOOL;
  `absolutereality:free` → 000 (Bild). **Rate-Limit: 1 Request/Minute pro Konto,
  global** (429, „nothing is used up", Retry-Fenster 28–56 s; die nie angefragten
  `deepseek-v4-flash:free`/`gemma-4-31b-it:free` ebenfalls 429 → global, nicht pro
  Modell); einige Modelle 5/min; Key ist Trial (1000 Calls gesamt). Die 21
  unorouter-Free-Modelle sind damit für Agenten-Arbeit praktisch unbrauchbar
  (1 Tool-Call/Minute + teils 503).
- **unorouter deaktiviert** (`disabled_providers: ["mistral","unorouter"]`,
  Operator-Wort 2026-09-20) — aus dem Picker entfernt.

## Offen

- **Free-Model-Benchmark (echt, CI)** — **Harness gebaut** (2026-09-20):
  `tools/measure/src/bin/free_model_bench.rs` + `tools/measure/free_models.tsv`
  (105 Modelle: google 12, groq 4, cloudflare 11, openrouter 19, kilo 18, kenari 15,
  zai 3, tokenrouter 1, opencode 7, nvidia 15) + `.github/workflows/free-model-bench.yml`
  (`workflow_dispatch`). Task-Set T1–T6 (N=10/5/3/5/3/3): Tool-Call, Multi-Step,
  Code-Bugfix, Structured Output, Reasoning, Long-Context; je Trial HTTP-Status +
  Latenz, 429-Backoff aus „retry in Ns", TSV mit per-Modell-Summary (tool_ok%,
  Task-Pass%, p50/p95, 429/5xx/timeout/pending). **`cargo check` 0 Fehler / 0
  Warnungen.** Offen: (Schritt 1) die 10 Provider-Keys als GitHub-Secrets setzen
  (`gh secret set <ENV> --repo <owner/omegaflow>`); (Schritt 2)
  `gh workflow run free-model-bench.yml`; (Schritt 3) Artefakt `free-model-bench.tsv`
  lesen und das Ranking in dieses Handover. **`operator-gebunden`** (Secrets +
  Dispatch-Wort).
- **Mistral-Disposition** — gemessen kein Free-API-Modell; die 7 Whitelist-Einträge
  als „frei" falsch; Provider deaktiviert. **`operator-gebunden`** (Operator-Queue).
- **Inception-Disposition** — `mercury-2` per-Token bezahlt (2,5e-7); Free-Tier
  „100 Mio Token" separate Linie (Riss). **`operator-gebunden`** (Operator-Queue).
- **Queue-Korpora Re-Lauf** (bau folge108, aus `post.md` gefaltet) — die 7
  `parser-gap`-Korpora (`13k`/`14k`/`15k`/`183l`/`2k`/`7k`/`candidate-staging`) nach
  dem `port.rs`-Fix brauchen den lokalen Release-Binär-Lauf (`--port` + `--probe`);
  ebenso `astro` 30 + `earth` 3 + `exotic` 16 (kein `force`-Direktiv). Lauf-Ort
  ungeklärt (Korpora gitignored, kein CI-`--port`-Workflow, lokaler Funktionslauf
  verweigert). **`operator-gebunden`** (Operator-Queue).
- **nvidia/zai Free-Status** — nvidia „Free Endpoint" = Preview-Credits (Filter
  clientseitig); zai free nur über die Docs-Pricing-Seite. **`wartend`** (Auslöser:
  Bedarf).
- **Chrome-DevTools-MCP** — in forschung-116 gepinnt (`@1.9.0`); Verifikation offen
  (forschung-Punkt). **`wartend`**.
- **Benchmark-Klassen B–D**, **Free-Model-Benchmark UnoRouter/Kenari**, **F2
  flare-Gate** (print-only n∈{400,600,1000}, an forschung), **09-16-Limbo** —
  **`wartend`** (Auslöser: Bedarf).
- `termin` — vC-Permeabilität (Smartwatch + Mantis-Shrimp), Lasair-LSST (API 502,
  Backend server-seitig), BepiColombo MORE (~April 2027).
- `blockiert` — TAP-Backends dachs/pithia (extern). `wartend` — adoption-Block,
  SuperDARN-Globus, GitHub-PII/GC (#4761801), Rubin-Review, Sonden-Antworten,
  `register_lookup`-Binary + `ci-check` (an bau gepostet).

## Operator-Queue (einmal vorlegen beim Operator-Rückkehr; einfache Sprache)

1. **PII-History-Rewrite** — *Lage:* in 1.472 alten Commits steht eine private
   Adresse; die aktuelle Version ist sauber, die Geschichte nicht. *Frage:* Geschichte
   umschreiben? *Ja:* Auftrag `docs/auftrag/auftrag-pii-history-rewrite.md` läuft
   (löscht alte Commits, Force-Push). *Nein:* bleibt. (seit 2026-09-15)
2. **Förderung/Sponsoring** — *Lage:* Förderung wirkt erst mit Publikum. *Frage:*
   Sponsoring-Konten (GitHub Sponsors / Open Collective) jetzt aufsetzen? *Ja:*
   anlegen. *Nein:* erst nach Preprint.
3. **Mantis-Shrimp-Hardware** — *Lage:* Wege Espressif / Crowd Supply /
   GSoC-OpenAstronomy-ESA SOCIS. *Frage:* Bewerbungen/Formulare senden? *Ja:* pro
   Akt. *Nein:* bleibt.
4. **Mistral + Inception** — *Lage:* Mistral hat kein kostenloses API-Modell (nur
   der Vibe-Plan ist gratis); Inception rechnet pro Token ab (2,5e-7), Free-Tier
   „100 Mio Token" laut früherem Stand. *Frage:* Mistral aus der Liste nehmen (oder
   kostenpflichtig aktivieren)? Inception als Gratis-Kontingent führen oder
   entfernen? *Ja/Nein:* entsprechend.
5. **Queue-Korpora (astro/earth/exotic + 7 parser-gap)** — *Lage:* 49 + 7
   Datenblöcke warten; der Lauf ist blockiert (kein Kraft-Direktiv, Korpora nur
   lokal). *Frage:* Lauf lokal freigeben oder CI-`--port`-Workflow bauen? *Ja:*
   freigeben, dann `--port` + `--probe`. *Nein:* bleibt blockiert. (Ernte-115,
   Bau-108)

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-20-entscheid-folge64.md` (neu)
- Move `handover-2026-09-20-entscheid-folge63.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` (1 entscheid-Zeile gefaltet + gelöscht)
- `docs/zustand/external-state.md` (Free-Model-Katalog-Zeile, eigener Hunk)
- `tools/measure/src/bin/free_model_bench.rs` (neu, Benchmark-Harness)
- `tools/measure/free_models.tsv` (neu, 105 Free-Modelle)
- `.github/workflows/free-model-bench.yml` (neu, CI)
- untracked: `~/.config/opencode/opencode.jsonc` (Whitelists, nicht im Repo)

## Benchmark

- Kein Doppellauf in diesem Atom — die unorouter-Probe war mechanisch (kein
  Agentenvergleich), die Registry-Auswertung deterministisch.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
