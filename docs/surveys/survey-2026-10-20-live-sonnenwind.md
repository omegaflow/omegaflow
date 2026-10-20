<!--
  title: Live-Sonnenwind: RTSW 1 min (Quelle IMAP), DSCOVR ist Archiv
  class: survey
  date: 2026-10-20
  sha256: 897999e87b1a5480e2432539632b0e57dba290bf5055bad35fa939d76d306708
  status: live
  see-also: docs/surveys/survey-2026-10-20-bpa-historie-und-gic-2024.md auftrag/auftrag-2026-10-20-sonne-erde-faeden.md
-->
# Live-Sonnenwind: RTSW 1 min (Quelle IMAP), DSCOVR ist Archiv

**Was das ist.** Die Korrektur einer falschen Annahme: DSCOVR/ACE bei CDAWeb ist
**kein Live-Feed**. Live kommt der Sonnenwind über die RTSW-Endpunkte von SWPC —
und der Datensatz sagt selbst, welche Sonde misst.

## Messung 2026-10-20

| Feed | Endpunkt | Status | Bytes |
|---|---|---|---|
| RTSW Wind (1 min) | `services.swpc.noaa.gov/json/rtsw/rtsw_wind_1m.json` | **200** | 2.535.108 |
| RTSW Mag (1 min) | `services.swpc.noaa.gov/json/rtsw/rtsw_mag_1m.json` | **200** | 1.507.763 |
| Kp (1 min) | `services.swpc.noaa.gov/json/planetary_k_index_1m.json` | **200** | 27.925 |

Erste Zeile des Wind-Feeds:

```json
{"time_tag":"2026-10-20T13:09:03","active":false,"source":"IMAP",
 "proton_speed":352.95,"proton_temperature":56052,"proton_density":10.97,...}
```

`source: IMAP` ist der Punkt: der Feed nennt die Sonde, nicht nur den Wert.

## Gegenprobe: DSCOVR ist zu Ende

CDAWeb HAPI `DSCOVR_H0_MAG` antwortet **200** — aber `stopDate` ist
**2026-09-13T23:59:59Z**. Ein rollierendes Heute-Fenster liefert 1.201 „no data".
ACE (`TS1/TS2_L2_ACE_DEF`) endet 2026-10-02/03. Das ist ein **Archiv**, kein
Live-Feed.

## Woran man einen Live-Feed erkennt (ohne Namen)

1. **rollierendes Fenster** (`1-day`, `last 4 days`, `7-day`) statt fester Zeitraum,
2. **`active`/`source`-Felder** im Datensatz,
3. **`Last-Modified` von heute**.

Der Name `products/solar-wind/mag-1-day.json` ist dagegen **404** — die alten
Pfade sind tot, die neuen liegen unter `json/rtsw/`.

## Folge für die Kette

Der L1-Block muss **RTSW** heißen, nicht DSCOVR. DSCOVR bleibt als Archiv für
die Zeit bis 2026-09-13; live ist IMAP. Die drei RTSW-Feeds sind keyless und
brauchen nur einen Block, keine Zugänge.
