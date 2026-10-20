<!--
  title: Survey — Kante Strom/Boden → Netzausfall, gemessen (2026-10-20)
  class: survey
  date: 2026-10-20
  sha256: ae5bc19a894f889a68be886fd3cb64c7a473a890d856911e0fbb924717b71946
  status: live
  see-also: docs/surveys/survey-2026-10-20-sonne-erde-kette.md tools/measure/src/bin/netzausfall_kante_probe.rs phi/sources.φ
-->
# Survey — Kante Strom/Boden → Netzausfall

**Anlass:** Die Sonne→Erde-Kette nennt „Netz" als Endsegment, aber die Kante
Strom/Boden → Netzausfall war im Haus nur als Datenlage notiert, nicht gemessen.
Dieser Survey misst sie mit dem vorhandenen Bestand und trennt sauber, was
gemessen ist und was die Daten nicht hergeben.

**Grenze vorab:** Eine Korrelation zwischen geomagnetischer Aktivität und
Ausfallraten ist **keine Kausalaussage**. Die ECOD-Daten tragen eine
Ursachenkategorie, aber keine Kante „GIC hat diesen Ausfall verursacht". Gemessen
wird eine Assoziation auf Tageskorn; Ursache und Wirkung bleiben unentschieden.

**Probe:** `tools/measure/src/bin/netzausfall_kante_probe.rs` (neu, dieser Survey).
Rohdaten wurden aus den CDN-Assets geladen und lokal nach `/tmp/opencode/outage_kante/`
gelegt; die Assets selbst sind nicht Teil dieses Commits.

## Daten und Assets (Stand 2026-10-20)

| Asset | Bytes | sha256 (Anfang) | Abdeckung |
|---|---:|---|---|
| `data.openei.org/grid_outage_events.bin` | 94.508.108 | `8c309b5a…` | 2014-11-02 .. 2023-12-02 |
| `transmission.bpa.gov/bpa_gic.bin` | 101.830 | `2a4c7f5c…` | **2026-10-03 .. 2026-10-06 (4 Tage)** |
| `space.fmi.fi/fmi_gic.bin` | 9.381.248 | `fce88a32…` | 1999-01-01 .. 2023-09-30 |
| `imag-data.bgs.ac.uk/abk_dbdt_1h.bin` | 4.580.616 | `d6c434d6…` | 1994-01-01 .. 2026-09-04 |
| `imag-data.bgs.ac.uk/sod_dbdt_1h.bin` | 4.581.512 | `eeb155bf…` | 1994-01-01 .. 2026-08-31 |
| `space.fmi.fi/fmi_image_mag_nur.bin` | 1.555.148 | `93f17d0a…` | **2003-10-29 .. 2003-11-01 (4 Tage)** |

## Ausfälle: Zeilen sind keine Ereignisse

Der Bin trägt **1.575.135 Records** = 3 Komponenten × **525.045 Zeilen**
(`duration_h`, `max_customers`, `cause_class`). Die ECOD-CSV enthält jedoch
exakte Mehrfachzeilen: dieselbe `start_time` + `fips` erscheint bis zu 56-mal
identisch. Die Probe gruppiert deshalb auf `(Start, County-Zentroid)`.

- **525.045 Zeilen → 167.772 eindeutige (Start, County)-Ausfälle**
- **2.845 Counties**, Fenster **2014-11-02 .. 2023-12-02**
- 1.087 Zeilen ohne County-Zentroid fallen im Compiler heraus; sie sind in
  den 167.772 nicht enthalten.

Ohne diese Dedup würde jede Ausfallrate um Faktor ~3,1 zu hoch liegen. Das ist
kein Detail, sondern der Unterschied zwischen einem Messwert und einem Artefakt.

## Räumliche Paarung (Schritt 1)

Regel: Jeder County-Zentroid wird dem nächsten der **11 BPA-Substationen**
zugeordnet, wenn die Haversine-Distanz **≤ 300 km** ist. Die BPA-Koordinaten
sind Stadtzentren der benannten Substation (keine veröffentlichte
Substation-Koordinate); das ist eine zweite, kleine Unsicherheit.

- **97 von 2.845 Counties** liegen innerhalb von 300 km
- **4.188 von 167.772 Ereignissen** (2,50 %)
- maximale Paardistanz **294,2 km**

| Station | Counties | Ereignisse |
|---|---:|---:|
| eugene_or | 13 | 514 |
| spokane_wa | 10 | 458 |
| the_dalles_or | 9 | 35 |
| lewiston_id | 22 | 247 |
| kalispell_mt | 10 | 29 |
| monroe_wa | 3 | 474 |
| wilsonville_or | 5 | 282 |
| vancouver_wa | 7 | 476 |
| ellensburg_wa | 3 | 133 |
| bothell_wa | 9 | 1.475 |
| moses_lake_wa | 6 | 65 |
| **Summe** | **97** | **4.188** |

**Der zeitliche Arm dieser Paarung ist leer.** Das ausgelieferte
`bpa_gic.bin` ist ein rollierendes 4-Tage-Fenster (2026-10-03 .. 2026-10-06),
kein Archiv. Es gibt **0 gemeinsame Tage** mit 2014–2023. Die räumliche Paarung
ist damit kartiert, aber nicht in Zeit messbar. Die BPA-Quelle selbst
(`transmission.bpa.gov/.../gic.txt`) sagt im Header „at 5-minute intervals,
last 4 days"; ein historisches BPA-GIC-Asset liegt im Haus nicht vor.

## Zeitliche Paarung und Fallback (Schritt 2)

Weil die räumliche Paarung zeitlich zu dünn ist (0 Tage), wird — wie im Auftrag
vorgesehen und hier ausdrücklich benannt — auf **nationale Tagesaggregate**
gegen **nicht-räumliche** Proxys gewechselt:

- Ausfall: alle US-Counties, **Events/Tag**, **Summe Kunden/Tag**, **Summe Dauer/Tag**
- Strom: `fmi_gic` (Finnland, 1 h) als GIC-Tagesmaximum
- Boden: `abk_dbdt_1h`, `sod_dbdt_1h` als dB/dt-Tagesmaximum; `fmi_image_mag_nur`
  als 10-s-dB/dt-Tagesmaximum
- Lags **0, 1, 2 Tage** (Proxy führt den Ausfall)
- Tageskorn ist UTC; ECOD-Zeitstempel sind lokale Zeit (siehe Riss 4)

Gemeinsames Fenster und n:

| Proxy | Abdeckung | Überlappung mit Ausfällen | n Tage |
|---|---|---|---:|
| BPA GIC (11 Stationen) | 2026-10-03 .. 2026-10-06 | keine | **0** |
| ABK dB/dt | 1994-01-01 .. 2026-09-04 | 2014-11-02 .. 2023-12-02 | **3.318** |
| SOD dB/dt | 1994-01-01 .. 2026-08-31 | 2014-11-02 .. 2023-12-02 | **3.318** |
| NUR dB/dt | 2003-10-29 .. 2003-11-01 | keine | **0** |
| FMI GIC | 1999-01-01 .. 2023-09-30 | 2014-11-02 .. 2023-09-30 | **2.418** |

Ausfalltage ohne Ereignis gehen als **0** in die Reihe ein; nur Tage mit
Proxy-Wert werden gepaart. Es werden 3 Metriken × 3 Lags = **9 Zellen** je
Proxy getestet; die Null nimmt das Maximum über dieselbe Familie.

## Null (Schritt 3)

Hausüblich: **Block-Shift**, 500 Replikate, zirkuläre Verschiebung des
Proxy-Vektors **innerhalb jedes Kalenderjahres**, 95-%-Quantil des
Maximal-|r| über die 9 Zellen; `p = (#Null ≥ beobachtet + 1) / (500 + 1)`.
Das erhält die jahreszeitliche Struktur und die Autokorrelation des Proxys.

## Ergebnis (Schritt 4)

| Proxy | beobachtetes max \|r\| | Lag | Metrik | n | Null 95 % | p | Verdikt |
|---|---:|---:|---|---:|---:|---:|---|
| ABK dB/dt | **0,0499** | 0 d | ln(1+Kunden) | 3.318 | 0,0975 | 0,7545 | **trägt nicht** |
| SOD dB/dt | **0,0449** | 0 d | ln(1+Kunden) | 3.318 | 0,1000 | 0,8244 | **trägt nicht** |
| FMI GIC | **0,0330** | 0 d | ln(1+Kunden) | 2.418 | 0,0787 | 0,6707 | **trägt nicht** |
| NUR dB/dt | — | — | — | 0 | — | — | nicht messbar |
| BPA GIC, County-paarig | — | — | — | 0 | — | — | nicht messbar |

Die Vorzeichen sind durchweg **negativ** und winzig: Events/Tag bei Lag 0
r = −0,0295 (ABK), −0,0346 (SOD), −0,0282 (FMI). Ein geomagnetisch aktiver Tag
hat in diesen Reihen also nicht mehr, sondern eher minimal weniger Ausfälle —
weit innerhalb des Nullbands. Das ist kein „Schutzeffekt", sondern Rauschen.

**Verdikt:** Im gemessenen, nicht-räumlichen Arm **trägt die Kante nicht**.
Die beobachteten Maxima liegen alle unter dem 95-%-Quantil des Block-Shift-Nulls.
Der räumliche Arm (BPA-Counties) ist **nicht gemessen**, nicht negativ — dafür
fehlt die historische BPA-GIC-Reihe.

## Risse (was die Messung nicht trägt)

1. **BPA-GIC ist kein Archiv.** Das Asset ist ein 4-Tage-Snapshot (2026-10-03 ..
   2026-10-06). Die 97-County-Paarung ist räumlich definiert, aber mit
   2014–2023 nicht zeitlich verbindbar. Ohne historische BPA-Reihe bleibt die
   einzige echte Strom/Boden→Ausfall-Paarung im US-Nordwesten ungemessen.
2. **Kein US-Boden.** ABK/SOD liegen in Skandinavien, NUR in Finnland. Es gibt
   im Bestand keine Bodenstation in den 97 gepaarten US-Counties. Der
   Boden→Ausfall-Arm ist deshalb nur global, nicht räumlich.
3. **NUR ist ein 4-Tage-Fenster** (Halloween 2003). Außerhalb dieses Fensters
   gibt es keine NUR-dB/dt-Werte; mit 2014–2023 überlappt nichts.
4. **ECOD-Zeitstempel sind lokal, nicht UTC.** Der Compiler übernimmt die
   `start_time` naiv; die Tagesaggregation ist dadurch um die Zeitzone
   (bis ~8 h) verschoben. Bei Tagesmaxima und Lags 0–2 d ist das ein Schmiereffekt,
   kein Nulldurchgang.
5. **ECOD-Mehrfachzeilen.** 525.045 Zeilen sind 167.772 eindeutige
   (Start, County)-Ausfälle; die Rohzeilenzahl ist als n unbrauchbar.
6. **Kein AE/Dst im Fenster.** Das Register hat `wdc_ae` nur als 1981–1984-Asset;
   für 2014–2023 fehlt der globale Sturmindex. Der Fallback nutzt deshalb
   dB/dt und FMI-GIC, nicht AE/Dst. Das ist eine benannte Ersatzgröße, kein
   äquivalenter Index.
7. **Nationale Aggregate sind wetterdominiert.** Die Tagesrate über alle
   US-Counties wird von konvektivem Wetter und Jahreszeit geprägt; der
   Block-Shift-Null kontrolliert Autokorrelation und Saisonlage des Proxys,
   aber nicht alle Konfounder. Ein kleiner GIC-Beitrag < |r| ≈ 0,05 ist mit
   diesem Design nicht ausschließbar.
8. **Stationskoordinaten sind Stadtzentren.** Die BPA-Substationen tragen keine
   veröffentlichten Koordinaten; die Paarung nutzt die Stadtzentren der
   Namensgeber. Die Distanzfehler liegen im Bereich weniger bis einiger
   zehn Kilometer und ändern die 97-County-Menge nicht qualitativ.

## Reproduktion

```bash
# Assets (CDN, 2026-10-20)
for u in data.openei.org/grid_outage_events.bin \
         transmission.bpa.gov/bpa_gic.bin \
         space.fmi.fi/fmi_gic.bin \
         space.fmi.fi/fmi_image_mag_nur.bin \
         imag-data.bgs.ac.uk/abk_dbdt_1h.bin \
         imag-data.bgs.ac.uk/sod_dbdt_1h.bin; do
  curl -sSfL -o "/tmp/opencode/outage_kante/$(basename "$u")" \
    "https://github.com/omegaflow/sources/releases/download/$u"
done

cargo build -p omegaflow-measure --bin netzausfall_kante_probe
./target/debug/netzausfall_kante_probe
```

Die Probe liest standardmäßig `/tmp/opencode/outage_kante/`; alternativ
`--outage`, `--bpa`, `--fmi`, `--abk`, `--sod`, `--nur`.

## Ablage

- Probe: `tools/measure/src/bin/netzausfall_kante_probe.rs`
- Dieser Survey: `docs/surveys/survey-2026-10-20-netzausfall-kante.md`
- `phi/sources.φ` wurde **nicht** angefasst.
- Das Papier und der Sonne→Erde-Survey wurden **nicht** angefasst.
