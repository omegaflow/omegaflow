<!--
  title: Handover — Mycelium-Folge 301 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. BDOM-Witness registriert (witness.rs), dom-cdn.yml + fünf Klima-Manifest-Workflows (surfrad/ecad/dwd-cdc/worldclim/aodn) gebaut; open-lidar-data-cdn-Lauf 38082503772 success (Asset 50 616 273 B, sha256 e6560344·); Stehender Pass am neuen HEAD neu geschrieben.
  class: handover
  date: 2026-10-10
  sha256: f908119797d9ddb2952fee5cede551abc424fce27b30971d2de20c8c2d0c88b8
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

## Burn: open 0.0000 · close 0.2213 · cap 0.5 — Grund: dieses Atom 1× line ($0.1940, „Mycelium-Linie in einem Pass starten") + 1× council ($0.0273) · deepseek-flash, kein pro/max (gemessen `session_burn`; der research-general und die UI-Runde tragen eigene Posten).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 301) |
| „ist eingegeben gh secret set EARTHDATA_EDL_TOKEN --repo omegaflow/omegaflow" (Repo-Secret gesetzt) | 2026-10-10 | Operator (Session) |
| „bitte archive search all falls nötig rat und max roster" | 2026-10-10 | Operator (Session) |
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

### Planetary Radar — Ephemeris-Block (Rat-Verdikt: getrennte Records)
- **Status:** eigen | **Bindung:** eigen (Manifestation)
- **Trigger:** nächster begrenzter Dispatch
- **Lage:** (gemessen 2026-10-10) Origin `https://ssd-api.jpl.nasa.gov/sb_radar.api`
  HTTP 200, 384 112 B, sha256 `0232b4c5…`; Doc v1.1.
  `tools/harvest/src/bin/sb_radar_compiler.rs` gebaut (`--inspect`), schreibt **kein**
  Wire-Record; `phi/sources.φ` trägt **keinen** `sb_radar`-Eintrag.
- **Blockade:** keine — das Feld ist entschieden (Rat 2026-10-10, durch UI-Roster bestätigt).
- **Braucht:** **ein** Dispatch: `sb_radar_compiler` um `--emit delay` erweitern, das **genau
  einen** 208-Byte-Record je Delay-Zeile schreibt (`val` SI-Sekunden, `epoch` UNIX/J2000,
  `freq/bin_width = (0, ·)`, Anker `at <zielplanet>` [Archivar-`parse.rs:298`], **Transmitter
  UND Receiver** deklariert [bistatisch], Epoch-Konvention + COM/Oberfläche), plus
  `#[test]` mit Fixture. Doppler-Vorzeichen/`phase`, `sigma`, `bp` (Peak vs. COM) bleiben
  **benannte Risspunkte** `pending` — nie zwei Skalare in ein `val`.

### Carriership — Asservatenkammer-Survey (Trägerschaft)
- **Status:** eigen | **Bindung:** eigen · Doc = Mountain-Trägerschaft
- **Trigger:** Doc-Carrier-Zensus
- **Lage:** (gemessen 2026-10-10) `docs/surveys/survey-2026-10-08-research-api-mcp.md`
  trägt die vier Kandidaten; Arme am Binary gemessen: `--perplexity` HTTP 200 (echter
  Text), `--consensus` Arm+Key vorhanden **HTTP 429** (Rate-Limit, transient), Elicit
  `descoped` (kommerziell), SciSpace kein Arm (`pending`, cookie-API 403).
- **Blockade:** das Survey ist Mountain-Trägerschaft — kein stilles Überschreiben.
- **Braucht:** `## An mountain` — die vier Marker auf den gemessenen Stand setzen.

### Architektur — GitHub/CI/CDN-Optimierung (Rat-Verdikt: Messung, dann Archive)
- **Status:** eigen | **Bindung:** eigen · Teile linie:mountain/river
- **Trigger:** nächster `ci-check`-Lauf
- **Lage:** (gemessen 2026-10-10) Survey
  `docs/surveys/survey-2026-10-10-github-ci-cdn-optimierung.md`; Maßnahme 1–5 gebaut,
  Maßnahme 3 (4-fach-nextest-Shard) in `ci-check.yml:47-49` verdrahtet — **jeder Shard baut
  `--release` neu**, kein `cargo nextest archive`/`--archive-file` im Baum; Shard-Wall-Clock
  ungemessen.
- **Blockade:** keine.
- **Braucht:** (a) **erst messen** — `ci_manage jobs <ci-check-run-id>` einmal lesen,
  Build- vs. Test-Anteil je Shard (Archive-Upload/Download in die Messung, Claude);
  (b) dominiert der Build: **ein** Dispatch `cargo nextest archive` einmal bauen + vier Shards
  per `--archive-file` (warm cache sccache/rust-cache gegenprüfen; Doctests eigener Schritt);
  (c) Test-Suite-Dedup als **zweite, getrennt eingeführte** Achse.

### Speicher — 1,76 TB Bulk vs. R2-10-GB (Rat-Verdikt: messen vor bauen)
- **Status:** pending | **Bindung:** eigen
- **Trigger:** gemessener R2-Konsument ODER größte Datei nähert sich 2 GiB ODER Releases-Lesezugriffe werden zum Engpass (Claude-Verschärfung)
- **Lage:** (gemessen 2026-10-10 via GitHub-API) `omegaflow/sources` = 344 Releases /
  1763 GB → $0; R2 10 GB frei, Zero-Egress. **Kein gemessener Live-Konsument.**
- **Blockade:** keine — das Rat-Verdikt (2026-10-10, UI-bestätigt) ist: kein Tier ohne Hyphe.
- **Braucht:** **kein Bau.** Ein Schritt: den **einen** manifestierenden Konsumenten messen
  (welcher gebaute Pfad liest ein `kernel-flatten`-Manifest?) und Größe/Zugriffsfrequenz gegen
  das 10-GB-Budget halten → Konsument benannt ⇒ R2-Hot-Tier `pending` mit Trigger; kein
  Konsument ⇒ `descoped` mit Befund. ToS-Risiko ⇒ Portabilität (content-addressed Keys), kein
  Sofortbau; R2-Bucket/Key = Operator-Akt (per-Akt-Wort).

### MCP — lokale no-leak-Server (Rat-Verdikt: Regel vor Knoten)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** gemessener Session-Bedarf (`sread`/`sgrep` reichen nicht)
- **Lage:** (gemessen 2026-10-10) Survey Säule F; `opencode.json:439-451` läuft bereits zwei
  lokale stdio-MCPs (`chrome-devtools`), aber mit `--autoConnect`/`--allowedUrlPattern https://*`/
  `npx -y` — **kein no-leak-Präzedenzfall** (Rat + Roster 2026-10-10).
- **Blockade:** keine.
- **Braucht:** **die Regel, nicht sofort einen Server** — eine `mcp`-`local`-Aufnahme nur für
  die no-leak-Klasse: exakter Versions-**Integrity-Pin** (Hash/Lockfile, nicht Tag),
  `command`/`args` nur aus der getrackten Config, Repo-Wurzel-Scope, **erzwungener** Egress-Deny
  (Netz-Namespace/Container ohne Route) + FS-Scope mit Testfall, kein Netz-Arm, kein Cloud-MCP.
  Der konkrete Server folgt erst einem gemessenen Bedarf.

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

- **VNP46A3 φ-Block** — nach dem Repo-Secret-Update ist der Lauf `vnp46a3-cdn 38084502714`
  **success** (der 401 war der alte Repo-Secret-Wert). Mycelium-Direktive gemessen
  (`ci_manage log 38084502714 --all`), + Mountain-Verdikt (`terms`/`at`/`ttl`) in **einem**
  Block:
  ```
  url https://github.com/omegaflow/sources/releases/download/data.laadsdaac.earthdatacloud.nasa.gov/vnp46a3_allangle_composite_snow_free.bin
  origin https://data.laadsdaac.earthdatacloud.nasa.gov/prod-lads/VNP46A3/VNP46A3.A2026213.h18v07.002.2026252141449.h5
  compiler tools/harvest/src/bin/vnp46a3_compiler.rs
  format black_marble_vnp46a3_nightlight
  sha256 d85ee99b9d967b5565bb79e145d9120ab09205eb41d4188b3f016e8944b94510
  ```
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


## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
