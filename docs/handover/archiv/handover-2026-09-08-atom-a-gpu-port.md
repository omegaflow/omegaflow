<!--
  title: Handover — Atom A: der skalare GPU-Port, gebaut und paritätsgemessen
  class: handover
  date: 2026-09-08
  sha256: 9a4f2192f10be5e06ba5800c98c51a66c06df9541cbfc41aab6b4968df5aa19b
  status: archived
  see-also: docs/TODO.md docs/handover/handover-2026-09-07-korona-konditional-session.md docs/handover/handover-2026-09-07-rechen-myzel-ci.md
-->

# Handover — Atom A: der skalare GPU-Port

Übergabe der Sitzung 2026-09-08: Atom A (GPU-Port der skalaren Transfer-
Entropie-Matrix) ist gebaut, gegen die CPU-Referenz paritätsgemessen und
committet. Was bleibt, ist der Voll-Lauf auf dem Desktop und der Anschluss an
das Rechen-Myzel.

## 1. Was gebaut ist (committet)

- `d49610e` — der Code:
  - `src/mathematikerin/shaders.rs`: `SCALAR_TE_WGSL`, Entry `scalar_te_compute` —
    286 Threads = 2 Richtungen × 11 Serien × 13 Lags, `shift = max(lag,1)`,
    skalares Silverman (mit `max == min`-Degeneriertentest) + unnormalisiertes
    Gaussian, f32-Log-Floor `1e-30`, `valid`-Bit statt NaN.
  - `src/mathematikerin/scalar_te_gpu.rs` (neu): `ScalarTeGpu` — compute-only
    Device (`compatible_surface: None`), Batch-Upload von xs + ys + Surrogaten
    in den 12×1024-Puffer, ein Dispatch, die f64-Reduktion der zehn
    Surrogat-d-Statistiken bleibt CPU. In `mod.rs`/`lib.rs` exponiert.
  - `tools/measure/src/bin/solar_seconds_matrix_probe.rs`: Surrogat-Erzeugung
    aus der Lag-Schleife gehoben (einmal pro Fenster; der `lag`-Seed-Faktor
    `0xD1B5…` fiel, der Fenster-`idx` bleibt) — die ~23,5-M-Surrogat-Erzeugung
    entfällt ~13×. GPU-Pfad in `row_for` (pro Fenster ein `run`, `measure`-
    Closure liest das Grid: roh `k=0`, Surrogat `k=1+s`); kein GPU-Gerät →
    CPU-Pfad unverändert. Fortschritts-Log pro Paar.
  - `src/mathematikerin/tests.rs`: fünf Paritäts-Gates (FP-, FN-Entscheidung,
    Symmetrie, n-floor, Surrogat-Slots — alle 286 Grid-Slots gegen die
    CPU-Referenz, 0 Abweichungen) + numerischer Floor `SCALAR_PARITY_TOL = 1e-3`.
- `867e0ec` — Register: Atom geschlossen, WGSL-FFT entschieden-gegen (kein
  f64/u64 in WGSL; ein f32-FFT bräche Byte-Identität und Vollkreis-RNG).

## 2. Der gemessene Befund am Kern

Das f32-Silverman meldete für eine exakt konstante Serie Varianz ~6e-16
(Akkumulationsrauschen) → `valid=1`. Geflickt durch den `max == min`-Test im
Silverman — deckt die konstante Serie exakt ab, ohne die CPU-Parität realer
Signale zu verschieben. Der f32/f64-Log-Floor (`1e-30` vs `1e-300`) ist als
benannte Toleranzgrenze abgedeckt; die Entscheidungs-Gates tragen die Zähne,
der numerische Floor nur die grobe Divergenz.

## 3. Was offen bleibt (pending, keine descoped)

- **Desktop-Voll-Lauf** der Matrix auf der GTX 970 (i7-2600K): die einzige
  verbliebene Atom-A-Pflicht. Der GPU-Pfad lief auf dieser Maschine nur über
  llvmpipe (Software-Adapter) — die Gates liefen grün, der 3-Jahres-Voll-Lauf
  gehört auf den Desktop. Register: TODO „Desktop-Fork (GTX 970)".
- **Matrix-Split ins freie Myzel** (vier Stücke: CDN-Check, `--pairs von:bis`,
  Workflow-YAML, Reduce-Job) — Folge-Pflicht, im Register `8e67a3d`. Das Stück
  (b) (`--pairs von:bis`) ist direkt im `solar_seconds_matrix_probe` baubar;
  der Rest ist CI-Arbeit.
- **Klassen-Benchmark gegen die publizierte PCMCI-Suite** (`afbec78`) — die
  Messung, die das Superlativ „fortschrittlichste TE-Maschine" erst
  beantwortbar macht: TPR/FPR gegen Runge/IDTxl/Tigramite, gegen publizierte
  Zahlen (die Python-Regel schließt den Code-Lauf aus).

## 4. Der Baum zur Übergabe

Parallel zu dieser Sitzung lief eine zweite Sitzung im selben Baum (`te.rs`,
`omega.rs`, `actuators.rs`, `membrane.rs` wurden mehrfach mitten im Refactor
editiert — die Kompilierung brach zwischenzeitlich durch fremde Hand). Meine
Dateien (`scalar_te_gpu.rs`, `shaders.rs`, der Probe) wurden von ihr nicht
angefasst; meine Commits tragen nur meine Hunks. Der GPU-Port ist gegen den
aktuellen Baum verifiziert (5 Gates grün, `cargo check` 0 Fehler / 0 Warnungen).

## 5. Register

Befunde und offene Pflichten stehen in `docs/TODO.md` (Nadel Ⅲ: Skalar-TE-
GPU-Port geschlossen; Matrix-Split / Flotte / Ernte-Ritual / Herzstück;
Klassen-Benchmark). Git ist die Historie.
