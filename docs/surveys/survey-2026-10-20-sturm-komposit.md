<!--
  title: Survey — Sturm-Komposit: welche Fäden sich um die Sturmonsetzeit verformen (2026-10-20)
  class: survey
  date: 2026-10-20
  sha256: bb293a2996ba5950bcf470d5b07679b3015be5e5491bb53fc87e330f74f8bdb1
  status: live
  see-also: docs/surveys/survey-2026-10-20-netzausfall-kante.md docs/surveys/survey-2026-10-20-sonne-erde-kette.md tools/measure/src/bin/sturm_komposit_probe.rs phi/sources.φ
-->
# Survey — Sturm-Komposit: welche Fäden sich verformen

**Anlass:** Nicht „trägt Kante X?", sondern die Rangliste: welche Fäden bewegen
sich um die Sturmonsetzeit, wie stark, und mit welchem Lag? Der Survey legt die
Schwelle **vor** der Messung fest (Dst-Minimum < −50 nT), misst jeden Faden als
Superposed Epoch gegen eine ruhige Baseline und stellt dieselbe Prozedur auf
zufällige ruhige Fenster als Null daneben. Ein `absent` wird benannt, nicht als
Null gefüllt.

**Fenster:** 2014-11-02 .. 2023-12-02 (Überlappung mit ECOD-Ausfällen und
FMI-GIC). **Ereignisdefinition:** erste Stunde mit Dst < −50 nT; Läufe mit
Lücken ≤ 3 h werden zusammengefasst, Ereignisse mit Onset-Abstand < 48 h
verschmolzen. Das Dst-Minimum der so gebildeten Episode steht in der Liste.
**Keine willkürliche Auswahl:** die Schwelle stand vor der Messung.

**Probe:** `tools/measure/src/bin/sturm_komposit_probe.rs` (neu, dieser Survey).
Sie liest die lokalen Bin-Assets; die WDC-Daten bleiben lokal unter
`sources/wdc.kugi.kyoto-u.ac.jp/`. Es gab **keinen CDN-Upload**.

## Daten und Abdeckung

| Faden | Asset (lokal) | Abdeckung | Kadenz | n Ereignisse |
|---|---|---|---:|---:|
| Sonnenwind Bz | `cdaweb.gsfc.nasa.gov/compiled/omni2_1h_2014_2023.bin` | 2014-01-01 .. 2023-12-31 | 1 h | 166 |
| Sonnenwind Speed | dito (`V1800`) | dito | 1 h | 166 |
| Sonnenwind Dichte | dito (`N1800`) | dito | 1 h | 166 |
| AE / AL / AU | `wdc.kugi.kyoto-u.ac.jp/wdc_ae.bin` | 2014-01-01 .. 2023-12-31 | 1 min → 1 h | 166 |
| Dst | dito | dito | 1 h | 166 |
| Sym-H | dito | dito | 1 min → 1 h | 166 |
| Boden ABK dB/dt | `imag-data.bgs.ac.uk/abk_dbdt_1h.bin` | 1994-01-01 .. 2026-09-04 | 1 h | 166 |
| Boden SOD dB/dt | `imag-data.bgs.ac.uk/sod_dbdt_1h.bin` | 1994-01-01 .. 2026-08-31 | 1 h | 166 |
| Strom FMI GIC | `space.fmi.fi/compiled/fmi_gic.bin` | 1999-01-01 .. **2023-09-30** | 1 h | **118** |
| Ausfälle ECOD | `data.openei.org/grid_outage_events.bin` | 2014-11-02 .. 2023-12-02 | Tag | **135** |
| TEC | `cdaweb.gsfc.nasa.gov/cdaweb_tec_2024_05.bin` | **nur Mai 2024** | nativ | **absent** |
| Polarlicht | THEMIS/PGM nur 2024-05-10, SSUSI nicht im Haus | **nur Mai 2024** | — | **absent** |

WDC: 10.605.408 Records (604.508.264 B), neu lokal über
`wdc_ae_compiler` aus WDC-HAPI geharvestet (`min_ae`, `min_asysym`,
`hour_dst`), Lizenz `wdc-kyoto-scientific-noncommercial`, kein Upload.
ECOD: 1.575.135 Records = 3 Komponenten × 525.045 Zeilen; nach Dedup auf
`(Start, County)` bleiben **167.772** Ausfälle.

**Zeitbasen:** WDC-Zeitstempel sind Unix/UTC; OMNI und ECOD tragen TDB und
werden mit `+J2000 − 69 s` (TT−UTC im Fenster) auf Unix gelegt — bei 1-h-Bins
ist der 69-s-Versatz benannt, aber vernachlässigbar. ECOD-Zeitstempel sind
**lokal**, nicht UTC. Die Probe korrigiert sie näherungsweise über die
County-Länge (`UTC ≈ lokal − round(lon/15) h`); der mediane Versatz im Fenster
ist **−6 h**. Das ist eine Längen-Näherung an die gesetzliche Zeitzone, keine
exakte Zonen-Auflösung (siehe Riss 3).

## Methode

1. **Baseline:** ruhige Stunden mit Dst > −20 nT, die nicht innerhalb von
   ±10 Tagen um ein Ereignis liegen. Im Fenster bleiben **35.299** solcher
   Stunden (von 87.648). z = (x − Baseline-Mittel)/Baseline-Streuung.
2. **Superposed Epoch:** Lags **−24 h … +72 h** um die Onsetzeit. Pro Lag wird
   der Median über die Ereignisse gebildet; Bootstrap mit **1.000** Resamples
   liefert das 95-%-Intervall des Peak-|z|.
3. **Verformungsmaß:** Peak-|z| der Mediankurve, Lag des Peaks, Fläche unter
   |z| über das Lagfenster, Zahl der Ereignisse mit Daten am Faden.
4. **Null:** dieselbe Prozedur auf **500** Ziehungen zufälliger ruhiger
   Startstunden (gleiche n, keine Stürme); 95-%-Quantil des Peak-|z|. Der
   Vergleich ist „über/unter Null", kein p-Wert.
5. **Tagesfäden (ECOD):** Lags werden auf Tagesbins abgebildet; ein Lag von
   −24 h heißt Tag −1. Die Tagesauflösung ist gröber als die Stundenfäden.

## Ereignisliste (166)

| # | Onset (UTC) | Dst-Min (UTC) | Dst-Min [nT] |
|---:|---|---|---:|
| 1 | 2014-02-18T23:00Z | 2014-02-19T08:00Z | -119 |
| 2 | 2014-02-21T00:00Z | 2014-02-22T01:00Z | -64 |
| 3 | 2014-02-23T19:00Z | 2014-02-23T19:00Z | -55 |
| 4 | 2014-02-27T20:00Z | 2014-02-27T23:00Z | -97 |
| 5 | 2014-04-11T23:00Z | 2014-04-12T09:00Z | -87 |
| 6 | 2014-04-30T06:00Z | 2014-04-30T09:00Z | -67 |
| 7 | 2014-08-27T10:00Z | 2014-08-27T18:00Z | -79 |
| 8 | 2014-09-12T22:00Z | 2014-09-12T23:00Z | -88 |
| 9 | 2014-10-09T07:00Z | 2014-10-09T07:00Z | -51 |
| 10 | 2014-10-20T17:00Z | 2014-10-20T17:00Z | -51 |
| 11 | 2014-10-28T00:00Z | 2014-10-28T01:00Z | -57 |
| 12 | 2014-11-10T17:00Z | 2014-11-10T17:00Z | -65 |
| 13 | 2014-11-16T06:00Z | 2014-11-16T07:00Z | -59 |
| 14 | 2014-12-12T16:00Z | 2014-12-12T16:00Z | -53 |
| 15 | 2014-12-22T04:00Z | 2014-12-22T05:00Z | -71 |
| 16 | 2014-12-24T22:00Z | 2014-12-26T01:00Z | -57 |
| 17 | 2015-01-04T13:00Z | 2015-01-04T21:00Z | -78 |
| 18 | 2015-01-07T09:00Z | 2015-01-07T11:00Z | -107 |
| 19 | 2015-02-02T06:00Z | 2015-02-02T06:00Z | -55 |
| 20 | 2015-02-17T22:00Z | 2015-02-18T00:00Z | -69 |
| 21 | 2015-02-24T03:00Z | 2015-02-24T07:00Z | -58 |
| 22 | 2015-03-01T06:00Z | 2015-03-02T08:00Z | -64 |
| 23 | 2015-03-17T08:00Z | 2015-03-17T22:00Z | -234 |
| 24 | 2015-03-22T07:00Z | 2015-03-22T07:00Z | -56 |
| 25 | 2015-04-10T04:00Z | 2015-04-11T09:00Z | -85 |
| 26 | 2015-04-15T11:00Z | 2015-04-16T23:00Z | -88 |
| 27 | 2015-05-11T03:00Z | 2015-05-11T04:00Z | -57 |
| 28 | 2015-05-13T05:00Z | 2015-05-13T06:00Z | -82 |
| 29 | 2015-06-08T07:00Z | 2015-06-08T08:00Z | -67 |
| 30 | 2015-06-22T20:00Z | 2015-06-23T04:00Z | -198 |
| 31 | 2015-06-24T22:00Z | 2015-06-25T16:00Z | -81 |
| 32 | 2015-07-04T21:00Z | 2015-07-05T05:00Z | -74 |
| 33 | 2015-07-13T08:00Z | 2015-07-13T15:00Z | -68 |
| 34 | 2015-07-23T06:00Z | 2015-07-23T08:00Z | -72 |
| 35 | 2015-08-15T12:00Z | 2015-08-16T07:00Z | -98 |
| 36 | 2015-08-19T04:00Z | 2015-08-19T06:00Z | -64 |
| 37 | 2015-08-23T08:00Z | 2015-08-23T08:00Z | -57 |
| 38 | 2015-08-26T09:00Z | 2015-08-27T20:00Z | -103 |
| 39 | 2015-09-07T16:00Z | 2015-09-09T12:00Z | -105 |
| 40 | 2015-09-11T09:00Z | 2015-09-11T14:00Z | -87 |
| 41 | 2015-09-20T08:00Z | 2015-09-20T15:00Z | -81 |
| 42 | 2015-10-04T05:00Z | 2015-10-04T09:00Z | -63 |
| 43 | 2015-10-06T19:00Z | 2015-10-07T22:00Z | -130 |
| 44 | 2015-10-09T01:00Z | 2015-10-09T21:00Z | -57 |
| 45 | 2015-10-12T18:00Z | 2015-10-14T05:00Z | -53 |
| 46 | 2015-10-18T09:00Z | 2015-10-18T09:00Z | -56 |
| 47 | 2015-11-03T12:00Z | 2015-11-04T12:00Z | -56 |
| 48 | 2015-11-07T04:00Z | 2015-11-07T06:00Z | -87 |
| 49 | 2015-11-09T14:00Z | 2015-11-10T13:00Z | -56 |
| 50 | 2015-12-14T19:00Z | 2015-12-14T19:00Z | -55 |
| 51 | 2015-12-20T07:00Z | 2015-12-20T22:00Z | -166 |
| 52 | 2015-12-31T14:00Z | 2016-01-01T00:00Z | -116 |
| 53 | 2016-01-20T10:00Z | 2016-01-20T16:00Z | -101 |
| 54 | 2016-02-01T07:00Z | 2016-02-03T02:00Z | -57 |
| 55 | 2016-02-16T19:00Z | 2016-02-16T19:00Z | -65 |
| 56 | 2016-03-06T19:00Z | 2016-03-06T21:00Z | -99 |
| 57 | 2016-03-16T23:00Z | 2016-03-16T23:00Z | -56 |
| 58 | 2016-04-02T22:00Z | 2016-04-02T23:00Z | -59 |
| 59 | 2016-04-07T20:00Z | 2016-04-08T00:00Z | -62 |
| 60 | 2016-04-13T01:00Z | 2016-04-14T20:00Z | -61 |
| 61 | 2016-04-16T20:00Z | 2016-04-16T21:00Z | -58 |
| 62 | 2016-05-08T04:00Z | 2016-05-08T07:00Z | -95 |
| 63 | 2016-08-03T10:00Z | 2016-08-03T15:00Z | -52 |
| 64 | 2016-08-23T20:00Z | 2016-08-23T21:00Z | -73 |
| 65 | 2016-09-01T09:00Z | 2016-09-02T02:00Z | -59 |
| 66 | 2016-09-28T17:00Z | 2016-09-29T09:00Z | -65 |
| 67 | 2016-10-13T11:00Z | 2016-10-13T17:00Z | -110 |
| 68 | 2016-10-25T14:00Z | 2016-10-25T17:00Z | -65 |
| 69 | 2016-10-27T17:00Z | 2016-10-29T03:00Z | -70 |
| 70 | 2016-11-10T16:00Z | 2016-11-10T17:00Z | -60 |
| 71 | 2017-03-01T21:00Z | 2017-03-01T21:00Z | -62 |
| 72 | 2017-03-27T10:00Z | 2017-03-27T16:00Z | -71 |
| 73 | 2017-03-31T06:00Z | 2017-03-31T06:00Z | -51 |
| 74 | 2017-04-04T07:00Z | 2017-04-04T10:00Z | -66 |
| 75 | 2017-04-20T04:00Z | 2017-04-20T04:00Z | -54 |
| 76 | 2017-04-22T04:00Z | 2017-04-22T08:00Z | -64 |
| 77 | 2017-04-24T04:00Z | 2017-04-24T04:00Z | -54 |
| 78 | 2017-05-28T00:00Z | 2017-05-28T07:00Z | -146 |
| 79 | 2017-07-16T12:00Z | 2017-07-16T13:00Z | -71 |
| 80 | 2017-08-19T08:00Z | 2017-08-19T08:00Z | -51 |
| 81 | 2017-08-23T12:00Z | 2017-08-23T12:00Z | -52 |
| 82 | 2017-08-31T10:00Z | 2017-08-31T11:00Z | -68 |
| 83 | 2017-09-07T23:00Z | 2017-09-08T01:00Z | -148 |
| 84 | 2017-09-10T22:00Z | 2017-09-10T22:00Z | -58 |
| 85 | 2017-09-13T05:00Z | 2017-09-13T05:00Z | -56 |
| 86 | 2017-09-16T02:00Z | 2017-09-16T05:00Z | -71 |
| 87 | 2017-09-18T08:00Z | 2017-09-18T08:00Z | -53 |
| 88 | 2017-09-27T13:00Z | 2017-09-28T08:00Z | -98 |
| 89 | 2017-10-12T08:00Z | 2017-10-14T05:00Z | -70 |
| 90 | 2017-10-15T04:00Z | 2017-10-15T04:00Z | -60 |
| 91 | 2017-10-26T14:00Z | 2017-10-26T14:00Z | -51 |
| 92 | 2017-11-07T10:00Z | 2017-11-08T04:00Z | -82 |
| 93 | 2017-11-10T07:00Z | 2017-11-10T07:00Z | -55 |
| 94 | 2017-11-21T06:00Z | 2017-11-21T06:00Z | -55 |
| 95 | 2017-12-05T10:00Z | 2017-12-05T10:00Z | -51 |
| 96 | 2018-04-20T08:00Z | 2018-04-20T09:00Z | -78 |
| 97 | 2018-05-06T01:00Z | 2018-05-06T04:00Z | -55 |
| 98 | 2018-08-26T00:00Z | 2018-08-26T07:00Z | -176 |
| 99 | 2018-09-10T22:00Z | 2018-09-11T10:00Z | -73 |
| 100 | 2018-09-22T07:00Z | 2018-09-22T07:00Z | -53 |
| 101 | 2018-10-07T20:00Z | 2018-10-07T20:00Z | -54 |
| 102 | 2018-11-05T05:00Z | 2018-11-05T05:00Z | -57 |
| 103 | 2019-05-11T15:00Z | 2019-05-11T21:00Z | -54 |
| 104 | 2019-05-14T06:00Z | 2019-05-14T07:00Z | -68 |
| 105 | 2019-08-05T09:00Z | 2019-08-05T19:00Z | -53 |
| 106 | 2019-09-01T06:00Z | 2019-09-01T20:00Z | -59 |
| 107 | 2019-09-05T05:00Z | 2019-09-05T05:00Z | -51 |
| 108 | 2019-09-27T22:00Z | 2019-09-27T22:00Z | -56 |
| 109 | 2019-10-25T08:00Z | 2019-10-25T08:00Z | -52 |
| 110 | 2020-02-18T15:00Z | 2020-02-18T16:00Z | -58 |
| 111 | 2020-04-20T11:00Z | 2020-04-20T12:00Z | -68 |
| 112 | 2020-09-25T23:00Z | 2020-09-27T21:00Z | -60 |
| 113 | 2020-09-28T04:00Z | 2020-09-28T21:00Z | -54 |
| 114 | 2021-03-01T05:00Z | 2021-03-01T06:00Z | -58 |
| 115 | 2021-04-17T05:00Z | 2021-04-17T05:00Z | -54 |
| 116 | 2021-04-26T06:00Z | 2021-04-26T06:00Z | -53 |
| 117 | 2021-05-12T14:00Z | 2021-05-12T14:00Z | -61 |
| 118 | 2021-08-27T18:00Z | 2021-08-27T23:00Z | -82 |
| 119 | 2021-09-17T20:00Z | 2021-09-17T21:00Z | -64 |
| 120 | 2021-10-12T06:00Z | 2021-10-12T14:00Z | -65 |
| 121 | 2021-10-17T21:00Z | 2021-10-17T23:00Z | -55 |
| 122 | 2021-11-04T06:00Z | 2021-11-04T13:00Z | -105 |
| 123 | 2022-01-14T20:00Z | 2022-01-14T22:00Z | -91 |
| 124 | 2022-02-03T09:00Z | 2022-02-03T10:00Z | -66 |
| 125 | 2022-02-10T17:00Z | 2022-02-10T19:00Z | -60 |
| 126 | 2022-03-05T20:00Z | 2022-03-05T21:00Z | -56 |
| 127 | 2022-03-12T04:00Z | 2022-03-13T23:00Z | -85 |
| 128 | 2022-04-14T16:00Z | 2022-04-14T21:00Z | -81 |
| 129 | 2022-05-28T04:00Z | 2022-05-28T07:00Z | -63 |
| 130 | 2022-07-02T07:00Z | 2022-07-02T10:00Z | -54 |
| 131 | 2022-07-07T17:00Z | 2022-07-07T22:00Z | -81 |
| 132 | 2022-07-19T12:00Z | 2022-07-19T20:00Z | -62 |
| 133 | 2022-08-08T06:00Z | 2022-08-08T07:00Z | -59 |
| 134 | 2022-09-04T08:00Z | 2022-09-04T17:00Z | -71 |
| 135 | 2022-10-14T10:00Z | 2022-10-14T10:00Z | -62 |
| 136 | 2022-10-22T15:00Z | 2022-10-22T18:00Z | -76 |
| 137 | 2022-11-03T10:00Z | 2022-11-03T18:00Z | -59 |
| 138 | 2022-11-07T15:00Z | 2022-11-07T17:00Z | -92 |
| 139 | 2022-12-07T16:00Z | 2022-12-07T17:00Z | -63 |
| 140 | 2022-12-27T05:00Z | 2022-12-27T15:00Z | -68 |
| 141 | 2023-01-04T07:00Z | 2023-01-04T09:00Z | -61 |
| 142 | 2023-01-15T12:00Z | 2023-01-15T13:00Z | -57 |
| 143 | 2023-02-15T10:00Z | 2023-02-15T17:00Z | -72 |
| 144 | 2023-02-26T22:00Z | 2023-02-27T12:00Z | -132 |
| 145 | 2023-03-23T15:00Z | 2023-03-24T02:00Z | -163 |
| 146 | 2023-04-23T16:00Z | 2023-04-24T05:00Z | -213 |
| 147 | 2023-04-27T08:00Z | 2023-04-28T06:00Z | -52 |
| 148 | 2023-05-06T05:00Z | 2023-05-06T05:00Z | -67 |
| 149 | 2023-05-20T06:00Z | 2023-05-20T11:00Z | -57 |
| 150 | 2023-06-16T03:00Z | 2023-06-16T09:00Z | -72 |
| 151 | 2023-06-25T00:00Z | 2023-06-25T01:00Z | -57 |
| 152 | 2023-07-16T23:00Z | 2023-07-16T23:00Z | -53 |
| 153 | 2023-07-26T08:00Z | 2023-07-26T08:00Z | -52 |
| 154 | 2023-08-05T01:00Z | 2023-08-05T06:00Z | -93 |
| 155 | 2023-09-02T09:00Z | 2023-09-02T15:00Z | -62 |
| 156 | 2023-09-12T17:00Z | 2023-09-12T22:00Z | -82 |
| 157 | 2023-09-18T18:00Z | 2023-09-19T02:00Z | -85 |
| 158 | 2023-09-25T00:00Z | 2023-09-25T02:00Z | -69 |
| 159 | 2023-10-21T01:00Z | 2023-10-21T07:00Z | -88 |
| 160 | 2023-10-28T10:00Z | 2023-10-28T11:00Z | -60 |
| 161 | 2023-11-04T23:00Z | 2023-11-05T19:00Z | -163 |
| 162 | 2023-11-22T05:00Z | 2023-11-22T06:00Z | -59 |
| 163 | 2023-11-25T12:00Z | 2023-11-25T19:00Z | -90 |
| 164 | 2023-12-01T11:00Z | 2023-12-01T13:00Z | -108 |
| 165 | 2023-12-14T04:00Z | 2023-12-14T09:00Z | -78 |
| 166 | 2023-12-17T12:00Z | 2023-12-17T16:00Z | -77 |

## Ergebnis: Superposed Epoch (Lag −24 h .. +72 h)

| Thread | n events | Peak \|z\| | Lag [h] | Area \|z\| | Bootstrap 95 % | Null 95 % | over/under null |
|---|---:|---:|---:|---:|---|---:|---|
| Dst | 166 | 6.45 | +1 | 278.2 | 6.27 .. 6.57 | 0.34 | over |
| Sym-H | 166 | 6.02 | +1 | 233.6 | 5.72 .. 6.32 | 0.35 | over |
| AE | 166 | 5.87 | +0 | 112.8 | 5.39 .. 6.20 | 0.50 | over |
| AL | 166 | 5.92 | +0 | 104.4 | 5.37 .. 6.37 | 0.50 | over |
| AU | 166 | 4.17 | -1 | 84.5 | 3.79 .. 4.74 | 0.48 | over |
| Bz | 166 | 3.09 | -1 | 40.8 | 2.81 .. 3.54 | 0.31 | over |
| Solar wind speed | 166 | 1.67 | +14 | 101.5 | 1.44 .. 2.02 | 0.47 | over |
| Solar wind density | 166 | 0.72 | +54 | 47.6 | 0.68 .. 0.94 | 0.46 | over |
| ABK dB/dt | 166 | 3.29 | +0 | 68.9 | 2.83 .. 3.87 | 0.36 | over |
| SOD dB/dt | 166 | 3.61 | -1 | 79.1 | 3.37 .. 4.60 | 0.34 | over |
| FMI GIC | 118 | 1.36 | +1 | 40.6 | 1.15 .. 1.86 | 0.70 | over |
| ECOD events/day | 135 | 0.45 | -24 | 36.1 | 0.35 .. 0.55 | 0.44 | over |
| ECOD ln(1+customers) | 135 | 0.18 | -24 | 9.5 | 0.10 .. 0.36 | 0.30 | under |

## Rangliste nach Peak-\|z\|

| Rank | Thread | Peak \|z\| | Lag [h] | Null 95 % | Factor | n |
|---:|---|---:|---:|---:|---:|---:|
| 1 | Dst | 6.45 | +1 | 0.34 | 19.01× | 166 |
| 2 | Sym-H | 6.02 | +1 | 0.35 | 17.32× | 166 |
| 3 | AL | 5.92 | +0 | 0.50 | 11.91× | 166 |
| 4 | AE | 5.87 | +0 | 0.50 | 11.70× | 166 |
| 5 | AU | 4.17 | -1 | 0.48 | 8.67× | 166 |
| 6 | SOD dB/dt | 3.61 | -1 | 0.34 | 10.51× | 166 |
| 7 | ABK dB/dt | 3.29 | +0 | 0.36 | 9.03× | 166 |
| 8 | Bz | 3.09 | -1 | 0.31 | 9.92× | 166 |
| 9 | Solar wind speed | 1.67 | +14 | 0.47 | 3.55× | 166 |
| 10 | FMI GIC | 1.36 | +1 | 0.70 | 1.94× | 118 |
| 11 | Solar wind density | 0.72 | +54 | 0.46 | 1.56× | 166 |
| 12 | ECOD events/day | 0.45 | -24 | 0.44 | 1.03× | 135 |
| 13 | ECOD ln(1+customers) | 0.18 | -24 | 0.30 | 0.61× | 135 |

## Was die Rangliste trägt — und was nicht

Die stärkste Verformung zeigen **Dst (6,45 |z|, +1 h)** und **Sym-H (6,02, +1 h)**,
gefolgt von den Auroral-Elektrojet-Indizes **AL (5,92, 0 h)** und **AE (5,87, 0 h)**.
Das ist kein unabhängiger Wettbewerb: Dst ist zugleich die Ereignisdefinition,
und Sym-H ist nahezu Dst in 1-min-Auflösung. Ihr Spitzenplatz ist teils
**definitorisch**, kein eigenständiger Befund (Riss 1). AL/AE antworten auf
Substürme und peaken **vor** dem Dst-Minimum; AU etwas schwächer und einen Lag
früher.

Der Sonnenwind liegt dahinter: **Bz 3,09 |z| bei −1 h** (Südwärtsdrehung vor
dem Dst-Onset), **Speed 1,67 bei +14 h**, **Dichte 0,72 bei +54 h**. Die
Geschwindigkeit hat die größte Fläche (101,5 |z|·h), weil sie nach dem Sturm
lange erhöht bleibt; ihr Peak ist trotzdem nur 3,55× Null. Die Dichte ist mit
1,56× Null der schwächste Stundenfaden und ihr +54-h-Peak ist nicht als
Sturmsignatur zu lesen.

Am Boden verformen sich **SOD dB/dt (3,61, −1 h)** und **ABK dB/dt (3,29, 0 h)**
klar über Null (10,5× bzw. 9,0×). **FMI GIC** liegt mit 1,36 |z| bei +1 h und
1,94× Null zwar über dem 95-%-Quantil, aber deutlich schwächer und nur für
**118** Ereignisse (Abdeckung endet 2023-09-30). Das ist ein gemessener, aber
dünner Arm — kein Beleg für eine starke GIC-Kopplung im Komposit.

**ECOD events/day** erreicht 0,45 |z| bei **−24 h** und liegt mit 1,03× nur
hauchdünn über dem Null-95-%-Quantil (0,44). Der Peak sitzt am **Rand des
Lagfensters**, also dort, wo die Suche nach dem Maximum besonders leicht
zufällig anschlägt. Das trägt **nicht** als belastbarer Vorlauf.
**ECOD ln(1+customers)** bleibt mit 0,18 unter Null (0,61×). Auf Tageskorn ist die
Ausfallrate damit kein stabiler Sturmzeiger — konsistent mit dem früheren
Netzausfall-Survey, der auf Tageskorn keine Kante fand.

## Risse

1. **Dst ist Definitions- und Messfaden zugleich.** Die Ereignisliste wird aus
   Dst < −50 nT gebildet; der Dst-Peak im Komposit ist deshalb teilweise
   zirkulär. Sym-H ist nahezu dieselbe Größe. Beide Zeilen sind
   Konsistenzprüfungen, keine unabhängigen Zeugen.
2. **Onset ist nicht der Sturmbeginn.** Als Onset gilt die erste Stunde unter
   −50 nT, also bereits die Hauptphase. Bz dreht früher südwärts; daher sein
   Lag von −1 h. Wer den solaren Antrieb als Onset setzt, verschiebt alle Lags.
3. **ECOD-Zeit ist lokal, die Korrektur ist eine Längen-Näherung.** Die
   Zuordnung `round(lon/15)` trifft die gesetzliche Zeitzone nicht exakt
   (politischer Zuschnitt, Sommerzeit). Der mediane Versatz ist −6 h; ein
   Restfehler von 1–2 h bleibt. Bei Tagesbins schmiert das, es erzeugt aber
   keinen Vorlauf.
4. **ECOD-Peak am Lagrand.** −24 h ist die erste getestete Stunde; der
   1,03×-Abstand zum Null-95-%-Quantil ist innerhalb der Nullstreuung und
   nicht als Frühwarnung lesbar.
5. **FMI GIC deckt nur 118 von 166 Ereignissen.** Die Reihe endet 2023-09-30;
   spätere Stürme (u. a. 2023-11-04) fehlen. Der Faden ist nicht mit den
   166-Ereignis-Fäden gleichzusetzen.
6. **Die Ereignisliste enthält schwache Einzelstunden-Stürme.** Die Schwelle
   war vorab fixiert; dadurch stehen auch kurze −51-nT-Episoden in der Liste.
   Der Median über 166 Ereignisse dämpft sie, entfernt sie aber nicht.
7. **Baseline ist saisonal ungematcht.** 35.299 ruhige Stunden nach
   ±10-d-Ausschluss sind über das ganze Fenster verteilt; z enthält
   Jahreszeit- und Sonnenzyklus-Lage. Die Null nutzt dieselbe Baseline, was den
   Vergleich fair hält, aber keine saisonale Dekomposition ersetzt.
8. **WDC „Best Available" ist provisional.** Die Version-Codes der HAPI-Daten
   werden nicht gefiltert; 99999-Füllwerte werden verworfen. Für eine
   Endversion müssten die Version-Codes mitgeführt werden.
9. **TEC und Polarlicht sind absent, nicht null.** Der TEC-Bestand ist nativ
   nur Mai 2024; THEMIS nur 2024-05-10; SSUSI liegt nicht im Haus. Für
   2014–2023 gibt es keinen Messwert, also auch keinen Rangplatz.
10. **Kp-Fallback ungenutzt.** Dst deckt alle Stunden des Fensters ab; die
    Ereignisliste braucht den Kp ≥ 6-Fallback nicht. Er ist dokumentiert, aber
    nicht gemessen.

## Reproduktion

```bash
# Datensätze liegen lokal unter omegaflow-work/sources; WDC wurde dort
# mit wdc_ae_compiler aus WDC-HAPI geharvestet (kein CDN-Upload).
cargo build --release -p omegaflow-measure --bin sturm_komposit_probe
./target/release/sturm_komposit_probe --data-root ../omegaflow-work/sources
```

## Ablage

- Probe: `tools/measure/src/bin/sturm_komposit_probe.rs`
- Dieser Survey: `docs/surveys/survey-2026-10-20-sturm-komposit.md`
- `phi/sources.φ` wurde **nicht** angefasst; keine CDN-Uploads.
- WDC-Rohdaten bleiben lokal unter `sources/wdc.kugi.kyoto-u.ac.jp/`.
