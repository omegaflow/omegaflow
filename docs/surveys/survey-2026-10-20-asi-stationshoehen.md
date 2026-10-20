<!--
  title: THEMIS-ASI-Stationshöhen — 24 Standorte, gemessen statt nominal
  class: survey
  date: 2026-10-20
  sha256: 8bc410246613e36fd96cc22603e55dd30c6ef292cb2ccd60472daa7781187e26
  status: live
  see-also: docs/surveys/survey-2026-10-12-standort-lokalisierung-ortho-dsm.md recherche/funde/notizen/2026-10-12-themis-asi-stations.md recherche/funde/notizen/2026-10-12-calibration.md
-->
# THEMIS-ASI-Stationshöhen — 24 Standorte, gemessen statt nominal

**Frage.** Tragen die 24 THEMIS-ASI-Stationen eine **gemessene ellipsoidische Höhe (WGS84)**
oder nur eine aus Stadt-/Flughafen-Wikipedia abgeleitete Zahl? Die Kalibrierung
(`2026-10-12-calibration.md`) nennt `0.0` im PGM-Kopf für die Mehrzahl der Stationen —
nominal, nicht gemessen. Diese Survey misst pro Station die Geländehöhe aus offenen
Rastern, konvertiert mit dem EGM2008-Geoid nach `h = H + N` und prüft die Methode zuerst
an den Stationen mit echter PGM-Zahl.

**Kurzantwort.** Für **10 Stationen** liegt ein 1-m-DTM (NRCan HRDEM) vor, für **4** ein
US-3DEP-Punkt (5 m), für **6** nur ein 1-m-DSM (Oberfläche, keine Geländehöhe) und für
**4** nur ASTER GDEM 30 m. **Eine** Station bleibt `pending`: MCGR, weil PGM-Koordinate
und PGM-Höhe zwischen den Epochen auseinanderlaufen. Die Methode trifft die PGM-Zahlen
an den geprüften Stationen im Mittel auf **±5,6 m** (Median ≈ 4,6 m), ohne FYKN/MCGR auf
**±4,4 m**; die Reststreuung ist koordinaten- und produktbedingt, nicht Modellrauschen.

## Methode (generalisiert aus 2026-10-12)

1. **Null prüfen, nicht annehmen.** PGM-Kopf aus `stream0` gelesen (gzip, erste Datei je
   Station/Epoche): `#"Geodetic latitude"`, `#"Geodetic Longitude"`, `#"Geodetic altitude"`.
   Epochen: 2013-01-15, 2014-01-15, 2022-01-15, 2023-03-01, 2024-05-10, 2026-10-01.
2. **Gelände (DTM).** `höhe(x,y)` aus dem **DTM**, nicht aus dem DSM: der DSM trägt
   Dächer/Baumkronen und ist für eine Bodenstation eine **Obergrenze**. Nur wo kein DTM
   trägt, wird der DSM als `Obergrenze` geführt und so benannt.
3. **Punktabfrage, kein Flächenmittel.** HRDEM WMS `GetFeatureInfo` (`dtm`/`dsm`,
   `EPSG:4326`, BBox ±0,00001°, 1×1 px). Ein größeres BBox mittelt über Nachbarzellen
   (ATHA: 569,75 m bei ±0,0005° vs. 571,40 m bei ±0,00001°) — der Fehler ist die
   Abfrage, nicht das Raster.
4. **Datum.** HRDEM ist orthometrisch **CGVD2013**, 3DEP **NAVD88**, ASTER/SRTM
   **EGM96-nah**. `h = H + N` mit **EGM2008** (GeographicLib GeoidEval 2.5.2). Die
   CGVD2013↔EGM2008-Differenz bleibt < 1 m und ist in der Unsicherheit enthalten.
5. **Koordinate.** PGM-Koordinate, wo sie präziser ist als Rev B (RANK, GBAY, WHIT);
   Rev B, wo der PGM-Kopf auf 0,01° rundet (CHBG) oder nachweislich die Stadt statt des
   Standorts trägt (ATHA). Die verwendete Koordinate steht in jeder Zeile.

## Quellen und Lizenzen

| Quelle | Zugang | Auflösung | Vertikalbezug | Lizenz |
|---|---|---|---|---|
| **NRCan HRDEM Mosaik** (`datacube.services.geo.ca/wrapper/ogc/elevation-hrdem-mosaic`, WMS/WCS) | keyless | 1 m | CGVD2013 (orthometrisch) | **Open Government Licence – Canada 2.0** |
| **USGS 3DEP EPQS** (`epqs.nationalmap.gov/v1/json`) | keyless | 5 m / 1 m | NAVD88 (orthometrisch) | **Public Domain (US Government)** |
| **ASTER GDEM 30 m** via OpenTopoData (`api.opentopodata.org/v1/aster30m`) | keyless | 30 m | EGM96-nah | NASA/METI, frei mit Attribution |
| **SRTM 90 m** via OpenTopoData (`/srtm90m`), Gegenprobe | keyless | 90 m | EGM96 | Public Domain (NASA/NGA) |
| **EGM2008-Geoid** via GeographicLib GeoidEval 2.5.2 | keyless | 1′ | — | NGA, öffentlich |
| **THEMIS-ASI PGM-Kopf** (`data.phys.ucalgary.ca/sort_by_project/THEMIS/asi/stream0/…`) | keyless | Instrument | ellipsoidisch (PGM-Angabe) | THEMIS/NASA-Missionsdaten, offen |

Keine CDN-Uploads, kein `phi/sources.φ`, keine bezahlten Keys.

## Gegenprobe an den 8 „bekannten" Stationen

Vergleich der DEM-Messung mit der **PGM-Zahl** (nicht mit der Wikipedia-Tabelle).
`Δ = neu − PGM`. FYKN hat im PGM-Kopf `0` (nominal); dort ist die Tabellenzahl 137,15
die Referenz, und die Zeile ist als **nicht PGM-gedeckt** markiert.

| Station | PGM-Kopf | neu h (m) | Δ (m) | Befund |
|---|---:|---:|---:|---|
| ATHA | 676 (AUGO II) | 550,22 | +0,40 zu 549,82 | PGM-Höhe gehört zu AUGO II; Standort AUGO I bestätigt |
| EKAT | 430,77 | 437,63 | **+6,86** | nur DSM (DTM am Punkt nodata), Obergrenze |
| FYKN | **0** | 155,33 | +18,18 zu Tabelle 137,15 | PGM nominal; Tabelle = Flughafen, Rev-B = 2,5 km östlich |
| GAKO | 605 | 601,12 | **−3,88** | 3DEP trifft PGM auf 4 m |
| MCGR | 102 (2013/14) · 0,0 (2022) | 115,23 | +13,23 | Koordinate/Epoche widersprüchlich → `pending` |
| PGEO | 756,26 | 761,62 | **+5,36** | HRDEM-DTM, Rev-B-Punkt |
| PINA | 247 | 237,66 | **−9,34** | HRDEM 10 m unter PGM/Wikipedia |
| TPAS | 236 | 235,59 | **−0,41** | ASTER 30 m trifft PGM auf 0,4 m |

**Bilanz der Gegenprobe:** mittlere absolute Abweichung **7,21 m** über alle acht,
**5,64 m** ohne FYKN (kein PGM-Nichtnull), **4,38 m** ohne FYKN und MCGR.
Vorzeichen: +5 von 8 positiv → leichter Hang zu höheren DEM-Werten, konsistent mit
DSM-Anteil und mit dem Rev-B-Punkt gegenüber dem Instrumentenpunkt. Die Methode ist
damit **brauchbar auf ±5–10 m**, nicht auf ±1 m; die Koordinate dominiert.

## Tabelle — alle 24 Stationen

`bisher` = `alt_m` in `src/archivar/themis_asi.rs` (abgeleitet aus Wikipedia/ASTER,
2026-10-12). `PGM` = `#"Geodetic altitude"` aus dem Kopf, wo gelesen.
`neu` = `H + N` aus der genannten Quelle. `Δ` = neu − bisher.

| Code | bisher (m) | PGM-Kopf | neu h (m) | Quelle | Koordinate | Unsicherheit | Δ (m) |
|---|---:|---:|---:|---|---|---:|---:|
| ATHA | 549,82 | 676 | **550,22** | HRDEM DTM 1 m | 54.714/−113.314 | ±3 | +0,40 |
| CHBG | 363,92 | 0.0 | **366,53** | HRDEM DTM 1 m | 49.814/−74.419 | ±3 | +2,61 |
| EKAT | 430,77 | 430,77* | **437,63** | HRDEM DSM 1 m | 64.717/−110.6703 | ±8 | +6,86 |
| FSIM | 154,61 | 0.0 | **152,85** | HRDEM DTM 1 m | 61.762/−121.221 | ±3 | −1,76 |
| FSMI | 175,62 | 0.0 | **176,95** | HRDEM DTM 1 m‡ | 59.984/−111.842 | ±5 | +1,33 |
| FYKN | 137,15 | 0 | **155,33** | 3DEP EPQS 5 m | 66.560/−145.214 | ±10 | +18,18 |
| GAKO | 589,72 | 605 | **601,12** | 3DEP EPQS 5 m | 62.407/−145.158 | ±5 | +11,40 |
| GBAY | — | 0.0 | **41,58** | HRDEM DTM 1 m | 53.31655/−60.460367 | ±3 | — |
| GILL | 103,12 | 0.0 | **93,12** | ASTER GDEM 30 m | 56.354/−94.656 | ±20 | −10,00 |
| INUV | 11,47 | 0.0 | **8,07** | HRDEM DTM 1 m | 68.413/−133.770 | ±3 | −3,40 |
| KAPU | 179,94 | 0.0 | **198,38** | HRDEM DTM 1 m | 49.392/−82.320 | ±3 | +18,44 |
| KIAN | 33,15 | 0.0 | **42,19** | 3DEP EPQS 5 m | 66.971/−160.438 | ±10 | +9,04 |
| KUUJ | 85,39 | 0.0 | **85,39** | ASTER GDEM 30 m | 58.155/−68.532 | ±20 | ±0,00 |
| MCGR | 102,57 | 102 / 0.0† | **115,23** | 3DEP EPQS 5 m | 62.953/−155.596 | ±10 | +12,66 |
| NRSQ | 40,53 | 0.0 | **40,53** | ASTER GDEM 30 m | 61.162/−45.442 | ±20 | ±0,00 |
| PGEO | 756,26 | 756,26* | **761,62** | HRDEM DTM 1 m | 53.815/−122.828 | ±3 | +5,36 |
| PINA | 247,63 | 247 | **237,66** | HRDEM DTM 1 m | 50.163/−96.066 | ±3 | −9,97 |
| RANK | −17,15 | 0.0 | **−26,41** | HRDEM DSM 1 m | 62.8247/−92.1147 | ±8 | −9,26 |
| SNAP | 406,20 | 0.0 | **418,93** | HRDEM DSM 1 m | 63.575/−110.866 | ±10 | +12,73 |
| SNKQ | −11,22 | 0.0 | **−28,75** | HRDEM DTM 1 m | 56.536/−79.231 | ±3 | −17,53 |
| TALO | 0,19 | 0.0 | **−20,29** | HRDEM DSM 1 m | 69.541/−93.554 | ±8 | −20,48 |
| TPAS | 240,59 | 236 | **235,59** | ASTER GDEM 30 m | 53.994/−100.941 | ±20 | −5,00 |
| WHIT | — | 0.0 | **749,16** | HRDEM DSM 1 m | 61.009746/−135.222545 | ±10 | — |
| YKNF | 179,14 | 0.0 | **135,06** | HRDEM DTM 1 m | 62.520/−114.313 | ±3 | −44,08 |

`*` PGM-Wert aus der Kalibrierung 2026-10-12, Kopf in dieser Runde nicht erneut geladen.
`†` MCGR: 102 im Kopf 2013/2014 bei −155,6; `0.0` im Kopf 2022 bei −154,42 (55 km östlich).
`‡` FSMI: DTM-Mosaik endet am Punkt; nächstgelegene DTM-Zelle 2 km nördlich (206,33 m),
DSM am Punkt 208,64 m → Differenz 2,3 m.

## Befunde (gemessen, nicht geglättet)

- **Die Prämisse „16 von 24 = 0.0" stimmt nicht exakt.** Die Köpfe zeigen **17** Stationen
  mit `0.0` bzw. `0` (inkl. **FYKN**, das die Aufgabe zu den 8 bekannten zählt) und **7**
  mit echter Zahl (ATHA, EKAT, GAKO, MCGR [epochenabhängig], PGEO, PINA, TPAS). MCGR ist
  der Sonderfall: 2013/14 `102`, 2022 `0.0` bei anderer Koordinate.
- **PGM `0.0` ist nominal, nicht null-echt.** Alle 17 liegen über Gelände zwischen
  −29 m und +749 m ellipsoidisch. `0.0` bleibt in der Quelle `0.0`; die Survey ersetzt
  es nicht durch eine Null, sondern durch eine gemessene Zahl mit Quelle.
- **Die größten Korrekturen sind Standort, nicht DEM:** YKNF **−44 m** (Tabelle = Stadt
  Yellowknife 206 m H, Station liegt auf 162 m H), TALO **−20 m** (Tabelle = Flughafen
  28 m H, Station an der Küste), SNKQ **−17,5 m** (Tabelle = Flughafen 32 m H, Station
  am Ufer), KAPU **+18,4 m** (Tabelle = Stadt 218 m H, Station auf 236 m H).
- **HRDEM 1 m bestätigt ATHA** (550,22 vs. 549,82) und **PGEO** (761,62 vs. 756,26,
  +5,4 m). PINA bleibt mit −9,3 m der größte Widerspruch gegen eine PGM-Zahl; Ursache
  ist der Punkt (Gemeinde vs. Instrument), nicht das Raster.
- **ASTER 30 m ist brauchbar, wo das Gelände flach ist:** TPAS trifft PGM auf 0,4 m,
  YKNF bestätigt HRDEM auf 3 m. In Relief/vegetation (GILL, KUUJ) bleibt sie ±20 m.
- **Der Rev-B-Punkt ist nicht der Instrumentenpunkt.** FYKN: Rev-B 2,5 km östlich des
  Flughafens, 148 m H statt 130 m; CHBG: PGM-Koordinate auf 0,01° gerundet, 1,5 km
  entfernt, 21 m Höhenunterschied. Die Höhe ist nur so gut wie die Koordinate.

## Was `pending` bleibt

| Station | warum kein neuer Punktwert |
|---|---|
| **MCGR** | PGM-Koordinate 2013/14 = Rev B (−155,6), 2022 = −154,42 (55 km östlich); PGM-Höhe 102 vs. `0.0`. Ohne Entscheid, welcher Kopf der Standort ist, kein eindeutiges `h`. |
| **SNAP** | nur HRDEM-DSM (447,73 m H), kein DTM; 418,93 m h ist eine **Obergrenze** über Borealkrone, nicht Gelände. |
| **WHIT** | nur HRDEM-DSM (741,57 m H), kein DTM; 749,16 m h ist Obergrenze im Tal (Gebäude/Baum). |
| **FSMI** | DTM-Mosaik endet am Punkt; 176,95 m h nutzt die nächstgelegene DTM-Zelle 2 km N. |
| **GILL · KUUJ · NRSQ** | nur ASTER 30 m (±20 m); HRDEM deckt Manitoba/Nord-Quebec nicht. |
| **EKAT** | DTM am Punkt nodata; DSM 464,86 m H → 437,63 m h ist Obergrenze. |

Diese sechs Zeilen tragen eine Zahl **mit** Unsicherheit, aber keine 1-m-Geländehöhe.
Sie sind in der Stationsdatei als „vorläufig" zu führen, nicht als ersetzt.

## Risse / Fallen (gemessen)

- **BBox-Mittelung:** ±0,0005° verschob ATHA um 1,6 m; Punktabfrage mit ±0,00001° ist
  reproduzierbar (571,40 / 571,29 / 571,17 m je nach 0,0002° Schritt).
- **CGVD2013 ≠ EGM2008:** < 1 m in Kanada; nicht herausgerechnet, in ±3 m enthalten.
- **ASTER/SRTM sind EGM96-nah:** EGM96↔EGM2008 ~0,1 m; vernachlässigt.
- **3DEP-EPQS liefert NAVD88**, nicht ellipsoidisch; die Konversion mit EGM2008 ist
  Pflicht, sonst liegt Alaska um 5–16 m falsch.
- **PGM-Kopf ist nicht per se der Standort:** ATHA mischt Stadtkoordinate und
  AUGO-II-Höhe; MCGR wechselt die Koordinate zwischen Epochen.
- **Ein DSM ist keine Stationshöhe.** Wo nur DSM vorliegt, ist die Zahl eine Obergrenze;
  sie wird hier so genannt und nicht als „gemessen" verkauft.

## Ablage

- Diese Survey: `docs/surveys/survey-2026-10-20-asi-stationshoehen.md`.
- Quellnotizen (Werkstatt): `recherche/funde/notizen/2026-10-12-themis-asi-stations.md`
  und `recherche/funde/notizen/2026-10-12-calibration.md` — dort datierter Nachtrag,
  keine Überschreibung der 2026-10-12-Werte.
- Code `src/archivar/themis_asi.rs` bleibt in dieser Runde unverändert: die Änderungen
  liegen teils innerhalb der Unsicherheit und brauchen das Operator-Wort (Muster
  2026-10-12). Die Survey nennt die Kandidaten.
