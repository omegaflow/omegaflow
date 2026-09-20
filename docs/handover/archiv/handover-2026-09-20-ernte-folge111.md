<!--
  title: Handover — Ernte-Folge 111 (Stand 2026-09-20)
  session: Ernte-Folge 111
  class: handover
  date: 2026-09-20
  sha256: 94bdac3afae97d1afa825b66056b70b80149d2b79ab5700dab5e48b22e3c52e0
  status: live
-->
# Handover — Ernte-Folge 111 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 111)

- **HEAD** `afbf5da1` (== `origin/main`). Arbeitsbaum: **fremd** — `R` Move
  `entscheid61 → archiv/`, `A` `handover-2026-09-20-entscheid-folge62.md`
  (Entscheid-Linie, nicht anfassen). Eigener Pfad-Satz: `phi/harvest.φ`,
  `docs/handover/post.md`, neues Handover folge111, Move
  `handover-2026-09-20-ernte-folge110.md` → `archiv/`. Snapshot
  `refs/safety/1789918504`.
- **Postfach** — `state/mail/mail_ledger.φ` (111 Zeilen); **neuer Eingang**
  `1789918147` (GitHub: Dritt-OAuth-App „ISH Chat", Scopes `read:user`/`user:email`
  am Konto autorisiert) — Sicherheitsereignis, kein Ernte-Punkt; als `An entscheid:`
  in `post.md` gesetzt.
- **CI-Status** — HEAD `afbf5da1`; der Zustand-Eintrag ist von Bau-Folge 102 auf
  `aa120ac6` gemessen → **fällig (HEAD-Wechsel)**, von Bau gehalten, nicht
  angefasst. Repo-weit weiter `ci-check`-Fehlläufe (Bau-Domäne, `afbf5da1`
  fixt die Format-Zelle). Ernte-relevant: **`harvest` `35519201933` success**,
  `harvest-dispatch` `35519201995` success.

## Offen

- **harvest-dispatch.yml Auto-Force-Fix** — `unverified`: `bash -n` grün, aber
  der Workflow hat keinen `workflow_dispatch`-Trigger und wird von `ci-check`
  nicht gelintet. Die `--skip 5`-Fortschreibung in `phi/harvest.φ` (dieser
  Commit) ist die erste Messung: der Push auf `phi/harvest.φ` triggert
  `harvest-dispatch.yml`, `args` geändert → Auto-Force-Pfad
  (`harvest-dispatch.yml:93-104`) → `harvest.yml` mit `force=true`. (Schritt:
  `ci_manage list` auf den auto-dispatched `harvest`-Lauf, dann
  `ci_manage view`/`log`.) `unverified` (Trigger: Push dieses Commits).
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
- **GitHub-OAuth „ISH Chat"** `post.md` — Operator-Wort, ob selbst autorisiert;
  sonst Entzug (Dritt-Akt).

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — nächste Re-Messung
  **2026-10-19**.

## Benchmark

- **Ernte-Folge 111** — kein Doppellauf: die CDN-Messung (`archive_search --sniff`)
  ging an **grind-flash** (Sieger der geschlossenen Routine-Klasse, `AGENTS.md`);
  die Register-/Handover-Arbeit in der Hauptsession. Kein pro/max für Routine.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/harvest.φ`, `docs/handover/post.md`, neues Handover
  `handover-2026-09-20-ernte-folge111.md`, Move
  `handover-2026-09-20-ernte-folge110.md` → `archiv/`.
- **Fremd (nicht anfassen):** `R` Move `entscheid61 → archiv/`,
  `A` `handover-2026-09-20-entscheid-folge62.md`. Nie ein nacktes `git commit`;
  committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
