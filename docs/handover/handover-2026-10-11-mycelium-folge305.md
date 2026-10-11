<!--
  title: Handover — Mycelium-Folge 305 (2026-10-11)
  session: Mycelium-Linie — Meta-Pass. Taucher-Batterie(n): cmb-cdn-Streaming vollständig (tar_stream/bzip2_tar_stream/fetch_raw_to_file_with + beide Compiler-Arme); vier Portal-Coverage-Manifestatoren + coverage-manifest-Workflow; MCP-no-leak-Gate (src/gate/mcp.rs) in commit_check; t420-Throttle-Boot-Lücke + matrix-rotor-Trap-Flush; keyless-OGC-EA-Index (ea_lidar_index.rs). **Korrektur: UK EA ist NICHT account-gated** — keyless via flai `open-lidar-data` S3 + EA `/tiles` (gemessen). Das flai-Aggregat ist bereits getrackt (`open_lidar_data_coverage.rs`, Welt). Extraterrestrische + Bathymetrie-LiDAR-Landschaft gemessen. OpenTopography-Key eingetragen.
  class: handover
  date: 2026-10-11
  sha256: 5ecf3f27fa6e113151b82a4478ca5db89c482ae31a6405995ea77d9ded2a0a9a
  status: live
-->
# Handover — Mycelium-Folge 305 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-11-mycelium-folge304.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.50 · cap 0.5 — Grund: Taucher-Batterie (1× line + 10 Dispatches: 5 Portal-Bins, `--url`-Arm, MCP-Gate, coverage-Workflow, t420/matrix-rotor). Der Gesamt-Burn ($2.4242/37 Sessions) ist von parallelen Linien-Sessions verunreinigt; die Zahl ist die Summe der eigenen top-session-Einträge (UK EA $0.1256 · NZ LINZ $0.1010 · Mycelium-line $0.0990 · +7 kleinere) und liegt an der Hard-Grenze (`BURN_CAP_HARD_USD`) — deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-11 | Operator (Session, Mycelium 305) |
| „bitte fang mal an zu bauen statt nur zu messen — setze eine Batterie von Tauchern auf die Aufgaben an und ERLEDIGT sie" | 2026-10-11 | Operator (Session, Mycelium 305) |
| „vorbestehend ist verboten" (kein Vertrösten auf vorhandenen Schmutz) | 2026-10-11 | Operator (Session, Mycelium 305) |
| „öffne die seite(n) zur registrierung ausgefüllt bis zur Kante" | 2026-10-11 | Operator (Session, Mycelium 305) |
| „key ist eingetragen" (OpenTopography) | 2026-10-11 | Operator (Session, Mycelium 305) |
| „habt ihr denn auch die auth quellen gegen secrets local geprüft? (nicht ausgeben!)" | 2026-10-11 | Operator (Session, Mycelium 305) |
| „was hat es mit flai open-lidar-data auf sich? ist das nicht das Aggregat für globale Abdeckung?" | 2026-10-11 | Operator (Session, Mycelium 305) |
| „sind da auch die bathymetry lidar daten drin oder gibt es ein pendant open-bathymetry?" | 2026-10-11 | Operator (Session, Mycelium 305) |
| „taucher mit harten bandagen auf extraterrestrisches lidar — für welche ephemeridenkörper gibt es lidar?" | 2026-10-11 | Operator (Session, Mycelium 305) |
| „ich habe den Eindruck du kehrst es gerade unter den Teppich" — Befunde gehören ins Register, nicht in den Chat | 2026-10-11 | Operator (Session, Mycelium 305) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-11-mycelium-folge304.md` §Operator-Wort-Register (via `git show <sha>:archiv/…`) | 2026-10-11 | gefaltet, nicht kopiert |

## Offen — eigen

### Portal-Coverage-Manifestatoren — 4 gebaut + Workflow; Überlappung mit dem flai-Aggregat
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) vier neue Bins, je `shard <absolute-url>`, `cargo build` grün + Einmal-Lauf gezogen:
  - `tools/harvest/src/bin/ahn4_coverage.rs` — ArcGIS FeatureServer `services.arcgis.com/nSZVuSZjHpEZZbRo/.../Kaartbladen_AHN4/FeatureServer/0`, Feld `AHN4_LAZ`, **1407** `.LAZ`; netloc `ns_hwh.fundaments.nl`; Lauf: `1 pages, 1407 tiles`.
  - `tools/harvest/src/bin/swisstopo_coverage.rs` — STAC `data.geo.admin.ch/api/stac/v0.9/collections/ch.swisstopo.swisssurface3d/items`, Assets `.las.zip`; netloc `data.geo.admin.ch`; bbox-Lauf `70 assets`.
  - `tools/harvest/src/bin/linz_coverage.rs` — S3 `nz-elevation` (`s3-ap-southeast-2`), COG `.tiff` (die LAZ-Kante ist key-gated, s. Blockade); **30835** tiles; netloc `nz-elevation.s3-ap-southeast-2.amazonaws.com`.
  - `tools/harvest/src/bin/ign_lidarhd_coverage.rs` — WFS `data.geopf.fr` (`IGNF_LIDAR-HD_METADONNEE:metadata`, Property `url_npl` → `.copc.laz`, **524687** tiles); netloc `data.geopf.fr`.
  - Workflow `.github/workflows/coverage-manifest.yml` (Matrix, `--ci-mode`, `[self-hosted, Linux]`).
- **Blockade:** keine. **Korrektur (gemessen 2026-10-11):** UK EA ist **nicht** account-gated — keyless Tiles via flai `open-lidar-data` S3 (`data/UK/DEFRA/LIDAR_<year>/copc/*.copc.laz`, 200, 2 048 828 B) und EA `/tiles/collections/survey/lidar_point_cloud/<year>/NaN/<tile>` (200, `application/zip` mit 25 `.laz`), Scottish SRS 206; `geoservices/data-requests` bleibt JWT-only, ist aber nicht der einzige Weg. `tools/harvest/src/bin/ea_lidar_index.rs` (OGC-Index-Metadatenkatalog, `numberMatched` **244324**) gebaut.
- **Braucht:** (a) die 4 Download-Manifeste + den EA-Katalog manifestieren + in `phi/sources.φ` registrieren (Mycelium `url`/`origin`/`compiler`, Mountain `terms`/`at`/`ttl`) und `coverage-manifest.yml` dispatchen; (b) **Konsolidierungs-Entscheidung** (s. `Open-LiDAR-Konsolidierung`): `ahn4`/`swisstopo`/`ign` überlappen das flai-Aggregat, `linz` bleibt eigenständig.

### Manifestation SPT-3G D1 `cmap` (cmb-cdn) — Streaming vollständig gebaut; Dispatch + sha256 offen
- **Status:** eigen | **Bindung:** eigen (Manifestation)
- **Trigger:** eigener Commit+Push → `gh workflow run cmb-cdn.yml`
- **Lage:** (gemessen 2026-10-11) die RAM-Kette (7,87 GB komprimiert + entpackter Tar + FITS) ist gebrochen: `TarScan` puffert keine Mitglieder mehr, `pub fn tar_stream` (`src/archivar/inflate.rs`) liest einen entpackten Tar in 64 KiB, `pub fn bzip2_tar_stream` (`src/archivar/bzip2.rs`) verkettet `Bz2Decoder`+`tar_stream`, `pub fn fetch_raw_to_file_with` (`src/archivar/fetch.rs`) lädt curl direkt auf Platte. Der Compiler `--tar`-Arm streamt den FITS-Member auf eine Temp-Datei (`stream_fits_member_to_file`), der `--url`-Arm lädt auf Temp-Tar + streamt gleich. `cargo check` (0/0) + `cargo build -p omegaflow-harvest --bin cmb_planck_compiler` grün; Fixture-Lauf: `tar member 'map.fits' streamed to /tmp/cmb_spt_d1_<pid>.fits`. `gunzip_tar_members`-Kontrakt unverändert (extract.rs, eht_uvfits_compiler.rs unberührt).
- **Blockade:** keine.
- **Braucht:** `gh workflow run cmb-cdn.yml`; die Manifestation schließt mit `sha256` im `phi/sources.φ:11559`-Block.

### MCP no-leak-Regel — Regel gebaut (Gate + Test); der Knoten (config) bleibt bewusst ungebunden
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `src/gate/mcp.rs` mit `pub fn mcp_leak_violations` (6 Regeln: `mcp-launch-unpinned` · `mcp-egress-unbounded` · `mcp-fs-unrestricted` · `mcp-yargs-conflict` · `mcp-executable-not-absolute` · `mcp-cwd-missing`) + Test (die aktuelle config → `{unpinned ×2, egress-unbounded ×1, cwd-missing ×2}`; konforme config → leer), exportiert via `src/lib.rs`, verdrahtet in `tools/gate/src/bin/commit_check.rs` (staged `opencode.json` → fail). `cargo check` + `cargo build -p omegaflow-gate --bin commit_check` grün. `opencode.json` ist **nicht** angefasst (Regel vor Knoten).
- **Blockade:** die host-scoped egress-Allowlist ist `pending` (braucht den per-Seat-Network-Capture — `--allowedUrlPattern` blockt auch Subressourcen); opencodes MCP-Schema hat **kein** Integrity-Feld, die gemessene Alternative ist ein getrackter Wrapper `bin/chrome-devtools-mcp` (sha512 `RnzXoJiUQ44hpOihWk90uOhLD/CnwDkDy0ldHMZONJ2nYQ+dWN1fq1luHHqyd+7FuYnyIlCY6uTThbN5ut9kSQ==`).
- **Braucht:** (a) Wrapper bauen + config flippen (erst wenn (b) steht); (b) die Seats' Host-Listen capturen und `--allowedUrlPattern https://*` ersetzen.

### Architektur — GitHub/CI/CDN-Optimierung (Rat-Verdikt: Messung, dann Archive)
- **Status:** wartend | **Bindung:** eigen · Teile linie:mountain/river
- **Trigger:** grüner `ci-check`-Lauf auf einem green tree
- **Lage:** (gemessen 2026-10-11 via `ci_manage`) die `ci-gate`-`subset`-Jobs hängen im Step `Run cargo test --lib`; die Nachbar-Jobs derselben Läufe sind rot: **register** `license_census` terms-vocab ×3 (`11170` `non-commercial`, `11711` `Gratis-Open-Data-Licentie-Vlaanderen-1.2`, `31412` `non-commercial`); **format** rustfmt-Diff in `tools/utils/src/bin/archive_search.rs`; **dropped-gate** 5 neue Drops. Keiner ist Mycelium-eigen (Register/Format/fremde Dateien).
- **Blockade:** der Shard-Build-vs-Test-Anteil ist erst an einem green ci-check messbar.
- **Braucht:** (a) grünen `ci-check` abwarten, dann `ci_manage jobs <id>`; (b) dominiert der Build: `cargo nextest archive` + Shards per `--archive-file`; (c) Test-Suite-Dedup als zweite Achse.

### Zweite CI-Lane — self-hosted t420: Throttle-Boot-Lücke gebaut
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `bin/runner_throttle.sh` implementiert bereits CPU/IO (`Nice=10`/`CPUWeight=20`/`IOWeight=20`/`CPUQuota=150%`), den adaptiven Ingress-Cap (`MIN_KBIT`/`MAX_KBIT`/`HIGH_MS`/`LOW_MS`) und `pause`/`resume`. Gemessene Lücke: die IFB/tc-Klassen sind Kernel-Runtime-State, `cmd_adaptive` setzte sie nach Reboot nicht neu → der Runner lief **ungedrosselt und still**. **Gebaut:** `cmd_adaptive` ruft jetzt `apply_cap "$MAX_KBIT"` beim Start.
- **Blockade:** keine (im Repo).
- **Braucht:** Tuning `install`/`set` als root auf t420 und die **Neu-Installation** des aktualisierten Skripts = Operator-Hand; per-Gerät-QoS = Router-Config (Operator).

### CI-Hygiene — `matrix-rotor` GitHub-Präemption: Trap-Flush gebaut; nur der persistente t420 beendet die Klasse
- **Status:** wartend | **Bindung:** eigen (Workflow, ggf. linie:river)
- **Trigger:** nächster `matrix-rotor`-Lauf
- **Lage:** (gemessen 2026-10-11, `ci_manage log 38118057124`) `rotor slice ended rc=137`, `The runner has received a shutdown signal`. Der Resume-Pfad ist gebaut (`OMEGAFLOW_STATE=data` + Workflow-Download `matrix-rotor.yml:51-88`; `matrix.rs:257-267`). **Gebaut:** `checkpoint` im SIGTERM-Trap (`matrix-rotor.yml:120`) flusht das jüngste State.
- **Blockade:** ein Runner-Shutdown bricht den Job ab (auch `always()`) — im Workflow nicht heilbar.
- **Braucht:** der einzige dauerhafte Modus ist der persistente t420 (`bin/matrix_watchdog.sh:11-21`); offen: Doc/Code-Divergenz `matrix_watchdog.sh:4` (Default `~/.local/state/omegaflow`) vs `matrix.rs:33` (`data`).

### Open-LiDAR-Konsolidierung — Welt-Aggregat schon getrackt; Überlappung + Klassen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `open-lidar-data` (eu-central-1) ist das **globale Aggregat** (18 Länder BE·CH·DE·DK·EE·ES·FI·FR·IE·LT·LU·LV·NL·PL·SE·SI·UK·US; COPC; OGL/CC). **Getrackt:** `open_lidar_data_coverage.rs` manifestiert den **ganzen** Bucket (`--prefix data/`, `shard <url> <6 bbox>` via COPC-Header-Range-Fetch) — der Welt-Manifestator **existiert schon**; dazu `noaa_nos_coverage.rs` (NOAA Küste/Topo-Bathy), `usgs_lidar_coverage.rs` (USGS 3DEP), `open_lidar_data_compiler.rs` (Belgien-EODaS). **Überlappung:** flai trägt NL AHN4, CH swisssurface3d, FR IGN, UK DEFRA → `ahn4_coverage`/`swisstopo_coverage`/`ign_lidarhd_coverage` **duplizieren** es (ohne bbox); **NZ fehlt in flai** → `linz_coverage` bleibt eigenständig. `ea_lidar_index.rs` ist ein Metadaten-Index (kein Tile-Manifest).
- **Blockade:** keine.
- **Braucht:** Mountain-Verdikt — `ahn4`/`swisstopo`/`ign` als Origin-Quellen behalten oder `descoped` zugunsten des bbox-tragenden flai-Aggregats? Brauchen wir `ea_lidar_index` oder reicht flai UK DEFRA?

### LiDAR-Quellen-Inventar — FMHY + Sammelseiten + neue keyless Buckets (gemessen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11 via 3 Taucher) **FMHY trägt kein LiDAR** (word-boundary `\ylidar\y` = 0; nur Geo-Viewer/Atlas in `docs/misc.md #Maps` + `docs/educational.md`; Sammel-Querverweise `simsieg/awesome-maps` · `awesomedata/awesome-public-datasets` · `osmlab/awesome-openstreetmap` · `jnv/lists`). Von den Sammelseiten nennt nur `freegisdata.rtwilson.com` Punktwolken (OpenTopography · 3DEP · Puget Sound · NCEI). **Neue keyless Buckets** (end-to-end verifiziert): `japan-pointcloud` + `kanagawa-pointcloud` (JP, CC-BY 4.0) · `canelevation-lidar-point-clouds` (CA, COPC, OGL-CA) · `srsp-open-data` (Schottland, OGL v3) · `giselevationingov` (Indiana, COPC, CC0) · `njogis-elevation` (NJ) · `dc-lidar-2015/-2018` · `colorado-public-elevation-data` (CC0) · `ept-m3dc-pmsp` + `laz-m3dc-pmsp` (São Paulo, EPT/LAZ, erster SAM-Aggregat) · `nz-coastal` (COG). **Neue keyless Indizes:** USGS TNM Access API (`tnmaccess.nationalmap.gov`, **9 755 359** LiDAR-Produkte) · OpenTopography-Catalog-API (`otCatalog`, ~500). Vollinventar: `state/mycelium/lidar_sources_measured.md`. Riss aufgelöst: `astrogeo-ard` **trägt** LOLA-COPC (`moon/lro/lola/*.copc.laz`, 206, 151 418 128 B) — der negative Kollektor-Fund war ein Teil-Listing.
- **Blockade:** keine.
- **Braucht:** je neuer Bucket ein `<x>_coverage.rs` (`shard <url>`); Reihenfolge/Umfang offen (Operator/Mountain); USGS-TNM-Access als **ein** US-weiter Index prüfen statt Einzelbuckets.

### Extraterrestrische LiDAR — keyless Bestand gemessen; LOLA passt in den shard-Kontrakt
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, keyless) **Mond:** LOLA — PDS `imbrium.mit.edu` (`.img`/`.tab`/PDS4) **und** COPC auf USGS `astrogeo-ard`/`asc-moon` S3 (`shard`-fähig, 151/201 MB Kacheln); LALT (Kaguya) `data.darts.isas.jaxa.jp/pub/pds3/…lalt…`; Clementine LIDAR PDS. Gated: Chandrayaan-1 LLRI (ISSDC), Chang'e LAM (BAO). **Mars:** MOLA PDS `pds-geosciences.wustl.edu/mgs/…` (`.img`/`.tab`). **Merkur:** MLA (MESSENGER) PDS (`.tab`/`.img`). **Kleine Körper:** Eros NLR · Bennu OLA · Ryugu Hayabusa2-LIDAR · Itokawa Hayabusa-LIDAR (SBN/DARTS PDS3/4; Hayabusa haus-registriert `phi/sources.φ:19529`). **Künftig:** BepiColombo BELA (PSA partial), JUICE GALA (nur SPICE), Hera PALT, MMX. Nur Mond/Mars/Merkur unter den Planeten führen Laser-Altimeter.
- **Blockade:** keine.
- **Braucht:** `lola_coverage.rs` (astrogeo-ard COPC → `shard`+bbox) als nächster Bau; MOLA/MLA/NLR/OLA/Hayabusa brauchen **Parser** (Mountain).

### Bathymetrie — Grids keyless; Topo-Bathy-Punktwolken getrennt
- **Status:** eigen | **Bindung:** eigen · **Teile linie:river**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, keyless) **Grids:** GEBCO 2024 (CEDA, GeoTIFF/NetCDF, ~4 GB), NOAA BlueTopo S3, ETOPO 2022, EMODnet WCS, GMRT `GridServer`, SRTM15+. **Topo-Bathy-Punktwolken:** NOAA `noaa-nos-coastal-lidar-pds` S3 COPC (+ STAC 193 Topobathy-Titel) — `shard`-fähig, `noaa_nos_coverage.rs` getrackt; flai NOAA `NGS_Topobathy_*` (3). Ein `open-bathymetry`-Pendant zum flai-Bucket **existiert nicht** (gemessen).
- **Blockade:** Grids sind Raster, keine Punktwolken → nicht im `shard`-Kontrakt.
- **Braucht:** Raster-Konsument für die Grids (`pending`); Topo-Bathy-Punktwolken laufen über `noaa_nos_coverage` + flai.

### OpenTopography — Key eingetragen; Konsument fehlt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11 via `bin/secrets_keys` + `archive_search --help`) **`OPENTOPOGRAPHY_API_KEY`** in `.secrets.local` (Operator-Wort „key ist eingetragen"; nur der Name gelesen). `archive_search` hat **keinen** OT-Arm (kein `--opentopo*`); Quelle nur als Lead `tools/utils/data/library.φ:2804`. Registrierung `portal.opentopography.org/newUser` vorbereitet bis zur Kante (Johannes Tyroller · `code@omegaflow.space` · Affiliation `Other` · Org `omegaflow`), nicht abgeschickt.
- **Blockade:** kein Konsument; ein CI-Lauf bräuchte den Key als GitHub-Actions-Secret.
- **Braucht:** Konsument bauen — `archive_search --opentopo <q>` (Katalog keyless) ODER ein eigenständiges Manifestator-Bin (Default).

## Registrierte Korrekturen & Vorfälle

- **Korrektur (Rücknahme):** „UK EA blockiert account" ist **falsifiziert** (gemessen 2026-10-11). Keyless Tiles: flai `open-lidar-data` (`data/UK/DEFRA/LIDAR_<year>/copc/*.copc.laz`, 200), EA `/tiles/collections/survey/lidar_point_cloud/<year>/NaN/<tile>` (200, ZIP mit `.laz`), Scottish SRS (206). `geoservices/data-requests` bleibt JWT-only, ist aber **nicht** der einzige Weg. Lehre (Operator-Prüfregel): kein Quellen-Verdikt ohne gemessenen **401 mit dem vorhandenen Token**.
- **Vorfall — Fremd-Werk überschrieben:** ein `grind-flash`-Taucher überschrieb die **getrackte** `tools/harvest/src/bin/open_lidar_data_coverage.rs` (201→361 Z.). **Restore** aus `git_safety refs/safety/1791704979`; Diff leer, Status clean. Lehre: jeder Taucher-Auftrag trägt „create NEW file; if the path exists, STOP".
- **Vorfall — Duplikate:** in diesem Atom `open_lidar_data_world_coverage.rs` gebaut (= Duplikat des getrackten Aggregat-Bins) → **gelöscht**; `ea_lidar_coverage.rs` (UK-eng) gebaut → **gelöscht** (superseded).
- **Stale-Gate-Blocker (geheilt):** `tools-build 38123001050` war rot an `tools/utils/src/bin/volume_builder.rs:346` (Fremd-Hunk, `SourceConfig.band/footprint`) → `tools-latest` stale → installiertes `commit_check` blockte `declared-not-tracked: phi/footprints.φ`+`harvest.φ`. **Mountain 311 (`6f47cf0c9`) hat die Release geheilt.**

## An mountain

Origin: mycelium-304 (2026-10-11).

- **Bayern-DOM `32605_5282_20_DOM.bin`** (dom-cdn `38117274707`, success) — Asset am CDN
  present (`--verdict` 206). Mycelium-Direktive + Mountain-Verdikt (`terms`/`at`/`ttl`)
  in **einem** Block:
  ```
  url https://github.com/omegaflow/sources/releases/download/download1.bayernwolke.de/32605_5282_20_DOM.bin
  origin https://download1.bayernwolke.de/a/dom20/DOM/32605_5282_20_DOM.tif
  compiler tools/harvest/src/bin/dom_compiler.rs
  format bdom
  sha256 a5da30f9f2766b6cfcb4e1898f097b8e9f0685c1bd08a3feaec54bc8a0acbaa8
  ```
  (sha256 + 100 000 048 B aus dem GitHub-Release-Asset-Digest, gemessen 2026-10-11.)
- **ECAD/WorldClim `sha256` auf die materialisierten Bytes gesetzt** (Materialisation,
  Mycelium — kein Verdikt-Akt, die Zeile nominiert die Bytes): `phi/sources.φ:11175`
  ecad `72080c01…` -> `198aea26…` (ecad-cdn `38117743707` success, Release-Digest
  9 389 128 B); `phi/sources.φ:31408` worldclim `20ffec98…` -> `25b428d1…`
  (Release-Digest 2 016 900 296 B).
- **te-ground-truth — Artefakt gelesen (Riss, getragen).** Lauf `te-bias-n 38038722712`
  (head `ea4025b28`) success; Artefakt `te-bias-n` (2755 B) gelesen. `POINT te-ground-truth`:
  der **Produktionsarm** (Takens-embedded KSG, dim 3, `omega.rs:489`) **NOT PASS** — bei c=0.20
  korrekt (TE(X->Y) 2.6483e-1 arrow, TE(Y->X) -1.2535e-2 still), aber das **Null-Control c=0**
  zeigt in **beiden** Richtungen einen Pfeil (TE(X->Y) -1.1298e-2, TE(Y->X) -6.5337e-3,
  beide > fam -8.5123e-3) → das Null-Control wird nicht rekonstruiert. Der **skalare
  Silverman-KDE-Referenzarm** (`docs/paper/gic-causal-driver.md:655-660`) nullt den Rückkanal
  bei starker Kopplung **nicht** (TE(Y->X) 3.6416e-2 > thr 2.0105e-2) — ein Riss gegen den
  Produktionsarm, getragen, nicht geglättet. Trigger des GIC-Papers (Mountain-folge307:128)
  ist gefeuert: §3.5/Abstract/§7 nachziehen.
- **VNP46A3 φ-Block** (aus folge302, offen) — Mycelium-Direktive + Mountain-Verdikt
  (`terms`/`at`/`ttl`) in **einem** Block:
  ```
  url https://github.com/omegaflow/sources/releases/download/data.laadsdaac.earthdatacloud.nasa.gov/vnp46a3_allangle_composite_snow_free.bin
  origin https://data.laadsdaac.earthdatacloud.nasa.gov/prod-lads/VNP46A3/VNP46A3.A2026213.h18v07.002.2026252141449.h5
  compiler tools/harvest/src/bin/vnp46a3_compiler.rs
  format black_marble_vnp46a3_nightlight
  sha256 d85ee99b9d967b5565bb79e145d9120ab09205eb41d4188b3f016e8944b94510
  ```
- **open-lidar-data φ-Block** (aus folge302, offen) — `register_sort` verlangt eine
  `ttl`-Zeile, daher Mycelium-Direktive + Mountain-Verdikt (`terms`/`at`/`ttl`) in
  **einem** Block:
  ```
  url https://github.com/omegaflow/sources/releases/download/open-lidar-data.s3.amazonaws.com/open_lidar_data_be_dhmv2.bin
  origin https://open-lidar-data.s3.amazonaws.com/data/BE/EODaS/LiDAR_DHMV_II-2013-2015/copc/
  compiler tools/harvest/src/bin/open_lidar_data_compiler.rs
  format las
  sha256 e6560344c5e939458c890e62d5972dbfb273f8aa2cdd6e00bdb6ec2feef09c4e
  ```
- **Vier Portal-Coverage-`phi`-Blöcke** (neu, Mycelium-Direktive + Mountain-Verdikt in **einem** Block): je Portal der `url`/`origin`/`compiler`/`format`-Block aus dem `coverage-manifest`-Lauf, plus `terms`/`at`/`ttl`. Netlocs: `ns_hwh.fundaments.nl` · `data.geo.admin.ch` · `nz-elevation.s3-ap-southeast-2.amazonaws.com` · `data.geopf.fr`.
- **Sechs Klima-Quellen registriert** (aus folge302) — die Compiler erzeugen
  **je-Akt-Instanzen** (Station/Tag/Variable), die Blöcke sind Workflow-Default-
  Repräsentanten; ein Serien-Manifest fehlt. Bitte `terms`/`ttl` je Compiler gegenprüfen.
- **Asservatenkammer-Survey** `docs/surveys/survey-2026-10-08-research-api-mcp.md` —
  die vier Marker auf den gemessenen Stand setzen (Perplexity live · Consensus 429
  transient · Elicit descoped · SciSpace pending).

## An future

Origin: mycelium-302 (2026-10-10).

- **KNMI Open Data API-Key** (`state/mail/mail_ledger.φ:270`, `opendata@knmi.nl`) —
  **Rotation nötig:** beim Zitieren der Ledger-Zeile wurde der Key-**Wert** in den
  Modell-Transcript gelesen (Secret-Hygiene-Incident). Der Wert gilt als exponiert;
  Rotation = Operator-Hand, danach Hinterlegung in `.secrets.local`.
- **UK EA LiDAR — korrigiert:** nicht account-gated (keyless via flai `open-lidar-data` S3 und EA `/tiles/collections/survey/...`, s. `## Registrierte Korrekturen`); kein Operator-Akt nötig.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
