<!--
  title: Handover — Entscheid-Folge 65 (opencode.db: 111 Fremd-Sessions gelöscht; Operator-Queue vorgelegt) (Stand 2026-09-20)
  session: Entscheid-Folge 65
  class: handover
  date: 2026-09-20
  sha256: 215d1715d6ff6e85df53c204ac583297b0f36a8afcb8b127e008c04d96aefbe3
  status: live
-->
# Handover — Entscheid-Folge 65 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Entscheid-Folge 65)

- **HEAD** `c2f197b2` („entscheid folge64: register free-model-bench secrets +
  dispatch") **== `origin/main`**, Arbeitsbaum sauber.
- **CI** — Watchdog-Snapshot 19:14: aktiv `ci-check` `35523145456`,
  `health-check` `35519449450`; rot (attempt 1) `35520538766`/`35517957987`.
  Live `ci_manage list`: **`free-model-bench` `35527517605` in_progress**
  (17:57:49Z) → der wartende Benchmark-Trigger ist noch nicht gefeuert; kein Poll.
- **Postfach** — `state/mail/mail_ledger.φ` neuester Eingang `1789922257`
  (`sales@pine64.org`, Developer-Hardware-Antwort) = die `post.md`-Zeile; kein
  gefaltet, Zeile gelöscht.
- **opencode.db bereinigt (Operator-Wort)** — `~/.local/share/opencode/opencode.db`
  trug **112 Sessions** (massiv „nur OK"-Test-Sessions); aktive Session
  `ses_f40058366ffeww4UguT1n83B6i` (`line`, `deepseek-v4-flash`) identifiziert, alle
  111 anderen aus `session` + zugehörige `event_sequence`/`event` gelöscht (Cascade),
  `VACUUM` + `wal_checkpoint(TRUNCATE)`. Datei **173 318 144 B → 1 433 600 B**;
  verbleibend: 1 Session, 26 message, 106 part, 5 todo, 358 event. Sicherung:
  `/tmp/opencode/opencode.db.bak-1789927394`. Entscheidung (Operator): die
  Burn-Messreihe wird verworfen — der Ethik-Einwand war benannt.

## Operator-Queue (einmal vorgelegt 2026-09-20; einfache Sprache)

1. **PII-History-Rewrite** — *Lage:* in 1.472 alten Commits steht eine private
   Adresse; die aktuelle Version ist sauber, die Geschichte nicht. *Frage:* Geschichte
   umschreiben? *Ja:* Auftrag `docs/auftrag/auftrag-pii-history-rewrite.md` läuft
   (löscht alte Commits, Force-Push). *Nein:* bleibt. (seit 2026-09-15)
3. **Mantis-Shrimp-Hardware** — *Lage:* Wege Espressif / Crowd Supply /
   Akt. *Nein:* bleibt.
5. **Queue-Korpora (astro/earth/exotic + 7 parser-gap)** — *Lage:* 49 + 7
   Datenblöcke warten; der Lauf ist blockiert (kein Kraft-Direktiv, Korpora nur
   lokal). *Frage:* Lauf lokal freigeben oder CI-`--port`-Workflow bauen? *Ja:*
   freigeben, dann `--port` + `--probe`. *Nein:* bleibt blockiert. (Ernte-115,
   Bau-108)

## Offen

- **Free-Model-Benchmark (echt, CI)** — Harness + Workflow stehen; Lauf
  `35527517605` `in_progress`. Offen: Artefakt `free-model-bench.tsv` lesen + das
  Ranking eintragen. **`wartend`** (Auslöser: Run-Abschluss — `ci_manage view
  35527517605` / Watchdog).
- **Mistral-Disposition** — kein Free-API-Modell; Provider deaktiviert.
  **`operator-gebunden`** (Queue 4).
- **Queue-Korpora Re-Lauf** — 7 `parser-gap`-Korpora + `astro`/`earth`/`exotic`
  brauchen den Release-Binär-Lauf (`--port` + `--probe`); Lauf-Ort ungeklärt.
  **`operator-gebunden`** (Queue 5).
- **nvidia/zai Free-Status** — nvidia Preview-Credits, zai free nur Docs-Pricing.
  **`wartend`** (Auslöser: Bedarf).
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

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-20-entscheid-folge65.md` (neu)
- Move `handover-2026-09-20-entscheid-folge64.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` (1 entscheid-Zeile gefaltet + gelöscht)
- außerhalb des Repos: `~/.local/share/opencode/opencode.db` bereinigt (kein Commit)

## Benchmark

- Kein Agenten-Doppellauf in diesem Atom — die DB-Bereinigung war mechanisch
  (deterministische SQL-Löschung), die Session-Identifikation eindeutig
  (jüngstes `time_updated`).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
