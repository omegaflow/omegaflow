<!--
  title: The solar 24-s seconds matrix — a 72-pair fleet measurement (XRS to EUV)
  class: paper
  date: 2026-09-12
  sha256: f14341db1d9206954403de76b4c1b1b6e4af7cbb80f195f957fcd6e7c52980c0
  status: live
  see-also: docs/blatt/blatt-solar-seconds-matrix.md
-->

# The solar 24-s seconds matrix — a 72-pair fleet measurement (XRS to EUV)

## Abstract

The solar EUV–X-ray coupling is measured as a 72-pair directed fleet: seven SDO/AIA EUV bands (304, 131, 171, 193, 211, 335, 94 Å) and the two GOES XRS channels XRSA and XRSB, nine actors, on 24-s cells over 2013–2015. Transfer entropy runs on flare-stacked windows in the GOES b_flux band with ten phase-randomized surrogates per cell; the family threshold is the strongest surrogate D over all 72 directed pairs × 13 lags × 10 surrogates, fam = 1.3966e-1. Four arrows clear the bound: 211A→193A (96 s, D = 1.658e-1), XRSA→131A (192 s, D = 1.468e-1), XRSB→131A (96 s, D = 1.644e-1), XRSB→193A (96 s, D = 1.485e-1). The hard X-ray channels drive the hot EUV bands 131A and 193A; the reverse EUV→XRS stays nearly silent. The matrix reduces to 4 arrows / 43 family bound / 25 still. The lone intra-EUV arrow 211A→193A is pending the conditional check.

## The measurement

The solar seconds matrix stacks nine actors on a common 24-s grid over 2013–2015: the seven SDO/AIA EUV bands (304, 131, 171, 193, 211, 335, 94 Å) and the two GOES XRS channels, XRSA (0.05–0.4 nm) and XRSB (0.1–0.8 nm). Nine actors give 9 × 8 = 72 directed pairs. The probe is `tools/measure/src/bin/solar_seconds_matrix_probe.rs` in the `omegaflow-measure` crate.

The coupling is measured with transfer entropy. Flare events are cut from the GOES XRSB b_flux as 24-s medians above the C5.0 threshold 5e-6 W/m², with a refractory of 75 cells (30 min); each event window spans ±40 min around the peak. A directed pair A→B reads "A drives B": transfer entropy runs with B as target and A as driver, and each event contributes D = (TE(A→B) − TE(B→A)) / (|TE(A→B)| + |TE(B→A)|). The pair stacks the mean per-event D over its events. Lags span 0–12 cells (0–288 s). Each cell carries ten phase-randomized surrogates. The family threshold is the strongest surrogate stacked D over all 72 directed pairs × 13 lags × 10 surrogates: fam = 1.3966e-1. A pair is an ARROW when D > fam, a family bound when D exceeds its own mean + 2σ surrogate floor but not fam, still otherwise; a pair below the event floor carries no statement.

The XRS side is registered at the shared CDN as `goes_xrs.bin` (fields `goes_xrs_xrsa`, `goes_xrs_xrsb`); the EUV-lines side is registered as `eve_lines_2011.bin`. Both are listed in `phi/sources.φ`. The measured seconds corpus itself is the AIA fullyear/monthly bins from `jsoc.stanford.edu` and the GOES-15 2-s XRS year tars (`goes15_xrs_2s_{2013,2014,2015}.tar`) from `ncei.noaa.gov`.

## The finding

| edge | lag | D | > fam | direction |
|---|---|---|---|---|
| 211A → 193A | 96 s | 1.658e-1 | yes | intra-AIA |
| XRSA → 131A | 192 s | 1.468e-1 | yes | XRS → AIA |
| XRSB → 131A | 96 s | 1.644e-1 | yes | XRS → AIA |
| XRSB → 193A | 96 s | 1.485e-1 | yes | XRS → AIA |

The distribution over the matrix: intra-AIA (42 pairs) 1 ARROW / 28 family bound / 13 still; AIA–XRS both directions (28 pairs) 3 ARROW / 14 family bound / 11 still; XRS-internal (2 pairs) 1 family bound / 1 still. Over the full matrix: 4 ARROW / 43 family bound / 25 still / 0 no-statement.

The verdict: the hard X-ray channels show arrows onto the hot EUV bands 131A and 193A (96–192 s, above the family threshold). The reverse (AIA → XRS) stays nearly silent: the EUV glow does not drive the hard X-ray emission. The causal direction runs from the hard flare signal to the EUV afterglow band inside the GOES window, not the other way. The silence of the 25 pairs is a finding, not noise (0 honored).

Pending: 211A → 193A, the only intra-AIA arrow, carries one witness (the matrix itself). 211 and 193 both answer the X-ray with somewhat different delays, and the pairwise transfer entropy reads that as "211 drives 193". This is the same class (intra-AIA, shared driver) at which the conditional probe measured its envelope-artifact tipping. Stage 2 of the division of labor: the matrix sieves, the conditional probe arbitrates. The arrow is pending the conditional check — labeled honestly, not over-interpreted.

## The form

- **Band cadence:** 24-s cells; lags 0–12 cells (0–288 s).
- **Flare stacking:** in the GOES b_flux window; per-event D normalized to [−1, 1] by the sum of the absolute transfer entropies.
- **Family pooling:** fam = strongest surrogate stacked D over 72 directed pairs × 13 lags × 10 surrogates — a multiple-comparison bound over the whole matrix, not a per-pair floor.
- **Event floor:** a pair with fewer than the aligned-cell floor carries no statement (0 honored).
- **Corpus anchor:** commit `b3b6a24`, corpus `4f0bcf2c465d7c82eedb1e1ef978f33f9baf4fc952603a1fb06e20e8b67baa2e`; one probe-corpus fetch fell transient and was retrieved by a targeted re-fetch; the reduce folded all 72 reports into one sheet without `missing` (no invented value for a gap).
- **Limits:** the conditional check on 211A → 193A is pending; F10.7, Lyman-α and OMNI Bz/density live at daily/hourly cadence — not seconds-resolvable — and are excluded. The single scalar family threshold does not test the conditional structure the arbitration stage carries.

## References

1. Neupert, W. M. (1968). Comparison of solar X-ray line emission with microwave emission during flares. *The Astrophysical Journal* 153, L59. ADS: 1968ApJ...153L..59N
2. Garcia, H. A. (1994). Temperature and emission measure from GOES soft X-ray measurements. *Solar Physics* 154, 275–308. ADS: 1994SoPh..154..275G
3. Lemen, J. R., Title, A. M., Akin, D. J., et al. (2012). The Atmospheric Imaging Assembly (AIA) on the Solar Dynamics Observatory (SDO). *Solar Physics* 275, 17–40. ADS: 2012SoPh..275...17L
4. Woods, T. N., Eparvier, F. G., Hock, R., et al. (2012). Extreme Ultraviolet Variability Experiment (EVE) on the Solar Dynamics Observatory (SDO): overview of science objectives, instrument design, data products, and model developments. *Solar Physics* 275, 115–143. ADS: 2012SoPh..275..115W
