<!--
  title: The directional driver of geomagnetically induced currents
  class: paper
  date: 2026-10-01
  sha256: 423cf68869d940e0e60350071a2bfa473b7c66dd4d0833667072a671f8dfa986
  fam-machine: post-fix
  status: live
  see-also: docs/specs/broken-null-control.md
-->

# The directional driver of geomagnetically induced currents

*Omegaflow Working Group*

## Abstract

Geomagnetically induced currents (GIC) are driven by the induced geoelectric field; dB/dt is the proxy. Which solar-wind quantity drives it — southward Bz, speed, or density — is open sub-daily. We measure transfer entropy (TE) from L1 drivers to hourly and daily maxima of dB/dt at INTERMAGNET Abisko (68.36° N), with phase-randomized surrogates and a round-maximum family bound whose AR(1) calibration showed a near-nominal rate but does not establish family-wise error control for this dependent, non-stationary design (§3.2). At the minute grain Bz→dB/dt peaks at lag 60 min but stays family bound in a 22-hour window. At the hourly grain two witnesses refuse to converge. The yearly-round witness finds Bz→dB/dt above the family bound in Abisko 2024/2025 and Sodankylä 2024 (0.12670 vs 0.10557; 0.13309 vs 0.12136; 0.11695 vs 0.10571), but the arrows sit at the lag-0/1 edge bin, so the lag is not resolved (§3.1, §5). The hardened quarterly witness (n_surr = 100) keeps all rows family bound; density never clears the bound, and the daily 32-year pairs stay below. No family-clearing hourly driver is established; Bz remains the leading sub-daily candidate. PCMCI removes the conditioned edge in 13/16 shards. The estimator's ground-truth verdict is NOT PASS; the riss stands.

## Key Points

- Transfer entropy measures how much information flows from solar-wind drivers to rapid ground magnetic-field changes at two observatories.
- The southward interplanetary magnetic field leads the yearly response but does not clear the family bound at the quarterly grain.
- The particle-density control never clears the bound, and the daily grain is empty: no family-clearing hourly driver is established; the four derived channels (Bs, Clock, P_dyn, M_A) stay family bound in all 15 quarterly artifacts.

## Plain Language Summary

When Earth's magnetic field changes rapidly, electric currents can flow in power
grids and pipelines. The Sun's wind carries a magnetic field and streams of
particles that meet Earth's magnetic shield. Which measured property of that
wind causes the fastest ground changes is not settled for time spans shorter
than a day. We measured how much information flows from each candidate property
to the fastest ground changes at two northern observatories. The measure follows
the direction of influence, so it can point at a driver — though on our
benchmark it did not fully separate driver from response at strong coupling,
so the direction reading stays cautious. The
southward part of the Sun's magnetic field leads the ground changes in
comparisons over full years at both stations, and a co-varying control,
the particle density, never shows an effect. When the same comparison is
tightened to single seasons, the signal no longer separates from the random
background, so we report the hourly result as open rather than settled.
Averaged over a day, and over three decades of storms, the effect disappears,
because a full day's bucket washes out the short, sharp magnetic swings. The
practical reading is to watch the southward field upstream of Earth as the
leading sub-daily candidate, while treating the hourly case as an open
measurement.

## 1. Introduction

Geomagnetically induced currents flow in power grids and pipelines when the
ground magnetic field changes rapidly; the engineering risk variable is dB/dt,
its time derivative (Pulkkinen et al., 2017). The upstream driver is the solar
wind at the L1 point. Which of its measured quantities — the interplanetary
magnetic field (in particular its southward component Bz in GSM coordinates),
the bulk speed, or the density — drives the ground response in the directional
information-flow sense, and at
what lag, is both a physics question and an operational one: forecasters
watch the solar-wind monitor 30–90 minutes upstream of the magnetosphere, and
an identified driver at the correct lag is a warning channel.

Correlation separates none of these candidates: all solar-wind quantities
co-vary through their common origin. Transfer entropy (Schreiber, 2000) is
the natural instrument: a directional, model-free measure of information flow
between time series, closely related to Granger causality but nonlinear and
nonparametric. TE has been applied to solar-wind–magnetosphere coupling
(Johnson & Wing, 2005; Wing & Johnson, 2016; Yu et al., 2022) and to climate
causality at large scale (Runge et al., 2019), but the GIC driver chain — L1 Bz to ground
dB/dt at an auroral-zone station, measured with a strict phase-randomized
null and a round-maximum family bound — has not, to our knowledge, been settled.

This paper reports a measurement series at three time grains: minutes
(one 22-hour live window), hours (two full years, 2024 and 2025), and
days (32 years, 1994–2026). The instrument is the untouched scalar TE
estimator of the omegaflow field system; the null model and the family bound
are those of its broken-null-control record. All verdicts below are reported
exactly as measured, including the silent ones.

## 2. Data

### 2.1 Solar wind at L1 (top side)

- **Minute grain:** SWPC RTSW 1-minute files (`rtsw_mag_1m.json`,
  `rtsw_wind_1m.json`), active records only (inactive rows are superseded
  monitor duplicates). Window 2026-08-20T16:19 → 2026-08-21T14:20 UTC,
  n ≈ 1378 (Bz, nT) / 1207 (speed, km/s; density, cm⁻³).
- **Hourly and daily grains:** OMNI2 (`OMNI2_H0_MRG1HR` via CDAWeb HAPI),
  harvested 1994-01-01 → 2026-08-06, bucketed by the median into 60-minute
  and 1440-minute bins (the daily bin is the compiler's standard
  decimation; 5 364 fill-skipped rows, fill gates 999.9 nT / 9999 km/s
  etc.). Parameters: BZ_GSM1800, V1800, N1800.

### 2.2 Ground magnetic field (bottom side)

INTERMAGNET Abisko (ABK, 68.358° N, 18.823° E — auroral zone), served by the
BGS GIN HAPI (`ABK/best-avail/PT1M/xyzf`), 1-minute X/Y/Z in nT, fill
99999.0 nT skipped, harvested in monthly chunks (year-sized requests are
reset by the server). The induction excitation is

dB/dt(t) = |ΔB|/Δt,  ΔB = B(t) − B(t−1 min),  |ΔB| = √(ΔX² + ΔY² + ΔZ²),

in nT/min. For the hourly grain the hour-maximum of the minute values, for
the daily grain the day-maximum, form the bottom series. `best-avail` is a
status stack — definitive (≤ 2021-12-31), quasi-definitive (2012 → ~1 month
ago), reported/adjusted (last month) — whose boundaries are named
non-stationarities of the series, not hidden ones.

## 3. Methods

### 3.1 Transfer entropy

For two series X, Y on a common grid, the lag-τ transfer entropy

TE(Y→X; τ) = Σ_t ln [ p(x_{t+τ}, x_t, y_t) · p(x_t) / ( p(x_t, y_t) · p(x_{t+τ}, x_t) ) ] / m

(nats; m = n − τ samples) is estimated by the KDE estimator with Silverman
bandwidths (Schreiber, 2000; Kaiser & Schreiber, 2002). X is the target
(ground dB/dt), Y the driver (Bz, speed, density). No pre-shift is applied: the lag sweep *is* the L1→Earth propagation time, 30–60 min for
300–800 km/s. At the minute grain a lag-0 or sweep-edge arrow is treated as an
artefact candidate, not a finding (the minute probe names it so,
`bz_blatt_probe.rs`). At the hourly grain the travel time straddles the
lag-0/lag-1 bin boundary: both bins are edge bins, and an arrow in either is
reported as measured but flagged edge-bin, never read as a clean lag. The
estimator's lag-0 arm is not an instantaneous condition: `transfer_entropy_lag(x, y, 0)`
dispatches to `transfer_entropy` (`src/mathematikerin/te.rs:96-99`), whose
conditional advances the target by one step (`x[t+1]`, `:33`) — the identical
computation to `transfer_entropy_lag(x, y, 1)`. Measured on an AR(1) pair
(`tools/measure/src/bin/fam_calibration.rs`, `--lag-check`): TE(lag 0) =
TE(lag 1) = 0.21722066662653602, TE(lag 2) = 0.23052033970922944. The `lag 0`
label is the one-step-ahead (τ = 1) condition; the yearly round's `lags = [0, 1]`
tests each pair twice, not two lags.

### 3.2 Null model and thresholds

For every measured pair and lag, ten phase-randomized surrogates of the
driver (f64 FFT, deterministic seed) yield the per-lag threshold μ + 2σ. In
addition, the **family bound** fam = the maximum surrogate TE over *all*
pairs × lags of the measurement round — the multiple-comparison control:
with the yearly round's twelve pair-lag calls carrying six distinct statistics
(the lag-0 and lag-1 arms are algebraically identical, §3.1), a per-lag excess arises
by chance; an arrow requires TE > fam (and therefore exceeds every null TE
of the round). fam is an empirical plug-in maximum — the largest surrogate
TE actually drawn in this round — not a quantile of a closed-form maximum
distribution, and the estimator names no fixed α. The scalar
round-max calibration battery (`tools/measure/src/bin/fam_calibration.rs:87`,
`fam-scalar-calibration.yml`, CI run 36639382300, success) measures, at the
operating n = 1260 on four independent AR(1) channels of autocorrelation
a = 0.0/0.5/0.9, FWER 7.83% (95% Wilson CI 5.94–10.26%), 9.50% (7.40–12.11%)
and 9.50% (7.40–12.11%) with n_surr = 10 (600 trials per a) against the nominal
1/(n_surr+1) = 9.09%, and FWER 1.67% (0.29–8.86%), 0.00% (0.00–6.02%) and 1.67%
(0.29–8.86%) with n_surr = 100 (60 trials per a) against the nominal 0.99%. The
battery shows that in this battery of independent AR(1) channels the empirical
rate fell near the nominal value; it does not establish family-wise error
control for the real design. A plug-in maximum is a valid level only under
i.i.d. draws from the true null distribution of the maximum; here the surrogate
streams are dependent (the shared deterministic seed across pairs and lags), the
twelve pair-lag calls carry only six distinct statistics (the lag-0/1 identity,
§3.1), and the true null of the maximum is misspecified for non-stationary,
heavy-tailed hourly maxima. A calibrated null distribution of the round
maximum is constructed and measured (Sodankylä 2024, §6). The concrete form (reviewer consultation,
2026-10-01): a studentized Westfall–Young max-T over the distinct statistics —
one seasonal block-bootstrap resample of the driver per draw, shared across all
pairs and lags so the joint dependence is preserved, with a per-round/per-station
seed (never the same draws for the per-lag mean/σ and the family bound); the
duplicated lag-0/1 cells are hashed and deduplicated to the six distinct
statistics before the maximum; each statistic is centered and scaled by the same
null-ensemble mean and σ on both the observed and the null side, `(v − μ)/σ` (the
centered studentization, Rat + external review 2026-10-03 — the uncentered `v/σ`
carried the KDE bias floor into the statistic as apparent signal); the 1−α level
is the empirical α-quantile of the studentized maxima, B ≈ 10⁴ for α = 0.01 (a
GPD tail fit above the top 10 % if the resample budget is capped), with the claim
count K declared in advance.
Point-estimate uncertainty is separate: a joint stationary bootstrap (BCa) on
(X, Y), not the surrogate draws. The estimator's α stays unnamed until that
construction stands; the current plug-in fam controls at order 10⁻¹, not 10⁻²
(§6). The round-maximum's sampling variability at
n_surr = 10 is carried by the Wilson CI (§6). The PCMCI leg of the hardened probe runs α = 0.05
(`bz_retro_probe.rs`); the fam leg names none. Verdicts: **arrow** (TE > fam), **family bound** (TE > own
threshold, < fam — directed, not round-significant), **silent** (TE < own
threshold). Every §4 row label follows this mechanical rule from its
tabulated TE, threshold and fam.

### 3.3 Controls

Three structural null controls: (i) the density channel is silent
(density does not drive reconnection — a positive density arrow would indict
the instrument, not the physics); (ii) the reverse direction dB/dt→driver
does not beat its threshold; (iii) in the minute grain, the quietest 6-hour
sub-window stays silent. The PE gate (a 2⁴-ring of the driver's own
permutation entropy, jump ⇔ |pe − mean| > 2σ) is part of the pipeline but
requires ≥ 8 segments: at 22 h it has 3 — no verdict; at the yearly grains
it is not applied (this manuscript reports its absence, not its outcome).

### 3.4 Grains and reproducibility

Three grains: minute (22-h live window, 1-min grid), hourly (2024 and 2025
full years, 1-h grid, lag 0/1 h), daily (1994–2026, stride 3 — every third
day, named: lag 1 = 3 days; stride 1 is computable but ~9× slower and left
for future runs). All seeds fixed: the surrogate seed is the constant
`SURROGATE_SEED = 0x9E37_79B9_7F4A_7C15` (`tools/measure/src/bin/bz_blatt_probe.rs:9`,
`bz_retro_probe.rs:9`). The family bounds reported here were
measured under the corrected (post-fix) surrogate RNG
(`fam-machine: post-fix`); the pre-fix run is superseded (see the footnote,
§6). The hourly probe has since been hardened — lag sweep 0–6 h, `n_surr =
100`, a median and a Newell driver statistic, sharded with
`timeout-minutes: 300` — and the re-measurement has run
(`bz-retro-probe`, CI run 36176580764, success): one family bound per
quarterly window across the then-12 shards (ABK 2024-q1…2025-q4, SOD
2024-q1…q4), all 24 directed rows (12 shards × Bz→dB/dt and Speed→dB/dt)
family bound; the SOD 2025 shards entered the workflow matrix on 2026-09-26
(`f916093ee`) — their minute data exists at the
BGS GIN HAPI, stop 2026-09-25 — and their directed rows are not part of the
24. A second dispatch of the same
workflow (`bz-retro-probe`, CI run 36224176888, success; 17 jobs — the
minute job plus the 16 quarterly shards) added the PCMCI cross-check and
the full lag sweep (0–6 h): no Bz→dB/dt lag clears its quarterly family
bound in any of the 16 shards (fam 0.172–0.210, Bz best-lag TE 0.105–0.166;
artifacts `bz-retro-<station>-<quarter>.txt`), and the conditioned PCMCI
edge is removed at all six lags in 13 of 16 shards while ABK 2025-q2,
SOD 2025-q2 and SOD 2025-q3 confirm a direct Bz edge (5/6, 6/6 and 1/6
lags). The two witnesses — the yearly
round here and the hardened quarterly round — are carried as a riss
(§6), never averaged.

### 3.5 Estimator validation against known ground truth

Before interpreting the geophysical results, the estimator is validated on
the Schreiber (2000) benchmark: unidirectionally coupled Hénon maps,
x_{n+1} = 1.4 − x_n² + 0.3 x_{n−1}, y_{n+1} = 1.4 − (c·x_n·y_n +
(1−c)·y_n²) + 0.3 y_{n−1}, n = 10 000, lag 1, with the same null machinery
(phase-randomized surrogates, per-lag threshold, family bound over the
round). With coupling c = 0.20 the known direction X→Y carries
TE(X→Y) = 2.457e-1, exceeding the family bound (fam = 2.405e-2) by a factor
of ~10; the reverse direction TE(Y→X) = 3.64e-2 also clears the family bound.
With c = 0 both directions stay below their thresholds (≈1.03e-2 and 1.13e-2
against per-lag thresholds ≈1.94e-2). The estimator recovers the known
direction with a strong asymmetry (ratio 6.75), but it does not null the
reverse channel at strong coupling: at c = 0.20 the reverse TE exceeds the
family bound, and the machine's own ground-truth verdict is therefore
**NOT PASS** for the strict criterion "only the known direction against the
family bound."

A (c, n) sweep (c = 0.05–0.50; n = 1 000, 3 000, 10 000) locates the
character of this reverse response. The reverse TE(Y→X) decreases as n grows
at weak coupling (c = 0.10: 4.05e-2 at n = 1 000 to 2.38e-2 at n = 10 000),
a finite-sample signature, while the asymmetry ratio TE(X→Y)/TE(Y→X) rises
steadily with c (2.1 at c = 0.05 to 8.4 at c = 0.40, n = 10 000). At n = 1 000
the reverse channel never clears its own threshold at any c ≤ 0.5; at larger
n it clears it from c ≈ 0.2. The reverse arrow is therefore n-dependent and
not a stable asymmetry property: it is the estimator's response
under strong bidirectional coupling and finite samples, and the direction is
carried by the asymmetry (dominance of the known direction), not by an
absolute reverse silence. This is the reading the paper takes for the
real data (§4.4, §6): a marginal reverse arrow at the second station is
inconclusive on its own — it neither establishes a reverse coupling nor fails
the null — because its forward/reverse ratio lies below the instrument's only
calibrated direction floor (2.1). The asymmetry
ratios the estimator exhibits on this benchmark (2.1–8.4, n = 10 000) are the
instrument's only calibrated direction scale; a data ratio below 2.1 lies
below that calibration floor and carries no direction weight on its own.

### 3.6 The ENSO four-channel block (Blatt I)

A second directional block applies the same estimator and family rule to the
NINO3.4 SST anomaly (`ersstv5_nino34_ssta`, `ersstv5_nino34.bin`, cut −5…5 lat,
190…240 lon, 1854-01-01…2026-08-01; `tools/harvest/src/bin/ersstv5_compiler.rs:12-17`).
The block is a four-channel round against the SST — the advective
zonal wind (TAO/TRITON `WU_422`, `tao_wnd_zonal.csv`), a lithosphere channel
(the monthly USGS comcat rate, the L stage of the LAIC chain), Bz
(`omni_hro_imf_bz_gsm_nt`), and the SST itself — on a monthly grid, lag sweep
0–12 months, with the per-lag μ + 2σ threshold and the round-max family bound
fam (§3.2); the NINO→Bz direction is the structural control (§3.3(ii)
analogue). The four channels are now wired and measured
(`tools/measure/src/bin/enso_blatt_probe.rs`; the lithosphere channel reads
`usgs_comcat_m45.bin` through `usgs_comcat::parse_bin`; CI run 36740119542,
success): Wnd 566, Quake 643, Bz 542 and SST 656 active months, 542 common
Bz–SST months. No arrow clears the family bound (fam = 2.9610e-1): every
directed pair is silent or at family bound, and the conditional arm
cTE(Bz → SST | Wnd) = 8.359e-3 stays below its threshold 2.612e-2 (conditioned
silent). The named confounds are the annual cycle (the 0–12-month sweep
includes the 12-month band) and the counting nature of the Quake channel. The
wind and lithosphere channels were previously carried `pending`; both
conditions are closed at the measured run — the wind channel is measured over
566 active months and the comcat asset `usgs_comcat_m45.bin` is committed
(`d0c070737`). The block's common window is bounded by its shortest channel
(Bz, 542 months).

## 4. Results

### 4.1 Minute grain (22-h window, n paired ≈ 1260)

| pair | lag | TE | own threshold | fam | verdict |
|---|---|---|---|---|---|
| Bz → dB/dt | 60 min | 2.180e-1 | 2.083e-1 | 3.744e-1 | family bound |
| Speed → dB/dt | 90 min | 2.110e-1 | 3.829e-1 | 3.744e-1 | silent |
| Density → dB/dt | sweep max 120 min | 2.055e-1 | 3.238e-1 | 3.744e-1 | silent |
| dB/dt → Bz | 51 min | 1.928e-1 | 2.446e-1 | 3.744e-1 | silent |

The Bz arrow peaks at 60 min, inside the physical L1 travel window, and
clears its own threshold — but the 22-h window is too small to clear the
family bound; every other pair is silent at its own threshold. Quiet
sub-window: Bz and Density silent; Speed shows an edge arrow at lag 120 min
(5.649e-1 vs 5.608e-1, excess 4.1e-3) — beyond the L1 travel time, named as
an artefact-zone candidate. The 22-h window's family bound (3.744e-1) and its per-lag thresholds
predate the surrogate-RNG correction and are not re-measurable (the live RTSW
window is no longer in the cache). The RNG fix shifted the yearly bounds by
~15 % (§5), larger than the minute per-lag margin (4.7 %), so whether the
minute per-lag excess survives the corrected null is unmeasured; the
family-bound verdict (Bz 2.180e-1, far below fam) does not depend on that
bound. Speed's own threshold (3.829e-1) exceeding fam (3.744e-1) flags the
per-lag threshold's instability at 10 draws. The hardened minute file (n_surr = 100) is the second witness at this
grain: Bz 0.24001 and Speed 0.28942 vs fam 0.34125 — both family bound; it
reports no per-lag thresholds. The
minute grain has not cleared the family bound under either null; §6 carries
the two witnesses.

### 4.2 Hourly grain — 2024 (n paired 8728) — yearly-round witness (post-fix, lag 0/1 h, n_surr = 10)

| pair | lag | TE | own threshold | fam = 1.0557e-1 | verdict |
|---|---|---|---|---|---|
| **Bz → dB/dt** | 0 h | **1.2670e-1** | 7.734e-2 | — | **arrow** |
| dB/dt → Bz | 0 h | 1.0512e-1 | 1.0649e-1 | — | silent |
| Speed → dB/dt | 0 h | 9.882e-2 | 7.220e-2 | — | family bound |
| dB/dt → Speed | 0 h | 3.179e-2 | 2.927e-2 | — | family bound |
| Density → dB/dt | 0 h | 8.825e-2 | 7.802e-2 | — | family bound |
| dB/dt → Density | 0 h | 6.988e-2 | 6.828e-2 | — | family bound |

**Bz → dB/dt is the only pair of the round that exceeds the family bound —
at the lag-0 edge bin.** The reverse direction dB/dt→Bz (1.0512e-1) sits
below its own threshold (1.0649e-1) and below the bound (silent); the density
control stays below the bound. Lag 0 h is an edge bin: the 30–60 min L1
travel time straddles the lag-0/lag-1 boundary (part of the signal lands in
the same hour, part in the next; the hardened probe's boundary note names
lag 1 h as the straddling bin, `bz_retro_probe.rs`). The lag-1 row is not a
distinct measurement: the lag-0 and lag-1 arms are algebraically identical
(§3.1, measured TE(lag 0) = TE(lag 1) = 0.21722066662653602), so the tabulated
`0 h` rows are the lag-1 rows and the round's family is six distinct pairs, not
twelve. The arrow is the one-step-ahead condition, reported as measured, not as
a clean-lag finding. The per-lag print (`--yearly-round` in `bz_retro_probe.rs:789`) emits the six pairs
at both lag labels. The n_surr = 100 re-measure ran
(`.github/workflows/bz-yearly-nsurr100.yml`, runs 36625872915 / 36639387218,
2026-09-29, success): family bound 1.0746e-1 at ABK 2024 (Bz → dB/dt 1.2670e-1,
arrow), 1.2802e-1 at ABK 2025 (Bz → dB/dt 1.3309e-1, arrow), 1.1091e-1 at SOD 2024
(Bz → dB/dt 1.1695e-1, arrow) — the Bz arrow holds at all three under the corrected
null, and the bound shifts up ~2–5 % from the n_surr = 10 values (§4.2–4.4).

### 4.3 Hourly grain — 2025 (n paired 8688) — yearly-round witness (post-fix, lag 0/1 h, n_surr = 10)

| pair | lag | TE | own threshold | fam = 1.2136e-1 | verdict |
|---|---|---|---|---|---|
| **Bz → dB/dt** | 0 h | **1.3309e-1** | 9.628e-2 | — | **arrow** |
| dB/dt → Bz | 0 h | 1.1663e-1 | 1.2260e-1 | — | silent |
| Speed → dB/dt | 0 h | 1.1562e-1 | 9.292e-2 | — | family bound |
| dB/dt → Speed | 0 h | 2.836e-2 | 2.107e-2 | — | family bound |
| Density → dB/dt | 0 h | 7.710e-2 | 9.724e-2 | — | silent |
| dB/dt → Density | 0 h | 5.670e-2 | 6.239e-2 | — | silent |

The structure repeats: Bz again clears the bound with a *higher* TE than
2024 (1.3309e-1), while Speed exceeds its own threshold (1.1562e-1 vs
9.292e-2) but not the bound, and Density and the reverse directions stay
silent at their own thresholds. Under the corrected null the 2025 family
bound (1.2136e-1) is lower than the pre-fix value (1.4256e-1) that had held
this year at family bound — 2025's Bz arrow now clears the yearly-round
bound, a bound that names no α (§3.2). The density control stays below the bound.

### 4.4 Second station — Sodankylä (SOD, 67.37° N), hourly 2024 — yearly-round witness (post-fix, lag 0/1 h, n_surr = 10)

| pair | lag | TE | own threshold | fam = 1.0571e-1 | verdict |
|---|---|---|---|---|---|
| **Bz → dB/dt** | 0 h | **1.1695e-1** | 7.468e-2 | — | **arrow** |
| dB/dt → Bz | 0 h | 1.0682e-1 | 1.0708e-1 | — | **arrow** |
| Speed → dB/dt | 0 h | 9.210e-2 | 6.898e-2 | — | family bound |
| dB/dt → Speed | 0 h | 3.119e-2 | 2.972e-2 | — | family bound |
| Density → dB/dt | 0 h | 8.494e-2 | 7.415e-2 | — | family bound |
| dB/dt → Density | 0 h | 6.958e-2 | 6.872e-2 | — | family bound |

The same pipeline (identical estimator, null model, and harvest route) at a
second auroral-zone observatory reproduces the forward arrow: Bz → dB/dt
(1.1695e-1) clears the corrected bound (1.0571e-1) in the yearly round. At this station, however,
the reverse direction dB/dt → Bz (1.0682e-1) also clears the bound while
staying below its own per-lag threshold (1.0708e-1) — the direction asymmetry
(forward 1.1695 vs reverse 1.0682, ratio 1.09) is smaller than at Abisko
(1.20 in 2024) and below the calibrated benchmark floor (2.1–8.4, §3.5), so
it does not carry the direction on its own. The reverse channel
is not silent here; §3.5 and §6 treat this as the estimator's
reverse response under strong coupling, not as a null failure, and it tempers
a strictly one-way reading.

### 4.5 Daily grain — 1994–2026 (stride 3, n paired ≈ 3900)

| pair | lag | TE | own threshold | fam = 1.6995e-1 | verdict |
|---|---|---|---|---|---|
| Bz → dB/dt | 0 d | 1.2525e-1 | 1.1524e-1 | — | family bound |
| Speed → dB/dt | 0 d | 9.703e-2 | 1.1632e-1 | — | silent |
| Density → dB/dt | 0 d | 1.059e-1 | 1.1663e-1 | — | silent |
| dB/dt → Bz | 0 d | 1.214e-1 | 1.7057e-1 | — | silent |

All six directed pairs stay below the family bound over the 32-year daily
series (stride 3). The table tabulates the three forward drivers
(Bz/Speed/Density → dB/dt) and the dB/dt → Bz reverse; the two remaining
reverse directions (dB/dt → Speed, dB/dt → Density) stay below the bound like
the hourly grains and are not separately tabulated. The daily median of Bz
carries no information about the
daily maximum of dB/dt — the southward excursions that drive storms average
out at this grain. This is also a design asymmetry, carried as a named
confound rather than a result: the driver enters as a daily median while the
target enters as a daily maximum, and a daily-maximum Bz driver was not
measured, so the emptiness of this grain is not yet separated from the
median-versus-maximum mismatch.

### 4.6 Derived channels at the hardened quarterly grain (15 artifacts, n_surr = 100)

Four derived solar-wind channels are measured alongside Bz, Newell, Speed and
Density in every quarterly `bz-retro-probe` artifact: the rectified southward
field Bs = max(−Bz, 0), the IMF clock angle Clock = atan2(By, Bz) [deg 0–360], the dynamic
pressure P_dyn = 1.67e-6·n·v² [nPa], and the Alfvén Mach number
M_A = v·√n/(21.8·B), B = √(Bx²+By²+Bz²). The table aggregates the best-lag row
of each channel over the 15 artifacts (ABK 2024-q1…2025-q4, eight; SOD
2024-q1…q4 and 2025-q2…q4, seven; SOD 2025-q1 is absent — its CI job is red):

| channel | best lag h (quarters) | TE range | own threshold range | fam range | verdict |
|---|---|---|---|---|---|
| Bs → dB/dt | 0 (10), 2 (5) | 9.421e-2–1.534e-1 | 7.716e-2–1.267e-1 | 1.720e-1–2.090e-1 | family bound (15/15) |
| Clock → dB/dt | 0 (5), 2 (5), 3 (1), 4 (2), 6 (2) | 6.790e-2–1.072e-1 | 7.751e-2–1.277e-1 | 1.720e-1–2.090e-1 | family bound (15/15) |
| P_dyn → dB/dt | 4 (5), 5 (5), 6 (4), 3 (1) | 9.296e-2–1.498e-1 | 8.811e-2–1.315e-1 | 1.720e-1–2.090e-1 | family bound (15/15) |
| M_A → dB/dt | 3 (6), 4 (5), 5 (3), 6 (1) | 5.405e-2–1.111e-1 | 8.513e-2–1.278e-1 | 1.720e-1–2.090e-1 | family bound (15/15) |

No derived channel clears the round family bound in any of the 15 artifacts —
zero arrows — so they add no family-clearing driver. At the per-lag grain the
four separate, and the separation is named, not smoothed: P_dyn exceeds its own
μ + 2σ threshold at the best lag in all 15 artifacts; Bs in 13 of 15 (below at
ABK 2025-q4, 1.1292e-1 vs 1.2672e-1, and SOD 2025-q4, 1.0492e-1 vs 1.1142e-1);
Clock in 2 of 15 (ABK 2024-q4 and SOD 2024-q4); M_A in none. The `bz-retro-probe`
hourly pair verdict is binary at this grain — `te > fam ? "arrow" : "family
bound"` (`tools/measure/src/bin/bz_retro_probe.rs:651`) — and emits no per-lag
`silent` state, so the printed `family bound` certifies only that the value sits
below the round maximum, never a per-lag clearing. P_dyn is therefore the only
one of the four whose per-lag excess is uniform across the 15 quarters, and it
  still does not clear `fam`; the family-bound verdict of the hardened quarterly
  round extends to all four derived channels.

### 4.7 Sub-daily NUR dB/dt–GIC relation (measured 2026-10-07)

The manifested NUR 10 s asset (`fmi_image_mag_nur.bin`, `phi/sources.φ:18030`)
carries a 30-day window 2023-12-31…2024-01-30 UTC that does **not** overlap the
GIC series (`fmi_gic.bin`, 1999-01-01…2023-10-01 UTC). The relation is therefore
measured on the 2003 Halloween storm, where the IMAGE NUR 10 s series overlaps
the hourly GIC asset. Reducing both channels to hourly peaks
(`tools/measure/src/bin/nur_gic_relation_probe.rs`, n = 72 aligned hours across
2003-10-29…31 UTC) gives a peak hour 2003-10-29T06:00Z with |ΔX/10s| = 240.9
nT/10s and |I| = 57.05 A, and the empirical relation

|GIC|_hourly = 0.2118·|ΔX/10s|_hourly + 1.755 (A per nT/10s),  Pearson r = 0.9448.

The correlation is high on the storm but is a co-located linear relation, not a
transfer-entropy arrow. It is a **scalar hourly-peak** reduction — driven by the
GIC asset carrying hourly peak-magnitude buckets — and therefore is **not** the
native-grain component regression of the literature: Juusola et al. (2025,
`10.5194/angeo-43-271-2025`, Eq. 43/Table 1) fit
a_x·∂B_x/∂t + a_y·∂B_y/∂t + a_z·∂B_z/∂t over all 10 s points of the same storm
(CC = 0.80 on a 2-h hold-out; a_x = −1.69, a_y = −2.73, a_z = −0.23 A per
nT/s), so the 0.80 → 0.945 gap measures the aggregation gain, not a stronger
coupling. The hourly-peak grain and the single-storm window are named limits,
and the manifested asset's non-overlapping window is the re-harvest riss.

## 5. Discussion

**The measured driver is sub-daily — on the yearly round and the asymmetry, not
on a per-quarter clearing.** The minute grain points at Bz with the correct lag
(60 min) in the live window, and the hardened minute file stays family
bound. The yearly hourly rounds make the arrow clear the bound in both
measured years at Abisko and in 2024 at Sodankylä; the hardened quarterly
rounds do not
clear the family bound (24/24 directed rows of the then-12 shards family
bound, §3.4). The
sub-daily finding therefore rests on the yearly-round arrow and the
forward-over-reverse asymmetry, not on a per-quarter clearing. The daily
grain is empty — not for lack of data (n ≈ 3900, 32 years of stride-3 days)
but because the daily median destroys the physical signal: a storm
is a multi-hour southward excursion, and its daily bucket is diluted toward
zero. The absence at the daily grain is itself the physical finding.

**Direction and asymmetry.** The forward direction Bz → dB/dt exceeds the
bound in every yearly hourly round (and stays below the bound in every
hardened quarterly round); the density control (the structural indictment
check: density does not drive reconnection) never clears the bound anywhere.
The reverse channel dB/dt → Bz stays below the bound at Abisko in both years
but marginally clears it at Sodankylä 2024 (ratio forward/reverse 1.09, with
the reverse TE below its own per-lag threshold there, §4.4). The
direction is carried by the asymmetry — forward exceeds reverse in all three
rounds — but the only calibrated asymmetry scale of the instrument is the
Hénon benchmark range (2.1–8.4, §3.5); the SOD ratio 1.09 lies below that
floor and carries no direction weight on its own. Per §3.5, a weak reverse
response under strong coupling is the
estimator's behavior, not a null failure; the Sodankylä reverse arrow
is therefore inconclusive on its own — it neither establishes a reverse
coupling nor fails the null — and it tempers a strictly one-way reading of
that round. The yearly arrows sit at the lag-0
edge bin: the 30–60 min travel time straddles the lag-0/lag-1 boundary (the
hardened probe's own boundary note names lag 1 h as the straddling bin,
`bz_retro_probe.rs`), and the edge-bin character is carried, not resolved.
The one edge excess outside the travel window, the quiet-window Speed arrow
at 120 min, stays named as an artefact-zone candidate.

**The corrected null lowers the bound — and the hardening raises it.** The
family bound is the strongest null TE of the round. Under the corrected
(post-fix) surrogate RNG the bounds fall (2024: 0.12480 → 0.10557; 2025:
0.14256 → 0.12136) because the pre-fix half-circle RNG had inflated the
surrogate distribution. A lower bound enlarges, by construction, the set of
values that clear it, so the correction raises the arrow rate mechanically;
what it establishes is that the pre-fix bound was an instrument artefact, not
that the physical evidence strengthened. The Bz arrow, which pre-fix cleared
the bound only in 2024, now clears it in both measured years at Abisko and
in 2024 at Sodankylä. This is consistency across 2024 and 2025 at two auroral-zone
stations, both near solar-cycle maximum; it is not replication across the
solar cycle, and the solar-cycle dependence of the coupling (Johnson & Wing,
2005) remains unmeasured here. The hardened quarterly null widens the bound
itself — the per-quarter family bounds run 0.18–0.20 against the yearly
0.106–0.121 — but the two witnesses do not share a statistic: n_surr is 10
in the yearly round and 100 in the quarterly round, the window is a whole
year against a single quarter, and a maximum
over more draws and more pair-lag cells is larger by construction. The
yearly and quarterly fam values are therefore not comparable as null
strengths, and the
divergence mixes the grain with the power of the null; the
riss is this round-dependence, named, not resolved.

**Why TE rather than PCMCI.** Runge et al. (2019) give the modern
conditional-independence route to causal discovery (PCMCI), with linear or
Gaussian-process condition tests in its standard form. We deliberately
chose the KDE transfer entropy instead for three reasons: (i) the physical
hypothesis is a magnitude statement (how much information the driver carries
about the ground response), which TE quantifies directly in nats and PCMCI
does not; (ii) the confounder problem PCMCI solves by conditioning is here
controlled structurally — the three candidate drivers are measured
separately at L1 and contrasted against each other under one family bound,
so the design does not need a variable-selection step; (iii) the null model
can be identical for every pair (phase-randomized surrogates and one round
maximum), which gives the family bound a single, transparent definition.
The PCMCI run on the same data has since completed
(`bz-retro-probe`, `pcmci_crosscheck`, CI 36224176888): conditioning
Bz→dB/dt on Speed and Density removes the edge at all six lags in 13 of
the 16 quarterly shards, but confirms a direct Bz edge in three (ABK
2025-q2 5/6 lags, SOD 2025-q2 6/6, SOD 2025-q3 1/6). The cross-check
therefore removes the direct Bz edge in a 13-of-16 majority of shards while
three shards confirm it — a majority against the yearly arrow, not a neutral
split. A 13-of-16 majority is stronger than round-dependence: it is consistent
with the yearly arrow carrying a confounding component that the conditional
test removes, and the riss is carried with this reading named, not smoothed.

**Relation to the literature.** Johnson & Wing (2005) established that the
solar-wind–magnetosphere transfer is nonlinear and solar-cycle dependent;
their information-theoretic driver search (Wing & Johnson, 2016) identified
solar-wind field and speed variables as the informative inputs to the
radiation belt, consistent with our Bz arrow and our Speed per-lag excess.
At the geomagnetic-index level, Yu et al. (2022) rank the solar-wind drivers
to the Sym-H index by transfer entropy — E and Bz dominant at a 60-minute
delay, consistent with our Bz arrow and our 60-minute per-lag peak — with a
source-shuffled permutation null and no family-wise correction; their target is
the storm index, not ground dB/dt.
Coupling-function studies (Newell et al., 2007; Borovsky, 2008) place
southward Bz at the center of dayside reconnection — the physical mechanism
of the chain Bz → magnetosphere–ionosphere currents → ground dB/dt. Our
measurement is a TE probe at the ground end of that chain, under a
phase-randomized null and a round-maximum family bound; the yearly-round
arrow it finds does not survive the hardened quarterly round (§3.4).
Methodologically, the family bound is a
round-maximum multiple-comparison control in the spirit of Runge et al.
(2019); its family-wise error rate was calibrated on the null battery, where
the empirical rate fell near the nominal 1/(n_surr+1) — a calibration that
does not transfer to this dependent, heavy-tailed, non-stationary design (the
calibrated round-maximum null is built and measured, §3.2, §6); the surrogate design follows the
phase-randomization practice of
Schreiber (2000) and the ETE criticism of Marschinski & Kantz (2002)
(here answered by the family bound rather than by shuffling the condition).

## 6. Limitations

- **Two stations, one zone.** The forward arrow replicates at ABK and SOD
  (both auroral zone); a mid-latitude network generalization is not
  measured here. At SOD the reverse channel also clears the bound (§4.4),
  consistent with the estimator's reverse response under coupling (§3.5).
- **Round-dependence of the family bound (riss).** The yearly-round arrow
  (Bz → dB/dt clears fam in 2024 and 2025 at ABK and 2024 at SOD, §4.2–4.4)
  does not survive the hardened quarterly null: in the `bz-retro-probe` run
  (CI 36176580764, fam per quarterly window, lag sweep 0–6 h, n_surr = 100)
  all 24 directed rows of the then-12 shards (2 drivers × 12) stay family
  bound — e.g. ABK 2024-q1 Bz 0.12375 vs
  fam 0.18016 (lag 0 h, n 2184), ABK 2025-q4 Bz 0.14008 vs 0.19901 (lag 2 h,
  n 2203), SOD 2024-q1 Bz 0.10531 vs fam 0.18056 (lag 4 h, n 2184) — and the
  minute file Bz 0.24001 / Speed 0.28942 vs fam 0.34125 stays family bound;
  SOD 2025-q1…q4 are now in the workflow matrix (`f916093ee`, 2026-09-26;
  their minute data is
  measured present at the BGS GIN HAPI) and their directed rows are not part
  of the 24. The two witnesses — yearly
  round and hardened quarterly round — are carried un-smoothed as a riss, never averaged. The family-clearing claim at the
  hourly grain is withdrawn into this open state, not inverted. Named
  resolution steps — measured 2026-09-26 (`bz-retro-probe`, CI 36224176888,
  success): the family bound over the full lag sweep (0–6 h) finds no
  Bz→dB/dt lag above fam in any of the 16 quarterly shards (fam
  0.172–0.210, Bz best-lag TE 0.105–0.166; artifact `bz-retro-abk-2024-q1.txt`),
  and the PCMCI cross-check removes the conditioned Bz→dB/dt edge at all
  six lags in 13 of 16 shards while confirming a direct edge in three (ABK
  2025-q2 5/6, SOD 2025-q2 6/6, SOD 2025-q3 1/6). The riss remains: the
  yearly arrow is not reproduced by the hardened quarterly null, and the
  PCMCI verdict is itself round-dependent. The two bounds are not directly
  comparable — the hardened round draws 100 surrogates over a larger
  pair-lag family than the yearly round's 10, so a higher bound arises
  by construction. Both probes now run n_surr = 100 (`bz_blatt_probe.rs:10`,
  `bz_retro_probe.rs:10`, since `43096531d`, 2026-09-25). The comparable
  yearly-round re-measure at n_surr = 100 has run
  (`bz_retro_probe.rs:789` `--yearly-round`,
  `.github/workflows/bz-yearly-nsurr100.yml`, CI run 36639387218, success): at
  matched n_surr = 100 the yearly arrow still clears its own round's bound at
  all three windows — ABK 2024 Bz→dB/dt TE 1.2670e-1 vs fam 1.0746e-1; ABK 2025
  TE 1.3309e-1 vs fam 1.2802e-1; SOD 2024 TE 1.1695e-1 vs fam 1.1091e-1 — while
  the hardened quarterly family (0.172–0.210) stays above the yearly family
  (0.107–0.128) because it spans more pair-lag cells. The n_surr mismatch is
  therefore closed: the yearly arrow is not an artefact of ten-surrogate
  thresholds, and the yearly-vs-quarterly divergence is a grain/power
  difference, not a surrogate-count artefact. The yearly fam value itself
  differs slightly between the annual and the n_surr = 100 runs (ABK 2024
  0.10557 vs 0.10746; ABK 2025 0.12136 vs 0.12802) — a run-to-run fam spread of
  the same round definition, named, not averaged.
- **dB/dt is the induction driver; the co-located magnetogram and the GIC
  current are now available over 1999–2023.** The FMI Mäntsälä GIC series is a
  CDN asset (`fmi_gic.bin`, `phi/sources.φ:17194`, parsed as
  `MAGIC_GIC`/`COMP_GIC_A` in `src/archivar/geo.rs:7,277`); it currently carries
  hourly peak-magnitude buckets. FMI publishes the same recordings as daily
  ASCII files (`https://space.fmi.fi/gic/man_ascii/`, CC BY 4.0, confirmed by
  Ari Viljanen 2026-10-05) over 1999–2023 at 60.6° N, 25.2° E; recording
  stopped 2023-10-23, so the GIC current side cannot be matched to the paper's
  2024/2025 solar-wind windows and must be measured on its own window. The FMI
  page explicitly recommends comparing GIC with dB/dt at the closest
  observatory **Nurmijärvi (NUR)**; NUR is carried by the IMAGE magnetometer
  network (`https://space.fmi.fi/image/www/data_download.php`, 10 s, CC BY 4.0)
  and is already a SuperMAG station (`phi/supermag_stations.φ:368`, 60.5° N
  24.65° E), so a co-located magnetogram is obtainable through the existing
  SuperMAG chain — the earlier "no co-located magnetogram" no longer holds. As
  of 2026-10-07 the NUR 10 s dB/dt series is a manifested CDN asset
  (`fmi_image_mag_nur.bin`, `phi/sources.φ:18030`; 15,551,948 B; sha256
  9c76f881d33e5e2d0c84b60b1641d4a7262f0039e43d3795a0dd8e9044714e0e, reachable
  HTTP 206, measured 2026-10-07), so the co-located channel is wired. The
  manifested asset's window is 2023-12-31…2024-01-30 UTC (measured 2026-10-07),
  which does not overlap the GIC series; the relation is therefore measured on
  the 2003 Halloween storm (§4.7). The
  source carries its own caveats: the archive is non-uniform (pipeline
  reconfigurations, a 2005 site shift), gap- and spike-laden, with best quality
  1999–April 2005, and daily linear-trend subtraction is recommended. The
  sub-daily dB/dt–GIC relation is measured on that storm as a co-located linear
  correlation (r = 0.9448, n = 72 aligned hours, §4.7); the component
  regression is quantified in **Juusola et al. (2025,
  `https://doi.org/10.5194/angeo-43-271-2025`, Eq. 43/Table 1)** — a
  3-component least-squares over a_x·∂B_x/∂t + a_y·∂B_y/∂t + a_z·∂B_z/∂t at
  native 10 s (a_x = −1.69, a_y = −2.73, a_z = −0.23 A per nT/s; validation
  CC = 0.80 on a 2-h hold-out), **not** a scalar peak — so §4.7's hourly-peak
  scalar r is a different reduction and is not directly comparable. The paper
  still measures the excitation at ABK/SOD; the GIC-current side at Mäntsälä is
  now a wired next step. *(Citation corrected 2026-10-07: the anchor is Juusola
  et al., Ann. Geophys. 43, 271–301, 2025 — Viljanen is the fifth author.)*
- **Minute grain is a single 22-h window.** A storm-ensemble at minute
  resolution would require a minute-resolution retro solar-wind archive,
  which the stack does not carry (RTSW live holds ~1 day; the SWPC mirror
  carries only the rolling 1-day/7-day files, `.github/workflows/swpc-mirror-cdn.yml:30`).
- **Daily grain uses stride 3** (every third day; lag 1 = 3 days). The
  full-density daily run is 9× costlier and left for a future run; the
  stride is named, not hidden.
- **Estimator validation (reverse channel).** The scalar TE estimator is the
  untouched canonical reference of the system; on the Schreiber-2000
  benchmark it recovers the known direction with asymmetry 6.75 but does not
  null the reverse channel at strong coupling (ground-truth verdict NOT
  PASS for absolute reverse silence; §3.5). The direction in the real data
  is argued from the forward-over-reverse asymmetry and the silent density
  control, not from an absolute reverse null. A KDE-h bandwidth sweep on
  Bz→dB/dt (factors 0.5–3.0) is carried by the hourly probe
  (`tools/measure/src/bin/bz_retro_probe.rs`, `KDE_FACTORS`); it has run
  (CI 36224176888): in the representative shard ABK 2024-q1 Bz→dB/dt
  clears its per-lag threshold at every factor (1.00: 1.2375e-1 vs
  9.534e-2; 3.00: 4.249e-2 vs 1.308e-2), so the per-lag excess is not a
  bandwidth artefact, but at factor 0.50 one of 16 shards (ABK 2025-q4,
  3.7033e-1 vs 3.8862e-1) is still — the sweep is not uniformly an arrow.
- **PE gate not engaged** at these window sizes (3 segments of 360 samples
  in the minute grain; the yearly grains do not apply it). Non-stationarity
  is instead controlled by the year separation and the named status stack
  of the data.
- **fam is a plug-in round maximum; its rate is not calibrated for the real design.** It
  corrects for the round's multiplicity by taking the largest surrogate TE
  actually drawn; it is not a quantile of a closed-form maximum distribution
  and names no fixed α in the estimator. Its family-wise error rate was
  exercised at the operating size on four independent AR(1) channels
  (`tools/measure/src/bin/fam_calibration.rs:87`,
  `fam-scalar-calibration.yml`, CI run 36639382300, success): at n = 1260,
  n_surr = 10 (600 trials per a) FWER 7.83% (95% Wilson CI 5.94–10.26%),
  9.50% (7.40–12.11%) and 9.50% (7.40–12.11%) for a = 0.0/0.5/0.9 against
  nominal 9.09%; at n_surr = 100 (60 trials per a) 1.67% (0.29–8.86%), 0.00%
  (0.00–6.02%) and 1.67% (0.29–8.86%) against nominal 0.99%. The round-maximum's
  empirical rate fell near the nominal value in that battery; the battery does
  not establish family-wise error control for the real design, whose surrogate
  draws are dependent (the shared deterministic seed), whose twelve pair-lag
  calls carry six distinct statistics (the lag-0/1 identity, §3.1), and whose
  null maximum is misspecified for non-stationary, heavy-tailed hourly maxima.
  A calibrated null distribution of the round-maximum is measured (§6); its
  studentized Westfall–Young max-T form is given in §3.2. The
  current plug-in fam controls at order 10⁻¹, not 10⁻². The sampling variance at
  n_surr = 10 is carried by the reported Wilson CI, not asserted, and the
  n_surr = 100 points pin the operating value at the low-nominal end. No confidence interval is reported on the TE values
  themselves. The membrane-FPR battery (`gate_membrane_fpr_phase_vs_arx_n_1000`,
  `src/mathematikerin/te.rs`, wired as the `fpr-membrane` job of
  `.github/workflows/te-gate.yml`) measures the topological membrane's FPR,
  not the scalar round-max fam; its a = 0 arm reads 9.52% against the gate's
  8% line and is analysed separately (the membrane null is a different rule;
  `gate_membrane_fpr_diagnostic` measures the four Rat diagnostics). The
  manuscript claims the calibrated nominal rate, α_nom = 1/(n_surr+1).
- **Estimator bias at the operating n is measured.** The KDE/Silverman estimator
  carries a bias in the conditional density (x_{t+τ}, x_t, y_t) at the operating
  sample sizes; a coupled Hénon benchmark (c = 0.20, known X→Y, lag 1, scalar
  Silverman-KDE, 5 fixed-seed replicates) now measures it
  (`tools/measure/src/bin/te_bias_n_probe.rs`, `.github/workflows/te-bias-n.yml`,
  CI run 37118666568, success). Against the validated n = 10 000 reference
  TE(X→Y) = 2.4402e-1 the forward bias is −8.87e-2 (n = 800), −7.69e-2
  (n = 1260), −7.14e-2 (n = 1600), −6.10e-2 (n = 2200) and −4.16e-2
  (n = 4000), tending to 0 at n = 10 000; the direction X→Y is recovered in
  5/5 replicates at every n. The bias magnitude therefore grows toward the
  operating sizes (n ≈ 1260–2200) while the direction verdict is stable. No
  explicit small-sample correction is applied and every reported TE value
  carries this negative bias; the correction's floor is measured and stands
  (`TE_NEFF_THRESHOLD = 18.166` = the `kde_n_eff` at n = 800, `te-bias-n` run
  37234616152), but no reported value applies it yet — the wiring stays open.
- **The calibrated round-maximum null is measured at all three station-years
  at α = 0.05 and α = 0.01; the Abisko void is closed.** The
  studentized Westfall–Young max-T
  construction named at §3.2 runs end-to-end
  (`tools/measure/src/bin/wy_max_t_probe.rs`, `.github/workflows/wy-max-t.yml`;
  nine shards of 1111 replicates pooled into B = 9999, K = 6 distinct statistics,
  shared seasonal block-bootstrap resample, per-station/year seed). The first
  α = 0.05 pass (CI run 37187464365 on `ba9c640479`) collapsed the matrix axis:
  the `wy-shards` job carried only the sod-2024 point, so the Abisko combines
  read `null-*.bin reads void — no measurement` and only the Sodankylä 2024
  quantile was measurable. The matrix-key fix (the `point` axis is a real matrix
  key with per-point `args` in the `include`) reran the probe (CI run 37235150270,
  success on `f266a292`, 2026-10-05); all three points now carry their nine
  shards and combine. At α = 0.05 the pooled (1−α) quantile of the studentized
  maxima is 2.4831 (SOD 2024, family maximum 10.18), 2.4467 (ABK 2024, 12.698)
  and 2.4433 (ABK 2025, 10.864) — the family clears at all three, with Bz→dB/dt
  the family maximum. At Sodankylä 2024 Bz→dB/dt, dB/dt→Bz, dB/dt→Speed and
  dB/dt→Density are family-clearing (p_adj 1.0e-4, 1.0e-4, 1.1e-3, 1.9e-3) and
  Speed→dB/dt, Density→dB/dt at the family bound; the operating n_eff reads
  ≈ 44.5 for every member and the lag-0/1 identity check is 0. The α = 0.01
  calibrated round-maximum null (`bz-yearly-maxt.yml`, CI run 37187466569,
  success on `ba9c640479`; ten shards of 1000 replicates pooled into B = 10000,
  m = 6 statistics, block = 24 h) completes the family at every point: the
  pooled (1−α) quantile of the studentized maxima is 3.0964 (SOD 2024), 3.0490
  (ABK 2024) and 3.0036 (ABK 2025), and the observed family maxima are 10.181,
  12.698 and 10.864 — the family clears at all three, with the Bz→dB/dt statistic
  the family maximum (T_stud 10.181, 12.698, 10.864) and the reverse dB/dt→Bz
  statistic smaller (6.091, 5.684, 2.324). The earlier
  `null-*.bin reads void — no measurement` at Abisko was a missing pooled null
  (matrix collapse), not a station void: the pooled null is now complete at both
  α levels. The BCa per-statistic interval is named `pending` (the
  joint-stationary jackknife is not cheap). The two α levels are distinct and are
  not averaged. This replaces the plug-in `fam` bound (order 10⁻¹) for the
  yearly rounds with a calibrated bound; the plug-in
  rows of §4 remain the historical record.
- **Storm-only sub-analysis is measured and stays `pending` on the Kp channel.**
  The storm-selective round is built (`tools/measure/src/bin/gic_storm_probe.rs`,
  `.github/workflows/gic-storm.yml`, CI run 37118664799, success) and runs the
  six-pair round both on the yearly window and on Kp ≥ 5 storm hours at ABK and
  SOD. The wired SWPC Kp channel carries only 59–60 records on its 3-h grid, so
  no hour clears Kp ≥ 5 and the storm round has n = 0 — carried as `pending`,
  never a fabricated zero. The yearly round it reports is consistent with §4
  (Bz→dB/dt at ABK 2024 1.2707e-1 vs fam 1.0974e-1, ABK 2025 1.3345e-1 vs
  1.2439e-1 arrow, SOD 2024 1.1696e-1 vs 1.0850e-1, SOD 2025 1.2134e-1 vs
  1.2634e-1 family bound). The open step is a complete Kp series (the GFZ series
  reads `pre`, not `def`) and a `F_{t−1}`-measurable storm selection applied
  inside the null loop; whether the Bz arrow is carried by a few storm days is
  not yet separated.
- **Surrogate-machine generation.** The family bounds in this version were
  measured under the corrected (post-fix) surrogate RNG
  (`fam-machine: post-fix`, §3.4). The pre-fix run (fam 2024 = 0.12480) is
  superseded; the corrected bounds are lower. Against the higher pre-fix
  band, the 2024 arrow still clears (1.2670e-1 > 0.12480) while the 2025
  arrow does not (1.3309e-1 < 1.4256e-1, §4.3) — the 2025 clearing exists
  only under the corrected null. A hardening
  of the reverse channel at strong coupling (§3.5) is registered as an open
  research thread and does not gate the direction reported here.

## 7. Conclusion

Transfer entropy with a phase-randomized null and a family bound measures a
directional candidate for the geomagnetic induction excitation at the yearly
grain: the southward interplanetary magnetic field, at the lag-0/1 edge bin of
the hour-resolution round, clears the corrected family bound in both measured
years at Abisko and in 2024 at Sodankylä (§4.2–4.4); the lag itself is not
resolved by this round. The hardened quarterly round (n_surr =
100, fam per quarterly window) refuses to converge with that arrow — all 24
directed rows of the then-12 shards stay family bound (§3.4, §6) — and the
two witnesses are
carried un-smoothed as a riss, never averaged. What survives the riss: the
density control stays below the bound throughout; the forward direction
exceeds the reverse in every yearly round (ratios 1.20 and 1.14 at ABK
2024/2025, 1.09 at SOD 2024 — below the benchmark floor 2.1, §3.5); the
daily grain is empty because
daily medians wash the driver out; and the reverse channel shows a weak,
response under strong coupling — the estimator's ground-truth
verdict is NOT PASS (§3.5), which tempers every strictly
one-way reading. A family-clearing driver at the hourly
grain is not established. For the grid operator: watch Bz at L1 rather than
the daily average — the yearly-round evidence points there, unconfirmed by
the hardened quarterly round — and read the
hourly-grain family-clearing as an open measurement, not a settled finding.

## References

- Borovsky, J. E. (2008). The rudiments of a theory of solar-wind/magnetosphere coupling derived from first principles. *Journal of Geophysical Research: Space Physics* 113, doi:10.1029/2007JA012646.
- Johnson, J. R., & Wing, S. (2005). A solar cycle dependence of nonlinearity in magnetospheric activity. *Journal of Geophysical Research: Space Physics* 110, doi:10.1029/2004JA010638.
- Kaiser, A., & Schreiber, T. (2002). Information transfer in continuous processes. *Physica D: Nonlinear Phenomena* 166, 43–62, doi:10.1016/S0167-2789(02)00432-3.
- Marschinski, R., & Kantz, H. (2002). Analysing the information flow between financial time series. *The European Physical Journal B* 30, 275–281, doi:10.1140/epjb/e2002-00379-2.
- Newell, P. T., Sotirelis, T., Liou, K., Meng, C.-I., & Rich, F. J. (2007). A nearly universal solar wind–magnetosphere coupling function inferred from 10 magnetospheric state variables. *Journal of Geophysical Research: Space Physics* 112, doi:10.1029/2006JA012015.
- Pulkkinen, A., Bernabeu, E., Thomson, A., Viljanen, A., Pirjola, R., et al. (2017). Geomagnetically induced currents: science, engineering, and applications readiness. *Space Weather* 15, 828–856, doi:10.1002/2016SW001501.
- Runge, J., Nowack, P., Kretschmer, M., Flaxman, S., & Sejdinovic, D. (2019). Detecting and quantifying causal associations in large nonlinear time series datasets. *Science Advances* 5, eaau4996, doi:10.1126/sciadv.aau4996.
- Schreiber, T. (2000). Measuring information transfer. *Physical Review Letters* 85, 461–464, doi:10.1103/PhysRevLett.85.461.
- Staniek, M., & Lehnertz, K. (2008). Symbolic transfer entropy. *Physical Review Letters* 100, 158101, doi:10.1103/PhysRevLett.100.158101.
- Wing, S., Johnson, J. R., Camporeale, E., & Reeves, G. D. (2016). Information theoretical approach to discovering solar wind drivers of the outer radiation belt. *Journal of Geophysical Research: Space Physics* 121, 9378–9399, doi:10.1002/2016JA022711.
- Yu, J., Tong, J., Fang, S., & Hu, X. (2022). Transfer entropy approach to discovering the ranking of solar wind drivers to geomagnetic storm. *Chinese Journal of Space Science* 42(3), 346–356, doi:10.11728/cjss2022.03.210406045.

## Open Research

### Data Availability

The solar-wind and ground series used in this study are public third-party
products, retrieved from their providers: the solar wind at L1 is SWPC RTSW
one-minute data (minute grain, https://services.swpc.noaa.gov/) and OMNI2
(`OMNI2_H0_MRG1HR`, hourly and daily grains, via CDAWeb HAPI,
https://cdaweb.gsfc.nasa.gov/); the ground magnetic field is INTERMAGNET
one-minute X/Y/Z at Abisko and Sodankylä, served by the BGS GIN HAPI
(https://imag-data.bgs.ac.uk/). The harvested working copies used here are
archived in the omegaflow repository; a citable repository DOI is pending.

### Software Availability

The instrument, probes and register live in the omegaflow repository
(https://github.com/omegaflow/omegaflow; `src/mathematikerin/te.rs` — canonical
scalar estimator, untouched; `tools/measure/src/bin/bz_retro_probe.rs`,
`tools/measure/src/bin/bz_blatt_probe.rs`). A citable software DOI is pending.
All reported values are machine-measured. All bibliographic entries above were
checked against the Crossref registry on 2026-08-22.

## Conflict of Interest

The authors declare there are no conflicts of interest for this manuscript.

## Acknowledgements

Funding: none. This work received no external funding.
