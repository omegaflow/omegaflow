<!--
  title: Befund — the pick-noise lever: the weighted joint fit moves the Zonen-Flotte mean to −6.0 km (from −61.7), but the deep-zone correlation surface is systematically bimodal (runner-up 0.88–1.00) — the pick is ambiguity-averaged, not sharp; the lever is exhausted at this band
  class: befund
  date: 2026-09-09
  sha256: 13acbbc397e8a47516422816c40aa786c5e38400535395a9fb2ca4375a046c94
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-zonen-flotte-folge.md docs/befund/befund-2026-09-09-zonen-flotte-delta-gate.md docs/befund/befund-2026-09-09-zonen-flotte-kante.md
-->

# Befund: the pick-noise lever — the mean moved, the ambiguity named

## Frage & Bindung

The register line of the Zonen-Flotte follow-up handover: "Rest-Streuung (88,6 km) — Pick-Rauschen (Lag-Tiefen-Steigung 0,16–0,41 s/5 km); √N trägt langsam." The council fixed the architecture: build A (sub-sample lag) + B (weights from the measured peak half-width, weighted joint inversion), carry C (runner-up) as a measurement — no ambiguity gate before the runner-up distribution is measured.

## Was gebaut wurde

- `correlate_window` now measures the full |corr| surface: parabolic refinement around the maximum (sub-sample lag, δ clamped to ±0.5 samples); peak half-width (half-max threshold, linear interpolation between samples on both sides; width below 1 sample → absent; edge/window-boundary pick → absent, never 0.0); runner-up ratio (highest local maximum outside the flank |j − k*| > 1).
- `peak_sigma_s`: σ = the measured half-width; single-sample peak → the honest sample floor 1/rate; edge pick → absent.
- `invert_depth_weighted` next to the untouched equal-weight `invert_depth_multi`: squared residuals × w with w = 1/σ²; non-finite weights → Absent.
- The fleet bin prints both statistics: the per-station median (baseline against the Δ-gate Befund) stays; new are the weighted joint inversion per event (σ-absent stations excluded by name, never fabricated) and the runner-up ratio per station as a measurement.
- Kalibrier-Gate (24 tests, te.rs discipline): sub-sample beats the nearest sample; equal height → the narrow peak carries the larger weight; uniform weights reproduce invert_depth_multi exactly; the sharp station dominates the weighted fit; the runner-up measures the distant peak, not the flank; a single-sample peak carries the floor; an edge pick carries no σ; polarity inversion carries the same surface (within one sample — measured: the base coda, ~3 % of the pick, shifts the argmax sample between polarities; the |corr| surface is no exact sign mirror).

## Die Messung (fleet re-run, the same 16 events, Zonen box, both gates active)

| Größe | Δ-Gate run | re-run |
|---|---|---|
| with-clamp mean | −14.6 km | −14.7 km |
| after-exclusion mean | −61.8 km | −61.7 km |
| station scatter (median per event) | 88.6 km | 88.6 km |
| weighted joint inversion | — | −6.0 km (11 events, sd 59.2, se 17.9) |

The baseline reproduces (±0.1 km; the 64 Δ-gate skips identical) — the instruments are orthogonal to the two gates. The sub-sample lag shifts individual station depths by ≤ 1 km.

## Die Befunde

1. The lever moved the mean: −61.7 → −6.0 km. The flat bias of the per-station median was the pick error. Per event, the joint depth sits closer to the catalog on 8 of 11 events (usb000ruzk cat 615.4: 635 instead of 494; us70005axg cat 591.0: 638 instead of 471), farther on 1 (us1000h4l1 cat 550: 590 instead of 548), level on 1 (usp000ae2r); 1 event carries a single station (usp000e9aq: 644).
2. The wall stands in the joint statistic too: us10006scr (catalog 596.4) clamps joint at the 660 wall — named, not counted; the edge stays the edge.
3. The surface is systematically bimodal at deep geometry: runner-up ratios 0.88–1.00 at essentially every station. The max-peak pick is an ambiguity average, not a sharp location — the −6.0 km is the centroid of a bimodal ambiguity cloud, rescued by the symmetry of the pP/sP confusion (council verdict). The sP confound the council named (IC.XAN, befund-tiefenphasen-polaritaet) is now the measured norm.
4. The lever is exhausted at this station band: the instrument moved the mean, not the event-to-event sd (59.2 vs 61.7 km) — the residual scatter is catalog truth / event terms, no longer pick noise (the council's third confound now carries the evidence). The next lever is not a pick instrument: external reference (TauP/KEB95) and the dual-phase inversion (sP-joint, follow-up atom).
5. An ambiguity gate at the measured distribution (0.88–1.00) would skip practically the whole fleet — no gate; the runner-up ratio stays a measured feature, not a gate trigger (council, unanimous).

## Benannt (Pendings, not still)

- sP-joint inversion of both phases (pP + sP, ak135 sP-lag separation) — follow-up atom, registered.
- External depth reference (TauP/KEB95) — pending, instrument named.
- The 660 wall stays unresolved (the joint statistic clamps equally).

## Verdikt

The weighted joint fit recovers an unbiased mean (−6.0 km), but the deep-zone correlation surface is systematically bimodal (runner-up 0.88–1.00) — the pick is ambiguity-averaged, not sharp; the pick-noise lever is exhausted and the next instrument is dual-phase joint inversion.
