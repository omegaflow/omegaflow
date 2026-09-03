<!--
  title: Rechercheauftrag — echte Gewässer & See-Baseline im Lirung-Kollabgebiet
  class: auftrag
  date: 2026-08-27
  status: pending
  see-also: docs/blatt-pfeil-sturzflut-tibet.md docs/auftrag/auftrag-cog-quelle.md docs/auftrag/auftrag-abfluss-trishuli.md docs/specs/livefeed-gate.md
-->

# Rechercheauftrag: die echten Gewässer im Lirung-Kollabgebiet

## Anlass (gemessener Befund, 0 honored)

`--cog-ndwi` misst (COG-Quelle Planetary Computer, 08-12, std-only-15-bit-
Reader, UTM) an den OSM-Koordinaten **kein offenes Wasser**:

- **Langjie Cuo (朗吉错)** 28.4070°N / 85.4018°E → 0 Wasser-Pixel, NDWI max 0.071
- **Tuomito (拓米托湖)** 28.3924°N / 85.4186°E → 0 Wasser-Pixel, NDWI max 0.071
- **Bhote-Koshi-Tal** 28.28°N / 85.38°E → **150 Wasser-Pixel** (Fluss erkannt,
  Messkette korrekt)

Der Kollaps selbst lag am **Lirung-Gletscher** (28.26–28.27°N, 85.50–85.53°E),
der USGS-Kollaps `us7000tbwb` (Typ `landslide`, M5.2 `ms_vx`, 2026-08-26
02:52Z). **Frage, die offen ist:** Wenn die beiden OSM-„Seen" kein offenes
Wasser sind — **wo liegt das echte Wasser**, und ist die Quelle überhaupt ein
**See-Ausbruch (GLOF)** oder eine **Fels+Eis-Avalanche**?

## Zweck

1. Die **echten Gewässer** (Glazialseen, Schmelzseen) im Kollabgebiet
   lokalisieren (Koordinaten), **nicht** die OSM-Punkte übernehmen.
2. Ihre **Vor-Ereignis-Wasserfläche** mit dem jetzt funktionierenden
   `--cog-ndwi` messen (Baseline).
3. Die **Quellnatur** belegen: See-Ausbruch (Wasserquelle) oder Fels+Eis-
   Avalanche (gravitative Masse) — das entscheidet den Kausalpfeil.

## Kernregel (0 honored)

Jede Koordinate, jede Wasserfläche und jede Quellnatur muss **gemessen und
belegt** sein (COG-Messung, Karte, offene Quelle). Kein Wert wird erfunden,
keine Koordinate wird aus einem Namen geglaubt. Nicht messbar = `pending`.

## Weg A — Wasser aus der Messung selbst lokalisieren (primär)

Der COG-Vor-Ereignis-Bestand ist verifiziert (08-12 18.7 % Wolken, 08-24
38.6 %, 08-22 78 %, 08-27 78.5 %). **Auftrag:**

1. **Gitter-Scan** der Wasserpixel über das ganze T45RUM-Fenster (Kollabgebiet
   28.2–28.5°N, 85.35–85.60°E): alle NDWI>0.2-Cluster mit Koordinaten + Fläche
   auflisten. Das findet die **echten** Seen, nicht die OSM-Punkte.
2. Für jedes gefundene Cluster: `--cog-ndwi` in engerem Fenster → gemessene
   Fläche (Pixel × 10 m²) als Vor-Baseline.
3. Dieselbe Suche auf einer **Post**-Szene, sobald eine klare da ist
   (08-27 ist 78.5 % Wolken; Post-Sentinel-1/CEMS hängt). Ob ein See
   **entleert/drainiert** ist, ist die gemessene Post-Antwort.

## Weg B — Seeinventar + Quellnatur (belegt, nicht geglaubt)

1. **Globale Gletschersee-Inventare** (RGI / Glacier Lakes, ICIMOD GLOF-Listen,
   im Vergleich zu OSM): echte Namen+Koordinaten der Seen am Lirung-Gletscher /
   oberhalb des Bhote Koshi. **Vorsicht:** OSM hat sich als unzuverlässig für
   „Langjie Cuo/Tuomito" erwiesen — jede Koordinate wird mit Weg-A-Messung
   gegen-geprüft, nicht übernommen.
2. **Quellnatur:** Wenn am Kollabpunkt (28.26–28.27°N, 85.50–85.53°E) ein
   See liegt → GLOF-Kandidat; wenn reiner Gletscher/Steilwand → Avalanche-
   Kandidat (Chamoli-2021-Analogon, Shugar et al. 2021). Beides nur mit
   belegtem Beleg (Bild/Inventar), nicht aus dem Namen.
3. **Sekundär:** CEMS EMSR927 (in Arbeit) und ECHO/ICIMOD-Sitreps — nur
   zitierbare Werte, sonst `pending`.

## Weg C — zeitliche Wasserentwicklung (wenn klare Post-Szene da)

1. Post-S2 08-27 (78.5 % Wolken): Wasser-Signatur nur soweit messbar, wie
   wolkenfrei; ehrlich benennen.
2. Post-Sentinel-1 (SAR, ~08-28/29): wassersensitiv, wetterfest — das ist die
   robuste Post-Wasserfläche. Bis dahin Post-Wasser `pending`.

## Lieferung

- **Gefundene Gewässer:** je Cluster Koordinaten, gemessene Vor-Wasserfläche
  (m²/Pixel), Quelle (COG-Szene + Datum), gegen-geprüft vs Inventar/OSM.
- **Quellnatur:** See-Ausbruch (mit belegtem See am Kollabpunkt) ODER
  Avalanche (ohne See) — als belegte Einordnung, sonst `pending`.
- Jeder OSM-Punkt, der kein Wasser trägt, wird als solche markiert
  (Koordinaten fehlerhaft oder kein offenes Wasser) — nicht stillschweigend
  verworfen.

## Abschluss

Liefert Weg A gemessene Wassercluster **vor** dem Ereignis, ist die Baseline
belegt. Ob ein **Vor-Ereignis-See** am Kollabpunkt existiert, entscheidet
GLOF vs. Avalanche für den präregistrierten Pfeil — sonst bleibt die
Quellnatur `pending`, 0 honored.

---

## Befunde Weg A (2026-08-27, `--cog-ndwi`-Scan, Szene 08-12 18.7 % Wolken) — 0 honored

**Gitterscan** über 28.20–28.50°N × 85.35–85.60°E (208 Punkte, Fenster
±0.012°), Wasser = NDWI>0.2:

- **Bhote-Koshi-Fluss:** dünne, kohärente Wasserlinie entlang 28.22–28.28°N
  bei 85.37–85.41°E (echte Fließgewässer-Linie, ~100–150 Wasser-Pixel im
  Fenster). Das ist das einzige klare offene Fließgewässer.
- **NE-Flächen** (28.36–28.50, 85.53–85.60): breite NDWI>0.2-Flächen, aber
  **Mean nahe 0** (−0.03…−0.21), verstreute Maxima (0.2–0.43) → **Gletscher/
  Schnee/Gelände, kein sauberer offener See** (ein See hätte stark positiven
  Mean mit kohärenter Fläche).
- **USGS-Kollabzentrum (28.271, 85.515), r=0.012: 0 Wasser-Pixel, NDWI
  max 0.194, min −0.636** → das Gebiet ist **sichtbares Gelände (nicht
  Wolke)** und trägt **kein Oberflächenwasser** am 08-12.
- Lirung-Gletscherkopf (28.30, 85.53): 37 Wasser-Pixel, max 0.328 — hohes
  NDWI durch Gletscher/Schnee, kein See.

**Interpretation (gemessen, nicht behauptet):** Am Kollabpunkt liegt am
08-12 **kein offener Oberflächensee** (wolkenfrei messbar). Das stützt eine
**Fels+Eis-Avalanche-Quelle** (Chamoli-2021-Analogon), nicht einen GLOF aus
einem sichtbaren Vor-See. **Einschränkung (0 honored):** ein subglazialer/
verborgener See oder eine wolkenverdeckte Stelle außerhalb des wolkenfreien
Kollabpunkts kann optisch nicht ausgeschlossen werden; die Post-SAR-Fläche
(CEMS/Sentinel-1) bleibt die offene Bestätigung.

---

## Befunde Weg B (2026-08-27, Quellen-Verifikation) — 0 honored

### SWE-Volltext + ICIMOD-Quelle (gemessen, verifiziert)

Severe Weather Europe (R. Colucci, 27/08/2026) **Volltext** abgerufen; zitiert
**ICIMOD via Reuters/AP** und **USGS** (`us7000tbwb`). Kernaussagen:

- **Eis-Fels-Avalanche** von einem vergletscherten Hang ins **obere Lhende
  Khola**, **blockierte** das Tal → temporärer **Landslide-Dam** → Schwall.
- USGS reklassifizierte das **M4.4-Erdbeben zu M5.2-Landslide** — „die
  Erschütterung wurde vom Massen-Movement erzeugt, nicht ausgelöst".
- **„No emptied pre-existing glacial lake has yet been shown to be the
  primary source"** → kein GLOF; temporärer Staubsee ≠ GLOF.
- Trishuli **+9 m in ~30 min** (Reuters/ICIMOD, berichtet — kein Pegel-Plot).

Reuters-Folgeartikel (27/08): **≥ 359 Tote, ~1000 Vermisste**; Nepal/China
warnen vor erneuter Flut durch Wasser, das sich in **zwei Seen** staut.

**Abgleich gegen gemessene Befunde:**
| GLM/SWE-Kernaussage | Messung |
|---|---|
| M4.4→M5.2 `landslide` (us7000tbwb) | **bestätigt** (meine FDSN-Messung) |
| Eis-Fels-Avalanche, kein GLOF | **konsistent** (08-12: kein Vor-See am Kollabpunkt) |
| kein entleerter Vor-See als Quelle | **konsistent** |
| EMSR927 | **nicht** im SWE-Artikel (nur meine direkte CEMS-Messung, 1 Produkt, closed=false) |

### Geolokalisierung (OSM-Nominatim + Fachquellen, gemessen/belegt)

- **Lende/Lhende Khola** 28.3496°N / 85.4464°E — **~11 km NW** des Kollabpunkts
- **Langtang-Lirung-Gipfel** 28.2575°N / 85.5158°E — **~1.5 km S** des
  Kollabpunkts (28.271/85.515) → Abbruch an der **Nordflanke** des Massivs,
  entwässert nach Tibet ins Lhende-Khola-System (Nepali Times/AP-Cook-Shugar).
- **RGI-„Lirung-Gletscher" = Südzunge** (Zunge Richtung Kyanjin, Terminus
  ~28.21–28.23N) im **Langtang-Khola-Einzugsgebiet** — von dieser Flut
  **nicht** betroffen. Der OSM-Punkt 28.2467/85.5421 gehört zu dieser Südzunge,
  **nicht** zur Kollaps-Flanke. **Begriff trennen**, um Verwechslung zu
  vermeiden (Claude-Recherche, Nepali Times/KTSA/Wikipedia-Sammelartikel).
- Die zwei OSM-„Seen" (Langjie Cuo 28.4070/85.4018, Tuomito 28.3924/85.4186)
  liegen in der **Lhende-Khola-Kette** talabwärts des Kollaps.

**Reconciliation (nicht erzählt):** Kollaps an der **Nordflanke des
Langtang-Lirung-Massivs** → talabwärts Lhende-Khola-Talsystem (ICIMOD/SWE-
Benennung) → die zwei Seen → Bhote Koshi. Konsistent; der exakte Quell-Hang
braucht noch das CEMS/S1-Bild.

### Quell-Inventar-Abgleich (Claude-Recherche, 0 honored)

- **Langjie Cuo / Tuomito:** in **keinem** offenen, autoritativen Inventar
  gefunden (Wikipedia/Geonames/ICIMOD-Literatur) — nur OSM (crowdgesourced,
  ungeprüft). Name nicht aus einem verifizierten Gletschersee-Katalog.
  Konsistent mit Messung: **kein Wasser** an beiden Punkten (08-12).
- **ICIMOD/UNDP 2020 PDGL (47 Seen, Koshi/Gandaki/Karnali):** die meisten
  liegen im *anderen* „Bhote Koshi" (Sindhupalchok/Kodari-Korridor), nicht im
  Trishuli/Gandaki-Becken. Für das Gandaki-Becken nennt der Bericht nur **3
  PDGL**, Namen/Koordinaten dort nicht gefunden → die zwei OSM-Seen werden von
  diesem Inventar **nicht** bestätigt.
- **Real dokumentierter See der Region:** der supraglaziale See der
  **2025er-Vorgängerflut** (DHM, 28.4043°N/85.6469°E, ~5150 m, 0.75→0.60 km²)
  liegt ~20 km östlich der OSM-Seen und gehört zum **2025er-Ereignis** (anderer
  Ort, anderes Datum). **Tool-Validierung (08-12, gemessen): NDWI max 0.412,
  270 Wasser-Pixel** → echte (schwache) Wasser-Signatur, klar unterscheidbar
  von Langjie/Tuomito (max 0.071, 0 Wasser) → Messkette validiert gegen einen
  bekannten See.

---

## Befunde Weg A/2 (2026-08-27, **zweite Vor-Baseline 08-24**) — 0 honored

08-24-Szene (S2B_MSIL2A_20260824T044659, **38.6 % Wolken**), COG B03/B08 von
Planetary Computer geladen (164 MB / 163 MB, 15-bit, Tiepoint identisch zu
08-12), `--cog-ndwi`:

- **Kollabzentrum (28.271, 85.515), r=0.012: 1 Wasser-Pixel von 63865,
  NDWI max 0.204** → **praktisch kein Oberflächenwasser** am 08-24.
  → **Zweite Vor-Baseline stützt „kein See am Kollabpunkt".**
- **Seen-Region (Langjie Cuo/Tuomito) ist in dieser Granule nodata**
  (28.39+°N / 85.42°E → 0 Pixel, alle 0/0): Folge der 38.6 % Wolken dort.
  → Auf 08-24 **nicht** messbar (ehrlich, szenenspezifisch, kein Tool-Bug —
  08-12 liefert dort reale negative NDWI).
- Tool-Verifikation: Pixel-Koordinaten der Seen-Fenster liegen korrekt im
  Bild (innerhalb 0–10979); die 0 kommen von all-nodata, nicht vom Decoder.

---

## Befund CEMS-EMSR927 / Post-S1-Zugang (2026-08-27) — 0 honored

- **EMSR927-Produkte sind nicht via `CDS_API_KEY`/`CMEMS` abrufbar.** Die
  CDS-/CMEMS-Keys adressieren den Klima- (Climate Data Store) bzw. Marine-
  Data-Store, **nicht** den Rapid-Mapping-Portal-Zugang. EMSR927-Endpoints
  liefern 404/HTML/Login (Event-Autorisierung nötig). → **Post-Fläche über
  EMSR bleibt `pending`** mit den aktuellen Keys (nicht erzählt).
- **Sentinel-1 (GRD) um den Kollabpunkt 08-26..08-29: 0 Szenen** (CDSE-STAC,
  offen) — der nächste Post-Pass liegt ~08-28/29, heute 08-27. → **Post-SAR
  `pending`**, nicht fabriziert.
- Post-S2 08-27 existiert (78.5 % Wolken, CDSE) — hohe Wolkenbedeckung, als
  Flut-/Narben-Footprint nur eingeschränkt verwertbar; robuste Post-Wasser-
  messung bleibt an SAR/CEMS gebunden.

---

## Befund Sentinel-1-Weg (2026-08-27) — 0 honored

**S1-Pipeline über Planetary Computer etabliert (key-frei, CDSE unnötig).**
- Microsoft PC hostet **Sentinel-1-GRD-COG** (VV/VH, IW) mit demselben
  SAS-Token wie S2 (`/api/sas/v1/token/sentinel-1-grd`).
- Vor-Ereignis-Szene um den Kollabpunkt gefunden und **geladen**:
  `S1D_IW_GRDH_1SDV_20260824T001844` (08-24, VV/VH, descending), VV-COG
  **673 MB, vollständig** (Tile-Offsets < Dateigröße, 25551×16740 px, 16-bit,
  EPSG:4326) → **SAR-Vor-Baseline 08-24 gespeichert.**
- **Post-Ereignis-S1:** weder auf PC noch CDSE vorhanden (nächster Pass
  ~08-28/29) → Post-SAR `pending`.
- **Grenze (ehrlich):** Einzelpixel-Lesen am Kollabpunkt scheiterte am
  groben/irregulären **GCP-Georeferenzierungs-Gitter** der Szene (bilinear
  divergierte); ein sauberer SAR-Wert braucht eine Raster-Bibliothek oder
  die **Post-vs-Vor-Differenz**. Der eigentliche Nutzen liegt in der
  Differenzierung, nicht im Einzelbild — optisch ist „kein See am
  Kollabpunkt" bereits gemessen.
- **CDSE-Auth als unnötig abgehakt:** direkter `password`-Grant wird vom
  CDSE-Realm für interaktive Konten abgelehnt (400→401→500); der key-freie
  PC-Weg umgeht das komplett.

---

## Befund MODIS/VIIRS-Flutprodukt (2026-08-27, GIBS key-frei) — 0 honored

**Quelle:** NASA **MODIS Global Flood Product** (`MCDWD`/`MODIS_Combined_Flood_1-Day`)
über **GIBS-WMS, key-frei** (kein EDL-Token nötig); VIIRS (`VCDWD`) analog.
Ereignistag-Granule (08-26) vorhanden. Download des rohen LANCE-HDF
(`nrt3.modaps`) scheitert am interaktiven EDL-Session-Auth — GIBS liefert die
Klassifikation ohne das.

**Messung (Kasten 85.2–85.9/28.0–28.6, täglich):**
- Das Produkt ist in diesem Steilgletscher-Gelände **~99 % No-Data** (Grau,
  konstant 99.1–99.4 % über 4 Tage → Background-Maske, kein Wolken-Artefakt).
- **Kollabpunkt (85.50–85.53/28.25–28.30): 100 % No-Data an allen Tagen** —
  das 250-m-Produkt klassifiziert die Stelle gar nicht.
- Wasser-Fokusbox (85.52–85.62/28.47–28.53, oberes Lhende-Khola):
  **08-24=404, 08-25=318, 08-26=48, 08-27=105 Wasser-Pixel** → das Cyan-
  „Wasser" war **vor dem Ereignis stärker** (404/318) als am Ereignistag (48)
  → **persistenter Gletscher/Schnee, kein neu gebildeter Ereignissee.**

**Fazit (negativ, messbar):** Das MODIS-Flutprodukt liefert **keinen Beleg
für einen am 08-26 neu entstandenen Wasserkörper** am Kollabgebiet; die
anfängliche Deutung des 08-26-Wasser-Signals war eine Fehldeutung, die der
Vor↔Nach-Vergleich korrigiert hat. Grenze: 250 m + No-Data in Steilterrain;
kein Widerspruch zur S2-Messung (kein Vor-See), aber auch keine neue
Wasserfläche im Auflösungsbereich.


---

## Befund Weg B/2 (2026-08-28, **ICIMOD-Primärdaten** — 0 honored)

### Freigeschaltete Quelle

ICIMOD-Regional-Database-System (`rds.icimod.org`), Knox-Token-Auth via
`/geoapi/auth/login_public/` → `Authorization: Token <tok>`; Datendownload
`/geoapi/datasets/{uuid}/download/direct/confirm/` (purpose=RESEARCH).
Zugangsdaten hinterlegt in `~/.secrets.local` (ICIMOD_USERNAME/PASSWORD/
KNOX_TOKEN) + GitHub-Secrets (ICIMOD_RDS_*).

**Inventar-Datensatz:** „Glacial lakes in the Koshi, Gandaki, and Karnali
river basins of Nepal, the TAR of China, and India" — UUID
`777e6cb6-c2db-47ab-852c-48bd0f24a158`, DOI `10.26066/RDS.1971946`,
BBox 80.07–88.72°O / 27.44–30.62°N. **Landsat 2015–16, NDWI semi-
automatisch, SRTM-Höhen, WGS-UTM.** 3624 Seen (Koshi/Gandaki/Karnali).
Shapefile `GL_3basins_2015.shp` geladen (pyshp), Region Gandaki/Trishuli
gefiltert → **124 Seen** im Zielgebiet (28.15–28.55°N, 85.30–85.70°O).

### Koordinaten → GL_ID / Typ entschlüsselt (Publikation lib.icimod/34905)

GL_ID `GLXXXE...` = Längengrad×1000 / Breitengrad×1000. Typ-Codes aus
Tabelle 4.2/3.1: **M(e)** End-Moräne, **M(l)** Lateral-Moräne, **M(o)**
sonstige Moräne, **I(s)** supraglazial, **I(v)** Eisdamm-Talform,
**B(c)** Karsee, **B(o)** sonstige Felsdamm-Glazialerosion.

### Gemessen — Seen am Kollabpunkt

Nächster erfasster See zum Kollabpunkt (28.271/85.515): **Id=37,
85.5620°O / 28.2197°N, M(e), 3988 m, 0.042 km²** — Distanz **0.070°**
(ca. 7 km). **Kein Inventar-See liegt am Kollabpunkt selbst.**

### Gemessen — „potentially dangerous" Trishuli-Seen (die Quelle des Schwalles?)

Die Publikation (Tabelle, gerendert betrachtet) führt im **Trishuli-Sub-
Basin** genau **2 gefährliche Seen, beide im TAR China**:

- **Nr. 43 — `GL085630E28162N` (Id=30)** 85.6304/28.1628, M(e), 0.140 km²,
  4983 m, Rank **I**. Beschreibung: „Lake at extreme end of dam; chances
  of landslides from the wall to the right; and of ice avalanches from the
  hanging source glacier". → Distanz zum Kollabpunkt **0.158° (17.6 km)**.
- **Nr. 44 — `GL085494E28508N` (Id=206)** 85.4945/28.5086, M(e), 0.277 km²,
  4749 m, Rank **II**. Beschreibung: „Lake expanding towards the debris-
  covered glacier; hanging moraine; steep outer dam slope; chances of snow
  and ice avalanches; length of moraine ~100 m". → Distanz zum Kollabpunkt
  **0.238° (26.5 km)**.

**Wichtige Einordnung (gemessen, nicht behauptet):** beide gefährlichen
Seen liegen **stromaufwärts in der oberen Lhende-Khola**, aber in einer
**Distanz von 17.6 / 26.5 km** zum Kollabpunkt. Keiner liegt am Kollab-
Hang. Das ICIMOD-2015-Inventar weist **keinen "potentially dangerous"-
See direkt am Kollabpunkt** aus — konsistent mit der optischen Messung
(kein Vor-See am Kollabpunkt, 08-12/08-24) und mit der SWE/ICIMOD-
Aussage „no emptied pre-existing glacial lake has yet been shown to be
the primary source".

### Korrektur (0 honored) — „nicht gefunden" → jetzt gefunden

Zuvor (2026-08-27, Claude-Recherche) hieß es, **Langjie Cuo / Tuomito /
die Gandaki-PDGL seien in keinem autoritativen Inventar gefunden**. Das
ist durch die **Primärdaten** widerlegt:

- **Langjie Cuo = Id=143, `GL085401E28406N`**, 85.4017°O / 28.4070°N,
  **B(c)** Karsee, 0.072 km², 3896 m — im Inventar, Trishuli-Sub-Basin.
- **Tuomito = Id=124, `GL085418E28392N`**, 85.4186°O / 28.3925°N,
  **E(o)**, 0.0039 km², 3956 m — im Inventar, Trishuli-Sub-Basin.
- **dh** beide liegen in der **Lhende-Khola-Kette**, **talabwärts** des
  Kollapses (28.39–28.41°N vs. Kollab 28.27°N) — das passt zur
  Stromrichtung und erklärt ihre (schwache, NDWI 0.071) optische
  Wasser-Signatur.
- Die **2 Gandaki/Trishuli-PDGL** (GL085630, GL085494) sind NICHT
  „Langjie Cuo"/„Tuomito" — sie sind **andere, größere (0.14/0.28 km²)**
  Moränenseen oberhalb. Die zuvor gemeldete Zahl „3 PDGL für Gandaki"
  ist durch die exakte Liste (**Nepal 1 + China 1** in der Trishuli-
  Zeile) präzisiert.

### Schließe (gemessen) — Quellnatur

- Am Kollab-Hang: **0 erfasster See** (nächster 7 km). → **kein
  Vor-Ereignis-GLOF-See direkt an der Quelle**, eher **Eis-Fels-Avalanche**
  (Chamoli-2021-Analogon).
- 2 gefährliche Moränenseen existieren **stromaufwärts** (17.6/26.5 km) —
  **Quelle für den Schwall** möglich, aber **nicht am/nahe am Kollabpunkt**.
  Eine zeitliche Lücke (See-Ausbruch fern + Kollaps) ist **nicht messbar**
  aus diesem Inventar (2015-Vor-Baseline, kein Post-2026-Inventar).
- **Rest offen (pending):** ob einer der 2 PDGL am 26.08. geborsten ist
  (kein Post-Inventar; SAR/CEMS Post-Fläche weiterhin `pending`).

---

## Befund Post-S2 08-27 (2026-08-28, `--cog-ndwi`) — 0 honored

Post-Szene `S2B_MSIL2A_20260827T045659` (08-27, 78.5 % Wolken), COG
B03/B08 von Planetary Computer geladen (je 263 MB, 10980², 15-bit),
`--cog-ndwi` (Rust std, neu gebaut):

- **Kollabzentrum (28.271/85.515) r=0.012:** 1 Wasser-Pixel von 63865,
  NDWI max 0.222, mean −0.011 → **kein offenes Wasser am Kollabpunkt.**
- **Kollab-Raum r=0.05:** 19 / 1.09 M px, mean 0.013 → praktisch kein Wasser.
- **Lhende-Khola-Tal (28.35/85.45) r=0.03:** 2 / 394 k px, mean −0.127.
- **Nordflanke-Kopf (28.30/85.55) r=0.03:** 99 / 394 k px, mean −0.048
  (Gletscher/Schnee/noise, kein zusammenhängendes Wasser).

**Ehrliche Grenze (0 honored):** 78.5 % Wolken → der optische Post-Footprint
ist **nur eingeschränkt messbar**. NDWI>0.2 findet **kein neu gebildetes
stehendes Wasser** am Kollabgebiet (konsistent mit Vor-Szenen 08-12/08-24).
Die **Sediment-/Narbenfläche** (Flutablagerung, nass-frisch) ist über NDWI
nicht erfassbar und braucht die Vor↔Post-Differenz im wolkenfreien Kanal oder
SAR. **Post-Flut-Fläche bleibt an S1/CEMS gebunden** (pending bis ~08-30).

---

## Befund Zeitreihe der 2 gefährlichen Seen 2015→2026 (2026-08-28) — 0 honored

Klare 08-24-COG (S2B, 38.6 % Wolken), B03/B08 von PC geladen (164/?MB,
10980², 15-bit, std-Rust--cog-ndwi). Beide Seen im T45RUM-Tile.

| See | ICIMOD-2015 (Area-Feld, verifiziert = Polygonfläche) | S2 08-24 NDWI>0.2 | Δ |
|---|---|---|---|
| GL085630 (85.630/28.162) | 0.140 km² | 0.135 km² (r=0.03) | ~−4 % |
| GL085494 (85.494/28.508) | 0.277 km² (= Polygon 0.279) | 0.075 km² (r=0.03) | **~−73 %** |

- **GL085630** blieb praktisch **stabil** (0.140→0.135 km²).
- **GL085494** zeigt ein **gemessenes Schrumpfen** (0.277→0.075 km², −73 %).
  Achtung: NDWI misst nur offene Wasserfläche > Schwelle; Schwebstoffe/Eis/
  Frostrand können die Fläche unterschätzen. Der 08-24-Wert ist eine
  Momentaufnahme (kein 2026-Inventar-Polygon). Als gemessener Indikator
  für eine **Veränderung** (Schrumpfung/Teil-Zerfall) geführt, nicht als
  definitive 2026-Polygonfläche.
- **Relevanz für die Flut:** beide Seen sind **nicht am Kollabpunkt** (17.6 /
  26.5 km). Ein Schrumpfen/Teil-Ausbruch von GL085494 bis 2026 wäre ein
  **Hinweis auf vorherigen Abfluss**, aber **kein** Auslöser des Kollapses
  (Entfernung + kein hydrologischer Pfad am Kollab-Hang, siehe
  Entwässerungs-Messung: Kollab läuft westlich in die Trishuli).
