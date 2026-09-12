<!--
  title: Broken null control — the phase-randomized surrogate gate
  class: paper
  date: 2026-09-12
  sha256: 24e196334915e901bc429ffe838168332ab42783d472f44e5393dbc388670448
  status: live
  see-also: docs/specs/broken-null-control.md
-->
# Broken null control — the phase-randomized surrogate gate

## Abstract

A transfer-entropy (TE) significance test is only as sound as its null. The same
directed coupling series is measured here under two nulls: a naive Fisher–Yates
shuffle of the driving series, which destroys autocorrelation, and a
phase-randomized surrogate, which preserves the power spectrum. Under the naive
threshold the test resolves a full causal cascade of 16 significant arrows;
under the phase-randomized gate it collapses to 2. All four control pairs —
solar-wind proton density against X-Ray, EUV-304, EUV-284 and Bz — break under
the naive threshold and hold under the phase-randomized threshold. A separate
defect, `next_rng` divided by `u32::MAX` instead of `u32::MAX >> 1`, rotated the
surrogate phases over a half circle and scaled every null distribution: the
false-positive rate measured 100 % before the fix and 6.7 % after
(`te_rng_fix_probe`). The Kalibrier-Gate in `src/mathematikerin/te.rs` holds
false positives, false negatives, symmetry and an n-floor as tests. A null that
is not phase-randomized over the full circle fabricates significance; the broken
null is the named control.

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

The limits the spec names remain open. **Multiple comparison:** 20 directed pairs
were tested; the 2 surviving arrows are within the unprotected false-positive
range, and a max-T correction over the pair matrix is pending. **Lag sweep:** only
τ ∈ {0, 60, 120} s was tested; the lag optimum is unverified. **Bandwidth
sensitivity:** Silverman is a heuristic; the dependence of the verdicts on h is
unmeasured. **Window drift:** the naive and phase runs are not on identical data
(the RTSW window rolls ~2 h between runs); n drifts accordingly. The estimator
carries a residual false-negative bias at n = 300 (0–3/10 true couplings found,
`te_fn_probe`) — named, and it does not drive false positives.

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
