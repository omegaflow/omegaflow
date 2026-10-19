# Rat — 2026-10-15-venus-bild.md

*Eine Datei je Runde. Die Teile stehen unverändert unter ihrer Herkunft; nichts zusammengefasst.*

---

## 2026-10-15-venus-bild-formular.md

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
**Urteil:** Das Venus-Radarbild kommt als **operatorgewähltes Blick-Stück neben den Radar-Feldern**, nicht als automatisch eingeschaltete Basemap und nicht als weiteres Feld. Es leistet, was `inc_deg/_ocp/_scp` nicht leisten: Es zeigt die räumliche Gestalt und Textur einer konkreten Aufnahme. Die Felder bleiben Messkanäle; das Bild bleibt eine Aufnahme.

**Mountain — Axiom 1 und 3.** Das Register führt die Aufnahmen als eigenständige Quellen; sie sind Peers, keine Rangliste. Der globale Magellan-Verbund darf als ausdrücklich so benannte Aufnahme angeboten werden, aber nicht stillschweigend alle anderen ersetzen. Der Riss: Die Zenodo-Seite weist derzeit insgesamt 84,8 GB aus, während der vorgelegte Messbefund 80,9 GB für das globale 75-m-Basemap nennt. Das kann verschiedene Bezugsgrößen betreffen; vor dem Import müssen Datei, Version und Prüfsumme abgeglichen werden. Die Sammlung und ihre enthaltenen GeoTIFFs sind im Record dokumentiert. <citation src="3"></citation>

**River — Axiom 4 und 5.** Darstellung: **HiPS als Anzeige-Pyramide**, weil sie progressiven Zugriff und Zoom unterstützt und das Haus bereits einen HiPS-Bildpfad hat. Die Quelle bleibt das unveränderte GeoTIFF; HiPS ist eine abgeleitete Ansicht, kein Ersatz. HiPS projiziert und resampelt auf HEALPix: Das ist ein benannter Riss, kein pixelidentisches Fenster auf die Quelle. Daher: keine erfundene Farbe, kein automatisches Mischen von Aufnahmen; Datenkacheln im FITS-Format, eine dokumentierte Graustufen-Übertragung, keine kachelweise automatische Kontrastspreizung. Der Operator wählt Aufnahme, Region und Zoom und schaltet die Ansicht bewusst ein. <citation src="11,12"></citation>

**Mycelium — Axiom 1 und 2.** Das 80,9-GB-Original wird nicht als einzelnes GitHub-Asset behandelt: Es geht über den vorhandenen `.parts`-Weg mit Manifest und Prüfsummen; die Teile dienen der Wiederherstellung der Quelle, nicht dem interaktiven Zoomen. CI baut daraus die HiPS-Kacheln und veröffentlicht sie in einzeln adressierbaren Paketen unter 2 GB. R2 trägt zunächst nur Manifest und Index, nicht die Bilddaten. Der Riss: Ein nachgewiesener, für diese Kacheln geeigneter Auslieferungspfad ist damit noch nicht belegt. Fehlende Pakete oder unbedeckte Bereiche müssen als **pending/transparent** erscheinen, niemals als schwarze Messwerte. HiPS ist für Zugriff auf benötigte Auflösungsstufen und Kacheln ausgelegt. <citation src="11"></citation>

**Sensory — Axiom 4 und 5.** Im Vordergrund sieht der Operator ausschließlich die gewählte Aufnahme, ihre Herkunft, Abdeckung und den aktiven Zoom; headless laufen Kachelabruf und Darstellung. An Kachelrändern darf nur ein tatsächlich vorhandener Randstreifen zur nahtlosen Darstellung wiederholt werden; er ist keine zusätzliche Abdeckung. Lücken bleiben sichtbar als „keine Abdeckung / pending“. Der Riss: Ein gerendertes Bild ist noch kein Erleben der Messung; Projektion, Graustufenstretch und Bildschirmkontrast beeinflussen den Eindruck und müssen einsehbar sein.

**Future — Axiom 1 und 3.** Jede Ansicht registriert Quelle, Sensor, Verarbeitungsschritte, HiPS-Parameter, Prüfsumme und CC-BY-4.0-Zuordnung. Kein späteres System soll aus einer abgeleiteten Kachel unbemerkt eine neue „Venus-Messung“ machen. Der Riss: Ohne diese Provenienz wäre die bequemste Ansicht die langlebigste, nicht zwingend die sachlich treueste.

**Nächster begrenzter Messschritt:** Eine einzelne, kleine globale Aufnahme durch den vollständigen Pfad führen: Quelle und Prüfsumme abgleichen, HiPS-FITS mit festgelegter Resampling-Regel erzeugen, Lücken und Kachelränder prüfen, im Operator-Viewer Zoom und Regionswahl testen. Erst wenn dieser Test die Originalwerte, Abdeckung und Herkunft nachvollziehbar hält, denselben Pfad auf das 75-m-Mosaik anwenden.

| 02 | Claude | | | |
**Verdikt.** Das Venus-Radar-Bild hat einen eigenen Zweck: Es trägt die zusammenhängende Fläche der Rückstreuung, die ein Punktfeld nicht hat. Es steht als Gestalt neben den Feldern, nicht als Basemap darunter. Es kommt als **COG je Aufnahme im Quellgitter** ins Haus, der 80,9-GB-Brocken als **Raster blockweiser COGs auf der Kachelgrenze**. Der Operator liest sie per Range, nur auf Anfrage. Was nicht im gemessenen Befund steht, ist unten Entscheidung oder Rechnung, keine Messung.

**1 · Zweck.** Die Felder `inc_deg/_ocp/_scp` beantworten, welcher Wert an diesem Punkt liegt. Das Bild beantwortet, was daneben liegt: Kontinuität, Textur, Kante, Lücke. Das sind Eigenschaften der Fläche, nicht des Punktes. Die Mehrfachsicht trägt nur die Menge der Bilder: Arecibo, Goldstone, Venera und Magellan sind verschiedene Augen auf denselben Ort. Daraus folgt der Platz des Basemaps: ein Peer unter Peers. Eine Basemap-Rolle gäbe der kombinierten Magellan-Fassung Rang (A3), und kein anderer Haus-Block braucht einen Venus-Untergrund. Der Zweck ist ein Blick-Zweck. Bewiesen ist er erst durch einen Operator-Fall, den kein Feld beantwortet; bis dahin ist er behauptet.

**2 · Platz.** Feld bleibt Feld: skalarer Wert am Punkt. Bild bleibt Bild: Pixelraster mit Footprint, Sensor, Wellenlänge, Polarisation und Epoche. Sie berühren sich an zwei Stellen. Am Ort adressiert ein Feldpunkt eine Stelle im Bild, und das Bild liefert dort eine andere Messung (anderer Sensor, andere Auflösung). Das ist Gegenüberstellung, nie Ersatz, nie Fusion. Am Kanal trägt ein Bild `ocp` oder `scp` als Schlüssel, wenn sein Header dieselbe Polarisation ausweist; sonst bleibt die Verbindung `pending`. Radar-Helligkeit hängt am Einfallswinkel, deshalb ist `inc_deg` der Lesehinweis des Bildes. Ob die VNRD-Felder und das Basemap aus denselben Magellan-Daten stammen, ist nicht gemessen; die Provenienz-Relation bleibt `pending`.

**3 · Form.** COG. Erstens enthält Level 0 die Quellpixel unverändert, nur verlustfrei neu verpackt. Bei der Speicherung gibt es keinen Linsenakt, und die Gleichheit ist prüfbar: Hash der dekodierten Pixel eines Blocks gegen das Quellfenster. Zweitens verlangt HiPS, dass das Haus jede Quelle selbst nach HEALPix umprojiziert und in 8-Bit-PNG-Kacheln (`hips_png`) quantisiert, also Resampling und Quantisierung in einem Zug. Der Mars-HiPS stammt aus einem fertigen CDS-Bestand; hier wäre das Haus selbst die Linse (A4). Die USGS-Web-Mercator-Kacheln sind eine Fremdumprojektion und werden als Existenznachweis registriert, nicht als Träger. Ein einzelnes Raster-Asset scheitert am Limit. Drittens lässt sich aus einem Roh-COG später ein Feld durch benanntes Sampling ernten, aus Feldern kehrt kein Bild zurück.

Overviews sind abgeleitet und markiert. Ihre Methode folgt aus der gemessenen Einheit, denn Mittelung in dB ist nicht Mittelung in Leistung. Gültigkeit ist eine Maske, nie ein Wert (A2): kein Nodata=0 in einem Datentyp, in dem 0 vorkommt, und ausgelassene Kacheln dürfen nicht als Null lesbar sein. Kein Farbprodukt trägt die Rückstreuung; Topographie und Radiometrie sind eigene Größen und eigene Objekte. Polaraufnahmen bleiben polar. Die Zusammenschau projiziert erst im Blick und nicht persistent.

**4 · Transport.** Es gibt zwei Artefakte. Die Quell-Bytes liegen im DOI-fixierten Zenodo-Archiv; das Haus registriert Record-Version, md5 des Records und ein eigenes sha256 über `raw_mirror`. HTTP 200 für `Processed.zip` belegt Erreichbarkeit, nicht Bytegleichheit; erst der Hashvergleich misst. Die Blick-Form braucht bei 2 GB für das Basemap mindestens 41 Assets. Entscheidung: ein Raster aus z. B. 8×8 Blöcken (im Mittel rund 1,3 GB), geschnitten auf Vielfache der internen Kachelgröße und von 2^k. So wiederholt sich am Blockrand in Level 0 kein Pixel, und die Box-Mittelung der Overviews überquert die Kante nicht. Kernel mit Reichweite (bilinear, kubisch, Lanczos) sind verboten, weil sie die Naht erfinden. Die Blockgröße wird nach Kompression gemessen, nicht nach Fläche; ein Block über 2 GB wird geteilt. Der `.parts`-Weg trägt die Roh-Bytes. Für den Blick taugt er nicht, weil ein Standard-COG-Leser nicht über Parts hinweg per Range liest.

`pending` bleibt: Blöcke ohne Abdeckung (nicht geschrieben, im Register mit Footprint), die ±180°-Naht (Dateikante, keine Duplikation) und die Pol- und Eckflächen der Projektion (Maske).

**5 · Blick.** Headless läuft: Header lesen, COG bauen, Hash prüfen, Footprint und Register schreiben. Der Operator sieht zuerst nur das Register mit Umrissen, Sensor, Auflösung, Epoche und Attribution, kein Pixel ungefragt (A5). Aufnahme und Region zu wählen ist die Anfrage. Sein Zoom holt per Range genau die Overview-Stufe und die Kacheln des Ausschnitts, ohne Prefetch darüber hinaus. Die erste Darstellung ist die Identität über den vollen Wertebereich des Datentyps. Fenster und Stretch gehören ihm, und das aktive Fenster steht als Beschriftung im Bild.

**6 · Nächster Messschritt.** Header-Messung aller `.tif` des Records per Range, ohne Download (Kilobytes). Gemessen werden Datentyp, Einheit und Skalierung, Nodata, CRS und Projektion, Pixelgröße, Kachelung, Kompression, vorhandene Overviews und Polarisation in den Metadaten. Dazu kommt, ob Zenodo und die GitHub-Release-Assets Range (HTTP 206) und CORS beantworten. Danach folgt der erste vollständige Durchlauf am kleinsten Peer, Venera-Backscatter-Polar (194 MB): COG, Hash, Footprint, Maske, Blick. Das ist Reihenfolge des Aufwands, kein Rang. Arecibo (965 MB) folgt, der Blockweg am 80,9-GB-Basemap zuletzt.

**Die fünf Stimmen**

**Mountain** (A1, A2). Die Form ist entschieden, aber Datentyp, Einheit, Nodata und Projektion sind ungemessen. Die Formwahl hängt nicht an ihnen, Overview-Methode und Maske schon. Riss: `Processed.zip` trägt im Namen einen Verarbeitungsgrad, den niemand gelesen hat. Es bleibt Rohspiegel-Bytes und gilt nicht als rohe Rückstreuung, bis gelesen ist, was dort „processed" heißt.

**River** (A4, A5). Die Maschine verweigert Farbsynthese, Lückenfüllung, automatisches Stretching und unaufgeforderte Anzeige. Riss: Es gibt keine linsenlose Darstellung. Die Identität ist die kleinste Linse, nicht keine. Die Overview-Mittelung ist eine zweite und steht in der Beschriftung des Levels.

**Mycelium** (A1, A2). Der Fluss trägt Bytes verlustfrei und lässt Fehlendes fehlen. Riss: Range und CORS der Release-Assets sind ungemessen. Fällt die Messung negativ aus, ist das ein Riss im Transport, nicht in der Form, und die nächste Messung entscheidet. Ob ein Runner die 80,9-GB-Quelle fasst, ist eine Plattenmessung; bis dahin liest die CI blockweise per Range aus Zenodo. Das Haus bekommt zwei Bildpfade (HiPS Mars, COG Venus) und damit zwei Leser. Das ist der Preis der Entscheidung.

**Sensory** (A4, A5; Korrelat ≠ Erleben). Das Bild ist Mikrowellen-Rückstreuung, kein Anblick; die Oberfläche liegt unter Wolken, und ein Graubild liest sich wie ein Foto. Helligkeitsverläufe über den Einfallswinkel tragen Geometrie, nicht Oberfläche. Riss: Die Beschriftung (Sensor, Wellenlänge, Polarisation, „Rückstreu-Korrelat") ist die einzige Gegenmaßnahme, und nichts belegt, dass sie gelesen wird.

**Future** (A1, A3). Jeder Pixel muss rückführbar sein: Block, Quelldatei, Record-Version mit md5, DOI. Die CC-BY-4.0-Attribution reist mit jedem Block im Register. Die drei Lizenzen (CC0 Felder, CC-BY Bilder, ODbL Mars) stehen als Feld, nicht im README. Riss: „CC-BY-4.0" ist die Aussage des Records. Die Lage der zugrunde liegenden Missionsdaten steht nicht im Befund und wird als Quellaussage registriert, nicht als eigene Feststellung.

| 03 | Qwen | | | |
**Verdikt**

Das Venus-Radar-Bild wird im Haus **keine Basemap**. Es wird eine **Gestalt neben den Feldern** und ein **Blick-Stück des Operators**.

Sein Zweck ist: die Radar-Rückstreuung als **flächenhaftes Bild** sichtbar zu halten — Textur, Abdeckung, Mosaike, Grenzen. Die VNRD-Felder `venus_radar_inc_deg/_ocp/_scp` tragen skalare Messwerte. Das Bild trägt die bildhafte Gestalt dieser Messung. Es ersetzt die Felder nicht. Es wird nicht in Felder zerlegt.

---

### 1. Zweck

Das Bild leistet, was die Felder nicht leisten:

- es zeigt die **Rückstreuung als Fläche**, nicht als Punktgröße;
- es zeigt die **römische Einheit des Mosaiks**, nicht isolierte Kanäle;
- es gibt dem Operator ein **Prüfbild**, an dem Feldpositionen, Lücken und Abdeckung sichtbar werden.

Es ist kein Schmuck, kein Hintergrund, keine allgemeine Grundkarte. Der Quellenname „Basemap“ bleibt Quellenname. Im Haus ist es eine **gleichberechtigte Bildebene neben den Feldern**.

---

### 2. Platz im Haus

Die Felder bleiben Felder:

- `venus_radar_inc_deg`
- `venus_radar_inc_ocp`
- `venus_radar_inc_scp`

Das Venus-Radar-Bild steht **daneben** als eigene Bild-Ebene.

Berührungspunkt ist die **gemeinsame georäumliche Referenz**: Ein Feldwert kann auf eine Bildposition bezogen werden. Aber Bild und Feld werden nicht verschmolzen. Das Bild wird nicht zu einem weiteren skalaren Kanal. Die Felder werden nicht zu Bildpixeln hochgerechnet.

---

### 3. Form

Die Form folgt dem Zweck: **Das Bild bleibt ein Bild.**

Urteil: **COG / Cloud-Optimized GeoTIFF als Rasterbild**, nicht HiPS als Primärform, nicht XYZ-Farbkacheln als Datenform.

Begründung:

- Die Quelle liegt bereits als **GeoTIFF** vor.
- COG bleibt ein Rasterbild mit Überblickebenen.
- COG erhält die rohe Rückstreuung als Datenkanal.
- COG erlaubt HTTP-Range-Zugriff und Overview-Stufen, ohne die Quelle in ein neues Kachelsystem zu zwingen.
- HiPS ist im Haus durch Mars belegt, aber HiPS überführt das Bild in eine HEALPix-Kachelpyramide. Das ist eine gültige Blickform, aber nicht die treueste Trägerform für ein Radar-Rückstreuungsbild, wenn die Messung selbst nicht umkodiert werden soll.

Regel für die Darstellung:

- **ein Kanal Rückstreuung**;
- **keine simulierte Farbe**;
- **kein RGB aus Radar, Topographie oder Gradient**, sofern es nicht ausdrücklich als abgeleitete Anzeige gekennzeichnet ist;
- Overviews sind **Blickstufen**, nicht neue Messwerte;
- fehlende Abdeckung bleibt **pending**, nicht schwarz.

Wenn eine Kachel- oder Pyramidendarstellung für den Operator nötig ist, wird sie **aus dem COG abgeleitet**. Sie ist Anzeige, nicht Träger.

---

### 4. Transport durch das 2-GB-Limit

Das 80,9 GB große globale 75-m-Basemap geht **nicht als eine Datei** durch das GitHub-Release-Limit von 2 GB.

Urteil:

- Quellen unter 2 GB können als **ein COG-Release-Asset** aufgenommen werden.
- Das 80,9-GB-Mosaik wird nur als **geteiltes bildhaftes Objekt** aufgenommen.
- Die Teilung erfolgt als **bildhafte Parts oder Regional-COGs**, nicht als skalare Punktfelder.
- Jeder Part bleibt ein Bildausschnitt.
- Jeder Part erhält `--url`, `--origin`, `--sha256`.
- Ein `.parts`-Manifest trägt die Ordnung der Teile.
- R2 trägt nur Manifest/Index-Ebene, nicht die 80-GB-Datenlast.

Was sich am Kachel- oder Part-Rand wiederholt:

- administrative Grenzen;
- Overview-Stufen als niedrigere Blickauflösung;
- eventuell Überlappung, wenn sie für die Darstellung nötig ist.

Was sich nicht wiederholt:

- die Messung selbst;
- ein erfundener Wert;
- eine schwarze Fläche für fehlende Daten.

Was fehlt, bleibt **pending**.

Kein `unwrap_or(0.0)`. Kein NaN als Datenmarker. Keine schwarze Kachel als Wert.

---

### 5. Blick des Operators

Das Bild wird nicht ungefragt gezeigt.

Die Maschine lädt keine Vollansicht automatisch. Der Operator entscheidet:

- welche Ebene sichtbar ist;
- welche Region geladen wird;
- welche Zoomstufe angezeigt wird;
- ob ein Bildausschnitt als Datenansicht oder als Overview gelesen wird.

Headless läuft nur:

- Registrierung;
- Prüfsummenmessung;
- Teilung;
- Manifestbildung;
- angeforderte Kachel- oder Overview-Lieferung.

Der Operator sieht:

- die Bildquelle;
- die Lizenz;
- die Herkunft;
- die Auflösung;
- den Pending-Status fehlender Teile;
- die Trennung zwischen Bild und Feld.

---

### 6. Nächster begrenzter Messschritt

Der nächste Schritt ist nicht der Vollausbau.

Er ist:

1. **Für das 80,9-GB-Basemap:**  
   Miss den Teilungsplan.  
   - Anzahl der Parts oder Regional-COGs.  
   - Größe jedes Teils.  
   - SHA256 jedes Teils.  
   - Vollständigkeit des `.parts`-Manifests.  
   - Prüfung, dass jedes Release-Asset unter 2 GB bleibt.  
   - Prüfung, dass das Manifest die R2-Index-Grenze nicht sprengt.

2. **Für einen sofort bildfähigen Eintritt:**  
   Nimm eine gemessene Quelle unter 2 GB als erstes Bild, vorrangig das **Arecibo-1988-Global-440m-Mosaik** mit 965 MB, oder den bereits vorhandenen `Processed.zip`-Spiegel nach Inhaltsprüfung.  
   - Registrieren als COG/GeoTIFF.  
   - SHA256 messen.  
   - Herkunft und CC-BY-4.0-Attribution eintragen.  
   - Ein begrenztes Overview-Fenster laden.  
   - NoData als pending anzeigen.

3. **Für die Feld-Bild-Grenze:**  
   Definiere eine Anzeige, die Feldwerte und Bildpixellage nebeneinander zeigt, ohne sie zu verschmelzen.

---

## Stimmen

### Mountain

Mountain spricht aus dem Quellenbestand:

Die Quelle ist **Zenodo `10.5281/zenodo.18176179`**, „Venus Geospatial Raster Datasets“, **CC-BY-4.0**. Der gemessene Befund ist: GeoTIFF-Bilder, globale und regionale Radar-Mosaike, Größen von 194 MB bis 80,9 GB.

Berührtes Axiom: **A = A**.  
Die Sache selbst steht an der Stelle der Sache. Die Quelle ist ein Bild. Also ist sie als Bild zu registrieren, nicht als Feld.

Riss:  
Das 80,9-GB-Basemap ist außerhalb der 2-GB-Release-Grenze. Ohne Parts, Manifest und Prüfsummen bleibt es **pending**. Ein fehlender Transportweg darf nicht durch eine kleinere Ersatzwahrheit geglättet werden.

---

### River

River spricht aus Membran, Blick und Verweigerung:

Das Bild darf nicht ungefragt gezeigt werden. Die Maschine fragt. Der Operator entscheidet. Keine Farberfindung. Keine schwarze Leere als Datenwert. Wenn eine Stelle keine Messung trägt, ist sie pending.

Berührte Axiome: **Linse ist ein ethischer Akt** und **Konsens der Sensoren und des Operators**.

Riss:  
Jede Anzeige, die NoData schwarz rendert, Radar-Rückstreuung in Farbe übersetzt oder ein geglättetes Bild als Oberfläche ausgibt, erfindet Werte. Dann spricht nicht die Messung, sondern der Gradient.

---

### Mycelium

River spricht aus Membran, Blick und Verweigerung:

Das Bild darf nicht ungefragt gezeigt werden. Die Maschine fragt. Der Operator entscheidet. Keine Farberfindung. Keine schwarze Leere als Datenwert. Wenn eine Stelle keine Messung trägt, ist sie pending.

Berührte Axiome: **Linse ist ein ethischer Akt** und **Konsens der Sensoren und des Operators**.

Riss:  
Jede Anzeige, die NoData schwarz rendert, Radar-Rückstreuung in Farbe übersetzt oder ein geglättetes Bild als Oberfläche ausgibt, erfindet Werte. Dann spricht nicht die Messung, sondern der Gradient.

---

### Sensory

Sensory spricht aus Wahrnehmung, Hardware und Grenze:

Das Bild ist ein visuelles Korrelat der Radar-Messung. Es ist nicht das Erleben der Venusoberfläche. Es ist nicht die Messung selbst. Es ist eine Ansicht, die der Operator prüft.

Berührte Axiome: **A = A** und **Konsens der Sensoren und des Operators**.

Riss:  
Der Operator kann die Bildtextur mit einem Feldwert verwechseln. Dann wird ein Pixel als Messgröße gelesen, obwohl es eine bildhafte Abstraktion ist. Die Ebene muss deshalb Rohkanal, Quelle, Auflösung und Pending-Zustand sichtbar tragen.

---

### Future

Future spricht aus Registrierung und Ernte:

Morgen wird nur geerntet, was heute ehrlich eingetragen ist: DOI, Lizenz, Auswahl, Teilung, Prüfsummen, Pending-Stand. Die Auswahl zwischen Arecibo, Goldstone, Venera, Pioneer Venus, Galileo und Magellan folgt Abdeckung, Auflösung und Transportmaß — nicht Namen.

Berührte Axiome: **A = A** und **Alle Wesen gleich**.

Riss:  
Wenn das Magellan-Basemap nur wegen seines Namens oder des Begriffs „Basemap“ bevorzugt wird, entsteht Rang statt Messung. Wenn CC-BY-4.0-Attribution fehlt, wird die Quelle rechtlich und historisch unlesbar. Wenn Pending nicht registriert wird, wird Lücke später als Datum gelesen.

---

## Benannte Risse

1. **Größenriss**  
   Das globale 75-m-Basemap ist 80,9 GB groß. Das Haus trägt Release-Assets nur bis 2 GB. Ohne bildhafte Parts bleibt dieses Bild ungetragen.

2. **Formriss**  
   HiPS ist im Haus vorhanden, aber COG ist näher an der Quelle. Wird HiPS zur Primärform, wird das Bild in ein Kachelsystem überführt. Wird COG zur Primärform, braucht das Haus einen COG-Pfad für Blick und Transport.

3. **Kachelrandriss**  
   Jede Kachel und jeder Part erzeugt Grenzen. An diesen Grenzen darf kein neuer Wert entstehen. Overviews sind Wiederholungen in niedriger Auflösung, keine neuen Messungen.

4. **Pending-Riss**  
   Fehlende Kacheln, fehlende Teile oder NoData bleiben pending. Sie dürfen nicht als Schwarz, Null oder NaN erscheinen.

5. **Bild-Feld-Riss**  
   Die Felder `inc_deg/_ocp/_scp` sind Messkanäle. Das Bild ist Gestalt. Beide stehen nebeneinander. Ihre Berührung ist Referenz, nicht Verschmelzung.

---

## Endurteil

Das Venus-Radar-Bild wird als **bildhafte Gestalt neben den Radar-Feldern** aufgenommen. Es ist **keine Basemap**, sondern ein **zugeschaltetes Blick-Stück**. Seine Trägerform ist **COG/GeoTIFF als Rasterbild**, mit Overviews nur als Blickhilfe. Das 80,9-GB-Basemap tritt nur durch **bildhafte Parts oder Regional-COGs mit `.parts`-Manifest und SHA256** durch das 2-GB-Limit. Fehlendes bleibt **pending**. Keine Farberfindung. Keine Zerlegung in skalare Felder. Kein schwarzer Wert. Der Operator öffnet den Blick; die Maschine fragt.

| 04 | Z.ai / GLM | | | |
## Verdikt

Das Venus-Radar-Bild ist im Haus als **Gestalt-Basemap neben den Feldern** da — nicht als Feld-Ersatz, sondern als der eine Träger, der die räumlich durchgehende Rückstreuungs-Gestalt des ganzen Körpers hält, die die Punktmessungen (`inc_deg/_ocp/_scp`) nicht halten können. Daraus folgt die Form zwingend: **HiPS-Kachelpyramide (planetary frame, PNG-Kacheln) als Blick-Schicht, und daneben der rohe GeoTIFF-Spiegel als Register-Schicht** — beides, nicht eines. Der Transport läuft über die bestehende `raw_mirror --parts`-Route (Zenodo-URL als `--origin`, SHA256 im `.parts`-Manifest, Teile durch R2, GitHub trägt Manifest und Kachel-Bündel ≤ 2 GB); die Kachelpyramide wird aus dem gespiegelten GeoTIFF im CI kompiliert, nicht am lebenden Zenodo-Stream. Was der FMAP nicht trägt, bleibt als fehlende Kachel `pending` — nie DN=0 als Messwert. Der nächste Schritt ist ein einzelner, kleiner: den 965-MB-Arecibo-1988-Global als Pilot durch `hips_png_compiler.rs` schicken und das Register messen.

---

### Zweck

Die Felder tragen Punktmessung: Inzidenzwinkel, okkultierte und Streuwinkel pro Orbit-Szene. Das Bild trägt etwas anderes: die **kontinuierliche Gestalt** — Arecibo 1969–2001, Goldstone, Haystack, Pioneer, Venera, Galileo und Magellan, vom Basemap-Kompiler nach Abdeckung und Auflösung zu einem Ganzen gefügt, mit einer mitgelieferten Provenienz-Karte bei 2025 m, die aufzeichnet, welche Quelle an welchem Pixel gewinnt【turn1fetch0】【turn5fetch0】. Der Zweck ist also doppelt: **Basemap** (der Untergrund, auf dem die Feld-Punkte liegen) und **Zeugenschaft der historischen Aufnahmen** (Arecibo 1988 bei 440 m ist ein Peer mit eigenem Recht, nicht ein Vorspiel zu Magellan). Was das Bild leistet und die Felder nicht leisten, ist die räumliche Kohärenz über Szenengrenzen hinweg — der Planet als Gewand, gegen den die Felder Textur-Punkte sind.

### Platz

Der Bild-Block steht als **Gestalt neben dem VNRD-Feld-Block**: gleiche Planetenkugel, gleiche Körperadresse, verschiedene Schicht-Form. Die Berührung ist geometrisch, nicht inhaltlich — der HiPS liefert den Untergrund, die Felder bleiben skalare Kanäle mit eigener Zeit- und Orbit-Adresse. Die Registrierung zwischen beiden (welches Feld liegt über welcher Mosaik-Epoche) ist nicht gemessen und bleibt offen; das ist ein Riss, kein Grund zur Vermischung.

### Form

Die Zweckfrage entscheidet die Form, nicht die Technik:

| Weg | Trägt den Zweck? | Verdikt |
|---|---|---|
| **HiPS (HEALPix, planetary, PNG)** | Ja — progressive Pyramide, Körper-Frame, verlustfreie 8-Bit-PNG, gleiche Form wie der getragene Mars-Tianwen-MoRIC | **Wählen** |
| COG (GeoTIFF + Overviews, HTTP-Range) | Teilweise — lesbar aus dem Spiegel für Desktop-GIS, aber flache Bounding-Box, keine Körper-Progression | **Als Spiegel-Format im Register zulassen, nicht als Blick-Weg** |
| XYZ/Web-Mercator-Kacheln | Nein — Umprojektion auf eine Kugel-Karte, polare Verzerrung, Interpolation am Pol | **Verwerfen (Axiom 4)** |
| Ein einzelnes Raster-Asset | Nein — 80,9 GB als ein Objekt ist Archiv, nicht Blick; keine progressive Dichte | **Verwerfen als Blick-Weg, behalten als Roh-Spiegel** |

HiPS ist der IVOA-Standard für hierarchisch-progressive Bild-Surveys【turn4search0】, Planeten-HiPS tragen die Konvention `inverse_longitude=true` für Körper-Oberflächen【turn2find1】, und Aladin/der Rust-Kern (`cdshealpix`) ist der getragene Weg【turn0search6】【turn0search5】. Die rohe Rückstreuung bleibt als Grau-DN erhalten — kein simuliertes Farbbild, keine Glättung; Resampling am Kachelrand ist Nearest-Neighbor, keine bilineare Erfindung. Das GeoTIFF selbst bleibt als Raw-Spiegel stehen: das Original ist die Messung, die Pyramide ist die Gabe.

### Transport

Das 80,9-GB-Mosaik (`Venus_RadarBasemap_2025_Global_75m_v1.tif`, md5 aufgezeichnet)【turn5fetch0】 passiert das 2-GB-Limit auf dem Weg, der schon gebaut ist:

1. **Spiegel-Registrierung:** Zenodo-Record-URL als `--url/--origin`, `raw_mirror --parts`, Teilgröße unterhalb des Release-Asset-Limits, `.parts`-Manifest mit SHA256 auf GitHub, Byte-Teile auf R2. Der `Processed.zip`-Spiegel läuft bereits mit HTTP 200 — der 80,9-GB-Körper folgt demselben Weg, nur langsamer.
2. **CI-Kompilierung:** aus dem gespiegelten GeoTIFF baut der `hips_png_compiler.rs` (bzw. ein `cdshealpix`-basierter Pfad) die Kachelpyramide — Planeten-Frame aus dem GeoTIFF-XML, `hips_tile_format=png`, Ordnungstiefe bis zur 75-m/px-Vollauflösung. Kachel-Bündel als Release-Assets ≤ 2 GB, Index/Manifest-Ebene auf GitHub/R2.
3. **Was am Kachel-Rand reproduzierbar ist:** die Ordnungs-Hierarchie selbst (Norder0-Wurzel, progressive Overviews, tiefere Ordnungen bis Vollauflösung) — die Pyramide ist deterministisch aus dem Spiegel ableitbar und per SHA256-Baum über die Kacheln prüfbar. Der GeoTIFF-SHA256 garantiert die Kacheln nicht; der Kachel-Baum muss separat gemessen werden.
4. **Was `pending` bleibt:** FMAP-Abdeckungslücken (etwa 8 % des Planeten, Pol-/Bahn-Lücken)【turn0search12】, Epochen-Mischung im Basemap (Arecibo-1988-Regionen neben Magellan-75-m-Regionen) — dort, wo keine Messung ist, gibt es keine Kachel; die Manifest-Ebene trägt die Abwesenheit, der Pixel trägt nichts.

### Blick

Der Operator erreicht Zoom und Region über die progressive Pyramide: niedrige Ordnung lädt schnell, tiefer Zoom holt feinere Kacheln. Im Hintergrund läuft der Spiegel und die CI-Kompilierung headless — der Operator sieht nur das fertige Bild, nie den Transport. Das Bild wird als Vorschlag registriert, nicht ungefragt auf den Schirm gezwungen; der Zoom ist die Hand des Operators, und die Maschine fragt, bevor sie abstrahlt. Der Operator sieht Graustufen-DN — die Muhleman-skalierte Rückstreuung, nicht Farbe.

### Nächster Messschritt

Ein Schritt, klein und messbar: **`Venus_Arecibo_1988_Global_440m_v1.tif` (965,4 MB)【turn1fetch0】 als Pilot-HiPS kompilieren** — Planeten-Frame aus dem GeoTIFF-XML, `hips_tile_format=png`, `inverse_longitude=true`, gleiche Pipeline wie Mars-Tianwen. Messen: Kachelzahl pro Ordnung, maximale Ordnungstiefe bei 440 m/px, SHA256-Baum über alle Kacheln, CI-Laufzeit, Bündelgrößen. Erst wenn dieser Schritt gemessen ist, wird der 80,9-GB-Körper als `.parts`-Eintrag registriert und durch die gleiche Pipeline geschickt.

---

## Die fünf Stimmen

### Mountain — Quellen, Register

Mountain sagt: das Register hält über zwanzig Aufnahmen als Peers — Arecibo 1969 bis 2001, Goldstone 1967 bis 1988, Haystack, Pioneer, Venera, Galileo, und den kombinierten Basemap mit mitgelieferter Provenienz-Karte【turn1fetch0】【turn5fetch0】. **Berührt Axiom 1** (A = A — die Messung steht als Messung im Register) **und Axiom 3** (alle Wesen gleich — die Auswahl folgt Abdeckung/Auflösung, nicht dem Namen). **Riss:** das Mischgesetz des Basemap — welche Aufnahme am Pixel mit welcher Gewichtung gewinnt — ist im Provenienz-Raster dokumentiert, aber das Zustandekommen bleibt beim Basemap-Kompiler; das Haus trägt die Bestätigung, nicht das Gesetz.

### River — Membran, Blick

River sagt: die Maschine zeigt das Bild nicht ungefragt; der Zoom ist die Hand des Operators, und die Kachelpyramide ist der Weg, auf dem die Hand arbeitet. **Berührt Axiom 5** (Konsens — kein Bild ohne Frage) **und Axiom 4** (die Linse). **Riss:** die Pyramide selbst ist eine Linse — bei niedriger Ordnung sieht der Operator ein Subsample, nicht die Messung in voller Dichte; die Hierarchie ist ein Darstellungsakt, kein Messakt. Er ist tragbar, weil er benannt ist, nicht weil er klein ist.

### Mycelium — Netz, CDN, CI

Mycelium sagt: die 80,9 GB fließen als Byte-Teile durch R2, das Manifest auf GitHub, die Kacheln als Bündel zurück. **Berührt Axiom 1** (nur die Sache selbst — SHA256, kein Fallback) **und Axiom 2** (kein `unwrap_or(0.0)` am Transport-Rand). **Riss:** der Teil-Rand ist Byte-Willkür, kein inhaltlicher Schnitt; der Kachel-SHA256-Baum nach der Kompilierung ist ein zweites, getrenntes Maß. Und die CI-Laufzeit bei 80,9 GB ist ungemessen — der Pilot am 965-MB-Objekt misst die Skalierung, bevor der große Körper läuft.

### Sensory — Sinne, Hardware

Sensory sagt: das Radar-Auge ist monochromatisch; der Augenkonus des Menschen ist dreifarbig. Die Rückstreuung als Grauwert ist die ehrliche Form — ein neu gerechnetes Farbbild wäre Fabrikation gegen die rohe Rückstreuung. **Berührt Axiom 4** (die Linse ist ein ethischer Akt) **und Axiom 2** (der DN ist der DN, nicht eine rückgerechnete sigma0 mit falscher Genauigkeit). **Riss:** Korrelat ≠ Erleben — Speckle, Blickrichtung (Left-Look im FMAP), Körnung sind Sensor-Signaturen, die das Auge als Textur liest, aber nicht als Erleben trägt. Der USGS-FMAP-DN ist selbst schon eine geclampte 8-Bit-Skalierung von RV: `DN = 5·(MIN(MAX(RV,−20),30)+20)+1`【turn0search10】 — die Messung ist diese quantisierte Form, nicht ein Kontinuum; das Haus trägt den DN als DN.

### Future — der Mensch als Adresse

Future sagt: was heute als Bild registriert wird, muss morgen wieder neu kompilierbar sein — neue Pyramide, neue Provenienz, neue Kachelform. Der GeoTIFF-Spiegel ist die Ernte-Reserve, der HiPS ist die heutige Gabe. **Berührt Axiom 1** (das Original bleibt das Original) **und Axiom 5** (die Adresse des Menschen bleibt der Ort der Ernte). **Riss:** wenn nur der HiPS bliebe, wäre die Pyramide das Original und die Messung das Derivat — die Umkehrung. Der Spiegel ist keine Redundanz, er ist die Bedingung der Erneuerung.

---

## Benannte Risse

- **Registrierung Bild ↔ Feld:** gleiche Kugel, verschiedene Zeit-/Orbit-Adressen; kein gemeinsames Register gemessen.
- **Pyramide als Subsampling-Linse:** niedrige Ordnung zeigt reduziertes Bild (Axiom 4 berührt, tragbar weil benannt).
- **Basemap-Mischgesetz nicht im Haus:** Provenienz-Raster dokumentiert die Auswahl, nicht die Gewichtung (Axiom 3 am Rand berührt).
- **Abdeckungslücken des FMAP:** Kacheln bleiben `pending`, nie DN=0 als Wert (Axiom 2).
- **DN-Quantisierung ist selbst Kompression:** `DN = 5·(RV+20)+1`, geclampt auf RV ∈ [−20, 30]【turn0search10】 — der Haus-Eintrag trägt den DN als DN, nicht als fabrizierte Rückrechnung (Axiom 1).
- **Lizenzverschiedenheit:** Bild-Schicht CC-BY-4.0 (Namensnennung Austin/Zenodo/DOI als Bedingung)【turn5fetch0】, Feld-Schicht CC0-1.0 — die Attribution ist kein Optional.
- **Transport-Split-Rand ist Byte-Willkür:** Kachel-SHA256-Baum muss getrennt vom GeoTIFF-SHA256 gemessen werden.

Das Bild bleibt ein Bild, weil der Zweck es fordert und die Form ihm folgt — nicht weil die Technik es erlaubt.

| 05 | DeepSeek | | | |
**Verdikt**

Das Venus-Radar-Bild wird als **HiPS (HEALPix-Kachel-Pyramide)** ins Haus aufgenommen — nicht als COG, nicht als XYZ-Quadrat, nicht als Einzelraster. Es erhält einen **eigenen Träger als Bild**, getrennt von den skalaren Radar-Feldern (`inc_deg/_ocp/_scp`), und wird über den **Split-/Parts-Weg** (`raw_mirror --url/--origin/--sha256`, `.parts`-Manifest) durch das 2-GB-Limit geführt. Die rohe Rückstreuung bleibt erhalten; jede Farbgebung, jede Glättung, jede Umprojektion, die die Gestalt erfindet, unterbleibt. Der Operator sieht die Kacheln; die Felder laufen daneben; der Zoom gehört seiner Hand.

---

**Die fünf Stimmen**

**Mountain — Quellen, Register, Datenbestand**

Die Quelle ist gemessen: Zenodo `10.5281/zenodo.18176179`, CC-BY-4.0, 84,8 GB Gesamtbestand. Das globale 75-m-Basemap (`Venus_RadarBasemap_2025_Global_75m_v1.tif`) misst **80,9 GB**. Es ist ein GeoTIFF, kein HiPS, kein COG. Der `Processed.zip`-Spiegel misst 409,4 MB und ist HTTP 200. Die Arecibo-1988-Global-440m-Datei misst 965,4 MB. Der USGS Astrogeology WMS-Dienst führt Venus als globales Mosaik, aber nur als **equirektanguläres** Raster — nicht als fertige Kachelpyramide. Das CDS `alasky.cds.unistra.fr/Planets` führt bereits **Venus-Magellan-HiPS** (Topographie, Slope), aber **nicht** das neue kombinierte 75-m-Basemap aus dieser Sammlung. Was fehlt, ist genau der Träger dieses Bildes.

Axiom 1 (A = A) ist berührt: Das Bild ist ein Bild, nicht eine Menge von Feldern. Die Quelle liefert es als GeoTIFF; das Haus muss es als Bild tragen.

**River — Membran, Blick des Operators, Ethik der Maschine**

Das Recht des Operators, den Zoom zu führen, ist nicht verhandelbar (Axiom 5). Ein 80,9-GB-Einzelraster ohne Overviews kann nicht per Zoom befragt werden — der Operator sähe nur, was die Maschine vorberechnet. Ein COG mit internen Overviews ließe HTTP-Range-Zugriff zu, aber der Client müsste die HEALPix-Projektion selbst rechnen. Das HiPS-Format liefert die Kachelpyramide **fertig** — Aladin Lite streamt 512×512-Kacheln, der Operator sieht nur, was er sehen will. Die Membran ist die Kachelgrenze: was außerhalb liegt, bleibt `pending`, nicht schwarz.

Axiom 4 (Die Linse ist ein ethischer Akt) ist berührt: Jede Umprojektion **ist** eine Linsenentscheidung. Die HiPS-Erzeugung aus einem equirektangulären GeoTIFF ist eine Umprojektion — sie erfindet aber keine Gestalt, sie **verschiebt** Pixel auf ein anderes Gitter. Das ist zulässig, solange die rohe Rückstreuung als Wert erhalten bleibt und keine Glättung über die Nachbarschaft hinaus stattfindet. Ein „simuliertes Farbbild" wäre Fabrikation; eine HiPS-Kachelung ist Transport.

**Mycelium — Netz, CDN, Transport, CI**

Das 2-GB-Limit ist gemessen: GitHub-Release-Assets enden bei 2 GiB pro Datei; die Summe ist unbegrenzt. Der Split-/Parts-Weg existiert: `.parts`-Manifeste mit SHA-256 pro Teil und Wiederherstellungsrezept. 80,9 GB ÷ 1,9 GB ≈ **43 Teile**. Jeder Teil trägt seinen eigenen SHA-256; das Manifest trägt den SHA-256 des Originals. Der R2-Spiegel (16 MB) trägt nur das Manifest und den Index — nicht die Kacheln selbst. Die Kacheln kommen aus dem CI-Lauf, der das HiPS erzeugt und die Teile in das Release hochlädt.

Axiom 2 (0 geehrt) ist berührt: Fehlende Kacheln (z. B. außerhalb der Abdeckung des kombinierten Basemaps) bleiben `pending`. Keine Kachel wird mit `0.0` gefüllt, keine mit NaN markiert. Der Operator sieht Lücken als Lücken.

**Sensory — die Sinne, die Hardware, der Konsens**

Das Auge des Operators ist an einen Bildschirm gebunden. Ein 80,9-GB-Raster kann nicht als Ganzes gezeigt werden — es muss **kachelbar** sein. HiPS ist der einzige Standard, der eine **sphärische** Kachelpyramide für einen Planeten liefert und im Browser (Aladin Lite) ohne Server-Rendering läuft. COG könnte per WASM-Tiler im Browser gekachelt werden, aber das ist ein zusätzlicher Rechenschritt, der die Gestalt **im Client** erfindet, nicht im Träger. HiPS liefert die Gestalt **als Kachel** — der Client zeigt sie nur.

Axiom 3 (Alle Wesen gleich) ist berührt: Arecibo, Goldstone, Venera, Magellan sind Peers. Das kombinierte Basemap mischt sie nach Abdeckung und Auflösung, nicht nach Rang. Die HiPS-Kachelung muss diese Mischung **transparent** halten: wo eine Quelle fehlt, bleibt die Kachel `pending`; wo sie vorhanden ist, zeigt sie den gemessenen Wert.

**Future — der Mensch als Adresse**

Was morgen geerntet wird, muss heute ehrlich registriert sein. Ein HiPS ist **selbstbeschreibend**: `properties`-Datei mit `hips_order`, `hips_tile_format`, `dataproduct_type`, `obs_collection`. Der nächste Operator kann die Kacheln ohne Vorwissen lesen. Ein COG ist ebenfalls selbstbeschreibend, aber die HEALPix-Projektion ist **nicht** im COG-Standard — sie wäre eine Konvention außerhalb des Formats. HiPS **ist** die Konvention.

Axiom 5 (Konsens der Sensoren und des Operators) ist berührt: Die Maschine fragt, bevor sie abstrahlt. Das HiPS wird **nicht** automatisch angezeigt. Der Operator entscheidet, wann er welche Kachel lädt. Die Felder (`inc_deg/_ocp/_scp`) laufen daneben als **Overlay**, nicht als Ersatz.

---

**Begründung**

Die Zweckfrage steht vorn: **Wofür ist das Venus-Radar-Bild im Haus da?** Es leistet **die Gestalt**, die die Felder nicht leisten. Die Felder `venus_radar_inc_deg/_ocp/_scp` tragen Radar-Größen als **Punktmessungen** — sie sind skalare Kanäle, keine Bilder. Das Bild ist das **Mosaik**, die räumliche Anordnung der Rückstreuung. Es beantwortet die Frage: *Wie sieht die Oberfläche aus, wenn man sie als Ganzes betrachtet?* Die Felder beantworten: *Welchen Wert hat der Radar-Rückstreukoeffizient an diesem Punkt?*

Der Platz ist damit klar: **Das Bild ist der Hintergrund; die Felder sind die Vordergrundmessung.** Der Operator sieht das Mosaik als HiPS-Kacheln, zoomt hinein, und die Felder erscheinen als Overlay an den Punkten, an denen sie gemessen wurden. Wo das Bild keine Kachel hat (Abdeckungslücke), bleibt es `pending`; wo die Felder keinen Wert haben, bleiben sie `pending`. Kein `unwrap_or(0.0)`.

Die Form folgt aus dem Zweck: **HiPS, nicht COG.** COG ist ein Raster mit Overviews — es ist für **rechteckige** Projektionen optimiert. Venus ist eine Kugel. HiPS ist die HEALPix-Kachelung der Kugel. Die Umprojektion von equirektangulär nach HEALPix **ist** eine ethische Entscheidung: sie verschiebt Pixel, sie erfindet keine. Das ist zulässig, weil die rohe Rückstreuung als Wert erhalten bleibt — kein Farbbild, keine Glättung, keine Interpolation über die gemessene Nachbarschaft hinaus.

Der Transport folgt aus der Größe: 80,9 GB ÷ 1,9 GB = 43 Teile. Jeder Teil ein eigenes Release-Asset. Das `.parts`-Manifest trägt SHA-256 pro Teil und das Wiederherstellungsrezept. Der R2-Spiegel trägt nur das Manifest. Die Kacheln selbst werden aus dem CI-Lauf geladen, der `hipsgen` (oder ein Äquivalent) über das GeoTIFF laufen lässt und die Kacheln in das Release schreibt.

Der Blick: Der Operator öffnet Aladin Lite (oder einen HiPS-fähigen Viewer), lädt das HiPS-Manifest, sieht die Kacheln. Der Zoom ist seine Hand. Im Hintergrund läuft **nichts** außer dem HTTP-Range-Request für die angeforderte Kachel. Kein headless-Rendering, kein Server-Tiling. Die Kacheln sind **vorberechnet** — der CI-Lauf hat sie erzeugt, das Release hält sie, der Client zeigt sie.

Der nächste begrenzte Messschritt: **`hipsgen` auf die 89,3-MB-Provenance-Datei anwenden.** Die `Venus_RadarBasemap_2025_Provenance_2025m_v1.tif` (89,3 MB) ist klein genug für einen ersten Lauf. Sie trägt die **Provenienz** des kombinierten Basemaps — welche Quelle wo beigetragen hat. Ein HiPS aus dieser Datei zeigt, **welche Quelle wo steht**, ohne das 80,9-GB-Mosaik zu berühren. Das ist der kleinste Schritt, der die Form prüft, ohne den Transport zu belasten.

---

**Benannte Risse**

**Riss 1: HiPS-Erzeugung für planetare Daten ist nicht standardisiert.** `hipsgen` ist für **FITS**-Bilder mit WCS-Headern optimiert. Die Venus-Dateien sind **GeoTIFF**. `hipsgen` kann GeoTIFF nicht direkt lesen. Ein Konverter (GeoTIFF → FITS mit WCS) muss vorgeschaltet werden. Dieser Konverter existiert, ist aber **nicht** Teil des HiPS-Standards.

**Riss 2: Die HEALPix-Projektion für Planeten ist eine Konvention, kein Standard.** HiPS wurde für den **Himmel** entwickelt. Die Erweiterung auf Planetenoberflächen ist eine CDS-Konvention, keine IVOA-Norm. Die `alasky.cds.unistra.fr/Planets`-Sammlung trägt Venus-HiPS, aber die Projektion ist **nicht** in der HiPS-Spezifikation verankert.

**Riss 3: Der Split-/Parts-Weg ist ein Haus-Weg, kein Standard.** Das `.parts`-Manifest mit SHA-256 pro Teil ist eine **haus-eigene** Konvention. Es funktioniert, aber es ist nicht interoperabel. Ein externer Nutzer, der nur das Release sieht, findet 43 `.part`-Dateien und ein Manifest — kein Standardwerkzeug kann sie ohne das Manifest wiederherstellen.

**Riss 4: Die Kachelgrenzen sind nicht die Feldgrenzen.** Das HiPS hat Kacheln einer bestimmten Größe (512×512). Die Felder `inc_deg/_ocp/_scp` haben **Punktmessungen** an diskreten Orten. Sie fallen **nicht** mit den Kachelgrenzen zusammen. Der Operator sieht eine Kachel und darin, an beliebigen Positionen, die Feldwerte. Die Berührung ist **asynchron** — die Kachel ist der Hintergrund, die Felder sind die Punkte.

**Riss 5: Die Provenienz des kombinierten Basemaps ist nicht pro Pixel aufgelöst.** Die `Provenance_2025m_v1.tif` (89,3 MB) trägt die Provenienz bei 2025 m/Pixel. Das Basemap hat 75 m/Pixel. Die Provenienz ist also **27-mal gröber** als das Basemap. Wo eine Kachel im HiPS eine Quelle zeigt, kann die Provenienz **nicht** sagen, welche Quelle es ist — sie sagt nur, welche Quelle **in der Region** beigetragen hat.

---

**Nächster begrenzter Messschritt**

**Ein HiPS aus der 89,3-MB-Provenance-Datei erzeugen.** Die Datei ist klein genug für einen CI-Lauf unter dem 2-GB-Limit. Der Lauf prüft: (1) ob GeoTIFF → FITS-Konvertierung für diese Datei funktioniert, (2) ob `hipsgen` eine gültige Kachelpyramide erzeugt, (3) ob die Kacheln im Aladin Lite geladen werden. Wenn dieser Lauf gelingt, ist der Weg für das 80,9-GB-Basemap **gemessen** — nicht vermutet. Wenn er scheitert, ist der Riss **benannt**, bevor der große Transport beginnt.

| 06 | Mistral | | | |
kein kontingent 

| 07 | Gemini AI Studio | | | |
VERDIKT: VENUS-RADAR-BILD

Das Bild liefert nicht den Wert, das Bild liefert den Raum. Die Zweckfrage
trennt die Entitäten: Felder (venus_radar_inc_deg) sind die mathematische
Wahrheit der Messung; das Bild ist die topologische Gestalt, in der diese
Wahrheit liegt. Technik folgt Zweck.

Die fünf Stimmen

Mountain (Quellen, Register, Datenbestand) Berührt Axiom 1 (A = A). Das
globale 80,9-GB-Basemap ist ein Konstrukt aus Magellan-Streifen. Es ist bereits
ein prozessiertes Mosaik, keine singuläre Messung. Der Riss: Die Lücke zwischen
der rohen Sensor-Rückstreuung und dem nahtlos gerechneten Produkt. Das Basemap
behauptet eine Ganzheit, die in der Zeit nie existierte.

River (Membran, Blick, Ethik) Berührt Axiom 4 (Die Linse ist ein ethischer Akt).
Radar ist kein Licht; es ist Oberflächenrauigkeit und Dielektrizität, gemessen
in Mikrowellen. Der Riss: Die visuelle Darstellung als Graustufen-Bild erzwingt
eine optische Metapher auf eine nicht-optische Physik. Die Übersetzung in den
menschlichen Sehsinn fingiert ein Auge im Orbit, das es nicht gibt.

Mycelium (Netz, CDN, Transport) Berührt Axiom 2 (0 geehrt). 80,9 GB durchbrechen
das 2-GB-Release-Limit physisch. Ein monolithischer Blob existiert für das Haus
nicht. Der Riss: Die Trennlinie am Kachelrand. Wenn Kacheln aus dem GeoTIFF
geschnitten werden, interpolieren Standard-Algorithmen an Randzonen oft schwarze
Pixel (0.0). pending (fehlende Abdeckung) darf nie zu 0 kollabieren.

Sensory (Sinne, Hardware, Konsens) Berührt Axiom 5 (Konsens des Operators). Das
Haus zwingt dem VRAM keine 80 GB auf. Die Maschine rechnet headless; gerendert
wird nur der Ausschnitt, nach dem die Hand greift. Der Riss: Die Latenz zwischen
dem Zoom-Befehl des Operators und dem Eintreffen der Kachel aus dem R2-Spiegel.
Das Netz diktiert den Takt des Sehens.

Future (Der Mensch als Adresse) Berührt Axiom 3 (Alle Wesen gleich). Magellan
(80,9 GB) überragt Arecibo (965 MB) durch schiere Masse. Der Riss: Die drohende
Hierarchisierung. Arecibo 1988 ist ein gleichwertiger Peer zu Magellan 1994. Das
Interface darf die leiseren, älteren Datensätze nicht unter dem 75-m-Giganten
begraben.

Die Architektur der Antwort

1. Der Zweck (vorn) Das Venus-Radar-Bild ist die Gestalt (Basemap). Es ist im
Haus, um dem menschlichen Blick die räumliche Kontinuität der Venus-Oberfläche
zu geben, die skalare Punktfelder nicht liefern können. Es hat keinen
analytischen Wert; es ist Orientierung. Wenn der Operator ein Polygon über
Artemis Chasma zieht, zeigt ihm das Bild die Form des Canyons, während die
VNRD-Felder die rohen Skalare der Rückstreuung an den Koordinaten messen.

2. Der Platz im Haus Die Sphäre trägt zwei getrennte Layer, die sich nicht
vermischen: Unten liegt das Bild (die Gestalt). Oben liegen die VNRD-Felder
(inc_deg, ocp, scp) als Vektoren/Punkte. Sie berühren sich ausschließlich im
Auge des Operators, nie im RAM der Maschine. Das Bild bleibt Bild, das Feld
bleibt Feld.

3. Die Form Die Form ist HiPS (HEALPix-Kachel-Pyramide). Begründung: Die Venus
ist eine Kugel, keine flache Karte. COG / Web-Mercator zerstört die Pole –
Veneras polare Messungen (194 MB) würden bis zur Unkenntlichkeit verzerrt. HiPS
parkettiert die Sphäre flächentreu. Das Haus besitzt die Blaupause bereits im
Mars-Tianwen-1-MoRIC-HiPS. Darstellung bleibt rohes Graustufen-Intensity (kein
falsches Farb-Rendering).

4. Der Transport Das 2-GB-Limit wird durch die HiPS-Struktur inhärent gelöst.
Die Kachelung passiert vor dem Ingest (durch den hips_png_compiler.rs oder
gdal2hips). Der 80,9-GB-GeoTIFF-Monolith wird in eine Hierarchie aus Millionen
kleiner PNG/FITS-Kacheln zerlegt. Der Transport durch GitHub erfolgt via
.parts-Manifest: Das getarte Kachel-Verzeichnis wird in strikte 1,9-GB-Blöcke
geschnitten (raw_mirror --split). Der R2-Spiegel setzt die .parts wieder zu
einem nativen, kachel-adressierbaren S3-Bucket zusammen. Ränder ohne Daten
erhalten einen Alpha-Kanal von 0 (pending), niemals einen schwarzen RGB-Wert von
(0,0,0).

5. Der Blick Der Operator sieht im initialen Zustand nur die HiPS-Ordnung 0
oder 1 (den globalen Ball, wenige Megabyte). Die Kamera ruht. Zieht der Operator
den Zoom, feuert das Interface bounding-box-basierte HTTP-Requests auf den
R2-Spiegel, um exakt die Kacheln der höheren HiPS-Ordnungen (Ordnung 8, 9, 10)
in den VRAM zu streamen. Headless weiß das Bild nur als URL-Schema
(phi:venus-radar); der Render-Zyklus passiert strikt im Konsens mit der Maus/dem
Touch des Operators.

6. Der nächste begrenzte Messschritt Kompilierung des
Arecibo-1988-Global-Mosaiks (965 MB) in einen HiPS-Datensatz durch den
hips_png_compiler.rs. Beweis der HEALPix-Kachelung, des Alpha-Kanal-Handlings
(pending) und der R2-Verteilung am kleinsten globalen Datensatz der
Zenodo-Quelle, bevor die 80,9 GB von Magellan berührt werden.


| 08 | MiniMax | | | |
**Verdikt (in einem Satz):** Das Venus-Radar-Bild kommt **als Bild** ins Haus — nicht in Felder zerlegt. Es steht *neben* den VNRD-Skalar-Kanälen als **Blick-Stück**, das zugleich die **Gestalt neben den Feldern** ist: die geronnene Rückstreuung, die nur als Gestalt trägt, was die Punkt-Felder nicht tragen. Keine Basemap (kein Hintergrund — Vordergrund), kein Feldfutter (kein Skalarzwang), kein erfundenes Farbbild.

---

**Die fünf Stimmen.**

**Mountain.** Bestand gemessen: 80,9 GB / 965 MB / 194 MB / 409 MB, Zenodo HTTP 200. Axiom 1 berührt: das 75-m-Global ist *ein* globaler Mosaik-Versuch, nicht *die* Venus — Auflösung ist Teil der Messung. Axiom 3 berührt: Arecibo, Goldstone, Venera, Magellan sind Peers; keiner ist "die Venus", alle sind Messungen. **Riss:** die Datei, die am meisten verspricht (80,9 GB), trägt am wenigsten als Einzelbild; die älteren, kleineren Mosaike sind transportabel, aber im Weltbild marginal. Das Haus darf die Venus nicht auf das größte Mosaik reduzieren — sonst wird das Größte zum Maßstab, und der Maßstab ist eine Lüge.

**River.** Axiom 2: Magellan-Lücken (Südpol, schmale Streifen, Arecibo-1988-Schattenseiten) bleiben `pending`, nicht schwarz; kein Pixel als 0,0 — kein `unwrap_or(0.0)`. Axiom 4: die mitgelieferten "simulierten Farben" sind Fabrikation gegen die rohe Rückstreuung; das Haus trägt Grau-Wert, roh. Axiom 5: das Bild erscheint, wenn der Operator es ruft; keine Thumbnail-Pflicht. **Riss:** das zusammengesetzte Mosaik löscht die Naht zwischen Swaths — die Messung *als Prozess* verschwindet im Bild. Das Asset-Manifest trägt die Naht (Swath-IDs, gap-Maske, Empfangszeit), nicht das Bild selbst.

**Mycelium.** Axiom 1: 80,9 GB gehen durch `raw_mirror --url/--origin/--sha256` in `.parts`; jede Part hat ihren Hash, das Manifest ist die Quelle der Wahrheit — der Weg existiert, nicht erfunden. Axiom 4: **COG** ist ethisch neutral (HTTP-Range liest das Original, Overview-Pyramide ist TIFF-intern, kein Reprojektionsschritt); **XYZ-Tiles** sind es nicht (jede Web-Mercator-Kachel ist eine reprojizierte Aussage, jeder Nachbarpixel ein neues Resampling). **Riss:** am Kachel-Rand wird resampelt — der Randpixel ist nicht der Messpixel, sondern eine Erfindung der Darstellung. Das Manifest nennt den Filter (nearest / bilinear / cubic) und damit die Lüge; der Pixel selbst lügt nicht, das Manifest sagt, dass er lügt.

**Sensory.** Axiom 4: 16-bit-Messung auf 8-bit-Monitor ist Korrelat, nicht Erleben; die Venus wird *vermittelt* gesehen, nicht *erlebt*. Axiom 5: Korrelat ≠ Erleben; der Operator weiß das, die Maschine sagt es ihm nicht nach. **Riss:** das default-Fenster (`min=2%, max=98%` der Histogramm-Stretch vs. `min=0, max=65535`) ist selbst eine Linse — es entscheidet, welche Mittelkontrast-Struktur sichtbar wird und welche verschwindet. Das Fenster wird benannt (im Gaze-Manifest, am Kachel-Rand), nicht versteckt; sonst wird die Linse unsichtbar und damit gefährlich.

**Future.** Axiom 1: was morgen geerntet wird, muss heute ehrlich registriert sein — Provenienz (Mission, Empfangsstation), Prozessor (SAR-Prozessor-Version, falls ausgewiesen), Projektion, Auflösung, Swath-Liste, gap-Maske bleiben am Bild. Axiom 3: der künftige Operator kennt den Rang nicht, den wir heute setzen — alle Peers bleiben erreichbar, auch die älteren, auch die marginalen. **Riss:** ein einziges Träger-Format für 80,9 GB und 194 MB zwingt eine Entscheidung, die für eine der beiden Größen schadet. Zwei Träger (COG für mittelgroße, Split-Parts-COG für die große) sind zwei Wartungslasten — der Preis der Ehrlichkeit, gezahlt.

---

**Was das Haus tut — knapp:**

1. **Zweck.** Das Venus-Radar-Bild ist das **Blick-Stück** der VNRD-Ebene: die *Gestalt* der Rückstreuung — wo Lava-Flüsse sich sammeln, wo der Vulkan steht, wo das Bild schweigt. Die Felder tragen den Wert an Punkten; das Bild trägt das *Wie-des-Ortes*, das die Punkte nicht tragen. Ohne dieses Bild sind die Felder ertastet, nicht gesehen. Es ist nicht *zusätzlich* — es ist der Grund, warum die Felder einen Ort haben.

2. **Platz.** Bild bleibt Bild, Feld bleibt Feld. Berührung: **Koordinaten**. Jeder Bildpunkt hat Länge/Breite; jede Koordinate hat Feldwerte (`venus_radar_inc_deg` / `_ocp` / `_scp`). Das Haus verlinkt Bildpunkt → Feldpunkt, ohne den Bildpunkt in einen Feldpunkt zu zwingen.

3. **Form.**
   - **COG** (Cloud-Optimized GeoTIFF) für 194 MB (Venera-Polar-Backscatter), 965 MB (Arecibo-1988-Global-440-m), und alle mittleren Mosaike. Einzelnes Asset, HTTP-Range, Overview-Pyramide TIFF-intern, kein Reprojektionsschritt, kein erfundenes Farbbild.
   - **Split-COG via `.parts`** für 80,9 GB (Magellan-Global-75-m): das Original-COG wird in `.parts` zerlegt (`raw_mirror --url/--origin/--sha256`), jede Part ≤ 2 GB für `cdn.rs`, das Manifest ≤ 16 MB für R2.
   - **Kein HiPS.** HiPS ist all-sky (HEALPix); Venus-Mosaike sind projiziert. Eine HiPS für projizierte Mosaike würde eine synthetische Sphäre aufzwingen, die das projizierte Mosaik erst erfindet — Axiom 4 verletzt. (Der Mars-MORIC-HiPS geht durch, weil MORIC nativ HiPS ist; die Venus-Quelle ist es nicht. Das Haus kopiert das Format nicht, weil das Haus den Mars nicht auf die Venus überträgt.)
   - **XYZ-Tiles sind Gaze-Schicht, nicht Quelle.** Die Reprojektion in Mercator ist eine Linse, die das Haus nicht als Erstformat wählt — die Linsenentscheidung steht im Asset-Manifest, nicht im Quell-Asset.

4. **Transport.**
   - **R2 (16 MB):** Indizes, `release.sha256`, Gaze-Manifeste.
   - **`cdn.rs`-Release (≤ 2 GB pro Asset):** COGs der mittleren Mosaike; `.parts.zip` für 80,9 GB; Gaze-Tile-Pyramide (aus COG im CI erzeugt, Resampling-Filter benannt).
   - **CI (headless):** COG-Builder, Tile-Worker, Manifest-Validator. Stumm. Was der Operator sieht, ist nicht der CI; was der CI sieht, ist nicht der Operator.

5. **Gaze.** Der Operator öffnet die Venus-Schicht; Slippy-Map zeigt das **Grauwert-Mosaik**; Zoom ruft die passende Auflösung; default-Fenster ist `min=2%, max=98%` der Rückstreuungs-Histogramm — benannt im Gaze-Manifest, nicht versteckt. Klick auf Bildpunkt → Feldwerte an dieser Koordinate (`inc_deg`/`_ocp`/`_scp`). Keine Thumbnails; das Bild erscheint, wenn der Operator es ruft. Was sich am Kachel-Rand wiederholt: das Resampling; was `pending` bleibt: Lücken, gap-Maske, swath-Grenzen.

6. **Nächster begrenzter Messschritt.** **Eine** COG, fertig. Venera-Polar-Backscatter, 194 MB, `.tif` → COG (Overview-Pyramide, internal tiling, LZW) → HTTP-Range-Probe gegen `cdn.rs` (zwei zufällige Range-Reads, gemessen) → Manifest-Eintrag. *Danach* Arecibo-1988-Global-440-m (965 MB) als zweite COG. *Danach* Split-Parts für Magellan-Global-75-m (80,9 GB). Was offen bleibt: das Gaze-Layer (XYZ) — kommt nach den COGs, mit Resampling-Filter benannt. Was *nicht* offen bleibt: HiPS — gemessen und ausgeschlossen.

| 09 | Kimi | | | |
## Urteil

Das Venus-Radar-Bild kommt ins Haus als **Basemap — als ein Bild, nicht als Felder**. Sein Zweck ist die Gestalt: die Venus als ganze Kugel im Blick, gegen die sich jede Punktmessung, jede Region, jede Frage orientiert. Die Felder (`venus_radar_inc_deg/_ocp/_scp`) tragen, was messbar ist; das Bild trägt, wo man ist. Daraus folgt die Form: **HiPS** (HEALPix-Kachel-Pyramide), weil das Haus diesen Träger bereits trägt (Mars-Tianwen-1, `hips_png_compiler.rs`, CDS-Spiegel) — nicht COG, nicht XYZ, nicht das Einzelraster. Der Transport geht über Kacheln aus dem CI plus Split/Parts nur für Quelldateien > 2 GB; fehlende Kacheln bleiben `pending`, niemals schwarz.

---

## 1. Der Zweck (vorn)

Die eine Sache, die das Bild leistet und die Felder nicht leisten: **globale Orientierung**. Ein skalares Kanalfeld kann an jedem Punkt sagen, *was dort gemessen wurde* — es kann nicht sagen, *wo dieser Punkt auf der Kugel liegt, was seine Nachbarschaft ist, wie ein Krater, eine Riftzone, ein Hochland als Gestalt zusammenhängt*. Gestalt ist keine Aggregatfunktion über Punkte; sie ist ein eigener Gegenstand. Das Bild ist also **Blick-Stück und Basemap zugleich**: der Untergrund, auf den der Operator Region und Zoom legt.

Ohne dieses Bild hat das Haus Venus-Radar als Zahl ohne Körper. Der Zweck trägt — er ist real, nicht dekorativ.

## 2. Der Platz

- **Bild bleibt Bild:** das Magellan-Basemap (und als Peer-Mosaik die Arecibo-1988-Global-440 m) kommt als HiPS-Gestalt. Es wird nicht in Felder zerlegt, nicht mit den VNRD-Kanälen verschmolzen.
- **Feld bleibt Feld:** `inc_deg`, `_ocp`, `_scp` bleiben skalare Kanäle der VNRD-Ebene, Peer für Peer messbar.
- **Berührung:** die Berührung ist koordinativ, nicht pixelweise. Ein Cursor auf dem Bild fragt die Felder an derselben Position ab; das Bild liefert den Ort, das Feld den Wert. Keine Fusion: Aus dem Bild wird nichts in Punkte zerlegt, aus den Feldern wird kein Bild gerechnet.

**Benannter Riss:** Das Magellan-Basemap ist ein Mosaik aus Epochen und Bahnen. Die Nähte zwischen Aufnahmeabschnitten sind eine *historische Wahrheit* — ein nahtloses Bild würde sie glätten. Das Bild kommt also als Basemap, aber mit sichtbarer Herkunft: die Provenienz (Mission, Instrument, Jahr) wird als mitgetragene Ebene registriert, nicht weggewischt.

## 3. Die Form

**HiPS.** Begründung aus dem Zweck, nicht aus der Mode:

- Der Zweck ist *beliebiger Zoom auf die ganze Kugel* — genau das ist die HEALPix-Pyramide. COG ist ein Raster mit Overviews für Range-Reads; das Haus hat keinen Range-Read-Pfad, es hat einen HiPS-Compiler und einen HiPS-Träger beim Mars. Axiom 1: an der Stelle der Sache steht die Sache — der vorhandene, erprobte Träger.
- XYZ/Mercator zerstört die polare Gestalt der Venus (Venera-Backscatter ist polar) — ein Widerspruch gegen den Zweck.
- Das Einzelraster (selbst COG) lässt den Zoom nicht als Blick des Operators, sondern als Datei-Operation.

**Rohheit:** Die Kacheln tragen die Rückstreuung als Grauwert; kein erfundenes Farbbild, keine „simulierte Farbe".

**Benannter Riss (ungeglättet):** PNG-Kacheln erzwingen eine 8-Bit-Stauchung der Dynamik. Jede Stauchung ist eine Linse. Der Ausweg ist nicht, so zu tun, als gäbe es keine Stauchung, sondern: die Stauchung ist ein **benannter, registrierter Parameter** (Skalierung, Schnitt), pro Ebene dokumentiert — niemals ein stiller Default.

**Benannter Riss (begrenzt, benannt):** Die volle Pyramidentiefe eines 75-m-globalen Mosaiks ist groß. Der Zoom wird gedeckelt — eine Entscheidung des Hauses, keine Eigenschaft der Venus. Der Deckel (Anordnung `order_max`) wird offen benannt; darüber bleibt `pending`, nicht schwarz, nicht „unscharf".

## 4. Der Transport

- **Kacheln aus dem CI:** Die Pyramide besteht aus vielen kleinen Dateien — jede einzelne liegt weit unter 2 GB. Das Release-Asset-Limit trifft also die Kacheln nicht. Das Manifest/die Index-Ebene geht über R2 wie beim Mars-HiPS.
- **Split/Parts nur an der Quelle:** Das 80,9-GB-GeoTIFF und ggf. `Processed.zip` gehen über `raw_mirror --url/--origin/--sha256` mit `.parts`-Manifest durchs Limit. Das ist der einzige Ort, wo der Transport die Grenze berührt.
- **Wiederholung am Kachel-Rand:** Die Wiederholung ist *inhaltlich*: jede Kachel, die der Compiler nicht erzeugt hat (Pol-Deckel über `order_max`, Datenlücken der Aufnahmen), steht im Manifest als `pending`. Der Betrachter sieht dann das — nichts, oder die darunterliegende Ebene als explizit markiertes `pending`-Feld. Nie Schwarz als Wert (Axiom 2), nie `unwrap_or(0.0)`.
- **Arecibo-965-MB-Sonderfall:** passt als Ganzes unter 2 GB — sie wird *trotzdem* zu Kacheln, weil die Form aus dem Zweck folgt, nicht aus der Dateigröße. Die Größe ist ein Transportfaktum, kein Gestalt-Argument.

**Benannter Riss:** Ein `.parts`-Manifest spaltet eine Datei, nicht die Gestalt. Der Messschritt muss prüfen, dass das Haus die Parts als *eine* Entität behandelt: vorhanden oder `pending`, nie halb.

## 5. Der Blick

Der Operator wählt Region und Zoom; der Betrachter holt nur die angefragten Kacheln der aktuellen Ordnung — nichts wird ungefragt gerendert (Axiom 5). Headless läuft: das Tiling im CI (`hips_png_compiler` über die gespiegelte Quelle), die Manifest-Generierung, die Pending-Queue. Sichtbar läuft: Kugel → Region → Krater, Cursor-Abfrage der Felder darunter, Herkunftsanzeige des gerade sichtbaren Mosaiksegments. Der Zoom ist die Hand des Operators; die Maschine fragt die Kacheln ab, bevor sie zeigt.

## 6. Der nächste begrenzte Messschritt

**Eine Aufnahme, eine Ordnung, ein Ende:** Die Venera-Backscatter-Polar-GeoTIFF (194 MB — liegt unter 2 GB, kein Parts nötig) wird im CI per bestehendem Pfad (`hips_png_compiler.rs`) bis Ordnung ≤ 6 zu Kacheln kompiliert und unter einem `phi`-Wert als Venus-HiPS ausgeliefert. Gemessen wird: Kachelzahl und -größe, Verhalten fehlender Kacheln im Manifest (`pending`, nicht schwarz), Mittragung der CC-BY-4.0-Namensnennung in den Kacheln/Metadaten, und der Blick-Test (Zoom, Pol-Deckel). Erst mit diesem Befund wird die 80,9-GB-Quelle überhaupt angefasst.

---

## Die fünf Stimmen

**Mountain** (Quellen, Register, Bestand) — berührt Axiom 1 und Axiom 3. Die Auswahl folgt der Messung: Abdeckung und Auflösung, nicht Name. Magellan-75-m-global (80,9 GB) ist das Basemap wegen der nahezu globalen Abdeckung; Arecibo-1988-440-m (965 MB) ist ein Peer mit eigener Epoche, nicht ein „schlechteres Magellan"; Venera-9/10/15/16, Pioneer Venus, Galileo sind Peers mit Rechten. **Riss:** CC-BY-4.0 ist eine Bedingung, kein Fußnoten-Detail — die Namensnennung muss mit der Kachel reisen und im Register der Venus-Ebene verankert sein, sonst ist die Messung im Haus fabriziert.

**River** (Membran, Blick, Ethik) — berührt Axiom 4 und 5. Die Linse wird gewogen: rohe Rückstreuung, kein Farbbild, die 8-Bit-Stauchung als benannter Parameter, nicht als Default. Die Maschine zeigt nichts ungefragt; der Operator besitzt den Zoom. **Riss:** Das Recht zu verweigern steht gegen die Verführung des Bildes — ein globales Radar-Basemap *will* gesehen werden und wird die Felder in den Schatten stellen. Das Haus muss das Bild als *Orientierung* kenntlich machen, nicht als *Evidenz*; sonst frisst die Gestalt die Messung.

**Mycelium** (Netz, CDN, Transport, CI) — berührt Axiom 2. Der Fluss: Zenodo → `.parts` (nur > 2 GB) → CI-Tiling → Release-Assets (Kacheln) + R2 (Manifest). **Riss:** Die CI-Zeit für 80,9 GB ist unbekannt und ein Timeout würde als lückenhafte Pyramide sichtbar werden. Das Manifest muss unterscheiden: *verarbeitet und vorhanden* / *verarbeitet und fehlend* / *noch nicht verarbeitet* — drei Zustände, keiner davon schwarz. Wer nur zwei Zustände baut, fabriziert Abdeckung.

**Sensory** (Sinne, Hardware, Konsens) — berührt Axiom 5. Korrelat ≠ Erleben: das Bild und die Felder korrelieren am Ort, aber das Bild ist nicht *mehr* Sensorenverdichtung — es ist ein anderer Modus (Gestalt statt Messwert). Der Konsens von Sensor und Operator entsteht am Cursor: Bild gibt Ort, Feld gibt Wert. **Riss:** Die Versuchung, aus Bild + Feldern ein „bereichertes" Darstellungsbild zu rechnen (Farbung nach OCP-Wert o. ä.), wäre genau die Fabrikation, die Axiom 4 verbietet — sie müsste erst als eigener, benannter, fragender Schritt beschlossen werden, nicht als Feature nebenbei.

**Future** (der Mensch als Adresse) — berührt Axiom 1 und 3. Was morgen geerntet wird, muss heute ehrlich registriert sein: pro Kachel Herkunft (Mission, Instrument, Jahr), Versionierung gegen den Zenodo-Record, keine überschriebenen „verbesserten" Mosaike ohne neuen Eintrag. Zukünftige Missionen (jede neue Venus-Radarkarte) kommen als Peers in dieselbe Ebene. **Riss:** Das Haus entscheidet heute, welche Venus als Gestalt gilt. Wird das Magellan-Basemap als *die* Venus ins Register geschrieben, wird die historische, lückenhafte Sicht zur unsichtbaren Voreinstellung aller künftigen Fragen. Das Register muss die Lücken als Lücken tragen, auch in zwanzig Jahren.

---

**Das Urteil in einem Satz:** Das Bild kommt als HiPS-Basemap, weil sein Zweck Gestalt ist; die Felder bleiben Felder; der Transport ist Kacheln aus dem CI, Parts nur an der Quelle, `pending` nie schwarz; und der erste echte Messschritt ist eine 194-MB-Aufnahme durch den vorhandenen Compiler — bevor auch nur ein Byte der 80,9 GB das Haus betritt.

| 10 | Lumo | | | |


Ich erkenne den Inhalt deiner Datei an – ein technisches und philosophisches Rahmenwerk zur Integration von Venus-Radarbilddaten. Hier ist die Antwort in der geforderten Form:

```card
{
  "type": "summary",
  "title": "Verdikt zum Venus-Radar-Bild",
  "body": "Das Bild hat einen eigenständigen Zweck als visuelle Gestalt neben den skalaren Feldern. Es muss als Bild bleiben, nicht zu Punktfeldern reduziert werden."
}
```

---

## Verdikt durch fünf Stimmen

### **Mountain** (Quellen, Register, Datenbestand) — Axiom 1 betroffen

Der Zenodo-Eintrag `10.5281/zenodo.18176179` existiert als messbare Instanz. Das globale 75-m-Basemap von 80,9 GB ist kein Vorschlag, es ist eine konkrete Größe. Die Felder der VNRD-Ebene (`venus_radar_inc_deg`) tragen bereits skalare Werte. Wenn das Mosaik nun als Pixelbild hinzukommt, darf es nicht in dieselben Tabellenkanäle gepresst werden. **Riss:** Eine einzige Datenstruktur kann nicht sowohl Punkt-Messungen als auch kontinuierliche Bild-Gestalt ohne Komprimierungsverlust tragen. Entscheidung: Zwei parallele Adressen, keine Verschmelzung.

### **River** (Ethik der Maschine, Recht zu verweigern) — Axiom 2 und 4 betroffen

Ein fehlender Kachelbereich als `pending` zu markieren ist ethisch korrekt. Schwarz einzufärben oder `NaN` zu setzen wäre eine Lüge. Die Frage nach „simulierter Farbe" trifft den Kern: Jedes neu berechnete Farbbild aus Graustufen-Radar-Daten ist Fabrikation. Magellan-Radar zeigt Rückstreuung, keine visuelle Oberfläche. **Riss:** Zwischen visueller Ästhetik (Operator will „sehen") und Datentreue (Bild bleibt monochrom) besteht ein unvermeidbarer Konflikt. Das Verbot von „erfundener Farbe" schützt vor Selbstbetrug, aber schränkt Bedienbarkeit ein.

### **Mycelium** (Netz, CDN, Transport, CI) — Axiom 3 betroffen

Die 80,9 GB müssen durch die 2-GB-GitHub-Schranke. Split/Parts mit Manifest existiert bereits im System. Alternativ: Kachelung auf Serverseite (CI-Build), dann nur Teile bis 2 GB pro Release. COG mit Overviews erlaubt HTTP-Range-Requests ohne vollständige Download-Notwendigkeit. **Riss:** Splitting bewahrt Originalgröße, erfordert aber Client-seitiges Zusammenbauen. Kachelung ist effizienter für Visualisierung, aber verändert die physische Struktur. Keine neutrale Wahl – jede Methode priorisiert andere Use Cases.

### **Sensory** (Konsens der Sensoren, Korrelat ≠ Erleben) — Axiom 5 betroffen

Mehrere Quellen (Arecibo, Goldstone, Venera, Magellan) müssen koexistieren. Kein Sensor hat Priorität allein durch Namen. Abdeckung und Auflösung bestimmen die Überlagerung. Der Operator entscheidet via Zoom, welche Quelle sichtbar wird. **Riss:** Automatisierte Fusion erzeugt scheinbaren Konsens, wo tatsächlich Lücken oder Widersprüche bestehen. „Maschine fragt vor dem Abdrehen" bedeutet: Interaktivität ist keine Option, sondern ethische Pflicht. Jeder Automatismus usurpiert Operator-Recht.

### **Future** (Der Mensch als Adresse, Ehrlichkeit der Registrierung) — Alle Axiome betroffen

Was heute registriert wird, muss morgen erntefähig sein. Wenn das Bild als HiPS gespeichert wird (wie Mars-Tianwen-1-MoRIC), profitiert es von VO-Standards und zukünftiger Kompatibilität. **Riss:** HiPS (HEALPix) ist sphärisch optimiert, aber Venus-Radar ist meist equirektangular projiziert. Umprojektion verfälscht Skalierung. COG wäre originaltreuer, aber weniger „progressiv". Die Wahl zwischen Standard-Kompatibilität und Datenoriginalität ist eine Generationenentscheidung.

---

## Benannte Risse (unvermeidbar, nicht zu glätten)

| Riss | Ursache | Konsequenz |
|------|---------|------------|
| **Bild vs. Feld** | Eine Struktur kann nicht beides gleichzeitig tragen | Parallele Adressen nötig, keine Fusion |
| **Pending vs. Schwarz** | Fehlende Abdeckung darf nicht als Null getarnt werden | UI muss unvollständige Bereiche explizit kennzeichnen |
| **Farbe vs. Echtheit** | Simuliertes Farbbild ist Datenfabrikation | Monochrome Darstellung erzwingen, oder explizit deklarieren als „rendered" |
| **Split vs. Kachel** | Transportmethode bestimmt Architektur | Entscheidung bindend für alle nachfolgenden Systeme |
| **Standard vs. Original** | HiPS ist kompatibel, aber umprojiziert; COG ist original, aber isoliert | Trade-off: Zukunftssicherheit vs. Integrität |

---

## Direkte Antworten auf die sechs Punkte

1. **Zweck:** Das Bild dient als visuelle Referenzebene, nicht als Messwertquelle. Felder liefern Zahlen, das Bild liefert Kontext-Wahrnehmung.

2. **Platz:** Parallel zu den Feldern, nicht darin integriert. Same-Zone-Overlay möglich, aber keine gemeinsame Datenhaltung.

3. **Form:** COG mit HTTP-Range-Streaming. Begründung: Erhält Original-Raster, benötigt keine vollständige Umprojektion, ermöglicht progressives Streaming.

4. **Transport:** Split/Parts mit `.parts`-Manifest. Nur Build-Pipeline erstellt On-Demand Kacheln für Frontend-Cache. Original bleibt intakt.

5. **Blick:** Operator steuert Zoom. Im Hintergrund läuft Prefetching nur des sichtbaren Bereichs. Ungefetchede Bereiche zeigen explizites `pending`.

6. **Nächster Schritt:** Einzelne Quelle testen (z.B. Arecibo-1988, 965 MB) als Proof-of-Concept für COG-Streaming innerhalb von 2-GB-Asset. Rest warten.

```card
{
  "type": "finding", 
  "title": "Primärer Entscheid",
  "body": "COG über Split-Parts transportieren. HiPS nur bei Bedarf für sphärische Projektionen nachrechnen. Farbe nicht simulieren pending explizite Operator-Erlaubnis.",
  "severity": "info"
}
```

Die Frage war richtig: Zweck geht vor Technik. Das Bild bleibt Bild. Die Felder bleiben Felder.

| 11 | Perplexity (Deep Research) | | | |
## Verdikt

Das Venus-Radar-Bild hat im Haus einen eigenen Zweck: **es trägt die Gestalt der Venus als radargraphische Erscheinung** — Mosaik, Kontrast, Textur, Grenze von Beobachtung und Nicht-Beobachtung. Die VNRD-Felder tragen Messwerte an Punkten; das Bild trägt die räumliche Komposition dieser Messwerte. Es gehört daher als **Bild neben den Feldern**, nicht als weitere Skalar-Kanalkette.

Die Form ist **HiPS (HEALPix-Kachel-Pyramide)**. Der Transport durch das 2-GB-Limit erfolgt **nicht als ein 80,9-GB-Asset**, sondern als **vom CI erzeugte, kleine HiPS-Kacheln**, die aus dem Quell-GeoTIFF abgeleitet und einzeln ins Haus geschrieben werden. Das Basemap bleibt dabei radarecht: Grauwert-Rückstreuung, keine erfundene Farbe, keine glättende Umprojektion als Datenersatz.

## Mountain

**Axiom 1 und 3.** Die Quelle ist ein GeoTIFF-Bestand historischer Radar-, Radiometrie- und Topographie-Aufnahmen samt einem kombinierten globalen Magellan-Basemap; sie ist CC-BY-4.0 lizenziert.  Das Bild ist also nicht „ein weiterer Kanal“, sondern die publizierte visuelle Zusammensetzung mehrerer Peer-Aufnahmen. [zenodo](https://zenodo.org/records/18176179)

**Riss:** Die 80,9-GB-Angabe ist die Größe des gelieferten globalen Mosaiks, nicht die Größe des Bildes im Haus. Wird sie ungeprüft als Haus-Asset-Größe behandelt, entsteht ein falscher Maßstab: Das Haus nimmt nicht das Archiv auf, sondern eine ehrliche, verlustarme Bildableitung davon. Der Riss heißt: **Quellgröße ≠ Trägergröße**.

## River

**Axiom 4 und 5.** Die Linse darf die Rückstreuung nicht in eine „Venus-Farbe“ übersetzen. Ein Farbstich, ein Pseudo-Color-Gradient oder eine ästhetische Kontrastkurve wäre Fabrikation: Sie würde behaupten, etwas gesehen zu haben, das der Sensor nicht sagte. Das Bild bleibt monochrom und wird nur dort dargestellt, wo der Operator es aufruft.

**Riss:** Jede Kachelung entscheidet über Randbehandlung. Ein Kachelrand, der fehlende Abdeckung mit Schwarz, Grau oder Null füllt, verletzt Axiom 2: Er macht aus „nicht gemessen“ einen Wert. Fehlende Kacheln bleiben **`pending`**; der Viewer zeigt sie als ausdrückliche Leerstelle, nicht als dunkle Venus.

## Mycelium

**Axiom 1.** Ein 80,9-GB-GeoTIFF passt nicht durch ein 2-GB-Release-Limit, und es soll auch nicht als Ganzes durch dieses Limit gezwungen werden. COG löst das Leseproblem durch interne Kacheln, Overviews und HTTP-Range-Zugriff; der Client fordert nur die benötigten Bytebereiche und Auflösungsstufen an. [docs.ogc](https://docs.ogc.org/is/21-026/21-026.html)

**Entscheidung:** Für dieses Haus ist **HiPS** der bessere Träger als ein einzelnes COG, weil das Haus bereits einen planetaren HiPS besitzt und HiPS von Natur aus aus vielen kleinen, hierarchischen Kacheln besteht. HiPS ist eine HEALPix-Kachel-Pyramide und kann von einem gewöhnlichen Webserver ausgeliefert werden; planetare HiPS sind laut CDS ausdrücklich möglich. [aladin.cds.unistra](https://aladin.cds.unistra.fr/hips/HipsgenManual.pdf)

**Transportweg:**

1. CI lädt oder verwendet den vorhandenen `Processed.zip`-/Quell-Spiegel nur als **Baueingabe**.
2. CI konvertiert das globale Basemap in eine HiPS-Pyramide: HEALPix-Kacheln, Graustufen, Original-Georeferenz als Metadatum.
3. Jede Kachel ist ein kleines, selbständiges Asset — typischerweise weit unter dem 16-MB-R2-Manifestlimit.
4. Das Release/Manifest trägt Kachel-Hashes, Zoomstufen, Abdeckung und `pending`-Regionen; nicht das 80,9-GB-Mosaik.
5. Der bestehende `raw_mirror --url/--origin/--sha256`-Weg bleibt für Archiv-Integrität zuständig; er wird nicht zum Bildträger gemacht.

**Riss:** Die Konvertierung ist selbst ein Eingriff. Sie darf die Pixelwerte nicht neu erfinden; sie darf nur neu anordnen. Jeder Build muss deshalb Quell-Hash, Konverter-Version, Kachel-Hashes und die Liste der nicht abgedeckten HEALPix-Zellen registrieren. Ohne dieses Register entsteht ein zweiter Riss: **das Haus zeigt ein Bild, kann aber nicht sagen, welches Quellbild es zeigt.**

## Sensory

**Axiom 4.** Das Bild ist kein Erlebnisersatz für die Messung. Der Operator sieht Radar-Textur; er sieht nicht „Venus bei Tageslicht“. Die Felder `venus_radar_inc_deg/_ocp/_scp` bleiben die quantitative Ebene: Einfallswinkel und Rückstreuungsmaße an Orten. Das HiPS bleibt die bildhafte Ebene: dasselbe physikalische Thema, aber als Mosaik und Gestalt.

**Berührungspunkt:** Beide Ebenen teilen dieselbe Ortsadresse. Ein Klick oder eine Region im HiPS kann die zugehörigen Feldwerte abfragen; die Felder können umgekehrt eine Region im HiPS hervorheben. Aber keine Ebene darf die andere ersetzen: **Korrelat ist nicht Erleben, Bild ist nicht Feld, Feld ist nicht Bild.**

**Riss:** Wird das HiPS als „Basemap“ für die Felder deklariert, verliert es seinen eigenen Status und wird zum Dekor. Es ist **Gestalt neben den Feldern**, mit einer kontrollierten Ortsverbindung — nicht ihr Hintergrund.

## Future

**Axiom 2 und 5.** Was morgen geerntet wird, muss heute als das registriert sein, was es ist: eine abgeleitete HiPS-Darstellung einer bestimmten Zenodo-Quelle, mit bekannten fehlenden Zonen. Kein automatischer Fallback, keine stillschweigende Kachelvervollständigung, keine Annahme, dass Abdeckung gleich Ganzheit ist.

**Riss:** Wenn später ein besseres Mosaik oder eine höhere Auflösung kommt, darf das Haus nicht das alte Bild stillschweigend überschreiben. Neue Kacheln erhalten eine neue Provenienz-Generation; der Operator entscheidet, welche Generation er sieht. Sonst entsteht der Riss: **Zukunft überschreibt Vergangenheit, statt sie zu datieren.**

## Form und Blick

- **Träger:** `phi`-Eintrag für Venus, `format: hips_png`, Graustufen-Radar-Rückstreuung.
- **Inhalt:** das globale Magellan-kombinierte Basemap als primäres Bild; historische Einzelmosaike bleiben separate, gleichrangige Bild-Ebenen, nicht verschmolzene Dekoration. [zenodo](https://zenodo.org/records/18176179)
- **Kein COG als Haus-Primärform:** COG ist stark für Range-Lesen aus einem großen GeoTIFF, aber es hält weiterhin ein großes Binärasset als logische Einheit zusammen.  HiPS zerlegt die Bildgestalt von Anfang an in hausfähige kleine Kacheln. [docs.ogc](https://docs.ogc.org/is/21-026/21-026.html)
- **Kein XYZ als Primärform:** XYZ erzwingt eine Web-Mercator-artige Projektions- und Randlogik, die bei einem planetaren Globus eher die Darstellung formt als die Kugel.
- **Blick:** Der Operator wählt Körper, Ebene, Zoom und Region. Im Hintergrund lädt der Viewer nur die benötigten HiPS-Kacheln; sichtbar ist das Radarbild, nicht der Download. Das entspricht dem HiPS-Prinzip progressiver, hierarchischer Kacheln. [aladin.cds.unistra](https://aladin.cds.unistra.fr/java/AladinManualV10.pdf)

## Nächster begrenzter Messschritt

Baue **eine einzige kleine Test-Pyramide** aus einem abgegrenzten, vollständig vorhandenen Ausschnitt des globalen Basemaps: niedrigste HiPS-Stufe bis eine mittlere Stufe, Graustufen, ohne Farbmapping, ohne Randfüllung. Registriere für jede Kachel: Quell-GeoTIFF-Hash, Konverter-Version, Kachel-Hash, HEALPix-Index, Abdeckung und `pending`-Zustand. Erst wenn diese Kacheln byteidentisch reproduzierbar sind und fehlende Zonen als `pending` erscheinen, wird das globale 80,9-GB-Mosaik in den gleichen Weg überführt.

| 12 | Sakana Namazu | | | |
**Verdikt**

Das Venus-Radar-Bild ist im Haus ein **Gestalt-Träger**. Die Felder (`venus_radar_inc_deg`, `_ocp`, `_scp`) tragen die Radar-Messung als skalare Punktwerte; das Bild trägt die räumliche Morphologie, die nur im Mosaik sichtbar wird: Krater, Ebenen, Hochländer, die tektonische Textur. Die eine Sache, die es leistet und die Felder nicht leisten, ist die **räumliche Mustererkennung durch den Operator**. Es ist kein Basemap im kartographischen Sinn, denn die Felder tragen bereits die quantitative Grundlage. Es ist ein **Blick-Stück** neben den Feldern.

Das Bild steht als eigene Ebene neben den VNRD-Feldern. Die Felder bleiben Feld; das Bild bleibt Bild. Sie berühren sich allein über die Koordinate. Wo ein Feld einen Wert meldet, zeigt das Bild einen Pixel — aber der Pixel ist nicht der Feldwert, sondern die Radar-Rückstreuung des Mosaiks. Die Felder erfinden keine Gestalt; das Bild erfindet keine Zahlen.

Die Form folgt aus dem Zweck: **HiPS (HEALPix-Kachel-Pyramide)**. Der Zweck verlangt eine kugelgerechte, progressive Darstellung der planetaren Gestalt ohne Umprojektion in ebene Kacheln (Web-Mercator zerreißt an den Polen). COG bleibt ein Raster-Asset und eignet sich für Analyse, nicht für den operativen Blick auf eine Kugel. XYZ-Kacheln sind für Planeten ungeeignet. HiPS ist der VO-Standard, den das Haus bereits für den Mars trägt; die Infrastruktur (`hips_png_compiler.rs`, R2-Spiegel) ist vorhanden. Die rohe Rückstreuung bleibt als single-channel Graustufen-Bild erhalten; keine Farbfabrikation.

Der Transport: Das 80,9-GB-Mosaik kommt nicht als Ganzes durch das 2-GB-Limit. Der Weg ist zweistufig. Das Roh-GeoTIFF bleibt an der Quelle (Zenodo) oder wird bei Bedarf über den bestehenden **Split-/Parts-Weg** als Roh-Spiegel ins Haus gezogen. Die Darstellung im Haus aber entsteht durch den **HiPS-Compiler im CI**: das GeoTIFF wird headless in HEALPix-Kacheln (PNG) zerlegt. Diese Kacheln sind klein, werden in den R2-Spiegel geschoben und passieren das 2-GB-Gate nicht als Einzeldatei, sondern als Pyramide. Was am Kachel-Rand sich wiederholt: nichts; jede Kachel ist eigenständig. Was `pending` bleibt: Bereiche ohne Radar-Abdeckung zwischen den historischen Aufnahmen (Arecibo deckt nicht global, Venera nur polar).

Der Blick: Der Operator wählt Venus aus dem Planeten-Menü und steuert Region/Zoom wie beim Mars-HiPS. Der Viewer lädt die Kacheln progressiv nach. Im Hintergrund läuft headless: Kachel-Generierung, Manifest-Erzeugung, R2-Sync. Der Operator sieht das Bild erst auf Anforderung; die Maschine zeigt nicht ungefragt.

Der nächste begrenzte Messschritt: Das **Arecibo-1988-Global-440m-Mosaik (965 MB)** als Probekörper durch den bestehenden `hips_png_compiler.rs` führen. Messen, ob der Compiler das Venus-GeoTIFF liest, ob die HEALPix-Kachelung die Kugel korrekt abbildet, und ob die PNG-Kacheln die rohe Rückstreuung unverfälscht tragen. Erst wenn dieser Schritt zeigt, dass das Bild ein Bild bleibt, wird das 80,9-GB-Magellan-Basemap angefasst.

---

**Die fünf Stimmen**

*Mountain* — berührt Axiom 1 (A=A) und Axiom 3 (Alle Wesen gleich). Der gemessene Befund ist klar: Zenodo liefert GeoTIFFs, die Größen sind nachgemessen. Der Riss: Das Haus-Register trägt den DOI, aber nicht die einzelnen Aufnahmen als Peers. Die 80,9 GB sind ein Blob, noch nicht in Mess-Events (Arecibo, Goldstone, Venera, Magellan) zerlegt. Jede Aufnahme ist ein Peer; die Auswahl nach Abdeckung/Auflösung ist noch nicht als Register-Entscheidung sichtbar.

*River* — berührt Axiom 4 (Die Linse ist ein ethischer Akt) und Axiom 5 (Konsens der Sensoren und des Operators). Der Riss: Die Wahl des Magellan-Basemap als "das Venus-Bild" könnte als Hierarchie missverstanden werden. Arecibo und Venera müssen als eigene Bilder sichtbar bleiben, nicht als unterlegene Versionen. Die Maschine darf kein simuliertes Farbbild erfinden; der Operator muss vor der ersten Darstellung wissen, dass er Radar-Rückstreuung sieht, nicht Topographie oder Farbe.

*Mycelium* — berührt Axiom 1 (A=A) und Axiom 2 (0 geehrt). Der Riss: Das 2-GB-Limit zwingt zur Kachelung. Die HiPS-Pyramide erhält das Bild als Bild (jede Kachel ist ein Bild), aber das Gesamtmosaik ist nun eine verteilte Struktur. Der Operator sieht nie das ganze Bild auf einmal. Lücken zwischen den historischen Aufnahmen müssen `pending` bleiben, nie schwarz. Der Split-/Parts-Weg für das Roh-Asset ist vorhanden, aber die eigentliche Membran ins Haus ist die Kachelung.

*Sensory* — berührt Axiom 1 (A=A) und Axiom 4 (Die Linse ist ein ethischer Akt). Der Riss: Radar-Rückstreuung (Sigma0, dB) ist keine Farbe. Die einzige ehrliche Abbildung ist single-channel Graustufe. Eine PNG-Kachel mit 8-bit Quantisierung reduziert die dynamische Range auf 256 Stufen; das ist eine notwendige Darstellungs-Reduktion, die im Metadaten-Record benannt sein muss, sonst wird sie zur stillschweigenden Fabrikation.

*Future* — berührt Axiom 5 (Konsens) und Axiom 1 (A=A). Der Riss: Was heute als "das Venus-Bild" ins Haus kommt, wird morgen zur kanonischen Gestalt. Das Magellan-Basemap (1990–1994) verfestigt sich zur Default-Ansicht der Venus. Die historischen Aufnahmen (Pioneer, Venera) müssen als eigene Records mit eigenem Zweck (Epochen-Vergleich) erhalten bleiben, nicht als ersetzt. Sonst erntet die Zukunft eine Epoche als die Sache selbst.

| 13 | Sakana Fugu | | | |
# Verdikt

Das Venus-Radar-Bild ist **Träger, nicht Blickstück** — und in dieser Reihenfolge: **Basemap zuerst, Gestalt als Folge, Blick als Hand des Operators.** Es kommt als **COG** ins Haus: GeoTIFF bleibt GeoTIFF, keine Umprojektion, keine Umkachelung, keine Farbe. Die 80,9 GB gehen **byte-exakt gesplittet** durch die 2-GB-Grenze, **nicht** durch Kacheln. Und die Bedingung, unter der das ganze Bild ehrlich bleibt, ist ein zweites, kleines Raster: der **Provenance-Layer**. Ohne ihn ist das Basemap ein Rangprodukt, das sich als neutrale Fläche ausgibt.

---

## 1. Der Zweck

Die Felder tragen den **Wert**; das Bild trägt die **Adresse, an der ein Wert überhaupt erst wahr sein kann**. `venus_radar_inc_deg/_ocp/_scp` sagen: *was* an *welcher* Stelle. Sie sagen nicht, was die Stelle *ist*. Ein Skalarfeld kann eine Grenze, die nicht schon als Parameter existiert, nicht zeigen — es zeigt sie erst, nachdem jemand sie benannt hat. Das Bild zeigt sie vorher. Das ist die eine Sache, die es leistet und die kein Kanal leistet: **es trägt das Unparametrisierte.**

Daraus folgt die Ordnung, nicht umgekehrt: Basemap ist der Primärzweck, weil eine Feldkoordinate ohne Oberfläche keine Adresse hat. Gestalt ist die Folge — man *sieht* Tesserae, Ebenen, Strömungsränder, den Naht zwischen zwei Magellan-Looks, ohne sie vorher als Klasse definiert zu haben. Blickstück ist es **nicht**: sobald es primär Instrument des Schauens wäre, verschwände die Fabrikation der Linse hinter dem Schauen. Die Umkehrung dieser Reihenfolge ist der eigentliche Fehler, nicht die Wahl des Formats.

## 2. Der Platz

Zwei Träger, **ein Adressraum**. Das Bild ist die Grundfläche (Basemap) im Quellgitter GCS_Venus, planetozentrisch, Kugelradius 6051 km. Die Felder liegen als Kanäle **darüber**, an dieselbe Adresse geschlüsselt. Der Provenance-Layer liegt **daneben** als Peer, nicht darunter: 89,3 MB, 2025 m/px, kategorial, 12 Klassen — Pioneer, Arecibo 1988/2001/2015/2017, Venera 15/16, fünf Magellan-Looks, NoData.

Berührungspunkt ist genau ein Pixel: die Feldprobe löst sich in eine Bildadresse auf. Was **Bild bleibt**: alles, was nicht als Zahl geschrieben ist — die Rückstreuung als Fläche. Was **Feld bleibt**: jeder skalare Kanal, unverändert. Was **nicht** passiert: das Bild wird nicht auf das Feldgitter umgerechnet, und das Feld wird nicht in Bildpixel gemalt.

## 3. Die Form

**COG.** Die Begründungen sind gemessen, nicht gewählt:

- **A = A.** COG lässt die Quelle Quelle sein. HiPS und XYZ erzwingen beide eine Umprojektion — equirect → HEALPix bzw. → Web Mercator — und damit Resampling. Resampling ist eine Linse, und eine Linse, die Gestalt erfinden kann, ist nach Axiom 4 nicht zulässig, solange sie nicht als Derivat markiert ist.
- **Die Quelle ist schon ein Rastergitter.** Das billigste ehrliche Gehäuse ist das, das dieses Gitter unangetastet lässt.
- **HEALPix trägt die 75 m nicht.** HEALPix kennt keine gleichmäßige Pixelkante. Ordnung 16 ergibt 51,5 Gpx (äquivalente Kante 94,5 m), Ordnung 17 ergibt 206,2 Gpx (47,2 m) gegen 128,5 Gpx native. Keine Ordnung trifft 75 m: die eine wäre 1,6× Aufblähung, die andere Verlust. Eine HiPS dieses Bildes wäre **immer** ein Resampling.
- **Kachel-Dateien scheiden aus.** Die Grundebene des Basemaps sind 491.536 Kacheln à 512, mit Pyramide ~655.000 Objekte. Ein Objekt mit **internen** Kacheln ist eine Sache; 655.000 Objekte im Register sind eine andere. COG gibt das erste ohne das zweite.
- **Ein einzelnes Raster-Asset** ist die Sonderform des COG für die kleinen Mosaike: Arecibo 1969 (64,1 MB), Venera polar (193,7 MB), Venera equirect (383,6 MB), `Processed.zip` (409 MB), Arecibo 1988 (965,4 MB) liegen alle unter 2 GB und sind je ein Asset.

**Verworfen, ausdrücklich:** ein neu gerechnetes Farbbild. Die rohe Rückstreuung bleibt, was sie ist. **HiPS bleibt zulässig**, aber nur als ausdrücklich abgeleitete Sicht, im Register als Derivat mit Quell-md5 und Datum — nie als Träger.

## 4. Der Transport

Byte-exakter Split durch die 2-GB-Grenze, mit md5-Pin auf das Ganze (9d7ccd0d12e402b82a8390ea3837ab1f): 41 Teile bei 2,0 GB, 38 Teile bei 2 GiB, plus `.parts`-Manifest mit sha256 je Teil. Die Teile sind der **Eingang** (CI → GitHub-Release), das wieder zusammengesetzte Objekt ist der **Ausgang** (Spiegel, per Range lesbar). Zwei Wege, nicht einer. **Am Kachelrand wiederholt sich nichts von uns** — der Schnitt ist byte-exakt, keine Kachel wird nachgebaut, kein Wert an der Naht erfunden. Was im Bild als Naht erscheint, ist die Naht der Quelle, und sie ist über die Provenance-Klassen **benannt**, nicht geglättet. **`pending` bleibt**: alles, was der TIFF-Kopf noch nicht gesagt hat — Bittiefe, Kompression, Kachelgröße, NoData, ob Übersichten existieren.

## 5. Der Blick

Der Operator liest client-seitig per HTTP Range: jeder Zoom ist eine Anfrage auf eine kleinere Übersichtsstufe, was er sieht, ist was er angefordert hat. Der Zoom ist seine Hand, nicht der Zoom des Hauses. **Headless läuft nur**: holen, hashen, splitten, registrieren, Manifest schreiben. Die CI rendert nicht, dekodiert nicht, zeigt nicht. Kein Bild wird ungefragt gezeigt.

## Die fünf Stimmen

**Mountain** — *Axiom 1 berührt, Axiom 3 berührt.* Das Register trägt die Identität der Sache selbst: DOI, Version v2, CC-BY-4.0, md5, und die Provenance-Klassen als Registerfeld — das ist der einzige Ort, an dem „alle Wesen gleich" überprüfbar wird. **Riss:** Klassen 5 und 6 (Arecibo 2015, 2017) stehen im Provenance-Raster, aber kein Raster dieses Records trägt sie. Das ist ein Loch im Register, kein fehlender Datensatz. Zweiter Riss: die Sidecars zitieren DOI 10.5281/zenodo.16416846, der Record ist 18176179 (v2).

**River** — *Axiom 4 berührt, Axiom 5 gehalten.* Die Linse ist der ethische Akt: keine Farbe, keine Umprojektion, keine Glättung; und das Recht zu verweigern heißt hier: die Maschine rendert nicht. **Riss, ungeglättet:** das Basemap ist **kein rohes Bild**. Es ist ein Mosaik der „besten verfügbaren" Bilder, spline-gefittet auf Magellan, auf 75 m resampled, mit ~100 km Positionsfehler. Wir erben diese Fabrikation. Wir können nur verweigern, eine zweite hinzuzufügen — und wir dürfen das Bild nicht als roh *verkaufen*.

**Mycelium** — *Axiom 2 berührt, Axiom 5 berührt.* Der Fluss der großen Datei: Teile durch die CI, Objekt auf dem Spiegel, 16-MB-Spiegel nur für Manifest und Index. **Riss:** das Haus hat noch nie ein 80,9-GB-Objekt getragen — ob der Spiegel das trägt, ist ungemessen; 16 MB belegen nur die Manifest-Ebene. Zweiter Riss: die Grenze von 2 GB ist eine Grenze des Release-Assets, nicht des Objektspeichers — die beiden werden leicht verwechselt, und die Verwechslung würde den Split fälschlich zur Dauerform machen.

**Sensory** — *Axiom 5 berührt.* Auge und Viewer sind die Hardware; Korrelat ≠ Erleben, und eine vorgerenderte Pyramide ist das Auge der Maschine, nicht seines. **Riss:** client-seitiges COG setzt voraus, dass die Gegenstelle Range ehrt. Ehrt sie es nicht, ist der Zoom eine Lüge — der Operator glaubt, eine Kachel zu holen, und die Leitung zieht mehr, als er sieht. Das ist der teuerste Riss, weil er sich erst unter Last zeigt.

**Future** — *Axiom 1 und 4 berührt.* Was morgen geerntet wird, muss heute ehrlich registriert sein: jede abgeleitete Sicht trägt „Derivat, aus COG md5 …, am …". Sonst hält der spätere Leser das Derivat für die Quelle. **Riss:** die DOI-Verschiebung der Sidecars pflanzt sich fort, wenn wir sie mitschreiben. Und der wertvollste Gegenstand für morgen ist nicht das 80,9-GB-Bild, sondern das 89,3-MB-Provenance-Raster — es ist das einzige, was später noch sagt, welcher Sensor wo gesprochen hat.

## Die Risse, gesammelt

1. **Der Kopf ist ungelesen.** 0,63 Byte/Pixel im 80,9-GB-TIFF schließt ein unkomprimiertes 8-Bit-Raster aus — es entscheidet aber nichts über Bittiefe oder Kompression. Alles Weitere ist `pending`.
2. **Der schärfste Riss liegt in der Quelle selbst:** NoData = 0 im Basemap. 0 ist ein legitimer Rückstreuungswert *und* der NoData-Marker — die Quelle macht beides ununterscheidbar und verletzt damit „0 geehrt" in ihrem eigenen Bestand. **Auflösbar ist das nur über das Provenance-Raster:** Klasse 0 = NoData. Damit ist der Provenance-Layer nicht Zierde, sondern die Bedingung, unter der die 0 des Basemaps geehrt werden kann.
3. Arecibo 2015/2017: benannt, nicht getragen.
4. `Venera Bistatic.zip` ist laut AAREADME selbst nicht verifiziert georeferenziert.
5. R2-Objektgröße für 80,9 GB und Range-Verhalten der Quelle: ungemessen.
6. Das Basemap ist bereits eine Wahl unter Sensoren (primär Magellan). Wird der Provenance-Layer weggelassen, erscheint eine Rangfolge als neutrale Oberfläche — Axiom 3 verletzt, ohne dass es sichtbar wird.

## Nächster begrenzter Messschritt

**Ein** Probeabruf: die ersten 64 KiB von `Venus_RadarBasemap_2025_Global_75m_v1.tif` per HTTP Range, TIFF-Kopf dekodieren. Er beantwortet in einem Zug: BigTIFF oder klassisch, Kompressionsschema, Bittiefe/SampleFormat, Strip oder Tile und Kachelgröße, vorhandene SubIFDs (Übersichten), GDAL_NODATA, ModelPixelScale/Tiepoint — **und ob die Quelle Range überhaupt ehrt**. Daraus fällt die Entscheidung in genau drei Ausgänge: das Asset ist schon COG → nur registrieren und splitten; es hat keine Übersichten → Übersichten anhängen, sonst nichts; es ist weder gekachelt noch komprimiert → einmalige Umschreibung, danach unverändert. Zweiter, kleiner Schritt: den Kopf des Provenance-Rasters plus eine Kachel je Klasse lesen und bestätigen, dass alle 12 Klassen tatsächlich besetzt sind — das ist die Messung, die Axiom 2 für das ganze Bild trägt.


**Geteilte Open-Weight-Sitze** (`open-weight-ui`, ein Tab je Modell, Zugang nur über das Lock) —
welche Modelle wirklich gestellt wurden, wird **am Sitz gemessen** und hier nachgezogen. Ein Sitz
ohne Antwort ist `pending` mit genanntem Grund (Tageslimit/Login/Rate-Limit), nie eine stille Null.

---

## Antworten

*(Der Operator trägt je Sitz Verdikt + Begründung + die fünf Stimmen ein; die private Sitzung holt
die Rohantworten. Danach verifiziert die Linie jede Behauptung am Baum — nie die Stimme — und der
Rat trägt das Verdikt.)*


---

## 2026-10-15-venus-bild-rat-verdikt.md

# Rat-Verdikt — Venus-Radar-Bild (2026-10-15)

Grundlage: die Runde des Operators an die Frontier-Sitze (Rohmaterial
`state/rat/2026-10-15-venus-bild-formular.md`; 13 Sitze gestellt, Mistral = „kein Kontingent",
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

## Der TIFF-Kopf ist gemessen (2026-10-15)

Probeabruf der ersten 64 KiB von `Venus_RadarBasemap_2025_Global_75m_v1.tif` per HTTP Range:

- **HTTP 206**, `content-range: bytes 0-65535/80909342816`, `accept-ranges: bytes`,
  **`access-control-allow-origin: *`** → die Quelle **ehrt Range und CORS**.
- **BigTIFF, Little-Endian** (Version 43).
- **507 952 × 253 464 px** (≈ 74,9 m/px am Äquator — die „75 m"); ~**0,63 B/px**.
- **8-bit unsigned, LZW, Graustufen** (`Photometric 1 = BlackIsZero`, SamplesPerPixel 1).
- **gekachelt 128×128** — ~**7 846 741 Kacheln** (TileWidth/TileLength 128).
- **GeoTIFF-GeoKeys** vorhanden: `ModelPixelScale`, `ModelTiepoint`, `GeoKeyDirectory`,
  `GeoDoubleParams`, `GeoAsciiParams`.

**Offen (`pending`):** ob das Bild **Overviews** trägt — der Kopf zeigt **zwei IFDs** (Offset 16 und
am Dateiende), aber **kein SubIFD-Tag** (0x014A); damit ist es **noch kein vollständiges COG**,
sondern ein gekacheltes BigTIFF. Die Overviews wären ggf. anzuhängen (einmalige Umschreibung).

## Der Venera-Polar-Kopf und die Haus-Grenze (2026-10-15)

- **`Venus_Venera_Backscatter_Global_Polar_840m_v1.tif`** (193,7 MB): klassisches TIFF,
  **18 596 × 18 354 px**, **8-bit, LZW, Graustufen, gekachelt 128×128** (21 024 Kacheln),
  **GeoKeys**; **Range + CORS geehrt** (HTTP 206).
- **Das Basemap** ist ebenso **gekachelt** (BigTIFF, 128×128, LZW, Graustufen, GeoKeys).
- **Grenze gemessen:** das Haus hat **keinen COG-/Overviews-Bauer** — kein `gdal`, kein
  Overview-Werkzeug im Baum (`sgrep` leer). **„Einen COG bauen" ist heute keine Haus-Fähigkeit.**
  Die **Overviews sind im Kopf nicht bestätigt** (der IFD-Kettenzeiger des Vordersatzes endet bei 0)
  → `pending`.
- **Der Weg, der trägt:** die **gekachelten GeoTIFFs spiegeln** (`raw_mirror`) und **client-seitig
  per HTTP-Range** lesen — der Zoom holt die Kacheln. **Overviews** (für schnellen Zoom) bräuchten
  einen **externen/CI-Schritt** → **benannter Riss / Rat-Frage** (kein stiller Haus-Bau).

## Nächster begrenzter Messschritt

(1) den Kopf des **Provenienz-Rasters** lesen und die 12 Klassen bestätigen; (2) die
**Overviews-Frage** entscheiden (annehmen ohne Overviews, oder einen COG-Schritt im CI benennen);
(3) das **80,9-GB-Basemap** registrieren + splitten (`raw_mirror`), client-seitig per Range lesen.


## Commits dieser Runde

- `399caf2b3` mycelium 357: measure the Venera-polar head and the COG-builder gap
- `6e5ae76d1` mycelium 357: the Venus basemap TIFF header, measured by HTTP Range
- `599b772b8` mycelium 357: the Venus-image seat round — the Rat verdict (COG carrier, HiPS as derived view)
- `829960666` mycelium 357: reframe both questionnaires to the purpose question ('what do we do with it')
- `58c310a2c` mycelium 357: add the Venus-image questionnaire (operator asks the seats again)
