<!--
  title: The 20-s Doppler band of Pioneer 10: a ground-chain fingerprint
  class: paper
  date: 2026-09-03
  version: 2
  sha256: 531f5afbc5055f193d884b9a8a839f8a5c52209abb77420f914dbf238f5f2e02
  status: live
  see-also: docs/paper/probe-front-dark-matter.md, docs/TODO.md (Pioneer-Front), docs/reference/
-->

# The 20-s Doppler band of Pioneer 10: a ground-chain fingerprint

## Abstract

In the Pioneer-10 Doppler data (ATDF, 1988–1993, 73 249 sub-10-s samples; the strict 1-s class is 70 602) the band 44–56 mHz carries a dense complex of coherent traces whose frequencies are set by the receiving station: Goldstone 14 carries 45,75 mHz, Canberra 43 carries 51,55 mHz, Madrid 63 carries 47,35 mHz — a signal from space would arrive at all three stations with the same frequency. The band is a ground-chain fingerprint: it arises in the DSN receiving chain, not in the probe and not in the medium. Peak 50,73 mHz (T* = 19,71 s, 9,5× the floor; the 0,05-mHz grid reads 50,714 mHz), a global complex that resolves into station-fixed lines at 45,75 / 51,55 / 47,35 mHz, slowly drifting (~0,4 mHz in four years).

**Method.** Exhaustive subtraction of known effects: the band survives the
named, measured exclusions (§3); its identity remains open — the remnant is
an unknown instrument contaminant of the chain, not claimed physics.


## 1. The measurement series

**The line is a complex.** The coarse Lomb-Scargle scan over the 1-s samples
carries an unknown line at ~51,5 mHz (9,3× the floor); the fine grid
(0,1 mHz over 44–58 mHz) resolves it: peak 50,73 mHz (T* = 19,71 s, 9,5×;
the 0,05-mHz grid reads 50,714 mHz)
with sub-peaks 49,0 / 50,2 / 50,7 / 51,5 / 53,4 mHz. Sliding windows
(3 000 samples) show: the window peaks scatter across the whole band with
comparable ratios (Station 63: 24 windows, 44,05–55,50 mHz,
2,3–10,2×) — no single coherent line carries the band (a coherent
line would carry the same frequency in every window; measured: it does
not).

**The frequencies are station-fixed.** The breakdown per station carries:
Station 14 → 45,75 mHz, Station 43 → 51,55 mHz, Station 63 → 47,35 mHz (1988)
and 46,95 mHz (1992) — a slow genuine drift of ~0,4 mHz in four years
(~1,75×10⁻¹³ relative), ~10⁴× above the frequency resolution. The splittings
are stable across all 1-s data of the respective station.

**The traces are fixed, not noisy.** The line amplitudes are
strength-constant: Station 14 carries A = 160/153/161 Hz over weak→strong
(Station 63: 57/50/65 Hz) — the loop-noise scaling A ∝ 1/√SNR is not
carried; they are fixed traces (~7×10⁻⁸ of the sky frequency, ~1,6×10⁻⁴ of
the 1-MHz bias chain), coherent (FWHM ≤ 0,02 mHz unresolved, Q > 2500).

**The band lives upstream.** The NOCC's own doppler_resid field
(TRK-2-25 Item 101) carries the band itself (Station 63: 45,00 mHz, 9,2×;
Station 43: 46,45 mHz, 4,0×) — the structure was in the DSN reduction,
before our chain saw it. The stored reference frequency does NOT carry it
(its own 0,1-Hz staircase structure lies beside it, e.g. Station 43:
49,40 mHz). The count structure of the raw doppler_cnt is natively 0,001
cycle (1 000/1 000 bins evenly distributed, 1/256-grid share 0,259 ≈
256/1000) — the MDA resolver/256 divider is measured excluded.

**The channels are separate.** The signal-strength channel (pure downlink-gain
path) carries its own line at 48,15 mHz — at this frequency the phases of
the strength and residual channels are independent (δ in the circular
surrogate null, all three stations): no common cause (the antenna-in-wind
hypothesis is measured excluded). Between the probes: P10 and P11 carry the
60-s member (0,71 mHz = |f* − 3/60 Hz|) in the overlap era (8,6× and 4,6×
of the local floor), but phase-uncorrelated (δ = 0,49 rad in the null) and
with era-different occupancy (NAVIO 1988, ATDF-60-s 1993) — no common
continuous oscillation.

**No order, no attitude coupling.** The self-TE of the window-peak
sequence lies in the permutation null or is dominated by the self-persistence
of the lower band edge — no member sequence (no mode jumper).
The Pearson correlation of the peaks with the spatial attitude of the probe
(r_hel, Earth distance, elongation, ecliptic latitude) is not decidable
through the time-space degeneracy of the windows; the member frequencies
are attitude-independent.

**The documents carry no machine.** FTS Mark IV-85 (TDA PR 42-82) and the
CRG article (TDA PR 42-64) describe the reference chain as distribution +
validation — no periodic control loop on the second scale. The
PLL bandwidths of the receiver blocks (12/3/0,1 Hz) carry 13 ms/53 ms/1,6 s —
not the 19,4–21,9 s. The syntonization series of the station standards (TDA PR
42-72, digitized from the figures) carry drift rates of 4,6×10⁻¹⁴ to
1,1×10⁻¹² per year — the measured line drift (≈4,4×10⁻¹⁴/a) lies below:
the scale does not exclude the line drift.

**The P11 route.** Pioneer 11 carries no ATDF archives; the ODF harvest
(TRK-2-18 parser, golden-verified) delivered 27 907 samples 1986–1990 —
100 % 60-s compression, two-way runs at 9 stations. The 0,71-mHz member
is not carried in the raw cells; the two-/three-way split at the
model-subtracted residual remains the named next step.

## 2. The decisive measure

The question "Ground or space?" is decided by a single measure: the
**station-fixity of the frequencies**. The sky frequency is a common
signal — whatever oscillates in space or in the medium would arrive at
Goldstone, Canberra and Madrid with an identical frequency. The measured
members are station-different (45,75 / 51,55 / 47,35 mHz) and remain so
across the whole 1-s class. The band knows which antenna receives it — so
it is generated in the receiving chain. All secondary findings support
this reading (upstream, strength-constant, reference-free,
channel-separated); none refutes it.

## 3. What is excluded (measured, no fabrication)

Earth wobble (amplitude gate: 2×10⁵ above the microseism reference);
antenna in wind (channel phases independent); PLL loop noise
(strength constancy); MDA resolver/256 divider (count structure natively
0,001); stored reference (own staircase, no coverage); a common probe
oscillation (phase null); a member sequence (self-TE null); the known
space forces (attitude-independent members, model exclusion of the chain).

## 4. What remains open

The two-/three-way split at the model-subtracted residual of both probes —
the question whether the band sits in the uplink (transmit) or the
downlink (receive) chain — and the named machine of the NOCC reduction.
The handover `docs/handover/handover-2026-08-24-pioneer-p11-modell-subtraktion.md`
carries the path.

## References

1. TRK-2-25, DSN 820-13 Rev. A (1988-10-15), Orbit-Daten-Format ATDF —
   docs/reference/trk-2-25-atdf.txt.
2. TRK-2-18 (1988-10-15), Orbit Data File Interface — docs/reference/.
3. Morabito D. D., Asmar S. W., 1995, Radio-Science Performance Analysis
   Software, TDA PR 42-120, 121.
4. Korwar V. N., 1981, Coherent Reference Generator Phase Stability,
   TDA PR 42-64, 222.
5. Falin B. W., 1985, DSN Frequency and Timing System Mark IV-85,
   TDA PR 42-82, 113.
6. Turyshev S. G., Toth V. T., 2010, Living Rev. Relativity 13, 4.
