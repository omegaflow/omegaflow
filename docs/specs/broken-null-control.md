<!--
  title: The Broken Null Control
  class: concept
   sha256: 63b7e8b704bcb510e724bd9b61969b30b08339fe397af714880b32387124efb6
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

## 6. Limits — measured and gated

Each limit the earlier record left open is now measured; the four
quantitative limits are regression gates in `src/mathematikerin/te.rs`,
the two heavy sweeps run as jobs in `.github/workflows/te-gate.yml`.

- **Multiple comparison (measured 2026-09-15, `te_null_limits_probe`; gate
  `gate_fam_max_t_kills_false_positive_keeps_true_coupling`):** fam over the
  20 directed pairs × lags {0, 1, 2} (60 cells) = 2.18e-1; the per-cell μ+2σ
  threshold names 5 arrows, **0 survive fam**. A synthetic matrix
  (fam = 2.26e-1) kills the one per-cell false positive and keeps all 8 true
  couplings.
- **Lag sweep (measured 2026-09-15; gate
  `gate_lag_sweep_verdict_flips_at_coupling_horizon`, te-gate job
  `lag-sweep`):** τ 0–360 s — the live control pairs stay silent at every lag;
  the Hénon forward arrow sits at τ ∈ {0, 1, 5} (optimum τ = 1) and dies from
  τ = 10 onward. The gate pins the forward arrow at τ = 1 and its silence at
  τ = 10, and at τ = 1 the reverse below the forward (`r1 < f1`) — the reverse's
  absolute silence is asserted only at τ = 10, not at the coupling lag. The
  thin margin is held as measured directionality, never claimed as silence.
- **Bandwidth sensitivity (measured 2026-09-15; gates
  `gate_bandwidth_factor_one_is_the_library_path` and
  `gate_bandwidth_te_declines_with_h`):** Silverman factor 0.5–3.0, threshold
  recomputed under the same h — the verdicts are stable; one marginal arrow on
  Density-RTSW → EUV-304 at factor 3.0 (excess +4.3e-4, dies under fam). The
  factor-1.0 inline h-path reproduces the library estimator identically; TE
  declines monotonically with h.
- **Residual FN bias (measured 2026-09-15, `te_fn_probe`; gate
  `gate_fn_bias_n300_vs_n500_quantified`, te-gate job `fn-bias`):** the
  estimator carries a residual false-negative bias at n = 300 (0–3/10 true
  couplings found); it is named, and it does not drive false positives.
- **Window drift — closed (measured 2026-09-16, `te_null_limits_probe`):**
  the naive and phase thresholds are computed in one process on one binning —
  both nulls see the identical arrays and the same seed per pair, so no window
  rolls between them. On the fixed window the phase gate holds all four control
  pairs silent; the naive gate breaks only Dichte-RTSW → Bz. The 2026-09-12
  all-four-naive breach was a different window — the fixed-window re-run is
  measured, not pending. The fixed-window table lives in the paper
  (`docs/paper/broken-null-control.md`, §The form).

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
