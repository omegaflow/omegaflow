<!--
  title: The flyby proof, Path 2 — the perigee fill-run (addendum)
  class: paper
  date: 2026-09-29
  sha256: 0224cbfac3fd0cac5348a91da24c79ba021af14d436f1b057d86b9c92064a7a4
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
16 | 09-28T14:58:50Z | 4.14 | −1.10 | 335.3 | 5.87 | 36721 | 1.11 | 0.67 | 34705 | pending | pending | pending | pending
17 | 09-28T15:58:50Z | 4.42 | −0.90 | 329.6 | 5.42 | 39043 | 0.99 | 0.67 | 35531 | pending | 4.26 | 342 | 1.249
18 | 09-28T16:58:50Z | 4.23 | −1.29 | 330.5 | 5.55 | 36622 | 1.01 | 0.67 | 44203 | pending | 4.19 | 335 | 1.460
19 | 09-28T17:58:50Z | 3.78 | −1.30 | 330.5 | 5.74 | 36122 | 1.05 | 0.67 | 36096 | pending | 4.01 | 346 | 0.938
20 | 09-28T18:58:50Z | 3.49 | −1.17 | 330.2 | 6.38 | 32550 | 1.18 | 1.33 | 38713 | pending | 3.66 | 343 | 1.094
21 | 09-28T19:58:50Z | 3.75 | −1.48 | 330.0 | 2.07 | 53613 | 0.37 | 1.33 | 39081 | pending | 3.41 | 334 | 0.944
22 | 09-28T20:58:50Z | 3.81 | −1.43 | 326.5 | 2.29 | 47586 | 0.40 | 1.33 | 36263 | pending | 3.37 | 335 | 1.099
23 | 09-28T21:58:50Z | 3.58 | −0.68 | 325.4 | 2.40 | 43107 | 0.42 | 0.67 | 36159 | pending | 2.64 | 338 | 1.577
24 | 09-28T22:58:50Z | 3.35 | −0.68 | 329.4 | 2.44 | 46740 | 0.44 | 0.67 | 34294 | pending | 3.44 | 337 | 1.321
25 | 09-28T23:58:50Z | 3.28 | −1.02 | 330.9 | 2.71 | 49780 | 0.50 | 0.67 | 39627 | pending | 3.94 | 337 | 1.665

\* cell 0 swarm: the fill crossed the Swarm-latency gate mid-run (the bin's
gate slice [t0, now − 86400 s], a few tens of seconds at run time). The HAPI
serves that window — measured 22:59Z: 190 samples in [22:58:50Z, 23:02:00Z],
mean 32 893 nT; the register's 32 248 nT is the first slice of it.

Cells 16–25 in the chain above are the re-measured Swarm supplement of
2026-09-30 (the HAPI stopDate advanced; see the Swarm section); the value is the
direct HAPI-window mean, not the bin register (the register's cell 25 stays
`pending` — its fetch stop is the start of cell 25 — and its cells 16–24 agree
within 6 nT, named, never smoothed).

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
same interval). The Kp cells were `pre` — a later addendum carries the `def`
values when GFZ releases them. Re-measured 2026-09-30 (`kp.gfz.de`): the
`status=def` query returned empty arrays; the preliminary series now reaches the
[21:00, 24:00Z) interval at 0.667 — the chain's cells 23–25 carry that value
(revised from the earlier run's 0.00). **Released 2026-10-08:** the
`status=def,nowcast` query returns **HTTP 200** with all 33 intervals
`status: "def"`; the register carries the 26 def kp values (cell 0→25:
`0 0 0 0 0 1.333 1.333 1.333 0.333 0.333 0.333 0.667 0.667 0.667 0.667 0.667 0.667 0.667 0.667 0.667 1 1 1 0.333 0.333 0.333`), i.e. the final series, not `pre`.

## Swarm (SW_FAST_MAGA_LR_1B, at the site — no transit)

- HAPI info (measured 2026-09-28T22:52Z): startDate 2026-03-28T08:09:2x,
  stopDate **2026-09-28T14:50:1x** — the latency is ~8 h, not the 24 h the
  fill bin assumes (`SWARM_LATENCY_S = 86400`).
- The bin's register therefore left cells 1–25 `pending` at that run (its fixed
  gate opens progressively over the next 24 h; cell 0 filled partially).
- The addendum fills the served window from the raw mirror (54 300 1-s
  samples, 09-27T23:45:00 → 09-28T14:50:1x), mean F per cell.
- Re-measured 2026-09-30 (`vires.services/hapi/info?id=SW_FAST_MAGA_LR_1B`):
  stopDate advanced to **2026-09-30T10:16:18Z** — the trigger fired; cells 16–25
  are served. The supplement fills them from the direct HAPI window
  (`start=2026-09-28T14:58:50Z&stop=2026-09-29T00:58:50Z&parameters=F&format=csv`;
  HTTP 200, 1 416 623 B, sha256 `d5185547…`;
  `data/vires.services/swarm-magalr-20260928T145850-20260929T005850.csv`). A
   re-run of `flyby_path2_fill` filled the register cells 16–24; the fetch-stop
   fix (`river 73`, `e7eae7862`) extended the stop past cell 25, and the 2026-10-03
   re-run (frozen RTSW snapshots, live Swarm) filled cell 25 in the register as
   well (39625 nT vs the direct window's 39627). Mean F per cell (0–15 from the
   mirror, 16–25 from the HAPI window):

cell | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25
---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---
F mean nT | 41470 | 35496 | 34519 | 38774 | 35799 | 40454 | 38635 | 38146 | 42603 | 37540 | 35734 | 37580 | 38685 | 33601 | 34271 | 42781 | 34705 | 35531 | 44203 | 36096 | 38713 | 39081 | 36263 | 36159 | 34294 | 39627
samples | 831 | 3561 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3069 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3600 | 3574 | 3600

F spans 18.8–53.7 kT over the 26-cell window (the orbit mixes latitudes); the
per-cell min/max live in the raw mirror. The register (bin) means for cells
16–25 agree within 6 nT (measured). The earlier stop boundary at cell 25 is
healed (register cell 25 filled 2026-10-03), never smoothed.

## OMNI2 (verification channel)

`data/cdaweb.gsfc.nasa.gov/omni2-20260926-20260929.csv`: HAPI 2.0 code 1201
"OK - no data for time range" — the ~6 d merged-1-h lag (measured 09-23)
leaves every tube cell `pending`. Verification cells fill ~4–6 d later.

## ACE (1-h, verification)

Cells 0–22 carry the 1-h mag/swepam means at their own times (no transit —
bin convention). Re-measured 2026-09-30: cells 23–25 now fill from the 1-h
files; cells 3 (bt/speed/density), 14 (speed/density) and 16 stay `pending`
(gaps/lag in the 1-h files).

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
is not applied — it is `pending` with the named trigger (the published
reconstruction), never a number. Agreement remains silence.

## Cells left pending — the complete list with reasons

- omni2_pressure / omni2_bz, all 26 cells — HAPI 1201 (no data, ~6 d lag;
  re-measured 2026-09-30).
- ace channels, cells 3 (bt/speed/density), 14 (speed/density), 16 — 1-h file
  gaps/lag (re-measured 2026-09-30); cells 23–25 now fill from the 1-h files.
- kp `def` values — the preliminary values stand until the final GFZ release
  (the 2026-09-30 re-run carries 0.667 for cells 23–25, revised from 0.00).
- JUICE in-situ field — after the flyby (the seal's comparison target).
- Δ, σ_recon — post-flyby reconstruction not published.

## 2026-10-03 fill re-run — the trajectory riss stands

The CI re-run (`flyby-path2-fill`, run 37116911686, 2026-10-03, success) read
the arc locally absent, fetched the CDN `ephemeris_juice.bin`, and measured
`sha256 018ce2ca680b195ceda4e4013ceed1a4d0c80ddb1732629d456ffa197bd33e20`
(538 696 B) against the seal `aeb3c82f…` (106 704 B, `:21`). The two arcs
differ, so no tube was built from the unsealed arc — verdict **riss**, both
witnesses named (`data/flyby2/tube-juice-2026-09-28.json`). The local
`data/ssd.jpl.nasa.gov/ephemeris_juice.bin` still measures `aeb3c82f…`
(106 704 B); the CDN asset is the renewed 538 696 B arc (re-measured
2026-10-03 by `curl` + sha256). Per the seal's own rule the renewal is named
before the flight; until it is named every tube cell stays unbuilt. The riss
is registered in the folge86 handover. Its origin is now measured
(Mycelium-227, 2026-10-03): the CDN asset was produced by the `kernel-flatten`
run `37029375744` @`1e6d21f2f` (success, 2026-10-02, step `--systems
planets,jupiter,saturn,mars,uranus,neptune,pluto,juice`; CDN `updated_at`
2026-10-02T22:07:06Z). The 2026-09-27 renewal `eee376eff…` is thereby
superseded; the current CDN arc is the third state `018ce2ca…`. The seal binds
to the Blatt arc `aeb3c82f…`; the trajectory choice — name `018ce2ca…` as the
trajectory or restore the sealed arc — is the operator's word (the seal clause,
preregistration §Addendum).

The operator's word (2026-10-03) resolves it: the sealed arc `aeb3c82f…` is the
**official** trajectory; the 2026-09-27 renewal `eee376ef…` is an admissible
**pre-flight line** (its bytes are present and hash-verified at
`data/ssd.jpl.nasa.gov/ephemeris_juice_renewed.bin`); the 2026-10-02 state
`018ce2ca…` is a **post-flight comparison line**. No line is averaged, none is
selected by outcome.

## 2026-10-04 — the three lines built: official, renewed and postflight placed

The `river 87` line switch (`flyby_path2_fill --line`, `c0dbc0ede`) and the
line-sharp fetch (`mycelium 227`, `38419ddcf`) dispatched the three lines on the
CDN arcs. Measured 2026-10-04 via `ci_manage` and the run artifacts
(`flyby-path2-fill.txt`, `tube-juice-2026-09-28*.json`):

- **official** (run `37158607593`, success): arc `aeb3c82f…` (106 704 B) — tube
  **placed**, perigee `2026-09-28T11:43:50Z` geocentric **15 034 km**, 26 hourly
  cells (`data/flyby2/tube-juice-2026-09-28.json`). Against the revised
  preregistration (`11:45:12 ± 10 s` / `15 018 km`) the sealed arc stands 82 s
  early / +16 km — the flyby is carried.
- **renewed** (run `37158609804`, success): arc `eee376ef…` (538 696 B) — tube
  **placed**, perigee `2026-09-28T11:58:50Z` geocentric **62 745 km**, 26 cells
  (`…-renewed.json`). The environment grid is identical to the official tube
  (same hourly snapshots); the two lines diverge only in the perigee witness:
  15 034 km vs 62 745 km. The renewed arc does not carry the flyby minimum —
  it stands as the named second witness, never averaged (riss named, operator's
  word 2026-10-03).
- **postflight** (run `37166537879`, success): measured `aeb3c82f…` (106 704 B)
  != the seal `018ce2ca…` → **riss**, no tube built. The CDN
  `ephemeris_juice.bin` at the run carried the official arc, not the postflight
  state.

Re-measured 2026-10-04 via `archive_search --sniff`: `ephemeris_juice.bin` =
`aeb3c82f…` (106 704 B); `ephemeris_juice_official.bin` = `aeb3c82f…`;
`ephemeris_juice_renewed.bin` = `eee376ef…`. The postflight arc `018ce2ca…`
(538 696 B) is **absent** from the CDN. The `juice-arc-restore` run
`37165635404` (success, 2026-10-04T00:48Z) restored only 106 704 B — its
`--systems juice` reproduces the sealed arc, not the producer's
`--systems planets,jupiter,saturn,mars,uranus,neptune,pluto,juice` that yielded
`018ce2ca…` (kernel-flatten `37029375744` @`1e6d21f2f`).

The restore was re-run with the producer's full systems list (Committed `b60b756f2`) and
returned the asset: `ephemeris_juice.bin` = `018ce2ca…` (538 696 B, `--sniff` 2026-10-04).
The `--line postflight` re-run (`37187374751`, success) then built the tube: **placed**,
perigee `2026-09-28T11:58:50Z` geocentric **62 745 km**, 26 hourly cells
(`data/flyby2/tube-juice-2026-09-28-postflight.json`). The postflight and renewed arcs
(the two 538 696 B states) agree at 62 745 km; the sealed official arc (106 704 B) stands
at 15 034 km. The riss is carried as three named witnesses, never averaged.

## 2026-10-08 — OMNI2 HAPI reachable; the RTSW snapshots fill the register

The OMNI2 merged-1-h HAPI URL (OMNI2_H0_MRG1HR, 2026-09-26..29) returned
**HTTP 200** (33 251 B) — the earlier 1201 "no data for time range" lag is
healed. The first fill (`flyby_path2_fill -- --flyby juice`) placed the
trajectory **official** (arc `aeb3c82f…`, sha-verified) but left every RTSW cell
and `omni2_bz` `pending`: run without `--snapshots`, the loader reaches only the
live SWPC URL, whose 24-h retention no longer holds the window. The frozen
mirrors (`data/services.swpc.noaa.gov/rtsw_{mag,wind}_1m-*.json`) carry it; the
re-run with `--snapshots data/services.swpc.noaa.gov` filled all 26 cells
(register `data/flyby2/tube-juice-2026-09-28.json` rewritten 2026-10-08 23:04,
41 528 B).

Measured register state per channel (n of 26 cells filled):

| channel | filled | pending |
|---|---|---|
| `rtsw_bt` `rtsw_bz` `rtsw_speed` `rtsw_density` `rtsw_temperature` `rtsw_pressure` | 26 | 0 |
| `omni2_bz` | 0 | 26 |
| `kp` (`status def`) | 26 | 0 |
| `omni2_pressure` | 26 | 0 |
| `swarm_f` | 26 | 0 |
| `ace_bt` | 25 | 1 (cell 16) |
| `ace_speed` | 23 | 3 (cells 3, 14, 16) |
| `ace_density` | 23 | 3 (cells 3, 14, 16) |

RTSW series per cell (cell 0→25), from the `--snapshots` fill:

cell | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25
---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---
`rtsw_bt` nT | 3.37 | 3.70 | 3.69 | 3.63 | 4.24 | 3.82 | 3.77 | 3.68 | 3.91 | 3.88 | 3.85 | 3.69 | 3.41 | 4.09 | 4.29 | 3.29 | 4.14 | 4.42 | 4.23 | 3.78 | 3.49 | 3.75 | 3.81 | 3.58 | 3.35 | 3.28
`rtsw_bz` nT | -0.02 | -0.11 | -0.08 | -0.65 | -1.59 | -1.34 | -1.08 | -0.89 | -0.53 | -0.39 | -0.57 | -0.52 | -0.56 | -0.05 | 0.57 | -0.64 | -1.10 | -0.90 | -1.29 | -1.30 | -1.17 | -1.48 | -1.43 | -0.68 | -0.68 | -1.02
`rtsw_speed` km/s | 376.0 | 367.0 | 360.1 | 353.5 | 355.7 | 352.5 | 348.5 | 349.0 | 350.3 | 350.6 | 353.1 | 353.8 | 348.5 | 344.7 | 341.2 | 342.4 | 335.3 | 329.6 | 330.5 | 330.5 | 330.2 | 330.0 | 326.5 | 325.4 | 329.4 | 330.9
`rtsw_density` cm⁻³ | 2.60 | 2.67 | 3.10 | 3.74 | 3.15 | 3.56 | 3.75 | 4.13 | 4.09 | 4.17 | 4.67 | 5.32 | 6.09 | 5.43 | 5.19 | 6.67 | 5.87 | 5.42 | 5.55 | 5.74 | 6.38 | 2.07 | 2.29 | 2.40 | 2.44 | 2.71
`rtsw_temperature` K | 40075 | 38174 | 39276 | 30829 | 38280 | 29779 | 31798 | 38149 | 35587 | 36702 | 40694 | 37992 | 37045 | 40693 | 40998 | 30396 | 36721 | 39043 | 36622 | 36122 | 32550 | 53613 | 47586 | 43107 | 46740 | 49780
`rtsw_pressure` nPa | 0.61 | 0.60 | 0.68 | 0.78 | 0.67 | 0.74 | 0.76 | 0.84 | 0.85 | 0.86 | 0.98 | 1.12 | 1.24 | 1.08 | 1.01 | 1.32 | 1.11 | 0.99 | 1.01 | 1.05 | 1.18 | 0.37 | 0.40 | 0.42 | 0.44 | 0.50

`omni2_bz` is **absent in the source**: the OMNI2_H0_MRG1HR response carries
`BX_GSE1800`/`BY_GSM1800`/`BZ_GSM1800` as the fill sentinel `999.9` across the
whole window (only `Pressure1800` carries values) — no B measurement, so the
channel stays `pending` (never 0.0). kp `status=def` is **released**: GFZ returns
**HTTP 200** with all intervals `def` (measured 2026-10-08), and the register
carries the 26 def kp values. `omni2_pressure` per cell (nPa, cell 0→25):

cell | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25
---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---
`omni2_pressure` nPa | 0.83 | 0.82 | 0.84 | 0.81 | 0.92 | 1.10 | 1.10 | 1.19 | 1.32 | 1.46 | 1.46 | 1.40 | 1.55 | 1.62 | 1.53 | 1.55 | 1.52 | 1.36 | 1.47 | 1.29 | 1.29 | 1.23 | 1.26 | 1.28 | 1.35 | 1.42

No value is smoothed; a cell without a measurement stays `pending` — never 0.0.

## Next steps

- The three lines are built (2026-10-04): official `15 034 km`, renewed and postflight
  `62 745 km`. The official/renewed/postflight riss is carried, never averaged.
- The CI fill (`flyby-path2-fill.yml`, river's named step) dispatched after the
  seal; the 2026-09-30 re-run filled the register's swarm cells 16–24, and the
  2026-10-03 re-run (with the `river 73` fetch-stop fix) filled cell 25.
- kp `def` fills on the final GFZ release; OMNI2 fills ~4–6 d after the perigee
  (HAPI 1201).
- The recon gate scores Δ when ESA/ESOC publishes the SPK + covariance.
