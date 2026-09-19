<!--
  title: Handover — Bau-Folge 98 (Stand 2026-09-20)
  session: Bau-Folge 98
  class: handover
  date: 2026-09-20
  sha256: cd97e79b7199b2876895053ec2ee3f9c9a091f2e9f00f41be2c071eb72036487
  status: live
-->
# Handover — Bau-Folge 98 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder offene
Punkt trägt seinen nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt. Das Handover wird **vor allem anderen gegen den Baum gehalten**.

## Stehender Pass (gemessen 2026-09-20, Session-Beginn)

- **HEAD** `34482126` (== `origin/main`, „entscheid folge59"); Safety-Net
  `refs/safety/1789857019` (Start).
- **Postfach** — `state/mail/mail_ledger.φ` neueste Zeile `1789474752`
  (2026-09-16), kein bau-relevanter Eingang. `post.md` trug die ernte-Zeile zu
  ds007471 (abgeholt, gelöscht); die ernte-Linie faltet dieselben EEG-Verdikte
  gerade in `phi/*.φ` (fremde uncommittete Arbeit, unangetastet).
- **CI** — Watchdog-Snapshot `2026-09-19T23:33Z` älter als HEAD; frisch via
  `ci_manage`: `openneuro-eeg-probe 35473247219` **failure** (brainvision-Arm rot,
  snirf-Arm **success**); die übrigen `ci-check`-Läufe gecancelt/überholt.

## Offen

- **icesat2_atl03** — `asset present` deckt ein 1-Granul-Fenster (`--limit 1
  --skip 1`) von 215 Granulen. (Schritt: nächster Harvest-Lauf `--skip 2`;
  `register_lookup --open` zeigt es.) · `pending`
- **ds007471 BrainVision** — Ursache gemessen: das `.vhdr` trägt
  `DataFile=IBS_0001.eeg` (Original-BrainVision-Name), OpenNeuro speichert den
  BIDS-Kompanion `sub-01/eeg/sub-01_task-jointaction_eeg.eeg`; der Compiler suchte
  nur den DataFile-Pfad. Fix in `tools/harvest/src/bin/brainvision_compiler.rs`
  (`resolve_companion`: DataFile-Pfad zuerst, sonst `.vhdr`-Stamm + `.eeg`/`.vmrk`)
  + Bin-Test, `cargo check` 0/0. (Schritt: nach dem Push
  `openneuro-eeg-probe.yml` neu dispatchen, staged count lesen; dann format-Block
  in `phi/harvest.φ` + `harvest.yml` → CDN — gehört der ernte-Linie, Post gesetzt.)
  · `wartend`

## Wartestellungen (kein Auswahlpunkt)

- **ci-check Harvest-Bin-Tests** — `tools/harvest/**` steht jetzt in den
  ci-check-Push-Pfaden; ein harvest-Push löst den `test`-Job aus. · `wartend`
- **`--sniff` Partial-Hash-Fix + lazy_chunk-/folge93-Gates + TE-Gates** —
  derselbe `ci-check`-Lauf. · `wartend`
- `te-gate` @`3d2e6adb` · `wartend`
- **allwise-cdn** — stündlicher Schedule. · `wartend`

## Benchmark

- **Bau-Folge 98**: BrainVision-Kompanion-Diagnose (OpenNeuro-GraphQL +
  `.vhdr`-Fetch) + Parser-Fix → `build` (flash), kein Doppellauf, keine neue
  Benchmark-Klasse. `cargo check` 0/0 (root + `-p omegaflow-harvest --all-targets`).

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `tools/harvest/src/bin/brainvision_compiler.rs`,
  `.github/workflows/ci-check.yml` (nur die `tools/harvest/**`-Zeile),
  `docs/handover/post.md`, `docs/handover/handover-2026-09-20-bau-folge98.md`
  (neu), Move `bau-folge-97` → `archiv/`.
- **Fremd (nicht anfassen):** `phi/harvest.φ`, `phi/pipeline/ledger.φ`,
  `phi/sources.φ`, `docs/handover/handover-2026-09-20-ernte-folge103.md`
  (untracked), die Moves `entscheid24`/`forschung44`/`forschung51`. Nie ein
  nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Nach dem Push:
`ci-check` und `openneuro-eeg-probe.yml` dispatchen. `/consent` ist der
session-weite Consent, nie das Commit-Wort.
