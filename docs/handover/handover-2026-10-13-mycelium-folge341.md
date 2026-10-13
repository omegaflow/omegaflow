<!--
  title: Handover — Mycelium-Folge 341 (2026-10-13)
  class: handover
  date: 2026-10-13
  sha256: 218f620a31ed9e6c58f244e917a95587c7862bc1d4071a3561d9943609ccbebc
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
- **Lage:** (gemessen 2026-10-13/341, Taucher) Endpunkt + Format geklärt: **einziger erreichbarer Weg ist der Wayback-Snapshot** `web.archive.org/web/20220609074246/http://paperdata.china-vo.org/yuanye/ocns2019.zip` (HTTP 200, 8 620 636 B, magic `zip`, sha256 `7b7ad66b…)`; Paper-DOI `10.1051/0004-6361/202038776` (A&A 645 A48). Zip-Inhalt = ASCII-PSV: `ocns2019/OCNS2019_full.csv` + `debias_hires_OCNS2019.csv` + `ReadMe.txt`; Paper-Spalten `obsTt`/`obsTrg`/`rsdIcrsA`/`rsdIcrsB` (ICRS/TT, Δα·cosδ, Δδ). **Feld-Vorschlag** (2 Zeilen, Kernel `inverse-square`, Force `em`, Einheit `deg`): `ocns2019_neptune_satellite_ra_deg` / `..._dec_deg`. Entwurf `state/mycelium/block-drafts/welle3-rest-nuts.φ`. **Offen (2 Token):** der rohe PSV-Spaltenname (aus `ReadMe.txt`) und `tau`. `terms` (China-VO PaperData) = Mountain. Gefaltet: `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.10~7d5c9163`.
- **Blockade:** `terms`/`field` = Mountains Recht; Host nur über Wayback (429 transient).
- **Braucht:** Mountain setzt `terms` + die 2 `field`-Zeilen (Spaltenname aus `ReadMe.txt`); dann Fold + CI-Manifestation.

### Lizenzklare Alternativen — Register-/Compiler-Swap (future-248/249)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je Alternative ein Compiler + Block gebaut
- **Lage:** (gemessen 2026-10-13/341) Bau-Liste `state/mycelium/alternativen-endpunkte-2026-10-13.md`. **Gefaltet in folge341** (`890a0045`): VizieR II/288 (`irsf_ii288`) · Zenodo 15225820 (`resonance_schumann`) · BGR `BGR_infrasound_mb_hf_product` · BGR `Reference_Data`; Register kanonisch (2552 Blöcke, keine Duplikate). **BGR-Endpunkte gemessen:** `download.bgr.de/bgr/geophysik/Infrasound_mb_hf_product/netCDF/BGR_infrasound_mb_hf_product.zip` (ZIP→netCDF, CC BY-NC-SA 4.0) und `.../Infrasound_PMCC_reference/netCDF/Reference_Data.zip` (ZIP→netCDF, CC BY 4.0). **CDN-Orphan-Takedown erledigt** (36 Orphan-Releases offline, 377→341; `raw.githubusercontent.com` + `services.arcgis.com` gehalten = Register-tragend). **Taucher-Auflösung (2026-10-13/341):** fast alle Endpunkte gemessen (LAMOST-Derivate 21231112/20645375, JADES 18328404, EMM EXI 10275974, CfA-CO 5501196, HI4PI 5500447, MAGIC PDR1 13898269, KiDS 17086764, DAWN 22734795, MIRAGE 21990891, PALAOA 967167, WorldClim, WMM2025, EDGAR SO2, ECCC GDPS, LoTSS/CASDA, COSMIC-RO, SRTM15+, PANGAEA 967167 CC-BY-4.0). **Risse (nicht geglättet):** CHIME/FRB-Cat2 registrierte canfar-URL tot (HTTP 500) → VizieR `J/A+A/698/A18/table1`; KASI `data.kasi.re.kr/api/KMTNet/search` 500; IRIS LMSAL direct 403 (nur Wayback 2022); IERS EOP Doppellinie (MAIA `finals.all` vs CDDIS-Verzeichnis); MPC Horizon als `block` (Bau) vs `compiler` (`horizons_compiler.rs`); drei Bau-Zeilen falsch als `block` klassiert (7600403 tar.gz / 5517308 netCDF / 4036589 zip — kein Leser); BGR-mb_hf terms-Riss (DataCite CC BY-NC-SA 4.0 vs DCAT-Aggregator CC BY 4.0). Tokens `xfer:mountain-336-2` · `xfer:river205-1`/`-2` · `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.1~8f771362`.
- **Blockade:** fehlende Leser für tar.gz/netCDF/grib2/HIPS; `terms`/`field`/`ttl` = Mountain.
- **Braucht:** je Alternative ein bounded step (Leser + Block), dann Mountains `terms`/`field`.

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
- **Risse aus dem Taucher-Lauf:** CHIME/FRB-Cat2 canfar-URL tot (500) · KASI-API 500 · IRIS LMSAL 403 · IERS-EOP-Doppellinie · MPC-Horizon block-vs-compiler. `register_sort` gemessen **kanonisch** (2552 Blöcke).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
