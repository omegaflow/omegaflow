<!--
  title: Untersuchungsauftrag — Abfluss-/Wasserstands-Reihe Trishuli (Flut 2026-08-26)
  class: auftrag
  date: 2026-08-27
  status: pending
  see-also: docs/blatt-pfeil-sturzflut-tibet.md docs/causal-arrow-preregistration.md docs/concepts/livefeed-gate.md
-->

# Untersuchungsauftrag: die Abfluss-Reihe der Trishuli (Flut 2026-08-26)

## Zweck

Für den **präregistrierten Kausalpfeil Niederschlag → Abfluss** braucht es
eine **co-lokale, co-temporale** Fluss-Response-Reihe am Trishuli/Bhote
Koshi nahe dem Ereignis (28.28°N, 85.38°E, 2026-08-26). Ohne diese Reihe
ist der Pfeil nicht messbar (0 honored). Dieser Auftrag sucht die Reihe —
**direkt oder über einen Umweg**.

## Kernregel (0 honored)

Jede Zahl, die der Auftrag bringt, muss eine **gemessene, belegte, offene
Quelle** tragen. Kein Wert wird erfunden, keine Quelle wird geglaubt.
Falsche Längen, fehlender Zeitbezug oder fehlender Ort = `pending`.

## Zielprofil der Reihe

- Ort: Trishuli-/Bhote-Koshi-Oberlauf, so nah an 28.28°N, 85.38°E wie möglich.
- Zeit: Werte um 2026-08-26 (± Tage), idealerweise stündlich–täglich.
- Größe: Abfluss (m³/s), Pegel (m) **oder** daraus ableitbare Flutfläche.
- Erfüllt eine Quelle nur einen Teil, wird das benannt (z. B. „beckenweit,
  nicht co-lokal"), nicht verschwiegen.

## Weg A — direkt (Abfluss-/Pegel-Gauge)

1. **Nepal (DHM)** — Department of Hydrology and Meteorology: Zeitreihen-
   Gauge am Trishuli (Betrawati/Sundarijal, Bhote Koshi). Prüfen: offene
   API oder CSV; sonst `pending`.
2. **WMO / Global Runoff Data Centre (GRDC)** — Weltabfluss-Sammlung:
   bislang tot (404), erneut kurzprüfen, nicht erneut verfolgen wenn tot.
3. **China (Tibet-Seite)** — Oberlauf in Tibet: keine offene Hydrologie
   erwartet; ehrlich als wahrscheinlich unzugänglich markieren.

## Weg B — Satelliten-Altimetrie (Wasserstand aus dem All, Umweg)

1. **DAHITI (TU München)** — schon angeschlossen (`--dahiti`, Key in
   `.secrets.local`). **Auftrag:** den *gesamten* globalen Stationskatalog
   durchsuchen — nicht nur die 3 Koshi-Stationen — nach jeder Station in
   27–29°N, 85–87°E (Bhote-Koshi-/Trishuli-/Sun-Koshi-Korridor). Falls eine
   dort liegt, deren letzte `wse`-Zeit auf **co-temporal** mit 08-26 prüfen.
2. **Hydroweb (LEGOS/CNES)** — Fluss-Pegel aus Altimetrie; nur große Flüsse,
   Trishuli vermutlich nicht gelistet; kurz prüfen.
3. **G-REALM / HydroSat** — Reservoirs/Flüsse aus Altimetrie; überwiegend
   groß; als Kandidaten notieren.

## Weg C — Radar-Hochwasserfläche (Umweg, vielversprechend)

Ohne Gauge kann **die Flutfläche selbst** die Response sein.

1. **Sentinel-1 SAR** (Copernicus Data Space, kostenlos mit Registrierung):
   Rückstreu-Änderung vor/nach 08-26 entlang des Trishuli — dunkle
   Wasserfläche im SAR-Bild ist der gemessene Flut-Umfang. Das ist eine
   **räumliche** Response: Ort ja, Zeit nur diskret (Bildzeiten).
2. **Sentinel-2 optisch** — Flut-Extent über Wasser-Signatur (SWIR);
   Bewölkung im Monsun beachten.
3. **Copernicus EMS Rapid Mapping** — Krisenkarten mit Flut-Ausdehnung;
   Zugang (CEMS) bislang declined — kurzprüfen, nicht erneut verfolgen wenn tot.
4. **Google Earth Engine** — Sentinel-1/-2 + globale Flutdatensätze
   (kostenloser Account nötig); wenn ein Account vorliegt, Flutfläche 08-26
   extrahieren.

## Weg D — indirekte Bestätigung (sekundär)

1. **ICIMOD / DFS / DHM-Sitreps** — zitierte gemessene Pegel-/Abflusswerte.
   **Vorsicht:** ein zitierter Wert ist nur dann brauchbar, wenn seine
   Originalquelle offen und belegt ist; sonst `pending` (kein Hörensagen).
2. **USGS-Nachbarereignis-Gauges** — deckt Nepal nicht; nur als Vergleich.

## Lieferung

Für jeden gefundenen Kandidaten: Quelle + URL, Ort (geo/ICRS), Zeitbereich,
letzter co-temporal-Wert, Verifikation (HTTP-Code), und das Feld, das die
Reihe trägt (`hydrosphere_river_flow_cfs` / `hydrosphere_river_stage_m` /
Flutfläche). Alles, was den Zielprofil-Kriterien nicht genügt, wird als
`pending` mit Begründung abgelegt — nicht verworfen, nicht erfunden.

## Abschluss

Erfüllt ein Kandidat Ort **und** Zeit (co-lokal, co-temporal), läuft das
versiegelte Protokoll (`te_pair_probe`, siehe `docs/blatt-pfeil-
preregistrierung.md`) gegen den Open-Meteo-Niederschlag — das Verdict ist
pre-committed. Sonst bleibt der Pfeil `pending`, 0 honored.

---

## Befunde (2026-08-27, Recherche-Agenten) — 0 honored

### Weg A — direkt: DHM-Nepal-Gauge (Fund)

Das **DHM Nepal** betreibt eine **offene, key-lose, live API** mit einem
**co-lokalen Pegel am Ereignisort**: **Bhotekoshi at Rasuwagadhi**
(ID 4913, Serie 23251, lat 28.2713 / lon 85.3776 — praktisch der Zielpunkt),
Pegel (m), 10-min-Auflösung. Weitere offene Pegel: Trishuli at Betrawati,
Trishuli Khola at Dhunche (oberhalb), Bhote Koshi at Bahrabise (unterhalb) —
mehrere reichen bis 08-27.

**Aber (die ehrliche Grenze):** der co-lokale Pegel **bricht am Flutbeginn ab**.
Letzter Wert **1.62 m am 2026-08-26 08:40 NPT (02:55 UTC)** — direkt vor der
~09:00-Flut. Die Telemetrie verstummte am Einsetzen; **der Flut-Peak wurde
vom offenen Pegel nie aufgezeichnet** (Warnschwelle 6.0 m; der Morgen lag bei
1.6–1.9 m). Es gibt **keinen** offen gemessenen Abfluss (m³/s, nur hinter
bezahltem Portal) und **keinen** gemessenen Peak-Pegel.

### Weg B — Altimetrie: negativ

DAHITI-*gesamter* Katalog (3166 Asien-Ziele) durchsucht: **keine** Station
im 27–30°N/84–88°E-Korridor liegt am Trishuli/Bhote Koshi (nächste: Sun/Dudh
Koshi-Konfluenz ~160 km SE, Narayani ~120–200 km SW, Tsangpo in Tibet bei
gleichem Längengrad aber anderem Fluss). Hydroweb (neues Portal
`hydroweb.next.theia-land.fr`): Nepali-Stationen enden alle vor dem Ereignis
(nächste LIKHU-KHOLA 2026-08-10). HydroSat liefert nur GRDC-in-situ-Gauges.
G-REALM: nur Seen, unerreichbar. → **Keine co-lokale UND co-temporale
Altimetrie-Reihe.**

### Weg C — SAR-Flutfläche: CEMS EMSR927 (Fund)

- **CEMS Rapid Mapping Aktivierung EMSR927 „Flood in Nepal"** (GLOF, Rasuwa),
  aktiviert 2026-08-26T09:53, Zentroid 85.354E / 28.212N (co-lokal),
  4 AOI, 1 Produkt. **Offizielles Flut-Ausdehnungsprodukt in Arbeit** — der
  beste co-lokale räumliche Response.
- **Sentinel-1 (Copernicus Data Space, STAC key-lose):** Vor-Ereignis-Szenen
  bestätigt (08-12/16/19/24, gleiche Umlaufbahn 00:18:44Z für 08-12/24 →
  co-lokale Differenzbildung bereit) — aber **kein Nach-Ereignis-Bild
  vorhanden** (nächster Pass ~08-28, noch nicht eingespielt). Flutfläche
  **heute nicht** extrahierbar, in ~3–5 Tagen nach nächstem Pass schon.
- GEE: Account nötig (401), kein Lauf. VIIRS: zu grob/offline. China-Seite:
  nicht bestätigt (ReliefWeb-API 403; ECHO-Tageskarte vorhanden).

### Fazit

Der co-lokale **Pegel existiert** (Weg A), trägt aber **nicht den Peak**
(Sensor-Ausfall am Flutbeginn) — die co-temporale Response bleibt damit
**unvollständig**. Der **Abfluss (m³/s) ist offen nicht messbar** (nur bezahlt).
Der räumliche Response liegt bei **CEMS EMSR927** (in Arbeit) + Sentinel-1
(nach ~08-28). Der präregistrierte Pfeil bleibt `pending`, 0 honored — mit
zwei offenen, benannten Fortsetzungen: DHM-Pegel-Reihe ziehen (co-lokale
Serie bis zum Abbruch) und CEMS/Sentinel-1 nach dem nächsten Pass.

---

## Befund Luft + Oberfläche am Kollabhang (2026-08-28) — 0 honored

### Luftmessungen (Open-Meteo archive-api, key-frei, Punkt 28.271/85.515, UTC)
08-20 00 bis 08-26 03 (letzte Woche vor Kollaps):
- Temp 2m: **min −6.8 °C (08-23 23:00) | max +3.3 °C (08-24 06:00) | mittel −0.2 °C**
- Taupunkt: min −7.3 | max +1.3 | mittel −1.0 °C
- rel. Feuchte: min 72 % | max 100 % | mittel **94.5 %**
- Schneetiefe: **0.0 m** (ganze Woche − kein Schnee)
- Bodendruck: mittel ~533.6 hPa
- Wind 10m: mittel 2.8 km/h (schwach), max 7.0

Deutung (Messung, keine Hypothese): Temperatur **pendelt um 0 °C** —
mehrfaches Gefrieren/Aufthauen (Schmelz-Schwelle), hohe Feuchte.
**Kein Schnee am Hang** (Mitte 0.0 m). → Die Oberfläche am Kollabhang war
eisschmelz-aktiv/randscharf (0 °C-Grenze), aber ohne Schneeauflage.

### Oberfläche (Terrain-Messung, aus DEM + Luft verbunden)
Der Kollabpunkt liegt auf einem Kamm (5510 m): östlich 85.53–85.56 °O steigt
auf 5939–6024 m, westlich fällt er gegen 85.44 °O auf 4311 m — entwässert
nach **Westen** (Trishuli/Rasuwa, Stationen 4913/190/191). Der Hang ist
hochalpin, eis-/kalt-regime-geprägt (Temp-Pendel um 0 °C, kein Schnee).

---

## Befund Oberflaeche Kollabhang (2026-08-28, klare 08-24-S2, NDWI B03/B08) — 0 honored

- Kollabzentrum 28.271/85.515 (r=0.012): NDWI mean −0.038, max 0.204,
  1 Wasser-Pixel von 63865 → **kein freies Wasser am Kollabhang**.
- Kollabraum (r=0.05): NDWI mean −0.089, max 0.410, 88 Wasser-Pixel von
  1 093 608 → vernachlaessigbar.
- Nordflanke 28.30/85.52 (r=0.02): mean −0.161, 51/176000 → keine
  Wasserflaeche.

Zusammen mit Luftmessung (0 °C-Schwelle, kein Schnee): der Kollabhang ist
**trocken-hochalpin (Fels/Geroell/Eis), ohne freistehendes Schmelzwasser**
und ohne Schneeauflage. Der Massenverlust ist nicht ueber freie Wasser-
flaechen am Hang sichtbar (nur ueber SWIR-Eis-Verlust nachweisbar, s.u.).

### SWIR-Oberflaeche (Eis vs Fels) — BLOCKIERT
B11/B12 (SWIR, 20 m) liegen im PC-Blob als COG vor, aber der Download
schlug fehl: **409 "Public access is not permitted on this storage account"**
— das PC-SAS-Token (exp 04:05Z) ist abgelaufen, das Neubezug (04:51Z) wird
nicht akzeptiert. CDS-OData (JP2) braucht Auth (keine Credentials im
secret-store). → Der Eis-vs-Fels-Zusammensetzungs-Nachweis (NDSI) bleibt
**offen**; die Wasserflaeche am Hang ist damit als ~0 gemessen.

---

## Befund SWIR-Eisoberflaeche (2026-08-28, klare 08-24-S2, NDSI B03+B11) — 0 honored

Ursache des frueheren Blocks: das PC-SAS-Token (exp 04:05Z) war abgelaufen.
Fix: **frisches Token** von
`planetarycomputer.microsoft.com/api/sas/v1/token/sentinel-2-l2a` holen und
sofort anhaengen (Token ist kurzlebig, ~46 min). B11/B12/SCL (20 m, 5490²)
wurden so geladen. `livefeed_gate` um `--cog-ndsi` erweitert
(B03 gruen + B11 SWIR, NDSI=(g-swir)/(g+swir), Schwelle 0.4; 10 m/20 m
automatisch auf gemeinsames 10 m-Raster hochskaliert).

NDSI am Kollabhang (klare 08-24):
- Kollabzentrum 28.271/85.515 (r=0.012): **eisanteil 0.386** (24672/63865),
  ndsi_mean 0.194, max 0.849
- Kollabraum (r=0.05): eisanteil 0.156, mean 0.047
- Nordflanke 28.30/85.52 (r=0.02): eisanteil 0.079, mean −0.099

NDSI der 2 Seen (klare 08-24): GL085630 eisanteil 0.021, GL085494 0.018
→ **eisfreies, offenes Wasser** (Eisanteil ~Randoer/Umgebung).

### Deutung (Messung)
Der Kollabhang traegt eine **substanzielle Eis-/Schneebedeckung**
(38.6 % NDSI>0.4 im Kollabzentrum), kombiniert mit **NDWI ohne freies
Wasser** (messung oben). → der Hang ist **vergletschert/gefroren
(Eis/Firn), aber ohne sichtbares Schmelzwasser**. Das stuetzt eine
**eis-/permafrost-induzierte Massenbewegung** (kein Schmelzwasser-Pfad,
kein Gletschersee am Kollabhang; die 2 PDGL sind eisfrei und fern).
Die NDSI-Flaechen sind eine Momentaufnahme (08-24), Eis-Anteil kann durch
Schnee-/Firn-/Schatten-Beitrag mitgefuehrt sein.

---

## Befund Seismik + Tektonik + Luft/Druck (2026-08-28, USGS FDSN + Open-Meteo) — 0 honored

### Seismik am Kollabpunkt (USGS FDSN, BBox 26.27-30.27N/83.5-87.5E, 08-10..08-28)
Zwei seismische Events, beide am exakten Kollabpunkt 28.271/85.515, beide
`type=landslide`, `depth=0` (kein tektonisches Beben, sondern die
Massenbewegung selbst):
- **02:52:10 UTC, M5.2 (ms_vx)** — eventid us7000tbwb, sig 601, felt 1, cdi 2
- **06:00:35 UTC, M4.2 (ms_vx)** — eventid us7000tc90, sig 271 (Kollaps-Folge)

Kein tektonisches Erdbeben im Kausalraum im Zeitfenster. → Die Seismizität
ist **nicht tektonisch, sondern durch die Massenbewegung (Landslide)**
erzeugt. depth=0 bestaetigt Oberflaechen-/Hangprozess, kein Bebenherd.

### Luft/Druck am Kollabpunkt (Open-Meteo archive, 28.271/85.515)
Bodendruck ~533 hPa (Althoehe), Temperatur pendelt um 0 °C (min −6.8,
max +3.3), hohe Feuchte (94.5 %), kein Schnee (0.0 m). → Schmelz-/Gefrier-
Schwellenlage ohne Schneeauflage. (siehe fruehere Befund-Zeile)

### Tektonischer Kontext
USGS liefert fuer die Events kein `tectonic`-Attribut (kein tektonischer
Herd). Die Region ist Himalaya-Kollisionszone, aber der gemessene Impuls ist
ein Landslide-Signal (M5.2/4.2, depth 0), kein Beben.
