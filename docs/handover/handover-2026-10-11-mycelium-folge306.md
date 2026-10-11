<!--
  title: Handover — Mycelium-Folge 306 (2026-10-11)
  session: Mycelium-Linie — Meta-Pass, ein Atom. LiDAR-Konsolidierung gemessen und gebaut: flai-Abdeckung je Land (NL/CH descope-Kandidat, FR partial, CA fehlt, DE nur Sachsen); generischer `coverage.rs --kind stac` (Planetary Computer 3dep-lidar-copc); Frame/Epoche-Riss gemessen → `--bbox-off` als sicherer Default in `coverage.rs`/`open_lidar_data_coverage.rs`/`noaa_nos_coverage.rs`; Z.ai-Stimme nachgefasst (Kachel-ID-Diff-Bedingung). SPT-3G D1 cmb-cdn dispatched (run 38129785553).
  class: handover
  date: 2026-10-11
  sha256: 240ac93eb995c835e09eb2a10df930eb1b51b2669368b62a2a8b23a521bfdf8a
  status: live
-->
# Handover — Mycelium-Folge 306 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-11-mycelium-folge305.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.46 · cap 0.5 — Grund: 4 Taucher (2 grind-flash Bau + 2 general Messung), 1 UI-Round (Z.ai); System-Delta des Atoms (session_burn $1.2481 → $1.7088). deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-11 | Operator (Session, Mycelium 306) |
| Genannte offene Punkte: (a) flai-Abdeckung je Land → descope; (b) `--kind stac` als erster Arm; (c) Frame/Epoche-Transform oder bbox-off; (d) Z.ai nachfassen | 2026-10-11 | Operator (Session, Mycelium 306) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-11-mycelium-folge305.md` §Operator-Wort-Register (via `git show <sha>:docs/handover/archiv/…`) | 2026-10-11 | gefaltet, nicht kopiert |

## Offen — eigen

### Open-LiDAR-Konsolidierung — gemessene Basis + Z.ai-Bedingung; Mountain-Verdikt offen
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `state/mycelium/flai_coverage_measured.md`: flai `open-lidar-data` trägt NL AHN4 2771 COPC (2020–2022) ≥ Origin 1407 `.LAZ`; CH swisssurface3d 79683 (2022+2023) ≫ Origin 70; FR **nur** Lidar_2021 (337212) gegen Origin 524687 über mehrere Jahre → **partiell**. CA fehlt im Bucket (2× gemessen); DE nur Sachsen (GeoSN, 14919). **Z.ai-Stimme** (`state/stimmen/20261011T095149Z_zai_lidar-verdikt.md`): NL/CH descopen, FR partial, CA/DE Origin zwingend — **aber Äquivalenz per Kachel-ID-Diff, nicht per File-Count** (Re-Tiling); Origin-Manifeste als Provenanz behalten. Riss getragen: File-Count ≠ Äquivalenz.
- **Blockade:** der Kachel-ID-Diff fehlt (Count ist kein Beweis).
- **Braucht:** (a) Mountain-Verdikt auf die gemessene Basis (descope je Land); (b) Kachel-ID-Diff-Adapter (unten); (c) Origin-Manifeste behalten, nur Ingest descopen.

### coverage.rs — STAC-Arm gebaut; Adapter-Schema (tile-id/footprint/epoch/href) noch nicht
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `tools/harvest/src/bin/coverage.rs` (neu, 597 Z.) baut `--kind stac` für Planetary Computer `3dep-lidar-copc`; zieht das STAC-Item-`bbox` (6 f64, gemessen an einem Item) + `assets.*.copc.laz`; emittiert `shard <url> <6 f64>` (mit bbox) bzw. `shard <url>` (Default, s. bbox-off); `--ci-mode` via `upload_release`; `cargo check` 0/0, Bin-Build grün. Ausgabe heute: nur `shard <url>` — Z.ai-Bedingung (tile-id, footprint, epoch, href) fehlt.
- **Blockade:** keine.
- **Braucht:** (a) Ausgabeschema um `tile-id`/`epoch`/`href` erweitern (Set-Diff-fähig); (b) `coverage-manifest.yml`-Matrix-Eintrag mit benanntem Page-Bound (`coverage.rs` hat MAX_PAGES=5000, PAGE_LIMIT=1000 — die 3dep-Kollektion ist groß); (c) die weiteren Arme `s3-copc|s3-laz|wfs|arcgis|ept|tnm` als Ausbaustufen.

### Frame/Epoche → bbox-off als sicherer Default gebaut; Transform-Layer `pending`
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `state/mycelium/frame-epoch-findings.md`: drei Emitter schreiben eine bbox in Nicht-ICRS-Rahmen (`coverage.rs` WGS84-Grad+Z-Meter; `open_lidar_data_coverage.rs`/`noaa_nos_coverage.rs` LAS/COPC-Header in per-Datei-CRS), der Loader `src/archivar/coverage_manifest.rs` vergleicht sie roh mit der ICRS-Hülle → stille Fehlverwerfung. **Gebaut:** `--bbox` ist jetzt Opt-in, Default = url-only (`shard <url>`, Loader `ShardPlacement::Pending` → nicht verworfen) in genau diesen 3 Bins; `cargo check` 0/0 + 3 Bin-Builds grün. Fünf Portal-Bins emittieren bereits url-only.
- **Blockade:** der Transform-Layer braucht CRS-aus-VLRs + projiziert→geodätisch (fehlt) + Epoche je Shard + Grammatik-Slot.
- **Braucht:** Transform-Layer als eigener Bau (Ellipsoid/IAU→ICRS) oder dauerhaft bbox-off; Entscheidung bei Mountain (Quellen-Verdikt-Rahmen).

### Manifestation SPT-3G D1 `cmap` (cmb-cdn) — dispatched; sha256 offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `cmb-cdn.yml` run 38129785553 endet
- **Lage:** (gemessen 2026-10-11) Streaming-Compiler gebaut (`tar_stream`/`bzip2_tar_stream`/`fetch_raw_to_file_with`, beide Compiler-Arme), `cargo check` grün. Workflow dispatched (`gh workflow run cmb-cdn.yml` → https://github.com/omegaflow/omegaflow/actions/runs/38129785553).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 38129785553` (bzw. `log`), dann sha256/Länge aus dem Release-Digest in den `phi/sources.φ:11559`-Block.

### MCP no-leak-Regel — Gate gebaut; Wrapper + egress-Allowlist offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `src/gate/mcp.rs` (6 Regeln + Test) in `commit_check` verdrahtet, `cargo check`/`build` grün; `opencode.json` bewusst nicht angefasst.
- **Blockade:** host-scoped egress-Allowlist braucht per-Seat-Network-Capture; opencode-MCP-Schema hat kein Integrity-Feld.
- **Braucht:** (a) getrackten Wrapper `bin/chrome-devtools-mcp` (sha512 `RnzXoJiUQ44hpOihWk90uOhLD/CnwDkDy0ldHMZONJ2nYQ+dWN1fq1luHHqyd+7FuYnyIlCY6uTThbN5ut9kSQ==`) bauen; (b) Seats' Host-Listen capturen, `--allowedUrlPattern https://*` ersetzen.

### Architektur — GitHub/CI/CDN-Optimierung
- **Status:** wartend | **Bindung:** eigen · **Teile linie:mountain/river**
- **Trigger:** grüner `ci-check`-Lauf
- **Lage:** (gemessen 2026-10-11) `ci-gate`-`subset` hängt im Step `Run cargo test --lib`; Nachbar-Jobs rot: register `license_census` terms-vocab ×3 · format rustfmt in `tools/utils/src/bin/archive_search.rs` · dropped-gate 5 neue Drops. Keiner Mycelium-eigen.
- **Blockade:** Shard-Build-vs-Test-Anteil erst an green ci-check messbar.
- **Braucht:** (a) grüner `ci-check` → `ci_manage jobs <id>`; (b) dominiert Build: `cargo nextest archive` + Shards `--archive-file`; (c) Test-Suite-Dedup.

### matrix-rotor — Trap-Flush gebaut; Nur der persistente t420 beendet die Klasse
- **Status:** wartend | **Bindung:** eigen (Workflow) · **Teile linie:river**
- **Trigger:** nächster `matrix-rotor`-Lauf
- **Lage:** (gemessen 2026-10-11) `checkpoint` im SIGTERM-Trap (`matrix-rotor.yml:120`) flusht den jüngsten State; ein Runner-Shutdown bricht den Job dennoch ab (im Workflow nicht heilbar).
- **Blockade:** nur der persistente t420 (`bin/matrix_watchdog.sh:11-21`) ist dauerhaft.
- **Braucht:** Doc/Code-Divergenz `matrix_watchdog.sh:4` (Default `~/.local/state/omegaflow`) vs `matrix.rs:33` (`data`) schließen.

### LiDAR-Quellen-Inventar — neue keyless Buckets + Indizes (gemessen); Bins offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `state/mycelium/lidar_sources_measured.md`: neue keyless Buckets (japan/kanagawa/srsp/giselevation/njogis/dc/colorado/são-paulo/nz-coastal) + keyless Indizes (USGS TNM Access `tnmaccess.nationalmap.gov`, 9 755 359 LiDAR-Produkte; OpenTopography ~500). `astrogeo-ard` trägt LOLA-COPC (Riss aufgelöst).
- **Blockade:** keine.
- **Braucht:** je Bucket ein `<x>_coverage.rs`; USGS-TNM-Access als **ein** US-weiter Index prüfen (vs Einzelbuckets, vs Planetary Computer 3dep).

### Extraterrestrische LiDAR — keyless Bestand gemessen; LOLA-COPC passt in `shard`
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, keyless) Mond LOLA (PDS `imbrium.mit.edu` + COPC `astrogeo-ard`), LALT, Clementine; Mars MOLA; Merkur MLA; kleine Körper Eros/Bennu/Ryugu/Itokawa (SBN/DARTS). Gated: Chandrayaan-1 LLRI, Chang'e LAM.
- **Blockade:** keine.
- **Braucht:** `lola_coverage.rs` (astrogeo-ard COPC) als nächster Bau; MOLA/MLA/NLR/OLA/Hayabusa brauchen **Parser** (Mountain).

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

- **Riss getragen (Z.ai vs. flai-Count):** „2771 vs 1407" ist **kein** Äquivalenzbeweis (Re-Tiling) — descope NL/CH braucht den Kachel-ID-Diff (`state/stimmen/20261011T095149Z_zai_lidar-verdikt.md`).
- **An river gefaltet** (river-folge185 → mycelium): der `format coverage_manifest`-Fetch-Loop steht (`src/archivar/main_flow.rs:5102-5224`); offen bleibt auf Mycelium-Seite (a) `usgs_lidar_coverage.rs`-Manifest auf `shard <url> <6 f64>` entscheiden (emittiert url-only, mit `--bbox` optional) — jetzt durch bbox-off-Default entschieden (url-only), (b) die 104 Nationalportale (`docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md`) enumerieren → in diesem Atom nicht begonnen.
- **Taucher-Lehre gehalten:** jeder Bau-Auftrag trug „create NEW file; if the path exists, STOP" — kein Fremd-Werk überschrieben (anders als im 305-Atom).
- **Vollständigkeit der flai-Walks:** 8 Länder (BE/EE/ES/FI/LV/PL/UK/US) am Page-Cap (100 000 Keys) abgebrochen, nicht vollständig gezählt — für den ID-Diff ggf. nachzuziehen.

## An mountain

Origin: mycelium-306 (2026-10-11).

- **Descope-Verdikt Open-LiDAR:** gemessene Basis (`state/mycelium/flai_coverage_measured.md`) + Z.ai-Bedingung (`state/stimmen/…`). Frage: `ahn4` (NL) und `swisstopo` (CH) als Origin-Quellen behalten oder zugunsten des flai-Aggregats descopen (Fläche+Epoche gleichauf, ID-Diff offen)? `ign_lidarhd` (FR) nur partial. `linz` (NZ) bleibt (nicht in flai). `ea_lidar_index` (Metadaten) — behalten oder reicht flai UK DEFRA?
- **Vier Portal-Coverage-`phi`-Blöcke** (aus folge305, offen): je Portal der `url`/`origin`/`compiler`/`format`-Block aus dem `coverage-manifest`-Lauf, plus `terms`/`at`/`ttl`. Netlocs: `ns_hwh.fundaments.nl` · `data.geo.admin.ch` · `nz-elevation.s3-ap-southeast-2.amazonaws.com` · `data.geopf.fr`. (Hinweis: jetzt url-only via bbox-off-Default.)
- **Bayern-DOM φ-Block** (aus folge304, offen) — Mycelium-Direktive + Mountain-Verdikt (`terms`/`at`/`ttl`): `url …/download1.bayernwolke.de/32605_5282_20_DOM.bin` · `origin …/32605_5282_20_DOM.tif` · `compiler tools/harvest/src/bin/dom_compiler.rs` · `format bdom` · `sha256 a5da30f9f2766b6cfcb4e1898f097b8e9f0685c1bd08a3feaec54bc8a0acbaa8`.
- **VNP46A3 φ-Block** (aus folge302, offen) — `…/vnp46a3_allangle_composite_snow_free.bin` · `origin …/VNP46A3.A2026213.h18v07.002.2026252141449.h5` · `compiler vnp46a3_compiler.rs` · `format black_marble_vnp46a3_nightlight` · `sha256 d85ee99b9d967b5565bb79e145d9120ab09205eb41d4188b3f016e8944b94510`.
- **open-lidar-data φ-Block** (aus folge302, offen) — `ttl`-Zeile nötig: `url …/open_lidar_data_be_dhmv2.bin` · `origin https://open-lidar-data.s3.amazonaws.com/data/BE/EODaS/LiDAR_DHMV_II-2013-2015/copc/` · `compiler open_lidar_data_compiler.rs` · `format las` · `sha256 e6560344c5e939458c890e62d5972dbfb273f8aa2cdd6e00bdb6ec2feef09c4e`.
- **Sechs Klima-Quellen** (aus folge302) — Compiler erzeugen je-Akt-Instanzen; ein Serien-Manifest fehlt. Bitte `terms`/`ttl` je Compiler gegenprüfen.
- **Asservatenkammer-Survey** `docs/surveys/survey-2026-10-08-research-api-mcp.md` — vier Marker auf den gemessenen Stand setzen (Perplexity live · Consensus 429 transient · Elicit descoped · SciSpace pending).

## An future

Origin: mycelium-302 (2026-10-10).

- **KNMI Open Data API-Key** (`state/mail/mail_ledger.φ:270`, `opendata@knmi.nl`) — **Rotation nötig:** der Key-**Wert** wurde beim Zitieren in den Modell-Transcript gelesen (Secret-Hygiene-Incident). Rotation = Operator-Hand, danach in `.secrets.local`.

Origin: mycelium-306 (2026-10-11).

- **t420 Runner-Throttle-Neuinstallation** — `bin/runner_throttle.sh` `cmd_adaptive` setzt die IFB/tc-Klassen jetzt beim Start neu (`apply_cap "$MAX_KBIT"`, gemessen 2026-10-11, Repo fest) — die stille Ungedrosseltheit nach Reboot ist repo-seitig geheilt. Die Skript-Neuinstallation (`install`/`set` als root) auf t420 und die per-Gerät-QoS am Router bleiben Operator-Hand.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
