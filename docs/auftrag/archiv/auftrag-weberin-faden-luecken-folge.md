<!--
  title: Auftrag — Weberin-Faden-Lücken (Folge): offene Subfragen der 9 Lücken + 3 Mess-Punkte
  class: auftrag
  date: 2026-09-07
  sha256: 4f682082f35737f22d30ae6fb40dbeafe4c3394b4ebc6f392b4f57c4b60af02e
  status: archived
  see-also: docs/auftrag/auftrag-extern-weberin-faden-luecken.md docs/handover/handover-2026-09-07-weberin-zeugen-faden-matrix.md docs/TODO.md
-->
# Auftrag — Weberin-Faden-Lücken (Folge): offene Subfragen der 9 Lücken + 3 Mess-Punkte

Dieser Auftrag ist die Folge von `docs/auftrag/auftrag-extern-weberin-faden-luecken.md`.
Der erste Auftrag hat für neun Faden-Kategorien maschinenlesbare Routen benannt; die
Bau-Arbeit danach hat für 5 der 9 Kategorien den Compiler + Feldblock gebaut (2, 3, 7, 8, 9)
und für Lead 4 das Rats-Urteil `decline` (position-only) gesetzt. Was jetzt noch offen ist,
sind die engen Subfragen der verbliebenen Lücken plus drei Mess-Punkte aus den gebauten
Daten. Dieser Auftrag ist selbsttragend; wer ihn ausführt, liest vorab `docs/SOURCE_PORT.md`
(§7–§9, §11, §13) für die Taxonomie und die Prüf-Kaskade. Es wird KEINE Datei geschrieben,
kein Register editiert, kein Force-Gate entschieden — es wird nur benannt, was die Messung
IST, und Befunde gemeldet. Der lokale `archive_search --leads`-Lauf (2026-09-07) hat drei
neue Hosts ausgeworfen, die in die offenen Fragen eingewoben sind.

## Status-Vokabular (bindend)

- `live` — 200, offen, maschinenlesbar.
- `blocked` — lebt, Zugang gesperrt (Form nach Blocked-Kanon: key/account/ip-blocked).
- `declined` — lebt, aber keine physikalische Messung am Punkt (Modell/position-only/kommerziell).
- `pending` — Route bekannt oder geahnt, Verdikt ausstehend.
- `not-published` — kein offener maschinenlesbarer Weg nach abgeschlossener Suche.

Regel: Ein toter Endpoint ist kein Endzustand — erst die Kaskade curl → r.jina.ai →
WebArchive → Websuche, dann ein Verdikt mit note, die den Recherche-Stand nennt.
Spekulationswörter sind verboten; ein ungemessener Befund heißt `pending`. Jeder Befund
trägt das Datum der Messung.

## Teil A — die 9 Lücken, Stand nach dem Bau + offene Subfrage

### 1. Boden-Gravimeter (gravity) — IGETS/GGP
- Gebaut/Stand: Stationsliste live (position), Zeitreihen `blocked account` (ISDC/GFZ).
- Offen: Gibt es eine offene Level-2/3-Zeitreihen-Route ohne Konto (DOI-Landingpages
  `dataservices.gfz-potsdam.de/igets/…`)? Stations-Anzahl und Messgröße (nm/s²) notieren.

### 2. Infraschall (acoustic) — BGR abgeleitete Detektionen
- Gebaut: `bgr_infrasound_compiler` + `bgr_infrasound.bin` Feldblock (live, 4 Bänder).
  vDEC-Roh bleibt `blocked account`.
- Offen: nichts Externes mehr — der `N_avail`-Punkt liegt in Teil B.

### 3. Hydrophon / Ozean-Lärm (acoustic) — NOAA NODD
- Gebaut: `noaa_nodd_bucket_harvester` + `noaa_nrs_psd.bin` Feldblock (live, Stationen 01/11).
- Offen: Deployment-Tiefe und quality_flag-Semantik — beide in Teil B.

### 4. Seismische Stations-Netze (seismic) — Weltlinien
- Urteil: council `decline` (position-only, kein Messwert). `fdsn_station_compiler` bleibt Tool.
- Offen: der Waveform-Reader (miniSEED/FDSN dataselect), der den Anker mit gemessener
  Bodenbewegung verbindet — der eigentliche nächste Build. Zusätzliche Station-Routen aus
  `archive_search`: `fdsnws.raspberryshakedata.com` (Netz AM, level=station) und GEOFON
  (`geofon.gfz.de/fdsnws/station/1`). Status dieser beiden Station-Endpoints messen.

### 5. GIC (electric) — geomagnetisch induzierte Ströme
- Stand: `not-published` (kontinuierlich); Zenodo 10594301 = punktuelles Einzel-Asset.
- Offen: Gibt es einen kontinuierlichen offenen GIC-Datensatz (Zeitreihe mit Position)?

### 6. Blitz-Bodennetze (electric) — WWLLN u. a.
- Stand: Thunder Hour `declined` (Aggregat); Ereignis `not-published`.
- Offen: offener Thunder-Hour-Nachfolger (NASA Earthdata GHRC DAAC vs. `wwlln.net/climate`).
  Neu aus `archive_search`: `lightning.nsstc.nasa.gov/data/` (NASA LIS/OTD) — Format,
  Zeitraum, Auflösung messen; ist das ein eigener Ereignis-/Blitz-Kanal neben GLM?

### 7. SuperDARN (electric)
- Gebaut: `superdarn_fitacf_compiler` + `superdarn_fitacf.bin` (live); Stationen via hdw.dat.
- Offen: nichts Externes — CDN-Upload ist CI-Pflicht.

### 8. HF-radar (advective)
- Gebaut: 183 Radial-Sites + 20 Wave-Datasets registriert (live).
- Offen: nichts Externes — CDN-Upload ist CI-Pflicht.

### 9. BGC-Argo (diffusion)
- Gebaut: `argo_bgc_profile_compiler` + `argo_bgc.bin` + DOXY/NITRATE/CHLA/BBP700/PH (live).
- Offen: nichts Externes — CDN-Upload ist CI-Pflicht.

## Teil B — die 3 offenen Mess-Punkte

### a) NOAA-Deployment-Tiefe (absent)
- Gemessen: `metadata.json` ohne Tiefen-Schlüssel, netCDF ohne `geospatial_vertical`,
  data.gov-Slug 404. Die Tiefe ist im Produkt nicht enthalten.
- Offen: Existiert ein offener Kanal (NCEI passive-acoustic-Metadaten, neues Datenportal,
  die `calibration/`-Objekte im Bucket) mit der Hydrophon-Tiefe je Deployment?

### b) BGR `N_avail` (0 = honest Read)
- Gemessen: die netCDF-4-Datasets für `N_avail` sind contiguous mit UNDEF-Adresse
  (nie beschrieben) → 0 ist der ehrliche Read, kein Messwert.
- Offen: Trägt das BGR-Produkt die Sensor-Verfügbarkeit woanders (README/Metadaten/
  andere Variable), oder ist sie im Produkt gar nicht enthalten?

### c) NRS `quality_flag`-Semantik (ungemessen)
- Gemessen: Wert 4 in-band, Bedeutung nicht decodiert.
- Offen: Die NRS-HMD-Dokumentation, die die quality_flag-Werte definiert
  (gut/schlecht/interpoliert/…), finden und die 4 zuordnen.

## Abschluss je Kategorie (ein Befund-Block)

Für jede offene Frage ein fester Befund:
1. Was die Messung IST (Größe, Einheit) — kein Force-Gate-Urteil.
2. Die gefundene Route (URL, Format, Position ja/nein) mit gemessenem HTTP-Status und Datum.
3. Das Verdikt aus dem Status-Vokabular.
4. note: was IST (Herkunft, Umfang, offene Restfrage). Absent heißt `not-published` —
   nie ein Ersatz, nie eine Null.
