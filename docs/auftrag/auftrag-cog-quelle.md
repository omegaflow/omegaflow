<!--
  title: Rechercheauftrag — COG-Bandquelle für Sentinel-2 NDWI (Seen Langjie Cuo / Tuomito)
  class: auftrag
  date: 2026-08-27
  status: pending
  see-also: docs/blatt-pfeil-sturzflut-tibet.md docs/auftrag-abfluss-trishuli.md docs/concepts/livefeed-gate.md
-->

# Rechercheauftrag: eine erreichbare COG-Bandquelle für die Seen

## Zweck

Der archivar hat einen **std-only-COG-NDWI-Reader** (`--cog-ndwi`), der aus
zwei lokalen COG-Bändern (B03 grün, B08 NIR) die Wasserfläche eines Geo-
Fensters misst. Er ist **verifiziert** (20 Tests grün), aber **nicht an
Daten angeschlossen**: Für die Seen Langjie Cuo (朗吉错, 28.4070°N 85.4018°E)
und Tuomito (拓米托湖, 28.3924°N 85.4186°E) und den Kollabpunkt am
Lirung-Gletscher (28.26°N 85.51°E) liegt **kein ladbares COG-Band** vor.

Das ist **`pending`, kein „sobald"** (0 honored): Die Messung ist möglich,
aber die Quelle fehlt. Dieser Auftrag findet die Quelle **jetzt** und
belegt sie — Ort, Zeitfenster (um 2026-08-26), Format (COG/TIFF), HTTP-Zugriff.

## Kernregel (0 honored)

Nur eine **belegte, erreichbare, getestete** Quelle zählt. Jede Kandidaten-
quelle wird mit einem tatsächlichen HTTP-/Range-Abruf verifiziert (Status-
code, Bytezahl), nicht aus einer Doku geglaubt. Nicht erreichbar oder nicht
COG = `pending` mit Begründung.

## Zielprofil der Quelle

- **Format:** COG / GeoTIFF (lesbar durch `geotiff-reader`), **nicht** JP2.
- **Bänder:** Sentinel-2 B03 (grün) und B08 (NIR), L1C oder L2A, u16.
- **Ort:** Fenster 85.35–85.60°E, 28.18–28.45°N (deckt beide Seen + Kollap).
- **Zeit:** mindestens eine Szene im Fenster **08-10…08-29 2026**. Idealer-
  weise eine klare **Vor**-Szene (08-12: 18.8 % Wolken) für die Baseline.
- **Zugriff:** HTTP-range-fähig oder einmalig herunterladbar, ohne OAuth
  (oder mit dokumentierbarem, einfachem Zugang).
- **Auflösung:** 10 m (R10m). 20 m (R20m) als akzeptabler Fallback (B03/B08
  liegen aber 10 m vor).

## Kandidaten (jeweils verifizieren, nicht nur notieren)

### 1. CDSE / Copernicus Data Space (aktiv, STAC key-los)
- Collection `sentinel-2-l2a` liefert die Bänder bisher als **JP2** (`image/jp2`)
  mit `s3://`-Refs. **Auftrag:** prüfen, ob es (a) einen **COG-**Asset-Varianten
  (`.tif`, `image/tiff`) gibt, (b) den s3-Pfad in einen **HTTP-COG**-Endpunkt
  übersetzt werden kann, (c) die creodias-OData `$value`-UUIDs für die Bänder
  (wie beim Thumbnail) anonym range-fähig sind.
- Auch prüfen: Collection `sentinel-2-l2a-tlm` und `sentinel-2-l1c` — gibt es
  dort COG-Assets für dasselbe Fenster?

### 2. creodias Datahub OData (Anonym-Test positiv)
- Der **Thumbnail** war anonym ladbar (`datahub.creodias.eu/odata/v1/Assets(UUID)/$value`,
  HTTP 200). **Auftrag:** für die 08-12-Szene die Bänder B03/B08 via OData
  `$value` abrufen — sind es JP2 oder gibt es `.tif`? Ist ein **Range-Request**
  auf den JP2/COG möglich?

### 3. Microsoft Planetary Computer (COG auf Azure, Sentinel-2 L2A)
- Hostet S2-L2A als **COG-GeoTIFF** auf Azure Blob. STAC-Suche braucht Token,
  aber die **COG-Blob-URLs** sind oft **anonym range-fähig**. **Auftrag:**
  Szene für 08-12/08-22/08-24 (T45RUM) finden, Blob-URL extrahieren, Range-
  Abruf testen. **Achtung:** Abdeckung von 2026-08 prüfen (historische COGs
  reichen teils nur bis 2023/24).
- **Ehrliche Grenze benennen:** falls die Abdeckung endet → `pending` mit dem
  letzten belegten Datum.

### 4. AWS Registry of Open Data (RODA) `sentinel-s2-l2a`
- War der klassische COG-Host für S2-L2A (`.tif`, range-fähig), aber die
  Veröffentlichung wurde eingestellt / zu Planetary Computer verschoben.
  **Auftrag:** aktuellen letzten Datenstand (Datum des neuesten Tiles) belegen;
  wenn vor 2026-08 → `pending`.

### 5. Google Cloud Storage `gcp-public-data-sentinel-2`
- Hostet S2-L1C (und L2A?) als COG. **Auftrag:** Format prüfen (COG `.tif`
  oder JP2?), 10-m-B03/B08 für T45RUM, anonym range-fähig? Abdeckung bis 08-27?

### 6. CEMS EMSR927 (Rapid Mapping, in Arbeit)
- Offizielles **Flut-Ausdehnungsprodukt** — wenn ausgeliefert, misst es die
  **Post**-Flutfläche direkt (kein NDWI nötig). Produkt-Endpoint gab 404,
  Aktivierung zeigt 1 Produkt. **Auftrag:** periodisch (z. B. alle 6 h) den
  Produkt-Endpoint re-abrufen; sobald greifbar, als Primärquelle übernehmen.

## Lieferung

Je Kandidat:
- **Quelle + URL** (exakter HTTP-Endpunkt)
- **Ort / TMS-Tile / EPSG**
- **Zeitfenster** der erreichbaren Szenen (erste/letzte co-temporal mit 08-26)
- **Format** (COG/TIFF ja/nein) und **Bandverfügbarkeit** (B03/B08)
- **Verifikation:** HTTP-Code + Bytezahl des Range-Abrufs
- **Bewertung:** erfüllt Zielprofil ja/teilweise/nein

Falls ein Kandidat COG-Bänder für **Vor**- und/oder **Nach**-Ereignis liefert,
wird er als die angeschlossene Quelle vermerkt — die Messung der
**Baseline** (Langjie Cuo / Tuomito vor 08-26) kann dann direkt über
`archivar --cog-ndwi` laufen.

## Abschluss

Sobald (nur) eine Quelle belegt ist, die COG-Bänder im Fenster trägt, ist das
`pending` in ein **Messvorhaben** übergegangen: `--cog-ndwi` gegen die
Vor-Szene → gemessene Seebaseline. Die Post-Flutfläche bleibt an CEMS/Post-S1
gebunden. Bis eine Quelle belegt ist, bleibt die Wasserfläche **pending,
0 honored** — das Wort „sobald" trägt keine Messung.

---

## Befunde (2026-08-27, Recherche abgearbeitet) — 0 honored

### Quelle gefunden + verifiziert: Planetary Computer (COG)

**K3 ist der belegte Fund.** Microsoft Planetary Computer hostet Sentinel-2
L2A als **COG** (`image/tiff; profile=cloud-optimized`) mit **B03/B08** in
10 m. Abruf **anonym** über den SAS-Token-Endpoint
`/api/sas/v1/token/sentinel-2-l2a` (HTTP 200, läuft stündlich ab), Range-
Abruf auf den Azure-Blob → **HTTP 206**, 512 B, `II*\0`-TIFF-Magic,
`Accept-Ranges: bytes`. Zeitfenster: Vor-Szenen **08-12 (18.7 % Wolken),
08-14, 08-19, 08-22, 08-24 (38.6 %)**; Post-Szene 08-27 (78.5 %). Ort: T45RUM.

**Ehrliche Einschränkung der Daten:** Die L2A-COGs liegen als **15-bit Gray,
Deflate, gekachelt 512²** vor. `geotiff-reader` und die `tiff`-Bibliothek
lehnen 15-bit Gray ab (nur u8 ≤ 8 bit; exakte Breiten sonst). → Eigenbau:
**manueller, std-only-15-bit-Decoder** (IFD-Parse, zlib-inflate, 15-bit
MSB-first-unpack, Tile-Stride = volle Tile-Breite bei gepolsterten Kanten),
im archivar `--cog-ndwi`. Dazu UTM-Projektion (COG liegt in EPSG:32645):
`utm`-Crate (pure Rust). **20 Tests grün.**

### Messung (08-12, Vor-Ereignis-Baseline)

- **Langjie Cuo (28.4070, 85.4018)** ±0.02°: 176000 Pixel, **Wasser 0**,
  NDWI mean −0.330, max **0.071** → **kein offenes Wasser**.
- **Tuomito (28.3924, 85.4186)** ±0.02°: 176000 Pixel, **Wasser 0**,
  NDWI mean −0.388, max **0.071** → **kein offenes Wasser**.
- **Bhote-Koshi-Tal (28.28, 85.38)** ±0.02°: **150 Wasser-Pixel** (NDWI>0.2),
  max **0.252** → **der Fluss wird erkannt** (Tool-Messkette korrekt).

**Fazit:** Die Messkette funktioniert (COG-Quelle + std-only-15-bit-Reader +
UTM-Projektion; der Bhote-Koshi-Fluss wird als Wasser erkannt). An den zwei
OSM-„See"-Koordinaten ist am 08-12 **kein offenes Wasser** — die Koordinaten
liegen woanders, oder die Merkmale sind keine offenen Wasserflächen. Die
gemessene Seebaseline (Wasserfläche vor dem Ereignis) ist damit **0 / keine**
an diesen Punkten — als gemessener Wert, nicht erfunden.

