<!--
  title: Handover — Mycelium-Folge 303 (2026-10-11)
  session: Mycelium-Linie — Meta-Pass. Roten harvest-dispatch gemessen und geheilt (phi/harvest.φ lro_trk/losc-Reihenfolge), Coverage-Manifestator bbox-Emission gebaut (LAS-Header-Range-GET -> shard <url> <6 f64>), te-ground-truth-Artefakt gelesen (Produktionsarm NOT PASS, Riss gegen den Skalararm), sieben CDN-Manifestationen dispatcht.
  class: handover
  date: 2026-10-11
  sha256: 9be324c5fe54c208a406a20a54e17009e9e6042bfca14efc39e3cc5a9a2d8fcb
  status: live
-->
# Handover — Mycelium-Folge 303 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge302.md` (→ `archiv/`).

**Faltung `## An mycelium`** (mountain-303): der Block aus folge302 ist gefaltet
(SURFRAD/ECAD/DWD/WorldClim/AODN-Compiler + Asservatenkammer-Träger, siehe
`## Offen — eigen` und `## An mountain`); kein erneuter Falt-Akt.

## Burn: open 0.0000 · close 0.0798 · cap 0.5 — Grund: dieses Atom 1× line ($0.0798) + 2× grind-flash (bbox open_lidar + noaa) + 1× general (ECAD-Koordinaten) · deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-11 | Operator (Session, Mycelium 303) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge302.md` §Operator-Wort-Register (via `git show <sha>:archiv/…`) | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation SPT-3G D1 `cmap` (cmb-cdn) — Relauf rot (Runner-Preemption), neu dispatcht
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Lauf `cmb-cdn` Abschluss am neuen HEAD
- **Lage:** (gemessen 2026-10-11 via `ci_manage log 38080553796`) der Relauf am head
  `23fe42a1a` schlug fehl: exit 137 — der Release-Build lief 6 m 31 s, dann wurde
  `cmb_planck_compiler --tar …/full_maps_d1.tar.bz2` 13 s nach Start „Getötet", der
  Runner erhielt ein shutdown-Signal (Preemption). Neu dispatcht am HEAD `a49a11860`:
  `38117250487`.
- **Blockade:** Self-hosted Runner-Preemption.
- **Braucht:** nach Abschluss `ci_manage log 38117250487` -> bei success sha256 in den
  SPT-`cmap`-Block (`cmb_spt_d1_n64.json`).

### Coverage-Manifest — bbox-Emission gebaut; River-Wiring + nationale Portale offen
- **Status:** eigen | **Bindung:** eigen (Core-Archivar) · Teile linie:river
- **Trigger:** Faltung des `## An river`-Blocks (Fetch-Loop `main_flow.rs`)
- **Lage:** (gemessen 2026-10-11) **zwei** der drei Manifestatoren emittieren jetzt
  `shard <url> <6 f64>` statt blanker URLs: `open_lidar_data_coverage.rs` und
  `noaa_nos_coverage.rs` (je `parse_las_bbox` — Magic `LASF`, f64 LE bei
  179/187/195/203/211/219, alle sechs finit -> bbox minx miny minz maxx maxy maxz —
  + `copc_bbox`, Range-GET `bytes=0-374` via `fetch_raw_bytes_headers`); `shard <url>`
  ohne Doubles -> `Pending` (kein fabriziertes 0). `cargo check` sauber; Roundtrip
  durch `parse_coverage_manifest` bestätigt. **`usgs_lidar_coverage.rs`** emittiert
  weiter `url size`-Zeilen (mischt COPC mit `ept.json`/`boundary.json`) — eigener
  Manifest-Schnitt nötig, nicht derselbe Fall.
- **Blockade:** (b) der Fetch-Loop liegt in `main_flow.rs` = River-Membran (fremde
  Domäne, kein Mycelium-Schreibakt) — `## An river`. `state/mycelium/national_lidar_portals.txt`
  ist **absent** (0 Treffer, `archive_search --root state`) — die 104-Portal-Tabelle
  lebt in `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md`.
- **Braucht:** (a) `usgs_lidar_coverage.rs`-Manifest auf `shard`-Form entscheiden;
  (b) `## An river` — Fetch-Loop über `format coverage_manifest`; danach die 104
  Nationalportale aus der Survey enumerieren.

### Bayern-DOM (`geodaten.bayern.de`) — Lauf dispatcht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `dom-cdn` Abschluss
- **Lage:** (gemessen 2026-10-11) `dom-cdn.yml` verlangt `url`+`asset` (keine
  Defaults); Asset-Name = Compiler-Default `leaf_stem(url).bin` =
  `32605_5282_20_DOM.bin` (`dom_compiler.rs:590-594`). Asset am CDN **absent**
  (`--verdict` 404). Dispatcht: `38117274707` (url `…/32605_5282_20_DOM.tif`,
  asset `32605_5282_20_DOM.bin`). Der Block existiert noch nicht in `phi/sources.φ`.
- **Blockade:** kein CDN-Asset bis zum Lauf.
- **Braucht:** nach success sha256 + φ-Block (Mountain-Verdikt, `## An mountain`).

### φ-Manifestation — SURFRAD · DWD CDC · WorldClim · AODN · ECAD · KNMI
- **Status:** wartend | **Bindung:** eigen (Manifestation) · ECAD Teil linie:mountain (Station)
- **Trigger:** je Workflow-Lauf-Abschluss
- **Lage:** (gemessen 2026-10-11) Blöcke stehen in `phi/sources.φ`. Dispatcht:
  surfrad `38117149584`, dwd-cdc `38117152122`, worldclim `38117153770`, aodn
  `38117301397` (var=TEMP, url=origin, start=0, count=64). **ECAD**: Asset
  **absent** (`--verdict` 404 auf `ecad_eca_tx_TX_STAID000001.bin`); STAID 1 =
  VAEXJOE/SE, lat 56.866667, lon 14.800000, alt 166 m (gemessen 2026-10-11 via
  `ECA_blend_station_tx.txt`, `knmi-ecad-assets-prd.s3.amazonaws.com/download/`),
  dispatcht `38117743707`. **KNMI**: wartet auf das Repo-Secret `KNMI_API_KEY`.
- **Blockade:** knmi auf Operator-Hand.
- **Braucht:** (a) `archive_search --verdict` je Asset nach Lauf; (b) knmi erst nach
  dem Secret.

### Planetary Radar — Manifestation dispatcht
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Lauf `sb-radar-cdn` Abschluss
- **Lage:** (gemessen 2026-10-11) `--emit delay` gebaut; Workflow dispatcht
  `38117147970`. `ssd-api.jpl.nasa.gov/sb_radar.bin` liegt am CDN (HTTP 206); der
  Delay-Emit schreibt ein eigenes Asset (sha256 `442cf…`, 574 296 B laut folge302).
- **Blockade:** kein Delay-Asset bis zum Lauf.
- **Braucht:** nach success `ci_manage view 38117147970` -> Asset/sha prüfen.

### Architektur — GitHub/CI/CDN-Optimierung (Rat-Verdikt: Messung, dann Archive)
- **Status:** wartend | **Bindung:** eigen · Teile linie:mountain/river
- **Trigger:** grüner `ci-check`-Lauf auf einem green tree
- **Lage:** (gemessen 2026-10-10 via `ci_manage jobs 38048598777`) die drei Shard-Läufe
  (2187-2189) brachen im **ersten** `cargo nextest run --release`-Step beim Kompilieren
  ab (`-D warnings` auf rotem Baum) -> **0 s Test-Zeit**; Build-vs-Test **ungemessen**
  bis grün. Strukturell: kein separater `cargo build`-Step; `rust-cache`-Key
  `v0-rust-test-Linux-arm64-…` trägt **keine** Shard-Nummer -> alle vier Shards teilen
  einen Cache; Runner-Overhead 16-34 s/Shard.
- **Blockade:** der Shard-Build-vs-Test-Anteil ist erst an einem green ci-check messbar.
- **Braucht:** (a) grünen `ci-check` abwarten, dann `ci_manage jobs <id>`; (b) dominiert
  der Build: **ein** Dispatch `cargo nextest archive` + vier Shards per `--archive-file`;
  (c) Test-Suite-Dedup als **zweite, getrennt eingeführte** Achse.

### Speicher — 1,76 TB Bulk vs. R2-10-GB (Rat-Verdikt: messen vor bauen)
- **Status:** pending | **Bindung:** eigen
- **Trigger:** gemessener R2-Konsument ODER größte Datei nähert sich 2 GiB ODER Releases-Lesezugriffe werden zum Engpass
- **Lage:** (gemessen 2026-10-10 via GitHub-API) `omegaflow/sources` = 344 Releases /
  1763 GB -> $0; R2 10 GB frei, Zero-Egress. **Kein gemessener Live-Konsument.**
- **Blockade:** keine — das Rat-Verdikt (2026-10-10, UI-bestätigt): kein Tier ohne Hyphe.
- **Braucht:** **kein Bau.** Den **einen** manifestierenden Konsumenten messen; kein
  Konsument -> `descoped` mit Befund. R2-Bucket/Key = Operator-Akt (per-Akt-Wort).

### MCP — lokale no-leak-Server (Rat-Verdikt: Regel vor Knoten)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** gemessener Session-Bedarf (`sread`/`sgrep` reichen nicht)
- **Lage:** (gemessen 2026-10-10) `opencode.json:439-451` läuft zwei lokale stdio-MCPs
  (`chrome-devtools`) mit `--autoConnect`/`--allowedUrlPattern https://*`/`npx -y` —
  **kein no-leak-Präzedenzfall**.
- **Blockade:** keine.
- **Braucht:** **die Regel** — `mcp`-`local` nur für die no-leak-Klasse: exakter
  Versions-**Integrity-Pin**, `command`/`args` nur aus der getrackten Config,
  Repo-Wurzel-Scope, **erzwungener** Egress-Deny + FS-Scope mit Testfall.

### Zweite CI-Lane — self-hosted t420
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Bedarf (regelt sich selbst)
- **Lage:** (gemessen 2026-10-10) 20 Workflows `runs-on: [self-hosted, Linux]`;
  Throttle aktiv; Konzept `docs/concepts/self-hosted-runner.md`. Die zwei roten
  `harvest-dispatch`-Läufe (38105695169/38105544121) und der cmb-cdn-Abbruch
  (exit 137) sind Self-hosted-Signale.
- **Blockade:** keine.
- **Braucht:** tunen via `/etc/default/runner-throttle`; `pending`: per-Gerät-QoS.

### CI-Hygiene — `matrix-rotor` GitHub-Präemption
- **Status:** wartend | **Bindung:** eigen (Workflow, ggf. linie:river)
- **Trigger:** nächster `matrix-rotor`-Lauf
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38054215238`) rot: „The runner
  has received a shutdown signal" -> `rotor slice ended rc=137`.
- **Blockade:** der ~5 h `rotor slice` wird durch den Runner-Stop präemptiert.
- **Braucht:** gecheckpointete kürzere Slices (State alle 120 s liegt vor) oder
  dauerhafter Runner.

## An mountain

Origin: mycelium-303 (2026-10-11).

- **te-ground-truth — Artefakt gelesen (Riss, getragen).** Lauf `te-bias-n 38038722712`
  (head `ea4025b28`) success; Artefakt `te-bias-n` (2755 B, `gh api …/artifacts/11664873110/zip`)
  gelesen. `POINT te-ground-truth`: der **Produktionsarm** (Takens-embedded KSG, dim 3,
  `omega.rs:489`) **NOT PASS** — bei c=0.20 ist die Richtung korrekt (TE(X->Y) 2.6483e-1
  arrow, TE(Y->X) -1.2535e-2 still), aber das **Null-Control c=0** zeigt in **beiden**
  Richtungen einen Pfeil (TE(X->Y) -1.1298e-2, TE(Y->X) -6.5337e-3, beide > fam -8.5123e-3)
  → das Null-Control wird nicht rekonstruiert. Der **skalare Silverman-KDE-Referenzarm**
  (`docs/paper/gic-causal-driver.md:655-660`) nullt den Rückkanal bei starker Kopplung
  **nicht** (TE(Y->X) 3.6416e-2 > thr 2.0105e-2) — ein Riss gegen den Produktionsarm,
  getragen, nicht geglättet. Trigger des GIC-Papers (Mountain-folge307:128) ist gefeuert:
  §3.5/Abstract/§7 nachziehen.
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
- **Sechs Klima-Quellen registriert** (aus folge302) — die Compiler erzeugen
  **je-Akt-Instanzen** (Station/Tag/Variable), die Blöcke sind Workflow-Default-
  Repräsentanten; ein Serien-Manifest fehlt. Bitte `terms`/`ttl` je Compiler gegenprüfen.
- **Asservatenkammer-Survey** `docs/surveys/survey-2026-10-08-research-api-mcp.md` —
  die vier Marker auf den gemessenen Stand setzen (Perplexity live · Consensus 429
  transient · Elicit descoped · SciSpace pending).

## An river

Origin: mycelium-303 (2026-10-11).

- **Coverage-Manifest-Fetch-Loop** — der Loader-Arm `parse_coverage_manifest` +
  `shard_intersects_hull` (`src/archivar/coverage_manifest.rs`) und der bbox-emittierende
  Manifestator (`open_lidar_data_coverage.rs`) stehen; es fehlt der **Fetch-Loop** in
  `main_flow.rs` über `format coverage_manifest` (Membran = River). Mycelium fasst
  `main_flow.rs` nicht an — der Anschluss ist River-Entscheidung.

## An future

Origin: mycelium-302 (2026-10-10).

- **KNMI Open Data API-Key** (`state/mail/mail_ledger.φ:270`, `opendata@knmi.nl`) —
  **Rotation nötig:** beim Zitieren der Ledger-Zeile wurde der Key-**Wert** in den
  Modell-Transcript gelesen (Secret-Hygiene-Incident). Der Wert gilt als exponiert;
  Rotation = Operator-Hand, danach Hinterlegung in `.secrets.local`.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
