<!--
  title: Hunga Tonga 2022 — the Lamb wave measured cross-medium (Kwajalein ear, Kyoto barometer, BGR infrasound)
  class: paper
  date: 2026-09-17
  sha256: 27695df8610397d29a905b3d2fea325df99b0c63154c18c1346ec9944d3759ee
  status: live
  see-also: docs/handover/handover-2026-09-17-forschung-folge56.md
-->
# Hunga Tonga 2022 — the Lamb wave measured cross-medium

*Omegaflow Working Group — Forschung-Folge 56, 2026-09-17*

## Abstract

The 15 January 2022 eruption of Hunga Tonga-Hunga Ha'apai launched a Lamb wave
that propagated globally. We measure its arrival at three
independent sensors in one pass, in named arrival windows: the NOAA CO-OPS
Kwajalein station 1820000 (6-minute air pressure + water level, 3741.5 km from the
source), the Kyoto-A 1-Hz barometer (Zenodo record 8098323, 8043.7 km), and the
BGR IS52 infrasound array (−7.38°, 72.48°, 11981.5 km). At Kwajalein the window
pressure peak is +2.29 hPa at 07:42:00 UTC, 208 s after the predicted direct
arrival, and the coupling-window water trough is −0.659 m at 10:00:00, 8280 s after
the pressure peak; the window coupling is 3.48 hPa/m. At Kyoto-A the pressure
maximum is 1017.45 hPa at 11:38:02, 311 s after the predicted direct arrival. At
BGR IS52 the nearest back-azimuth detection
to the predicted direct arrival lies 1360 s later at 107.7° against the predicted
114.4° (residual −6.7°). The three channels are consistent with the direct Lamb
path; the global pressure extremes fall outside the window and are not the
coupling. The council seals the window values at the one ear (Kwajalein); the two
detection channels are reported unsealed.

## 1. The question

A Lamb wave is a grazing acoustic-gravity mode of the atmosphere. It travels at
the Lamb phase speed, ~0.306 km/s, and couples the air column to the water below:
the pressure pulse and the sea-surface response are the two faces of one
propagation. The cross-medium question is directional — does the measured
atmospheric arrival carry the water response at the same station, and does an
independent infrasound array detect the same arrival with the predicted
back-azimuth?

Handover 54 read the arrival with a global-extrema method: it took the global
pressure maximum and the global water minimum over the whole day and divided them.
The council struck that method (2026-09-17): the global pressure maximum
(1011.50 hPa at 2022-01-15 22:00:00) and the global water minimum are uncoupled,
tide-driven extremes hours from the predicted arrival; the ratio 3.67 hPa/m was not
a Lamb signal. The corrected method reads the extrema inside a *named* window
around the predicted arrival, and reports the global extremes separately as a
bound.

## 2. Data

- **Kwajalein ear.** NOAA CO-OPS station 1820000 (lat 8.731667°, lon 167.73611°),
  6-minute `air_pressure` and `water_level`, GMT. 960 pressure and 960 water
  samples. Great-circle distance to the source 3741.5 km.
- **Kyoto-A.** 1-Hz surface pressure, Zenodo record 8098323 (Kazama 2023),
  `data/220115.txt`, 86323 samples; station lat 35.02938°, lon 135.78347°,
  elevation 60.82 m. Great-circle distance to the source 8043.7 km.
- **BGR IS52.** The infrasound array back-azimuth detections, CDN asset
  `bgr_infrasound_IS52_2022.bin` (6 484 088 B); station lat −7.38°, lon 72.48°,
  derived from the bin. Great-circle distance to the source 11981.5 km.

The source is Hunga Tonga-Hunga Ha'apai (lat −20.536°, lon −175.382°); the water
start is 2022-01-15 04:14:45 UTC. The Lamb phase speed is 0.306 km/s.

## 3. Method

Each channel is read inside a window named in the probe
(`tools/measure/src/bin/tonga_lamb_crosscheck_probe.rs`):

- The **arrival window** is the predicted direct arrival ±7200 s
  (`ARRIVAL_WINDOW_S`).
- The **coupling window** is the measured pressure peak +21600 s
  (`COUPLING_WINDOW_S`), so the water response is read after the air pulse, not
  against it.
- The **global extremes** are reported separately and marked "not the coupling".

The predicted direct arrival is `water_start + distance / Lamb_speed`. The BGR
back-azimuth is compared against the great-circle bearing from the station to the
source.

## 4. Measurement

### Kwajalein ear

- predicted direct Lamb arrival: 2022-01-15 07:38:32 UTC
- arrival window: 05:38:32 … 09:38:32 UTC
- pressure: baseline 1009.01 hPa → window peak 1011.30 hPa (+2.29) at 07:42:00;
  measured − predicted = **208 s**
- global pressure bound (not the coupling): minimum 1006.80 hPa at 2022-01-14
  03:36:00; maximum 1011.50 hPa at 2022-01-15 22:00:00
- coupling window: 07:42:00 … 13:42:00 UTC
- water: baseline 0.099 m → coupling-window trough −0.560 m (−0.659) at 10:00:00;
  measured − predicted = 8488 s
- measured air→water lag (trough − peak): **8280 s**
- coupling (window extrema, one ear): **3.48 hPa/m** [Δp 2.29 hPa / Δh 0.659 m]

### Kyoto-A

- predicted direct Lamb arrival: 2022-01-15 11:32:51 UTC
- measured pressure maximum: 1017.45 hPa at 2022-01-15 11:38:02;
  measured − predicted = **311 s**
- measured pressure minimum (later): 1013.87 hPa at 2022-01-16 02:37:14

### BGR IS52

- predicted back-azimuth (station → source): 114.4°
- predicted direct arrival: 2022-01-15 15:07:20 UTC (azim 114.4°)
- nearest back-azimuth detection to the direct arrival: 2022-01-15 15:30:00,
  dt = 1360 s, azim 107.7°, residual −6.7°
- the antipodal and multi-lap paths do not resolve against the detection list
  (residuals −176.7°, 154.6°, −20.8°)

## 5. The Siegel

The council (2026-09-17) ruled: **Blatt ja, Siegel ja — mit benannter Grenze.**
The Siegel covers only the window values at the one ear (Kwajalein): the +2.29 hPa
window peak at 208 s and the −0.659 m coupling-window trough at 8280 s after the
peak, coupling 3.48 hPa/m. The two independent detection channels (the Kyoto-A
waveform maximum at 311 s, the BGR IS52 back-azimuth detection at 1360 s /
−6.7°) are reported unsealed. The global-extrema method of Handover 54 stays
struck: 3.67 hPa/m was not a Lamb signal.

The over-printed line "220115.txt absent from the Zenodo archive" from the
pre-correction probe is replaced here by the measurement: the archive carries
`data/220115.txt` (the member list reads `data/220102.txt` … `data/220131.txt`;
`data/220101.txt` is absent, the Zenodo description names the 1 January 2022
recording failure). The absent file was a probe key error (the `data/` prefix),
not an archive gap.

## 6. Named pending

- **One ear.** The coupling 3.48 hPa/m is read at a single station; a second
  co-located air+water pair is not measured. The seal covers the one ear only.
- **BGR window width.** The direct detection lies 1360 s after the prediction;
  the probe reads the nearest detection, not a matched-filter arrival. A tighter
  arrival estimate needs the array's own slowness/azimuth solution.
- **Kyoto baseline.** The Kyoto reading is the day's pressure extrema (maximum
  1017.45 hPa, minimum 1013.87 hPa); no pre-arrival baseline is built, so the
  maximum is not reported as an anomaly.

## Data and code

- Probe: `tools/measure/src/bin/tonga_lamb_crosscheck_probe.rs`
- Workflow: `.github/workflows/tonga-lamb-crosscheck.yml` (CI run 35188454719,
  artifact `tonga-lamb-crosscheck`, SHA ce260038)
- Sources: `phi/sources.φ` (`zenodo.org` record 8098323 as `kyoto_pressure`;
  `download.bgr.de` asset `bgr_infrasound_IS52_2022.bin`)
