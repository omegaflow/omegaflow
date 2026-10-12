<!--
  title: Survey — Standort-Lokalisierung (Orthofoto · DSM/DOM · Street-Level)
  class: survey
  date: 2026-10-12
  sha256: a1e2208751d082709440ab68b0db2ae0fa9b4bbec05e07877d9ccffff008252f
  status: live
  see-also: docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md state/river/themis/calibration-2026-10-12.md
-->
# Survey — Standort-Lokalisierung (Orthofoto · DSM/DOM · Street-Level)

**Frage.** Wie lokalisiert man ein **Objekt auf dem Gelände** (Instrument, Dom, Mast, Gebäude)
mit **offenen** Quellen — generalisiert über beliebige sources, nicht nur THEMIS?

**Provenienz.** Erhoben 2026-10-12 aus der THEMIS-ASI-Dom-Aufgabe (AUGO I, Athabasca; 2–3 m Objekt),
generalisiert von zwei Recherche-Tauchern. HTTP-Werte via `archive_search --verdict` gemessen
(Leiter direkt → Proton → Wayback); `206 (1 Byte)` = Range-Probe-Artefakt = erreichbar, nicht Inhalt.
`--verdict` misst **Reichweite**, nicht Lizenz/Format/Auflösung — Auflösung/Lizenz sind Programm-Angabe
oder `pending`.

## Die generalisierte Methode (drei Stufen)

1. **Struktur (DSM − DTM).** `höhe(x,y) = dsm(x,y) − dtm(x,y)` aus **einem** Produkt, **einem** Datum,
   **einer** Epoche. Ohne **beide** Raster (dsm *und* dtm) keine Objekthöhe, kein Befund → `pending`.
   Ein Orthofoto misst **keine** Höhe; ein DSM **trägt die Struktur**, die das Auge im Bild nicht auflöst.
2. **Identifikation (Orthofoto).** Grundriss/Kontext; das Objekt muss in **mehrere Pixel** fallen
   (2–3 m @ 0,33 m/px ≈ 6–9 px; @ 10–20 cm ≈ 10–30 px). Schein durch Schatten/Parallaxe.
3. **Bestätigung (Street-Level / Site-Plan).** Mapillary/KartaView/Panoramax (offen), Google SV,
   oder nationaler Site-Plan.

**Der gemessene Beleg (THEMIS-ASI-Dom):** HRDEM `dsm − dtm` (1 m native EPSG:3979) lokalisierte ein
isoliertes ~1,6–1,8 m-Objekt; die Orthofotos (ESRI 33 cm / Bing 17 cm / Google soft) lösten es **nicht** auf.

## A — DSM/DOM (Oberflächenmodelle) · Bedingung: **DSM + DTM** aus demselben Produkt

| Land/Scope | Programm | Endpunkt | HTTP | Auflösung | DSM/DTM | Lizenz | Zugang |
|---|---|---|---|---|---|---|---|
| CA | **NRCan HRDEM** (Mosaik) | `datacube.services.geo.ca/wrapper/ogc/elevation-hrdem-mosaic` | 200 | 1 m | **DSM + DTM** | OGL-Canada | **keyless WCS/WMS** |
| Global | **Copernicus DEM** | `copernicus-dem-30m.s3.amazonaws.com/` | 200 | 30/90 m | **nur DSM** | Copernicus | keyless S3 |
| Global | AWS Terrain Tiles (Mapzen/SRTM) | `s3.amazonaws.com/elevation-tiles-prod/skadi/…` | 206 | ~30 m | DEM | SRTM PD u. a. | keyless |
| US | USGS **3DEP Elevation** | `elevation.nationalmap.gov/arcgis/rest/services/3DEPElevation/ImageServer` | 200 | 1 m | DEM (liDAR) | PD | keyless REST |
| Global | **OpenTopography** | `portal.opentopography.org/` · `/API/otCatalog` | 200 | LAZ/DEM | je Datensatz | Terms | Portal frei; GlobalDEM **keyed** (401) |
| Global | ALOS **AW3D30** / DLR **TanDEM-X 90 m** | `eorc.jaxa.jp/…` · `download.geoservice.dlr.de/TDM90/` | 206 | 30/90 m | nur DSM | Registration | blocked account |
| DE | BKG **DGM** | `sgx.geodatenzentrum.de/wms_dgm200` | 200 | 200 m | DEM | dl-de/by-2-0 | keyless |

## B — Orthofotos (DOM/Orthoimagery)

| Land/Scope | Programm | Endpunkt | HTTP | Auflösung | Lizenz | Zugang |
|---|---|---|---|---|---|---|
| US | USGS **NAIP** | `imagery.nationalmap.gov/arcgis/rest/services/USGSNAIPImagery/ImageServer` | 200 | 0,6–1 m | PD | keyless REST |
| FR | IGN **BD ORTHO** | `data.geopf.fr/wms-r/wms` | 200 | 20 cm | Etalab | keyless WMS |
| CH | **SWISSIMAGE** | `wms.geo.admin.ch/` | 200 | 10 cm | swisstopo free | keyless WMS |
| NL | PDOK Luchtfoto RGB | `service.pdok.nl/hwh/luchtfotorgb/wms/v1_0` | 206 | 7,5/25 cm | CC0 | keyless WMS |
| ES | PNOA-Ortho | `www.ign.es/wms-inspire/pnoa-ma` | 206 | 25/50 cm | CC-BY 4.0 | keyless |
| NZ | LINZ Basemaps | `basemaps.linz.govt.nz/` | 206 | 0,3–0,5 m | CC-BY 4.0 | keyless |
| Global | **OpenAerialMap** (Drohnen) | `map.openaerialmap.org/` · `api.openaerialmap.org/meta` | 206 | cm | meist CC-BY | Web/API |
| Global | ESRI World Imagery (Tile) | `server.arcgisonline.com/…/World_Imagery/MapServer` | 200 | ~30–60 cm @z19 | ESRI-ToS | keyless |
| Global | Bing Aerial (Tile) | `ecn.t0.tiles.virtualearth.net/tiles/…?g=1` | 206 | ~30 cm @z19 | Bing-ToS | keyless |
| Global | Google Satellite (Tile) | `mt1.google.com/vt/lyrs=s` | 200 | soft ~15–30 cm @z21 | Google-ToS | keyless |

## C — Street-Level / Oblique

| Scope | Programm | Endpunkt | HTTP | Lizenz | Zugang |
|---|---|---|---|---|---|
| Global | **Mapillary** | `graph.mapillary.com/` | 200 (Token-Root 500) | CC-BY-SA | **Token keyed** |
| Global | **KartaView** | `kartaview.org/api/` | 206 | CC-BY-SA | keyless |
| Global | **Panoramax** (offen) | `api.panoramax.xyz/api/` | 200 | je Bild offen | **keyless** |
| Global | Google Street View (Static) | `maps.googleapis.com/maps/api/streetview` | 400 ohne Key | Google-ToS | **Key key-needed** |
| Global | Bing Streetside | `bing.com/maps/streetside` | 206 | Bing-ToS | kein öffentl. API |

## Risse / Fallen (gemessen)

- **`--verdict` misst Reichweite, nicht Inhalt** — `206 (1 Byte)` ist Probe-Artefakt.
- **Datum-Versatz 3979 ↔ 4326** (HRDEM nativ 3979): Reprojektion verschiebt/rundet; Back-Projektion im **nativen** Frame.
- **Glättung im 4326-Produkt:** ein isolierter 1,6–1,8 m-Kandidat verschwindet dort; `dsm − dtm` hebt den Datum-Versatz heraus.
- **1 m Zelle vs. 2–3 m Objekt** = Auflösungsgrenze; ein dünner Mast wird weggemittelt.
- **Copernicus DEM ist DSM ohne DTM** → global kein `dsm − dtm`; ein DTM muss lokal her.
- **Satelliten-DSM** (30/90 m) ≫ Objekt → nur Kontext.
- **Auth-Grenzen** (kein Quellen-Verdikt, Operator-Aufgabe): Google SV, Mapillary, OpenTopography-GlobalDEM, ALOS.

## Offene Messungen (pending)

| Punkt | Suchbegriff / Endpunkt | Status |
|---|---|---|
| NRCan National Ortho | `maps.geogratis.gc.ca/wms/canimage` | 404 |
| Apple Look Around | „Apple Look Around API endpoint" | pending |
| Bing Streetside API | „Bing Maps Streetside REST" | Web 206, kein API-Doc |
| Auflösung je Zoom (ESRI/Bing/Google) | Programm-Doku je Zoom | nicht via `--verdict` messbar |

**Der Deckel:** höchste Geländehöhe = HRDEM (CA, 1 m) / 3DEP (US, keyless REST); die **Standort-Koordinate**
und die **Objektgröße unter der Zellgröße** bleiben der Boden — kein DEM schließt beide.
