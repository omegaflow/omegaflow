<!--
  title: Handover — Bau-Folge 97 (Stand 2026-09-19)
  session: Bau-Folge 97
  class: handover
  date: 2026-09-19
  sha256: cc65e7117e640146a80cee1cf62e57636453121a26756858995ad10c236362d7
  status: live
-->
# Handover — Bau-Folge 97 (2026-09-19)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder offene
Punkt trägt seinen nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt. Das Handover wird **vor allem anderen gegen den Baum gehalten**.

## Stehender Pass (gemessen 2026-09-19, Session-Beginn)

- **HEAD** `2a8c8aaf` (== `origin/main`, „bau folge96"); Safety-Net
  `refs/safety/1789853923` (Start).
- **Postfach** — `state/mail/mail_ledger.φ` lokal vorhanden; neueste Zeile
  `1789853943` (Rubin-Forum-Summary), kein bau-relevanter Eingang. `post.md` leer.
- **CI** — Watchdog-Snapshot (21:33Z) veraltet; frisch gemessen:
  **success** `openneuro-cdn 35470937379` (Idempotenz-Skip),
  **success** `harvest 35470954779` (icesat2_atl03) @`2a8c8aaf`;
  **in_progress** `ci-check 35470928166` @`2a8c8aaf`.
- **Konsens-Befund (umgesetzt):** ci-check fuhr die neuen Harvest-Bin-Tests nicht
  (`default-members=["."]`); die drei Bins + ein Probe-Arm sind jetzt gebaut.

## Offen

- **ds007471 BrainVision-Probe** `phi/pipeline/ledger.φ:126-128`. Arm + Compiler im
  Baum; CI-Arm gebaut: `.github/workflows/openneuro-eeg-probe.yml`
  (`--dataset ds007471 --subject sub-01`, ohne `--ci-mode`) + ci-check-Bin-Test;
  Probe `run 35473247219` dispatcht. (Schritt: `ci_manage view 35473247219` einmal
  lesen; das Verdikt ist der **staged count** im Log, nicht die Exit-Farbe —
  `contains("sub-01")` über-matcht sub-010…). Grün ⇒ format-Block in `phi/harvest.φ`
  + `harvest.yml` → CDN. · `wartend`
- **ds008192 SNIRF-Probe** `phi/pipeline/ledger.φ:130-132`. Arm + Compiler im Baum;
  derselbe CI-Arm (`--dataset ds008192 --subject sub-101 --session 02`); Probe
  `run 35473247219` dispatcht. (Schritt: wie ds007471; staged count lesen.) Grün ⇒
  format-Block in `phi/harvest.φ` + `harvest.yml` → CDN. · `wartend`

## Wartestellungen (kein Auswahlpunkt)

- **ci-check-Bin-Tests** (openneuro/brainvision/snirf) — `test`-Job des
  `ci-check`-Laufs `35473248876` (nach Push dispatcht). · `wartend`
- **`--sniff` Partial-Hash-Fix + lazy_chunk-/folge93-Gates + TE-Gates** —
  derselbe `ci-check`-Lauf. · `wartend`
- `te-gate 35462518676` @`3d2e6adb` · `wartend`
- **allwise-cdn** — 127/304 Spans, Final absent; stündlicher Schedule. · `wartend`

## Benannte Konfounde (Rat 2026-09-19)

- **`tools/harvest/**` fehlt in den ci-check-Push-Pfaden** — die drei neuen
  Bin-Test-Zeilen laufen nur, wenn `src/`/`phi/`/`docs/`/`tools/{register,utils}`
  sich ändern; ein harvest-only-Push löst keinen `test`-Job aus, und die Tests der
  übrigen ~70 Arme laufen in keinem Job. `pending` (Schritt: `tools/harvest/src/bin/**`
  in die ci-check-Push-Pfade aufnehmen oder auto-dispatch ci-check anstoßen).
- **icesat2_atl03** — `asset present` deckt ein 1-Granul-Fenster (`--limit 1
  --skip 1`) von 215 Granulen; die Note benennt es als `1/215`. (Schritt: nächster
  Harvest-Lauf `--skip 2`; `register_lookup --open` zeigt es.)
- **Probe-Verdikt** — der staged count im Log ist die Messung, nicht die grüne Farbe.

## Benchmark

- **Bau-Folge 97**: Probe-Arm-Bau (Workflow + ci-check-Bins) → `grind-pro`;
  ds007822-Produktionspfad + CI-Verifikation → `grind-flash`. Kein Doppellauf;
  keine neue Benchmark-Klasse (Routine-Klassen gemessen geschlossen).
  `cargo check` 0/0 (root + `-p omegaflow-harvest --all-targets`).

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `.github/workflows/ci-check.yml` (nur die drei
  Bin-Test-Zeilen — fremde `timeout-minutes`-Hunks unangetastet),
  `.github/workflows/openneuro-eeg-probe.yml` (neu), `phi/harvest.φ`,
  `phi/pipeline/ledger.φ`, `docs/handover/handover-2026-09-19-bau-folge97.md`
  (neu), Move `bau-folge-96` → `archiv/`.
- **Fremd (nicht anfassen):** `.github/workflows/hyperscanning-te.yml`,
  `.github/workflows/te-gate.yml`, `src/mathematikerin/te.rs`, die drei
  `handover-2026-09-16-*`-Moves, `docs/zustand/external-state.md`. Nie ein
  nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Nach dem Push:
`openneuro-eeg-probe.yml` und `ci-check` dispatchen. `/consent` ist der
session-weite Consent, nie das Commit-Wort.
