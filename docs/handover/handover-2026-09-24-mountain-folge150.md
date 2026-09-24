<!--
  title: Handover — Mountain-Folge 150 (2026-09-24)
  session: Mountain-Folge 150
  class: handover
  date: 2026-09-24
  sha256: 95cb566d991547040d01ddfe354c278224e139cc6d7d8bb4c951177364aaa2da
  status: live
-->
# Handover — Mountain-Folge 150 (2026-09-24)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits,
der Arbeitsbaum darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt: **Trigger** (das Ereignis/Datum/Wort/der
Lauf, dessen Eintreffen den Punkt kippt — Status = f(Trigger)) / **Lage** (der
Zustand, gemessen, mit Messstempel) / **Blockade** (woran es hängt, oder „keine")
/ **Braucht** (was es löst: der wörtliche, kopierbare Schritt). Sortierung von
Handlungsfähigkeit zu Nicht-Handlungsfähigkeit (autonom → operator-gebunden →
blockiert → wartend → termin → LOCK); Stufen 2–6 werden benannt, nie dispatcht.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung.

## Stehender Pass (gemessen 2026-09-24, Mountain-Folge 150)

- **HEAD** `15ca40c46` (`handover format: carry a Wort line and gate unmeasured
  points before presentation`), `origin/main == HEAD`. Arbeitsbaum: **fremd/
  unberührt** `opencode.json` (`M`, chrome-devtools-MCP `--browserUrl`) und
  `src/archivar/port.rs` (`M`).
- **Postfach:** `state/mail/mail_ledger.φ` vorhanden; keine Mountain-Zeile
  (letzte Eingänge HubSpot/SSDC/ORCID/bounces). `post.md` trägt `An river` +
  `An mycelium` — **keine Mountain-Zeile**. Keine Mail.
- **CI** (`ci_manage list`/`view`/`log`, live 2026-09-24): `register-dropped
  35995278042` @`2099b5691` **success** → `3323 dropped, 2640 commit-resolved`,
  Netto **683**; `ci-check 35989139086` @`6d2aaf5aa` **failure** —
  `dropped-gate: baseline 678 | current 949 | delta 271`; `health-check
  35990890566` **in_progress**; `register-dropped 36000037355` @`15ca40c46`
  in_progress; `ci-check 35998482967` in_progress, `36000037458` pending.
- **`register_lookup --open`:** 116 Docs, 574 offen — **keine
  `[mountain]`-Zustandszeile**.
- **`open_points_check` folge149:** 10 Pfad-Refs, 0 absent, 0 guardians.
- **`git_safety --snapshot`:** `refs/safety/1790253422`.

## Offen (aufgeschlüsselt)

### 1. dropped-gate grün — Baseline auf HEAD-Netto
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** `register-dropped 35995278042` — **eingetroffen** (success
  2026-09-24T12:17:59Z).
- **Lage (gemessen 2026-09-24 via `ci_manage log 35995278042`):** baseline 678;
  Lauf @`2099b5691`: 3323 dropped − 2640 commit-resolved → Netto **683**, delta
  **+5**; `ci-check 35989139086` @`6d2aaf5aa` rot (`baseline 678 | current 949 |
  delta 271`).
- **Blockade:** keine.
- **Braucht:** `docs/zustand/dropped-baseline.md` von 678 auf **683** bumpen +
  Mess-Stempel (`@2099b5691`, run `35995278042`); unaufgelöste Gruppen als
  Handover-Zeile tragen, auf lebende Punkte zeigende routen.

### 2. AFAD-Block-Verifikation
- **Status:** wartend | **Bindung:** eigen (CI)
- **Trigger:** Abschluss `health-check 35990890566`
- **Lage (gemessen 2026-09-24 via `ci_manage list`):** **in_progress**;
  `source-census 35923610972` success.
- **Blockade:** CI-Runner-Queue.
- **Braucht:** `ci_manage view 35990890566`; bei rot `ci_manage log 35990890566
  --all` (AFAD-Fetch + `--verify phi`).

### 3. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Geräteankunft (Tracking `LZ473049629CN`)
- **Lage (gemessen 2026-09-24 via `state/mail/mail_ledger.φ`):** nicht angekommen.
- **Blockade:** physische Ankunft.
- **Braucht:** nach Ankunft Bring-up + Kopplung messen.

## Benchmark

- Kein Doppel: dieser Atom ist der Planungs-Pass (read-only) — keine
  Agenten-Dispatchung, kein Benchmark-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-24-mountain-folge150.md` (neu)
- Move `handover-2026-09-24-mountain-folge149.md` → `archiv/` (eigene Linie, atomar)
- **Fremd/unberührt:** `opencode.json`, `src/archivar/port.rs`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
