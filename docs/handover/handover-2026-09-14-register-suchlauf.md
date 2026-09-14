<!--
  title: Handover — Register-Suchlauf + Tor-1 (Stand 2026-09-14)
  session: Register-Suchlauf
  class: handover
  date: 2026-09-14
  sha256: 8febecec564d2314c8e7b24bc2897e972925cea18d07e9aebb8b4c087be20e43
  status: live
  see-also: docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md phi/blocked_sources.φ
-->
# Handover — Register-Suchlauf + Tor-1 (2026-09-14)

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
  `s3://usgs-lidar-public`). (Schritt: `src/archivar/las.rs` erweitern, Referenz
  laz-rs — laz 0.13.0, copc-reader 0.9.0; gemessen an `19961009ATM2_143020JR.copc.laz`)

## Bau — Route gemessen, Decoder/Konsument fehlt

- VLASS — `https://cirada.ca/vcsscatalogue` HTTP 200; FITS-Struktur-Reader fehlt.
  (Schritt: std-only FITS-Reader, Referenz fitparser/fits-header; oder CADC-TAP `cirada.VCSS`)
- NOAA CORS RINEX — Hatanaka + RINEX-2 std-only-Reader (Referenz rinex 0.22.0, crx2rnx 2.7.0)
- NOAA ERI — std-only JPEG-in-TIFF-Decoder, Compression 7 (Referenz tiff 0.11.3, jpeg-decoder 0.3.2)
- Himawari-8 AHI — HSD-Block 5 (nominal) ist dekodiert (`src/archivar/hsd.rs`
  `CalibrationBand`, an echtem Granulat verifiziert: B01 gain 0.3773583529411764,
  offset -7.547167058823528); offen: Kalibrierung im `himawari_hsd_compiler.rs`
  anwenden (Counts → Radianz) + Block 6.
- GK2A / GOES-16 — GSICS-Kalibrierung: GK2A NMSC/GSICS-KMA · GOES-16 STAR NESDIS
  Koeffizienten-txt (Mai 2025); `CALIB_GSICS_PENDING` auflösen.
- las-laz — siehe Nadeln.

## Ernte (nach dem jeweiligen Bau)

- GSICS-/HSD-Kalibrierung anwenden (Himawari Block 6, GOES-16, GK2A) ·
  VLASS/CORS/ERI-Compiler + CDN-Manifestation (Register-Pflicht, `--ci-mode`).

## Tor-1 (Forschung, 2026-09-14 gemessen)

- **12 Quellen descoped** — keine Nadel-Kreuzung frißt ihr Feld (stat. Photometrie,
  Radiokontinuum, Niederschlags-Isotopie, Vessel-Tracks u. a.); Gründe + Belege im
  Kopf von `phi/blocked_sources.φ`.
- **Pending bleiben:** MACHO (keine Klassen-Spalte gefunden), LAMOST DR11 (Konsument
  PAST II `10.3847/1538-3881/ac0f08`), NOIRLab (Wiedervorlage Gaia DR4 2026-12-02).
- **AQS** keylos offen (OpenAQ-S3 200) · **Babamul** 401 hält (nicht anonym offen).
- **ONC** — kein lokales `.mat`; die Bin-Geometrie-Messung läuft als CI-Lauf
  (`onc-cdn.yml`, run 34837837926).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
