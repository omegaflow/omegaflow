<!--
  title: Handover — Ernte-Folge 114 (Stand 2026-09-20)
  session: Ernte-Folge 114
  class: handover
  date: 2026-09-20
  sha256: 0fb30211d78ec48b065c0fd9b0a88e018b5a0204d5363f2f5a8ac7b56e142af2
  status: live
-->
# Handover — Ernte-Folge 114 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 114)

- **HEAD** `aa85dea6` (== `origin/main`). Arbeitsbaum: **fremd** — `R`/`A` Move
  `entscheid61 → archiv/` + neues `handover-2026-09-20-entscheid-folge62.md`
  (nicht anfassen). Eigener Pfad-Satz: neues Handover folge114, Move
  `handover-2026-09-20-ernte-folge113.md` → `archiv/`. Snapshot
  `refs/safety/1789919739`.
- **Postfach** — `state/mail/mail_ledger.φ`; **kein neuer Eingang** seit
  `1789918147` (GitHub-OAuth „ISH Chat", `noreply@github.com`). Postfach-Zeile in
  `external-state.md` steht auf Folge 113 (kein neuer Eingang → kein Neu-Schreiben).
- **CI-Status** — HEAD `aa85dea6`. Ernte-relevant: **`harvest` `35521080432`
  in_progress** @`aa85dea6` (der skip-6-force-Lauf, gefeuert von
  `harvest-dispatch` `35521045964` success): **register-Job success**, harvest-Job
  läuft seit 15:55:23Z (`ci_manage log` → 404, kein Log bis Abschluss). Davor
  `harvest` `35520226671` success (skip-5). Repo-weit `ci-check`-Kette
  cancelled/rot (Bau-Domäne); die `external-state.md`-CI-Zeile ist Bau-geführt
  (HEAD `5b59c2ab`) und **nicht angefasst**.

## Offen

- **icesat2_atl03 skip-6** `phi/harvest.φ:76-83` — `args --skip 6` gesetzt; der
  force-Lauf ist **`35521080432` @`aa85dea6`** (register-Job success, harvest-Job
  läuft). (Schritt: nach Run-Abschluss `ci_manage log 35521080432` einmalig →
  Granule/Records/Größe/CDN-sha256 in die `note`, `--skip` auf 7 fortschreiben.)
  `wartend` (Trigger Run-Abschluss).
- **Queue-Korpora post_body-Migration** `phi/pipeline/ledger.φ:94-120` (7 Blöcke,
  `parser-gap`) — der `--port`-Konverter lässt `source`/`method`/`body`/`pos`
  fallen; die Kopf-Migration ist offene Konverter-Arbeit. (Schritt: Konverter um
  die POST-/source-Kopf-Migration erweitern, `src/archivar` `--port`-Pfad.)
  `blockiert` + Owner **bau**.
- **Queue-Korpora ausstehend (astro/earth/exotic)** `phi/pipeline/ledger.φ:82-92`
  — 3 Korpora, `--port` blockiert: kein `force`-Direktiv → `default_kernel_for("")`
  = None. Operator-Wort „abarbeiten" liegt vor; die 7 übrigen Korpora liefen
  bereits (`ledger.φ:94-120`). (Schritt: `force`-Direktiv/Converter-Pfad für die 3
  klären — Owner bau —, dann `--port` + `--probe`; dann Disposition.)
  `operator-gebunden`.

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

- **Ernte-Folge 114** — kein Doppellauf: Routine (`ci_manage`-Read + Handover),
  Hauptsession (flash-Tier). Der skip-6-Lauf ist ein CI-Job, kein lokaler
  Funktionslauf.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** neues Handover `handover-2026-09-20-ernte-folge114.md`, Move
  `handover-2026-09-20-ernte-folge113.md` → `archiv/`.
- **Fremd (nicht anfassen):** `R` Move `entscheid61 → archiv/`,
  `A` `handover-2026-09-20-entscheid-folge62.md`. Nie ein nacktes `git commit`;
  committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
