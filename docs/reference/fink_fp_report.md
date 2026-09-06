# Nadel V over anonymous Fink-LSST forced photometry — measured report

date 2026-09-05. Probe: `tools/measure/src/bin/lsst_anomaly_probe.rs`
(`--fink-fp`, `--fink-fp-scan`). `cargo check -p omegaflow-measure` zero warnings.
All 18 bin unit tests green, incl. the two FP-parser tests on the real 6-row
response fixture. No commit (operator commits).

## FP endpoint wiring (measured, anonymous)

`POST https://api.lsst.fink-portal.org/api/v1/fp`, body `{"diaObjectId":"…"}`,
no token, UA `omegaflow-nadel-v-lsst-scan/1.0`. HTTP 200. Each row:
`r:band, r:midpointMjdTai, r:scienceFlux, r:psfFlux, r:ra, r:dec, r:visit,
r:diaForcedSourceId, r:diaObjectId, r:timeProcessedMjdTai …`.

The FP path reuses the cone/lightcurve plumbing: rows fold mjd(TAI) → unix → TDB
on the one axis (naif0012.tls), then a Rømer light-travel term from the Cerro
Pachón geodetic constant when the sun/earth ephemeris is cached (measured here:
every row got the station correction). scienceFlux is the light-curve value —
finite and > 0 enters, a negative/absent scienceFlux stays absent (never 0.0).
psfFlux is counted in its own census column, never substituted (measured on
313998569858662581: psfFlux < 0 on 1032 of 1033 rows while scienceFlux > 0 on
1032 — a psfFlux curve would be empty). The dense scienceFlux rows assemble the
LSS1 asset per band (u/g/r/i/z/y at their LSST central frequencies) and run the
Nadel-V achromatic + non-periodic dip cut.

## Real per-band epoch counts — FP vs the sources path (measured)

| diaObjectId | cone nDiaSources / class / SIMBAD | FP rows (total) | FP per band (scienceFlux>0) | sources path per band |
|---|---|---|---|---|
| 313998569858662581 | 1206 / 22 / Fail | 1033 (1 absent) | g197 r188 i385 u47 y20 z196 | g205 r205 i373 u43 y12 z207 |
| 170028511006818402 | 227 / 11 / GiP | 215 (0 absent) | g54 r50 i56 y1 z54 | g68 r44 i39 z42 |
| 313853517569720388 | 567 / 11 / Fail | 962 (3 absent) | g182 r183 i340 u37 y15 z205 | g180 r142 i188 u6 |

The three objects are real members of the saved anonymous cone
(`fink_cone_148_84.json`, ra 148.84 dec 2.55). FP row counts and the non-detection
absent-count reproduce the committed context (c133a95) for 313998569858662581
exactly. Two sparse cone objects (313994140536275083, 314003015419822293, both
nDiaSources 1, class −1) return HTTP 200 with `[]` — no FP table row. Anonymous FP
does NOT reach the sparse cone tail; it covers only the objects the Fink-LSST FP
table carries (measured here: the alert-dense members). The anonymous FP gain is
therefore a densification/homogenisation of the multi-band surface for FP-covered
objects (per-visit all-band cadence, non-detections counted absent), not a rescue
of the single-detection cone objects.

## Achromatic scan verdict over dense FP

Per-object verdict on the TDB fold axis, per the committed gate (deepest-dip per
band on the full residual series, ratio gate, FAP aperiodicity gate on the
coincident |Δt| ≤ 1800 s join, N_MIN 24, N_COINC_MIN 12):

| diaObjectId | pair | verdict | deepest dips (each band, full series) | FAP |
|---|---|---|---|---|
| 313998569858662581 | i/g | 1 pre-exclusion candidate | i −0.867 (−9.7σ) t≈7.04 d; g −0.997 (−9.7σ) t≈1.02 d; |Δt| 6.0 d; ratio 0.87 | 8.7e-2 |
| 170028511006818402 | i/g | 0 (chromatic) | i −0.216 (−2.0σ); g −0.672 (−5.4σ); ratio 0.32 | 2.2e-3 (reads periodic) |
| 313853517569720388 | i/z | 1 pre-exclusion candidate | i −0.983 (−13.6σ) t≈9.0 d; z −0.564 (−3.4σ) t≈5.9 d; |Δt| 3.1 d; ratio 1.74 | 2.4e-1 |

All three objects are cone-classified natural dimmers (class 11 / 22; one SIMBAD
"GiP"), so both pre-exclusion candidates are excluded upstream by the natural-class
crossmatch — no unexcluded anomaly candidate remains.

## What must be contradicted

1. Anonymous FP is not universal: two nDiaSources=1 cone objects returned HTTP 200
   with an empty array. The breakthrough reaches only the FP-covered (here:
   alert-dense) objects. The anonymous cone's sparse single-detection tail stays
   unreachable by FP.
2. Both pre-exclusion candidates rest on deepest dips that are NOT time-coincident
   (6.0 d and 3.1 d apart; coincidence window 1800 s). The committed gate measures
   the deepest dip of each band's full series and takes their depth ratio, so two
   independent deep excursions of similar depth at different epochs can pass the
   achromatic test; coincidence constrains only the FAP join. This is a property of
   the committed scan_lss1 (HEAD c3dd482 has the same semantics), now made visible
   by the dense FP surface. Neither candidate is one achromatic event by epoch.
3. FP scienceFlux vs sources scienceFlux totals are comparable for the three
   FP-covered objects (1033 vs 1045, 215 vs 193, 962 vs 516); the FP value is the
   homogeneous per-visit all-band coverage, not a raw row-count increase.

## Outputs in /tmp/opencode

Raw FP samples: `fink_fp_<id>.json` (live HTTP, saved). LSS1 assets:
`lsst_lightcurves_fp_<id>.bin` (313998569858662581, 170028511006818402,
313853517569720388). Empty-table samples: `fink_fp_313994140536275083.json` =
`[]`, `fink_fp_314003015419822293.json` = `[]`. Test fixture (must be committed
with the bin): `tools/measure/src/bin/lsst_fp_313998569858662581_6rows.json`
(referenced by `include_str!` in the FP-parser unit test).
