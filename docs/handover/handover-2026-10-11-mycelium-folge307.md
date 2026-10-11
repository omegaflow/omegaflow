<!--
  title: Handover — Mycelium-Folge 307 (2026-10-11)
  session: Mycelium-Linie — Meta-Pass, ein Atom. Set-Diff-Schema für LiDAR-Konsolidierung gebaut (`coverage.rs --emit-ids`: tile-id/epoch/href-Sidecar, `shard`-Manifest byte-identisch); extraterrestrischer LOLA-COPC-Emitter `lola_coverage.rs` (astrogeo-ard `moon/lro/lola`, url-only); USGS-TNM-Access gemessen (9 755 359 LPC-Zeilen, ein US-weiter LAZ-Index; COPC/EPT = 0 → nur S3/PC-Azure).
  class: handover
  date: 2026-10-11
  sha256: 334a8832f06ad1e767e02ebc64dfadd491b09447f9c83ade57eb2fcfe5996be2
  status: live
-->
# Handover — Mycelium-Folge 307 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-11-mycelium-folge306.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.1163 · cap 0.5 — Grund: 3 Taucher (alle grind-flash: 2 Bau + 1 Messung). deepseek-flash, kein pro/max. (session_burn total 1.6303 → 1.7466.)

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-11 | Operator (Session, Mycelium 307) |
| Name leer → neueste offene Mycelium-Übergabe (folge306) | 2026-10-11 | Operator (Session, Mycelium 307) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-11-mycelium-folge306.md` §Operator-Wort-Register (via `git show <sha>:docs/handover/archiv/…`) | 2026-10-11 | gefaltet, nicht kopiert |

## Offen — eigen

### Open-LiDAR-Konsolidierung — Set-Diff-Schema in `coverage.rs` gebaut; Adapter-Abdeckung + ID-Diff-Lauf offen
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `coverage.rs` trägt jetzt `--emit-ids <path>`: Sidecar `<id>\t<epoch>\t<href>` aus STAC `id`/`properties.datetime`, fehlend = `-`; `shard`-Manifest byte-identisch; `cargo check`/build grün, Test `emit_ids_writes_tile_identity_and_absent_fields_render_as_dash` (CI). Gemessene Basis unverändert: `state/mycelium/flai_coverage_measured.md` (NL AHN4 2771 COPC ≥ Origin 1407 `.LAZ`; CH 79683 ≫ 70; FR partiell 337212/524687) + Z.ai-Bedingung (`state/stimmen/20261011T095149Z_zai_lidar-verdikt.md`).
- **Blockade:** der ID-Diff verlangt **dasselbe** Schema in *allen* Adaptern — die per-Portal-Bins (`ahn4_coverage.rs`/`swisstopo_coverage.rs`/`ign_lidarhd_coverage.rs`/`open_lidar_data_coverage.rs`) emittieren weiter nur `shard <url>`, kein id/epoch; der Diff-Lauf selbst fehlt.
- **Braucht:** (a) `--emit-ids` (oder äquivalentes id/epoch-Schema) in den per-Portal-Adaptern; (b) ID-Diff-Lauf flai⊇origin je Land; (c) Mountain-Verdikt auf die gemessene Basis (descope je Land, Origin-Manifeste behalten).

### coverage.rs — STAC-Arm + id/epoch-Schema; weitere Arme offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `tools/harvest/src/bin/coverage.rs` baut `--kind stac` (Planetary Computer `3dep-lidar-copc`), url-only Default (`--bbox` opt-in), jetzt + `--emit-ids` Sidecar; `cargo check`/build 0/0.
- **Blockade:** keine.
- **Braucht:** (a) `coverage-manifest.yml`-Matrix-Eintrag mit benanntem Page-Bound (MAX_PAGES=5000, PAGE_LIMIT=1000); (b) die weiteren Arme `s3-copc|s3-laz|wfs|arcgis|ept|tnm` als Ausbaustufen (Z.ai: alle Adapter dasselbe Output-Schema).

### Frame/Epoche → bbox-off als sicherer Default; Transform-Layer `pending`
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `state/mycelium/frame-epoch-findings.md`: drei bbox-Emitter (`open_lidar_data_coverage`/`noaa_nos_coverage`/`coverage.rs`) defaulten url-only (`ShardPlacement::Pending` → nicht verworfen). Fünf Portal-Bins bereits url-only.
- **Blockade:** Transform-Layer braucht CRS-aus-VLRs (zweiter Range-Request) + projiziert→geodätisch (fehlt) + Epoche je Shard + Grammatik-Slot.
- **Braucht:** Transform-Layer als eigener Bau oder dauerhaft bbox-off; Entscheidung bei Mountain (Quellen-Verdikt-Rahmen).

### Manifestation SPT-3G D1 `cmap` (cmb-cdn) — dispatched; sha256 offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `cmb-cdn.yml` run 38129785553 endet
- **Lage:** (gemessen 2026-10-11 09:56Z) Streaming-Compiler gebaut; Workflow run 38129785553 noch `in_progress` (Step `cmb → Compile the SPT-3G D1 CMB map`). Trigger nicht gefeuert.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 38129785553` (bzw. `log`), dann sha256/Länge aus dem Release-Digest in den `phi/sources.φ:11559`-Block.

### MCP no-leak-Regel — Gate gebaut; Wrapper + egress-Allowlist offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `src/gate/mcp.rs` (6 Regeln + Test) in `commit_check` verdrahtet; `opencode.json` bewusst nicht angefasst.
- **Blockade:** host-scoped egress-Allowlist braucht per-Seat-Network-Capture; opencode-MCP-Schema hat kein Integrity-Feld.
- **Braucht:** (a) getrackten Wrapper `bin/chrome-devtools-mcp` (sha512 `RnzXoJiUQ44hpOihWk90uOhLD/CnwDkDy0ldHMZONJ2nYQ+dWN1fq1luHHqyd+7FuYnyIlCY6uTThbN5ut9kSQ==`) bauen; (b) Seats' Host-Listen capturen, `--allowedUrlPattern https://*` ersetzen.

### Architektur — GitHub/CI/CDN-Optimierung
- **Status:** wartend | **Bindung:** eigen · **Teile linie:mountain/river**
- **Trigger:** grüner `ci-check`-Lauf
- **Lage:** (gemessen 2026-10-11 09:56Z) `ci-gate`-`subset` hängt weiter im Step `Run cargo test --lib` (×5, 38129998370/38129857453/38129513768/38129344392/38128784728). Nachbar-Jobs rot: register `license_census` terms-vocab · format rustfmt · dropped-gate. Keiner Mycelium-eigen.
- **Blockade:** Shard-Build-vs-Test-Anteil erst an green ci-check messbar.
- **Braucht:** (a) grüner `ci-check` → `ci_manage jobs <id>`; (b) dominiert Build: `cargo nextest archive` + Shards `--archive-file`; (c) Test-Suite-Dedup.

### matrix-rotor — Trap-Flush gebaut; Nur der persistente t420 beendet die Klasse
- **Status:** wartend | **Bindung:** eigen (Workflow) · **Teile linie:river**
- **Trigger:** nächster `matrix-rotor`-Lauf
- **Lage:** (gemessen 2026-10-11) `checkpoint` im SIGTERM-Trap (`matrix-rotor.yml:120`) flusht den jüngsten State; ein Runner-Shutdown bricht den Job dennoch ab (im Workflow nicht heilbar).
- **Blockade:** nur der persistente t420 (`bin/matrix_watchdog.sh:11-21`) ist dauerhaft.
- **Braucht:** Doc/Code-Divergenz `matrix_watchdog.sh:4` (Default `~/.local/state/omegaflow`) vs `matrix.rs:33` (`data`) schließen.

### LiDAR-Quellen-Inventar — TNM-Access gemessen; weitere Bucket-Bins offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `state/mycelium/usgs_tnm_index_measured.md`: TNM Access `api/v1/products` HTTP 200, **9 755 359** LPC-Produkte (Dataset-String `Lidar Point Cloud (LPC)`, spaces!), per-Tile `boundingBox`+`downloadURL`(LAZ, `rockyweb.usgs.gov/vdelivery`)+`publicationDate`; bbox-Query misst (124 in einem 0,1°-Fenster); `prodFormats=COPC` → **0**. Riss: `archive_search --verdict` auf die 3dep-STAC gab 502, `curl` 200 — beide Linien gehalten. Neue keyless Buckets/Indizes: `state/mycelium/lidar_sources_measured.md`.
- **Blockade:** keine.
- **Braucht:** (a) je Bucket ein `<x>_coverage.rs` (`lola_coverage.rs` gebaut); (b) TNM als **einen** US-weiten LAZ-Index verdrahten (COPC/EPT bleibt S3/PC-Azure).

### Extraterrestrische LiDAR — `lola_coverage.rs` gebaut; MOLA/MLA/NLR/OLA/Hayabusa-Parser offen
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `tools/harvest/src/bin/lola_coverage.rs` (539 Z.) walkt `astrogeo-ard` `moon/lro/lola` (S3 ListObjectsV2, paginiert) und emittiert url-only `shard <url>` für `.copc.laz`; DEFAULT_BUCKET/PREFIX aus `state/mycelium/lidar_sources_measured.md:60-61`; `cargo check`/build 0/0. MOLA/MLA/NLR/OLA/Hayabusa (SBN/DARTS), Clementine, LALT: keyless gemessen (`lidar_sources_measured.md`).
- **Blockade:** die nicht-COPC-Körper brauchen Parser (Mountain).
- **Braucht:** Parser für MOLA/MLA/NLR/OLA/Hayabusa (Mountain); LOLA-Bin in `coverage-manifest.yml` verdrahten.

### Bathymetrie — Grids keyless; Topo-Bathy-Punktwolken getrennt
- **Status:** eigen | **Bindung:** eigen · **Teile linie:river**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, keyless) Grids GEBCO/BlueTopo/ETOPO/EMODnet/GMRT/SRTM15+; Topo-Bathy-Punktwolken NOAA `noaa-nos-coastal-lidar-pds` (`noaa_nos_coverage.rs` + STAC 193 Topobathy). Ein `open-bathymetry`-Pendant existiert nicht (gemessen).
- **Blockade:** Grids sind Raster, nicht im `shard`-Kontrakt.
- **Braucht:** Raster-Konsument (`pending`); Topo-Bathy läuft über `noaa_nos_coverage` + flai.

### OpenTopography — Key eingetragen; Konsument fehlt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `OPENTOPOGRAPHY_API_KEY` in `.secrets.local` (nur Name gelesen); `archive_search` hat keinen OT-Arm.
- **Blockade:** kein Konsument; CI bräuchte den Key als Actions-Secret.
- **Braucht:** Konsument (`archive_search --opentopo <q>` oder eigenes Manifestator-Bin).

## Registrierte Korrekturen & Vorfälle

- **Adressierter Block gefaltet & veraltet:** `## An mycelium` (river-182/185) — (a) `usgs_lidar_coverage.rs` emittiert bereits `shard <url>` (url-only, `:256`), nicht mehr `url size`; (b) die Nationalportale sind in `state/mycelium/national_lidar_portals.txt` (144 Z.) enumeriert. Kein neuer Akt nötig.
- **Riss getragen (Z.ai vs. flai-Count):** „2771 vs 1407" ist kein Äquivalenzbeweis (Re-Tiling) — descope braucht den Kachel-ID-Diff (`state/stimmen/20261011T095149Z_zai_lidar-verdikt.md`).
- **Riss getragen (`--verdict` vs `curl`):** die Planetary-Computer `3dep-lidar-copc`-STAC-Kollektion antwortet auf `archive_search --verdict` mit 502, auf `curl` GET mit 200 (gemessen 2026-10-11) — beide Linien gehalten, nicht geglättet.
- **Vollständigkeit der flai-Walks:** 8 Länder (BE/EE/ES/FI/LV/PL/UK/US) am Page-Cap (100 000 Keys) abgebrochen — für den ID-Diff ggf. nachzuziehen.

## An mountain

Origin: mycelium-307 (2026-10-11).

- **Descope-Verdikt Open-LiDAR:** gemessene Basis (`state/mycelium/flai_coverage_measured.md`) + Z.ai-Bedingung (`state/stimmen/20261011T095149Z_zai_lidar-verdikt.md`). Frage: `ahn4` (NL) und `swisstopo` (CH) als Origin-Quellen behalten oder zugunsten des flai-Aggregats descopen (Fläche+Epoche gleichauf, ID-Diff jetzt schema-seitig möglich, Lauf offen)? `ign_lidarhd` (FR) nur partial. `linz` (NZ) bleibt. `ea_lidar_index` (Metadaten) — behalten oder reicht flai UK DEFRA?
- **Vier Portal-Coverage-`phi`-Blöcke** (aus folge305, offen): `url`/`origin`/`compiler`/`format`-Block je Portal aus dem `coverage-manifest`-Lauf, plus `terms`/`at`/`ttl`. Netlocs: `ns_hwh.fundaments.nl` · `data.geo.admin.ch` · `nz-elevation.s3-ap-southeast-2.amazonaws.com` · `data.geopf.fr`. (url-only via bbox-off-Default.)
- **Bayern-DOM φ-Block** (aus folge304, offen) — Mountain-Verdikt (`terms`/`at`/`ttl`): `url …/download1.bayernwolke.de/32605_5282_20_DOM.bin` · `origin …/32605_5282_20_DOM.tif` · `compiler tools/harvest/src/bin/dom_compiler.rs` · `format bdom` · `sha256 a5da30f9f2766b6cfcb4e1898f097b8e9f0685c1bd08a3feaec54bc8a0acbaa8`.
- **VNP46A3 φ-Block** (aus folge302, offen) — `…/vnp46a3_allangle_composite_snow_free.bin` · `origin …/VNP46A3.A2026213.h18v07.002.2026252141449.h5` · `compiler vnp46a3_compiler.rs` · `format black_marble_vnp46a3_nightlight` · `sha256 d85ee99b9d967b5565bb79e145d9120ab09205eb41d4188b3f016e8944b94510`.
- **open-lidar-data φ-Block** (aus folge302, offen) — `ttl`-Zeile nötig: `url …/open_lidar_data_be_dhmv2.bin` · `origin https://open-lidar-data.s3.amazonaws.com/data/BE/EODaS/LiDAR_DHMV_II-2013-2015/copc/` · `compiler open_lidar_data_compiler.rs` · `format las` · `sha256 e6560344c5e939458c890e62d5972dbfb273f8aa2cdd6e00bdb6ec2feef09c4e`.
- **Sechs Klima-Quellen** (aus folge302) — Compiler erzeugen je-Akt-Instanzen; ein Serien-Manifest fehlt. Bitte `terms`/`ttl` je Compiler gegenprüfen.
- **Asservatenkammer-Survey** `docs/surveys/survey-2026-10-08-research-api-mcp.md` — vier Marker auf den gemessenen Stand setzen (Perplexity live · Consensus 429 transient · Elicit descoped · SciSpace pending).

## An future

Origin: mycelium-302 (2026-10-10).

- **KNMI Open Data API-Key** (`state/mail/mail_ledger.φ:270`, `opendata@knmi.nl`) — **Rotation nötig:** der Key-**Wert** wurde beim Zitieren in den Modell-Transcript gelesen (Secret-Hygiene-Incident). Rotation = Operator-Hand, danach in `.secrets.local`.

Origin: mycelium-306 (2026-10-11).

- **t420 Runner-Throttle-Neuinstallation** — `bin/runner_throttle.sh` `cmd_adaptive` setzt die IFB/tc-Klassen beim Start neu; die Skript-Neuinstallation (`install`/`set` als root) auf t420 und die per-Gerät-QoS am Router bleiben Operator-Hand.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
