<!--
  title: Kadenz-Ernte — drei native Bänder: EUVS 60 s, AIA 12 s, TEC 900 s
  class: survey
  date: 2026-10-20
  sha256: 592492cb276c874cf9356484d9b62decf9da0656eec169902ecad94553a960e2
  status: live
  see-also: docs/surveys/survey-2026-10-20-sonne-erde-kette.md docs/surveys/survey-2026-10-20-band-stack.md
-->
# Kadenz-Ernte — drei native Bänder

**Anlass:** Drei Fäden verlangten ein Band in seiner eigenen Kadenz statt eines
Tagesmittels: GOES/EUVS Lyman-α mit 60 s, SDO/AIA 171 Å mit 12 s und TEC mit
900 s. Vor jedem Laden steht die Zahl: Dateizahl, `Content-Length` je Datei,
Summe, erwartetes Asset. Kein Massendownload ohne diese Zeile.

**Grenze vorab:** Die Ernte lädt Rohdateien unter `sources/<netloc>/` und
kompiliert sie lokal. Es gibt **keinen CDN-Upload** und **keinen Edit an
`phi/sources.φ`** in diesem Lauf; die Registerblöcke unten sind Vorschlagstext.
Die Zeitstempel der Assets sind TDB-Sekunden seit J2000; die UTC-Angaben im
Text sind daraus mit dem Sprungsekunden-Offset des jeweiligen Jahres
zurückgerechnet (`t_unix = t_tdb + 946728000 − 32,184 − ΔAT`).

## Kanäle (gemessen, nicht angenommen)

- `ncei.noaa.gov` / GOES-15 EXIS EUVS, `geuv-l2-avg1m` V5: HTTP 200, 11
  Jahresdateien 2010–2020, `Content-Length` 40.618.289 … 232.057.857 B. Der
  Katalog endet mit GOES-15 am 2020-03-04; `goes16/geuv-l2-avg1m` antwortet
  **404** (`science/euvs/goes16/` ebenso). Das Produkt ist 1-min-*gemittelt*
  (Readme §2.2: „corrected 1-minute Lyman α irradiances"); der rohe
  10-s-Datensatz `geuv-l2-ir10s` existiert daneben und wurde nicht geladen.
- `jsoc.stanford.edu` / `aia.lev1_euv_12s[? WAVELNTH=171 ?]`: `rs_list` mit
  `@31d@12s` liefert **status 6** („out of memory for query result") — die
  Monatsabfrage bricht ab. `@1d@12s` liefert 7.160–7.195 Sätze je Tag in
  ~17 s. Der Jahreslauf braucht daher 365 Tagesabfragen, nicht 12.
- `cdaweb.gsfc.nasa.gov` / `gps/tec15min_igs/2024`: HTTP 200, 92 Tagesdateien,
  `Content-Length` 1.158.992 … 1.204.437 B. Jede Datei trägt **96** Epochen
  (96 × 900 s = 86.400 s), `tecUQR` als REAL4 über `lat[71] × lon[73]`; der
  Test im Compiler bestätigt `dim_sizes = [71, 73]`, 96 Records.
- `lasp.colorado.edu` / EVE L2B (nur als Gegenprobe gemessen): Stundendatei
  2014-05-30 = **405.066 B**, 2024-07-18 = **764.217 B**, beide HTTP 200. Bei
  8.760 h/Jahr sind das **3,5–6,7 GB/Jahr roh**; 10,5 Jahre wären
  **37–70 GB**. Deshalb nicht der gewählte Arm, nicht wegen der Lizenz
  (`other-attribution`, Register-Block `eve`).
- Lizenz-Zeilen aus dem Register (nur gelesen): `goes_euvs` = `PD-USGov`
  (`https://gml.noaa.gov/about/disclaimer.html`), `aia` = `CC0-1.0`
  (`http://jsoc.stanford.edu/JSOC_Data_Policy.html`), `cdaweb_tec` =
  `CC0-1.0` (`https://spdf.gsfc.nasa.gov/data_use_policy.html`).

## Volumen vor dem Laden

| Faden | Dateien | Roh-Bytes | Asset-Bytes | Kadenz | Zeitraum |
|---|---:|---:|---:|---:|---|
| EUVS Lyman-α | 11 | **2.303.358.283** | **100.166.828** | 60 s | 2010-04-07 .. 2020-03-04 |
| AIA 171 Å | 365 JSON | **125.459.216** | **51.973.368** | 12 s | 2013-12-31 .. 2014-12-31 |
| TEC UQR | 92 CDF | **107.289.845** | **1.464.840.200** | 900 s | 2024-05-01 .. 2024-07-31 |

Die Asset-Größe ist keine Schätzung: sie folgt aus `n × record_bytes + 8`
(20 B für `GEUV`/`AIA1`, 32 B für `CTEC`) und ist nach dem Schreiben an der
Datei gemessen. Roh = Summe der `Content-Length` (EUVS) bzw. der lokalen
Dateigrößen (AIA-Cache, TEC-CDF).

## Faden 1 — GOES-15 EUVS Lyman-α, 60 s

**Asset:** `sources/ncei.noaa.gov/compiled/geuv_l2_avg1m/geuv_l2_avg1m_lya1216_2010_2020.bin`
— 5.008.341 Sätze, 100.166.828 B, `sha256 72c2579c82fd4c836f9e6ed0b0f74ff5b2826991ece9acea6778d9f64d22bf8a`.

| | |
|---|---|
| n | **5.008.341** |
| erster Satz | 2010-04-07T19:44:57Z (`t_tdb 323.941.566`) |
| letzter Satz | 2020-03-04T18:16:00Z (`t_tdb 636.638.409`) |
| Kadenz | **60 s** modal (3.232 von 5.008 geprüften Schritten exakt 60 s, 3 weitere 60,001 s; alle übrigen >60 s = Fülllücken) |
| Feld | `goes_euvs_lya1216`, W/m² in 1 AU, Komponente 2 |
| Werte | erster 6,096e-3, letzter 5,702e-3 W/m² |

**Was fehlt, gemessen:** Der Katalog beginnt am 2010-03-25, das Asset am
2010-04-07 — die ersten 13 Tage tragen Flags (Checkout/Eklipse) und werden
verworfen. Von 406.080 Zeilen des Jahres 2010 bleiben 366.562 (90,3 %); 2012
bleiben 492.806 von 527.040 (93,5 %). Die Lücken sind **benannt**, nicht
interpoliert. Der Faden endet am 2020-03-04, weil GOES-15 dort endet; die
angestrebten 10,5 Jahre sind **9,91 Jahre**.

**Nicht geladen:** GOES-14 `geuv-l2-avg1m` y2009 (77.353.873 B) und y2010
(195.284.980 B) liegen als Gegenprobe im Ordner, sind aber Kalibrierungsphase
(Readme: „GOES-14 measurements occurred primarily from 2018 through 2020")
und wurden nicht kompiliert. GOES-16/17/18 tragen in diesem Archiv kein
EUVS-1-min-Produkt (404).

## Faden 2 — AIA 171 Å, 12 s, 2014

**Asset:** `sources/jsoc.stanford.edu/compiled/aia_171_12s_2014.bin` —
2.598.668 Sätze, 51.973.368 B, `sha256 62adb6c8617d3c0950eeb70cef759fb02ba69dce96d35d4dd93265c3951ae604`.

| | |
|---|---|
| n | **2.598.668** |
| erster Satz | 2013-12-31T23:59:36Z (TAI-Rand des 1-d-Fensters) |
| letzter Satz | 2014-12-31T21:46:00Z |
| Kadenz | **12 s** modal (1.827 von 2.599 geprüften Schritten exakt 12 s; 11/13 s = Jitter, größere Sprünge = fehlende Bilder) |
| Feld | `aia_171_dn`, DN/s (`DATAMEAN / EXPTIME`), Komponente 2 |
| Werte | erster 121,745, letzter 101,448 DN/s |

**Der 368-MB-Punkt:** Ein Kanal kostet 20 B je Satz, also
**51.973.368 B**. Die im Auftrag genannten ~368 MB/Jahr sind der **7-Kanal**-Satz
(2.598.668 × 7 × 20 + 8 = **363.813.528 B**), nicht ein Kanal. Gemessen wurde
ein Kanal (171); die sechs anderen Bänder wären sechs weitere Cache-Läufe à
125 MB JSON und wurden nicht geladen.

**Nicht geladen:** die Pixel. Der `exportdata`-Weg für ein lev1-FITS antwortet
**HTTP 500**; die Bildgröße ist damit in diesem Lauf **nicht gemessen** und
die Pixel-Ernte bleibt `pending`. Der Kanal ist die Serie der
Scheibenmittel, kein Bildstapel.

## Faden 3 — TEC UQR, 900 s, Mai–Juli 2024

**Asset:** `sources/cdaweb.gsfc.nasa.gov/compiled/cdaweb_tec_2024_05_07.bin` —
45.776.256 Sätze, 1.464.840.200 B,
`sha256 39c5c0b6e44f344b4e6d98ea2256f51124a83ed25475d4552d984574b2eaee2a`.

| | |
|---|---|
| n | **45.776.256** = 8.832 × 71 × 73 |
| erster Satz | 2024-05-01T00:00:00Z, lat −87,5°, lon −180°, 14,5 TECU |
| letzter Satz | 2024-07-31T23:45:00Z, lat +87,5°, lon +180°, 8,6 TECU |
| Kadenz | **900 s**; 8.831 von 8.831 Schritten exakt 900 s |
| Felder | `cdaweb_tec_tecu` (TECU), `lat` 71 Werte, `lon` 73 Werte |
| Wertebereich | −12,9 … 114,3 TECU |

Der Mai-2024-Arm (493.587.464 B, 15.424.608 Sätze) ist unverändert
mitgeführt; Juni (477.665.288 B, 14.927.040) und Juli (493.587.464 B,
15.424.608) sind neu. Das Asset ist die Konkatenation der drei Monats-Bins;
die Monatsgrenzen sind lückenlos (2024-05-31T23:45 → 2024-06-01T00:00).

**Nicht geladen:** drei weitere Monate bis Oktober. Der Compiler hält alle
Sätze im Speicher; sechs Monate wären 91,5 M Sätze ≈ 2,9 GB RAM zusätzlich
zum CDF-Puffer. Bei 7 GB Gesamtspeicher und paralleler Last wurde bei drei
Monaten (1,46 GB Asset) gestoppt. Die Erweiterung auf sechs Monate ist ein
eigener Lauf mit monatsweiser Kompilation und Konkatenation.

## Bilanz

| | Zahl |
|---|---:|
| geprüft | **3** |
| geladen und kompiliert | **3** |
| mit Beispiel belegt | **3** |
| benannte Lücke | **4** (0,59 Jahre EUVS; 6 AIA-Bänder; AIA-Pixel; 3 TEC-Monate) |

Alle drei Bänder tragen ihre Kadenz nativ: 60 s, 12 s, 900 s. Kein Faden
wurde auf ein Tagesmittel geglättet. Die zwei großen Lücken sind
instrumentell (GOES-15 endet 2020) bzw. speicherbedingt (TEC), nicht
methodisch.

## Vorschlag Registerblöcke (nicht angewandt)

Die vorhandenen Blöcke tragen `goes_euvs_lya1216` mit `ttl 31536000` und
`dt 86400`, `aia_171_dn` mit `dt 86400` und `cdaweb_tec_tecu` mit `dt 86400`.
Für die nativen Bänder schlage ich **eigene Schlüssel** vor, damit die
vorhandenen Tages-Aggregate nicht überschrieben werden. Der Register-Eigner
entscheidet.

```
url https://github.com/omegaflow/sources/releases/download/ncei.noaa.gov/geuv_l2_avg1m_lya1216_2010_2020.bin
format goes_euvs
terms PD-USGov https://gml.noaa.gov/about/disclaimer.html
origin https://www.ncei.noaa.gov/data/goes-space-environment-monitor/access/science/euvs/goes15/geuv-l2-avg1m/
compiler tools/harvest/src/bin/euvs_compiler.rs
at sun
ttl 3600
field goes_euvs_lya1216_pt1m goes_euvs_lya1216_pt1m inverse-square em wm2_1au 60 0.0 0.0
```

```
url https://github.com/omegaflow/sources/releases/download/jsoc.stanford.edu/aia_171_12s_2014.bin
format aia
terms CC0-1.0 http://jsoc.stanford.edu/JSOC_Data_Policy.html
origin http://jsoc.stanford.edu/cgi-bin/ajax/jsoc_info?op=rs_list (aia.lev1_euv_12s, T_OBS/DATAMEAN/EXPTIME, @1d@12s, WAVELNTH 171)
compiler tools/harvest/src/bin/aia_compiler.rs
at sun
ttl 86400
field aia_171_dn_pt12s aia_171_dn_pt12s inverse-square em count 12 0.0 0.0
```

```
url https://github.com/omegaflow/sources/releases/download/cdaweb.gsfc.nasa.gov/cdaweb_tec_2024_05_07.bin
format cdaweb_tec
terms CC0-1.0 https://spdf.gsfc.nasa.gov/data_use_policy.html
origin https://cdaweb.gsfc.nasa.gov/pub/data/gps/tec15min_igs/2024/
compiler tools/harvest/src/bin/cdaweb_tec_compiler.rs
at earth
ttl 86400
field cdaweb_tec_tecu_pt900s cdaweb_tec_tecu_pt900s inverse-square em TECU 900 0.0 0.0
```

## Was quer liegt

- **Der Auftrag nennt 368 MB für einen Kanal.** Gemessen sind es 51.973.368 B
  je Kanal; 363.813.528 B sind der 7-Kanal-Satz. Die Zahl im Auftrag trifft
  sieben Bänder, nicht eines.
- **10,5 Jahre sind mit diesem Katalog nicht erreichbar.** GOES-14 ist nur
  sporadisch, GOES-16+ fehlt im EUVS-Archiv; die Reihe endet 2020-03-04 nach
  9,91 Jahren. Ein Anschluss an GOES-R verlangt einen anderen Katalog und
  eine Kreuzkalibrierung — beides gemessen offen (`404`).
- **JSOC-Monatsabfragen sterben an ihrer eigenen Größe** (status 6). Der
  12-s-Lauf ist nur tageweise möglich; das ist kein Timeout, sondern die
  Speichergrenze der Abfrage.
- **AIA endet am 2014-12-31 um 21:46**, nicht um 23:59. Die letzten ~2 h des
  Jahres fehlen in `aia.lev1_euv_12s`; das ist eine Bildlücke, kein
  Parser-Ausfall (die Tagesabfrage liefert 7.195 Sätze).
- **TEC-UQR wird negativ** (min −12,9 TECU). Das ist die ungemappte
  Schrägdurchgangs-Größe; ein Null- oder Betrags-Schnitt wäre eine stille
  Glättung und wurde nicht angewandt.
- **EVE: die 3,5–6,7 GB/Jahr waren das Spektren-Produkt (`EVS`), nicht die
  Linien.** Das Linien-Produkt `EVL_L2B_*_008_01.fit.gz` ist **0,4 MB/Tag ≈
  145 MB/Jahr** — billiger als GOES EUVS (2,30 GB für 9,91 Jahre). Die
  Lizenz bleibt `other-attribution` (LASP), der Zugang ist offen.

## Nachtrag — offene Fenster und Alternativen (2026-10-20, archive_search + Probe)

| Fenster | Befund | Route |
|---|---|---|
| **EUVS 2020-04 … 2020-12** | GOES-16/17 `EXIS-L1b-SFEU` im offenen S3 erst **2021+**; NCEI `avg/2020/` nur **Jan–Mar** (GOES-15 endet 2020-03) | **SDO/EVE L2B `EVL`** bei LASP: täglich `EVL_L2B_YYYYDDD_008_01.fit.gz`, **60 s**, 1440 Zeilen, **MEGS-P Lyman-alpha 121–122 nm**; 2020-04-29 und 2020-12-31 verifiziert; **~110 MB** für Apr–Dez |
| **EVE L2B** | Verzeichnis offen (`lasp.colorado.edu/eve/data_access/eve_data/products/level2b/`), Tagesordner 2014 und 2020 vorhanden | 0,4 MB/Tag; 2010–2020 möglich; MEGS-A fehlt nach 2014, MEGS-P/Lyman-α bleibt |
| **AIA-Pixel** | JSOC `exportdata` = 500 | **Helioviewer `getJP2Image`** = **200**, 1,08 MB JP2 (2014-01-01, sourceId 10); ein Jahr bei 12 s ≈ **2,6 TB** → nur Cutouts/Fenster, nicht Vollscheibe |
| **TEC Monate 4–6** | kein Quellenloch; CDFs sind tagesweise | **monatsweise** verarbeiten und anhängen; 3 Monate = 1,46 GB, 6 Monate = 2,9 GB — Streaming statt RAM |

**Konsequenz:** Das fehlende halbe Jahr EUVS ist **schließbar** — mit EVE L2B
`EVL` (~110 MB für Apr–Dez 2020), als **unabhängige Linie**, nicht als Spiegel.
Die Cross-Kalibration läuft über die Überlappung 2010–2020 mit GOES-15. Der
„zweite Arm" EVE ist im Linien-Produkt billig; teuer sind nur die Spektren.
