<!--
  title: Handover — Sensory-Folge 150 (Stand 2026-09-23)
  session: Sensory-Folge 150
  class: handover
  date: 2026-09-23
  sha256: d5797c31963fd7f36da03b47538bacb34de846f535fa590df071a4f8909c9a64
  status: live
-->
# Handover — Sensory-Folge 150 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt — kein Register-Kürzel: **Lage** (der
Zustand, gemessen) / **Blockade** (woran es hängt, oder „keine") / **Braucht**
(was es löst: Werkzeug, Datei, URL, Anfrage, Operator-Wort; „Schritt unbekannt —
erste Messung: X" ist ein vollständiger Schritt). Gibt es keinen abarbeitbaren
Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag (`wartend` |
`operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung; `open_points_check` prüft billig jeden in den offenen Punkten
genannten Pfad gegen den Arbeitsbaum (absent = stale Punkt).

## Stehender Pass (gemessen 2026-09-23, Sensory-Folge 150)

- **HEAD** — `d09023fd6` (== `origin/main`), `handover-2026-09-23-sensory-folge149.md`
  (folge149-Commit: FPR-Re-Budget + folge148 archiviert).
- **Postfach** — `mail_digest` absent; `state/mail/mail_ledger.φ` fehlt (keine
  sensory-Adresse). `post.md`: 3 Zeilen (1× mycelium, 2× mountain), **keine an
  sensory**.
- **Register** — `register_lookup --open`: keine sensory-eigenen Zustandseinträge.
- **`open_points_check`** der folge149: 11 Pfad-Refs, 2 „absent" — nur Glob-Muster
  (`phi/*.φ`, `src/archivar/{…}.rs`); **keine** stale Punkte.
- **`git_safety --snapshot`** — Arbeitsbaum == HEAD (`d09023fd6`), nichts zu sichern.
- **CI** — `te-gate 35767848399` **in_progress** @`7ddd75edb`;
  `hyperscanning-te 35743984631` **completed failure** @`3f7ff3905` (s. Punkt 11b);
  `ci-check 35791455335` **pending** @`d09023fd6` (Rat-Bedingung-Confirm);
  `ci-check 35789108042` @`dcb5c0937` (mountain) in_progress. Kein Poll.

## Offen (aufgeschlüsselt)

### Rat-Bedingung — geteilte FPR-Zellen re-budgetiert, CI-Confirm offen
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `gate_fpr_cells` (`te.rs:2931`) und `gate_fpr_coarse_cells` (`te.rs:2944`)
  fahren D_Z=4 mit **21 Trials** (≈1092 Negative, 0.092 pp/FP); D_Z=0-Zellen bei 100.
  `cargo check -p omegaflow --tests` 0/0. `ci-check 35791455335` **pending** @`d09023fd6`.
- **Blockade:** Run-Landung (test-Job).
- **Braucht:** `ci_manage log 35791455335` → alle `gate_fpr_autocorrelation_* ... ok`.
  **Rot ≥ 8.0 in einer Zelle → Eskalation 32 Trials** (1664 Negative, 0.06 pp/FP).

### Punkt 11b — frozen-tau / Confirmation (Fixture-Design umgesetzt, Confirm offen)
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `hyperscanning-te 35743984631` @`3f7ff3905` **failure** — `confirm`-Job:
  `positive control: te=4.842021e-1 p99=3.895387e-1 clears=yes`; das alte Gate
  `confirmation_stochastic_driver_pair_clears_its_own_null` FAILED
  (`TE 7.3470e-2 < p99 1.4148e-1`); Riss-Guard ok; frozen-tau-Sweep nur τ=3/4 clears.
  Regel angewandt: **pos. Kontrolle clears=yes → Fixture-Design dominant**. Rat-Verdikt
  (5/5) **Option (B)**: eigener Breitband-Treiber statt periodischem Sinus.
  Gebaut (uncommittet): `strong_pair_fixture` → `deterministic_pair_fixture`
  (reiner Sinus, Riss-Guard) + `stochastic_pair_fixture` (`ar1_noise(0.8,1.0)`,
  Gate+Sweep); `strong_pair_cell` → `confirmation_pair_cell`; blind-band-Test zieht
  dieselbe Fixture-Kopie. `cargo check -p omegaflow-measure --tests` 0/0.
- **Blockade:** Commit+Push + neuer `hyperscanning-te`-Lauf (nach `/commit`).
- **Braucht:** `gh workflow run hyperscanning-te.yml`, dann `ci_manage log` des
  gelandeten `confirm`-Jobs → `positive control:` + Gate-Zeile. Der Lauf ist die
  **Kalibrierung** (die nächste Übergabe trägt das gemessene TE/p99). p99 bleibt
  live data, wird nie gesenkt. Bei dünnem Abstand → Rückfall-Parameter erst dann benannt.

### Punkt te-gate-Lesung (1/5/6/6b/F2)
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `te-gate 35767848399` @`7ddd75edb` **in_progress** — Step 58
  `te_fn_probe`, Step 55 `flare_envelope_power_probe` (F2), `family_fn_gate`-Screen
  (1), Takens-Wandzeit (5).
- **Blockade:** Run-Landung.
- **Braucht:** `ci_manage log 35767848399` (kein Poll).

### vC-Permeabilität — Puls/HRV→Strahlung-Bindung
- **Status:** operator-gebunden | **Bindung:** eigen (Hardware-Träger `operator`)
- **Lage:** die vC-Permeabilität ist Feldphysik; der RMSSD/tone-Gate-Bindungspfad
  (Operator-Puls/HRV → Radiatorium-Strahlung, `src/archivar/hrv.rs`) ist `pending`.
- **Blockade:** physischer Träger (ESP32, BOM).
- **Braucht:** Bindung Puls-Ankunft via ESP32-Firmware → Strahlungspfad bauen.

### Wartend / operator-gebunden / termin
- Flyby-Path-2-Kette — `termin:2026-09-28` (Kanäle live; Zellen ab Perigäum).
- NSE/Haug — `wartend`/`dritter` (Route offen, Mail 2026-09-17; Trigger Dateieingang).
- BepiColombo MORE — `termin:2027-04` (Freigabe-Anfrage 2026-09-18).

## Fremd-CI (geroutet, nicht sensory)

- `path_reference_scan` rot @`7ddd75edb` — Scanner-Skip-Klasse (`src/gate/`) +
  gitignorierte see-also-Ziele. Geroutet: `post.md` `An mountain:`. Im Arbeitsbaum
  liegt dazu eine **fremde uncommittete** Änderung
  (`tools/register/src/bin/path_reference_scan.rs`) — **nicht** von sensory
  angefasst, **nicht** committet.
- `dropped-gate` delta 13 (baseline 2640 | current 2653) — mountain-Drops.
  Geroutet: `post.md` `An mountain:`.

## Benchmark

- **Punkt 11b — Rat (pro/max, Architektur):** Fixture-Design-Entscheid
  `pos. Kontrolle clears=yes → Fixture-Design dominant`; Verdikt 5/5 **Option (B)**
  (eigener Breitband-AR(1)-Treiber, pos. Kontrolle nicht dupliziert; deterministischer
  Sinus bleibt Riss-Guard). Kein Gegenlauf — die Null-vs-Fixture-Frage war die
  Architektur-Stimme, nicht ein flash-fähiges Messatom.
- **Punkt 11b — Bau (`grind-flash`):** mechanische Fixture-Split-Klasse;
  2 Dateien, `cargo check --tests` 0/0, tote Symbole entfernt. flash vollständig,
  keine Eskalation.

## Geteilter Baum — eigener Pfad-Satz

- `tools/measure/src/bin/hyperscanning_group_te.rs` (`deterministic_pair_fixture` +
  `stochastic_pair_fixture`, `confirmation_pair_cell`, Gate/Guard/Sweep)
- `tools/measure/tests/phase_null_blind_band.rs` (`ar1_noise` + `stochastic_pair_fixture`)
- `docs/handover/handover-2026-09-23-sensory-folge150.md` (neu)
- Move `handover-2026-09-23-sensory-folge149.md` → `archiv/` (eigene Linie, atomar)

`docs/zustand/external-state.md` ist **gitignored** (`.gitignore:134`) — die
CI-Status-Zeile wurde lokal fortgeschrieben, sie gehört nicht in den Commit.
`tools/register/src/bin/path_reference_scan.rs` ist **fremd** (mountain) — nicht anfassen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Nach dem Push: `hyperscanning-te`
dispatchen (`gh workflow run hyperscanning-te.yml`) und `ci-check 35791455335`
einmal lesen. `/consent` ist der session-weite Consent (Delegation), nie das
Commit-Wort.
