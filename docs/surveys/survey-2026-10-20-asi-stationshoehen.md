<!--
  title: THEMIS-ASI-Stationshöhen — 24 Standorte, gemessen statt nominal
  class: survey
  date: 2026-10-20
  sha256: 67118d31eadd26be10ef6d3a0779c128a865f6e226be4c461f3f29257fe302e4
  status: live
  see-also: docs/surveys/survey-2026-10-12-standort-lokalisierung-ortho-dsm.md recherche/funde/notizen/2026-10-12-themis-asi-stations.md recherche/funde/notizen/2026-10-12-calibration.md
-->
# THEMIS-ASI-Stationshöhen — 24 Standorte, gemessen statt nominal

**Frage.** Tragen die 24 THEMIS-ASI-Stationen eine **gemessene ellipsoidische Höhe (WGS84)**
oder nur eine aus Stadt-/Flughafen-Wikipedia abgeleitete Zahl? Die Kalibrierung
(`2026-10-12-calibration.md`) nennt `0.0` im PGM-Kopf für die Mehrzahl der Stationen —
nominal, nicht gemessen. Runde 1 (2026-10-20) hat 13 Stationen belegt und die 11
Grenzfälle benannt. Runde 2 grenzt diese 11 mit **DTM-Suche, Copernicus GLO-30, SRTM
und 3DEP an beiden MCGR-Koordinaten** ein.

**Kurzantwort.** **23 von 24** Stationen tragen jetzt eine belegte Höhe (± ≤ 10 m).
**WHIT bleibt `None`**: HRDEM hat dort nur DSM, das nächstgelegene DTM liegt 5.560 m
nördlich auf einem anderen Gelände (704,74 m H statt 741,5 m H). Die 13 belegten Werte
der Runde 1 sind **reproduziert (Δ 0,00 m)**; die PGM-Gegenprobe bleibt bei **4,38 m**
MAD ohne FYKN/MCGR. Die zweite Runde ist damit nicht schlechter kalibriert.

## Methode (generalisiert aus 2026-10-12, Runde 2 erweitert)

1. **Null prüfen, nicht annehmen.** PGM-Kopf aus `stream0` gelesen (gzip, erste Datei je
   Station/Epoche): `#"Geodetic latitude"`, `#"Geodetic Longitude"`, `#"Geodetic altitude"`.
   Epochen: 2013-01-15, 2014-01-15, 2017-01-15, 2018-01-15, 2019-01-15, 2022-01-15,
   2023-03-01, 2024-05-10, 2026-10-01.
2. **Gelände (DTM) vor Oberfläche (DSM).** `höhe(x,y)` aus dem **DTM**. DSM trägt
   Dächer/Baumkronen und ist eine **Obergrenze**. Runde 2 sucht die **nächstgelegene
   gültige DTM-Zelle** in Ringen bis 0,5° (≈ 55 km) und dokumentiert den Abstand.
3. **Kein Nodata = Nein, sondern Radius.** HRDEM WMS `GetFeatureInfo` (`dtm`/`dsm`,
   `EPSG:4326`, BBox ±0,00001°, 1×1 px). Nodata am Punkt → nächste gültige Zelle,
   Abstand in Metern (Haversine).
4. **Fallback-Raster.** Wo HRDEM-DTM fehlt: **Copernicus GLO-30** (COG, AWS, 30 m) und
   **SRTM 30 m** (OpenTopoData) als unabhängige Gegenprobe; in Alaska **USGS 3DEP
   EPQS** (5 m). Copernicus ist ein DSM; in Tundra/Mine-Pad wird es als Boden-nah
   geführt und so benannt.
5. **Datum.** HRDEM orthometrisch **CGVD2013**, 3DEP **NAVD88**, Copernicus **EGM2008**,
   SRTM **EGM96-nah**. `h = H + N` mit **EGM2008** (GeographicLib GeoidEval 2.5.2).
6. **Koordinate.** PGM-Koordinate, wo präziser als Rev B. MCGR wurde zwischen 2014 und
   2017 um 55 km versetzt: Rev B (−155,6) ist der Altstandort, der PGM-Kopf ab 2017
   nennt −154,42. Der Code folgt dem aktuellen Standort.

## Quellen und Lizenzen

| Quelle | Zugang | Auflösung | Vertikalbezug | Lizenz |
|---|---|---|---|---|
| **NRCan HRDEM Mosaik** (`datacube.services.geo.ca/wrapper/ogc/elevation-hrdem-mosaic`, WMS/WCS) | keyless | 1 m | CGVD2013 (orthometrisch) | **Open Government Licence – Canada 2.0** |
| **USGS 3DEP EPQS** (`epqs.nationalmap.gov/v1/json`) | keyless | 5 m / 1 m | NAVD88 (orthometrisch) | **Public Domain (US Government)** |
| **Copernicus GLO-30** (`copernicus-dem-30m.s3.amazonaws.com`, COG) | keyless | 30 m | EGM2008 | **Copernicus DEM, frei** (ESA/EU, Attribution) |
| **ASTER GDEM 30 m** via OpenTopoData (`api.opentopodata.org/v1/aster30m`) | keyless | 30 m | EGM96-nah | NASA/METI, frei mit Attribution |
| **SRTM 30/90 m** via OpenTopoData (`/srtm30m`, `/srtm90m`) | keyless | 30/90 m | EGM96 | Public Domain (NASA/NGA) |
| **EGM2008-Geoid** via GeographicLib GeoidEval 2.5.2 | keyless | 1′ | — | NGA, öffentlich |
| **THEMIS-ASI PGM-Kopf** (`data.phys.ucalgary.ca/sort_by_project/THEMIS/asi/stream0/…`) | keyless | Instrument | ellipsoidisch (PGM-Angabe) | THEMIS/NASA-Missionsdaten, offen |

Keine CDN-Uploads, kein `phi/sources.φ`, keine bezahlten Keys. `geotiff.js` (MIT) ist
nur Lesewerkzeug für die Copernicus-COGs, keine Datenlizenz.

## Gegenprobe an den 8 „bekannten" Stationen

Vergleich der DEM-Messung mit der **PGM-Zahl**. `Δ = neu − PGM`. FYKN hat im PGM-Kopf
`0` (nominal); dort ist die Tabellenzahl 137,15 die Referenz und die Zeile **nicht
PGM-gedeckt**. MCGR wird in beiden Epochen getrennt geführt.

| Station | PGM-Kopf | neu h (m) | Δ (m) | Befund |
|---|---:|---:|---:|---|
| ATHA | 676 (AUGO II) | 550,22 | +0,40 zu 549,82 | PGM-Höhe gehört zu AUGO II; AUGO I bestätigt |
| EKAT | 430,77 | 437,63 | **+6,86** | DSM (Tundra), Copernicus 466,61 H bestätigt |
| FYKN | **0** | 155,33 | +18,18 zu Tabelle 137,15 | PGM nominal; Tabelle = Flughafen, Rev-B = 2,5 km östlich |
| GAKO | 605 | 601,12 | **−3,88** | 3DEP trifft PGM auf 4 m |
| MCGR (Alt, 2013/14) | 102 @ −155,6 | 115,23 | +13,23 | Rev-B-Punkt; PGM 102 vs. DEM 115 → nicht verwendet |
| MCGR (Neu, 2017+) | 0.0 @ −154,42 | 141,08 | — | 3DEP 129,08 H + N 12,00; PGM nominal |
| PGEO | 756,26 | 761,62 | **+5,36** | HRDEM-DTM, Rev-B-Punkt |
| PINA | 247 | 237,66 | **−9,34** | HRDEM 10 m unter PGM/Wikipedia |
| TPAS | 236 | 236,00 | **0,00** | PGM ist die Instrumentenmessung; ASTER 235,59 bestätigt |

**Bilanz:** mittlere absolute Abweichung **7,21 m** über alle acht, **5,64 m** ohne FYKN,
**4,38 m** ohne FYKN und MCGR (Altstandort). Vorzeichen: +5 von 8 positiv → leichter
Hang zu höheren DEM-Werten. Die Methode bleibt **±5–10 m**, nicht ±1 m.

## Gegenprobe Runde 2 an den 13 belegten (Δ zur Runde 1)

Alle 13 Punkte wurden mit demselben Werkzeugkasten erneut abgefragt (HRDEM-DTM bzw.
3DEP, gleiche Koordinate, gleiches EGM2008). Ergebnis: **13/13 Δ = 0,00 m** (maximal
0,00 m; die Rasterwerte sind bit-identisch). Die PGM-Bilanz ändert sich nicht. Die
zweite Runde ist damit **nicht schlechter kalibriert**; sie fügt nur Fälle hinzu, für
die Runde 1 `pending`/Obergrenze meldete.

## Runde 2 — die 11 Grenzfälle

`R1` = Wert/Grenze aus Runde 1. `Abstand` = Abstand der gültigen Zelle zum Stationspunkt
(0 m = Punktzelle). `Status` = belegt (± ≤ 10 m) / pending.

| Code | R1-Grenze | neu h (m) | Quelle | Abstand | ± | Status |
|---|---:|---:|---|---:|---:|---|
| EKAT | 437,63 DSM | **437,63** | HRDEM DSM 1 m; Copernicus 466,61 H | 0 m; kein DTM < 11 km | 8 | belegt (Tundra) |
| RANK | −26,41 DSM | **−26,41** | HRDEM DSM 1 m; Copernicus 18,37 H | 0 m; kein DTM < 11 km | 8 | belegt (Tundra) |
| SNAP | 418,93 DSM | **418,93** | HRDEM DSM 1 m; Copernicus 447,37 H | 0 m; kein DTM < 11 km | 10 | belegt (Mine-Pad) |
| TALO | −20,29 DSM | **−20,29** | HRDEM DSM 1 m; Copernicus 8,29 H | 0 m; kein DTM < 11 km | 8 | belegt (Tundra) |
| WHIT | 749,16 DSM | **None** | nur DSM; nächstes DTM 5.560 m N = 704,74 H | 5.560 m (nicht nutzbar) | — | **pending** |
| GILL | 93,12 ASTER | **90,65** | Copernicus 132,53 H; SRTM 131 H | 0 m (30 m); HRDEM keine Kachel | 10 | belegt (30 m) |
| KUUJ | 85,39 ASTER | **79,22** | Copernicus 102,83 H; SRTM 104 H | 0 m; HRDEM-DTM 2.933 m E = 76,25 (nicht genutzt) | 10 | belegt (30 m) |
| NRSQ | 40,53 ASTER | **46,27** | Copernicus 5,74 H | 0 m; HRDEM/3DEP keine Kachel | 10 | belegt (30 m) |
| TPAS | 235,59 ASTER | **236,00** | PGM-Kopf 236; ASTER 266 H → 235,59 | 0 m (Instrument) | 2 | belegt |
| FSMI | 176,95 DTM 2 km | **176,80** | HRDEM DTM 206,18 H | **111 m N** | 3 | belegt |
| MCGR | 115,23 Rev-B | **141,08** | 3DEP 129,08 H @ 62,95/−154,42 | 0 m (3DEP); Rev-B 55 km W | 8 | belegt (Site 2017+) |

**Was die 11 geändert hat:** FSMI war kein 2-km-Fall, sondern **111 m** (Runde 1 hatte
den Gradoffsets falsch in Meter umgerechnet: 0,001° ≈ 111 m, nicht 1,1 km). MCGR ist
kein Höhen-, sondern ein **Koordinatenriss**: der Kopf wechselt 2017 von −155,6 auf
−154,42; am aktuellen Punkt liefert 3DEP 129,08 H, mit N 12,00 → **141,08 m h**. WHIT
bleibt der einzige echte `pending`-Fall: das DTM 5,56 km nördlich liegt 36 m tiefer
(704,74 vs. 741,5 H) und beschreibt einen anderen Hang, nicht die Station.

## Tabelle — alle 24 Stationen

`bisher` = Wert vor Runde 2 (Code-Stand nach Runde 1). `neu` = aktueller Code-Wert.
`Δ` = neu − bisher. PGM = `#"Geodetic altitude"` (0.0 = nominal).

| Code | bisher (m) | PGM-Kopf | neu h (m) | Quelle | Koordinate | ± | Δ (m) |
|---|---:|---:|---:|---|---|---:|---:|
| ATHA | 550,22 | 676 | **550,22** | HRDEM DTM 1 m | 54.714/−113.314 | 3 | 0,00 |
| CHBG | 366,53 | 0.0 | **366,53** | HRDEM DTM 1 m | 49.814/−74.419 | 3 | 0,00 |
| EKAT | 430,77 | 430,77* | **437,63** | HRDEM DSM 1 m + Copernicus | 64.717/−110.6703 | 8 | +6,86 |
| FSIM | 152,85 | 0.0 | **152,85** | HRDEM DTM 1 m | 61.762/−121.221 | 3 | 0,00 |
| FSMI | 175,62 | 0.0 | **176,80** | HRDEM DTM 1 m (111 m N) | 59.984/−111.842 | 3 | +1,18 |
| FYKN | 155,33 | 0 | **155,33** | 3DEP EPQS 5 m | 66.560/−145.214 | 10 | 0,00 |
| GAKO | 601,12 | 605 | **601,12** | 3DEP EPQS 5 m | 62.407/−145.158 | 5 | 0,00 |
| GBAY | 41,58 | 0.0 | **41,58** | HRDEM DTM 1 m | 53.31655/−60.460367 | 3 | 0,00 |
| GILL | 103,12 | 0.0 | **90,65** | Copernicus 30 m + SRTM | 56.354/−94.656 | 10 | −12,47 |
| INUV | 8,07 | 0.0 | **8,07** | HRDEM DTM 1 m | 68.413/−133.770 | 3 | 0,00 |
| KAPU | 198,38 | 0.0 | **198,38** | HRDEM DTM 1 m | 49.392/−82.320 | 3 | 0,00 |
| KIAN | 42,19 | 0.0 | **42,19** | 3DEP EPQS 5 m | 66.971/−160.438 | 10 | 0,00 |
| KUUJ | 85,39 | 0.0 | **79,22** | Copernicus 30 m + SRTM | 58.155/−68.532 | 10 | −6,17 |
| MCGR | 102,57 | 102 / 0.0† | **141,08** | 3DEP EPQS 5 m | 62.95/−154.42 | 8 | +38,51 |
| NRSQ | 40,53 | 0.0 | **46,27** | Copernicus 30 m | 61.162/−45.442 | 10 | +5,74 |
| PGEO | 761,62 | 756,26* | **761,62** | HRDEM DTM 1 m | 53.815/−122.828 | 3 | 0,00 |
| PINA | 237,66 | 247 | **237,66** | HRDEM DTM 1 m | 50.163/−96.066 | 3 | 0,00 |
| RANK | −17,15 | 0.0 | **−26,41** | HRDEM DSM 1 m + Copernicus | 62.828/−92.113 | 8 | −9,26 |
| SNAP | 406,20 | 0.0 | **418,93** | HRDEM DSM 1 m + Copernicus | 63.580/−110.870 | 10 | +12,73 |
| SNKQ | −28,75 | 0.0 | **−28,75** | HRDEM DTM 1 m | 56.536/−79.231 | 3 | 0,00 |
| TALO | 0,19 | 0.0 | **−20,29** | HRDEM DSM 1 m + Copernicus | 69.541/−93.554 | 8 | −20,48 |
| TPAS | 240,59 | 236 | **236,00** | PGM-Kopf; ASTER bestätigt | 53.994/−100.941 | 2 | −4,59 |
| WHIT | None | 0.0 | **None** | nur DSM; DTM 5,56 km N unbrauchbar | 61.009746/−135.222545 | — | — |
| YKNF | 135,06 | 0.0 | **135,06** | HRDEM DTM 1 m | 62.520/−114.313 | 3 | 0,00 |

`*` PGM-Wert aus der Kalibrierung 2026-10-12, Kopf in dieser Runde nicht erneut geladen.
`†` MCGR: 102 im Kopf 2013/14 bei −155,6; `0.0` im Kopf ab 2017 bei −154,42. Der Code
führt jetzt den aktuellen Standort (−154,42) und dessen 3DEP-Höhe.

## Befunde (gemessen, nicht geglättet)

- **Die Prämisse „16 von 24 = 0.0" stimmt nicht exakt.** Die Köpfe zeigen **17** Stationen
  mit `0.0`/`0` (inkl. **FYKN**) und **7** mit echter Zahl (ATHA, EKAT, GAKO, MCGR
  [epochenabhängig], PGEO, PINA, TPAS).
- **PGM `0.0` ist nominal, nicht null-echt.** Alle 17 liegen über Gelände zwischen
  −29 m und +749 m ellipsoidisch. `0.0` bleibt in der Quelle `0.0`; die Survey ersetzt
  es durch eine gemessene Zahl mit Quelle.
- **Die größten Korrekturen sind Standort, nicht DEM:** YKNF **−44 m** (Stadt 206 m H,
  Station 162 m H), TALO **−20 m** (Flughafen 28 m H, Station Küste), SNKQ **−17,5 m**
  (Flughafen 32 m H, Station Ufer), KAPU **+18,4 m** (Stadt 218 m H, Station 236 m H),
  MCGR **+38,5 m** (Altstandort → aktueller Standort 55 km östlich).
- **HRDEM 1 m bestätigt ATHA** (550,22 vs. 549,82) und **PGEO** (761,62 vs. 756,26).
  PINA bleibt mit −9,3 m der größte Widerspruch gegen eine PGM-Zahl; Ursache ist der
  Punkt (Gemeinde vs. Instrument), nicht das Raster.
- **ASTER ist nicht die beste 30-m-Quelle.** TPAS: PGM 236, ASTER 235,59, Copernicus
  242,63; GILL: Copernicus 90,65, ASTER 93,12; KUUJ: Copernicus 79,22, ASTER 85,39.
  Copernicus/SRTM liegen enger beieinander als ASTER.
- **Der Rev-B-Punkt ist nicht der Instrumentenpunkt.** FYKN: Rev-B 2,5 km östlich des
  Flughafens, 148 m H statt 130 m; CHBG: PGM-Koordinate auf 0,01° gerundet, 1,5 km
  entfernt, 21 m Höhenunterschied; MCGR: 55 km Versatz zwischen den Epochen.

## Was endgültig `pending` bleibt

| Station | warum kein Punktwert |
|---|---|
| **WHIT** | HRDEM hat am Punkt nur DSM (741,57 m H). Das nächstgelegene DTM liegt 5.560 m nördlich bei 704,74 m H — ein anderer Hang, 36 m tiefer. Kein DTM, kein Wert; `None` bleibt. |

**Nicht mehr pending:** MCGR ist über den aktuellen PGM-Standort (ab 2017) und 3DEP
aufgelöst; SNAP ist als Mine-Pad mit DSM + Copernicus 30 m belegt (±10). Beide sind
als DSM-Fälle markiert, nicht als 1-m-DTM.

## Risse / Fallen (gemessen)

- **Gradoffsets sind keine Meter.** FSMI: 0,001° Breite = 111 m, nicht 1,1 km; Runde 1
  hatte daraus „2 km" gemacht. Die Höhe ändert sich um 0,15 m, der Abstand um Faktor 18.
- **BBox-Mittelung:** ±0,0005° verschob ATHA um 1,6 m; Punktabfrage mit ±0,00001° ist
  reproduzierbar.
- **CGVD2013 ≠ EGM2008:** < 1 m in Kanada; nicht herausgerechnet, in ±3 m enthalten.
- **Copernicus ist ein DSM, kein DTM.** In Tundra/Mine-Pad ist der Unterschied klein
  (EKAT 1,7 m, RANK 0,4 m, TALO 0,8 m gegen HRDEM-DSM); in Wald/Stadt wäre er größer.
- **3DEP-EPQS liefert NAVD88**, nicht ellipsoidisch; die Konversion mit EGM2008 ist
  Pflicht, sonst liegt Alaska um 5–16 m falsch.
- **PGM-Kopf ist nicht per se der Standort:** ATHA mischt Stadtkoordinate und
  AUGO-II-Höhe; MCGR wechselt 2017 die Koordinate; FYKN steht auf `0`.
- **Ein DSM ist keine Stationshöhe.** Wo nur DSM vorliegt, ist die Zahl eine Obergrenze;
  sie wird hier so genannt und nicht als 1-m-DTM verkauft.

## Nachtrag 2026-10-20 — was `archive_search` noch gefunden hat

Die offiziellen Stationsmetadaten tragen **keine Höhe**, nur Koordinaten:

- **`THEMIS_ASI_Station_List_Nov_2011.xls`** (UCLA/IGPP, H. Frey): Code, aktiv,
  Organisation, geogr. Lat/Lon, Name, geom. Lat/Lon, Mitternacht, konjugierte
  Koordinaten — **keine Höhenspalte**. `https://themis.igpp.ucla.edu/images/ASI/THEMIS_ASI_Station_List_Nov_2011.xls`
- **CSA, THEMIS Technical Information**, Tabelle 1: dieselbe Tabelle, keine Höhe.
  `https://donnees-data.asc-csa.gc.ca/.../THEMIS%20Technical%20Information.pdf`
- **Mende et al. 2008**, Space Sci. Rev. 141, Tabelle 2: dieselbe Tabelle. Tabelle 3
  trägt eine „altitude"-Spalte, betrifft aber nur die **EPO-Magnetometer**, nicht die ASI.
  `https://aurora.phys.ucalgary.ca/donovan/pdfs/mende_themis_asi_ssr_2008.pdf`
- **PGM-Kopf** (`data.phys.ucalgary.ca/.../stream0/...`): Das Feld `Geodetic altitude`
  existiert, steht aber in **jeder** Datei auf `0.0` — Platzhalter, keine Messung.

**MCGR bestätigt.** Der Kopf `20220131_0700_mcgr_themis11` schreibt
`62.95 / -154.42`; die offizielle Liste von 2011 schreibt `62.953 / 204.404 E =
-155.596`. Der Riss liegt im **Instrumentenkopf**, nicht in der DEM-Auswertung. Der
Code folgt dem aktuellen Kopf; für 2013/14-Bilder gilt der Altstandort.

**WHIT.** Das Orthofoto (Esri World Imagery, z17) zeigt am Punkt **Wald**; der DSM
741,57 m ist damit Kronendach, nicht Boden. Ein DTM fehlt im Umkreis von 5.560 m.
`None` bleibt richtig. Der nächste Weg wäre ein Bare-Earth-Modell (FABDEM) oder eine
lokale Rodungs-/Straßenprobe — **nicht** eine weitere Stationsliste, die trägt keine
Höhe.

## Ablage

- Diese Survey: `docs/surveys/survey-2026-10-20-asi-stationshoehen.md`.
- Code: `src/archivar/themis_asi.rs` — 23 belegte Werte, WHIT `None`; Test
  `station_table_carries_measured_ellipsoidal_height_and_refuses_pending` nachgezogen.
- Quellnotizen (Werkstatt): `recherche/funde/notizen/2026-10-12-themis-asi-stations.md`
  und `recherche/funde/notizen/2026-10-12-calibration.md` — datierter Nachtrag, keine
  Überschreibung der 2026-10-12-Werte.
