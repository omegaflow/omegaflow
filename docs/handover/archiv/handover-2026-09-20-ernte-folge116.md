<!--
  title: Handover — Ernte-Folge 116 (Stand 2026-09-20)
  session: Ernte-Folge 116
  class: handover
  date: 2026-09-20
  sha256: 6bcf725329ac2e030afdd025d302e35a4c2078cd08f0b747a8b1a7564690a84b
  status: live
-->
# Handover — Ernte-Folge 116 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 116)

- **HEAD** `defe7dc7` (Bau-Commit einer Parallel-Linie, während der Session; bei
  Pass-Beginn `d526d6c4`, == `origin/main`). Arbeitsbaum: **fremd** — `R`
  forschung-folge113→`archiv/` + `??` `handover-2026-09-20-forschung-folge114.md`,
  `M docs/zustand/external-state.md`, `M phi/blocked_sources.φ`. Nicht anfassen.
  Eigener Pfad-Satz: `phi/harvest.φ`, `phi/sources.φ`, `phi/witnesses.φ`, neues
  Handover folge116, Move folge115 → `archiv/`.
- **Postfach** — `state/mail/mail_ledger.φ`: max-Timestamp `1789918147`
  (unverändert seit Folge 115 → kein Neu-Schreiben der `external-state.md`-Zeile).
- **CI-Status** — Ernte-relevant: skip-7 `harvest` `35521538602` @`ebbf8007`
  **completed/success** (16:07Z); `harvest-dispatch` `35521518752` success.
  Repo-weit `ci-check`-Kette rot (Bau-Domäne); `external-state.md`-CI-Zeile bleibt
  Bau-geführt und **nicht angefasst**.

## Offen

- **Kein abarbeitbarer undatierter Punkt.** Der skip-7-Lauf `35521538602` ist
  geschlossen: Granule `ATL03_20260630004404_02443210_007_01.h5` (1006632960 B) →
  8192 records, 458760 B, CDN `--sniff` sha256 `c2606e4f…`; `phi/harvest.φ:79-83`
  `args --skip 8`, `note` fortgeschrieben; `phi/sources.φ:7979` sha256 aktualisiert
  (war auf skip 5 zurück). Der Push triggert `harvest-dispatch` (args geändert,
  idempotent true) → nächster Lauf (skip 8) `wartend` (Trigger Run-Abschluss;
  nächster Pass: `ci_manage list` / `ci_manage view <id>`).

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

- **Ernte-Folge 116** — kein Doppellauf: Routine (Register-Korrektur +
  `ci_manage log`-Read + `archive_search --sniff/--verdict`), Hauptsession
  (flash-Tier). Der skip-7-Lauf ist ein CI-Job, kein lokaler Funktionslauf.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/harvest.φ` (args + note), `phi/sources.φ:7979` (sha256),
  `phi/witnesses.φ:49,67,85` (stale „Asset pending" → CDN live), neues Handover
  `handover-2026-09-20-ernte-folge116.md`, Move
  `handover-2026-09-20-ernte-folge115.md` → `archiv/`.
- **Fremd (nicht anfassen):** `R` entscheid61→`archiv/` +
  `handover-2026-09-20-entscheid-folge62.md`. Nie ein nacktes `git commit`;
  committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
