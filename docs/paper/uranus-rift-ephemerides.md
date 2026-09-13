<!--
  title: The Uranus rift — ephemeris divergence at the planet centre
  class: paper
  date: 2026-09-12
  sha256: bd50d132d0e3d368f4c5346b4008c42f2a4f7edf56867794a9181577d529b5a1
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-uranus-riss-kontur.md docs/handover/archiv/handover-2026-09-08-uranus-riss-diurnal-reduktion.md
-->
# The Uranus rift — ephemeris divergence at the planet centre

## Abstract

Three independent planetary ephemeris families — JPL DE, IMCCE INPOP, and IA RAS EPM — are read in one frame at the centre of Uranus. Pairwise divergences are DE−INPOP 32.1, DE−EPM 39.5, and INPOP−EPM 47.0 mas (standard error ~1.3 mas). A topocentric diurnal reduction removes the parallax/aberration confound: the fitted parallax coefficient is c_par 0.94–0.95 and the diurnal aberration coefficient c_aber ≈ 0, and the residual RMS falls from 242.9/222.6/228.9 mas to 77.5/73.6/73.3 mas — under the observational mean uncertainty ⟨σ⟩ = 87.8 mas. The observations do not adjudicate the rift. At the planet centre the spread is 0.36–1.57e6 m (0.048″ median); two editions of one house, DE441 and DE442, diverge by 1.57e6 m, against the 1.59e6 m Weberin rift. The absolute-offset/aberration decomposition is measured: the absolute satellite residual per line is |c0| = 11.8 mas (EPM2021), 14.1 mas (DE441), 35.9 mas (INPOP19a) — EPM2021 lies closest to zero.

## The measurement

Three ephemeris families are read in one frame at the centre of Uranus:

- **DE (NASA JPL, ssd.jpl.nasa.gov).** The CDN asset `ephemeris_uranus.bin` carries the DE441 barycentre; `ephemeris_uranus_c.bin` carries the DE441 barycentre composed with the ura111xl-799 moon model (2-d granules, 0.1-d sampling, window 1980–2040); `ephemeris_de440_earth.bin` and `ephemeris_de442_earth.bin` carry the two DE editions.
- **INPOP (IMCCE).** `ephemeris_inpop_uranus.bin` (ftp.imcce.fr).
- **EPM (IA RAS).** `ephemeris_epm_uranus.bin` (ftp.iaaras.ru).

The pairwise divergence at the planet centre across the three lines: DE−INPOP 32.1, DE−EPM 39.5, INPOP−EPM 47.0 mas, with a standard error ~1.3 mas.

The raw absolute satellite residual carries RMS ~220 mas, mean ~165 mas, with a ~140 mas variation over ~1.2 h — the signature of a diurnal term (parallax, diurnal aberration, or both). The term is common-mode across the three lines (ΔRMS ~6 mas), so the pairwise differences are clean of it, but the absolute statement needs it removed. The corrected station (WGS84 + IAU 1982 GMST + analytic ω×r) yields the decomposition c_par 0.94–0.95, c_aber ≈ 0: the published satellite tables carry the topocentric parallax and no diurnal aberration, topocentric astrometric, as the source paper states. The reduction moves the residual RMS 242.9/222.6/228.9 → 77.5/73.6/73.3 mas — under ⟨σ⟩ = 87.8 mas. The ~170-mas floor was the parallax itself.

## The finding

The residual divergence at the planet centre is 0.36–1.57e6 m, with a 0.048″ median. The spread is not a bureau dispute: DE441 and DE442, two editions of the same house, diverge by 1.57e6 m, against the 1.59e6 m Weberin rift across three houses. This is scale consistency, not identity — two different measurement pairs. That two editions of one house diverge at rift scale at the centre measures the Uranus position as observationally underdetermined: no two orbits since discovery, one visitor (1986), a century of arcminute astrometry.

Verdict: the observations do not adjudicate the rift. After the diurnal reduction all three lines lie under ⟨σ⟩ = 87.8 mas, so the divergence is not an observable contradiction in the absolute sense; the rift anchor (32.1/39.5/47.0 mas) stands as the calibration anchor under any reduction change.

## The form

- **What the reduction removed.** The topocentric parallax (c_par 0.94–0.95) and the diurnal aberration (c_aber ≈ 0) — the common-mode ~170-mas floor. The pairwise rift is invariant under the reduction (32/39/47 mas); a rift that moved would name the reduction as the artifact, not the measurement.
- **The absolute decomposition (measured).** The absolute-offset/aberration decomposition: which line carries the observations closest to zero in the absolute residual, once the diurnal term is gone. The ~20″ aberration and ~0.45″ parallax split must isolate the diurnal share rather than the whole. Measured: the absolute residual per line is |c0| = 11.8 mas (EPM2021), 14.1 mas (DE441), 35.9 mas (INPOP19a) — EPM2021 lies closest to zero. The tables carry ~0.94–0.97 of the ~0.45″ diurnal parallax (~190 mas) and essentially none of the ~20″ annual aberration (c_ann ≈ −0.001…−0.003, −18…−56 mas, 0.1–0.3 %), confirming topocentric astrometric. Pairwise rift after the annual-aberration reduction: DE−INPOP 33.5 mas, DE−EPM 13.9 mas, INPOP−EPM 44.8 mas.
- **Inner-planet contrast.** The two sources read carry no inner-planet comparison; the contrast is not carried by these data. Named absent.
- **The moon-model level.** ura111 and ura184_part-3 agree at 5–25 mas (grind-pro); the initial suspicion about ura111 was measured false. The rift is not the moon model.

## References

1. Folkner, W. M., et al. 2014, "The Planetary and Lunar Ephemerides DE430 and DE431", IPN Progress Report 42-196 — **[ADS: 2014IPNPR.196C...1F]**
2. Park, R. S., et al. 2021, "The JPL Planetary and Lunar Ephemerides DE440 and DE441", AJ 161, 105 — **[ADS: 2021AJ....161..105P]**
3. Fienga, A., et al. 2011, "The INPOP10a planetary ephemeris and its applications in fundamental physics", Celest Mech Dyn Astr 111, 363 — **[ADS: 2011CeMDA.111..363F]**
4. Pitjeva, E. V., & Pitjev, N. P. 2014, "Development of planetary ephemerides EPM and their applications", Celest Mech Dyn Astr 119, 237 — **[ADS: 2014CeMDA.119..237P]**
