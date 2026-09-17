<!--
  title: Broken null control — the phase-randomized surrogate gate
  class: paper
  date: 2026-09-12
  sha256: 03e956611a1056f6e81a4a845b914d9b1261a49de0533ffe5acf39fc91be74b0
  status: live
  see-also: docs/specs/broken-null-control.md
-->
# Broken null control — the phase-randomized surrogate gate

## Abstract

A transfer-entropy (TE) significance test is as sound as its null. One directed
coupling series is measured under two nulls: a naive Fisher–Yates shuffle of the
driving series, destroying autocorrelation, and a phase-randomized surrogate,
preserving the power spectrum. The naive threshold resolves a full causal cascade
of 16 significant arrows; the phase-randomized gate collapses it to 2. All four
control pairs — solar-wind proton density against X-Ray, EUV-304, EUV-284 and
Bz — break under the naive threshold, hold under the phase-randomized one on the
2026-09-12 window. On the full 60-s CI grid (`te-null-limits`, run 35077126787,
2026-09-16) the single-cell control is not clean: Dichte-RTSW → X-Ray is
significant under both nulls (TE 2.94e-2 > phase thr 2.81e-2); the family bound
resolves it, `fam = 2.63e-1` with 0 of 60 cells surviving. A defect, `next_rng`
divided by `u32::MAX` instead of `u32::MAX >> 1`, rotated surrogate phases over a
half circle and scaled every null distribution: the false-positive rate measured
100 % before the fix and 6.7 % after (`te_rng_fix_probe`). The Kalibrier-Gate in
`src/mathematikerin/te.rs` holds false positives, false negatives, symmetry and
an n-floor as tests. A null not phase-randomized over the full circle fabricates
significance; the broken null is the named control.

## The measurement

The estimator is a KDE transfer entropy Y→X at lag τ: Gaussian kernel,
bandwidth by Silverman's rule `h = 1.06 σ n^(−1/5)` per series. The significance
threshold is `mean + 2σ` over ten surrogates of the driving series. Two nulls
are contrasted:

- *Naive:* Fisher–Yates shuffle of the driving series (destroys
  autocorrelation).
- *Phase-randomized:* FFT of the padded series, randomize phases with conjugate
  symmetry, inverse FFT, truncate — preserves the power spectrum, hence the
  autocorrelation, while destroying the coupling.

The RNG discipline is the full circle. `next_rng` returns
`((*rng >> 33) as f64) / ((u32::MAX >> 1) as f64)`, a draw over `[0, 1)`, and
the surrogate phase is `next_rng(rng) * 2.0 * π`. The division is by
`u32::MAX >> 1`, never by `u32::MAX`: dividing by `u32::MAX` yields `[0, 0.5)`
and scales every null distribution.

The family bound `fam` is canonical: the maximum of the *surrogate TE values*
(not the surrogate thresholds) over all pairs × lags of the round. The per-lag
threshold `μ + 2σ` stays the single-cell test; `fam` is the round
(multiple-comparison) correction, a single TE number, never a threshold. An
arrow is fam-significant only above this maximum.

The null channel runs with the measurement, always. The four control pairs pair
the solar-wind proton density (Dichte-RTSW, an em-quiet but wind-driven channel)
against X-Ray, EUV-304, EUV-284 and Bz. It *must not* be significant. It is
measured, not assumed.

## The finding

Under the naive threshold the DAG held **16 arrows** and the verdict read
*"magnetic energy flux through the transition region into the hot corona (the
Alfvén channel)"* — a coronal-heating cascade. Under the phase threshold it
collapsed to `{ EUV-304→X-Ray, Bz-RTSW→X-Ray }`, and the Alfvén cascade is
silent.

| pair | n | TE | naive thr. | phase thr. |
|---|---|---|---|---|
| Dichte-RTSW → X-Ray | 1190 | 5.74e-2 | 3.58e-2 (breaks) | 6.86e-2 (holds) |
| Dichte-RTSW → EUV-304 | 1188 | 3.22e-2 | 3.09e-2 (breaks) | 4.63e-2 (holds) |
| Dichte-RTSW → EUV-284 | 1188 | 3.14e-2 | 2.55e-2 (breaks) | 5.71e-2 (holds) |
| Dichte-RTSW → Bz | 857 | 2.01e-1 | 1.98e-1 (breaks) | 2.20e-1 (holds) |

Every control pair breaks under the naive threshold and holds under the phase
threshold. The naive threshold was the artifact.

The RNG defect is a second, independent break of the null. A `next_rng` that
divided by `u32::MAX` instead of `u32::MAX >> 1` rotated the surrogate phases
over a half circle. On synthetic independent pairs (a ⊥ b, no true coupling) the
false-positive rate measured 100 %; the false-negative rate measured 90 % (b
follows a). After the one-line fix the false-positive rate fell to 6.7 %
(`te_rng_fix_probe`). Re-measured on live NOAA data on 2026-08-23 under the
corrected RNG, the null control holds identically — all four density-RTSW pairs
still break under the naive shuffle threshold and hold under the phase-randomized
threshold — and the cascade stays silent.

## The form

The Kalibrier-Gate lives as tests in `src/mathematikerin/te.rs` (`#[cfg(test)]`).
Every change to the estimator or to the null must pass all four:

- `calibration_fp_independent_ar1_stays_near_chance` — false positives on
  independent AR(1) series stay near chance.
- `calibration_fn_true_coupling_is_found` — a true coupling is found.
- `calibration_symmetry_identical_series_measure_equally` — a = b measures
  equally in both directions.
- `calibration_n_floor_no_statement_below_threshold` — below the n-floor (n = 30)
  report "no statement possible (n)", never a null finding.

The limits the spec names, measured 2026-09-15 (`te_null_limits_probe`; the
cached live window 2026-09-08 → 2026-09-15, 2:1 decimation of the 60 s grid →
120 s cadence, n ≈ 876 per pair — the full 60 s grid is a CI job):

- **Bandwidth sensitivity:** the Silverman factor swept over
  h ∈ {0.5, 0.75, 1.0, 1.5, 2.0, 3.0}, the threshold recomputed under the same
  h (the inline h-path at factor 1.0 reproduces the library threshold
  identically — instrument check). TE declines monotonically with h on every
  pair and the threshold follows; the verdict is stable. All four control
  pairs stay silent at every factor except one marginal arrow on
  Density-RTSW → EUV-304 at the extreme factor 3.0 (TE 9.12e-3 vs
  thr 8.69e-3, excess +4.3e-4 — the null's tail; it dies under fam). On the
  coupled Hénon system the true direction holds the arrow at every factor
  and the reverse stays silent at every factor.
- **Lag sweep:** τ 0–360 s on the same pairs. The live control pairs stay
  silent at every lag (verdict stable; TE flat at 4–9e-2 against a slightly
  growing threshold). The Hénon forward direction holds the arrow at
  τ ∈ {0, 1, 5} with the optimum at τ = 1 (excess +9.5e-2) — the map's exact
  one-step coupling — and turns silent from τ = 10 onward; the verdict flips
  precisely at the coupling horizon. The reverse stays silent at every lag.
- **Multiple comparison:** the family bound fam over the 20 directed pairs ×
  lags {0, 1, 2} (60 cells, 600 surrogate values) is 2.18e-1. The per-cell
  μ+2σ threshold names 5 arrows (X-Ray → EUV-304 at all three lags,
  X-Ray → EUV-284 at lags 0–1, all marginal); **0 survive fam** — every
  per-cell arrow of the live matrix sits inside the unprotected
  false-positive range, the suspicion the paper named. The correction is
  not over-conservative: on a synthetic matrix (10 independent AR(1) pairs,
  4 coupled pairs, n = 300, 56 cells) fam = 2.26e-1 kills the one per-cell
  false positive (1/40 = 2.5 % on the independent cells) and keeps all 8
  true couplings.

**Window drift closed** (`te_null_limits_probe`, 2026-09-16): the naive
Fisher–Yates threshold and the phase-randomized threshold are computed in one
process on one binning — both nulls see the identical arrays and the same seed
per pair, so no window rolls between them. Live common 60 s grid (cached
2026-09-16, decimated 2:1 → 120 s cadence, τ = 1):

| pair | n | TE | naive thr. | naive | phase thr. | phase |
|---|---|---|---|---|---|---|
| Dichte-RTSW → X-Ray | 909 | 3.91e-2 | 4.03e-2 | silent | 5.48e-2 | silent |
| Dichte-RTSW → EUV-304 | 848 | 4.22e-2 | 5.23e-2 | silent | 7.26e-2 | silent |
| Dichte-RTSW → EUV-284 | 848 | 2.82e-2 | 4.10e-2 | silent | 5.59e-2 | silent |
| Dichte-RTSW → Bz | 918 | 1.13e-1 | 9.20e-2 | arrow | 1.61e-1 | silent |
| Hénon X→Y (true) | 1000 | 1.51e-1 | 4.69e-2 | arrow | 5.88e-2 | arrow |
| Hénon Y→X (reverse) | 1000 | 4.36e-2 | 3.73e-2 | arrow | 5.33e-2 | silent |

On this window the phase gate holds all four control pairs silent; the naive
gate breaks only Dichte-RTSW → Bz and is silent on the other three. The earlier
2026-09-12 finding (all four control pairs breaking under the naive threshold)
was measured on a different window; the fixed-window re-run does not reproduce
the naive breach for X-Ray, EUV-304 or EUV-284, while the phase gate is silent
on all four. The phase threshold exceeds the naive threshold on every pair of
this run. On the coupled Hénon system the asymmetry appears in one run: both
nulls find the true X→Y arrow, while the reverse Y→X is a naive false positive
(TE 4.36e-2 > naive 3.73e-2) that the phase null silences. The estimator carries
a residual false-negative bias at n = 300 (0–3/10 true couplings found,
`te_fn_probe`) — named, and it does not drive false positives.

**Full 60-s grid (CI, `te-null-limits`, run 35077126787, 2026-09-16).** The four
SWPC channels load from the cache (the earlier run's `cached_body` built
`<name>.json.json` and read them absent; fixed). Live common 60-s grid
`n_cells = 5767`, decimated 2:1 → 120 s. Both nulls on the identical window,
same seed per pair, τ = 1:

| pair | n | TE | naive thr. | naive | phase thr. | phase |
|---|---|---|---|---|---|---|
| Dichte-RTSW → X-Ray | 834 | 2.94e-2 | 1.87e-2 | arrow | 2.81e-2 | arrow |
| Dichte-RTSW → EUV-304 | 774 | 6.37e-2 | 6.78e-2 | silent | 8.70e-2 | silent |
| Dichte-RTSW → EUV-284 | 774 | 4.31e-2 | 4.97e-2 | silent | 5.71e-2 | silent |
| Dichte-RTSW → Bz | 831 | 1.43e-1 | 1.25e-1 | arrow | 1.85e-1 | silent |
| Hénon X→Y (true) | 1000 | 1.51e-1 | 4.69e-2 | arrow | 5.88e-2 | arrow |
| Hénon Y→X (reverse) | 1000 | 4.36e-2 | 3.73e-2 | arrow | 5.33e-2 | silent |

On this window the control pair Dichte-RTSW → X-Ray is significant under **both**
nulls (TE 2.94e-2 > phase thr 2.81e-2, excess +1.3e-3) — a single-cell control
breach, not smoothed. The family bound resolves it: over the live 20-pair matrix
(60 cells, lags {0,1,2}) `fam = 2.63e-1` and **0 cells survive** — the
Dichte→X-Ray cell sits inside the unprotected range. The synthetic matrix (10
independent + 4 coupled AR(1) pairs, 56 cells) reads `fam = 2.26e-1`, kills the
one per-cell false positive (1/40 = 2.5 % on the independent cells) and keeps all
8 true couplings. The bandwidth sweep: the Dichte→X-Ray arrow holds only at
Silverman factors 1.0–2.0 and dies at 0.5/0.75/3.0 (the null's tail); the true
Hénon direction holds at every factor and the reverse stays silent. The lag
sweep: the Dichte→X-Ray arrow sits at τ = 1 alone (excess +2.1e-4); the true
Hénon direction holds τ ∈ {0, 1, 5} and turns silent from τ = 10; the reverse
stays silent. The single-cell verdict is window- and bandwidth-conditional; the
fam correction is the stable statement.

The scientific content is a negative: the cascade was an artifact of the test,
and fixing the test leaves silence where a result was expected. 0 honored — the
silence is the answer.

## References

1. Schreiber, T. (2000). Measuring information transfer. *Physical Review
   Letters* 85, 461–464. DOI: 10.1103/PhysRevLett.85.461. ADS:
   2000PhRvL..85..461S.
2. Theiler, J., Eubank, S., Longtin, A., Galdrikian, B., Farmer, J. D. (1992).
   Testing for nonlinearity in time series: the method of surrogate data.
   *Physica D* 58, 77–94. DOI: 10.1016/0167-2789(92)90102-S. ADS:
   1992PhyD...58...77T.
3. Kraskov, A., Stögbauer, H., Grassberger, P. (2004). Estimating mutual
   information. *Physical Review E* 69, 066138. DOI: 10.1103/PhysRevE.69.066138.
   ADS: 2004PhRvE..69f6138K.
4. Frenzel, S., Pompe, B. (2007). Partial mutual information for coupling
   analysis of multivariate time series. *Physical Review Letters* 99, 204101.
   DOI: 10.1103/PhysRevLett.99.204101. ADS: 2007PhRvL..99t4101F.
