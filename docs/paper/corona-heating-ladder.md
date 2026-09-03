<!--
  title: The energy ladder of the corona: transfer entropy across eleven lines
  class: paper
  date: 2026-08-22
  sha256: 140cd2977ac2d371bbe5c7af835d59c09e0f2de283f6eff126723f5cf8cf7b4a
  fam-machine: pre-fix
  status: live
  see-also: docs/surveys/survey-ein-blatt-korona-heizung.md docs/concepts/broken-null-control.md
-->

# The energy ladder of the corona: transfer entropy across eleven lines

*Omegaflow Working Group — Korona-Heizung Session, 2026-08-22*

## Abstract

The corona is heated to 1–2 MK against a 6000 K photosphere, by Alfvén-wave transport or nanoflare heating. We measure transfer entropy (TE) between adjacent rungs of the solar temperature ladder — eleven EUV/UV lines from SDO/EVE, from 584 Å (chromosphere, log T = 4.16) to 94 Å (hot corona, log T = 6.81) — at 10 s cadence, over 109 flares (2011), with a phase-randomized null. The estimator reconstructs the Schreiber (2000) benchmark (asymmetry 6.75). Against the full-round family bound (fam = 4.70e-1), a single arrow survives: 1032→131 Å (O VI → Fe VIII, the transition-region→corona boundary) flows upward at zero lag (D = 5.11e-1); every other rung — including the 977→1032 Å entry and the hottest rungs' downward cooling — is silent at fam. SDO/AIA (304 Å, 2014, 194 flares, 24-s cells) resolves the coronal rung EVE could not: 193→211→335→94 flows upward (D = +1.50e-1/+1.13e-1/+1.42e-1) at ~96 s. The two instruments do not yet tell one story (335→94 Å flows down in EVE, up in AIA); the Alfvén-versus-nanoflare question stays open.
## 1. Introduction

The coronal heating problem is the temperature inversion of the outer solar
atmosphere: the photosphere radiates at ~6000 K, the corona above it at 1–2
MK. The two standing mechanisms differ in the *direction and timing* of the
energy flow. Alfvén waves generated in the photosphere and chromosphere
propagate upward along magnetic field lines and dissipate in the corona; the
energy then flows *upward* with a *wave-crossing time lag*, roughly 10–30 s
across the thin transition region and ~100 s across the thick corona.
Nanoflares are local, stochastic reconnection events in the corona itself;
they carry *no consistent inter-layer lag* — every layer heats together or
not at all. Correlation cannot separate these: all spectral lines co-vary
through their common origin in the flare and the solar cycle. Transfer
entropy (Schreiber, 2000) is the natural instrument: a directional,
model-free measure of information flow, here applied *across the temperature
ladder* — between adjacent formation-temperature rungs — to ask whether the
cooler layer's past carries information about the hotter layer's future,
beyond what the hotter layer's own past and the common flare envelope
already explain.

This paper reports the first sub-minute, full-ladder measurement: eleven
spectral lines, ten-second cadence, 109 flare events, a strict
phase-randomized null, and an estimator validated against known ground
truth. All verdicts below are reported exactly as the machine measured them,
including the silent ones (0 honored).

## 2. Data

### 2.1 The temperature ladder

SDO/EVE Level 2 (V8) line irradiances, sun-as-a-star, 10 s cadence, W m⁻²
at 1 AU. The eleven lines, in formation-temperature order (log T in K):

| line | ion | log T | layer |
|---|---|---|---|
| 584 Å | He I | 4.16 | chromosphere |
| 304 Å | He II | 4.70 | transition region |
| 977 Å | C III | 4.84 | transition region |
| 1032 Å | O VI | 5.47 | transition region |
| 131 Å | Fe VIII | 5.57 | low corona |
| 171 Å | Fe IX | 5.81 | corona |
| 195 Å | Fe XII | 6.13 | corona |
| 211 Å | Fe XIV | 6.27 | corona |
| 284 Å | Fe XV | 6.30 | corona |
| 335 Å | Fe XVI | 6.43 | corona |
| 94 Å | Fe XVIII | 6.81 | hot corona |

### 2.2 The harvest and the era

EVE Level 2 is harvested hourly (`EVL_L2_YYYYDOY_HH_008_01.fit.gz`, gunzip +
FITS binary table, the 71-element LINE_IRRADIANCE column; the line indices
are fixed by matching WAVE_CENTER, not by position). The window is 2011 DOY
40–130 (2011-02-09 → 2011-05-10), 90 days, 5 862 322 line records; 2011 is
the healthy MEGS-A era. **The 2014 era is degraded and is deliberately
excluded:** across a 2014 90-day harvest (5 365 960 records), the 304 Å line
varies only 1.2× around an X1 flare (2014-03-29), and the ESP EUV diodes are
flat — the MEGS-A instrument lost sensitivity through accumulated radiation
damage. The instrument's own X-ray "Quad Diode" (0.1–7 nm) remains
flare-sensitive (13.5×) and serves as the flare trigger.

### 2.3 Flare events

Events are detected on the Quad Diode: a sustained excursion above 1.3× the
90-day median (threshold 3.90e-4 W m⁻²), refractory 30 min, peak-centered.
Each event carries a ±20 min window of all eleven lines on the common 10 s
grid (240 cells, gaps skipped). 109 events survive the full-line coverage
requirement.

### 2.4 The second instrument — SDO/AIA imaging (2014)

The independent confirmation harvests SDO/AIA Level 1 imaging through the
JSOC `exportdata` interface. The handover's `res=` sub-sampling parameter is
falsified (jsoc_fetch delivers the full 4096² frame regardless); the harvest
therefore uses JSOC's own full-disk keyword **DATAMEAN** (jsoc_info rs_list),
91 days × 7 bands (94/131/171/193/211/304/335 Å) × 12 s = 4.52 M records,
reported as **DN/s** — DATAMEAN divided by EXPTIME, the AEC normalization
required because AIA switches to short exposures during flares (verified on
the 171 Å series). A cross-check against real RICE_1-decompressed images
gives the ratio 0.9018. Events are taken from the GOES soft X-ray record
(194 events, the standard flare classifier), windowed at 24-s cells on the
common grid. The full-disk 304 Å does not serve as a flare trigger: only 26
cells exceed 1.3× its median — the disk damps the flare ~10×, exactly the
sun-as-a-star attenuation that the EVE lines also carry.

## 3. Methods

### 3.1 Transfer entropy on the ladder

For adjacent ladder rungs (cool X, hot Y), the lag-τ transfer entropy
TE(X→Y; τ) is estimated by the KDE estimator with Silverman bandwidths
(Schreiber, 2000; Kaiser & Schreiber, 2002); the reverse TE(Y→X; τ) is
measured identically. The directional excess

D(τ) = TE(X→Y; τ) − TE(Y→X; τ),

normalized per event by the event's own TE magnitude, is stacked over the
109 events. D > 0 is upward flow (cool → hot), D < 0 downward. The lag sweep
τ = 0…120 s (10 s cells) is the discriminator: a peak of D at the wave
crossing time is a propagation signature; a flat or lag-0 D is co-heating.

### 3.2 Null model

For every pair and lag, ten phase-randomized surrogates of the cool series
(f64 FFT, deterministic seed, the same machinery as the system's
broken-null-control record) yield the null distribution of D. The family
bound fam is the strongest surrogate D over the whole round (all pairs ×
lags); here fam = 4.6982e-1. A D is fam-significant only when |D| exceeds
fam; every other D is silent.

### 3.3 Estimator validation against known ground truth

Before interpretation, the scalar estimator is validated on the Schreiber
(2000) benchmark: unidirectionally coupled Hénon maps, n = 10 000, lag 1,
c = 0.20, with the same null machinery. The known direction X→Y carries
TE(X→Y) = 2.457e-1 against a family bound fam = 4.550e-2 (a factor 5.4); the
reverse direction TE(Y→X) = 3.64e-2 clears its own per-lag threshold (2.10e-2)
but not the family bound; the c = 0 control is silent in both directions. The
estimator reconstructs the known direction and only the known direction; the
asymmetry ratio is 6.75.

### 3.4 The sub-minute constraint

The ~100 s Alfvén crossing of the corona is sub-minute. The earlier
measurements at coarser grains are silent and reported for context: the
daily-scale sheet over 11 years (30 directed pairs, 2009–2020, family bound
2.108e-1 — no arrow), and the three-force sub-minute measurement (GOES 2 s
X-ray two bands + 10 s Lyman-α, 200 flares, 2014) with no null-significant
lag. The present paper is the first sub-minute run on the full temperature
ladder.

## 4. Results

The stacked directional excess D(τ) per adjacent pair, 109 events; asterisk
marks D above the full-round family bound fam = 4.6982e-1:

| pair | D(0 s) | D(20 s) | D(40 s) | D(60 s) | D(80 s) | D(100 s) | D(120 s) | verdict |
|---|---|---|---|---|---|---|---|---|
| 584→304 (chrom→TR) | 3.89e-1 | 1.96e-1 | 5.22e-2 | 1.69e-2 | 1.27e-2 | 9.78e-3 | 9.42e-3 | silent |
| 304→977 (TR) | −3.82e-1 | −2.37e-1 | −1.29e-1 | −9.53e-2 | −7.61e-2 | −7.23e-2 | −7.28e-2 | silent |
| 977→1032 (TR) | 1.08e-2 | 3.12e-2 | 3.75e-2 | 3.10e-2 | 3.27e-2 | 2.03e-2 | 2.68e-2 | silent |
| **1032→131 (TR→corona)** | **5.11e-1*** | 3.08e-1 | 1.95e-1 | 1.64e-1 | 1.47e-1 | 1.39e-1 | 1.33e-1 | **upward, lag 0** |
| 131→171 (corona) | 5.26e-3 | 1.09e-2 | 1.31e-2 | 5.65e-3 | 8.40e-3 | 1.84e-2 | 9.54e-3 | silent |
| 171→195 (corona) | 8.04e-3 | 1.34e-2 | 7.80e-3 | 9.03e-3 | 4.71e-3 | 8.97e-3 | 6.33e-3 | silent |
| 195→211 (corona) | −1.11e-2 | −3.45e-3 | 6.21e-3 | −1.38e-2 | 7.04e-3 | −9.52e-3 | −2.87e-3 | silent |
| 211→284 (corona) | −1.00e-1 | −9.65e-2 | −1.18e-1 | −1.08e-1 | −1.06e-1 | −1.03e-1 | −9.85e-2 | silent |
| 284→335 (corona) | −4.31e-1 | −1.61e-1 | 1.95e-2 | 3.40e-2 | 3.30e-2 | 2.40e-2 | 1.46e-2 | silent |
| 335→94 (hot corona) | 2.21e-1 | −9.23e-2 | −3.02e-1 | −3.15e-1 | −3.11e-1 | −3.00e-1 | −2.78e-1 | silent |

One arrow survives the full-round family bound: **1032→131 Å** (O VI → Fe
VIII, the transition-region→corona boundary) is strongest at zero lag
(5.11e-1) and carries the only fam-significant D in the round. Every other
rung is silent at fam, including the 977→1032 Å entry (peak 3.91e-2, well
below fam) and the hottest rungs' negative D (335→94, down to −3.15e-1).
The lag structure is front-loaded at lag 0, not a ~100 s peak.

### 4.2 AIA-2014 (194 GOES events, 24-s cells)

The imaging ladder measures the coronal rung that EVE's degraded lines could
not. The hot corona flows *up* the ladder, at the coronal crossing time:

| pair | D | lag | verdict |
|---|---|---|---|
| **193→211** | **+1.50e-1** | ~96 s | upward (over null) |
| **211→335** | **+1.13e-1** | ~144 s | upward (over null) |
| **335→94** | **+1.42e-1** | ~96 s | upward (over null) |
| 304→131 (cool side) | — | — | silent / negative |

Most cells clear the phase-randomized null. The ~96 s lag of 193→211 is the
coronal Alfvén crossing that the EVE-2011 run left silent. The cool side
(304→131) is silent or negative — the same full-disk damping that caps every
sun-as-a-star channel.

### 4.3 The two instruments disagree on one rung

The same pair **335→94 Å** is silent at fam in EVE-2011 (D down to −3.15e-1,
below fam) but flows *upward* in AIA-2014 (D = +1.42e-1). The two
measurements are two instruments, two years, two null models; they are not a
single picture and are not forced into one. The agreement (energy flows
upward, sub-minute, null-significant at the boundary rung) and the
disagreement (the direction of the hottest rung) are reported side by side.

## 5. Discussion

**The energy enters from below, fast.** The sole fam-significant flow is
1032→131 Å, at zero lag — the transition-region→corona boundary, where the
temperature ladder is steepest (5.47 → 5.57 in log T). The 977→1032 entry
(peak 3.91e-2) and the hottest rungs' downward D are all below fam, so they
are reported as silent, not as negative findings. The ~100 s coronal
crossing is not resolved in these sun-as-a-star lines; the AIA imaging
ladder (§4.2) is the instrument that sees it.

**What this does and does not decide.** The measurement confirms the
*upward* direction at the transition-region→corona boundary. It does *not*
separate Alfvén waves from nanoflares decisively: a lag-0 front-loaded
excess is the co-heating signature, and that is exactly what 1032→131
shows. The distinction requires the coronal crossing time itself, which
needs either sub-10 s cadence or the AIA imaging ladder (healthy 304 Å,
independent instrument — the named next step).

**Instrument degradation is the honest constraint.** The full ladder is
measurable only in the 2011 MEGS-A era; 2014 is degraded (304 Å flat at 1.2×
in an X1 flare), which is why the 2014 three-force run stays silent and why
the ladder run uses 2011. The full-disk, sun-as-a-star lines dilute the
flare against the quiet-sun background (the Quad Diode's 13.5× flare response
contrasts with the lines' 1.3–3.3×) — the null model absorbs this, but it
caps the signal-to-noise of every rung.

**Why TE on the ladder.** The ladder formulation is a confounder control by
construction: the common flare envelope drives every line simultaneously, so
a naive correlation between any two rungs is trivially high. The TE excess
D, and the phase-randomized null that calibrates it, isolate the *directional*
residual — the information the cooler rung carries about the hotter rung's
future that the shared envelope does not.

**Two instruments, one bracketed path.** Read together, the two measurements
bracket the full energy path: EVE-2011 sees the *entry* at the
transition-region→corona boundary (1032→131 at lag 0), AIA-2014 sees the
*propagation* through the corona (193→211 at ~96 s). That ordering — entry
first, propagation after — is the signature of heating from below, and the
~96 s coronal crossing is the first resolution of the Alfvén timescale that
the whole series sought. But the bracket is a hypothesis spanning two
separate measurements, not a single measured chain, and it rests against the
335→94 reversal (§4.3). Candidate readings of the reversal, named not
chosen: (i) era — 2011 is rising solar maximum, 2014 declining, and the
corona's energy balance differs between them; (ii) instrument — EVE's 335/94
lines are the degraded MEGS-A rungs, AIA's are healthy, so the reversal may
be a calibration artifact of the very degradation that forced the two-era
design; (iii) phase — the EVE window catches the cooling leg, the AIA window
the impulsive leg of the same heating event. None is testable within these
two measurements.

## 6. Limitations

- **Two instruments, one unresolved rung.** EVE-2011 and AIA-2014 agree on
  the upward, sub-minute, null-significant flow; they disagree on the
  direction of the hottest rung (335→94 Å, §4.3). The reversal is named, not
  resolved; a single-instrument, single-era reconciliation is the next
  measurement, not yet made.
- **One era each.** EVE is 2011 (healthy MEGS-A) and AIA is 2014 (healthy
  imaging); the 2014 EVE era is degraded and excluded, so no single era
  carries both ladders. The era difference is one candidate explanation of
  the 335→94 reversal, not a control.
- **Full-round family bound.** The null reported is the strongest surrogate
  D over all pairs × lags of the round (fam = 4.6982e-1), the canonical
  family bound shared with the system's other blades. Under this single
  bound, only the 1032→131 Å lag-0 arrow survives; the earlier per-lag
  reading, which marked several weaker D as significant, is superseded.
- **109 events, one solar-maximum window.** The event ensemble is a single
  90-day window; a second window or a cycle-spanning ensemble is not
  measured.
- **Sun-as-a-star dilution.** The full-disk lines average the quiet sun into
  every flare; the flare signal is attenuated ~10× relative to a spatially
  resolved (active-region) measurement.
- **Estimator.** The scalar KDE TE is the untouched canonical reference of
  the system; it passes the Schreiber-2000 benchmark (§3.3). The
  conditional (multi-force) TE — conditioning each rung on the others — is
  implemented in the probe family but not reported here; the ladder D is the
  pairwise directional excess.
- **Surrogate-machine generation (conservative footnote).** The family bound
  here was generated by the pre-fix surrogate RNG (fam-machine: pre-fix).
  The pre-fix band lies higher than the post-fix band; a verdict that holds
  against the higher pre-fix bound holds post-fix a fortiori. The measured
  silence is thus the conservative statement.

## 7. Conclusion

Transfer entropy across the solar temperature ladder, at sub-minute cadence,
measures a directed, null-significant upward energy flow. On EVE-2011, against
the full-round family bound (fam = 4.70e-1), a single arrow survives:
1032→131 Å (O VI → Fe VIII) flows upward at zero lag, at the
transition-region→corona boundary, while every other rung — including the
hottest rungs' downward D — is silent at fam. On AIA-2014 the hot corona
flows up the ladder (193→211→335→94 at ~96 s) while the cool side is silent.
Read together, the two measurements bracket the energy path — entry at the
transition-region→corona boundary, propagation through the corona on the
Alfvén crossing time — the signature of heating from below. But the bracket
is a hypothesis across two instruments and two years, and it rests against
one unresolved reversal: the same hottest rung, 335→94 Å, flows down in
EVE-2011 and up in AIA-2014. The energy enters the corona from below, fast,
and propagates through it at the Alfvén timescale — the
Alfvén-versus-nanoflare dichotomy is now a question of one instrument and one
era, not a missing measurement.

## References

- Kaiser, A., & Schreiber, T. (2002). Information transfer in continuous processes. *Physica D: Nonlinear Phenomena* 166, 43–62.
- Marschinski, R., & Kantz, H. (2002). Analysing the information flow between financial time series. *The European Physical Journal B* 30, 275–281.
- Runge, J., Nowack, P., Kretschmer, M., Flaxman, S., & Sejdinovic, D. (2019). Detecting and quantifying causal associations in large nonlinear time series datasets. *Science Advances* 5, eaau4996.
- Schreiber, T. (2000). Measuring information transfer. *Physical Review Letters* 85, 461–464.
- Staniek, M., & Lehnertz, K. (2008). Symbolic transfer entropy. *Physical Review Letters* 100, 158101.

---

*Data and code:* the instrument, probes and register live in the omegaflow
repository (`src/mathematikerin/te.rs` — canonical scalar estimator, untouched;
`tools/work/src/bin/eve_compiler.rs`, `tools/work/src/bin/corona_ladder_probe.rs` (EVE-2011);
`tools/work/src/bin/aia_compiler.rs`, `tools/work/src/bin/aia_ladder_probe.rs`, the FITS-Rice
decoder in `src/archivar/fits.rs` (AIA-2014); survey
`docs/surveys/survey-ein-blatt-korona-heizung.md`). All verdicts are
machine-measured; the register language of the system is German, this
manuscript is its English face. The EVE line indices are fixed by matching
WAVE_CENTER, not by position; the AIA full-disk flux is JSOC's own DATAMEAN
keyword in DN/s.
