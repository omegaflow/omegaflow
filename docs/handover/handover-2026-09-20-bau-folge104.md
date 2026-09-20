<!--
  title: Handover — Bau-Folge 104 (Stand 2026-09-20)
  session: Bau-Folge 104
  class: handover
  date: 2026-09-20
  sha256: 1512fad5071f8a8af23c53e37a44346acc781de82c250af847c30e07cfc19b97
  status: live
-->
# Handover — Bau-Folge 104 (2026-09-20)

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

- **HEAD** `d526d6c4` (== `origin/main`, „forschung: classify catalog fields,
  correct field-token doc, add --cod port"). Arbeitsbaum: fremde gestagte
  Entscheid-Arbeit (`R`/`A` folge61 → `archiv/`, folge62 neu) — nicht angefasst.
  Snapshot `refs/safety/1789920401`.
- **Postfach** — `post.md` bau-Zeile (post_body-Migration) abgeholt und gelöscht;
  `state/mail/mail_ledger.φ` letzter Eingang `1789918147` (zitiert, kein neuer
  Eingang). Zwei `An entscheid:`-Zeilen bleiben (fremd).
- **CI** — HEAD `d526d6c4`: `ci-check` `35521529582` **pending**; kein grüner
  `ci-check` am HEAD (`docs/zustand/external-state.md:22`, Bau-geführt, zitiert).
  `release-build`/`tools-build`/`harvest` grün. Der `failed`-Run `35516232478`
  liegt auf Branch `tools-latest` (`e3c478af`), nicht am Bau-HEAD.

## Offen

- **Queue-Korpora Re-Lauf nach Konverter-Fix — `operator-gebunden`** — der
  `--port`-Fix (ra/dec/plx/z-Migration in `src/archivar/port.rs`) ist gepusht;
  die 7 `parser-gap`-Korpora (`phi/pipeline/ledger.φ:94-120`) brauchen den
  `--port`-Re-Lauf, um den Survivor-Stand zu messen. Lokaler Funktionslauf ist
  der Session verweigert, kein CI-`--port`-Workflow (Korpora gitignored).
  (Schritt: Operator-Wort für den lokalen Release-Binär-Lauf auf den 7 Korpora,
  dann `--port` + `--probe`, dann die 7 Ledger-Notes auf das Ergebnis
  fortschreiben.) · `operator-gebunden`
- **`ci-check`-Verdikt am Fix-HEAD — `wartend`** — der Push dieses Atoms
  triggert `ci-check` neu (`paths: src/**, tools/utils/**, docs/**`). Der Fix
  trägt den rustfmt-Stand aus Folge 103 + die Konverter-Erweiterung. (Schritt:
  den neuen Run einmalig aus dem Watchdog-Snapshot `/tmp/opencode/ci_status.md`
  lesen; bei Rot `ci_manage log <id>`.) · `wartend`

## Wartestellungen (kein Auswahlpunkt)

- **`te-gate`** `35513982359` in_progress (Forschung-Linie, 3σ-FPR-Fix).
  (Schritt: `ci_manage view 35513982359`.) · `wartend`
- **allwise-cdn** — stündlicher Schedule. · `wartend`

## Benannter Rest (gemessen, kein Konverter-Fix)

- **`dist_key` / `pos`** — `dist_key` (61 Treffer) trägt keine Scale
  (`1000/Plx`-Ausdruck, sonst `sy_dist`/`dist` ohne Einheit) → nicht mechanisch
  migrierbar; `pos` (5 Treffer) ist laut `docs/SOURCE_PORT.md:275`
  (`pos without body directive`) ein Quellen-Befund, kein Parser-Fix. Beide
  bleiben absent (0 honored), nicht fabriziert. (Schritt: Quellen-Recuration der
  Distanz-Einheit bzw. `body`/`on`-Direktive, dann ggf. Konverter-Arm.) ·
  `pending`

## Benchmark

- **Bau-Folge 104**: Atom „`--port`-Konverter ra/dec/plx/z-Migration". Delegation
  `grind-flash` (mechanisch), `cargo check --all-targets` 0/0. Kein Doppellauf —
  die Klasse „routine mechanical port" hat den gemessenen Sieger `grind-flash`
  (2026-09-16, 8 Profile identisch); kein pro/max nötig.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/archivar/port.rs`; `src/archivar/tests.rs`;
  `phi/pipeline/ledger.φ` (7 `parser-gap`-Notes); `docs/handover/post.md`
  (bau-Zeile gelöscht); neues
  `docs/handover/handover-2026-09-20-bau-folge104.md`; Move
  `handover-2026-09-20-bau-folge103.md` → `archiv/`.
- **Fremd (nicht anfassen):** `docs/handover/handover-2026-09-20-entscheid-folge61.md`
  → `archiv/` und `docs/handover/handover-2026-09-20-entscheid-folge62.md`
  (Entscheid-Linie, gestaged). Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Push triggert
`ci-check` selbst; die Session pollt nicht. `/consent` ist der session-weite
Consent, nie das Commit-Wort.
