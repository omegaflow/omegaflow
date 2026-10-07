<!--
  title: Survey — Störungs-Experiment: fehlende Fäden (2026-10-05)
  class: survey
  date: 2026-10-05
  sha256: b58af0d7fce8716d5aea24a202383d21ff3db867d15cfbf23f49c02765976edb
  status: live
  see-also: docs/concepts/tools-map.md phi/sources.φ docs/handover/archiv/handover-2026-10-05-river-folge93.md
-->
# Survey — Störungs-Experiment: fehlende Fäden (2026-10-05)

Operator-Vision (2026-10-05): *„An jedem Punkt einen Stein ins Wasser fallen lassen und
schauen, was passiert."* Das gekoppelte System als **Messung** — ein Kanal wird gestört,
das ganze Feld antwortet; `field_te_query` ist der Stein. Diese Auswertung trägt den
gemessenen Stand der Fäden und die verifizierten Beschaffungsrouten. Fünf flash-Taucher
(`grind-flash`/`general`, gemessen am Register + `archive_search --verdict`) und drei
UI-Stimmen (z.ai GLM-5.3 Deep Think Max, claude.ai Sonnet 5.5 Extra hoch, Kimi K3 via
`tryingopen.com`) liegen zugrunde; die API-Flotte (`zai glm-4.5-flash`, `gemini-2.5-flash`)
als Breite. **Eine Stimme ist nie eine Messung** — jede Tabellenzeile trägt den
Messwert der Taucher; Stimmen-Aussagen ohne Baum-Beleg sind als solche markiert.
Der weitere Rahmen — **alles gegen alles** im baryzentrischen ICRS, Lücken als
Zielangaben, Triangulation, `ozzy` — liegt im Auftrag
`docs/auftrag/auftrag-universelles-vlies.md` (Operator-Vision 2026-10-05).

## 1 Der gemessene Stand der Verdrahtung (heute)

Aus der verifizierten Drei-Zustands-Liste (`field_te_query`; `phi/pipeline/descriptors/*.te`
+ `field_te_query.rs`):

- **Am Draht (7):** GOES XRS · AIA2013 · ERBQ-Event · ERSST→NINO3.4 · TAO-Wind · OMNI2 (Bz) · ABK `dbdt`.
- **Probe-gelesen, nicht am Draht (8):** RTSW/SWPC · EVE 1032/131 · QBO `qbo_30hpa` · D20
  `d20_thermocline` · Kp `magnetosphere_kp_3h` · Swarm HAPI · EEG ds007822/ds007471 · Newell `dΦ/dt`.
- **Declined/descoped (5):** SOI · NSRR · TUH · JUICE-Doppler · WWP.

Die Weberin trägt die **globalen Netze** (INTERMAGNET 154, SuperDARN ~35, IU/GE/AM-FDSN,
Argo, WWLLN) und die großen Ephemeriden (DE440/441/442, INPOP19a, EPM2021). Regionale
Cluster (BGR-Infraschall, BPA/FMI-GIC) sind **Beispiele**, nicht das globale Feld.

## 2 Die fehlenden Fäden — verifizierte Route

Taucher-gemessen am 2026-10-05 (`archive_search --verdict/--sniff`, Register via `sgrep`).

| Netz | Register | Endpunkt | Zugang | Format | Verdikt |
|---|---|---|---|---|---|
| Infraschall CTBTO IMS/vDEC | `declined_sources.φ:4480` (blocked) | `vdec.ctbto.org` 404; Antrag via `ctbto.org/.../vdec` | Antrag, Wochen | IMS/miniSEED, `maw` 30 min | offene Produkte: BGR PMCC (`gdk.gdi-de.org`, CC BY 4.0) + IRIS IM-Metadaten (`service.iris.edu`) |
| Infraschall BGR-Produkte | `sources.φ:9315` (`bgr_infrasound.bin`) | `download.bgr.de/...product.zip` (403 direkte) | frei (Proxy-Route) | netCDF, 30 min | gebaut; Origin-Route messen |
| CEEIN C9 (IAP Prag) | `blocked_sources.φ:171` (pending) | `ceein.infp.ro/fdsnws/` 200 | frei | FDSN miniSEED (bis ~2023-10-30) | Mycelium-Arm |
| GNSS-TEC global (Madrigal) | nein (nur `re3data`) | `cedar.openmadrigal.org` 206 | frei (Name/E-Mail/Affil.) | HDF5/ASCII, 5 min, 1°×1° | Kandidat, Quellenblock fehlt |
| GNSS-TEC (IGN/BKG, ohne Earthdata) | nein | `igs.bkg.bund.de/root_ftp/`; `igs.ign.fr` | frei (BKG HTTPS) | IONEX | Kandidat |
| IGS IONEX (CDDIS) | `sources.φ:1295` | `cddis.nasa.gov/archive/gnss/products/ionex/` 200 | Key `{EARTHDATA_EDL_TOKEN}` | IONEX `tec` | gebaut (Token-Präsenz pending) |
| Pegel UHSLC | nein (nur `library.φ` Host) | `uhslc.soest.hawaii.edu/erddap/tabledap/global_hourly_fast` | frei | ERDDAP CSV/NetCDF | Kandidat (Fast-Delivery-API) |
| Pegel PSMSL | `dead_sources.φ:940` (`map.json`) | `psmsl.org/data/obtaining/complete.php` | frei | ZIP, RLR-Monat | tot re-checken; Monat für TE ungeeignet |
| Pegel IOC-SLSM (VLIZ) | nein | `api.ioc-sealevelmonitoring.org/v1/doc` | v1 frei, v2 Key | JSON/CSV, 1 min | Kandidat (1-min global) |
| Pegel DART | `sources.φ:461` | `ndbc.noaa.gov/data/realtime2/<ID>.dart` | frei | ASCII, 15 min/1 min/15 s | gebaut |
| GIC Kanada NRCan | nein | `spaceweather.gc.ca` (Magnetik) | frei (Magnetik); GIC nicht offen | IAGA-2002 | GIC `blocked account`; Magnetik über INTERMAGNET |
| GIC Schweden | nein | `space.fmi.fi/MAGN/GIC/` | frei (FMI) | ASCII | nordische GIC-DB = Erweiterung von `fmi_gic` |
| GIC Neuseeland | nein | `gns.cri.nz` (Eyrewell via INTERMAGNET) | frei (Magnetik) | IAGA-2002 | Transpower-GIC restricted |
| GIC Russland IZMIRAN | nein | `izmiran.ru/data`, `mag.gcras.ru` | frei (direct) | IAGA-2002 | MOS-Observatorium messen |
| Seismik EIDA/ORFEUS | nein | `orfeus-eu.org/eidaws/routing/1/query` 200 | frei | StationXML/miniSEED | Kandidat (Routing), Nodes einzeln |
| Seismik CEIC China (CB) | nein | EarthScope `fdsnws/station?network=CB` 200 | über EarthScope offen | StationXML | dataselect CB prüfen |
| Seismik NIED (BO/Hi-net) | `dead_sources.φ:884` (BO tot) | `fnet.bosai.go.jp` — FDSN 404; `mowlas.bosai.go.jp` | Registrierung | SAC/WIN | kein FDSN; NIED-Weg messen |
| Seismik GSRAS Russland | nein | `gsras.ru` (direkt kein Response; EarthScope-GSN-Stationen OBN/ARU/YAK) | GSN frei | miniSEED | GSN-Arm deckt ab |
| Gravimetrie EGGN | nein | EQUIP-G `epos-eu.org/equip-g` | pending | pending | Nachfolger messen |
| Gravimetrie China CGrav | nein | `data.earthquake.cn` | restricted (Antrag) | nationales Netz | Antrag = Dritter |
| Gravimetrie IGETS/BGI | `sources.φ:9528` / `blocked:166` | `isdc.gfz.de` / `api.sedoo.fr/get-agrav-rest/` | sftp-Account / frei | ASCII/JSON | IGETS gebaut; BGI Mycelium-Arm |
| Wetter ERA5 / GFS | descoped (`declined:1279`, `noaa_nodd_disposition:254`) | `cds.climate.copernicus.eu` / `nomads.ncep.noaa.gov` | Key / frei | GRIB/NetCDF | **Modellprodukt** — kein Messfaden (§3) |
| Blitze GLM (GOES) | `sources.φ:9732` (nur 18) | `noaa-goes{16,18,19}.s3.amazonaws.com/GLM-L2-LCFA/` | frei (S3) | NetCDF, ~20 s | 16/17/19 ergänzen |
| Blitze MTG-LI (Europa) | nein | `data.eumetsat.int` / `api.eumetsat.int` | EUMETSAT-Account | NetCDF, 10 min; L2-ID **`EO:EUM:DAT:0691`** = „LI Lightning Flashes" (daneben `0687` = Accumulated Flash Area; gemessen 2026-10-05) | Kandidat |
| Blitz Asien/Pazifik | nein | keine offene Quelle gemessen | — | — | benannte Lücke |
| Aerosol/Radiosonden IGRA2 | nein | `ncei.noaa.gov/products/.../integrated-global-radiosonde-archive` | frei | CSV/NetCDF | Bonus, in-situ |

## 3 Ranking — Zugang × Nutzen (Auswertung)

1. **Sofort ohne Key (Stunden):** IOC-SLSM (1 min global) · Madrigal-TEC · UHSLC-Fast ·
   GFS/AWS · CODE/IGN/BKG-IONEX · EIDA-Routing · GLM S3 (GOES-16/18/19) · EarthScope-FDSN
   (deckt russische GSN-Stationen).
2. **Account/Kostenpflicht (Tage–Wochen):** vDEC (längster Vorlauf) · NIED/mowlas ·
   EUMETSAT-MTG-LI · CDDIS (Earthdata) · SuperMAG · BGI AGrav · ChinArray.
3. **Nicht offen (benannte Lücke):** GIC-Rohdaten Kanada/Schweden/NZ (Ersatz: FMI +
   SuperMAG/INTERMAGNET + dH/dt-Proxy) · CTBTO-Rohdaten ohne Antrag · permanentes
   CN-Seismiknetz · offene Blitznetze Asien/Pazifik.

**Korrektur zur Ausgangs-Tafel:** ERA5/GFS sind **Reanalyse/Forecast-Modellprodukte**, kein
Messfaden — bereits `descoped` (`declined_sources.φ:1279`); für „gemessen statt modelliert"
tragen Stationsdruck/Radiosonden (IGRA2) die Atmosphäre. „Open-Meteo = 3 Stationen" ist
Doku-Behauptung; das Register trägt ein Template + viele declined-Endpunkte.

## 4 Der Stein und die Kopplung

Der Stein ist `field_te_query` mit **einer Quelle, vielen Zielen**: die Paar-Matrix misst,
welche Kanäle antworten und mit welcher Verzögerung. Die Takte der fehlenden Fäden
(15 s DART · 20 s GLM · 1 min IOC/IGETS · 10 min LI · 30 min BGR · 1 h ERA5) müssen vor
einer TE auf eine gemeinsame Auflösung und Lag-Fenster gebracht werden. Abgeleitete
Kanäle (SWPC-E-Feld = deterministische Funktion von B, GIM, ERA5) erzeugen **künstliche
Kopplung** und sind als Stör-Treiber, nie als unabhängiger Zeuge zu behandeln.

## 5 Nächste Schritte

- **Mountain (Quellen-Zeilen):** UHSLC, IOC-SLSM, Madrigal, EIDA-Routing, MTG-LI als
  neue `sources.φ`-Kandidaten (je Register-Zeile + Format + Lizenz); `:7088` NED-z auf
  Kernel 6 (inverse-linear) — siehe `An mountain` der River-Übergabe.
- **Mycelium (Ernte/Transport):** BGI AGrav + CEEIN C9 + nordische GIC-DB manifestieren;
  GOES-18-Bucket um 16/19 erweitern.
- **River (Stein):** die 8 probe-gelesenen Kanäle in die `field_te_query`-Matrix heben;
  die Matrix als Störungs-Experiment betreiben.

## 6 Quellen / Stimmen (Rohspuren)

- Taucher A–E (flash, gemessen 2026-10-05): Register + `archive_search --verdict/--sniff`.
- UI z.ai GLM-5.3 Deep Think Max, claude.ai Sonnet 5.5 Extra hoch: vollständige Tabellen +
  Ranking; **Claude ohne Live-Requests** („aus dem Gedächtnis"/„ungeprüft" markiert).
- GLM-Nachfrage „mit echter websuche" = **gemessene Nicht-Suche:** GLM-5.3 erklärt in diesem
  Chat, es habe **kein Web-Tool** und verweigert simulierte Suchergebnisse; die Nachlieferung
  ist ungeprüftes Trainingswissen mit ✅/⚠️/▲-Marken. Ihre neuen Endpunkt-Hinweise wurden
  live verifiziert (2026-10-05, `archive_search --verdict`): INTERMAGNET-GitHub lebt
  (neben `imag-data.bgs.ac.uk`), GFZ `isdc` + `dataservices` beide 200, CDDIS-IONEX 200,
  EUMETSAT LI-L2 = `EO:EUM:DAT:0687`. **Widerlegt:** `registry.opendata.aws/era5/` = HTTP 404
  (kein key-freier ERA5-Datensatz dort); CODE/AIUB `ftp`+`https` kein Response (pending/tot).
- GLM dritte Runde (mit Zugriff auf Live-Tools, korrigierte Fassung) — ihre Korrekturen
  wurden stichprobenweise verifiziert (2026-10-05, `archive_search --verdict`):
  **bestätigt:** EarthScope-FDSN = `service.earthscope.org` (`.edu` kein Response) ·
  EUMETSAT LI-L2 = `EO:EUM:DAT:0691` („LI Lightning Flashes"; `0687` = Acc. Flash Area) ·
  ICGEM `icgem.gfz.de` lebt (identisch zu `icgem.gfz-potsdam.de`) · P2PQuake-API lebt.
  **widerlegt:** „DART über `/data/realtime2/` = 404" — `realtime2/21414.dart` liefert
  200 (282968 B); richtig ist nur, dass `/dart.shtml` 404 ist und die Landeseite
  `/dart/dart.shtml` heißt. GOES-AWS-**Registry-Seite** `registry.opendata.aws/noaa-goes18/`
  = 404; der Bucket `noaa-goes18.s3.amazonaws.com/GLM-L2-LCFA/` ist der Endpunkt.
- Kimi K3 via `tryingopen.com`: **`This site has run out of API credit`** — kein Ergebnis.
- API-Flotte: `zai glm-4.5-flash` + `gemini-2.5-flash` (Breite, halluzinationsanfällig:
  erfundene GeoNet-/NRCan-APIs — am Baum widerlegt).
- Rohantworten privat: `state/stimmen/2026-10-05_*`.
