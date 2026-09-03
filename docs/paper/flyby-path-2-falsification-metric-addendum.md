<!--
  title: The flyby proof, Path 2 — the falsification metric (addendum)
  class: paper
  date: 2026-08-28
  sha256: 9ad59892a85f208e835d485e6e2bfab9d90cc6c8c2d7a468731b47aa16d71960
  status: live
  see-also: docs/paper/flyby-path-2-preregistration.md docs/paper/flyby-path-1-cold-cases.md docs/concepts/der-paradigmenwechsel.md docs/concepts/blatt-papier-resultat.md
-->
# The flyby proof, Path 2 — the falsification metric (addendum)

## Abstract

This addendum seals the falsification metric for the Path-2 pre-registration, registered 2026-08-28 — before the JUICE Earth flyby (28./29 September 2026). It does not change the sealed pre-registration; it only names what "agreement of the measured field state" means, so the verdict is not post-hoc. The pre-registered field state (plasma-pressure gradient, IMF-Bz, Kp, Swarm, transit-time corrected) is compared against the in-situ JUICE field measurement at the perigee tube, per channel, by the normalized channel residual: RMS over the tube of (measured − pre-registered)/σ, σ being the channel's own measurement dispersion. The threshold is fam, the family bound of phase-randomized surrogates, with multiple-comparison correction across all channels. Agreement = no channel above fam (silence is a full finding); over fam = falsification. No hardcoded threshold; pending and 0 honored stay untouched.


## Scope

This Blatt is an addendum to the Path-2 pre-registration, sealed 2026-08-28 — before the JUICE Earth flyby. It does not modify the sealed pre-registration; it only names the missing falsification metric, so the verdict after the flyby is not post-hoc. The original pre-registration (header sha256 6f24f98a01decc82025652ec0302afd75a06e53bc14743fec79d5ca0ef44b2d0) stays untouched.

## The two trajectory seals (unchanged)

- `ephemeris_juice.bin` sha256 `aeb3c82ff3de672116ff7f8c28592d97ea05c5e78b8f652521d2cf3cae57488a` (JUICE, flyby window ±21 d around 28.09.2026).
- Europa Clipper (03.12.2026) sha256 `dae553fb3ef787b4faba8b844e224193a87aba718a7d20fe021e2d097703682c`.

## The metric

The verdict of Path 2 is "the agreement of the measured field state with the pre-registered one". This addendum fixes what "agreement" means, in three parts.

### Comparison basis (A = A)

The pre-registered field state is the field along the JUICE orbit at the perigee — plasma-pressure gradient, IMF-Bz, Kp, Swarm magnetic field — computed from the living channels with their transit time (RTSW at L1 leads by the L1 transit time; Kp 3-h; Swarm at the site). The measurement it is compared against is the in-situ field measurement of JUICE at the same perigee tube: the field state measured by the spacecraft, not a reconstruction from the same upstream channels. A = A: the prediction of the thing is tested against the thing itself, on two independent paths.

### Statistic: the normalized per-channel residual

For each channel c (plasma-pressure gradient, IMF-Bz, Kp, Swarm) over the perigee tube, the normalized channel residual is the RMS over the tube of (measured − pre-registered)/σ, where σ is the channel's own measurement dispersion over the tube. Every cell not yet measurable stays pending, never 0.0.

### Threshold: fam (phase-randomized surrogates)

The threshold is fam, the family bound of the phase-randomized surrogate null: the pre-registered series is phase-randomized to build the null ensemble, and each channel residual is tested against it, with multiple-comparison correction across all channels. Agreement = no channel carries a residual above fam (silence is a full finding). Over fam = the pre-registered field state does not agree with the measured one — falsification. No hardcoded threshold is used; fam derives from the live data.

## What stays unchanged

- Only the field state is pre-registered; no mm/s number (0 honored).
- pending cells stay pending, never 0.0; a source that failed is missing, not zero.
- The trajectory hashes and the original pre-registration header sha are untouched.
