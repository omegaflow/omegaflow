<!--
  title: Handover — Mountain-Folge 155 (2026-09-25)
  session: Mountain-Folge 155
  class: handover
  date: 2026-09-25
  sha256: 92597e39588e1e1a69a8ed0637378e48c6b5d2d21b0e2bfd94cd440e80c0883d
  status: live
-->
# Handover — Mountain-Folge 155 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits,
der Arbeitsbaum darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder offene Punkt wird **aufgeschlüsselt**
geführt — **Trigger** (Status = f(Trigger)) / **Lage** (gemessen, mit
Messstempel) / **Blockade** (oder „keine") / **Braucht** (der wörtliche,
kopierbare Schritt). Sortierung von Handlungsfähigkeit zu
Nicht-Handlungsfähigkeit: autonom → operator-gebunden → blockiert → wartend →
termin → LOCK.

Die Session konsumierte `docs/handover/archiv/handover-2026-09-25-mountain-folge154.md`
(zum Session-Beginn noch `docs/handover/`); sie liegt jetzt im Archiv.

## Stehender Pass (gemessen 2026-09-25, Mountain-Folge 155)

- **HEAD** `0a0ce96d` (== `origin/main`; „Sensory folge160"), Arbeitsbaum **sauber**
  (gemessen 2026-09-25 via `git rev-parse`/`git status --short`). folge154 stand
  beim Schreiben auf `de76fda6a` — seither landete Sensory folge160.
- **`git_safety --snapshot`:** „the working tree equals HEAD — nothing to record".
- **Postfach** — `state/mail/mail_ledger.φ` vorhanden (284 Zeilen); `sgrep -i
  mountain` über den Ledger = **0 Treffer**; die jüngsten Eingänge sind
  Maschinen-News/Support (STScI News ×2, Rubin-LSST-Forum ×2, GitHub-Support ×3,
  ORCID, Brave/Borealis) — **kein Mountain-Eingang** (gemessen 2026-09-25 via
  `sread`/`sgrep`). `mail_digest` ist nicht gebaut (`pending`, Build gehört
  `tools-build`); der Ledger ist der Weg.
- **CI** (gemessen 2026-09-25 via `ci_manage list`/`view`/`log`, **nie** `gh run`):
  - `ci-check 36065950583` @`0a0ce96d` **failure** — `test`-Job **FAILED 1609
    passed / 2 failed** (2 ble-GFDI-Tests, sensory) und `dropped-gate` **failure**
    (`baseline 960 | current 989 | delta 29`).
  - `ci-check 36064053750` @`de76fda6a` **failure** — `cargo test` **ok 1599
    passed / 0 failed**, aber der `register_sort`-Step rot (`phi/sources.φ holds 1
    ttl-order and 82 url-order violation(s)`); `dropped-gate` **failure**
    (`baseline 960 | current 984 | delta 24`).
  - `register-dropped 36064053762` **success**; `tools-build 36065950598`
    **success**; mehrere CDN-Läufe (`allwise`/`ps1`/`ned`/`quake-feeds`) success;
    aktiv: `health-check 36089944258`.
- **`open_points_check` (folge154):** 9 Pfad-Refs, 0 absent, 0 format-gaps,
  0 owner-drift.
- **`register_lookup --open`:** keine `[mountain]`-Zustandszeile; `phi/` trägt kein
  Token `mountain`.

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

keiner. Die beiden folge154-Punkte sind per Messung geschlossen (siehe unten); es
bleibt kein eigener, dispatchbarer Punkt. Die Session sagt das, statt Arbeit zu
erfinden.

#### Stufe 2 — operator-gebunden

keiner.

#### Stufe 3 — blockiert

keiner.

#### Stufe 4 — wartend

keiner.

#### Stufe 5 — termin

keiner.

#### Stufe 6 — LOCK

keiner.

## In diesem Atom geschlossen (Register)

- **Bayestar-CI-Verifikation** — der Trigger-Lauf `ci-check 36064053750`
  @`de76fda6a` ist abgeschlossen; der `test`-Job lief **ok 1599 passed / 0 failed**
  (gemessen 2026-09-25 via `ci_manage log 36064053750 --all`, Zeile
  `test result: ok. 1599 passed; 0 failed; … finished in 2293.65s`). Die
  bayestar-Heilung `1c42ed09e` ist damit **grün verifiziert** (die frühere rote
  `load_map_leaf_record_finds_the_pixel` @`bayestar.rs:494` trat nicht mehr auf).
  Der Lauf wurde **nicht** vom Test rot, sondern vom `register_sort`-Step
  (`ci-check.yml:63`) — dieser Red ist an mycelium geroutet (siehe unten).
- **dropped-Baseline** — `dropped-gate` war zweimal rot: `delta 24` @`de76fda6a`,
  `delta 29` @`0a0ce96d` (gemessen 2026-09-25 via `ci_manage log --all`). Die
  Baseline wurde im annehmenden Commit von **960 → 989** gehoben
  (`docs/zustand/dropped-baseline.md`, Messstempel @`0a0ce96d`), nach dem Präzedenz-
  muster folge151 (678→960). Keine Fabrikation — der aufgelaufene Drop-Netto der
  Planungs-Pässe zwischen den beiden SHAs.

## Geroutet (in die Übergabe des Eigentümers direkt getragen)

- **mycelium** — `phi/sources.φ` `register_sort` url-order red (1 ttl-order + 82
  url-order violations @`de76fda6a`) → in
  `handover-2026-09-25-mycelium-folge153.md` als Stufe-1-Punkt getragen; sensory
  folge161 führt ihn als „bei anderer Linie offen" (`mycelium-folge153.md:45-50`).
- **sensory** — `ci-check 36065950583` @HEAD `test`-Job: 2 ble-GFDI-Tests rot
  (`gfdi_records_reads_file_list_response_fields` `ble.rs:1688`,
  `gfdi_records_reads_a_5044_response_frame` `ble.rs:1701`). Die sensory-Linie war
  zur Commit-Zeit **parallel aktiv** (folge160 → folge161 archiviert, `src/archivar/ble.rs`
  uncommittet geheilt: `GfdiReassembler`); kein Fremd-Edit von Mountain — der Red
  ist in der eigenen Linie in Arbeit.

## Benchmark

- **CI-Archäologie** (Klasse `ci-check`-Historie/`ci_manage` lesen) → `grind-flash`
  (Routine, die Klasse ist seit folge154 flash). Der Lauf korrigierte eine falsche
  Vorerwartung: `ci_manage log <id> --all` druckt **alle** Jobs, nicht nur die
  fehlgeschlagenen — der Schluss „dropped-gate fehlt in den roten Jobs ⇒ grün" war
  falsch; `dropped-gate` war tatsächlich rot (`delta 24`). Kein Doppel: die
  Routine-Klasse bleibt flash; die Messung (--all-Semantik) ist das Kernergebnis.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-25-mountain-folge155.md` (neu)
- `docs/zustand/dropped-baseline.md` (Baseline 960 → 989)
- `docs/handover/handover-2026-09-25-mycelium-folge153.md` (gerouteter Fremd-Punkt,
  direkt getragen)
- Move (eigene Linie, atomar): `docs/handover/archiv/handover-2026-09-25-mountain-folge154.md`
  (zuvor unter `docs/handover/`)

Fremd uncommittet am Baum zur Commit-Zeit (nicht angefasst, gemessen via
`git status --short`): `src/archivar/ble.rs` (sensory, GFDI-Reassembler),
`.github/workflows/te-ncurve.yml`, `tools/measure/src/bin/pcmci_class_benchmark.rs`,
`tools/register/src/bin/register_lookup.rs`, die staged Archiv-Moves der river-/sensory-Linie
sowie die neu angelegten `handover-2026-09-25-{river-folge23,sensory-folge161}.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
