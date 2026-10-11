<!--
  title: Handover — Mycelium-Folge 308 (2026-10-11)
  session: Mycelium-Linie — Meta-Pass, ein Atom (ein Operator-Wort). SPT-3G D1 partial-sky/RING-Pfad im `cmb_planck_compiler` gebaut (`pix2ang_ring` + `FitsTable`-Partial-Branch); `coverage-manifest.yml` um USGS-TNM + LOLA erweitert; `ci-check.yml` um den cmb-Bin-Test; `self-hosted-runner` per-Gerät-QoS descoped; MCP/Wrapper unverändert.
  class: handover
  date: 2026-10-11
  sha256: fd929adce6cf0444de353d06856ea7c77d2dc2aa6bd05c5b6894058c256696e2
  status: live
-->
# Handover — Mycelium-Folge 308 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-11-mycelium-folge307.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0700 — Grund: 1 grind-flash (cmb-Fix), 2 general (D1-Diagnose + coverage-Wiring), deepseek-flash, kein pro/max. (session_burn-Total nicht exakt isolierbar, DB-Rollover.)

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | 2026-10-11 | Operator (Session, Mycelium 308) |
| Name leer → neueste offene Mycelium-Übergabe (folge307) | 2026-10-11 | Operator (Session, Mycelium 308) |
| „Die Übergabe IST der Stand … Mycelium ist die Meta-Linie — der Stehende Pass. Sie schließt als erste … erst danach öffnen die anderen Linien." | 2026-10-11 | Operator (Session, Mycelium 308) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-11-mycelium-folge307.md` §Operator-Wort-Register (via `git show <sha>:docs/handover/archiv/…`) | 2026-10-11 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation SPT-3G D1 `cmap` (cmb-cdn) — Lauf rot: partial-sky/RING; Compiler-Fix gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `cmb-cdn.yml`-Lauf (re-dispatch)
- **Lage:** (gemessen 2026-10-11, `ci_manage log 38129785553`) Lauf `38129785553` **failure**: `cmb_planck_compiler --tar … --nside 64` → `NAXIS2 36557509 != 12*NSIDE^2 (8192)` (exit 1). **Ursache gemessen** (HTTP-Range-Kopf des tar, `/tmp/opencode`, 2026-10-11): `full_095ghz.fits` trägt `ORDERING='RING' NSIDE=8192 INDXSCHM='EXPLICIT' OBJECT='PARTIAL' NAXIS1=76 TFIELDS=10`, Spalten `1 PIXEL (J)` / `2 TEMPERATURE (D,uK)` — partial-sky, RING, expliziter Pixel-Index; der Compiler erzwang NESTED-Vollhimmel. **Gebaut (dieses Atom):** `pix2ang_ring` (`src/mathematikerin/healpix.rs`) + Partial-Branch (`FitsTable`-Read, PIXEL+TEMPERATURE, `ang2pix_nest`-Downsample, leere Grob-Pixel → `None`) in `cmb_planck_compiler.rs`; der SMICA-Vollhimmel-Pfad bleibt. `cargo check` 0/0; `cargo fmt --` angewandt; `ci-check.yml:49` um `--bin cmb_planck_compiler` erweitert (der neue Bin-Test läuft in CI; lokal strukturell verweigert).
- **Blockade:** keine (Fix liegt im Baum).
- **Braucht:** (a) `gh workflow run cmb-cdn.yml` (re-dispatch); (b) bei grün sha256/Länge aus dem Release-Digest in den `phi/sources.φ:11620`-Block.

### Open-LiDAR-Konsolidierung — Set-Diff + NL-Diff gemessen; Sidecar-Upload + CH-Lauf + Mountain-Verdikt offen
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) Schema `--emit-ids <id>\t<epoch>\t<href>` in `coverage.rs`/`ahn4_coverage.rs`/`swisstopo_coverage.rs`/`open_lidar_data_coverage.rs`; `coverage_id_diff.rs`. NL-Diff (`state/mycelium/lidar_id_diff_measured.md`): flai ⊇ AHN4-Origin außer `08BZ1`/`40HZ1` (raw 1375 missing = Namensraum-Riss). **Neu gemessen:** `--ci-mode` lädt nur das Haupt-Manifest hoch (`coverage.rs:716`, `ahn4_coverage.rs:493`, `swisstopo_coverage.rs:534`, `open_lidar_data_coverage.rs:286`) — der `--emit-ids`-Sidecar erreicht den CDN **nicht** (die `emit_ids`-Blöcke schreiben nur lokal).
- **Blockade:** Sidecar-Upload fehlt; CH-Origin-Walk > 10 000 Items (Bound unvollständig).
- **Braucht:** (a) Sidecar-Upload verdrahten — Option B (hält den R2-Mirror): nach dem `emit_ids`-Block `if ci_mode && !upload_release(&netloc, ids_pfad) { return Err(...) }` in den vier Bins; Option A: Workflow-Schritt `gh release upload "${{ matrix.netloc }}" <ids> --clobber --repo omegaflow/sources` (bypasst den R2-Mirror); (b) voller CH-Lauf in CI; (c) Mountain-Verdikt auf die gemessene NL-Basis (descope NL; Origin-Manifest behalten).

### coverage-manifest — USGS-TNM + LOLA verdrahtet; weitere Arme offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `coverage.rs` trägt `--kind stac|tnm` (url-only Default, `--bbox` opt-in; TNM `api/v1/products`, `datasets=Lidar Point Cloud (LPC)`, offset-Walk, feste `MAX_PAGES=5000`, kein `--max-pages`). **Neu:** `coverage-manifest.yml`-Matrix um `coverage --kind tnm --limit 1000` (netloc `tnmaccess.nationalmap.gov`) und `lola_coverage` (netloc `astrogeo-ard.s3.amazonaws.com`) erweitert.
- **Blockade:** keine.
- **Braucht:** (a) weitere Arme `s3-copc|s3-laz|wfs|arcgis|ept`; (b) STAC-Arm als eigene Matrix-Zeile (Planetary Computer) bei Bedarf.

### Coverage-Manifest-Fetch-Loop (aus river-folge186 §An mycelium gefaltet) — usgs-Format + 104 Portale offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, river-folge186) Der Arm `format coverage_manifest` steht in `src/archivar/main_flow.rs:5102-5224` (Manifest fetchen → `parse_coverage_manifest` → `shard_intersects_hull` über die eine `enclosure_rho` → überlappende Shards in den Cache; `cargo check`/`fmt` grün).
- **Blockade:** keine.
- **Braucht:** (a) `usgs_lidar_coverage.rs`-Manifest auf `shard <url> <6 f64>` entscheiden (emittiert weiter `url size`); (b) die 104 Nationalportale (`docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md`) enumerieren.

### Frame/Epoche — bbox-off bleibt; Transform gehört an den Query-Time-Loader (Rat+UI bestätigt)
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) Research `state/mycelium/frame-epoch-feasibility.md` + Rat + UI (`state/stimmen/20261011T1035Z_mycelium_architektur-verdikte.md`): `--bbox-off` bleibt der verlustfreie Default (`ShardPlacement::Pending` → `coverage_manifest.rs:78`); der Frame-Transform gehört an den Query-Time-Loader. Die ICRS-Hälfte ist gebaut (`itrf_to_icrs`, `receiver.rs:1918`); `src/archivar/projection.rs` (`utm_inverse`, `to_geodetic`) gebaut, `cargo check` 0/0. Nur `coverage.rs` erreicht den Konsumenten (`main_flow.rs:5103`).
- **Blockade:** eine 7. Token/Grammatik-Slot-Erweiterung des `shard`-Vertrags = Wire-Kontrakt-Änderung (Operator/Rat-Wort), nicht Session-Schreibakt.
- **Braucht:** (a) Loader-**Wiring** (Shard-File-CRS lesen + `to_geodetic` anwenden) offen; (b) **wenn** je ein Grammatik-Slot erwogen wird → Operator/Rat-Wort; sonst bleibt bbox-off permanent.

### MCP no-leak-Regel — Wrapper verdrahtet; Allowlist-Capture Z.ai/DeepSeek offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `src/gate/mcp.rs` (6 Regeln + Test) in `commit_check` verdrahtet; getrackter Wrapper `bin/chrome-devtools-mcp` (Pin `chrome-devtools-mcp@1.9.0`), beide `opencode.json`-MCP-Einträge laufen über ihn; `chrome-devtools-voices` trägt `--no-javascript-evaluation`. Capture (`state/mycelium/mcp_allowlist_capture_measured.md`): Duck = `duck.ai`+`improving.duckduckgo.com`, Qwen = `chat.qwen.ai`+`aplus.qwen.ai` → same-origin; **Z.ai/DeepSeek `pending`** (Capture leer/Timeout). `https://*` bleibt vorerst.
- **Blockade:** eine enge Allowlist blockt Subressourcen; die restliche Host-Menge ist ohne Capture unbekannt.
- **Braucht:** (a) Z.ai/DeepSeek-Capture nachziehen; (b) nach dem Config-Restart die Seats prüfen; (c) `(2)` härter prüfen verlangt den `Gestalt`-Slot-Umfang — offen.

### OpenTopography — Konsument gebaut; CI-Secret gesetzt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `tools/harvest/src/bin/opentopography_coverage.rs` walkt `portal.opentopography.org/API/otCatalog` (keyless; `globaldem` keyed → 401); Workflow `.github/workflows/opentopography-cdn.yml`; `bin/secrets-sync.sh --set` gesetzt (`OPENTOPOGRAPHY_API_KEY` + 39 referenzierte Secrets).
- **Blockade:** keine.
- **Braucht:** Workflow bei Bedarf dispatchen (lang; self-hosted).

### Architektur — GitHub/CI/CDN-Optimierung
- **Status:** wartend | **Bindung:** eigen · **Teile linie:mountain/river**
- **Trigger:** grüner `ci-check`-Lauf
- **Lage:** (gemessen 2026-10-11) subset-Hang-Ursache (ungekappter Live-Netz-Sweep `tests.rs:8234`/`:8253`) gefixt (`#[ignore]`); `dropped-legacy-baseline.txt` re-seed. Neue `ci-gate`-Läufe stehen in `subset → Run cargo test --lib` (gemessen 2026-10-11, `ci_manage status`).
- **Blockade:** Shard-Build-vs-Test-Anteil erst an green ci-check messbar.
- **Braucht:** (a) grüner `ci-check` → `ci_manage jobs <id>`; (b) dominiert Build: `cargo nextest archive` + Shards `--archive-file`; (c) Test-Suite-Dedup.

### matrix-rotor — persistenter t420 trägt die Klasse; Doc/Code-Divergenz geschlossen
- **Status:** wartend | **Bindung:** eigen (Workflow) · **Teile linie:river**
- **Trigger:** nächster `matrix-rotor`-Lauf
- **Lage:** (gemessen 2026-10-11) `checkpoint` im SIGTERM-Trap (`matrix-rotor.yml:120`) flusht den jüngsten State; ein Runner-Shutdown bricht den Job dennoch ab. **Neu geschlossen:** die Doc/Code-Divergenz `matrix_watchdog.sh:4` vs `matrix.rs:33` — der Kommentar nennt jetzt den echten Default `data` (`matrix_state_path`, `matrix.rs:29-36`).
- **Blockade:** nur der persistente t420 (`bin/matrix_watchdog.sh:11-21`) ist dauerhaft.
- **Braucht:** —

### LiDAR-Quellen-Inventar — TNM gemessen + verdrahtet; weitere Bucket-Bins offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `state/mycelium/usgs_tnm_index_measured.md`: TNM Access HTTP 200, 9 755 359 LPC-Produkte (spaces!); per-Tile `boundingBox`+`downloadURL`(LAZ)+`publicationDate`; `prodFormats=COPC` → 0. Der `--kind tnm`-Arm ist gebaut und in `coverage-manifest.yml` verdrahtet.
- **Blockade:** keine.
- **Braucht:** (a) je Bucket ein `<x>_coverage.rs`; (b) COPC/EPT bleibt S3/PC-Azure.

### Extraterrestrische LiDAR — `lola_coverage` verdrahtet; MOLA/MLA/NLR/OLA/Hayabusa-Parser offen
- **Status:** eigen | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `tools/harvest/src/bin/lola_coverage.rs` walkt `astrogeo-ard` `moon/lro/lola` (S3 ListObjectsV2, paginiert), emittiert url-only `shard <url>` für `.copc.laz`; in `coverage-manifest.yml` verdrahtet.
- **Blockade:** die nicht-COPC-Körper brauchen Parser (Mountain).
- **Braucht:** Parser für MOLA/MLA/NLR/OLA/Hayabusa (Mountain).

### Bathymetrie/Raster — BlueTopo/EMODnet Compiler gebaut; Register-Blöcke + BlueTopo-Gestalt offen
- **Status:** eigen | **Bindung:** eigen · **Teile linie:river**
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) Research + Rat + UI: die `FieldIdentity::Witness(WitnessKind::Gestalt)`-Klasse (`witness.rs:24`) + Sampling-Compiler tragen die Raster, kein Vertragswechsel. Punkt/XYZ-servierbar (gemessen): GEBCO/GMRT/ETOPO/BlueTopo; raster-only: SRTM15+, EMODnet. **Gebaut:** `bluetopo_compiler.rs` (ERDDAP griddap CSV, GMR1-Gestalt) + `emodnet_bathymetry_compiler.rs` (neue `EMOD`-Gestalt in `witness.rs`/`geo.rs`); `cargo check` 0/0.
- **Blockade:** die `phi/sources.φ`-Register-Blöcke (Datei von fremder Linie gehalten) + BlueTopo eigene Gestalt.
- **Braucht:** (a) `phi/sources.φ`-Register-Blöcke (Mountain-Aufenthalt); (b) BlueTopo eigene Gestalt-Identität (heute unter `GMR1` — Riss benannt); (c) UI-Schärfung: Sampling-Parameter + vertikales Datum.

### DEMETER (CDPP/REGARDS) — CI-Pfad dispatched; Register-Zeile offen (aus future-folge233 §An mycelium gefaltet)
- **Status:** wartend | **Bindung:** eigen · **Teile linie:mountain**
- **Trigger:** Lauf `demeter-cdn` run `38135966376` endet
- **Lage:** (gemessen 2026-10-11) CI-Pfad stand: `.github/workflows/demeter-cdn.yml` (+ `demeter-aggregate-cdn.yml`), `demeter_compiler.rs` (CDN-Tag `regards.cnes.fr`, Monats-Assets `demeter_isl_YYYYMM.bin`, 64 B/record); Konsument `src/archivar/demeter.rs` + `demeter_harvest.rs`. Lizenz (gemessen): `cdpp.cnes.fr` **Licence Etalab-2.0** → redistributable mit Attribution. Lauf `38135966376` dispatcht (queued). `phi/sources.φ` trägt noch **keine** DEMETER-Zeile.
- **Blockade:** eine neue Quellen-Zeile = gemeinsamer Akt (url/origin/compiler = Mycelium; Admission/terms/field = Mountain); kein Allein-Schreibakt.
- **Braucht:** (a) `ci_manage view 38135966376`; (b) Register-Block DEMETER in `phi/sources.φ` (url/origin/compiler/CDN-Tag `regards.cnes.fr` + Mountain terms `Etalab-2.0`/admission/field).

## Registrierte Korrekturen & Vorfälle

- **cmb-cdn-Lauf rot, Ursache gemessen (2026-10-11):** `38129785553` failure im Compile-Step; Log: `NAXIS2 36557509 != 12*NSIDE^2 (8192)`. Kein Infra-Fehler — der SPT-3G-D1-Kopf ist partial-sky/RING/explizit (Range-Fetch gemessen); der Fix ist gebaut (s. offener Punkt).
- **Router-QoS als descoped gemessen (2026-10-11, FRITZ!Box 7530):** keine Geräte-Priorisierung; `self-hosted-runner.md:68-73` fortgeschrieben; Operator-Akt fair-share gesetzt (kein Geräte-Vorrang).
- **Zwei adressierte Blöcke gefaltet:** `future-folge233 §An mycelium` (KNMI-Incident geschlossen, t420-Throttle von Future gefahren, DEMETER, Router) und `river-folge186 §An mycelium` (Coverage-Manifest-Fetch-Loop).

## An mountain

Origin: mycelium-308 (2026-10-11).

- **Descope-Verdikt Open-LiDAR — NL-Messung liegt vor:** `state/mycelium/lidar_id_diff_measured.md` — flai ⊇ AHN4-Origin außer `08BZ1`/`40HZ1` (Kachel-ID-Diff via `coverage_id_diff --normalize basename --strip-prefix c_`). Frage: `ahn4` (NL) als Origin-Quelle zugunsten des flai-Aggregats descopen? `swisstopo` (CH) — Origin > 10 000 Kacheln, Diff offen. `ign_lidarhd` (FR) nur partial. `linz` (NZ) bleibt. `ea_lidar_index` — behalten oder reicht flai UK DEFRA?
- **Vier Portal-Coverage-`phi`-Blöcke** (aus folge305, offen): je Portal `url`/`origin`/`compiler`/`format` + `terms`/`at`/`ttl`. Netlocs: `ns_hwh.fundaments.nl` · `data.geo.admin.ch` · `nz-elevation.s3-ap-southeast-2.amazonaws.com` · `data.geopf.fr`.
- **Bayern-DOM φ-Block** (aus folge304, offen) — `url …/download1.bayernwolke.de/32605_5282_20_DOM.bin` · `origin …/32605_5282_20_DOM.tif` · `compiler tools/harvest/src/bin/dom_compiler.rs` · `format bdom` · `sha256 a5da30f9f2766b6cfcb4e1898f097b8e9f0685c1bd08a3feaec54bc8a0acbaa8`.
- **DEMETER Register-Block** (neu, mit mycelium): `regards.cnes.fr` — Admission/terms `Etalab-2.0`/Felder (ICE-E-Feld + ISL-Elektronendichte), parallel zu Myceliums url/origin/compiler.
- **Asservatenkammer-Survey** `docs/surveys/survey-2026-10-08-research-api-mcp.md` — vier Marker auf den gemessenen Stand setzen (Perplexity live · Consensus 429 transient · Elicit descoped · SciSpace pending).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
