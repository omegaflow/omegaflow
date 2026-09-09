<!--
  title: Befund — the sP-dual inversion: the sP leg tightens the Zonen precision (sd 59.2 → 45.6 km) but not the accuracy (mean −5.6 vs −6.0 km); the residual is the shared catalog-depth window anchor, not a phase defect
  class: befund
  date: 2026-09-09
  sha256: cad1515e962de386246eeaed66fe1629f5e8ff4ae9b0193da594fce5ed92e570
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-zonen-flotte-pick-hebel.md docs/befund/befund-2026-09-09-zonen-flotte-pick-hebel.md docs/befund/befund-2026-09-09-zonen-flotte-kante.md
-->
# Befund: the sP-dual inversion — precision tightened, accuracy unchanged

## Frage & Bindung

The register line of the pick-hebel handover: "sP-Joint-Inversion beider Phasen — Nachfolger-Atom (pP + sP, ak135-sP-Lag-Trennung; die Runner-up-Messung ist die Bedingung, die jetzt steht)." The council fixed the architecture measure-first: the sP corr gate must be set from the measured sP |corr| distribution, never the borrowed 0.30; the sP Δ-fold structure stays unmeasured; the dual fit tightens precision, not accuracy (both phases anchor their windows on the catalog depth).

## Was gebaut wurde

- `DepthPhase` (PP/SP) + `phase_lag` — one prediction switch for both phases.
- `invert_depth_dual(deltas, lags, weights, phases)` next to the untouched pP functions: the weighted residual over stations × both phases; sP absent → pP-only; non-finite weights or length mismatch → Absent.
- `StationMeasure.s_p_sigma_s` — the sP pick's half-width sigma (`peak_sigma_s`, unchanged).
- The fleet bin: sP |corr| per station (measurement, no gate), sP-leg admission counts, and the dual fit via `--sp-gate`; the pP-only weighted joint stays the baseline.
- Kalibrier-Gate (29 tests): the dual reproduces the weighted fit exactly with no sP legs (identity); recovers the depth from both phases; the sP leg pulls a biased pP back; a wrong sP with a wide sigma does not move the depth; leg order symmetric.

## Die Messung (fleet re-run, the same 16 events)

The sP |corr| distribution over the fleet (n=86 stations): min 0.44, p25 0.82, median 0.90, p75 0.94, max 1.00 — the sP phase is strong fleet-wide; the borrowed 0.30 gate would have admitted nearly everything, so the gate = the measured median 0.90.

| Größe | pP weighted joint (baseline) | sP dual fit (gate 0.90) |
|---|---|---|
| mean offset | −6.0 km (11 events) | −5.6 km (12 events) |
| sd across events | 59.2 km | 45.6 km |
| se = sd/√N | 17.9 km | 13.2 km |

## Die Befunde

1. The sP leg tightens precision, not accuracy: sd 59.2 → 45.6 km (~23 %), mean −6.0 → −5.6 km (unchanged). The dual fit is a keeper.
2. The unchanged mean is the shared catalog-depth window anchor, not a phase defect: both phases anchor their correlation windows on the same catalog depth, so the residual carries the anchor. The council's confound is now the measurement.
3. The wall stands in both fits: us10006scr (cat 596.4) clamps at the 660 wall in the dual and the pP-only fit — named, not counted.
4. One sP benefit: us6000q5tp (cat 271) recovered a dual depth (313 km) where the pP-only weighted fit was pending (its single pP station carried a sigma-absent pick) — the sP leg fed a node the pP thread could not.
5. The gate sits inside a plateau, not on an edge: the distribution is high-mass (0.82–0.94); the median 0.90 admits 1–5 sP legs per event (~half the picks sit below the gate). The gate is reported as a measured median with its insensitivity named, never as a sharp physical edge.

## Benannt (Pendings, not still)

- sP Δ-fold structure — unmeasured (a register duty, not 0.0).
- pP-fold-skipped stations' sP legs — pending.
- The 660 wall stays unresolved (both fits clamp us10006scr).

## Verdikt

The sP-dual inversion stands: precision shrinks measured (sd 59.2 → 45.6 km, se 17.9 → 13.2, n=12), accuracy does not — the remaining −5.6 km is the shared catalog-depth window anchor, not a phase defect. Gate = measured median 0.90 (n=86, plateau 0.82–0.94 named); pending: the sP Δ-fold structure and the pP-skipped stations' sP legs.
