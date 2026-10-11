<!--
  title: Handover — Mycelium-Folge 307 (2026-10-11)
  session: Mycelium-Linie — Meta-Pass, ein Atom (zwei Operator-Worte). Set-Diff-Schema für die LiDAR-Konsolidierung gebaut: `--emit-ids` (tile-id/epoch/href-Sidecar) in `coverage.rs`/`ahn4_coverage.rs`/`swisstopo_coverage.rs`/`open_lidar_data_coverage.rs`; `coverage_id_diff.rs` (raw + basename-Normalisierung + `--strip-prefix`); `coverage.rs --kind tnm` (USGS TNM Access). NL-Kachel-ID-Diff gemessen: flai ⊇ AHN4-Origin außer 08BZ1/40HZ1. Extraterrestrischer LOLA-COPC-Emitter `lola_coverage.rs`. USGS-TNM gemessen.
  class: handover
  date: 2026-10-11
  sha256: 088f2ca769299c604fd0a0210a31f3f434024152c8c1fd504ce5ea68b1944229
  status: live
-->
# Handover — Mycelium-Folge 307 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-11-mycelium-folge306.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.45 · cap 0.5 — Grund: 9 grind-flash (Bau/Messung) + 1 general, deepseek-flash, kein pro/max. (session_burn-Total $0.8782/24 Sessions, DB-Rollover → Atom-Delta nicht exakt isolierbar; nahe `BURN_CAP_HARD_USD` 0.50.)

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-11 | Operator (Session, Mycelium 307) |
| Name leer → neueste offene Mycelium-Übergabe (folge306) | 2026-10-11 | Operator (Session, Mycelium 307) |
| „bitte umsetzen [ID-Diff-Schema + Diff-Lauf + Mountain-Verdikt; weitere coverage-Arme; Frame/Epoche-Transform; cmb-cdn prüfen; MCP-Wrapper; CI-Subset-Hang]" | 2026-10-11 | Operator (Session, Mycelium 307) |
| **„ja"** — `OPENTOPOGRAPHY_API_KEY` als GitHub-Actions-Secret im eigenen Repo hinterlegen | 2026-10-11 | Operator (Session, Mycelium 307) |
| **„ja"** — `opencode.json`-MCP-Konfiguration anfassen (Wrapper `bin/chrome-devtools-mcp` + egress-Allowlist) | 2026-10-11 | Operator (Session, Mycelium 307) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-11-mycelium-folge306.md` §Operator-Wort-Register (via `git show <sha>:docs/handover/archiv/…`) | 2026-10-11 | gefaltet, nicht kopiert |

## Offen — eigen

### Open-LiDAR-Konsolidierung — Set-Diff-Schema gebaut + NL-Diff gemessen; Mountain-Verdikt + CH-Lauf offen
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) **Schema:** `--emit-ids <path>` schreibt `<id>\t<epoch>\t<href>` (fehlend = `-`) in `coverage.rs` + `ahn4_coverage.rs` + `swisstopo_coverage.rs` + `open_lidar_data_coverage.rs`; `shard`-Manifest byte-identisch. **Diff:** `tools/harvest/src/bin/coverage_id_diff.rs` (`--origin`/`--mirror`, `--normalize basename`, `--strip-prefix`). **NL-Messung** (`state/mycelium/lidar_id_diff_measured.md`): flai NL 1384 COPC vs AHN4-Origin 1375 unique `Name`; raw missing 1375 (Namensraum-Riss), mit `--normalize basename --strip-prefix c_` **missing = 2** (`08bz1`, `40hz1`) → **flai ⊇ AHN4-Origin außer 2 Kacheln** (Kachel-ID-Diff, nicht File-Count — Z.ai-Bedingung erfüllt). **CH:** unread (s. Korrekturen).
- **Blockade:** CH-Origin-Walk > 10 000 Items (Bound 10 Seiten unvollständig) → voller Lauf gehört in CI; Mountain-Verdikt ausstehend.
- **Braucht:** (a) `--emit-ids`-Sidecars in `coverage-manifest.yml` verdrahten + hochladen (CI-Lauf); (b) voller CH-Diff; (c) Mountain-Verdikt auf die gemessene NL-Basis (descope NL; Origin-Manifeste behalten).

### coverage.rs — STAC + TNM + id-Schema gebaut; weitere Arme offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `coverage.rs` trägt `--kind stac` (url-only Default, `--bbox` opt-in) und neu `--kind tnm` (USGS TNM Access `api/v1/products`, `datasets=Lidar Point Cloud (LPC)`, `max`/`offset`-Paginate, url-only, `--emit-ids` id=`id`/`sourceId`/`title`, epoch=`publicationDate`); `cargo check`/build 0/0.
- **Blockade:** keine.
- **Braucht:** (a) `coverage-manifest.yml`-Matrix-Eintrag für `tnm` + Page-Bound; (b) weitere Arme `s3-copc|s3-laz|wfs|arcgis|ept`.

### Frame/Epoche → bbox-off als sicherer Default; Transform-Layer `pending`
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `state/mycelium/frame-epoch-findings.md`: drei bbox-Emitter defaulten url-only (`ShardPlacement::Pending` → nicht verworfen). Fünf Portal-Bins bereits url-only.
- **Blockade:** Transform-Layer braucht CRS-aus-VLRs (zweiter Range-Request) + projiziert→geodätisch (fehlt) + Epoche je Shard + Grammatik-Slot.
- **Braucht:** Transform-Layer als eigener Bau oder dauerhaft bbox-off; Entscheidung bei Mountain. **Nicht in diesem Atom umgesetzt** (Architektur; braucht Mountain/Rat-Wort).

### Manifestation SPT-3G D1 `cmap` (cmb-cdn) — dispatched; sha256 offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `cmb-cdn.yml` run 38129785553 endet
- **Lage:** (gemessen 2026-10-11 10:0xZ, `ci_manage view`) run 38129785553 noch `in_progress` (head `82ec4f9`), Step `cmb → Compile the SPT-3G D1 CMB map`. Trigger nicht gefeuert.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 38129785553` (bzw. `log`), dann sha256/Länge aus dem Release-Digest in den `phi/sources.φ:11559`-Block.

### MCP no-leak-Regel — Wrapper gebaut + verdrahtet; Allowlist-Verschärfung offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `src/gate/mcp.rs` (6 Regeln + Test) in `commit_check` verdrahtet. **Neu:** getrackter Wrapper `bin/chrome-devtools-mcp` (Pin `chrome-devtools-mcp@1.9.0` + stille Defaults, kein `@latest`-Drift), beide `opencode.json`-MCP-Einträge laufen über ihn; `chrome-devtools-voices` trägt jetzt `--no-javascript-evaluation` (deaktiviert `evaluate`/`initScript`/`javascript:`-URLs). Gemessen `chrome-devtools-mcp@1.9.0 --help`: `--allowedUrlPattern` ist ein **Array** und verlangt **Chrome 149+** (installiert: `Google Chrome 155.0.8059.39`); `--proxyServer` als Alternative. `opencode.json` gültiges JSON (`jaq -e`).
- **Blockade:** eine **enge** Allowlist blockt auch Subressourcen (Fonts/CDNs) → die Seats' tatsächliche Host-Menge ist ohne per-Seat-Network-Capture unbekannt; ein enges Muster blind würde die Stimmen brechen. `https://*` bleibt daher vorerst.
- **Braucht:** (a) per-Seat-Network-Capture der Stimmen-Gruppe (CDP `browser_network`, JIT, focus:false) → Host-Menge; (b) `--allowedUrlPattern`-Array aus (a) setzen; (c) Config-Restart-Test (opencode lädt MCP-Config nur beim Start).

### OpenTopography — Konsument gebaut; CI-Secret gesetzt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `tools/harvest/src/bin/opentopography_coverage.rs` walkt `https://portal.opentopography.org/API/otCatalog` (keyless, HTTP 200; 1152 Datasets, 707 PointCloud; `globaldem` keyed → 401 = Auth-Grenze) und emittiert `shard <href>` (je Dataset die DOI/Landing-`url`; **keine** per-Tile-URL — gemessen, nicht erfunden); `API_Key` nur auf `portal.opentopography.org/API/`-Hrefs. Workflow `.github/workflows/opentopography-cdn.yml` (`workflow_dispatch`, self-hosted). `bin/secrets-sync.sh --set` gesetzt: `OPENTOPOGRAPHY_API_KEY` **und** alle `.github`-referenzierten Secrets (39) + `OMEGAFLOW_SECRETS_FILE`.
- **Blockade:** keine.
- **Braucht:** Workflow bei Bedarf dispatchen (lang; self-hosted).

### Architektur — GitHub/CI/CDN-Optimierung
- **Status:** wartend | **Bindung:** eigen · **Teile linie:mountain/river**
- **Trigger:** grüner `ci-check`-Lauf
- **Lage:** (gemessen 2026-10-11) `ci-gate`-`subset` hängt im Step `Run cargo test --lib` (`38129998370` in_progress, runner `ubuntu-24.04-arm`, `timeout-minutes: 30`). **Hang-Ursache gemessen (aus dem Baum, Log `unread` — 404 während des Laufs):** `src/archivar/tests.rs:8234 test_live_sources_extract` (Sweep `live_sweep(...,600)` → `curl -m 2048s` je Quelle, `--connect-timeout 32 ×3`, **kein** `#[ignore]`, kein Offline-Gate) — ein ungekappter Live-Netz-Sweep im Unit-Suite; Zweitkandidaten `test_dst_and_co2_mlo_channels_extract_live` (`:8253`, live), `local_http_head` (`:7663`, `TcpListener::accept()` ohne Deadline). **Rote Jobs** (run 38129998370): `dropped-gate` failure (`current 13 | pinned 100 | new 13`) · `format` failure (rustfmt `tools/measure/src/bin/eclipse_shadow_probe.rs:354,634`).
- **Blockade:** Shard-Build-vs-Test-Anteil erst an green ci-check messbar; rote Jobs + Hang sind **nicht Mycelium-eigen** (Archivar-Tests = Mountain/River, dropped-baseline = Mountain/gate, format = measure-Datei).
- **Braucht:** (a) `src/archivar/tests.rs`-Live-Tests gaten (`#[ignore = "live network"]` oder Offline-Gate) — **Fix bei mountain/river**; (b) grüner `ci-check` → `ci_manage jobs <id>`; (c) dominiert Build: `cargo nextest archive` + Shards `--archive-file`; (d) Test-Suite-Dedup.

### matrix-rotor — Trap-Flush gebaut; Nur der persistente t420 beendet die Klasse
- **Status:** wartend | **Bindung:** eigen (Workflow) · **Teile linie:river**
- **Trigger:** nächster `matrix-rotor`-Lauf
- **Lage:** (gemessen 2026-10-11) `checkpoint` im SIGTERM-Trap (`matrix-rotor.yml:120`) flusht den jüngsten State; ein Runner-Shutdown bricht den Job dennoch ab (im Workflow nicht heilbar).
- **Blockade:** nur der persistente t420 (`bin/matrix_watchdog.sh:11-21`) ist dauerhaft.
- **Braucht:** Doc/Code-Divergenz `matrix_watchdog.sh:4` (Default `~/.local/state/omegaflow`) vs `matrix.rs:33` (`data`) schließen.

### LiDAR-Quellen-Inventar — TNM-Access gemessen; weitere Bucket-Bins offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `state/mycelium/usgs_tnm_index_measured.md`: TNM Access `api/v1/products` HTTP 200, **9 755 359** LPC-Produkte (Dataset-String `Lidar Point Cloud (LPC)`, spaces!); per-Tile `boundingBox`+`downloadURL`(LAZ)+`publicationDate`; `prodFormats=COPC` → **0**. Der `--kind tnm`-Arm ist gebaut.
- **Blockade:** keine.
- **Braucht:** (a) je Bucket ein `<x>_coverage.rs` (`lola_coverage.rs` gebaut); (b) TNM als `--kind tnm` in `coverage-manifest.yml` verdrahten (COPC/EPT bleibt S3/PC-Azure).

### Extraterrestrische LiDAR — `lola_coverage.rs` gebaut; MOLA/MLA/NLR/OLA/Hayabusa-Parser offen
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `tools/harvest/src/bin/lola_coverage.rs` (539 Z.) walkt `astrogeo-ard` `moon/lro/lola` (S3 ListObjectsV2, paginiert) und emittiert url-only `shard <url>` für `.copc.laz`; `DEFAULT_BUCKET`/`PREFIX` aus `state/mycelium/lidar_sources_measured.md:60-61`; `cargo check`/build 0/0.
- **Blockade:** die nicht-COPC-Körper brauchen Parser (Mountain).
- **Braucht:** Parser für MOLA/MLA/NLR/OLA/Hayabusa (Mountain); LOLA-Bin in `coverage-manifest.yml` verdrahten.

### Bathymetrie — Grids keyless; Topo-Bathy-Punktwolken getrennt
- **Status:** eigen | **Bindung:** eigen · **Teile linie:river**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, keyless) Grids GEBCO/BlueTopo/ETOPO/EMODnet/GMRT/SRTM15+; Topo-Bathy-Punktwolken NOAA `noaa-nos-coastal-lidar-pds` (`noaa_nos_coverage.rs` + STAC 193 Topobathy).
- **Blockade:** Grids sind Raster, nicht im `shard`-Kontrakt.
- **Braucht:** Raster-Konsument (`pending`); Topo-Bathy läuft über `noaa_nos_coverage` + flai.

## Registrierte Korrekturen & Vorfälle

- **CH-Origin-Zahl widerlegt (Riss):** `flai_coverage_measured.md` nennt CH `swisssurface3d` 79683 ≫ Origin **70**; der gebundene Origin-Walk (`swisstopo_coverage --max-pages 10`) und ein Cursor-Probe erreichten **>10 000 Items** mit `next`-Link weiterhin vorhanden (gemessen 2026-10-11). Die „70" ist ein Flächen-Indikator-Fehler, kein Origin-Count — CH-Diff `unread` bis zum vollen Lauf.
- **Riss getragen (Namensraum):** NL raw missing = 1375, normalisiert+`c_`-gestrippt = 2 — File-Key (`C_01CZ1`) ≠ Origin-`Name` (`01CZ1`) ist ein Namensraum-Riss, kein Abdeckungs-Riss; beide Linien im Report gehalten.
- **Fix eigener Zustand:** der erste 307-Commit ließ die konsumierte `handover-…-folge306.md` am alten Pfad (nur die `archiv/`-Kopie war im Commit) — die Doppelung ist in diesem Commit als Löschung geschlossen.
- **Adressierter Block gefaltet & veraltet:** `## An mycelium` (river-182/185) — `usgs_lidar_coverage.rs` emittiert bereits `shard <url>`; die Nationalportale liegen in `state/mycelium/national_lidar_portals.txt` (144 Z.).
- **Vollständigkeit der flai-Walks:** 8 Länder (BE/EE/ES/FI/LV/PL/UK/US) am Page-Cap abgebrochen — für den ID-Diff ggf. nachzuziehen.

## An mountain

Origin: mycelium-307 (2026-10-11).

- **Descope-Verdikt Open-LiDAR — NL-Messung liegt vor:** `state/mycelium/lidar_id_diff_measured.md` — flai ⊇ AHN4-Origin außer `08BZ1`/`40HZ1` (Kachel-ID-Diff via `coverage_id_diff --normalize basename --strip-prefix c_`; raw 1375 missing = Namensraum-Riss). Frage: `ahn4` (NL) als Origin-Quelle zugunsten des flai-Aggregats descopen (Kachel-Äquivalenz gemessen, 2 Lücken), Origin-Manifest als Provenanz behalten? `swisstopo` (CH) — Origin > 10 000 Kacheln (die „70" widerlegt), Diff offen. `ign_lidarhd` (FR) nur partial. `linz` (NZ) bleibt. `ea_lidar_index` (Metadaten) — behalten oder reicht flai UK DEFRA?
- **Vier Portal-Coverage-`phi`-Blöcke** (aus folge305, offen): je Portal der `url`/`origin`/`compiler`/`format`-Block aus dem `coverage-manifest`-Lauf, plus `terms`/`at`/`ttl`. Netlocs: `ns_hwh.fundaments.nl` · `data.geo.admin.ch` · `nz-elevation.s3-ap-southeast-2.amazonaws.com` · `data.geopf.fr`. (Hinweis: jetzt url-only via bbox-off-Default.)
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
