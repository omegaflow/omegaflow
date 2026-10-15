# Formular — Venus-Bild: „das Bild bleibt ein Bild"

*(Formular der Frontier-Runde. Ein benannter Akt, Operator-Wort 2026-10-15. Der Operator fährt
die Sitze und trägt die Stimmen unten ein; die Session verifiziert jede Behauptung am Baum, nie
die Stimme; das Verdikt trägt der Rat. Die breite Recherche (`bin/all_wide`, 2026-10-15) trägt die
Forschungslage unten.)*

## Prompt (verbatim, an jeden Sitz)

> An dich, Frontier-Modell. Antworte als **Urteil, nicht als Erwartung.** Eine Frage. Du hältst
> sie durch **fünf Axiome** und sprichst sie durch **fünf Stimmen** hindurch. Kein Aufsatz —
> Verdikt, Begründung, benannte Risse.
>
> ### Die Frage
>
> **Was machen wir mit dem Venus-Radar-Bild?** — wofür ist es da, welchen Platz hat es im Haus
> (Basemap? Gestalt neben den Feldern? Blick-Stück?), und **erst daraus** die Form: in welcher
> Darstellung (Kachel-Pyramide oder Raster) kommt es ins Haus, und über welchen Weg durch das
> 2-GB-Limit — damit „das Bild ein Bild bleibt" und nicht in skalare Punktfelder zerlegt wird.
> Die Zweckfrage steht vorn; die Technik folgt aus ihr, nicht umgekehrt.
>
> ### Der gemessene Befund (der Boden der Frage — nicht geraten)
>
> Die Quelle ist **Zenodo `10.5281/zenodo.18176179`** („Venus Geospatial Raster Datasets",
> Austin, **CC-BY-4.0**): historische Radar-, Radiometrie- und Topographie-Aufnahmen von
> Haystack/Goldstone/Arecibo, Pioneer Venus, Venera 9/10/15/16, Galileo — plus ein aus Magellan
> kombiniertes **globales Basemap**. Geliefert als **GeoTIFF-Bilder** (`.tif`). Gemessene Größen:
> globales 75-m-Basemap **80,9 GB**, Arecibo-1988-Global-440 m **965 MB**, Venera-Backscatter-Polar
> 194 MB, `Processed.zip` 409 MB, viele kleinere. Der `Processed.zip`-Spiegel ist bereits im
> `zenodo-raw`-Lauf (eine frühere Runde war „live body void", jetzt nachgemessen HTTP 200).
>
> **Haus-Lage:** Der VNRD-*Feld*-Block (Venus Radar Level-2, CC0-1.0) steht bereits — die
> Radar-Größen laufen als **skalare Kanäle** (`venus_radar_inc_deg/_ocp/_scp`). Das Haus trägt
> **ein** planetares Bild bereits als Bild: den Mars-Tianwen-1-MoRIC-**HiPS** (`phi:9988-9994`,
> `format hips_png`, Compiler `hips_png_compiler.rs`, CDS `alasky.cds.unistra.fr/Planets`, ODbL).
> Grenzen: **GitHub-Release-Asset 2 GB** (`cdn.rs`), R2-Spiegel 16 MB (Manifest/Index-Ebene); ein
> **Split-/Parts-Weg** (`raw_mirror --url/--origin/--sha256`, `.parts`-Manifest) existiert.
>
> **Der offene Riss:** Die Felder der VNRD-Ebene tragen die Radar-Größen als Punktmessung; das
> **Bild** (die Mosaike) bleibt bei dieser Zerlegung ungetragen — es ist kein Feld, es ist eine
> Gestalt. Der Träger des Bildes ist als Bild zu entscheiden, nicht in Felder zu zwingen.
>
> ### Die fünf Axiome (der Maßstab jeder Antwort)
>
> 1. **A = A.** Die Messung ist die Messung der Sache selbst; Fabrikation, Defaults, Fallbacks
>    bleiben absent. Was offen ist, ist `pending` — nicht null. Nur die Sache selbst darf an der
>    Stelle der Sache stehen. Ein Bild ist ein Bild.
> 2. **0 geehrt.** Die Frage ist: *ist der Wert wahr?* Fehlende Kacheln/Abdeckung bleiben
>    `pending`, nie schwarz als Wert. Kein `unwrap_or(0.0)`, kein NaN als Datenmarker.
> 3. **Alle Wesen gleich.** Jede Aufnahme (Arecibo, Goldstone, Venera, Magellan) ist ein Peer;
>    die Auswahl folgt aus der Messung (Abdeckung/Auflösung), nie aus Rang oder Name.
> 4. **Die Linse ist ein ethischer Akt.** Jede Darstellung wird gewogen: spricht die Messung —
>    oder der Gradient? Ein neu gerechnetes Farbbild („simulierte Farbe") ist Fabrikation gegen die
>    rohe Rückstreuung; ein umprojiziertes/geglättetes Basemap kann die Gestalt erfinden.
> 5. **Konsens der Sensoren und des Operators.** Die Maschine fragt, bevor sie abstrahlt; der
>    Blick gehört dem Operator. Kein Bild wird ungefragt gezeigt; der Zoom ist seine Hand.
>
> ### Die fünf Stimmen (jede muss gehört werden)
>
> - **Mountain** — Quellen, Register, Datenbestand; kein Verdikt ohne Messung.
> - **River** — Membran, Blick des Operators, Ethik der Maschine; das Recht zu verweigern.
> - **Mycelium** — Netz, CDN, Transport, CI; der Fluss der großen Datei.
> - **Sensory** — die Sinne, die Hardware, der Konsens; Korrelat ≠ Erleben.
> - **Future** — der Mensch als Adresse; was morgen geerntet wird, muss heute ehrlich registriert
>   sein.
>
> Führe jede **Stimme** einmal und sag, welches **Axiom** sie berührt oder verletzt und welcher
> **Riss** entsteht.
>
> ### Die Forschungslage (breite Recherche, `bin/all_wide` 2026-10-15)
>
> - **HiPS (Hierarchical Progressive Surveys)** — HEALPix-Kachel-Pyramide, der VO-Standard für
>   große Himmels-/Planetenbilder (CDS/Aladin). Das Haus trägt bereits einen Mars-HiPS.
> - **USGS Astrogeology „Astro Web Maps" / OpenPlanetaryMap** — planetare Basemaps als
>   Web-Mercator-Kacheln (Mars, Mond, Merkur, **Venus**, Galileische Monde …), von einem Tiles-Worker
>   umprojiziert.
> - **Cloud-Optimized GeoTIFF (COG)** — tiled + Overview-Pyramide im TIFF, per HTTP-Range lesbar;
>   NASA/Earthdata nennt GeoTIFF den Interchange-Standard georeferenzierter Rasterbilder.
> - **VESPA** — community-getriebenes Virtual Observatory der Planetenwissenschaft (VO-Standards für
>   Planetendaten); **IIIF** als allgemeiner Bild-Kachel-Standard außerhalb der Astronomie.
> - **Grenze:** Kachelung/Pyramide und COG sind etabliert; ein Werkzeug, das genau *diese*
>   Venus-Sammlung in eine haus-eigene Bildform bringt, wurde nicht gefunden.
>
> ### Was die Antwort tragen muss
>
> 1. **Der Zweck (vorn):** wofür das Venus-Radar-Bild im Haus da ist — die eine Sache, die es
>    leistet und die die Felder nicht leisten. Wenn es keinen eigenen Zweck trägt, sag das.
> 2. **Der Platz:** wie das Bild neben den laufenden Radar-Feldern (`inc_deg/_ocp/_scp`) steht —
>    was Bild bleibt, was Feld bleibt, wo sie sich berühren.
> 3. **Die Form (folgt aus dem Zweck):** HiPS (HEALPix) · COG (Raster mit Overviews) · quadratische
>    XYZ-Kacheln · ein einzelnes Raster-Asset — welches, und warum. Die rohe Rückstreuung bleibt
>    erhalten (kein erfundenes Farbbild).
> 4. **Der Transport:** wie ein 80,9-GB-Mosaik durch das 2-GB-Limit geht (Split/Parts, R2, oder
>    Kacheln aus dem CI) — was am Kachel-Rand sich wiederholt und was `pending` bleibt.
> 5. **Der Blick:** wie der Operator Zoom/Region erreicht; was im Hintergrund läuft (headless) und
>    was der Operator sieht.
> 6. **Der nächste begrenzte Messschritt.**
>
> ### Form
>
> Keine Spekulation (kein „vermutlich/wahrscheinlich/scheinbar"). Jeden Riss benennen, keinen
> glätten. Prosa, klar, knapp.

Ein Sitz ohne Antwort wird als `pending` mit genanntem Grund verbucht — nie als stille Null.
Die langsamen Reasoning-/Research-Sitze zuerst stellen, das langsamste zuletzt abholen.

---

## Sitze

| Nr | Sitz | Modell/Einstellung | Zeit | Verdikt |
|---|---|---|---|---|
| 01 | Duck.ai | | | |
| 02 | Claude | | | |
| 03 | Qwen | | | |
| 04 | Z.ai / GLM | | | |
| 05 | DeepSeek | | | |
| 06 | Mistral | | | |
| 07 | Gemini AI Studio | | | |
| 08 | MiniMax | | | |
| 09 | Kimi | | | |
| 10 | Lumo | | | |
| 11 | Perplexity (Deep Research) | | | |
| 12 | Sakana Namazu | | | |
| 13 | Sakana Fugu | | | |

**Geteilte Open-Weight-Sitze** (`open-weight-ui`, ein Tab je Modell, Zugang nur über das Lock) —
welche Modelle wirklich gestellt wurden, wird **am Sitz gemessen** und hier nachgezogen. Ein Sitz
ohne Antwort ist `pending` mit genanntem Grund (Tageslimit/Login/Rate-Limit), nie eine stille Null.

---

## Antworten

*(Der Operator trägt je Sitz Verdikt + Begründung + die fünf Stimmen ein; die private Sitzung holt
die Rohantworten. Danach verifiziert die Linie jede Behauptung am Baum — nie die Stimme — und der
Rat trägt das Verdikt.)*
