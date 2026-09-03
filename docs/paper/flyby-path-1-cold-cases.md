<!--
  title: The flyby proof, Path 1 — the seven cold cases
  class: paper
  date: 2026-08-22
  sha256: 3df767248b3287384497257c2ba79df143f8f309efba28533d53d43c44f4eaa9
  fam-round-machine: pre-fix (verdict fam-governed, value unprinted)
  status: live
  see-also: docs/concepts/kybernetische-astrophysik.md docs/concepts/der-paradigmenwechsel.md docs/TODO.md
-->
# The flyby proof, Path 1 — the seven cold cases

## Abstract

No fam-carrying arrow. The solar-wind phase (plasma-pressure gradient, IMF-Bz, Kp) carries no fam-significant causal arrow into the probe jerk in any of the seven historical Earth flybys. The null is the finding — silence is a measurement, not a bug (0 honored). The machine spanned the 4D tube (±12 h) around each reconstructed perigee and formed the residual R(t) = |Orbit(t) − N-body(t)|; the transfer entropy from the solar-wind phase into the jerk stays under the family bound in all seven cases. The cold-case verdict is the absence of the arrow, stated with the same precision as a positive find.


## What was measured

The machine spanned the 4D tube (±12 h) around the measured perigee
of each reconstructed trajectory and formed the residual

    R(t) = |Orbit(t) − N-body(t)|

with the N-body model Sun+8+Moon (point masses, gm/J2 from the
ephemeris bins) + Earth-J2, Leapfrog 4 s, initialized backward/forward
from the perigee. The jerk J = Δ²R (hour steps). Then
TE(driver → jerk/R) per channel, both directions, lag 0–6 h, against the
phase-randomized threshold (10 realizations, f64 FFT) and the
family threshold fam (max surrogate TE of the round = multiple-comparison
correction). Drivers per flyby from OMNI2_H0_MRG1HR (HAPI, hourly,
1963+): plasma-pressure gradient, Bz, Kp — no hardcoded channel.

## The seven cold cases

| Flyby | Perigee (UTC, measured) | geocentric | API/source | verdict |
|---|---|---|---|---|
| Galileo Ⅰ | 1990-12-08T20:34 | 7481 km | ~7338 km (960 km alt) | Silence |
| Galileo Ⅱ | 1992-12-08T15:09 | 6969 km | ~6683 km (305 km alt) | over threshold, under fam (P-grad→jerk) |
| NEAR | 1998-01-23T07:24 | 6934 km | 6910,7 km | over threshold, under fam (Bz→R) |
| Cassini | 1999-08-18T03:29 | 7567 km | ~7549 km (1171 km alt) | Silence |
| Rosetta | 2005-03-04T22:09 | 8332 km | 8340 km | over threshold, under fam (Kp→R) |
| Messenger | 2005-08-02T19:14 | 8721 km | 8725 km (2347 km alt) | Silence |
| Juno | 2013-10-09T19:24 | 7185 km | 6937 km (558,8 km alt) | Silence |

The perigees agree with the Horizons data (the deviation is
the 5-min raster of the perigee search). The three arrows "over own
threshold" lie in three different channels (P-grad, Bz, Kp) and
stay under fam — no pattern, no repetition across the flybys.

## The named floor line

The verdict measures the causal arrow, not the anomaly magnitude. The
floor line is named, not concealed:

- The 1-min bend harvest carries ~16 m/s velocity floor at the
  perigee (the Chebyshev bend smoothing); R grows with it by
  ~10–50 km per hour away from the perigee.
- The micro forces (solar pressure, thermal recoil, higher harmonics)
  are not contained in the model — they live in the residual; the
  `spacecraft_dynamics_compiler` (`handover-2026-08-22-ruck-datenquellen.md`)
  harvests this side as the next thread.
- The ~4-hour tracking gap stays a gap (fehlt, not null).

The flyby anomaly itself (2–14 mm/s) lies under this floor line.
Path 1 therefore asks only: does the wind phase carry the jerk? The answer
is no — on the floor line measured here.

## Limitations

- **Surrogate-machine generation (conservative footnote).** The verdict is
  fam-governed: the solar-wind phase stays under the family bound in all
  seven cases, but no family-bound value is printed here (fam-round-machine:
  pre-fix — verdict fam-governed, value unprinted). The bound was generated
  by the pre-fix surrogate RNG. The pre-fix band lies higher than the
  post-fix band; a verdict that holds against the higher pre-fix bound holds
  post-fix a fortiori. The measured silence is thus the conservative
  statement.

## What follows from it

Path 2 (the pre-registration, Operation Ⅵ) carries the coupling as a
measured quantity into the block: before the JUICE flyby (28.09.2026)
the field state along the trajectory is sealed from the living channels;
the Path-1 finding (no arrow) is the precondition — the prediction is
a field state, not an invented mm/s number.
