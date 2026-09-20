<!--
  title: Handover — Ernte-Folge 115 (Stand 2026-09-20)
  session: Ernte-Folge 115
  class: handover
  date: 2026-09-20
  sha256: 3627e533bc5c38055a7920c1ab2c21377d8e32344133b3b099aac0497833dc8e
  status: live
-->
# Handover — Ernte-Folge 115 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 115)

- **HEAD** `8df40994` (== `origin/main`). Arbeitsbaum: **fremd** — `M`
  `docs/concepts/tools-map.md`; `R`/`A` Move `entscheid61 → archiv/` + neues
  `handover-2026-09-20-entscheid-folge62.md`; `M/??` `tools/utils/src/bin/archive_search*`
  (forschung/bau). Nicht anfassen. Eigener Pfad-Satz: `phi/harvest.φ`, `post.md`,
  neues Handover folge115, Move folge114 → `archiv/`. Snapshot `refs/safety/1789919959`.
- **Postfach** — `state/mail/mail_ledger.φ`: kein neuer Eingang seit `1789918147`
  (gemessen: max-Timestamp == `1789918147`). Postfach-Zeile in `external-state.md`
  bleibt Folge 113 (kein neuer Eingang → kein Neu-Schreiben).
- **CI-Status** — HEAD `8df40994`. Ernte-relevant: `harvest` `35521080432`
  @`aa85dea6` **completed/success** (skip 6): register-Job success, harvest-Job
  success. Repo-weit `ci-check`-Kette cancelled/rot (Bau-Domäne); die
  `external-state.md`-CI-Zeile ist Bau-geführt (HEAD `5b59c2ab`) und **nicht angefasst**.

## Offen

- **Kein abarbeitbarer undatierter Punkt.** Der skip-6-Lauf `35521080432` ist
  geschlossen: Granule `ATL03_20260630003802_02443209_007_01.h5` (771751936 B) →
  8192 records, 458760 B, CDN `--sniff` sha256 `4e1f8aa3…`; `phi/harvest.φ:76-83`
  `args --skip 6 → 7`, `note` fortgeschrieben. Der Push triggert
  `harvest-dispatch` (args geändert, idempotent true) → nächster Lauf (skip 7)
  `wartend` (Trigger Run-Abschluss; nächster Pass: `ci_manage list`).
- **Zwei Fremd-Owner-Punkte ausgelagert** (`post.md`): post_body-Migration → bau;
  queue-Korpora astro/earth/exotic (Operator-Wort/Lauf-Ort) → entscheid. Nicht
  mehr im eigenen Handover.

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

## Benchmark

- **Ernte-Folge 115** — kein Doppellauf: Routine (`ci_manage log`-Read +
  Register-Edit), Hauptsession (flash-Tier). Der skip-6-Lauf ist ein CI-Job, kein
  lokaler Funktionslauf.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/harvest.φ` (args + note), `docs/handover/post.md` (zwei
  eigene Zeilen), neues Handover `handover-2026-09-20-ernte-folge115.md`, Move
  `handover-2026-09-20-ernte-folge114.md` → `archiv/`.
- **Fremd (nicht anfassen):** `M docs/concepts/tools-map.md`, `R`+`A`
  entscheid61→`archiv/` + `handover-2026-09-20-entscheid-folge62.md`,
  `M/??` `tools/utils/src/bin/archive_search*`. Nie ein nacktes `git commit`;
  committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
