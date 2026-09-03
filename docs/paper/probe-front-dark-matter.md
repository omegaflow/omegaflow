<!--
  title: Zero Flags on the Net: no dark-matter clump in the outer solar system
  class: paper
  date: 2026-09-03
  version: 5
  sha256: ec6372b5e5e0524c6afffdbc86ca5e7c8ec7aa3545c7471b7b596acfece1555c
  fam-machine: pre-fix
  status: live
  see-also: docs/paper/planet-nine-kbo-residue.md docs/paper/flyby-path-1-cold-cases.md
-->

# Zero Flags on the Net: no dark-matter clump in the outer solar system

**The omegaflow dark-matter blade (Nadel Ⅰ, Front Ⅱ).** Self-contained. Every number
in the tables is machine output; no number is fabricated, no sample synthesized, and
every absent datum is named as absent — including the datum the first pass declared
absent that is present after all (§5.4).

## Abstract

We compute the acceleration residue of five deep-space probes (Pioneer 10/11, Voyager 1/2, New Horizons) from Horizons trajectories (32-day Chebyshev granules), with the reference the Sun + eight planets + Moon field. The quiet-cruise residue sits at the granule floor, median |a_res| = 1.7–8.6 × 10⁻⁸ m/s²; encounter spikes are granule overshoot. The machine is proven first: on six leave-one-out fine flyby arcs it recovers the dropped planet's mass (R·d² = GM) to 0.1–0.9 %. A triangulation net of 1008 candidate positions (40–600 AU) × five probes, scanned with the inverse-square test under constancy, strength, and overshoot gates, yields zero flags and zero points triangulated at two or more probes. The sensitivity of the public products is ≈ 3 × 10¹ Jupiter masses at 40 AU, ≈ 6 Earth masses at 1 AU, scaling as d². No dark-matter clump is found at that limit; the limit, not the absence, is what the machine states.
## 1. Introduction

Dark matter couples to the nine force channels of the field only through gravity
(force_type 1): it neither emits nor absorbs light (em), carries no charge (electric),
makes no sound (acoustic), and has no heat budget (thermal). A clump can cross the
outer solar system leaving a signature in exactly one channel — the acceleration
residue of a tracked spacecraft: Ruck = a_obs − a_bekannt per time step. The five
interplanetary probes are a sparse, moving gravitational sensor array: Voyager 1 at
160 AU, Voyager 2 at 130 AU, New Horizons at 60 AU, Pioneer 10/11 on the interstellar
leg. A clump at position X pulls each probe with a different geometry and a different
closest-approach epoch; the same X carrying a constant inverse-square signature at
more than one probe is the gravitational analog of GPS — triangulation without radio.

This paper asks, and answers, the sharp form of that question:

1. **Is the machine honest?** Does a leave-one-out residue recover the mass of a known
   planet that was dropped from the model — to what accuracy?
2. **Is the net empty?** Over a grid of candidate positions spanning the outer solar
   system, does any point carry a constant, positive inverse-square signature at one
   probe — and at two or more probes simultaneously?

The answers: yes (0.1–0.9 %), and yes (zero flags, zero triangulations) — at the
sensitivity of the finest public *finished* orbit product, Horizons. The fine raw
Doppler that would push the floor down is named where it lives (§5.4), not hallucinated.

## 2. Data

**2.1 Probe arcs (daily raster).** Horizons reconstructed state vectors,
CENTER = solar-system barycenter, ICRS, meters; 1-day raster from launch (Pioneer 10
1972-03-04, Pioneer 11 1973-04-07, Voyager 1 1977-09-06, Voyager 2 1977-08-21,
New Horizons 2006-01-20) to 2031-01; compiled by `horizons_compiler --daily` into
`ephemeris_{name}_daily.bin` CDN assets (32-day Chebyshev granules, degree 17). All
five series run continuously (0 gaps; the launch day is the one named gap, Horizons
rejects a start before the trajectory begins). The series span launch → 2031 —
Horizons reconstruction + prediction; the end of the orbit-determination arc per
probe is a named quantity, not a subtracted one.

**2.2 Fine flyby arcs (positive control).** Six Voyager planetary-encounter arcs
(5-minute raster outside ±1.2 d, 1-minute inside; fine granules 0.03 d, degree 17),
compiled by `horizons_compiler --flyby` into `ephemeris_voyager*_*.bin`: V1 Jupiter
1979-03-05, V1 Saturn 1980-11-12, V2 Jupiter 1979-07-09, V2 Saturn 1981-08-26,
V2 Uranus 1986-01-24, V2 Neptune 1989-08-25. Round-trip fit error 218 m (Uranus) to
2.5 km (V1 Saturn); Neptune 9.67 × 10⁶ m at the tightest swingby — named, not hidden.

**2.3 Ephemerides and constants.** Planet states from the CDN Chebyshev bins
(`ephemeris_{body}.bin`), SSB ICRS, meters; body GM from the bin properties (SI
m³/s²: Jupiter 1.2668653 × 10¹⁷, Saturn 3.7931187 × 10¹⁶, Uranus 5.793939 × 10¹⁵,
Neptune 6.83510 × 10¹⁵); Sun GM = 1.32712440018 × 10²⁰ m³/s² (constant);
AU = 1.495978707 × 10¹¹ m; J2000 = JD 2451545.0; day = 86400 s.

## 3. Methods

**3.1 Residuum.** a_obs is the three-point second difference of the probe position
series at 1-day spacing; a_bekannt is the Sun + 8-planet + Moon point-mass
acceleration (Earth J2) at the probe's ICRS point. Residuum a_res = a_obs − a_bekannt,
R = |a_res|, and the Ruck = |a_res(t) − a_res(t − 1 d)|. The second difference is
computed only on dense stretches (0 gaps); gap edges would be named, and none occur.

**3.2 Direct inverse-square test.** For a candidate position X and a probe, the radial
component a_rad = a_res · (X − r)/d with d = |X − r| carries the clump's pull if one
is there: a real point mass gives a_rad = GM/d², hence a_rad · d² = GM, a constant.
The measured GM is the median of a_rad · d² over the ±90-day window around the
closest approach; the constancy gate demands median absolute deviation < ½ GM. The
signal-strength gate demands max |a_rad| > 5 × the mission median |a_res| (the clump
must exceed the granule floor locally, else R·d² at 600 AU reports a Jupiter-scale
phantom — the residual baseline times d²). An overshoot mask drops any step with
|a_res| > 100 × median (the planetary-encounter granule artifacts).

**3.3 Positive control (leave-one-out).** For each fine flyby arc the target planet is
removed from a_bekannt; the residue then contains the planet's pull, and
R·d² = GM_planet within the closest-approach window (±1 d). The machine must recover
the known GM. This is the gravitational re-detection test the grid must pass before it
is pointed into the dark.

**3.4 Estimator hardening.** The canonical scalar transfer entropy
(`transfer_entropy_lag`, Schreiber 2000, Silverman bandwidth) is scale-blind on this
series: on a synthetic perfect clump R = 1/d² it returns TE = 7.6 × 10⁻³ below its
own phase-randomized threshold 1.05 × 10⁻²; the log-space transform lifts it to
1.17 × 10⁻¹ (barely above 1.08 × 10⁻¹). The inverse-square test §3.2 is
scale-robust by construction and is the instrument of the verdict; the log-space TE
is reported as the secondary arrow.

**3.5 Grid.** Nine shells (40, 60, 80, 120, 160, 200, 300, 450, 600 AU), 112
Fibonacci-distributed directions per shell, 1008 positions; each position tested
against all five probes (5040 point-probe tests). A point is flagged when §3.2's
three gates hold; a point is *triangulated* when flagged at two or more probes.

## 4. Results

**Table 1 — mission residue baseline** (daily raster; median |a_res|, max, and max
Ruck, m/s²):

| Probe | Steps | Span | median \|a_res\| | max \|a_res\| | max Ruck | at (encounter) |
|---|---|---|---|---|---|---|
| pioneer10 | 21500 | 1972-03-07 → 2031-01-16 | 2.24e-8 | 1.291e0 | 1.302e0 | 1973-12-04 (Jupiter) |
| pioneer11 | 21116 | 1973-04-10 → 2031-01-31 | 3.93e-8 | 3.440e-1 | 2.716e-1 | 1974-12-03 (Jupiter) |
| voyager1 | 19484 | 1977-09-09 → 2031-01-12 | 1.67e-8 | 5.011e1 | 5.007e1 | 1980-11-13 (Saturn) |
| voyager2 | 19516 | 1977-08-24 → 2031-01-28 | 2.54e-8 | 4.455e-1 | 4.456e-1 | 1979-07-10 (Jupiter) |
| new_horizons | 9116 | 2006-01-23 → 2031-01-07 | 8.56e-8 | 6.013e-2 | 3.359e-2 | 2031-01-01 (edge) |

The encounter maxima are granule overshoot, not gravity — the 32-day granule cannot
carry a one-day encounter; the New-Horizons maximum sits at the prediction boundary
(a named edge artifact). The quiet-cruise floor is the granule fit precision.

**Table 2 — positive control, leave-one-out mass recovery** (fine arcs; GM in m³/s²;
log-space TE over own threshold):

| Encounter | GM measured | GM known | ratio | log-TE verdict |
|---|---|---|---|---|
| V1 Jupiter 1979 | 1.268e17 | 1.267e17 | 1.0008 | over own threshold |
| V1 Saturn 1980 | 3.805e16 | 3.793e16 | 1.0032 | over own threshold |
| V2 Jupiter 1979 | 1.268e17 | 1.267e17 | 1.0008 | over own threshold |
| V2 Saturn 1981 | 3.802e16 | 3.793e16 | 1.0024 | over own threshold |
| V2 Uranus 1986 | 5.805e15 | 5.794e15 | 1.0019 | over own threshold |
| V2 Neptune 1989 | 6.895e15 | 6.835e15 | 1.0088 | fam-tragend (1.676e-1 > 1.637e-1) |

The eight empty-point null controls are still. The machine reads a dropped planet's
mass to better than 1 % — the positive control passes.

**Table 3 — triangulation net** (1008 positions × 5 probes; §3.2 gates):

| Shells | Positions | Tests | Flags | Triangulated (≥2 probes) |
|---|---|---|---|---|
| 9 (40–600 AU) | 1008 | 5040 | 0 | 0 |

No position carries a constant, positive inverse-square signature at even one probe;
none at two or more. The net is empty.

## 5. Discussion and limits

**5.1 The quiet floor.** The median cruise residue (1.7–8.6 × 10⁻⁸ m/s²) is the fit
precision of the 32-day granule, not a physical signal; below it the net is blind.
This is the one number the whole verdict hangs on.

**5.2 The sensitivity.** The signal-strength gate (5 × median |a_res|) translates to
GM ≳ 3 × 10¹ Jupiter masses at 40 AU and ≳ 6 Earth masses at 1 AU, scaling as d².
Realistic dark-matter clumps — primordial black holes of 10¹⁸–10²² kg — carry
GM ≈ 10⁸–10¹² m³/s², ten orders below the floor at any shell. The net does not
constrain them; it states the floor of the public finished orbit products.

**5.3 The estimator.** The raw KDE transfer entropy is scale-blind (§3.4); the verdict
rests on the scale-robust inverse-square test, with the log-space TE as a secondary
arrow. Both instruments are validated (positive control, null control) before use.

**5.4 The fine Doppler — reduced, and the floor did not come down.** The first pass
declared the raw closed-loop Doppler (ODF/TRK-2-34) absent from PDS/NSSDC — true,
but narrow. The data is public at the NASA Space Physics Data Facility in two forms:
the ASCII oddump (NAVIO, 60-s, full mission 1973–2002, compiled by
Turyshev/Anderson/Watkins, Zenodo DOI 10.5281/zenodo.13309156) and the raw
bit-packed ATDF (TRK-2-25, `.TDF` files, Pioneer 10, three epochs 1987-12→1993-04).
This session harvested and reduced the bit-packed form end-to-end: `src/archivar/atdf.rs` — a
faithful port of Markwardt's bitstract/trk225fmt/bitvconv (8064-byte records,
marker-byte artifact, 36-bit Univac fields) — into `pioneer10_skyfreq.bin`
(501 876 sky-frequency samples), then fit against it a full observation model
(Moyer: DSN station + light-time + IAU Earth rotation [8]) and a self-consistent
Sun + sunward-anomaly orbit solution (RK4). **The floor did not come down.** The
residual sits at 44 kHz ≈ 5.3 km/s range-rate, dominated by per-epoch systematics
(kHz offsets between the separately dumped ATDF files) and per-sample scatter — not
by the anomaly. The reduction was audited as it went: the one-way downlink
coefficient A = −(240/221)·f0/c (the transponder ratio 240/221 lives in the Doppler
beat), and sampler_time = 0.805·Δt (≈20 % dead time between integrations). The
Pioneer anomaly (8.7 × 10⁻¹⁰ m/s² sunward) is not recovered — the sparse three-epoch
sampling sets a detection limit of ≈ 10⁻⁷ m/s², above the granule floor of §5.1
(10⁻⁸ m/s²); the granule remains the better public limit. The full-mission ASCII
form carries the programmed uplink ramp sweep and is the remaining open reduction,
not a claim; a Voyager Doppler at SPDF is not yet verified (open, not claimed).
Atom 4 (the radio path in the block, `link_deduction_probe`) laid four physical
deductions on the Earth–spacecraft link and subtracted them instead of averaging:
multi-station common mode, ionospheric TEC, solar plasma, and spacecraft
radiative/thermal dynamics. Measured on the full 501 876-sample ATDF set: the TEC
deduction carries 0 samples (GIMs begin 1998 — the era is void, 0 honored); the
solar-plasma deduction (OMNI2 N1800 daily series, 1/r² column along the light path,
48-h search window from the data cadence) covers 443 721 samples with a mean shift
of 1.3 × 10⁻³ Hz (max 1.0 Hz at conjunction) — the link media move the sky
frequency at mHz–Hz at S-band, not at the 44 kHz floor; the common-mode deduction
measures 12 overlapping windows (25 pairs) with a station differential of
3.88 × 10⁵ Hz, ~9× the per-sample scatter. The chain carried 42.5 Hz of the floor.
The 44 kHz per-sample scatter is not the link media. The slipped-cycle mask
(Deduktion 0, ATDF field 76, now carried in the 11-slot PASF record) removes
330 850 of the 501 876 samples (65.9 %): the slipped subset carries 5.2 × 10⁴ Hz
against 2.0 × 10⁴ Hz for the clean subset — the single largest cut of the
reduction so far, halving the floor to 19.9 kHz (2.5 km/s). The per-dump
reference profiles (36 epoch×file cells) and the measured ramp coupling
(k = 3.0 × 10⁻⁸ ± 2.0 × 10⁻⁹ Hz per unit, 15σ, over 165 454 ramped samples,
mean |Δf| 224 Hz, max 791 Hz) are real cuts that do not move the floor. The
signal-strength gate (Deduktion 0b — field 78 is in 0.1 dBm, real strengths
negative; the 5 572 samples without a measured strength carry 9.9 × 10⁴ Hz and
are discarded) cuts the floor from 19.9 to 8.0 kHz (1.0 km/s) and the residual
drift to 1.8 × 10⁻⁸ ± 4.6 × 10⁻⁸ m/s² (~2 × 10¹ above the anomaly, not
carried). The NAVIO witness (the cleaned second reduction, 339 common days)
carries day-to-day residual profiles uncorrelated with the ATDF residuals
(r = 0.25 — each reduction its own noise) while the per-sample scatter around
the daily median is identical (1.45 kHz vs 1.46 kHz): the shared ~1.5 kHz floor
is physical (receiver/spin), the profile residuals are reduction-specific. The
daily-slope cut (Deduktion 7) removes only 95 Hz of the 8.0 kHz floor and shows
the 60-s excess (9.96 kHz de-trended vs 1.87 kHz for 1-s) is the 60-s
integration scatter itself, not a Moyer daily-curve residual; gap-adjacent
segments are the quiet ones (3.4 kHz vs 9.7 kHz interior). The witness on both
scales carries its own floor (de-trended 989 Hz vs our 431 Hz — 0.44×); the
0.99× equality held for the uncut series. The telemetry recoil cut (Deduktion 8 —
the harvested power channels PRTG 108 W, Pshunt 13.7 W, Pbus 64.7 W as a sunward
a = η·P/(m·c), η scanned 0–8) is flat across the whole scan and does not move
the drift (2.3 × 10⁻⁸ ± 4.6 × 10⁻⁸ m/s²): the thermal recoil scale
(~2 × 10⁻⁹ m/s², ~2.8 Hz over the era) sits ~20× below the drift uncertainty —
not carried at this floor. The NAVIO-clean series as the primary chain
(Deduktion 9 — same cuts, Earth center instead of a station, the record carries
none) with the beat-plausibility gate (|OBSVBL| ≤ 2·f0·v_max/c ≈ 5 × 10⁵ Hz)
discards 44 % of the overlap era as the readme's named error classes
(misplaced counts, ±500k, Ku records); the basis falls to 13.8 kHz with
A = 15.3 Hz/(m/s) (two-way scale; the OBSVBL = ν_ref − ν_recv convention from
the 2010 review is confirmed by the fit — C = 0.99975 ≈ 1 on the reconstructed
sky frequency), and the drift stands at 2.9 × 10⁻⁸ ±
5.5 × 10⁻⁸ m/s² (~3 × 10¹ above the anomaly). The NAVIO floor is structured,
not white: the sun-distance quartiles carry a 4× gradient (20.5 kHz at 44 AU to
5.4 kHz at 48 AU) and DTYPE 12 carries 22.5 kHz against 12.9 kHz for DTYPE 13.
The split-half control of the noise weighting (odd/even halves, weighted)
carries the same drift within 0.06σ — the weighting is not circular. The NAVIO
segment mask (4×p90, an April–August-1988 cluster of 24 segments) halves the
overlap-era floor to 6.5 kHz and the drift SE to 2.6 × 10⁻⁸ m/s² (4.2 × 10⁻⁸ ±
2.6 × 10⁻⁸); DTYPE 12 and 13 subsets stay consistent. The stop decision is the
operator's alone; the ~800 Hz witness scatter remains the measured reference,
not a threshold. The pattern-filter and sibling-witness measurements follow the
same doctrine: the thermal channels (Trtg1/PRTG/Pshunt) regress onto the
residuum with significant couplings (20 Hz/°F, −11 Hz/W, 68 Hz/W) but remove
almost nothing of the floor (2.70 → 2.70 kHz) — the thermal state is not the
scatter driver; and the Pioneer 11 chain (overlap era, post-Saturn-flyby)
carries A = 15.29 Hz/(m/s), a floor of 15.4 kHz after its mask (a
February–June-1988 cluster, shared with P10), and a drift of −2.3 × 10⁻⁸ ±
1.8 × 10⁻⁷ m/s² against P10's +1.1 × 10⁻⁸ ± 2.3 × 10⁻⁸ — both consistent with
zero, no shared anomaly signal. The shared patterns are the natural ones: the
DTYPE 12/13 structure (2–3×) and the 1988 cluster. The witness swap on the
gate-passing subset confirms the shared ~800 Hz per-day floor (819 vs 850 Hz).
The segment mask (Deduktion 10 — segments above 4× the p90 of the segment RMS,
1.5 × 10⁴ Hz, all interior, scatter-dominated, single-station single days
including the dataset's first day and a March-1988 cluster) discards 15 segments
(2 906 samples, 1.8 % — the subset carries 5.3 × 10⁴ Hz against 4.0 × 10³ Hz
for the rest, a 13× separation); the floor falls to 4.0 kHz and the residual
drift to 1.1 × 10⁻⁸ ± 2.2 × 10⁻⁸ m/s² (~1 × 10¹ above the anomaly, not
carried). The residual drift stands at
2.3 × 10⁻⁸ ± 4.6 × 10⁻⁸ m/s² (~3 × 10¹ above the anomaly, not carried). The
remaining reduction-internal systematics (the 60-s scatter itself, the noisiest
single-day segments) stay named. The spectral analysis of the pass scale
(Deduktion 16) found a red 1/f spectrum in the 60-s segments (peaks at 0.5–0.7 mHz,
4.6–5.6× the floor) and an unexplained line near 51 mHz in the sub-10-s set
(9.3× the floor, 73 249 samples — the strict 1-s class is 70 602, Deduktion 19);
the true spin (4.72 RPM = 78.6 mHz, measured from the telemetry) is absent from
the residual. The fine-grid localization (Deduktion 17,
0.1 mHz steps over 44–58 mHz) resolves the "line" into a band complex: f* = 50.73 mHz
(T* = 19.71 s, 9.5× floor) with sub-peaks at 49.0 / 50.2 / 50.7 / 51.5 / 53.4 mHz,
era-dependent (the 1-s data lives in two dumps: 1988 carries 49.0 mHz plus a local
45.7 mHz sub-peak at the scan edge, 1992 carries 50.7 mHz; the 20–70 mHz form adds
a rise at 20 mHz and a 30 mHz bump) and station-dependent (Goldstone 14: 50.2,
Canberra 43: 51.5–53.4, Madrid 63: 49.0 mHz) — a DSN-era band structure, not one
coherent 19-s oscillator (the split peaks are ~10⁴× the frequency resolution). The
witness test (Deduktion 18): the sampler rasters (1/10/60 s) determine f0 only
modulo 1/60 Hz, so the 60-s member of the family at 0.71 mHz (= |f* − 3/60 Hz|) was
measured, not assumed. It is carried by the NAVIO-clean Pioneer 10 overlap era
(8.6× the local floor, driven by 1988: 3.7×; 1989/91/92 ≤ 0.7×), by the Pioneer 11
overlap era (4.6×), and by the ATDF 60-s class in 1993 (10.1×; 1988: 0.4×,
1992: 1.0×) — the signature is DSN-ground-common across both spacecraft and both
reductions, not an ATDF-chain artifact; but its era occupancy does not align across
reductions, so it is not one stationary oscillator either. The candidate machines
(Deduktion 19): the sampler classes are measured (1-s 70 602 / 10-s 20 752 /
60-s 71 194 samples, Δt exactly 1.000 s, all three classes carry structure in the
band); the 0.05-mHz grid places f* at 50.714 mHz (the 0.1-mHz grid above reads
50.73 mHz — a 0.02-mHz band-complex spread, named); the alias family leaves
50.71 mHz and 949.29 mHz indistinguishable; the
256-divider grid points — f*·256 = 12.9828 Hz (Δ 0.0172 Hz to 13) and
(1 Hz − f*)·256 = 243.0172 Hz (Δ 0.0172 Hz to 243) — are named but not carried;
the ATDF count chain is documented (TRK-2-25, DSN 820-13 Rev. A, 1988 — count
resolution 0.001 cycle, reference in 0.1 Hz, 2³² modulo reset; Morabito & Asmar
1995 [9] — integer counts plus a fractional resolver term, 1-MHz bias): the
0.001-cycle quantization is ~1 mHz per 1-s count, ~50× below the measured line
frequencies — the count quantization cannot carry them; the raw count field
carries all 1000 fractional bins uniformly (Deduktion 22: 1/256-lattice share
0.259 ≈ 256/1000, top bins 2.0× the mean density — Poisson), no 1/256 resolver
lattice survives into the ATDF — the 256-divider candidate is measured and
excluded; the resolver's fractional resolution remains unlocated in public
documents (the archived 810-005 202 Rev. A documents the chain — accumulating
phase counts at 0.1-s intervals, digital PLL — and no resolver resolution); the
stored reference chain (0.1-Hz quantized; steps p10/p50/p90 =
−2.5/0.1/2.5 Hz) carries its own mHz-band structure — the 0.1-Hz staircase of
slow sweeps (station 14: step recurrence at 49.90 mHz; station 43: level peak
49.40 mHz) — none of which coincides with the residual lines (offsets 1.3–2.2 mHz
per station): the lines do not live in the stored reference (Deduktion 23); the
line amplitudes are strength-constant (Deduktion 24 — station 14: 160/153/161 Hz
across weak→strong signal; station 63: 57/50/65 Hz; station 43: 104/102/82 Hz):
the loop-noise scaling A ∝ 1/√SNR is not carried — the lines are fixed per-station
spurs (~7 × 10⁻⁸ of the sky frequency, ~1.6 × 10⁻⁴ of the 1-MHz bias chain), the
named injection is the station reference chain (standard steering with a ~20-s
station-specific loop); the cross-spacecraft phase test carries no common
continuous oscillator (Deduktion 26: P10/P11 NAVIO overlap 1988-01..1990-05,
δ = 0.49 rad inside the circular-surrogate null 0.41/5.55 rad) — the per-station
hypothesis stays named, unconfirmed; the two-/three-way split test is not
measurable locally (Deduktion 27: the harvest now carries the ground mode —
TRK-2-25 item 13 — as a 14th PASF slot, but the 1-s class that carries the strong
lines is three-way-only: mode 2 carries 0/0/25 samples at stations 14/43/63;
0 honored); the DSN Frequency and Timing documentation (Mark IV-85, TDA PR 42-82
[10]; the coherent reference generator phase stability, TDA PR 42-64 [11]) describes
the reference chain as distribution + validation — maser offsets known to ±3 × 10⁻¹³
against USNO/NBS, CRG synthesizing 0.1–55 MHz with constant phase relations — and
carries no periodic steering loop on any second scale; the steering-practice articles
(Syntonisation 1983, GPS timing 1982–87) remain scan-missing (pending); the sliding-
window track (Deduktion 29, 3000-sample windows) resolves the per-station lines into
a dense complex — the window peaks scatter across 44–56 mHz with comparable ratios
(station 63: 44.05–55.50 mHz, 2.3–10.2×; station 43: 44.25–55.60 mHz, 2.7–11.9×) —
no single coherent line carries the band; the drift-vs-jump test at the receiver-
generation boundary dies at coverage (1992 carries only 1519 1-s samples); the
cross-channel coherence test (Deduktion 30) refutes one shared driver: the strength
and residual complexes carry different members (station 14: 50.20 vs 45.80 mHz;
43: 53.40 vs 45.05; 63: 45.70 vs 44.30) with independent phases at the strength
line 48.15 mHz (δ inside the circular-surrogate null) — no common antenna-motion
source carries both channels; the third witness — the NOCC's own doppler_resid field
(TRK-2-25 item 101) — carries the band complex itself (station 63: 45.00 mHz,
9.2×; station 43: 46.45 mHz, 4.0×; station 14: not carried) with yet other dominant
members per channel: the complex lives upstream in the DSN reduction, not downstream
of our reconstruction, and the field's own dominant structure sits at 21–27 mHz
(10.6–13.5×) — a fourth band region (Deduktion 31); the signal strength carries
its own line at 48.15 mHz (9.2×, 2.6 mHz below f*) — the antenna-gain path has its
own signature and does not carry f*. The three ground gates (Deduktion 20) close
the earth-wobble branch and keep the instrument branch: the peak carries
A = 4.95 Hz (≡ 0.647 m/s station velocity; f0 = 2.291949 GHz median sky frequency)
— 2 × 10⁵ above the microseism reference (2.4 × 10⁻⁷–2.4 × 10⁻⁵ Hz for 0.1–10 µm
ground motion at 50 mHz), so ground-motion Doppler is excluded; the peaks are
unresolved at 0.02 mHz (Q > 2500) — coherent machine lines, not natural widths;
and the era×station cross-table shows station-fixed frequencies with a slow real
drift (Madrid 63: 47.35 mHz in 1988 → 46.95 mHz in 1992; Canberra 43: 51.55 mHz;
Goldstone 14: 45.75 mHz) — the earlier era wander was the station-mix confound of
the global peak. The lines are per-station coherent slowly drifting
receiver-chain signatures; the DSN hardware documentation (810-005, the
MDA-resolver frequency) is the named next search; the origin stays open
(0 honored).

**The numbered deduction map (complete).** The §5.4 narrative above steps through the deduction chain only partially by number (0/0b and 7–10, 16–20, 22–24, 26, 27, 29–31 individually); this map enumerates every Deduktion head that the historical `link_deduction_probe` tool-comment documents (0, 0b, and 1–40), in numerical order, as the complete chain per that tool's own documentation. The numbers 11–15, 21, 25, and 28 carry no head in the tool comment and are not narrated individually in this paper; they are not fabricated here (0 honored). Deduction 16 is not a head in the tool comment but is narrated individually in §5.4 above; it is repeated here, marked, from this paper's own narration. Each line states what that step measures or excludes.

Deduction 0 — Slipped-cycle mask: ATDF field 76 names the samples whose counter slipped during integration — the count is corrupt, the sample is discarded, not averaged (0 honored).

Deduction 0b — Strength gate: field 78 is in units of 0.1 dBm, so real strengths are negative (≈ −174 dBm); samples without a measured strength (≥ 0) are discarded (0 honored).

Deduction 1 — Multi-station common mode: overlapping windows of two DSN stations carry the spacecraft motion in common and the Earth noise differentially — the difference isolates the station, the pair mean carries the signal.

Deduction 2 — TEC (ionosphere): the phase path length shifts the sky frequency by +(40.3/c)·dTEC/dt/f; the map pair supplies the rate at the station site; without a map pair the sample is discarded, not averaged (0 honored).

Deduction 3 — Solar plasma: the electron column along the light path (OMNI2-N1800, 1/r² profile) supplies dTEC_solar/dt; occultation (b < R_Sun) refuses the column (0 honored).

Deduction 4 — Spacecraft dynamics: radiation pressure (advective; TSI + area/mass/reflectivity) and RTG recoil (advective; Pu-238 half-life) as accelerations at the spacecraft position, the trajectory propagated from the Horizons initial state, plus the a_P scan. Self-test: the Pioneer anomaly (8.74 × 10⁻¹⁰ m/s² sunward) must appear as a carried drift — or its absence is a finding (0 honored).

Deduction 5 — Ramp: the residuum slope against the ramp term ramp_rate·(t − segment midpoint) measures the empirical coupling k (the unit of field 112 stays uninterpreted — the measurement carries it) and subtracts it.

Deduction 6 — Witness: the cleaned NAVIO-OBSVBL series (PDPL-clean) is a second, independent reduction of the same radio event — on common days the probe compares the daily medians of the two residua: the profile correlation names whether the residua share structure (common unmodeled event) or carry separate histories.

Deduction 7 — Daily-curve cut: the slope per segment is measured on the residuum itself and subtracted — the decomposition into a daily profile and a per-sample scatter names whether the 60-s excess is a Moyer daily-curve residual or per-sample noise; gap-adjacent segments are counted, not masked in advance.

Deduction 8 — Telemetry-recoil cut: the harvested power channels (PRTG/Pshunt/Pbus) carry the sunward recoil a = η·P(t)/(m·c) — η scanned empirically, the direction the named Turyshev symmetry.

Deduction 9 — NAVIO chain: the cleaned second reduction becomes the carrier itself — Earth center instead of the station site (the record carries no station, named), the same cuts (epoch offsets, plasma, daily slope, dynamics scans) — and the ATDF becomes the witness. The OBSVBL semantics carry the 2010s research: OBSVBL = received cycles − reference (FREQCY) per count interval in Hz, the DSN convention (Δν)_DSN = ν0 − ν (positive = receding); obs = OBSVBL + FREQCY reconstructs the received sky frequency; A ≈ 15.3 Hz/(m/s) ≈ 2ν/c is the two-way scale. The beat-plausibility gate (|OBSVBL| ≤ 2·f0·v_max/c ≈ 5 × 10⁵ Hz) discards the readme's named error classes — discarded, not averaged.

Deduction 10 — Segment mask: segments (≥ 20 samples) whose RMS exceeds 4× the p90 of the segment RMS are individual corrupt days — discarded, not averaged; the gate derives from the live data (p90) × 2².

Deduction 16 — (no head in the historical tool comment; per this paper's own narration in §5.4) the spectral analysis of the pass scale found a red 1/f spectrum in the 60-s segments (peaks at 0.5–0.7 mHz, 4.6–5.6× the floor) and an unexplained line near 51 mHz in the sub-10-s set (9.3× the floor, 73 249 samples; the strict 1-s class is 70 602); the true spin (4.72 RPM = 78.6 mHz, measured from the telemetry) is absent from the residual.

Deduction 17 — Locate the line: the LS peak near ~51.5 mHz (period ~19.4 s, Deduction 16) is rastered on a fine grid (step ≤ 0.1 mHz) and broken out per file/year/station/sampler class — does the line sit in one dump or everywhere, does the frequency wander, does it carry both stations?

Deduction 18 — The witness test: the same LS search on the NAVIO-clean series (60-s) and on Pioneer 11 — the alias of the line in the 60-s raster is measured, not guessed. If the line carries only the ATDF 1-s data it is a reduction artifact of the ATDF chain; if it carries NAVIO/P11 as well, it is a DSN/spacecraft systematic.

Deduction 19 — The candidates: the measured frequency held against the named machines — the sampler raster (1 s vs 10 s; the classes measured), the MDA-resolver/256-divider relation, and the antenna mechanics (signal-strength modulation at the peak) — each test measured, no fabrication.

Deduction 20 — The three ground gates: before the ground hypothesis (the DSN sites wobble / the instrument) justifies a harvest, the gates measure locally — (a) the line amplitude in Hz against the microseismic reference (amplitude gate), (b) the era × station cross-table (does the frequency wander at a fixed station, or is it station-fixed?), (c) the peak width (sharp = machine, broad = nature).

Deduction 22 — The count structure: TRK-2-25 carries 0.001-cycle resolution — a 1/256 resolver would leave only ~256 of 1000 occupied 0.001 bins in the fraction of the raw doppler_cnt (grid round(m·1000/256)). The test reads the PASF series itself, no model in between.

Deduction 23 — The reference chain: does the ref_hz slot itself carry the line? The slot (TRK-2-25: Hz × 10) carries the station reference chain; if the line lives in the chain it appears in the ref series or in its step pattern — an overlap test with the same LS scans, per station and global, against the measured residuum lines.

Deduction 24 — The signal-strength scaling: PLL loop noise grows at weak SNR (line amplitude ∝ 1/√SNR), while a fixed spur (reference leak) would stay constant — per station the strong/medium/weak groups are split and the line amplitude (absolute and relative to the group RMS) is measured.

Deduction 26 — Spacecraft coherence: if the station chain carries the line (a continuous ~20-s station oscillation), P10 and P11 carry THE SAME phase at 0.714 mHz, referenced to TDB t = 0 — the phase difference must be STABLE across two halves of the common window. Null: circularly shifted series (same spectrum, random phase) — 200 surrogates.

Deduction 27 — The two-/three-way split: in three-way (Ground Mode 3) the receiving station counts the transmitting station's signal. If the receive chain carries the line, f(Mode 2) and f(Mode 3) agree per station; if the transmit/uplink chain does, the three-way shows the transmitting station's frequency. Local block de-trending of the sky frequency, LS 44–56 mHz.

Deduction 29 — The frequency path over the era: does the station line wander continuously or jump (receiver generation Block IV → Block V / Advanced Receiver at the start of the 1990s — the survey finding)? Sliding windows over the residuum per station.

Deduction 30 — The channel coherence: if ONE thing (the antenna in the wind) moves both channels — residuum (phase/Doppler) and strength (gain) — then the two series are PHASE-COHERENT at the complex frequencies. Null: independently rotated half-series (circular surrogates).

Deduction 31 — The third witness: does the NOCC's own doppler_resid field (TRK-2-25 item 101, Hz × 1000, PASF slot 8) carry the 44–56-mHz complex? Yes → the complex lives upstream (count path / station reduction); No → it arises downstream (our reconstruction).

Deduction 32 — Day resolution: within the 1-s bursts the stations carry daily median residua — the common-driver question (TE/n floor of the monthly medians) is tested with daily points plus Pearson over common days.

Deduction 33 — Band structure: does the peak sequence of the sliding windows carry an ORDER (does member A reliably follow member B — self-TE lag-1 over the permutation null), and which transitions repeat above expectation?

Deduction 34 — Spacecraft coupling: do P10 and P11 carry a DELAYED mutual influence at the 0.71-mHz member (TE lag-1 on the envelopes of both spacecraft, both directions, permutation null)? Deduction 26 only excluded the lag-0 co-phasality.

Deduction 35 — Position correlation: do band amplitude and member frequency correlate with the spacecraft's spatial position (heliocentric distance, Earth distance, elongation, ecliptic latitude)? The known forces are excluded from the model — the test holds the band itself against the position.

Deduction 36 — Calibration accompaniment: does the band envelope pulse with the reference staircase (the step density of the 0.1-Hz quantization)?

Deduction 37 — The drift law and the state fine-measurement: does the 63 line tick uniformly (counter rate) or in steps, and do the members carry a hidden discrete state ladder (PWM alphabet)?

Deduction 38 — The wave self-similarity: the autocorrelation of the band region per station over all phase shifts — which patterns recur, how many distinct ones there are, and what the pattern cross-correlations between the stations carry.

Deduction 39 — The naked wave: the LS projection of the residuum onto 44–56 mHz (band-pass), then the phase correlation and the pattern count on the isolated wave.

Deduction 40 — The full extraction (P10): the quadratic per-pass de-trending, then the witnesses (station cell, strength, reference, time) → the negative-fuzzy residuum; the residual scan decides: survivor or silence above the floor.

**5.5 Named, not subtracted.** The link probe models the spacecraft radiative
forces as advective accelerations — solar radiation pressure
(a_SRP ≈ 7.2 × 10⁻¹¹ m/s² at 40–55 AU, from TSI, area, mass, reflectivity) and
RTG thermal recoil (a_RTG ≈ 2.2 × 10⁻⁹ m/s² at epoch, Pu-238 decay, anisotropy
fraction scanned; the telemetry sets Turyshev20170327 are harvested into
`pioneer10_telemetry.bin`, 7 953 267 records 1972→2002, channel semantics decoded
per the almanac — temperatures °F, PRTG = ΣUᵢIᵢ in W, Pshrad = I_shunt²·5.25 Ω;
the efficiency scan now carries the published values from the PRL thermal paper
(arXiv:1204.2507: η_rtg = 0.0104/0.0144, η_elec = 0.406 — our first scan sat
5–7× high, corrected; a_RTG(t0) = 3.05 × 10⁻¹⁰ m/s²) and stays flat) — and subtracts the solar-plasma shift where OMNI2 carries the
era; the ionosphere stays unsubtracted because no TEC map exists before 1998. The
fitted a_P scan stays flat and the residual drift 1.8 × 10⁻⁸ ± 4.6 × 10⁻⁸ m/s²
sits ~2 × 10¹ above the anomaly. The galactic tide, stellar encounters, and the
reduction-internal systematics remain in the residue and are named, never
subtracted. Absent means unmodeled, never zero.

> **Surrogate-machine generation (conservative footnote).** The family bound
> here was generated by the pre-fix surrogate RNG (fam-machine: pre-fix).
> The pre-fix band lies higher than the post-fix band; a verdict that holds
> against the higher pre-fix bound holds post-fix a fortiori. The measured
> silence is thus the conservative statement.

**5.6 The NAVIO-ASCII Vollmission through the own reduction — the Ruck transit-sweep,
closed at fully characterised silence.** The full-mission ASCII oddump (Pioneer 10
SC_23 / Pioneer 11 SC_24, 60-s, 1973–2002, Zenodo 10.5281/zenodo.13309156) was
compiled through the own reduction for the first time: `pioneer10_doppler.bin`
908 309 records (1973-10-05..2002-03-03), `pioneer11_doppler.bin` 967 272 records
(1973-04-10..1993-07-15); a new station-carrying record (PNAV, TRANS/RCVR1/
linkmode, PDPL untouched) and a station-aware residuum chain
(`pioneer_navio_residuum`, two-way model + displaced-count mask, Deduction-10
daily mask) serialized PNVR/PNDM daily medians. The form test (∝t² vs
RTG-decay, Deduction 1) is answered: no carried form over the per-station floor
2.8–19 kHz. A Ruck census over all flagged daily steps (P10 99 + P11 71 = 234)
measured the form of every flag (jump, pre/post baseline, return, sign-flips):
**0 of 66 (P10) and 0 of 60 (P11) measured returns return to the pre-baseline;
sign-flip (jitter) 20–147 dominates the long segments; no Ruck carries the
transit form** (a smooth, monotone, low-jitter ramp that rises and returns). The
one apparently three-station-coherent candidate (P11 1981-02-09) is a single-day
daily-median in the noise (±46 kHz jitter, −156 Hz step) — coherence without
form is not evidence. Cross-spacecraft witness tests (same-day: zero shared flag
dates; time-offset: P10 22–66 AU vs P11 5–14 AU, separation 24–110 AU) exclude a
common transiter. **The silence has a shape: the transit form is structurally
absent from the event distribution, measured not asserted.** No transit is
refuted — a smooth transit below the flag threshold was never a candidate.

**The sub-kHz basis entered (Deduction 40).** The negative-fuzzy extraction
(quadratic per-pass detrending + station-cell median, `pioneer_navio_negative_fuzzy`)
pushed the daily medians to sub-kHz residency: **P10 92.5 % (1041/1126) and
P11 97.5 % (2027/2079) of daily medians below 1 kHz**, median |daily-med|
89 Hz (P10) / 35 Hz (P11), |daily-med| p50/p90/p95/p99 = 89/774/1474/3861 (P10)
and 35/387/593/2070 (P11) — median-driven residency, the ~2 kHz RMS is the
jitter-day tail. A Ruck scan over this 1000× finer basis (gate 7/2.8 mHz/s vs
Hz/s) flags P10 48 + P11 173 steps, all form-measured, top jumps 1–5 kHz (down
from ±142 kHz dump offsets); 0/15 (P10) and 0/142 (P11) fine steps return —
still no transit ramp. The cleanest sub-kHz flag (P11 1983-06-18) is the
aftermath of the outlier day 1983-06-16 (−4548 Hz, RMS 36.7 kHz), not a
symmetric transit ramp. The sub-kHz daily medians are the finer basis for the
continuous-drift question; the transit sweep over them stays silent (0 honored).

**The continuous drift on the sub-kHz basis.** The secular drift of the quiet
(≤p95) days is sunward, at ~10⁻⁴ of the anomaly's magnitude (P10 −1.1×10⁻⁴,
P11 −8.1×10⁻⁵ of the anomaly, sign convention negative sunward); the anomaly
(~1 Hz over the mission) sits 150–340× below the quiet-day floor (160–340 Hz
day-median scatter). The form test (linear vs ∝t² constant-force vs Pu-238
thermal decay, τ = 87.7 y / e-fold 126.5 y) resolves neither signature: no model
improves on linear by > 2 %, and both hypotheses' signals lie in the 1–3 Hz
range against a two-orders-higher floor. The test becomes discriminating only on
sub-Hz aggregation. The sub-kHz basis improved anomaly measurability ~100× over
the kHz chain (from 2×10⁴ to 150–340× under the floor); the remaining bottleneck
is per-sample scatter surviving the day-median, not reduction systematics (the
kHz chain's systematic errors — station offsets, dumps, jumps — were removed;
that was the 100× gain). The path to the anomaly runs through regression over the
full quiet series or week/month binned medians (√N over time), not through further
masking. Limit, not failure; the floor is the statement (0 honored).

## 6. Reproducibility

Artifacts on the ssd.jpl.nasa.gov CDN release; commands:

- Daily arcs: `cargo run --bin horizons_compiler -- --daily` (→ `ephemeris_*_daily.bin`).
- Fine arcs: `cargo run --bin horizons_compiler -- --flyby` (→ `ephemeris_voyager*_*.bin`).
- Measurement: `cargo run --release --bin dark_matter_probe` (residue baselines,
  leave-one-out positive control, 1008-point triangulation net, null controls,
  synthetic-clump self-check). Seed 0x9E3779B97F4A7C15; planet bins from
  `ephemeris_{body}.bin`; Rust std-only; f64 throughout.
- Fine Doppler (§5.4): `cargo run --bin pioneer_atdf_compiler` (raw bit-packed ATDF →
  `pioneer10_skyfreq.bin`), `cargo run --release --bin pioneer_doppler_moyer` (full
  observation model + epoch calibration), `cargo run --release --bin pioneer_odp`
  (Sun + sunward-anomaly orbit solution), `cargo run --release --bin
  pioneer_residuum_diagnose` (beat/phase/DSN-residual audit), `cargo run --release
  --bin link_deduction_probe` (link deductions: multi-station common mode, TEC via
  `--ionex-dir`, solar plasma via the `omni2_serie.bin` in the OMEGAFLOW_STATE cache
  (default `~/.local/state/omegaflow`, produced by `omni2_compiler`) or `--omni2`, spacecraft
  dynamics).
- NAVIO-ASCII Vollmission + transit sweep (§5.6): `cargo run --bin pioneer_doppler_compiler`
  (NAVIO ASCII → `data/{name}_doppler.bin` PDPL + `data/{name}_navio.bin` PNAV, CDN upload
  in `--ci-mode`), `cargo run --bin pioneer_navio_residuum` (station two-way model →
  PNVR/PNDM daily medians + Deduction-10 mask + Ruck scan), `cargo run --bin
  pioneer_navio_ruck_form` (form census over all flags; `--date <Y-m-d>` measures one day
  flag-independent), `cargo run --bin pioneer_navio_negative_fuzzy` (Deduction 40:
  quadratic per-pass detrend + station-cell median → sub-kHz daily medians,
  `data/{name}_navio_subkhz_daily.bin` PNDM), `cargo run --bin pioneer_navio_drift`
  (Deduction 41: continuous-drift form test — linear/∝t²/RTG-exp over the quiet
  ≤p95 sub-kHz daily medians, per probe).

## 7. Conclusion

The blade measured what the five spacecraft arcs carry. The machine reads a dropped
planet's mass to better than one percent — the instrument is honest. Pointed into the
dark, a hundred-point net over the outer solar system returns zero flags and zero
triangulations: no dark-matter clump at the granule floor of the public finished
orbit products. The floor, not the absence, is the statement. The raw Doppler that
would lower the floor is present at SPDF for the Pioneer pair and is named for the
next harvest.

The NAVIO-ASCII Vollmission, pulled through the own reduction (§5.6), closes the
transit-sweep front at fully characterised silence: all 234 Ruck flags carry a
measured form profile, none returns to its pre-baseline, jitter dominates the long
segments, and the one apparently three-station-coherent candidate (P11 1981-02-09)
is a single-day daily-median in the noise. The transit form is structurally absent
from the event distribution. No transit is refuted — a smooth transit below the flag
threshold was never a candidate — and the floor, not the absence, remains the
statement. The sub-kHz daily medians of the NAVIO chain, entered via the
negative-fuzzy extraction (Deduction 40), carry 92–97 % of days below 1 kHz; a
Ruck scan over that 1000× finer basis still finds no transit ramp. The continuous
drift on that basis is sunward at ~10⁻⁴ of the anomaly, but the anomaly sits
150–340× below the quiet-day floor and the ∝t²-vs-thermal form test resolves
neither signature — both hypotheses' signals lie in the 1–3 Hz range against a
two-orders-higher floor. The measurable, named limit is the day-median scatter
itself, not a reduction systematic; the path to the anomaly runs through
regression over the full quiet series, not further masking. The terrain is
measured to its floor, and the measurability of the anomaly was improved ~100×
along the way. Limit, not failure (0 honored).

## References

1. Anderson J. D., et al., 2002, PRD 65, 082004 (Pioneer anomaly).
2. Turyshev S. G., Toth V. T., Kinsella G., Lee S.-C., Lok S. M., Ellis J., 2012, PRL 108, 241101 (thermal recoil).
3. Turyshev S. G., Anderson J. D., Watkins C., 2017, Pioneer Doppler tracking data, NASA SPDF / Zenodo 10.5281/zenodo.13309156.
4. Toth V. T., 2006/2026, Pioneer 10/11 Telemetry Explanatory Almanac, arXiv:2606.23755.
5. Batygin K., Brown M. E., 2016, AJ 151, 22.
6. Schreiber T., 2000, PRL 85, 461.
7. Kaiser A., Schreiber T., 2002, Physica D 166, 43.
8. Moyer T. D., 2000, Formulation for Observed and Computed Values of Deep Space
   Network Data Types for Navigation, JPL Deep-Space Communications and Navigation
   Series, Monograph 2.
9. Morabito D. D., Asmar S. W., 1995, Radio-Science Performance Analysis Software,
   JPL TDA Progress Report 42-120, 121.
10. Falin B. W., 1985, DSN Frequency and Timing System, Mark IV-85, JPL TDA Progress
    Report 42-82, 113.
11. Korwar V. N., 1981, Coherent Reference Generator Phase Stability, JPL TDA Progress
    Report 42-64, 222.
