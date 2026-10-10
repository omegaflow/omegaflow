<!--
  title: Survey — Weltweite LiDAR-/Punktwolken-Landschaft (offene Quellen)
  class: survey
  date: 2026-10-10
  sha256: 7a60a3c5928f9f19a08cb2ed37d4c6b393764f01a5a78e33aaed499cb8e086f7
  status: live
  see-also: docs/handover/handover-2026-10-10-mycelium-folge297.md
-->
# Survey — Weltweite LiDAR-/Punktwolken-Landschaft

**Frage.** Welche **weltweiten** offenen LiDAR-/Punktwolken-Quellen gibt es, was ist
davon schon registriert, und was fehlt zum Bau — nicht nur Bayern.

Diese Survey korrigiert einen **Riss**: die Mycelium-296-Zeile „omegaflow hat **kein**
LiDAR registriert (nur Hayabusa-PDS4-LIDAR)" ist falsch; 297 trug sie weiter. Daher hier
die gemessene Landschaft.

## Was schon steht (registriert / gebaut)

- **USGS 3DEP** — registriert: `phi/sources.φ:20260`
  `usgs-lidar-public.s3.amazonaws.com/las_brooks_camp_2012.bin` (`format las`, `terms PD`,
  `compiler las_compiler.rs`, `at earth`, Felder `las_x/y/z_icrs_m`); Origin
  `procedure: fetch https://usgs-lidar-public.s3.amazonaws.com/AK_BrooksCamp_2012/ept-data/0-0-0-0.laz`.
  Host `usgs-lidar-public.s3.amazonaws.com` HTTP 200 (327891 B Listing, 2026-10-10).
- **NOAA NOS Coastal Lidar** — im Katalog `phi/pipeline/catalog/noaa_nodd_inventory.φ:26`
  (`s3://noaa-nos-coastal-lidar-pds`); Host `noaa-nos-coastal-lidar-pds.s3.amazonaws.com`
  HTTP 200 (386818 B, 2026-10-10).
- **LAS/COPC-Reader** — `src/archivar/las/` liest Header/VLRs/COPC-Info+Hierarchie +
  `ept.json`; der **LASzip-Chunk-Dekoder** ist gebaut (`laszip.rs`, `LazDecoder`/
  `has_laszip_vlr`, `las/mod.rs`). Die beiden `blocked parser-def las-laz`
  (NOAA NOS Coastal, USGS 3DEP EPT) sind seit 2026-09-17 aufgelöst
  (`docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md:77`).
- **Copernicus DEM** — registriert (`copernicus-dem-cdn`/`copernicus-dem-90m-cdn` success;
  Predictor-Refactor zentral in `src/archivar/tiff.rs`).

## Weltweite Landschaft (gemessen 2026-10-10)

| Quelle | Host | Status | Inhalt / Format | Lizenz |
| --- | --- | --- | --- | --- |
| USGS 3DEP | `usgs-lidar-public.s3.amazonaws.com` | 200 | EPT + LAZ/COPC, USA | PD |
| NOAA NOS Coastal | `noaa-nos-coastal-lidar-pds.s3.amazonaws.com` | 200 | COPC-LAZ, US-Küste | PD |
| OpenTopography | `portal.opentopography.org` | 200 | globale Punktwolken + DEMs (API-Key nötig) | je Datensatz (CC) |
| open-lidar-data | `s3.eu-central-1.amazonaws.com/open-lidar-data/` | 200 (390654 B) | LAS/LAZ + COPC, Europa→global geplant | frei (Bucket) |
| Canada CanElevation | `open.canada.ca` | 200 | nationale Elevation/LiDAR | OGL-Canada |
| Australia ELVIS | `elevation.fsdf.org.au` | 200 | nationale LiDAR/DEM | CC-BY |
| Netherlands AHN | `www.ahn.nl` | 200 | AHN DTM/DSM Punktwolke | CC0/CC-BY |
| UK Environment Agency LiDAR | `environment.data.gov.uk` | 200 | DTM/DSM/Point Cloud | OGL |
| Bayern DOM20 | `geodaten.bayern.de` (via easygeodata gemessen 296) | — | laser DOM 0,2 m → 1 m, Float32 NHN COG | CC-BY-4.0 |

Die Status-Angaben sind **Host-Roots** (HTTP 200), kein Nachweis eines konkreten
Datenendpunkts — der wird je Quelle **vor** dem Registrieren gemessen.

## Der Bau, je Quelle

Der Reader (`las/`) und der TIFF-Predictor stehen. Was je Quelle fehlt, ist ein
**Manifestator** (`tools/harvest`, Vorlage `copernicus_dem_compiler.rs`/`las_compiler.rs`):
`fetch` des Endpunkts → Reader → `.bin` → CDN-Aufrufer (`*-cdn.yml`). Die Registrierung
(`url`/`origin`/`compiler`/`at`/`terms`/`ttl`) ist Mycelium; `at`/`terms` folgen dem
gemessenen Host.

## Offen (pending)

- **open-lidar-data** — ein LAZ/COPC-Key aus dem Bucket-Listing extrahieren, Reader gegen
  die echte Datei, dann Block + Aufrufer.
- **Bayern DOM20** — COG-Kachel-URL messen, `dom_compiler` bauen, Block + Aufrufer.
- **OpenTopography** — API-Key-Weg messen; die `terms` je Datensatz sind heterogen.
- **NOAA NOS / USGS 3DEP** — die registrierten Zeilen tragen je **eine** Fixture; eine
  echte weltweite Ernte (viele Kacheln) braucht einen Chunk-/Slab-Manifestator.
