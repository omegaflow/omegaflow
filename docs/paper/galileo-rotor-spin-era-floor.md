<!--
  title: Galileo rotor-spin Doppler line and the era-confounded residue floor
  class: paper
  date: 2026-09-05
  sha256: 345ac23dc788be3b92f9cf8bfa7be95cfa6fa2fa5adaae9fd4bf83a16051593b
  status: live
  see-also: docs/befund/befund-galileo-rotor-spin-epoch-anchor.md docs/befund/befund-galileo-alpha-zeit-sonnenzyklus.md docs/befund/befund-galileo-banden-kamm-ton.md docs/befund/befund-galileo-te-spec.md
-->
# Galileo rotor-spin Doppler line and the era-confounded residue floor

## Abstract

We measure the coherent 52.39 mHz line in the Galileo S-band Doppler residue of
1990 December 7-10 and anchor its epoch to the spacecraft. A Rust DAF decode of
the NAIF all-spin-bus rotor CKs (frame -77000) yields 52.39006 mHz (19.0876 s,
0.3291764 rad/s, 3.143403 rpm) over 90.24 h of dense interval time from 494
segments, 1.000001 of the tone: the tone is the Galileo rotor spin,
epoch-anchored. The residue noise floor is not a plasma-elongation effect: the
coherent channel is quiet at conjunction and loud at opposition, inverse to
scintillation, and under era control (1996, 5-6 AU) the 70x/17x pooled contrast
collapses to 1.9x/2.4x - era/solar-cycle/distance-confounded. Conditional
transfer entropy from the recorded reference, mode, and cadence fields is null
(37 forward tables; cadence degenerate, 99.87 % of samples at 1 s), so the floor
is not spec-field-driven. One identified, epoch-anchored coherent line stands
against a decoupled, era-confounded floor.

## 1. The measurement

The Galileo residue corpus (`galileo_resid.bin`, 14 077 825 records, 1990-11-29
to 1997-02-28) carries one isolated, coherent, narrow line at 52.39 mHz
(parabola-interpolated 52.385-52.390 mHz, period 19.1 s), present only at
station 42 in its single two-way window: 1990-12-07 to 1990-12-10, n_locked
98 149 one-second samples. The line explains 98.9-100 % of the segment variance
in the long one-second segments of 1990-12-09/10 (segment n 2 150-11 421; RMS of
the detrended residues 5.3-6.7 Hz). It carries no harmonic at 104.77 mHz (0.0 %),
no subharmonic at 26.2 mHz (0.0 %), and no sideband family in the 45-58 mHz
window. This paper identifies that line: its epoch is anchored to the
spacecraft's dual-spin rotor, and the noise floor around it is characterized as
era-confounded and field-decoupled.

Two independent characterizations of the same window bracket the measurement.
The exact 50 mHz comb (50/100/150/200 mHz) that appears in the thin December
1990 two-way passes of the 70-m stations is the sampling-grid degeneracy of the
60-s cadence (normalized variance ≤ 8 %, reproduced by white noise on the same
grid), not a line family; it is not part of this finding. The station-42 tone is
the one member of the window that is a genuine coherent line.

## 2. Method

**Rotor-CK epoch anchor.** The daily Galileo all-spin-bus CK products
(`rtr` family, frame -77000) were read with the boxed DAF reader (a Rust DAF CK
decode; the daily products are BIG-IEEE DAF, and the reader carries a small
big-endian mirror alongside its LTL-only path). SCLK to ET conversion runs over
`mk00062a.tsc` (partition 77, piecewise tick interpolation); coverage is read
from the segment summaries, not from the day-of-year in the file name. Eight
daily products from the NAIF prime-mission `rtr` archive (`ck90341a_rtr.bc` to
`ck90344b_rtr.bc`) carry 494 segments, all with summary IC(1) = -77000 (ROTOR),
IC(2) = 1 (type 1), record length 7 (unit quaternion plus angular rate):
489 953 pointing records, 0 defective norms, median_dt 0.667 s. The spin
increment per interval is theta = 2 acos(|dot(q_i, q_{i+1})|); only dense,
alias-free intervals are summed (dt at most min(3 median_dt, 9 s) and theta less
than pi - 0.05). The rate is the summed angle over the summed interval time of
the window.

**Residue noise floor.** The floor metric is the median of the daily residue RMS
(Hz) per (mode, day), residues pooled over stations with lock transitions
(|resid| > 1000 Hz) excluded as their own class. Geometry (solar elongation at
the Earth, heliocentric distance) comes per day from the ephemeris bins.
Era control fixes the calendar era as the solar-cycle proxy and reads the
geometry cells that a single era actually visits.

**Field decoupling.** Directed transfer entropy runs the omegaflow estimator
(`transfer_entropy_lag`, Silverman KDE) with phase-randomized and block-bootstrap
surrogates and an era-conditional null (`conditional_te_stats`), lags 1/2/3/5,
both directions, over (station, epoch) runs of at least 14 consecutive days with
at least 30 cleaned samples per day. A same-day, within-day paired control tests
the level association of the spec state with the noise height of the same day.

## 3. Results

**The line is the rotor.** Over the window 1990-12-07 ~00:55 to 1990-12-11 ~00:06
the accepted dense interval time is 324 862.447 s (90.24 h of the ~96 h), the
summed rotation angle 106 937.061 rad = 17 019.6 revolutions, and the measured
rotor spin is:

- rotation rate 0.3291764 rad/s
- period 19.087592 s
- 3.143403 rpm = 52.390056 mHz

Per daily half-product (a ~00:..12:, b ~12:..24:): December 7 a 52.390739 mHz
and b 52.387595 mHz; December 8 a 52.385546 mHz and b 52.383848 mHz; December 9
a 52.391963 mHz and b 52.389730 mHz; December 10 a 52.252720 mHz and b
52.544532 mHz. The December 7-9 products lie within 52.384-52.392 mHz; December
10 carries a ~0.29 mHz half-day oscillation across its two half-products
(observed, mechanism and duration not determined; the window mean stays 52.390
mHz). Against the tone: 52.390056/52.39 = 1.000001 (delta +0.00006 mHz) - the
station-42 tone and the reconstructed rotor spin are the same frequency to one
part in 10^6. The 19.10 s label was the loose rounding of the same number
(52.39 mHz implies 19.0876 s; measured 19.087592 s, 0.0124 s below the label).
The design spin rate of 3.15 rpm (Osborn 1983) lies 0.21 % above the measured
rate of these days; the dual-spin nominal 0.3300 rad/s +/-0.0015 encloses the
measured 0.3291764 rad/s. The despun scan-platform CK (frame -77001,
`gll_plt_rec_1990_tav_v00.bc`) carries no 19.1 s contribution in the same window
- the platform is inertially fixed by construction - so the two CK frames
together are the complete attitude statement: the low-gain antennas on the
spinning rotor see the 52.39 mHz rotation during the Earth-encounter window, the
despun platform does not.

**The noise floor is era-confounded, not plasma-elongation-driven.** The coherent
modes are quiet at conjunction and loud at opposition:

| Mode | Opposition (150-180 deg) | Conjunction (0-30 deg) | contrast |
|---|---|---|---|
| 1 (one-way) | 8.2 Hz (10 days) | 7.5 Hz (133 days) | flat, ~1.1x |
| 2 (two-way) | 42.0 Hz (16 days) | 1.5 Hz (77 days) | 28x |
| 3 (three-way) | 79.5 Hz (15 days) | 4.9 Hz (60 days) | 16x |

Plasma scintillation is loud at small elongation (conjunction) and quiet at
large elongation (opposition). The measured coherent channel is the inverse:
quiet at conjunction, loud at opposition. The plasma-elongation reading is
killed by the direction itself. The magnitude of the contrast does not survive
era control either. The opposition geometry visits two separated eras - 11 days
in 1990-11-29..12-09 (Earth cruise, ~1 AU, solar maximum) and 4-5 days in
1996-06-26..06-30 (5.19 AU, solar minimum). Pooled over eras the contrast reads
70x (mode 2) and 17x (mode 3); within the single era that visits both regimes at
the same distance (1996, 5-6 AU) it collapses to 1.9x (mode 2: 2.8 Hz opposition
against 1.5 Hz conjunction) and 2.4x (mode 3: 16.1 Hz against 6.7 Hz). The loud
opposition value is a 1990-cruise/solar-maximum number (mode 2 46.9 Hz, mode 3
93.8 Hz in 1990; 2.8 Hz and 16.1 Hz for the same geometry in 1996) and the quiet
conjunction is observable only in the late era (no conjunction days exist before
1994). The residual contrast of at most ~2.4x rests on a single 4-5-day window.
Mode 1 stays flat under the same control (1996: 10.9 Hz opposition, 4 days,
against 8.9 Hz conjunction, 30 days). The residue floor is therefore a
distance/solar-cycle/era confound, not a surviving plasma-elongation law and not
a surviving geometry law of the measured magnitude.

**The floor is decoupled from the recorded reference/mode/cadence fields.** Over
37 forward tables (26 on the reference axis, 8 on the mode axis, 3 on the
cadence axis; n = 14-33 days per run) no testable forward cell of the
reference-to-noise or mode-to-noise transfer entropy exceeds the
phase-randomized null, and the few block-null crossings collapse under
era-conditioning (the one isolated cell that survives the conditional null does
so by 0.007 nats at n = 21 and does not replicate under any other seed or
window). The reverse direction (noise to reference) crosses more often than the
forward direction - a physically impossible arrow that marks the wide surrogate
null at these n - and carries no forward evidence. The cadence axis is
degenerate in the realized field: 99.87 % of the cleaned samples sit at 1 s
(12 060 056 of 12 076 707), mode 2 is 99.59 % one-second, and the 60-s
configurations that the comb analysis presupposes are not the measured cadence
of this corpus. The same-day level association is measured independently with a
within-day paired control: pooled over 133 days the mode-1-vs-mode-2 noise
difference of the same day is -0.07 decade (median |resid|) and +0.03 decade
(RMS), p = 0.30/0.49 - the day-level mode difference flips sign across
(station, month) blocks and does not survive the same-day pairing. The recorded
reference, mode, and cadence fields carry no directed path into the residue
noise height.

## 4. Discussion and bounds

The Galileo residue corpus reduces to one identified, epoch-anchored coherent
line and a floor. The line is a spacecraft property: the rotor-spin tone at
52.39006 mHz is present only in the station-42 two-way window of December 1990
and matches the reconstructed rotor rate to one part in 10^6. The floor is a
measurement-series property of the era: its height tracks the solar-cycle/era
axis (loud 1990 cruise at ~1 AU and solar maximum, quiet 1995-1997 at 5-6 AU and
solar minimum) and its day-level structure tracks the day, not the recorded spec
state of the same day.

The bounds of the measurement are the bounds of the window and the corpus.
(1) The epoch anchor covers four days of December 1990; other epochs of the
Galileo mission are not addressed here. The reconstructed NAIF products are the
reference attitude products of a reconstruction - the spin rate is the rate of
that reconstruction, not an independent onboard measurement; the December 10
half-day oscillation (52.253/52.545 mHz) is observed with its mechanism open.
(2) The era control rests on the one year (1996) in which one mode visits both
geometry regimes at one distance; its opposition cell is a single 4-5-day
window, and its opposition days are bimodal (mode 2: 2.8/28.3/59.6/0.01/0.04 Hz
over five days). The middle elongation band (30-150 deg) is unoccupied for the
coherent modes - the solar axis is not independently separable from the
distance/era axis on this corpus. (3) The mode-2 conjunction floor is fragile:
the 1.5 Hz pooled value depends on two all-lock days and station pooling (0.65 Hz
over 75 non-lock days; per-station-day conjunction medians 0.28-0.58 Hz). (4) The
field-decoupling test has power only for persistent couplings at the day lag;
a pure simultaneous level link is covered by the within-day control, and the
three-way (mode 3) windows are too sparse (0-2 days per month) for a directed
verdict. (5) The received-strength field is uncalibrated AGC; only its order is
used, and the mode-1 floor does carry a strength-dependent term at the AGC clamp
(10-20x at the clamp against the strong plateau per station) that is itself
era/station-collocated.

What stands after the bounds: one coherent, spacecraft-anchored line at 52.39
mHz, and a noise floor that is not explained by solar plasma along the line of
sight (the direction is inverse), not explained by the recorded reference, mode,
or cadence fields (directed transfer entropy null; same-day association null),
and not a surviving geometry law of the measured magnitude (era control collapses
the 70x/17x contrast to 1.9x/2.4x). The floor is era/solar-cycle/distance-
confounded and decoupled from the recorded fields.

## 5. References

1. Anderson J. D., Armstrong J. W., Campbell J. K., Estabrook F. B., Krisher T. P., Lau E. L., 1992, Gravitation and celestial mechanics investigations with Galileo, Space Science Reviews 60, 565 - DOI 10.1007/BF00216869.
2. Woo R., Armstrong J. W., 1979, Spacecraft radio scattering observations of the power spectrum of electron density fluctuations in the solar wind, Journal of Geophysical Research 84, 7288 - DOI 10.1029/JA084iA12p07288.
3. Asmar S. W., Armstrong J. W., Iess L., Tortora P., 2005, Spacecraft Doppler tracking: noise budget and accuracy achievable in precision radio science observations, Radio Science 40, RS2001 - DOI 10.1029/2004RS003101.
4. Armstrong J. W., 2006, Low-frequency gravitational wave searches using spacecraft Doppler tracking, Living Reviews in Relativity 9, 1 - DOI 10.12942/lrr-2006-1.
5. Osborn F. W., 1983, Design of the Galileo remote science pointing actuators, 17th Aerospace Mechanisms Symposium, NASA NTRS 19830016631 - https://ntrs.nasa.gov/citations/19830016631.
6. NASA NAIF, Galileo prime-mission rotor CK kernels, frame -77000 (`ck90341a_rtr.bc` ... `ck90344b_rtr.bc`), December 1990 - https://naif.jpl.nasa.gov/pub/naif/GLL/kernels/ck/.
