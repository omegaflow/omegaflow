<!--
  title: Handover — Mycelium-Folge 342 (2026-10-13)
  class: handover
  date: 2026-10-13
  sha256: 31be206fb29d7dc12397e80e7463257e4580af57066186ccc1fbd5b9260c946d
  status: live
-->
# Handover — Mycelium-Folge 342 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-13-mycelium-folge341.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.0386 — gemessen `opencode.db` (session `ses_ecaf157adffe`) beim Handover-Schreiben (offen; wächst bis Session-Schluss); deepseek-flash, kein pro/max (Taucher-Schwarm separat).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „hast du die ttls gewürfelt?" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md` |
| „mache weiter" (weitergetragener folge337-Bericht) | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md` |
| „ja und den rest bauen schick einfach agenten" | 2026-10-12 | `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md` |
| „kümmer dich bitte darum" (vier Endpunkt-Reste: ICGEM · gamma-cat · SEANOE · FDSN; dazu Fermi-4LAC · LHAASO-KM2A · OCNS2019) | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md` |
| „nein nur lokal OCNS2019 für die kalibrierung unserer ephemeriden so wie mit allem lokal für experimente sind nicht offene lizenzen ok aber nicht veröffentlichen" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md`; `AGENTS.md` (CDN-Manifestation, `local-only`) |
| „ab jetzt Reports immer in einfacher Sprache; Maschinensprache untereinander ok" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md`; `AGENTS.md` (Language doctrine) |

## Offen — eigen

### ENTSO-E A78 — Compiler-Request repariert; lebende Register-Zeile fehlt
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`
- **Lage:** (gemessen/umgesetzt 2026-10-13/342) der `entsoe_compiler` baute A78 mit `in_Domain` **ohne** `out_Domain` → HTTP 400 `invalidRequest` (Token akzeptiert, sonst 401). **Behoben** (`tools/harvest/src/bin/entsoe_compiler.rs`): `build_url` sendet `in_Domain` **und** `out_Domain`; `--out-domain <EIC>` optional, fehlt es, spiegelt `out_Domain` das jeweilige `in_Domain` (nicht stumm — jede Fetch-Zeile druckt `in -> out`). Belegt über `entsoe-py` (`entsoe.py:1162-1169`: A78 baut in+out_Domain). `cargo build -p omegaflow-harvest --bin entsoe_compiler` grün (0 Warnungen). **Offen:** kein lebender Eintrag in `phi/sources.φ` für ENTSO-E (`sgrep -i entsoe phi/sources.φ` leer) — `url`/`origin`/`compiler` = Mycelium, `terms`/`field` = Mountain.
- **Blockade:** `terms`/`field` = Mountains Recht; Register-Baum fremd-dirty.
- **Braucht:** Mountain `terms`/`field` für den ENTSO-E-A78-Block; danach `url`/`origin`/`compiler` (Mycelium) + Workflow-Dispatch.

### Vier Endpunkte — Adressen jetzt gemessen (ICGEM · gamma-cat · SEANOE · GEOFON-FDSN)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Zulassung + `terms`/`field`
- **Lage:** (gemessen 2026-10-13/342, Taucher) die vier „nicht messbaren" Adressen sind **messbar**:
  - **ICGEM** — Draft-Host war falsch; korrekt `icgem.gfz.de`: statische Modelldatei `https://icgem.gfz.de/getmodel/gfc/<sha256>/XGM2019e.gfc` (`--verdict` 206; `--sniff` 87 899 864 B, sha256 partial `b0c4e6dd…`). Lizenz: `citation`/`licence` je Modell im Header (FAQ:410), kein Blanket-Open.
  - **gamma-cat** — Einzelkatalog liegt in `output/`: `https://raw.githubusercontent.com/gammapy/gamma-cat/master/output/gammacat.ecsv` (`--verdict` 206; `--sniff` 200, 109 058 B, sha256 `9f1c75389f55eb3317d776cfcc015dcc327c1dfe952ba86c512374516761030d`). Lizenz **BSD-3-Clause**.
  - **SEANOE** — Dataset `https://www.seanoe.org/data/00311/42182/` (`--verdict` 206, 412 475 B, sha256 `6359909c6c7f38c409a371bdfac5df282b43041c2e0072f4f77b9f7906b9184f`); DOI `10.17882/42182` (DataCite-Record 200). **Lizenz nicht messbar** (`/licence/` 404, Seite rendert keinen Body) → `pending named`.
  - **GEOFON-FDSN** — Placeholder aus `state/mycelium/block-drafts/welle2-compiler.φ:1-8`; CDN-Asset `.../geofon.gfz-potsdam.de/fdsn_waveform.bin` **404** (nicht manifestiert); `geofon` fehlt in `phi/sources.φ`. Die FDSN-Klasse lebt bereits (EarthScope/GeoNet/USGS/GEOFON ungenutzt).
- **Blockade:** Zulassung/`terms`/`field` = Mountains Recht; kein `phi`-Eintrag.
- **Braucht:** Mountain-Zulassung; danach je Block `url`/`origin`/`compiler` (Mycelium) + Manifestation. An Mountain: `xfer:mycelium-342-1` … `-4`.

### Fermi-4LAC DR3 — Assetname-Riss + fehlender Format-Arm
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Wort: Roh-FITS als Asset **oder** `F4LC`-Arm
- **Lage:** (gemessen 2026-10-13/342) Block `phi/sources.φ:14553-14558` trägt `url …/fermi.gsfc.nasa.gov/table_4LAC.fits`, `format fits`, aber **keine `compiler`- und keine `field`-Zeile**. Der Compiler `fermi_4lac_dr3_compiler.rs` liest die Roh-FITS, schreibt ein `F4LC`-Bin und lädt als Asset **`fermi_4lac_dr3.bin`** — der Block erwartet `table_4LAC.fits` (Namens-Riss). `extract.rs:5048` kennt `format fits` (`fits_to_json`), aber keinen `F4LC`-Arm.
- **Blockade:** `src/archivar/extract.rs` ist fremd-dirty (Rivers CDF-Arm, uncommitted); `field` = Mountain.
- **Braucht:** Mountains Wort; danach `compiler`-Zeile + `field`-Mapping. An Mountain: `xfer:mycelium-342-5`.

### NVIDIA-NSSDC/SSDC-Schlüssel ohne Konsument
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eine Quelle, die den Schlüssel braucht
- **Lage:** (gemessen 2026-10-13/341) NSSDC/SSDC-Schlüssel sind gesetzt, haben aber **keinen Konsumenten** im Code; CDDIS ist bereits verdrahtet (`ionex_compiler.rs:45` sendet den Bearer). Der frühere „nur über Operator"-Vermerk ist widerlegt (Zugang mit den Schlüsseln messbar).
- **Blockade:** keine Quelle mit Bedarf.
- **Braucht:** beim nächsten NSSDC/SSDC-Kandidaten prüfen; ein Konsument ist der Schritt.

### OCNS2019 — lokal für die Ephemeriden-Kalibrierung (Disposition `local-only`)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Kalibrierungs-Lauf
- **Lage:** (gemessen 2026-10-13/342 via `curl`: sha256 der lokalen Kopie == die Wayback-Messung; Operator-Wort 2026-10-13 „nur lokal … nicht veröffentlichen"). Arbeitskopie geladen: `data/paperdata.china-vo.org/ocns2019.zip` (8 620 636 B, sha256 `7b7ad66b2cf1f53b48a8bd51a010437db21983620469231456c46d86366342ef`). **Kein** Eintrag in `phi/sources.φ`, **kein** CDN-Release; die Block-Entwürfe (`state/mycelium/block-drafts/welle3-rest-nuts.φ`, `welle2-planet.φ`) tragen `# LOCAL-ONLY — NICHT manifestieren`. Die Disposition `local-only` ist in `AGENTS.md` (CDN-Manifestation) verankert.
- **Blockade:** keine.
- **Braucht:** die Kalibrierung nutzt die lokale Kopie; kein Manifestationsschritt.

### Ersatzquellen-Registerblöcke — 4 Restcompiler (Klassen-Träger)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Commit der Block-Emission + Register-Eintrag
- **Lage:** (gemessen 2026-10-13/341, Taucher) Formate: `heasarc_w3browse` → `format heasarc_w3browse` · `jpl_sbdb` → `format catalog_mpcorb` · `gesdisc_gnssro` → `format cosmic_ro` · `opentopography_raster` → kein `format`. Die 4 Compiler emittieren den Block im Arbeitsbaum (uncommitted). An Mountain: `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af`.
- **Blockade:** Format/Block-Emission = Mountains Register-Recht.
- **Braucht:** Mountain committet die Block-Emission der 4 Compiler + registriert sie.

### JADES DR4 — Tag `jades.herts.ac.uk` ist keine lebende Quelle
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** eine registrierte JADES-Zeile in `phi/sources.φ`
- **Lage:** (gemessen 2026-10-13/342, `ci_manage log 38385822162`) `jades_spectra_compiler` läuft durch (`5190 records, 1 276 070 B — roundtrip parses`), aber der Upload verweigert: `tag jades.herts.ac.uk is not a live source in phi/sources.φ — a CDN release is only created for a registered source`. Der JADES-Block fehlt im Register.
- **Blockade:** Zulassung/`terms`/`field` = Mountains Recht.
- **Braucht:** Mountain-Zulassung; danach `url`/`origin`/`compiler` (Mycelium) + Re-Run `jades-cdn`. An Mountain: `xfer:mycelium-342-7`.

### Lokal-only + CDN-Bereinigung (Operator-Wort 2026-10-13) — gemessen/umgesetzt
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Register-Edits (CelesTrak-Block, WDC-Token)
- **Lage:** (gemessen 2026-10-13/342, drei Taucher)
  **(a) Lokale Arbeitskopien geladen** (`data/`, gitignored, kein `phi`-Eintrag): `data/datacenter.iers.org/EOP_20_C04_one_file_1962-now.txt` (5 067 517 B, sha256 `c17ba033…`) · `data/www.geoazur.fr/apdb_mars_transit_usno.txt` (194 616 B, sha256 `aa1b6cf7…`) · `data/data.kasi.re.kr/miris.json` (6 276 B, sha256 `8162f752…`; KASI-API jetzt 200, der 500 ist weg) · `data/icgem.gfz.de/XGM2019e.gfc` (540 638 936 B, sha256 `bd436324…`; Header = **CC BY 4.0**) · `data/www.ldeo.columbia.edu/gcmt_catalog.ndk` (28 367 820 B, sha256 `aae7913b…`) · `data/www.kari.re.kr/kdsa01.bsp` (8 192 B, sha256 `0eff0b07…`) · `data/vires.services/CS_OPER_MAG_2026-07-31.csv` (3 832 261 B, sha256 `ae1c43f2…`; HAPI-GET ohne Auth) · `data/celestrak.org/EOP-All.csv` (2 227 320 B, sha256 `fdcea807…`, Wayback 2022).
  **(b) Weitergabe-Prüfung der 6 — Korrektur:** **nur 3 davon sind CDN-Releases** (celestrak · wdc.kugi.kyoto · api.sedoo); KARI/VIRES/GCMT tragen **direkte Origin-`url`s** (kein CDN-Release, also keine Weiterverbreitung durch uns). Verdikte: **KARI KPLO NO** (Urheberrecht: Verbreitung untersagt) · **ESA VIRES NO** (ESA-Terms: Weitergabe nur nach ESA-Bestätigung) · **Columbia GCMT NO** („personal, noncommercial use only … may not transmit") · **CelesTrak UNCLEAR** (Policy nur Zugang, kein Verbreitungsrecht) · **AGRav YES** (nicht-kommerziell + Attribution) · **WDC Kyoto YES** (wissenschaftlich/nicht-kommerziell + Attribution; Register-Token `free-open` falsch).
  **(c) Offene Alternativen — für alle 10 gefunden** (`archive_search --all`, Volltext gelesen): geoazur→**IMCCE NSDC**/JPL SSD · KASI→**NOIRLab KS4**/NRAO VLBA · IERS-EOP→IERS/CDDIS/NAIF · ICGEM→**ESA GOCE**/GFZ-DOI · KARI→JPL/GFZ Mondkernels · CelesTrak→**USNO MAIA**/IERS/NOAA · VIRES→**GFZ Swarm**/INTERMAGNET (HAPI auth-frei) · GCMT→**USGS ComCat**/EarthScope · Kyoto→**NOAA NCEI**/ISGI/OMNIWeb · AGRav→**NOAA NCEI**/IGETS.
  **(d) CDN-Bereinigung:** die **CelesTrak**-Release (`celestrak.org`, Asset `celestrak_eop.bin`) ist **offline** (`gh release delete`, verifiziert „not found") — UNCLEAR-Recht, Inhalt PD über USNO/MAIA gedeckt; lokale Kopie gehalten. WDC Kyoto + AGRav bleiben (Weitergabe erlaubt).
- **Blockade:** die Register-Edits (CelesTrak-Block entfernen/überführen; WDC-Token `free-open` → `wdc-kyoto-scientific-noncommercial`) = Mountains Recht; `phi/sources.φ` fremd-dirty.
- **Braucht:** Mountain zieht den CelesTrak-Block ab und korrigiert den WDC-Token; danach sind CDN und Register deckungsgleich. An Mountain: `xfer:mycelium-342-9` · `xfer:mycelium-342-10`.

### CHIME/FRB Catalog 2 — registrierte Origin tot
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** eine registrierte, erreichbare Ersatz-Origin
- **Lage:** (gemessen 2026-10-13/342) canfar-URL direct+Proton **500**, kein Wayback-Snapshot; `chime-frb.ca/catalog` bestätigt canfar als einzigen Cat-2-Endpunkt. Kein erreichbarer Ersatz. `origin`-Wechsel = Mycelium; `terms` = Mountain.
- **Blockade:** kein erreichbarer Endpunkt.
- **Braucht:** CADC/CHIME-Ersatzendpunkt messen (Trigger) oder `descoped` mit Befund.

### Lizenz-Verifikation der gefalteten Blöcke
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** ein erneut erreichbarer Host / neue Lizenzangabe
- **Lage:** (gemessen 2026-10-13/341) **57 gefaltete Blöcke an der Quelle geprüft**: 30 am Original bestätigt; `eccc_gdps_15km` (tote URL → `open.canada.ca`), `srtm15_plus_globaldem` (→ PD) korrigiert. **pending:** `edgar_v81_so2`, `argo_bgc` + `srtm15_plus_v2.5.5.nc` (Host `ERR_CONNECTION_CLOSED`), `cams_so2` (API-Wurzel), 17 PD-Blöcke (Domain 200, Lizenztext nicht content-verifiziert). Riss: `phi:8170` (Environment-Canada-ArcGIS) trägt dieselbe tote ECCC-URL.
- **Blockade:** keine.
- **Braucht:** die `pending`-Lizenzen erneut messen, sobald der Host antwortet (Trigger).

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass oder eine `archive_search`-Arm-Änderung
- **Lage:** (gemessen 2026-10-13/340) auf den `archive_search --help`-Stand gesynct. Ruht.
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben.

## An mountain

Origin: mycelium-342 (2026-10-13):

- `xfer:mycelium-342-1` **ICGEM-Adresse gemessen** — Host `icgem.gfz.de` (nicht `icgem.gfz-potsdam.de/getmodel`); statisch `https://icgem.gfz.de/getmodel/gfc/<sha256>/XGM2019e.gfc` (206; sniff 87 899 864 B, sha256 partial `b0c4e6dd…`); Lizenz je Modell im Header.
- `xfer:mycelium-342-2` **gamma-cat-Einzeldatei** — `https://raw.githubusercontent.com/gammapy/gamma-cat/master/output/gammacat.ecsv` (206; 109 058 B, sha256 `9f1c7538…`); Lizenz BSD-3-Clause.
- `xfer:mycelium-342-3` **SEANOE-Dataset** — `https://www.seanoe.org/data/00311/42182/` (206; 412 475 B, sha256 `6359909c…`), DOI `10.17882/42182`; Lizenz `pending` (nicht messbar).
- `xfer:mycelium-342-4` **GEOFON-FDSN-Placeholder** — Draft `state/mycelium/block-drafts/welle2-compiler.φ:1-8`; CDN-Asset 404; `geofon` fehlt in `phi/sources.φ`.
- `xfer:mycelium-342-5` **Fermi-4LAC-Riss** — Block `phi:14553-14558` `format fits` ohne `compiler`/`field`; Compiler lädt Asset `fermi_4lac_dr3.bin`, Block erwartet `table_4LAC.fits`; kein `F4LC`-Arm in `extract.rs`.
- `xfer:mycelium-342-6` **ENTSO-E braucht eine lebende Register-Zeile** — Compiler repariert (`out_Domain`); `terms`/`field` offen.
- `xfer:mycelium-342-7` **JADES DR4 braucht eine Register-Zeile** — Compiler grün (5190 records), Upload verweigert: Tag `jades.herts.ac.uk` ist keine lebende Quelle in `phi/sources.φ`.
- `xfer:mycelium-342-8` **OCNS2019 lokal (Disposition `local-only`)** — Arbeitskopie `data/paperdata.china-vo.org/ocns2019.zip` (sha256 `7b7ad66b…`) für die Neptun-Satelliten-Ephemeriden-Kalibrierung; **kein** `phi`-Eintrag, **kein** CDN-Release (Operator-Wort 2026-10-13).
- `xfer:mycelium-342-9` **CelesTrak-Block abziehen** — die CDN-Release `celestrak.org` (`celestrak_eop.bin`) ist offline genommen (UNCLEAR-Recht); der Register-Block `phi/sources.φ` (Quantities `eop_iers_ut1_utc_s`/`…polar_motion_x/y_arcsec`) verwaist. Die PD-Alternative USNO/MAIA trägt EOP bereits.
- `xfer:mycelium-342-10` **WDC-Kyoto-Feld-Token korrigieren** — `phi/sources.φ` trägt `terms free-open` (`wdc_ae.bin`), gemessen ist `wdc-kyoto-scientific-noncommercial` (wissenschaftlich, nicht-kommerziell, Attribution) — die Host-Regel `state/future/licence-host-rules.tsv:56` ist richtig.
- **Carried from folge341:** `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af` (4 Restcompiler-Formate) · `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.2~98652e66` (Ersatzquellen-Register + Risse) · `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.3~7d22b061` (Welle 3: 2 Schranken) · `xfer:mycelium-341-1` (Risse: CHIME/KASI/IRIS/IERS-EOP/MPC).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
