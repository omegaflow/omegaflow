<!--
  title: Ref — Ephemeriden-Quellen-Index (8 Ziele, gemessen 2026-10-12)
  class: ref
  date: 2026-10-12
  sha256: 1a8973b3a7e687cb6cfd00d1b4fba118b3b44724a9979433fce56d967f7d2a99
  status: live
  see-also: docs/plans/ref-mariner-faden-index.md docs/surveys/survey-2026-10-10-ephemeris-quellen.md
-->
# Ref — Ephemeriden-Quellen-Index (8 Ziele, gemessen 2026-10-12)

Eine gemeldete Quellenliste wurde **gemessen**, nicht übernommen: jeder Eintrag trägt seine
Messung (HTTP-Status, Lizenz-String, Datei/Größe, ID-Auflösung). Zur lokalen Bestandsaufnahme
(wo dieselbe Recherche schon im Baum liegt) siehe den Mariner-Faden-Index.

## LLR / APOLLO — direkt downloadbar
- **Zenodo `7818557`** „APOLLO normal point data: 2006 through 2020" — **cc-by-4.0** (Zenodo `license.id`; DataCite „Creative Commons Attribution 4.0 International"). Dateien: `APOLLO_2006_2020.crd` 7 445 279 B · `APOLLO_2006_2020.np` 343 680 B (**4296 Zeilen**). Direkte URLs `https://zenodo.org/api/records/7818557/files/<name>/content`.

## Asteroiden-Massen / BC430 — direkt downloadbar
- **Zenodo `3558850`** „BC430 Asteroid Ephemerides" — **cc-by-4.0**; `BC430.tar.gz` 59 541 013 B; 300 Asteroiden, 1800–2200, mit Massen (Baer & Chesley).

## Pioneer 10 / 11 Doppler — direkt downloadbar
- **Zenodo `13309156`** (Pioneer **10**, `SC_23.asc` 678 784 528 B) und **`13309040`** (Pioneer **11**, `SC_24.asc` 721 591 221 B) — **cc-by-4.0**; aus dem NAVIO-Format, ASCII.

## Venus-Radar — Zenodo + PDS
- **Zenodo `18176179`** „Venus Geospatial Raster Datasets" — **cc-by-4.0**; u. a. `Venus_RadarBasemap_2025_Global_75m_v1.tif` 80 909 342 816 B · `Processed.zip` 409 371 501 B.
- **PDS Geosciences „Venus Radar Data"** (Arecibo 12,6 cm, 1988–2020) — `https://pds-geosciences.wustl.edu/missions/venus_radar/index.htm`.
  **Nutzung:** PDS verlangt **Zitation** — „Please cite: Bruce A. Campbell and Donald B. Campbell 2022 Planet. Sci. J. 3 55, doi:10.3847/PSJ/ac4f43."; PDS-weit „no cost … All PDS archived data may be exported … under … TSPA". **Keine** explizite Lizenz-/Redistributionsklausel.

## Pioneer Venus Orbiter — PDS-PPI
- PDS3-Volume **`PVO-V-ORSE-1-ODR-OPENLOOP--V1.0`** (Open-Loop-Okkultation, Seasons 1–15) — `https://pds-ppi.igpp.ucla.edu/data/PVO-V-ORSE-1-ODR-OPENLOOP--V1.0/`; NSSDCA `PSPA-00410`.
  **Nutzung:** AAREADME = nur Haftungsausschluss + Errata; keine Lizenz-/PD-Aussage.

## VLBI — JPL erreichbar / CDDIS account-gated
- **JPL** `https://ssd.jpl.nasa.gov/planets/obs_data.html` — erreichbar; „Spacecraft VLBI"-Sektion (Magellan, Venus Express, Phobos, MGS, Odyssey, MRO, Galileo). **Keine** Nutzungs-/Lizenz-/PD-Aussage auf der Seite; die verlinkte JPL-„Image Policy" betrifft nur Bilder/Video. Datenlizenz: **unbestimmt**.
- **CDDIS** `https://cddis.nasa.gov/archive/vlbi` — 302 auf **Earthdata-Login**; Inhalt nicht anonym lesbar.

## Mariner 5 / 9 — request-only (IDs korrigiert)
- **Mariner 5:** NSSDCA **`PSCM-00001`** „Doppler Radio Tracking Data" — request-only. Analyse-Paper: NTRS `19760022055` (NASA-CR-148482).
- **Mariner 9:** `71-051A-00E` ist **kein** gültiger NSSDCA-Dataset (NSSDCA absent); gültig: **`PSPG-00238`** (Primary Mission Doppler) und **`PSPG-00313`** (Complete Mission Doppler+Range), beide request-only.

## NTRS `19900020832` — ein Findmittel, kein Dataset
- „Data catalog series … Volume 1B: Descriptions of data sets from planetary and heliocentric spacecraft and investigations" (1987) — NTRS-**Citation**; NSSDCA trägt keine Collection `19900020832`.

## Nutzungsbedingungen — Fazit (gemessen 2026-10-12)
- **NASA-Policy allgemein** (`https://www.nasa.gov/nasa-brand-center/images-and-media/`): NASA-Inhalte sind „generally not subject to copyright in the United States"; faktengebrauch ohne Erlaubnis, **Quelle nennen**; Insignia/Logotyp **nicht** PD.
- **PDS** (Venus-Radar + PVO-ORO): keine explizite Datenlizenz; PDS-weit kostenlos + **TSPA**; **Zitation erbeten**, keine Haftung.
- **JPL SSD**: keine Aussage auf der Seite → Lizenz unbestimmt, **nicht** als „public domain" ausgeben.
- **Zenodo-Häuser:** `cc-by-4.0`, Attribution.
- Die Datensätze selbst bleiben beim Anbieter (nur verlinken).
