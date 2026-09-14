<!--
  title: Survey — Warteliste: offene Alternativen (Stand 2026-09-14)
  class: survey
  date: 2026-09-14
  sha256: 8e8ea47e3674804df1effe966bb6a132f2b357811027f3fbe791ae7501751503
  status: live
  see-also: docs/handover/handover-2026-09-14-entscheid-folge6.md phi/pipeline/ledger.φ
-->
# Warteliste — offene Alternativen (Stand 2026-09-14)

Fünf günstige Taucher (grind-flash, token-limitiert) haben für die extern
gebundenen Wartepunkte (Handover `entscheid-folge6`, §Warten auf Rückmeldung)
offene Datenquellen gesucht und gemessen (curl HTTP-Code, 2026-09-14). Ziel: das
Warten hinfällig machen. Kein Befund — die gemessenen Routen und ihr Verdikt.

## Hinfällig gemacht (offene Route gemessen, anonym)

| Warte | offene Alternative | Messung |
|---|---|---|
| ned-objdir Bulk-z | NED eigener öffentlicher TAP `ned.ipac.caltech.edu/tap/sync` (ADQL, CSV) | 200 `text/csv` |
| ned-objdir (Spiegel) | VizieR TAP `tapvizier.cds.unistra.fr/TAPVizieR/tap/sync` — GLADE+ VII/291, 2MRS J/ApJS/199/26, WISExSCOS J/ApJS/239/36 | 200 CSV/TSV |
| BiSON/Broomhall | GONG `gong2.nso.edu/` (p-mode Frequenzen; `gong_modes.bin` schon registriert) + SDO/HMI `jsoc.stanford.edu/` + VIRGO/SPM `soho.nascom.nasa.gov/data/` | 200 |
| New-Horizons-Doppler | PDS-SBN `pdssbn.astro.umd.edu/holdings/pds4-nh_rex:plutocruise_tnf-v1.0/` — TRK-2-34 TNF + Uplink-Tabellen | 200 |
| Rubin RSP-Datenrechte | Fink-LSST-Portal `api.lsst.fink-portal.org/api/v1/schema` (+ `/objects`) — LSST-Alerts + Lichtkurven, JSON | 200 |
| Copernicus Data Space | AWS Open Data `registry.opendata.aws/sentinel-1/` + STAC `earth-search.aws.element84.com/v1` (S2/Landsat/DEM) | 200 |
| ICIMOD RDS | RGI-Gletscherumrisse `glims.org/RGI/` + ICESat-2 `icesat-2.gsfc.nasa.gov/` | 200 |
| Borealis | Zenodo `zenodo.org/api/records` + Harvard Dataverse `dataverse.harvard.edu/api/search` | 200 |

## Bleibt offen (keine offene Route gemessen)

| Warte | Befund | nächster Schritt |
|---|---|---|
| Voyager Roh-Doppler | kein öffentliches ODF/ODR im PDS-PPI-Listing (nur abgeleitete ROCC); PDS-Rings nur abgeleitete OCC | JPL/DSN-Anfrage hält; weitere PDS-Volumes prüfen |
| Juno gravity science | kein offenes RSS-Produkt (`/mission/Juno/JNO/RSS` 200, `/data` 404) | JPL/NAV-Anfrage hält |
| CSES-Limadou | `cses.space` 000, `www.cses.ac.cn` 468; Swarm trägt die Ionen-Kennzahlen (registriert) | CDPP-Archiv `cdpp-archive.cnes.fr` (200, DEMETER) messen |
| NSE/Haug | kein Host-Signal; die Anfrage-URL fehlt | Anfrage-Text/URL benennen, dann messen |
| DAHITI | Hydroweb `hydroweb.theia-land.fr` 000; JRC GSW nur Ausdehnung, kein Pegel | Zenodo `q=DAHITI` je Station (DOI-Serie) + Sentinel-3-Altimetrie |
| LISA Pathfinder (roh) | ESA-Archiv `lpf.esac.esa.int` 403 (WAF/Auth); Papers/Zenodo offen | Selbstregistrierung 2026-09-15 |

## Register-Disposition

Die hinfällig machenden Routen sind als `ausstehend kandidat` in
`phi/pipeline/ledger.φ` eingetragen (Runtime-Zustand, nicht committet). Die
Wartepunkte selbst leben im Handover `entscheid-folge6` §Warten auf Rückmeldung;
wo eine offene Route gemessen ist, ist das Warten hinfällig.
