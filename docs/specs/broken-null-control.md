<!--
  title: The Broken Null Control
  class: concept
   sha256: 101360e76bd2b8a1edf1c34d47d5c2ea897451c4e4d82e61c3c8c0a23a8d3b56
  status: live
-->
# The Broken Null Control

**A negative result and its methodological prescription.**
Naive shuffle surrogates manufacture a spurious solar coronal-heating
cascade; autocorrelation-preserving (phase-randomized) surrogates erase
it. The null channel must be a first-class part of the measurement, not
a footnote.

*Recovered from the omegaflow project (commit `662cba9`), 2026-08-19.
Reproducible with `cargo run --release --bin nobel_probe_corona`.*
*Written in English for citability; the identity of the instrument lives
in German.*

---

## 1. The claim

Transfer-entropy (TE) significance tests that shuffle the driving series
(i.i.d. Fisher–Yates) produce **false-positive causal arrows** between
solar channels. The reason: solar time series are strongly
autocorrelated, and a naive shuffle destroys the autocorrelation, so the
null distribution sits *below* the true "no-coupling but same dynamics"
level. Correcting the surrogates to preserve the power spectrum
(phase randomization) **collapses a full causal cascade (16 significant
arrows) down to 2**, and the null control — which was broken under the
naive threshold — holds.

This is a cautionary, reproducible result for any field that reads
causality out of time series with surrogate significance (space weather,
climate, neuroscience).

## 2. Method

**Estimator** (KDE transfer entropy, Y→X at lag τ):

```
TE(Y→X; τ) = (1/m) Σ_t ln[ p(x_{t+τ}, x_t, y_t) · p(x_t)
                          / (p(x_t, y_t) · p(x_{t+τ}, x_t)) ],
m = n − τ,  τ = 0 ⇒ canonical one-step estimator.
```

Gaussian kernel, bandwidth by Silverman's rule `h = 1.06 σ n^(−1/5)`
per series. Series are binned to a common 60 s grid (mean per cell);
no up-interpolation.

**Significance.** Threshold = mean + 2σ over 10 surrogates of the
driving series:
- *Naive:* Fisher–Yates shuffle (destroys autocorrelation).
- *Phase-randomized:* FFT of the padded series, randomize phases with
  conjugate symmetry, inverse FFT, truncate — preserves the power
  spectrum, hence the autocorrelation, while destroying the coupling.
  (`surrogate_stats` vs `surrogate_stats_phase` in `src/te.rs`.)

**Null control.** The solar-wind proton density (Dichte-RTSW, an
em-quiet but wind-driven channel) is paired against the same targets
(X-Ray, EUV-304, EUV-284, Bz). It *must not* be significant. It is
measured, not assumed.

## 3. Data

Live NOAA SWPC archives, harvested 2026-08-19: GOES XRS 0.05–0.4 nm
(`xrays-7-day.json`, n≈10 078 @ 1 min), GOES EUVS 304 Å and 284 Å
(`euvs-7-day.json`, n≈10 024), RTSW magnetometer `bz_gsm` and
`proton_density` (`rtsw_mag_1m.json` / `rtsw_wind_1m.json`, ~1.2 d
window). All series aligned to Sun-time (`t − d/c` for photons,
`t − 1.481e11/(v·1000)` for L1 plasma). TE is shift-invariant, so the
TDB↔UTC constant cancels.

## 4. Results

**The cascade, before and after the fix.**

| pair | n | TE | threshold | verdict |
|---|---|---|---|---|
| Bz → EUV-304 (naive) | 939 | 6.02e-2 | 4.94e-2 | significant |
| EUV-304 → EUV-284 (naive) | 2823 | 5.71e-2 | 1.04e-2 | significant |
| Bz → EUV-304 (phase) | 859 | 3.94e-2 | 4.42e-2 | **not** |
| EUV-304 → EUV-284 (phase) | 1748 | 5.22e-2 | 8.08e-2 | **not** |

Under the naive threshold the DAG held **16 arrows** and the verdict
read *"magnetic energy flux through the transition region into the hot
corona (the Alfvén channel)"* — a clean, publishable coronal-heating
cascade. Under the phase threshold it collapsed to
`{ EUV-304→X-Ray, Bz-RTSW→X-Ray }`, and the Alfvén cascade is silent.

**The null control, all four pairs.**

| pair | n | TE | naive thr. | phase thr. |
|---|---|---|---|---|
| Dichte → X-Ray | 1190 | 5.74e-2 | 3.58e-2 (breaks) | 6.86e-2 (holds) |
| Dichte → EUV-304 | 1188 | 3.22e-2 | 3.09e-2 (breaks) | 4.63e-2 (holds) |
| Dichte → EUV-284 | 1188 | 3.14e-2 | 2.55e-2 (breaks) | 5.71e-2 (holds) |
| Dichte → Bz | 857 | 2.01e-1 | 1.98e-1 (breaks) | 2.20e-1 (holds) |

Every control pair breaks under the naive threshold and holds under the
phase threshold. The naive threshold was the artifact.

## 5. Prescription

1. **The null channel runs with the measurement, always.** The cells
   that must stay silent are *configuration*, not a footnote; when they
   speak, the significance machinery — not the physics — is on trial.
2. **Autocorrelation-preserving surrogates are mandatory** for
   autocorrelated series (phase randomization; block bootstrap as
   cross-check).
3. **Small n is not silence.** Below a floor (n = 30 here) report
   "no statement possible (n)", never a null finding.
4. **Report the surrogate spread** (mean and σ), not just the
   point estimate and threshold.
5. **Name the window.** The control ran on the ~2 d seconds-window;
   the OMNI↔GOES intersection was empty (ingest lag, stopDate 06.08.).
6. **The family bound fam is canonical.** fam = the maximum of the
   *surrogate TE values* (not the surrogate thresholds) over all
   pairs × lags of the round; the per-lag threshold μ + 2σ stays the
   single-cell test, fam the round (multiple-comparison) correction. An
   arrow is fam-significant only above this maximum. All blades carry
   the same definition: fam is a single TE number, never a threshold.

## 6. Open limits

- **Multiple comparison:** 20 directed pairs tested; the 2 surviving
  arrows are within the unprotected false-positive range. A max-T
  correction over the pair matrix is outstanding.
- **Lag sweep:** only τ ∈ {0, 60, 120} s was tested; the lag optimum
  is unverified.
- **Bandwidth sensitivity:** Silverman is a heuristic; the dependence
  of the verdicts on h is unmeasured.
- **Window drift:** the naive and phase runs are not on identical data
  (the RTSW window rolls ~2 h between runs); n drifts accordingly. The
  finding is about the method, demonstrated by the control both times.

The honest scientific content of this result is a **negative**: the
pretty cascade was an artifact of the test, and fixing the test leaves
silence where a result was expected. 0 honored — the silence is the
answer.

---

## 7. Re-measurement after the RNG fix (2026-08-23)

The original record ran on a broken `next_rng` ([0, 0.5) instead of
[0, 1); half-circle phase rotation — root cause of the 100 % false-positive
rate of the topological path, measured and healed in `36df723`). After the
fix this record was re-measured (`nobel_probe_corona`, live NOAA data,
2026-08-23T13:25Z, seconds window ~2 d):

- **The null control holds identically:** all four density-RTSW pairs
  still break under the naive shuffle threshold and hold under the
  phase-randomized threshold. The prescription survives the corrected
  RNG — it was never about the RNG scale, it is about the autocorrelation
  of the null channel.
- **The cascade stays silent:** no arrow on Bz → 304 → 284. DAG of the
  re-measurement: `{EUV-304→X-Ray, EUV-284→X-Ray, EUV-284→Density-RTSW}`
  (all marginal, excess ~4–13e-3). The original record carried
  `{EUV-304→X-Ray, Bz-RTSW→X-Ray}`; the strongest arrow
  (EUV-304→X-Ray) survives, the remainder is window-dependent — the live
  windows roll ~4 d between the runs. This is the same finding under a
  corrected instrument, not a new claim.

0 honored: the re-measurement is a measurement — the old numbers were
not patched, the old record stands as the predecessor under its named
condition.
