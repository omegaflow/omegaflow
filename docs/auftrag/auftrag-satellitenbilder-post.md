<!--
  title: Rechercheauftrag — offenes Satellitenbild nach dem 26.08.2026 (Flut-/Narben-Footprint)
  class: auftrag
  date: 2026-08-28
  status: pending
  see-also: docs/paper/blatt-pfeil-sturzflut-tibet.md docs/auftrag/auftrag-seen-kollabgebiet.md docs/auftrag/auftrag-cog-quelle.md
-->

# Rechercheauftrag: ein freies Satellitenbild nach der Trishuli-Flut

## Anlass (gemessener Stand)

Der Kollaps (26.08.2026 02:52Z, USGS `landslide` M5.2, 28.271/85.515) und der
Kausalpfeil Kollaps→Flut sind gemessen. **Was fehlt:** ein **offenes,
wolkenfreies, nach dem 26.08. aufgenommenes Satellitenbild** über dem
Kollabgebiet, das die **Flutfläche / Kollapsnarbe** misst. Bisher geprüft und
nicht ausreichend:

- Sentinel-2 08-27 = **78 % Wolken** (verdeckt); kein späterer S2-Pass archiviert.
- MODIS/VIIRS = No-Data im Steilterrain; Sentinel-3 = zu grob.
- Sentinel-1 (SAR): nächster Overpass ≈ **08-30** (PC/CDSE), Vor-Baseline 08-24 liegt.
- Landsat 8/9: nächster Pass ≈ Anfang September, noch keine Granule.
- Hochauflösend (Planet/Maxar): von Reuters/ICIMOD genutzt, **bisher als
  kommerziell angenommen — nicht auf freien Download geprüft.**

## Zweck

Finde **mindestens eine** frei herunterladbare, ausreichend klare
Nach-Ereignis-Aufnahme über 28.27/85.515 (Zielfenster ≥ 08-26), aus der der
räumliche Footprint (Flutfläche, Kollapsnarbe, evtl. Staubsee) messbar wird.

## Kernregel

Jede Quelle mit **HTTP-Code + Auflösung + Bewölkung + Download-URL** belegen.
Nicht frei herunterladbar = kein Beleg. Nicht messbar = nicht behaupten.

## Kandidaten (in dieser Reihenfolge prüfen)

### Weg A — Post-Disaster-Open-Data der kommerziellen Anbieter (übersehen!)
1. **Maxar Open Data Program** (Open Data Program nach Katastrophen):
   `https://www.maxar.com/open-data` — sucht nach „Nepal flood August 2026";
   stellt WorldView-/GeoEye-Szenen als GeoTIFF **frei** nach großen Ereignissen.
2. **Planet** — Disaster-Imagery: `https://www.planet.com/disaster-data/` und
   der Reuters-Beleg („Planet Labs PBC/Handout") — prüfen, ob die Szenen
   offen bereitstehen (auch über `https://www.planet.com/explorer/` oder
   öffentliche s3/API-Links).
3. **International Charter Space & Major Disasters** (falls aktiviert):
   `https://disasterscharter.org/` — Rapid-Response-Bilder, teils frei geteilt.

### Weg B — nochmal offene EO, breiter + wiederholt
4. **ASF (Alaska Satellite Facility)** für Sentinel-1 (alle S1-GRD):
   `https://search.asf.alaska.edu/` via `EARTHDATA_EDL_TOKEN` — ggf. früherer
   Post-Pass als PC/CDSE; und Sentinel-1-Interferometrie (InSAR) für die Narbe.
5. **Sentinel-2-Spätpässe** (CDSE/PC) **täglich neu prüfen**: S2-Revisit
   ~5 Tage → ~08-29/30, falls wolkenärmer. Auch **Sentinel-2-Pfad benachbart**
   (Bewölkung variiert).
6. **NASA GIBS / Worldview** — andere Layer (True Color 250 m, Thermal):
   für den Fall, dass ein späterer klarer Tag erscheint.
7. **Copernicus EMS / CEMS-EMSR927** erneut prüfen (Delineation-Produkt), sobald
   die Aktivierung Produkte ausgibt (bisher gated).

### Weg C — dokumentiert/sekundär (nur zitierbar, nicht downloadbar)
8. Reuters/AP/ICIMOD-Bildbelege (Planet, Reuters-Drohne) — als **Referenz,
   kein offener Datensatz**; Koordinaten/Zuschreibung notieren.

## Lieferung

- **Gefundene Quelle:** Name, Satellit, Sensor/Auflösung, Aufnahmedatum/-zeit,
  Bewölkung, Download-URL, HTTP-Code, Lizenz (frei ja/nein).
- **Wenn gefunden:** Footprint-Messung (Flutfläche/Narbe) mit dem verfügbaren
  Tool oder manuell über Pixelzählung, im Blatt/auftrag-seen-kollabgebiet.
- **Wenn keine frei:** benennen, was genau fehlt und wann es kommt.

## Abschluss

Erst wenn eine **frei herunterladbare, klare Nach-Ereignis-Aufnahme** belegt
ist, geht der räumliche Footprint von `pending` in eine **Messung** über.
Bis dahin bleibt er nicht messbar — nicht aus Übersehen, sondern aus
fehlender freier Datenlage.

---

## Befunde

_(Stand 2026-08-28 — Recherche-Agent + eigene Nachmessung)_

### Frei herunterladbare Nach-Ereignis-Aufnahme GEFUNDEN: Landsat 9

**`LC09_L2SP_141040_20260826_02_T1`** (WRS-2 path/row 141/040, UTM 32645)
— Aufnahme **26.08.2026 04:47 UTC** (~2 h nach Kollaps), **30 m**, Szene
**47,5 % Bewölkung**. Über **Planetary Computer** (`landsat-c2-l2` SAS-Token,
**kein Login**) als echter 16-bit-TIFF abrufbar, **HTTP 200 verifiziert**.
Bänder: `green`(B3), `nir08`(B5) heruntergeladen (je ~100 MB), `--cog-ndwi`:

| Punkt | wasser_pixel | NDWI mean/max | Befund |
|---|---|---|---|
| Kollabpunkt 28.271/85.515 | **0 / 7209** | −0.019 / 0.139 | **kein offenes Wasser** am 26.08. |
| Bhote-Koshi-Tal 28.28/85.38 | 0 / 7209 | −0.221 / 0.014 | kein sauberes Wassersignal (Tal < 30 m) |
| Lende-Khola-Box 28.50/85.57 | 12 / 19980 | −0.011 / 0.376 | vereinzelte Wasser-Signatur |

**Einordnung:** Der Kollabpunkt ist auch am **Flut-Tag (26.08.) optisch
wasserfrei** — konsistent mit der S2- und der negativen MODIS-Messung.
Einschränkung (ehrlich): 47,5 % Szenen-Bewölkung; die genaue Wolkenfreiheit
des Kollab-Pixels konnte über die QA-Maske (ungewöhnliche Bandstruktur) nicht
abschließend verifiziert werden — das stark negative NDWI-Minimum (−0.588,
dunkles Terrain) spricht gegen einen geschlossenen Wolkenblock, ist aber
kein harter Beleg für wolkenfrei.

### Weitere Kandidaten (Agent-Befund)
- **Landsat 9 Nachbarzeile** (141/041, 67,8 % Wolken): frei, aber stärker bewölkt.
- **Sentinel-2 08-27** (CDSE): 78,5 % Wolken (verdeckt).
- **EU-Space S2-Bild 08-27** (4960×3507): zeigt nur das **Tal Betrawati/Gerkhu**
  stromabwärts, **nicht** den Kollabpunkt.
- **Maxar/Planet/Charter:** Maxar-Seite 200, aber **keine Nepal-Aktivierung
  belegt**; Planet-Disaster-Data **404**; Charter **keine Nepal-Aktivierung**
  gefunden → kommerziell, kein offener Download.
- **Sentinel-1 (SAR):** kein Post-GRD über dem Kollabpunkt ingestiert (letzter
  Pass 24.08.); nächste Pässe ~28.–31.08., noch nicht da. ASF durch
  Session-Auth nicht verifizierbar.
- **CEMS EMSR927:** Aktivierung bestätigt (200), Produkte (Delineation) am
  28.08. **noch nicht** öffentlich (üblich 24–72 h).

### Fazit
Es existiert **mindestens eine frei herunterladbare Nach-Ereignis-Aufnahme
über dem Kollabpunkt: Landsat 9 (26.08., 30 m, 47,5 % Wolken)** — gemessen
wasserfrei am Kollabpunkt. Für die robuste **Flut-/Narbenflächen-Messung**
bleibt die **Sentinel-1-SAR-Szene (~28.–31.08.)** der stärkste Kandidat;
CEMS-EMSR927-Delineation in den nächsten Tagen kostenlos erwartbar.
