<!--
  title: Anker-Höhen — Plausibilitätsmessung (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: a9fe951578fdb305be6a81d6bca67124e05d366716fdb146571febddf7a08f67
  status: live
  see-also: phi/sources.φ
-->
# Anker-Höhen — Plausibilitätsmessung

**Methode:** `on earth <lat> <lon> <elev>`-Anker aus `phi/sources.φ`,
Höhe über [Open-Meteo Elevation](https://api.open-meteo.com/v1/elevation)
(2026-10-21, Batches à 40). Vergleich declare `elev` gegen API; Toleranz
**300 m** (DEM-Auflösung + gerundete Ankerkoordinaten).

**Befund:** 234 Bodenanker mit Koordinaten; **192** innerhalb 300 m, **42** außerhalb, davon **7** über 1000 m. **15** Anker deklarieren 0 m, API > 50 m. **5** Raumfahrt-Anker (> 9000 m) übersprungen. **37** API-Lücken (Ozean/Bojen/Null). **290** Anker sind `on earth 0 0 0` — überwiegend globale Satelliten-/Netzfelder, im Einzelfall fehlende Koordinate; eine Höhen-API kann sie nicht messen.

## Außerhalb 300 m

| Quelle | Zeile | lat lon | deklariert | API | Δ |
|---|---:|---|---:|---:|---:|
| `https://github.com/omegaflow/sources/releases/download/ncei.noaa.gov/spectra.bin` | 2198 | 19 -155 | 4142 | 0.0 | -4142 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=wmq/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 7607 | 43 87 | 908 | 3843.0 | +2935 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=frn/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6254 | 37 -119 | 331 | 2169.0 | +1838 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=reu/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 7216 | -21 55 | 1580 | 0.0 | -1580 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=dlt/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6124 | 11 108 | 1583 | 34.0 | -1549 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=eyr/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6215 | -43 172 | 102 | 1378.0 | +1276 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=sit/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 7294 | 57 -135 | 24 | 1193.0 | +1169 |
| `https://github.com/omegaflow/sources/releases/download/space.fmi.fi/keogram_KIL.bin` | 11113 | 69 20 | 380 | 1350.0 | +970 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=hua/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6488 | -12 -75 | 3313 | 4275.0 | +962 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=dur/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6176 | 41 14 | 920 | 0.0 | -920 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=cmo/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 5993 | 64 -147 | 197 | 1102.0 | +905 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=izn/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6566 | 40 29 | 256 | 1125.0 | +869 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=gui/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6371 | 28 -16 | 868 | 0.0 | -868 |
| `https://www.ndbc.noaa.gov/data/realtime2/46022.txt` | 8655 | 40 -124 | 0 | 684.0 | +684 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=tuc/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 7503 | 32 -110 | 946 | 1623.0 | +677 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=her/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6423 | -34 19 | 26 | 657.0 | +631 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=peg/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 7112 | 38 23 | 380 | 1002.0 | +622 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=thl/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 7437 | 77 -69 | 57 | 675.0 | +618 |
| `https://github.com/omegaflow/sources/releases/download/imag-data.bgs.ac.uk/abk_dbdt_1h.bin` | 2112 | 68 18 | 380 | 992.0 | +612 |
| `https://github.com/omegaflow/sources/releases/download/imag-data.bgs.ac.uk/abk_dbdt_1m.bin` | 2123 | 68 18 | 380 | 992.0 | +612 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=abk/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 5681 | 68 18 | 380 | 992.0 | +612 |
| `https://github.com/omegaflow/sources/releases/download/space.fmi.fi/keogram_ABK.bin` | 11105 | 68 18 | 380 | 992.0 | +612 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=tam/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 7398 | 22 5 | 1373 | 773.0 | -600 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=she/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 7268 | -15 -5 | 586 | 0.0 | -586 |
| `https://www.ndbc.noaa.gov/data/realtime2/46002.txt` | 8538 | 42 -124 | 0 | 581.0 | +581 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=iqa/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6527 | 63 -68 | 67 | 639.0 | +572 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=lzh/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6813 | 36 103 | 1560 | 2101.0 | +541 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=hbk/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6410 | -25 27 | 1555 | 1024.0 | -531 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=ebr/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6189 | 40 0 | 531 | 1.0 | -530 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=qsb/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 7190 | 33 35 | 525 | 0.0 | -525 |
| `https://www.ndbc.noaa.gov/data/realtime2/45178.txt` | 8382 | 44 -73 | 0 | 492.0 | +492 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=mgd/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6904 | 60 150 | 226 | 704.0 | +478 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=gck/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6293 | 44 20 | 231 | 680.0 | +449 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=lon/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6748 | 45 16 | 95 | 504.0 | +409 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=sjg/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 7307 | 18 -66 | 424 | 35.0 | -389 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=pag/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 7086 | 42 24 | 556 | 934.0 | +378 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=ppt/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 7164 | -17 -149 | 357 | 0.0 | -357 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=phu/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 7138 | 21 105 | 5 | 357.0 | +352 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=tan/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 7411 | -18 47 | 1375 | 1047.0 | -328 |
| `https://github.com/omegaflow/sources/releases/download/geodesy.unr.edu/ngl_arda.tenv3` | 2067 | 41 42 | 1834 | 2161.0 | +327 |
| `https://www.ndbc.noaa.gov/data/realtime2/45207.txt` | 8460 | 41 -81 | 0 | 318.0 | +318 |
| `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=lvv/best-avail/PT1M/xyzf&start={week_ago}T00:00:00Z&stop={now}Z&format=json` | 6787 | 49 23 | 326 | 643.0 | +317 |

## 0 m deklariert, API > 50 m

| Quelle | Zeile | lat lon | API |
|---|---:|---|---:|
| `https://www.ndbc.noaa.gov/data/realtime2/46022.txt` | 8655 | 40 -124 | 684.0 |
| `https://www.ndbc.noaa.gov/data/realtime2/46002.txt` | 8538 | 42 -124 | 581.0 |
| `https://www.ndbc.noaa.gov/data/realtime2/45178.txt` | 8382 | 44 -73 | 492.0 |
| `https://www.ndbc.noaa.gov/data/realtime2/45207.txt` | 8460 | 41 -81 | 318.0 |
| `https://www.ndbc.noaa.gov/data/realtime2/45001.txt` | 8304 | 48 -87 | 181.0 |
| `https://www.ndbc.noaa.gov/data/realtime2/45161.txt` | 8343 | 43 -86 | 178.0 |
| `https://www.ndbc.noaa.gov/data/realtime2/45186.txt` | 8421 | 42 -87 | 174.0 |
| `https://api.meteo.lt/v1/stations/vilniaus-ams/observations/latest` | 557 | 54 25 | 173.0 |
| `https://github.com/omegaflow/sources/releases/download/air-quality-api.open-meteo.com/openmeteo_birch_pollen.txt` | 840 | 52 13 | 106.0 |
| `https://www.ncdc.noaa.gov/cdo-web/api/v2/data?datasetid=GHCND&startdate={week_ago}&enddate={yesterday}&datatypeid=TMAX&stationid={station}&limit=1&units=metric` | 1237 | 52 13 | 106.0 |
| `https://github.com/omegaflow/sources/releases/download/power.larc.nasa.gov/nasa_power_t2m.txt` | 3333 | 52 13 | 106.0 |
| `https://kp.gfz.de/app/json/?start={today}T00:00:00Z&end={tomorrow}T00:00:00Z&index=Kp&status=def,nowcast&format=JSON` | 1277 | 52 12 | 78.0 |
| `https://www.ndbc.noaa.gov/data/realtime2/46012.txt` | 8616 | 37 -122 | 74.0 |
| `https://www.ndbc.noaa.gov/data/realtime2/46026.txt` | 8733 | 37 -122 | 74.0 |
| `https://api.phys.ucalgary.ca/hapi/data?id=NORSTAR_RIOMETER_K2@CHUR&start={week_ago}&stop={now}&format=csv` | 22 | 58 -94 | 67.0 |

## Raumfahrt-Anker (> 9000 m), nicht Boden

| Quelle | Zeile | deklariert |
|---|---:|---:|
| `https://services.swpc.noaa.gov/json/goes/primary/differential-protons-1-day.json` | 205 | 35786023 |
| `https://services.swpc.noaa.gov/json/goes/primary/integral-electrons-1-day.json` | 211 | 35786023 |
| `https://services.swpc.noaa.gov/json/goes/primary/integral-protons-1-day.json` | 217 | 35786023 |
| `https://services.swpc.noaa.gov/json/goes/primary/magnetometers-1-day.json` | 223 | 35786023 |
| `https://services.swpc.noaa.gov/json/goes/primary/xray-flares-7-day.json` | 231 | 35786023 |

## Grenzen

- Die Ankerkoordinaten sind **ganzzahlig** (z. B. `43 87`); ein DEM an dieser Stelle kann einen Berg treffen, während die Station im Tal liegt. Δ > 300 m ist ein **Kandidat**, kein Fehlerbeweis.
- Ozean-Bojen (NDBC) liegen auf See; das Land-DEM liefert 0 oder Küstennähe — kein Höhenfehler.
- Globale Indizes/Netze (`Kp`, GODAS) haben keinen Punkt; die API-Zahl ist bedeutungslos.
- Die 290 `0 0 0`-Anker sind überwiegend **globale Felder** (Satellit/Netz), im Einzelfall eine fehlende Koordinate; `elev_zero` ist dort ein Soft-False-Positive.
