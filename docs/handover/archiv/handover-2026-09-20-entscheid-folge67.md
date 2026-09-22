<!--
  title: Handover — Entscheid-Folge 67 (Stand 2026-09-20)
  session: Entscheid-Folge 67
  class: handover
  date: 2026-09-20
  sha256: bd1f4dfdf8a3f9dae40923972aa97283edd23853c4ede9f1758ee34aef0099c5
  status: live
-->
# Handover — Entscheid-Folge 67 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Entscheid-Folge 67)

- **HEAD** `fbd0f153` == `origin/main`. Zwischen folge66 und jetzt haben fremde
  Linien committet/gepusht: Forschung-Folge 124 (`register_lookup --dropped` gebaut
  — diffed aufeinanderfolgende Handover je Linie und flaggt offene Punkte, die im
  nächsten fehlen, ohne auflösenden Commit; plus hyperscanning-Strang
  re-registriert), Ernte-Folge 123 (192 Katalog-Kandidaten disponiert; faltete die
  2 ernte-`post.md`-Zeilen). Arbeitsbaum: mein Satz (unten) + **fremd**
  `phi/pipeline/ledger.φ` — nicht angefasst.
- **CI** — Watchdog-Snapshot 21:22; `ci_manage list` ~21:30Z: `te-gate`
  `35531196101` **in_progress** (seit 19:06, >2 h — Watchdog entscheidet);
  `ci-check` `35532930434` pending / `35531572974` in_progress; `free-model-bench`
  `35527517605` in_progress, `35532890353` pending; `free-model-agent-bench`
  `35532892161` in_progress (Job grün, kein Verdikt); `ps1-cdn` `35531164844`
  in_progress. Kein Poll.
- **Postfach** — `mail_ledger.φ` 114 Zeilen, letzter Eingang `1789930255`
  (`info@pine64.org`: Ox64 zugesagt, bittet um Versanddaten/Telefon); Antwort
  gesendet (`sent_ledger` `1789931195`); kein neuer Eingang. `post.md` **leer** —
  ernte faltete ihre 2 Zeilen, ich meine 2 entscheid-Zeilen (Pine64, Cookie).

## Operator-Queue (Stand folge67; einfache Sprache)

1. **PII-History-Rewrite** — **Operator-Wort 2026-09-20: Nein / ans Ende
   geschoben.** (bleibt offen)
2. **Queue-Korpora Re-Lauf** — an ernte gepostet (`post.md`, vor dem Falten).
3. **Cookie-Transfer** — `operator-gebunden`, Auslöser „Bedarf" (Ziel-Site aktiv →
   Cookie-Editor-Export → `state/cookies/<host>.json`).

## Offen

- **P2/P3/P4 Bench-Läufe** — `wartend` (Auslöser: Run-Abschluss `35527517605` /
  `35532890353` / `35532892161`; `ci_manage view` einmalig, Artefakt
  `free-model-bench.tsv` lesen, Ranking eintragen).
- **ernte-Historie-Analyse** — `register_lookup --dropped` ist gebaut (Forschung-
  Folge 124), das Binär auf `tools-latest` aber noch stale → liest `pending`.
  (Schritt: nach dem nächsten tools-build `register_lookup --dropped`.)
- **ISH Chat (GitHub-Dritt-OAuth-App, Scopes `read:user`/`user:email`)** —
  `external-state.md` führt es als `post.md`-Zeile; dort steht sie nicht mehr und
  folge66 nennt sie nicht → **trägerlos**. (Schritt: `github.com/settings/
  connections/applications` prüfen, ggf. widerrufen — `operator-gebunden`.)
- **nvidia/zai Free-Status**, **Chrome-DevTools-MCP**
  (forschung), **Benchmark-Klassen B–D**, **F2-flare-Gate**, **09-16-Limbo** —
  `wartend` (Auslöser: Bedarf).
- `termin` — vC-Permeabilität (Smartwatch + Mantis-Shrimp), Lasair-LSST (API 502),
  BepiColombo MORE (~April 2027), Rubin-Review (Umzug 2026-09-24). `blockiert` —
  TAP-Backends dachs/pithia (extern). `wartend` — adoption-Block, SuperDARN-Globus,
  GitHub-PII/GC (#4761801), Sonden-Antworten, `register_lookup`-Binary + `ci-check`.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-20-entscheid-folge67.md` (neu)
- Move `handover-2026-09-20-entscheid-folge66.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` (2 entscheid-Zeilen gefaltet/gelöscht, sha256 neu)
- `docs/zustand/external-state.md` (CI-Zeile)
- `tools/measure/src/bin/free_model_agent_bench.rs` (Task-Auswahl + Antwort-Sammlung)
- `.github/workflows/free-model-agent-bench.yml` (`task`-Input + Chromium + `.answers`)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`).
