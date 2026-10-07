<!--
  title: Hyperscanning EEG Triads — A Two-Level Transfer-Entropy Pre-registration
  class: paper
  date: 2026-10-05
  sha256: fd354d4064ce380044be458591e060d00721f37f52aa32999e50ae9445c9d35c
  status: live
  see-also: .github/workflows/hyperscanning-te.yml, tools/measure/src/bin/hyperscanning_group_te.rs, src/mathematikerin/te.rs, docs/handover/archiv/handover-2026-10-05-sensory-folge232.md
-->
## Hyperscanning EEG Triads — A Two-Level Transfer-Entropy Pre-registration

### Zweck / Purpose

A two-level transfer-entropy (TE) screen over hyperscanning EEG triads is
implemented in `tools/measure/src/bin/hyperscanning_group_te.rs`. Per task the
screen tests one TE family over every triad and every ordered pair
(`hyperscanning_group_te.rs:35-38`); the null is the empirical distribution over
phase-randomized surrogates, read at two levels — the family maximum (the
max-statistic carries the whole family, `FWER = 1 - percentile`, the stricter
line) and the per-cell distribution (each ordered pair against its own surrogate
series, naming the weaker transfer the family maximum masks)
(`hyperscanning_group_te.rs:35-38`).

OpenNeuro `ds007822` is used as a **feasibility cohort** for the screen — the
cohort exercises the manifest, fetch and estimator path. No result claim about
hyperscanning coupling is made here.

### Daten / Data

The data source is OpenNeuro `ds007822`
(`.github/workflows/hyperscanning-te.yml:50-73`). The workflow POSTs to
`https://openneuro.org/crn/graphql` for `dataset(id:"ds007822"){ latestSnapshot
{ hexsha } }`, then for `latestSnapshot { files(tree: "…", recursive: true)
{ filename urls } }` (`.github/workflows/hyperscanning-te.yml:53-60`). It keeps
every file whose name ends in `_eeg.set`, downloads `_channels.tsv` sidecars,
and builds `manifest.txt` (`.github/workflows/hyperscanning-te.yml:64-84`).

The manifest line shape is `<task> <triad> <slot> <path>`
(`hyperscanning_group_te.rs:51`, parser `hyperscanning_group_te.rs:61-82`). The
task token is read from the file name by `sed -E
's/.*_task-([^_]+)_eeg\.set/\1/'` (`.github/workflows/hyperscanning-te.yml:69`),
the triad token from the subject folder `sub-G<NN>S<NN>` →
`G<NN>` (`.github/workflows/hyperscanning-te.yml:70`), the slot from the subject
folder, the path from the downloaded file
(`.github/workflows/hyperscanning-te.yml:73`).

The dataset is registered at the asset level in `phi/harvest.φ:190-197`:
`format openneuro_pd_eeg`, `tag openneuro.org`, `args --dataset ds007822`,
`pattern ^sub-G[0-9]+S[0-9]+_task-pd(decision|feedback|rest)_eeg\.bin$`,
`idempotent true`, `timeout 240`. The per-file source lines in `phi/sources.φ`
carry `format openneuro_pd_eeg`, `origin https://openneuro.org/crn/graphql`,
`compiler tools/harvest/src/bin/openneuro_compiler.rs`, `at earth`, `ttl 86400`
(e.g. `phi/sources.φ:2716-2721`). No line in `phi/sources.φ` carries the literal
token `ds007822`: `archive_search ds007822 --root phi` resolves
`phi/harvest.φ:194` and the research-agent note
`phi/pipeline/research/agent_output/grind_suchliste_a_urteil.φ:43`, and reports
the disposition register absent of the token.

One named electrode per participant is taken, not the common average
(`hyperscanning_group_te.rs:52-55`). `--channel` is a comma-separated label
list; the default is `Fz` (`hyperscanning_group_te.rs:568-575`). Multiple labels
pool every channel into one joint family maximum over channels × triads ×
ordered pairs, so the FWER is corrected across the whole channel set
(`hyperscanning_group_te.rs:52-55`; the joint-family structure is asserted by
`joint_channel_family_gate`, `hyperscanning_group_te.rs:1630`).

An absent recording drops its triad from the family, never a fabricated 0
(`hyperscanning_group_te.rs:57`). A series that is absent or shorter than
`MIN_N` is not loaded (`hyperscanning_group_te.rs:183-186`), the missing member
prints `absent — no readable […] series (0 honored)`
(`hyperscanning_group_te.rs:696-700`), a triad with fewer than two members is
not pushed (`hyperscanning_group_te.rs:702-706`), an empty task prints
`no complete triad carries a series — pending (0 honored)`
(`hyperscanning_group_te.rs:710-712`), and a run on no readable series exits
`2` with no measurement (`hyperscanning_group_te.rs:554-556`,
`hyperscanning_group_te.rs:777-782`).

### Methode / Estimator

The one estimator is the Takens-embedded topological TE, `dim 3`
(`hyperscanning_group_te.rs:12`, `hyperscanning_group_te.rs:32-33`). Per series
the mutual-information lag tau replaces the lag grid — `find_mi_lag`
(`src/mathematikerin/te.rs:1974`), applied per member in `member_taus`
(`hyperscanning_group_te.rs:303-311`). The observed cell is
`topological_te_estimate` (`src/mathematikerin/te.rs:3228-3253`), which embeds
with the MI lag and the cross MI lag (`find_cross_mi_lag`,
`src/mathematikerin/te.rs:2045`) and evaluates the KSG estimate
`transfer_entropy_embedded_ksg` with `TE_KSG_K = 4`
(`src/mathematikerin/te.rs:2364`). On the surrogate path the observed tau values
are frozen and the surrogate is re-estimated with
`topological_te_estimate_frozen` (`src/mathematikerin/te.rs:3255-3284`,
`hyperscanning_group_te.rs:313-329`), so observed and surrogate paths run the
same estimator function (`hyperscanning_group_te.rs:32-34`).

The null model is selected by `--null` (`hyperscanning_group_te.rs:615-624`):

- `phase` (default): each series of a triad is rotated alone —
  `phase_randomized_surrogate` (`src/mathematikerin/te.rs:1687`), called per
  member in `randomized_triad` (`hyperscanning_group_te.rs:289-300`).
- `coherent-phase`: every series of a triad is rotated with one shared phase
  vector, preserving the linear cross-structure — the pair null for transfer
  beyond the linear cross-correlation (`hyperscanning_group_te.rs:48-50`),
  `coherent_phase_surrogates` (`src/mathematikerin/te.rs:1704`), called in
  `randomized_triad` (`hyperscanning_group_te.rs:282-288`).

Per task, `surrogate_family_maxima` (`hyperscanning_group_te.rs:331-385`) builds
the family of maxima over (channels × triads × ordered pairs) and the per-cell
distributions. The family-max threshold is `percentile(family.maxima, pct)`
(`hyperscanning_group_te.rs:717`); a family-max survivor is a cell with
`te > threshold` (`hyperscanning_group_te.rs:721-722`). The FWER of the
max-statistic is `1 - percentile` (`hyperscanning_group_te.rs:36`). A per-cell
survivor is a cell above its own surrogate distribution
(`per_cell_survivors`, `hyperscanning_group_te.rs:387-392`); the two lists are
printed separately, and a per-cell survivor outside the family set is labelled
`PER-CELL SURVIVOR (masked by the family maximum)`
(`hyperscanning_group_te.rs:752-764`).

The per-cell confirmation run (`--nominees`) reads the nominee artifact
(`hyperscanning_group_te.rs:42-47`) and tests each nominated cell against its
own per-cell null with a fresh seed (`run_confirmation`,
`hyperscanning_group_te.rs:456-552`).

### Schwellen / Thresholds

Screen defaults, fixed in the bin (`hyperscanning_group_te.rs:12-19`):

- `DIM = 3` (`hyperscanning_group_te.rs:12`)
- `DEFAULT_SURROGATES = 200` (`hyperscanning_group_te.rs:13`)
- `DEFAULT_PERCENTILE = 95.0` (`hyperscanning_group_te.rs:14`)
- `SEED = 0x9E37_79B9_7F4A_7C15` (`hyperscanning_group_te.rs:17`)
- `MIN_N = 32` (`hyperscanning_group_te.rs:19`)

Confirmation defaults, fixed in the bin (`hyperscanning_group_te.rs:15-18`):

- `CONFIRM_SURROGATES = 1000` (`hyperscanning_group_te.rs:15`)
- `CONFIRM_PERCENTILE = 99.0` (`hyperscanning_group_te.rs:16`)
- `CONFIRM_SEED = 0x2545_F491_4F6C_DD1D` (`hyperscanning_group_te.rs:18`)

Workflow input defaults (`.github/workflows/hyperscanning-te.yml:9-36`):

- `null_model` default `phase` (`.github/workflows/hyperscanning-te.yml:16`)
- `channel` default `Fz` (`.github/workflows/hyperscanning-te.yml:20`)
- `max_points` default `"4096"` (`.github/workflows/hyperscanning-te.yml:24`)
- `percentile` default `"95"` (screen; confirmation 99)
  (`.github/workflows/hyperscanning-te.yml:29-32`)
- `surrogates` default `"200"` (screen; confirmation 1000)
  (`.github/workflows/hyperscanning-te.yml:33-36`)

`max_points = 0` reads each series in full (`hyperscanning_group_te.rs:56`); the
confirmation step passes p99 and 1000 surrogates by the bin defaults
(`.github/workflows/hyperscanning-te.yml:102-118`).

### Kalibrier-Gates / Calibration

The canonical CPU Kalibrier-Gate lives in `src/mathematikerin/te.rs` tests:

- FP: `calibration_fp_independent_ar1_stays_near_chance`
  (`src/mathematikerin/te.rs:4807`) — 30 independent AR(1) trials, requires
  `meas >= 20` and `fp <= 8`.
- FN: `calibration_fn_true_coupling_is_found`
  (`src/mathematikerin/te.rs:4836`) — 20 coupled trials, requires the found
  fraction `> 0.5`.
- symmetry: `calibration_symmetry_identical_series_measure_equally`
  (`src/mathematikerin/te.rs:4876`) — `a = b` measures equal within `1e-12`.
- n-floor: `calibration_n_floor_no_statement_below_threshold`
  (`src/mathematikerin/te.rs:5091`) — `n = 16` carries no verdict.

The family/coherence gates of the group screen live in
`tools/measure/src/bin/hyperscanning_group_te.rs`:

- `family_fn_gate` (`hyperscanning_group_te.rs:1051`) — a coherent sheet
  nonlinear cell carries no transfer beyond its coherent null.
- `family_fp_gate` (`hyperscanning_group_te.rs:1562`) — independent structured
  series stay under their own per-cell null at p95 (`fp <= 6` of a minimum 16
  measurable trials).
- `coherent_null_fp_gate` (`hyperscanning_group_te.rs:1597`) — independent
  rich-series pairs stay under their own coherent null (`fp <= 6`).
- `joint_channel_family_gate` (`hyperscanning_group_te.rs:1630`) — the joint
  two-channel family maximum equals the maximum of the per-channel maxima, and
  the joint per-cell FP control holds (`fp <= 8`).
- `self_null_discriminator` (`hyperscanning_group_te.rs:1210`) and
  `fn_gate_sweep` (`hyperscanning_group_te.rs:1519`) — the KSG/KDE self-null
  discriminator and the n/s coherent-null sweep.
- `nominees_round_trip` (`hyperscanning_group_te.rs:1727`) — the nominee
  artifact round-trips.

The workflow runs this list in the screen job:
`family_fn_gate family_fp_gate joint_channel_family_gate coherent_null_fp_gate
self_null_discriminator fn_gate_sweep nominees_round_trip`
(`.github/workflows/hyperscanning-te.yml:49`). The confirm job runs
`confirmation_stochastic_driver_pair_clears_its_own_null`
(`hyperscanning_group_te.rs:1839`) and
`riss_guard_deterministic_pair_measures_below_its_own_null`
(`hyperscanning_group_te.rs:1853`)
(`.github/workflows/hyperscanning-te.yml:161`), with the print-only
diagnostics `frozen_tau_delay_sweep_stochastic_pair`
(`hyperscanning_group_te.rs:1870`),
`positive_control_rich_aperiodic_driver_pair`
(`hyperscanning_group_te.rs:1918`) and the `phase_null_blind_band` test
(`.github/workflows/hyperscanning-te.yml:164-170`).

The numeric FP outcomes are measured (run `37197710877`, head `f69fd6496`,
`success` 2026-10-04T13:46:16Z; log read via `ci_manage log 37197710877 --all`):
the three FP gates of the `plan` job each carried **0 survivors of 20 measurable
trials** — `coherent-per-cell-FP gate`, `per-cell-FP gate`, `joint per-cell-FP
gate`. The FN line is the `family_fn_gate` and the joint-family screen reported
in `### Ergebnisse / Results`.

### Vorregistrierung / Pre-registration

Hypotheses and decision rule, fixed by the code cited above:

- **H1 (corrected family finding).** A task carries a corrected finding when at
  least one ordered pair exceeds the family-max threshold
  `percentile(family.maxima, 95)` (`hyperscanning_group_te.rs:717-722`). The
  family-max survivor is the FWER-corrected finding; its FWER is
  `1 - percentile` (`hyperscanning_group_te.rs:36`).
- **H2 (weaker masked transfer).** A cell that exceeds its own per-cell
  distribution but not the family maximum is a weaker transfer that the family
  maximum masks (`hyperscanning_group_te.rs:36-38`,
  `hyperscanning_group_te.rs:752-764`). It is reported separately, never merged
  with H1.
- **Confirmation rule.** Each H2 nominee is tested in the confirmation run
  against its own fresh per-cell null at p99 with 1000 surrogates
  (`hyperscanning_group_te.rs:15-18`, `hyperscanning_group_te.rs:42-47`); the
  cell is `CONFIRMED` when its observed TE exceeds the fresh per-cell threshold
  (`hyperscanning_group_te.rs:527-532`), otherwise `NOT-CONFIRMED`. A nominee
  whose series or triad is absent prints `PENDING` and is not counted
  confirmed (`hyperscanning_group_te.rs:503-517`, `hyperscanning_group_te.rs:537-540`).
  The family-maximum path stays the FWER carrier; the confirmation replaces
  only the family re-test (`hyperscanning_group_te.rs:46-47`).

The registered result is the decision rule above; the rule was frozen before
the cohort runs. The ds007822 feasibility values the rule read are reported in
`### Ergebnisse / Results` (run `37197710877`, 2026-10-04). The screen
parameters above are the tree's fixed values
(`hyperscanning_group_te.rs:12-19`, `.github/workflows/hyperscanning-te.yml:9-36`).

### Ergebnisse / Results — ds007822 feasibility screen (2026-10-04)

Measured by run `37197710877` (head `f69fd6496`, `success`
2026-10-04T13:46:16Z; the `screen` job is split per task condition since
`f69fd6496`). Parameters: channel `Fz`, `max_points 4096`, `surrogates 200`,
`percentile 95`, null `phase`, dim 3 (`hyperscanning_group_te.rs:12-19`); the
per-cell confirmation is a fresh per-cell null at p99 with 1000 surrogates
(`hyperscanning_group_te.rs:15-18`). The numbers are read from the run log
(`ci_manage log 37197710877 --all`, lines `=== <task>` and `=== confirmation
<task>`).

| task | triads | cells | family-max p95 | observed max | family-max survivors | per-cell survivors | confirmed |
|---|---|---|---|---|---|---|---|
| `pdrest` | 11 | 66 | 8.3159e-1 | 4.0343e-1 | 0 | 2 | 2/2 |
| `pdfeedback` | 11 | 66 | 7.9215e-1 | 3.7459e-1 | 0 | 2 | 2/1 |
| `pddecision` | 11 | 66 | 8.3607e-1 | 4.2518e-1 | 0 | 2 | 2/0 |

`family-max survivors = 0` in every task condition: no cell exceeds its family
maximum, so **H1 carries no survivor** in the feasibility cohort. The two
per-cell nominees per task (`H2`) confirm inconsistently across the conditions
(2/2 · 2/1 · 2/0) — no family-significant correction is found, and the family-max
silence is the measured finding (`no cell breaks the family maximum — the
family-max silence is the finding`). The confirmation is reported for
transparency; the FWER carrier is the family-maximum path alone. This is the
feasibility cohort `ds007822` — no coupling claim is made (0 honored: the
absence of a family-significant cell is a measured absence, not a zero).

### Ergebnisse / Results — n-scaling curve (run `37217425078`, 2026-10-05)

Measured by run `37217425078` (`hyperscanning-te`, head `f289d7c88`, `success`
2026-10-05T02:05:48Z; the `screen` job is split per task condition). Parameters:
channel `Fz`, `max_points 512, 1024, 2048, 4096`, `surrogates 200`, `percentile 95`,
null `phase`, dim 3; read from the run log
(`ci_manage log 37217425078 --all`, the `===== channel Fz n <N> =====` blocks and the
`=== <task>` summaries). `p95` = family-max p95; `pc` = per-cell survivors.

| task | n=512 | n=1024 | n=2048 | n=4096 |
|---|---|---|---|---|
| `pdrest` | p95 5.4861e-1 · pc 1 | p95 6.2251e-1 · pc 2 | p95 6.9468e-1 · pc 3 | p95 8.3159e-1 · pc 2 |
| `pdfeedback` | p95 5.7863e-1 · pc 1 | p95 6.3788e-1 · pc 4 | p95 7.0519e-1 · pc 4 | p95 7.9215e-1 · pc 2 |
| `pddecision` | p95 5.1350e-1 · pc 3 | p95 6.3336e-1 · pc 1 | p95 7.3259e-1 · pc 0 | p95 8.3607e-1 · pc 2 |

**`family-max survivors = 0` at every n and every task condition** — the family
maximum is never broken as n grows; only the per-cell count (the weaker masked
transfer, H2) varies with n. The n=4096 row reproduces the ds007822 values at
`max_points 4096`, so the curve is consistent with the feasibility screen. This is
still `ds007822` — 0 honored.

### Ergebnisse / Results — second cohort `ds007471` (run `37235249763`, 2026-10-05)

Measured by run `37235249763` (`hyperscanning-te`, head `feb7f0e55`, `success`
2026-10-05T04:39:25Z). Parameters: `cohort=ds007471`, channel `Cz`,
**`max_points=2048`** (a compute-bounded deviation from the 4096 default, recorded
here; the full-cap run exceeded the platform job cap), 31 triads, 62 cells,
`surrogates 200`, `percentile 95`, null `phase`, dim 3. Read from the run log
(`ci_manage log 37235249763 --all`) and the artifact
`hyperscanning-te-report-jointaction` (Artifact ID `11325587633`, 4380 B).

| cohort | triads | cells | family-max p95 | observed max | family-max survivors | per-cell survivors | confirmed |
|---|---|---|---|---|---|---|---|
| `jointaction` (ds007471) | 31 | 62 | 6.5144e-1 | 7.0851e-1 | **3** | 51 | 50/51 |

**The family maximum is broken here.** Observed max `7.0851e-1` > family-max p95
`6.5144e-1`; the three family-max survivors (`FAMILY-MAX SURVIVOR`) are:

- `pair-02@Cz R→L` `tau 19/17` TE `6.5198e-1` — confirmed (fresh p99 threshold
  `4.9826e-1`);
- `pair-18@Cz L→R` `tau 25/25` TE `7.0597e-1` — confirmed (threshold `6.5226e-1`);
- `pair-18@Cz R→L` `tau 25/25` TE `7.0851e-1` — confirmed (threshold `6.6064e-1`).

All three survive the fresh per-cell confirmation at p99, and 50 of the 51 per-cell
nominees confirm. This is the first **family-significant** finding in the line:
**H1 carries 3 survivors** in the `jointaction` condition of `ds007471` at `Cz`
(95 % FWER), where the feasibility cohort `ds007822` carried none. 0 honored: this is
the measured count at the registered parameters (one cohort, one channel, one
`max_points`) — the cross-channel repetition follows in `### Ergebnisse / Results — joint
cross-channel Fz,Cz` (run `37292154121`); no cross-cohort claim is made here.

### Ergebnisse / Results — joint cross-channel `Fz,Cz` (`ds007471`, run `37292154121`, 2026-10-05)

Measured by run `37292154121` (`hyperscanning-te`, head `632b164ba`, `success`
2026-10-05T13:39:22Z). Parameters: `cohort=ds007471`, `--channel Fz,Cz`,
`max_points=2048`, `surrogates 200`, `percentile 95`, null `phase`, dim 3,
62 triads (= 31 triads × 2 channels), 124 cells. Read from the run log
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
nominees, 88 confirmed, 0 pending. 0 honored: the observed max `7.0851e-1` is numerically
the same line as the single-channel run (`pair-18@Cz`), not an added channel — the joint
screen's own expression is the raised p95 that drops `pair-02`.

### Ergebnisse / Results — Nachweise

- **FP/FN numbers** — measured (run `37197710877`, 2026-10-04) — see
  `### Ergebnisse / Results`.
- **n-scaling curve** — measured (run `37217425078`, 2026-10-05) — see
  `### Ergebnisse / Results — n-scaling curve`; `family-max survivors = 0` at every n.
- **Second cohort** — measured (run `37235249763`, 2026-10-05) — see
  `### Ergebnisse / Results — second cohort ds007471`; **H1 carries 3 family-max
  survivors** (all three confirmed at p99), 50/51 per-cell nominees confirmed.
- **Joint cross-channel family** — built (commit `62742649c`): `--channel` accepts a
  comma-separated label list and pools every channel into ONE joint family maximum over
  channels × triads × ordered pairs (`hyperscanning_group_te.rs:52-55`), asserted by
  `joint_channel_family_gate` (`hyperscanning_group_te.rs:1630`). The joint **measurement**
  over two real channels (`Fz,Cz`, ds007471) is measured (run `37292154121`, 2026-10-05) —
  see `### Ergebnisse / Results — joint cross-channel Fz,Cz`; **2 family-max survivors**
  (`pair-18@Cz` both directions, confirmed at p99), 88/93 per-cell nominees confirmed,
  the single-channel `pair-02@Cz` survivor masked by the raised joint p95.
- **Method paper** — the publishable consolidation of this pre-registration is
  `docs/paper/hyperscanning-te-method.md` (measured 2026-10-05 F234); it consumes the
  joint measurement above. The `coherent-phase` null of the discovery cohort is
  measured there (run `37331134587`, `success` 2026-10-05T17:03:57Z, §9.5): the same
  three family-max survivors under the stricter shared-phase null. The full
  `max_points=4096` discovery run is **measured** (run `37438929801`, `shards=4`,
  `success` 2026-10-06T11:09:29Z, §9.6): the family maximum is broken with two
  survivors (`pair-02@Cz R→L` TE `7.6521e-1`, `pair-18@Cz L→R` TE `7.0130e-1`), 55
  per-cell nominees.

### Träger / Carrier

All pre-registered validation artifacts are measured; the last open marker — the full
`max_points=4096` discovery run `37438929801` — is closed by its `merge`-job result
(§9.6). No open marker remains; the former carrier point in the Sensory handover is
resolved.
