<!--
  title: Survey — Die Weberin: offene Quellen-Routen, Re-Run (Stand 2026-09-14)
  class: survey
  date: 2026-09-14
  sha256: 0f9de1e425e3086bfdecd74558ae371c3803ab1152a88c4fed14a838dfbd7c07
  status: live
  see-also: docs/surveys/survey-2026-09-13-weberin-quellen.md docs/surveys/survey-2026-09-07-weberin-thread-matrix.md
-->
# Die Weberin — offene Quellen-Routen, Re-Run (Stand 2026-09-14)

Konsolidiertes Einzel-Dokument: die drei Snapshot-Blätter `survey-2026-09-13-weberin-quellen{,-folge,-treffer}.md`
zusammengeführt und mit dem **Re-Run vom 2026-09-14** nachgezogen. Der Re-Run
fährt alle dokumentierten Routen der elf Quellenklassen mit dem gefixten
`archive_search --verdict` erneut (6 Taucher, ~55 Routen, Proton-VPN-Exit hoch).
Kein Register-Eingriff — der Snapshot bleibt der Snapshot; dieses Blatt ist die
eine, nachgezogene Fassung.

Status-Vokabular: `live` = offen, maschinenlesbar · `blocked` = lebt, Zugang
gesperrt · `declined` = lebt, aber keine physikalische Messung am Punkt ·
`pending` = Route bekannt, Verdikt ausstehend · `not-published` = kein offener
maschinenlesbarer Weg nach abgeschlossener Suche.

## Die elf Klassen (Re-Run 2026-09-14)

| # | Klasse (Kraft) | Route | HTTP | Verdikt |
|---|---|---|---|---|
| 1.1 | Boden-Gravimeter IGETS (gravity) | `isdc.gfz.de/igets-data-base` | 200 (162 937 B) | live (Stationsliste + Position) |
| 1.2 | IGETS DOI-OAI | `doidb.wdc-terra.org/oaip/oai` | 200 (462 B) | live |
| 1.3 | IGETS data-access | `isdc.gfz.de/igets-data-base/data-access` | 200 (74 428 B) | live |
| 1.4 | IGETS Alt-EOST | `igets.u-strasbg.fr/tr005.php` | 200 (7 938 B) | live (JPG/PDF) |
| 2.1 | Infraschall BGR maw | `download.bgr.de/…/BGR_infrasound_maw_product.zip` | 200 (89 510 967 B) | live (netCDF-ZIP, CC BY 4.0) |
| 2.2 | Infraschall BGR mb_lf | `…/BGR_infrasound_mb_lf_product.zip` | 200 (93 554 515 B) | live |
| 2.3 | Infraschall BGR mb_hf | `…/BGR_infrasound_mb_hf_product.zip` | 200 (67 122 764 B) | live |
| 2.4 | Infraschall BGR hf | `…/BGR_infrasound_hf_product.zip` | 200 (77 186 702 B) | live |
| 2.5 | Infraschall Belegpaper | `essd.copernicus.org/articles/14/4201/2022/` | 200 (357 741 B) | live |
| 2.6 | CTBTO vDEC-Seite | `www.ctbto.org/specials/vdec/` | 403 (Wayback 200) | blocked (Vertragsweg) |
| 2.7 | CTBTO vDEC | `vdec.ctbto.org/` | 404 | not-published |
| 3.1 | NRS GCS-Bucket | `storage.googleapis.com/…/noaa-passive-bioacoustic/o?prefix=nrs/` | 200 (1 147 084 B) | live |
| 3.2 | NRS Metadaten-XML | `…/nrs/audio/01/…/NRS_2014-2015_01.xml` | 200 (3 058 B) | live (lat/lon) |
| 3.3 | NRS README | `…/nrs/README.pdf` | 200 (20 055 B) | live |
| 3.4 | NCEI passive-acoustic | `ncei.noaa.gov/products/passive-acoustic-data` | 200 (96 728 B) | live |
| 3.5 | PMEL NRS | `pmel.noaa.gov/acoustics/…` | 200 (44 512 B) | live |
| 3.6 | ONC locations | `data.oceannetworks.ca/api/locations?method=get` | 401 | blocked key |
| 3.7 | Borealis ONC | `borealisdata.ca/dataverse/oceannetworkscanada` | 200 (88 681 B) | live |
| 4.1 | Seismik EarthScope IU | `service.earthscope.org/fdsnws/station/1/query?…network=IU` | 200 (10 345 B) | live |
| 4.2 | Seismik GEOFON GE | `geofon.gfz.de/fdsnws/station/1/query?…network=GE` | 200 (14 820 B) | live |
| 4.3 | Seismik RaspberryShake AM | `data.raspberryshake.org/fdsnws/station/1/query?…network=AM` | 200 (3 775 512 B) | live |
| 4.4 | Seismik ISC QuakeML | `isc.ac.uk/fdsnws/event/1/query?format=xml&…` | 200 (894 340 B) | live (engem Zeitfenster) |
| 5.1 | GIC BPA | `transmission.bpa.gov/…/gic/gic.txt` | 200 (83 404 B) | live (4-Tage-Rollfenster) |
| 5.2 | GIC BPA | `transmission.bpa.gov/…/gic/gic.aspx` | 200 (7 917 B) | live |
| 5.3 | GIC BGS B_GIC | `geomag.bgs.ac.uk/…/gic_services.html` | 200 (25 965 B) | declined (Modell) |
| 5.4 | GIC BGS geoelectric | `geomag.bgs.ac.uk/…/geoelectric.html` | 200 (26 060 B) | declined (kein GIC-Kanal) |
| 5.5 | GIC Zenodo Alberta | `zenodo.org/api/records/10594301` | **200** (6 097 B) | pending → **Route offen** |
| 5.6 | PANGAEA | `pangaea.de/advanced/search.php?q=GIC&format=json` | 200 (12 019 B) | live (kein GIC-Treffer) |
| 6.1 | WWLLN Thunder-Hour 2025 | `wwlln.net/climate/th_yr/data/WWLLN_th_2025.nc.zip` | 200 (48 685 822 B) | live (netCDF-Gitter) |
| 6.2 | WWLLN Index | `wwlln.net/climate/th_yr/data/` | 200 (5 693 B) | live |
| 6.3 | WWLLN readme | `wwlln.net/climate/th_yr/data/readme.pdf` | 200 (146 002 B) | live |
| 6.4 | LIS/OTD | `lightning.nsstc.nasa.gov/data/` | 200 (54 746 B) | blocked account (GHRC) |
| 6.5 | GHRC | `ghrc.earthdata.nasa.gov/` | 200 (1 093 B) | blocked account |
| 6.6 | GLM S3-Liste | `noaa-goes16.s3.amazonaws.com/?list-type=2&prefix=GLM-L2-LCFA/` | 200 (302 468 B) | live (in-register) |
| 7.1 | SuperDARN Radar-Info | `superdarn.ca/radar-info` | 200 (247 907 B) | live (Radar lat/lon) |
| 7.2 | SuperDARN | `superdarn.ca/` | 200 (28 521 B) | live |
| 7.3 | SuperDARN VT | `vt.superdarn.org/` | 200 (61 845 B) | live |
| 7.4 | SuperDARN Globus | `sdc-serv.usask.ca/data-access` | 200 (38 895 B) | blocked account |
| 7.5 | SuperDARN FRDR RAWACF | `frdr-dfdr.ca/repo/collection/superdarn` | 200 (26 840 B) | live (DOI, DMap) |
| 7.6 | SuperDARN data | `data.superdarn.ca/` | absent (DNS) | not-published |
| 8.1 | HF-Radar allDatasets | `hfradar.ioos.us/erddap/tabledap/allDatasets.json?…` | 200 (19 924 B) | live |
| 8.2 | HF-Radar BML_PBON | `hfradar.ioos.us/erddap/tabledap/BML_PBON.json` | 200 (7 008 066 B) | live (antenna lat/lon) |
| 8.3 | HF-Radar Radials-Index | `hfradar.ioos.us/radials-erddap/erddap/index.json` | 200 (706 B) | live |
| 8.4 | HF-Radar ThREDDS | `hfrnet-tds.ucsd.edu/` | absent (Timeout) | pending |
| 8.5 | HF-Radar EMODnet (EU) | `erddap.emodnet-physics.eu/…/HFRADAR_NADR_Totals/index.json` | 200 (21 322 B) | live |
| 8.6 | HF-Radar Ifremer (EU) | `erddap.osupytheas.fr/…/HFRADAR_grid_copernicus/index.json` | 200 (24 903 B) | live |
| 8.7 | HF-Radar hfrnode | `hfrnode.eu/` | 200 (6,4 s) | live (langsam) |
| 8.8 | HF-Radar Kanada | `ceotr.ocean.dal.ca/erddap/info/codar_totals_2015/index.json` | 200 (8 120 B) | live |
| 9.1 | BGC-Argo Index | `data-argo.ifremer.fr/argo_bio-profile_index.txt.gz` | 200 (14 302 471 B) | live (Vollkatalog) |
| 9.2 | Argovis Vocabulary | `argovis-api.colorado.edu/argo/vocabulary?parameter=data` | 200 (1 195 B) | live |
| 9.3 | Ifremer ERDDAP | `erddap.ifremer.fr/erddap/index.json` | 200 (718 B) | live (Kern-Argo) |
| 9.4 | PolarWatch | `polarwatch.noaa.gov/erddap/index.json` | 200 (730 B) | declined (kein BGC) |
| 10.1 | HAWC 2HWC | `data.hawc-observatory.org/…/2HWC.yaml` | 200 (Playwright; curl -k 200, 18 588 B) | live (Browser) — curl braucht das TLS-Intermediate |
| 10.2 | HAWC 3HWC | `data.hawc-observatory.org/…/3HWC.yaml` | 200 (Playwright; curl -k 200, 51 833 B) | live (Browser) — curl braucht das TLS-Intermediate |
| 10.3 | LHAASO 1LHAASO | `casdc.china-vo.org/…/table.csv` | 200 (16 456 B) | live |
| 10.4 | Telescope Array Zenodo | `zenodo.org/records/8427755` | **200** (62 404 B) | pending → **Route offen** |
| 10.5 | Super-K | `www-sk.icrr.u-tokyo.ac.jp/sk/lowe/` | 200 (747 B) | declined (position-only) |
| 11.1 | ANTARES | `api.antares.noirlab.edu/v1/loci?page[limit]=10&page[offset]=0` | 200 (20 652 B) | live |
| 11.2 | Fink/LSST | `api.lsst.fink-portal.org/api/v1/conesearch` | **200** (42 B) | pending → **antwortet** |
| 11.3 | Lasair-ZTF | `lasair-ztf.lsst.ac.uk/api/query/` | 404 (Wartung) | blocked key (Token); **Wartung 14.–16.9., at-risk 17.–18.9.** — Wiedervorlage |
| 11.4 | ALeRCE | `api.alerce.online/alerts/v1/objects/` | **200** (5 740 B, via VPN) | pending → **antwortet** |
| 11.5 | TNS | `wis-tns.org/…/tns_public_objects.csv.zip` | 403 | blocked (UA-Gate) |
| 11.6 | Gaia Alerts | `gsaweb.ast.cam.ac.uk/alerts` | 200 (7 574 B) | declined (HTML-Portal) |

## Änderungen gegenüber dem Snapshot (2026-09-13 → 2026-09-14)

| Route | 2026-09-13 | 2026-09-14 |
|---|---|---|
| 5.5 Zenodo Alberta-GIC | 000 (Timeout) | **200** — Route offen |
| 10.4 Zenodo TA-Amate | 504 | **200** — Route offen |
| 11.2 Fink/LSST conesearch | 000 | **200** (42 B) |
| 11.4 ALeRCE | 000 | **200** (via VPN) |
| 11.3 Lasair-ZTF | 401 | **404** |
| 8.7 hfrnode.eu | 200 | **absent** (Timeout) |
| 10.1/10.2 HAWC | 200 (curl -k) | **200 via Playwright/Browser**; curl braucht das TLS-Intermediate (der Server sendet nur das Leaf-Zert, `verify error 21`) |
| 8.7 hfrnode.eu | 200 | **200** (6,4 s — der „absent" war ein zu kurzer Timeout) |
| 11.3 Lasair-ZTF | 401 | **404** — geplante Wartung (Lasair-ZTF/-LSST offline 14.–16.9., at-risk 17.–18.9.) |

## Harte Bandagen (Playwright + `-k` + `--cacert`)

- **HAWC** (`data.hawc-observatory.org`): die TLS-Kette ist unvollständig — der Server sendet nur das Leaf-Zert (`openssl s_client` → `unable to get local issuer certificate`, verify 21). **Playwright holt das YAML (200)** und der Katalog parst (RA/Dec/Flux/Index); `curl -k` liefert 200 (18 588 B). Fix für den Archivar: das Let's-Encrypt-Intermediate ins CA-Bundle (`OMEGAFLOW_CA_BUNDLE`).
- **Lasair**: der 404 ist die **Wartungsseite** („Due to scheduled maintenance, both Lasair-ZTF and Lasair-LSST will be fully offline from Monday morning Sept 14 through Wednesday Sept 16, and operating at-risk on Sept 17-18"). Kein toter Pfad — Wiedervorlage 2026-09-18.
- **hfrnode.eu**: antwortet 200, nur langsam (6,4 s) — der vorige `absent` war ein Timeout.

## Gesamtbild (2026-09-14)

- **Vollständig live:** seismische FDSN-Netze (4), HF-Radar-Radials (8), BGC-Argo (9), ANTARES (11.1).
- **Offene Route, account-/key-Blockiertes Roh-Zwilling:** IGETS (SFTP), BGR-Infraschall (vDEC), NOAA-NRS (ONC-Token), SuperDARN (Globus/FRDR).
- **Offen, aber nicht gebaut:** WWLLN-netcdf (6), BPA-GIC (5), LHAASO (10.3).
- **Neu offen (Re-Run):** Zenodo GIC (5.5), Zenodo TA (10.4), Fink (11.2), ALeRCE (11.4).
- **Harte Reste:** HAWC-TLS-Kette (10.1/10.2) · GIC kontinuierlich + Stationskoordinaten · TA-Vollkatalog · Fink/ALeRCE-Persistenz.

Kein Wert ist fabriziert; jeder ungemessene Punkt bleibt `pending`.
