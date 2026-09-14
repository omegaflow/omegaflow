<!--
  title: Survey — Warteliste: offene Alternativen (Stand 2026-09-14, Runde 2)
  class: survey
  date: 2026-09-14
  sha256: f2188370d0225a20268fcf50bd44186eefc1d92ae494a441e64acd6d8197bc86
  status: live
  see-also: docs/handover/handover-2026-09-14-entscheid-folge6.md phi/pipeline/ledger.φ
-->
# Warteliste — offene Alternativen (Stand 2026-09-14, Runde 2)

Zwei Runden günstiger Taucher (grind-flash, token-limitiert; rollierende
Proton-VPNs, Brave-Suche, Playwright) haben für die extern gebundenen
Wartepunkte (Handover `entscheid-folge6`, §Warten auf Rückmeldung) offene
Datenquellen gesucht und gemessen (curl HTTP-Code, 2026-09-14). Ziel: das
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
| Juno gravity science | `atmos.nmsu.edu/PDS/data/jnogrv_1001/` — rohe ODF/TNF/RSR/OLF, anonym | 200 (ODF/TNF/RSR/OLF) |
| LISA Pathfinder (roh) | `heasarc.gsfc.nasa.gov/FTP/lpf/data/fits/` — 72 rohe DRS-FITS + `summ/` | 200 (Range-GET 206) |

## Teilweise hinfällig (Teilroute gemessen)

| Warte | gemessene Teilroute | was fehlt |
|---|---|---|
| Voyager Roh-Doppler | PDS-Rings `pds-rings.seti.org/pds4/bundles/voyager_rss_raw/` — rohe **ODR** (Open-Loop, Okkultation): VG1 Jupiter, VG2 Jupiter, VG2 Uranus/PODR, anonym | closed-loop DSN-Doppler ODF/TRK-2-34/TNF — nur Cassini/Maven/DART tragen TRK-2-34-Bundles, nicht Voyager |
| DAHITI | Zenodo `records/17928117` (Fluss-WSE), `records/21291632` (Aserbaidschan-Pegel) — cc-by-4.0, anonym | die volle DAHITI-Serie bleibt hinter `api_key` (`dahiti.dgfi.tum.de/api/v2/...` 403) |
| NSE/Haug | IOP NJP 12, 105006 (2010) Fig. 5b/6 + arXiv 1008.4298 + Stuttgarter Diss. `impulse.mlz-garching.de/record/2120` | die rohen/reduzierten TRISP-NSE-Dateien bleiben bei MPI-FKF (kein ILL/MLZ/MPG-DOI) |

## Bleibt offen (keine offene Route gemessen)

| Warte | Befund | nächster Schritt |
|---|---|---|
| Voyager Roh-Doppler (closed-loop) | kein öffentliches ODF/TRK-2-34 für Voyager gemessen | JPL/DSN-Anfrage hält (nur für closed-loop) |
| CSES-Limadou | kein anonymer Pfad; `cses.space` 000, CDPP-Archiv nur DEMETER + Registrierung | ASI SSDC `limadou.ssdc.asi.it` selbst registrieren (keine CN-Mobilnummer nötig) |
| NSE/Haug (Rohdaten) | kein maschinenlesbares Deposit (2010 vor Open-Data-Politik) | Diss.-Volltext `impulse.mlz-garching.de/record/2120` holen / Fig. 5b digitalisieren |
| DAHITI (volle Serie) | `dahiti.dgfi.tum.de` API 403 ohne `api_key` | Registrierung oder Per-Station-DOI |

## Register-Disposition

Die hinfällig machenden Routen sind als `ausstehend kandidat` in
`phi/pipeline/ledger.φ` eingetragen (Runtime-Zustand, nicht committet). Die
Wartepunkte selbst leben im Handover `entscheid-folge6` §Warten auf Rückmeldung;
wo eine offene Route gemessen ist, ist das Warten hinfällig.

## Identität geklärt

**NSE/Haug** = die Neutron-Spin-Echo-Zwischenstreufunktion I(q,t) von
unterdotiertem YBa₂Cu₃O₆₊ₓ aus Haug et al., *New J. Phys.* 12, 105006 (2010),
gemessen an **TRISP** (MLZ/FRM-II, MPI-FKF). Beleg: `state/mail/mail_ledger.φ:8`
(gesendete Mail) und `docs/handover/archiv/handover-2026-09-12-entscheid-folge3.md:59`.
