<!--
  title: Hunga Tonga 2022 — the Lamb wave measured cross-medium (two CO-OPS ears, Kyoto barometer, BGR infrasound)
  class: paper
  date: 2026-09-17
  sha256: c5445f387e88096791169295bcdc8585bb633a4a90447c5e48f7b44ea579902a
  status: live
  see-also: docs/handover/handover-2026-09-17-forschung-folge57.md
-->
# Hunga Tonga 2022 — the Lamb wave measured cross-medium

*Omegaflow Working Group — Forschung-Folge 56, 2026-09-17*

## Abstract

The 15 January 2022 eruption of Hunga Tonga-Hunga Ha'apai launched a Lamb wave
that propagated globally. We measure its arrival at three independent sensor
classes in one pass, in named arrival windows: the NOAA CO-OPS co-located
air-pressure + water-level pairs (Kwajalein 1820000, 3741.5 km; Wake Island
1890000, 4843 km; Guam Apra Harbor 1630000, 5775 km; Midway Sand Island 1619910,
5425 km; Kahului Maui 1615680, 5045 km), the Kyoto-A 1-Hz barometer (Zenodo
record 8098323, 8043.7 km), and the BGR IS52 infrasound array (−7.38°, 72.48°,
11981.5 km). At two independent ears the window pressure peak and the
coupling-window water trough are measured with the same pressure residual at
different distance: Kwajalein +2.29 hPa at +208 s with a −0.659 m trough 8280 s
later, Wake Island +3.43 hPa at +208 s with a −0.332 m trough 7200 s later. The
window coupling ratio is station-local, not a constant: 3.48 hPa/m (Kwajalein),
10.34 hPa/m (Wake), 5.86 hPa/m (Guam), 4.09 hPa/m (Midway), 9.27 hPa/m
(Kahului) — a measured spread of 3.48–10.34 hPa/m across the five physical-sign
stations. At Kyoto-A the pressure maximum is 1017.45 hPa at 11:38:02, 311 s after
the predicted direct arrival. At BGR IS52 the nearest back-azimuth detection to
the predicted direct arrival lies 1360 s later at 107.7° against the predicted
114.4° (residual −6.7°; vapp 352 m/s → slowness 2.84 s/km). The channels are
consistent with the direct Lamb path; the global pressure extremes fall outside
the window and are not the coupling. The council seals the cross-medium
mechanism at two independent ears, the Kyoto-A detection, and the BGR IS52 array
detection; no single coupling ratio stands.

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
- **Second and further CO-OPS ears.** The same 6-minute `air_pressure` +
  `water_level` products, GMT, the same named windows. Wake Island 1890000
  (lat 19.290556°, lon 166.6175°), 4843 km, bearing 335.0°; Guam Apra Harbor
  1630000 (lat 13.443389°, lon 144.65636°), 5775 km, bearing 307.5°; Midway
  Sand Island 1619910 (lat 28.211666°, lon −177.36°), 5425 km; Kahului Maui
  1615680, 5045 km. Five Hawaii stations carry a pressure arrival whose
  coupling-window water rose (sign not physical): Honolulu 1612340, Nawiliwili
  1611400, Mokuoloe 1612480, Kawaihae 1617433, Hilo 1617760. Absent: Pearl
  Harbor 1612401 (pressure and water both void), Pago Bay 1631428 (air void),
  Pago Pago 1770000 (air void).
- **Kyoto-A.** 1-Hz surface pressure, Zenodo record 8098323 (Kazama 2023),
  `data/220115.txt`, 86323 samples; all 30 archive members
  `data/220102.txt` … `data/220131.txt` parse (`data/220101.txt` is absent, the
  1 January 2022 recording failure named in the Zenodo description); station
  lat 35.02938°, lon 135.78347°, elevation 60.82 m. Great-circle distance to the
  source 8043.7 km.
- **BGR IS52.** The infrasound array back-azimuth detections, CDN asset
  `bgr_infrasound_IS52_2022.bin` (6 484 088 B); the bin carries the apparent
  velocity component (comp 2) alongside the back-azimuth; station lat −7.38°,
  lon 72.48°, derived from the bin. Great-circle distance to the source
  11981.5 km.

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
- coupling (window extrema): **3.48 hPa/m** [Δp 2.29 hPa / Δh 0.659 m]

### Wake Island ear

- predicted direct Lamb arrival: 2022-01-15 08:38:52 UTC
- pressure: baseline 1013.87 hPa → window peak 1017.30 hPa (+3.43) at 08:42:00;
  measured − predicted = **208 s**
- coupling window: 08:42:00 … 14:42:00 UTC
- water: baseline 0.100 m → coupling-window trough −0.232 m (−0.332) at 10:42:00
- measured air→water lag (trough − peak): **7200 s**
- coupling (window extrema): **10.34 hPa/m** [Δp 3.43 hPa / Δh 0.332 m]

### Guam Apra Harbor ear

- predicted direct Lamb arrival: 2022-01-15 09:29 UTC
- pressure: baseline 1011.47 hPa → window peak 1014.30 hPa (+2.83) at 09:30;
  measured − predicted = **42 s**
- coupling window: 09:30 … 15:30 UTC
- water: baseline 0.085 m → coupling-window trough −0.398 m (−0.483) at 14:48
- measured air→water lag (trough − peak): **19080 s**
- coupling (window extrema): **5.86 hPa/m** [Δp 2.83 hPa / Δh 0.483 m]

### Midway Sand Island

- predicted direct Lamb arrival: 2022-01-15 09:10 UTC
- pressure: window peak +0.50 hPa above baseline; measured − predicted = **467 s**
- coupling (window extrema): **4.09 hPa/m**

### Kahului Maui

- predicted direct Lamb arrival: 2022-01-15 08:49 UTC
- pressure: window peak +2.09 hPa above baseline; measured − predicted = **628 s**
- coupling (window extrema): **9.27 hPa/m**

### Hawaii stations — pressure arrival, water sign not physical

Five stations carry a pressure arrival in the named window, but the
coupling-window water rose (a rising trough is not a physical Lamb response,
0 honored — the coupling is not formed):

- Honolulu 1612340: measured − predicted = 688 s
- Nawiliwili 1611400: 661 s
- Mokuoloe 1612480: 635 s
- Kawaihae 1617433: 454 s
- Hilo 1617760: 434 s

### Absent stations

- Pearl Harbor 1612401: pressure and water both void
- Pago Bay 1631428: air void
- Pago Pago 1770000: air void

### Kyoto-A

- predicted direct Lamb arrival: 2022-01-15 11:32:51 UTC
- 29-day pre-arrival pooled baseline: **1012.665 hPa** (mean of daily means
  1012.666 hPa, median 1012.881 hPa)
- measured pressure maximum: 1017.45 hPa at 2022-01-15 11:38:02;
  measured − predicted = **311 s**
- anomaly vs pooled baseline: **+4.79 hPa** (synoptic weather)
- anomaly vs the day's local 2-h pre-arrival baseline (07:32:51 … 09:32:51 UTC,
  mean 1014.88 hPa): **+2.57 hPa** = 1.44 sd; 2 of 29 ordinary days exceed it
- pulse: **+0.6–0.7 hPa above the immediate ambient**
- pulse shape (named from the reported sequence): rise from 1016.3–1016.8 hPa at
  11:30:00 to the 1017.45 hPa maximum at 11:38:02 (rise ≈ 480 s), decay to
  ~1015.4 hPa by 11:49:30 (decay ≈ 690 s)
- measured pressure minimum (later): 1013.87 hPa at 2022-01-16 02:37:14

### BGR IS52

- predicted back-azimuth (station → source): 114.4°
- predicted direct arrival: 2022-01-15 15:07:20 UTC (azim 114.4°)
- nearest Tonga-consistent detection: 2022-01-15 15:30:00, dt = **1360 s**,
  azim 107.7°, residual −6.7°
- apparent velocity vapp 352.13 m/s → slowness **2.84 s/km** (Lamb 306 m/s →
  3.27 s/km); a_rms 0.0109 Pa; freq 1.03 Hz
- the antipodal and multi-lap paths do not resolve against the detection list
  (residuals −176.7°, 154.6°, −20.8°)
- the PMCC product is a detection list on a ~300 s grid: a sub-300 s
  matched-filter arrival is not recoverable; the raw waveform is vDEC
  account-blocked (a measured access state)

## 5. The Siegel

The council (2026-09-17, second ruling) rules: **Blatt ja, Siegel ja — mechanism,
not ratio.** The seal covers the cross-medium mechanism at two independent ears:
Kwajalein 1820000 (window pressure peak +2.29 hPa at +208 s; coupling-window water
trough −0.659 m at 8280 s after the peak) and Wake Island 1890000 (peak +3.43 hPa
at +208 s; trough −0.332 m) — identical pressure residual at different distance
(3741.5 / 4843 km) and azimuth. The coupling ratio is station-local, not a
constant: 3.48 hPa/m (Kwajalein), 10.34 hPa/m (Wake), measured spread
3.48–10.34 hPa/m across the five physical-sign stations (Guam 5.86 at +42 s,
Midway 4.09, Kahului 9.27). At the five Hawaii stations (Honolulu, Nawiliwili,
Mokuoloe, Kawaihae, Hilo) the pressure arrival is present but the coupling-window
water rose — sign not physical. The mechanism seal is bounded by the measured
station set; the spread, the sign inversion, and the water lag are read as the
local water-column response (interpretation, no model built). No single ratio
stands for the cross-medium coupling. Kyoto-A is sealed as a detection at +311 s:
pulse +0.6–0.7 hPa above immediate ambient against the 29-day baseline (pooled
mean 1012.665 hPa); the window maximum 1017.45 hPa is not an amplitude anomaly
(1.44 sd vs the day's 2-h pre-arrival baseline; 2 of 29 ordinary days exceed it)
— the discrimination is carried by timing and pulse shape. BGR IS52 is sealed as
the first Tonga-consistent array detection at 15:30:00 (azim 107.7° vs 114.4°
predicted, residual −6.7°; vapp 352 m/s → slowness 2.84 s/km vs Lamb 3.27 s/km);
the 1360 s residual is the PMCC grid's nearest detection, not a matched-filter
arrival. The global-extrema method of Handover 54 stays struck.

## 6. Named pending

- **BGR matched-filter arrival.** The PMCC detection list runs on a ~300 s grid;
  a sub-300 s arrival estimate is not recoverable from it. The raw waveform is
  account-blocked (vDEC) — a measured access state, not a verdict. Next step:
  none before access is granted.
- **Kyoto pulse shape.** The shape metric that carries the Kyoto discrimination
  (rise time, width) is named in §4; the seal sentence stands with it. *(This
  entry falls away in the same atom the §4 shape line lands — it is a build
  condition, not a standing pending.)*

## Data and code

- Probe: `tools/measure/src/bin/tonga_lamb_crosscheck_probe.rs`
- Workflow: `.github/workflows/tonga-lamb-crosscheck.yml` (CI run 35188454719,
  artifact `tonga-lamb-crosscheck`, SHA ce260038)
- Sources: `phi/sources.φ` (`zenodo.org` record 8098323 as `kyoto_pressure`;
  `download.bgr.de` asset `bgr_infrasound_IS52_2022.bin`)
- The wider CO-OPS station set (Midway, Kahului, the five Hawaii stations, the
  three absent stations) is measured in Forschung-Folge 56, wave 1, against the
  same named windows.
