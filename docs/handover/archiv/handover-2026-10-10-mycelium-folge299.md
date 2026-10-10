<!--
  title: Handover — Mycelium-Folge 299 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. cmb-SPT-Resume gebaut (--tar + persistenter $HOME-Cache); sb_radar_compiler gebaut (Feld pending); open-lidar-data-COPC-Manifestator gebaut (CRS per --crs deklariert, Datei trägt keine); vnp46a3-Fetch-Diagnostik (HTTP-Code); surfrad_compiler (orphan, committet).
  class: handover
  date: 2026-10-10
  sha256: c8c6217b9e3ab78486d29e20a9ef99f02405c947bc11817a232adfd8878f29b2
  status: live
-->
# Handover — Mycelium-Folge 299 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge298.md` (→ `archiv/`).

**Faltung `## An mycelium`** (mountain-303): bereits in folge298 gefaltet
(Carriership-Punkt). Der Sender entfernt den Block bei seinem nächsten Pass;
kein erneuter Falt-Akt.

## Burn: open 0.0000 · close 0.1905 · cap 0.5 — Grund: Meta-Pass + 3× grind-flash (cmb-Resume, sb_radar, open-lidar-data-Manifestator) · deepseek-flash, kein pro/max (gemessen `session_burn` total $1.2187→$1.4092).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 299) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge298.md` §Operator-Wort-Register (via `git show <sha>:archiv/…`) | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — SPT-3G D1 `cmap` (cmb-cdn)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Lauf `cmb-cdn` Abschluss am neuen HEAD
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38058517809`) der Re-Lauf starb
  mit `exit code 137` mitten im 7,87-GB-Tarball-Download (4h52m); der Runner-Shutdown
  ist die Ursache, kein Transfer-Bound. Die Wurzel ist gebaut: `cmb_planck_compiler`
  hat jetzt einen `--tar <lokale.tar.bz2>`-Modus (gemeinsame
  `tarball_member_from_bytes`), `.github/workflows/cmb-cdn.yml` lädt den Tarball
  vorgeschaltet mit `curl -C -` in den persistenten Pfad
  `$HOME/.cache/omegaflow/full_maps_d1.tar.bz2` (überlebt den Runner-Stop; der
  Checkout wird gewischt).
- **Blockade:** der Re-Download startet am neuen HEAD.
- **Braucht:** bei success den sha256 in den SPT-`cmap`-Block
  (`cmb_spt_d1_n64.json`); ein abgebrochener Resume-Lauf nennt seinen Offset.

### CI — VNP46A3-CDN LAADS-Auth
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Lauf `vnp46a3-cdn` nach Token-Autorisierung
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38080557462`) `vnp46a3: fetch
  …A2026213.h18v07… returned (exit status: 22) — curl: (22) The requested URL returned
  error: 401`. Der LAADS-Granule-Fetch antwortet **401** — mit vorhandenem
  `EARTHDATA_EDL_TOKEN` (Log maskiert, nicht leer) und dem bereits gebauten
  `Authorization: Bearer`-Header (Commit `b61840610`). Die frühere Braucht
  („kein Auth-Header") war eine falsche Prämisse; die Diagnostik (fetch gibt jetzt curl's
  stderr) hat den Code gemessen.
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
  `tools/harvest/src/bin/sb_radar_compiler.rs` ist gebaut (baut grün): `--inspect`
  nennt `fields`, sonst `field=value` je `data`-Zeile; es schreibt **kein**
  Wire-Record — die physikalische Abbildung (radar delay/doppler → 26×f64) ist
  `pending`.
- **Blockade:** das Feld (radar delay/doppler) ist nicht entschieden — Architektur,
  gehört durch die fünf Stimmen.
- **Braucht:** Rat-Entscheidung Feld + `at`-Anker (`parse.rs:298`,
  `at <zielplanet>`); dann Compiler-Lauf und den Block (`origin`/`compiler`/`terms`/
  `ttl`) schreiben — kein Block ohne Feld.

### Carriership — Asservatenkammer-Zensus (`research-api-mcp`)
- **Status:** eigen | **Bindung:** eigen (gefaltet aus mountain-303)
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) `docs/surveys/survey-2026-10-08-research-api-mcp.md`
  trägt 3 offene Marker (Consensus · Perplexity · Elicit/SciSpace). Der Survey-Stand:
  `--consensus` und `--perplexity` sind im `archive_search --help` als Arme geführt
  (keys `CONSENSUS_API_KEY`/`PERPLEXITY_API_KEY`); Elicit `descoped` (kommerziell);
  SciSpace kein Arm (`pending`, cookie-API, HTTP 403). Die Auth-Route/Keys sind
  Mycelium-Domäne.
- **Blockade:** keine.
- **Braucht:** die zwei lebenden Arme am gebauten Binary mit einem Messlauf
  bestätigen (`archive_search --consensus "<q>"`, `--perplexity "<q>"`); SciSpace als
  `pending` benannt lassen.

### Weltweite LiDAR-Landschaft + `open-lidar-data` (COPC) + Manifestator
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Operator-Wort „alle" liegt vor (gemessen: „ich möchte alle haben", Mycelium 296)
- **Lage:** (gemessen 2026-10-10) die Survey
  `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md` (401 Z., 4 Taucher, jede
  URL per `--verdict` inkl. Proton-Stufe) trägt die weltweite Landschaft: global nur
  GEDI/ICESat-2; Europa bundesweit offen, DE länderweise; Nordamerika USGS 3DEP
  (`phi/sources.φ:20260`, registriert) + NOAA NOS im Katalog; LatAm/Asien-Pazifik/
  Afrika; Unterwasser/Bathymetrie (GEBCO/ETOPO/SRTM15+/Seabed 2030/NCEI/IHO-DCDB/
  EMODnet/IBCSO). PNOA/CNIG/RO/GR/SK/PT-IGEO/BG/Hessen/Thüringen/Hamburg/IBCAO/HCMC
  liefern auf direkt UND Proton keine Antwort → außer Betrieb, kein `pending`; BIG-Indonesien
  (206) und Alaska (200) erreichbar. **Im Baum:** `src/archivar/las/` (Header/VLRs/COPC +
  LASzip-Chunk-Dekoder), `las_compiler.rs` + `las-cdn.yml` (USGS 3DEP, `format las`),
  `hayabusa-lidar-cdn.yml`, `portal_harvest.rs` (CKAN → Lizenz `redistributable`/
  `terms-unknown`/`blocked`; **NC erlaubt, ND gesperrt**), der open-lidar-data-Bucket
  `open-lidar-data.s3.amazonaws.com` (keyless, 999 COPC-Keys, `data/<Land>/…/copc/*.copc.laz`).
  **Gebaut in diesem Atom:** `tools/harvest/src/bin/open_lidar_data_compiler.rs` (COPC →
  `las`-`.bin`, Reader-Aufruf `LazDecoder`/`parse_series`/`write_bin`) +
  `.github/workflows/open-lidar-data-cdn.yml`; baut grün.
- **Blockade (aufgelöst, kein Riss):** die CRS. Gemessen 2026-10-10 (Header-Dump der
  ersten 256 KB): die Datei trägt **keine** CRS-VLR — nVLR=2, nur `copc` + `laszip`;
  die CRS ist **deklarierte Dataset-Metadaten**, kein Dateiinhalt. Die vermeintliche
  ESRI-vs-`proj.db`-Divergenz war ein Fehl-Lesen von `proj.db`; die Autorität
  (`archive_search --jina https://epsg.io/31370.wkt`) ist EPSG:31370 mit
  central_meridian 4,36748666666667, Standardparallelen 51,1666672333333 / 49,8333339,
  FE 150000.013, FN 5400088.438 (International 1924). Der Compiler nimmt die CRS jetzt als
  **deklarierte** Eingabe `--crs <epsg>` (nie code-gewählt, wie `--body`), Arm-Konstanten
  auf die gemessenen EPSG-Werte gesetzt (31370 + 3812).
- **Braucht:** (a) den vom Provider deklarierten CRS des BE-Datensatzes messen
  (`registry.opendata.aws/open-lidar-data/` → `github.com/flai-ai/open-lidar-data`),
  dann `open-lidar-data-cdn.yml` mit `--crs <epsg>` dispatchen;
  (b) je weiterer Survey-Quelle ein Manifestator (Vorlage `las_compiler.rs`/
  `copernicus_dem_compiler.rs`) + Mycelium-Registrierung. Survey-Bauordnung §„Was fehlt":
  usgs-lidar-global + noaa-nos → open-lidar-data → NL AHN4/CH swissSURFACE3D/FR IGN/UK EA/
  NZ LINZ → GEDI/ICESat-2 (HDF5) → Bathymetrie-Grid → Bayern DOM20.

### Bayern-DOM-Rasterquelle (`geodaten.bayern.de`) + DOM-/GeoTIFF-Reader
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Operator-Wort „alle" liegt vor
- **Lage:** (gemessen 2026-10-10) Bayern laser-DOM „DOM20" (nativ 0,2 m → 1 m),
  CC BY 4.0; Float32, 1 m, NHN, COG. TIFF-Predictor-Refactor geheilt (`3c4786a3b`);
  `copernicus_dem_compiler.rs` ist die Vorlage.
- **Blockade:** keine.
- **Braucht:** (a) einen COG-Kachel-URL messen; (b) ein `dom_compiler`-Bin auf
  `tiff::apply_predictor`; (c) `sources.φ`-Block (format dom, at earth, CC-BY-4.0, ttl).

### φ-Manifestation — SURFRAD (gefaltet aus mountain-303)
- **Status:** eigen | **Bindung:** eigen (Manifestation)
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10, mountain-303) `tools/harvest/src/bin/surfrad_compiler.rs`
  (Stundenmittel `surfrad_shortwave_down` + `surfrad_direct_normal` W/m², `cargo check`
  0/0; Sample `https://gml.noaa.gov/aftp/data/radiation/surfrad/tbl/2024/tbl24001.dat`
  HTTP 200). `phi/sources.φ` trägt **keinen** gml.noaa.gov/surfrad-Block.
- **Blockade (Grenzfall, Rat):** die φ-Zeile ist zweiteilig — Mycelium die
  Materialisierung (`url`/`origin`/`compiler`), Mountain das Verdikt (`terms`/`at`/`ttl`);
  kein stiller Schreibakt.
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
  gebaut; Maßnahme 3 (ci-check 4-fach-nextest-Shard) wartet auf die Messung der
  Shard-Wall-Clock.
- **Blockade:** keine.
- **Braucht:** (a) ci-check-Shard am Log bestätigen; (b) **Rat Runde 2**:
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
