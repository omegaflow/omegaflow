<!--
  title: Verdrahtung: Sonnenwind, Aerosol/Thermik, SuperDARN, CME/LASCO
  class: survey
  date: 2026-10-20
  sha256: 530818cb8858ea941d7cfbe47cd83c383307efb818c8af4f21c925de603e4056
  status: live
  see-also: docs/surveys/survey-2026-10-20-live-sonnenwind.md docs/surveys/survey-2026-10-20-corona-kette.md
-->
# Verdrahtung: Sonnenwind, Aerosol/Thermik, SuperDARN, CME/LASCO

Vier Fäden wurden am 2026-10-20 gegen das Register und die Endpunkte geprüft.
Drei Blöcke lagen schon im Register; neu sind die Herkunft des S5P-Listings,
das Geschwindigkeitsfeld am SuperDARN-Block und die Tagesernte am
LASCO-Block. Der RTSW-Block bleibt unverändert live. Kein CDN-Upload.

## 1. Live-Sonnenwind (RTSW/IMAP)

| | |
|---|---|
| Endpunkt | `https://services.swpc.noaa.gov/json/rtsw/rtsw_wind_1m.json`, `…/rtsw_mag_1m.json`, `…/planetary_k_index_1m.json` |
| Messung 2026-10-20 15:47Z | Wind **200**, 2.565.014 B, 3.473 Zeilen; Mag **200**, 1.514.175 B, 3.595 Zeilen; Kp **200**, 28.003 B, 358 Zeilen |
| Quellen im Wind-Fenster | IMAP 1.161, ACE 1.226, SOLAR1 1.086; DSCOVR **0** |
| Erste Zeile | `time_tag 2026-10-20T15:43:03`, `source IMAP`, `proton_speed 367.03`, `proton_density 9.23` |
| Letzte IMAP-Zeile | `2026-10-20T15:45:03`, `proton_speed 362.88`, `proton_density 9.22` |
| Felder | `proton_speed`, `proton_density` (Wind); `bt`, `bz_gsm` (Mag); `kp_index`, `estimated_kp`, `kp` (Kp) |
| Kadenz | 1 min; Block `ttl 60` |
| Lizenz | PD-USGov, `https://www.swpc.noaa.gov/content/disclaimer` |
| Status | **aktiv** — Block `phi/sources.φ:32250`, Compiler `swpc_rtsw_compiler` |

Der Block war bereits aktiv und ist nicht dupliziert worden. Er paart Wind und
Mag über dieselbe UTC-Minute und dieselbe `source`; die Sonde bleibt damit
Datum, nicht Annahme. DSCOVR ist im aktuellen Ein-Tages-Fenster nicht mehr
enthalten; HAPI `DSCOVR_H0_MAG` endet am 2026-09-13. Eine Aktivierung des
DSCOVR-Archivs als Live-Feed wäre falsch.

## 2. Aerosol/Thermik

| | |
|---|---|
| Endpunkt | `https://meeo-s5p.s3.amazonaws.com/?list-type=2&prefix=OFFL/L2__O3_TCL/&max-keys=100` |
| Messung 2026-10-20 | **200**, 31.587 B, `KeyCount 100`, `IsTruncated true`; erster Schlüssel `OFFL/L2__O3_TCL/2018/04/30/S5P_OFFL_L2__O3_TCL_20180430T105026_…nc` |
| Felder | `xmlcount Contents` → `s5p_o3_tcl_objects` (Objektzahl, kein Aerosolwert) |
| Kadenz | Listing auf Abruf; keine Messkadenz |
| Lizenz | Copernicus, `https://sentinels.copernicus.eu/documents/247904/690755/Sentinel_Data_Legal_Notice` |
| Status | **Vorschlag** — `ttl 0`, `status vorschlag`; Herkunft `origin` ergänzt |

Der Endpunkt liefert ein S3-Listing, keine Ozon- oder Aerosolgröße. Deshalb
bleibt der Block inaktiv: `xmlcount` ist ehrlich, aber kein Messwert. Die
FIRMS-CSV-Blöcke (`phi/sources.φ:803`, `:814`, `:825`; gespiegelt `:14376`)
lagen bereits vor und wurden nicht dupliziert.

## 3. SuperDARN

| | |
|---|---|
| Endpunkt (Referenz) | `https://api.bas.ac.uk/superdarn/mirror/v3/` |
| Messung 2026-10-20 | **200**, 3.149 B JSON, 55 `api_routes`; keine Messfelder |
| Endpunkt (Daten) | `https://superdarn.ca/`, Asset `superdarn.ca/superdarn_fitacf.bin` |
| Felder | `superdarn_fitacf_los_velocity_ms` (Losgeschwindigkeit) |
| Kadenz | 600 s; Block `ttl 604800` |
| Lizenz | `other-attribution`, `https://superdarn.ca/data-access`; BAS-Referenz OGL-UK-3.0 |
| Status | **aktiv** — Feld am Block `phi/sources.φ:11396` ergänzt; BAS-Block bleibt Referenz |

Im Register lagen bereits fünf SuperDARN-Blöcke: BAS (nur Referenz, keine
Felder), `superdarn.ca` (Format ohne Feld), `sdc-serv.usask.ca` (FITACF,
Losgeschwindigkeit 600 s, `sha256` gepinnt), FRDR (RAWACF, Lag-0-Leistung
600 s) und Zenodo (CPCP, Potenzialabfall 86400 s). Der `superdarn.ca`-Block
war der einzige Datenblock ohne Feld und Kadenz; die Zeile ist nachgetragen,
kein zweiter Block gebaut.

## 4. CME/LASCO

| | |
|---|---|
| Endpunkt | `https://cdaw.gsfc.nasa.gov/CME_list/UNIVERSAL_ver2/text_ver/` |
| Messung 2026-10-20 | Verzeichnis **200**, 76.485 B, 364 Monatsdateien; jüngste `univ2026_07.txt` |
| Monatsdatei | `univ2026_07.txt`: **200**, 17.852 B, 146 Records; Kopf `LAST UPDATED: 2026/10/12` |
| Haus-Asset | `soho_lasco_cme.bin`, 193 Records ab 2024-01-01 (rollierend) |
| Felder | Geschwindigkeit, Breite, PA, Beschleunigung, Masse, Energie |
| Kadenz | Ereignis; Monatsdatei, Ernte `ttl 86400` statt 2.592.000 |
| Lizenz | PD-USGov, `https://sti.nasa.gov/disclaimers/` |
| Status | **aktiv** — `ttl` am Block `phi/sources.φ:11590` auf 86400 gesetzt |

Die jüngste Monatsdatei ist Juli 2026; August bis Oktober fehlen im Verzeichnis,
obwohl der Kopf der Juli-Datei den 2026-10-12 nennt. Die Tagesernte liest nur
die jüngste Monatsdatei, nicht die 364 Dateien. Das Haus-Asset bleibt die
rollierende 2024+-Linie; die Monatsdateien sind die feinere Linie für einzelne
Monate, ohne Massenladung.

## Bilanz

| | Zahl |
|---|---:|
| geprüft | **4** |
| aktiv | **2** (RTSW, LASCO) |
| Vorschlag | **1** (S5P) |
| Referenz ergänzt | **1** (SuperDARN) |
| Duplikate vermieden | **2** (FIRMS, RTSW) |

## Lizenzen

- RTSW: PD-USGov — klar.
- S5P: Copernicus Sentinel Data Legal Notice — klar, Namensnennung.
- SuperDARN: `other-attribution` für `superdarn.ca`; BAS-Referenz OGL-UK-3.0.
  Eine SPDX-Kennung führt die Quelle nicht; der Block trägt den Namen.
- LASCO: PD-USGov — klar.

## Was quer liegt

- Der DSCOVR-Block am Dateianfang der RTSW-Feeds ist Archiv, kein Live-Kanal;
  im aktuellen Wind-Fenster ist DSCOVR gar nicht mehr enthalten.
- Das S5P-Listing belegt Erreichbarkeit, nicht Aerosol. Ein Messblock müsste
  die NetCDF-Datei lesen; das ist hier nicht geschehen.
- LASCO hat im Verzeichnis eine Lücke August–Oktober 2026; die Tagesernte
  erkennt den nächsten Monat, sie füllt die Lücke nicht.
