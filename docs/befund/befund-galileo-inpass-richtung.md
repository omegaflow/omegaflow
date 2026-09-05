<!--
  title: Befund — In-Pass-Richtung: Floor↔Rauschen simultan, kein anhaltender Floor→Rauschen-Pfeil
  class: befund
  date: 2026-09-05
  sha256: 9ba2a69627c63050f4fc8672cadc8bc9ea0911c62cb7e3e26a2369a2edd174ad
  status: done
  antwortet-auf: docs/befund/befund-galileo-inpass-staerke-rampe.md
-->
# Befund — Galileo in-pass floor direction (station 43/63, mode 1)

date: 2026-09-05
source data: data/galileo_resid.bin (GASR; [0]=tdb [1]=resid_hz [2]=station [3]=mode [7]=signal_strength)
probe: tools/measure/src/bin/galileo_floor_onset_lead.rs (new, additive; cargo check 0 errors 0 warnings)
prior: tools/measure/src/bin/galileo_pass_strength_ramp.rs found a genuine floor-vs-plateau noise covariance at stations 43/63 within dual passes.

## Question
Is the within-pass AGC-floor / noise covariance directional — the floor state (signal_strength == -2560) PRECEDES and DRIVES a louder noise within a pass — or simultaneous state covariance?

## Method
- pass = contiguous tracking arc per station, boundary = time gap between consecutive samples > 600 s.
- tracked sample = non-lock (|resid| <= 1000 Hz) and strength != 0; arcs split at a lock/null sample between tracked samples or a gap > 120 s.
- floor = strength <= -2560 (AGC clamp), plateau = strength >= -1900; dual pass = both a plateau run and a floor run, each >= 30 tracked samples (the ramp covariance regime).
- floor onset = first tracked sample of a floor run preceded by an in-arc non-floor tracked sample; only onsets NOT at the pass/arc start (so a pre-onset window exists) are usable.
- noise = RMS of resid about the window mean; pre window = up to 60 tracked samples of the non-floor run immediately before onset (>= 30 required); post window = up to 60 tracked samples of the floor run from the onset (>= 30 required).

## Measured
Per station (mode 1; onsets interior to dual passes):

| station | usable onsets | med noise BEFORE (Hz) | med noise AFTER (Hz) | med delta (Hz) | frac AFTER > BEFORE |
|---|---|---|---|---|---|
| 43 | 56 | 0.712 | 1.316 | +0.201 | 0.607 |
| 63 | 13 | 0.452 | 0.707 | +0.054 | 0.615 |

pooled: 69 usable onsets | med_pre 0.698 | med_post 1.279 | med_delta +0.121 | mean_delta 23.9 (tail-dominated) | AFTER>BEFORE 42/69 = 0.609 | med_ratio 1.28.

Pooled 30-sample noise profile vs onset offset (median RMS; window starts at offset):
- -90: 0.301  (n 63)
- -60: 0.354  (n 64)
- -30: 0.454  (n 69)
-   0: 0.753  (n 69)
- +30: 0.550  (n 56)
- +60: 0.466  (n 55)
- +90: 0.330  (n 50)

Dual-pass floor runs that could NOT be tested for a pre-onset window (the measured limit on coverage): station 43: 84 floor runs >= 30 samples, of which 22 begin at an arc start (5 at the very first tracked sample of the pass) and 6 had an adjacent non-floor run < 30 samples — 56 fully usable. Station 63: 44 floor runs, 30 at an arc start (3 at pass start), 1 pre-short — 13 usable. Floor onset is NOT almost always at pass start; pre-onset windows exist for the majority, so a verdict is measurable.

## Verdict
SIMULTANEOUS COVARIANCE, not a sustained floor->noise lead.

Basis (measured): the median onset raises noise only modestly (median ratio AFTER/BEFORE 1.28, AFTER>BEFORE in 61 % of onsets); the pooled profile shows the noise already rising across the plateau BEFORE the onset (-90..-30: 0.301 -> 0.454) and returning to the pre-onset level by ~+90 samples inside long floor runs (peak 0.753 at the onset window -> 0.330) — the loud phase is a transient centered on the plateau->floor transition, it does not "stay elevated" through the steady floor state, and it begins before the clamp is reached. That is state/fade covariance, not a PLL-like floor-driven arrow.

Limit named: the +90 decay (0.330, n=50) comes from a DIFFERENT subpopulation than the onset peak (n=69) — the return to the pre-onset level is measured only on floor runs long enough to carry a +90 window (the profile n falls from 69 at onset to 50 by +90 as shorter floor runs end). The decay magnitude is a property of the long-floor subset, not of the pooled onsets.

Reconciliation with the same-day pairing (G3, befund-galileo-sameday-floor-strong-paarung): Der Boden ist ein genuiner angehobener-Rausch-Zustand (gemessen bei fixem Tag, Same-Day-Paarung) ohne Floor→Rauschen-Pfeil (diese Messung) — die Lautheit ist ein Zustands-/Übergangs-Marker, kein Treiber.

Directional minority (0 honored, named): 13 of 69 onsets (10 at station 43, 3 at station 63; pass-end / loss-of-lock events on 1997-01-04, 01-11, 01-17, 01-18, 01-25, 01-29, 01-31, 02-11 and 1997-01-09, 01-16, 02-02) show post-onset noise of 11-316 Hz from quiet plateaus (pre < 7 Hz) — a genuine floor-first explosion. These carry the pooled mean (mean_delta 23.9 Hz) and dominate the earlier floor-vs-plateau contrast, but they are the tail, not the median mechanism.
