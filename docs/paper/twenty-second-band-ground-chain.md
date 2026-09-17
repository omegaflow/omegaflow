<!--
  title: The 20-s Doppler band of Pioneer 10: a ground-chain fingerprint
  class: paper
  date: 2026-09-17
  version: 11
  sha256: 9621399c78a17c993a9c8aba56b18722aa93862814445a0309af4f9947413b18
  status: live
  see-also: docs/paper/probe-front-dark-matter.md, docs/handover/archiv/handover-2026-09-09-mechanische-reste.md (Pioneer-Front), docs/reference/
-->

# The 20-s Doppler band of Pioneer 10: a ground-chain fingerprint

## Abstract

In Pioneer-10 Doppler data (ATDF, 1987-12→1993-04, 73 249 sub-10-s samples; strict 1-s: 70 602), the ~20-s band (44–58-mHz grid) carries a dense complex of coherent traces with station-fixed **dominant** frequencies: on the canonical residual (§1), 1988 peaks: Goldstone 14 → 57,11 mHz, Canberra 43 → 44,40 mHz, Madrid 63 → 51,99 mHz; a common signal arrives with the same frequency. The band is a ground-chain fingerprint: it arises in the DSN receiving chain, not the probe nor the medium.

The complex is dense, not fixed lines: 3 000-sample windows scatter across the band (Station 63: 24 windows, 44,05–55,50 mHz, 2,3–10,2×); no single coherent line carries it. The earlier per-station values 45,75, 51,55, 47,35 mHz are sub-peaks: ranks 4 (45,75), 2 (51,50, one 0,05-mHz step below 51,55), 17 (47,35, outside top-5); the global peak 50,73 mHz does not reproduce (§1, §4).

**Method.** Exhaustive subtraction: the band survives the measured exclusions (§3); its identity remains open: an unknown instrument contaminant, not claimed physics. The chain is reconstructed in full: plasma deduction from OMNI2 N1800, common-mode empty (minimum inter-station gap 110 s), TEC deduction a dated boundary (GIM maps begin 1998); the per-station claim is the measured census, not a fixed-line pick.


## 1. The measurement series

**The band is a dense complex.** The coarse Lomb-Scargle scan over the 1-s samples
carries an unknown line at ~51,5 mHz (9,3× the floor); the fine grid
(0,1 mHz over 44–58 mHz) resolves it with sub-peaks 49,0 / 50,2 / 50,7 / 51,5 /
53,4 mHz. Sliding windows
(3 000 samples) show: the window peaks scatter across the whole band with
comparable ratios (Station 63: 24 windows, 44,05–55,50 mHz,
2,3–10,2×) — no single coherent line carries the band (a coherent
line would carry the same frequency in every window; measured: it does
not). The pooled full-era census (all years, canonical series) peaks at
49,16 mHz (strict 1-s class) / 44,65 mHz (sub-10-s class); the earlier version's
global peak 50,73 mHz does not reproduce as a dominant.

**The dominant frequencies are station-fixed, and cut-conditional.** On the
canonical series the 1988
dominant peaks are station-different: Station 14 → 57,11 mHz, Station 43 →
44,40 mHz, Station 63 → 51,99 mHz (sub-10-s class, 44–58-mHz grid, 0,05 mHz; the
strict 1-s class carries the same dominants). The stage that sets the 1988
dominants is the daily-curve segment-slope cut (Deduction 7): on the rx14-1988
strict-1.0-s cell the peak is 45,75 mHz before the cut (resid_d) and 57,11 mHz
after (resid_e), +11,36 mHz, carried by the one 9,84-h (9 435-sample)
mixed-station segment that holds the cell — the 19,83-h segment alone leaves it
at 45,75, and min-segment thresholds 20–1 000 all give 57,11. The 57,11 mHz is
not manufactured by the cut: it is a real member of the dense complex already
before it (rank 6 in resid0/resid_c, rank 9 in resid_d) and the cut re-ranks it
to the dominant. Station 43 is cut-invariant (44,40 mHz pre and post), Station
63 moves 53,50 → 51,99 mHz. The per-station dominants are therefore
cut-conditional (a per-station slope instead of the documented per-segment one
gives rx14 47,85 / rx43 51,42 mHz), while the station-fixity of the band is
carried by the uncut, the per-segment and the per-station slope alike. The
per-station values of the
earlier version appear, on the canonical series, as sub-peaks of this complex:
45,75 mHz at rank 4
(Station 14, 4,2×) — the pre-cut dominant, see above —, 51,50 mHz at rank 2
(Station 43, one 0,05-mHz grid step
below the earlier 51,55), 47,35 mHz at rank 17 (Station 63, outside the top-5);
47,35 mHz is dominant in no cell of the era × station × class × band census. A
surrogate sub-peak null (199 phase-randomized surrogates per cell, full-circle
phase rotation) puts the hit rate of a random frequency against the real top-5
within the 44–56-mHz null band at 0,0208 per claim; two of the four earlier claims (45,75 mHz and the 1992
46,95 mHz) survive above the null (mean + 2σ = 0,65), the other two (51,55 /
47,35 mHz) do not. The earlier "slow drift" sentence (~0,4 mHz in four years)
rested on the 47,35-mHz (1988) → 46,95-mHz (1992) pair; its 1988 anchor does not
appear, so the drift is not carried.

**The traces are fixed, not noisy.** The line amplitudes were measured on the
earlier selected members: Station 14 carries A = 160/153/161 Hz over weak→strong
(Station 63: 57/50/65 Hz) — the loop-noise scaling A ∝ 1/√SNR is not
carried; they are fixed traces (~7×10⁻⁸ of the sky frequency, ~1,6×10⁻⁴ of
the 1-MHz bias chain), coherent (FWHM ≤ 0,02 mHz unresolved, Q > 2500). On the
canonical residual the line amplitude re-anchors to ~5 Hz (Station 14, 1988);
the 160-Hz value is re-measured by its own census (workflow
`pioneer-band-amplitude`, run 35119481936, sha `70e2030b`): Station 14, 1988
sub-10-s strength terciles carry A = 169/152/363 Hz over weak→strong
(max/min = 2,15) — weak and mid stand (160/153), the strong tercile rises to
363 Hz, so the flat 160/153/161 scaling is not carried.

**The band lives upstream.** The NOCC's own doppler_resid field
(TRK-2-25 Item 101) carries the band itself (Station 63: 45,00 mHz, 9,2×;
Station 43: 46,45 mHz, 4,0×) — the structure was in the DSN reduction,
before our chain saw it. The stored reference frequency does NOT carry it
(its own 0,1-Hz staircase structure lies beside it, e.g. Station 43:
49,40 mHz). The count structure of the raw doppler_cnt is natively 0,001
cycle (1 000/1 000 bins evenly distributed, 1/256-grid share 0,259 ≈
256/1000) — the MDA resolver/256 divider is measured excluded (Morabito &
Asmar 1995, ref. 3).

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
not the 17,2–22,7 s of the band period (44–58 mHz). The syntonization series of
the station standards (TDA PR 42-72, digitized from the figures) carry drift
rates of 4,6×10⁻¹⁴ to 1,1×10⁻¹² per year; the struck drift claim (§1,
≈4,4×10⁻¹⁴/a from the 47,35 → 46,95 mHz pair) sat within this scale — the
standard drift scale does not exclude a slowly drifting member.

**The P11 route.** Pioneer 11 carries no ATDF archives; the ODF harvest
(TRK-2-18 parser, golden-verified) delivered 27 907 samples 1986–1990 —
100 % 60-s compression, two-way runs at 9 stations. The 0,71-mHz member
is not carried in the raw cells; the two-/three-way split at the
model-subtracted residual (measured for P10 in §4) remains the named next step
for the P11 route.

## 2. The decisive measure

The question "Ground or space?" is decided by a single measure: the
**station-fixity of the frequencies**. The sky frequency is a common
signal — whatever oscillates in space or along the common interplanetary path
would arrive at Goldstone, Canberra and Madrid with an identical frequency. The
measured dominant frequencies are station-different (57,11 / 44,40 / 51,99 mHz,
1988) and remain so across the whole 1-s class. The band knows which antenna
receives it — so it is generated in the receiving chain. The station-local
atmosphere is a named branch — the ionosphere is unmodeled (no GIM before
1998), the troposphere unmeasured; it is closed by the channel separation (§1:
the strength channel carries different members with independent phases) and by
the transmitter-dependent amplitude (§4: an atmosphere above the receiving
antenna does not know which station sent the uplink). All secondary findings
support this reading (upstream, strength-constant, reference-free,
channel-separated); none refutes it.

## 3. What is excluded (measured, no fabrication)

Earth wobble (amplitude gate: 2×10⁵ above the microseism reference);
antenna in wind (channel phases independent); PLL loop noise
(strength constancy); MDA resolver/256 divider (count structure natively
0,001); stored reference (own staircase, no coverage); a common probe
oscillation (phase null); a member sequence (self-TE null); the known
space forces (attitude-independent members, model exclusion of the chain);
the station-local atmosphere (channel separation; transmitter-dependent
amplitude §4).

## 4. What remains open

The two-/three-way split is measured (version 4): the PASF sampler-<10-s class
(sampler < 10 s) three-way (ground mode 3) samples of the NOCC doppler_resid slot
(TRK-2-25 Item 101 — the same series as §1's upstream test) are joined against
the NAVIO (rx, tx) pair,
103 261 matched (nearest within ±120 s); the mode census of the class is 30 973
(1-way) / 11 631 (2-way, station 63 only) / 154 150 (3-way, 78,3 %) — the band
sits in mode 3. The band frequency is receiver-fixed — for a fixed receiver it
is the same across different transmitters — so the line follows the receive
chain, not the transmit chain. The band amplitude, in contrast, depends on the
(receiver, transmitter) pair: within epoch 1988 at comparable n the pair factors
are rx14 13,9×, rx43 ≈3124×, rx63 ≈59×, and the LS amplitude is n-independent —
a two-way-link effect (the uplink sets the strength), not an epoch/n artifact.
The split's own per-rx 1988 peaks (rx14 46,58, rx43 44,12, rx63 50,92 mHz)
differ from the canonical-residual census of §1 (57,11 / 44,40 / 51,99 mHz); the
qualitative station- and receiver-fixity is carried by both series. The value
divergence is measured, not a selection artifact: the split's `topk` rule and the
census `peak_of_cell` rule return the same 1988 numbers on the same file
(rx14 46,58, rx43 44,12, rx63 50,92 mHz), and the raw 6-file harvest carries the
same values — neither the selection rule nor the file set separates them. The §1
numbers are the census of `resid_e`, the full subtraction chain (which also masks
samples: rx14 1988 n 10 182 → 8 450); on the same raw 1988 cell the run-linear
detrend (≤1,0 mHz) and the 0,02/0,05-mHz grid (≤0,6 mHz) are second-order, while
the chain supplies the rest (rx14 +10,1 mHz). The divergence is the processed
series, not the method. The rx63 anchor of the split is
the 1993 epoch peak at 55,9 mHz (52 985 of the 92 130 rx63 samples) — an epoch
peak, not a grid artifact. The named machine of the NOCC reduction is the
Regres formulation of the JPL Orbit Determination Program (Moyer 2000, ref. 12).
Read stage by stage against it, the retrace chain matches the light-time
solution (§8); the charged-particle correction (§10.2.2) is Deduction 2 and stays
empty (GIM maps begin 1998, after the ATDF era); the solar-corona correction
(§10.4) is Deduction 3, its model differing (the OMNI2 N1800 1/r² column against
the corona range model); the time-scale and station-clock algorithms (§2, §7),
the individual-leg troposphere correction (§10.2.1) and the antenna correction
(§10.5) are absent from the chain. The station-fixity of the band is a measured
property of the receive chain. The field census (run `pioneer-cell-census`,
1 071 540 records, 3 receivers) separates the band by record field: the resid
cells (r[8], NOCC doppler_resid) carry the paper signature — 1988 mode3-lt10
peaks rx14 46,581 / rx43 44,119 / rx63 50,920 mHz, mode3-s1.000 46,577 / 47,732
/ 55,959 — while the fsky cells (r[1], raw sky frequency, same cells, same n)
carry a different station-fixed complex (1988 mode3-lt10 46,585 / 52,816 /
45,100; mode3-s1.000 46,579 / 57,842 / 45,094). Two receivers move their
dominant peak between the fields (rx43 +8,7, rx63 −5,8 mHz on the 1988 cell;
+10,1 / −10,9 on s1.000), rx14 does not (Δ 0,002–0,004 mHz). The 44–58-mHz
complex of the resid series is therefore not carried upstream of the reduction
chain. The two fields are two stages of one chain: r[1] is the count-difference
observable (the TRK-2-18 equation — the counter difference over the compression
interval against the bias-chain reference, reconstructed here from the ATDF
items; no ODP stage operates on it), while r[8] has passed the NOCC chain — the
computed-model subtraction (light-time §8, media §10, station clock §2/§7), the
spec-named endpoint average of the compression interval (Residual = (Rj + Ri)/2,
TRK-2-18; transfer cos(πντ), nulling ν = (k+½)/τ — 50 mHz at τ = 10 s) and the
1-mHz ATDF quantization (item 60). The dominant movement between the fields is
a re-ranking within the dense station-fixed complex, not a creation; the
stage-by-stage ablation of §1 measures the same act (rx14 45,75 → 57,11 mHz
under the daily-curve cut, the 57,11 member at rank 6 before it). rx14's
46,58-mHz member dominates both fields — it survives two different pipelines;
whether rx43's 44,119 and rx63's 50,920 sit as sub-members in their fsky cells
(one complex, re-ranked) or are absent there (created inside the NOCC chain) is
the open measure — the probe holds both fields on the same records, and the
cross-rank of the two dominants per cell is the decisive print. The fsky complex
itself needs no reduction stage: it is receive-chain-borne, shaped only by the
observable's own constructions — the first difference over the sampler interval
(transfer 2 sin(πντ), nulls at k/τ, near-unity across the band for τ ≤ 10 s) and
the reference staircase mix-in (×104,25), the latter measured beside the band
(§1). §8 (light-time) remains the first named candidate for the resid complex,
refined from carrier to re-ranker. The 50,000-mHz concentration is the sampling
raster, not a 20-s signal: for a sampling interval Δt with Δt × 50 mHz integer
(Δt = 20, 40, 60, 80, 100, 120 s), the grid frequency 50,000 mHz is the fold
image of the series' near-DC content, and the fine classes (Nyquist ≥ 50 mHz)
carry none — 218 census lines peak at exactly 50,000 mHz, 214 of them in the
10–30-s and 60-s sampler classes across all receivers and years (both fields), 0
in the resid fine classes (whose peaks span 44,0–58,0 and carry the
station-fixed band), 4 in fsky fine cells (all st63-lt10-1992; the cell st63
mode1 lt10 1992, n=9 370, reads resid 51,360 / fsky 50,000). The four fsky-fine
cells lie outside the fold argument; the reference-path hypothesis (the item-40
staircase, ×104,25 in r[1], on both sides of the residual difference) is the
nearest unmeasured candidate — the decisive measurement is the LS of the
reference field r[2] in these cells plus the per-cell floor ratio.
The per-station census of the canonical residual is the standing measurement
(the era × station × class × band table); the 1989–1991 Markwardt-ATDF files are
absent from the harvest (a genuine archive gap — that product covers 1987/1988/1994,
and Markwardt himself documents an unreadable-tape gap for June 1990–June 1991
(gr-qc/0208046)),
but the doppler value is not absent: it is harvested from the TRK-2-25 ODF
`86334o97343_sc23.odf` (into `pioneer10_odf.bin`, 1973–1998) and from the ASCII
ODDUMP `.asc.gz` (the NAVIO series, 1973–2002).

## 5. Position against the literature

Station-dependent periodic residuals are a documented artifact class in
Pioneer Doppler. Levy et al. (2009, Adv. Space Res.) resolve periodic terms
at the sidereal day and its harmonic and attribute them to station-condition-
dependent media-model errors; Anderson et al. (2002) report the diurnal and
seasonal variation; Bertotti & Giampieri (1998) treat solar-coronal plasma as
dispersive Doppler noise. The ~20-s complex (44–56 mHz) with station-dependent
dominant frequencies reported here does not appear in these treatments or the review
literature (Turyshev & Toth 2010), whose periodicities sit at day and year
scales. The reported band is therefore a new instance of an established
artifact class, not a new class. Whether a 20-s-scale line lives in the
JPL-internal TDA/IPN report record (the class of documents cited in
§1) is beyond the open record — a limit of this search, not a proof of
novelty.

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
7. Levy A., Christophe B., Bério P., Métris G., Courty J.-M., Reynaud S.,
   2009, Pioneer 10 Doppler data analysis: disentangling periodic and secular
   anomalies, Adv. Space Res. 43, 1538.
8. Anderson J. D., Laing P. A., Lau E. L., Liu A. S., Nieto M. M.,
   Turyshev S. G., 2002, Study of the anomalous acceleration of Pioneer
   10 and 11, Phys. Rev. D 65, 082004.
9. Bertotti B., Giampieri G., 1998, Solar coronal plasma in Doppler
   measurements, Solar Phys. 178, 85.
10. Seward, 1983, Standards Syntonization in the Deep Space Network, TDA
    Progress Report 42-72 (October–December 1982), Jet Propulsion Laboratory.
11. Markwardt C. B., 2002, Independent Confirmation of the Pioneer 10
    Anomalous Acceleration, arXiv gr-qc/0208046.
12. Moyer T. D., 2000, Formulation for Observed and Computed Values of Deep
    Space Network Data Types for Navigation, DESCANSO Monograph 2, JPL
    Publication 00-7, Jet Propulsion Laboratory, Pasadena.
