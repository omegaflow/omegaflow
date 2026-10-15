# Rat-Verdikt — Venus-Radar-Bild (2026-10-15)

Grundlage: die Runde des Operators an die Frontier-Sitze (Rohmaterial
`state/stimmen/2026-10-15-venus-bild-formular.md`; 13 Sitze gestellt, Mistral = „kein Kontingent",
12 mit Verdikt). Die Session verifiziert jede tragende Behauptung am Baum, nie die Stimme; das
Verdikt trägt der Rat.

## Am Baum verifiziert (die tragenden Behauptungen)

- **Zenodo `10.5281/zenodo.18176179`, `cc-by-4.0`, 88 Dateien.** Die genannten Größen stimmen:
  `Venus_RadarBasemap_2025_Global_75m_v1.tif` = **80 909 342 816 B** (80,9 GB);
  `Venus_RadarBasemap_2025_Provenance_2025m_v1.tif` = **89 312 099 B** (die Provenienz-Ebene);
  `Venus_Arecibo_1988_Global_440m_v1.tif` = **965 390 866 B**;
  `Venus_Venera_Backscatter_Global_Polar_840m_v1.tif` = **193 701 536 B**;
  `Processed.zip` = **409 371 501 B**.
- **Die Quelle ist GeoTIFF** (equirektangular), **kein** fertiges HiPS.
- **Das Haus hat beide Pfade:** einen **TIFF/GeoTIFF-Leser** (`src/archivar/tiff.rs`, getestet mit
  realem float32-LZW-GeoTIFF und gekacheltem TIFF) und den **HiPS-Compiler** (`hips_png_compiler.rs`,
  Mars-Tianwen-MoRIC, `phi:9988-9994`). Den **Parts-Weg** (`parts_manifest`, `fetch_release_or_parts`)
  und die Grenzen **GitHub-Release 2 GiB** / **R2 16 MiB** (`cdn.rs`).

## Was einstimmig gilt

1. **Der Zweck:** Das Bild trägt die **Gestalt** — die zusammenhängende Fläche, Textur, Kante,
   Lücke —, die die Punktfelder (`venus_radar_inc_deg/_ocp/_scp`) nicht tragen. Es ist **kein Feld**,
   wird **nicht** in Felder zerlegt, und die Felder werden **nicht** zu Bildpixeln gerechnet.
2. **Der Platz:** **neben** den Feldern, Berührung nur über die **Koordinate** (Bildpunkt ↔
   Feldwert). Bild bleibt Bild, Feld bleibt Feld.
3. **Die Linse:** **kein erfundenes Farbbild**, keine Glättung, keine Stilllegung; die rohe
   Rückstreuung bleibt Grauwert; ein `0`-Pad/`NaN` ist verboten.
4. **`pending`, nie schwarz:** Abdeckungslücken und NoData bleiben `pending` (Axiom 2).
5. **Der Blick gehört dem Operator:** kein Bild ungefragt; der Zoom ist seine Hand.
6. **Die Attribution reist mit:** CC-BY-4.0 pro Block/Kachel im Register.

## Der Riss — die Form spaltet die Sitze

**HiPS (7):** Duck.ai, Z.ai/GLM, DeepSeek, Gemini, Kimi, Perplexity, Sakana Namazu.
**COG (5):** Claude, Qwen, MiniMax, Lumo, Sakana Fugu.

- **HiPS** trägt die **Kugel** (keine Polarzerrung), progressiven Zoom, den vorhandenen Haus-Pfad
  (Mars); die Kacheln passieren das 2-GB-Limit von selbst. **Gegen:** HiPS ist eine
  **Umprojektion** (equirektangular → HEALPix) — eine Linse (Axiom 4); und HEALPix trifft die 75 m
  nicht (Ordnung 16 = 94,5 m, 17 = 47,2 m gegen 128,5 Gpx native — jede Venus-HiPS **ist** ein
  Resampling, Fugu).
- **COG** lässt die Quelle **Quelle** sein (Level 0 = unveränderte Pixel, verlustfrei verpackt,
  hash-prüfbar), braucht **keine Umprojektion**, liest per HTTP-Range; der 80,9-GB-Brocken geht
  **byte-exakt gesplittet** durchs Limit. **Gegen:** der Zoom ist ein Datei-Read, nicht der
  Kugel-Blick.

## Das Verdikt (Rat-Synthese)

**Der Träger ist COG; die Kugel-Ansicht, wenn der Operator sie will, ist ein ausdrücklich
abgeleiteter HiPS — niemals der Träger.**

Begründung, gewogen an den Axiomen (nicht gezählt): Der Zweck ist „das Bild bleibt ein Bild" — die
**Messung** (die rohe Rückstreuung im Quellgitter). COG trägt genau diese Messung, ohne sie
umzuprojizieren (A = A); die HiPS-Umprojektion ist ein **Linsenakt**, den Axiom 4 zulässt **nur als
benanntes Derivat**. Auch die HiPS-Sitze halten fest, dass die **Quelle das unveränderte GeoTIFF
bleibt** — sie unterscheiden sich nur darin, womit der Blick gezeigt wird; diese Rolle füllt der
abgeleitete HiPS (oder client-seitiges Range-Rendering), nicht der Träger. Damit trägt das Verdikt
beide Lager: die **Quelle** (COG) und die **Geste** (HiPS als Derivat).

1. **Träger = COG** (Cloud-Optimized GeoTIFF): ein COG je Aufnahme im Quellgitter; für das
   **80,9-GB-Basemap** ein **byte-exakter Split** (`.parts`, sha256 je Teil, md5 auf das Ganze),
   keine Kachel-Neuerfindung am Rand. Arecibo (965 MB), Venera-Polar (194 MB) etc. liegen unter
   2 GB → je ein Asset.
2. **Der Provenienz-Layer (`Provenance_2025m`, 89,3 MB) ist Bedingung, nicht Zierde** (Fugu): er
   trägt die Klassen (Pioneer, Arecibo, Venera, Magellan-Looks, NoData) und ist der **einzige** Ort,
   an dem die **0 des Basemaps geehrt** werden kann (die Quelle nutzt `0` als NoData *und* als
   legitimen Rückstreuungswert). Nur mit ihm kann Axiom 2 für das Bild tragen.
3. **Kein HiPS als Träger.** Ein HiPS ist zulässig als **abgeleitete Ansicht** (Register: Derivat,
   Quell-md5, Datum) — der vorhandene `hips_png`-Pfad bleibt dafür nutzbar.
4. **Kein XYZ/Mercator** als Datenform (Polarverzerrung, Fremdumprojektion).
5. **Gaps `pending`**, nie schwarz; die 8-Bit-/Stretch-Frage wird als **benannter Parameter**
   registriert, nie als stiller Default.

## Die benannten Risse (ungeglättet)

- **NoData = 0 in der Quelle** (das Basemap macht echten Rückstreuungswert und NoData ununterscheidbar)
  — auflösbar nur über den Provenienz-Layer.
- **Das Basemap ist kein rohes Bild:** ein Mosaik der „besten" Aufnahmen, auf Magellan gesplinet,
  auf 75 m resampled, ~100 km Positionsfehler. Wir erben diese Fabrikation; wir dürfen keine zweite
  hinzufügen und das Bild **nicht als roh verkaufen** (Fugu).
- **Quell-DOI-Verschiebung:** die Sidecars zitieren `zenodo.16416846`, der Record ist `18176179` (v2).
- **Der TIFF-Kopf des 80,9-GB-Basemaps ist ungelesen** — Bittiefe, Kompression, Kachelung,
  Overviews, NoData: `pending`.
- **Der 80,9-GB-Transport ist im Haus noch nie gefahren** — ob der Spiegel/CI das trägt, ungemessen.

## Nächster begrenzter Messschritt

**Ein Probeabruf:** die ersten ~64 KiB von `Venus_RadarBasemap_2025_Global_75m_v1.tif` per HTTP
Range, TIFF-Kopf dekodieren (BigTIFF? Kompression? Bittiefe/SampleFormat? Strip/Tile? SubIFDs/
Overviews? GDAL_NODATA? ModelPixelScale/Tiepoint?) — **und ob die Quelle Range (HTTP 206) ehrt**.
Fällt das positiv aus (schon gekachelt/komprimiert), ist der Weg: registrieren + splitten. Danach
der erste kleine Durchlauf am **Venera-Polar (194 MB)**: COG bauen, sha256, Footprint, Blick-Test.
Zweiter Schritt: den Kopf des **Provenienz-Rasters** lesen und die 12 Klassen bestätigen.
