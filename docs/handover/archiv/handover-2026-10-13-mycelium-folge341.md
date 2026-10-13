<!--
  title: Handover — Mycelium-Folge 341 (2026-10-13)
  class: handover
  date: 2026-10-13
  sha256: d7c32d91a5fe7fe74647a9c264f4f1f3716bd5e7900186439ce5a6a7fc62d424
  status: live
-->
# Handover — Mycelium-Folge 341 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-13-mycelium-folge340.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.0393 — gemessen `opencode.db` beim Handover-Schreiben (offen; wächst bis Session-Schluss); deepseek-flash, kein pro/max (Taucher-Schwarm separat).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „hast du die ttls gewürfelt?" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md` |
| „mache weiter" (weitergetragener folge337-Bericht) | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md` |
| „ja und den rest bauen schick einfach agenten" | 2026-10-12 | `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass … /commit (Push), danach den frischen Pass" | 2026-10-12/13 | session-weiter Delegations-Consent |
| „ab jetzt Reports immer in einfacher Sprache; Maschinensprache untereinander ok" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md`; `AGENTS.md` (Language doctrine) |
| „also du verteilst jetzt schön an einen Taucher-Schwarm und erledigst die Arbeit" | 2026-10-13 | Operator (Session 341) · `state/operator-gespraeche/2026-10-13-mycelium.md` |

## Offen — eigen

### Ersatzquellen-Registerblöcke — Formate der 4 Restcompiler (Klassen-Träger)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Commit der Block-Emission + Register-Eintrag
- **Lage:** (gemessen 2026-10-13/341, Taucher) die Formate stehen: `heasarc_w3browse` → `format heasarc_w3browse` · `jpl_sbdb` → `format catalog_mpcorb` · `gesdisc_gnssro` → `format cosmic_ro` · `opentopography_raster` → **kein `format`** (`witness gestalt` + `record srtm`; Parser-Arm in `witness.rs:24-25`). Die 4 Compiler **emittieren den vollständigen Block bereits im Arbeitsbaum** (uncommitted `M`); der HEAD-Stand tut es nicht. SRTM-Kollision: getrennte Blöcke (Remote-Witness `phi:29365` von der CDN-Gate ausgenommen; der Compiler materialisiert ein bbox-Subset als eigenes CDN-Asset). Notiz: `state/mycelium/restcompiler-formate-2026-10-13.md`. An Mountain: `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af`.
- **Blockade:** Format/Block-Emission ist Mountains Register-Recht; die Compiler-Änderung ist fremd gehalten.
- **Braucht:** Mountain committet die Block-Emission der 4 Compiler + registriert sie.

### LHAASO/VizieR `J/ApJS/271/25` (Ersatz für `casdc`)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `at`/`field` + das CI-Fetch-Wiring
- **Lage:** (gemessen 2026-10-13/338) Block `phi/sources.φ:10799-10802`; Compiler `cargo build -p omegaflow-harvest --bin lhaaso_compiler` grün; liest ASU-TSV. An Mountain: `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91`.
- **Blockade:** `at`/`field` = Mountains Recht; kein CI-Workflow ruft `lhaaso_compiler`.
- **Braucht:** Mountain `at sun`/`field`; danach das Fetch-Wiring.

### OCNS2019 — Ersatzblock für `geoazur`/APDB (Mycelium-Teil gemessen; terms/field offen)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms` + `field`
- **Lage:** (gemessen 2026-10-13/341, Taucher) Endpunkt + Format geklärt: **einziger erreichbarer Weg ist der Wayback-Snapshot** `web.archive.org/web/20220609074246/http://paperdata.china-vo.org/yuanye/ocns2019.zip` (HTTP 200, 8 620 636 B, magic `zip`, sha256 `7b7ad66b…)`; Paper-DOI `10.1051/0004-6361/202038776` (A&A 645 A48). Zip-Inhalt = ASCII-PSV: `ocns2019/OCNS2019_full.csv` + `debias_hires_OCNS2019.csv` + `ReadMe.txt`; Paper-Spalten `obsTt`/`obsTrg`/`rsdIcrsA`/`rsdIcrsB` (ICRS/TT, Δα·cosδ, Δδ). **Feld-Vorschlag** (2 Zeilen, Kernel `inverse-square`, Force `em`, Einheit `deg`): `ocns2019_neptune_satellite_ra_deg` / `..._dec_deg`. Entwurf `state/mycelium/block-drafts/welle3-rest-nuts.φ` (Duplikat: `welle2-planet.φ` trägt denselben Block mit `terms unclear` — beide Entwürfe, **keiner** im Register). **Offen (2 Token):** der rohe PSV-Spaltenname (aus `ReadMe.txt`) und `tau`. **Lizenz (gemessen 2026-10-13/341):** DataCite führt kein `rightsList`, und die NADC/China-VO „Terms and Conditions" gewähren **keine offene Lizenz** („copyright … belongs to the original data authors … refer to the … data publisher's statement") — OCNS2019 ist **nicht lizenzklar**, der Riegel verweigert die Manifestation. Gefaltet: `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.10~7d5c9163`.
- **Blockade:** keine offene Lizenz an der Quelle; Host nur über Wayback (429 transient).
- **Braucht:** eine Lizenzklärung (China-VO) **oder** eine lizenzklare Ersatzroute für die Neptun-Astrometrie; danach die 2 `field`-Zeilen (Spaltenname aus `ReadMe.txt`) + Fold.

### Lizenzklare Alternativen — Register-/Compiler-Swap (future-248/249)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je Alternative ein Compiler + Block gebaut
- **Lage:** (gemessen 2026-10-13/341) Bau-Liste `state/mycelium/alternativen-endpunkte-2026-10-13.md`. **Gefaltet in folge341** (`890a0045`): VizieR II/288 (`irsf_ii288`) · Zenodo 15225820 (`resonance_schumann`) · BGR `BGR_infrasound_mb_hf_product` · BGR `Reference_Data`; Register kanonisch (2552 Blöcke, keine Duplikate). **BGR-Endpunkte gemessen:** `download.bgr.de/bgr/geophysik/Infrasound_mb_hf_product/netCDF/BGR_infrasound_mb_hf_product.zip` (ZIP→netCDF, CC BY-NC-SA 4.0) und `.../Infrasound_PMCC_reference/netCDF/Reference_Data.zip` (ZIP→netCDF, CC BY 4.0). **CDN-Takedown (3 Pässe, gemessen 2026-10-13/341):** 36 Orphan-Releases offline (377→341) + `egg.astro.cornell.edu` (ALFALFA, keine Lizenz) + `occultations.ct.utfpr.edu.br` (Orphan-Tag — **die Quelle selbst ist registriert** als `sosb_lucky_star`, `terms vizier-scientific-free`, Block-`url` unter Tag `vizier.cfa.harvard.edu`; kein registriertes Asset verloren) + `sdc.emiratesmarsmission.ae` (Orphan-Tag — **EMM ist mit unserem Konto zugänglich**; Authentifizierung ist kein Ausschlusskriterium, `AGENTS.md`; die EMM-Dust-Daten laufen bereits über Zenodo `emm_exi_cdod`; ein SDC-Block fehlt) → **338**. Gehalten `raw.githubusercontent.com` + `services.arcgis.com` (Register-tragend). `cdn_reconcile` danach: **orphan 10** — klassifiziert: PD/offen (`alasky…MoRIC` ODbL-1.0 · `bom.gov.au` CC-BY-4.0 · `content.cld.iop.org`/`iopscience.iop.org` CC-BY-4.0 · `planetarymaps.usgs.gov`/`ser.im-ldi.com`/`www2.census.gov`/`opentopodata.org`[GEBCO] PD · `mast.stsci.edu` PD-Default); `pending` = **0**: `gis.ccc.govt.nz` (Christchurch) gemessen — die CCC-„Developer and open data centre terms and conditions" (§2.1) erlauben Nutzen/Kopieren/Veröffentlichen/Anpassen für **jeden Zweck** → offen lizenziert (bespoke, kein CC); `www.kari.re.kr` = KARI-Public-Release-Term (gemessen). **Alle 10 Orphans PD/offen, `fragwürdig` = 0** — der CDN ist frei von fragwürdigen Assets. **5 weitere Ersatz-Blöcke gefaltet** (`bd83f39`: Fermi-4LAC-DR3-PD, MODVOLC-FIRMS keyless, WMM2025, mars-dust Zenodo, BL-narrowband Zenodo). **Tool-Riss:** 6 der ursprünglich 13 sind netloc-Tag-Divergenzen, keine echten Waisen. **Taucher-Auflösung (2026-10-13/341):** fast alle Endpunkte gemessen (LAMOST-Derivate 21231112/20645375, JADES 18328404, EMM EXI 10275974, CfA-CO 5501196, HI4PI 5500447, MAGIC PDR1 13898269, KiDS 17086764, DAWN 22734795, MIRAGE 21990891, PALAOA 967167, WorldClim, WMM2025, EDGAR SO2, ECCC GDPS, LoTSS/CASDA, COSMIC-RO, SRTM15+, PANGAEA 967167 CC-BY-4.0). **Risse (gemessen 2026-10-13/341, `archive_search --verdict`):** CHIME/FRB-Cat2 registrierte canfar-URL direct+Proton HTTP 500, kein Wayback-Snapshot; KASI `data.kasi.re.kr/api/KMTNet/search` direct+Proton 500, kein Snapshot; IRIS LMSAL direct+Proton 403 (Wayback-Snapshot 2022-01-20); IERS EOP Doppellinie (MAIA `finals.all` vs CDDIS-Verzeichnis); MPC Horizon als `block` (Bau) vs `compiler` (`horizons_compiler.rs`); drei Bau-Zeilen falsch als `block` klassiert (7600403 tar.gz / 5517308 netCDF / 4036589 zip — kein Leser). **Nachtrag (Operator-Nachfrage gemessen):** BGR-mb_hf terms — DataCite (autoritativ) = **CC BY-NC-SA 4.0** (Register korrekt); die Taucher-Behauptung eines DCAT-Aggregators „CC BY 4.0" ist **nicht gemessen** (Tavily HTTP 432, Mwmbl leer) — keine Riss-Behauptung ohne Messung. Tokens `xfer:mountain-336-2` · `xfer:river205-1`/`-2` · `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.1~8f771362`.
- **Blockade:** fehlende Leser für tar.gz/netCDF/grib2/HIPS; `terms`/`field`/`ttl` = Mountain.
- **Braucht:** je Alternative ein bounded step (Leser + Block), dann Mountains `terms`/`field`.

### CHIME/FRB Catalog 2 — registrierte Origin tot (canfar HTTP 500)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** eine registrierte, erreichbare Ersatz-Origin + Mountains `terms`
- **Lage:** (gemessen 2026-10-13/341) der registrierte Block `phi:28383-28387` (`chime_frb_cat2`, `format chime_frb_cat2`, `compiler chime_frb_compiler.rs`) trägt `origin https://www.canfar.net/storage/vault/file/AstroDataCitationDOI/CISTI.CANFAR/25.0066/data/table/chimefrbcat2.csv` — `--verdict` direct+Proton **HTTP 500**, kein Wayback-Snapshot. Ein gemessener erreichbarer Ersatz ist **nicht** vorhanden: VizieR führt `J/ApJ/947/83` nicht (TAP listet unter `J/ApJ/947/` kein `/83`); `J/A+A/698/A18/table1` ist ein Auswahl-Sample, nicht der Cat-2-Katalog. Der zwischenzeitlich gesetzte falsche VizieR-Endpunkt wurde auf die kanonische canfar-Kennung zurückgenommen (`a0561388a`) — kein erfundener Endpunkt im Register. Zwei Risse: (1) der Endpunkt fehlt; (2) `compiler tools/harvest/src/bin/chime_frb_compiler.rs` liest das canfar-CSV-Format (`dm_fitb`/`dm_exc_ne2001`/`dm_exc_ymw16`), nicht ein Ersatzformat. Die `terms`-URL (`…/25.0066/`) liegt auf demselben toten canfar-Wirt.
- **Blockade:** der `origin`-Wechsel ist Mycelium; `terms` ist Mountain.
- **Braucht:** ein gemessener erreichbarer Cat-2-Endpunkt (CADC/CHIME) **oder** eine gemessene lizenzklare VizieR-Kennung; danach das Compiler-Format anpassen und `origin`/`terms` setzen.

### Lese-Verdrahtung Fermi-4LAC/FINK + LHAASO-Zahlenwert (umgesetzt 2026-10-13/341)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** ein Format-Leser für 4LAC; das Register-Wort für das KM2A-Feld
- **Lage:** (gemessen/umgesetzt 2026-10-13/341) **FINK erledigt** — Block gefaltet (`e29991c`: `url .../api.lsst.fink-portal.org/fink_cutout.bin`, `terms CC-BY-4.0` [Zenodo `4036589`], `format fink_cutout`, `origin https://api.lsst.fink-portal.org/api/v1/cutouts`, compiler `fink_cutout_compiler.rs`; Leser-Arm `extract.rs:130` vorhanden). **LHAASO-Fix umgesetzt** (`79a89bb`): der Compiler skaliert N0 je Komponente nach SI (WCDA `1e-13`, E0 3 TeV; KM2A `1e-16`, E0 50 TeV), `--component WCDA|KM2A` (Default WCDA), Leerzeilen-Header-Bug gefixt; Register-`origin` um `Comp` erweitert (`878d0bf`), Workflow-URL angeglichen; `cargo build -p omegaflow-harvest --bin lhaaso_compiler` grün. **Fermi-4LAC:** der Block ist inzwischen gefaltet (`bd83f39`: `url …/fermi.gsfc.nasa.gov/table_4LAC.fits`, `format fits`, terms PD) — **offen bleibt der Format-Leser** in `src/archivar` (`fermi_4lac_dr3_compiler.rs` gibt keinen Format-Token aus). **Riss (LHAASO):** `lhaaso_sky1` trägt nur WCDA → die 21 KM2A-dominanten Quellen fehlen; das Feld trägt kein E0 (Einheiten-Riss). **CDDIS/NSSDC/SSDC:** CDDIS **schon implementiert** (`ionex_compiler.rs:45` sendet `Authorization: Bearer EARTHDATA_EDL_TOKEN`, verweigert ohne); NSSDC/SSDC-Schlüssel sind gesetzt, haben aber **keinen Konsumenten** im Code — der frühere „nur über Operator"-Vermerk ist widerlegt (Zugang mit den Schlüsseln messbar).
- **Blockade:** 4LAC braucht den Format-Leser; das KM2A-Feld ist Register-Recht.
- **Braucht:** ein 4LAC-Format-Leser (`src/archivar`) + Block; Mountains Wort für das KM2A-Feld/Asset.

### Lizenz-Verifikation der gefalteten Blöcke (gemessen 2026-10-13/341)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** ein erneut erreichbarer Host / neue Lizenzangabe
- **Lage:** (gemessen 2026-10-13/341) **57 gefaltete Blöcke an der Quelle geprüft**: 30 am Original bestätigt (20 DataCite/DOI, 7 kanonische CC-URLs, 3 inhaltlich: open_meteo CC-BY-4.0 ✓, igets_l1 CC-BY-4.0 ✓, irsf_ii288 vizier-scientific-free ✓); `openaq_pm25` war bereits korrekt (`docs.openaq.org/resources/licenses` = CC BY 4.0). **Korrigiert (block_commit `546f1000`):** `eccc_gdps_15km` terms-URL tot (404) → `https://open.canada.ca/en/open-government-licence-canada` (200); `srtm15_plus_globaldem` terms → **PD** (OpenTopography „free of all copyright restrictions"; Datensatz-Landing „Use License: Not Provided"). **pending:** `edgar_v81_so2` (kein Lizenz-Identifikator an der Quelle, nur Zitationsbedingungen), `argo_bgc` + `srtm15_plus_v2.5.5.nc` (Host `ERR_CONNECTION_CLOSED`), `cams_so2` (API-Wurzel), 17 PD-Blöcke (Domain 200, Lizenztext nicht content-verifiziert). **Zusatz-Riss:** `phi:8170` (Environment-Canada-ArcGIS-Block) trägt dieselbe tote ECCC-Lizenz-URL-Variante.
- **Blockade:** keine.
- **Braucht:** die `pending`-Lizenzen erneut messen, sobald der Host antwortet (Trigger); dann ggf. nachziehen.

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass oder eine `archive_search`-Arm-Änderung
- **Lage:** (gemessen 2026-10-13/340) auf den `archive_search --help`-Stand gesynct. Ruht.
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben.

## An mountain

Origin: mycelium-341 (2026-10-13):

- `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af` **Formate der 4 Restcompiler** (gemessen: `heasarc_w3browse`/`catalog_mpcorb`/`cosmic_ro`/opentopography witness+record) — die Compiler emittieren den Block bereits im Arbeitsbaum, nur uncommitted.
- `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91` **`at`/`field` für den LHAASO/VizieR-Block + CI-Fetch-Wiring** (`phi/sources.φ:10799-10802`).
- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.2~98652e66` **Ersatzquellen-Register + Risse** (MAXI/OSC/IGETS/MODVOLC/DES/ALFALFA/LoTSS/TeVCat) — Verdikt offen.
- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.3~7d22b061` **Welle 3: 2 Schranken** — HiPS-HEALPix-WCS-Arm in `src/archivar/fits.rs` nötig, 4LAC als eigene Quelle oder gestrichen?
- **CDN-Orphan-Takedown erledigt (folge341)** — 36 Releases offline; die zwei Register-tragenden (`raw.githubusercontent.com`, `services.arcgis.com`) gehalten. `xfer:mountain-336-1`.
- **Risse aus dem Taucher-Lauf:** CHIME/FRB-Cat2 canfar-URL tot (500) · KASI-API 500 · IRIS LMSAL 403 · IERS-EOP-Doppellinie · MPC-Horizon block-vs-compiler. `register_sort` gemessen **kanonisch** (2554 Blöcke). `xfer:mycelium-341-1`

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
