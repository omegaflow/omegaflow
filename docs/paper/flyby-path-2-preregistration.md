<!--
  title: The flyby proof, Path 2 — the pre-registration (Operation Ⅵ)
  class: paper
  date: 2026-08-22
  sha256: 6f24f98a01decc82025652ec0302afd75a06e53bc14743fec79d5ca0ef44b2d0
  status: live
  see-also: docs/paper/flyby-path-1-cold-cases.md docs/concepts/der-paradigmenwechsel.md docs/TODO.md
-->
# The flyby proof, Path 2 — the pre-registration (Operation Ⅵ)

## Abstract

This Blatt is the seal, registered 2026-08-22 — before the JUICE Earth flyby (28./29 September 2026). It is not backdated and not changed after the flyby; what changes after the seal is named. The trajectory stands as ephemeris_juice.bin (sha256 aeb3c82ff3de672116ff7f8c28592d97ea05c5e78b8f652521d2cf3cae57488a); the second window, Europa Clipper (03.12.2026), carries sha256 dae553fb3ef787b4faba8b844e224193a87aba718a7d20fe021e2d097703682c. Only the field state is pre-registered — no mm/s number (0 honored; a number from a missing arrow would be fabricated). The field state along the perigee — plasma-pressure gradient, IMF-Bz, Kp, Swarm — is a function of the measured plasma with transit-time correction; every cell not yet measurable stays pending, never 0.0.


## The seal

- **Trajectory:** the JUICE orbit stands in the block as
  `ephemeris_juice.bin` (Horizons −28, flyby window ±21 d around
  28.09.2026, two-stage 5-min/1-min, perigee in the tube).
  `sha256 aeb3c82ff3de672116ff7f8c28592d97ea05c5e78b8f652521d2cf3cae57488a`.
  The second window, Europa Clipper (03.12.2026), carries
  `sha256 dae553fb3ef787b4faba8b844e224193a87aba718a7d20fe021e2d097703682c`.
  If the flattener renews the orbit before the flight, the new version
  is named — the seal binds to the state of this Blatt.
- **Path-1 finding:** no fam-carrying arrow from the solar-wind phase into
  the jerk (seven cold cases). The prediction form follows from it:
  **only the field state** is pre-registered — no mm/s number (0
  honored; a number from a missing arrow would be fabricated).
- **The prediction:** the field state along the JUICE orbit at the perigee —
  plasma-pressure gradient, IMF-Bz, Kp, Swarm magnetic field — computed
  from the living channels with their transit time (RTSW at L1 leads by
  the L1 transit time; Kp 3-h; Swarm at the site). The field state is a
  function of the measured plasma, not a postdicted value.

## What arrives and is sealed by 28.09.

The field state at the perigee tube fills from the living
channels, as soon as the measurement (transit-time corrected) is present.
Every cell that is not yet measurable stays `pending` — never 0,0. The
seal stands with the method and the trajectory hash; the field values
join as soon as they exist.

## What arrives after the flyby

The Doppler residuals. The verdict: the agreement of the measured
field state with the pre-registered one — the first pre-registered
astrophysical experiment whose prediction is a field state.
