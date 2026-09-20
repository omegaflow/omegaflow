<!--
  title: Handover — Ernte-Folge 110 (Stand 2026-09-20)
  session: Ernte-Folge 110
  class: handover
  date: 2026-09-20
  sha256: 799c611dced54f82b5b27d3341e42bacf7f5cbfd4331b536cdb5a7683a358d02
  status: live
-->
# Handover — Ernte-Folge 110 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 110)

- **HEAD** `5b59c2ab` (== `origin/main`). Arbeitsbaum: **fremd** — `R` Move
  `entscheid61 → archiv/`, `A` `handover-2026-09-20-entscheid-folge62.md`
  (Entscheid-Linie, nicht anfassen). Eigener Pfad-Satz:
  `.github/workflows/harvest-dispatch.yml`, neues Handover folge110, Move
  `handover-2026-09-20-ernte-folge109.md` → `archiv/`. Snapshot
  `refs/safety/1789917646`.
- **Postfach** — `state/mail/mail_ledger.φ` bis Dateiende gelesen (243 Zeilen);
  jüngster Eingang unverändert `1789906306` (CSES-Limadou-Weiterleitung),
  **kein neuer Eingang**. `post.md`: zwei `An entscheid:`-Zeilen
  (Chrome-Debugger-MCP, Lasair-Token) — stehen, nicht ernte-eigen.
- **CI-Status** — HEAD `5b59c2ab`; der Zustand-Eintrag ist von Bau-Folge 102 auf
  `aa120ac6` gemessen → **fällig (HEAD-Wechsel)**, aber von Bau gehalten, nicht
  angefasst. Repo-weiter **CI-Backlog** (gemessen `ci_manage list`): viele Läufe
  seit ~10:34Z in `in_progress`/queued (u. a. `ci-check` `35517957987`,
  `35517136467`, `health-check` `35505471538`), kein Runner-Fortschritt.
  Ernte-relevant: **`harvest` `35519201933` FORMAT=icesat2_atl03 in_progress**
  (Register-Job grün, Harvest-Job ohne Log, `updated_at 15:20:02` → queued).

## Offen

- **icesat2_atl03** `phi/harvest.φ:75-83` — Force-Lauf `35519201933`
  (FORMAT=icesat2_atl03, `--skip 4`) dispatched, **queued** (CI-Backlog;
  `updated_at 15:20:02`, Vorlauf dauerte 4 min). Der `--skip`-Fortschritt hängt
  an diesem Lauf. (Schritt: bei Abschluss `ci_manage view 35519201933` + `log`,
  Granule/records/Bytes/CDN-sha256 in die note, `args --skip 4 → 5`.)
  `wartend` (Trigger: Run-Abschluss).
- **harvest-dispatch.yml Auto-Force-Fix** — `unverified`: `bash -n` grün, aber
  der Workflow hat keinen `workflow_dispatch`-Trigger und wird von `ci-check`
  nicht gelintet. (Schritt: der nächste `phi/harvest.φ`-Push — die
  `--skip 5`-Fortschreibung — ist die erste Messung; dann `ci_manage list` auf
  den auto-dispatched `harvest`-Lauf mit `force=true`.) `unverified`.
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-20`
  — Re-Messung 2026-09-20 (research-max): beide DaCHS-Prozesse laufen, PostgreSQL
  `localhost:5432` tot; alle DB-Pfade 500 bzw. VOTable ERROR; VOSI `availability`
  fälschlich „up"; kein Spiegel. (Schritt: GET
  `…/tap/sync?…SELECT TOP 1 * FROM ivoa.ObsCore` 500→200 bzw.
  `pithia…/tap/tables` 500→200; pithia-Kontakt `tomasik@cbk.waw.pl` wäre
  Dritt-Akt → Consent.) `blockiert`.
- **Lasair-LSST API** `external-state.md:23` / `phi/blocked_sources.φ:8-11` —
  Backend 502 auf allen Pfaden; `LASAIR_LSST_TOKEN` lokal nicht vorhanden →
  Key-Gap. Post an `entscheid` steht in `docs/handover/post.md`. (Schritt:
  Operator-Wort, dann Token-Messung.) `blockiert` + `operator-gebunden`.

## Wartend (kein Auswahlpunkt)

- **allwise-cdn** — stündlicher Schedule. (Schritt: `ci_manage view <allwise-run>`.) `wartend`.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno) `phi/blocked_sources.φ:39-53`
  — Anfragen offen. `wartend`.
- **BepiColombo bc_mpo_more** `phi/blocked_sources.φ:26-29` — Freigabe ~April. `wartend`.
- **Babamul / IA2 TAP / GHRC** `phi/blocked_sources.φ` — kein gebauter Konsument → `pending`. `wartend`.

## Operator-gebunden

- **Queue-Korpora** `phi/pipeline/ledger.φ:114-120` — `--port` braucht das
  Operator-Wort (gehört zur entscheid-Queue).

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — nächste Re-Messung
  **2026-10-19**.

## Benchmark

- **Ernte-Folge 110** — kein Doppellauf: die Run-Messung (`ci_manage`) lief in der
  Hauptsession; die `harvest-dispatch.yml`-Änderung ging an **grind-flash**
  (Sieger der geschlossenen Routine-Klasse, `AGENTS.md`), Council nur für das
  Design (Architektur-Entscheid). Kein pro/max für Routine.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `.github/workflows/harvest-dispatch.yml`, neues Handover
  `handover-2026-09-20-ernte-folge110.md`, Move
  `handover-2026-09-20-ernte-folge109.md` → `archiv/`.
- **Fremd (nicht anfassen):** `R` Move `entscheid61 → archiv/`,
  `A` `handover-2026-09-20-entscheid-folge62.md`. Nie ein nacktes `git commit`;
  committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
