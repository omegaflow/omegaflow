<!--
  title: Survey — Weltweite LiDAR-/Punktwolken-Landschaft (offene Quellen)
  class: survey
  date: 2026-10-10
  sha256: 8351d34a57765ba3f6838e9c608959a80f50336a08748a53b9053ec4fd37f12a
  status: live
  see-also: docs/handover/handover-2026-10-10-mycelium-folge297.md
-->
# Survey — Weltweite LiDAR-/Punktwolken-Landschaft

**Frage.** Welche **weltweiten** offenen LiDAR-/Punktwolken-Quellen gibt es, was ist
davon schon registriert, und was fehlt zum Bau.

**Provenienz.** Erhoben 2026-10-10 von vier Recherche-Tauchern (Europa · Amerika ·
Asien-Pazifik · Afrika/Nahost + Spaceborne), jede URL mit `archive_search --verdict`
gemessen (Leiter direkt → Proton → Wayback); eine Stichprobe
(`data.geo.admin.ch`, `hoydedata.no`, `data.linz.govt.nz`, `nsidc.org/…/atl08`,
`registry.opendata.aws/japan_pointcloud`) am Hauptstandort nachgemessen und bestätigt.
`HTTP 206 (1 byte)` ist ein CDN-/Proxy-Artefakt des Erreichbarkeits-Checks =
erreichbar, nicht Inhalt. `--verdict` misst **Erreichbarkeit**, nicht Lizenz/Format —
Format/Lizenz sind „laut Katalog" markiert oder `pending (unmeasured)`.

Diese Survey korrigiert einen **Riss**: die Mycelium-296-Zeile „omegaflow hat **kein**
LiDAR registriert (nur Hayabusa-PDS4-LIDAR)" ist falsch; 297 trug sie weiter.

## Was schon im Baum steht

- **USGS 3DEP** — registriert: `phi/sources.φ:20260`
  `usgs-lidar-public.s3.amazonaws.com/las_brooks_camp_2012.bin` (`format las`, `PD`,
  `las_compiler.rs`, Felder `las_x/y/z_icrs_m`). Host HTTP 200 (327891 B).
- **NOAA NOS Coastal Lidar** — Katalog `phi/pipeline/catalog/noaa_nodd_inventory.φ:26`
  (`s3://noaa-nos-coastal-lidar-pds`, Host 200, 386818 B).
- **LAS/COPC-Reader** — `src/archivar/las/` (Header/VLRs/COPC-Info+Hierarchie +
  `ept.json`); LASzip-Chunk-Dekoder gebaut (`laszip.rs`). Die beiden
  `blocked parser-def las-laz` (NOAA NOS, USGS 3DEP EPT) seit 2026-09-17 gelöst
  (`survey-2026-09-14-…:77`).
- **Copernicus DEM** — registriert (`copernicus-dem-cdn`, Predictor zentral in
  `src/archivar/tiff.rs`).

## Globale Deckung — spaceborne + Aggregatoren (die einzige echte Weltdeckung)

| Scope | Programm | URL | HTTP | Format | Lizenz | Zugang |
| --- | --- | --- | --- | --- | --- | --- |
| Global | NASA **GEDI L2A** (Höhenmetriken, Footprint) | earthdata.nasa.gov/data/catalog/lpcloud-gedi02-a-002 | 200 | HDF5/Waveform | PD | Earthdata-Login/bulk+S3 |
| Global | NASA **GEDI L4A** (Aboveground Biomass V2.1) | daac.ornl.gov/cgi-bin/dsviewer.pl?ds_id=2056 | 200 | HDF5 | PD | Earthdata-Login/bulk |
| Global | **ICESat-2 ATL08** (Land/Vegetation Height V7) | nsidc.org/data/atl08/versions/7 | 200 | HDF5 | PD | Earthdata-Login/bulk |
| Global | **ICESat-2 ATL03** (Geolocated Photons V7) | nsidc.org/data/atl03/versions/7 | 200 | HDF5 | PD | Earthdata-Login/bulk |
| Global | OpenAltimetry (ICESat-2/GEDI Web) | openaltimetry.earthdatacloud.nasa.gov | 200 | Web-Tool | PD | Web |
| Global | ETH/GLAD **Global Canopy Height** (ML aus GEDI) | glad.umd.edu/dataset/gedi | 200 | GeoTIFF ~10 m | CC-BY 4.0 | Download |
| Global | Global Canopy Height (Zenodo-Archiv) | doi.org/10.5281/zenodo.3939042 | 200 | GeoTIFF | CC-BY | Download |
| Global | **OpenTopography** (Portal/Katalog) | portal.opentopography.org | 200 | LAZ/DEM | je Datensatz | API-Key/Login |
| Global | OpenTopography GlobalDEM API | portal.opentopography.org/API/globaldem | 400 (Param.) | GeoTIFF/API | je DEM | API-Key |
| Global | Copernicus DEM (satellitär) | dataspace.copernicus.eu | 200 | GeoTIFF 30/10 m | Copernicus | Login/API |
| Global | OpenAerialMap (Drohnen-Orthos, **kein** LiDAR) | openaerialmap.org | 206 | Orthofotos | meist CC-BY | Web/API |
| Global | OpenLiDARMap (TUM, Crowd-Aggregator) | github.com/TUMFTM/OpenLiDARMap | 504 → 206 exit | Aggregator | s. Repo | Git/Web |
| Meta | European Point Clouds (TU Delft/EURO SDR) | europeanpointclouds.tudelft.nl/map.html | 206 | Meta-Katalog (+ `catalogue_data.json`) | — | API/Web |
| Meta | INSPIRE Geoportal / data.europa.eu | inspire-geoportal.ec.europa.eu | 206 | Discovery | — | API |

**Kernaussage:** GEDI + ICESat-2 sind die **einzige globale LiDAR-Deckung**
(homogene HDF5-Punkte, Earthdata-Login); alles bodengestützte LiDAR ist national
oder punktuell.

## Europa

**Bundesweit offen (Punktwolken):**

| Land | Programm | URL | HTTP | Lizenz | Zugang |
| --- | --- | --- | --- | --- | --- |
| NL | AHN4 | ahn.nl | 200 | offen (bedingungslos) | bulk/API |
| CH | swisstopo swissSURFACE3D | data.geo.admin.ch · swisstopo.admin.ch | 206/200 | swisstopo free | bulk/API (STAC) |
| DK | Dataforsyningen DHM | dataforsyningen.dk | 206 | frie geodata | bulk/API |
| NO | Kartverket/hoydedata | hoydedata.no · kartverket.no | 206/200 | NLOD/CC-BY | bulk |
| SE | Lantmäteriet Markhöjdmodell | geotorget.lantmateriet.se | 206 | CC-BY/offen | bulk/API |
| FI | NLS Laser 0,5 p | tiedostopalvelu.maanmittauslaitos.fi | 206 | CC-BY 4.0 | bulk |
| EE | Maa-amet (multitemporal) | geoportaal.maaamet.ee | 200 | open data | bulk |
| LV | LGIA Lidar | lgia.gov.lv | 206 | CC-BY | bulk |
| LT | geoportal.lt / data.gov.lt | geoportal.lt | 200 | CC-BY | bulk (Order) |
| PL | GUGiK ISOK | geoportal.gov.pl | 200 | offen | bulk |
| CZ | ČÚZK DMR 5G | cuzk.cz | 200 | offen | bulk/ATOM |
| SK | GKU DMR 5.0 / Open-Data | data.gov.sk | 200 | offen | bulk |
| SI | GURS CLSS | clss.si · geoportal.gov.si | 206/200 | CC-BY 4.0 | bulk |
| ES | PNOA LiDAR | pnoa.ign.es / centrodedescargas.cnig.es | **pending** (Timeout) | CC-BY 4.0 | bulk |
| PT | DGT / Dados LiDAR | cdd.dgterritorio.gov.pt · dados.gov.pt | 200 | CC-BY | bulk |
| FR | IGN LiDAR HD | geoservices.ign.fr/lidarhd · data.gouv.fr | 206/200 | Etalab | bulk |
| BE | Flandern DHMV-II / Wallonien | geopunt.be · geoportail.wallonie.be | 206/200 | open/CC-BY 4.0 | bulk |
| LU | BD-L-Lidar2024 | data.public.lu/…/lidar-2024… | 200 | **CC0** | bulk |
| IRL | GSI LiDAR | (GSI) | — | offen | bulk |
| UK | EA National LiDAR | environment.data.gov.uk | 200 | OGL v3 | bulk/API |
| UK-SCO | Scottish Remote Sensing Portal | remotesensingdata.gov.scot | 206 | OGL | bulk |
| UK-WLS | Lie Geoportal Wales | lle.gov.wales | 200 | OGL | bulk |
| AT | BEV ALS-Höhenraster / basemap.at | data.bev.gv.at · basemap.at | 200 | offen/CC-BY | bulk |
| HU | Lechner Knowledge Center | data.lechnerkozpont.hu | 200 | offen | bulk |
| HR | DGU Geoportal | geoportal.dgu.hr | 206 | offen | bulk |
| RS | RGZ DTM | rgz.gov.rs | 200 | offen | bulk |
| RO | ANCPI LAKI III | ancpi.ro | 200 | offen (gov) | bulk |
| IT | MASE Geoportale | gn.mase.gov.it | 200 | CC-BY | bulk |
| CY | data.gov.cy Höhenmodell | data.gov.cy | 200 | offen | bulk |

**Deutschland — kein bundesweit offenes Punktwolken-Portal:** der Bund (BKG
`gdz.bkg.bund.de`) liefert **bundesweite DEM-Raster** (dl-de/by-2-0), die
Punktwolken liegen offen bei den Ländern — Bayern (`geodaten.bayern.de`, 206),
NRW (`opengeodata.nrw.de`, 206), BW (200), Niedersachsen (200), Sachsen (laser
2015–2023, COPC); Hessen/Thüringen/Hamburg hier nicht erreichbar (`pending`).

**Geo-Blocks / Lücken (gemessen, kein Quellen-Verdikt):** `service.gsi.go.jp` (JP,
403 — Ersatz: AWS `japan_pointcloud`, 206) · `spatialdata.gov.scot` (403 direkt+Exit,
Wayback 200 → Access-/Geo-Zustand). `pending` (ohne Antwort): ES
`pnoa.ign.es`/`centrodedescargas.cnig.es`, RO `geoportal.gov.ro`, GR
`geoportal.gov.gr`, BG `gis.mrrb.government.bg`, SK `zbgis.sk`, PT `igeo.pt`,
DE `geodaten.hessen.de`/`geodaten.thueringen.de`/`geoportal.hamburg.de`.

## Nordamerika

| Land/Staat | Programm | URL | HTTP | Lizenz |
| --- | --- | --- | --- | --- |
| USA | USGS 3DEP (S3) | usgs-lidar-public.s3.amazonaws.com | 200 | PD |
| USA | NOAA Digital Coast | noaa-nos-coastal-lidar-pds.s3.amazonaws.com | 200 | PD |
| USA | AWS Open Data usgs-lidar | registry.opendata.aws/usgs-lidar | 206 | PD |
| CA | CNRA (SF Bay Delta) | data.cnra.ca.gov/dataset/2025-san-francisco-bay-delta-lidar | 200 | CC-BY |
| TX | TNRIS/StratMap | geographic.texas.gov/stratmap/elevation-lidar.html | 206 | PD |
| WA | WA Lidar Portal | lidarportal.dnr.wa.gov | 200 | PD |
| MN | MN Geospatial Commons | gisdata.mn.gov/en/dataset/elev-lidar-statewide-gen2 | pending | PD |
| NC | NC SDD / FRIS | sdd.nc.gov/sdd · fris.nc.gov/… | 522/403→wayback | PD |
| OR | DOGAMI Lidar | oregon.gov/dogami/lidar | 206 | PD |
| US-Terr. | PR/USVI im NOAA-Bucket | `…/laz/geoid18/9102/index.html` | 206 | PD |
| CA (Kanada) | CanElevation S3 | canelevation-lidar-point-clouds.s3.ca-central-1.amazonaws.com | 200 | OGL-Canada |
| CA | CanElevation Dataset | open.canada.ca/data/en/dataset/7069387e-… | 200 | OGL |
| CA-BC | LidarBC Portal | lidar.gov.bc.ca | 200 | OGL-BC |
| CA-ON | Ontario Classified Point Cloud | geohub.lio.gov.on.ca | 200 | OGL-Ontario |
| CA-QC | Québec LiDAR | open.canada.ca/data/en/dataset/757bb61a-… | 200 | OGL-QC |
| MX | INEGI (MDE/RDE) | inegi.org.mx/app/geo2/elevacionesmex | pending | Términos INEGI |
| MX | LiDAR Jalisco / Guadalajara | lidar.jalisco.gob.mx · lidar.guadalajara.gob.mx | pending→wayback | offen |

## Mittel-/Südamerika

| Land | Programm | URL | HTTP | Lizenz |
| --- | --- | --- | --- | --- |
| BR | INPE Embracedata LiDAR | embracedata.inpe.br/lidar/2024/ | 200 | offen |
| BR | São Paulo PMSP LiDAR (S3) | registry.opendata.aws/pmsp-lidar/ | 206 | CC |
| CL | IDE Chile / Datos | ide.cl · datos.gob.cl | 200 | offen |
| PE | Geoportal IDEP | geoidep.gob.pe | 200 | offen |
| EC | Geoportal IGM | geoportaligm.gob.ec | 503→wayback | offen |
| CO | IGAC Datos Abiertos | igac.gov.co/datos-abiertos | 301→403→wayback | offen |
| AR | IGN Datos Abiertos | ign.gob.ar/content/datos-abiertos/ | 200 | offen |
| AR | IGN cloud-native Spiegel | data.source.coop/nlebovits/ign-argentina/ | 206 | offen |
| UY | Catálogo Datos Abiertos (Vuelo Minas 2025) | catalogodatos.gub.uy | 200 | offen |
| BO | IDE-EPB | ideepb.geo.gob.bo | pending→wayback | offen |

## Asien-Pazifik

| Land | Programm | URL | HTTP | Lizenz |
| --- | --- | --- | --- | --- |
| JP | GSI 基盤地図情報 (403 direkt) — Ersatz AWS | service.gsi.go.jp · registry.opendata.aws/japan_pointcloud/ | 403 · 206 | pending |
| KR | NGII / data.go.kr | data.go.kr | 200 | offen |
| KR | VWorld | vworld.kr | pending→wayback | offen |
| TW | data.gov.tw DTM 20 m | data.gov.tw/en/datasets/176927 | 200 | TW-OGDL |
| TW | NLSC | nlsc.gov.tw | 200 | pending |
| HK | CEDD GEO Open Data LiDAR | ginfo.cedd.gov.hk/geoopendata/eng/LIDAR.aspx | 200 | pending |
| HK | DATA.GOV.HK / CSDI | data.gov.hk · portal.csdi.gov.hk | 200/206 | pending |
| IN | ISRO Bhuvan | bhuvan.nrsc.gov.in | 200 | pending |
| IN | NRSC Bhoonidhi | bhoonidhi.nrsc.gov.in | 200 | Login |
| CN | NESDC / LiDARNET / forestdata | nesdc.org.cn · lidar.pku.edu.cn · forestdata.cn | 200/206 | Registrierung |
| SG | SG PointCloud | sgpointcloud.gpslands.com | 200 | pending |
| SG | SLA Digitised Land Info | sla.gov.sg/geospatial/digitised-land-information/ | 403→wayback | SG-ODL |
| ID | Ina-Geoportal / DEMNAS | tanahair.indonesia.go.id · big.go.id | 206/200 | pending |
| PH | **LiPAD** (PHIL-LiDAR 1) | lipad.dream.upd.edu.ph | 206 | pending (Antrag) |
| TH | GISTDA Open Data LiDAR | opendata.gistda.or.th | 200 | pending |
| MY | MyGeoportal / MyGDI | mygeoportal.gov.my | 200 | pending |
| VN | HCMC Geoportal LiDAR 2012 | geoportal-stnmt.tphcm.gov.vn | pending (Timeout) | pending |
| AU | ELVIS | elevation.fsdf.org.au | 200 | pending |
| AU | Geoscience Australia DEM LiDAR 5 m | ga.gov.au · services.ga.gov.au/…/DEM_LiDAR_5m_2025 | 200/206 | CC-BY |
| AU-QLD | Queensland LiDAR | data.qld.gov.au/dataset/queensland-lidar-data-lidar-coverage | 202 | CC-BY |
| AU-NSW | NSW LiDAR + 1m DEM | data.nsw.gov.au/data/dataset/2022-23-lidar… | 200 | CC-BY |
| AU-VIC | Vicmap Elevation | land.vic.gov.au/…/vicmap-elevation | 403 | pending |
| NZ | **LINZ Data Service** | data.linz.govt.nz | 200 | **CC-BY 4.0** |
| NZ | LiDAR 1m DSM / Northland PC 2024 | data.linz.govt.nz/layer/122082-new-zealand-lidar-1m-dsm/ | 200 | CC-BY 4.0 |
| Pazifik | Pacific Data Hub | pacificdata.org | 403→wayback | pending |
| Pazifik | PGRSC Moana Data Service | pgrsc.org | 200 | pending |
| Guam | NOAA PDS | noaa-nos-coastal-lidar-pds…/laz/msl/551/index.html | 206 | PD |

## Afrika & Nahost

| Land | Programm | URL | HTTP | Lizenz |
| --- | --- | --- | --- | --- |
| ZA | Kruger NP Rivers LiDAR (2012) | catalogue.ceda.ac.uk/uuid/a2e82c7f… | 200 | CEDA Terms |
| ZA | UAV-LiDAR Savanne (Zenodo) | zenodo.org/records/7636076 | 200 | CC-BY |
| AO/TZ | LiDAR Bicuar/Mtarure (Edinburgh) | datashare.ed.ac.uk/items/313bd627-… | 200 | CC |
| MZ | CMS LiDAR Mangrove Zambezi (NASA ORNL) | earthdata.nasa.gov/data/catalog/ornl-cloud-cms-lidar… | 200 | PD |
| Pan-AF | Digital Earth Africa | digitalearthafrica.org | 200 | CC-BY |
| EG | Egyptian Survey Authority | esa.gov.eg | 200 | n/a |
| TR | HGM Harita Genel Müdürlüğü | harita.gov.tr | 200 | n/a |
| IL | MAPI/govmap | govmap.gov.il · mapi.gov.il | 403 (geo) | — |
| SA | GASGI | gasgi.gov.sa | pending | — |
| ZA | NGI | ngi.gov.za | pending (tot) | — |
| KE | KNSDI | knsdi.go.ke | pending (tot) | — |
| NG | NGIS | ngis.ng | pending (tot) | — |

**Kernaussage Afrika/Nahost:** keine offene nationale Massen-LiDAR-Deckung messbar;
Rückgrat bleiben GEDI/ICESat-2 + GLAD-Canopy + Copernicus DEM, Bodenkampagnen nur
nach Maß (CEDA/Zenodo/Edinburgh).

## Bathymetrie / Unterwasser (topo-bathy, Multibeam, globale Relief-Modelle)

Gemessen 2026-10-10 (`--verdict`, Leiter inkl. Proton). Unterwasser ist teils
multibeam-**Punktwolke**, teils **Grid** (aus Multibeam + Satellit abgeleitet).

| Scope | Programm | URL | HTTP | Format | Lizenz | Zugang |
| --- | --- | --- | --- | --- | --- | --- |
| Global | **GEBCO** (Grid 2024, inkl. Seabed 2030) | gebco.net | 200 | GeoTIFF/NetCDF ~15″ | frei (citation) | bulk |
| Global | **NOAA/NCEI ETOPO** (Global Relief) | ncei.noaa.gov/products/etopo-global-relief-model | 200 | NetCDF/GeoTIFF | PD | bulk |
| Global | **SRTM15+** (UCSD/Scripps) | topex.ucsd.edu/marine_topo/ | 200 | Grid | frei | bulk |
| Global | **Seabed 2030** (Nippon Foundation/GEBCO) | seabed2030.org | 200 | Program + Grid | frei | Web/API |
| Global | **NOAA NCEI Bathymetry** (Multibeam) | ncei.noaa.gov/maps/bathymetry/ | 206 | MB-System Punktwolke/Grid | PD | API/bulk |
| Global | **IHO DCDB** (Datenzentrum, via NCEI) | ngdc.noaa.gov/mgg/bathymetry/ | 200 | Multibeam Punktwolke | PD | bulk/API |
| Global | Marine Geoscience Data System | marine-geo.org | 200 | MB/Grid | divers | API/bulk |
| Global | PANGAEA (Kampagnen-Bathymetrie) | pangaea.de | 200 | divers (NetCDF/CSV) | CC-BY | API/bulk |
| EU | **EMODnet Bathymetry** | emodnet.ec.europa.eu/en/bathymetry | 200 | Grid DTM + Multibeam-Index | offen | API/bulk |
| Arktis | **IBCAO** | ibcao.org | **kein Response** (direkt+Proton), Wayback 2008 | Grid | frei | pending |
| Antarktis | **IBCSO** | ibcso.org | 200 | Grid | frei | bulk |
| AU | **AusSeabed** (Meeresboden AU) | ausseabed.gov.au | **403** direkt+Proton | MB/Grid | offen | blocked (Geo/CF) |
| US | Rolling Deck to Repository (R2R) | rvdata.us | **503** direkt+Proton | MB/Grid-Kampagnen | divers | pending |
| US | NOAA Digital Coast (bathymetry) | coast.noaa.gov/digitalcoast | 206 | Bathy-Punktwolke | PD | bulk |
| US | USACE JALBTCX Topo-Bathy LiDAR | spatial.usace.army.mil/…/JALBTCX…MapServer/0 | pending | LAZ (topo-bathy) | PD | REST |

**Kernaussage Unterwasser:** *Punktwolken*-Multibeam liegt bei NCEI/IHO DCDB,
Marine-Geo, PANGAEA und den Kampagnen-Repos (R2R/AusSeabed — teils blockiert);
die *globale* Abdeckung existiert nur als **abgeleitetes Grid** (GEBCO/ETOPO/
SRTM15+), nicht als Punktwolke. Für omegaflow heißt das: `slab2_depth.bin` ist da,
aber **kein** Bathymetrie-Multibeam-Manifestator.

### Alternative Routen für die blockierten/toten Hosts (gemessen 2026-10-10)

| blockiert/tot | Alternative Route | HTTP | was sie trägt |
| --- | --- | --- | --- |
| **IBCAO** (`ibcao.org` kein Response) | `www.ngdc.noaa.gov/mgg/bathymetry/arctic/` | **200** | NCEI/NGDC-Arktis-Bathymetrie-Seite (IBCAO-Grid-Auslieferung) |
| IBCAO (Datensatz) | `doi.pangaea.de/10.1594/PANGAEA.905295` | **200** | IBCAO-Dataset (PANGAEA-Mirror) |
| IBCAO (regional im Grid) | `www.gebco.net` | 200 | GEBCO 2024 trägt die IBCAO-Region |
| **AusSeabed** (`ausseabed.gov.au` 403; `data.ausseabed.gov.au` tot) | `portal.aodn.org.au` · `catalogue.aodn.org.au` | **200 · 200** | AODN (Australian Ocean Data Network) — Multibeam/Backscatter |
| AusSeabed (Behördenweg) | `www.ga.gov.au` | **200** | Geoscience Australia (AusSeabed-Programmträger) |
| **R2R** (`rvdata.us` + `data.rvdata.us` + `ieda.ldeo.columbia.edu` tot) | `www.marine-geo.org` | **200** | MGDS — trägt dieselben R2R-Multibeam-Fahrten |
| R2R (Kampagnen-Daten) | `www.bco-dmo.org` · `www.pangaea.de` | **206 · 200** | BCO-DMO / PANGAEA (Fahrt-/Proben-Daten) |
| R2R (Metadaten/Tools) | `github.com/rvdata` | **206** | R2R-Code/Metadaten-Repo |
| **JALBTCX Topo-Bathy** (REST pending) | `coast.noaa.gov/dataviewer/` | **206** | NOAA Digital Coast **Data Access Viewer** (der Download-Pfad) |
| JALBTCX (Katalog) | `catalog.data.gov/dataset?q=jalbtcx` | **206** | data.gov-Katalog der JALBTCX-Datensätze |
| Topo-Bathy (Bathy-Arm) | `www.ncei.noaa.gov/maps/bathymetry/` | 206 | NCEI-Bathymetrie (Multibeam-Punktwolke) |
| noch blockiert | `chs.coast.noaa.gov` | 403 (Geo) | — (Exit-Wechsel = Operator-Wort) |

Damit hat **jede** der vier problematischen Quellen eine erreichbare Route:
IBCAO → NGDC/PANGAEA/GEBCO · AusSeabed → AODN/GA · R2R → MGDS/BCO-DMO/PANGAEA ·
JALBTCX → Digital-Coast-Data-Access-Viewer + data.gov-Katalog.

## Was fehlt — der Bau

Reader (`src/archivar/las/`) und TIFF-Predictor stehen. Je Quelle fehlt ein
**Manifestator** (`tools/harvest`; Vorlagen `las_compiler.rs` /
`copernicus_dem_compiler.rs`): `fetch` des Endpunkts → Reader → `.bin` →
CDN-Aufrufer (`*-cdn.yml`). Die Registrierung (`url`/`origin`/`compiler`/`at`/
`terms`/`ttl`) ist Mycelium; `at`/`terms` folgen dem gemessenen Host.

**Reihenfolge (nach Deckung/Offenheit):**
1. **usgs-lidar-global** + **noaa-nos** je über einen Slab-/Chunk-Manifestator
   (statt Einzel-Fixture).
2. **open-lidar-data** (S3, keyless, COPC) — Bund-übergreifend: ein Key, Reader,
   Block + Aufrufer.
3. **NL AHN4 · CH swissSURFACE3D · FR IGN LiDAR HD · UK EA · NZ LINZ** — je
   Point-Cloud-Slabs.
4. **GEDI L2A/L4A + ICESat-2 ATL03/ATL08** — die globale Deckung (HDF5-Reader
   nötig; Earthdata-Login).
5. **GEBCO/ETOPO/SRTM15+** (Unterwasser-Grid) + **NCEI/IHO-DCDB-Multibeam** —
   Bathymetrie (Grid-Reader + Multibeam-Arm; `netcdf`/Grid im Archivar messen).
6. **Bayern DOM20** — COG-Kachel, `dom_compiler`.

Erster konkreter Schritt: einen LAZ/COPC-Key aus dem open-lidar-data-Bucket
(`curl` der S3-Liste) extrahieren, Reader gegen die echte Datei, dann Block +
`*-cdn.yml`.

## Offene Messungen — Proton-nachgemessen (2026-10-10)

Die zuvor als `pending` geführten Hosts wurden mit `archive_search --verdict`
**inkl. Proton-Stufe** (`socks5h://127.0.0.1:25344`) erneut gemessen:

- **Tot/kein Response auf direkt UND Proton, nur veralteter Wayback-Snapshot:**
  `pnoa.ign.es` (Wayback 2015) · `centrodedescargas.cnig.es` (2010) ·
  `geoportal.gov.ro` (2012) · `geoportal.gov.gr` (kein Snapshot) · `zbgis.sk`
  (2017) · `igeo.pt` (2002) · `gis.mrrb.government.bg` (2013) ·
  `geodaten.hessen.de` (2003) · `geodaten.thueringen.de` (kein Snapshot) ·
  `geoportal.hamburg.de` (kein Snapshot) · `ibcao.org` (2008) ·
  `geoportal-stnmt.tphcm.gov.vn` (2022). → **kein `pending` mehr**, sondern
  gemessen: Host aus dieser Sicht außer Betrieb/umbenannt.
- **Nach der Erinnerung doch erreichbar:** `geoservices.big.go.id` (**206**) ·
  `gis.data.alaska.gov` (**200**).
- **403 auf direkt und Proton (Geo/CF, Exit-Wechsel = Operator-Wort):**
  `ausseabed.gov.au`, `spatialdata.gov.scot`.
- **503 auf direkt und Proton:** `rvdata.us`.

Geo-Blocks sind Zugangs-Zustände, kein Quellen-Verdikt; ein weiterer Exit
(`bin/proton-wg.sh suggest <host>`) braucht Operator-Wort.

