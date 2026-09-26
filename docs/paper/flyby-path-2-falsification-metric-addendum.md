<!--
  title: The flyby proof, Path 2 — the falsification metric (addendum)
  class: paper
  date: 2026-08-28
  sha256: b54d52984feddea16f6b413ef1806109b05682b8eedf24a000d07b6e4ff87655
  status: live
  see-also: docs/paper/flyby-path-2-preregistration.md docs/paper/flyby-path-1-cold-cases.md docs/concepts/der-paradigmenwechsel.md docs/concepts/blatt-papier-resultat.md
-->
# The flyby proof, Path 2 — the falsification metric (addendum)

## Abstract

This addendum seals the falsification metric for the Path-2 pre-registration, registered 2026-08-28 — before the JUICE Earth flyby (28./29 September 2026). It does not change the sealed pre-registration; it only names what "agreement of the measured field state" means, so the verdict is not post-hoc. The pre-registered field state (plasma-pressure gradient, IMF-Bz, Kp, Swarm, transit-time corrected) is compared against the in-situ JUICE field measurement at the perigee tube, per channel, by the normalized channel residual: RMS over the tube of (measured − pre-registered)/σ, σ being the channel's own measurement dispersion. The threshold is fam, the family bound of phase-randomized surrogates, with multiple-comparison correction across all channels. Agreement = no channel above fam (silence is a full finding); over fam = falsification. No hardcoded threshold; pending and 0 honored stay untouched.


## Scope

This Blatt is an addendum to the Path-2 pre-registration, sealed 2026-08-28 — before the JUICE Earth flyby. It does not modify the sealed pre-registration; it only names the missing falsification metric, so the verdict after the flyby is not post-hoc. The original pre-registration's sealed prediction form (header sha256 6f24f98a01decc82025652ec0302afd75a06e53bc14743fec79d5ca0ef44b2d0 at sealing) stays untouched; its body advanced by a named pre-flyby readiness note (2026-09-26), so its header sha256 is now 502e06c3d9b0b66652552973e612d514dc82fac293d0453beae7e27933be2f01.

## The prediction chain (Trishuli-muster)

This addendum also carries the prediction chain that links the pre-registration
to the in-situ measurement, in the timestamped form of the Trishuli Blatt
(`docs/paper/sturzflut-tibet-pfeil.md` §3.2): a chain of anchors with
their own time, not a narrative. For Path 2 the chain is the transit-time
corrected link from each upstream living channel to the JUICE perigee tube,
per channel — the object the σ-Metric (§"The metric") then weighs. Every cell
that is not yet measurable stays `pending`, never 0.0.

| anchor (per channel) | time | state |
|---|---|---|
| plasma-pressure gradient | upstream (RTSW/L1) reading | `pending` (fills when measured, transit-time corrected) |
| IMF-Bz | upstream (RTSW/L1) reading | `pending` (fills when measured, transit-time corrected) |
| Kp | 3-h cadence, the interval whose transit reaches the tube | `pending` (fills when measured) |
| Swarm magnetic field | at the site | `pending` (fills when measured) |
| JUICE in-situ field | at the perigee tube | `pending` (fills after the flyby, 28./29.09.) |

The chain runs in one direction only: the upstream channel state at its own
earlier time (carried by its own transit time to the tube) is the pre-registered
prediction of the field; the JUICE in-situ field at the tube is the thing itself
measured on an independent path. A = A: the prediction of the thing is tested
against the thing. No field value is filled before its measurement exists; an
unfilled link is `pending`, never 0.0. The σ-Metric is the measure of the chain's
agreement — agreement = no channel residual above fam (silence is a full finding).

## Chain readiness (measured 2026-09-26)

The chain stands ready; no cell value exists before its measurement — the perigee
tube lies in the future (28./29.09.2026), every cell stays `pending` until filled
(0 honored). Readiness per channel, re-measured 2026-09-26 (reachability ladder,
stage 1 direct):

| channel | route (`phi/sources.φ`) | reachability | cadence | transit | fills |
|---|---|---|---|---|---|
| plasma-pressure gradient | RTSW wind `:164-169` | HTTP 200, stage 1 | 1 min (ttl 60) | L1 lead `d/v_sw` (the same feed's measured speed); light time `d/c` = 5.0 s (`d` = 1.5e9 m, `c` = 2.99792458e8 m/s) | at the perigee, minutes after |
| IMF-Bz | RTSW mag `:158-162` | HTTP 200, stage 1 | 1 min | same | at the perigee, minutes after |
| Kp | NOAA `noaa-planetary-k-index.json` — route removed from the register | HTTP 200, stage 1 (the URL serves); not registered | 3-h (interval stamps measured in the file) | none (at Earth); 3-h alignment to the perigee interval | at the interval, ≤ 3 h |
| Swarm magnetic field | EFIA-LP / FACATMS / MAGA-LR `:5922-5941` | HTTP 200, HAPI | 1-h window; the yesterday window served complete → latency ≤ 1 d | none (at the site) | ≤ 1 d after |
| JUICE in-situ field | after the flyby | — | — | — | after 28./29.09. |

Route line drift, measured 2026-09-26 against the working tree: RTSW mag/wind
(`:158-169`) still sit at the cited lines; Swarm moved to `:5922-5941` (table above;
formerly `:6059-6078`); OMNI2 moved to `:605` (formerly `:918-928`). The Kp route
(`noaa-planetary-k-index.json`) was removed from `phi/sources.φ` in commit
`61e272ab0` (mycelium folge156 — the six index lines dropped as "misclassified").
The NOAA URL still serves HTTP 200 (stage 1), but the Kp cell cannot fill from a
registered route until the route is re-registered. This is a riss — the sealed
chain names a Kp channel the register no longer carries; it is carried here, not
smoothed.

OMNI2 (`:605`): HTTP 200, 1-h merged; the 13.–19.09. window served populated
on 20.09., but a 19.09. window re-measured 2026-09-26 returns an empty HAPI
payload (header only) → latency is multi-day, ~6 d (measured 2026-09-23) — it
cannot carry the pre-flyby prediction; it is the verification channel. Its
time-shift convention is verified before any OMNI2 cell fills.

The RTSW feed is multi-source and carries a per-reading `source` field (measured:
newest reading `source "IMAP", active:false`); every fill records the per-reading
`source` and `active` flag. The seal reads "RTSW at L1", so any L1 monitor
qualifies; the per-reading log is the duty.

DSCOVR (measured 2026-09-20): the dedicated keyless DSCOVR JSON routes are retired
— `products/solar-wind/plasma-7-day.json`, `products/solar-wind/mag-7-day.json`,
`json/dscovr/dscovr_mag_1m.json`, `json/dscovr/dscovr_fc_1m.json` all HTTP 404
(stage 1 and 2; the wayback register holds only a 2016 snapshot). The L1 real-time
solar wind lives in the registered RTSW feed (`:158-169`). No new `sources.φ` line:
a DSCOVR entry would duplicate the same source family; a DSCOVR-only isolation
(`where source DSCOVR`) is `pending` a parser check and is not required by the seal.

## The operator seal (pending)

The seal line is set by the operator, over the verified prediction chain, before
the 28.09.2026 deadline. It is not set by the machine and not set before the
chain is measured and checked. Until the operator's word falls, the seal is:

`Seal line: Sealed by Johannes Tyroller (2026-09-03): Read and approved. This is the prediction chain that will be tested at the 28./29.09. flyby.`

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
- The trajectory hashes stay untouched; the pre-registration header sha advanced only by the named pre-flyby readiness note (2026-09-26, §"Scope"), never by the metric.
