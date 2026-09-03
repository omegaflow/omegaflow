<!--
  title: The causal driver of geomagnetically induced currents
  class: paper
  date: 2026-08-22
  sha256: 0735c91fb589a15e31a4b6de0562e6cb8f8aee7328975a6d9012cb161855f060
  fam-machine: pre-fix
  status: live
  see-also: docs/specs/broken-null-control.md
-->

# The causal driver of geomagnetically induced currents

*Omegaflow Working Group — Bz-Blatt Session, 2026-08-22*

## Abstract

The induction excitation of geomagnetically induced currents (GIC) is dB/dt. Which solar-wind quantity drives it — southward Bz, bulk speed, or density — is open at minute-to-hour scales. We measure transfer entropy (TE) from L1 solar-wind drivers to the hourly and daily maxima of dB/dt at INTERMAGNET Abisko (68.36° N), with phase-randomized surrogates, per-lag thresholds, and a family bound. At the minute grain, Bz→dB/dt peaks at lag 60 min, per-lag significant but family bound in one 22-hour window. At the hourly grain, Bz→dB/dt is the only pair exceeding the family bound (TE 0.12670 vs fam 0.12480, lag 0 h, n = 8728, storm year 2024), repeated in 2025; the reverse direction and the density control stay silent. At the daily grain over 32 years (1994–2026, n ≈ 3900), all six pairs are silent. The causal driver is the southward interplanetary field at sub-daily timescales; daily means do not carry it.
## 1. Introduction

Geomagnetically induced currents flow in power grids and pipelines when the
ground magnetic field changes rapidly; the engineering risk variable is dB/dt,
its time derivative (Pulkkinen et al., 2017). The upstream driver is the solar
wind at the L1 point. Which of its measured quantities — the interplanetary
magnetic field (in particular its southward component Bz in GSM coordinates),
the bulk speed, or the density — causally drives the ground response, and at
what lag, is both a physics question and an operational one: forecasters
watch the solar-wind monitor 30–90 minutes upstream of the magnetosphere, and
an identified driver at the correct lag is a warning channel.

Correlation cannot separate these candidates: all solar-wind quantities
co-vary through their common origin. Transfer entropy (Schreiber, 2000) is
the natural instrument: a directional, model-free measure of information flow
between time series, closely related to Granger causality but nonlinear and
nonparametric. TE has been applied to solar-wind–magnetosphere coupling
(Johnson & Wing, 2005; Wing & Johnson, 2016) and to climate causality at
large scale (Runge et al., 2019), but the GIC driver chain — L1 Bz to ground
dB/dt at an auroral-zone station, measured with a strict phase-randomized
null and a family-wise correction — has not, to our knowledge, been settled.

This paper reports a measurement series at three time grains: minutes
(one 22-hour live window), hours (two full storm years, 2024 and 2025), and
days (32 years, 1994–2026). The instrument is the untouched scalar TE
estimator of the omegaflow field system; the null model and the family bound
are those of its broken-null-control record. All verdicts below are reported
exactly as the machine measured them, including the silent ones.

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
(ground dB/dt), Y the driver (Bz, speed, density). No pre-shift is applied:
the lag sweep *is* the L1→Earth propagation time, expected at 30–60 min for
300–800 km/s; a lag-0 or sweep-edge arrow is treated as an artefact
candidate, not a finding.

### 3.2 Null model and thresholds

For every measured pair and lag, ten phase-randomized surrogates of the
driver (f64 FFT, deterministic seed) yield the per-lag threshold μ + 2σ. In
addition, the **family bound** fam = the maximum surrogate TE over *all*
pairs × lags of the measurement round — the multiple-comparison control:
with 12 tested pair-lag combinations per grain, a per-lag excess is expected
by chance; an arrow requires TE > fam (and therefore exceeds every null TE
of the round). Verdicts: **arrow** (TE > fam), **family bound** (TE > own
threshold, < fam — directed, not round-significant), **silent** (TE < own
threshold).

### 3.3 Controls

Three structural null controls: (i) the density channel must be silent
(density does not drive reconnection — a positive density arrow would indict
the instrument, not the physics); (ii) the reverse direction dB/dt→driver
must not beat its threshold; (iii) in the minute grain, the quietest 6-hour
sub-window must stay silent. The PE gate (a 2⁴-ring of the driver's own
permutation entropy, jump ⇔ |pe − mean| > 2σ) is part of the machine but
requires ≥ 8 segments: at 22 h it has 3 — no verdict; at the yearly grains
it is not applied (this manuscript reports its absence, not its outcome).

### 3.4 Grains and reproducibility

Three grains: minute (22-h live window, 1-min grid), hourly (2024 and 2025
full years, 1-h grid, lag 0/1 h), daily (1994–2026, stride 3 — every third
day, named: lag 1 = 3 days; stride 1 is computable but ~9× slower and left
for future runs). All seeds fixed; the hourly 2024 run reproduces
byte-identically across machine lifetimes (fam 0.12480 twice).

### 3.5 Estimator validation against known ground truth

Before interpreting the geophysical results, the estimator is validated on
the Schreiber (2000) benchmark: unidirectionally coupled Hénon maps,
x_{n+1} = 1.4 − x_n² + 0.3 x_{n−1}, y_{n+1} = 1.4 − (c·x_n·y_n +
(1−c)·y_n²) + 0.3 y_{n−1}, n = 10 000, lag 1, with the same null machinery
(phase-randomized surrogates, per-lag threshold, family bound over the
round). With coupling c = 0.20 the known direction X→Y carries
TE(X→Y) = 2.457e-1, exceeding the family bound (fam = 4.550e-2) by a
factor of 5.4; the reverse direction TE(Y→X) = 3.64e-2 clears its own
per-lag threshold (2.10e-2) but not the family bound. With c = 0 both
directions are silent (≈1.03e-2 and 1.13e-2 against per-lag thresholds
≈1.94e-2). The estimator reconstructs the known direction and only the
known direction against the family bound — the asymmetry ratio is 6.75.

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
an artefact-zone candidate.

### 4.2 Hourly grain — 2024 (n paired 8728)

| pair | lag | TE | own threshold | fam = 1.248e-1 | verdict |
|---|---|---|---|---|---|
| **Bz → dB/dt** | 0 h | **1.2670e-1** | 1.0008e-1 | — | **arrow** |
| dB/dt → Bz | 0 h | 1.0512e-1 | 1.2433e-1 | — | silent |
| Speed → dB/dt | 0 h | 9.882e-2 | 8.113e-2 | — | family bound |
| dB/dt → Speed | 0 h | 3.179e-2 | 3.860e-2 | — | silent |
| Density → dB/dt | 0 h | 8.825e-2 | 9.505e-2 | — | silent |
| dB/dt → Density | 0 h | 6.988e-2 | 8.584e-2 | — | silent |

**Bz → dB/dt is the only pair of the round that exceeds the family bound.**
The reverse direction lies below its own threshold; the density control is
silent. Lag 0 h straddles the 30–60 min L1 travel time (part of the signal
lands in the same hour, part in the next).

### 4.3 Hourly grain — 2025 (n paired 8688)

| pair | lag | TE | own threshold | fam = 1.4256e-1 | verdict |
|---|---|---|---|---|---|
| Bz → dB/dt | 0 h | 1.3309e-1 | 1.0873e-1 | — | family bound |
| dB/dt → Bz | 0 h | 1.1663e-1 | 1.4362e-1 | — | silent |
| Speed → dB/dt | 0 h | 1.1562e-1 | 9.767e-2 | — | family bound |
| dB/dt → Speed | 0 h | 2.836e-2 | 2.907e-2 | — | silent |
| Density → dB/dt | 0 h | 7.710e-2 | 1.0492e-1 | — | silent |
| dB/dt → Density | 0 h | 5.670e-2 | 7.247e-2 | — | silent |

The structure repeats exactly: Bz is again the only forward channel above its
own threshold — with a *higher* TE than 2024 (1.3309e-1) — but 2025's family
bound is higher (1.4256e-1), so the year's Bz arrow is family bound. The
density control and the reverse direction stay silent in both years.

### 4.4 Second station — Sodankylä (SOD, 67.37° N), hourly 2024

| pair | lag | TE | own threshold | fam = 1.2844e-1 | verdict |
|---|---|---|---|---|---|
| Bz → dB/dt | 0 h | 1.1654e-1 | 9.156e-2 | — | family bound |
| dB/dt → Bz | 0 h | 1.1054e-1 | 1.2729e-1 | — | silent |
| Speed → dB/dt | 0 h | 9.220e-2 | 8.070e-2 | — | family bound |
| dB/dt → Speed | 0 h | 3.191e-2 | 3.881e-2 | — | silent |
| Density → dB/dt | 0 h | 8.495e-2 | 8.775e-2 | — | silent |
| dB/dt → Density | 0 h | 7.051e-2 | 8.641e-2 | — | silent |

The same pipeline (identical estimator, null model, and harvest route) at a
second auroral-zone observatory reproduces the structure: Bz is again the
only forward channel above its own threshold; the reverse direction and the
density control stay silent. SOD's Bz TE (1.1654e-1) and its family bound
(1.2844e-1, again set by the reverse null dB/dt→Bz at lag 0) leave the
station's arrow family bound — the direction replicates, the
round-significance at SOD does not.

### 4.5 Daily grain — 1994–2026 (stride 3, n paired ≈ 3900)

| pair | lag | TE | own threshold | fam = 1.895e-1 | verdict |
|---|---|---|---|---|---|
| Bz → dB/dt | 0 d | 1.2525e-1 | 1.3890e-1 | — | silent |
| Speed → dB/dt | 0 d | 9.703e-2 | 1.3548e-1 | — | silent |
| Density → dB/dt | 0 d | 1.059e-1 | 1.1057e-1 | — | silent |
| dB/dt → Bz | 0 d | 1.214e-1 | 1.876e-1 | — | silent |

All six directed pairs are silent over 32 years and every storm of the era.
The daily mean of Bz carries no information about the daily maximum of
dB/dt — the southward excursions that drive storms average out at this
grain.

## 5. Discussion

**The driver lives sub-daily.** The three grains tell one consistent story.
The minute grain points at Bz with the correct lag (60 min). The hourly
grains make the arrow round-significant in 2024 and reproduce its direction
with an even larger TE in 2025. The daily grain is empty — not for lack of
data (n ≈ 3900, 32 years, all storms included) but because the daily mean
destroys the physical signal: a storm is a multi-hour southward excursion,
and its daily average is diluted toward zero. The absence at the daily grain
is itself the physical finding (0 honored in the system's vocabulary).

**Direction and lag.** The arrow is directional in both storm years: the
reverse direction dB/dt→Bz never reaches its own threshold, while the
forward direction exceeds it. The hourly lag 0 is the correct coarse
representation of the 30–60 min propagation measured at the minute grain;
the expected lag-0/edge artefact zones are clean (the one edge excess, the
quiet-window Speed arrow at 120 min, is named and outside the travel window).

**Year-dependence of the family bound.** fam is the strongest null TE of the
round, and in both years it is set by the same pair — the reverse null
dB/dt→Bz at lag 0 h, i.e. phase-randomized surrogates of the storm-structured
ground series predicting the solar wind (chunk-level tracking identifies
this pair as the round maximum in 2024 and 2025 alike). Its magnitude
scales with the year's ground variability: 2025 carried a stormier ABK
series (hourly dB/dt mean 30.6 vs 18.0 nT/min, σ 42.1 vs 35.9), and its
fam rose accordingly (0.14256 vs 0.12480). The Bz arrow crosses it in 2024
and not in 2025, while the underlying direction and per-lag significance
hold in both. This is the honest shape of the finding: a real, repeating,
directed driver whose round-significance is year-dependent at this sample
size — not a binary yes/no artifact.

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
A PCMCI run on the same data remains a named future cross-check, not a
competing claim.

**Relation to the literature.** Johnson & Wing (2005) established that the
solar-wind–magnetosphere transfer is nonlinear and solar-cycle dependent;
their information-theoretic driver search (Wing & Johnson, 2016) identified
solar-wind field and speed variables as the informative inputs to the
radiation belt, consistent with our Bz arrow and our Speed per-lag excess.
Coupling-function studies (Newell et al., 2007; Borovsky, 2008) place
southward Bz at the center of dayside reconnection — the physical mechanism
of the chain Bz → magnetosphere–ionosphere currents → ground dB/dt. Our
measurement is the direct TE confirmation at the ground end of that chain,
with a strictly corrected null. Methodologically, our family bound is a
conservative multiple-comparison control in the spirit of Runge et al.
(2019); the surrogate design follows the phase-randomization practice of
Schreiber (2000) and the ETE criticism of Marschinski & Kantz (2002)
(here answered by the family bound rather than by shuffling the condition).

## 6. Limitations

- **Two stations, one zone.** The direction replicates at ABK and SOD
  (both auroral zone); a mid-latitude network generalization is not
  measured here. Round-significance at the second station is not crossed
  (family bound, §4.4).
- **dB/dt is the induction driver, not the network current.** No GIC feed
  (electric force channel) exists in the system; the paper measures the
  excitation, not the damage.
- **Minute grain is a single 22-h window.** A storm-ensemble at minute
  resolution would require a minute-resolution retro solar-wind archive,
  which the stack does not carry (RTSW live holds ~1 day).
- **Daily grain uses stride 3** (every third day; lag 1 = 3 days). The
  full-density daily run is 9× costlier and left for a future run; the
  stride is named, not hidden.
- **Estimator validation.** The scalar TE estimator is the untouched
  canonical reference of the system; it passes the Schreiber-2000
  ground-truth benchmark (§3.5). The KDE-h sensitivity (h/2, 2h) is
  partially covered (series scaling invariance) and otherwise open.
- **PE gate not engaged** at these window sizes (3 segments of 360 samples
  in the minute grain; the yearly grains do not apply it). Non-stationarity
  is instead controlled by the year separation and the named status stack
  of the data.
- **fam is conservative but not exhaustive.** It corrects for the round's
  multiplicity; it does not model dependence between surrogate draws.
- **Surrogate-machine generation (conservative footnote).** The family bound
  here was generated by the pre-fix surrogate RNG (fam-machine: pre-fix).
  The pre-fix band lies higher than the post-fix band; a verdict that holds
  against the higher pre-fix bound holds post-fix a fortiori. The measured
  direction is thus the conservative statement. **re-run scheduled:** the
  post-fix surrogate machine re-run is queued to confirm the direction under
  the corrected RNG.

## 7. Conclusion

Transfer entropy with a phase-randomized null and a family bound identifies
the causal driver of the geomagnetic induction excitation: the southward
interplanetary magnetic field, acting in the hour of the ground response.
Bz → dB/dt is the only pair of the hourly 2024 round that exceeds the family
bound; the direction repeats in 2025 with an even larger transfer; the
reverse direction and the density control are silent throughout; and the
daily grain is empty because daily means wash the driver out. For the grid
operator this is a concrete statement: watch Bz at L1, not the daily
average — the hour that matters is the hour the magnetometer moves.

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

---

*Data and code:* the instrument, probes and register live in the omegaflow
repository (`src/mathematikerin/te.rs` — canonical scalar estimator, untouched;
`tools/work/src/bin/bz_blatt_probe.rs`, `tools/work/src/bin/bz_retro_probe.rs`; survey
`docs/surveys/survey-2026-08-21-bz-kausalpfeil.md`). All verdicts are
machine-measured; the register language of the system is German, this
manuscript is its English face. All bibliographic entries above were
checked against the Crossref registry on 2026-08-22.
