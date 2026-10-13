<!--
  title: Handover — Mycelium-Folge 342 (2026-10-13)
  class: handover
  date: 2026-10-13
  sha256: 28499d862cdc6dddf644c553638984af9ea540f898b71b38f840216d9d568bdb
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
| „ab jetzt Reports immer in einfacher Sprache; Maschinensprache untereinander ok" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md`; `AGENTS.md` (Language doctrine) |

## Offen — eigen

### ENTSO-E A78 — Compiler-Request repariert; lebende Register-Zeile fehlt
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`
- **Lage:** (gemessen/umgesetzt 2026-10-13/342) der `entsoe_compiler` baute A78 mit `in_Domain` **ohne** `out_Domain` → HTTP 400 `invalidRequest` (Token akzeptiert, sonst 401). **Behoben** (`tools/harvest/src/bin/entsoe_compiler.rs`): `build_url` sendet `in_Domain` **und** `out_Domain`; `--out-domain <EIC>` optional, fehlt es, spiegelt `out_Domain` das jeweilige `in_Domain` (nicht stumm — jede Fetch-Zeile druckt `in -> out`). Belegt über `entsoe-py` (`entsoe.py:1162-1169`: A78 baut in+out_Domain). `cargo build -p omegaflow-harvest --bin entsoe_compiler` grün (0 Warnungen). **Offen:** kein lebender `phi/sources.φ`-Eintrag für ENTSO-E (`sgrep -i entsoe phi/sources.φ` leer) — `url`/`origin`/`compiler` = Mycelium, `terms`/`field` = Mountain.
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

### OCNS2019 (Ersatz für geoazur/APDB) — Lizenz keine offene
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (klären oder lassen)
- **Lage:** (gemessen 2026-10-13/342) nur der Wayback-Weg ist erreichbar: `web.archive.org/web/20220609074246/http://paperdata.china-vo.org/yuanye/ocns2019.zip` (`--verdict` 206; `--sniff` 8 620 636 B, magic zip, sha256 `7b7ad66b2cf1f53b48a8bd51a010437db21983620469231456c46d86366342ef`). Paper-DOI `10.1051/0004-6361/202038776` führt **kein** DataCite-Recht (404/absent) → keine offene Lizenz; der Manifest-Riegel verweigert.
- **Blockade:** keine offene Lizenz.
- **Braucht:** Operator-Entscheidung (China-VO-Klärung oder lassen). Wird in Futures Queue geführt.

### Ersatzquellen-Registerblöcke — 4 Restcompiler (Klassen-Träger)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Commit der Block-Emission + Register-Eintrag
- **Lage:** (gemessen 2026-10-13/341, Taucher) Formate: `heasarc_w3browse` → `format heasarc_w3browse` · `jpl_sbdb` → `format catalog_mpcorb` · `gesdisc_gnssro` → `format cosmic_ro` · `opentopography_raster` → kein `format`. Die 4 Compiler emittieren den Block im Arbeitsbaum (uncommitted). An Mountain: `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af`.
- **Blockade:** Format/Block-Emission = Mountains Register-Recht.
- **Braucht:** Mountain committet die Block-Emission der 4 Compiler + registriert sie.

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
- **Carried from folge341:** `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af` (4 Restcompiler-Formate) · `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.2~98652e66` (Ersatzquellen-Register + Risse) · `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.3~7d22b061` (Welle 3: 2 Schranken) · `xfer:mycelium-341-1` (Risse: CHIME/KASI/IRIS/IERS-EOP/MPC).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
