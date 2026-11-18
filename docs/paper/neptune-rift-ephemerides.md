<!--
  title: The Neptune rift — ephemeris divergence at the planet centre
  class: paper
  date: 2026-09-16
  sha256: d991b102bac4711a0bd73463fd1ab60fcc2dda2c8e06996efdf24b366a8b82a8
  status: live
  see-also: docs/paper/uranus-rift-ephemerides.md
-->
# The Neptune rift — ephemeris divergence at the planet centre

## Abstract

Three independent planetary ephemeris families — JPL DE, IMCCE INPOP, and IA RAS EPM — are read in one frame at the centre of Neptune over 1970–2030 (731 epochs, 30-day step). The same 899−8 centre offset is added to all three barycentres, so it cancels in every pairwise difference: the rift is the inter-house divergence itself. The pairwise divergences are de441−inpop19a 953 km, de441−epm2021 2704 km, and inpop19a−epm2021 3208 km in the mean, with maxima 2515/7476/9668 km. The divergence grows toward the present: the 1980s–1990s means sit at 0.3–1.5e3 km, the 2020s at 2.1e3/5.8e3/7.6e3 km. The measurement is model-against-model — no observations adjudicate — and the rift at Neptune's centre is larger than the Uranus rift (0.36–1.57e6 m = 360–1570 km).

## The measurement

Three ephemeris families are read in one frame at the centre of Neptune:

- **DE (NASA JPL, ssd.jpl.nasa.gov).** The CDN asset `ephemeris_neptune_c.bin` carries the DE441 barycentre composed with the `nep097xl-899.bsp` kernel (the 899−8 centre offset).
- **INPOP (IMCCE).** `ephemeris_inpop_neptune.bin` (ftp.imcce.fr).
- **EPM (IA RAS).** `ephemeris_epm_neptune.bin` (ftp.iaaras.ru).

The probe samples JD 2440587.5–2462502.5 (1970-01-01 to 2030-01-01) on a 30-day step — 731 epochs. The 899−8 offset from the DE kernel is added to all three barycentres, so it cancels exactly in every pairwise difference; the measured rift is the houses' own barycentre divergence at the planet centre.

The pairwise divergence at the planet centre: de441−inpop19a mean 953 km (max 2515 km), de441−epm2021 mean 2704 km (max 7476 km), inpop19a−epm2021 mean 3208 km (max 9668 km).

## The finding

The three-house divergence at Neptune's centre grows toward the present. The per-decade mean |Δ| in km:

| decade | de441−inpop19a | de441−epm2021 | inpop19a−epm2021 |
|---|---|---|---|
| 1970s | 855 | 1913 | 1889 |
| 1980s | 360 | 1347 | 1160 |
| 1990s | 304 | 1475 | 1527 |
| 2000s | 736 | 2204 | 2634 |
| 2010s | 1353 | 3465 | 4449 |
| 2020s | 2110 | 5818 | 7583 |

The 1980s–1990s minimum is the epoch best covered by modern observations; the 2020s carry the largest divergence, 2.1e3/5.8e3/7.6e3 km. The rift at Neptune's centre is larger than the Uranus rift (0.36–1.57e6 m = 360–1570 km). The measurement is model-against-model: no observations are read, so the rift is not adjudicated here — it is the inter-model uncertainty at the ice giant's centre, carried as a source of positional divergence.

## The form

- **The offset cancels.** The same `nep097xl-899.bsp` 899−8 centre offset is applied to all three barycentres; it is a common mode and drops out of every pairwise difference. A rift that moved with the offset would name the offset, not the house.
- **Model-against-model.** Unlike the Uranus rift, no satellite astrometry or topocentric reduction enters; the finding is the divergence of three published ephemeris solutions at the planet centre. It is not an observational contradiction.
- **The 2020s growth.** The divergence is smallest where the observational arc is densest (1980s–1990s) and largest in the 2020s; the shape is the houses' solution spread over the common window, not a drift of one house alone.
- **The self-test.** The probe carries a `--calibrate-inject` recovery self-test; the CI run did not exercise it — named absent.

## References

1. Folkner, W. M., et al. 2014, "The Planetary and Lunar Ephemerides DE430 and DE431", IPN Progress Report 42-196 — **[ADS: 2014IPNPR.196C...1F]**
2. Park, R. S., et al. 2021, "The JPL Planetary and Lunar Ephemerides DE440 and DE441", AJ 161, 105 — **[ADS: 2021AJ....161..105P]**
3. Fienga, A., et al. 2011, "The INPOP10a planetary ephemeris and its applications in fundamental physics", Celest Mech Dyn Astr 111, 363 — **[ADS: 2011CeMDA.111..363F]**
4. Pitjeva, E. V., & Pitjev, N. P. 2014, "Development of planetary ephemerides EPM and their applications", Celest Mech Dyn Astr 119, 237 — **[ADS: 2014CeMDA.119..237P]**
