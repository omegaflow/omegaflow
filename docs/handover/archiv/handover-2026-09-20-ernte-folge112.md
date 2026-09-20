<!--
  title: Handover — Ernte-Folge 112 (Stand 2026-09-20)
  session: Ernte-Folge 112
  class: handover
  date: 2026-09-20
  sha256: 1f87a215bd5bcd8d1919247cbb17fc1906c1158f800516b949d19c505e556277
  status: live
-->
# Handover — Ernte-Folge 112 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 112)

- **HEAD** `539c0f7d` (== `origin/main`, folge111-Commit). Arbeitsbaum: **fremd**
  — `R` Move `entscheid61 → archiv/`, `A` `handover-2026-09-20-entscheid-folge62.md`,
  ` M` `phi/sources.φ` (Entscheid-Linie, nicht anfassen). Eigener Pfad-Satz:
  `docs/zustand/external-state.md` (Postfach-Zeile), neues Handover folge112,
  Move `handover-2026-09-20-ernte-folge111.md` → `archiv/`. Snapshot
  `refs/safety/1789918701`.
- **Postfach** — `state/mail/mail_ledger.φ` (244 Zeilen); **kein neuer Eingang**
  seit `1789918147` (GitHub-OAuth „ISH Chat", in folge111 als `An entscheid:`
  in `post.md` gesetzt). Postfach-Zeile in `external-state.md` auf `1789918147`
  fortgeschrieben.
- **CI-Status** — HEAD `539c0f7d`. Ernte-relevant: **`harvest-dispatch`
  `35520190417` success** (head `539c0f7d`) und **`harvest` `35520226671`
  in_progress** (der auto-dispatched skip-5-Lauf). Repo-weit weiter
  `ci-check`-Fehlläufe (Bau-Domäne). Der `external-state.md`-CI-Eintrag ist von
  Bau auf `5b59c2ab` gemessen und damit fällig — von Bau gehalten, nicht angefasst.

## Offen

- **harvest `35520226671` (icesat2_atl03 skip-5, auto-force)** `phi/harvest.φ:76-83`
  — `in_progress` (erzeugt 2026-09-20T15:38:38Z). Der Auto-Force-Pfad ist
  **verifiziert**: `harvest-dispatch` `35520190417` protokolliert
  `format icesat2_atl03: asset present, args changed — dispatch harvest.yml force=true`
  und `https://…/runs/35520226671`; `format lro_trk: asset present — no dispatch`
  (kein args-Wechsel — korrekt). (Schritt: `ci_manage view 35520226671`, bei
  `success` `ci_manage log` → Granule/Records/Größe/CDN-sha256 in die `note`,
  `--skip` auf 6 fortschreiben; bei `failure` Ursache im Log messen.) `wartend`.
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-20`
  — Re-Messung 2026-09-20 (research-max): beide DaCHS-Prozesse laufen, PostgreSQL
  `localhost:5432` tot; alle DB-Pfade 500 bzw. VOTable ERROR; VOSI `availability`
  fälschlich „up"; kein Spiegel. (Schritt: GET
  `…/tap/sync?…SELECT TOP 1 * FROM ivoa.ObsCore` 500→200 bzw.
  `pithia…/tap/tables` 500→200; pithia-Kontakt `tomasik@cbk.waw.pl` wäre
  Dritt-Akt → Consent.) `blockiert`.
- **Lasair-LSST API** `external-state.md` / `phi/blocked_sources.φ:8-11` —
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
- **GitHub-OAuth „ISH Chat"** `post.md` — Operator-Wort, ob selbst autorisiert;
  sonst Entzug (Dritt-Akt).

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — nächste Re-Messung
  **2026-10-19**.

## Benchmark

- **Ernte-Folge 112** — kein Doppellauf: der Auto-Force-Nachweis lief in der
  Hauptsession (`ci_manage log 35520190417`, flash-Tier); die Lauf-Extraktion
  ging an **grind-flash** (Sieger der geschlossenen Routine-Klasse, `AGENTS.md`).
  Kein pro/max für Routine.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `docs/zustand/external-state.md` (Postfach-Zeile), neues
  Handover `handover-2026-09-20-ernte-folge112.md`, Move
  `handover-2026-09-20-ernte-folge111.md` → `archiv/`.
- **Fremd (nicht anfassen):** `R` Move `entscheid61 → archiv/`,
  `A` `handover-2026-09-20-entscheid-folge62.md`, ` M` `phi/sources.φ`. Nie ein
  nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
