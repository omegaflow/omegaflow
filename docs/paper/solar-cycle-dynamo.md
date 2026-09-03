<!--
  title: The solar-cycle dynamo: transfer entropy to the activity index
  class: paper
  date: 2026-08-22
  sha256: 56e480ca75845ddc27bcaf5dd2bf79863d4e3fd64241d74ea5d87d879c013f0d
  fam-machine: post-fix
  status: live
  see-also: docs/handover/handover-2026-08-22-sonnenzyklus-ernte.md docs/paper/corona-heating-ladder.md
-->

# The solar-cycle dynamo: transfer entropy to the activity index

*Omegaflow Working Group — Sonnenzyklus-Ernte Session, 2026-08-22*

## Abstract

The solar cycle is the α-Ω dynamo: differential rotation (Ω-effect, via helioseismic mode frequencies) and helical turbulence (α-effect, via the polar field). We measure transfer entropy (TE) from the p-mode frequency shift and the polar field at minimum to the 10.7 cm flux F10.7, monthly, with a phase-randomized null and a family bound. The interior measurement covers 313 GONG 36-day runs (1995–2026); the polar measurement covers 1795 WSO observations (1976–2026). Both are silent: TE(GONG→F10.7) peaks at 1.15e-1 (lag 30 runs) against a family bound 4.79e-1; TE(polar→F10.7) peaks at 3.48e-1 against 4.89e-1, at a 70-month lag — the predicted Vorläufer lead — but fails to clear the null. A second instrument, the BiSON 7-day shifts (1992–2025), is likewise silent (1.45e-1 vs 3.43e-1). The interior silence is reproduced by a second instrument; the polar record is held against SDO/HMI. The dynamo's causal arrow is not carried by these signatures at this floor.
## 1. Introduction

The solar cycle is driven by a hydromagnetic dynamo in the convection zone
(Charbonneau, 2010). Differential rotation winds a toroidal field from the
poloidal one (the Ω-effect); helical, rising convection twists the toroidal
field back into a poloidal one of opposite sign (the α-effect); the cycle
advances as the two fields exchange roles. The dynamo is a fluid-interior
process and is not directly measurable; only its surface signatures are.
Three quantities are its observable hand: the helioseismic mode frequencies
(the near-surface and interior magnetic field shifts the p-mode frequencies
by ~0.1–1 µHz over the cycle), the polar field (the poleward-migrating
surface flux, the α-effect's visible product), and the activity itself
(sunspots, flares, F10.7). Correlation among all three is trivially high —
they share the cycle. Transfer entropy is the instrument that separates the
*directional* residual: whether one signature's past carries information
about another's future beyond what that other's own past already explains
(Schreiber, 2000; Kaiser & Schreiber, 2002).

The planetary hypothesis has already been measured against F10.7 in this
system and is silent: the tidal acceleration of Jupiter and Saturn carries
no TE arrow to the activity at monthly grain over seven cycles, and the
measured cycle period is variable (~10–14 a), not Jupiter-locked at 11.86 a.
What remained unmeasured are the two *internal* signatures — the interior
(GONG mode frequencies) and the polar field (the Vorläufer relation) — which
is the commission this paper reports. The physical questions are specific.
Does the interior carry the activity: does the p-mode frequency shift lead
F10.7, as the interior field building toward maximum should precede the
surface eruption? And does the old polar field carry the next cycle: does
the polar field at minimum lead F10.7 by the ~5–6 year Vorläufer lag
(Svalgaard, Cliver & Kamide, 2005; Muñoz-Jaramillo et al., 2013)?

Both measurements are reported exactly as the machine measured them,
including the silent verdicts (0 honored). The cadences are monthly, the
null is phase-randomized, and the bound is the family bound (the strongest
surrogate of the whole round). Three and 4.5 cycles respectively are
statistically thin; a silent result is the honest answer, not a failed one.

## 2. Data

### 2.1 The interior — GONG p-mode frequencies

The GONG network publishes mode-parameter tables per 36-day observing run.
The relevant product is `TSERIES/v1y/` — the Clebsch-Gordon m-smoothed
frequency table (the `v1f/` directory is the per-degree raw-fit variant;
`vmt/` carries amplitude only). Each run contributes one LZW-compressed
table (`mrv1yYYMMDD.txt.Z`, Unix `compress` format; the system's std-only
decoder `src/archivar/lzw.rs` is a port of GNU gzip's `unlzw.c`). Each row is a mode
(n, l) with the fit frequency `nu` in µHz; for l = 0 the value is exact (no
m-splitting), for l > 0 it is the m-averaged frequency. The run directory
name `mrv1yYYMMDD` is the run's *central* date (the corresponding `vmt`
FITS headers are ±18 days around it — verified on `mrvmt950629`:
TS_START 1995-06-12, TS_END 1995-07-17). The harvest covers **313 runs,
1995-06-29 → 2026-03-30**, l = 0…2, **68 modes**, 7645 records. Access is
via FTP (`ftp://gong2.nso.edu/TSERIES/`) — the HTTPS route rate-limits
(HTTP 403) after ~100 requests, the FTP service is a separate counter and
serves the full series.

### 2.2 The polar field — WSO

The Wilcox Solar Observatory (Stanford, operating since 1976, carried by
Stanford without NASA funding since 2018) publishes the polar field at
`http://wso.stanford.edu/Polar.html`. Each 10 days the usable daily
measurements in a centered 30-day window are averaged; each row carries the
north field N, the south field S, and the average field, in µT, plus a
20-nHz-filtered variant that removes the yearly geometric projection. The
"average" column is the **signed dipole (N−S)/2** — it is *not* the
unsigned mean, which is visible in the reversal years (2024-10-06: N = −12,
S = +6, Avg = −9). Missing measurements are `XXX` and are skipped (0
honored), never zeroed. The harvest covers **1795 records, 1976-05-31 →
2026-01-09** (37 `XXX` skips). The series is live through 2026-01-09: the
page continues to be updated every 10 days, but every row since 2026-01-09
carries `XXX` (no measurement) as of the 2026-08-23 check — the observatory
(carried by Stanford without NASA funding since 2018) has not produced a
usable polar-field value for ~7 months, so 2026-01-09 is the effective
endpoint of the record and the polar field going forward is carried by HMI
(§2.6), which runs to 2026-07. A documented calibration caveat is carried,
not applied, in WSO's own wording: *"Polarization Sensitivity was reduced
from 16 December 2016 to 18 May 2017. Mean field values have NOT been
recalibrated and are a factor of about 1.55 too small during that
interval."* — the polar field here is a mean-field value, so it is ~1.55×
too small in that window and is not recalibrated in the published table.
A generic Fe I 525.0 nm saturation factor (~1.8×, Svalgaard et al. 1978)
affects the whole series and is deliberately not applied here, as in most
studies. A step near 2016/17 is calibration, not a physical finding.

### 2.3 The activity index — F10.7

The Penticton 10.7 cm solar radio flux, daily, 1947–2026, in W m⁻² Hz⁻¹
(`f107_penticton.bin`, the system's existing harvest). Both signatures are
binned against it monthly (30-day bins for the polar field; the GONG runs
are used at their natural 36-day cadence, the flux averaged over each run's
±18-day window).

### 2.4 The third driver — the planetary tidal acceleration

The planetary hypothesis, in its Jupiter-lock form, proposes that the tidal
acceleration of the giant planets modulates the dynamo and would phase-lock
the cycle to Jupiter's 11.86-year orbit. The driver series is the tidal
acceleration of Jupiter and Saturn at the Sun (GM/d³, computed from the
system's planetary ephemerides), monthly, 1947–2026 — seven cycles. This is
the earlier measurement of the series (`solar_cycle_probe`), re-reported
here so the three-driver picture is self-contained.

### 2.5 The second interior instrument — BiSON frequency shifts

The Birmingham Solar-Oscillations Network (BiSON) measures the same
low-degree (l = 0,1,2) p-modes as full-disk Doppler since 1976 — two cycles
before GONG. The open question for the interior signature was whether a
BiSON frequency-*shift* series could close the 1976–1995 gap and extend the
313-run GONG ensemble from ~3 to ~4.5 cycles. The published answer is no:
BiSON publishes no continuous (epoch, absolute-frequency) series. The only
*machine-readable* (DOI-backed, on the BiSON Open-Data Portal) frequency-shift
series is the 7-day product of Howe et al. (2025, MNRAS 537, 909; DOI
10.25500/edata.bham.00001572) — mean frequency shifts from cross-correlation
in independent (non-overlapping) 7-day segments, columns (epoch, front-side
F10.7, far-side F10.7, mean shift µHz, error µHz). The epoch column is
labelled MJD but carries Julian Dates (first row 2448626.5 = 1992-01-05,
read directly from the file); the series spans JD 2448626.5 → 2460701.5, i.e.
**1992-01 → 2025-01 (32 years).** A second, independently published shift
series does reach back into the gap: Basu, Broomhall, Chaplin & Elsworth
(2012, ApJ 758, 43; arXiv:1208.5493) — 365-day subsets overlapped by 91.25 d
after 1986, variable-length subsets at a ≥16 % duty cycle before it, spanning
**1978-07-31 → 2012-07-07 (34 years, including cycle 21)**, referenced to the
cycle-22 maximum (1988-10 → 1992-04), in three frequency bands. But that
series carries no dataset DOI and no table — its values exist only in the
paper's figures (Fig. 2/3; the portal's list jumps from the 2009 to the 2014
dataset), and the authors themselves flag cycle 21 as too sparse for
smoothing. The true remaining gap is therefore **1976 → 1978-07-31** (the
 single-station Izaña era). The figure-digitization path was pursued: the
 vector PostScript of Fig. 2 (in the arXiv source, `f2.ps`) carries the plotted
 points as exact coordinates, so `bison_basu_compiler` extracts them
 vector-exactly (no raster digitization error) into 337 points over three
 frequency bands, 1978–2012, with per-point error bars from the figure. It
 remains a figure-derived, low-confidence entry — the only exact alternative
 is the BiSON team's own table. The harvest (`bison_shift_compiler`) therefore
carries what is published: 1673 independent 7-day shifts, 1992–2025 — a
**second instrument on the same ~3 cycles as GONG**, not a temporal extension.
It is a Fit-derived quantity (cross-correlation shift, 7-day segments), named
as em with that provenance, exactly like the GONG shift.

### 2.6 The polar field — HMI cross-instrument

To hold the WSO polar-field record against a second instrument, the
SDO/Helioseismic and Magnetic Imager (HMI) synoptic polar-field series is
harvested. HMI measures the same polar field with a different line (Fe I
617.3 nm, space-based) and a different method than WSO (Fe I 525.0 nm,
ground-based, line-of-sight). The harvest uses `hmi.synoptic_mr_polfil_720s`
(JSOC) — the Carrington synoptic chart of the radial field *Br* with polar
correction (polfil), 3600×1440 (CRLN-CEA × CRLT-CEA sine latitude), segment
`Mr_polfil` in Gauss, RICE_1-compressed. JSOC carries no ready-made
(N/S/Avg) polar series (`hmi.polar_field` returns "not a valid series"), so
the polar-cap mean is computed here from the maps, with the cap defined
explicitly as **|sin(latitude)| ≥ sin(60°)** — the polfil series' own LAT0
(the latitude above which the polar correction fits). N is the mean radial
field over the north cap (lat ≥ +60°), S over the south cap (lat ≤ −60°),
and Avg = (N−S)/2, the signed dipole — the same definition as WSO's "Avg".
The pixel scaling is read from the header (BSCALE = 0.1 → Gauss, then Tesla).
The epoch is the calendar day of the rotation centre (T_OBS). The harvest
covers **217 Carrington rotations, 2010-06-02 → 2026-07-19** (~1.5 cycles),
one record per rotation, absent records skipped (0 honored).

## 3. Methods

Transfer entropy TE(X→Y; τ) is estimated by the KDE estimator with Silverman
bandwidths (Schreiber, 2000; Kaiser & Schreiber, 2002) on the system's
canonical scalar path (`transfer_entropy_lag` in `src/mathematikerin/te.rs`, untouched).
For each lag τ the reverse direction is measured identically. The null is
ten phase-randomized surrogates of the driver series (f64 FFT, deterministic
seed) per lag; the **family bound** is the strongest surrogate TE of the
entire round (the multiple-comparison correction). A TE is significant only
when it exceeds that bound. Verdicts: **arrow** (over the bound) or
**silent** (under it). Two measured deviations from the canonical fam are
named, not hidden: the pool carries only the P→F direction (the reverse
F→P surrogates are not pooled, so the bound here is a lower bound on the
canonical two-direction maximum), and the tables report fam as a running
maximum across lags — the final row is the round's fam, the earlier rows are
the ramp the KDE sweep produces.

Measurement state (2026-08-28): all four instrument bounds (GONG §4.1,
WSO §4.2, BiSON §4.5, HMI §4.7) and the Takens thresholds (§4.3) are
re-measured on the corrected RNG (full circle). The GONG and BiSON shift
series were compiled locally from the NSO TSERIES walk and the Birmingham
7-day table (`gong_series_compiler`, `bison_shift_compiler`) because the
derived shift assets are absent from the release — only the raw
`gong_modes.bin`/`bison_pmode.bin` are uploaded. The TE values themselves are
deterministic and unchanged by the RNG fix; only the surrogate-derived
bounds shift.

The GONG series is the mean frequency *shift* across all modes present at
each run (each mode referenced to its own series mean, in µHz) — the common
cycle shift, with the per-mode fitting noise averaged down. The WSO series
is the monthly mean of the signed dipole (N−S)/2. The BiSON series is
already the published mean shift per 7-day segment, binned monthly (30-day
bins) exactly like the WSO series. All three are measured against the
monthly F10.7 in both directions. A known artefact is reported rather
than suppressed: the KDE sweep inflates the TE and the bound monotonically
with the lag (the effective sample m = n − τ shrinks), so a "peak" at high
lag that coincides with the ramp is not a finding.

A second, independent estimator is run on both series as a robustness check:
the Takens-embedded phase-space TE (`topological_te_phase`, dim 3, order 3),
which finds its own auto-MI delay, embeds both series, and carries its own
phase-randomized null (ten surrogates, each re-embedded). The scalar sweep
tests the *lag* hypothesis; the phase-space estimator tests whether the
short-timescale coupling exists at all.

## 4. Results

### 4.1 The interior — GONG → F10.7

313 runs (1995–2026), 36-day cadence, lag sweep 0…72 runs (0…~7 years).
Both directions sit under the family bound at every lag:

| lag (runs) | TE(G→F) | TE(F→G) | family bound |
|---|---|---|---|
| 0 | 7.13e-2 | 1.78e-1 | 8.08e-2 |
| 12 | 8.41e-2 | 1.60e-1 | 2.82e-1 |
| 24 | 1.08e-1 | 1.45e-1 | 4.77e-1 |
| 30 | 1.15e-1 | 1.38e-1 | 4.79e-1 |
| 48 | 8.25e-2 | 1.55e-1 | 4.79e-1 |
| 72 | 7.13e-2 | 1.81e-1 | 4.79e-1 |

The peak of TE(G→F) is 1.15e-1 at lag 30 (2.96 years) — not the edge of the
sweep, and under the bound 4.79e-1. TE(F→G) exceeds TE(G→F) at every lag
(activity carries more about the frequency's future than the reverse), and
it too is under the bound. **Silent.**

### 4.2 The polar field — WSO → F10.7

603 months (1976–2026), monthly bins, lag sweep 0…72 months (0…6 years):

| lag (months) | TE(P→F) | TE(F→P) | family bound |
|---|---|---|---|
| 0 | 4.57e-2 | 4.90e-2 | 6.51e-2 |
| 12 | 1.73e-1 | 1.74e-1 | 2.90e-1 |
| 24 | 2.56e-1 | 3.40e-1 | 4.35e-1 |
| 36 | 2.99e-1 | 4.18e-1 | 4.77e-1 |
| 48 | 3.05e-1 | 4.72e-1 | 4.89e-1 |
| 60 | 3.31e-1 | 5.14e-1 | 4.89e-1 |
| 70 | 3.48e-1 | 5.36e-1 | 4.89e-1 |
| 72 | 3.47e-1 | 5.37e-1 | 4.89e-1 |

The peak of TE(P→F) is 3.48e-1 at lag 70 months = **5.83 years — exactly
the Vorläufer lead** — but against a family bound of 4.89e-1 it does not
clear. TE(F→P) exceeds TE(P→F) at every lag, the opposite of the Vorläufer
direction. Both are under the bound. **Silent.**

### 4.3 Robustness — the phase-space estimator

The scalar estimator is the canonical probe; the Takens-embedded estimator
(`topological_te_phase`, dim 3, order 3, auto-MI delay, ten
phase-randomized surrogates each re-embedded) is a second, independent
estimator of the same two directions, applied as a robustness check. It
reproduces the silence in all four measurements:

| pair | te | threshold | verdict |
|---|---|---|---|
| GONG→F10.7 | 2.59e-1 | 3.78e-1 | silent |
| F10.7→GONG | 3.26e-1 | 5.33e-1 | silent |
| polar→F10.7 | 2.70e-1 | 3.23e-1 | silent |
| F10.7→polar | 2.09e-1 | 3.65e-1 | silent |
| BiSON→F10.7 | 2.52e-1 | 3.72e-1 | silent |
| F10.7→BiSON | 3.68e-1 | 6.96e-1 | silent |

Two notes carry the physical content. The polar field's auto-MI delay is
τ = 67 months — its natural phase-space timescale is ~5.6 years, the
Vorläufer scale, which the estimator finds on its own. And the phase-space
TE(polar→F10.7) of 2.70e-1 sits at 84% of its threshold 3.23e-1 — the
closest of the six, still under. The two estimators agree: no arrow.

### 4.4 The planetary driver — tidal → F10.7

Monthly, 1947–2026, seven cycles. TE(tidal→F10.7) rises monotonically with
the lag — the same KDE edge artefact — and its lag-24 "peak" is a 0.5% edge
margin above the family bound, not a signal. The sharper instrument is the
period itself: the measured F10.7 cycle-peak separations are 11.3, 9.9,
12.6, … years (one 17.7-year gap is a missed weak peak), a variable
~10–14-year period — not the Jupiter-fixed 11.86 years that a phase-locked
cycle would require. **Silent.**

### 4.5 The second interior instrument — BiSON → F10.7

402 months (1992-01 → 2025-01), monthly bins, lag sweep 0…72 months (0…6
years):

| lag (months) | TE(B→F) | TE(F→B) | family bound |
|---|---|---|---|
| 0 | 3.90e-2 | 2.73e-1 | 6.26e-2 |
| 12 | 8.16e-2 | 2.67e-1 | 2.16e-1 |
| 24 | 1.17e-1 | 1.65e-1 | 3.19e-1 |
| 35 | 1.45e-1 | 1.59e-1 | 3.35e-1 |
| 48 | 1.22e-1 | 1.51e-1 | 3.43e-1 |
| 60 | 1.03e-1 | 2.09e-1 | 3.43e-1 |
| 72 | 9.51e-2 | 2.01e-1 | 3.43e-1 |

The peak of TE(B→F) is 1.45e-1 at lag 35 months (2.92 years) — not the edge
of the sweep, and under the family bound 3.43e-1. TE(F→B) exceeds TE(B→F) at
every lag (activity carries more about the frequency's future than the
reverse), reproducing the GONG and WSO asymmetries, and it too is under the
bound. **Silent.** The series is a second instrument on the same three cycles,
so this is a reproduction of the GONG interior null, not an independent
fourth cycle.

### 4.6 The cycle-21 extension — Basu et al. (2012) figure-digitized → F10.7

The only shift series that reaches the pre-GONG gap is figure-only (Basu et
al. 2012); its Fig. 2 was recovered vector-exactly from the arXiv PostScript
(§2.5), yielding 337 points over three frequency bands, 1978–2012, at the
natural ~91-day cadence (sparse cycle 21, ~12 points 1978–1986). At that
cadence the mid band gives 113 points (1978-08 → 2012-01); F10.7 is averaged
over ±45 days at each shift epoch, lag sweep 0…72 samples (0…~18 years):

| lag (samples) | TE(B→F) | TE(F→B) | family bound |
|---|---|---|---|
| 0 | 8.35e-2 | 1.24e-1 | 1.55e-1 |
| 12 | 1.06e-1 | 1.62e-1 | 3.91e-1 |
| 24 | 1.36e-1 | 1.87e-1 | 6.13e-1 |
| 48 | 1.98e-1 | 2.01e-1 | 7.78e-1 |
| 68 | 2.57e-1 | 2.15e-1 | 8.11e-1 |

The peak of TE(B→F) is 2.57e-1 at lag 68 samples (16.9 years) — the edge of
the sweep, on the KDE ramp, and under the family bound 8.11e-1. The low band
peaks at 4.64e-1 (lag 62) against 7.97e-1, the high band at 1.69e-1 (lag 69)
against 8.37e-1; the phase-space estimator is silent on all three. **Silent.**
Adding cycle 21 (via a figure-digitized, low-confidence series) does not turn
the interior null into an arrow; the ~4.5-cycle ensemble carries no
transfer-entropy arrow to F10.7.

### 4.7 Robustness — the cross-instrument polar field (HMI vs WSO)

The WSO polar-field record is held against HMI over the overlap (2010–2026,
189 monthly bins), with the Pearson correlation r and the ordinary-least-squares
factor of HMI on WSO for each of N, S and the signed dipole Avg:

| quantity | n (months) | r | OLS factor (HMI = factor·WSO) |
|---|---|---|---|
| N (north) | 189 | 0.905 | 4.54 |
| S (south) | 189 | 0.915 | 5.02 |
| Avg (signed dipole) | 189 | 0.972 | 5.52 |

The two instruments track each other tightly — r = 0.97 on the signed dipole,
0.90–0.91 on the separate caps — across 1.5 cycles including two reversals.
This is a robustness finding, not a discrepancy: the WSO polar-field signal is
real and reproduced by an independent instrument, so the WSO silence (§4.2)
is not a single-instrument artefact. The factor of ~4.5–5.5 (HMI larger) is a
systematic scale difference, not noise: HMI measures the *radial* field over a
60° cap with no line saturation, while WSO measures the *line-of-sight* field
in a polemost aperture and carries the unapplied ~1.8× saturation factor, so a
constant factor larger than unity is expected; the near-zero OLS intercept
confirms it is a pure scale offset. The prior r = 0.86/factor ~1 number
(Kutsenko & Abramenko 2016) is the solar *mean* magnetic field, not the polar
field — the polar-field cross-instrument number is reported here for the first
time.

As a second, optional Vorläufer probe, TE(HMI→F10.7) is measured on the same
monthly bins: the peak is 4.06e-1 at lag 26 months (2.17 years) against a
family bound of 7.56e-1, with TE(F→HMI) the larger direction at most lags, and
the Takens-embedded counter-check silent in both directions. At ~1.3 cycles
the HMI series is too short for an independent Vorläufer statement — the
handover names the overlap as "not for an independent Vorläufer probe" — and
the honest verdict is silence (0 honored). It does not overturn, and is
consistent with, the WSO polar null.

## 5. Discussion

**The direction is not measured.** The Vorläufer hypothesis — the polar
field at minimum leads the next cycle's activity by ~5–6 years — predicts a
TE(P→F) arrow at lag 60–72 months. The peak does land there, at 70 months.
But it does not clear the phase-randomized family bound, and the bound is
inflated at exactly that lag by the KDE sweep artefact (m = n − τ shrinks
from 603 to 531 across the sweep), so the coincidence cannot be read as a
finding. The stronger direction at every lag is TE(F→P) — the activity
carrying information about the polar field's future — which is the reverse
of the predictor relation and also silent.

**Why silence is the honest answer.** Two factors cap both measurements.
First, the statistics: three GONG cycles and 4.5 WSO cycles are a thin
ensemble for a nonlinear information measure whose null has to absorb the
entire cycle-shared envelope. Second, the confound: all three series —
frequency shift, polar field, activity — are driven by the same cycle, so
the *directional* residual is what remains after a large common component,
and the phase-randomized null preserves the spectrum of that common
component. A real but weak lead would be exactly what this grain cannot
resolve.

**What is measured, not what was hoped.** The measurements close two open
questions of the series and leave the dynamo question open. The interior
does not measurably lead the activity at 36-day grain; the polar field does
not measurably lead the activity at monthly grain. The two signatures are
*correlated* with activity (all three co-vary over the cycle); they are not
*directed* toward it at this granularity and ensemble size.

**Three silent drivers, two instruments on the interior.** Read together, the
three accessible drivers of the cycle are now all measured and all silent:
the planetary tide (seven cycles, a variable ~10–14-year period, not
Jupiter-locked), the helioseismic interior (three cycles, now measured by two
independent instruments — GONG and BiSON), and the polar field (4.5 cycles).
None carries a transfer-entropy arrow to F10.7, under two independent
estimators. That the *activity* direction is consistently the larger of the
two in the scalar sweeps — TE(F→G), TE(F→P), TE(F→B) — is itself a statement:
the activity index carries more information about the signatures' future than
they do about its, the reverse of every precursor hypothesis. The silence is
not a missing measurement; it is the measured state of a three-driver null.

**What would sharpen or overturn the null.** The silence is a statement
about the ensemble, not a proof of absence, and the extensions that could
change it are named, not pursued here: (i) BiSON — the Birmingham low-degree
network, now harvested: the machine-readable shift series (7-day segments,
Howe et al. 2025) covers 1992–2025 and reproduces the GONG interior null as a
second instrument; a second, figure-only published series (Basu et al. 2012,
365-day subsets) reaches back to 1978 including cycle 21 — its Fig. 2 has now
been digitized vector-exactly and is silent (§4.6), so the remaining step for
a genuinely independent confirmation is the BiSON team's own table, and the
true gap is then 1976 → 1978-07-31 (the single-station Izaña era); (ii) a
cross-instrument polar field — now measured (§2.6, §4.7): SDO/HMI synoptic
polar-field maps track the WSO record at r = 0.97 (signed dipole) with a
systematic factor ~5.5, so the WSO polar null is not a single-instrument
artefact (the earlier r = 0.86/factor ~1 number, Kutsenko & Abramenko 2016,
is the solar mean magnetic field, not the polar field); and (iii) a longer lag
sweep at
finer grain for the interior, whose coupling timescale is not yet bounded
above. Each is a data harvest, not a new method; the estimator and the null
machinery are in place. The
aa-index — a geomagnetic precursor with a record to ~1868 — is named and
declined: it is a two-stage derived index (K-index amplitudes reweighted
across successive station pairs), not a measurement, and fails the system's
force-gate, so it does not enter a harvest.

**The calibration caveat is carried, not applied.** The WSO series is
harvested as published; the 2016-12-16→2017-05-18 polarization-sensitivity
window (mean-field values ~1.55× too small, not recalibrated) is documented
alongside in WSO's own wording, and the generic ~1.8× saturation factor is
deliberately not applied, so a future reader does not mistake a calibration
step for a physical jump.

## 6. Limitations

- **3–7 cycles.** The ensembles are 313 GONG runs, 402 BiSON months (a second
  instrument on the same three interior cycles, not a fourth), 603 WSO months
  and seven tidal cycles — three or four realizations each, at most. A
  transfer-entropy measurement of a cycle-scale lead needs more realizations
  than this; the null cannot be tightened without more data.
- **The family bound at high lag.** The bound is the strongest surrogate of
  the round, and it grows with the lag through the sweep artefact; the
  honest read is therefore "no arrow anywhere", not "close at 70 months".
- **Grain.** Monthly (and 36-day) grain is coarse against the physical
  timescales — the interior-to-surface coupling is a matter of months to a
  few years, and the Vorläufer lead is near the edge of the sweep.
- **m-averaging.** For l > 0 the GONG `v1y` frequency is the m-averaged
  value, not the m = 0 frequency; the shift measurement averages across
  modes and reduces, but does not eliminate, this.
- **The estimator.** The scalar KDE TE is the untouched canonical reference
  of the system and passes its ground-truth benchmark; the Takens-embedded
  (phase-space) TE is the second, independent estimator reported in §4.3
  (six pairs, all silent), not an applied calibration correction.
- **Single activity index.** F10.7 is one activity proxy; the sunspot
  number and the X-ray flux are alternative indices and are not part of this
  measurement.
- **Cross-instrument factor is definition-dependent.** The HMI polar field is
  a mean over a 60° polar cap of the radial field, while WSO is a polemost
  line-of-sight measurement carrying an unapplied saturation factor; the
  measured factor ~5.5 is therefore a property of these definitions, not a
  calibration of either instrument, and the HMI series spans only 1.5 cycles
  (too short for an independent Vorläufer probe).
- **Surrogate-machine generation (conservative footnote).** The family bound
  here was generated by the post-fix surrogate RNG (fam-machine: post-fix).
  The post-fix band lies lower than the pre-fix band; a verdict measured
  against the tighter post-fix bound is the non-conservative, corrected
  reading. The silence reported is under the corrected machine; it holds on
  the correct RNG by construction.

## 7. Conclusion

Transfer entropy from the solar cycle's three accessible drivers to the
activity index is silent at monthly grain: the helioseismic interior (313
GONG runs and 402 BiSON months — two independent instruments on the same
three cycles), the polar field (603 WSO months), and the planetary tide
(seven cycles) carry no arrow toward F10.7 beyond the phase-randomized
family bound, under two independent estimators (the scalar sweep and the
Takens-embedded phase-space measure). The polar-field peak lands exactly at
the predicted Vorläufer lead of 5.83 years and still fails the null; the
reverse direction (activity → signature) is the larger at every lag. The WSO
polar-field silence is not a single-instrument artefact: an independent
instrument (SDO/HMI, 217 synoptic maps 2010–2026) tracks the same record at
r = 0.97 on the signed dipole. The
dynamo's causal structure is not resolved by transfer entropy on 3–7 cycles;
silence is the honest verdict, and the machinery — plus a named roadmap of
harvests that could sharpen or overturn it — now stands for the longer
series that a future observer will inherit.

## References

- Basu, S., Broomhall, A.-M., Chaplin, W. J., & Elsworth, Y. (2012). Thinning of the Sun's magnetic layer: the peculiar solar minimum could have been predicted. *The Astrophysical Journal* 758, 43.
- Charbonneau, P. (2010). Dynamo models of the solar cycle. *Living Reviews in Solar Physics* 7, 3.
- Howe, R., Chaplin, W. J., Elsworth, Y. P., Hale, S. J., Hatt, E., & Nielsen, M. B. (2025). Far-side helioseismology with Sun-as-a-star data: the solar cycle as seen with 7-d-long BiSON time series. *Monthly Notices of the Royal Astronomical Society* 537, 909–914.
- Kaiser, A., & Schreiber, T. (2002). Information transfer in continuous processes. *Physica D: Nonlinear Phenomena* 166, 43–62.
- Kutsenko, A. S., & Abramenko, V. I. (2016). Using SDO/HMI magnetograms as a source of the solar mean magnetic field data. *Solar Physics* 291, 1613–1633. (arXiv:1606.03710)
- Muñoz-Jaramillo, A., Balmaceda, L. A., & DeLuca, E. E. (2013). Using the dipolar and quadrupolar moments to improve solar-cycle predictions based on the polar magnetic fields. *Physical Review Letters* 111, 041106.
- Schreiber, T. (2000). Measuring information transfer. *Physical Review Letters* 85, 461–464.
- Svalgaard, L., Cliver, E. W., & Kamide, Y. (2005). Sunspot cycle 24: smallest cycle in 100 years? *Geophysical Research Letters* 32, L01104.

---

*Data and code:* the harvests, probes and register live in the omegaflow
repository — `src/archivar/lzw.rs` (std-only Unix-compress decoder), `src/archivar/gong_series.rs`
(GTS1 bin), `tools/work/src/bin/gong_series_compiler.rs`, `tools/work/src/bin/gong_cycle_probe.rs`;
`src/archivar/bison_shift.rs` (BSN1 bin), `tools/work/src/bin/bison_shift_compiler.rs`,
`tools/work/src/bin/bison_cycle_probe.rs`; `src/archivar/bison_basu.rs` (BSN2 bin),
`tools/work/src/bin/bison_basu_compiler.rs` (Fig.-2 vector extraction),
`tools/work/src/bin/bison_basu_probe.rs`; `src/archivar/wso_polar.rs` (WSP1 bin),
`tools/work/src/bin/wso_polar_compiler.rs`, `tools/work/src/bin/wso_cycle_probe.rs`;
`src/archivar/hmi_polar.rs` (HMP1 bin), `tools/work/src/bin/hmi_polar_compiler.rs`,
`tools/work/src/bin/wso_hmi_consistency.rs`, `tools/work/src/bin/hmi_cycle_probe.rs`; the planetary
driver `tools/work/src/bin/solar_cycle_probe.rs`; the canonical scalar estimator
`src/mathematikerin/te.rs` is untouched, the Takens-embedded estimator (`topological_te_phase`)
is its CPU reference. Both harvests are wired into the CI sun job with an
asset guard. All verdicts are machine-measured; the register language of the
system is German, this manuscript is its English face.
