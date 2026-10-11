<!--
  title: Handover — Mycelium-Folge 305 (2026-10-11)
  session: Mycelium-Linie — Meta-Pass. Taucher-Batterie (Operator-Wort): cmb-cdn-Streaming vollständig (tar_stream/bzip2_tar_stream/fetch_to_file + beide Compiler-Arme), vier Portal-Coverage-Manifestatoren gebaut (NL AHN4 · CH swisstopo · NZ LINZ · FR IGN) + coverage-manifest-Workflow, MCP-no-leak-Gate (src/gate/mcp.rs) gebaut + in commit_check verdrahtet, t420-Throttle-Boot-Lücke + matrix-rotor-Trap-Flush gebaut; UK EA blockiert account.
  class: handover
  date: 2026-10-11
  sha256: 82e2fc674a3abd2d1dc1a349c8744577e21326e271e742f7bc0b5e88b03d8883
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
| Vorherige Worte der Linie: `archiv/handover-2026-10-11-mycelium-folge304.md` §Operator-Wort-Register (via `git show <sha>:archiv/…`) | 2026-10-11 | gefaltet, nicht kopiert |

## Offen — eigen

### Portal-Coverage-Manifestatoren — 4 gebaut + Workflow; UK EA blockiert; Manifestation offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) vier neue Bins, je `shard <absolute-url>`, `cargo build` grün + Einmal-Lauf gezogen:
  - `tools/harvest/src/bin/ahn4_coverage.rs` — ArcGIS FeatureServer `services.arcgis.com/nSZVuSZjHpEZZbRo/.../Kaartbladen_AHN4/FeatureServer/0`, Feld `AHN4_LAZ`, **1407** `.LAZ`; netloc `ns_hwh.fundaments.nl`; Lauf: `1 pages, 1407 tiles`.
  - `tools/harvest/src/bin/swisstopo_coverage.rs` — STAC `data.geo.admin.ch/api/stac/v0.9/collections/ch.swisstopo.swisssurface3d/items`, Assets `.las.zip`; netloc `data.geo.admin.ch`; bbox-Lauf `70 assets`.
  - `tools/harvest/src/bin/linz_coverage.rs` — S3 `nz-elevation` (`s3-ap-southeast-2`), COG `.tiff` (die LAZ-Kante ist key-gated, s. Blockade); **30835** tiles; netloc `nz-elevation.s3-ap-southeast-2.amazonaws.com`.
  - `tools/harvest/src/bin/ign_lidarhd_coverage.rs` — WFS `data.geopf.fr` (`IGNF_LIDAR-HD_METADONNEE:metadata`, Property `url_npl` → `.copc.laz`, **524687** tiles); netloc `data.geopf.fr`.
  - Workflow `.github/workflows/coverage-manifest.yml` (Matrix, `--ci-mode`, `[self-hosted, Linux]`).
- **Blockade:** UK EA (`tools/harvest/src/bin/ea_lidar_coverage.rs` **nicht** gebaut) — die `.laz`-Tiles liegen nur hinter `environment.data.gov.uk/geoservices/data-requests` (**401**, Authorization header required); der keyless File-Store trägt nur Index-Archive (`.gpkg.zip` 35 532 884 B, gemessen), keine `.laz` (download-url generisch 200, SAS-URL 404). **Keine** Fabrikation.
- **Braucht:** (a) die 4 Manifeste manifestieren + in `phi/sources.φ` registrieren (Mycelium: `url`/`origin`/`compiler`/`format`; Mountain: `terms`/`at`/`ttl`) und die Workflows dispatchen; (b) UK EA: EA-Account (Operator) für den Survey-`data-request`, ODER Bau gegen das OGC-Index als reines Coverage-Metadaten-Werkzeug (Entscheidung offen).

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
- **UK EA LiDAR** — der Survey-Tile-Download ist account-gated (`environment.data.gov.uk/geoservices/data-requests` 401). Braucht einen EA-Account (Operator) oder die Entscheidung, gegen das OGC-Index als Metadaten-Werkzeug zu bauen.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
