<!--
  title: Handover — Mycelium-Folge 309 (2026-10-11)
  session: Mycelium-Linie — Meta-Pass, ein Atom. SPT-3G-D1 re-dispatcht; die drei neuen Compiler (modvolc/its_live/masie) mit `--ci-mode`-Upload verdrahtet und je `*-cdn.yml` gebaut; `fingrid-cdn.yml` gebaut; die freien GIC-Restquellen gemessen (EAGLE-I/OE-417 frei, aber keine GIC-Ampere-Form; BGS nur Plots → pending). Adressierte Blöcke (future-233, mountain-315, river-186×2) gefaltet.
  class: handover
  date: 2026-10-11
  sha256: 861654e9a19741df453ee55a7aa7b2aebc334fa37921123198aa29d1774f56c0
  status: live
-->
# Handover — Mycelium-Folge 309 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-11-mycelium-folge308.md` (→ `archiv/`).

## Burn: open 0.0040 · close 0.045 — Grund: 6 grind-flash (4 Workflows + 2 Compiler-Uploads) + 1 general (GIC-Restquellen), deepseek-flash, kein pro/max. (session_burn-Total nicht exakt isolierbar, geteilte DB über 21 Sessions/3 Linien.)

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | 2026-10-11 | Operator (Session, Mycelium 309) · `state/operator-gespraeche/2026-10-11-mycelium.md` |
| „Die Übergabe IST der Stand … Mycelium ist die Meta-Linie — der Stehende Pass. Sie schließt als erste." | 2026-10-11 | Operator (Session, Mycelium 309) |
| Name leer → neueste offene Mycelium-Übergabe (folge308) | 2026-10-11 | Operator (Session, Mycelium 309) |
| Faltbündel §An mycelium aus `future-folge233`, `mountain-folge315`, `mountain-folge316`, `river-186` (×2) — via `register_lookup --addressed mycelium` | 2026-10-11 | Register |

## Offen — eigen

### SPT-3G D1 `cmap` (cmb-cdn) — Fix gebaut, Lauf re-dispatcht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `cmb-cdn`-Lauf `38146848045` endet
- **Lage:** (gemessen 2026-10-11) Der partial-sky/RING-Fix liegt im Baum (`a30522141`: `pix2ang_ring` + Partial-`FitsTable`-Branch). Re-dispatcht 14:20Z → run `38146848045`; zieht `full_maps_d1.tar.bz2` (~2 h). Die roten Vorläufe `38138848760`/`38139491025` waren derselbe Kopf am Pre-Fix-SHA.
- **Blockade:** keine
- **Braucht:** (a) `ci_manage log 38146848045`; (b) bei grün sha256/Länge aus dem Release-Digest in den `phi/sources.φ:11620`-Block (Datei aktuell von fremder Linie gehalten — Mountain-Aufenthalt).

### Open-LiDAR-Konsolidierung — Set-Diff gemessen; CH-Lauf rot (2 Ursachen)
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, `ci_manage jobs 38139624730`) Der coverage-manifest-Voll-Lauf `38139624730` **failure**: `ea_lidar_index` bricht bei page 28 ab — `startIndex=27000` → HTTP 404, der Walk bleibt incomplete (exit 2); `swisstopo` ebenfalls failure (Ursache `unread` — s. Braucht). Erfolgreich: `ahn4`, `coverage --kind tnm`, `linz`, `ign_lidarhd`, `lola`. Der Sidecar-Upload ist verdrahtet (`--emit-ids` in `coverage-manifest.yml`).
- **Blockade:** `ea_lidar_index`-Pagination hat keinen Endpunkt-Test (404 statt Stop); CH-Origin-Walk > 10 000 Items.
- **Braucht:** (a) `ci_manage log 38139624730` für die `swisstopo`-Ursache (gemessen, nicht geraten); (b) `ea_lidar_index`-Walk: 404/leere Seite als natürliches Ende behandeln statt abort; (c) Mountain-Verdikt auf die NL-Basis (`state/mycelium/lidar_id_diff_measured.md`; flai ⊇ AHN4-Origin außer `08BZ1`/`40HZ1`).

### coverage-manifest — USGS-TNM + LOLA verdrahtet; weitere Arme offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `coverage.rs` trägt `--kind stac|tnm`; `coverage-manifest.yml`-Matrix um `coverage --kind tnm --limit 1000` (tnmaccess) und `lola_coverage` (astrogeo-ard) erweitert — beide im Lauf `38139624730` **success**.
- **Blockade:** keine
- **Braucht:** (a) weitere Arme `s3-copc|s3-laz|wfs|arcgis|ept`; (b) STAC-Arm als eigene Matrix-Zeile (Planetary Computer) bei Bedarf.

### Coverage-Manifest-Fetch-Loop (gefaltet aus river-folge186) — usgs-Registrierung offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, river-folge186) Der Arm `format coverage_manifest` steht in `src/archivar/main_flow.rs:5102-5224`. Riss korrigiert: `usgs_lidar_coverage.rs` emittiert bereits `shard <url>` (Commit `8739dffaa`); der echte Gap ist der `format`-Dispatch (`main_flow.rs:5103` dispatcht nur auf `coverage_manifest`, die Quelle deklariert `usgs_lidar_coverage`) + die fehlende Registrierung.
- **Blockade:** `phi/sources.φ` von fremder Linie gehalten.
- **Braucht:** (a) das USGS-Manifest mit `format coverage_manifest` registrieren (oder den Dispatch erweitern); (b) Portal-Inventar (gemessen: 124 Survey-Tabellenzeilen / 143 txt-Zeilen) → neue Arme CKAN-REST > Earthdata-Granule-Index > CSW/OGC.

### Frame/Epoche — Operator-Wort **A** gewortet + gebaut; weitere CRS offen
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, Operator-Wort **A**) `shard <url> [<6 f64> <EPSG> [<epoch(TDB s)>]]`; `resolve_placement` transformiert zur Query-Zeit (4326; UTM 326xx/327xx); unbekannt/fehlend → `Pending`. `coverage.rs` stempelt `EPSG:4326` beim `--bbox`. Test im `#[cfg(test)]`. Roster 9:2 für A: `state/stimmen/20261011T1310Z_mycelium_frame-epoche-roster.md`.
- **Blockade:** keine
- **Braucht:** (a) weitere CRS in `projection.rs` (Lambert-93/CH1903+/RD New — heute nur UTM); (b) Wire-Kontrakt-Doc fortschreiben (`shard`-Grammatik mit CRS/Epoche).

### MCP no-leak-Regel — Wrapper + Capture fertig; Allowlist erweitert
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `src/gate/mcp.rs` in `commit_check` verdrahtet; `bin/chrome-devtools-mcp` (Pin 1.9.0); Allowlist um `fe-static.deepseek.com`/`challenges.cloudflare.com`/`cdn.deepseek.com` erweitert (`jaq -e` gültig). Capture: `state/mycelium/mcp_allowlist_capture_measured.md`.
- **Blockade:** eine enge Allowlist blockt Subressourcen.
- **Braucht:** (a) nach dem Config-Restart die Seats prüfen; (b) `(2)` härter prüfen verlangt den `Gestalt`-Slot-Umfang.

### OpenTopography — Konsument gebaut; CI-Secret gesetzt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `opentopography_coverage.rs` walkt `portal.opentopography.org/API/otCatalog` (keyless; `globaldem` keyed → 401); Workflow `.github/workflows/opentopography-cdn.yml`; `bin/secrets-sync.sh --set` gesetzt.
- **Blockade:** keine
- **Braucht:** Workflow bei Bedarf dispatchen (lang; self-hosted).

### Architektur — GitHub/CI/CDN-Optimierung
- **Status:** wartend | **Bindung:** eigen · **Teile linie:mountain/river**
- **Trigger:** grüner `ci-check`-Lauf
- **Lage:** (gemessen 2026-10-11) Die neuen `ci-gate`-Läufe stehen in `subset → Run cargo test --lib` (mehrere queued). Neue Workflows dieses Atoms (`modvolc`/`its_live`/`masie`/`fingrid-cdn.yml`) sind noch nicht dispatcht.
- **Blockade:** Shard-Build-vs-Test-Anteil erst an green ci-check messbar.
- **Braucht:** (a) `ci_manage status` auf grünen `ci-check` → `ci_manage jobs <id>`; (b) dominiert Build: `cargo nextest archive` + Shards `--archive-file`; (c) Test-Suite-Dedup.

### matrix-rotor — persistenter t420 trägt die Klasse; Lauf transient abgebrochen
- **Status:** wartend | **Bindung:** eigen (Workflow) · **Teile linie:river**
- **Trigger:** nächster `matrix-rotor`-Lauf
- **Lage:** (gemessen 2026-10-11, `ci_manage log 38143487268`) Der Lauf **failure** — Ursache gemessen: **„The runner has received a shutdown signal"** (SIGTERM 14:10:59, „The operation was canceled"); kein Assertion-Rot. Ein Runner-Shutdown bleibt transient; der persistente t420 (`bin/matrix_watchdog.sh`) trägt die Klasse. Die Doc/Code-Divergenz `matrix_watchdog.sh:4` vs `matrix.rs:33` ist geschlossen.
- **Blockade:** nur der persistente t420 ist dauerhaft.
- **Braucht:** —

### LiDAR-Quellen-Inventar — TNM gemessen + verdrahtet; weitere Bucket-Bins offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `state/mycelium/usgs_tnm_index_measured.md`: TNM Access HTTP 200, 9 755 359 LPC-Produkte; `--kind tnm` in `coverage-manifest.yml` verdrahtet (Lauf success).
- **Blockade:** keine
- **Braucht:** (a) je Bucket ein `<x>_coverage.rs`; (b) COPC/EPT bleibt S3/PC-Azure.

### Extraterrestrische LiDAR — `lola_coverage` verdrahtet; MOLA/MLA/NLR/OLA/Hayabusa-Parser offen
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `lola_coverage.rs` walkt `astrogeo-ard` `moon/lro/lola` (S3 ListObjectsV2), emittiert url-only `shard <url>`; im Lauf `38139624730` success.
- **Blockade:** die nicht-COPC-Körper brauchen Parser (Mountain).
- **Braucht:** Parser für MOLA/MLA/NLR/OLA/Hayabusa (Mountain).

### Bathymetrie/Raster — BlueTopo/EMODnet Compiler gebaut; Register-Blöcke + BlueTopo-Gestalt offen
- **Status:** eigen | **Bindung:** eigen · **Teile linie:river**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `FieldIdentity::Witness(WitnessKind::Gestalt)` trägt die Raster. Gebaut: `bluetopo_compiler.rs` (GMR1-Gestalt) + `emodnet_bathymetry_compiler.rs` (neue `EMOD`-Gestalt); `cargo check` 0/0.
- **Blockade:** die Register-Blöcke in `phi/sources.φ` (Datei von fremder Linie gehalten) + BlueTopo eigene Gestalt.
- **Braucht:** (a) Register-Blöcke (Mountain-Aufenthalt); (b) BlueTopo eigene Gestalt-Identität (heute unter `GMR1` — Riss benannt); (c) UI-Schärfung: Sampling-Parameter + vertikales Datum.

### DEMETER (CDPP/REGARDS) — Lauf in Arbeit; Register-Zeile offen
- **Status:** wartend | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** Lauf `demeter-cdn` run `38135966376` endet
- **Lage:** (gemessen 2026-10-11, `ci_manage view 38135966376`) Der Lauf ist **in_progress** (created 11:32Z, updated 11:33Z, selbstgehosteter `demeter-residential`-Runner). Korrektur aus 308 gehalten: die Bin trägt 6 ISL-Langmuir-Sonden-Größen, **kein** ICE-E-Feld; die Monats-Assets `demeter_isl_YYYYMM.bin` existieren noch nicht. **Riss (mountain-folge316, 2026-10-11):** die Lizenz „Licence Etalab-2.0" (308:118) ist bestritten — mountain-316 nennt CDPP Data-Centre-Lizenz V1.1 (nicht-kommerziell, Weitergabe an Dritte nur mit schriftlicher Zustimmung). `archive_search --verdict https://cdpp.cnes.fr` = HTTP 206 (1 B), kein Lizenztext im Kopf. **Kein url/origin bis der Riss gemessen ist.** `phi/sources.φ` trägt **keine** DEMETER-Zeile.
- **Blockade:** eine neue Quellen-Zeile = gemeinsamer Akt (url/origin/compiler = Mycelium; terms/field = Mountain); `phi/sources.φ` von fremder Linie gehalten; **Lizenz-Riss ungemessen**.
- **Braucht:** (a) `ci_manage view 38135966376` auf Ende; danach die Monats-Asset-Namen des Tags messen; (b) **zuerst** die CDPP-Lizenz am Terms-Pfad messen (`archive_search --playwright https://cdpp.cnes.fr/` → Terms/Legal; bei nicht-kommerziell nur lokale Registrierung, kein CDN-Mirror); (c) Mycelium-Register: `url …/regards.cnes.fr/demeter_isl_YYYYMM.bin` · `format demeter_isl` · `origin https://regards.cnes.fr/api/v1/rs-catalog/engines/legacy/dataobjects/search?q=DatasetName:DMT_N1_1144` · `compiler tools/harvest/src/bin/demeter_compiler.rs` · `at earth` · `ttl 604800`; Mountain: `terms <Lizenz> <URL>` + 6 `field`-Zeilen.

### NEU — drei neue Compiler manifestiert (modvolc/its_live/masie) + fingrid; Register-Aufenthalt
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, dieses Atom) **Gebaut:** `modvolc_compiler` + `its_live_compiler` um `--ci-mode`-`upload_release` erweitert (masie/fingrid-Muster), `cargo check -p omegaflow-harvest` 0/0. **Vier Workflows neu** unter `.github/workflows/`: `modvolc-cdn.yml`, `its_live-cdn.yml`, `masie-cdn.yml`, `fingrid-cdn.yml`. `masie` (ubuntu-latest, täglich), `its_live` (wöchentlich Mo), `modvolc` (self-hosted; Quelle gemessen: `http://modis.higp.hawaii.edu/data/<year>/<doy>/MODVOLC.A<yyyyddd>.<HHMM>.061.NRT.dat`, HTTP 206), `fingrid` (self-hosted; `data.fingrid.fi`, `read_secret` liest **nur** `.secrets.local` → Workflow materialisiert sie aus `secrets.FINGRID_API_KEY`).
- **Blockade:** `fingrid` hat **keine** Zeile in `phi/sources.φ` (der Compiler schreibt rohes API-JSON — eine `format`-/Consumer-Entscheidung fehlt); die übrigen drei sind registriert (mountain-315).
- **Braucht:** (a) die vier neuen Workflows dispatcht messen (nach Push); (b) fingrid-Register-Aufenthalt + `format`-/Consumer-Entscheidung (Mountain/Mycelium gemeinsam); (c) `FINGRID_API_KEY` im CI-Secret prüfen.

### NEU — GIC-Restquellen gemessen (river-186): EAGLE-I/OE-417 frei, BGS pending
- **Status:** eigen | **Bindung:** eigen · **Teile linie:river/mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) Fingrid-Workflow gebaut (s. o.). **EAGLE-I** (ORNL/figshare) frei CC-BY-4.0: per-year CSV `fips_code,county,state,customers_out,run_start_time`; **kein GIC-Ampere** — Netz-End-`count`, braucht FIPS→Zentroid-Join. **OE-417** frei Public-Domain: ORNL-Semicolon-CSV (`openenergyhub.ornl.gov/api/records/1.0/download/?dataset=oe-417-annual-summaries&format=csv`), 341 Zeilen, Event-Tabelle (`number_of_customers_affected`/`demand_loss_mw`), braucht eigenen Delimiter-Parser. **BGS GIC** — Riss: die Dossiers nennen es „frei", gemessen liefert BGS **nur Plots** (BGIC-Index, modelliertes E-Feld), keine Bulk-Serie, Lizenz unklar → `pending` (Service/`spaceweather@bgs.ac.uk`).
- **Blockade:** EAGLE-I/OE-417 passen nicht auf die GIC-`electric`/Ampere-Form — sie brauchen eine neue `field`-Familie (count/power), also eine Mountain-Feldentscheidung.
- **Braucht:** (a) Mountain-Feldentscheidung count/power → dann je Compiler+Workflow (EAGLE-I/OE-417); (b) BGS: `archive_search --playwright https://geomag.bgs.ac.uk/data_service/space_weather/gic_services.html` + Webservice-Roots auf `.csv/.dat` prüfen; sonst `descoped` mit Befund.

### NEU — Re-Admission-CDNs (mountain-316): vier Neu-Admits brauchen url/origin
- **Status:** wartend | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** Mountains `terms`-Verdikt je Kandidat
- **Lage:** (gemessen 2026-10-11, mountain-316) Vier Kandidaten sind re-admittiert — GRDC-Caravan, RouteViews, EPH-VSOP, GloFAS; nach Mountains `terms`-Verdikt brauchen sie je `url`/`origin` (Mycelium) für die CDN-Manifestation. See-also: `docs/surveys/survey-2026-10-09-redistribution-alternativen.md`.
- **Blockade:** Mountains `terms`-Verdikt ausstehend.
- **Braucht:** je Kandidat die Register-Direktive (`url`/`origin`/`compiler`/`format`/`at`/`ttl`) nach dem terms-Verdikt.

## Registrierte Korrekturen & Vorfälle

- **BGS GIC „frei" ist eine Doc-Zeile, kein Zustand (gemessen 2026-10-11):** `state/river/gic-sources-broad-2026-10-11.md:34-35` / `handover-…-river-186.md:79,105` listen BGS GIC als frei; gemessen liefert BGS nur Plots. → `pending`, nicht frei.
- **EAGLE-I-Reihe reicht bis 2025 (gemessen):** das figshare-Record trägt `eaglei_outages_2025.csv` (1,4 GB) — die Dossiers datieren 2014–2023.
- **matrix-rotor rot = Runner-Shutdown (gemessen):** `38143487268` failure durch SIGTERM, kein Assertion-Rot.
- **mountain-315-Claims geprüft:** die drei Compiler verlangen `--body` — `modvolc`/`its_live` taten es, `masie` nicht (masie nimmt `--url`/`--out`); nur `masie` uploade bereits, `modvolc`/`its_live` **nicht** — dieser Riss ist in diesem Atom geschlossen (beide um `--ci-mode` erweitert).
- **Vier adressierte Blöcke gefaltet:** `future-folge233`, `mountain-folge315`, `mountain-folge316`, `river-folge186` (river-182 + river-186 §An mycelium).
- **DEMETER-Lizenz-Riss (mountain-316):** „Etalab-2.0" (308:118) vs. CDPP Data-Centre-Lizenz V1.1 (nicht-kommerziell) — ungemessen; vor url/origin messen.

## An mountain

Origin: mycelium-309 (2026-10-11).

- **Drei neue Compiler workflow-manifestiert** (mountain-315 erledigt): `modvolc-cdn.yml`, `its_live-cdn.yml`, `masie-cdn.yml`. `modvolc`/`its_live` haben jetzt `--ci-mode`-Upload (Quelle gemessen für modvolc). Bitte die drei Läufe nach dem Push mitlesen.
- **GIC-Restquellen:** EAGLE-I und OE-417 sind frei/keyless, aber **keine GIC-Ampere-Form** — sie brauchen eine neue `field`-Familie (count aus `customers_out`; power aus `demand_loss_mw`) + je einen Compiler. BGS GIC: nur Plots → `pending`. Entscheidung: Feld-Familie count/power freigeben?
- **Descope-Verdikt Open-LiDAR** unverändert offen (`state/mycelium/lidar_id_diff_measured.md`).
- **DEMETER Register-Block** (`regards.cnes.fr`) — `terms Etalab-2.0 <URL>` + 6 `field`-Zeilen, parallel zu Mycelium (Lauf in Arbeit).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
