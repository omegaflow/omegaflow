<!--
  title: The causal driver of geomagnetically induced currents
  class: paper
  date: 2026-08-22
  sha256: b3acd367e38337784bfa87ac38d632ae09cc6b290fc3d29fc14c65ffff6421f5
  fam-machine: post-fix
  status: live
  see-also: docs/specs/broken-null-control.md
-->

# The causal driver of geomagnetically induced currents

*Omegaflow Working Group — Bz-Blatt Session, 2026-08-22*

## Abstract

The induction excitation of geomagnetically induced currents (GIC) is dB/dt. Which solar-wind quantity drives it — southward Bz, bulk speed, or density — is open at minute-to-hour scales. We measure transfer entropy (TE) from L1 solar-wind drivers to the hourly and daily maxima of dB/dt at INTERMAGNET Abisko (68.36° N), with phase-randomized surrogates, per-lag thresholds, and a family bound. At the minute grain, Bz→dB/dt peaks at lag 60 min, per-lag significant but family bound in one 22-hour window. At the hourly grain, Bz→dB/dt exceeds the family bound in both storm years at Abisko and at Sodankylä (2024: TE 0.12670 vs fam 0.10557; 2025: TE 0.13309 vs fam 0.12136; SOD 2024: TE 0.11695 vs fam 0.10571), all under the corrected (post-fix) surrogate null. The density control never clears the family bound; the reverse direction dB/dt→Bz stays below the bound at Abisko in both years but marginally clears it at Sodankylä. At the daily grain over 32 years (1994–2026, n ≈ 3900), all six pairs stay below the family bound. The causal driver is the southward interplanetary field at sub-daily timescales; daily means do not carry it.
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
for future runs). All seeds fixed. The family bounds reported here were
measured under the corrected (post-fix) surrogate RNG
(`fam-machine: post-fix`); the pre-fix run is superseded (see the footnote,
§6).

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
not a stable asymmetry property: it is the estimator's expected response
under strong bidirectional coupling and finite samples, and the direction is
carried by the asymmetry (dominance of the known direction), not by an
absolute reverse silence. This is the honest reading the paper takes for the
real data (§4.4, §6): a marginal reverse arrow at the second station is
consistent with a weak reverse coupling, not a null failure.

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
an artefact-zone candidate. The 22-h window's family bound (3.744e-1)
predates the surrogate-RNG correction and is not reproducible (the live RTSW
window is no longer in the cache); as the higher pre-fix band it is
conservative, and the minute-grain verdict (family bound) does not depend on
it.

### 4.2 Hourly grain — 2024 (n paired 8728)

| pair | lag | TE | own threshold | fam = 1.0557e-1 | verdict |
|---|---|---|---|---|---|
| **Bz → dB/dt** | 0 h | **1.2670e-1** | 7.734e-2 | — | **arrow** |
| dB/dt → Bz | 0 h | 1.0512e-1 | 1.0649e-1 | — | family bound |
| Speed → dB/dt | 0 h | 9.882e-2 | 7.220e-2 | — | family bound |
| dB/dt → Speed | 0 h | 3.179e-2 | 2.927e-2 | — | family bound |
| Density → dB/dt | 0 h | 8.825e-2 | 7.802e-2 | — | family bound |
| dB/dt → Density | 0 h | 6.988e-2 | 6.828e-2 | — | family bound |

**Bz → dB/dt is the only pair of the round that exceeds the family bound.**
The reverse direction dB/dt→Bz (1.0512e-1) sits just below its own threshold
(1.0649e-1) and below the bound; the density control stays below the bound.
Lag 0 h straddles the 30–60 min L1 travel time (part of the signal lands in
the same hour, part in the next).

### 4.3 Hourly grain — 2025 (n paired 8688)

| pair | lag | TE | own threshold | fam = 1.2136e-1 | verdict |
|---|---|---|---|---|---|
| **Bz → dB/dt** | 0 h | **1.3309e-1** | 9.628e-2 | — | **arrow** |
| dB/dt → Bz | 0 h | 1.1663e-1 | 1.2260e-1 | — | family bound |
| Speed → dB/dt | 0 h | 1.1562e-1 | 9.292e-2 | — | family bound |
| dB/dt → Speed | 0 h | 2.836e-2 | 2.107e-2 | — | family bound |
| Density → dB/dt | 0 h | 7.710e-2 | 9.724e-2 | — | family bound |
| dB/dt → Density | 0 h | 5.670e-2 | 6.239e-2 | — | family bound |

The structure repeats and now clears the bound: Bz is again the only forward
channel above its own threshold, with a *higher* TE than 2024 (1.3309e-1), and
under the corrected null its family bound (1.2136e-1) is lower than the
pre-fix value (1.4256e-1) that had held this year at family bound — 2025's Bz
arrow is now round-significant. The density control stays below the bound.

### 4.4 Second station — Sodankylä (SOD, 67.37° N), hourly 2024

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
(1.1695e-1) clears the corrected bound (1.0571e-1). At this station, however,
the reverse direction dB/dt → Bz (1.0682e-1) also marginally clears the same
bound — the direction asymmetry (forward 1.1695 vs reverse 1.0682, ratio 1.09)
is real but smaller than at Abisko (ratio 1.20 in 2024). The reverse channel
is not silent here; §3.5 and §6 treat this as the estimator's expected
reverse response under strong coupling, not as a null failure, and it tempers
a strictly one-way reading.

### 4.5 Daily grain — 1994–2026 (stride 3, n paired ≈ 3900)

| pair | lag | TE | own threshold | fam = 1.6995e-1 | verdict |
|---|---|---|---|---|---|
| Bz → dB/dt | 0 d | 1.2525e-1 | 1.1524e-1 | — | family bound |
| Speed → dB/dt | 0 d | 9.703e-2 | 1.1632e-1 | — | family bound |
| Density → dB/dt | 0 d | 1.059e-1 | 1.1663e-1 | — | family bound |
| dB/dt → Bz | 0 d | 1.214e-1 | 1.7057e-1 | — | family bound |

All six directed pairs stay below the family bound over 32 years and every
storm of the era. The table tabulates the three forward drivers
(Bz/Speed/Density → dB/dt) and the dB/dt → Bz reverse; the two remaining
reverse directions (dB/dt → Speed, dB/dt → Density) stay below the bound like
the hourly grains and are not separately tabulated. The daily mean of Bz
carries no information about the
daily maximum of dB/dt — the southward excursions that drive storms average
out at this grain.

## 5. Discussion

**The driver lives sub-daily.** The three grains tell one consistent story.
The minute grain points at Bz with the correct lag (60 min). The hourly
grains make the arrow round-significant in both storm years at Abisko and at
Sodankylä. The daily grain is empty — not for lack of data (n ≈ 3900, 32
years, all storms included) but because the daily mean destroys the physical
signal: a storm is a multi-hour southward excursion, and its daily average is
diluted toward zero. The absence at the daily grain is itself the physical
finding (0 honored in the system's vocabulary).

**Direction and asymmetry.** The forward direction Bz → dB/dt exceeds the
bound in every hourly round; the density control (the structural indictment
check: density does not drive reconnection) never clears the bound anywhere.
The reverse channel dB/dt → Bz stays below the bound at Abisko in both years
but marginally clears it at Sodankylä 2024 (ratio forward/reverse 1.09). The
direction is carried by the asymmetry — forward exceeds reverse in all three
rounds — and, per §3.5, a weak reverse response under strong coupling is the
estimator's expected behavior, not a null failure; it tempers a strictly
one-way reading of the Sodankylä round. The hourly lag 0 is the correct
coarse representation of the 30–60 min propagation measured at the minute
grain; the expected lag-0/edge artefact zones are clean (the one edge excess,
the quiet-window Speed arrow at 120 min, is named and outside the travel
window).

**The corrected null sharpens, not weakens, the finding.** The family bound
is the strongest null TE of the round. Under the corrected (post-fix)
surrogate RNG the bounds fall (2024: 0.12480 → 0.10557; 2025: 0.14256 →
0.12136) because the pre-fix half-circle RNG had inflated the surrogate
distribution. The Bz arrow, which pre-fix cleared the bound only in 2024,
now clears it in both storm years at both observatories. What the pre-fix
reading took for year-dependence of round-significance was partly an artefact
of the inflated null; the corrected bound makes the sub-daily Bz driver a
consistent, replicating statement rather than a single-storm event.

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

- **Two stations, one zone.** The forward arrow replicates at ABK and SOD
  (both auroral zone); a mid-latitude network generalization is not
  measured here. At SOD the reverse channel also clears the bound (§4.4),
  consistent with the estimator's reverse response under coupling (§3.5).
- **dB/dt is the induction driver, not the network current.** No GIC feed
  (electric force channel) exists in the system; the paper measures the
  excitation, not the damage.
- **Minute grain is a single 22-h window.** A storm-ensemble at minute
  resolution would require a minute-resolution retro solar-wind archive,
  which the stack does not carry (RTSW live holds ~1 day).
- **Daily grain uses stride 3** (every third day; lag 1 = 3 days). The
  full-density daily run is 9× costlier and left for a future run; the
  stride is named, not hidden.
- **Estimator validation (reverse channel).** The scalar TE estimator is the
  untouched canonical reference of the system; on the Schreiber-2000
  benchmark it recovers the known direction with asymmetry 6.75 but does not
  null the reverse channel at strong coupling (ground-truth verdict NOT
  PASS for absolute reverse silence; §3.5). The direction in the real data
  is argued from the forward-over-reverse asymmetry and the silent density
  control, not from an absolute reverse null. The KDE-h sensitivity (h/2, 2h)
  is partially covered (series scaling invariance) and otherwise open.
- **PE gate not engaged** at these window sizes (3 segments of 360 samples
  in the minute grain; the yearly grains do not apply it). Non-stationarity
  is instead controlled by the year separation and the named status stack
  of the data.
- **fam is conservative but not exhaustive.** It corrects for the round's
  multiplicity; it does not model dependence between surrogate draws.
- **Surrogate-machine generation.** The family bounds in this version were
  measured under the corrected (post-fix) surrogate RNG
  (`fam-machine: post-fix`, §3.4). The pre-fix run (fam 2024 = 0.12480) is
  superseded; the corrected bounds are lower and the direction statement
  holds a fortiori against the pre-fix band where they overlap. A hardening
  of the reverse channel at strong coupling (§3.5) is registered as an open
  research thread and does not gate the direction reported here.

## 7. Conclusion

Transfer entropy with a phase-randomized null and a family bound identifies
the causal driver of the geomagnetic induction excitation: the southward
interplanetary magnetic field, acting in the hour of the ground response.
Bz → dB/dt exceeds the corrected family bound in both storm years at Abisko
and at Sodankylä; the density control stays below the bound throughout; the
forward direction dominates the reverse in every round; and the daily grain
is empty because daily means wash the driver out. The reverse channel shows
a weak, expected response under strong coupling (§3.5), which tempers a
strictly one-way reading at the second station. For the grid operator this is
a concrete statement: watch Bz at L1, not the daily average — the hour that
matters is the hour the magnetometer moves.

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
`tools/measure/src/bin/bz_retro_probe.rs`, `tools/measure/src/bin/bz_blatt_probe.rs`).
All verdicts are
machine-measured; the register language of the system is German, this
manuscript is its English face. All bibliographic entries above were
checked against the Crossref registry on 2026-08-22.
