<!--
  title: Survey — Die Weberin: offene Quellen-Routen, Re-Run (Stand 2026-09-14)
  class: survey
  date: 2026-09-14
  sha256: d090c2707c32ea0bb5a4b4d89b318c04a3e705ee8825dfd2249eee7fd748d6f5
  status: live
  see-also: docs/concepts/die-weberin.md docs/surveys/survey-2026-09-13-weberin-quellen.md docs/surveys/survey-2026-09-07-weberin-thread-matrix.md
-->
# Die Weberin — offene Quellen-Routen, Re-Run (Stand 2026-09-14)

Träger: `docs/concepts/die-weberin.md` (Weberin-Punkte).

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
| 11.4 | ALeRCE | `api.alerce.online/alerts/v1/objects/` | **200** (5 740 B, via VPN) | pending → **antwortet** — Nachzug 2026-09-17: in `sources.φ`:856 registriert (`format alerce`) |
| 11.5 | TNS | `wis-tns.org/…/tns_public_objects.csv.zip` | 403 | blocked (UA-Gate) — Nachzug 2026-09-17: in `sources.φ`:1353 registriert, Key `TNS_API_KEY`/`TNS_UA` |
| 11.6 | Gaia Alerts | `gsaweb.ast.cam.ac.uk/alerts` | 200 (7 574 B) | declined (HTML-Portal) |

Nachzug 2026-09-17: HAWC 2HWC/3HWC, LHAASO 1LHAASO und ANTARES sind in
`phi/sources.φ` registriert und kompiliert (`hawc_2hwc.sky1`:1108,
`hawc_3hwc.sky1`:1115, `lhaaso_sky1.sky1`:1101,
`antares_events_2007_2017.sky1`:1129). Telescope Array (Zeile 10.4) bleibt
pending — kein Eintrag in `sources.φ`.

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

## Batch-Re-Run durch `archive_search --all` (2026-09-14)

Die zwei früheren Such-Batches (die „Wand-Party" und „ehrlich benannt") wurden als
Rezepte rekonstruiert und je Subject durch den neuen `archive_search --all`
gefahren (eine Query durch alle 13 Modi: openalex, arxiv, crossref, ads, ntrs, wiki,
github, crates, librs, brave, datacite, zenodo, wayback).

### Batch 2 — „Wand-Party" (commercial/redistribution-Declines, 10)

| Subject | offene Route |
|---|---|
| Scopus | `dev.elsevier.com` (freier Non-Commercial-Key) |
| Dimensions | `dimensions.ai/metricssignup` |
| MarineTraffic/Kpler | `marinecadastre.gov/accessais` (CC0) |
| Google Dataset Search | **keine offene Route** (ABSENT) |
| BOM SWS | `sws-data.sws.bom.gov.au/register` (freier Key) |
| OpenTopography | `opentopography.org/developers` (Academic-Key) |
| Semantic Scholar | Apify-Scraper (keyless Felder) |
| ThingSpeak | `mathworks.com/help/thingspeak/readdata` (öffentliche Channels) |
| Earth Networks | NOAA-Blitz-Detektion (GLM-Route) |
| IAEA WISER | `iaea.org/services/networks/gnip` (GNIP public domain) |

### Batch 1 — „ehrlich benannt" (Sonden-Rohdaten/Paywall/gated, 13)

| Subject | offene Route |
|---|---|
| Voyager ODF | `pds-ppi.igpp.ucla.edu/mission/Voyager/VG2/RSS` |
| New Horizons REX | `pdssbn.astro.umd.edu/…/pds4-nh_documents:rex-v2.0` |
| Galileo RSS | `pds-ppi.igpp.ucla.edu/archive1/GOMW_5002/…/RSS.PDF` |
| LISA Pathfinder | `lpf.esac.esa.int/lpfsa` (Science Archive) |
| GRACE-FO | `podaac.jpl.nasa.gov/dataset/GRACEFO_L1B_ASCII_GRAV_JPL_RL04` |
| SuperMAG | `supermag.jhuapl.edu/mag` |
| AMS-02 | `heasarc.gsfc.nasa.gov/W3Browse/ams-02/ams02spec.html` |
| Woo/Armstrong 1979 | Semantic-Scholar-Paper |
| JPL/DSN ODF · Juno EDR · Super-K · Telescope Array · CSES | Doku-/Info-Routen (kein offener Roh-Daten-Endpoint) |

**Kernbefund:** fast jede „Mauer" war keine; nur **Google Dataset Search** ist ehrlich
ABSENT — die DSN/JPL-ODF und die Paywall-Volltexte bleiben echte Absenzen.

## Pro-Grind — die drei echten Absenzen (2026-09-14)

### Google Dataset Search — ABSENT bestätigt
Kein offener maschinenlesbarer Weg (kein API, kein Dump, kein Index):
`datasetsearch.research.google.com` ist eine Client-SPA (Playwright → nur die JS-Hülle,
Login-Prompt); Google **crawlt** schema.org/Dataset-Markup, kein Query-Endpoint. Die
„API"-Treffer sind bezahlte Dritt-Scraper (DataForSEO, anakin.io). Offene Alternativen
(HTTP 200, gemessen): DataCite `api.datacite.org/dois` · OpenAIRE
`api.openaire.eu/search/datasets` · B2FIND (CKAN `b2find.eudat.eu/api/3/action/package_search`) ·
re3data `re3data.org/api/v1/repositories` (1 023 770 B Registry-XML) · DataCite Commons
(GraphQL `api.datacite.org/graphql`). Sie tragen die Topic-Suche über DOI-/Metadaten-Register —
nur der webweite schema.org-Crawl fehlt.

### DSN/JPL-ODF (Erd-Vorbeiflüge) — request-only bestätigt
Kein offener ODF/TRK-2-34-Bestand der Erd-Encounter (Galileo, NEAR, Cassini, Juno).
PDS-Search-API (`pds.nasa.gov/api/search/1/products`, facet `ref_lid_instrument →
ref_lid_target`): Galileo/Cassini nur Jupiter-/Saturn-System; **Juno TRK-2-34 erst
2022-02-25 → 2024-12-27** (Jupiter-Phase; Flyby 2013-10-09 fehlt, 0 Treffer „2013");
NEAR `earth` nur auf Mission-Bundle-Ebene. NSSDCA `PSPG-00721`: NEAR-ODF für
Mathilde/Eros — **nicht** den Erd-Vorbeiflug 1998-01-23. NAIF/SPICE nur Geometrie-Kernels.
Das **Format** ist offen (Juno TNF, NEAR ODF); die Erd-Encounter-Rohdaten liegen bei JPL/DSN.

### Paywall-Volltexte — 4/5 bestätigt, Hinson 1997 offen
| Paper | DOI | Verdikt |
|---|---|---|
| Woo & Armstrong 1979 | `10.1029/JA084iA12p07288` | Paywall (`is_oa:false`; NTRS abstract-only) |
| Armstrong 1998 | `10.1029/98RS02317` | Paywall (`is_oa:false`) |
| Wohlmuth 1997 | `10.1007/978-94-015-8790-7_41` | Paywall (CiteSeerX-Lead tot: 429/404) |
| Haw 1997 | `10.2514/2.3240` | Paywall (`is_oa:false`) |
| **Hinson 1997** | `10.1029/97GL01608` | **offen (bronze OA, Wiley `doi/pdfdirect`)** — der 403 ist Cloudflare-Bot-Schutz, keine Paywall; Zahlen zudem aus offenen PDS-Daten re-derivierbar (`GO-J-RSS-1-ODF-V1.0`, `galileo_odf_compiler.rs`) |

**Fazit:** Die drei Absenzen halten. Nur Hinson 1997 kippt — offener Bronze-OA-Volltext
plus der offene PDS-Datenweg; der Paywall ist für die Borduhr-Sprung-Frage irrelevant.

## Nachtrag 2026-09-16 — Re-Run der leer gebliebenen Zweitlinien

Die Klassen, die im Re-Run ohne offene Route blieben, wurden erneut gemessen
(`archive_search --all`/`--leads`/`--verdict`; register_lookup gegen
`sources.φ`/`dead_sources.φ`/`blocked_sources.φ` vor jeder Suche). Ergebnis:

| Klasse | Messung 2026-09-16 | Verdikt |
|---|---|---|
| Messenger ODF Erd-Flyby | `pds-geosciences.wustl.edu/messenger/urn-nasa-pds-mess-rs-raw/` 200, Bundle 1.0 mit `data-odf/` Jahresordner **2007–2015, kein 2005** | request-only bestätigt (Erd-Encounter fehlt im offenen Bundle; `messenger_odf.bin` trägt Venus/Merkur) |
| Rosetta Erd-Swingby ODF | `archives.esac.esa.int/psa/ftp/ROSETTA/` 404; `…/psa/ftp/INTERNATIONAL-ROSETTA-MISSION/` 200, trägt `RSI/` | pending (RSI-Unterbaum auf Erd-Swingby-ODFs 2005/2007/2009 zu prüfen) — Nachzug 2026-09-17: `rosetta_odf.bin` in `sources.φ`:6473 registriert |
| Juno Gravity (`jnogrv`) | `atmos.nmsu.edu/PDS/data/jnogrv_1001/` `--verdict` 200 (2897 B) | live (Quelle des gebauten `juno_odf.bin`, sources.φ:6308) |
| JUNO-Neutrino-Release | Erst-Physik publiziert (Nature `s41586-026-10538-z`, 59,1 d Daten, 2026-06-10); Zenodo 101 = Talks/Poster, DataCite ohne Datensatz | `not-published` bestätigt — Daten-Release absent (Wiedervorlage: IHEP-Open-Data-Policy) |
| Hydrophon | `--leads hydrophone` 267 Dateien, 0 neue Hosts; NRS-GCS-Bucket live (Re-Run 3.1–3.5) | live, aber **ohne Compiler** (kein NRS-Eintrag in sources.φ) — Bau-Reihenfolge, kein Quellen-Verdikt |
| TNO Sternbedeckung | `--all "stellar occultation transneptunian object astrometry"`: Zenodo `10.5281/zenodo.21185812` (Quaoar, 14 a Sternbedeckungen, cc-by-4.0, `Quaoar_paper.zip`), Zenodo `10.5281/zenodo.10620251` (TNBFits.zip), A&A 2020 `aa39054-20` (37 Occultation-Astrometrien, Lucky Star) | neue Astrometrie-Direktlinien-Kandidaten; Occultation-DB-URL (`J.Phys.Conf.Ser.` 1365, 012024) pending — Nachzug 2026-09-17: DES/OSSOS/Gaia-SSO-TNO in `sources.φ` registriert (`des_y6_tno.bin`:1565, `ossos_tno.bin`:1571, `gaia_sso_tno.bin`:1687) |

Kometen (`dcom5_comets`, `cometels_flat.json`) und TNO-Gaia/DES/OSSOS sind
in-register; Gravimeter (IGETS) und Infraschall (BGR) sind geschlagen und
gebaut — nicht neu gefahren. Wayback/CDX und lib.rs meldeten 503/403
(Quellen-Zustand, keine dieser Klassen inhaltlich).
