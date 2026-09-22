<!--
  title: Handover — Entscheid-Folge 63 (Free-Modelle live verifiziert) (Stand 2026-09-20)
  session: Entscheid-Folge 63
  class: handover
  date: 2026-09-20
  sha256: 57fa4a11709aa482d919a7f264e1ffa54935ea4d63720e8bc74d2f7f01389b2a
  status: live
-->
# Handover — Entscheid-Folge 63 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Entscheid-Folge 63)

- **HEAD** `defe7dc7` („port: migrate ra_key/dec_key/plx_key/z_key …", fremde
  Port-Linie, **unpushed**); `origin/main` = `813a82c4`. **Fremd im Baum** (nicht
  angefasst): `D docs/handover/handover-2026-09-20-bau-folge103.md` (staged) + der
  unpushed Port-Commit. `git_safety --snapshot` → `refs/safety/1789920852`.
- **CI** — Watchdog-Snapshot 2026-09-20T18:10: aktiv `tools-build` `35521838130`,
  `ps1-cdn` `35521569959`, `ci-check` `35520538766`, `health-check` `35519449450`;
  rot (attempt 1) die `ci-check`-Kette `35517957987`/`35517136467`/`35516232478`.
- **Postfach** — neuester Ledger-Eingang `1789918147` (GitHub: Dritt-OAuth-App
  „ISH Chat", `read:user`/`user:email` — Sicherheitsereignis, keine Handlung); davor
  `1789906306` (Sotgiu-Limadou-Weiterleitung). `external-state.md:20` trägt den Wert.

## Free-Modelle live verifiziert (2026-09-20) — erledigt in diesem Atom

Gemessen an der Quelle (`<provider>/models` mit Bearer-Key aus `auth.json`,
Pricing-Seiten via `archive_search --playwright`); Roh-JSON unter
`/tmp/opencode/freemodels/`. Der per-Token-freie Katalog je Provider:

| Provider | #Modelle | per-Token frei | Anmerkung |
|---|---|---|---|
| openrouter | 446 | 24 | 21×`:free` + `openrouter/free` + 2× lyria (Musik) |
| kilo | 380 | 23 | wie openrouter, `kilo-auto/free`, + `stepfun/step-3.7-flash:free` |
| unorouter | 260 | 137 | `:free`-Suffix; großteils Bild-/Embedding-/Audio-Modelle |
| kenari | 80 | 15 | `pricing.free:true`, alle `tool_call:true` |
| zai | 11 | 3 | `glm-4.7-flash`/`glm-4.5-flash`/`glm-4.6v-flash` (nur Docs-Pricing, nicht im `/models`) |
| tokenrouter | 140 | 1 | `nvidia/nemotron-3-nano-omni-…:free` |
| opencode (Zen) | 77 | 7 | `big-pickle` + `mimo-v2.5-free`/`ling-3.0-flash-fin-free`/`nemotron-3-ultra-free`/`nemotron-3.5-lightning-free`/`muse-spark-1.3-contributor-free`/`jev-1.13-free` (Doku `opencode.ai/docs/zen`); `deepseek-v4-flash-free` nur im Katalog, nicht in der Free-Liste |
| mistral | 46 | **0** | kein `free`/`pricing`; gratis nur der Vibe-Plan (Riss) |
| inception | 2 | **0** | `mercury-2` 2,5e-7 / `mercury-2.5` 4e-8 (bezahlt; Free-Tier 100 Mio Token = separate Linie) |
| nvidia | 82 | 38 „Free Endpoint" | Free-Marker nur auf build.nvidia.com (`?filters=availability:free_endpoint` = 38); 24 API-IDs zugeordnet, 14 Nicht-LLM (`pending`); Whitelist auf 15 frei-LLMs erweitert |

**Config bereinigt** (`~/.config/opencode/opencode.jsonc`, untracked): kenari +3
(`agnes-2-0-flash:free`, `agnes-2-5-flash:free`, `laguna-xs-2-1:free`); opencode
(Zen) +`nemotron-3.5-lightning-free`/`jev-1.13-free`, −`muse-spark-1.2-contributor-free`
(nicht in der Doku-Free-Liste); die per-Token-freien, aber **nicht tool-fähigen**
`glm-5.2` und `nvidia/nemotron-3.5-content-safety` aus openrouter/kilo/unorouter
entfernt. Die übrigen Whitelists waren korrekt. Verifiziert am Roh-JSON
(mistral/inception/kenari) + Zen-Doku (`opencode.ai/docs/zen`).

**Free-Tier-Provider (gemessen, Kontingent statt per-Token-0):** `google` 50
Modelle, 21 mit Free Tier (gemini-3.8/3.7/3.6/3.5-flash, 3.5/3.1-flash-lite,
3-flash-preview, 2.5-pro/flash/flash-lite, gemma-4 u. a.; 12 ohne — Omni,
3.1-pro, Nano-Banana, Veo, Lyria); die Free-Ratelimits stehen nicht in
`g_limits.txt` → `pending`. `groq` 13 Modelle, 10 mit Free-Plan-Zeile
(gpt-oss-120b/20b/safeguard-20b, qwen3.8-27b, whisper-large-v3/-turbo, 2×
orpheus, 2× llama-prompt-guard); `allam-2-7b`, `groq/compound`,
`groq/compound-mini` ohne Free-Zeile. `cloudflare-workers-ai` 10.000
Neurons/Tag gratis, 7 Modelle nur bezahlt (kimi-k2.6, kimi-k2.7-code, glm-5.2,
glm-5.3, glm-5.3-flash, deepseek-v4-flash-0731, deepseek-v4-pro-0813).
**Whitelists gesetzt:** google 17 (Free-Tier-Modelle), groq 10 (Free-Plan),
cloudflare-workers-ai 20 (alle außer die 7 bezahlten), nvidia 15 (frei-LLMs).
**`models`-Block ergänzt** für opencode-unbekannte IDs (`jev-1.13-free`, kenari
`agnes-2-0/2-5-flash:free`/`laguna-xs-2-1:free`, tokenrouter
`nemotron-3-nano-omni-…:free`, google `3.5-transcribe`/`-live`/`robotics-er-2`) —
sonst erscheinen sie nicht. **`mistral` deaktiviert** (`disabled_providers`).

## Offen

- **Mistral-Disposition** — gemessen kein Free-API-Modell; die 7 Whitelist-Einträge
  sind als „frei" falsch. **`operator-gebunden`** (Operator-Queue 4).
- **nvidia/zai Free-Status** — nvidia „Free Endpoint" = Preview-Credits (Filter
  clientseitig); zai free nur über die Docs-Pricing-Seite. **`wartend`** (Auslöser:
  Bedarf).
- **Chrome-DevTools-MCP** — CDP-Debug der Membran; Operator-Wort ausstehend.
  **`operator-gebunden`** (Operator-Queue 5).
- **Queue-Korpora astro/earth/exotic** (30/3/16 Blöcke) — `--port` blockiert: kein
  `force`-Direktiv, Korpora gitignored, kein CI-`--port`-Workflow.
  **`operator-gebunden`** (Operator-Queue 6).
- **API-Keys CORE / S2 / Materials Project** — bewilligt, nicht in `.secrets.local`,
  ungemessen. **`operator-gebunden`** (Operator-Queue 7).
- **Benchmark-Klassen B–D** (Quellen-Recherche, Register-Digest, Force-Gate-Diagnose)
  nicht gelaufen. **`wartend`** (Auslöser: Bedarf).
- **Free-Model-Benchmark auf UnoRouter/Kenari ausweiten** (je 10 Free-Modelle).
  **`wartend`** (Auslöser: Bedarf).
- **F2 — flare-Gate-Power** — print-only Probe n∈{400,600,1000}. An forschung
  gepostet. **`wartend`**.
- **09-16-Limbo** — forschung-Pass. **`wartend`**.
- `termin` — **vC-Permeabilität** (Smartwatch + Mantis-Shrimp), **Lasair-LSST**
  (API 502, Backend server-seitig), **BepiColombo MORE** (~April 2027).
- `blockiert` — TAP-Backends dachs/pithia (extern). `wartend` — adoption-Block,
  SuperDARN-Globus, GitHub-PII/GC (#4761801), Rubin-Review, Sonden-Antworten,
  `register_lookup`-Binary + `ci-check` (an bau gepostet).

## Operator-Queue (einmal vorlegen beim Operator-Rückkehr; einfache Sprache)

1. **PII-History-Rewrite** — *Lage:* in 1.472 alten Commits steht eine private
   Adresse; die aktuelle Version ist sauber, die Geschichte nicht. *Frage:* Soll ich
   die Geschichte umschreiben? *Ja:* der Auftrag
   `docs/auftrag/auftrag-pii-history-rewrite.md` läuft (löscht alte Commits, danach
   Force-Push). *Nein:* bleibt. (seit 2026-09-15)
3. **Mantis-Shrimp-Hardware** — *Lage:* Wege Espressif / Crowd Supply /
   Akt. *Nein:* bleibt.
5. **Chrome-DevTools-Debugger** — *Lage:* für die Fehlersuche im Browser (Membran)
   fehlt ein Debugger-Zugang. *Frage:* `opencode.json` `mcp.chrome-devtools` pinnen
   (npm `chrome-devtools-mcp@1.9.0`, `--no-usage-statistics --no-performance-crux`)?
   *Ja:* einbinden. *Nein:* bleibt ohne Debugger. (Forschung-109)
6. **Queue-Korpora (astro/earth/exotic)** — *Lage:* 49 Datenblöcke warten; der Lauf
   ist blockiert, weil kein Kraft-Direktiv vorliegt und die Daten nur lokal (nicht in
   CI) liegen. *Frage:* Lauf lokal freigeben oder einen CI-`--port`-Workflow bauen?
   *Ja:* freigeben, dann `--port` + `--probe`. *Nein:* bleibt blockiert. (Ernte-115)
7. **API-Keys CORE / Semantic Scholar / Materials Project** — *Lage:* alle drei
   angekommen, noch nicht in `.secrets.local`, ungeprüft. *Frage:* ablegen und testen?
   *Ja:* ablegen + messen. *Nein:* bleibt. (Forschung-113)

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-20-entscheid-folge63.md` (neu)
- Move `handover-2026-09-20-entscheid-folge62.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` (3 entscheid-Zeilen gefaltet + gelöscht)
- `docs/zustand/external-state.md` (Free-Model-Katalog-Zeile, eigener Hunk)

## Benchmark

- Free-Model-Katalog-Messung: `general` (flash) gegen kein Doppel — flash war
  vollständig und am Roh-JSON verifiziert (mistral 0 free, inception bezahlt,
  kenari 15). Sieger flash; kein pro/max-Doppellauf (Antwort korrekt).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
