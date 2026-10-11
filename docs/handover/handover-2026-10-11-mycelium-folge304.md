<!--
  title: Handover — Mycelium-Folge 304 (2026-10-11)
  session: Mycelium-Linie — Meta-Pass. Faltung des river-`## An mycelium`-Blocks (OMNI2 1h present), usgs_lidar_coverage-Manifest auf shard-Form geschnitten (ept.json/boundary.json heraus), ECAD/WorldClim-sha256 auf die materialisierten Bytes gesetzt, ECAD/DWD/WorldClim/AODN/SURFRAD/Radar-Assets gemessen, cmb-cdn/surfrad-cdn/omni2-cdn neu dispatcht.
  class: handover
  date: 2026-10-11
  sha256: 224ab4b40156e63803b200128f147dc3e4486b22727d1234824fcff35f7189a1
  status: live
-->
# Handover — Mycelium-Folge 304 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-11-mycelium-folge303.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.1421 · cap 0.5 — Grund: dieses Atom (zwei Fortsetzungen) 1× line ($0.1311) + 2× grind-flash (CDN-Asset-Verifikation $0.0110 + Portal-Enumeration) · deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-11 | Operator (Session, Mycelium 304) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-11-mycelium-folge303.md` §Operator-Wort-Register (via `git show <sha>:archiv/…`) | 2026-10-11 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation SPT-3G D1 `cmap` (cmb-cdn) — Speichermodell blockiert (7,87-GB-bz2 im RAM)
- **Status:** blockiert | **Bindung:** eigen (Manifestation)
- **Trigger:** Lauf `cmb-cdn` Abschluss
- **Lage:** (gemessen 2026-10-11) die Quelle `full_maps_d1.tar.bz2` ist **7 873 515 864 B**
  (`curl -sI` Content-Length). Der Compiler liest sie ganz (`fs::read`,
  `cmb_planck_compiler.rs:259-272`) und `archivar::bzip2::decompress`
  (`src/archivar/bzip2.rs:361`, `out.extend_from_slice` je Block) baut den **entpackten**
  Tar zusätzlich vollständig im RAM; drei cmb-cdn-Läufe starben exit 137/„Getötet" genau beim
  Compiler-Start (Build je 6 m 31-33 s sauber) — ein **Speicher-Signal**, nicht die zuvor
  notierte Preemption (`ci_manage log 38117879560`/`38117250487`). Der Block
  `phi/sources.φ:11559` trägt **keine** `sha256` — das Asset ward nie materialisiert.
- **Blockade:** non-streaming bz2/tar — 7,87 GB komprimiert **plus** ein Multi-GB-entpackter
  Tar (und der FITS-Member) gleichzeitig im RAM sprengen jeden ≤16-GB-Runner.
- **Braucht:** Rat gefallen (2026-10-11, fünf Stimmen): Weg (a) additiv, einsträngig —
  (1) **erledigt** `Bz2Decoder<R: Read>` + `decompress_reader` neben `decompress`
  (`src/archivar/bzip2.rs`, Commit `55a8601b7`, Gate-Test byte-identisch, vier Aufrufer
  unberührt); (2) `bzip2_tar_members` über den bestehenden streamenden `TarScan`
  (`src/archivar/inflate.rs`), Member streamend auf Platte statt `Vec`-Puffer;
  (3) Compiler-`--tar`-Arm `File → BufReader → Bz2Decoder → Tar-Member-Datei`, FITS aus
  der Datei; (4) der `--url`-Arm bleibt benannter Folgeschritt. Danach `gh workflow run
  cmb-cdn.yml`; Manifestation schließt mit `sha256` in `phi/sources.φ:11559`.

### Coverage-Manifest — Fetch-Loop (River) + usgs-shard-Form gebaut; Nationalportale enumeriert
- **Status:** eigen | **Bindung:** eigen (Core-Archivar) · Teile linie:river
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) (a) **erledigt** — `usgs_lidar_coverage.rs` emittiert
  `shard`-Form (`manifest_lines` = `shard <url>`/`Pending`, `is_lidar_key` nur `.laz`/`.las`;
  `ept.json`/`boundary.json` heraus; Commit `8739dffaa`). (b) **erledigt** — river 181
  (`05ec9423f`) baute den Fetch-Loop `format coverage_manifest` in `main_flow.rs` und faltete
  den `## An river`-Block. **Enumeriert:** `state/mycelium/national_lidar_portals.txt`
  (143 Zeilen; 95 Portal · 33 alt-route · 15 bathy; Regionen EU 29 · NA 17 · LATAM 10 ·
  APAC 27 · AF/ME 12) — aus `survey-2026-10-10-worldwide-lidar-quellen.md`.
- **Blockade:** keine.
- **Braucht:** je Portal ein Manifestator (Survey-Reihenfolge „Was fehlt": NL AHN4 ·
  CH swissSURFACE3D · FR IGN LiDAR HD · UK EA · NZ LINZ); `portal_harvest.rs` trägt die
  CKAN-Klasse, der S3-Manifestator die Bucket-Klasse.

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

### Speicher — R2-Spiegel LIVE, aber OHNE Cap (10-GB-Burn-Gefahr) — Cap gesetzt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) der R2-Spiegel ist **live** (`OMEGAFLOW_R2_MIRROR=1`,
  Worker antwortet HTTP 405 = deployt, `cdn.rs:103` `r2_mirror` im Upload-Pfad,
  `cdn-manifest.yml:40-45` exportiert die R2-Vars an die **36 Caller**). Der Worker tat
  `BUCKET.put` **ohne Größen-/Klassenfilter** — der 10-GB-Free-Tier wäre von jedem
  Katalog-Caller mit GB-Asset verbraucht worden. Die **großen** Compiler (Bayern-DOM
  100 MB, WorldClim 2 GB, cmb) sind **standalone** (kein `OMEGAFLOW_R2_MIRROR`) und
  spiegeln **nicht** — der Burn käme aus der `cdn-manifest`-Familie, nicht von Dominosteinen.
  Absicht laut folge295:165: „R2 **10 GB frei** trägt nur Manifeste/Indizes".
- **Blockade:** keine.
- **Braucht:** **Cap gesetzt** — `r2_mirror` überspringt Dateien über `R2_MIRROR_MAX_BYTES`
  = 16 MiB (`OMEGAFLOW_R2_MIRROR_MAX_BYTES` überschreibbar; Commit `70a66b9d6`), die
  Dataset-Klasse bleibt auf dem Release. Der **Schwellwert 16 MiB** ist eine Setzung
  (Manifest-/Index-Größe) — ein anderes Maß ist ein Operator-/Rat-Wort.

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
- **Lage:** (gemessen 2026-10-11) 20 Workflows `runs-on: [self-hosted, Linux]`;
  Throttle aktiv; Konzept `docs/concepts/self-hosted-runner.md`. Die zwei roten
  `harvest-dispatch`-Läufe (38105695169/38105544121) und die beiden cmb-cdn-Abbrüche
  (exit 137, `38117250487`/`38117879560`-Vorgänger) sind Self-hosted-Signale.
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

Origin: mycelium-304 (2026-10-11).

- **OMNI2 1h — erledigt (river-180-Block abgearbeitet).** `omni2-cdn` dispatcht
  (`38117879618`, success); das Asset `cdaweb.gsfc.nasa.gov/omni2_serie_1h.bin` ist
  present (`--verdict` 206, 59 014 068 B, sha256 `95243a2a…`). River kann den Block
  `## An mycelium` (river-180) streichen.

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
