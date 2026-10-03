<!--
  title: Hyperscanning EEG Triads — A Two-Level Transfer-Entropy Pre-registration
  class: paper
  date: 2026-10-03
  sha256: 11ccdb90e8ccfeb8c802a88b80e77b6f556acb68412e1630f44d88dbe442ec53
  status: live
  see-also: .github/workflows/hyperscanning-te.yml, tools/measure/src/bin/hyperscanning_group_te.rs, src/mathematikerin/te.rs, docs/handover/handover-2026-10-03-sensory-folge227.md
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
not pushed (`hyperscanning_group_te.rs:702-706`), an empty task prints `no
complete triad carries a series — pending (0 honored)`
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

The numeric FP/FN outcomes of the cohort runs are **pending** — Braucht: the CI
artifact `hyperscanning-te-report`
(`.github/workflows/hyperscanning-te.yml:143-152`) of the workflow run(s). The
run in flight measured 2026-10-03 via `ci_manage status` is `37128441536`
(`in_progress`, job `screen`, step `Run the family-wise TE screen and write the
per-cell nominees (Takens tau, dim 3, cap 4096)`; its `confirm` job is
`success`); `37129875260` is `pending`; `37129873384` is `cancelled`. No numeric
FP/FN value is read into this document.

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

No numeric result is reported by this document. Data were fetched and screened
by the workflow run `37128441536` before this document
(`ci_manage status`, 2026-10-03); no value of that run is reported here. The
screen parameters above are the tree's fixed values
(`hyperscanning_group_te.rs:12-19`, `.github/workflows/hyperscanning-te.yml:9-36`).
The registered result is the decision rule; the data value it will read is
`pending` the artifact `hyperscanning-te-report`
(`.github/workflows/hyperscanning-te.yml:146`).

### Offene Ergebnisse / Pending results

- **FP/FN numbers** — pending — Braucht: artifact `hyperscanning-te-report`
  (`.github/workflows/hyperscanning-te.yml:146`) of run `37128441536` or
  `37129875260` (`ci_manage status`, 2026-10-03).
- **n-scaling curve** — pending — Braucht: the `scaling.txt` / `scaling_*.txt`
  members of the same artifact
  (`.github/workflows/hyperscanning-te.yml:119-142`,
  `.github/workflows/hyperscanning-te.yml:151`).
- **Second cohort** — pre-registered candidate (measured 2026-10-03): **`ds007471`**
  (dyads, 64-channel BrainVision `.vhdr` — 32 `_R` + 32 `_L` in one file per
  `sub-01`..`sub-32`, 1000 Hz, CC0) is the primary second cohort; `ds008192` (fNIRS +
  MoCap) and `ds004103` (fMRI) stay other-modality candidates. The read path is
  measured: CI probe `openneuro-eeg-probe` run `37127134440` (success) read `ds007471`
  sub-01 through `brainvision_compiler` and `ds008192` sub-101 through `snirf_compiler`
  (`ci_manage view 37127134440`, 2026-10-03). The dyad join is the `_L`/`_R` channel
  split inside one file; the manifest arm is built (commit `1226e9082`):
  `brainvision_compiler --participant L|R` writes one per-participant bin, and the
  workflow input `cohort=ds007471` builds `manifest.txt` with
  `jointaction pair-<NN> {L,R}`. Braucht: the run (after the ds007822 validation) before
  analysis.

### Träger / Carrier

This document's carrier is the Sensory handover
`docs/handover/handover-2026-10-03-sensory-folge227.md` — the open point
`### Hyperscanning-TE — Präregistrierung/Methodenpapier (ds007822 = Machbarkeit)`,
whose `Lage` names this draft and whose `Braucht` names the validation artifact
that will carry the FP/FN and n-scaling numbers.
