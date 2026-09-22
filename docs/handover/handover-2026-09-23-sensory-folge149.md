<!--
  title: Handover — Sensory-Folge 149 (Stand 2026-09-23)
  session: Sensory-Folge 149
  class: handover
  date: 2026-09-23
  sha256: 5bf2614cbfcf0a12e1469edbf9117bcaa72f60d1ddd538fbc39bbb6347a35410
  status: live
-->
# Handover — Sensory-Folge 149 (2026-09-23)

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
erste Messung: X" ist ein vollständiger Schritt). Kein Dokument wächst ohne
Messung; die Droh-Sprache ersetzt den Schritt nicht. Der Planungs-Pass legt
**alle** eigenen Punkte vor und schlägt vor, jeden parallel abarbeitbaren zu
dispatchen; `operator-gebunden`, `blockiert` und `wartend` werden benannt, nie
dispatcht. Gibt es keinen abarbeitbaren Punkt, sagt die Session das. Jeder Punkt
trägt seinen Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung; `open_points_check` prüft billig jeden in den offenen Punkten
genannten Pfad gegen den Arbeitsbaum (absent = stale Punkt); eine Session, die nur
dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-23, Sensory-Folge 149)

- **HEAD** — `f676260da`. Eigene Vorsession: `d75f40469`
  (`handover-2026-09-22-sensory-folge148.md`).
- **Postfach** — `post.md` trägt 1 Zeile **an mycelium** (nicht sensory);
  `mail_digest` absent; `state/mail/mail_ledger.φ` fehlt (keine sensory-Adresse).
- **Register** — `register_lookup --open`: keine sensory-eigenen Zustandseinträge.
- **`open_points_check`** der folge148: 15 Pfad-Refs, 4 „absent" — nur Glob-Muster
  (`phi/*.φ`, `src/archivar/{…}.rs`); **keine** stale Punkte.
- **`git_safety --snapshot`** — Arbeitsbaum == HEAD (`f676260da`), nichts zu sichern.
- **CI** — `ci-check 35767837058` @`7ddd75edb` **failure**: `format`/`build`/`clippy`
  grün; `test` rot **nur** an `path_reference_scan` (fremd, geroutet), `dropped-gate`
  delta 13 (fremd, geroutet); das FPR-Gate
  `gate_fpr_autocorrelation_coherent_phase_null_binned_n_surr_200` ist **grün** →
  Punkt 2 der folge148 ist confirmt und geschlossen. `te-gate 35767848399`
  **in_progress** @`7ddd75edb`; `hyperscanning-te 35743984631` **in_progress**
  @`3f7ff3905`. Kein Poll.

## Offen (aufgeschlüsselt)

### Rat-Bedingung — geteilte FPR-Zellen re-budgetiert, CI-Confirm offen
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `gate_fpr_cells` (`te.rs:2931`) und `gate_fpr_coarse_cells` (`te.rs:2944`)
  fahren D_Z=4 jetzt mit **21 Trials** (7→21; ≈1092 Negative, 0.092 pp/FP) statt 7
  (0.27 pp/FP); die drei D_Z=0-Zellen bleiben bei 100. Betroffen: 6
  `gate_fpr_autocorr`-Tests (18 Zellen) + die ignorierten n=1000-Tests in
  `te-gate.yml` (24 Zellen). `cargo check -p omegaflow --tests` 0/0. Kein Riss: die
  8.0-Linie, die Null, der Seed unangetastet.
- **Blockade:** CI-Landung (nach `/commit`).
- **Braucht:** `test`-Job des nächsten `ci-check` → alle
  `gate_fpr_autocorrelation_* ... ok`. **Rot ≥ 8.0 in einer Zelle → Eskalation
  32 Trials** (1664 Negative, 0.06 pp/FP). Blindband bleibt: die 8.0-Linie +
  3σ-Rise ist ein Grob-Leck-Instrument, nie still.

### Punkt 11b — frozen-tau / Confirmation
- **Status:** wartend | **Bindung:** eigen
- **Lage:** pos. Kontrolle `positive_control_rich_aperiodic_driver_pair`
  (`tools/measure/src/bin/hyperscanning_group_te.rs:1756`) gebaut; Rat-Verdikt
  „kein Riss". `hyperscanning-te 35743984631` @`3f7ff3905` **in_progress**.
- **Blockade:** Run-Landung.
- **Braucht:** `ci_manage log` des gelandeten Laufs → `positive control:`-Zeile.
  `clears=yes` → Fixture-Design dominant; `clears=no` → Null-Konstruktion. Die
  p99-Schwelle ist live data, wird nie gesenkt.

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
  gitignorierte see-also-Ziele. Geroutet: `post.md` `An mountain:`.
- `dropped-gate` delta 13 (baseline 2640 | current 2653) — mountain-Drops.
  Geroutet: `post.md` `An mountain:`.

## Benchmark

- **Rat-Bedingung (grind-flash):** mechanische Tupel-Klasse, flash-first gegen die
  TE-/Null-Klasse (Vorsession: `grind-max`, benanntes hartes Atom). Ergebnis:
  2 Hunks `7→21`, `cargo check --tests` 0/0; kein pro/max-Gegenlauf nötig
  (flash vollständig, Klasse mechanisch).
- **Punkt 2 (grind-max, Vorsession 148):** Urteils-Atom, bereits im Register.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (`gate_fpr_cells:2931`, `gate_fpr_coarse_cells:2944`; 7→21)
- `docs/handover/post.md` (zwei `An mountain:`-Zeilen)
- `docs/handover/handover-2026-09-23-sensory-folge149.md` (neu)
- Move `handover-2026-09-22-sensory-folge148.md` → `archiv/` (eigene Linie, atomar)

`docs/zustand/external-state.md` ist **gitignored** (`.gitignore:134`) — die
CI-Status-Zeile wurde lokal fortgeschrieben, sie gehört nicht in den Commit.
`docs/zustand/dropped-baseline.md` wurde **nicht** angefasst (mountain-Drops, geroutet).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Nach dem Push: `ci-check`
(dropped-gate + FPR-Gate) und `hyperscanning-te` dispatchen.
`/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
