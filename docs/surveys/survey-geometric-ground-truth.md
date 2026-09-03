<!--
  title: Geometric ground truth — the spatial chain against JPL Horizons
  class: survey
  date: 2026-08-23
  sha256: 3a46b3a38ce8f4719f3ce5af96f537e995b9a162f1afe71a7bbbaee92186cfb8
  status: live
  see-also: docs/paper/solar-cycle-dynamo.md docs/concepts/ein-blatt-axiom.md
-->
# Geometric ground truth — the spatial chain against JPL Horizons

The question a reviewer asks of any spatial claim (flyby, Planet 9, a probe
position in 1999): *"Where is the body, and how do you know your ICRS/TDB
transformation puts it there?"* This survey answers it with a measurement,
not with theory.

## The question

The compilers build Chebyshev granules from SPK into `ephemeris_<body>.bin`
(barycentric ICRS, meters, TDB) — the same granules that place every body in
the block. `ephemeris_horizons_check` evaluates that chain and holds it
against an **independent** source: the JPL Horizons API, which evaluates the
same bodies from its own ephemeris and returns ICRF vectors centered at the
solar-system barycenter. The 3D distance between the two answers is the
residual of the whole spatial transformation chain — SPK reading, Chebyshev
coefficient extraction, granule windowing, τ evaluation, frame (ICRS↔ICRF),
epoch (TDB), and units (meters).

## The measurement

1096 epochs, JD 2451545.0 → 2462502.5 (2000–2030), 10-day step, per planet,
`|Chebyshev − Horizons|`:

| body | residual (m, median) | angular residual (arcsec, mean / max) |
|---|---|---|
| Mercury | 116 759 | 0.416 / 0.538 |
| Venus | 116 634 | 0.222 / 0.228 |
| Earth | 116 626 | 0.161 / 0.166 |
| Mars | 116 715 | 0.106 / 0.118 |
| Jupiter | 165 184 | 0.046 / 0.092 |
| Saturn | 307 607 | 0.043 / 0.063 |
| Uranus | 761 415 | 0.054 / 0.065 |
| Neptune | 944 905 | 0.046 / 0.082 |

Two facts stand out.

**First, the chain is correct.** The residual is smooth over thirty years,
with no granule-boundary jumps (the 32-day granule grid leaves no trace), no
time-dependent drift, no order-of-magnitude error. A bug in the SPK read, the
coefficient loading, the τ evaluation or the frame would surface as jumps at
granule boundaries or as ~1e6–1e8 m errors — absent everywhere. The Chebyshev
decode and evaluation reproduce the ephemeris faithfully.

**Second, the residual is a near-constant translation, not a rotation.** The
angular residual scales exactly as ~116 km / heliocentric distance for the
inner planets (Mercury 0.42″, Venus 0.22″, Earth 0.16″, Mars 0.11″ — a 1/r
law to three digits). A frame rotation would give a constant *angle*; a
granule error would give a time-dependent *wobble*. A fixed ~116 km offset of
every body is the signature of a **solar-system-barycenter realization
difference**: the system's SSB origin (from its DE selection, with its own
asteroid treatment) sits ~116 km from Horizons' DE441 SSB. The outer planets
carry the same barycenter offset at their own distance, plus a slowly growing
component from the differing perturbation model.

## What this proves, and what it does not

Proved (after the decisive recompile below): the spatial transformation chain
is validated to **sub-meter** against an independent source — the arithmetic
and the chain (SPK → Chebyshev → block position) are faithful, and a gross
spatial error would have shown itself. This is the *geometric* counterpart of
the statistical ground truth (Schreiber-2000 Hénon maps,
`broken-null-control.md`), which validates the transfer-entropy estimator.

The first measurement alone (CDN bins) showed only sub-arcsecond, with a
~116 km global offset; that offset was traced to a lighter-DE compile, not
the chain — see below.

## The decisive recompile

To separate the chain from the model, the planets were recompiled directly
from the full `de441.bsp` and re-checked against Horizons:

| body | CDN bin residual (m) | full-de441 recompile residual (m) |
|---|---|---|
| Mercury | 116 759 | **0.84** |
| Mars | 116 715 | **0.16** |
| Jupiter | 165 184 | 136 043 |
| Saturn | 307 607 | 289 333 |
| Neptune | 944 905 | 194 148 |

The inner planets collapse to **sub-meter** (Mars 0.16 m, Mercury 0.84 m) —
the ~116 km offset was a lighter-DE compile artifact in the CDN bins, not the
chain. The outer planets do *not* collapse, for a named reason: Horizons does
not serve them from DE441 at all. Its response header reads `jup365_merged`,
`sat441l`, `nep098_merged` — the outer planets are served from refined
barycenter-merged solutions, not the raw DE441 planetary barycenter. The
system's de441-based outer-planet position is the DE441 barycenter, which
differs from those merged solutions by ~100–300 km. That is Horizons' model
choice, not a chain error — the inner-planet sub-meter agreement is the
decisive same-ephemeris proof.

## The register duty

- `open` — the CDN planetary bins carry a ~116 km SSB offset because the CI
  compiled them from a lighter DE selection; recompiling the planets from the
  full `de441.bsp` and re-uploading collapses the inner planets to sub-meter.
  The check exists and reports the number.
- Named, not a duty — the outer planets (Jupiter/Saturn/Uranus/Neptune) differ
  from Horizons by ~0.01–0.04″ because Horizons serves them from merged
  barycenter solutions (`jup365`, `sat441`, `nep098`, …), not DE441; the
  system carries the DE441 barycenter, the documented standard.

## The honest sentence for a reviewer

*"Our ICRS/TDB chain is geometrically validated against JPL Horizons to
sub-meter on the inner planets (Mars 0.16 m over thirty years) when both read
DE441; the outer-planet difference is Horizons' merged-barycenter solution,
not our chain."*
