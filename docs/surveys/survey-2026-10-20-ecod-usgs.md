<!--
  title: Survey — ECOD-Ausfälle an USGS-Bodenstationen, räumlich gepaart (2026-10-20)
  class: survey
  date: 2026-10-20
  sha256: 7c14553f0f1d3c2bb0d8fff9638ec553112b6ed1500773d88bdb7eeda7c578a8
  status: live
  see-also: docs/surveys/survey-2026-10-20-netzausfall-kante.md docs/surveys/survey-2026-10-20-bpa-historie-und-gic-2024.md tools/measure/src/bin/ecod_usgs_probe.rs
-->
# Survey — ECOD-Ausfälle an USGS-Bodenstationen

**Anlass.** Der nationale Proxy der Ausfall-Kante trug nicht (|r| ≤ 0,05, p ≥ 0,67).
Die BPA-County-Paarung war räumlich kartiert, aber zeitlich leer, weil das BPA-GIC-
Asset nur vier rollierende Tage umfasst. Dieser Survey ersetzt den fehlenden
US-Boden durch **sechs USGS-Observatorien mit historischer 1-Minuten-Reihe** und
paart jedes ECOD-Ereignis mit der nächsten Station. Die Frage: trägt die räumliche
Paarung mehr als der nationale Proxy?

**Grenze vorab.** Gemessen wird eine **Assoziation** zwischen |dB/dt| und der
Ausfallrate auf Stundenkorn, keine Kausalaussage. ECOD trägt keine Kante „GIC hat
diesen Ausfall verursacht"; die Ursachenkategorie wird hier nicht benutzt.

## 1. Stationen

Sechs Observatorien, deren Koordinaten aus dem USGS-Observatory-Katalog stammen
(`geomag.usgs.gov/ws/observatories/`). Die Auswahl deckt CONUS von Ost (FRD),
Mitte (BOU), Nordwest (NEW), Südwest (TUC), Golf (BSL) und West (FRN) ab.

| Code | Name | Lat | Lon | Counties | Ereignisse | max. Paardistanz | Stunden mit dB/dt |
|---|---|---:|---:|---:|---:|---:|---:|
| FRD | Fredericksburg | 38,205 | −77,373 | 966 | 53.278 | 998,2 km | 83.907 |
| BOU | Boulder | 40,137 | −105,237 | 480 | 5.330 | 996,3 km | 83.972 |
| NEW | Newport | 48,265 | −117,122 | 116 | 4.008 | 928,0 km | 83.126 |
| TUC | Tucson | 32,174 | −110,733 | 62 | 2.626 | 987,3 km | 84.054 |
| BSL | Stennis Space Center | 30,350 | −89,635 | 849 | 65.257 | 998,8 km | 82.052 |
| FRN | Fresno | 37,091 | −119,718 | 80 | 30.603 | 772,8 km | 84.220 |

Die Stunden sind die Zahl der Stunden mit mindestens einem gültigen
1-Minuten-Differenzenwert; die Spanne ist 2014-01 .. 2023-12, nicht nur das
ECOD-Fenster.

## 2. Daten

| Träger | Inhalt | Abdeckung | Quelle |
|---|---|---|---|
| ECOD `grid_outage_events.bin` | 167.772 eindeutige (Start, County)-Ereignisse, 2.845 County-Zentroide | 2014-11-02 .. 2023-12-02 | `data.openei.org`, CDN |
| USGS/BGS IAGA-2002 | 720 Monatsdateien, 1-min X/Y/Z, `best-avail`/definitiv | 2014-01 .. 2023-12 | `imag-data.bgs.ac.uk/GIN_V1`, Kopfzeile `Source of Data: USGS` |

Die USGS-Web-API selbst liefert 1-min-Daten nur als Tages-/Monatsabruf; ein
`sampling_period=3600`-Abruf gibt durchgehend `null` zurück. Der historische
Träger ist deshalb die INTERMAGNET-Reduktion derselben USGS-Stationen über BGS,
nicht ein neuer USGS-Compiler. 720 von 720 Monatsabrufen waren nicht leer; die
Rohdaten liegen lokal unter `/tmp/opencode/ecod_usgs/iaga/` (2,1 GB) und sind
nicht Teil des Commits.

## 3. Paarung

Regel: Jeder County-Zentroid wird der **nächsten** der sechs Stationen
zugeordnet, wenn die Haversine-Distanz **≤ 1000 km** ist. Ereignisse ohne Station
in diesem Radius werden **`absent`** gezählt, nicht als 0.

- **2.553 von 2.845 Counties** gepaart (89,7 %)
- **161.102 von 167.772 Ereignissen** gepaart (96,02 %)
- **6.670 Ereignisse `absent`** (Alaska, Hawaii, Puerto Rico und Randlagen)
- Die 1000-km-Schwelle ist bewusst weit: die magnetische Störung ist bei
  Mittelbreiten großräumig, aber die Zuordnung bleibt eine Näherung.

## 4. Messung

- **Prädiktor:** stündliches Maximum von |dH/dt| in nT/min aus den 1-Minuten-
  Werten `H = sqrt(X² + Y²)`; Differenzen nur bei 30 s ≤ Δt ≤ 180 s. Keine
  Interpolation über Lücken.
- **Ziel:** Ausfälle pro 1.000 County-Stunden der zugeordneten Counties, stündlich.
  Stunden ohne ECOD-Ereignis sind echte Nullen im ECOD-Fenster.
- **Vorlauf:** Lag −3 .. +3 h; negativ heißt dB/dt **vor** dem Ausfall.
- **r:** Pearson über alle gepaarten Stations-Stunden, gepoolt; n = 455.138.
- **Null A (Stationsshuffle, 500 Ziehungen):** Die Stationslabels der gepaarten
  Counties werden permutiert; die Panelgrößen bleiben, die räumliche Zuordnung
  zerbricht. p = (#Null ≥ beobachtet + 1) / (500 + 1), Maximum über die 7 Lags.
- **Null B (ruhige Fenster):** Stunden unter dem 25-%-Quantil von |dB/dt| gegen
  Stunden über dem 75-%-Quantil; Rate pro 1.000 County-Stunden.
- **TE:** `transfer_entropy_binned`, 4 Bins, dB/dt → Ausfallrate, Leads 0 .. +3 h,
  dieselbe Stationsshuffle-Null mit 500 Ziehungen. n = 395.902 Samples in
  Segmenten ≥ 96 h; Lücken ≤ 3 h linear interpoliert, längere Lücken trennen
  Segmente.
- **Nationaler Vergleich:** Mittel der sechs Stations-|dB/dt|-Reihen gegen die
  Ausfallrate **aller** 2.845 Counties, gleiche Lags, gleiches Stundenkorn.

## 5. Ergebnis

### 5.1 r über die Lags

| Lag [h] | r | n |
|---:|---:|---:|
| −3 | −0,02379 | 455.138 |
| −2 | −0,02240 | 455.138 |
| −1 | −0,02147 | 455.138 |
| 0 | −0,02053 | 455.138 |
| +1 | −0,01990 | 455.138 |
| +2 | −0,02050 | 455.138 |
| +3 | −0,02048 | 455.138 |

**Beobachtetes Maximum: |r| = 0,02379 bei Lag −3 h, n = 455.138.**
Das Vorzeichen ist durchweg **negativ**: mehr |dB/dt| geht mit **weniger**
Ausfällen einher, nicht mit mehr. Nach Herausrechnen der mittleren Tagesstunde
je Station und Stunde bleibt **r = −0,00496** (n = 455.138): der rohe Wert ist
fast vollständig ein **diurnaler Konfund**, kein geomagnetischer Effekt.

### 5.2 Null

| Null | 95-%-Quantil | p | Ziehungen |
|---|---:|---:|---:|
| Stationsshuffle, max \|r\| über 7 Lags | 0,01794 | **0,0020** | 500 |
| Stationsshuffle, max TE über 4 Leads | 0,005311 | **1,0000** | 500 |

### 5.3 Ruhige und aktive Fenster

| Fenster | Stunden | Ausfallrate pro 1.000 County-Stunden |
|---|---:|---:|
| ruhig (|dB/dt| ≤ q25 = 0,585 nT/min) | 113.785 | 1,611696 |
| aktiv (|dB/dt| ≥ q75 = 1,569 nT/min) | 113.785 | 0,888709 |

Median |dB/dt| = 0,928 nT/min. Verhältnis aktiv/ruhig = **0,551**: in aktiven
Stunden ist die Ausfallrate **45 % niedriger**, nicht höher.

### 5.4 Transferentropie

| Lead [h] | TE | Samples |
|---:|---:|---:|
| 0 | 0,004127 | 395.902 |
| 1 | 0,004127 | 395.902 |
| 2 | 0,004453 | 395.902 |
| 3 | 0,004346 | 395.902 |

Beobachtetes Maximum TE = 0,004453 bei Lead 2 h. Die Stationsshuffle-Null liegt
darüber (siehe 5.2): TE trägt nicht.

### 5.5 Nationaler Vergleich

Der nationale Proxy (6-Stationen-Mittel, alle Counties) erreicht **|r| = 0,02102**
bei Lag −3 h, n = 76.975. Die räumliche Paarung liegt mit 0,02379 nur **0,0028
|r|-Einheiten** darüber — derselbe Größenbereich, dieselbe negative Richtung.

## 6. Verdikt

**Die räumliche Paarung trägt nicht mehr als der nationale Proxy und trägt die
Kante nicht.** Das beobachtete Maximum |r| = 0,02379 liegt über dem 95-%-Quantil
der Stationsshuffle-Null (0,01794, p = 0,0020) — statistisch ist der rohe Arm
also nicht null. Aber:

1. das Vorzeichen ist **negativ**, gegen die GIC-Erwartung;
2. nach Diurnal-Kontrolle bleiben **r = −0,00496** — der scheinbare Effekt ist
   die Tagesstruktur, nicht das Feld;
3. die Transferentropie fällt **unter** die Stationsshuffle-Null (0,004453 gegen
   0,005311, p = 1,0000);
4. der nationale Proxy erreicht **|r| = 0,02102** — nur 0,0028 unter dem
   räumlichen Wert, bei demselben negativen Vorzeichen;
5. die ruhigen Fenster tragen die höhere Ausfallrate (Verhältnis 0,551).

Damit ist die räumliche Paarung **kein neuer Träger** der Kante. Sie ist ein
sauberer, keyless reproduzierbarer Arm, aber die gemessene Assoziation zeigt in
die falsche Richtung, verschwindet nach Diurnal-Kontrolle und fällt in der
TE-Null. Die Kante Strom/Boden → Netzausfall bleibt im nicht-räumlichen wie im
räumlichen Arm **nicht getragen**.

## 7. Risse

1. **ECOD-Zeitstempel sind lokal, nicht UTC.** Der Compiler übernimmt `start_time`
   naiv; die Stundenaggregation ist um die Zeitzone (bis ~8 h) verschoben. Bei
   Lags ±3 h ist das ein Schmiereffekt und kann das negative Vorzeichen
   mitverursachen.
2. **Diurnaler Konfund.** |dB/dt| und Ausfallmeldungen haben beide eine
   Tagesstruktur. Die Stationsshuffle-Null kontrolliert sie nicht; die
   ruhig/aktiv-Tabelle zeigt den Konfund offen. Nach Herausrechnen der mittleren
   Tagesstunde je Station bleibt von |r| = 0,02379 nur **−0,00496** — der rohe
   Arm misst überwiegend Tageszeit, nicht Feld.
3. **1000-km-Schwelle.** Sie paart 96 % der Ereignisse, ist aber größer als die
   Korrelationslänge einzelner Störungen. Eine engere Schwelle würde die
   Paarungsrate senken und den Arm neu schneiden; sie ist hier nicht variiert.
4. **Nur H, kein D.** `|dH/dt|` ist nicht der vollständige horizontale Vektor;
   die Deklination geht nicht ein. Für GIC ist `dD/dt` mitunter der größere Term.
5. **Stationslücken.** `best-avail` liefert nicht für jede Stunde Werte; 82.000
   bis 84.000 von ~87.600 möglichen Stunden je Station. Fehlende Stunden sind
   `absent`, nicht 0.
6. **Ausfalldenominator.** „Pro 1.000 County-Stunden" nimmt an, dass jedes
   gepaarte County in jeder Stunde exponiert ist. ECOD ist eine Meldeliste, keine
   vollständige Exposition; die Rate ist eine Näherung.
7. **Keine Ursachenkategorie.** Der Arm trennt nicht nach `cause_class`; ein
   GIC-Effekt müsste in einer Teilmenge stärker sein, wird hier aber verwischt.
8. **Nationaler Arm ohne eigene Null.** Der Vergleich 0,02379 gegen 0,02102 ist
   ein Punktschätzer-Vergleich; der nationale Arm hat in dieser Runde keine
   Stationsshuffle-Null (die frühere Tagesnull liegt bei |r| ≈ 0,05).

## 8. Reproduktion

```bash
# ECOD
curl -sSfL -o /tmp/opencode/ecod_usgs/grid_outage_events.bin \
  https://github.com/omegaflow/sources/releases/download/data.openei.org/grid_outage_events.bin

# USGS/BGS IAGA-2002, 6 Stationen × 120 Monate
for sta in FRD BOU NEW TUC BSL FRN; do
  for y in $(seq 2014 2023); do
    for m in $(seq 1 12); do
      mm=$(printf '%02d' "$m")
      last=$(date -d "${y}-${mm}-01 +1 month -1 day" +%d)
      curl -sSfL -o "/tmp/opencode/ecod_usgs/iaga/${sta}_${y}_${mm}.iaga" \
        "https://imag-data.bgs.ac.uk/GIN_V1/GINServices?Request=GetData&format=iaga2002&observatoryIagaCode=${sta}&samplesPerDay=1440&publicationState=best-avail&dataStartDate=${y}-${mm}-01&dataEndDate=${y}-${mm}-${last}&dataType=variation"
    done
  done
done

cargo build --release -p omegaflow-measure --bin ecod_usgs_probe
./target/release/ecod_usgs_probe --outage /tmp/opencode/ecod_usgs/grid_outage_events.bin \
  --dir /tmp/opencode/ecod_usgs/iaga --draws 500
```

Die Probe liest standardmäßig dieselben Pfade. `--draws` senkt die Nullzahl für
Testläufe; der dokumentierte Lauf nutzt 500.

## 9. Ablage

- Probe: `tools/measure/src/bin/ecod_usgs_probe.rs`
- Dieser Survey: `docs/surveys/survey-2026-10-20-ecod-usgs.md`
- `phi/sources.φ` wurde **nicht** angefasst.
- Die BPA-, Sonne→Erde- und Sturm-Komposit-Dateien wurden **nicht** angefasst.
