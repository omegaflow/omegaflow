<!--
  title: Handover — Mycelium-Folge 299 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. cmb-SPT-Resume gebaut (--tar + persistenter $HOME-Cache); sb_radar_compiler gebaut (Feld pending); open-lidar-data-COPC-Manifestator gebaut (CRS-Riss 31370); vnp46a3-Fetch-Diagnostik (HTTP-Code); surfrad_compiler (orphan, committet).
  class: handover
  date: 2026-10-10
  sha256: 365000976085659c476acdf8bab86070f5d8fe9e519cdd43763093ff7f2c2e20
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
- **Trigger:** Lauf `vnp46a3-cdn` Abschluss am neuen HEAD
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38045886149`) der Lauf starb an
  `vnp46a3: fetch https://data.laadsdaac.earthdatacloud.nasa.gov/prod-lads/VNP46A3/
  VNP46A3.A2026213.h18v07.002.2026252141449.h5 returned (exit status: 22)` — curl
  HTTP-Fehler **bei vorhandenem `EARTHDATA_EDL_TOKEN`** (im Log maskiert, nicht leer;
  der `Authorization: Bearer`-Header stand zur Laufzeit bereits im Compiler,
  Commit `b61840610`). Die frühere Braucht („kein Auth-Header") war eine falsche
  Prämisse. `fetch()` gibt jetzt curl's stderr (den HTTP-Code) im Log aus.
- **Blockade:** HTTP-Code noch nicht sichtbar (vor der Diagnostik-Änderung).
- **Braucht:** Re-Dispatch `vnp46a3-cdn` am neuen HEAD; der Log nennt den Code.
  Bei 401/403: Auth-Route — Token-Erneuerung/DAAC-Autorisierung als Operator-Hand.

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
- **Blockade (Riss, nicht geglättet):** die CRS-Inverse für EPSG:31370/3812. Der Reader
  dekodiert die COPC-Datei, aber `resolve_crs` braucht Belgisch Lambert. Zwei Linien
  konvergieren nicht: die klassische ESRI-Definition (lon0=4,367975; lat1=49,8333;
  lat2=51,1667) und die gemessene lokale EPSG-Registry `/usr/share/proj/proj.db`
  (lon0=4,2202952; lat1=51,1; lat2=49,5). Der Compiler trägt vorerst die ESRI-Linie.
- **Braucht:** (a) die WKT/VLR der BE-Datei in CI messen (Compiler-`--inspect` gegen die
  gemessene COPC-URL) → den Riss auflösen, DANN `open-lidar-data-cdn.yml` dispatchen;
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

### SURFRAD-Compiler — Registrierung (orphan committet)
- **Status:** eigen | **Bindung:** eigen (Ernte)
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) `tools/harvest/src/bin/surfrad_compiler.rs`
  (566 Z., baut grün, Stundenmittel shortwave_down/direct_normal, `format surfrad`,
  `at earth`) lag als orphaned staged `A` im Baum und wurde in diesem Atom
  committet; `phi/sources.φ` trägt **keinen** gml.noaa.gov/surfrad-Block.
- **Blockade:** keine.
- **Braucht:** einen Messlauf (eine Station, ein Tag) → Ausgabe-Zeilen in den
  `phi/sources.φ`-Block (Mountain-Verdikt + Mycelium-Direktive `url`/`compiler`/Tags),
  dann CDN-Manifestation.

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
