<!--
  title: Survey — Die Weberin: offene Quellen-Routen (Stand 2026-09-13)
  class: survey
  date: 2026-09-13
  sha256: 4640770d061ff3ce47d7afb4383dc4c22583716cb057c6f18c04bc6bb5275111
  status: live
  see-also: docs/concepts/die-weberin.md docs/surveys/survey-2026-09-07-weberin-thread-matrix.md docs/auftrag/archiv/auftrag-extern-weberin-faden-luecken.md
-->

# Die Weberin — offene Quellen-Routen (Stand 2026-09-13)

Dieser Snapshot führt die elf Quellenklassen nach, die die Faden-Matrix
(`docs/surveys/survey-2026-09-07-weberin-thread-matrix.md`) als `pending`
oder `not-published` benannte. Vier Taucher haben je Klasse mit dem
Recherche-Werkzeug `archive_search` gearbeitet (`--leads`, `--verdict`,
`--brave`, `--crossref`, `--arxiv`, `--wayback`, `--playwright`). Die
Prüf-Kaskade ist direkt curl → WebArchive → Websuche; der `r.jina.ai`-Schritt
wurde repo-weit entfernt (Commit e4d734f). Alle Messungen datieren
2026-09-13, lokaler Exit, kein `proton*`-Interface aktiv.

Status-Vokabular (bindend): `live` = offen, maschinenlesbar; `blocked` =
lebt, Zugang gesperrt (account/key/ip); `declined` = lebt, aber keine
physikalische Messung am Punkt (Modell, Vorhersage, position-only,
kommerziell); `pending` = Route bekannt/geahnt, Verdikt ausstehend;
`not-published` = kein offener maschinenlesbarer Weg nach abgeschlossener
Suche. Absent heißt `not-published` — nie ein Ersatz, nie eine Null.
Dieses Blatt ist ein Snapshot: kein Register-Eingriff, kein Compiler-Bau
(Operator-Wort, nur Snapshot).

## 1. Boden-Gravimeter (gravity) — IGETS/GGP

**Was die Messung IST:** zeitvariierende Schwerebeschleunigung eines
supraleitenden Gravimeters, Größe nm/s². Level 1 = Roh-Schwere + lokaler
Luftdruck (1–2 s, auf 1 min dezimiert); Level 2 = instrumentell korrigiert
(tidalanalyse-fertig); Level 3 = Schwere-Residuen.

| Route | Format | Position | HTTP | Datum |
|---|---|---|---|---|
| `https://isdc.gfz.de/igets-data-base` (= `igets.gfz-potsdam.de`) | HTML-Tabelle | ja (N Lat / E Lon / Height MSL) | 200 | 2026-09-13 |
| `https://doi.org/10.5880/igets.<st>.l1.001` | DOI-Landing | — | 200 | 2026-09-13 |
| `https://dataservices.gfz-potsdam.de/igets/showshort.php?id=…` | HTML | — | 200 | 2026-09-13 |
| `doidb.wdc-terra.org/oaip/oai` (set `DOIDB.IGETS`) | OAI-PMH | nein | live (2026-08-16) | — |
| `https://isdc.gfz.de/igets-data-base/data-access` | HTML (Hinweis) | — | 200 | 2026-09-13 |
| `igetsftp.gfz.de` (SFTP, account) | SFTP | — | kein HTTP | 2026-09-13 |
| `http://igets.u-strasbg.fr/tr005.php` (Alt-EOST) | HTML (nur JPG/PDF) | — | 200 | 2026-09-13 |

**Verdikt:** anonyme Stationsliste MIT Position = **live** (65 Stationszeilen,
24 mit DOI-Landing). Zeitreihen Level 1/2/3 ohne Konto = **blocked account**
(registrierter SFTP-Account) bzw. **not-published** (kein offener Weg
gemessen). `igetsftp.gfz.de/igets.bin` ist bereits in `sources.φ` (Z. 7944).

**note:** Messgröße nm/s². Der SFTP-Zugang bleibt account-gebunden (Passwort
über `igets-support@gfz.de`). Kein offener L2/L3-Weg.

## 2. Infraschall (acoustic) — CTBTO IMS / BGR

**Was die Messung IST:** PMCC-reprozessierte Infraschall-Detektionslisten je
netCDF: mittlerer Rückazimut (deg), scheinbare Geschwindigkeit (m/s),
RMS-Amplitude (Pa), Mittenfrequenz (Hz) plus Station `long`/`lat`/`elev`.
Rohwellenformen (Pa) nur über vDEC.

| Route | Format | Position | HTTP | Datum |
|---|---|---|---|---|
| `https://download.bgr.de/bgr/geophysik/Infrasound_maw_product/netCDF/BGR_infrasound_maw_product.zip` (DOI 10.25928/bgrseis_bblf-ifsd) | netCDF-ZIP | ja | 206 (Range) | 2026-09-13 |
| `…/Infrasound_mb_lf_product/…/BGR_infrasound_mb_lf_product.zip` (10.25928/bgrseis_mblf-ifsd) | netCDF-ZIP | ja | 206 | 2026-09-13 |
| `…/Infrasound_mb_hf_product/…/BGR_infrasound_mb_hf_product.zip` (10.25928/bgrseis_mbhf-ifsd) | netCDF-ZIP | ja | 206 | 2026-09-13 |
| `…/Infrasound_hf_product/…/BGR_infrasound_hf_product.zip` (10.25928/bgrseis_bbhf-ifsd) | netCDF-ZIP | ja | 206 | 2026-09-13 |
| `https://geoportal.bgr.de/smartfindersdi-csw/api?…` | CSW/XML | — | 200 | 2026-09-13 |
| `https://essd.copernicus.org/articles/14/4201/2022/` | Belegpaper | — | 200 | 2026-09-13 |
| `https://www.ctbto.org/specials/vdec/` | HTML (Vertragsweg) | — | 403 (Wayback) | 2026-09-13 |
| `https://vdec.ctbto.org/` | — | — | 404 | 2026-09-13 |

**Verdikt:** abgeleitete Detektionslisten = **live** (4 offene netCDF-ZIPs,
CC BY 4.0, 53 IMS-Stationen, 2003–2020, vier Frequenzbänder).
Rohwellenform = **blocked account** (vDEC, Antrag + Vertrag); offen nicht
publiziert = **not-published**. `bgr_infrasound.bin` ist bereits in
`sources.φ` (Z. 5797).

**note:** Positionsfelder in jeder Datei; 18 Jahre. Restfrage: heutiger
vDEC-Einstiegspfad (`www.ctbto.org` 403, Vertragsweg bleibt account).

## 3. Unterwasser-Mikrofone (acoustic) — NOAA NRS / ONC

**Was die Messung IST:** Unterwasser-Schalldruck (Hydrophon). Roh:
FLAC-komprimiertes WAV (5 kHz). Abgeleitet: Schallpegel-Metriken (HMD/PSD,
dB re 1 µPa²/Hz), NRS-Band 10–2000 Hz.

| Route | Format | Position | HTTP | Datum |
|---|---|---|---|---|
| `https://storage.googleapis.com/storage/v1/b/noaa-passive-bioacoustic/o?prefix=nrs/` | GCS-Bucket | — | 200 | 2026-09-13 |
| `nrs/audio/01/nrs_01_2014-2015/metadata/NRS_2014-2015_01.xml` | XML | ja (lat/lon/Region) | 200 | 2026-09-13 |
| `nrs/README.pdf` | PDF | — | 200 | 2026-09-13 |
| `https://www.ncei.noaa.gov/products/passive-acoustic-data` | HTML (quality_flag) | — | 200 | 2026-09-13 |
| `https://pmel.noaa.gov/acoustics/noaanps-ocean-noise-reference-station-network` | HTML | — | 200 | 2026-09-13 |
| `https://data.oceannetworks.ca/api/locations?method=get` | JSON (ONC) | — | 401 | 2026-09-13 |
| `https://data.oceannetworks.ca/api/devices?method=get&deviceCategoryCode=HYDROPHONE` | JSON (ONC) | — | 401 | 2026-09-13 |
| `https://borealisdata.ca/dataverse/oceannetworkscanada` | Dataverse | — | 200 | 2026-09-13 |
| `https://ncei.noaa.gov/products/ocean-noise` (alt) | — | — | 404 (umgezogen) | 2026-09-13 |

**Verdikt:** NRS Roh + Metriken + Deployment-Position = **live** (anonym, GCS,
Dataset-DOI 10.7289/V5M32T0D). ONC Hydrophon-/Deployment-Feed = **blocked key**
(Token nötig, `Registration` offen). Deployment-Tiefe je ONC-Station =
**pending** (hinter Token nicht gemessen).

**note:** NRS = 12–13 Stationen, FLAC-Roh + CSV/NC-Metriken. Der
`quality_flag`-Wert 4 ist decodiert: **1 = Good, 2 = Not evaluated/Unknown,
3 = Compromised/Questionable, 4 = Bad/Unusable** (NCEI/SoundCoop-HMD-Matrix,
QARTOD-adaptiert).

## 4. Seismische Stations-Netze als Weltlinien (seismic)

**Was die Messung IST:** Bodenbewegung (Breitband-Seismometer, m/s); die
Station selbst ist die Weltlinie (Position + Start/End-Zeit).

| Route | Format | Position | HTTP | Datum |
|---|---|---|---|---|
| `https://service.earthscope.org/fdsnws/station/1/query?level=station&format=text&network=IU` | FDSN text | ja | 200 (116 IU-Zeilen) | 2026-09-13 |
| `https://geofon.gfz.de/fdsnws/station/1/query?level=station&format=text&network=GE` | FDSN text | ja | 200 (147 GE-Zeilen) | 2026-09-13 |
| `https://data.raspberryshake.org/fdsnws/station/1/query?level=station&format=text&network=AM` | FDSN text | ja | 200 (7890 AM-Stationen) | 2026-09-13 |
| `…/fdsnws/dataselect/1/query?…` (EarthScope, GEOFON, RaspberryShake) | miniSEED | — | 200 / 200 / 204 | 2026-09-13 |
| `https://www.isc.ac.uk/fdsnws/event/1/query?format=xml&starttime=…&endtime=…&minmagnitude=6` | QuakeML 1.2 | Hypozentrum | 200 (enger Zeitraum) | 2026-09-13 |
| `fdsnws.raspberryshakedata.com` (Auftragshost) | — | — | DNS-tot | 2026-09-13 |
| `isc-mirror.iris.washington.edu` | — | — | DNS-tot | 2026-09-13 |

**Verdikt:** EarthScope + GEOFON Station/dataselect = **live** (Position ja,
`Network|Station|Latitude|Longitude|Elevation|SiteName|StartTime|EndTime`).
RaspberryShake AM = **live** am korrigierten Host (Redistribution lizenziert
eingeschränkt, DOI 10.7914/SN/AM). ISC-QuakeML = **live** mit engen
Zeitfenstern. `fdsnws.raspberryshakedata.com` = **not-published** (DNS-tot,
ersetzt).

**note:** Umfang: EarthScope 151 229 Stations-Epochen (IU 116), GEOFON 19 883
(GE 147), RaspberryShake AM 7890 unique. Der Waveform-Reader läuft über
dataselect (miniSEED); offene Restfrage bleibt ein **QuakeML-1.2-Parser**
(existiert im Register nicht).

## 5. GIC — geomagnetisch induzierte Ströme (electric)

**Was die Messung IST:** Gleichstrom in Ampere über die
Neutralleiter-Erdung von Hochspannungstransformatoren.

| Route | Format | Position | HTTP | Datum |
|---|---|---|---|---|
| `https://transmission.bpa.gov/business/operations/gic/gic.txt` | TSV | nein (nur Namen) | 200 (86 845 B) | 2026-09-13 |
| `https://transmission.bpa.gov/business/operations/gic/gic.aspx` | HTML | — | 200 | 2026-09-13 |
| `https://geomag.bgs.ac.uk/data_service/space_weather/gic_services.html` | HTML (B_GIC-Index) | — | 200 | 2026-09-13 |
| `https://geomag.bgs.ac.uk/data_service/space_weather/geoelectric.html` | HTML/PNG (E-Feld) | 3 Sites | 200 | 2026-09-13 |
| `https://zenodo.org/api/records/10594301` (Alberta 2023) | JSON/netCDF | 5 Stationen | 000 (Timeout) | 2026-09-13 |
| `https://www.pangaea.de/advanced/search.php?q=…&format=json` | JSON | — | 200 (kein GIC-Treffer) | 2026-09-13 |

**Verdikt:** BPA = **live** (11 Substationen, 5-Minuten-Werte, aber nur
4-Tage-Rollfenster, Stationsnamen ohne lat/lon). BGS B_GIC = **declined**
(Modell aus 3 Observatorien + Konduktanzmodell); BGS geoelectric =
**declined** (gemessenes E-Feld, Plots, kein GIC-Kanal). Alberta/Zenodo =
**pending** (lokal 000). PANGAEA: kein Fund. Die Kombination
**kontinuierlich + Stationskoordinaten** bleibt nach Suche
**not-published**.

**note:** `fmi_gic.bin` ist bereits gebaut (`sources.φ` Z. 5904);
`space.fmi.fi/MAGN/spaceweather/` = 404 (falscher Pfad). Alberta 2023
(DOI 10.5281/zenodo.10594301) ist belegt, aber Zenodo lokal nicht messbar.

## 6. Blitz-Bodennetze (electric) — WWLLN

**Was die Messung IST:** VLF-Sferics von Blitzentladungen; die
WWLLN-Thunder-Hour ist monatlich summierte Gewitterstunden pro Gitterzelle —
ein Klimatologie-Gitter, kein Ereignis-Feed.

| Route | Format | Position | HTTP | Datum |
|---|---|---|---|---|
| `https://wwlln.net/climate/th_yr/data/WWLLN_th_2025.nc.zip` | netCDF-ZIP | Gitter 0,05° | 200 (60 836 418 B) | 2026-09-13 |
| `https://wwlln.net/climate/th_yr/data/WWLLN_th_2024.nc.zip` | netCDF-ZIP | Gitter | 200 (28 574 993 B) | 2026-09-13 |
| `https://wwlln.net/climate/th_yr/data/` | Apache-Index | — | 200 | 2026-09-13 |
| `https://wwlln.net/climate/th_yr/data/readme.pdf` | PDF | — | 200 | 2026-09-13 |
| `https://lightning.nsstc.nasa.gov/data/` (LIS/OTD) | HTML | — | 200 | 2026-09-13 |
| `https://ghrc.earthdata.nasa.gov/` | HTML | — | 200 | 2026-09-13 |
| `https://noaa-goes16.s3.amazonaws.com/?list-type=2&prefix=GLM-L2-LCFA/` | S3-Liste | Gitter | 200 | 2026-09-13 |
| `http://wwlln.net/cgi-bin/output.cgi?format=json&type=stations&limit=100` | JSON | — | dead | — |

**Verdikt:** WWLLN Thunder-Hour = **live** (2005–2025, 21 Jahre, netCDF,
0,05° lat/lon, monatlich; Variablen `lat`, `lon`,
`thunder_hours(nmon,nlat,nlon)` [Stunden], 15-km-Radius). LIS/OTD =
**blocked account** (GHRC-DAAC, freie Earthdata-Registrierung). WWLLN
Echtzeit-Roh/Stations-Feed = **not-published**.

**note:** Die Thunder-Hour ist ein eigenständiger electric-Kanal, aber
aggregierte Klimatologie. GLM (GOES-16 S3) ist bereits in-register. Der
WWLLN-netcdf-Compiler + CDN-Manifestation sind noch offen (Compiler-Gerüst
mit `netcdf.rs` in `src/archivar/netcdf.rs`).

## 7. SuperDARN-Polarradar (electric)

**Was die Messung IST:** HF-Radar-Rückstreuung an ionosphärischen
Plasmaunregelmäßigkeiten → Konvektionsgeschwindigkeit; RAWACF/FITACF sind
Radar-Echo-Parameter.

| Route | Format | Position | HTTP | Datum |
|---|---|---|---|---|
| `https://superdarn.ca/radar-info` | HTML-Tabelle | ja (Radar lat/lon) | 200 | 2026-09-13 |
| `https://superdarn.ca/` | HTML | — | 200 | 2026-09-13 |
| `https://vt.superdarn.org/` | HTML | — | 200 | 2026-09-13 |
| `https://sdc-serv.usask.ca/data-access` (Globus) | HTML | — | 200 | 2026-09-13 |
| `https://www.frdr-dfdr.ca/repo/collection/superdarn` (RAWACF 2007–2023) | DOI-Datensätze | — | 200 | 2026-09-13 |
| `https://data.superdarn.ca/` | — | — | 000 (kein A-Record) | 2026-09-13 |

**Verdikt:** Radar-Positionen = **live** (HTML-Tabelle, u. a. Goose Bay,
Kapuskasing, Prince George, Rankin Inlet, Saskatoon). RAWACF-Jahresdatensätze
= **live** via FRDR (offen, DOI-basiert, DMap). FITACF/Grid-DMap via Globus =
**blocked account** (freie Registrierung, Account). Konvektions-PNG =
**declined** (positionslos). `data.superdarn.ca` = **not-published**.

**note:** `superdarn_fitacf.bin` ist bereits gebaut. Offene Route ist RAWACF
über FRDR; Radar-Positionen separat offen als HTML-Tabelle.

## 8. HF-Radar-Oberflächenströme (advective)

**Was die Messung IST:** Oberflächenströmungsgeschwindigkeit in cm/s aus
HF-Radar-Bragg-Rückstreuung.

| Route | Format | Position | HTTP | Datum |
|---|---|---|---|---|
| `https://hfradar.ioos.us/erddap/tabledap/allDatasets.json?datasetID,title,minTime,maxTime,timeSpacing` | JSON | — | 200 (204 Datensätze) | 2026-09-13 |
| `https://hfradar.ioos.us/erddap/tabledap/BML_PBON.json` | JSON | ja (`antenna_lat`,`antenna_lon`) | 200 | 2026-09-13 |
| `https://hfradar.ioos.us/radials-erddap/erddap/index.json` | JSON | — | 200 | 2026-09-13 |
| `https://hfrnet-tds.ucsd.edu/` | THREDDS/Gitter | — | 000 (Timeout) | 2026-09-13 |
| `https://erddap.emodnet-physics.eu/erddap/info/HFRADAR_NADR_Totals/index.json` (EU) | JSON | — | 200 | 2026-09-13 |
| `https://erddap.osupytheas.fr/erddap/info/HFRADAR_grid_copernicus/index.json` (EU) | JSON | — | 200 | 2026-09-13 |
| `https://www.hfrnode.eu/` | HTML | — | 200 | 2026-09-13 |
| `https://ceotr.ocean.dal.ca/erddap/info/codar_totals_2015/index.json` (Kanada) | JSON | — | 200 | 2026-09-13 |

**Verdikt:** US-Radials = **live** (anonym, stündlich bis 2026-09-13, jeder
Datensatz führt `station`, `antenna_lat`, `antenna_lon`). Nicht-US-Netze
(EMODnet, Ifremer-Copernicus, hfrnode, Kanada) = **live**. Das gridded
RTV-Produkt auf `hfrnet-tds.ucsd.edu` = **pending** (Timeout, DNS lebt).

**note:** `phi/pipeline/frame_registry.φ` registriert 204
`hfradar.ioos.us/erddap/tabledap/<ID>.json`-Radials. Die Radar-Sites tragen
Koordinaten; nur das Gitter-ThREDDS ist offen.

## 9. BGC-Argo (diffusion)

**Was die Messung IST:** gelöstes O₂, Nitrat, pH (in-situ), Chlorophyll,
partikuläres Backscatter — je Argo-Profil, an der Profilposition (lat/lon),
als Tiefenprofil. Einheiten µmol/kg, mg/m³, m⁻¹, dimensionslos (pH). Die fünf
Variablen stehen schon im Register (DOXY, NITRATE, CHLA, BBP700,
PH_IN_SITU_TOTAL).

| Route | Format | Position | HTTP | Datum |
|---|---|---|---|---|
| `https://data-argo.ifremer.fr/argo_bio-profile_index.txt.gz` | gzip-CSV | ja (lat/lon je Profil) | 200 | 2026-09-13 |
| `https://argovis-api.colorado.edu/argo?…&data=doxy,nitrate,ph_in_situ_total,chla` | JSON | ja | 200 | 2026-09-13 |
| `https://argovis-api.colorado.edu/argo/vocabulary?parameter=data` | JSON | — | 200 | 2026-09-13 |
| `https://erddap.ifremer.fr/erddap/index.json` | JSON | — | 200 | 2026-09-13 |
| `https://polarwatch.noaa.gov/erddap/index.json` | JSON | — | 200 (0 Argo-Treffer) | 2026-09-13 |

**Verdikt:** offizieller BGC-Argo-Index = **live** (Vollkatalog, Spalten
`file,date,latitude,longitude,ocean,profiler_type,institution,parameters,
parameter_data_mode,date_update`, Format v2.2, tagesaktuell). Argovis =
**live** (zweiter offener BGC-Kanal; Schlüssel `doxy, nitrate,
ph_in_situ_total, chla, bbp470/532/700, cdom, turbidity`). Ifremer-ERDDAP =
**live** für Kern-Argo; BGC nur als Synthetik/Gitter = **declined**.
PolarWatch = **declined** (kein BGC-Argo-Kanal).

**note:** `argo_bgc.bin` ist bereits gebaut. Restfrage: ob `/bgcargoplus`
eigene Coverage jenseits `/argo` trägt (Box leer gemessen, `pending`).

## 10. Einzelteleskope ohne offenen Weg (em, S²-Zeuge)

**Was die Messung IST:** VHE-/UHE-γ- und Neutrino-Beobachtungen, als
Quell-/Ereigniskataloge mit Position (RA/Dec) und möglichst Energie.
Teilchen-Abstammung = Photon (`em`), Wurzel im Abstammungs-Feld.

### HAWC (2HWC/3HWC)
- Route: `https://data.hawc-observatory.org/datasets/2hwc-survey/2HWC.yaml`
  und `…/3hwc-survey/3HWC.yaml` — YAML, Position ja (RA/Dec in Grad),
  Flux + Spektralindex. `curl -k` **200** (2HWC 18 588 B), 2026-09-13.
- Verdikt: **live** (offener Quellkatalog; zusätzlich FITS-Healpix-Skymaps
  unter `…/3hwc-survey/fitsmaps.php`). Vorbehalt: unvollständige TLS-Kette
  (`-k` nötig → TLS-Gap für den Archivar-Fetch).
- note: Der Stand „HAWC not-published" ist **veraltet**.

### LHAASO (1LHAASO)
- Route: `https://casdc.china-vo.org/archive/LHAASO-Gamma-Ray-sources/table.csv`
  — CSV, **200** (16 456 B), Spalten u. a. `Source name, Ra, Dec, TS, index`;
  Landeseite `nadc.china-vo.org/res/r100752/` 200 (DOI 10.12149/100752).
- Verdikt: **live** (offizieller Katalog; kein per-Event-Format).
- note: Der Stand „LHAASO declined" ist **veraltet** (verwies auf eine
  News-Seite statt das NADC/CASDC-Archiv).

### Telescope Array (TA)
- Route: `https://zenodo.org/records/8427755` (DOI 10.5281/zenodo.8427755,
  Amaterasu-Einzelereignis). HTTP heute **504** (Zenodo standortweit down);
  `telescopearray.org` 200 (Info-Site, kein Datenportal).
- Verdikt: **pending** (Route bekannt, HTTP 200 heute unmessbar).
  Vollkatalog = **not-published** (kein Auger-artiges Open-Data-Portal).
- note: Nach Zenodo-Wiederkehr die 504→200-Messung nachholen.

### Super-Kamiokande
- Route: `https://www-sk.icrr.u-tokyo.ac.jp/sk/lowe/` — **200**
  (Solar-ν-Rate/Energiespektren). Kein per-Event-Format; Richtung = Sonne.
- Verdikt: **declined** (position-only/Einzelrichtung, kein S²-Katalog).

### JUNO
- Route: keine offene Datenfreigabe gefunden; Publikationsliste nur Referenz.
- Verdikt: **not-published** (kein offener maschinenlesbarer Weg; Reaktor-ν
  trägt ohnehin keine per-Event-S²-Richtung).

## 11. Broker-Punktpositionen der Himmelswarnungen (em, S²-Zeuge)

**Was die Messung IST:** Position (ra/dec) und em-Photometrie transienter
Himmelsobjekte; Distanz meist absent (distanzloser S²-Richtungs-Zeuge).

| Broker | Route | Position | Photometrie | Distanz | HTTP | Datum |
|---|---|---|---|---|---|---|
| ANTARES | `https://api.antares.noirlab.edu/v1/loci?page[limit]=10&page[offset]=0` | ja | ja (unbanded) | nein | 200 | 2026-09-13 |
| Fink/LSST | `https://api.lsst.fink-portal.org/api/v1/conesearch` | ja | nein | nein | 000 (Host hängt) | 2026-09-13 |
| Lasair-ZTF | `https://lasair-ztf.lsst.ac.uk/api/query/` | ja | ja (gmag) | nein | 401 (Token) | 2026-09-13 |
| ALeRCE | `https://api.alerce.online/alerts/v1/objects/` | ja | ja | nein | 000 (Host hängt) | 2026-09-13 |
| TNS | `https://www.wis-tns.org/system/files/tns_public_objects/tns_public_objects.csv.zip` | ja | ja | nein | 403 (UA) | 2026-09-13 |
| Gaia Alerts | `https://gsaweb.ast.cam.ac.uk/alerts` | — | — | — | 200 (HTML-Portal) | 2026-09-13 |

**Verdikt:** ANTARES = **live** (anonym, JSON:API, Pagination
`page[limit]`/`page[offset]`, Bestand 10 000 Loci, Mag-Band absent).
Lasair = **blocked key** (anonym 401, Token in `.secrets.local`). TNS =
**blocked** (User-Agent-Gate, Bulk-CSV). Fink + ALeRCE = **pending** (Host
lebt, API antwortet heute nicht — TLS-Handshake, dann 0 Bytes). Gaia Alerts
= **declined** als maschinenlesbare Quelle (HTML-Portal, Login).

**note:** Fink-Konus = position-only (keine Magnitude); die
Per-Objekt-Lichtkurven (`/api/v1/sources`, `/api/v1/fp`) bleiben `pending`.
ANTARES ist bereits in `phi/witnesses.φ:11`, Lasair in `:41`, Fink in `:17`.

## Korrekturen gegen die Faden-Matrix (2026-09-07)

- **HAWC** ist nicht `not-published` — der 2HWC/3HWC-Katalog liegt offen als
  YAML auf `data.hawc-observatory.org` (mit TLS-Ketten-Vorbehalt).
- **LHAASO** ist nicht `declined` — der 1LHAASO-Katalog liegt offen als CSV
  auf `casdc.china-vo.org`.
- **WWLLN** ist live erreichbar (netCDF 2005–2025); es fehlt nur der
  Compiler, nicht die Quelle.
- **GIC** hat mit BPA einen offen gemessenen Kanal (TSV, 5-min) — die
  Kombination kontinuierlich + Stationskoordinaten bleibt `not-published`.
- **RaspberryShake** läuft am korrigierten Host `data.raspberryshake.org`
  (der Auftragshost `fdsnws.raspberryshakedata.com` ist DNS-tot).
- **BGC-Argo** ist durch den offenen `argo_bio-profile_index` vollständig
  adressiert; Argovis ist ein zweiter Kanal.

## Gesamtbild

Drei Klassen haben eine offene, maschinenlesbare Route, deren roher
Zwillingskanal account-/key-blocked ist (IGETS-Stationsliste vs. SFTP;
BGR-Infraschall vs. vDEC; NOAA-NRS vs. ONC-Token). Vier Klassen sind
vollständig live (seismische FDSN-Stationen, HF-Radar-Radials, BGC-Argo,
ANTARES). Vier Klassen haben eine offene, aber noch nicht gebaute Route
(WWLLN-netcdf, BPA-GIC, HAWC, LHAASO). Zwei Klassen bleiben ohne offenen
Punkt-Kanal (GIC kontinuierlich + Stationskoordinaten; TA-Vollkatalog). Kein
Wert ist fabriziert; jeder ungemessene Punkt ist als `pending` benannt.
