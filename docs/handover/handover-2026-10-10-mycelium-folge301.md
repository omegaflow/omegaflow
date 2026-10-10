<!--
  title: Handover — Mycelium-Folge 301 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. BDOM-Witness registriert (witness.rs), dom-cdn.yml + fünf Klima-Manifest-Workflows (surfrad/ecad/dwd-cdc/worldclim/aodn) gebaut; open-lidar-data-cdn-Lauf 38082503772 success (Asset 50 616 273 B, sha256 e6560344·); Stehender Pass am neuen HEAD neu geschrieben.
  class: handover
  date: 2026-10-10
  sha256: 6b444d1fb2d3f271b88ac6ae832d81fe9b58274454b3475f1c95f99651df9168
  status: live
-->
# Handover — Mycelium-Folge 301 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge300.md` (→ `archiv/`).

**Faltung `## An mycelium`** (mountain-303): der Block ist in folge300 gefaltet
(SURFRAD/ECAD/DWD/WorldClim/AODN-Compiler + Asservatenkammer-Träger); kein
erneuter Falt-Akt. Der Sender entfernt ihn bei seinem nächsten Pass.

## Burn: open 0.0000 · close 0.3084 · cap 0.5 — Grund: dieses Atom 1× line + 1× grind-flash · deepseek-flash, kein pro/max (gemessen `session_burn` total $2.3494→$2.6578).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 301) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge300.md` §Operator-Wort-Register (via `git show <sha>:archiv/…`) | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — SPT-3G D1 `cmap` (cmb-cdn)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Lauf `cmb-cdn` Abschluss am neuen HEAD
- **Lage:** (gemessen 2026-10-10 via `ci_manage view 38080553796`) der Re-Lauf ist
  weiterhin **pending** (head `23fe42a1a`, seit 19:37:45Z, noch kein Job gestartet).
  Die Wurzel ist gebaut: `cmb_planck_compiler --tar <lokale.tar.bz2>`, Workflow lädt
  den Tarball via `curl -C -` in den persistenten Pfad (überlebt den Runner-Stop).
- **Blockade:** der Lauf wartet in der Warteschlange.
- **Braucht:** bei success den sha256 in den SPT-`cmap`-Block (`cmb_spt_d1_n64.json`).

### CI — VNP46A3-CDN LAADS-Auth (Ursache gemessen: Repo-Secret veraltet)
- **Status:** wartend | **Bindung:** eigen (Manifestation) · Repo-Secret = Operator-Hand
- **Trigger:** `vnp46a3-cdn`-Re-Dispatch nach Repo-Secret-Update
- **Lage:** (gemessen 2026-10-10) der LAADS-Granule-Fetch des Laufs am 2026-10-10T19:37Z
  antwortet **401** (`ci_manage log 38080557462`) mit vorhandenem `EARTHDATA_EDL_TOKEN` und
  gebautem `Authorization: Bearer`-Header. **Ursache gemessen:** das GitHub-Repo-Secret
  `EARTHDATA_EDL_TOKEN` (omegaflow/omegaflow) steht auf **2026-10-06T08:34:06Z**
  (`gh secret list --repo omegaflow/omegaflow`), nicht auf dem heute Mittag erneuerten
  Wert — der Workflow liest `${{ secrets.EARTHDATA_EDL_TOKEN }}`, nicht die lokale Datei.
  Der 401 ist der **alte** Token, kein fehlender.
- **Blockade:** das erneuerte Token wurde im Repo-Secret noch nicht hinterlegt.
- **Braucht:** das GitHub-Secret `EARTHDATA_EDL_TOKEN` (Repo omegaflow/omegaflow) auf den
  erneuerten Wert setzen (Operator-Hand; 16 Workflows lesen es), dann Re-Dispatch.

### φ-Manifestation — open-lidar-data (COPC) — Trigger gefeuert
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** (gefeuert) Lauf `open-lidar-data-cdn 38082503772` **success** am 2026-10-10
- **Lage:** (gemessen via `ci_manage log 38082503772 --all`) Asset
  `open_lidar_data_be_dhmv2.bin`, **50 616 273 B**, sha256
  `e6560344c5e939458c890e62d5972dbfb273f8aa2cdd6e00bdb6ec2feef09c4e`; 1 446 179
  COPC-Punkte geframed; Mycelium-Direktive gemessen (url/origin/compiler/format las).
- **Blockade:** `register_sort` verlangt eine `ttl`-Zeile (gemessen:
  `register_sort: block has no ttl line`) — der Block ist ohne Mountain-Verdikt
  strukturell unvollständig; kein stiller Schreibakt.
- **Braucht:** `## An mountain` — Mycelium-Direktive + Mountain-Verdikt
  (`terms`/`at`/`ttl`) in **einem** Block.

### Bayern-DOM (`geodaten.bayern.de`) — Witness + Workflow gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `dom-cdn` Abschluss
- **Lage:** (gemessen 2026-10-10) `BDOM` als Gestalt-Witness registriert
  (`src/archivar/witness.rs:24`, Test `bdom_is_gestalt`, `cargo check` 0/0).
  `.github/workflows/dom-cdn.yml` gebaut (`# auto-dispatch: manual`). COG-Kachel-URL
  `https://download1.bayernwolke.de/a/dom20/DOM/32605_5282_20_DOM.tif` gemessen
  (`--verdict` 200, `--sniff` tiff, 48 788 700 B, sha256 `a35e8d86…52945c3`).
- **Blockade:** kein CDN-Asset bis zum Lauf.
- **Braucht:** `dom-cdn.yml` am neuen HEAD dispatchen; danach sha256 + φ-Block
  (Mountain-Verdikt, `## An mountain`).

### φ-Manifestation — SURFRAD · ECAD · DWD CDC · WorldClim · AODN
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** je Workflow-Lauf-Abschluss (`surfrad-cdn`, `ecad-cdn`, `dwd-cdc-cdn`, `worldclim-cdn`, `aodn-cdn`)
- **Lage:** (gemessen 2026-10-10) fünf Manifest-Workflows gebaut, je `--ci-mode` und
  `# auto-dispatch: manual`: `surfrad-cdn.yml` (NETLOC `gml.noaa.gov`) · `ecad-cdn.yml`
  (`ecad.eu`) · `dwd-cdc-cdn.yml` (`opendata.dwd.de`) · `worldclim-cdn.yml`
  (`geodata.ucdavis.edu`) · `aodn-cdn.yml` (`thredds.aodn.org.au`). **Riss:** der
  `ecad_compiler` NETLOC ist `ecad.eu`, **nicht** der S3-Quellhost
  `knmi-ecad-assets-prd.s3.amazonaws.com` — die φ-Identität (`url`/`origin`) ist
  Mountain-Sache.
- **Blockade:** je Asset fehlt (Workflow nicht gelaufen) + Mountain-Verdikt.
- **Braucht:** je Workflow dispatchen; danach je φ-Block (Mycelium
  `url`/`origin`/`compiler`, Mountain `terms`/`at`/`ttl`).

### Planetary Radar — Ephemeris-Block (Compiler gebaut)
- **Status:** eigen | **Bindung:** eigen (Manifestation)
- **Trigger:** Rat-Entscheidung Feld/`at`, dann Lauf
- **Lage:** (gemessen 2026-10-10) Origin `https://ssd-api.jpl.nasa.gov/sb_radar.api`
  HTTP 200, 384 112 B, sha256 `0232b4c5…`; Doc v1.1.
  `tools/harvest/src/bin/sb_radar_compiler.rs` gebaut (`--inspect`), schreibt **kein**
  Wire-Record — die physikalische Abbildung (radar delay/doppler → 26×f64) ist `pending`.
- **Blockade:** das Feld ist nicht entschieden — Architektur, gehört durch die fünf Stimmen.
- **Braucht:** Rat-Entscheidung Feld + `at`-Anker (`parse.rs:298`, `at <zielplanet>`);
  dann Compiler-Lauf + Block.

### Carriership — Asservatenkammer-Survey (Trägerschaft)
- **Status:** eigen | **Bindung:** eigen · Doc = Mountain-Trägerschaft
- **Trigger:** Doc-Carrier-Zensus
- **Lage:** (gemessen 2026-10-10) `docs/surveys/survey-2026-10-08-research-api-mcp.md`
  trägt die vier Kandidaten; Arme am Binary gemessen: `--perplexity` HTTP 200 (echter
  Text), `--consensus` Arm+Key vorhanden **HTTP 429** (Rate-Limit, transient), Elicit
  `descoped` (kommerziell), SciSpace kein Arm (`pending`, cookie-API 403).
- **Blockade:** das Survey ist Mountain-Trägerschaft — kein stilles Überschreiben.
- **Braucht:** `## An mountain` — die vier Marker auf den gemessenen Stand setzen.

### Architektur — GitHub/CI/CDN-Optimierung (Survey + Rat)
- **Status:** eigen | **Bindung:** eigen · Teile linie:mountain/river
- **Trigger:** Rat Runde 2 / nächster `ci-check`-Lauf
- **Lage:** (gemessen 2026-10-10) Survey
  `docs/surveys/survey-2026-10-10-github-ci-cdn-optimierung.md`; Maßnahme 1–5 gebaut,
  Maßnahme 3 (4-fach-nextest-Shard) in `ci-check.yml` verdrahtet; Shard-Wall-Clock noch
  nicht am Lauf gemessen.
- **Blockade:** keine.
- **Braucht:** (a) Shard-Wall-Clock aus dem nächsten `ci-check`-Log bestätigen;
  (b) Rat Runde 2: Test-Suite-Dedup als eigener begrenzter Dispatch.

### Speicher — 1,76 TB Bulk vs. R2-10-GB
- **Status:** eigen (Architektur) | **Bindung:** eigen
- **Trigger:** Entlastung/Rebalancing nötig
- **Lage:** (gemessen 2026-10-10 via GitHub-API) `omegaflow/sources` = 344 Releases /
  1763 GB → $0; R2 10 GB frei trägt Manifeste/Indizes.
- **Blockade:** keine.
- **Braucht:** Architektur-Wort (Survey-Säule B), ob R2 als Hot-Tier kommt.

### MCP — lokale no-leak-Server
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Architektur-Wort
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
- **Blockade:** der ~5 h `rotor slice` wird durch den Runner-Stop präemptiert.
- **Braucht:** gecheckpointete kürzere Slices (State alle 120 s liegt vor) oder
  dauerhafter Runner; Survey-Säule A.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** `hips-png-cdn`-Lauf Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) `37932098229` und
  `38043712533` in_progress; `phi/pipeline/ledger.φ:110` `ausstehend`.
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

## An mountain

Origin: mycelium-301 (2026-10-10).

- **open-lidar-data φ-Block** — Mycelium-Direktive (gemessen, `ci_manage log
  38082503772 --all`) + Mountain-Verdikt (`terms`/`at`/`ttl`) in **einem** Block, da
  `register_sort` eine `ttl`-Zeile verlangt:
  ```
  url https://github.com/omegaflow/sources/releases/download/open-lidar-data.s3.amazonaws.com/open_lidar_data_be_dhmv2.bin
  origin https://open-lidar-data.s3.amazonaws.com/data/BE/EODaS/LiDAR_DHMV_II-2013-2015/copc/
  compiler tools/harvest/src/bin/open_lidar_data_compiler.rs
  format las
  sha256 e6560344c5e939458c890e62d5972dbfb273f8aa2cdd6e00bdb6ec2feef09c4e
  ```
- **Fünf Klima-Quellen** — die Mycelium-NETLOC-Direktiven stehen (Workflows gebaut);
  der `ecad`-Riss (`ecad.eu` vs. Quellhost `knmi-ecad-assets-prd.s3.amazonaws.com`) ist
  zu entscheiden. Verdikt-Zeilen je Block nach dem ersten Lauf.
- **Asservatenkammer-Survey** `docs/surveys/survey-2026-10-08-research-api-mcp.md` —
  die vier Marker auf den gemessenen Stand setzen (Perplexity live · Consensus 429
  transient · Elicit descoped · SciSpace pending).

## An future

Origin: mycelium-301 (2026-10-10).

- **KNMI Open Data API-Key angekommen** (`state/mail/mail_ledger.φ:268`, Betreff
  „Your API Key") — der Wert wurde **nicht** ausgelesen; Hinterlegung in
  `.secrets.local` = Operator-Hand.
- **`EARTHDATA_EDL_TOKEN`** — das **Repo-Secret** (omegaflow/omegaflow) trägt noch den
  Wert vom `2026-10-06T08:34Z` (`gh secret list`); das heute erneuerte Token dort
  hinterlegen (Operator-Hand) — 16 Workflows lesen es. Kein erneutes Ausstellen nötig.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
