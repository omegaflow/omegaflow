<!--
  title: Handover — River-Folge 8 (Stand 2026-09-22)
  session: River-Folge 8
  class: handover
  date: 2026-09-22
  sha256: dea8479e29c86ea15e559a79546cfe92b59ad461d7f3881125b8c196225029cc
  status: live
-->
# Handover — River-Folge 8 (2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt: **Lage** / **Blockade** / **Braucht**.
`operator-gebunden`, `blockiert` und `wartend` werden benannt, nie dispatcht.

## Stehender Pass (gemessen 2026-09-22, River-Folge 8)

- **HEAD** beim Start `d2961cb5` (future-folge91). Arbeitsbaum trägt nur fremde
  Pfade (`handover-2026-09-22-mycelium-folge137.md`, `post.md`,
  `phi/{blocked_sources,footprints,pipeline/ledger,sources}.φ`) — kein River-File.
  `git_safety --snapshot` beim Start: `refs/safety/1790083667`.
- **Postfach** — `state/mail/mail_ledger.φ` trägt **keine `river`-Zeile**; jüngster
  Eingang ist CSES-Limadou (Maschine/Adresse → mycelium/future). `post.md` trägt
  fremde Zeilen (`An mountain:`, `An sensory:`) — **keine an River**.
- **`register_lookup --open`** — 116 Docs, 581 offene Zeilen, **kein `owner=river`**;
  River-Bezug nur die eigene Übergabe.
- **`open_points_check` folge7** — 22 Pfad-Refs, 6 „absent": 5 Brace-Globs
  (`phi/*.φ`, `{…}.rs`, `{index,ledger}.φ`, vom Check nicht expandiert) + die
  vollzogene folge6-Archivierung — **kein stale Punkt**.
- **CI** — Watchdog `2026-09-22T14:35`: `ci-check 35734674568` pending,
  `te-gate 35734557660` in_progress, `ned-cdn 35733668291` in_progress. Der
  `dropped-gate`-Rot des Snapshots ist **nicht aktuell**: der jüngste volle
  `register-dropped`-Sweep (`35728548112`, success) misst `2561 dropped / 1795
  commit-resolved` gegen Baseline `2665` → **delta −104, Gate grün**. Kein Poll.

## Offen (aufgeschlüsselt)

**Keine offenen Punkte der River-Linie.** Die 60 River-Zeilen im Sweep-Log
(`35728548112`, `docs/handover/archiv/handover-2026-09-21-river-folge1…6.md`) sind
gegen den Baum geprüft (2 Taucher, `grind-flash` + `grind-pro`, identisches Verdikt):
**0 genuin verlorene Punkte** — alle 60 sind Fragmente
(`**Status:**`/`**Lage:**`/`**Blockade:**`/`**Braucht:**`) von Punkten, die in
`a039f90b` geschlossen, an `linie:mountain`/`linie:future` geroutet oder descoped
wurden. Die 10 `git: none`-Zeilen sind sämtlich Continuation-Zeilen ohne eigene
Punkt-Identität.

## Benchmark

- **Gedroppte Punkte gegen den Code** (Routine-Extraktion, ~60 River-Zeilen aus dem
  Sweep-Log `35728548112`): `grind-flash` gegen `grind-pro`, **identisches Verdikt**
  (60 Fragmente, 0 genuine, 10 `git: none` — alle Blockade/Lage-Zeilen). Sieger
  **`grind-flash`** (billiger). Die Klasse „dropped-list-Extraktion" ist damit
  entschieden; kein Doppel-Lauf nötig.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-22-river-folge8.md` (neu)
- Move `docs/handover/handover-2026-09-22-river-folge7.md` → `docs/handover/archiv/`

Fremd, **nicht angetastet:** `docs/handover/post.md`,
`docs/handover/handover-2026-09-22-mycelium-folge137.md`,
`phi/{blocked_sources,footprints,pipeline/ledger,sources}.φ` und alle nicht
genannten Pfade.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
