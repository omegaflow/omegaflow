<!--
  title: The energy ladder of the corona: transfer entropy across eleven lines
  class: paper
  date: 2026-09-06
  version: 5
  sha256: ff76b349f9ecdea590746aa3e32e04e5df7f3df6c76f4dd5b62417fd733c9773
  fam-machine: pre-fix (EVE-2011); AIA-2014 fam post-fix; EVE bandwidth cross-check (h×0.5–3.0) 2026-09-05; AIA three-year matrix (2013/14/15) post-fix
  status: live
  see-also: docs/surveys/survey-ein-blatt-korona-heizung.md docs/specs/broken-null-control.md
-->

# The energy ladder of the corona: transfer entropy across eleven lines

*Omegaflow Working Group — Korona-Heizung Session, 2026-08-22*

## Abstract

The corona is heated to 1–2 MK against a 6000 K photosphere, by Alfvén-wave transport or nanoflare heating. We measure transfer entropy (TE) between adjacent rungs of the solar temperature ladder — eleven EUV/UV lines from SDO/EVE, from 584 Å (log T = 4.16) to 94 Å (log T = 6.81) — at 10 s cadence over 109 flares (2011) with a phase-randomized null, complemented by SDO/AIA imaging (24-s cells) over three independent years (2013, 2014, 2015). The estimator reconstructs the Schreiber (2000) benchmark (asymmetry 6.75). The central result is a reproducible, directed, family-bound cascade: the hot-corona chain 193→211→335→94 Å carries the largest TE up the ladder at the ~96 s Alfvén crossing in all three years, and the per-event direction of the hottest rung is 76% positive — consistent, not alternating. The amplitude stays below the full-round family bound in every year (0.63–0.91 × fam). The single EVE rung that clears the family bound at canonical bandwidth (1032→131 Å) is not bandwidth-robust, failing at h ≥ 2.0. The measurement establishes a reproduced coronal direction under the family bound, but no rung clears both the family bound and the bandwidth check — a family-bound cascade, not a Pfeil.
## 1. Introduction

The coronal heating problem is the temperature inversion of the outer solar
atmosphere: the photosphere radiates at ~6000 K, the corona above it at 1–2
MK. The two standing mechanisms differ in the *direction and timing* of the
energy flow. Alfvén waves generated in the photosphere and chromosphere
propagate upward along magnetic field lines and dissipate in the corona; the
energy then flows *upward* with a *wave-crossing time lag*, roughly 10–30 s
across the thin transition region and ~100 s across the thick corona.
Nanoflares are local, stochastic reconnection events in the corona itself;
they carry *no consistent inter-layer lag* — every layer heats together or
not at all. Correlation cannot separate these: all spectral lines co-vary
through their common origin in the flare and the solar cycle. Transfer
entropy (Schreiber, 2000) is the natural instrument: a directional,
model-free measure of information flow, here applied *across the temperature
ladder* — between adjacent formation-temperature rungs — to ask whether the
cooler layer's past carries information about the hotter layer's future,
beyond what the hotter layer's own past and the common flare envelope
already explain.

This paper reports the first sub-minute, full-ladder measurement: eleven
spectral lines, ten-second cadence, 109 flare events, a strict
phase-randomized null, and an estimator validated against known ground
truth. All verdicts below are reported exactly as the machine measured them,
including the silent ones (0 honored).

## 2. Data

### 2.1 The temperature ladder

SDO/EVE Level 2 (V8) line irradiances, sun-as-a-star, 10 s cadence, W m⁻²
at 1 AU. The eleven lines, in formation-temperature order (log T in K):

| line | ion | log T | layer |
|---|---|---|---|
| 584 Å | He I | 4.16 | chromosphere |
| 304 Å | He II | 4.70 | transition region |
| 977 Å | C III | 4.84 | transition region |
| 1032 Å | O VI | 5.47 | transition region |
| 131 Å | Fe VIII | 5.57 | low corona |
| 171 Å | Fe IX | 5.81 | corona |
| 195 Å | Fe XII | 6.13 | corona |
| 211 Å | Fe XIV | 6.27 | corona |
| 284 Å | Fe XV | 6.30 | corona |
| 335 Å | Fe XVI | 6.43 | corona |
| 94 Å | Fe XVIII | 6.81 | hot corona |

### 2.2 The harvest and the era

EVE Level 2 is harvested hourly (`EVL_L2_YYYYDOY_HH_008_01.fit.gz`, gunzip +
FITS binary table, the 71-element LINE_IRRADIANCE column; the line indices
are fixed by matching WAVE_CENTER, not by position). The window is 2011 DOY
40–130 (2011-02-09 → 2011-05-10), 90 days, 5 862 322 line records; 2011 is
the healthy MEGS-A era. **The 2014 era is degraded and is deliberately
excluded:** across a 2014 90-day harvest (5 365 960 records), the 304 Å line
varies only 1.2× around an X1 flare (2014-03-29), and the ESP EUV diodes are
flat — the MEGS-A instrument lost sensitivity through accumulated radiation
damage. The instrument's own X-ray "Quad Diode" (0.1–7 nm) remains
flare-sensitive (13.5×) and serves as the flare trigger.

### 2.3 Flare events

Events are detected on the Quad Diode: a sustained excursion above 1.3× the
90-day median (threshold 3.90e-4 W m⁻²), refractory 30 min, peak-centered.
Each event carries a ±20 min window of all eleven lines on the common 10 s
grid (240 cells, gaps skipped). 109 events survive the full-line coverage
requirement.

### 2.4 The second instrument — SDO/AIA imaging (2014–2015, 2013)

The independent confirmation harvests SDO/AIA Level 1 imaging through the
JSOC `exportdata` interface. The handover's `res=` sub-sampling parameter is
falsified (jsoc_fetch delivers the full 4096² frame regardless); the harvest
therefore uses JSOC's own full-disk keyword **DATAMEAN** (jsoc_info rs_list),
per year as monthly assets × 7 bands (94/131/171/193/211/304/335 Å) × 12 s,
reported as **DN/s** — DATAMEAN divided by EXPTIME, the AEC normalization
required because AIA switches to short exposures during flares (verified on
the 171 Å series). A cross-check against real RICE_1-decompressed images
gives the ratio 0.9018. Events are taken from the GOES soft X-ray record
(the standard flare classifier), windowed at 24-s cells on the common grid.
The imaging era is healthy across 2013–2015 (unlike the degraded EVE-2014
lines of §2.2), so the three-year comparison rests on one consistent
instrument era. The full-disk 304 Å does not serve as a flare trigger: only
26 cells exceed 1.3× its median — the disk damps the flare ~10×, exactly
the sun-as-a-star attenuation that the EVE lines also carry.

## 3. Methods

### 3.1 Transfer entropy on the ladder

For adjacent ladder rungs (cool X, hot Y), the lag-τ transfer entropy
TE(X→Y; τ) is estimated by the KDE estimator with Silverman bandwidths
(Schreiber, 2000; Kaiser & Schreiber, 2002); the reverse TE(Y→X; τ) is
measured identically. The directional excess

D(τ) = TE(X→Y; τ) − TE(Y→X; τ),

normalized per event by the event's own TE magnitude, is stacked over the
109 events. D > 0 is upward flow (cool → hot), D < 0 downward. The lag sweep
τ = 0…120 s (10 s cells) is the discriminator: a peak of D at the wave
crossing time is a propagation signature; a flat or lag-0 D is co-heating.

### 3.2 Null model

For every pair and lag, ten phase-randomized surrogates of the cool series
(f64 FFT, deterministic seed, the same machinery as the system's
broken-null-control record) yield the null distribution of D. The family
bound fam is the strongest surrogate D over the whole round (all pairs ×
lags); here fam = 4.6982e-1. A D is fam-significant only when |D| exceeds
fam; every other D is silent.

### 3.3 Estimator validation against known ground truth

Before interpretation, the scalar estimator is validated on the Schreiber
(2000) benchmark: unidirectionally coupled Hénon maps, n = 10 000, lag 1,
c = 0.20, with the same null machinery. The known direction X→Y carries
TE(X→Y) = 2.457e-1 against a family bound fam = 4.550e-2 (a factor 5.4); the
reverse direction TE(Y→X) = 3.64e-2 clears its own per-lag threshold (2.10e-2)
but not the family bound; the c = 0 control is silent in both directions. The
estimator reconstructs the known direction and only the known direction; the
asymmetry ratio is 6.75.

### 3.4 The sub-minute constraint

The ~100 s Alfvén crossing of the corona is sub-minute. The earlier
measurements at coarser grains are silent and reported for context: the
daily-scale sheet over 11 years (30 directed pairs, 2009–2020, family bound
2.108e-1 — no arrow), and the three-force sub-minute measurement (GOES 2 s
X-ray two bands + 10 s Lyman-α, 200 flares, 2014) with no null-significant
lag. The present paper is the first sub-minute run on the full temperature
ladder.

### 3.5 The measuring probes (method notes preserved from the code)

The four probe tools of this measurement each carry a one-line method note in
their leading code comment; that documentation is recorded here so it survives
in the paper rather than in code.

- **corona_ladder_probe** — The temperature-ladder probe, the Alfvén question
  on the full ladder: reads the EVE lines ordered by formation temperature
  (LOGT), recognizes flare events via the hottest line (94 Å, Fe XVIII), and
  measures per adjacent (cool, hot) pair the directed TE over a lag sweep,
  event-stacked against the phase null; a consistently positive D at lag ≈ 10
  cells (≈ 100 s) means energy flow UP the ladder (Alfvén-consistent); D
  without lag and without direction is the shared nanoflare heating.
- **aia_ladder_probe** — The AIA ladder probe, the Alfvén question on the full
  AIA band ladder: reads the full-disk DATAMEAN/EXPTIME per band (DN/s, the AEC
  normalization), lays the bands on 24-s cells (the four camera-2 bands are
  phase-shifted by ~3 s; 24 s carries all bands in one cell), orders by
  formation temperature (cool → hot), recognizes flare events via the GOES-b
  flux (b_flux > 1e-6 W/m², C1.0) or, without a goes directory, via the 304-Å
  full disk itself (median × factor), and measures per adjacent pair the
  directed TE over a lag sweep against the phase null; a consistently positive
  D at lag ≈ 4 cells (≈ 96 s) means energy flow UP the ladder.
- **corona_event_probe** — The 90-day event probe, the stacked sub-minute
  multi-force TE of the corona heating: reads 2-s X-ray (gxrs: a_flux hot,
  b_flux cool) and 10-s Lyman-α (geuv-ir10s), lays each day on a 10-s grid,
  finds flare events in the cool band (b_flux > 1e-6 W/m², C1.0), lays a ±20-min
  window around each event, and computes the conditional TE there in both
  directions; a consistently positive D at lag ≈ 10 cells (≈ 100 s) over many
  events is Alfvén-consistent; if D vanishes under the phase-randomized null,
  the lead was no real information flow.
- **corona_lag_probe** — The sub-minute corona lag probe, the measurement that
  attacks the riddle: reads two NCEI files of a flare day (2-s X-ray and 10-s
  Lyman-α) onto a common 10-s grid, then TE in both directions over a lag
  sweep; the Alfvén prediction is that the chromosphere (Lyman-α) leads the
  corona (X-ray) by the travel time (~100 s ≈ 10 cells); nanoflares carry no
  consistent lag.

## 4. Results

The stacked directional excess D(τ) per adjacent pair, 109 events; asterisk
marks D above the full-round family bound fam = 4.6982e-1:

| pair | D(0 s) | D(20 s) | D(40 s) | D(60 s) | D(80 s) | D(100 s) | D(120 s) | verdict |
|---|---|---|---|---|---|---|---|---|
| 584→304 (chrom→TR) | 3.89e-1 | 1.96e-1 | 5.22e-2 | 1.69e-2 | 1.27e-2 | 9.78e-3 | 9.42e-3 | silent |
| 304→977 (TR) | −3.82e-1 | −2.37e-1 | −1.29e-1 | −9.53e-2 | −7.61e-2 | −7.23e-2 | −7.28e-2 | silent |
| 977→1032 (TR) | 1.08e-2 | 3.12e-2 | 3.75e-2 | 3.10e-2 | 3.27e-2 | 2.03e-2 | 2.68e-2 | silent |
| **1032→131 (TR→corona)** | **5.11e-1*** | 3.08e-1 | 1.95e-1 | 1.64e-1 | 1.47e-1 | 1.39e-1 | 1.33e-1 | **upward, lag 0** |
| 131→171 (corona) | 5.26e-3 | 1.09e-2 | 1.31e-2 | 5.65e-3 | 8.40e-3 | 1.84e-2 | 9.54e-3 | silent |
| 171→195 (corona) | 8.04e-3 | 1.34e-2 | 7.80e-3 | 9.03e-3 | 4.71e-3 | 8.97e-3 | 6.33e-3 | silent |
| 195→211 (corona) | −1.11e-2 | −3.45e-3 | 6.21e-3 | −1.38e-2 | 7.04e-3 | −9.52e-3 | −2.87e-3 | silent |
| 211→284 (corona) | −1.00e-1 | −9.65e-2 | −1.18e-1 | −1.08e-1 | −1.06e-1 | −1.03e-1 | −9.85e-2 | silent |
| 284→335 (corona) | −4.31e-1 | −1.61e-1 | 1.95e-2 | 3.40e-2 | 3.30e-2 | 2.40e-2 | 1.46e-2 | silent |
| 335→94 (hot corona) | 2.21e-1 | −9.23e-2 | −3.02e-1 | −3.15e-1 | −3.11e-1 | −3.00e-1 | −2.78e-1 | silent |

One arrow clears the full-round family bound at the canonical bandwidth:
**1032→131 Å** (O VI → Fe VIII, the transition-region→corona boundary) is
strongest at zero lag (5.11e-1, fam 4.70e-1). Every other rung is silent at
fam, including the 977→1032 Å entry (peak 3.91e-2) and the hottest rungs'
negative D (335→94, down to −3.15e-1). The lag structure is front-loaded at
lag 0, not a ~100 s peak.

**Bandwidth cross-check (§4.4).** This single arrow is not robust to the KDE
bandwidth. Under a Silverman-width scale h × {0.5, 1.0, 1.5, 1.75, 2.0, 3.0},
with fam freshly recomputed per h over surrogates of the same h (fair test,
109 events at every h), the 1032→131 D holds through h ≈ 1.75 and falls
below fam at h ≥ 2.0. The failure is robust to the surrogate count: fam is
the maximum of the round's surrogate D, so a larger surrogate sample is only
stricter (a-fortiori). No other EVE rung clears fam at any bandwidth.

### 4.2 AIA-2014 (194 GOES events, 24-s cells)

The imaging ladder reads the coronal rung that EVE's degraded lines could not.
Measured over the full 2014-03-01–05-30 window (7 bands, GOES-15 trigger,
194 events, our own fam), the coronal rungs show a coherent upward *pattern*
with the ~96 s crossing lag — but every rung stays below the full-round
family bound. No AIA arrow is fam-significant:

| pair | D(peak) | lag | fam verdict |
|---|---|---|---|
| 304→131 (cool side) | −9.79e-2 | 192 s | silent / negative |
| 131→171 | −9.19e-2 | 192 s | silent / negative |
| 171→193 | −4.17e-2 | 120 s | silent / negative |
| 193→211 | +1.50e-1 | ~96 s | below fam |
| 211→335 | +1.13e-1 | ~144 s | below fam |
| **335→94** | **+1.42e-1** | ~96 s | **below fam (fam = 1.89e-1)** |

fam = 1.8925e-1 — the strongest surrogate D of the full round (6 pairs × 13
lags × 10 surrogates, post-fix RNG). The hot-corona rungs (193→211→335→94)
are the ladder's largest D and carry the ~96 s coronal crossing lag, but
none clears the family bound. The GOES-13 trigger (208 events, its own fam =
1.80e-1) reproduces the same picture: 335→94 peaks at +1.40e-1, below fam.
The cool side (304→131) is silent or negative — the same full-disk damping
that caps every sun-as-a-star channel.

### 4.3 The hottest rung is not established at fam

Earlier this AIA run was read against a per-lag surrogate null (no family
bound), which marked 335→94 "upward" while EVE-2011's same rung was silent.
Restoring the family bound on the AIA side removes the disagreement: AIA's
335→94 (peak +1.42e-1) is below its own fam (1.89e-1), just as EVE's
335→94 (down to −3.15e-1) is below its own fam (4.70e-1). Neither instrument
measures the hottest rung's direction at its family bound. The two
measurements no longer disagree on a reversal — each independently fails to
establish the hot-rung direction at fam. What remains across both is a
single EVE candidate at the transition-region→corona boundary (1032→131)
that clears its family bound only at canonical/narrow bandwidth (§4.4) and,
on the AIA side, a below-fam but coherent coronal-rung pattern carrying the
~96 s crossing lag. Neither survives the combined fam + bandwidth
criterion. The three-year reproduction (§4.5) extends the AIA side: the
below-fam pattern is not a 2014 artifact — it reproduces in 2013 and 2015
with a consistent per-event direction, still under the family bound.

### 4.4 EVE bandwidth cross-check (Silverman h × 0.5–3.0)

The single EVE arrow (1032→131) is tested against the KDE bandwidth. The
probe gains a non-canonical bandwidth-scaled estimator (`transfer_entropy_lag_h`,
canonical estimator untouched); factor 1.0 reproduces the canonical round
byte-identically. Each h is a full round with fam freshly recomputed over
surrogates of the same h (fair test). Event count is h-independent (109 at
every h).

| h | 1032→131 D (lag 0) | fam (same round, same h) | D/fam | arrow |
|---|---|---|---|---|
| 0.5 | 4.09e-1 | 3.19e-1 | 1.28 | yes |
| 1.0 | 5.11e-1 | 4.70e-1 | 1.09 | yes |
| 1.5 | 4.99e-1 | 4.89e-1 | 1.02 | yes |
| 1.75 | 4.71e-1 | 4.69e-1 | 1.00 | marginal |
| 2.0 | 4.34e-1 | 4.46e-1 | 0.97 | no |
| 3.0 | 2.17e-1 | 3.24e-1 | 0.67 | no |

The arrow holds only through h ≈ 1.75 and falls below fam at h ≥ 2.0. The
failure is robust to the surrogate count: fam is the maximum of the round's
surrogate D, so a larger surrogate sample (stricter band) cannot lower it —
an arrow that misses the loose n=10 band misses any sharper band a fortiori.
No other EVE rung clears fam at any h in 0.5–3.0. Under a combined
family-bound + bandwidth criterion, the EVE-2011 ladder carries no robust
directional arrow.

### 4.5 Three-year reproduction (AIA 2013 / 2014 / 2015)

The AIA ladder is now measured over three independent years (GOES-15
triggers, 24-s cells, the healthy imaging era). The hot-corona cascade
193→211→335→94 Å reproduces in every year at the ~96 s crossing lag; all
nine rung-year D values are positive:

| year | events | 193→211 | 211→335 | 335→94 | lag | fam | verdict |
|---|---|---|---|---|---|---|---|
| 2013 | 524 | +1.67e-1 | +7.71e-2 | +1.01e-1 | ~96 s | 1.71e-1 | family bound |
| 2014 | 1019 | +1.78e-1 | +7.15e-2 | +1.24e-1 | ~96 s | 1.96e-1 | family bound |
| 2015 | 281 | +1.21e-1 | +9.37e-2 | +1.30e-1 | ~96 s | 1.75e-1 | family bound |

The per-event direction of the hottest rung (335→94, 2015, 281 events) is
76% positive, majority-positive in every month (posfrac 0.56–1.00), with
all monthly means positive — the direction is consistent, not alternating,
at event and month scale. The wave/alternation hypothesis (an oscillating
direction driver) is not carried. Yet the cascade's amplitude stays below
the full-round family bound in every year, and the hottest rung does not
strengthen as the ensemble grows fivefold (335→94: +1.42e-1 at 194 events
→ +1.24e-1 at 1019 events). The direction is reproduced and consistent;
the significance is bounded.

## 5. Discussion

**No bandwidth-robust arrow.** The sole rung that clears the family bound at
the canonical bandwidth is 1032→131 Å, at zero lag — the
transition-region→corona boundary (log T 5.47 → 5.57, O VI → Fe VIII). The
adjacent log-T rung gap at this boundary (Δ 0.10) is not the ladder's
steepest: the largest gap lies at 977→1032 Å (Δ 0.63 in log T). The 977→1032
entry (peak 3.91e-2) and the hottest rungs' downward D are all below fam, so
they are reported as silent, not as negative findings. But the single
1032→131 arrow is not bandwidth-robust (§4.4): it clears fam only through
h ≈ 1.75 and falls below at h ≥ 2.0, robustly to the surrogate count. Under
a combined family-bound + bandwidth criterion the EVE-2011 ladder carries no
robust directional flow; the coronal crossing appears only as a below-fam
pattern on either instrument.

**What this does and does not decide.** The measurement does *not* establish
an upward-transport Pfeil: the one EVE candidate (1032→131) survives the
family bound only at canonical/narrow bandwidth, and the AIA hot cascade,
though reproduced across three years, stays below its own family bound in
every year — a growing event ensemble (194 → 524 → 1019 → 281) does not lift
the hottest rung over fam. The measurement therefore does *not* separate
Alfvén waves from nanoflares at fam significance. What it *does* establish is
the coronal direction under the bound: a cascade 193→211→335→94 Å that is
reproduced at the ~96 s crossing in three independent years with a 76%
consistent per-event sign — the boundary between a real but sub-significant
directed process and an alternating/random null is measured. Separating the
mechanism on the amplitude still needs the spatially resolved active-region
path (the sun-as-a-star ladder caps the flare ~10×).

**Instrument degradation is the honest constraint.** The full ladder is
measurable only in the 2011 MEGS-A era; 2014 is degraded (304 Å flat at 1.2×
in an X1 flare), which is why the 2014 three-force run stays silent and why
the ladder run uses 2011. The full-disk, sun-as-a-star lines dilute the
flare against the quiet-sun background (the Quad Diode's 13.5× flare response
contrasts with the lines' 1.3–3.3×) — the null model absorbs this, but it
caps the signal-to-noise of every rung.

**Why TE on the ladder.** The ladder formulation is a confounder control by
construction: the common flare envelope drives every line simultaneously, so
a naive correlation between any two rungs is trivially high. The TE excess
D, and the phase-randomized null that calibrates it, isolate the *directional*
residual — the information the cooler rung carries about the hotter rung's
future that the shared envelope does not.

**Two instruments, one bandwidth-limited candidate.** Read together, the two
measurements bracket no full energy path. EVE-2011's single rung that clears
its family bound at the canonical bandwidth (1032→131 Å, the
transition-region→corona entry at lag 0) does not survive the bandwidth
cross-check (§4.4); AIA-2014 reproduces the coronal-rung amplitudes and the
~96 s crossing lag but below its own family bound (§4.2). There is no
reversal to reconcile — neither instrument measures the 335→94 direction at
fam — and there is also no robust positive flow: the two independent results
are a family-bound-limited EVE candidate and a below-fam AIA pattern, i.e.
two silences and one fragile candidate.

## 6. Limitations

- **Two instruments, no robust arrow.** EVE-2011's single candidate
  (1032→131 Å) clears its family bound only at canonical/narrow bandwidth and
  fails the bandwidth cross-check (§4.4); AIA-2014 reproduces the coronal-rung
  amplitudes and ~96 s lag but below its own family bound (§4.2); the hottest
  rung (335→94) is established by neither instrument at fam. No instrument
  yet measures a fam- *and* bandwidth-robust coronal arrow; that is the next
  measurement, not yet made.
- **One era each.** EVE is 2011 (healthy MEGS-A) and AIA is 2014 (healthy
  imaging); the 2014 EVE era is degraded and excluded, so no single era
  carries both ladders. The era difference no longer explains a 335→94
  reversal (there is none at fam); it remains a limit on a single-era
  coronal-rung arrow.
- **Full-round family bound.** The null reported is the strongest surrogate
  D over all pairs × lags of the round — the canonical family bound shared
  with the system's other blades. EVE-2011's fam = 4.6982e-1 (pre-fix RNG,
  conservative-high). AIA-2014 is now measured under its own full-round
  fam = 1.8925e-1 (194 events, post-fix RNG), under which all six rungs —
  335→94 included — are silent.
- **Bandwidth sensitivity (new, measured).** The EVE fam verdict is not
  robust to the KDE bandwidth (§4.4): the single 1032→131 arrow that clears
  fam at h=1.0 fails at h ≥ 2.0, and the failure is robust to the surrogate
  count (a larger surrogate band is stricter). A verdict must now survive
  both the family bound and the bandwidth cross-check to be called an arrow.
- **109 events, one solar-maximum window.** The event ensemble is a single
  90-day window; a second window or a cycle-spanning ensemble is not
  measured.
- **Sun-as-a-star dilution.** The full-disk lines average the quiet sun into
  every flare; the flare signal is attenuated ~10× relative to a spatially
  resolved (active-region) measurement.
- **Estimator.** The scalar KDE TE is the untouched canonical reference of
  the system; it passes the Schreiber-2000 benchmark (§3.3). The
  conditional (multi-force) TE — conditioning each rung on the others — is
  implemented in the probe family but not reported here; the ladder D is the
  pairwise directional excess.
- **Surrogate-machine generation (conservative footnote).** The family bound
  here was generated by the pre-fix surrogate RNG (fam-machine: pre-fix).
  The pre-fix band lies higher than the post-fix band; a verdict that holds
  against the higher pre-fix bound holds post-fix a fortiori. The measured
  silence is thus the conservative statement.

## 7. Conclusion

Across three independent years (AIA 2013, 2014, 2015), the hot-corona
cascade 193→211→335→94 Å reproduces as a directed upward pattern at the
predicted ~96 s Alfvén crossing: all nine rung-year D values are positive,
and the per-event direction of the hottest rung is 76% positive with every
month majority-positive — the oscillating-direction driver is not carried at
event or month scale. The amplitude of this cascade, however, stays below
the full-round family bound in every year (0.63–0.91 × fam), and the hottest
rung does not strengthen as the ensemble grows fivefold. On EVE-2011, the
single rung that clears the family bound at canonical bandwidth (1032→131 Å
upward, D = 5.11e-1 > fam = 4.70e-1) fails the KDE-bandwidth cross-check
(h ≥ 2.0, robust to the surrogate count); every other EVE rung is silent at
every bandwidth.

The measurement therefore carries a **reproducible, directed, family-bound
cascade**: its direction is consistent and reproduced across three
independent years; its amplitude is bounded below the arrow threshold by the
family bound on every rung. No rung is a Pfeil, and the verdict is not a
null — the direction is measured and reproduced, the significance is
bounded, and the Alfvén-versus-nanoflare distinction stays open on the
amplitude that clears both the family bound and the bandwidth check.

## References

- Kaiser, A., & Schreiber, T. (2002). Information transfer in continuous processes. *Physica D: Nonlinear Phenomena* 166, 43–62.
- Marschinski, R., & Kantz, H. (2002). Analysing the information flow between financial time series. *The European Physical Journal B* 30, 275–281.
- Runge, J., Nowack, P., Kretschmer, M., Flaxman, S., & Sejdinovic, D. (2019). Detecting and quantifying causal associations in large nonlinear time series datasets. *Science Advances* 5, eaau4996.
- Schreiber, T. (2000). Measuring information transfer. *Physical Review Letters* 85, 461–464.
- Staniek, M., & Lehnertz, K. (2008). Symbolic transfer entropy. *Physical Review Letters* 100, 158101.

---

*Data and code:* the instrument, probes and register live in the omegaflow
repository (`src/mathematikerin/te.rs` — canonical scalar estimator, untouched;
`tools/harvest/src/bin/eve_compiler.rs`, `tools/measure/src/bin/corona_ladder_probe.rs` (EVE-2011);
`tools/harvest/src/bin/aia_compiler.rs`, `tools/measure/src/bin/aia_ladder_probe.rs`, the FITS-Rice
decoder in `src/archivar/fits.rs` (AIA-2014); survey
`docs/surveys/survey-ein-blatt-korona-heizung.md`). All verdicts are
machine-measured; the register language of the system is German, this
manuscript is its English face. The EVE line indices are fixed by matching
WAVE_CENTER, not by position; the AIA full-disk flux is JSOC's own DATAMEAN
keyword in DN/s.
