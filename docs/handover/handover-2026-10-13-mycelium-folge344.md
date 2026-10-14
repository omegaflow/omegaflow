<!--
  title: Handover — Mycelium-Folge 344 (2026-10-13)
  class: handover
  date: 2026-10-13
  sha256: c815bcb16b316b9f8d804959213b65a89d4d81bdcc8aaa6ebe9e1902697d1757
  status: live
-->
# Handover — Mycelium-Folge 344 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`).
Diese Session konsumierte `handover-2026-10-13-mycelium-folge343.md` (→ `archiv/`).

Gefaltet: `## An mycelium` aus `future-folge252` (Origin future-247) — die lizenzfragwürdigen
CDN-Assets hat **Future** offline genommen (Operator-Wort „nimm die dateien runter"), offen ist
nur der Bau der lizenzklaren Ersatz-Blöcke; neu darin: **OCNS2019** als APDB-Ersatz. Die Blöcke
aus `mountain-folge340` und `river-folge207` sind bereits in folge343 gefaltet.

## Burn: open 0.00 · close 0.0321 — gemessen `opencode.db` Session „Mycelium-Linie: Pass schließen und Runde starten" beim Handover-Schreiben (offen, wächst bis Session-Schluss); deepseek-flash, kein pro/max.

## CI-Stand am HEAD `fd1d693e` (gemessen `ci_manage status`, 2026-10-13T20:3xZ)

- `cdaweb-tec-cdn` `38462006436` **failure** — Grund `unread` (Job-Log per `ci_manage log` nur der Cleanup-Tail; kein Step-Fehler abrufbar). Träger mountain/river.
- `register-coverage` `38462010026` **success** (der frühere XFER-NOECHO river→mountain ist bereinigt).
- In flight: `ci-gate` `38462425790`, `ci-check` `38462010120`, `register-dropped` `38462009965`, `m10-mdis-raw-cdn` `38462006450` — Ergebnis aus Watchdog, nicht pollend.

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
- **Braucht:** Mountain trägt `terms`/`field`/`ttl` für `nrt3.modaps.eosdis.nasa.gov`; danach `gh workflow run modvolc-cdn.yml` (Vorlauf `38429838801` rot, erwartet). An Mountain: `xfer:mycelium-343-1`.

### ENTSO-E A78 — Compiler repariert; lebende Register-Zeile fehlt
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`
- **Lage:** (gemessen 2026-10-13/342) `entsoe_compiler` sendet jetzt `in_Domain` **und** `out_Domain` (`--out-domain` optional, spiegelt sonst `in`); `cargo build` grün. Kein lebender Eintrag in `phi/sources.φ` (`sgrep -i entsoe` leer).
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

### Ersatzquellen-Registerblöcke — lizenzklare Alternativen (future-folge252 gefaltet)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field` für die Alternativen
- **Lage:** (gemessen future-247/252) Future hat die fragwürdig lizenzierten CDN-Assets offline genommen (geozur-Release, 39 `no-license-stated`); offen ist nur der **Bau** der lizenzklaren Alternativen als neue Blöcke — `url`/`origin`/`compiler` **plus Feld-Mapping** (die Alternative trägt andere Observable-Namen). Neu aus future-252: **OCNS2019** (Yuan et al. 2021, China-VO PaperData, Wayback-Snapshot) als APDB-Ersatz — **Operator-Wort: `local-only`** (nicht offene Lizenz nur lokal für Experimente, nie manifestiert) → Kalibrier-Arbeitskopie unter `data/`, keine `phi`-Zeile.
- **Blockade:** Feld-Mapping/`field` = Mountains Recht; OCNS2019 bleibt lokal.
- **Braucht:** die Alternativen-Blöcke bauen (Feld-Mapping) nach Mountains `terms`/`field`. An Mountain: `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.1~8f771362` (gefaltet) · `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.10~7d5c9163` (OCNS2019 — `local-only`, kein CDN-Akt).

### Register→Manifest — CI-Trigger gebaut; Block-Direktive an Mountain
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `harvest`-Direktive im Block
- **Lage:** (gemessen/umgesetzt 2026-10-13/344) `.github/workflows/harvest-dispatch.yml` reagiert jetzt zusätzlich auf eine ausdrückliche Manifest-Zeile `harvest <actor>` in `phi/sources.φ` (added `+harvest <actor>` → als Format-Token gelesen → `header_of` löst den Workflow mit `# harvest: format=<actor>`). Die Doppelbedeutung bleibt aufgelöst: ein nacktes `format <name>` ist wieder die Parser-Arm-Zeile allein, die Manifestation nennt ihren Aktor ausdrücklich. Additiv (kein Bestandsblock ändert sein Verhalten). Schumanns Block trägt `format csv`; mit `harvest raw_zenodo` feuert der bestehende `zenodo-raw-cdn.yml`. Der generische Aktor für die compiler-lose Klasse (`# harvest: format=csv arm=raw_mirror`) ist noch nicht gebaut — er braucht Mountains Direktive.
- **Blockade:** `harvest` als neue Register-Direktive = Mountains Register-Recht (+ Parser-Anerkennung).
- **Braucht:** Mountain trägt `harvest raw_zenodo` in den Schumann-Block (`phi:1034-1043`) und erkennt die Direktive im Parser an; danach den generischen Aktor bauen. An Mountain: `xfer:mycelium-344-1`.

### Lokal-only + CDN-Bereinigung — Register-Rest
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Register-Edits (CelesTrak-Block, WDC-Token)
- **Lage:** (gemessen 2026-10-13/342) die CelesTrak-CDN-Release `celestrak.org` ist offline (UNCLEAR-Recht); der Register-Block verwaist — die PD-Alternative USNO/MAIA trägt EOP. WDC-Kyoto-Token steht `free-open`, gemessen `wdc-kyoto-scientific-noncommercial`. WDC/AGRav/GCMT/VIRES/KARI bleiben (Weitergabe zulässig).
- **Blockade:** die Register-Edits = Mountains Recht; `phi/sources.φ` fremd-dirty.
- **Braucht:** ~~CelesTrak-Block~~ **erledigt 2026-10-14/344** (Release `celestrak.org` gelöscht + Block `phi:10148-10157` abgezogen, nach Mountain's EOP-Verdikt); offen nur noch der WDC-Token. An Mountain: `xfer:mycelium-342-10`.

### Restcompiler-Formate (4) — Blockemission offen (Mountain)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Commit der Block-Emission
- **Lage:** (gemessen 2026-10-13/341) `heasarc_w3browse` → `format heasarc_w3browse` · `jpl_sbdb` → `format catalog_mpcorb` · `gesdisc_gnssro` → `format cosmic_ro` · `opentopography_raster` → kein `format`. Die 4 Compiler emittieren den Block im Arbeitsbaum (uncommitted).
- **Blockade:** Format/Block-Emission = Mountains Register-Recht.
- **Braucht:** Mountain committet die Block-Emission + registriert. An Mountain: `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af`.

### Lizenz-Verifikation — nachgemessen (17 PD + 4 Pendings)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`-Korrekturen
- **Lage:** (gemessen 2026-10-13/343, Taucher) **17-PD-Liste rekonstruiert** aus `state/mycelium/block-drafts/*.φ` (17 Zeilen `terms PD`; ein expliziter Taucherbericht existiert nicht). **16/17 bestätigt** an den Behörden-Policies (USGS `copyrights-and-credits`, NOAA `sos.noaa.gov/copyright`, NASA-Brand-Center; IRSA via Wayback; alle 200/206, Zitate vorhanden) — **1 pending:** `hyperleda.json` — die Quelle ist NED (`origin …/ned.ipac.caltech.edu/tap/sync…NEDTAP.objdir`); **browser-verifiziert 343** (Startseite · About NED · Acknowledging NED · „Guidelines for Automated Queries"): keine Lizenz-/Terms-Seite, nur Danksagung + `Copyright © 2026, California Institute of Technology` + Nutzungsrichtlinien für automatische Abfragen; re3data führt als Terms nur IPACs `image-use-policy` (`other`). Der Token `PD` ist an NED **nicht belegt**. Die 4 Pendings: `edgar_v81_so2` **CC-BY-4.0 bestätigt** (`copyright.txt`-Zitat) · `argo_bgc` **frei ohne Einschränkung** (Attribution erbeten; **kein** CC-BY-4.0-Identifikator) · `srtm15_plus_v2.5.5.nc` **CC-BY-4.0 unbelegt** (Quelle ohne Lizenzsatz; gilt PD/OpenTopography) · `cams_so2` **Copernicus data licence** (frei/attribution), **nicht** CC-BY-4.0. **ECCC-Riss aufgelöst:** `phi:8177` `…/licence/readme_en/` ist lebendig (200); tot war nur die Variante ohne `readme_en` — der Block trägt aber einen Token, der die ECCC-End-use-Licence benennt (nicht `OGL-Canada-2.0`).
- **Blockade:** `terms` = Mountains Recht.
- **Braucht:** Mountain korrigiert vier `terms`-Zeilen (`argo_bgc`, `srtm15_plus_v2.5.5.nc`, `cams_so2`, `phi:8177`). An Mountain: `xfer:mycelium-343-2`.

### BASE — erster Datensatz-Arm; zwei neue Kandidaten
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Zulassung der zwei Kandidaten
- **Lage:** (gemessen 2026-10-13/343) BASE als **erster Arm für Datensatz-/Repositoriums-Discovery** in die Kaskade (`AGENTS.md`) und die Werkzeugkarte (`tools-map.md`) aufgenommen (findet DataCite-/Repositoriums-DOIs, die arXiv/Crossref/OpenAlex übersehen). Lücken-Abgleich: zwei neue Kandidaten (**Venus-Radar-PDS**, **ALFALFA**), ein IGETS-`origin`-Riss; Entwurf `state/mycelium/block-drafts/2026-10-13-base-funde.φ`. IGETS selbst ist bereits registriert (`phi:10303-10309`).
- **Blockade:** `terms`/`field` = Mountains Recht.
- **Braucht:** Mountain-Zulassung + `terms`/`field`; danach `url`/`origin`/`compiler` (Mycelium) + Manifestation. An Mountain: `xfer:mycelium-343-3`.

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass oder eine `archive_search`-Arm-Änderung
- **Lage:** (gemessen 2026-10-13/343) auf den `archive_search --help`-Stand; der `--base`-Eintrag (`tools-map.md:328`) von „keyless" auf **`BASE_API_KEY`** korrigiert (net.rs:4181; Teil von `--all`). `--base` angefragt und laufend: „fast radio bursts" → 16 Treffer (Lyutikov/Popov 2020, drei FRB-Community-Newsletter 2026, SciDB-Datensätze).
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben.

### NVIDIA-NSSDC/SSDC-Schlüssel ohne Konsument
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eine Quelle, die den Schlüssel braucht
- **Lage:** (gemessen 2026-10-13/341) NSSDC/SSDC-Schlüssel gesetzt, aber **kein Konsument** im Code; CDDIS verdrahtet (`ionex_compiler.rs:45`).
- **Blockade:** keine Quelle mit Bedarf.
- **Braucht:** beim nächsten NSSDC/SSDC-Kandidaten prüfen; ein Konsument ist der Schritt.

### archive_search — Arm-Ausbeute gemessen + Eskalations-Leiter gebaut
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — gebaut (Commit-Wort + CI-Test)
- **Lage:** (gemessen 2026-10-13/344) über `state/research/spills/` (2 514 Dateien, 715 MB): 3 438 Arm-Marker; Ausbeute = `[arm] url`-Zeilen je Lauf. **Tier A (≥10/Lauf):** zenodo 169,3 · openalex 64,4 · linkup 44,1 · exa 36,1 · psychporta 23,8 · semanticscholar 20,5 · crossref 17,7 · firecrawl 13,1 · core 12,9. **Tier B (3–10):** reactome 8,5 · serper 8,1 · datacite 7,6 · re3data 7,5 · go 7,5 · arxiv 7,2 · pangaea 6,8 · chembl 6,7 · dataeuropa 6,6 · dataverse 5,9 · seanoe 5,3 · ads 4,9. **Blockiert (nur `pending`):** base (IP-Access-denied) · tavily (HTTP 432) · serpapi/searchapi (HTTP 429) · tap · sparql · shodan · rss-bridge · opencellid · oapen · oai-pmh · googlecse · gfw · gemini · consensus · cc. **0 Treffer im Korpus:** ~47 Arme (themenabhängig „absent", nicht tot).
- **Gebaut (2026-10-13/344):** (a) **Query-Log** `state/research/queries.φ` (`ts | session | mode | lines | hits | query`) schreibt jeder Netz-Lauf (`net.rs log_query`); (b) **`archive_search --stats`** aggregiert per Arm; (c) **Eskalations-Leiter** in `--all`: Arme nach gemessener Ausbeute sortiert (Log-yield, sonst Built-in-Tier), Abbruch bei ≥ `OMEGAFLOW_ALL_STOP` Trefferzeilen (Default 20) nach ≥6 Armen. `rustfmt --check` grün; `cargo build -p omegaflow-utils --bin archive_search` grün (0 Warnungen). Probe `--all "omegaflow kybernetik"` → 6 Arme, dann `=== stop: 595 result line(s)`.
- **Blockade:** keine.
- **Braucht:** `/commit` (Wort) + CI (`cargo test`); danach den Log durch echte Läufe füllen, `--stats` gegen die gewachsene Nutzung prüfen und die Built-in-Tier bei Bedarf erneuern.

### Fäden schließen — alle Quellen gemessen (Operator-Wort: „erst alle Fäden")
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field` für die neuen Quellen
- **Lage:** (gemessen 2026-10-13/344, fünf Taucher) die **fünf fehlenden Fäden** haben eine gemessene Quelle (Draft `state/mycelium/block-drafts/2026-10-13-faden-schliessung.φ`): (1) **Sonnen-Magnetfeld** — SDO/HMI Synoptic FITS (`jsoc.stanford.edu`, 17,6 MB, HTTP 200) + HMI Polar-Field + **SILSO SSN** (`terms CC-BY-NC-4.0` — das Haus ist selbst NC, registerbar; Präzedenz `phi:29157`); (2) **Stern-Aktivität** — VizieR `J/ApJ/835/61` `logR'HK` + LISIRD LaTiS EUV; (3) **21 cm** — EBHIS/WALLABY/CGPS (CDS) + EDGES Zenodo `16881868` **CC-BY-4.0**; (4) **Wolken** — ISCCP HGM (NOAA CDR) + PATMOS-x (S3 offen); (5) **TGF** — Fermi-GBM-TGF-Catalog + HEASARC-WWLLN-Sferics + Zenodo `6782049`. **Echte Gates (Zugang, nicht Lizenz):** WSO ohne Lizenz (`pending`) · ASIM login-gated · CERES/MODIS account-gated · GALAH-Aktivität nicht auf VizieR. **LOD gebaut (Code, 2026-10-13/344):** `src/archivar/celestrak_eop.rs` trägt jetzt `COMP_LOD=4` (`COMP_MAX=4`), `src/archivar/extract.rs` mappt `eop_iers_lod_s`, der Compiler liest CSV-Spalte 5 (LOD); `rustfmt --check` + `cargo build -p omegaflow-harvest --bin celestrak_eop_compiler` grün (0 Warnungen) + Rauchtest (8 Records = 2 Zeilen × 4 Komponenten, roundtrip parst). Register-Zeile `quantity eop_iers_lod_s` = Mountains Recht (`xfer:mycelium-344-3`).
- **Blockade:** `terms`/`field`/`ttl` = Mountain; Manifestation = Mycelium.
- **Braucht:** Mountain setzt `terms`/`field`/`ttl` für die gemessenen Quellen; danach `url`/`origin`/`compiler` (Mycelium) + CDN-Manifestation. An Mountain: `xfer:mycelium-344-2`.

### DE-2-Manifestation — gebaut (gefaltet aus river, P7.1)
- **Status:** eigen (Rest: erster CDN-Pull via CI) | **Bindung:** eigen
- **Trigger:** `harvest-dispatch` auf dem Push auf `phi/sources.φ` (bzw. `gh workflow run de2-cdn.yml`)
- **Lage:** (gemessen 2026-10-13/344 via Taucher + `commit_check`; umgesetzt) Der 1:1-Mirror (`cdn_mirror_compiler`) **scheiterte im CI** (`curl: (52) Empty reply` beim 1,5-Jahres-HAPI-Einzelfetch) → ersetzt durch den **Monatsfenster-HAPI-Compiler** `tools/harvest/src/bin/de2_compiler.rs` (holt `format=csv` Monat für Monat, hängt zu **einem CSV** zusammen, lädt hoch — `hapi_csv_body` liest CSV + `hapi`-Extrakt). Block `phi:586` trägt jetzt `format csv` + `origin https://cdaweb.gsfc.nasa.gov/hapi` + `url …/de2_ua16s_all.csv` + `compiler …` + `harvest de2`; Workflow `.github/workflows/de2-cdn.yml` (`# harvest: format=de2 asset=fehlt arm=de2_compiler`) baut und startet ihn. `rustfmt --check` + `cargo build -p omegaflow-harvest --bin de2_compiler` grün (0 Warnungen); Einheitentest für die CSV-Konkatenation grün. Erster echter Nutzer der `harvest <actor>`-Direktive.
- **Riss (`asset=`):** River meldet „kein Code liest `asset=`" — **widerlegt**: `harvest-dispatch.yml:106` liest es (`fehlt`→immer dispatch, `present`→nur bei args-Änderung). Da das Asset noch fehlt, bleibt `asset=fehlt` (sonst würde die Provisionierung abgeschaltet und ein Zustand behauptet, den es nicht gibt) → nach dem ersten Pull `present`. An River: `## An river` / `xfer:mycelium-344-4`.
- **Blockade:** CDAWeb-HAPI ist vom Sandkasten **nicht** erreichbar (`curl 52`); ob der CI-Runner die Monatsfenster bekommt, entscheidet der nächste Lauf (`/pub/data` ist von CI erreichbar, der HAPI-`data`-Pfad war es im 1:1-Versuch nicht).
- **Braucht:** `/commit` + Push; `harvest-dispatch` startet `de2-cdn.yml` → CI-Pull → Asset → dann `asset=present` + K messbar. River-Token gefaltet: `xfer:river.ses_ec901ad7dffehLKKusa1DO806h.1~aac84d7f`.

## An mountain

Origin: mycelium-344 (2026-10-13):

- `xfer:mycelium-344-1` **Register→Manifest — CI-Trigger gebaut, Block-Direktive offen.** `.github/workflows/harvest-dispatch.yml` reagiert jetzt auf eine ausdrückliche `harvest <actor>`-Zeile im Block (added `+harvest <actor>` → Format-Token → `header_of` löst den Workflow mit `# harvest: format=<actor>`). Bitte (a) die Direktive `harvest <actor>` im Parser anerkennen (kein Archivar-Arm, reine Manifest-Nennung) und (b) `harvest raw_zenodo` in den Schumann-Block (`phi:1034-1043`) setzen — dann feuert `zenodo-raw-cdn.yml` bei der nächsten `phi`-Änderung. Danach baue ich den generischen Aktor `# harvest: format=csv arm=raw_mirror`.
- `xfer:mycelium-344-2` **Fünf fehlende Fäden — gemessene Quellen** (Draft `state/mycelium/block-drafts/2026-10-13-faden-schliessung.φ`): (1) **Sonnen-Magnetfeld** `http://jsoc.stanford.edu/data/hmi/synoptic/hmi.Synoptic_Mr.<CR>.fits` (FITS, HTTP 200, 17 645 550 B) + HMI Polar-Field `hmi.meanpf_720s` + **SILSO SSN** `https://www.sidc.be/SILSO/DATA/SN_d_tot_V2.0.csv` (**CC-BY-NC-4.0** — das Haus ist selbst NC, registerbar); (2) **Stern-Aktivität** `https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync table=J/ApJ/835/61/table6` (logR'HK) + LISIRD LaTiS EUV; (3) **21 cm** EBHIS/WALLABY/CDS + EDGES Zenodo `16881868` (CC-BY-4.0); (4) **Wolken** ISCCP HGM (NOAA CDR) + PATMOS-x; (5) **TGF** Fermi-GBM-TGF `https://fermi.gsfc.nasa.gov/ssc/data/access/gbm/tgf/` + HEASARC-WWLLN-Sferics. Bitte `terms`/`field`/`ttl` je Block; danach registriere ich `url`/`origin`/`compiler` + manifestiere.
- `xfer:mycelium-344-3` **LOD — Auftrag umgeleitet (CelesTrak abgezogen).** Das LOD-Feld ist gebaut (`celestrak_eop` `COMP_LOD=4`, `extract.rs` mappt `eop_iers_lod_s`, Compiler liest CSV-Spalte 5) — aber Mountain's EOP-Verdikt (2026-10-14, konvergent, sieben Seats) streicht CelesTrak: **Release `celestrak.org` gelöscht, Block `phi:10148-10157` abgezogen** (ausgeführt 2026-10-14/344). Damit wandert die EOP auf **USNO/MAIA `finals2000A.all`** (Block `phi:8121`, Format `finals`, Felder `ut1_utc`/`pmx`/`pmy`). **Neuer Bedarf:** das LOD-Feld im **`finals`-Arm** (bzw. eine `field`/`quantity`-Zeile dort) ergänzen — sonst ist LOD erneut absent. Der `celestrak_eop`-Code/das Workflow bleiben als manueller Failover stehen (Z.ai: „höchstens als manueller Failover hinter USNO"). An Mountain: `xfer:mycelium-344-5`.
- `xfer:mycelium-344-5` **LOD in den `finals`-Arm (nach CelesTrak-Abzug).** Der EOP-Block `phi:8121` (USNO/MAIA `finals2000A.all`, `format finals`, Felder `ut1_utc`/`pmx`/`pmy`) trägt **kein LOD**; nach dem CelesTrak-Abzug ist die LOD-Lücke (Geodynamo-Faden) sonst wieder offen. Bitte die LOD-Extraktion im `finals`-Arm (bzw. eine `quantity`-Zeile dort) ergänzen — die CelesTrak-Mechanik (`COMP_LOD`, Compiler) steht als manueller Failover.
- `xfer:mycelium-343-1` **MODVOLC-Route umgestellt auf den NASA-LANCE-Spiegel** — `NETLOC nrt3.modaps.eosdis.nasa.gov`; Basis `https://nrt3.modaps.eosdis.nasa.gov/archive/allData/61/MODVOLC/Recent` (bzw. `…/$year/$doy`); Release-Tag `nrt3.modaps.eosdis.nasa.gov`; anonym HTTP 200, `.dat`-Format identisch; NASA-Datapolitik offen/frei. **Riss:** LANCE nur NRT; Archiv 2000–2019 bei LAADS hinter Login. Bitte `terms`/`field`/`ttl` für den Netloc eintragen; danach `gh workflow run modvolc-cdn.yml`.
- `xfer:mycelium-343-2` **vier `terms`-Korrekturen aus der Lizenz-Nachmessung** — `argo_bgc` (frei/Attribution, kein `CC-BY-4.0`), `srtm15_plus_v2.5.5.nc` (`CC-BY-4.0` unbelegt → PD/OpenTopography), `cams_so2` (**Copernicus data licence**, nicht CC-BY-4.0), `phi:8177` (ECCC-End-use-Licence statt `OGL-Canada-2.0`); 16/17 PD bestätigt, 1 pending (`hyperleda.json` — Token `PD` an NED **nicht belegt**, browser-verifiziert).
- `xfer:mycelium-343-3` **BASE-Datensatz-Arm — zwei neue Kandidaten + zwei Risse** (gemessen 2026-10-13, `archive_search --base` gegen die offenen Lücken; Entwurf `state/mycelium/block-drafts/2026-10-13-base-funde.φ`): (a) **Venus Radar Multi-Look Map Products** (PDS Geosciences, Campbell; DOI `10.17189/1522443`) — **neu** (`venus_radar_Processed.zip` ist nur ein Zenodo-Derivat), PD-USGov, 15×960 MB = ~13,4 GiB → über dem 2-GiB-Release-Limit (`cdn.rs:230`), Split/R2 klären; Endpunkt `…/pds-geosciences.wustl.edu/venus/urn-nasa-pds-venus_radar_level2/data/` (200). (b) **ALFALFA Ultra-compact HVCs** (VizieR `J/ApJ/768/77`) — **neu**, asu-tsv 200 (16 104 B); **Riss:** AAS-CC-BY-NC-Vorbehalt auf der VizieR-Lizenzseite. (c) **IGETS-Riss:** der registrierte `origin` (`phi:10305`) ist der alte Portal-URL; der Download ist sftp + Registrierung (`blocked account`).
- `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af` **Formate der 4 Restcompiler** (`heasarc_w3browse`/`catalog_mpcorb`/`cosmic_ro`/opentopography witness+record) — die Compiler emittieren den Block im Arbeitsbaum, nur uncommitted.
- `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.2~accd94dd` **`ttl`/`no-cadence` für `osc`/`chime_frb_cat2`/`lhaaso`** — ich hatte `ttl` gewürfelt (Operator-Wort „hast du die ttls gewürfelt?"), zurückgenommen; `register_sort` bricht ohne `ttl` ab (exit 2); die `ttl 31536000` auf `lamost_dr10`/`cfa_co` sind ebenso ungemessen.
- `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.3~8eff6a4e` **`register_sort`-Kanonisierung** — `register_sort --write phi/sources.φ` (4 ttl-order + 22 url-order Verletzungen auf der ttl-ergänzten Fassung).
- `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91` **`at`/`field` für den LHAASO/VizieR-Block + CI-Fetch-Wiring** (`phi/sources.φ:10799-10802`); kein CI-Workflow ruft `lhaaso_compiler`.
- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.2~98652e66` **Ersatzquellen-Register + Risse** (MAXI/OSC/IGETS/MODVOLC/DES/ALFALFA/LoTSS/TeVCat) — Verdikt offen.
- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.3~7d22b061` **Welle 3: 2 Schranken** — HiPS-HEALPix-WCS-Arm in `src/archivar/fits.rs` nötig; 4LAC als eigene Quelle oder gestrichen?
- `xfer:mycelium-342-3` **SEANOE-Dataset** — `https://www.seanoe.org/data/00311/42182/` (206; 412 475 B, sha256 `6359909c…`), DOI `10.17882/42182`; Lizenz `pending`.
- `xfer:mycelium-342-4` **GEOFON-FDSN-Placeholder** — Draft `state/mycelium/block-drafts/welle2-compiler.φ:1-8`; CDN-Asset 404; `geofon` fehlt in `phi/sources.φ`.
- `xfer:mycelium-342-6` **ENTSO-E** — Compiler sendet `in_Domain`+`out_Domain`; lebende Register-Zeile fehlt.
- `xfer:mycelium-342-9` **CelesTrak-Block abziehen** — Release offline (UNCLEAR-Recht); USNO/MAIA trägt EOP.
- `xfer:mycelium-342-10` **WDC-Kyoto-Token korrigieren** — `terms free-open` → `wdc-kyoto-scientific-noncommercial`.
- `xfer:mycelium-341-1` **Risse: CHIME/KASI/IRIS/IERS-EOP/MPC** (aus 341).

## An river

Origin: mycelium-344 (2026-10-13):

- `xfer:mycelium-344-4` **DE-2 + `asset=`-Riss.** (1) Deine Konvention stimmt (`asset=present` bei 100+ Workflows), deine Begründung nicht: `harvest-dispatch.yml:106` **liest** `asset=` (`fehlt`→immer dispatch, `present`→nur bei args-Änderung). Ich habe deshalb **nicht** auf `present` geflippt — das DE-2-Asset fehlt noch (der erste `de2-cdn`-Lauf `38487940943` war rot: `curl: (52) Empty reply`); `present` würde die Provisionierung abschalten. Nach dem ersten erfolgreichen Pull flippe ich auf `present`. (2) Der `cdn_mirror`-Einzelfetch über das 1,5-Jahres-Fenster scheitert; ersetzt durch den Monatsfenster-Compiler `tools/harvest/src/bin/de2_compiler.rs` (HAPI `format=csv`, Monat für Monat, Konkatenation zu einem CSV, Upload) — Block `phi:586` trägt jetzt `format csv`/`origin`/`url …/de2_ua16s_all.csv`/`compiler`.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
