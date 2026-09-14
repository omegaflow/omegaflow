<!--
  title: The eclipse clock — greatest eclipse from five ephemeris worldlines
  class: paper
  date: 2026-09-13
  sha256: 476e4b7fef27949e17a1be867ee94d24fbee2def9f2f998d3ee63cfcc9f7c0e2
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-finsternis-schattenortung.md
-->

# The eclipse clock — greatest eclipse from five ephemeris worldlines

## Abstract

The greatest eclipse of 2017-08-21 is located from five independent Sun/Moon/Earth worldlines alone; no eclipse catalog enters the computation. The probe runs five stages — geocentric syzygy, shadow-axis nearest to the geocentre, deepest ground coverage, the pairwise rift, and the rotation/curvature audit — and maps each instant to the rotating surface through body_fixed_to_icrs_smooth; magnitude is the ratio of apparent diameters. On the one-voice line de440/de442/epm2021 greatest eclipse stands at 36.9664N 87.6176W, 4.8 km from the Espenak canon point 36.9667N 87.6717W, magnitude 1.03147 (+0.0009 from the canon 1.0306), at 18:25:34.9Z — +4.9 s after the canon's UTC instant 18:25:30Z (the canon carries TD 18:26:40, ΔT 70 s on the catalog row). The earlier "65 s before the canon" was the canon's dynamical-time scale laid on the UTC axis, not a physics claim. The house rift is inpop19a (3.05 s / 1.7 km); the de441 42.5 km / −90.5 s reading was a stale local 19-MB earth bin (2026-09-09) — on the 2026-09-11 CDN asset the de441 line re-verified into the one voice (2026-09-13), see The form.

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

Two channels carry the two residual numbers, and they do not mix. The +4.9 s instant is geocentric geometry — rotation does not enter the axis-nearest-geocentre search — and is the flat-minimum amplification of a small axis difference between Espenak's Meeus algorithm and the raw worldlines: h'' ≈ 0.35–0.59 m/s² at the minimum, so 4.9 s corresponds to a ~65–85 m axis displacement. The 4.8 km point is the rotation channel: body_fixed_to_icrs_smooth feeds the Earth rotation angle with TDB, not UT1; the TDB-fed rotation leads the UT1-fed rotation by 0.2939° = 26.1 km of longitude (ΔT 70.3 s from the Espenak–Meeus polynomial delta_t_espenak_meeus in astrometry.rs; the catalog row rounds it to 70 s). The measured pierce sits 4.8 km from the canon, not 26.1 km. The w0 anchor's provenance is read: w0 = 190.1470° (the IAU prime-meridian angle W at J2000), identical across all five earth bins, sourced from PCK `BODY399_PM` coefficient 1 in pck00010.tpc/pck00011.tpc (NAIF), written as slot 4 (`w0_deg` = `wgccre.pm_deg`) in the ephemeris bin. w0 is a fixed IAU constant at J2000 and absorbs zero ΔT: no UT1 correction, no ΔT subtraction, no separate ΔT term sits anywhere in body_fixed_to_icrs_smooth / orientation_angles_at / iau_rotate_to_icrs; ΔT enters the rotation chain at exactly one point, the time-axis argument jd = tdb / 86400.0 + J2000_EPOCH. The rotation audit measures the full ΔT effect 0.2939° = 26.1 km (ΔT 70.3 s); the pierce lands 4.8 km from the canon because it is re-searched (deepest_pierce sweeps the full surface at the TDB greatest-eclipse instant, +4.9 s time-channel), so the 26.1 − 4.8 = 21.3 km residual is absorbed by the re-derived instant/point. The exact sub-point sweep-rate over that +4.9 s interval remains unmeasured.

## The form

- The de441 line re-verified into the one voice (2026-09-13, probe + gate on the 2026-09-11 CDN asset): greatest eclipse 4.8 km / +4.9 s (2017) and +2.7 s (2024), Earth centre 0.0 m from de440 and 191.3 m from de442 — the 42.5 km / −90.5 s / +10.1 s drift was a stale local 19-MB earth bin (2026-09-09 15:52Z) that the ttl-served cache carried past the CDN regeneration, not a defect of the de441 data.
- The body_fixed_to_icrs matrix path defect stands as fixed (2026-09-09: Rz(90+α)·Rx(90−δ)·Rz(W), pole read from column 2); de441 mars re-verified 2026-09-13 on the 2026-09-11 asset: anchor Δ 0.0 km (the pre-fix 6045.3 km reading is closed).
- The calibration gate is data-bound: point < 15 km, magnitude < 0.002. It holds on the one-voice line de440 (4.8 km, +0.0009); the de441 re-verification read 4.8 km / +0.0009 — the same numbers. Where the data bins sit absent, the gate names its skip and stays unrun — never a silent no-op.
- The 2024 canon point Δ reads 793.3 km on every line alike (de440/de442/de441/epm2021/inpop) — and it is a search-box boundary, not a point definition and not an ephemeris drift. The unseeded deepest_pierce that sets the stage-2a point passes the same half-range (90°) to latitude and longitude, so its first sweep spans longitude [−90°, +90°] around lon 0° (latitude needs ±90°, longitude needs ±180° to close the circle); the shrinking refinement (5° → 1° → 0.2° → 0.04° → 0.008°) then drifts at most 5+1+0.2+0.04+0.008 = 6.248° past the box edge and terminates at −96.248°W (= −90° − 6.248°, the printed value, on all five lines). The 2024 greatest-eclipse point lies at 25.3°N 104.1°W (NASA SEdata, catalog row 09561; the probe's canon constant −104.1383°) — 14.138° beyond 90°W — and is never reached; the 7.89° longitude gap at 25.26°N is the 793.3 km. The 2017 point (87.67°W) sits inside [−90°, +90°], so the same search lands 4.8 km from the canon. Stage 2b's seeded walk tracks the axis across the window and reads the pierce: fraction 1.02874 = (1 + 1.05748)/2, the θ = 0 axis magnitude, at 104.62°W — the axis, not the definition, is what stage 2a fails to reach. The probe now sweeps the unseeded deepest_pierce with half_lon 180° and half_lat 90° (longitude closes the circle, latitude needs only ±90°); the 2024 re-verification (2026-09-13, on the compiled tree) reads the stage-2a point at 8.4 km from the canon (de440/de442/de441/epm2021; 7.1 km on inpop19a) and magnitude +0.0009 (1.05748 against the canon 1.0566) — the 793.3 km boundary artifact is closed, and the calibration gate (point < 15 km, magnitude < 0.002) holds.

## References

1. Espenak, F., & Meeus, J. 2006, Five Millennium Canon of Solar Eclipses: −1999 to +3000, NASA/TP-2006-214141. [ADS 2006fmcs.book.....E — resolved via ADS API 2026-09-12]
2. Folkner, W. M., Williams, J. G., Boggs, D. H., Park, R. S., & Kuchynka, P. 2014, "The Planetary and Lunar Ephemerides DE430 and DE431", IPN Progress Report 42-196, 1. [ADS 2014IPNPR.196C...1F — standard literature, not carried by the sources]
3. Park, R. S., Folkner, W. M., Williams, J. G., & Boggs, D. H. 2021, "The JPL Planetary and Lunar Ephemerides DE440 and DE441", AJ, 161, 105. [DOI 10.3847/1538-3881/abd414; ADS 2021AJ....161..105P — standard literature, not carried by the sources]
4. Fienga, A., Deram, P., Viswanathan, V., et al. 2019, "INPOP19a planetary ephemerides", Notes Scientifiques et Techniques de l'IMCCE, S109. [ADS 2019NSTIM.109.....F — resolved via ADS API 2026-09-12]
5. Pitjeva, E. V., & Pitjev, N. P. 2014, "Development of planetary ephemerides EPM and their applications", Celest. Mech. Dyn. Astron., 119, 237. [DOI 10.1007/s10569-014-9569-0]
6. NASA GSFC, "Total Solar Eclipse of 2024 April 08 — Besselian Elements", SEdata, eclipse.gsfc.nasa.gov/SEsearch/SEdata.php?Ecl=20240408 (retrieved 2026-09-13): greatest eclipse 18:18:29 TDT = 18:17:15 UT, γ 0.3431, magnitude 1.0566, ΔT 74 s, circumstances 25.3°N 104.1°W.
