<!--
  title: Handover — Ernte-Folge 109 (Stand 2026-09-20)
  session: Ernte-Folge 109
  class: handover
  date: 2026-09-20
  sha256: 6ee6c1888a0873028b55fb43e2a61578b5fdd944cc90f5d0c14151a561e00098
  status: live
-->
# Handover — Ernte-Folge 109 (2026-09-20)

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

## Stehender Pass (gemessen 2026-09-20, Folge 109)

- **HEAD** `aa120ac6` (== `origin/main`). Arbeitsbaum: **fremd** — `MM`
  `docs/zustand/external-state.md` (Bau-Folge 102 + Entscheid-Folge 61/62),
  `A` `docs/handover/handover-2026-09-20-bau-folge102.md` +
  `handover-2026-09-20-entscheid-folge62.md`, `R` Moves bau101/entscheid61
  (nicht anfassen). Eigener Pfad-Satz: `phi/harvest.φ`, neues Handover folge109,
  Move `handover-2026-09-20-ernte-folge108.md` → `archiv/`. Planungs-Snapshot:
  tree == HEAD (nichts zu sichern).
- **Postfach** — `state/mail/mail_ledger.φ` bis Dateiende gelesen (243 Zeilen);
  jüngster Eingang unverändert `1789906306` (CSES-Limadou WG, an entscheid),
  **kein neuer Eingang**. Der `external-state.md`-Eintrag ist von der
  Entscheid-Linie gehalten → nicht angefasst.
- **CI-Status** — HEAD `aa120ac6`; der Zustand-Eintrag ist von Bau-Folge 102 auf
  denselben HEAD gemessen (`ci_manage list`). Ernte-relevant: **`harvest`
  `35516529023` @`37369e95` success** (icesat2 `--skip 3`) — der Trigger des
  offenen Punkts ist gefeuert.

## Offen

- **icesat2_atl03** `phi/harvest.φ:76-83` — Run `35516529023` (`--skip 3`)
  **success** gemessen (`ci_manage view` + `log`): Granule
  `ATL03_20260630001527_02443206_007_01.h5` → 8192 records, 458760 B; CDN
  `--sniff` sha256 `91ac2126…`. `args` auf `--skip 4` fortgeschrieben. **Befund:**
  `harvest-dispatch.yml:77-79` dispatcht nur bei `asset fehlt` — nach dem ersten
  Lauf steht `asset present`, also bleibt der `--skip`-Fortschritt ohne `force`
  stehen. (Schritt: nach dem Push
  `gh workflow run harvest.yml -f format=icesat2_atl03 -f force=true`; dann bei
  Abschluss `ci_manage view <run-id>`, note fortschreiben, `--skip 5`.)
  `wartend` (Trigger: dieser Commit gepusht).
- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-20`
  — Re-Messung 2026-09-20 (research-max): beide DaCHS-Prozesse laufen, PostgreSQL
  `localhost:5432` tot (Restarts 2026-09-17/18), alle DB-Pfade 500 bzw. VOTable
  ERROR; VOSI `availability` fälschlich „up"; IVOA-Registry kennt keinen Spiegel.
  (Schritt: GET `…/tap/sync?…SELECT TOP 1 * FROM ivoa.ObsCore` 500→200 bzw.
  `pithia…/tap/tables` 500→200; Operator-Kontakt pithia `tomasik@cbk.waw.pl`
  wäre Dritt-Akt → Consent.) `blockiert`.
- **Lasair-LSST API** `external-state.md:23` / `phi/blocked_sources.φ:8-11` —
  Backend 502 auf allen Pfaden; `LASAIR_LSST_TOKEN` lokal nicht vorhanden → Key-
  Gap, nicht Routen-Gap. Post an `entscheid` steht in `docs/handover/post.md`.
  (Schritt: Operator-Wort, dann Token-Messung.) `blockiert` + `operator-gebunden`.

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

- **Ernte-Folge 109** — kein Doppellauf: die Run-Log-Extraktion (`ci_manage view`/
  `log` + Compiler-Lesen) lief flash-frei in der Hauptsession; kein pro/max
  dispatcht (keine Routen-Recherche nötig).

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/harvest.φ`, neues Handover
  `handover-2026-09-20-ernte-folge109.md`, Move
  `handover-2026-09-20-ernte-folge108.md` → `archiv/`.
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md` (`MM`),
  `handover-2026-09-20-bau-folge102.md`, `handover-2026-09-20-entscheid-folge62.md`
  (`A`), Moves bau101/entscheid61 (`R`). Nie ein nacktes `git commit`; committet
  wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
