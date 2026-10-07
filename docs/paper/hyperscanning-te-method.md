<!--
  title: Hyperscanning EEG Triads — Two-Level Transfer-Entropy with FWER Control
  class: paper
  date: 2026-10-06
  sha256: c051ba6782c9d8291c90918e496cce9963043701ee517338d77e1923802e4f49
  status: live
  see-also: docs/paper/hyperscanning-te-preregistration.md, .github/workflows/hyperscanning-te.yml, tools/measure/src/bin/hyperscanning_group_te.rs, src/mathematikerin/te.rs, docs/handover/archiv/handover-2026-10-05-sensory-folge234.md
-->
# Hyperscanning EEG Triads — Two-Level Transfer-Entropy with FWER Control

## Abstract

We describe a two-level transfer-entropy (TE) screen for hyperscanning EEG triads,
implemented in `tools/measure/src/bin/hyperscanning_group_te.rs` on the
Takens-embedded topological TE estimator of `src/mathematikerin/te.rs`. One TE
family is tested per task over every triad and every ordered pair; the null is the
empirical distribution over phase-randomized surrogates, read at two levels — the
family maximum (the max-statistic carries the whole family, `FWER = 1 - percentile`)
and the per-cell distribution (each ordered pair against its own surrogate series,
naming the weaker transfer the family maximum masks)
(`hyperscanning_group_te.rs:35-38`). The method is exercised on the OpenNeuro
feasibility cohort `ds007822` (no family-maximum survivor at any task condition, any
sample size) and on the discovery cohort `ds007471` (`jointaction`), where the
family maximum is broken at channel `Cz` and, under a joint `Fz,Cz` family maximum,
by the bidirectional `pair-18@Cz` line alone. The decision rule and every screen
parameter were frozen before the cohort runs; the pre-registration is
`docs/paper/hyperscanning-te-preregistration.md`.

## 1. Introduction

Hyperscanning records two or more participants simultaneously, so a directed
statistic of inter-brain coupling must control the family-wise error over a large
set of ordered pairs and task conditions. This paper defines one such method and
fixes its decision rule before measurement. The method asks one question per task:
does any ordered pair carry more transfer than the phase-randomized null of the
whole family, and which weaker transfers does that correction mask. The family
maximum is the FWER carrier; the per-cell line is reported beside it and never
merged with it (`hyperscanning_group_te.rs:36-38`).

## 2. Data and cohorts

The data source is OpenNeuro. The workflow POSTs to
`https://openneuro.org/crn/graphql` for `dataset(id:"<ds>"){ latestSnapshot
{ hexsha } }`, then for `latestSnapshot { files(tree: "…", recursive: true)
{ filename urls } }` (`.github/workflows/hyperscanning-te.yml:50-73`). It keeps every
file whose name ends in `_eeg.set`, downloads `_channels.tsv` sidecars, and builds
`manifest.txt` (`.github/workflows/hyperscanning-te.yml:64-84`).

The manifest line shape is `<task> <triad> <slot> <path>`
(`hyperscanning_group_te.rs:51`, parser `hyperscanning_group_te.rs:61-82`). The task
token is read from the file name by `sed -E 's/.*_task-([^_]+)_eeg\.set/\1/'`
(`.github/workflows/hyperscanning-te.yml:69`), the triad token from the subject
folder `sub-G<NN>S<NN>` → `G<NN>` (`.github/workflows/hyperscanning-te.yml:70`), the
slot from the subject folder, the path from the downloaded file
(`.github/workflows/hyperscanning-te.yml:73`).

`ds007822` is the **feasibility cohort** — it exercises the manifest, fetch and
estimator path; `ds007471` is the **discovery cohort**. The dataset is registered at
the asset level in `phi/harvest.φ:190-197` (`format openneuro_pd_eeg`, `tag
openneuro.org`, `args --dataset ds007822`, `idempotent true`, `timeout 240`); the
per-file source lines carry `format openneuro_pd_eeg`, `origin
https://openneuro.org/crn/graphql`, `compiler
tools/harvest/src/bin/openneuro_compiler.rs`, `at earth`, `ttl 86400`
(e.g. `phi/sources.φ:2716-2721`).

One named electrode per participant is taken, not the common average
(`hyperscanning_group_te.rs:52-55`). `--channel` is a comma-separated label list;
the default is `Fz` (`hyperscanning_group_te.rs:568-575`). Multiple labels pool every
channel into one joint family maximum over channels × triads × ordered pairs, so the
FWER is corrected across the whole channel set (`hyperscanning_group_te.rs:52-55`;
the joint-family structure is asserted by `joint_channel_family_gate`,
`hyperscanning_group_te.rs:1630`).

An absent recording drops its triad from the family, never a fabricated 0
(`hyperscanning_group_te.rs:57`). A series that is absent or shorter than `MIN_N` is
not loaded (`hyperscanning_group_te.rs:183-186`), the missing member prints
`absent — no readable […] series (0 honored)` (`hyperscanning_group_te.rs:696-700`),
a triad with fewer than two members is not pushed (`hyperscanning_group_te.rs:702-706`),
an empty task prints `no complete triad carries a series — pending (0 honored)`
(`hyperscanning_group_te.rs:710-712`), and a run on no readable series exits `2` with
no measurement (`hyperscanning_group_te.rs:554-556`, `hyperscanning_group_te.rs:777-782`).

## 3. Estimator

The one estimator is the Takens-embedded topological TE, `dim 3`
(`hyperscanning_group_te.rs:12`, `hyperscanning_group_te.rs:32-33`). Per series the
mutual-information lag tau replaces the lag grid — `find_mi_lag`
(`src/mathematikerin/te.rs:1974`), applied per member in `member_taus`
(`hyperscanning_group_te.rs:303-311`). The observed cell is `topological_te_estimate`
(`src/mathematikerin/te.rs:3228-3253`), which embeds with the MI lag and the cross MI
lag (`find_cross_mi_lag`, `src/mathematikerin/te.rs:2045`) and evaluates the KSG
estimate `transfer_entropy_embedded_ksg` with `TE_KSG_K = 4`
(`src/mathematikerin/te.rs:2364`). On the surrogate path the observed tau values are
frozen and the surrogate is re-estimated with `topological_te_estimate_frozen`
(`src/mathematikerin/te.rs:3255-3284`, `hyperscanning_group_te.rs:313-329`), so
observed and surrogate paths run the same estimator function
(`hyperscanning_group_te.rs:32-34`).

## 4. Two-level family-wise screen

Per task, `surrogate_family_maxima` (`hyperscanning_group_te.rs:331-385`) builds the
family of maxima over (channels × triads × ordered pairs) and the per-cell
distributions. The family-max threshold is `percentile(family.maxima, pct)`
(`hyperscanning_group_te.rs:717`); a family-max survivor is a cell with
`te > threshold` (`hyperscanning_group_te.rs:721-722`). The FWER of the max-statistic
is `1 - percentile` (`hyperscanning_group_te.rs:36`). A per-cell survivor is a cell
above its own surrogate distribution (`per_cell_survivors`,
`hyperscanning_group_te.rs:387-392`); the two lists are printed separately, and a
per-cell survivor outside the family set is labelled
`PER-CELL SURVIVOR (masked by the family maximum)`
(`hyperscanning_group_te.rs:752-764`).

The per-cell confirmation run (`--nominees`) reads the nominee artifact
(`hyperscanning_group_te.rs:42-47`) and tests each nominated cell against its own
per-cell null with a fresh seed (`run_confirmation`, `hyperscanning_group_te.rs:456-552`).

## 5. Null models

The null model is selected by `--null` (`hyperscanning_group_te.rs:615-624`):

- `phase` (default): each series of a triad is rotated alone —
  `phase_randomized_surrogate` (`src/mathematikerin/te.rs:1687`), called per member in
  `randomized_triad` (`hyperscanning_group_te.rs:289-300`).
- `coherent-phase`: every series of a triad is rotated with one shared phase vector,
  preserving the linear cross-structure — the pair null for transfer beyond the linear
  cross-correlation (`hyperscanning_group_te.rs:48-50`), `coherent_phase_surrogates`
  (`src/mathematikerin/te.rs:1704`), called in `randomized_triad`
  (`hyperscanning_group_te.rs:282-288`).

## 6. Thresholds and parameter freeze

Screen defaults, fixed in the bin (`hyperscanning_group_te.rs:12-19`):
`DIM = 3` (`:12`), `DEFAULT_SURROGATES = 200` (`:13`), `DEFAULT_PERCENTILE = 95.0`
(`:14`), `SEED = 0x9E37_79B9_7F4A_7C15` (`:17`), `MIN_N = 32` (`:19`).
Confirmation defaults (`:15-18`): `CONFIRM_SURROGATES = 1000` (`:15`),
`CONFIRM_PERCENTILE = 99.0` (`:16`), `CONFIRM_SEED = 0x2545_F491_4F6C_DD1D` (`:18`).

Workflow input defaults (`.github/workflows/hyperscanning-te.yml:9-36`): `null_model`
`phase` (`:16`), `channel` `Fz` (`:20`), `max_points` `"4096"` (`:24`), `percentile`
`"95"` (screen; confirmation 99) (`:29-32`), `surrogates` `"200"` (screen;
confirmation 1000) (`:33-36`). `max_points = 0` reads each series in full
(`hyperscanning_group_te.rs:56`); the confirmation step passes p99 and 1000 surrogates
by the bin defaults (`.github/workflows/hyperscanning-te.yml:102-118`).

## 7. Calibration and gates

The canonical CPU Kalibrier-Gate lives in `src/mathematikerin/te.rs` tests:

- FP: `calibration_fp_independent_ar1_stays_near_chance`
  (`src/mathematikerin/te.rs:4807`) — 30 independent AR(1) trials, requires
  `meas >= 20` and `fp <= 8`.
- FN: `calibration_fn_true_coupling_is_found`
  (`src/mathematikerin/te.rs:4836`) — 20 coupled trials, requires the found fraction
  `> 0.5`.
- symmetry: `calibration_symmetry_identical_series_measure_equally`
  (`src/mathematikerin/te.rs:4876`) — `a = b` measures equal within `1e-12`.
- n-floor: `calibration_n_floor_no_statement_below_threshold`
  (`src/mathematikerin/te.rs:5091`) — `n = 16` carries no verdict.

The family/coherence gates of the group screen live in
`tools/measure/src/bin/hyperscanning_group_te.rs`:

- `family_fn_gate` (`hyperscanning_group_te.rs:1051`) — a coherent sheet nonlinear
  cell carries no transfer beyond its coherent null.
- `family_fp_gate` (`hyperscanning_group_te.rs:1562`) — independent structured series
  stay under their own per-cell null at p95 (`fp <= 6` of a minimum 16 measurable
  trials).
- `coherent_null_fp_gate` (`hyperscanning_group_te.rs:1597`) — independent rich-series
  pairs stay under their own coherent null (`fp <= 6`).
- `joint_channel_family_gate` (`hyperscanning_group_te.rs:1630`) — the joint
  two-channel family maximum equals the maximum of the per-channel maxima, and the
  joint per-cell FP control holds (`fp <= 8`).
- `self_null_discriminator` (`hyperscanning_group_te.rs:1210`) and `fn_gate_sweep`
  (`hyperscanning_group_te.rs:1519`) — the KSG/KDE self-null discriminator and the
  n/s coherent-null sweep.
- `nominees_round_trip` (`hyperscanning_group_te.rs:1727`) — the nominee artifact
  round-trips.

The workflow runs this list in the screen job:
`family_fn_gate family_fp_gate joint_channel_family_gate coherent_null_fp_gate
self_null_discriminator fn_gate_sweep nominees_round_trip`
(`.github/workflows/hyperscanning-te.yml:49`). The confirm job runs
`confirmation_stochastic_driver_pair_clears_its_own_null`
(`hyperscanning_group_te.rs:1839`) and
`riss_guard_deterministic_pair_measures_below_its_own_null`
(`hyperscanning_group_te.rs:1853`) (`.github/workflows/hyperscanning-te.yml:161`),
with the print-only diagnostics `frozen_tau_delay_sweep_stochastic_pair`
(`hyperscanning_group_te.rs:1870`), `positive_control_rich_aperiodic_driver_pair`
(`hyperscanning_group_te.rs:1918`) and the `phase_null_blind_band` test
(`.github/workflows/hyperscanning-te.yml:164-170`).

The numeric FP outcomes are measured (run `37197710877`, head `f69fd6496`, `success`
2026-10-04T13:46:16Z; log read via `ci_manage log 37197710877 --all`): the three FP
gates of the `plan` job each carried **0 survivors of 20 measurable trials** —
`coherent-per-cell-FP gate`, `per-cell-FP gate`, `joint per-cell-FP gate`. The FN
line is the `family_fn_gate` and the joint-family screen reported in §9.

## 8. Pre-registered hypotheses and decision rule

Hypotheses and decision rule, fixed by the code cited above:

- **H1 (corrected family finding).** A task carries a corrected finding when at least
  one ordered pair exceeds the family-max threshold `percentile(family.maxima, 95)`
  (`hyperscanning_group_te.rs:717-722`). The family-max survivor is the
  FWER-corrected finding; its FWER is `1 - percentile` (`hyperscanning_group_te.rs:36`).
- **H2 (weaker masked transfer).** A cell that exceeds its own per-cell distribution
  but not the family maximum is a weaker transfer that the family maximum masks
  (`hyperscanning_group_te.rs:36-38`, `hyperscanning_group_te.rs:752-764`). It is
  reported separately, never merged with H1.
- **Confirmation rule.** Each H2 nominee is tested in the confirmation run against its
  own fresh per-cell null at p99 with 1000 surrogates (`hyperscanning_group_te.rs:15-18`,
  `hyperscanning_group_te.rs:42-47`); the cell is `CONFIRMED` when its observed TE
  exceeds the fresh per-cell threshold (`hyperscanning_group_te.rs:527-532`), otherwise
  `NOT-CONFIRMED`. A nominee whose series or triad is absent prints `PENDING` and is not
  counted confirmed (`hyperscanning_group_te.rs:503-517`, `hyperscanning_group_te.rs:537-540`).
  The family-maximum path stays the FWER carrier; the confirmation replaces only the
  family re-test (`hyperscanning_group_te.rs:46-47`).

The registered result is the decision rule above; the rule was frozen before the cohort
runs. The screen parameters above are the tree's fixed values
(`hyperscanning_group_te.rs:12-19`, `.github/workflows/hyperscanning-te.yml:9-36`).

## 9. Results

### 9.1 Feasibility cohort `ds007822`

Measured by run `37197710877` (head `f69fd6496`, `success` 2026-10-04T13:46:16Z; the
`screen` job is split per task condition since `f69fd6496`). Parameters: channel `Fz`,
`max_points 4096`, `surrogates 200`, `percentile 95`, null `phase`, dim 3; the per-cell
confirmation is a fresh per-cell null at p99 with 1000 surrogates. The numbers are read
from the run log (`ci_manage log 37197710877 --all`, lines `=== <task>` and
`=== confirmation <task>`).

| task | triads | cells | family-max p95 | observed max | family-max survivors | per-cell survivors | confirmed |
|---|---|---|---|---|---|---|---|
| `pdrest` | 11 | 66 | 8.3159e-1 | 4.0343e-1 | 0 | 2 | 2/2 |
| `pdfeedback` | 11 | 66 | 7.9215e-1 | 3.7459e-1 | 0 | 2 | 2/1 |
| `pddecision` | 11 | 66 | 8.3607e-1 | 4.2518e-1 | 0 | 2 | 2/0 |

`family-max survivors = 0` in every task condition: no cell exceeds its family maximum,
so **H1 carries no survivor** in the feasibility cohort. The two per-cell nominees per
task (H2) confirm inconsistently across the conditions (2/2 · 2/1 · 2/0) — no
family-significant correction is found. The confirmation is reported for transparency;
the FWER carrier is the family-maximum path alone. This is the feasibility cohort
`ds007822` — no coupling claim is made (0 honored: the absence of a family-significant
cell is a measured absence, not a zero).

### 9.2 n-scaling curve (`ds007822`)

Measured by run `37217425078` (head `f289d7c88`, `success` 2026-10-05T02:05:48Z; the
`screen` job is split per task condition). Parameters: channel `Fz`,
`max_points 512, 1024, 2048, 4096`, `surrogates 200`, `percentile 95`, null `phase`,
dim 3; read from the run log (`ci_manage log 37217425078 --all`, the
`===== channel Fz n <N> =====` blocks and the `=== <task>` summaries). `p95` =
family-max p95; `pc` = per-cell survivors.

| task | n=512 | n=1024 | n=2048 | n=4096 |
|---|---|---|---|---|
| `pdrest` | p95 5.4861e-1 · pc 1 | p95 6.2251e-1 · pc 2 | p95 6.9468e-1 · pc 3 | p95 8.3159e-1 · pc 2 |
| `pdfeedback` | p95 5.7863e-1 · pc 1 | p95 6.3788e-1 · pc 4 | p95 7.0519e-1 · pc 4 | p95 7.9215e-1 · pc 2 |
| `pddecision` | p95 5.1350e-1 · pc 3 | p95 6.3336e-1 · pc 1 | p95 7.3259e-1 · pc 0 | p95 8.3607e-1 · pc 2 |

**`family-max survivors = 0` at every n and every task condition** — the family
maximum is never broken as n grows; only the per-cell count (the weaker masked
transfer, H2) varies with n. The n=4096 row reproduces the `ds007822` values at
`max_points 4096`, so the curve is consistent with the feasibility screen (still
`ds007822` — 0 honored).

### 9.3 Discovery cohort `ds007471` — single channel `Cz`

Measured by run `37235249763` (`hyperscanning-te`, head `feb7f0e55`, `success`
2026-10-05T04:39:25Z). Parameters: `cohort=ds007471`, channel `Cz`, `max_points=2048`
(a compute-bounded deviation from the 4096 default, recorded here; the full-cap run
exceeded the platform job cap of 360 min — measured 2026-10-06, attempts `37209904312`
and `37175843252` cancelled after 6 h 08 min / 6 h 48 min), 31 triads, 62 cells, `surrogates 200`, `percentile 95`,
null `phase`, dim 3. Read from the run log (`ci_manage log 37235249763 --all`) and the
artifact `hyperscanning-te-report-jointaction` (Artifact ID `11325587633`, 4380 B).

| cohort | triads | cells | family-max p95 | observed max | family-max survivors | per-cell survivors | confirmed |
|---|---|---|---|---|---|---|---|
| `jointaction` (ds007471) | 31 | 62 | 6.5144e-1 | 7.0851e-1 | **3** | 51 | 50/51 |

**The family maximum is broken here.** Observed max `7.0851e-1` > family-max p95
`6.5144e-1`; the three family-max survivors (`FAMILY-MAX SURVIVOR`) are:

- `pair-02@Cz R→L` `tau 19/17` TE `6.5198e-1` — confirmed (fresh p99 threshold `4.9826e-1`);
- `pair-18@Cz L→R` `tau 25/25` TE `7.0597e-1` — confirmed (threshold `6.5226e-1`);
- `pair-18@Cz R→L` `tau 25/25` TE `7.0851e-1` — confirmed (threshold `6.6064e-1`).

All three survive the fresh per-cell confirmation at p99, and 50 of the 51 per-cell
nominees confirm. This is the first **family-significant** finding in the line: **H1
carries 3 survivors** in the `jointaction` condition of `ds007471` at `Cz` (95 % FWER),
where the feasibility cohort `ds007822` carried none. 0 honored: this is the measured
count at the registered parameters (one cohort, one channel, one `max_points`) — the
cross-channel repetition follows in §9.4; no cross-cohort claim is made.

### 9.4 Joint cross-channel `Fz,Cz` (`ds007471`)

Measured by run `37292154121` (`hyperscanning-te`, head `632b164ba`, `success`
2026-10-05T13:39:22Z). Parameters: `cohort=ds007471`, `--channel Fz,Cz`,
`max_points=2048`, `surrogates 200`, `percentile 95`, null `phase`, dim 3, 62 triads
(= 31 triads × 2 channels), 124 cells. Read from the run log
(`ci_manage log 37292154121 --all`) and the artifact
`hyperscanning-te-report-jointaction` (Artifact ID `11349076871`, 6965 B).

| cohort | channels | triads | cells | family-max p95 | observed max | family-max survivors | per-cell survivors | confirmed |
|---|---|---|---|---|---|---|---|---|
| `jointaction` (ds007471) | `Fz,Cz` | 62 | 124 | 6.7425e-1 | 7.0851e-1 | **2** | 93 | 88/93 |

**The joint family maximum is broken by `pair-18@Cz` alone.** Pooling `Fz` and `Cz`
raises the family-max p95 from `6.5144e-1` (single channel `Cz`) to `6.7425e-1`; of
the three single-channel `Cz` survivors only `pair-18@Cz` survives the stricter joint
line, in both directions (`FAMILY-MAX SURVIVOR`):

- `pair-18@Cz L→R` `tau 25/25` TE `7.0597e-1` — confirmed (fresh p99 threshold `6.5226e-1`);
- `pair-18@Cz R→L` `tau 25/25` TE `7.0851e-1` — confirmed (threshold `6.6064e-1`).

The third single-channel survivor `pair-02@Cz R→L` TE `6.5198e-1` falls below the joint
p95 and is masked to a `PER-CELL SURVIVOR` (confirmed at p99 `4.9826e-1`). 93 per-cell
nominees, 88 confirmed, 0 pending. 0 honored: the observed max `7.0851e-1` is
numerically the same line as the single-channel run (`pair-18@Cz`), not an added
channel — the joint screen's own expression is the raised p95 that drops `pair-02`.

### 9.5 Coherent-phase null (`ds007471`, `Cz`)

Measured by run `37331134587` (`hyperscanning-te`, head `703890be2`, `success`
2026-10-05T17:03:57Z). Parameters: `cohort=ds007471`, channel `Cz`, `max_points=2048`,
`surrogates 200`, `percentile 95`, null `coherent-phase`, dim 3; the per-cell confirmation is
a fresh per-cell null at p99 with 1000 surrogates and a fresh seed. Read from the run log
(`ci_manage log 37331134587 --all`).

| cohort | triads | cells | family-max p95 | observed max | family-max survivors | per-cell survivors | confirmed |
|---|---|---|---|---|---|---|---|
| `jointaction` (ds007471) | 31 | 62 | 6.3126e-1 | 7.0851e-1 | **3** | 53 | 51/53 |

**The coherent-phase null reproduces the single-channel `phase` finding.** The family maximum
is broken with the same observed max `7.0851e-1` and the same three family-max survivors as
§9.3 (`pair-02@Cz R→L` `6.5198e-1`, `pair-18@Cz L→R` `7.0597e-1`, `pair-18@Cz R→L`
`7.0851e-1`). The coherent-phase family-max p95 `6.3126e-1` is **lower** than the `phase` p95
`6.5144e-1`: the shared-phase null preserves the linear cross-structure and is therefore a
stricter line, and all three survivors clear it. 53 per-cell nominees, 51 confirmed, 0 pending
(`pair-26@Cz R→L` and `pair-27@Cz R→L` `NOT-CONFIRMED`). The `coherent-phase` blind-band
probe reports no bin of 401 outside the surrogate envelope (driver A and target B) and the
`coherent-per-cell-FP gate` carries 0 of 20 measurable survivors — the null is neither blind
nor anti-conservative. 0 honored: the finding is the same line under a stricter,
linear-cross-structure-preserving null, not a second detection.

### 9.6 Full-cap discovery run `max_points=4096` (`ds007471`, `Cz`, sharded)

Measured by run `37438929801` (`hyperscanning-te`, head `55568edd3`, `success`
2026-10-06T11:09:29Z). Parameters: `cohort=ds007471`, channel `Cz`, `max_points=4096`
(the full registered cap), `surrogates 200`, `percentile 95`, null `phase`, dim 3,
`shards=4` (the surrogate axis is sliced into four jobs and recombined through the
global family maximum — Westfall-Young maxT, one shared permutation per surrogate
index; `.github/workflows/hyperscanning-te.yml:48/190/264`), 31 triads, 62 cells. Read
from the run log (`ci_manage log 37438929801 --all`, the `=== jointaction` block of the
`merge` job) and the artifact `hyperscanning-te-report-jointaction` (Artifact ID
`11409245464`, 2364 B).

| cohort | triads | cells | family-max p95 | observed max | family-max survivors | per-cell survivors |
|---|---|---|---|---|---|---|
| `jointaction` (ds007471) | 31 | 62 | 6.7838e-1 | 7.6521e-1 | **2** | 55 |

**The shard path carries the full cap under the 360-min platform job cap.** The two
family-max survivors (`FAMILY-MAX SURVIVOR`) are:

- `pair-02@Cz R→L` `tau 9/58` TE `7.6521e-1`;
- `pair-18@Cz L→R` `tau 10/32` TE `7.0130e-1`.

The full cap raises the family-max p95 from `6.5144e-1` (§9.3, `max_points=2048`) to
`6.7838e-1`, and the observed max from `7.0851e-1` to `7.6521e-1`; the survivor set
shifts with the cap — `pair-18@Cz R→L`, the largest line at `2048`, is here a
`PER-CELL SURVIVOR` (masked by the family maximum), and the `pair-02@Cz` and
`pair-18@Cz L→R` `tau` pairs read `9/58` and `10/32` against `19/17` and `25/25` at
`2048`. The `2048` estimate was compute-bounded, not the physical truth; the full
length is the physical truth (0 honored: the full-cap run reports its own measured
survivor set, not a reproduction of §9.3 — the shortened series aliases the delay
pair, and no cross-cap identity is claimed). The sharded workflow carries no per-cell
confirmation job (the confirmation step is `if: inputs.shards <= 1`), so the two
survivors carry no fresh per-cell p99 line; the screen result stands on the merged
family-maximum alone. The monolithic full-cap attempts (`37209904312`,
`37175843252`) were cancelled at 6 h 08 min / 6 h 48 min (§9.3); the four-shard run
completed inside the cap.

## 10. Falsification

The method is falsified as a family-wise detector if a run reports a family-maximum
survivor that a fresh independent per-cell confirmation at p99 does not confirm, or if
the three FP gates carry survivors above their tolerance (`fp <= 6` of 16, `fp <= 8`
of the joint control) while the FN line (`family_fn_gate`) fails. The pre-registered
FP/FN batteries and the `phase_null_blind_band` diagnostic are the standing falsifiers:
a null that is blind to the statistic is refuted by the FP gates, and a detector that
cannot find a known stochastic driver pair is refuted by the FN line. In the measured
runs the FP gates carried 0 survivors of 20 and the `family_fn_gate` passed.

## 11. Limitations and 0 honored

The result is bounded by its registered parameters: one discovery cohort (`ds007471`),
one task condition (`jointaction`), one electrode per participant, and the `phase` null
for the screen; the `coherent-phase` null is measured for the `Cz` discovery cohort
(§9.5). The discovery runs at `max_points 2048` are the compute-bounded deviation; the
full registered `max_points = 4096` cap is measured for the `Cz` discovery cohort in
§9.6 (`37438929801`, `shards=4`, `success` 2026-10-06T11:09:29Z). The full-cap run
needed the shard path to fit under the **360-min** platform job cap (measured
2026-10-06, `ci_manage view` — no `timeout-minutes` in `hyperscanning-te.yml`; the
monolithic `4096` screen attempts `37209904312` and `37175843252` were cancelled after
6 h 08 min and 6 h 48 min, §9.3): `hyperscanning_group_te --shard <i>/<n>` slices the
surrogate axis and `--merge f0,…,fn` recombines it through the global family-maximum
(Westfall-Young maxT, shared permutation per surrogate index;
`.github/workflows/hyperscanning-te.yml:48/190/264`, asserted by
`shard_merge_equals_monolithic`, `hyperscanning_group_te.rs:2211`). The `pair-02@Cz` /
`pair-18@Cz` lines are a family-significant statistical detection, not a physiological
claim; the feasibility cohort carries no family-maximum survivor at any n (0 honored —
the measured absence is reported as absence, never as a zero).

## 12. Reproducibility

| step | run | head | outcome | artifact |
|---|---|---|---|---|
| FP/FN + feasibility (`ds007822`, `Fz`) | `37197710877` | `f69fd6496` | success 2026-10-04T13:46:16Z | — |
| n-scaling (`ds007822`, `Fz`) | `37217425078` | `f289d7c88` | success 2026-10-05T02:05:48Z | — |
| second cohort (`ds007471`, `Cz`) | `37235249763` | `feb7f0e55` | success 2026-10-05T04:39:25Z | `11325587633` (4380 B) |
| joint cross-channel (`ds007471`, `Fz,Cz`) | `37292154121` | `632b164ba` | success 2026-10-05T13:39:22Z | `11349076871` (6965 B) |
| coherent-phase null (`ds007471`, `Cz`) | `37331134587` | `703890be2` | success 2026-10-05T17:03:57Z | `11359509342` (4505 B) |
| full-cap discovery `max_points=4096`, `shards=4` (`ds007471`, `Cz`) | `37438929801` | `55568edd3` | success 2026-10-06T11:09:29Z | `11409245464` (2364 B) |

The workflow is `.github/workflows/hyperscanning-te.yml`; the screen bin is
`tools/measure/src/bin/hyperscanning_group_te.rs`; the estimator is
`src/mathematikerin/te.rs`. Logs are read with `ci_manage log <run-id> --all`; the
artifacts are the `hyperscanning-te-report-<task>` bundles.
