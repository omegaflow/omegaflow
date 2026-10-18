<!--
  title: Survey — Härtefälle: offene Routen (Sonden · Redistributions-Declines · Gated China · C/NOFS-NWM · MODVOLC)
  class: survey
  date: 2026-10-17
  sha256: e55a329e09115bb050f443e154596022af986618090ab12c9a113f96b856e23c
  status: live
  see-also: docs/auftrag/archiv/auftrag-sonden-rohdaten-anfrage.md docs/surveys/survey-2026-10-09-redistribution-alternativen.md docs/handover/handover-2026-10-14-sensory-folge259.md
-->
# Survey — Härtefälle: offene Routen (2026-10-17)

**Anlass (Operator-Wort):** die wirklich schweren Quellen — die mit **laufenden Anfragen** oder
**Absagen**. Drei `taucher` + zwei eigene Messungen (2026-10-17); Werkzeuge `bin/where`,
`archive_search` (`--nssdc`, `--heasarc`, `--openaire`, `--zenodo`, `--verdict` …) und HTTP per
`curl -w`. **Keine Architektur** — die Zulassung ist Mountains Verdikt, CDN/Route Myceliums.

## Versandvorbereitung (2026-10-18) — vorbereitet, Operator-Sendewort nötig

Drei offene Punkte sind als **exakte, sendbare Entwürfe** vorbereitet — **nicht gesendet, nicht
gelöst**. Der Operator hält das Sendewort; jede Antwort ist eine Zulieferung und wird am Baum
gemessen. Artefakt: `state/stimmen/2026-10-18-haertefaelle-anfragen.md`.

| Punkt | Kanal (gemessen) | Stand 2026-10-18 | Artefakt |
|---|---|---|---|
| **RBN** Lizenz/Redistribution | Webformular `reversebeacon.net/contact` (keine E-Mail veröffentlicht) | **vorbereitet, Operator-Sendewort nötig** — Lizenzseiten 5/5 **404**, nur „freely available for study and analysis", **kein** Grant; Callsign-Feld braucht Operator-Wert oder `N/A` | §1 |
| **CEIC D18000** | `datashare@seis.ac.cn`, Cc `dcogcd@163.com` | **vorbereitet, Operator-Sendewort nötig** — DOIs 4/4 **404**, DataCite `prefix=10.12080` = **0**, Order → `login.jsp` | §2 |
| **DOSIS-3D / EuTEF DOSTEL** | `hreda.info@sdc.upm.es` (HREDA registration support) | **vorbereitet, Operator-Sendewort nötig** — TAP `v_doi_investigation`: `data_available=false` für beide | §3 |

Die drei Entwürfe sind **Fragen, keine Anträge mit Zusage**; sie behaupten keine Datenrechte. Die
Antworten sind Zulieferungen — Zulassung/Register bleibt Mountains bzw. Myceliums Hand.

### Alternativrouten (`archive_search --all`, 2026-10-18)

- **RBN:** keyless **CC-BY-4.0** auf Zenodo — `10.5281/zenodo.13362701` (Amateur Radio
  Spot Data 2018-11…2019-05, Monats-TARs RBN/WSPRNet/PSKReporter), `2483022` (HamSCI 2017
  SEQP, aggregiertes RBN-CSV), `5799649`, `1067790`. → **Anfrage nicht nötig** für einen
  Korpus; der volle RBN-Stream bleibt ohne Redistributions-Grant.
- **CEIC D18000:** kein exakter Mirror; aber CEA/CENC-abgeleitete **CC-BY-4.0**-Wellenformen
  auf Zenodo (`23103204` Weiyuan SAC, `13978646` CENC-Katalog+Wellenformen, `10499328`
  CNSN). → Alternativen vorhanden, **nicht** das D18000-Produkt.
- **DOSIS:** keine offene Datenroute in `--all` gefunden (nur Paper/ORCID DLR-Dosimetrie).
  → Anfrage bleibt.

## A — Sonden-Rohdaten (request-only)

| Sonde | offene Route | HTTP | keyless | bleibt |
|---|---|---|---|---|
| **Cassini** closed-loop (TNF/ODF) | `atmos.nmsu.edu/pdsd/archive/data/co-s-rss-1-*/…/odf/` (PDS3, DSN-NAV) | 200 | ja | **offen** — im Haus schon als `cassini_tnf.bin` geerntet |
| **Mariner 10** (762 7-Track-Tapes) | nur Teilroute: `spdf.gsfc.nasa.gov/pub/data/mariner/mariner10/…/red_tele_signal_data_venus_occlt/PSPA-00316_DD029604_05-FEB-74.tar` (19,6 MB) | 200 | ja | Roh-Tapes (`PSCM-00009`) request-only |
| **Voyager 1/2** closed-loop | **Saturn-Encounter offen**: SPDF `pub/data/voyager/voyager{1,2}/radio_science_rss/saturn_encounter_data/` (Univac-1108-Binär; V1 `PSPA-00049` **14 TARs**, V2 `PSPA-00123` **6 TARs**). Cruise/Post-Saturn: keine Route (NSSDC `PSNO-00007` „archive, not distribution") | 206 | ja | **Saturn offen**, Rest request-only |
| **Viking 1/2** Doppler+Range | keine Route (`PSPG-00011` „Data Identified but not Received") | – | – | request-only/absent |
| **Juno** pre-EFB 2013-10-09 | keine Route (Juno-ODF erst post-EFB; EFB = 2013-282) | – | – | request-only |

**Riss (nicht geglättet):** die Fünfer-Liste stammt aus `docs/auftrag/archiv/auftrag-sonden-rohdaten-anfrage.md:27`
(archiviert; „PDS trägt nur open-loop + derived"); der Live-Bestand trägt **Cassini** längst als
`cassini_tnf.bin` (`phi/sources.φ:10160`). Die Aufgabe trägt eine veraltete Prämisse. **Summe: 1/5 offen.**

**Inhaltstrick angewandt (2026-10-17):** Label-Suche (`PRODUCER_ID=DSN`, `ODF`, `ATDF`, `TRK-2-34`,
`Voyager ISS EDR`) über `--openaire`/`--ia-search`/`--heasarc` = **absent**; PDS-Rings
`pds4/bundles/` trägt nur `voyager_rss_raw` (ODR open-loop) neben `cassini_iss/uvis/vims`, `gll.rss`.
**Kamera offen**, **Tracking zu**. **Neu nutzbar:** die **NSSDCA-Metadaten** sind **keyless**
(`archive_search --nssdca <id>`): `PSNO-00007` (Voyager, „Ready for Offline Distribution") ·
`PSCM-00009` (Mariner 10, dito) · `PSPG-00011` / `PSPG-00148` (Viking 1/2 Orbiter, „Data Identified but
not Received") — der Rohbestand bleibt offline, aber der Zustand ist belegbar benannt.

**Kamera-Direktlinks (aufgelöst, alle 200):** Voyager ISS `pds-imaging.jpl.nasa.gov/data/voyager/VGISS_*`
(+ `qedr/`) · Juno JNOJNC `planetarydata.jpl.nasa.gov/img/data/juno/JNOJNC_0001.tar.gz` (**15 425 206 433 B**)
· Mariner 10 CLR ASU `ser.im-ldi.com`.

## B — Redistributions-Declines (41) — die Blockierten, frisch gemessen

| Quelle | legale Route heute | HTTP | Riss |
|---|---|---|---|
| **ALMA** ObsCore | ESO Science Archive (`archive.eso.org/…/eso-data-access-policy.html`) **CC BY 4.0**, Dritt-Distribution erlaubt | 200 | deckt nur **ESO-bediente** ALMA-Records, JAO-Archiv separat |
| **LSST/Rubin** TAP | **nein** (401, `WWW-Authenticate: Basic`; Policy: „No access … without Rubin data rights") | 401 | nur Ersatz: Gaia (andere Daten) |
| **GRDC** | **ja**: GRDC-Caravan `zenodo.org/records/15349031` **CC BY 4.0** | 200 | Subset (5356 Einzugsgebiete), nicht die volle DB |
| **RIPE RIS** | fragil: `archive.routeviews.org` | 200 | **RouteViews BY vs BY-SA**: Text sagt BY, Inline-Link zeigt `by-sa/4.0` |
| **IMCCE** INPOP19a/NOE-4 | Ersatz: JPL `ssd.jpl.nasa.gov/ftp/eph/planets/bsp/de440.bsp` (PD) | 200 | **EPM2021 offen**: `ftp.iaaras.ru/pub/epm/EPM2021/SPICE/epm2021.bsp` = **200, 154 278 912 B**; `quasar.ipa.nw.ru` tot (der alte Host, nicht das Produkt); im Register als `origin` |
| **TRISP/YBCO-NSE** (MPI-FKF) | kein Roh; Ersatz NIST NCNR (`nist.gov/open/license`) | 200 | NIST **SRD vs non-SRD** je Datensatz unbestimmt |

**Feld-fähige „neu-zulassen" (gemessen):** `giro.uml.edu/didbase` 200 **CC BY-NC-SA 4.0**
(NC-kompatibel) · `ds.iris.edu` 200 (frei, Attribution) · CTBTO-Original 403 · `data.mendeley.com` /
`waterisotopesDB.org` 200 (Lizenz je Dataset ungemessen) · IAEA WISER 403 (Login) · `globalfloods.eu`
200 (Copernicus; **Forecast → kein Zeuge**). Re-Admission = **Mountain**.

## C — Gated China — offene Alternativen

| Quelle | offene Alternative | HTTP | deckt die Größe? |
|---|---|---|---|
| **CEIC D18000** Schwere | **keine** keyless Route zur Quelle; offene **Ersatzapparate**: IGETS/GFZ (CC-BY-4.0, im Register) · BGI · ICGEM | 200 | **nein** — andere Instrumente |
| **CN-NSSDC `有条件共享`** (Lunar/Mars Topographie, `10.12176/…`) | LOLA-RDR (PDS) · SLDEM2015 (`imbrium.mit.edu`) · MOLA (PDS, registriert) · HRSC (ESA PSA) · ICGEM (SHC) | 200 | **ja** (Topographie/SHC) |
| **CSES/Limadou** | ScienceDB `scidb.cn` (`10.57760/sciencedb.06921`, **CC0**) — im Haus `cses_efd/hpm/lap.bin` | 200 | **Sample** (ein Orbit); Voll-Archiv registrierungsgattert |

**Riss (Korrektur, nicht geglättet):** Zenodo `10.5281/zenodo.7112529` / `…7855090` (CC-BY-4.0, offen)
sind **Figuren-Subsets** (~1,0 / 1,3 MB, „a constant value is removed for reading") — **nicht** die
D18000-Reihe. Eine frühere Note („dieselbe CEIC-Schwere, republiziert") war **überzogen**. Für D18000
selbst existiert **keine** keyless Route.

**Direktlinks der Alternativen (aufgelöst, 200):** LOLA-RDR
`pds-geosciences.wustl.edu/lro/lro-l-lola-3-rdr-v1/lrolol_1xxx/` · SLDEM2015
`imbrium.mit.edu/DATA/SLDEM2015/GLOBAL/{FLOAT_IMG,JP2}/` (README 1769 B) · MOLA
`pds-geosciences.wustl.edu/mgs/mgs-m-mola-3-prdr-l1a-v1/` · ICGEM
`icgem.gfz-potsdam.de/tom_longtime`.

## D — C/NOFS NWM (Absage → `absent`)

**Die Absage (UT Dallas, `docs/handover/antwort-mail-2026-10-16-ceic-utdallas.md`):** das NWM hat nie
funktioniert (Kammer-Leck, verrauschte Ionenquelle) → **es gibt keine NWM-Neutralwinddaten**. Das ist
kein `pending`, sondern **`absent`** — physikalisch leer. Der CINDI-`blocked account`-Wait ist
**hinfällig**. Offene Alternativen (200): **DE-2** `cdaweb.gsfc.nasa.gov/pub/data/de/de2/`
(`neutral_gas_wats/`, `combined_plasma_neutrals_ua/` = kollokales in-situ-Paar) · **ICON/MIGHTI**
`…/pub/data/icon/` (remote, **kein** in-situ-Paar).

**NSSDCA belegt es — keyless (`archive_search --nssdca <id>`):** `SPTH-00002` „DE-2 WATS 2-sec Neutral
Wind, Density and Temperature in ASCII" → **„Ready for Electronic Access"**; `SPIO-00105` „DE-2 IDM
0.25-sec ion drift in ASCII" → **„Ready for Electronic Access"**. Das ist das **kollokale
in-situ-Ion↔Neutral-Paar**, das die NWM-Lücke füllt — **offen bestätigt**, nicht behauptet.

**Direktlinks (aufgelöst, gemessen 2026-10-17, alle HTTP 200):**
- **WATS Neutralwind** (2 s ASCII): `https://cdaweb.gsfc.nasa.gov/pub/data/de/de2/neutral_gas_wats/n_T_v_2s_ascii/<YYYYDDD>_de2_wats_2s_v01.asc`
- **IDM Ionendrift** (250 ms ASCII): `https://cdaweb.gsfc.nasa.gov/pub/data/de/de2/plasma_idm/vi_250ms_ascii/<Jahr>/<MM>/<YYYYDDDT…>_de2_idm_250ms_v01.asc`
- **Kollokaler Satz** (16 s ASCII): `…/combined_plasma_neutrals_ua/ua_16s_ascii/<YYYYDDD>_de2_ua_16s_v01.asc`
- **CDF-Varianten:** `…/neutral_gas_wats/wind2s_wats_cdaweb/` · `…/plasma_idm/vion250ms_cdaweb/` · `…/combined_plasma_neutrals_ua/ua16s_all_cdaweb/`
- **NSSDCA-Detailseite:** `https://nssdc.gsfc.nasa.gov/nmc/dataset/display.action?id=SPTH-00002` (`SPIO-00105` analog)

## E — MODVOLC Collection 6 (HIGP-Host tot → historische Route bei NASA LAADS)

**Gemessen:** `modis.higp.hawaii.edu/data/…` → **Timeout (HTTP 000)**; DNS löst auf (`128.171.157.89`,
CNAME `hotspot3.higp.soest.hawaii.edu`), HTTP aber tot. `--wayback` kein Snapshot. Lokal:
`data/modis.higp.hawaii.edu/2000/` = **75 570** Dateien, 429 MB (nur Jahr 2000). Die damalige
Register-Route (nrt3 **Collection 6.1, nur Recent**, 200) ist **gefallen** — s. Abschluss unten.

**Die historische Collection 6 liegt unter anderem Namen bei NASA LAADS:**
`https://ladsweb.modaps.eosdis.nasa.gov/archive/allData/6/MODVOLC/<Jahr>/<DOY>/MODVOLC.A<YYYYDDD>.HHMM.006.dat`
→ **303 → Earthdata Login** (auth-gated). Das Haus trägt `EARTHDATA_EDL_TOKEN` (`.secrets.local`) —
die fehlenden Jahre 2001–2019 sind damit erreichbar. **Naming-Riss:** HIGP `…000.000000` · LAADS `…006`
· LANCE `…061.NRT` — deshalb fand die **Dateinamen**-Suche nichts; die **Struktur**-Suche (Jahr/DOY)
fand LAADS. `--all` (416 Z., `zenodo`/`crossref`/`dataverse` …) = **kein** freies Mirror.
Offene Alternativen (200): **NASA FIRMS** `firms.modaps.eosdis.nasa.gov` · **MIROVA** `mirovaweb.it`.

**Riss (Lizenz — nachvollziehbar, nicht „unbelegt"):** Das Register führte damals `terms PD
https://www.earthdata.nasa.gov/` — die Lizenz **wurde gemessen**
(mountain-340, 2026-10-13: HIGP-Route = keine offene Lizenz → `local-only`; der NASA-LANCE-Spiegel =
PD über die NASA-Datapolitik `earthdata.nasa.gov/…/data-information-guidance`; mycelium-343 stellte die
Route auf `nrt3.modaps.eosdis.nasa.gov`). **Neu und widersprechend:** LAADS Support: „this product is
**not approved for public distribution**." Zwei gemessene Zeilen, die sich nicht einigen — die
**NASA-Politik** (full/open sharing) gegen die **produktspezifische LAADS-Absage**. Ob die PD-Zeile
die **LANCE**-Route deckt und ob `modvolc_thermal.bin` öffentlich bleiben darf, ist **Mountains
Verdikt**, nicht meine Zeile. Der CDN-Träger hat jedenfalls unter `terms PD` veröffentlicht; ob das
trägt, ist damit **offen**. Der `license_census`-Gate prüft das Vorhandensein einer `terms`-Zeile,
nicht ihre **Wahrheit** — die Lücke bleibt.

**Kanäle + Lizenz (Lumo-Befund, verifiziert 2026-10-17):** LANCE `allData/61` trägt **MODVOLC** (200,
`MODVOLC.A…061.NRT.dat`); **LAADS** ist das **Level-1/Atmosphere**-Archiv (`MOD021KM`/`MYD021KM`, 200).
Die Support-Mail ist ein **Betriebs-, kein Lizenzinstrument**. `terms PD` ist der **falsche Wert**:
NASA ist *open* (Attribution erbeten, **kein** Endorsement, „as is"), nicht CC0/PD.

**Rohdaten-Route (der eigentliche Ausweg):** der MODVOLC-Algorithmus ist **publiziert** — **Wright,
Flynn, Garbeil, Harris & Pilger, JVGR `10.1016/j.jvolgeores.2003.12.008`** (288 Zit.): **4 µm
(MODIS Band 21/22) + 12 µm (Band 32)**; Verbesserung Gabrieli et al. 2020 („Maximum Radiance
Algorithm"). Die **Rohdaten** `MOD021KM`/`MYD021KM` liegen **offen** auf LAADS (200, noch **nicht** im
Register). → **Das Haus kann sein eigenes Thermalanomalie-Feld aus der offenen L1 rechnen** und
MODVOLC nur zum **Abgleich** nutzen (intern, **nicht** redistribuiert).

**Konsequenz (entschärft):** die Verteilungs-Wand betrifft nur das **fremde Derivat**. Unser Produkt
aus offener NASA-L1 ist mit Attribution verteilbar (`terms free-open`, **kein**
`decline-redistribution`). Die **Ratsfrage** bleibt nur für den Fall, dass wir das **MODVOLC-Derivat
selbst** weitergeben wollen.

**Operator-Wort (2026-10-17) — der Rückzug ist entschieden:** „wir nehmens runter im Zweifel lieber
nicht … was wollen wir uns streiten, wenn wir eh selber rechnen können." → **MODVOLC-CDN-Asset
zurückziehen** (Mycelium); der Register-Block fällt auf `local-only` (kein `terms PD`), lokal bleibt es
zum **Abgleich**; das **Feld** trägt der **L1-Block `MOD021KM`** (NASA-open + Attribution). Quelle:
Operator (Session). Damit ist der 4:2-Ratsspalt **durch das Operator-Wort** entschieden (konservativ).

**Abschluss (gemessen, Sensory-Pass 2026-10-17):** Der Register-Block ist **gefallen** —
`phi/sources.φ` trägt **keine** MODVOLC-Zeile mehr (Mycelium `796ae91c3`, „no fabricated PD line");
MODVOLC ist `local-only`. Das CDN-Asset ist **zurückgezogen**:
`…/releases/download/nrt3.modaps.eosdis.nasa.gov/modvolc_thermal.bin` → **HTTP 404** (der Tag steht
noch, **ohne** das Asset). Der Träger des Feldes ist der **L1-Block `MOD021KM`/`MYD021KM`** (NASA-open + Attribution, Wright et al. 2003) — noch **nicht** im Register; terms/field/ttl + der
Band-21/22/32-Reader liegen bei Mountain (`xfer:mycelium-361-7`). Die `terms decline-redistribution`-Zeile,
die der Dispatch als „bereits gesetzt" nannte, hat es **nie** gegeben — der Dispatch las `e789578d3`
(die Entschärfung) mit dem älteren 02:41-Dispatch zusammen.

## F — WALDO / AWESOME-VLF (Route gelöst, kein Suchknoten)

`vlf.stanford.edu` tot → **`https://waldo.world`** (200) trägt die AWESOME-VLF-Daten unter
**CC0-1.0** (DataCite `10.48322/fzcf-f191` broadband · `10.48322/6t7b-s324` narrowband ·
`10.48322/fwte-dv13` VLF/LF — Belege `docs/auftrag/auftrag-2026-10-11-wartend-aufloesung.md:29-39`).
Offen bleibt nur `waldo-massdaten` (der Bulk), **kein** Discovery-Problem — der Inhaltstrick hat hier
nichts zu holen.

**Direktlink aufgelöst (2026-10-17):** der Bulk liegt im **AWS-Open-Data-Bucket**
`https://craam-files-bucket.s3.amazonaws.com/?list-type=2` (keyless listbar, HTTP 200; Muster
`/<YYYY>/<MM>/<DD>/narrowband/<STATION>/<FILE>.mat`; Beispiel
`…/2015/01/01/narrowband/FE/FE150101000500NAA_008A.mat` = HTTP 200, **352 232 B**). Lizenz **PD** über
`https://registry.opendata.aws/craam-open-vlf/` (200). Das Haus führt den Block bereits
(`phi/sources.φ:1801`, `terms PD`) → `waldo-massdaten` ist damit eine **Route**, kein Warten.

## G — weitere Nüsse (gemessen 2026-10-17, zweiter Pass)

**Voyager-Saturn closed-loop (die Route ist da, der Parser fehlt).** `…/voyager1/radio_science_rss/
saturn_encounter_data/` = **200** (V1: 14 `PSPA-00049_DD0595xx_*.tar`, 3–17 MB; V2: 6
`PSPA-00123_DD0595xx_*.tar`, 3–28 MB; `SHA1SUM` + `attrib/`). Die Attributdatei bestätigt
`MACHINE_REPRESENTATION = "Univac 1108"`, Tape-Image, `NUMBER_OF_BYTES_IN_CANONICAL_STREAMS =
12 728 148`; Range-GET = **206**, `application/x-tar`. Die blanket-Aussage „Voyager closed-loop =
request-only" gilt nur für Cruise/Post-Saturn. Zulassung/Parser = Mountain.

**EPM2021 (der Host war tot, nicht das Produkt).** `ftp.iaaras.ru/pub/epm/EPM2021/SPICE/
epm2021.bsp` = **200, 154 278 912 B** (61,7 MB im 40-s-Fenster); `quasar.ipa.nw.ru:80` =
connection refused. Der Register-Block `weberin_verdicts` führt EPM2021 längst als `origin`
(`terms Open-Licence-2.0`). Das `pending` maß den alten Host.

**Jeder Punkt nachgeprüft (Katalog + Inhalt + Leiter, 2026-10-17, dritter Pass):**
- **OSDR:** API **lebt** — `/geode-py/ws/studies` (Liste) und `/geode-py/ws/repo/search?term=dosimetry`
  (**123** Treffer, u. a. `OS-166` „Space Radiation Dosimetry On Board Cosmos 936", `OS-161`
  „HZE-Particle Dosimetry", `OS-513` „Environmental Radiation Measurements on Mir"). Aber die Treffer
  sind **Experiment-/Studien-Records** (bioscience), **kein** Roh-Zeitreihen-Feed: `DOSTEL` **0**,
  `Liulin` **0**, `TEPC` **1** (nicht Dosimetrie). Die Prämisse „OSDR trägt ISS-Dosimetrie" ist als
  **Roh-Feed nicht belegt**; die alte `/bio/api`-Route ist **404**, der Redirect geht auf
  `science.nasa.gov/biological-physical/data/`. **Offene Roh-Dosimetrie liegt außerhalb OSDR:**
  CDAWeb `ISS_DOSANL_TEPC` (1-min Dosis/Äquivalent, 2010–2018) und `BIOSENTINEL_LET_LETS`
  (160-Bin-LET, 2022–2025); PDS/PPI `PSFP-00706` (MSL-RAD-EDR) und `PSFP-00698` (LRO-CRaTER-EDR),
  Level 0, 200. HREDA `DOSIS-3D` = **403** („Anonymous users cannot download data").
- **pskreporter:** Query-API `retrieve.pskreporter.info/query` liefert **`<receptionReports>`**
  (Empfangsrecords: Callsign, Frequenz, Mode) — **Registry**, kein Feld am Punkt. **reversebeacon:**
  Telnet-Port **7000 offen**, `raw_data/` 200 (HTML) — ebenfalls Empfangsreport-Netz. Eignungsfrage.
- **dods.wh.gov:** `--verdict` = direkt **keine Antwort** · Proton **absent** · Wayback **ohne
  Snapshot** (auch Timemap leer) → **absent**, nicht nur DNS.
- **C/NOFS NWM:** NSSDCA `spacecraft=c/nofs` und `CINDI` = **absent** (NSSDCA listet C/NOFS gar
  nicht); **CDAWeb** `pub/data/cnofs/cindi/` = **200**, führt aber **nur IVM** (Ionendrift,
  500 ms CDF/HDF), **kein NWM**. Das Archiv belegt **kein NWM-Produkt**; die stärkere Aussage
  „nie funktioniert" ist **widerlegt** (Earle 2013 / Haaser 2013: meridionale NWM-Daten 2011,
  nur N-S-Kanal) — es gibt nur **kein öffentliches Datenprodukt**.
- **Juno pre-EFB:** PDS `jnogrv_0001` (**Outer Cruise**, post-EFB) ODF beginnt bei **`2013284`**
  (2013-10-11); `jnogrv_1001` ist „Jupiter, science phase from July 2016". Ein pre-EFB-ODF-Volume
  existiert **nicht** — bestätigt.
- **Viking:** `PSPG-00011` = „Data Identified but not Received" (Tracking); **`PSPG-00457`** =
  „Ready for Offline Distribution" (SDDPT Radio Science); **`PSPA-00015`** = „Doppler and Range
  Tracking Data", „Ready for Offline Distribution" → **offline**, nicht online.
- **Mariner 10:** `PSCM-00009` = „Ready for Offline Distribution"; die Teilroute ist **live** —
  `…/red_tele_signal_data_venus_occlt/` führt **9 TARs** (`PSPA-00316_DD0296xx`), HEAD **200**,
  **19 619 840 B**, `application/x-tar`, Range-Kopf `PSPA-00316_…dir/`. Vollkorpus request-only.
- **Voyager:** Saturn closed-loop **live** (V1 14 TARs, V2 6 TARs, Univac-1108); PDS
  `voyager_rss_raw` = **Jupiter (V1/V2) + Uranus (V2 49xr)** open-loop. **Post-Saturn closed-loop
  existiert** — NSSDCA `PSCM-00004` (Uranus) / `PSCM-00003` (Neptun), Doppler+Range, Univac 1108 —
  aber **„Ready for Offline Distribution"**; `PSNO-00007` = offline (PDS, Imaging). Cruise/Post-Saturn
  bleibt **request-only** (nicht „nirgends").
- **CEIC D18000:** Zenodo-Suche `q=D18000` = **0**; OpenAIRE/DataCite = **absent**; die **D18000-Seiten**
  auf `data.earthquake.cn` sind **200** (Katalog sichtbar, 重力/Gravity/D18000), der Download verlangt
  **注册/登录** → **Registrierungs-Gate, keine fehlende Quelle**. Zenodo `7112529`/`7855090` = **200**
  (Sichuan/Lushan-Subset: **74 Rohdateien** + Code + Figuren, **kein** Voll-Mirror).
- **LSST/Rubin:** TAP `data.lsst.cloud/api/tap` = **401** (`WWW-Authenticate: Basic realm="data.lsst.cloud"`);
  offene Alternative **Fink** `api.lsst.fink-portal.org/api/v1/schema` = **200**.

**Vier Taucher, harte Bandagen (2026-10-17):**
- **OSDR:** Experiment-Route `/geode-py/ws/repo/experiments/<OS-ID>` → `files: []` bei `OS-166`,
  `OS-161`, `OS-202`, `OS-291`, `OS-513`; nur Prosa-Mittelwerte. Kein Roh-Feed.
- **RBN:** Handshake `Please enter your call:` → Callsign (kein Passwort/Key), `Spot rate: 4/s`;
  Schema `de,freq,dx,db,wpm,time,diff,band,type,mode,epoch`; `call_info` lat/long = QRZ-Lookup;
  Tages-CSV `data.reversebeacon.net/rbn_history/YYYYMMDD.zip` (20261016: **24 568 336 B**,
  **368 763** Zeilen, 243 Spotter); `raw_data/` ist ein Index (robots disallow). Registry.
- **CEIC:** D18000 = **zwei** Datensätze — flow 1998–2015 (**300 GB**) und absolute 2009–2015
  (**22 GB**); Order-Endpunkt → **302 `login.jsp`**; Registrierung faktisch **+86/China**.
  Zenodo `7112529`/`7855090` = **74 Rohdateien** + Code + Figuren (Sichuan/Lushan 2010–2013),
  **kein** Voll-Mirror; NEDC-DOIs `10.12080/nedc.gc_zl.ds0001.2026` = **404**.

**Fünf Taucher, zweite Runde (2026-10-17) — die Restlücken:**
- **ISS-Rohspektren:** CDAWeb `ISS_DOSANL_TEPC` hat nur `DOSE`/`DOSE_EQ60` (1-min); PDS-Registry
  `TEPC/DOSTEL/Liulin/DOSIS/MATROSHKA` = **0**; NSSDCA absent. Offene Spektren nur **nicht-ISS**:
  `BIOSENTINEL_LET_LETS` (160 Bin), CRaTER `10.5281/zenodo.22883077` (CC-BY-4.0), IFJ PAN
  `10.48733/ifjpan/oebph2` (Dosis-Tabelle). **DOSIS-3D-XLSX: hartes ESA-LDAP, kein anonymer Pfad,
  Inhalt ungeprüft** (DOSTEL-DOI `10.57780/esa-8hfqjak`: `data_available=false`, 0 Dateien).
- **RBN-Lizenz:** keine offene Lizenz, `about/terms/privacy/legal/copyright` = **404**; einziger Satz:
  „freely available for study and analysis" + Bitte um Ideen — **kein Redistributions-Grant**.
  CDN = **pending**, schriftliche Anfrage nötig. → **vorbereitet, Operator-Sendewort nötig**
  (Webformular `reversebeacon.net/contact`, Entwurf `state/stimmen/2026-10-18-haertefaelle-anfragen.md` §1).
- **CEIC:** Ausländer-Zugang **nein** — `^1[3-9]\d{9}`, Name „nur Han", Provinz-Adresse, Nutzungsregel
  „中国公民都可申请"; Order → 302 `login.jsp` (Auth, nicht Geo). Email-Kanal: `dcogcd@163.com`,
  `datashare@seis.ac.cn` (ob Ausland beliefert wird: offen). **DOI-Korrektur:** `ds0001.2026`
  existiert **nicht**; die echten sind `ds0005.2026` (flow), `ds0006.2026`, `ds00001.2024` — alle
  „DOI does not exist" (2026-Charge ungemünzt); 2025er lösen auf. → **vorbereitet,
  Operator-Sendewort nötig** (Mail an `datashare@seis.ac.cn`, Cc `dcogcd@163.com`, Entwurf
  `state/stimmen/2026-10-18-haertefaelle-anfragen.md` §2; nachgemessen 2026-10-18: 4/4 DOI 404,
  DataCite `prefix=10.12080` = 0).
- **Voyager:** kein Web-Bestellpfad — `order/download/request.action` = identische **3 645-B-Soft-404**
  (md5 `de205919…`); realer Weg = **CRUSO** `gsfc-dl-nssdca-request@mail.nasa.gov`, Ermessen +
  Kostenübernahme + ggf. Digitalisierung. Uranus/Neptun = **physische Univac-1108-Tapes**, kein
  Digitalisat; Saturn = digital/keyless (14+6 TARs).
- **C/NOFS 2011 NWM:** nicht beziehbar — UTD-Portal lieferte nie NWM (Wayback: 159 IVM, **0 NWM**),
  CGI seit 2018 **403**; formale Route nur E-Mail (Hairston `hairston@utdallas.edu`, Heelis
  `roderick.heelis@utdallas.edu`, Earle `earle@vt.edu`); alte Terms: „may not be redistributed".

**Vier Taucher mit `bin/all_wide` (die breite 109-Modus-Leiter, `OMEGAFLOW_ALL_STOP=400`; kein `--wide`-Flag),
2026-10-17 — die vier Restlücken erschöpfend:**
- **DOSIS-3D/DOSTEL:** HREDA keyless TAP: `v_dataset` 163 Zeilen, `v_file_metadata` 105 618 616 — für
  DOSIS/DOSTEL **0**; `EuTEF DOSTEL` `10.57780/esa-8hfqjak` und `DOSIS 3D` `10.57780/esa-znu2ukt` =
  `data_available=false`. `file_metadata_oid=2488390` gehört zu `SutureInSpace_ISS` (Name trägt
  „DOSIS-3D" nur als Herkunft); Download-Route 403/500 = **kein** sauberer Auth-Kanten-Beweis (Riss).
  Zenodo `DOSTEL` = 2 Volltext-Zufallstreffer, `type=dataset` 0; DataCite = **1** echtes Dataset
  (`10.48733/ifjpan/oebph2`, Dosis-Tabelle). Nächstbestes: **CRaTER GCR LET-Spektren**
  `10.5281/zenodo.22883077` (CC-BY-4.0, echtes LET-Spektrum, aber **LRO/Mond**).
  → **vorbereitet, Operator-Sendewort nötig** (Mail an `hreda.info@sdc.upm.es`, Entwurf
  `state/stimmen/2026-10-18-haertefaelle-anfragen.md` §3; nachgemessen 2026-10-18: HREDA-TAP
  `v_doi_investigation` = `data_available=false` für beide, auch für die drei übrigen
  DOSIS-Investigationen).
- **CEIC D18000:** `all_wide` über alle Arme = **kein** Voll-Mirror. Nur `7855090`/`7112529`
  (Sichuan/Lushan, 7 Kampagnen 2010–2013, 1 Absolutpunkt belegt: `AG201006.txt` → `51060301`);
  RAR5 → p7zip kann 109/110 Einträge nicht dekodieren (Punktzahl bleibt Behauptung, nicht Messung).
  **Am Original bestätigt:** 300,0 GB / 22,0 GB; `doi.org` „**This DOI cannot be found**",
  DataCite `prefix=10.12080` = **0** registrierte DOIs.
- **Voyager Uranus/Neptun:** **kein** keyless Digitalisat. Gegenprobe, die es beweist:
  `PSCM-00002` (Mariner 9) trägt den Satz „transferred to CD-WO as part of NASA's **Science Digital
  Data Preservation Task**" — bei `PSCM-00003`/`-00004` **fehlt** er. NTRS/DataCite/OpenAIRE/Zenodo/
  Wayback = nur Papers/Bild-CDs; `voyager2_rss_neptune_raw` = **404**. Uranus offen nur open-loop
  (Parkes PODR, 1986-01-22…25). **Präzedenz:** Pioneer 10/11 Doppler **sind** auf Zenodo
  (`10.5281/zenodo.13309156`/`13309040`, ASCII, John Anderson — derselbe Data Provider) → Voyager
  hat das Gegenstück nicht.
- **C/NOFS NWM:** **kein** Produkt. CDAWeb HAPI: 3636 IDs, 5 C/NOFS (alle IVM/PLP/VEFI); SPASE
  `NWM`/`neutral wind meter` = 0; DataCite `C/NOFS` = 14 (kein NWM); Wayback-CDX 1080 URLs:
  **5 NWM-Seiten, 0 NWM-Dateien**. **Riss:** das Suchformular bot `R1=NWM` an, obwohl die Datenseite
  „keine NWM-Daten" sagt; CGI nie archiviert → nicht messbar. **Ungemessen:** `ads`/`base` (Token fehlt)
  — genau dort wäre ein NWM-Datensatz registriert.

**Der stille Eimer „nicht gemessen":** **50** Register-Blöcke tragen keine `terms`-Zeile
(überwiegend Routen, Manifeste, PDFs, JSON — nicht jede ist eine Quelle). Jede ist eine offene
Lizenz-Messung, keine Abwesenheit; `license_census` prüft Existenz, nicht Wahrheit.

## Grenze

Alle Lizenzen „ungemessen" heißen `pending`, nie frei. Die Re-Admission der Kandidaten (B) und die
Register/CDN-Nähte (A, C, E) sind nicht meine Hand. Kein stilles `pending` — jeder Rest ist benannt.
