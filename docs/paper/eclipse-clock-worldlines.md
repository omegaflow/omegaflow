<!--
  title: The eclipse clock — greatest eclipse from five ephemeris worldlines
  class: paper
  date: 2026-09-13
  sha256: 08c1016eefe1d881b7ec63cbf709ce0f63f81d3e89c10f4cd135d9bfd0c77ebc
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-finsternis-schattenortung.md
-->

# The eclipse clock — greatest eclipse from five ephemeris worldlines

## Abstract

The greatest eclipse of 2017-08-21 is located from five independent Sun/Moon/Earth worldlines alone; no eclipse catalog enters the computation. The probe runs five stages — geocentric syzygy, shadow-axis nearest to the geocentre, deepest ground coverage, the pairwise rift, and the rotation/curvature audit — and maps each instant to the rotating surface through body_fixed_to_icrs_smooth; magnitude is the ratio of apparent diameters. On the one-voice line de440/de442/epm2021 greatest eclipse stands at 36.9664N 87.6176W, 4.8 km from the Espenak canon point 36.9667N 87.6717W, magnitude 1.03147 (+0.0009 from the canon 1.0306), at 18:25:34.9Z — +4.9 s after the canon's UTC instant 18:25:30Z (the canon carries TD 18:26:40, ΔT 70 s on the catalog row). The earlier "65 s before the canon" was the canon's dynamical-time scale laid on the UTC axis, not a physics claim. The house rift is inpop19a (3.05 s / 1.7 km); de441 has drifted from the one voice to 42.5 km / −90.5 s and is named for re-verification.

## The measurement

The probe eclipse_shadow_probe computes the greatest eclipse from the raw Sun/Moon/Earth worldlines; the Espenak canon is read only at the verdict, beside the result, never inside the search. Five worldline sets are loaded, each carrying its own Sun, Moon and Earth granule:

- de441 — ssd.jpl.nasa.gov, ephemeris_sun.bin, ephemeris_moon.bin, ephemeris_earth.bin (Horizons banner {source: DE441})
- de440 — ssd.jpl.nasa.gov, ephemeris_de440_sun.bin, ephemeris_de440_moon.bin, ephemeris_de440_earth.bin
- de442 — ssd.jpl.nasa.gov, ephemeris_de442_sun.bin, ephemeris_de442_moon.bin, ephemeris_de442_earth.bin
- inpop19a — ftp.imcce.fr, ephemeris_inpop_sun.bin, ephemeris_inpop_moon.bin, ephemeris_inpop_earth.bin
- epm2021 — ftp.iaaras.ru, ephemeris_epm_sun.bin, ephemeris_epm_moon.bin, ephemeris_epm_earth.bin

The search runs five stages. Stage 1 finds the geocentric syzygy — the minimum geocentric elongation of Sun and Moon. Stage 2a finds greatest eclipse: the instant at which the shadow axis passes nearest to the geocentre, and its surface intersection. Stage 2b finds the deepest ground coverage — the greatest-duration locus. Stage 3 lays every line beside every other and prints the pairwise rift. Stage 4 measures the rotation path (the TDB-fed angle against the UT1-fed angle) and the curvature of the axis-miss minimum. Each surface instant is mapped to the rotating Earth through body_fixed_to_icrs_smooth (the smooth rotation path, not the nearest-rotation-matrix branch — that defect is named under The form). Magnitude is the ratio of the apparent diameters: the Moon's angular radius over the Sun's.

## The finding

For the 2017-08-21 event the one-voice line places greatest eclipse at 36.9664N 87.6176W — 4.8 km from the Espenak canon point 36.9667N 87.6717W — magnitude 1.03147, +0.0009 from the canon 1.0306, at 18:25:34.9Z. The canon 18:26:40 is Terrestrial Dynamical Time (catalog row 09546 prints "TD of Greatest Eclipse 18:26:40", ΔT 70 s on the same row); converted by the catalog's own ΔT it is 18:25:30 UT, so the instant sits +4.9 s after the canon. The h-minimum is flat; the position and the magnitude sit inside the calibration gate, the instant is the residual.

The one voice across de440, de442 and epm2021: geocentric Moon 0.3 m apart, Earth centre 191 m apart, eclipse time under 1 ms apart. The Sun triad (DE441/INPOP/EPM) united carries pairwise separations of 22.4/16.8/32.0 km. inpop19a sits 3.05 s / 1.7 km from DE, +1.8 s / 6.5 km from the canon.

## The 65 s, decomposed

The earlier handover carried the instant as "65 s before the canon". The verdict laid the canon constant on the UTC axis (canon_unix = EVENT_UNIX + 18:26:40); the catalog's greatest-eclipse column is TD, ΔT 70 s on the same row. 65 s = −70 s (TD→UT) + ~+5 s (the true residual). 2024 reproduces the same shape: 71 s = −74 s (ΔT 74 s) + ~+3 s — +2.6 s on de440/de442/epm2021, +5.6 s on inpop19a, +10.1 s on de441.

Two channels carry the two residual numbers, and they do not mix. The +4.9 s instant is geocentric geometry — rotation does not enter the axis-nearest-geocentre search — and is the flat-minimum amplification of a small axis difference between Espenak's Meeus algorithm and the raw worldlines: h'' ≈ 0.35–0.59 m/s² at the minimum, so 4.9 s corresponds to a ~65–85 m axis displacement. The 4.8 km point is the rotation channel: body_fixed_to_icrs_smooth feeds the Earth rotation angle with TDB, not UT1; the TDB-fed rotation leads the UT1-fed rotation by 0.2939° = 26.1 km of longitude (ΔT 70.3 s from the Espenak–Meeus polynomial delta_t_espenak_meeus in astrometry.rs; the catalog row rounds it to 70 s). The measured pierce sits 4.8 km from the canon, not 26.1 km — where the remaining ΔT is absorbed is a pending measurement, the w0 anchor's provenance not yet read from the bin.

## The form

- The de441 line has drifted from the one voice: on the current bins it places greatest eclipse at 42.5 km / −90.5 s (2017) and +10.1 s (2024), and its Earth centre sits 116 km from de440/de442. This is a data-state defect named for re-verification (register duty), not a physics claim.
- The body_fixed_to_icrs matrix path defect stands as fixed (2026-09-09: Rz(90+α)·Rx(90−δ)·Rz(W), pole read from column 2); de441 mars still carries pre-fix matrices (Δ anchor 6045.3 km) and is the named remaining stone.
- The calibration gate is data-bound: point < 15 km, magnitude < 0.002. It holds on the one-voice line (4.8 km, +0.0009). Against the drifted de441 bins it would read 42.5 km; the gate test still names LINES[0]=de441 and carries a silent early-return when data/ is absent, so its read of the drift is pending the de441 re-verification.

## References

1. Espenak, F., & Meeus, J. 2006, Five Millennium Canon of Solar Eclipses: −1999 to +3000, NASA/TP-2006-214141. [ADS 2006fmcs.book.....E — resolved via ADS API 2026-09-12]
2. Folkner, W. M., Williams, J. G., Boggs, D. H., Park, R. S., & Kuchynka, P. 2014, "The Planetary and Lunar Ephemerides DE430 and DE431", IPN Progress Report 42-196, 1. [ADS 2014IPNPR.196C...1F — standard literature, not carried by the sources]
3. Park, R. S., Folkner, W. M., Williams, J. G., & Boggs, D. H. 2021, "The JPL Planetary and Lunar Ephemerides DE440 and DE441", AJ, 161, 105. [DOI 10.3847/1538-3881/abd414; ADS 2021AJ....161..105P — standard literature, not carried by the sources]
4. Fienga, A., Deram, P., Viswanathan, V., et al. 2019, "INPOP19a planetary ephemerides", Notes Scientifiques et Techniques de l'IMCCE, S109. [ADS 2019NSTIM.109.....F — resolved via ADS API 2026-09-12]
5. Pitjeva, E. V., & Pitjev, N. P. 2014, "Development of planetary ephemerides EPM and their applications", Celest. Mech. Dyn. Astron., 119, 237. [DOI 10.1007/s10569-014-9569-0]
