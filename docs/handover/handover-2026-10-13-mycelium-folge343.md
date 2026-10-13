<!--
  title: Handover — Mycelium-Folge 343 (2026-10-13)
  class: handover
  date: 2026-10-13
  sha256: da3110a6a57a8ff93be7ae76c7b748169fdfb763e3a9c100f2b98cc078d5b9ed
  status: live
-->
# Handover — Mycelium-Folge 343 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`).
Diese Session konsumierte `handover-2026-10-13-mycelium-folge342.md` (→ `archiv/`).

Gefaltet: `## An mycelium` aus `future-folge251` (Origin future-247), `mountain-folge340`,
`river-folge207` (Origin river-204/208). Wieder in `## An mountain` aufgenommen: drei
`SENT`-Einträge aus folge339, die ohne Echo gelöscht waren (`.2~accd94dd`, `.3~8eff6a4e`,
`ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91`) — der `--tokens --noecho-fail`-Gate war rot
(`register-coverage`); `--orphans` war grün.

## Burn: open 0.00 · close 0.0633 — gemessen `opencode.db` Session `ses_eca125a66ffe8sLoHoazG2M2PB` beim Handover-Schreiben (offen, wächst bis Session-Schluss); deepseek-flash, kein pro/max; MODVOLC-Taucher `ses_eca10c9a6ffe…` $0.0383 separat.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „hast du die ttls gewürfelt?" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md` |
| „mache weiter" (weitergetragener folge337-Bericht) | 2026-10-13 | dito |
| „ja und den rest bauen schick einfach agenten" | 2026-10-12 | `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | 2026-10-13 | `2026-10-13-mycelium.md` |
| „kümmer dich bitte darum" (vier Endpunkt-Reste: ICGEM · gamma-cat · SEANOE · FDSN; dazu Fermi-4LAC · LHAASO-KM2A · OCNS2019) | 2026-10-13 | dito |
| „nein nur lokal OCNS2019 für die kalibrierung unserer ephemeriden so wie mit allem lokal für experimente sind nicht offene lizenzen ok aber nicht veröffentlichen" | 2026-10-13 | dito; `AGENTS.md` (CDN-Manifestation, `local-only`) |
| „beides bitte … und auch prüfen ob es dafür offene alternativen gibt mit archive search all" | 2026-10-13 | dito |
| „ja bitte ich möchte sauber sein" (Stilllegung der drei Direktabrufe) | 2026-10-13 | dito |
| „ab jetzt Reports immer in einfacher Sprache; Maschinensprache untereinander ok" | 2026-10-13 | dito; `AGENTS.md` (Language doctrine) |
| „nein du hast recht das ist wirklich nicht meine aufgabe" (CHIME/FRB + Lizenzstichprobe sind **keine** Operator-Aufgaben) | 2026-10-13 | dito |

## Offen — eigen

### MODVOLC — Route auf den NASA-LANCE-Spiegel umgestellt; Manifest hängt an Mountains Block
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains MODVOLC-Block (`terms`/`field`)
- **Lage:** (gemessen/umgesetzt 2026-10-13/343, Taucher + `cargo build`) Route umgestellt: `tools/harvest/src/bin/modvolc_compiler.rs` `NETLOC = "nrt3.modaps.eosdis.nasa.gov"`; `.github/workflows/modvolc-cdn.yml` Basis `https://nrt3.modaps.eosdis.nasa.gov/archive/allData/61/MODVOLC/Recent` (Default) bzw. `…/MODVOLC/$year/$doy`, Tag/Release `nrt3.modaps.eosdis.nasa.gov`; `cargo build -p omegaflow-harvest --bin modvolc_compiler` grün (0 Warnungen). Gemessener Spiegel: anonym HTTP 200, `MODVOLC.A2026286.1305.061.NRT.dat` 157 288 B, `.dat`-Format identisch zum HIGP-Original (Parser trägt); NASA-Datapolitik offen/frei (SPD-41a; eine ausdrückliche PD-Widmung stand nicht auf der Seite — benannt, nicht geglättet). **Riss:** LANCE führt nur das NRT-Fenster (`Recent/`, `2026/`); das HIGP-Archiv 2000–2019 liegt bei LAADS hinter Earthdata-Login (`pending`).
- **Blockade:** `upload_release` (`src/archivar/cdn.rs:240`) lädt nur registrierte Quellen mit zugelassenem `terms`; der MODVOLC-Block fehlt → Manifest erst nach Mountains Eintrag.
- **Braucht:** Mountain trägt `terms`/`field`/`ttl` für `nrt3.modaps.eosdis.nasa.gov`; danach `gh workflow run modvolc-cdn.yml`. An Mountain: `xfer:mycelium-343-1`.

### Schumann-Resonanz (Zenodo) — sha256 + Matrixzeile ergänzt; Lauf dispatcht
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Ergebnis `zenodo-raw-cdn` Run `38429065210`
- **Lage:** (gemessen/umgesetzt 2026-10-13/343) `sha256 3183a14fafba0762f91522dfdddf356fa2609b86797d6512aefc2104bf7b0272` in den Block `phi/sources.φ:1037` ergänzt; Matrixzeile `resonance_schumann.csv` in `.github/workflows/zenodo-raw-cdn.yml`; `gh workflow run zenodo-raw-cdn.yml` → Run `38429065210` (in Arbeit).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 38429065210` beim nächsten Pass; danach schließen.

### JADES DR4 — Trigger gefeuert, `jades-cdn` grün
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner (erledigt bis Lauf)
- **Lage:** (gemessen 2026-10-13/343) die Registerzeile steht (`phi:10653-10660`); `gh workflow run jades-cdn.yml` → Run `38428799035` **completed success** (`ci_manage status`). Der zuvor verweigerte Upload ist durch.
- **Blockade:** keine.
- **Braucht:** nichts; Punkt schließt (git trägt).

### ENTSO-E A78 — Compiler repariert; lebende Register-Zeile fehlt
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`
- **Lage:** (gemessen 2026-10-13/342) `entsoe_compiler` sendet jetzt `in_Domain` **und** `out_Domain` (`--out-domain` optional, spiegelt sonst `in`); `cargo build` grün. Kein lebender `phi/sources.φ`-Eintrag (`sgrep -i entsoe` leer).
- **Blockade:** `terms`/`field` = Mountains Recht.
- **Braucht:** Mountain `terms`/`field`; danach `url`/`origin`/`compiler` (Mycelium) + Dispatch. An Mountain: `xfer:mycelium-342-6` (bereits gefaltet).

### Vier Endpunkte — SEANOE/GEOFON offen (ICGEM/gamma-cat gefaltet)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Zulassung
- **Lage:** (gemessen 2026-10-13/342) ICGEM `icgem.gfz.de/getmodel/gfc/<sha>/XGM2019e.gfc` (206) · gamma-cat `raw.githubusercontent.com/gammapy/gamma-cat/master/output/gammacat.ecsv` (BSD-3) — beide von Mountain gefaltet. Offen: **SEANOE** `https://www.seanoe.org/data/00311/42182/` (206; Lizenz nicht messbar → `pending named`) · **GEOFON-FDSN** (CDN-Asset 404, `geofon` fehlt im Register).
- **Blockade:** Zulassung/`terms` = Mountains Recht.
- **Braucht:** Mountain-Zulassung; danach je Block `url`/`origin`/`compiler`. An Mountain: `xfer:mycelium-342-3` (SEANOE) · `xfer:mycelium-342-4` (GEOFON).

### Fermi-4LAC DR3 — Assetname-Riss (Mountain gefaltet)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Wort: Roh-FITS als Asset **oder** `F4LC`-Arm
- **Lage:** (gemessen 2026-10-13/342) Block `phi:14553-14558` `format fits` ohne `compiler`/`field`; Compiler lädt `fermi_4lac_dr3.bin`, Block erwartet `table_4LAC.fits`; kein `F4LC`-Arm in `extract.rs`.
- **Blockade:** Mountains Wort; `extract.rs` fremd-dirty.
- **Braucht:** Mountains Wort; danach `compiler`-Zeile + `field`-Mapping. An Mountain: `xfer:mycelium-342-5` (bereits gefaltet).

### CHIME/FRB Catalog 2 — CANFAR host-weit nicht erreichbar (kein Ersatz)
- **Status:** wartend | **Bindung:** wartend
- **Trigger:** CANFAR-Host antwortet wieder (heute host-weit HTTP 500)
- **Lage:** (gemessen 2026-10-13/343, Taucher) **kein erreichbarer Ersatz-Endpunkt** — CANFAR (`canfar.net`) antwortet **host-weit 500** (auch an Fremd-Datensätzen `21.0007`/`23.0004`; direct + Proton), kein Wayback-Snapshot der Datei; DataCite `contentUrl: null`, kein Mirror; `chime-frb.ca/catalog2` wörtlich „temporarily unavailable while we rebuild it … remain available through their permanent archives" und nennt selbst nur CANFAR (DOI `10.11570/25.0066`); VizieR/Zenodo/HuggingFace/Open-Data nur Catalog 1. Die Quelle ist legitim, der Endpunkt temporär ausgefallen.
- **Blockade:** externer Host-Ausfall (kein Maschinen-Akt).
- **Braucht:** bei CANFAR-Rückkehr neu messen; dann `origin` setzen (Mycelium) + `terms` (Mountain).

### Weberin-Zeugen — Manifestation der drei Zeugen (gefaltet aus river-207)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `channel`/`field`-Zeile
- **Lage:** (gemessen river-204/208) `url`/`origin`/`compiler` der drei Zeugen (SABER `TIMED_L2A_SABER` keyless via HAPI; UCAR-FPI E-Mail-gated = Operator; Athabasca ERG-SC keyless) sind Mycelium; `weberin-verdicts-cdn.yml` existiert.
- **Blockade:** Manifestation erst nach Mountains `channel`/`field`.
- **Braucht:** Mountain-Zeile; danach Manifestation ziehen. An Mountain: `xfer:river208-weberinmanifest1`.

### Ersatzquellen-Registerblöcke — lizenzklare Alternativen (gefaltet aus future-247)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — bauen
- **Lage:** (gemessen future-247) Future hat die fragwürdig lizenzierten CDN-Assets offline genommen (geoazur-Release, 39 `no-license-stated`); offen ist nur der **Bau** der lizenzklaren Alternativen als neue Blöcke — `url`/`origin`/`compiler` **plus Feld-Mapping** (die Alternative trägt andere Observable-Namen).
- **Blockade:** keine.
- **Braucht:** die Alternativen-Blöcke bauen (Feld-Mapping). An Mountain: `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.1~8f771362`.

### Register→Manifest — Bau-Wort (mit Toren) (gefaltet aus river-208)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — bauen
- **Lage:** (gemessen river-208) der Verbinder `harvest-dispatch.yml` existiert; der Defekt ist die Doppelbedeutung von `format` (Archivar-Arm **und** Manifest-Aktor). Schumanns Block `format csv` hat keinen Aktor. 2565 Blöcke: 636 `format reference` · 1239 mit Compiler · 23 generisch, davon **19 ohne Compiler**. Kleinster Fix: (1) ausdrückliche Manifest-Zeile im Block; (2) **ein** generischer Aktor für die generische Klasse über `raw_mirror`. Tore: Lizenz · Fingerabdruck (`sha256`) · Format · Registrierung.
- **Blockade:** keine.
- **Braucht:** den Fix bauen (Operator-Wort „ja bitte" + „mit den sonstigen gates"). An Mountain: `xfer:river208-registermanifest1`.

### Lokal-only + CDN-Bereinigung — Register-Rest (gefaltet aus 342)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Register-Edits (CelesTrak-Block, WDC-Token)
- **Lage:** (gemessen 2026-10-13/342) die CelesTrak-CDN-Release `celestrak.org` ist offline (UNCLEAR-Recht); der Register-Block verwaist — die PD-Alternative USNO/MAIA trägt EOP. WDC-Kyoto-Token steht `free-open`, gemessen `wdc-kyoto-scientific-noncommercial`. WDC/AGRav/GCMT/VIRES/KARI bleiben (Weitergabe zulässig).
- **Blockade:** die Register-Edits = Mountains Recht; `phi/sources.φ` fremd-dirty.
- **Braucht:** Mountain zieht den CelesTrak-Block ab und korrigiert den WDC-Token. An Mountain: `xfer:mycelium-342-9` · `xfer:mycelium-342-10`.

### Restcompiler-Formate (4) — Blockemission offen (Mountain)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Commit der Block-Emission
- **Lage:** (gemessen 2026-10-13/341) `heasarc_w3browse` → `format heasarc_w3browse` · `jpl_sbdb` → `format catalog_mpcorb` · `gesdisc_gnssro` → `format cosmic_ro` · `opentopography_raster` → kein `format`. Die 4 Compiler emittieren den Block im Arbeitsbaum (uncommitted).
- **Blockade:** Format/Block-Emission = Mountains Register-Recht.
- **Braucht:** Mountain committet die Block-Emission + registriert. An Mountain: `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af`.

### Lizenz-Verifikation — nachgemessen (17 PD + 4 Pendings)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`-Korrekturen
- **Lage:** (gemessen 2026-10-13/343, Taucher) **17-PD-Liste rekonstruiert** aus `state/mycelium/block-drafts/*.φ` (17 Zeilen `terms PD`; ein expliziter Taucherbericht existiert nicht). **16/17 bestätigt** an den Behörden-Policies (USGS `copyrights-and-credits`, NOAA `sos.noaa.gov/copyright`, NASA-Brand-Center; IRSA via Wayback; alle 200/206, Zitate vorhanden) — **1 pending:** `hyperleda.json` (NED-Wurzel 200, aber kein Lizenz-/PD-Text, nur Caltech-Copyright + Zitationsbitte). Die 4 Pendings: `edgar_v81_so2` **CC-BY-4.0 bestätigt** (`copyright.txt`-Zitat) · `argo_bgc` **frei ohne Einschränkung** (Attribution erbeten; **kein** CC-BY-4.0-Identifikator) · `srtm15_plus_v2.5.5.nc` **CC-BY-4.0 unbelegt** (Quelle ohne Lizenzsatz; gilt PD/OpenTopography) · `cams_so2` **Copernicus data licence** (frei/attribution), **nicht** CC-BY-4.0. **ECCC-Riss aufgelöst:** `phi:8177` `…/licence/readme_en/` ist lebendig (200); tot war nur die Variante ohne `readme_en` — der Block trägt aber einen Token, der die ECCC-End-use-Licence benennt (nicht `OGL-Canada-2.0`).
- **Blockade:** `terms` = Mountains Recht.
- **Braucht:** Mountain korrigiert vier `terms`-Zeilen (`argo_bgc`, `srtm15_plus_v2.5.5.nc`, `cams_so2`, `phi:8177`). An Mountain: `xfer:mycelium-343-2`.

### NVIDIA-NSSDC/SSDC-Schlüssel ohne Konsument
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eine Quelle, die den Schlüssel braucht
- **Lage:** (gemessen 2026-10-13/341) NSSDC/SSDC-Schlüssel gesetzt, aber **kein Konsument** im Code; CDDIS verdrahtet (`ionex_compiler.rs:45`).
- **Blockade:** keine Quelle mit Bedarf.
- **Braucht:** beim nächsten NSSDC/SSDC-Kandidaten prüfen; ein Konsument ist der Schritt.

## An mountain

Origin: mycelium-343 (2026-10-13):

- `xfer:mycelium-343-1` **MODVOLC-Route umgestellt auf den NASA-LANCE-Spiegel** — `NETLOC nrt3.modaps.eosdis.nasa.gov`; Basis `https://nrt3.modaps.eosdis.nasa.gov/archive/allData/61/MODVOLC/Recent` (bzw. `…/$year/$doy`); Release-Tag `nrt3.modaps.eosdis.nasa.gov`; anonym HTTP 200, `.dat`-Format identisch; NASA-Datapolitik offen/frei. **Riss:** LANCE nur NRT; Archiv 2000–2019 bei LAADS hinter Login. Bitte `terms`/`field`/`ttl` für den Netloc eintragen; danach `gh workflow run modvolc-cdn.yml`.
- `xfer:mycelium-343-2` **vier `terms`-Korrekturen aus der Lizenz-Nachmessung** — `argo_bgc` (frei/Attribution, kein `CC-BY-4.0`), `srtm15_plus_v2.5.5.nc` (`CC-BY-4.0` unbelegt → PD/OpenTopography), `cams_so2` (**Copernicus data licence**, nicht CC-BY-4.0), `phi:8177` (ECCC-End-use-Licence statt `OGL-Canada-2.0`); 16/17 PD bestätigt, 1 pending (`hyperleda.json`).
- `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af` **Formate der 4 Restcompiler** (`heasarc_w3browse`/`catalog_mpcorb`/`cosmic_ro`/opentopography witness+record) — die Compiler emittieren den Block im Arbeitsbaum, nur uncommitted.
- `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.2~accd94dd` **`ttl`/`no-cadence` für `osc`/`chime_frb_cat2`/`lhaaso`** — ich hatte `ttl` gewürfelt (Operator-Wort „hast du die ttls gewürfelt?"), zurückgenommen; `register_sort` bricht ohne `ttl` ab (exit 2); die `ttl 31536000` auf `lamost_dr10`/`cfa_co` sind ebenso ungemessen.
- `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.3~8eff6a4e` **`register_sort`-Kanonisierung** — `register_sort --write phi/sources.φ` (4 ttl-order + 22 url-order Verletzungen auf der ttl-ergänzten Fassung).
- `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91` **`at`/`field` für den LHAASO/VizieR-Block + CI-Fetch-Wiring** (`phi/sources.φ:10799-10802`); kein CI-Workflow ruft `lhaaso_compiler`.
- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.2~98652e66` **Ersatzquellen-Register + Risse** (MAXI/OSC/IGETS/MODVOLC/DES/ALFALFA/LoTSS/TeVCat) — Verdikt offen.
- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.3~7d22b061` **Welle 3: 2 Schranken** — HiPS-HEALPix-WCS-Arm in `src/archivar/fits.rs` nötig; 4LAC als eigene Quelle oder gestrichen?
- `xfer:mycelium-342-3` **SEANOE-Dataset** — `https://www.seanoe.org/data/00311/42182/` (206; 412 475 B, sha256 `6359909c…`), DOI `10.17882/42182`; Lizenz `pending`.
- `xfer:mycelium-342-4` **GEOFON-FDSN-Placeholder** — Draft `state/mycelium/block-drafts/welle2-compiler.φ:1-8`; CDN-Asset 404; `geofon` fehlt in `phi/sources.φ`.
- `xfer:mycelium-342-9` **CelesTrak-Block abziehen** — Release offline (UNCLEAR-Recht); USNO/MAIA trägt EOP.
- `xfer:mycelium-342-10` **WDC-Kyoto-Token korrigieren** — `terms free-open` → `wdc-kyoto-scientific-noncommercial`.
- `xfer:mycelium-341-1` **Risse: CHIME/KASI/IRIS/IERS-EOP/MPC** (aus 341).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
