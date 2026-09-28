<!--
  title: Flyby-Path-2 — the pre-registered JUICE Earth-flyby prediction (revised)
  class: paper
  date: 2026-09-27
  version: 2
  sha256: e25d8faf41f56b7e7aa270ac93e1ce71a06f4e5e8906f2c1e246f7bb3b772908
  status: live
  see-also: docs/paper/flyby-path-2-preregistration.md docs/paper/flyby-path-2-falsification-metric-addendum.md docs/auftrag/auftrag-flyby2-kette.md
-->
# Flyby-Path-2 — the pre-registered JUICE Earth-flyby prediction (revised)

This Blatt replaces the sealed falsification metric of
`flyby-path-2-falsification-metric-addendum.md` as the scoring instance, before the
JUICE Earth perigee (2026-09-28). The sealed pre-registration
(`flyby-path-2-preregistration.md`, seal 2026-08-22) and its trajectory hashes stay
untouched; git carries the superseded state. The old metric was measured as
tautological by three independent foreign voices (GLM-5.2/5.3 + Claude Sonnet 5,
recorded `docs/handover/archiv/handover-2026-09-24-mycelium-folge152.md:78`):
the pre-registered field state was computed from the same upstream channels the
in-situ measurement is compared against, so it could not fail. This revision
substitutes a metric quarried outside the prediction.

## Abstract

The question: does the post-flyby reconstructed JUICE orbit agree with the sealed
trajectory, beyond the model-noise floor? The method: compare the post-flyby
orbit against the sealed arc, with a threshold that does not depend on the flyby
outcome — the rift between two independent JPL ephemeris editions (DE441 vs.
DE442) at the perigee, plus the published 1-σ position uncertainty. The
prediction: **the sealed trajectory is not refuted** — falsified iff the measured
deviation exceeds the edition rift by more than 3σ. Result: `pending — scored
after the perigee 2026-09-28`.

## Data

- **JUICE ephemeris** — `ephemeris_juice.bin`, CDN
  `https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov-ephemeris/ephemeris_juice.bin`
  (`phi/sources.φ:3500`). CDN sha256
  `eee376effcb4def668a61d47ab7ea2e6f7b0b7cc3f884ea6634997349d4389b5` (538 696 B,
  measured 2026-09-27 via `archive_search --sniff`). Sealed sha256
  `aeb3c82ff3de672116ff7f8c28592d97ea05c5e78b8f652521d2cf3cae57488a`. The riss is
  carried, never averaged (preregistration §"Addendum").
- **RTSW 1-minute** — SWPC mirror, `phi/sources.φ:165-176` (mag bt/bz, wind
  speed/density/temperature; multi-source with per-reading `source`/`active`).
- **Perigee** — **2026-09-28 11:45:12 UTC ± 10 s**, geocentric distance
  **≈ 0.00010039 AU = 15 018 km** (range-rate sign change at 11:45:11.6, minimum
  delta vertex at 11:45:11.6). Source: JPL Horizons `COMMAND='-28'`,
  `CENTER='500@399'`, `QUANTITIES='20'`; query timestamp **2026-09-27 09:48:45 UTC**
  (Horizons header stamp `Sun Sep 27 02:48:45 2026 Pasadena`). **JUICE is `-28`,
  not `-61`** (measured: `-61` resolves to Juno).
- **Window** — perigee ± 24 h for the field chain; ± 21 d for the sealed
  trajectory window (the sealed arc).
- **DSN status 2026-09-28** — `pending`: `eyes.nasa.gov/apps/dsn-now/` renders no
  readable tracking table (JS/canvas), and the live feed
  `eyes.nasa.gov/dsn/data/dsn.json` (2026-09-27 09:48:57 UTC) carries no JUICE
  entry. Tracked/not-tracked is not measurable from these sources.

## Methods

- **Falsification metric (Council 2026-09-27, decision A).** Threshold quarried
  outside the prediction: the rift between two independent JPL ephemeris editions
  and the published reconstruction uncertainty.
- **Predicate:** with
  - Δ = max over the sealed ±21 d window of |x_recon(t) − x_sealed(t)| [km, ICRS, geocentric],
  - δ = max over the same window of |x_DE441(t) − x_DE442(t)| [km] (both editions
    evaluated at the same locations; live data, not a hardcoded number),
  - σ_recon = the published 1-σ position uncertainty of the post-flyby solution [km],
  the prediction stands iff **Δ ≤ δ + 3·σ_recon**; **falsified iff Δ > δ + 3·σ_recon**.
- **The riss:** Δ is measured against both the sealed hash `aeb3c82f…` and the
  renewed CDN hash `eee376ef…`; if the two verdicts diverge, both witness lines
  are carried, never averaged.
- **Field-state chain** (`flyby_path2_fill`, `tools/measure/src/bin/flyby_path2_fill.rs`)
  stays as the descriptive / verification channel — the perigee tube filled from
  RTSW/Kp/Swarm, transit-time corrected. It is no longer the falsification gate
  (the superseded fam metric).

## Failure

If **Δ > δ + 3·σ_recon**, the sealed trajectory is refuted. Agreement is silence —
a full finding. δ does not depend on the flyby; the rule and δ are fixed before
2026-09-28, so the verdict is not post-hoc.

## Estimator validation

- **DE441 vs. DE442 rift (δ)** — **measured** (first carried 2026-09-27,
  reproduced 2026-09-28): **δ = 0,16847323696971178 km** (max |x_DE441(t) −
  x_DE442(t)| over the sealed-arc-clamped perigee window, 997 hourly samples),
  evaluated live by `flyby_ephemeris_gate` (`tools/measure`; built `d310d5888`);
  both NAIF kernels registered (`phi/sources.φ:3448`/`:3469`). The value is
  evaluated live by the gate, never carried as a hardcoded constant.
- **σ_recon** — `pending`: arrives with the published post-flyby solution and its
  covariance; absent σ_recon → the verdict stays `pending`, never declared.
- **Named limits:** the field-state σ/fam tautology is not repaired by this rule —
  it is replaced; the field chain remains descriptive.

## Results

`pending — scoring after the perigee 2026-09-28` (post-flyby ephemeris + σ_recon).

## Discussion

Expected physics: an anomaly in the Earth-flyby energy transfer of the Anderson
class (Anderson et al. 2008; Turyshev & Toth 2011; Acedo & Bel 2016); the
trajectory test can only bound a position deviation, not a mm/s Doppler residual.

## Limitations

- `src/archivar/doppler.rs` is **not built**; `flyby2_probe.rs` is **not built**;
  no DSN/ESTRACK residual route is registered. The Doppler channel is a named
  `pending` verification channel, never part of the rule until built.
- DSN tracking status for 2026-09-28 is `pending` (JS page, see Data).
- δ and σ_recon are `pending` (see Estimator validation); the rule is complete but
  its two numbers arrive from the named measurements.
- The edition rift is a model-noise floor, not a reconstruction bound — hence
  σ_recon in the predicate.
- The ±21 d window bounds sensitivity: a deviation signature slower than the
  window is outside the rule.

## Preregistration seal

- **Sealed:** 2026-09-27 (body sha256 in this header).
- **Operator seal:** operator-bound, not yet set — the operator's hand.
- **Perigee:** 2026-09-28 11:45:12 UTC ± 10 s.
- **Scoring opened:** after the perigee (post-flyby ephemeris + published σ_recon).
- **Supersedes:** the falsification metric of
  `flyby-path-2-falsification-metric-addendum.md` (as the scoring instance only;
  the seal and the trajectory hashes stand untouched).
