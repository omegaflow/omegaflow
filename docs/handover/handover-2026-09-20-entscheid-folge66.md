<!--
  title: Handover — Entscheid-Folge 66 (pflichtenfreies Funding gemessen; Queue 1/2 auf Ende) (Stand 2026-09-20)
  session: Entscheid-Folge 66
  class: handover
  date: 2026-09-20
  sha256: f4332b1370a9af78442053142d8468e754222db01bbc3d0db9057423b9ff1cc4
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
2. **Sponsoring-Konten** — **Operator-Wort 2026-09-20: Nein / erst nach dem
   Preprint.** (bleibt offen)
3. **Mantis-Shrimp-Hardware-Bewerbungen** (Espressif / Crowd Supply /
   GSoC-OpenAstronomy-ESA SOCIS) — offen.
4. **Mistral + Inception** — offen.
5. **Queue-Korpora Re-Lauf** (astro/earth/exotic + 7 parser-gap) — offen.
6. **Hardware-Sponsoring (Pine64/Framework/Tuxedo)** — offen.
7. **Komplett pflichtenfreies Funding** — **Operator-Wort 2026-09-20:** bei
   **Eigenprize** zur nächsten Runde bewerben; bei **Akademie Schloss Solitude**
   trotz fehlendem Master bewerben (Call Herbst 2027). Entwürfe in `state/mail/`.

## Offen

- **Komplett pflichtenfreies Funding** — gemessen (research-max, 2026-09-20): **kein
  offener Weg erfüllt alle drei Kriterien dauerhaft** (personenbezogen · pflichtenfrei ·
  Vollunterhalt). Ergebnis in `docs/surveys/survey-funding-pflichtfrei.md` §I.
  **Eigenprize** (`eigen.build`) der einzige weltweit offene, bedingungslose Einzelpreis;
  **Akademie Schloss Solitude** der einzige Kunst-Weg mit Forscher-Zugang.
  **Operator-Wort 2026-09-20:** bei Eigenprize zur nächsten Runde bewerben, bei Solitude
  trotz fehlendem Master (Herbst 2027). Entwürfe: `state/mail/eigenprize-application.md`,
  `state/mail/solitude-application.md`. **`termin`** — Eigenprize-Frist ungemessen
  (monitor `eigen.build`), Solitude Herbst 2027. (Schritt: Formular zur Runde ausfüllen;
  per-Akt-Consent vor Absenden.)
- **Funding-Modell-Benchmark — zwei Achsen** (Operator-Wort 2026-09-20: agentisch).
  - **(A) parametrisch** — Task T7 in `free_model_bench.rs` (Workflow-Input `T1..T7`);
    misst Wissen ohne Tools.
  - **(B) agentisch** — neu `tools/measure/src/bin/free_model_agent_bench.rs` +
    `.github/workflows/free-model-agent-bench.yml`: jedes der 105 Modelle als
    `opencode run --pure --format json --model <provider>/<id> --agent general` mit
    `archive_search` (bash); Score = `T7_EXPECT`; TSV `status, ms, tool_calls`.
  - `cargo check` sauber (beide Bins). **`termin`** (nach `/commit`+Push):
    `gh workflow run free-model-bench.yml -f task=T7` und `gh workflow run
    free-model-agent-bench.yml`. (Schritt: `ci_manage view <id>`; Artefakt lesen.)
  - **`pending`** (agentisch, benannt): npm-Override models.dev-bekannter Provider;
    Event-Serialisierung (eine JSON-Zeile/Event); `archive_search --playwright`-Browser
    im Runner; `FREE_MODEL_KEYS`-Secret (Operator). Erster CI-Lauf misst sie.
- **Free-Model-Benchmark (echt, CI)** — Artefakt `free-model-bench.tsv` lesen +
  Ranking eintragen. **`wartend`** (Auslöser: Run-Abschluss `35527517605` —
  `ci_manage view 35527517605`).
- **Mistral-Disposition**, **Inception-Disposition**, **Queue-Korpora Re-Lauf**,
  **Hardware-Sponsoring** — **`operator-gebunden`** (Queue 3–6).
- **nvidia/zai Free-Status**, **Chrome-DevTools-MCP** (forschung), **Benchmark-
  Klassen B–D**, **F2-flare-Gate**, **09-16-Limbo** — **`wartend`** (Auslöser:
  Bedarf).
- `termin` — vC-Permeabilität (Smartwatch + Mantis-Shrimp), Lasair-LSST (API 502),
  BepiColombo MORE (~April 2027). `blockiert` — TAP-Backends dachs/pithia (extern).
  `wartend` — adoption-Block, SuperDARN-Globus, GitHub-PII/GC (#4761801),
  Rubin-Review, Sonden-Antworten, `register_lookup`-Binary + `ci-check` (an bau
  gepostet).

## Benchmark

- **Doppellauf (flash vs pro/max), Klasse „pflichtenfreies Funding":** `general`
  (flash) und `research-max` (pro/max), identische Aufgabe, 2026-09-20. **Sieger:
  research-max** — präzisere gemessene Pflichtfreiheit (Zitat je Treffer), 19
  Programme, Kernbefund; flash lieferte gute Breite, aber mehr „ungemessen" ohne
  Belegzitate. Die Klasse ist damit geschlossen.
- **Zweiter Lauf derselben Klasse** (Eigenprize + Kunst-Stipendien-Modell),
  research-max, 2026-09-20 — der gemessene Klassensieger, kein Doppellauf nötig.
- **Rat (Architektur, 2026-09-20):** Benchmark-Form = (a) Pflicht-Klassifikation,
  ein Task T7; (b) Coverage und (c) beides verworfen (Coverage zählt Namens-Nennung,
  nicht Pflicht-Wissen).
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
- `docs/surveys/survey-funding-pflichtfrei.md` (§I ergänzt)
- `tools/measure/src/bin/free_model_bench.rs` (T7)
- `.github/workflows/free-model-bench.yml` (Input `T1..T7`)
- `tools/measure/src/bin/free_model_agent_bench.rs` (neu, agentisch)
- `.github/workflows/free-model-agent-bench.yml` (neu, agentisch)
- außerhalb des Repos: `state/mail/eigenprize-application.md`,
  `state/mail/solitude-application.md` (gitignored, kein Commit)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
