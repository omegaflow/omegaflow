<!--
  title: Auftrag — Wartend-Auflösung: die gemessenen Routen in Register und Compiler ziehen
  class: auftrag
  date: 2026-10-11
  sha256: 1afe5ee396e4e9f2db9c222120d81336dc641752952683a14d6e9514c7160364
  status: live
  see-also: state/zustand/wartend.φ docs/handover/archiv/handover-2026-10-11-mountain-folge312.md docs/SOURCE_PORT.md
-->
# Auftrag — Wartend-Auflösung (2026-10-11)

**Zweck.** Eine Taucher-Batterie über den vollen `archive_search`-Katalog hat 16 Warten
aus `state/zustand/wartend.φ` aufgelöst (Route gemessen offen; Wait gefallen) und für
die restlichen Routen gemessen. Dieser Auftrag trägt **jede gemessene Route wörtlich**
(URL, Status, Lizenz, Beweis-Kommando), damit die nächste Runde die Taucher-Staffel
**ohne Neumessung** loslassen und die Zeilen in `phi/sources.φ` + die Compiler bauen kann.
Kein Verdikt ohne Messung: jede Zeile nennt die Messung; `unmeasured` bleibt `unmeasured`.

**Arbeitsteilung.** Jeder Punkt ist `eigen` (Mountain). „Register-Zeile" heißt: einen
`url`/`terms`/`format`/`origin`/`compiler`/`at|on`/`ttl`/`field`-Block in `phi/sources.φ`
schreiben (Mountain-Verdikt-Schreibrecht). „Compiler" heißt: ein `tools/harvest/src/bin/*.rs`
nach der Vorlage `copernicus_icoads_compiler.rs`. Jeder Dispatch ist **ein begrenzter
Schritt** (eine Zeile bzw. ein Compiler), mit `cargo build -p omegaflow-harvest --bin <name>`
als Gate.

---

## Teil A — Register-Zeilen für die gemessenen Routen

### A1 · `vlf-awesome` — WALDO ersetzt den toten Stanford-Host
- **Route:** `https://waldo.world` (HTTP 200, 53 624 B) — das AWESOME-Datenheim.
  DataCite-DOIs (Landing HTTP 206): `10.48322/fzcf-f191` (broadband), `10.48322/6t7b-s324`
  (narrowband), `10.48322/fwte-dv13` (VLF/LF), via `https://spase-metadata.org/ISWI/NumericalData/AWESOME/…`.
- **Lizenz:** **CC0-1.0** (`https://doi.org/10.48322/fzcf-f191`); WALDO: „data released without restriction".
- **Beweis:** `archive_search --all "AWESOME VLF data"` + `--datacite "AWESOME VLF"` (CC0 in den
  DataCite-Rights) + `--verdict https://waldo.world`; `vlf.stanford.edu` direct/socks pending, nur
  Wayback `20100803100801`.
- **Schritt:** in `phi/sources.φ` die tote `vlf.stanford.edu`-`origin`-Zeile auf den WALDO-DOI
  umschreiben (`terms CC0-1.0 https://doi.org/10.48322/fzcf-f191`); `vlf_awesome_compiler.rs` +
  `vlf-awesome-cdn.yml` auf die WALDO-Quelle prüfen.

### A2 · `cassini-trk` — NMSU PDS (TRK-2-34/2-18)
- **Route:** `https://atmos.nmsu.edu/pdsd/archive/data/co-s-rss-1-sroc15-v10/` (Orbiter-Volumes
  `cors_0366…0372`, `SROC15_156/180/`, `CALIB/`, `CATALOG/`) und
  `https://atmos.nmsu.edu/pdsd/archive/data/co-ss-rss-1-scc4-v10/` (Cruise `scc4_233…244`).
  TRK-Spec: `https://pds-geosciences.wustl.edu/radiosciencedocs/urn-nasa-pds-radiosci_documentation/dsn_trk-2-34/dsn_trk-2-34.2017-05-03.pdf`.
- **Lizenz:** PD (NASA PDS; exakte URL `unmeasured`).
- **Beweis:** `sfetch … --links` (Volumes live), `archive_search --jina dsid=CO-S-RSS-1-SROC15-V1.0`.
- **Schritt:** `url`-Zeile auf die NMSU-Basis; **offen:** die exakte TRK-2-34-Datei in einem
  `cors_*`-Volume öffnen (`unmeasured`).

### A3 · `juno-jplnav` — NMSU PDS (Juno-ODF)
- **Route:** `https://atmos.nmsu.edu/PDS/data/jnogrv_0001/DATA/ODF/` (Dateien `GRV_OCRU_*.ODF`).
- **Lizenz:** PD (NASA PDS).
- **Beweis:** `--verdict …/DATA/ODF/` → 200/11411 B; `sfetch …/ODF/ --links` → echte ODF-Dateinamen.
- **Schritt:** `url`-Zeile; die JPL-NAV-Mail ist für den Gravity-ODF gegenstandslos.

### A4 · `flyby-odf-esoc-datenpaket` — ESA PSA Rosetta RSI
- **Route:** `https://archives.esac.esa.int/psa/ftp/INTERNATIONAL-ROSETTA-MISSION/RSI/RO-X-RSI-1-2-3-EAR2-0061-V1.0/`
  (Unterordner `DATA/ CALIB/ CATALOG/ DOCUMENT/`); nur **EAR2** im PSA-RSI-Baum.
- **Lizenz:** `unmeasured` (ESA PSA).
- **Beweis:** `sfetch …/RSI/ --links` + `sgrep EAR` (nur EAR2).
- **Schritt:** `url`-Zeile; EAR1/EAR3 bleiben der Hoffmann-Mail-Wait.

### A5 · `flyby-odf-kinetx` + A6 · `flyby-odf-turyshev` — PDS-Korpus (drei Häuser)
- **Galileo:** `https://pds-ppi.igpp.ucla.edu/annex/GO-X-RSS-1-ODR-V1.0/`; PDS4-Bundle
  `https://pds-rings.seti.org/pds4/bundles/gll.rss/gll.rss.raw`.
- **Cassini 1999:** `co-ss-rss-1-scc*-v10` (wie A2).
- **MESSENGER:** `https://pds-geosciences.wustl.edu/messenger/urn-nasa-pds-mess-rs-raw/data-odf/`
  (Collection `urn:nasa:pds:mess-rs-raw:data-odf`, ODF vom JPL Multi-Mission Navigation).
- **Lizenz:** PD (NASA PDS).
- **Beweis:** `--exa "GO-X-RSS-1-ODR"`, `--jina urn:nasa:pds:mess-rs-raw:data-odf`, `sfetch --links`.
- **Schritt:** `url`-Zeilen; **offen:** die exakten Earth-Flyby-ODF-Dateinamen (Galileo 1990/92,
  MESSENGER 2005) aus den Cruise-Collections öffnen (`unmeasured`).

### A7 · `ephemeris-juice-recon` (+ `flyby-path2-recon` SPK-Teil) — ESAC JUICE orbc SPK
- **Route:** `https://spiftp.esac.esa.int/data/SPICE/JUICE/kernels/spk/juice_orbc_000116_230414_310721_v01.bsp`
  (HTTP 206); Meta-Kernel `…/mk/juice_ops_v473_20261011_001.tm`. `orbc` = rekonstruierte + prädizierte
  Cruise-Bahn (Flight Dynamics), post-Perigäum-Zyklus vorhanden.
- **Lizenz:** `unmeasured` (ESA/ESAC SPICE).
- **Beweis:** `--verdict …/juice_orbc_…bsp` → 206; `sfetch …/spk/aareadme.txt`.
- **Schritt:** `url`-Zeile; River-Gate konsumiert `flyby_ephemeris_gate --recon <orbc.bsp>` direkt
  (Deadline 2026-11-01 wird gegenstandslos). **1-σ-Kovarianz** bleibt offen (nicht im SPICE-Baum).

### A8 · `themis-gmag` — CDAWeb HAPI (190 GMAG-Stationen)
- **Route:** `https://cdaweb.gsfc.nasa.gov/hapi/dataset?id=THG_L2_MAG_<CODE>&time.min=…&format=csv`
  über `https://cdaweb.gsfc.nasa.gov/hapi/info?id=THG_L2_MAG_<CODE>`; 190 IDs (ABK, ATU, BMLS, CBB …).
- **Lizenz:** UCLA IGPP THEMIS Data Policy `https://themis.igpp.ucla.edu/roadrules.shtml` (200).
- **Beweis:** `curl …/hapi/info?id=THG_L2_MAG_ABK` → 200, 2007-01-01→2026-10-09; Catalog-AWK → 190 IDs.
- **Schritt:** je Station eine `url`-Zeile (oder ein Serien-Block); keyless, kein Key.

### A9 · `wind-swe-mfi` — CDAWeb HAPI
- **Route:** `https://cdaweb.gsfc.nasa.gov/hapi/dataset?id=WI_K0_SWE` (Np/V_GSE/V_GSM/Pressure,
  1994-11-01→2026-10-06) und `…?id=WI_H2_MFI` (BF1/BGSM/BGSE, 1994-11-13→2026-09-27).
- **Lizenz:** NASA SPDF open (exakte SPD-41-URL `unmeasured`).
- **Beweis:** `curl …/hapi/info?id=WI_K0_SWE` / `WI_H2_MFI` → 200 mit Coverage.
- **Schritt:** zwei `url`-Zeilen; danach die GIC-Route verdrahten.

### A10 · `carisma-zugang` — keyless CSA/Univ.-Alberta
- **Route:** `https://www.carisma.ca/data` (200) + `https://www.carisma.ca/carisma-data-repository` (200);
  `https://open.canada.ca/data/en/dataset/06f5e364-6e2c-4d1c-95c2-9fb7d871ca20` (200);
  `https://data.space-environment.ca/data-archive` (200); Zenodo-Slice `https://doi.org/10.5281/zenodo.10594301`.
- **Lizenz:** Zenodo-Slice = **CC-BY-4.0**; open.canada.ca = OGL-Canada.
- **Beweis:** `--all "CARISMA magnetometer data"` + `--verdict`; HAPI CDAWeb/AMDA tragen **kein** CARISMA
  (`sgrep -c carisma` = 0), `data.carisma.ca` 403.
- **Schritt:** `url`-Zeile auf carisma.ca/open.canada.ca; die Registrierungs-Mail ist gegenstandslos.

### A11 · `ssdc-limadou` — keyless CSES-Limadou
- **Route:** `https://www.cseslimadou.it/public-data-download/` (200); `https://www.leos.ac.cn/` (206);
  `https://limadou.ssdc.asi.it/` (200, ASI SSDC); Zenodo `https://zenodo.org/records/19219097`.
- **Lizenz:** `unmeasured` (CSES/LIMADOU-Policy, kein offener Token gefunden).
- **Beweis:** `--all "CSES L2 electron flux"` + `--verdict`; AMDA/CDAWeb HAPI → `cses`/`limadou` = 0;
  **`cses.org` ist die Wahlstudie, kein Satellit** (Falschspur benannt).
- **Schritt:** `url`-Zeile auf `cseslimadou.it/public-data-download`; CAS-Login nicht mehr nötig.

### A12 · `gosat-gw` — keyed Route lebt
- **Route:** `archive_search --gosat GWT3F_L1B` → **980 TANSO-3-L1B-Produkte** (z. B.
  `TANSO3_202610110904036NO1F35001400_1BO00_101101.h5`, 17 244 194 B); Portal
  `https://product.gosat-gw.nies.go.jp/` (200). Credentials `GOSAT_GW_MAIL`/`GOSAT_GW_PASS` liegen im Werkzeug.
- **Lizenz:** TANSO-3 Data Policy („free of charge", „open to all users", Attributionspflicht, keine
  ausdrückliche Redistributions-Grant): `https://www.nies.go.jp/soc/doc/TANSO-3_DataPolicy_en_20260622b.pdf`.
- **Beweis:** `archive_search --gosat GWT3F_L1B` (980 Zeilen), `--pdf-text` der Policy.
- **Schritt:** im Werkzeug-Katalog (`docs/concepts/tools-map.md` / `--help`) den `--gosat`-Arm
  dokumentieren; `gportal.nies.go.jp` ist tot.

### A13 · `superdarn-globus-map` — keyless BAS-Mirror statt Globus
- **Route:** `https://api.bas.ac.uk/superdarn/mirror/v3/` (200; JSON-Routen `/api/v3/files`,
  `/radars`, `/coverage/temporal`; Doku `http://bslsuperdarnc.nerc-bas.ac.uk:8093/docs/`);
  `https://superdarn.ca/data-download` (FITACF keyless; RAWACF über FRDR
  `https://www.frdr-dfdr.ca/repo/collection/superdarn`); RST-Prozedur baut MAP lokal.
- **Lizenz:** `unmeasured` (SuperDARN Data Policy / BAS copyright).
- **Beweis:** `--all "SuperDARN MAP convection data download"` + `curl api.bas.ac.uk/superdarn/mirror/v3/`.
- **Schritt:** `url`-Zeile auf die BAS-API; das Operator-Wort „warte bis Glasfaser" galt dem
  **Globus-Transfer**, nicht dieser keyless Route.

### A14 · `voyager-nssdca` (Teil-Route) — PDS Ring-Moon Systems
- **Route:** `https://pds-rings.seti.org/pds4/bundles/voyager_rss_raw/` (Voyager 1/2 Jupiter RSS);
  gebündeltes Volume `https://pds-rings.seti.org/viewmaster/archives-volumes/VG_28xx/VG_2803.tar.gz`.
- **Lizenz:** PD (NASA PDS).
- **Schritt:** `url`-Zeile; das NSSDCA-CDW-Set (VGR_8201–8245) bleibt `wartend`.

### A15 · `viking-nssdca` (Teil-Route) — PDS Geosciences
- **Route:** `https://pds-geosciences.wustl.edu/premgn/mg_1001/vikgrav/vmar001n.dat` (derivierte Gravitation).
- **Lizenz:** PD (NASA PDS).
- **Schritt:** `url`-Zeile; das Roh-PSPG-Set bleibt `wartend`.

### A16 · `mariner10-nssdca` (Teil-Route) — PDS/PPI Trajektorie
- **Route:** `https://pds-ppi.igpp.ucla.edu/mission/Mariner%2010` (M10-H-POS-6-M1/M3-FLYBY-TRAJ,
  DOI `10.17189/1519736`/`…1519737`); IPN-Report `https://ipnpr.jpl.nasa.gov/progress_report/42-21/21Q.html`.
  NAIF `MARINER10` = **404**.
- **Lizenz:** PD (NASA PDS); NSSDCA-Roh-Set (PSCM-00009) nur offline.
- **Schritt:** `url`-Zeile auf PPI; der Roh-Magnetband-Wait bleibt.

### A17–A20 · Bereits gebaut — nur den Riss schließen
- `copernicus-insitu-surface-marine` → Compiler **`copernicus_icoads_compiler.rs`** (sources.φ:10915).
- `copernicus-insitu-gruan-ua` → Compiler **`copernicus_cuon_compiler.rs`**, Dataset
  `insitu-comprehensive-upper-air-observation-network`.
- `noaa-global-hourly` → Compiler **`noaa_isd_compiler.rs`** (sources.φ:11649-11660).
- `noaa-oar-hourly-gdp` → Compiler **`gdp_drifter_compiler.rs`** (sources.φ:1130-1137).
- **Schritt:** die vier Wartend-Zeilen sind gegen den Baum veraltet (falsche Compiler-Namen); keine
  neuen Bauten.

---

## Teil B — Compiler-Bauten (Vorlage: `copernicus_icoads_compiler.rs`)

**Copernicus CDS — Route/Gate (gemessen):** API `https://cds.climate.copernicus.eu/api`;
`POST {API}/retrieve/v1/processes/{dataset}/execution` → Poll `{API}/retrieve/v1/jobs/{job}`;
Header `PRIVATE-TOKEN: <CDS_API_KEY>` (**401** ohne Token). Lizenz je Dataset **CC-BY-4.0**
(SPDX-Link + Provider-Policy-PDF). Format: die gebauten Compiler fordern `"data_format":"csv"`
(der Wartend-Vermerk „netCDF-Bulk" ist falsch).

- **B1** `insitu-observations-surface-land` (CC-BY-4.0 + global-land-observations-data-policy).
- **B2** `insitu-observations-near-surface-temperature-us-climate-reference-network` (USCRN).
- **B3** `insitu-observations-igra-baseline-network` (IGRA).
- **B4** `insitu-observations-woudc-ozone-total-column-and-profiles` (WOUDC).
- **B5** `insitu-observations-gruan-reference-network` (GRUAN).
- **B6** `insitu-observations-gnss` (GNSS).
- **Gate je Bin:** `cargo build -p omegaflow-harvest --bin <name>` grün; `CDS_API_KEY` vorhanden.

**NOAA S3 — keyless (anonyme Listung 200), Lizenz NODD („open to the public"):**
- **B7** `noaa-nos-coastal-lidar-pds` (Prefixe `dem/`, `entwine/`, `laz/index.html`) → `noaa_nos_lidar_compiler.rs`.
- **B8** `noaa-eri-pds` (per-Event-Ordner, 2005_Hurricane_Katrina/…) → `noaa_eri_compiler.rs`.
- **Riss:** `noaa-global-hourly` und `noaa-oar-hourly-gdp` sind **schon gebaut** (A19/A20).

---

## Teil C — offene Messlücken (in Teil A sichtbar, `unmeasured`)

- Die **Dateinamen** der Earth-Flyby-ODFs (A5/A6) und der TRK-2-34-Datei im Cassini-Volume (A2) sind
  nicht einzeln geöffnet.
- Die **Lizenz-URLs** von ESA PSA/SPICE (A4/A7), SuperDARN/BAS (A13), CSES (A11) sind ungemessen.
- Die **1-σ-Kovarianz** der JUICE-Rekonstruktion (A7) ist nicht im SPICE-Baum.

## Teil D — noch nicht auftragsreif (kein Schritt; bitte nicht mit Tauchern belegen)

- `demeter-18400` — kein keyless Mirror (PSA/SPDF/CDAWeb/AMDA absent); Operator-Wort 2026-10-08
  „bitte vertage CDPP" bindet.
- `mindat-api-level1` — kein keyless Bulk (OpenMindat tokenpflichtig; CC-SA nicht in Kraft); Wiedervorlage 2026-10-13.
- `inpop-license` — Daten „submitted to conditions" (`https://www.imcce.fr/mentions-legales`), **keine**
  freie Redistribution; nur die Calceph-Library ist CeCILL-frei.
- Credits (`deepseek`/`moonshot`/`zai`) — Riss: `deepseek` 50 req/Tag unbestätigt; `moonshot` China-Gate
  vs. 5M-Freitoken (Widerspruch); `zai` nur Storage frei, Open-Weights keyless.
- `basis-fellowship`, `rr-brustgurt`, `ox64-m2c`, Termine (`emodnet` 2026-10-19, `gaia-dr4` 2026-12-02,
  `bepicolombo` 2027-04-01, `europa-clipper` 2026-12-03) — reine Warten/Akte.
