<!--
  title: The eclipse clock — greatest eclipse from five ephemeris worldlines
  class: paper
  date: 2026-09-12
  sha256: 7e2a08c6f0f468e2ae9eef21117fd80442c153bf3c37f7ddb0a84272177d1611
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-finsternis-schattenortung.md
-->

# The eclipse clock — greatest eclipse from five ephemeris worldlines

## Abstract

The greatest eclipse of 2017-08-21 is located from five independent Sun/Moon/Earth worldlines alone; no eclipse catalog enters the computation. The probe runs three stages — geocentric syzygy, shadow-axis nearest to the geocentre, deepest ground coverage — and maps each instant to the rotating surface through body_fixed_to_icrs_smooth; magnitude is the ratio of apparent diameters. On the de441 line greatest eclipse stands at 36.9664N 87.6176W, 4.8 km from the Espenak canon point 36.9667N 87.6717W, with magnitude 1.03102 (+0.0004 from the canon 1.0306), at 18:25:34.9Z — 65 s before the canon 18:26:40. The three NASA editions (de440, de441, de442) are one voice: geocentric Moon 0.3–2.0 m apart, Earth centre 0–191 m, under 2 ms eclipse time. The house rift is inpop19a (3.05 s / 1.7 km); epm2021 sits at DE. The calibration gate holds: point < 15 km, magnitude < 0.002.

## The measurement

The probe eclipse_shadow_probe computes the greatest eclipse from the raw Sun/Moon/Earth worldlines; the Espenak canon is read only at the verdict, beside the result, never inside the search. Five worldline sets are loaded, each carrying its own Sun, Moon and Earth granule:

- de441 — ssd.jpl.nasa.gov, ephemeris_sun.bin, ephemeris_moon.bin, ephemeris_earth.bin (Horizons banner {source: DE441})
- de440 — ssd.jpl.nasa.gov, ephemeris_de440_sun.bin, ephemeris_de440_moon.bin, ephemeris_de440_earth.bin
- de442 — ssd.jpl.nasa.gov, ephemeris_de442_sun.bin, ephemeris_de442_moon.bin, ephemeris_de442_earth.bin
- inpop19a — ftp.imcce.fr, ephemeris_inpop_sun.bin, ephemeris_inpop_moon.bin, ephemeris_inpop_earth.bin
- epm2021 — ftp.iaaras.ru, ephemeris_epm_sun.bin, ephemeris_epm_moon.bin, ephemeris_epm_earth.bin

The search runs in three stages. Stage 1 finds the geocentric syzygy — the minimum geocentric elongation of Sun and Moon. Stage 2a finds greatest eclipse: the instant at which the shadow axis passes nearest to the geocentre, and its surface intersection. Stage 2b finds the deepest ground coverage — the greatest-duration locus. Stage 3 lays every line beside every other and prints the pairwise rift. Each surface instant is mapped to the rotating Earth through body_fixed_to_icrs_smooth (the smooth rotation path, not the nearest-rotation-matrix branch — that defect is named under The form). Magnitude is the ratio of the apparent diameters: the Moon's angular radius over the Sun's.

## The finding

For the 2017-08-21 event the de441 line places greatest eclipse at 36.9664N 87.6176W — 4.8 km from the Espenak canon point 36.9667N 87.6717W — with magnitude 1.03102, +0.0004 from the canon 1.0306, at 18:25:34.9Z, 65 s before the canon 18:26:40. The h-minimum is flat; the position and the magnitude are exact, the instant is the residual.

The three NASA editions are one voice: across de440, de441 and de442 the geocentric Moon lies 0.3–2.0 m apart, the Earth centre 0–191 m apart, and the eclipse time stays under 2 ms. The rift lies between the houses: inpop19a differs by 3.05 s and 1.7 km, while epm2021 sits at DE. The Sun triad (DE441/INPOP/EPM) united carries pairwise separations of 22.4/16.8/32.0 km.

## The form

- The 65 s offset of the h-minimum instants is carried as a named residual finding, not as a physics claim: the minimum is flat, position and magnitude agree with the canon, and the instant is the one number that does not. It reproduces in 2024 at ~71 s. Further inquiry is optional; the finding stands named.
- The body_fixed_to_icrs matrix path is named: the nearest-rotation-matrix branch rotated the 2017 Earth surface ~117° of longitude wrong on pre-fix bins (sub-solar point: matrix path −2.2N/+21.2E; the smooth path 12.0N/−95.8W). The builder defect is fixed (2026-09-09: Rz(90+α)·Rx(90−δ)·Rz(W), pole read from column 2); four of five stale bins were recompiled and re-verified (orientation_probe Δ 0.0 km on de440/de442/inpop/epm earth); de441 mars still carries pre-fix matrices (Δ anchor 6045.3 km) and is the named remaining stone.
- The calibration gate is data-bound: point < 15 km and magnitude < 0.002 from the canon, asserted in the probe's test module on the de441 line.

## References

1. Espenak, F., & Meeus, J. 2006, Five Millennium Canon of Solar Eclipses: −1999 to +3000, NASA/TP-2006-214141. [ADS 2006fmcs.book.....E — resolved via ADS API 2026-09-12]
2. Folkner, W. M., Williams, J. G., Boggs, D. H., Park, R. S., & Kuchynka, P. 2014, "The Planetary and Lunar Ephemerides DE430 and DE431", IPN Progress Report 42-196, 1. [ADS 2014IPNPR.196C...1F — standard literature, not carried by the sources]
3. Park, R. S., Folkner, W. M., Williams, J. G., & Boggs, D. H. 2021, "The JPL Planetary and Lunar Ephemerides DE440 and DE441", AJ, 161, 105. [DOI 10.3847/1538-3881/abd414; ADS 2021AJ....161..105P — standard literature, not carried by the sources]
4. Fienga, A., Deram, P., Viswanathan, V., et al. 2019, "INPOP19a planetary ephemerides", Notes Scientifiques et Techniques de l'IMCCE, S109. [identifier not carried by the sources — pending verification]
5. Pitjeva, E. V., & Pitjev, N. P. 2018, "Development of planetary ephemerides EPM and their applications", Celest. Mech. Dyn. Astron., 130, 57. [identifier not carried by the sources — pending verification]
