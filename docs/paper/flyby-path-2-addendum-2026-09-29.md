<!--
  title: The flyby proof, Path 2 — the perigee fill-run (addendum)
  class: paper
  date: 2026-09-29
  sha256: 0a3cc4ef150e38b7787bb0a5b8891dcdfdfd542baa5cdf7b588486c756698f5b
  status: live
  see-also: docs/paper/flyby-path-2-preregistration.md docs/paper/flyby-path-2-preregistration-revised.md docs/paper/flyby-path-2-falsification-metric-addendum.md docs/auftrag/auftrag-flyby2-kette.md
-->
# The flyby proof, Path 2 — the perigee fill-run (addendum)

This addendum records the fill-run of the Path-2 chain at the JUICE Earth
perigee (2026-09-28). It is dated by its own run; every measurement carries its
measured UTC timestamp. The machine clock measured the run window
2026-09-28T22:41:53Z → 2026-09-28T23:01:08Z.

## Retention measured (RTSW 1-minute)

- `archive_search --verdict https://services.swpc.noaa.gov/json/rtsw/rtsw_mag_1m.json`
  (2026-09-28T22:42Z): stage 1 direct HTTP 206 (range probe) — reachable.
- `archive_search --sniff` (2026-09-28T22:45Z): HTTP 200, 1 649 655 bytes,
  magic bytes absent (JSON, no magic — the sniff reports the fact), sha256
  `bd1c1462…` of the rotating live file at that instant.
- Head sample (fetched 22:43Z, first record of the mirror): `{"time_tag":
  "2026-09-28T22:36:00", "active": false, "source": "ACE", "bt": 3.09, …}` —
  the file is newest-first, one object per minute per source.
- Oldest → newest record of the 22:43Z mirror: **2026-09-27T22:40:00 →
  2026-09-28T22:36:00** — a **23 h 56 min** window (mag 3911 / wind 3985
  records). The 24-h retention measured 2026-09-23 stands.
- **Remaining window:** the tube starts 12 h before the perigee
  (2026-09-27T22:58:50Z grid); its earliest upstream readings (t + lead, lead ≈
  1.0–1.26 h) were needed from ~09-27T21:43Z — already beyond the live feed at
  22:43Z, and captured by the earlier 15:02Z mirror (window back to
  ~09-27T15:00Z). The 22:43Z mirror froze everything the live feed still held.
  The hard deadline (first fill-run ≤ 24 h after the first perigee tube cell,
  auftrag §Retention) was 2026-09-28T22:58:50Z; the fill-run started
  22:41:53Z — **inside the window**.

## Raw data written (data/ is gitignored)

| file | bytes | sha256 |
|---|---|---|
| data/services.swpc.noaa.gov/rtsw_mag_1m-20260928T2243Z.json | 1 620 | eb87e2d6e4369092e45d1b579537750e6810e5017387c359e61ec71c18930ae5 |
| data/services.swpc.noaa.gov/rtsw_wind_1m-20260928T2243Z.json | 2 880 | ddc9b3f54462066180106b000cc1c86e8dad585c306fea86f344d4e6f6eb3861 |
| data/services.swpc.noaa.gov/ace_mag_1h-20260928T2255Z.json | 223 452 | b7aece1c798875a774d2369760cc0707ebcf4119aa492bcad2e1238203f2fbcd |
| data/services.swpc.noaa.gov/ace_swepam_1h-20260928T2255Z.json | 75 281 | 150b8c06ac8b482f9a637c52957ba6a94338e1118b18f147113f57fab2ac004e |
| data/kp.gfz.de/kp-20260926-20260929.json | 909 | d84890407f3ee4f1c5576501c1ae1fb63806280a5125d68efe97f857be2e4f48 |
| data/vires.services/swarm-magalr-20260927T2345-20260928T1450.csv | 2 136 276 | dd2ad4d6db10fe35cebe89d98502fd3fa9cdb1a18a2c62338b60e5c6ad819d9e |
| data/cdaweb.gsfc.nasa.gov/omni2-20260926-20260929.csv | 86 | 5d71c13876468ab4ec2bd19cdaf5617a4e91b89623859273c7e5816e984f2f8f |
| data/flyby2/tube-juice-2026-09-28.json | 40 771 | 5a149eadd16738ce7569535f1fc1644ddf796cac6a250188e517554a8be2355a |

The fill also merged the earlier local mirrors
`rtsw_{mag,wind}_1m-20260927T015523ZZ.json`, `-20260927T014804ZZ.json`,
`-20260928T1502Z.json` (union 5797 mag / 5711 wind readings, the bin's count).

## Trajectory

The arc `data/ssd.jpl.nasa.gov/ephemeris_juice.bin` measures sha256
**aeb3c82ff3de672116ff7f8c28592d97ea05c5e78b8f652521d2cf3cae57488a** (106 704 B)
= the seal hash (`flyby-path-2-preregistration.md:21`) — verdict **placed**.
The renewed-hash riss carried on 2026-09-27 (`eee376ef…`, 538 696 B) is resolved
by the current arc: the tube builds from the sealed arc; no trajectory-naming
word was needed. Perigee from the arc scan (5-min grid): 2026-09-28T11:43:50Z,
geocentric 15 034 km. The Horizons-measured perigee stands at
2026-09-28T11:45:12Z ± 10 s (revised paper §"Data"). The 82-s difference lies
inside the scan grid and shifts no hourly cell; both lines are carried.

## The fill

- **Grid:** t0 = 843 822 000 s TDB = 2026-09-27T22:58:50Z, 26 hourly cells,
  perigee at 11:43:50Z (cell 12 by cell-span arithmetic; the bin's round()
  marker prints "perigee" on cell 13 — convention named, not smoothed).
- **Transit (per the preregistration):** lead = d/v_sw − d/c, d = 1.5e9 m,
  c = 299 792 458 m/s (light time 5.0 s); a reading at t maps to the tube cell
  at t + lead. Wind readings carry their own proton_speed; mag readings use the
  nearest wind speed within 300 s.
- **Aggregation:** per cell per channel — mean, count, and the distinct
  (source, active) pairs. Plausibility gates: |b| ≤ 1000 nT, speed ∈ (0, 5000]
  km/s, density > 0, temperature ∈ (0, 1e8] K; pressure = n·m_p·v² (nPa).
  Cells without a measurement stay `pending` — never 0.0.

The tube register `data/flyby2/tube-juice-2026-09-28.json` carries the
per-cell source/active log; the distinct sources across the tube are
**SOLAR1 (active: true)**, **ACE (active: false)**, **IMAP (active: false)**.

## The filled chain (perigee tube, 26 hourly cells)

cell | t_utc | bt nT | bz nT | v km/s | n 1/cm³ | T K | p nPa | kp (pre) | swarm F nT | omni2 p/bz | ace bt nT | ace v km/s | ace n 1/cm³
---|---|---|---|---|---|---|---|---|---|---|---|---|---
0 | 09-27T22:58:50Z | 3.37 | −0.02 | 376.0 | 2.60 | 40075 | 0.61 | 0.00 | 32248* | pending | 4.15 | 373 | 0.131
1 | 09-27T23:58:50Z | 3.70 | −0.11 | 367.0 | 2.67 | 38174 | 0.60 | 0.00 | pending | pending | 4.19 | 365 | 0.381
2 | 09-28T00:58:50Z | 3.69 | −0.08 | 360.1 | 3.10 | 39276 | 0.68 | 0.00 | pending | pending | 4.86 | 360 | 0.600
3 | 09-28T01:58:50Z | 3.63 | −0.65 | 353.5 | 3.74 | 30829 | 0.78 | 0.00 | pending | pending | 4.34 | pending | pending
4 | 09-28T02:58:50Z | 4.24 | −1.59 | 355.7 | 3.15 | 38280 | 0.67 | 0.00 | pending | pending | 4.24 | 367 | 0.475
5 | 09-28T03:58:50Z | 3.82 | −1.34 | 352.5 | 3.56 | 29779 | 0.74 | 1.33 | pending | pending | 4.23 | 365 | 0.595
6 | 09-28T04:58:50Z | 3.77 | −1.08 | 348.5 | 3.75 | 31798 | 0.76 | 1.33 | pending | pending | 4.32 | 360 | 0.613
7 | 09-28T05:58:50Z | 3.68 | −0.89 | 349.0 | 4.13 | 38149 | 0.84 | 1.33 | pending | pending | 4.09 | 360 | 0.971
8 | 09-28T06:58:50Z | 3.91 | −0.53 | 350.3 | 4.09 | 35587 | 0.85 | 0.33 | pending | pending | 4.10 | 356 | 0.730
9 | 09-28T07:58:50Z | 3.88 | −0.39 | 350.6 | 4.17 | 36702 | 0.86 | 0.33 | pending | pending | 3.82 | 357 | 0.501
10 | 09-28T08:58:50Z | 3.85 | −0.57 | 353.1 | 4.67 | 40694 | 0.98 | 0.33 | pending | pending | 3.67 | 356 | 0.976
11 | 09-28T09:58:50Z | 3.69 | −0.52 | 353.8 | 5.32 | 37992 | 1.12 | 0.67 | pending | pending | 4.30 | 346 | 0.757
12 | 09-28T10:58:50Z | 3.41 | −0.56 | 348.5 | 6.09 | 37045 | 1.24 | 0.67 | pending | pending | 4.50 | 349 | 0.844
13 | 09-28T11:58:50Z (perigee marker) | 4.09 | −0.05 | 344.7 | 5.43 | 40693 | 1.08 | 0.67 | pending | pending | 3.86 | 340 | 0.665
14 | 09-28T12:58:50Z | 4.29 | 0.57 | 341.2 | 5.19 | 40998 | 1.01 | 0.67 | pending | pending | 4.46 | pending | pending
15 | 09-28T13:58:50Z | 3.29 | −0.64 | 342.4 | 6.67 | 30396 | 1.32 | 0.67 | pending | pending | 4.71 | 332 | 1.911
16 | 09-28T14:58:50Z | 4.14 | −1.10 | 335.3 | 5.87 | 36721 | 1.11 | 0.67 | pending | pending | pending | pending | pending
17 | 09-28T15:58:50Z | 4.42 | −0.90 | 329.6 | 5.42 | 39043 | 0.99 | 0.67 | pending | pending | 4.26 | 342 | 1.249
18 | 09-28T16:58:50Z | 4.23 | −1.29 | 330.5 | 5.55 | 36622 | 1.01 | 0.67 | pending | pending | 4.19 | 335 | 1.460
19 | 09-28T17:58:50Z | 3.78 | −1.30 | 330.5 | 5.74 | 36122 | 1.05 | 0.67 | pending | pending | 4.01 | 346 | 0.938
20 | 09-28T18:58:50Z | 3.49 | −1.17 | 330.2 | 6.38 | 32550 | 1.18 | 1.33 | pending | pending | 3.66 | 343 | 1.094
21 | 09-28T19:58:50Z | 3.75 | −1.48 | 330.0 | 2.07 | 53613 | 0.37 | 1.33 | pending | pending | 3.41 | 334 | 0.944
22 | 09-28T20:58:50Z | 3.81 | −1.43 | 326.5 | 2.29 | 47586 | 0.40 | 1.33 | pending | pending | 3.37 | 335 | 1.099
23 | 09-28T21:58:50Z | 3.58 | −0.68 | 325.4 | 2.40 | 43107 | 0.42 | 0.00 | pending | pending | pending | pending | pending
24 | 09-28T22:58:50Z | 3.35 | −0.68 | 329.4 | 2.44 | 46740 | 0.44 | 0.00 | pending | pending | pending | pending | pending
25 | 09-28T23:58:50Z | 3.28 | −1.02 | 330.9 | 2.71 | 49780 | 0.50 | 0.00 | pending | pending | pending | pending | pending

\* cell 0 swarm: the fill crossed the Swarm-latency gate mid-run (the bin's
gate slice [t0, now − 86400 s], a few tens of seconds at run time). The HAPI
serves that window — measured 22:59Z: 190 samples in [22:58:50Z, 23:02:00Z],
mean 32 893 nT; the register's 32 248 nT is the first slice of it.

Measured features (fact level): bz southward through the tube except cell 14;
density peak 6.67 cm⁻³ at cell 15 (13:58:50Z); pressure peak 1.32 nPa there;
speed declines from 376 to ~326 km/s across the tube. The plasma-pressure
gradient channel of the seal is carried as the pressure series here; its
per-cell gradient derives from these cells and stays `pending` where a
neighbor is `pending` (the chain, not a fabricated gradient).

## Kp (GFZ, def + nowcast)

`data/kp.gfz.de/kp-20260926-20260929.json` (HTTP 400 status line with a valid
JSON body — the response carries the full payload; measured, not assumed):
24 intervals 2026-09-26T00:00 → 2026-09-28T21:00Z, all `status: "pre"`
(preliminary; the final `def` release replaces them). The GFZ datetime is the
interval **start**; each cell carries the 3-h interval containing its start
(the bin's fill_kp). The perigee instant 11:45:12Z lies in the interval
[09:00, 12:00Z] → **Kp 0.667** (the bin's perigee-marker cell 13 carries the
same interval). The Kp cells are `pre` — a later addendum carries the `def`
values when GFZ releases them.

## Swarm (SW_FAST_MAGA_LR_1B, at the site — no transit)

- HAPI info (measured 2026-09-28T22:52Z): startDate 2026-03-28T08:09:2x,
  stopDate **2026-09-28T14:50:1x** — the latency is ~8 h, not the 24 h the
  fill bin assumes (`SWARM_LATENCY_S = 86400`).
- The bin's register therefore leaves cells 1–25 `pending` (its fixed gate
  opens progressively over the next 24 h; cell 0 filled partially).
- The addendum fills the served window from the raw mirror (54 300 1-s
  samples, 09-27T23:45:00 → 09-28T14:50:1x), mean F per cell:

cell | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15
---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---
F mean nT | 41470 | 35496 | 34519 | 38774 | 35799 | 40454 | 38635 | 38146 | 42603 | 37540 | 35734 | 37580 | 38685 | 33601 | 34271 | 42781
samples | 831 | 3561 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3069

F spans 18.9–53.6 kT over the window (the orbit mixes latitudes); the per-cell
min/max live in the raw mirror. Cells 16–25 stay `pending` until the HAPI
advances. The divergence between the register (bin gate) and this supplement
(measured availability) is named, never smoothed.

## OMNI2 (verification channel)

`data/cdaweb.gsfc.nasa.gov/omni2-20260926-20260929.csv`: HAPI 2.0 code 1201
"OK - no data for time range" — the ~6 d merged-1-h lag (measured 09-23)
leaves every tube cell `pending`. Verification cells fill ~4–6 d later.

## ACE (1-h, verification)

Cells 0–22 carry the 1-h mag/swepam means at their own times (no transit —
bin convention); cells 3, 14, 16, 23–25 are `pending` (measured gaps/lag in
the 1-h files at fetch time).

## The σ-metric — superseded as the scoring instance, `pending`

The falsification metric (RMS (measured − pre-registered)/σ, threshold fam,
`flyby-path-2-falsification-metric-addendum.md`) was superseded as the scoring
instance by the revised Blatt (`flyby-path-2-preregistration-revised.md`,
2026-09-27): the field chain recorded here is the **descriptive /
verification channel**. The live rule is Δ ≤ δ + 3·σ_recon with
δ = 0.16847323696971178 km (measured, DE441 vs DE442, `flyby_ephemeris_gate`,
built `d310d5888`) and Δ, σ_recon **`pending`** — the post-flyby
reconstruction (SPK + 1-σ covariance) is not published;
`data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` is absent (measured
2026-09-28). No in-situ JUICE field measurement exists yet, so the σ-metric
cannot be applied — it is `pending` with the named trigger (the published
reconstruction), never a number. Agreement remains silence.

## Cells left pending — the complete list with reasons

- swarm_f cells 1–25 in the tube register — the bin's fixed 24-h Swarm
  latency gate (register state); cells 16–25 additionally have no HAPI data
  yet (stopDate 09-28T14:50:1x). Cells 0–15 fill from the raw mirror above.
- omni2_pressure / omni2_bz, all 26 cells — HAPI 1201 (no data, ~6 d lag).
- ace channels, cells 3 (bt), 14 (speed/density), 16, 23–25 — 1-h file gaps/lag.
- kp `def` values — the `pre` values stand until the final GFZ release.
- JUICE in-situ field — after the flyby (the seal's comparison target).
- Δ, σ_recon — post-flyby reconstruction not published.

## Next steps

- The CI fill (`flyby-path2-fill.yml`, river's named step) dispatches after
  2026-09-29T00:00Z and extends the tail cells.
- Swarm cells 16–25 fill as the HAPI stopDate advances (re-measure, not a
  loop — the trigger is the stopDate change).
- The recon gate scores Δ when ESA/ESOC publishes the SPK + covariance.
