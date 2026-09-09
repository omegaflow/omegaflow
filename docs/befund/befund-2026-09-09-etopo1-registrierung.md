<!--
  title: Befund — ETOPO1-Gitter registriert: das 395-MB-Raster trägt eine phi/sources.φ-Zeile, die CDN-Manifestation ist CI-Duty (Quelle verifiziert, nicht mehr nur lokal)
  class: befund
  date: 2026-09-09
  sha256: 0ff769f0e08a20f98766ef849025c68aefa075509da351cd8ee4ff814b79cd71
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-zonen-flotte.md docs/handover/archiv/handover-2026-09-09-seismische-ortung-tsunami.md
-->
# Befund: ETOPO1-Gitter registriert

## Frage & Bindung

Registerzeile „ETOIPO1-Referenz-Kernel — Vollauflösung + CDN-Manifestation des 395-MB-Gitters (Folge-Pflicht des Eikonal-Befunds)." Ein geernteter Datensatz, der nur lokal liegt, ist eine Registerschuld: die CDN-Manifestation ist eine Session-Duty, nicht ein Wunsch.

## Was gebaut wurde

Das 1-Bogenminuten-Eis-Oberflächen-Gitter `ETOPO1_Ice_g_gdal.grd.gz` (395.076.022 B) lag als lokale Arbeitskopie in `data/ngdc.noaa.gov/` (gitignoriert). Registriert in `phi/sources.φ`:

```
url https://www.ngdc.noaa.gov/mgg/global/relief/ETOPO1/data/ice_surface/grid_registered/netcdf/ETOPO1_Ice_g_gdal.grd.gz
format reference
sha256 98c62471284141f35e91c70f584ba13b2affd7f18b904a55005b453a6838f3a9
ttl 86400
```

Die Form ist die des Registers für feldlose statische Assets (`format reference` + `sha256`). Die Quelle ist curl-verifiziert (HEAD 200, Ranged-GET 206, content-length 395.076.022 — byte-identisch zur lokalen Kopie). Der CI-Manifestator (`kernel-flatten.yml`, `--ci-mode`) bringt das Asset aufs CDN; ein lokaler Lauf speist das gemeinsame Gedächtnis nie.

## Die Befunde

1. **Die Registerschuld ist gehoben:** das Gitter hat jetzt eine Quelle im Register; die Manifestation ist CI-Duty, nicht lokaler Bestand.
2. **Zitations-Defekt benannt:** die Handover-Zeile zitiert `befund-2026-09-09-tsunami-eikonal`, der nie committed wurde. Der Eikonal-Stein lebt im archivierten Handover `docs/handover/archiv/handover-2026-09-09-seismische-ortung-tsunami.md` (Zeilen ~58–61, Dijkstra über ein Bathymetrie-Gitter). Der Verweis wird gemessen korrigiert (auf den archivierten Handover), kein nachträglich fabrizierter Befund.

## Verdikt

Das ETOPO1-Gitter ist registriert — die CDN-Manifestation ist benannt und an den CI übergeben. Der Eikonal-Löser selbst (Dijkstra über das volle Gitter) bleibt `pending`; die Voraussetzung (das Gitter am dauerhaften Ort) steht.
