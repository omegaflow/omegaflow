<!--
  title: Handover — Register-Suchlauf: 11 Such-Gaps gemessen (Stand 2026-09-14)
  session: Register-Suchlauf
  class: handover
  date: 2026-09-14
  sha256: 0ded66ec6245dbc01d5917253f9920ea93bc0e62608c44233a985f7014625e88
  status: live
  see-also: docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md phi/blocked_sources.φ
-->
# Handover — Register-Suchlauf: 11 Such-Gaps gemessen (2026-09-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

## Nadeln

- LASzip-Chunk-Decoder (Arithmetic-Coder + Chunk-Tabelle) für die zwei
  `blocked parser-def las-laz`-Quellen (`s3://noaa-nos-coastal-lidar-pds`,
  `s3://usgs-lidar-public`) — höchste Hebelwirkung, entblockt zwei Quellen.
  (Schritt: `src/archivar/las.rs` erweitern, Referenz laz-rs — laz 0.13.0,
  copc-reader 0.9.0; gemessen an `19961009ATM2_143020JR.copc.laz`)

## Register-Suchlauf — 11 Such-Gaps gemessen (archive_search, 2026-09-14)

Alle 11 Routen sind OFFEN gemessen; die Punkte sind damit Bau-Gaps (Route bekannt,
Konsument/Decoder fehlt). Kein Punkt wurde in `sources.φ` gehoben — das braucht
erst das Halten (Ernte + CDN-Manifestation), ein eigenes Atom.

- WFAU VSA (VISTA/VVV) — NOIRLab Astro Data Lab TAP spiegelt `vhs_dr5`; kein
  Konsument. (Schritt: Nadel-Kanal benennen, sonst pending)
- WFAU WSA (UKIDSS DR10+) — NOIRLab spiegelt `ukidss_dr11plus`; kein Konsument.
  (Schritt: wie VSA)
- VLASS — `https://cirada.ca/vcsscatalogue` HTTP 200; FITS-Struktur-Reader fehlt.
  (Schritt: std-only FITS-Reader, Referenz fitparser/fits-header; oder CADC-TAP
  `cirada.VCSS`)
- NOAA CORS RINEX — `rinex 0.22.0` + `crx2rnx 2.7.0` (Hatanaka) als Referenz;
  SBF (Septentrio) absent. (Schritt: RINEX-2/Hatanaka std-only-Reader)
- NOAA ERI — `tiff 0.11.3` + `jpeg-decoder 0.3.2` als Referenz.
  (Schritt: std-only JPEG-in-TIFF-Decoder, Compression 7)
- GK2A AMI — NMSC GSICS + GSICS-KMA-Produkte (gsics.atmos.umd.edu) + DOI
  10.3390/rs13071303. (Schritt: GSICS-Koeffizienten dekodieren)
- GOES-16 ABI — STAR NESDIS GSICS-ABI-Harmonisierung, Koeffizienten-txt je Kanal
  (Release Mai 2025). (Schritt: Koeffizienten laden, `CALIB_GSICS_PENDING` auflösen)
- Himawari-8 AHI — HSD-Block 5 (nominal) dekodiert (`src/archivar/hsd.rs`
  `CalibrationBand`, an echtem Granulat `HS_H08_20220101_0000_B01_FLDK_R10_S0110`
  verifiziert: B01 gain 0.3773583529411764, offset -7.547167058823528).
  (Schritt: Kalibrierung in `himawari_hsd_compiler.rs` anwenden — Counts → Radianz;
  Block 6 (Update) zusätzlich)
- AQS EPA — keylos via Apify (pay-per-record) + OpenAQ-S3. (Schritt: Route prüfen)
- Babamul — LSST-Alerts seit 2026-02-25 öffentlich über die Rubin-Broker (DOI
  10.1088/1538-3873/ae6fef); Python-Client boom-astro/babamul. (Schritt: anonymen
  Zugang neu messen)
- las-laz — laz-rs als Referenz. (Schritt: siehe Nadeln)

## Bau-Gaps (aus der Inventur, unverändert)

19 Punkte ohne Suchbedarf (Route bekannt, Konsument/Parser/Ernte fehlt) — vollständig
in `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` §1 gelistet.
(Schritt: je Punkt der Register-Notiz in `phi/blocked_sources.φ` folgen)

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
