<!--
  title: Handover — Entscheid-Folge 66 (Queue 1/2 auf Ende) (Stand 2026-09-20)
  session: Entscheid-Folge 66
  class: handover
  date: 2026-09-20
  sha256: af11789ffe1fac5b6641a7ffd38ec695a87bc4f76df9b3010026bdf3aebddf9a
  status: live
-->
# Handover — Entscheid-Folge 66 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Entscheid-Folge 66)

- **HEAD** bei Session-Beginn `e09a996b`; `origin/main` == HEAD (fremde Linien
  haben zwischenzeitlich gepusht: Ernte `ps1-cdn`, Bau folge113). Der Arbeitsbaum
  trägt **fremde** uncommittete Bau-Arbeit (`src/archivar/port.rs`,
  `src/mathematikerin/force.rs`, `src/gate/commit_gate_vocab.json`,
  `docs/zustand/external-state.md`, `docs/handover/post.md`) — nicht meiner, nicht
  angefasst; mein Commit ist pfad-begrenzt.
- **CI** — Watchdog-Snapshot 19:14; live `ci_manage list`: **`free-model-bench`
  `35527517605` in_progress** (17:57:49Z) → Trigger nicht gefeuert, kein Poll;
  `ci-check` `35526010713` in_progress, `35527911970` pending; rot (attempt 1)
  `35520538766`/`35517957987`.
- **Postfach** — `state/mail/mail_ledger.φ` neuester Eingang `1789922257`
  (Pine64, Developer-Hardware → verweist auf `info@pine64.org`); kein neuer
  Eingang; `post.md` leer.

## Operator-Queue (Stand folge66; einfache Sprache)

1. **PII-History-Rewrite** — **Operator-Wort 2026-09-20: Nein / ans Ende
   geschoben.** (bleibt offen)
2. **Queue-Korpora Re-Lauf** (astro/earth/exotic + 7 parser-gap) — offen.

## Offen

- **Free-Model-Benchmark (echt, CI)** — Artefakt `free-model-bench.tsv` lesen +
  Ranking eintragen. **`wartend`** (Auslöser: Run-Abschluss `35527517605` —
  `ci_manage view 35527517605`).
- **Queue-Korpora Re-Lauf** — **`operator-gebunden`**.
- **nvidia/zai Free-Status**, **Chrome-DevTools-MCP** (forschung), **Benchmark-
  Klassen B–D**, **F2-flare-Gate**, **09-16-Limbo** — **`wartend`** (Auslöser:
  Bedarf).
- `termin` — vC-Permeabilität (Smartwatch + Mantis-Shrimp), Lasair-LSST (API 502),
  BepiColombo MORE (~April 2027). `blockiert` — TAP-Backends dachs/pithia (extern).
  `wartend` — adoption-Block, SuperDARN-Globus, GitHub-PII/GC (#4761801),
  Rubin-Review, Sonden-Antworten, `register_lookup`-Binary + `ci-check` (an bau
  gepostet).

## Benchmark

- **Bau T7 (2026-09-20):** `free_model_bench.rs` um T7 + `T7_EXPECT` erweitert,
  Workflow-Input `T1..T7`; `cargo check` sauber (0 Fehler, 0 Warnungen).
- **Bau agentischer Harness (2026-09-20, grind-max):** `free_model_agent_bench.rs`
  (std-only, `--format json`, Timeout via `try_wait`, tool_calls aus Event-Strom,
  `--emit-config`) + `.github/workflows/free-model-agent-bench.yml` (opencode + 
  `archive_search` via `bin/.tools_ensure`, Config aus `free_models.tsv` +
  `FREE_MODEL_KEYS`); `cargo check` sauber.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-20-entscheid-folge66.md` (neu)
- Move `handover-2026-09-20-entscheid-folge65.md` → `archiv/` (eigene Linie, atomar)
- `tools/measure/src/bin/free_model_bench.rs` (T7)
- `.github/workflows/free-model-bench.yml` (Input `T1..T7`)
- `tools/measure/src/bin/free_model_agent_bench.rs` (neu, agentisch)
- `.github/workflows/free-model-agent-bench.yml` (neu, agentisch)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
