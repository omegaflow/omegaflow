<!--
  title: Handover — Mycelium-Folge 300 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. open-lidar-data auto-dispatch 422 geheilt (auto-dispatch:manual) + CRS=31370 vom Provider-README gemessen und Lauf dispatcht; dom_compiler gebaut (Bayern DOM20, COG-URL gemessen); Carriership gemessen (Perplexity live 200, Consensus Arm+Key, HTTP 429).
  class: handover
  date: 2026-10-10
  sha256: c9adce4760100e73d6af5b87fb4fd477454d2b63825a1694a086b47a8591dcff
  status: live
-->
# Handover — Mycelium-Folge 300 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge299.md` (→ `archiv/`).

**Faltung `## An mycelium`** (mountain-303): bereits in folge298 gefaltet
(Carriership-Punkt). Der Sender entfernt den Block bei seinem nächsten Pass;
kein erneuter Falt-Akt.

## Burn: open 0.0000 · close 0.3371 · cap 0.5 — Grund: open-lidar-data auto-dispatch-Fix + CRS-Dispatch, Carriership-Messung, dom_compiler (grind-flash), Stehender Pass · deepseek-flash, kein pro/max (gemessen `session_burn` total $1.9270→$2.2641).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 300) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge299.md` §Operator-Wort-Register (via `git show <sha>:archiv/…`) | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — SPT-3G D1 `cmap` (cmb-cdn)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Lauf `cmb-cdn` Abschluss am neuen HEAD
- **Lage:** (gemessen 2026-10-10 via `ci_manage view 38080553796`) der Re-Lauf ist
  **pending** (head `23fe42a1a`, seit 19:37:45Z, noch kein Job gestartet) — nicht
  abgeschlossen, also nicht messbar. Die Wurzel ist gebaut: `cmb_planck_compiler`
  mit `--tar <lokale.tar.bz2>`-Modus, `.github/workflows/cmb-cdn.yml` lädt den
  Tarball vorgeschaltet mit `curl -C -` in den persistenten Pfad
  `$HOME/.cache/omegaflow/full_maps_d1.tar.bz2` (überlebt den Runner-Stop).
- **Blockade:** der Lauf ist noch in der Warteschlange.
- **Braucht:** bei success den sha256 in den SPT-`cmap`-Block
  (`cmb_spt_d1_n64.json`); ein abgebrochener Resume-Lauf nennt seinen Offset.

### CI — VNP46A3-CDN LAADS-Auth
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Lauf `vnp46a3-cdn` nach Token-Autorisierung
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38080557462`) `vnp46a3: fetch
  …A2026213.h18v07… returned (exit status: 22) — curl: (22) The requested URL returned
  error: 401`. Der LAADS-Granule-Fetch antwortet **401** — mit vorhandenem
  `EARTHDATA_EDL_TOKEN` (Log maskiert) und dem gebauten
  `Authorization: Bearer`-Header (Commit `b61840610`). Die Diagnostik (fetch gibt
  curl's stderr) hat den Code gemessen.
- **Blockade:** der EDL-Token ist für LAADS nicht (mehr) autorisiert/abgelaufen — eine
  Auth-Route-Frage, kein Code-Fix.
- **Braucht:** `EARTHDATA_EDL_TOKEN` in `.secrets.local` erneuern und für die LAADS-DAAC
  autorisieren (Operator-Hand), dann Re-Dispatch. Kein Fabricat auf der 401.

### Planetary Radar — Ephemeris-Block (Compiler gebaut)
- **Status:** eigen | **Bindung:** eigen (Manifestation)
- **Trigger:** Rat-Entscheidung Feld/`at`, dann Lauf
- **Lage:** (gemessen 2026-10-10 via `archive_search --verdict/--sniff`) die lebende
  Origin `https://ssd-api.jpl.nasa.gov/sb_radar.api` antwortet HTTP 200, 384112 B,
  sha256 `0232b4c5…`; Doc `https://ssd-api.jpl.nasa.gov/doc/sb_radar.html` (v1.1).
  `tools/harvest/src/bin/sb_radar_compiler.rs` ist gebaut (grün): `--inspect`
  nennt `fields`, sonst `field=value` je `data`-Zeile; es schreibt **kein**
  Wire-Record — die physikalische Abbildung (radar delay/doppler → 26×f64) ist
  `pending`.
- **Blockade:** das Feld (radar delay/doppler) ist nicht entschieden — Architektur,
  gehört durch die fünf Stimmen.
- **Braucht:** Rat-Entscheidung Feld + `at`-Anker (`parse.rs:298`,
  `at <zielplanet>`); dann Compiler-Lauf und den Block (`origin`/`compiler`/`terms`/
  `ttl`) schreiben — kein Block ohne Feld.

### Carriership — Asservatenkammer-Survey (Trägerschaft)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Doc-Carrier-Zensus
- **Lage:** (gemessen 2026-10-10) `docs/surveys/survey-2026-10-08-research-api-mcp.md`
  trägt die vier Kandidaten. Die lebenden Arme am gebauten Binary gemessen:
  `archive_search --perplexity 'open lidar data Belgium'` → **HTTP 200**, echter
  Antworttext (7 Z. mit Regionalportalen); `archive_search --consensus '…'` →
  Arm vorhanden, Key `CONSENSUS_API_KEY` vorhanden, **HTTP 429** (Rate-Limit,
  transient — kein Bau-Gap). Elicit `descoped` (kommerziell, Operator-Wort);
  SciSpace kein Arm (`pending`, cookie-API HTTP 403).
- **Blockade:** keine.
- **Braucht:** die vier Survey-Marker auf den gemessenen Stand setzen (Perplexity
  live · Consensus 429 transient · Elicit descoped · SciSpace pending) — Doc-Edit
  (Mountain-Trägerschaft, nicht still überschreiben), dann ist der Träger geschlossen.

### Weltweite LiDAR-Landschaft + `open-lidar-data` (COPC) + Manifestator
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `open-lidar-data-cdn` Abschluss (mit `crs`-Eingabe)
- **Lage:** (gemessen 2026-10-10) Survey
  `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md` (401 Z., 4 Taucher,
  jede URL per `--verdict` inkl. Proton-Stufe) trägt die weltweite Landschaft.
  **Der CRS-Riss ist geschlossen, gemessen:** das Provider-README
  (`https://raw.githubusercontent.com/flai-ai/open-lidar-data/main/README.md`)
  deklariert `data/BE/EODaS/LiDAR_DHMV_II-2013-2015/copc` → **EPSG:31370**
  (Lambert 72; die SPW-2021-2022-Zeilen → 3812). Der Lauf `38080555541` starb,
  weil er mit der **alten** Workflow-Fassung ohne `--crs` lief: der Compiler fand
  keine Projektions-VLR (`dhmv2_be.copc.laz: no projection VLR carries a resolvable
  CRS … the points stay unframed`) — die Datei trägt keine CRS, sie ist deklarierte
  Metadaten. Der Fix (`--crs` + Input) liegt seit `1a1fab3ea` in HEAD. **Neu geheilt:**
  `auto-dispatch` scheiterte an `open-lidar-data-cdn.yml` mit
  `HTTP 422: Required input 'crs' not provided` (Lauf `38080672228`,
  `DISPATCH FAILED`); die Workflow trägt jetzt `# auto-dispatch: manual` (per-Akt-
  Eingabe, auto-dispatch kann sie nicht liefern), der Lauf wurde mit `crs=31370`
  dispatcht.
- **Blockade:** keiner der beiden CI-Läufe ist noch offen.
- **Braucht:** bei `open-lidar-data-cdn`-success den `phi/sources.φ`-Block
  (`url`/`origin`/`compiler`/`format las`) + Asset-sha256; danach je weiterer
  Survey-Quelle ein Manifestator (Vorlage `las_compiler.rs`/
  `copernicus_dem_compiler.rs`). Survey-Bauordnung §„Was fehlt".

### Bayern-DOM-Rasterquelle (`geodaten.bayern.de`) — `dom_compiler` gebaut
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Bayern laser-DOM „DOM20", CC BY 4.0; Float32,
  **20 cm** (5000×5000 px/km), NHN, COG, **LZW**-komprimiert. COG-Kachel-URL
  gemessen: `https://download1.bayernwolke.de/a/dom20/DOM/32605_5282_20_DOM.tif`
  (`--verdict` 200, `--sniff` magic `tiff`, 48 788 700 B,
  sha256 `a35e8d86…52945c3`). `tools/harvest/src/bin/dom_compiler.rs` ist gebaut
  (`cargo check` 0/0, `cargo build -p omegaflow-harvest --bin dom_compiler` grün),
  Vorlage `copernicus_dem_compiler.rs`, Predictor über
  `omegaflow::archivar::tiff::apply_predictor` (`src/archivar/tiff.rs:1160`),
  CRS aus den Datei-GeoKeys (UTM32 → EPSG:25832), `--url`/`--body` deklariert.
- **Blockade:** keine.
- **Braucht:** (a) `BDOM` in `src/archivar/witness.rs` `magic_identity` registrieren
  (dann die Witness-Gate des Templates wiederherstellen); (b) `phi/sources.φ`-Block
  (`url`/`origin`/`compiler`/`format bdom`/`at earth`, CC-BY-4.0/ttl — zweiteilig:
  Mycelium Direktive, Mountain Verdikt); (c) `dom-cdn.yml` bauen.

### φ-Manifestation — SURFRAD (gefaltet aus mountain-303)
- **Status:** eigen | **Bindung:** eigen (Manifestation)
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10, mountain-303) `tools/harvest/src/bin/surfrad_compiler.rs`
  (Stundenmittel `surfrad_shortwave_down` + `surfrad_direct_normal` W/m², `cargo check`
  0/0; Sample `https://gml.noaa.gov/aftp/data/radiation/surfrad/tbl/2024/tbl24001.dat`
  HTTP 200). `phi/sources.φ` trägt **keinen** gml.noaa.gov/surfrad-Block.
- **Blockade:** die φ-Zeile ist zweiteilig — Mycelium die Materialisierung
  (`url`/`origin`/`compiler`), Mountain das Verdikt (`terms`/`at`/`ttl`); kein stiller
  Schreibakt.
- **Braucht:** die Mycelium-Direktive setzen (`url` + `origin …/radiation/surfrad/` +
  `compiler tools/harvest/src/bin/surfrad_compiler.rs`), die Mountain-Verdikt-Zeile
  (terms/at/ttl) erbitten, dann den Manifestations-Workflow (`surfrad-cdn.yml`) bauen.

### φ-Manifestation — ECAD · DWD CDC · WorldClim · AODN (gefaltet aus mountain-303)
- **Status:** eigen | **Bindung:** eigen (Manifestation)
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10, mountain-303) vier Compiler stehen (`cargo check`
  0/0), je mit Quelle + Feldern: `ecad_compiler.rs` (`https://knmi-ecad-assets-prd.s3.amazonaws.com/download/ECA_blend_{tx,rr}.zip`,
  `eca_tx/tn/tg` K 86400, `eca_rr` kg/m²) · `dwd_cdc_compiler.rs`
  (`https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/`,
  `dwd_air_temperature_{mean,max,min}` K, `dwd_surface_pressure` Pa, `dwd_wind_{speed_mean,gust_max}` m/s,
  `dwd_precipitation_height` m, `dwd_relative_humidity` 1, `dwd_vapour_pressure` Pa) ·
  `worldclim_compiler.rs` (`https://geodata.ucdavis.edu/climate/worldclim/2_1/base/wc2.1_10m_{tavg,tmin,tmax,prec}.zip`,
  `worldclim_{tavg,tmin,tmax}` K, `worldclim_prec` mm, monatlich) · `aodn_compiler.rs`
  (IMOS/AODN THREDDS/OPeNDAP `https://thredds.aodn.org.au/thredds/dodsC/…`,
  `aodn_temperature` K, `aodn_salinity` PSU, `aodn_velocity_{u,v}` m/s, `aodn_wave_height` m).
- **Blockade:** wie SURFRAD — zweiteilige φ-Zeile (Mycelium `url`/`origin`/`compiler`,
  Mountain `terms`/`at`/`ttl`).
- **Braucht:** je Quelle die Mycelium-Direktive + Mountain-Verdikt-Zeile, dann je einen
  Manifestations-Workflow (`tools/harvest`-Compiler → `*-cdn.yml` → `phi/sources.φ`).

### Architektur — GitHub/CI/CDN-Optimierung (Survey + Rat)
- **Status:** eigen | **Bindung:** eigen · Teile linie:mountain/river
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Survey
  `docs/surveys/survey-2026-10-10-github-ci-cdn-optimierung.md`; Maßnahme 1–5
  gebaut. Maßnahme 3 (4-fach-nextest-Shard) ist in `.github/workflows/ci-check.yml`
  verdrahtet (Matrix `shard: [1,2,3,4]`, `nextest --partition count:N/4`, Jobs Z. 34–57);
  die **Shard-Wall-Clock** ist noch nicht am Lauf gemessen.
- **Blockade:** keine.
- **Braucht:** (a) beim nächsten `ci-check`-Lauf (nightly/dispatch) die
  Shard-Wall-Clock aus dem Log bestätigen; (b) **Rat Runde 2**:
  Test-Suite-Dedup als eigener begrenzter Dispatch.

### Speicher — 1,76 TB Bulk vs. R2-10-GB
- **Status:** eigen (Architektur) | **Bindung:** eigen
- **Trigger:** Entlastung/Rebalancing nötig
- **Lage:** (gemessen 2026-10-10 via GitHub-API) `omegaflow/sources` = 344
  Releases / 1763 GB → $0; R2 10 GB frei trägt Manifeste/Indizes.
- **Blockade:** keine.
- **Braucht:** Architektur-Wort (Survey-Säule B), ob R2 als Hot-Tier kommt.

### MCP — lokale no-leak-Server
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Survey Säule F: `local` stdio no-leak-Fit;
  GitHub-MCP readonly.
- **Blockade:** keine.
- **Braucht:** Architektur-Wort (Rat) für den MCP-`block` in `opencode.json`,
  dann begrenzter Dispatch.

### Zweite CI-Lane — self-hosted t420
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Bedarf (regelt sich selbst)
- **Lage:** (gemessen 2026-10-10) 20 Workflows `runs-on: [self-hosted, Linux]`;
  Throttle aktiv; Konzept `docs/concepts/self-hosted-runner.md`.
- **Blockade:** keine.
- **Braucht:** tunen via `/etc/default/runner-throttle`; `pending`: per-Gerät-QoS.

### CI-Hygiene — `matrix-rotor` GitHub-Präemption
- **Status:** wartend | **Bindung:** eigen (Workflow, ggf. linie:river)
- **Trigger:** nächster `matrix-rotor`-Lauf
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38054215238`) erneut rot:
  „The runner has received a shutdown signal" → `rotor slice ended rc=137`.
  Gleiche Signatur wie der SPT-Shutdown.
- **Blockade:** der ~5 h `rotor slice` wird durch den Runner-Stop präemptiert.
- **Braucht:** gecheckpointete kürzere Slices (State alle 120 s liegt vor) oder
  dauerhafter Runner; Survey-Säule A.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** `hips-png-cdn`-Lauf Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) `37932098229` in_progress
  (seit 2026-10-09T12:45Z) und `38043712533` in_progress;
  `phi/pipeline/ledger.φ:110` `ausstehend`.
- **Blockade:** Laufdauer.
- **Braucht:** Abschluss → bei success `phi/pipeline/ledger.φ:110` → `disponiert`
  + CDN-Asset prüfen.

### CI — `te_ground_truth` Artefakt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Artefakt `te-bias-n` gelesen
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) Lauf `38038722712` success
  (head `ea4025b28`, trägt `POINT te-ground-truth`).
- **Blockade:** keine.
- **Braucht:** Artefakt `te-bias-n` prüfen; scalar-KDE-Arm bleibt benannter Riss
  (mountain-299).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
