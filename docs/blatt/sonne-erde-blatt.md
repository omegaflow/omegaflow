<!--
  title: SONNE-ERDE-BLATT — Kanal-Register-Audit (fünf Klassen)
  class: sheet
  date: 2026-10-01
  sha256: 4accdd0c8bba761ac87bdede02d6918cc6839f1d100f47ab2ddc31e23dea5954
  status: live
  see-also: phi/sources.φ docs/paper/gic-causal-driver.md
-->
# SONNE-ERDE-BLATT — Kanal-Register-Audit (fünf Klassen)

**Datum:** 2026-10-01 · **Axiom:** A = A
**Ordnung:** 0 honored — je Kandidat der gemessene Register-Zustand (`trägt` / `absent` / `pending`), nichts interpoliert. Der Träger nennt die Quelle:Zeile, nicht die Physik.

## Die fünf Kanal-Klassen

- **Sonne.** RTSW trägt (`phi/sources.φ:158` `url …/rtsw_mag_1m.json`, `:164` `url …/rtsw_wind_1m.json`); OMNI trägt (`:575` `url …OMNI2_H0_MRG1HR…`, daneben `:736`/`:1377`/`:1391`). GOES trägt: XRS `:152` `url …/xrays-7-day.json` (Archiv `:812` `url` / `:814` `origin`), EUV `:417` `url …/euvs-7-day.json`. SDO trägt nicht als eigener Kanal (kein `sdo`-Treffer); AIA trägt (`:1802` `url` / `:1804` `origin https://jsoc.stanford.edu/cgi-bin/ajax/jsoc_fetch`). EVE trägt (`:2362` `url` / `:2364` `origin https://lasp.colorado.edu/eve/data_access/evewebdata/products/level2`).
- **Magnetfeld.** INTERMAGNET trägt: 154 Stations-`url`s `:5383–:6913` (Schritt 10), dazu `origin https://imag-data.bgs.ac.uk/GIN_V1/hapi/data` `:1788`/`:1796`; die zwei Band-Archive (`abk`, `sod`) tragen `:1786`/`:1794`.
- **Ozean.** SST (ersstv5/nino34) trägt (`:11176` `url` / `:11178` `origin https://coastwatch.pfeg.noaa.gov/erddap/griddap/nceiErsstv5`). Argos trägt nicht im Register (`register_lookup argos` → 0 Treffer; keine `argos`-Zeile in `phi/sources.φ`); die nächsten gemessenen Argo-Zeilen: Argovis `:1230` `url https://argovis-api.colorado.edu/argo…`, Argo-DAC `:8788`/`:8803`/`:8818`, Argo-BGC `:9101` `url` / `:9103` `origin https://data-argo.ifremer.fr/argo_bio-profile_index.txt.gz`.
- **Atmosphäre.** Open-Meteo trägt (`:218` `url https://api.open-meteo.com/v1/forecast…`, `:3499` `url …archive-api.open-meteo.com`, `:8760` `url …air-quality-api.open-meteo.com…`). ERA5 trägt nicht in `phi/sources.φ` (kein `era5`-Treffer); das Register-Verdikt ist `declined` (`phi/declined_sources.φ:1007`, `:1230` `url …reanalysis-era5-single-levels`, `:1231`).
- **Boden.** USGS-Erdbeben-Feed trägt (`:96` `url …earthquake.usgs.gov…geojson…`, `:103` `url …earthquake.usgs.gov…quakeml…`, `:11195` `origin https://earthquake.usgs.gov/fdsnws/event/1/query`). ISC-Feed trägt (`:9433` `url` / `:9435` `origin https://www.isc.ac.uk/fdsnws/event/1/query`; weiter `:11400` `url` / `:11402` `origin http://download.isc.ac.uk/isc-ehb/`).

## Gemessene Nicht-Pfeile

- **ENSO-Vier-Kanal-Block (Blatt I).** Kein Pfeil schlägt die Familien-Schwelle. Der Block gegen NINO3.4 (`ersstv5_nino34_ssta`) ist gemessen (`tools/measure/src/bin/enso_blatt_probe.rs`, CI-Lauf 36740119542): `fam = 2.9610e-1`, jedes gerichtete Paar still oder auf Familien-Grenze, `cTE(Bz → SST | Wnd) = 8.359e-3` unter seiner Schwelle `2.612e-2` (bedingt still). Der Block trägt keinen Pfeil (`docs/paper/gic-causal-driver.md:256-280`, §3.6).
