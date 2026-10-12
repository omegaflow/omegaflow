<!--
  title: Ref — Mariner-Faden-Index (state-Bestand, Stand 2026-10-12)
  class: ref
  date: 2026-10-12
  sha256: 4c78b9913bc985d4ab81334074b8fd8e158620460c20917efa467aa9652aea00
  status: live
  see-also: docs/plans/ref-ephemeriden-quellen-index.md docs/surveys/survey-2026-10-10-ephemeris-quellen.md
-->
# Ref — Mariner-Faden-Index (state-Bestand, Stand 2026-10-12)

Einstieg in den Mariner-Faden: was schon gemessen im lokalen Bestand liegt, damit nicht neu
gesucht wird. Die genannten `state/`-Pfade sind der lokale (private) Recherche-Korpus; die
öffentlich nutzbaren Quellen stehen im Ephemeriden-Quellen-Index.

## Mariner 10 (Merkur-Imaging) — die Pose liegt geschlossen vor
- `state/research/m10-camera-angles/camera_angles.txt` (+ `MANIFEST.md`) — **811 Bilder** à 3 Zeilen:
  `[Julian Date + FDS-Frame-ID]` · `[SX SY SZ]` (Sondenposition) · `[C1 C2 C3]` (Kamera-Pointing-Winkel).
  Zeitraum `1974-03-29`–`1975-03-17` = die **drei Merkur-Encounters** (M1 · M2 · M3).
- **Format-Doku:** `https://ser.im-ldi.com/M10/TXT/angles.html` („Mariner 10 Improved Camera Angles and Spacecraft Vector"). Zeile 1 = JD + FDS-Clock (**zugleich der Bild-Dateiname**), Zeile 2 = Sondenposition, Zeile 3 = Kamera-Pointing-Winkel.
- **Quelle:** Robinson et al. 1999, JGR 104(E12) 30847, DOI `10.1029/1999JE001081` (811 Bilder, alle drei Flybys). Die exakte Einheit/der Rahmen von `C1C2C3` sowie das Zentrum/die Einheit von `SXSY/SZ` sind in den zugänglichen Dokumenten **nicht** benannt.
- **NAIF M10** (`https://naif.jpl.nasa.gov/pub/naif/M10/`): IK/CK/PCK/SCLK/EK leer; FK nur Name→ID; SPK = `M10_archive_1.bsp` (50 KB, nur 3/21–4/4/74). Dem `aareadme` nach wurde die Robinson-Navigationslösung **nie** in C-Kernels überführt → die ASU-Datei ist die einzige Pose-Quelle.

### FOV (verifiziert)
- **Narrow-Angle:** 1500 mm Fokus, **0,5°** FOV; **Wide-Angle:** 62 mm, **11°×14°**; 832 × 700 Bildpunkte, GEC-1-inch-Vidicon (NSSDCA Experiment `1973-085A-01`).
- **JPL 615-148 Table 11** (über die extremen Reseau-Marks): NAO ≈ 0,366°×0,472°; WAO 9,03°×11,65°. Fokallängen A 1493,6 / B 1500,1 mm (Robinson 1999).

## Mariner 4 (DSC-0119) — Parser-Gap
- `state/mountain/mariner4_dsc0119_format.md` — ODG/ODP-Format gemessen (Record-Layout, Datentypen 1–15, Code-/Weight-Word, Byte-Walk an `PSCM-00021`). **Offen:** Table 3 (variable Record-Länge) ist im 16-Seiten-Scan nicht enthalten — vor dem Parser zu pinnen.

## Mariner 2 / 5 / 9 — Routen
- **Mariner 5:** NSSDCA `PSCM-00001` (request-only).
- **Mariner 9:** NSSDCA `PSPG-00238` / `PSPG-00313` (request-only) — **nicht** `71-051A-00E`.
- (Detail + IDs: Ephemeriden-Quellen-Index.)

## Ephemeriden-Umfeld (lokale Rohrecherche)
- `state/mountain/eph_research/eph-1-llr … eph-8-pvo` — je ~250–270 Zeilen Rohsuche.
- Roster-Runde: `state/stimmen/2026-10-10_mountain_ephemeris-sources-round.md` → `docs/surveys/survey-2026-10-10-ephemeris-quellen.md`.

## Generierte Karte (nicht kuratiert)
- `state/research-map.md` (vom `state_cartograph` erzeugt) — Vollbestand; nur über lokale Lesewerkzeuge sichtbar.
