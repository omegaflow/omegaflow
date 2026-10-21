<!--
  title: Survey — Hunga Tonga 2022: dieselbe Front in sechs Netzen
  class: survey
  date: 2026-10-20
  sha256: 0b9bade896a30246133c11246ca8a9a78a64c9c2ddf185c15a053024c399aa49
  status: live
  see-also: docs/paper/tonga-lamb-crosscheck.md tools/measure/src/bin/hunga_tonga_netz_probe.rs
-->
# Survey — Hunga Tonga-Hunga Ha'apai: die Front in sechs Netzen

**Frage.** Erscheint das Ereignis vom 2022-01-15 04:14:45 UTC in mehreren
Netzen — Seismik, Luftdruck/Infraschall, Tsunami, Ionosphäre, Blitz,
Aerosol/Thermik — mit konsistenter Laufzeit und Richtung? Das ist der
Übertragbarkeitstest für das „Netz als Instrument".

**Grenze vorab.** Alle Zeiten sind UTC. „Dieselbe Front" heißt hier: derselbe
Quellzeitpunkt und dieselbe radiale Geometrie. Es heißt **nicht** dieselbe
Geschwindigkeit. Die elastische Welle, die Lamb-Welle, der Ozean und die
Ionosphäre tragen verschiedene Phasen; gemessen wird, ob jede Faden ihren
eigenen physikalischen Laufzeit- und Richtungstest besteht — nicht, ob alle
zur selben Sekunde ankommen.

**Probe.** `tools/measure/src/bin/hunga_tonga_netz_probe.rs` (neu, dieser
Survey). Rohdaten werden zur Laufzeit aus den frei zugänglichen Routen gelesen; es gibt
keinen CDN-Upload. `phi/sources.φ` bleibt unberührt.

## Bestandsaufnahme (gemessen 2026-10-20, nicht angenommen)

| Faden | Quelle | Abdeckung | Lizenz | Kadenz | Zugang |
|---|---|---|---|---|---|
| Seismik | EarthScope FDSN `dataselect`, BHZ | 8 Stationen, 757–9.795 km, Jan 2022 | EarthScope/FDSN, Attribution | 40–100 Hz | HTTP 200 |
| Luftdruck | NOAA CO-OPS `air_pressure` | 5 Pazifik-Stationen, Jan 2022; Pago Pago ohne Luft | US-Government, public domain | 6 min | HTTP 200 |
| Luftdruck (2) | Kyoto-A 1 Hz, Zenodo 8098323 | Jan 2022, 30 Tagesdateien | CC BY-NC 4.0 | 1 Hz | Hausmessung, nicht neu gezogen |
| Infraschall | BGR IS52 PMCC, Haus-Release | 2022, Detektionsliste | Ursprung BGR, Lizenz im Quellenregister | ~300 s | HTTP 200 (6.484.088 B) |
| Tsunami | NOAA CO-OPS `water_level`, MSL | 6 Pazifik-Stationen, Jan 2022 | US-Government, public domain | 6 min | HTTP 200 |
| Ionosphäre | CDAWeb HAPI `GPS_TEC15MIN_IGS` | globale 71×73-Karte, Jan 2022 | NASA SPDF / IGS, Attribution | 15 min | HTTP 200 |
| Blitz | NOAA GOES-17 `GLM-L2-LCFA`, AWS S3 | 2022-01-15 und 01-13, je 226 Granulen 03:45–05:00 | NOAA Open Data, public domain | 20 s | HTTP 200 |
| Aerosol/Thermik | **GOES-17 ABI FDCF/AODF**, AWS S3 | Vollbild, Jan 2022 | NOAA | 10 min | **keyless** (HTTP 200) |
| Aerosol/Thermik | **TROPOMI `AER_AI`**, MEEO S3 `meeo-s5p` | 2022-01-15, 14 Orbits | ESA/Copernicus | Orbit | **keyless** (HTTP 200, 175 MB) |
| Aerosol/Thermik | NASA FIRMS API | Jan 2022 vorhanden | NASA | Tag/Orbit | **Key fehlt**: HTTP 400, `Invalid MAP_KEY` — Route keyed, Daten nicht absent |
| Aerosol/Thermik | Copernicus TROPOMI OData | Jan 2022 im Katalog (z. B. `S5P_RPRO_L2__AER_AI_20220115T200153`) | Copernicus | Orbit | **Key fehlt**: HTTP 401, `Token not found` — Route keyed, Daten nicht absent |

Die TROPOMI-Produkte sind im öffentlichen Katalog gelistet (`Online: true`), der
Download über Copernicus OData verlangt einen Token; FIRMS verlangt einen `MAP_KEY`.
**Das ist ein Zugangsbefund, kein Datenbefund.** Beide Fäden sind keyless erreichbar:
GOES-17 ABI FDCF/AODF auf `noaa-goes17.s3.amazonaws.com` und TROPOMI `AER_AI` auf
`meeo-s5p.s3.amazonaws.com` (siehe Nachtrag). Der erste Lauf hatte nur die
API-Routen geprüft und daraus `absent` gemacht — das war zu kurz.

## Methode

- **Quellzeit** `t0 = 2022-01-15 04:14:45 UTC`; Epizentrum 20,536° S,
  175,382° W.
- **Seismik.** Echte miniSEED-Wellenform, Fenster 02:15–06:15 UTC (±2 h).
  Dataselect liefert mehrere Location-Codes; die Probe trennt die Records am
  miniSEED-Record-Längenfeld und behält genau einen Stream (`00.BHZ`).
  Detektor: 20-s-RMS, Basis 02:15–03:30, Schwelle `max(6σ, 0,5·Basis)`.
  Laufzeit-Fit durch den Ursprung, zusätzlich nur im Band 2–6 km/s.
  NULL: derselbe Detektor am 2022-01-13, gleiches Fenster.
- **Luftdruck.** 6-min-Reihe, 1,5-h-Median abgezogen (synoptische Drift),
  Onset im Fenster „vorhergesagte Lamb-Ankunft ±30 min", Schwelle
  `max(4σ, 0,2 hPa)`. Amplitude ist der größte Betrag im folgenden 2-h-Fenster.
  NULL: 2022-01-13, gleiche Stationen und Fenster.
- **Tsunami.** 6-min-Wasserstand (MSL). Gezeitenmodell aus Konstante plus
  M2, S2, K1, O1, nur auf dem Vorbeben-Abschnitt gefittet; danach wird das
  Modell überall abgezogen. Onset: erste gleichsinnige Überschreitung von
  `max(2,5σ, 0,06 m)` der dominanten Auslenkung in 24 h.
  NULL: 2022-01-13, gleiche Prozedur.
- **Ionosphäre.** 3×3-Zelle um Tonga, TEC relativ zum globalen Tagesmittel
  (entfernt die globale solar/geomagnetische Skalierung), verglichen mit dem
  Mittel der Tage 01-13, 01-14, 01-16, 01-17. Onset: `max(3σ, 20 %)`.
  NULL: 2022-01-13 gegen die drei übrigen Vergleichstage.
- **Blitz.** GOES-17-Granulen 03:45–05:00 UTC, 500-km-Ring um das Vent.
  Onset: erste Granule mit mindestens 3 Blitzen und Schwerpunkt ≤400 km.
  NULL: 2022-01-13, gleiches Fenster.
- **Infraschall.** Nächste BGR-Rückazimut-Detektion zur vorhergesagten
  direkten Lamb-Ankunft; Richtungstest als Differenz zum Großkreis-Peil
  Station→Vent. NULL: gleiche Regel am 2022-01-13.

## Tabelle

| Faden | Onset (UTC) | Amplitude | Lag (s) | Distanz (km) | Null |
|---|---|---|---|---|---|
| Seismik IU.AFI | 04:17:40 | 15.695 counts (Basis 2.464) | 175 | 831 | kein Onset |
| Seismik II.MSVF | 04:18:20 | 60.937 counts (Basis 5.125) | 215 | 758 | kein Onset |
| Seismik IU.SNZO | 04:24:20 | 10.538 counts (Basis 1.776) | 575 | 2.495 | kein Onset |
| Seismik IU.NWAO | 04:44:20 | 3.374 counts (Basis 859) | 1.775 | 6.678 | kein Onset |
| Seismik IU.COLA | 04:56:40 | 4.734 counts (Basis 661) | 2.515 | 9.795 | falscher Onset 06:01 |
| Seismik IU.HNR | 05:32:00 | 6.445 counts (Basis 2.304) | 4.635 | 2.921 | falscher Onset 04:25:40 |
| Seismik IU.ANMO | 04:05:00 (vor t0) | 1.286 counts (Basis 667) | −585 | 9.539 | falscher Onset 04:34:20 |
| Seismik IU.MAJO | absent | — | — | 8.010 | kein Onset |
| Druck Kwajalein | 07:18:00 | +2,00 hPa | 10.995 | 3.746 | 09:48, +0,30 hPa |
| Druck Wake Island | 08:24:00 | +2,20 hPa | 14.955 | 4.848 | 10:12, +0,20 hPa |
| Druck Kahului | 08:36:00 | +1,90 hPa | 15.675 | 5.051 | 12:42, −0,30 hPa |
| Druck Guam Apra | 09:00:00 | +2,20 hPa | 17.115 | 5.781 | 11:12, +0,30 hPa |
| Druck Honolulu | 08:36:00 | +1,60 hPa | 15.675 | 5.033 | 10:06, −0,30 hPa |
| Tsunami Pago Pago | 05:30:00 | 0,42 m | 4.515 | 857 | 15:24, 0,26 m |
| Tsunami Kwajalein | 09:30:00 | 0,15 m | 18.915 | 3.746 | 05:18, 0,25 m |
| Tsunami Wake Island | 10:36:00 | 0,28 m | 22.875 | 4.848 | 13:48, 0,08 m |
| Tsunami Honolulu | 12:00:00 | 0,55 m | 27.915 | 5.033 | 16:36, 0,14 m |
| Tsunami Guam Apra | 15:18:00 | 0,28 m | 39.795 | 5.781 | 06:06, 0,13 m |
| Tsunami Kahului | 06:00:00 | 0,39 m | 6.315 | 5.051 | 14:24, 0,24 m |
| TEC Tonga-Zelle | 05:00:00 | −57 % (Zelle 10,7 TECU) | 2.715 | 0 (über Kopf) | kein Onset, max. +15 % |
| GLM Vent-Cluster | 04:14:42 | 4 Blitze/20 s, 3,3·10⁻¹² J | −2 | 0–47 | kein Cluster (17 Einzelblitze) |
| BGR IS52 | 15:30:00 | v_app 352 m/s, az −6,7° | +1.316 | 11.995 | 15:15, az-Residuum −113,1° |

Lesart der Spalte **Lag**: Sekunden nach `t0`; bei BGR Sekunden nach der
vorhergesagten direkten Lamb-Ankunft (15:08:04 UTC), nicht nach `t0`.

## Laufzeit

**Seismik.** Fünf der acht Stationen tragen einen Onset mit scheinbarer
Geschwindigkeit 3,52–4,74 km/s; der Fit durch den Ursprung im Band 2–6 km/s
ist **3,87 km/s** (5 Stationen). Das ist das Rayleigh-Oberflächenwellenband,
nicht die P-Phase. Der Fit über alle sieben Onsets (1,46 km/s) ist durch HNR
(0,63 km/s), den Vor-Ereignis-Treffer ANMO (−16,29 km/s) und MAJO (absent)
verzerrt und wird nicht als Laufzeit zitiert. Der NULL-Tag liefert drei
Treffer (HNR 04:25:40 mit 4,46 km/s, COLA 06:01, ANMO 04:34:20); einer davon
liegt im Oberflächenwellenband. Der seismische Detektor ist damit nicht
sauber, aber die fünf Ereignis-Onsets sind untereinander konsistent.

**Luftdruck.** Fünf Stationen, Fit **0,328 km/s** — 7 % über der
Lamb-Phasengeschwindigkeit 0,306 km/s. Die Onsets liegen 13–29 min **vor**
der vorhergesagten Front, weil der Detektor die erste Überschreitung der
breiten Druckstörung nimmt; das Maximum liegt nach der Hausmessung bei
+42…+208 s (Kwajalein/Wake +208 s, Guam +42 s). NULL: fünf Treffer, aber
Amplituden 0,20–0,30 hPa gegen 1,60–2,20 hPa am Ereignistag.

**Tsunami.** Vier der fünf Stationen im Band 0,1–0,3 km/s geben **0,170 km/s**
(Pago Pago, Kwajalein, Wake, Honolulu); der Fit über alle fünf ist 0,178 km/s.
Vorhergesagt sind ~0,20 km/s für eine 4-km-Wassertiefe. Kahului fällt mit
0,80 km/s (Onset 06:00) heraus. Der NULL-Tag erzeugt ebenfalls drei Treffer
im selben Band (Fit 0,128 km/s), aber ohne gemeinsame Phase und mit
Amplituden 0,08–0,26 m; die Ereignis-Onsets Pago Pago 05:30, Kwajalein 09:30
und Wake 10:36 liegen nahe der 0,2-km/s-Vorhersage, Honolulu 12:00 eine gute
Stunde danach.

**Ionosphäre.** Der TEC-Onset liegt 45 min nach `t0`, das Maximum −57 %
gegenüber dem Vergleichsmittel um 06:30. Bei 15-min-Raster und globaler
IGS-Glättung ist das keine akustische Ankunft (die wäre ~10–20 min); gemessen
ist eine ionosphärische Störung, nicht ihre erste Wellenfront.

**Blitz.** Der GLM-Cluster beginnt 04:14:42, also 2 s vor `t0` (Granulen
sind 20 s lang; die Eruption fiel in die Granule). Schwerpunkt 20–47 km vom
Vent; das ist eine Quellort-Aussage, keine Laufzeit.

**Infraschall.** Die nächste Detektion liegt 1.316 s nach der vorhergesagten
direkten Lamb-Ankunft, bei 107,7° gegen 114,4° vorhergesagt (−6,7°). Die
PMCC-Liste hat ~300-s-Raster; 1.316 s ist die nächste Detektion, keine
matched-filter-Ankunft. Der NULL-Tag liefert eine Detektion 416 s nach der
Vorhersage, aber bei 1,3° (−113,1° Residuum) — zeitlich nah, richtungslos.

## Verdikt

**Ja, mindestens drei Phänomene.** Fünf Fäden tragen einen eigenen Onset mit
konsistenter Laufzeit **und** radialer Richtung: Seismik (3,87 km/s), Druck
(0,328 km/s), Tsunami (0,170 km/s), GLM (Quellcluster am Vent) und
Infraschall (Richtung −6,7°). Die Ionosphäre trägt eine TEC-Anomalie mit
57 % Amplitude und sauberem NULL, aber der Onset ist mit 45 min und 15-min-
Raster keine Frontmessung. **Thermik am Vent ist gemessen absent:** FDCF
liefert am Vent Fill (`Mask=150`), FIRMS VIIRS keine Detektion im 0,5°-Fenster;
die 17 Vollbild-Pixel sind global, nicht Hunga Tonga. Aerosol bleibt über
AODF/AER_AI offen.

Das „Netz als Instrument" überträgt sich damit **auf den Quellzeitpunkt und
die Geometrie**, nicht auf eine gemeinsame Geschwindigkeit. Jeder Faden
besteht seinen eigenen Test: elastisch 3,9 km/s, atmosphärisch 0,33 km/s,
ozeanisch 0,17 km/s. Die Abweichungen (Druck +7 %, Tsunami −15 %) sind
physikalisch lesbar; die Ausreißer sind benannt, nicht geglättet.

## Nachtrag 2026-10-21 — Aerosol/Thermik war nicht absent

Der erste Lauf prüfte nur die API-Routen (FIRMS `MAP_KEY`, Copernicus Token) und
machte daraus `absent`. Die Daten selbst liegen keyless. Gemessen mit GOES-17
ABI FDCF (`noaa-goes17.s3.amazonaws.com/ABI-L2-FDCF/...`), Vollbild-Skalare:

| Lauf (UTC) | Feuerpixel | max FRP (MW) | max Fire-T (K) |
|---|---:|---:|---:|
| 2022-01-15 04:00–04:09 | 14 | 2.759,860 | 1.134,167 |
| **2022-01-15 04:10–04:19 (t0)** | **17** | **2.833,355** | **1.213,368** |
| 2022-01-15 04:20–04:29 | 19 | 2.670,716 | 2.330,347 |
| 2022-01-15 04:30–04:39 | 14 | 2.360,856 | 1.438,945 |
| **NULL 2022-01-13 04:10–04:19** | **8** | **137,625** | **483,284** |

**Ortsprüfung (2026-10-21).** Der ABI-Fixed-Grid-Index des Vents ist
`x=4832, y=3977` (Scanwinkel `x=0,11873`, `y=−0,07086`; der Index liegt
innerhalb des Vollbilds). An diesem Pixel liefert FDCF an **beiden** Tagen
`Mask=150`, `Power=−9` (Fill), `Temp=4000 K` (Fill) — **kein Retrieval**.
Der keyed FIRMS-Zwilling (`VIIRS_SNPP_SP`, `FIRMS_MAP_KEY` im Haus) liefert
am Ereignis- und NULL-Tag **keine Detektion im 0,5°-Fenster**. Die 17
Vollbild-Pixel sind damit **nicht** Hunga Tonga; der Kontrast ist global,
nicht am Vent.

**Thermik-Verdikt.** Am Vent ist die Thermik in FDCF und VIIRS **gemessen
absent** — nicht wegen eines fehlenden Keys, sondern weil die Produkte dort
nichts tragen. Das ist die belastbare Form von `absent`: Route geprüft,
Schlüssel geprüft, Pixel geprüft. Der Vollbild-Kontrast bleibt als globale
Feuerstatistik stehen, nicht als Vulkan-Signal.

**Terms-Riss.** Der FDCF-Header trägt den String
`license = Unclassified data. Access is restricted to approved users only.`
Der AWS-Bucket liefert die Datei dennoch keyless. Das ist ein **Terms**-Befund,
kein Zugangsbefund — vor einer Register-Zeile muss die NOAA-Open-Data-Policy
gegen diesen Header-String gemessen werden, nicht angenommen.

**Aerosol, zweite Linie.** GOES-17 ABI AODF (keyless, 550 nm):
`max_aod550_sea` = **0,20845** (04:10) und **0,21505** (04:20) am Ereignistag
gegen **0,19078** am NULL-Tag; `std_dev` 0,036 / 0,035 gegen 0,018. Der
AOD-Kontrast ist schwach und das Produkt ist ein Vollbild-Tagesmittel — er trägt
die Thermik, nicht die Plume-Karte.

**TROPOMI AER_AI, dritte Linie.** Keyless auf MEEO S3:
`https://meeo-s5p.s3.amazonaws.com/OFFL/L2__AER_AI/2022/01/15/S5P_OFFL_L2__AER_AI_20220115T044821_20220115T062952_22059_02_020301_20220116T183749.nc`
— HTTP 200, **175.219.152 B**, HDF5/NetCDF4, 14 Orbits am 2022-01-15. Die
Pixel-Extraktion am Vent (Lat/Lon-Index) steht noch aus; der Orbit 22059 deckt
04:48–06:29 UTC, also die erste Stunde nach t0.

**Grenze.** Die Thermik-Aussage ist jetzt vent-lokal (FDCF-Pixel + FIRMS-Fenster).
Die **Aerosol**-Pixel-Extraktion am Vent (TROPOMI AER_AI, Lat/Lon-Index) steht
noch aus; sie ist der nächste Schnitt, nicht `absent`. Die AODF-Zahlen bleiben
Vollbild-Tagesmittel und tragen die Thermik nicht.

## Werkzeuge, Specs und Zwillinge (gemessen 2026-10-21)

Gefunden mit `archive_search` (kein neuer Suchindex): `--cmr` für die
NASA-Kollektionen, `--mwmbl` für offene Produktseiten, `--verdict` für den
Dreistufen-Zugang, `--sniff` für Magic-Bytes. Alle folgenden Zeilen sind
gemessen, nicht aus einer Produktliste abgeschrieben.

| Zweck | Werkzeug / Produkt | Route | Messung |
|---|---|---|---|
| Thermal/Fire | **GOES-17 ABI FDCF** | `noaa-goes17.s3.amazonaws.com` | 200, 2,25 MB, HDF5 |
| Aerosol (NOAA) | **GOES-17 ABI AODF** | dieselbe AWS-Bucket | 200, 9,5–13 MB, HDF5 |
| Aerosol (NASA DT) | **GOES-17 XAERDT_L2_ABI_G17** (Dark Target, 10 km) | CMR `C2859265967-LAADS` | Konzept 200; Daten keyed (Earthdata) — **Zugang, nicht absent** |
| Aerosol UV | **TROPOMI AER_AI L2** | `meeo-s5p.s3.amazonaws.com` | 200, 175.219.152 B, HDF5 |
| Aerosol UV (L3) | **S5P OFFL_L3_AER_AI** | Earth Engine `COPERNICUS_S5P_OFFL_L3_AER_AI` | 200, 109.271 B |
| Aerosol UV (Spiegel) | **TEMIS ABSAAI** | `temis.nl/airpollution/absaai/` | 200, 70.324 B |
| Aerosol UV (unabhängig) | **OMPS NPP/N20 UVAI** | CMR `C1564538427-GES_DISC` | Konzept 200; Daten keyed (Earthdata) — **Zugang, nicht absent** |
| Thermal-Zwilling | **MODIS/VIIRS Active Fire** | FIRMS `…Global_7d.csv` | 200, 4,9 MB (7 d); Historie keyed — **Zugang, nicht absent** |
| Spec/Guide | **GOES-17 FDCF Produktseite** | Earth Engine `NOAA_GOES_17_FDCF` | 200, 111.991 B |

### Livestreams (gemessen 2026-10-21, ~05:20Z)

Die historischen Hunga-Tonga-Dateien liegen in denselben Buckets wie die
aktuellen. Für eine stehende Wache ist deshalb gemessen, welche Routen **live**
sind — nicht nur archiviert:

| Produkt | Route | Letztes Objekt (UTC) | Kadenz | Status |
|---|---|---|---|---|
| **GOES-18 ABI FDCF** | `noaa-goes18` | `2026/294/05/…s20262940500216…` · 05:10:18 | 10 min | **live** |
| **GOES-18 ABI AODF** | `noaa-goes18` | `…s20262940500216…` · 05:12:56 | 10 min | **live** |
| **GOES-18 GLM** | `noaa-goes18` | `…s20262940518400…` · 05:19:18 | 20 s | **live** |
| **GOES-19 ABI FDCF** | `noaa-goes19` | `…s20262940500213…` · 05:10:11 | 10 min | **live** |
| **GOES-19 ABI AODF** | `noaa-goes19` | `…s20262940500213…` · 05:14:11 | 10 min | **live** |
| **GOES-19 GLM** | `noaa-goes19` | `…s20262940519000…` · 05:19:30 | 20 s | **live** |
| **FIRMS MODIS 7d** | `firms.modaps…Global_7d.csv` | `Last-Modified` 04:45:36 | rollierend | **live** |
| **FIRMS VIIRS 7d** | `…SUOMI_VIIRS_C2_Global_7d.csv` | `Last-Modified` 04:47:19 | rollierend | **live** |
| **TROPOMI NRTI AER_AI** | `meeo-s5p` `NRTI/L2__AER_AI/` | `…20261021T001254…` · 01:30:21 | 5 min, ~1 h Latenz | **live** |
| **TEMIS ABSAAI** | `temis.nl` | HTTP 200, kein `Last-Modified` | täglich | live, Latenz ungemessen |
| **OMPS NRT UVAI** | CMR `NPP_NMMIEAI_L2_NRT` | NRT-Kollektion | ~3 h | keyed, live |

**AODF ist tageslichtabhängig.** Der Live-Lauf 05:00–05:09Z liefert
`max_aod550_sea=None`; um 05:00Z ist es über dem GOES-West-Disk Nacht. Ein
Tageslauf (z. B. 18:00Z) trägt den Wert.

**Wichtig für die Wache:** GOES-17 ist der historische Hunga-Tonga-Satellit
(2022), nicht der heutige Live-Satellit. Die aktuelle Westlinie ist **GOES-18**,
die Ostlinie **GOES-19**. Eine stehende Thermik-Wache muss auf G18/G19 zeigen;
die 2022er-Dateien bleiben der Fallbeweis.

**Keyed ist kein Ausschluss.** Ein Schlüssel ist ein Zugangsweg, kein Datenmangel.
Ob er im Haus liegt, entscheidet `bin/secrets_keys`; die Route bleibt gültig, auch
wenn sie nicht keyless ist. XAERDT, OMPS und die FIRMS-Historie sind damit
vollwertige zweite Linien — sie sind **keyed**, nicht `absent`.

**Was das für die Messung heißt.** Die Thermik hat einen echten Zwilling auf
demselben Instrument (AODF) und einen unabhängigen Algorithmus (XAERDT); das
Aerosol hat mit TROPOMI AER_AI, TEMIS und OMPS drei unabhängige UV-Linien.
Damit ist die Hunga-Tonga-Front nicht auf einen einzelnen Sensor gestellt.

**Was nicht trägt.** Die geratenen ATBD-PDF-URLs unter
`goes-r.gov/products/ATBDs/baseline/…` antworten mit HTTP 200, aber `--sniff`
findet **keine PDF-Magic** und `--pdf-text` keinen Textlayer — das sind
Fehlseiten, keine Specs. Sie werden hier nicht zitiert. Verifiziert sind die
Produktseiten (Earth Engine, CMR); die ATBD-PDFs bleiben `unmeasured`.

## Was quer liegt

- **Seismik-Detektor.** Drei von acht Stationen treffen am NULL-Tag; ANMO
  triggert 9 min vor `t0`; MAJO hat keinen Onset; HNR liefert 0,63 km/s.
  Die fünf konsistenten Stationen tragen den Fit, der automatische Detektor
  allein nicht.
- **Druck-Onset vs. Maximum.** Der Onset liegt systematisch 13–29 min vor
  der vorhergesagten Front; erst das Maximum sitzt bei +42…+208 s. Wer nur
  den Onset liest, überschätzt die Geschwindigkeit um 7 %.
- **Tsunami Kahului.** 06:00 UTC bei 5.051 km Entfernung ergibt 0,80 km/s —
  kein Tsunami. Der Punkt ist ein Gezeiten-/Wetterrest, kein Signal; er
  wurde nicht als Tsunami-Onset in den Fit genommen.
- **Tsunami-Amplitude Kwajalein.** 0,15 m am Ereignistag liegen unter dem
  NULL-Maximum 0,25 m. Die Station trägt über die Laufzeit, nicht über die
  Amplitude.
- **TEC-Konfundierung.** Am 2022-01-15 war Kp 3,7–4,3 (OMNI, 1-h). Die
  TEC-Anomalie ist gegenüber dem Vergleichsmittel sauber, aber ein
  geomagnetischer Beitrag ist mit diesem Design nicht zu trennen. Die
  15-min-Karte kann die akustische Ankunft nicht auflösen.
- **GLM.** Nur 22 Blitze im 500-km-Ring in 75 min; GOES-17 steht am westlichen
  Rand der Scheibe, das Produkt ist 20-s-Granulen. Ein dichter „Blitzring"
  ist in dieser Abtastung nicht gemessen — wohl aber ein Cluster am Vent
  (4 Blitze/20 s, Schwerpunkt 27 km). Der NULL-Tag hat 17 Einzelblitze ohne
  Cluster; die Trennung läuft über die Cluster-Bedingung, nicht über die
  Gesamtzahl.
- **BGR.** 1.316 s Abstand zur Vorhersage bei ~300-s-Detektionsraster; die
  Rohwellenform ist zugangsbeschränkt (vDEC). Der Richtungstest (−6,7° gegen
  −113,1° am NULL-Tag) ist der tragende Teil, nicht die Sekunde.
- **Aerosol/Thermik.** Die API-Routen verlangten `MAP_KEY`/Token; die Daten
  liegen keyless auf NOAA-AWS (GOES-17 FDCF/AODF) und MEEO-S3 (TROPOMI AER_AI).
  Die Thermik ist am Vent **gemessen absent** (FDCF Fill, FIRMS 0 Detektionen);
  die AER_AI-Pixel-Extraktion steht noch aus.

## Reproduktion

```
cargo build --release -p omegaflow-measure --bin hunga_tonga_netz_probe
./target/release/hunga_tonga_netz_probe
```

Laufzeit ~2 min (226 GLM-Granulen je Tag, 16 Seismik-Fenster, 24 CO-OPS-
Reihen über zwei Tage, 5 TEC-Tage). Die Rohdaten bleiben zur Laufzeit; dieser Commit trägt
nur Probe und Survey.

Die keyless Aerosol-/Thermik-Routen (Nachtrag):

```sh
curl -O https://noaa-goes17.s3.amazonaws.com/ABI-L2-FDCF/2022/015/04/OR_ABI-L2-FDCF-M6_G17_s20220150410320_e20220150419387_c20220150419494.nc
curl -O https://noaa-goes17.s3.amazonaws.com/ABI-L2-AODF/2022/015/04/OR_ABI-L2-AODF-M6_G17_s20220150410320_e20220150419387_c20220150421190.nc
curl -O https://meeo-s5p.s3.amazonaws.com/OFFL/L2__AER_AI/2022/01/15/S5P_OFFL_L2__AER_AI_20220115T044821_20220115T062952_22059_02_020301_20220116T183749.nc
```

`hunga_tonga_netz_probe` liest die FDCF-Ereignis- und NULL-Datei und gibt die
Vollbild-Skalare und den Vent-Pixel aus; `--live` listet das neueste
GOES-18-Objekt des Tages und misst FDCF/AODF. Die AER_AI-Pixel-Extraktion ist
noch nicht verdrahtet.
