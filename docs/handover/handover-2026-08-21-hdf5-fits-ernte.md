<!--
  title: Handover: HDF5-Erntekarte + FITS-Reader
  class: handover
  date: 2026-08-21
  sha256: b872356b6d056e9e6b4e4135a93089994ef82f206b01f668c5678b3edb302069
  see-also: docs/SOURCE_PORT.md docs/handover/handover-2026-08-21-ncei-ssi-hdf5.md
-->
# Handover: HDF5-Erntekarte + FITS-Reader

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # fremde Arbeit benennen, nichts übernehmen
cargo check                      # muss 0/0 sein
ls src/hdf5.rs                   # der HDF5-Reader (parallele Session, ggf. schon da)
grep -n "EARTHDATA_EDL_TOKEN" .github/secrets.template
```

Referenzen (stehend): `src/hdf5.rs` (der entstehende HDF5-Reader),
`src/bin/spectral_compiler.rs` + `src/spectral.rs` (die SSI-Ernte — eigenes
Atom), `src/netcdf.rs`/`src/cdf.rs` (Gold-Standard-Parser als Muster),
`docs/SOURCE_PORT.md` (der eine Pfad), `docs/concepts/sources-v2-spec.md`
(Grammatik + Force-Unit-Registry).

## Auftrag A — HDF5-Ernte (drei MUSS-Quellen)

HDF5 gehört fast ausschließlich zur Erdbeobachtung + SSI + Gravitationswellen.
Der HDF5-Reader (parallele Session) schließt diese drei auf:

1. **Black Marble — VNP46A1/VJ146A1** (em). VIIRS Day/Night Band,
   at-sensor TOA-Nachtradianz, HDF5 (.h5, 28 SDS), 15 arc-sec global,
   seit 2012, ~40 MB/Datei, 340–648 Dateien/Tag. LAADS DAAC
   (`ladsweb.modaps.eosdis.nasa.gov/archive/allData/5200/VNP46A1/`),
   DOI 10.5067/VIIRS/VNP46A1.002. **Suomi-NPP endet 1. Nov. 2026** →
   Nachfolger VJ146A1 (NOAA-21/20). Auth: Earthdata-Login —
   **`EARTHDATA_EDL_TOKEN` liegt in `.secrets.local` vor**. Frame:
   gridded lat/lon (15-arc-sec-Kachel, `hXXvYY`) → `map`-Extract auf der
   Erde. Die Composite-Stufen A2/A3/A4 sind abgeleitet → decline, A1 ist
   die Messung.
2. **VIIRS/MODIS Surface Reflectance — VNP09/MOD09** (em). Der echte
   „Blue-Marble"-Kanal: Oberflächen-Reflexion (Albedo). HDF-EOS5 (VIIRS =
   HDF5; MODIS = HDF4). LP DAAC / LAADS, Earthdata-Login (dieselbe
   `EARTHDATA_EDL_TOKEN`). Das „Blue Marble"-Composite-Bild selbst ist
   decline (Ästhetik, kein Feldwert).
3. **LIGO GWOSC Strain** (gravity). GWOSC (gw-openscience.org) stellt die
   Strain-Daten als **HDF5** (.hdf5) bereit — offen, kein Key. Force:
   gravity (Strain h(t), die Messung selbst). Frame: Detektor-Position
   (H1 Hanford / L1 Livingston / V1 Virgo) → `on earth`-Punkt. Exakte
   GWOSC-Download-URL in der Session verifizieren (gwosc.org → Daten →
   S3-Bucket `gw-openscience.org`).

## Auftrag B — FITS-Reader (pure Rust, std-only)

Galaxie und Universum messen in **FITS**, nicht HDF5. Ein FITS-Reader ist
der fehlende Schlüssel — als Gold-Standard-Parser wie `netcdf.rs`/`cdf.rs`:
Rust std-only, keine externen Crates (die `fitsio`/`cfitsio`-Referenzen und
der FITS-Standard (Pence et al., A&A 2010) nur als Inspiration/Format-
Referenz). Scope: Header (80-Byte-Card-Blöcke), primäre + Erweiterungs-HDU,
Image- und BINTABLE-Daten, BSCALE/BZERO-Skalierung, WCS (RA/DEC aus dem
Header). Das schließt auf: Gaia-FITS-Kataloge, SDSS, 2MASS, Pan-STARRS,
DES, Planck, JWST. Die HDF5-Simulationen der Galaxien/Kosmologie
(IllustrisTNG, EAGLE, CAMELS, FLAMINGO, AbacusSummit, Uchuu) bleiben
decline — Modelle, keine Messung.

## Verifizierter Kontext (2026-08-21, recherchiert)

- **Black Marble VNP46A1 = HDF5** bestätigt (LAADS-Produktseite: „Data
  Format: HDF5", .h5, 28 SDS, at-sensor TOA Nachtradianz nW·cm⁻²·sr⁻¹).
- **Blue Marble** = fertiges RGB-Composite (PNG/GeoTIFF) → decline; die
  Rohmessung ist VIIRS/MODIS Surface Reflectance (HDF-EOS).
- **Sonnensystem**: kaum HDF5 — Heliophysik ist CDF/JSON/live (schon in
  `sources.φ`), Planeten sind PDS4/CDF/FITS. HDF5 dort ≈ nur die SSI
  (eigenes Atom).
- **Galaxie/Universum**: Messungen = FITS (Gaia, SDSS, LSST/Rubin, DES,
  Planck); HDF5 = Simulationen → decline.
- **Secrets** (`.secrets.local`, nur Marker): `EARTHDATA_EDL_TOKEN` ✅,
  `MAST_TOKEN` ✅, `IRSA_USER`/`IRSA_PASS` ✅, `NASA_API_KEY` ✅,
  `NASA_ADS_TOKEN` ✅, `EUMETSAT_KEY`/`EUMETSAT_SECRET` + `CEDA` ✅,
  `LASAIR_TOKEN` ✅. Keine HDF5/FITS-Quelle hat einen Auth-Blocker.

## Gates

- cargo check 0/0, cargo test. Pro Ernte: eine echte Datei laden, Parsen,
  Samples extrahieren, roundtrip. Kein Fenster, keine Radiatoren (kopf-los).
- Register: `phi/sources.φ` (neue Blöcke), `phi/dead_sources.φ` (die
  Simulation-Declines mit verifizierter URL + note), `ledger.φ`, TODO.md.
- Ein Commit je Einheit; das letzte schließt die offenen Posten.
- Diese Datei nach Abschluss archivieren (Regel in AGENTS.md).

## Nicht anfassen

Die Membran/`fs`-Rendering-Physik, `src/te.rs`, die NCEI-SSI-Ernte
(eigenes Atom), der Asteroiden-Langbogen (eigenes Atom), der
`wm2_1au`-Pfad, die OMEGAFLOW_HIDDEN-Radiator-Stille. Nur: HDF5-Ernte
(Black Marble, VIIRS SR, LIGO) + FITS-Reader.
