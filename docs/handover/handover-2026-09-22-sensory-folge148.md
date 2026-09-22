<!--
  title: Handover — Sensory-Folge 148 (Stand 2026-09-22)
  session: Sensory-Folge 148
  class: handover
  date: 2026-09-22
  sha256: 776069556f1d6cf7798985bc425aa662c32b8bcbb83ea56408cae02d024f62b3
  status: live
-->
# Handover — Sensory-Folge 148 (Stand 2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** abgearbeitet.
Jeder Punkt trägt **Lage / Blockade / Braucht** und seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-22, Sensory-Folge 148)

- **HEAD** — `cd460f1ad` (mountain folge135; HEAD bewegte sich während der
  Session von `e12219b75`). Eigene Vorsession: `3f7ff3905`
  (`handover-2026-09-22-sensory-folge147.md`).
- **Postfach** — `post.md` leer; `mail_digest` **absent** (Lücke benannt, kein
  Null). Keine sensory-Adresse in `state/mail/mail_ledger.φ`.
- **Register** — `register_lookup --open`: **keine sensory-eigenen
  Zustandseinträge**; alle Dispositionen `[mycelium]`-getaggt.
- **`open_points_check`** der folge147: 16 Pfad-Refs, 2 „absent" — nur
  Glob-Muster (`phi/*.φ`, `src/archivar/{…}.rs`); **keine** stale Punkte.
- **`git_safety --snapshot`** — der Arbeitsbaum trägt fremde uncommittete Arbeit
  (`phi/*.φ`, `tools/harvest/*`, `tools/measure/*`); eigene Pfade s. unten.
- **CI am HEAD** — `ci-check 35746049660` @`e12219b75` **failure**: `format` +
  `build` grün; `test` rot (`gate_fpr_autocorrelation_coherent_phase_null_binned_n_surr_200`,
  FPR 8.52 % bei a=0 D_Z=4); `clippy` rot (`te.rs:3350` needless-range-loop);
  `dropped-gate` 2517 | 2586 | delta 69. `te-gate 35734557660` **in_progress**,
  `hyperscanning-te 35743984631` **pending**. Fremd-Failures `hdf5-real-granule`,
  `superdarn-rawacf-cdn`. Kein Poll.
- **Tools-Frische** — Manifest `git_sha=d16f2db0f` < HEAD → stale;
  `register_lookup --dropped --count` nutzbar (2640).

## Offen (aufgeschlüsselt)

### Punkt 2 — exact-n DFT-Rotation: clippy + FPR-Gate gefixt, CI-Confirm offen
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `ci-check 35746049660` @`e12219b75` war rot: (a) `clippy`
  `needless-range-loop` `te.rs:3350` (Test-DFT-Referenzschleife) — gefixt
  (`.iter().enumerate()`); (b) `test` rot: FPR 8.52 % bei a=0 D_Z=4 > 8. Ursache
  **gemessen**: die exact-n-Rotation (`3f7ff3905`) verschob die deterministische
  Surrogat-Trajektorie; das Parent `35743679830` @`a25f9da46` (gepolstert) war
  grün; `git log a25f9da46..e12219b75 -- te.rs` = genau `3f7ff3905`. Rat-Verdikt
  **steht**: 8.52 % = Fixed-Seed-Excursion an der Auflösungsgrenze (364 Negative,
  0.27 pp/FP, 2 FP über der 29.1-Linie; Pool 5.13 %) — kein Null-Defekt. Fix
  `te.rs:4656`: D_Z=4 Trials 7→21 (1092 Negative, 0.092 pp/FP); die 8.0-Linie,
  die Null, der Seed, der Assert unangetastet. `cargo check --tests` 0/0.
- **Blockade:** CI-Landung.
- **Braucht:** `test`-Job des nächsten `ci-check` →
  `gate_fpr_autocorrelation_coherent_phase_null_binned_n_surr_200 ... ok`.
  **Rot ≥ 8.0 → Verdikt kippt: die Null wird die Arbeit, nicht die Trials.**
  Blindband benannt: die 8.0-Linie + 3σ-Rise ist ein Grob-Leck-Instrument — eine
  echte 8–9 %-Einzelzellen-Rate passiert das erweiterte Gate ~13–28 % der Zeit;
  diese Toleranz ist Design, nie still.

### Rat-Bedingung — geteilte FPR-Zellen-Auflösung
- **Status:** wartend | **Bindung:** eigen
- **Lage:** die geteilte Batterie `gate_fpr_cells` (`te.rs:2931`) fährt D_Z=4 mit
  7 Trials (0.27 pp/FP), D_Z=0 mit 100 (0.25 pp/FP); `gate_fpr_coarse_cells`
  (`te.rs:2944`) trägt dieselben 7-Trial-D_Z=4-Zellen in die ignorierten
  n=1000-Varianten (te-gate.yml). Die Geschwister (Block/Shift/XShift/Restricted/
  Arx) sitzen latent-fragil unter derselben 8.0-Linie; die akkumulierte
  Fehl-Auslösung über 48 Zellen ist die Design-Schuld.
- **Blockade:** keine (eigenes Atom).
- **Braucht:** gemessenes Re-Budget als eigenes Atom — Trial-Tupel in
  `gate_fpr_cells`/`gate_fpr_coarse_cells` ändern, `ci-check` neu, Zahlen
  aufzeichnen. 32 Trials (1664 Negative, 0.06 pp/FP) als Eskalation, nur falls
  eine Zelle je innerhalb 1σ der Linie liegt.

### Punkt 11b — frozen-tau / Confirmation
- **Status:** wartend | **Bindung:** eigen
- **Lage:** pos. Kontrolle `positive_control_rich_aperiodic_driver_pair`
  (`tools/measure/src/bin/hyperscanning_group_te.rs:1756`) gebaut; Rat-Verdikt
  „kein Riss". `hyperscanning-te 35743984631` **pending**.
- **Blockade:** CI-Landung.
- **Braucht:** `ci_manage log` des gelandeten Laufs → `positive control:`-Zeile.
  `clears=yes` → Fixture-Design dominant; `clears=no` → Null-Konstruktion. Die
  p99-Schwelle ist live data, wird nie gesenkt.

### Punkt te-gate-Lesung (1/5/6/6b/F2)
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `te-gate 35734557660` @`b29cc096` **in_progress** — Step 58
  `te_fn_probe`, Step 55 `flare_envelope_power_probe` (F2), `family_fn_gate`-Screen
  (1), Takens-Wandzeit (5).
- **Blockade:** Run-Landung.
- **Braucht:** `ci_manage log 35734557660` (kein Poll).

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

## Benchmark

- **Punkt 2 (clippy + FPR, `grind-max`):** Urteils-Atom pro/max — TE-/Null-
  Konstruktion ist ein benanntes hartes Atom; kein flash-Gegenlauf. Ergebnis:
  Root cause gemessen (exact-n verschob die Trajektorie), Fix 7→21 Trials.
- **dropped-gate-Attribution (`grind-flash`):** mechanische Routine-Klasse —
  flash-first, Sieger-Klasse 2026-09-16 zitiert. Ergebnis: delta 69 = mycelium
  138→139 41 + sensory 146→147 18 + mountain 133→134 10 (+ folge135 8).

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` (clippy-Fix `:3350`, FPR-Test `:4656`)
- `docs/zustand/dropped-baseline.md` (Baseline 2517 → 2640)
- `docs/handover/handover-2026-09-22-sensory-folge148.md` (neu)
- Move `handover-2026-09-22-sensory-folge147.md` → `archiv/` (eigene Linie, atomar)

`docs/zustand/external-state.md` ist **gitignored** (`.gitignore:134`) — die
CI-Status-Zeile wurde lokal fortgeschrieben, sie gehört nicht in den Commit.

Fremde uncommittete Arbeit im selben Baum (`phi/*.φ`, `tools/harvest/*`,
`tools/measure/*`) wurde **nicht** angefasst. Der `format`-Job ist grün
(`35746049660`) — der repo-weite Format-Punkt der folge147 ist geschlossen; die
Baseline 2517 → 2640 ist gesetzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. Nach dem Push: `ci-check`
(dropped-gate gegen 2640 + das FPR-Gate) und `hyperscanning-te` dispatchen.
`/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
