<!--
  title: Anker-Höhen — Plausibilitätsmessung (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: c2ebffd636c23a4bf5815a1472154eb249cf5108a086abafa2f5624040fafa9a
  status: live
  see-also: phi/sources.φ
-->
# Anker-Höhen — Plausibilitätsmessung

**Methode:** `on earth <lat> <lon> <elev>`-Anker aus `phi/sources.φ`,
Höhe über [Open-Meteo Elevation](https://api.open-meteo.com/v1/elevation)
(2026-10-21, Batches à 40). Vergleich deklariert `elev` gegen API; Toleranz
**300 m** (DEM-Auflösung + gerundete Ankerkoordinaten).

**Befund:** 195 Bodenanker mit Koordinaten; **191** innerhalb 300 m, **4** außerhalb, davon **4** über 1000 m. **7** Anker deklarieren 0 m, API > 50 m. **5** Raumfahrt-Anker (> 9000 m) übersprungen. **76** API-Lücken (Ozean/Bojen/Null). **290** Anker sind `on earth 0 0 0` — überwiegend globale Satelliten-/Netzfelder, im Einzelfall fehlende Koordinate; eine Höhen-API kann sie nicht messen.

## Außerhalb 300 m

| Quelle | Zeile | lat lon | deklariert | API | Δ |
|---|---:|---|---:|---:|---:|
| `https://gml.noaa.gov/webdata/ccgg/trends/co2/co2_weekly_mlo.csv` | 1124 | 19.5 -155.6 | 0 | 3927.0 | +3927 |
| `https://gml.noaa.gov/webdata/ccgg/trends/ch4/ch4_mm_gl.txt` | 3573 | 19.5362 -155.5763 | 0 | 3388.0 | +3388 |
| `https://gml.noaa.gov/webdata/ccgg/trends/n2o/n2o_mm_gl.txt` | 3580 | 19.5362 -155.5763 | 0 | 3388.0 | +3388 |
| `https://gml.noaa.gov/webdata/ccgg/trends/sf6/sf6_mm_gl.txt` | 3587 | 19.5362 -155.5763 | 0 | 3388.0 | +3388 |

## 0 m deklariert, API > 50 m

| Quelle | Zeile | lat lon | API |
|---|---:|---|---:|
| `https://gml.noaa.gov/webdata/ccgg/trends/co2/co2_weekly_mlo.csv` | 1124 | 19.5 -155.6 | 3927.0 |
| `https://gml.noaa.gov/webdata/ccgg/trends/ch4/ch4_mm_gl.txt` | 3573 | 19.5362 -155.5763 | 3388.0 |
| `https://gml.noaa.gov/webdata/ccgg/trends/n2o/n2o_mm_gl.txt` | 3580 | 19.5362 -155.5763 | 3388.0 |
| `https://gml.noaa.gov/webdata/ccgg/trends/sf6/sf6_mm_gl.txt` | 3587 | 19.5362 -155.5763 | 3388.0 |
| `https://www.ndbc.noaa.gov/data/realtime2/45001.txt` | 8304 | 48.061 -87.793 | 181.0 |
| `https://api.meteo.lt/v1/stations/vilniaus-ams/observations/latest` | 557 | 54.625992 25.107064 | 160.0 |
| `https://kp.gfz.de/app/json/?start={today}T00:00:00Z&end={tomorrow}T00:00:00Z&index=Kp&status=def,nowcast&format=JSON` | 1277 | 52.07 12.68 | 76.0 |

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
- **Korrektur 2026-10-21:** Die erste Fassung dieses Surveys hat die Anker mit `awk +0` als ganze Zahlen angezeigt und die API damit an gerundeten Koordinaten abgefragt. Die Rohanker tragen 2–4 Dezimalen (z. B. `43.81 87.71`, `19.5362 -155.5763`); die Zahlen oben sind die Rohwerte. Die 4 Ausreißer sind Mauna-Loa/globale Mittel, keine DEM-Fehler.
