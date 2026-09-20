<!--
  title: Handover — Ernte-Folge 113 (Stand 2026-09-20)
  session: Ernte-Folge 113
  class: handover
  date: 2026-09-20
  sha256: 3b87fd145b998fd32cf7692a461f95edf0e79fd541938d8001eb055ba20b619d
  status: live
-->
# Handover — Ernte-Folge 113 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 113)

- **HEAD** `bfa09ee0` (== `origin/main`, folge112-Commit). Arbeitsbaum: **fremd**
  — `R` Move `entscheid61 → archiv/`, `A` `handover-2026-09-20-entscheid-folge62.md`,
  ` M` `survey-2026-09-20-browser-anbindung.md` (nicht anfassen). Eigener
  Pfad-Satz: `phi/harvest.φ`, `phi/pipeline/ledger.φ`, `phi/sources.φ`,
  `phi/blocked_sources.φ`, `docs/handover/post.md`,
  `docs/zustand/external-state.md`, neues Handover
  folge113, Move `handover-2026-09-20-ernte-folge112.md` → `archiv/`.
  Snapshot `refs/safety/1789918995`.
- **Postfach** — `state/mail/mail_ledger.φ`; **kein neuer Eingang** seit
  `1789918147` (GitHub-OAuth „ISH Chat", letzte Zeile `noreply@github.com`).
  Postfach-Zeile in `external-state.md` auf Folge 113 fortgeschrieben.
- **CI-Status** — HEAD `bfa09ee0`. Ernte-relevant: **`harvest` `35520226671`
  success** (icesat2_atl03 skip-5, head `539c0f7d`) und **`harvest-dispatch`
  `35520190417` success**; **kein neuer harvest-Lauf @`bfa09ee0`** (args
  unverändert). Repo-weit `ci-check`-Kette cancelled/rot (Bau-Domäne); die
  `external-state.md`-CI-Zeile ist Bau-geführt (HEAD `5b59c2ab`) und **nicht
  angefasst**.

## Operator-Entscheide (2026-09-20, Operator am Terminal; entscheid beschäftigt)

- **ISH Chat (GitHub-OAuth)** — vom Operator **selbst autorisiert** → kein
  Widerruf. Post-Zeile `post.md` gelöscht. `blocked_sources`-Bezug entfällt.
- **Lasair-LSST** — `LASAIR_LSST_TOKEN` **vorhanden** (`.secrets.local`,
  41 Zeichen; gemessen) → **kein Key-Gap**. Backend `api.lasair.lsst.ac.uk/api/`
  502 über Proton-Exit, direct 000 → Token-Query unmessbar, solange der Upstream
  tot ist. Post-Zeile gelöscht; `external-state.md`-Zeile aktualisiert.
- **Queue-Korpora** — Operator-Wort „abarbeiten". `--port` + `--probe` auf allen
  7 Korpora gelaufen (Release-Binär `target/release/omegaflow`, lokal): 3641
  Blöcke konvertiert, **39 parsen**, **0 Survivor**; 3602 fallen im Konverter
  (alte Grammatik `source/method/body/pos`). `ledger.φ` auf `parser-gap`
  fortgeschrieben (Owner bau, post_body-Migration).

## Offen

Kein eigener abarbeitbarer undatierter Punkt — die offenen Punkte sind `wartend`
(Trigger Push), `blockiert` (Upstream) oder Owner **bau**.

- **icesat2_atl03 skip-6** `phi/harvest.φ:76-83` — `args --skip 6` gesetzt;
  `harvest-dispatch` feuert den force-Lauf auf diesem Commit. (Schritt:
  `ci_manage list` → Lauf-id, dann `ci_manage log <id>`; Granule/Records/Größe/
  CDN-sha256 in die `note`, `--skip` auf 7 fortschreiben.) `wartend`.
- **Queue-Korpora post_body-Migration** `phi/pipeline/ledger.φ` (7 Blöcke,
  `parser-gap`) — der `--port`-Konverter lässt `source`/`method`/`body`/`pos`
  fallen; die Kopf-Migration ist offene Konverter-Arbeit. (Schritt: Konverter um
  die POST-/source-Kopf-Migration erweitern, `src/archivar` `--port`-Pfad.)
  `blockiert` + Owner **bau**.

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

- **Ernte-Folge 113** — kein Doppellauf: die `--port`/`--probe`-Disposition lief
  in der Hauptsession (Release-Binär, flash-Tier); Routine, kein pro/max. Die
  Konverter-Migration bleibt ein bau-Atom.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/harvest.φ`, `phi/pipeline/ledger.φ`, `phi/sources.φ`,
  `phi/blocked_sources.φ`, `docs/handover/post.md`,
  `docs/zustand/external-state.md`, neues Handover
  `handover-2026-09-20-ernte-folge113.md`, Move
  `handover-2026-09-20-ernte-folge112.md` → `archiv/`.
- **Fremd (nicht anfassen):** `R` Move `entscheid61 → archiv/`,
  `A` `handover-2026-09-20-entscheid-folge62.md`, ` M`
  `survey-2026-09-20-browser-anbindung.md`. Nie ein nacktes `git commit`;
  committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
