<!--
  title: Handover — Bau-Folge 103 (Stand 2026-09-20)
  session: Bau-Folge 103
  class: handover
  date: 2026-09-20
  sha256: d1f2a33b4c0872bd1d3010a4e20dad1c165c8e2c96bc4d9ec6bfefbf642e5a5d
  status: live
-->
# Handover — Bau-Folge 103 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder offene
Punkt trägt seinen nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt.

## Stehender Pass (gemessen 2026-09-20, Session-Beginn)

- **HEAD** `5b59c2ab` (== `origin/main`, „handover: complete forschung folge108
  move to archiv", 2026-09-20 17:19 +02:00). Arbeitsbaum: fremde gestagte
  Entscheid-Arbeit (`R`/`A` folge61 → `archiv/`, folge62 neu) — nicht angefasst.
- **Postfach** — `state/mail/mail_ledger.φ` letzter Eingang `1789906306`
  (unverändert); `post.md` trägt zwei `An entscheid:`-Zeilen, kein bau-Eingang.
- **CI** — HEAD `5b59c2ab`: `ci-check` `35519217922` **pending**; `ci-check`
  `35517957987` @`aa120ac6` **in_progress**; die ci-check-Kette davor **cancelled**
  (per-ref-Concurrency-Supersede). `release-build` `35517958024` @`aa120ac6` und
  `35517840926` @`4b3b7345` **success** (windows-Matrix grün, `daf.rs`-pread
  hält); `tools-build`/`paper-check`/`esp32-firmware`/`harvest` success. **Kein
  grüner `ci-check`** @`4b3b7345`/`aa120ac6`/`5b59c2ab`. Die `format`-Zelle ist
  rot gemessen (rustfmt-Diffs aus dem abgeschlossenen `format`-Job `35517957987`
  @`aa120ac6`) und in dieser Session gefixt — 15 Dateien.

## Offen

- **`ci-check`-Verdikt am Fix-HEAD — `wartend`** — der `format`-Fix
  (`src/mathematikerin/te.rs:5394`, `src/archivar/bsp_reader/daf.rs:82`,
  `src/archivar/hdf5.rs:2368/2403` + 11 `tools/utils/src/bin/archive_search`-Dateien:
  Mod-Sort in `archive_search.rs`, clinicaltrials, cochrane, core, ensembl, entrez,
  materialsproject, openfda, pdb, reactome, semanticscholar, unpaywall) ist
  gepusht; der Push triggert `ci-check` neu (`paths: src/**, tools/utils/**,
  docs/**`). Die Diffs stammen aus dem abgeschlossenen `format`-Job `35517957987`
  @`aa120ac6`; HEAD `5b59c2ab` unterscheidet sich nur in `docs/`, die `.rs`-Dateien
  sind identisch — die Diffs sind damit vollständig. (Schritt: den neuen
  `ci-check`-Run einmalig aus dem Watchdog-Snapshot lesen; bei Rot
  `ci_manage log <id>`.) · `wartend`

Kein abarbeitbarer undatierter Punkt — der einzige offene Punkt ist die
`ci-check`-Wartestellung (Auslöser: Run-Abschluss).

## Wartestellungen (kein Auswahlpunkt)

- **`te-gate`** `35513982359` in_progress (Forschung-Linie, 3σ-FPR-Fix). (Schritt:
  `ci_manage view 35513982359`.) · `wartend`
- **allwise-cdn** — stündlicher Schedule. · `wartend`

## Benchmark

- **Bau-Folge 103**: Atom „`ci-check` `format`-Zelle — rustfmt-Diffs anwenden".
  Kein Doppellauf — die Diffs stammen aus dem CI-Log (`ci_manage log 35517957987`,
  abgeschlossener `format`-Job @`aa120ac6`), die Anwendung ist mechanisch;
  flash-first nicht ausgelöst, keine Delegation nötig.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/mathematikerin/te.rs`; `src/archivar/bsp_reader/daf.rs`;
  `src/archivar/hdf5.rs`; `tools/utils/src/bin/archive_search.rs`
  + `tools/utils/src/bin/archive_search/{clinicaltrials,cochrane,core,ensembl,entrez,materialsproject,openfda,pdb,reactome,semanticscholar,unpaywall}.rs`;
  `docs/zustand/external-state.md` (nur die CI-Status-Zeile);
  `docs/handover/handover-2026-09-20-bau-folge103.md` (neu); Move
  `handover-2026-09-20-bau-folge102.md` → `archiv/`.
- **Fremd (nicht anfassen):** `docs/handover/handover-2026-09-20-entscheid-folge61.md`
  → `archiv/` und `docs/handover/handover-2026-09-20-entscheid-folge62.md`
  (Entscheid-Linie, gestaged). Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Push triggert
`ci-check` (`paths: src/**, tools/utils/**, docs/**`) selbst; `release-build` ist
tag/dispatch-only und ist @`aa120ac6` grün. Die Session pollt nicht; das
`ci-check`-Verdikt liest der nächste Pass aus dem Watchdog-Snapshot. `/consent`
ist der session-weite Consent, nie das Commit-Wort.
