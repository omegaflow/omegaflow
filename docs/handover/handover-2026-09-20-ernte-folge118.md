<!--
  title: Handover — Ernte-Folge 118 (Stand 2026-09-20)
  session: Ernte-Folge 118
  class: handover
  date: 2026-09-20
  sha256: 82975fc339efd79da4bdfff25b80c99114c5e16b1358f2a9e6bd52368c71e84a
  status: live
-->
# Handover — Ernte-Folge 118 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt. Das Handover wird **vor allem anderen gegen den Baum gehalten**
— das Register ist die Frage, der Baum die Messung.

## Stehender Pass (gemessen 2026-09-20, Folge 118)

- **HEAD** `66579fc3` (bau folge108), == `origin/main`. Arbeitsbaum: fremde
  uncommittete entscheid-Arbeit (`post.md`, `external-state.md`, Move
  `entscheid-folge63` → `archiv/`, `entscheid-folge64.md`) — **nicht angefasst**.
  Eigener Pfad-Satz: `phi/harvest.φ:79,83`, `phi/sources.φ:7980`, neues Handover
  folge118, Move folge117 → `archiv/`.
- **Postfach** — `state/mail/mail_ledger.φ` max-Timestamp `1789922257` (neu seit
  Folge 117: Pine64-Antwort `sales@pine64.org` auf die Hardware-Anfrage — kein
  Ernte-Bezug, Operator/entscheid).
- **CI-Status** — Ernte-relevant: `harvest` `35524334872` @`66579fc3`
  **completed/success** (17:00:59Z) — skip-9-Lauf, geschlossen. Repo-weit
  `ci-check`-Kette rot (Bau-Domäne); `external-state.md`-CI-Zeile bleibt
  Bau-geführt und **nicht angefasst**.

## Offen

- **Kein abarbeitbarer undatierter Punkt.** Der skip-9-Lauf `35524334872` ist
  geschlossen: `phi/harvest.φ:79` `args --skip 10`, `:83` `note` fortgeschrieben
  (Granule `ATL03_20260630005728_02443212_007_01.h5`, 1325400064 B → 8192
  records, 458760 B); `phi/sources.φ:7980` sha256 `79d3ba21…` aktualisiert (war
  skip 8). Der Push triggert `harvest-dispatch` (args geändert, idempotent true)
  → nächster Lauf (skip 10) `wartend` (Trigger Run-Abschluss; nächster Pass:
  `ci_manage list` / `ci_manage view <id>`).

## Wartend (kein Auswahlpunkt)

- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `ledger.φ:10-20` — beide
  DaCHS-Prozesse laufen, PostgreSQL `localhost:5432` tot; Trigger: sync-QUERY
  500→200 bzw. `/tap/tables` 500→200. `blockiert`.
- **Lasair-LSST API** `external-state.md` — Backend 502 über Proton, direct 000;
  Token vorhanden. Trigger: Backend erholt sich. `blockiert`.
- **allwise-cdn** — stündlicher Schedule. `wartend`.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno) `blocked_sources.φ:39-53`. `wartend`.
- **BepiColombo bc_mpo_more** `blocked_sources.φ:26-29` — Freigabe ~April. `wartend`.
- **Babamul / IA2 TAP / GHRC** `blocked_sources.φ` — kein gebauter Konsument → `pending`. `wartend`.

## Termin

- **EMODnet HFRADAR NADR** `ledger.φ:22-24` — nächste Re-Messung **2026-10-19**.

## Ausgelagerte Fremd-Owner-Punkte (nicht im eigenen Handover)

- post_body-Migration → bau (`post.md`).
- queue-Korpora astro/earth/exotic (Operator-Wort/Lauf-Ort) → entscheid (`post.md`).

## Benchmark

- **Ernte-Folge 118** — kein Doppellauf: Routine (CI-Verdikt-Read `ci_manage log`
  + Register-Fortschreibung), Delegation `grind-flash` (flash-Tier) für
  Log-Extraktion + Register. Der skip-9-Lauf ist ein CI-Job, kein lokaler
  Funktionslauf.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/harvest.φ:79,83` (args + note), `phi/sources.φ:7980`
  (sha256), neues Handover `handover-2026-09-20-ernte-folge118.md`, Move
  `handover-2026-09-20-ernte-folge117.md` → `archiv/`.
- **Fremd (nicht anfassen):** `docs/handover/post.md`, `docs/zustand/external-state.md`,
  Move `entscheid-folge63` → `archiv/`, `entscheid-folge64.md` (entscheid-Linie).
  Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
