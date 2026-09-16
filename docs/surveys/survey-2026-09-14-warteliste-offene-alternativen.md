<!--
  title: Survey — Warteliste: offene Alternativen (Stand 2026-09-14, Runde 2)
  class: survey
  date: 2026-09-14
  sha256: 510de3668190314ee6b20672fb89748c9799e0ba74f8a3fee9872ff0e130dd44
  status: live
  see-also: docs/handover/handover-2026-09-14-entscheid-folge6.md phi/pipeline/ledger.φ
-->
# Warteliste — offene Alternativen (Stand 2026-09-14, Runde 2)

Drei Runden Taucher (grind-flash + grind-pro, token-limitiert; rollierende
Proton-VPNs, Brave-Suche, Playwright) haben für die extern gebundenen
Wartepunkte (Handover `entscheid-folge6`, §Warten auf Rückmeldung) offene
Datenquellen gesucht und gemessen (curl HTTP-Code, 2026-09-14). Ziel: das
Warten hinfällig machen. Kein Befund — die gemessenen Routen und ihr Verdikt.

## Schon lokal gedeckt (`.secrets.local`, Repo-Root, gitignored)

Ein Taucher, der `.secrets.local` nicht liest, misst 403 und hält den Punkt
fälschlich für offen. Die folgenden Wartepunkte tragen ihren Schlüssel bereits
lokal — der Zugang existiert, es ist kein Warten.

| Warte | lokaler Schlüssel | gebaute Route |
|---|---|---|
| DAHITI (volle Serie) | `DAHITI_API_KEY` | `livefeed_gate --dahiti <id> --api-key <key>` liest `.secrets.local` (`secret_local`, `tools/gate/src/bin/livefeed_gate.rs:223`); `docs/specs/livefeed-gate.md:111` |
| ICIMOD RDS | `ICIMOD_USERNAME` / `ICIMOD_PASSWORD` / `ICIMOD_KNOX_TOKEN` | Zugang lokal |
| CSES-Limadou | `SSDC_USER` / `SSDC_PASS` (ASI SSDC) | Zugang lokal |
| Lasair / Rubin-Alerts | `LASAIR_TOKEN` / `LASAIR_LSST_TOKEN` | Zugang lokal |
| NASA ADS | `NASA_ADS_TOKEN` | `archive_search --ads` |
| Zenodo | `ZENODO_TOKEN` | `archive_search --zenodo` |

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
| NSE/Haug | IOP NJP 12, 105006 (2010) Fig. 5b/6 + arXiv 1008.4298 + Stuttgarter Diss. `impulse.mlz-garching.de/record/2120` + DTU-Orbit-Volltext-PDF `backend.orbit.dtu.dk/ws/files/9907046/plugin_1367_2630_12_10_105006.pdf` (2026-09-16 gemessen: 200, magic pdf, 1 341 734 B, sha256 f50e1ffd8783d731cf82a0be73fd57563746a778f3783525f9c6d2ec781ec6ec) | die rohen/reduzierten TRISP-NSE-Dateien bleiben bei MPI-FKF (kein ILL/MLZ/MPG-DOI); Präzedenz gemessen: Zenodo `10.5281/zenodo.18306252` (RESEDA/BaZrO₃, FRM-II, 2026) zeigt, dass FRM-II-Spin-Echo-Deposits existieren — der Haug/YBCO-Datensatz bleibt abwesend |

## Bleibt offen (keine offene Route gemessen)

Zwei `grind-pro`-Taucher (Runde 3) bestätigen die zwei Restoffenen mit Messung.

| Warte | Befund (Runde 3, gemessen) | nächster Schritt |
|---|---|---|
| Voyager Roh-Doppler (closed-loop) | Voyager closed-loop wurde als ATDF/ODF aufgezeichnet, aber nie an PDS freigegeben — offen überleben die open-loop ODR-Okkultation UND die Saturn-Encounter-Daten (UNIVAC-1108-Binär, closed-loop Doppler+Range: V1 `PSPA-00049`, V2 `PSPA-00123`, SPDF 200). PDS-PPI/Voyager trägt nur PWS + RSS-Doku; NAIF nur SPICE (`spk/lsk/pck`); natives ATDF/ODF/TRK-2-34 und das Cruise-/Post-Saturn-Fenster fehlen. Einzige nicht-anonyme Ablage dafür: NSSDC `PSNO-00007` (SDDPT, "archive, not distribution") | JPL/DSN-Anfrage (Cruise) hält (request-only); UNIVAC-1108-Parser für die Saturn-TARs offen |
| NSE/Haug (Rohdaten) | kein Deposit: arXiv `e-print/1008.4298` = nur TeX + 8 Figuren (null Datendateien); IOP-Suppdata nicht auflösbar (Radware-Bot-Manager); iMPULSE `record/2120` nur Metadaten + toter Volltext-OpenURL; MPG Edmond / Zenodo / Dataverse ohne Datensatz; DataCite ohne DOI | Keimer/MPI-FKF-Anfrage hält; Teilroute Fig 5b gemessen descoped (trägt Γ(T), nicht I(q,t)) |

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
