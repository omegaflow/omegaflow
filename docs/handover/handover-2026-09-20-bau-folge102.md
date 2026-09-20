<!--
  title: Handover — Bau-Folge 102 (Stand 2026-09-20)
  session: Bau-Folge 102
  class: handover
  date: 2026-09-20
  sha256: 3f31566c55a2d21647c1c71a5dbac53af0c6f2e6990dd8709da7ac6a8757b0aa
  status: live
-->
# Handover — Bau-Folge 102 (2026-09-20)

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

- **HEAD** `aa120ac6` (== origin/main), Arbeitsbaum sauber; `git_safety --snapshot`
  → `refs/safety/1789916309`. Die folge101-Arbeit steht committet/gepusht
  @`4b3b7345` (portabler `daf.rs`-pread, hdf5/snirf/brainvision-Parserzelle,
  clippy-Lints); `docs/zustand/external-state.md` ist von der Ernte-Linie committet
  (`9e6e2a45`/`aa120ac6`) — die zuvor gestagte fremde Arbeit ist weg, die Datei frei.
- **Postfach** — `post.md` trägt nur eine `An entscheid:`-Zeile (Ernte, Lasair-Token),
  kein bau-Eingang; `state/mail/mail_ledger.φ` verschlüsselt, letzter Eintrag
  `1789906306` (externer Zustand zitiert).
- **CI** — `release-build` `35517958024` @`aa120ac6` (Tag `v2026-09-20`) und
  `35517840926` @`4b3b7345` **success** (windows-Matrix grün — der portable
  `daf.rs`-pread hält); `ci-check` `35517957987` in_progress @`aa120ac6` (Tag
  `v2026-09-20`), `ci-check` `35517938423` pending @`aa120ac6` (main);
  `tools-build`/`esp32-firmware`/`paper-check` in_progress bzw. success. Kein
  grüner `ci-check` @`4b3b7345`/`aa120ac6` bisher.

## Offen

- **ci-check Rot-Zelle — Fix @`4b3b7345`, Verdikt ungemessen** — die clippy- und
  test-Zelle (hdf5/snirf/brainvision) ist gefixt und gepusht; der `ci-check`-Lauf
  am Fix-SHA wurde per Concurrency superseded, der Lauf am aktuellen HEAD
  `35517957987` (Tag `v2026-09-20`) läuft, `35517938423` (main) ist pending.
  (Schritt: `ci_manage view 35517938423` bzw. `35517957987` einmalig; bei Rot
  `ci_manage log <id>` + Fix.) · `wartend`

Kein abarbeitbarer undatierter Punkt — der einzige offene Punkt ist die
`ci-check`-Wartestellung (Auslöser: Run-Abschluss). Der session-eigene
abarbeitbare Punkt (CI-Zeile am neuen HEAD) ist umgesetzt.

## Wartestellungen (kein Auswahlpunkt)

- **`te-gate`** `35513982359` in_progress (Forschung-Linie, 3σ-FPR-Fix). (Schritt:
  `ci_manage view 35513982359`.) · `wartend`
- **allwise-cdn** — stündlicher Schedule. · `wartend`

## Benchmark

- **Bau-Folge 102**: Atom „folge101-Abschluss — CI-Zeile am HEAD + release-build-Verdikt".
  Kein Doppellauf — Routine (eine Ledger-Zeile, ein Run-Verdikt), kein harter
  Klasse-Fall; flash-first nicht ausgelöst, keine Delegation nötig.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `docs/zustand/external-state.md` (nur die CI-Status-Zeile),
  `docs/handover/handover-2026-09-20-bau-folge102.md` (neu), Move
  `handover-2026-09-20-bau-folge101.md` → `archiv/`.
- **Fremd (nicht anfassen):** `docs/handover/handover-2026-09-20-entscheid-folge61.md`
  → `archiv/` und `docs/handover/handover-2026-09-20-entscheid-folge62.md`
  (Entscheid-Linie, gestaged); die übrigen Ernte-/Forschung-Pfade sind committet.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Push triggert
`ci-check` (Pfade `src/**`) selbst; `release-build` ist tag/dispatch-only und ist
@`aa120ac6` grün. Die Session pollt nicht; das `ci-check`-Verdikt liest der nächste
Pass aus dem Watchdog-Snapshot. `/consent` ist der session-weite Consent, nie das
Commit-Wort.
