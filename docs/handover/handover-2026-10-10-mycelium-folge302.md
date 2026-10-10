<!--
  title: Handover — Mycelium-Folge 302 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. sb_radar_compiler `--emit delay` gebaut (grind-flash, 208-B-Record + Test, cargo check 0/0); ci-check-Shard-Messung (Build-vs-Test ungemessen bis grüner Lauf, Deps cache-geteilt, kein separater Build-Schritt); gaia_rrl RrlRecord Debug-Derive (Test-Kompilfehler latent); LiDAR-Survey-Träger gesetzt.
  class: handover
  date: 2026-10-10
  sha256: dc534d80ef200a3e549b178977d16011ecf9c2b7583ec7f5730133b0a3600d46
  status: live
-->
# Handover — Mycelium-Folge 302 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge301.md` (→ `archiv/`).

**Faltung `## An mycelium`** (mountain-303): der Block ist gefaltet
(SURFRAD/ECAD/DWD/WorldClim/AODN-Compiler + Asservatenkammer-Träger, siehe `## Offen — eigen`);
kein erneuter Falt-Akt. Der Sender entfernt ihn bei seinem nächsten Pass.

## Burn: open 0.0000 · close 0.4137 · cap 0.5 — Grund: dieses Atom 1× line + 1× grind-flash ($0.0760, Radar `--emit delay`) + 1× general (ci-check-Shard-Messung) · deepseek-flash, kein pro/max (gemessen `session_burn`: total $1.6642→$2.0779 über 8 Sessions, mehrere Linien parallel — nur die eigenen Posten sind diesem Atom zurechenbar).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 301) |
| „ist eingegeben gh secret set EARTHDATA_EDL_TOKEN --repo omegaflow/omegaflow" (Repo-Secret gesetzt) | 2026-10-10 | Operator (Session) |
| „bitte archive search all falls nötig rat und max roster" | 2026-10-10 | Operator (Session) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge301.md` §Operator-Wort-Register (via `git show <sha>:archiv/…`) | 2026-10-10 | gefaltet, nicht kopiert |

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

### Worldwide-LiDAR-Quellen-Survey — Träger (Mycelium)
- **Status:** eigen | **Bindung:** eigen · Doc-Trägerschaft
- **Trigger:** nächster begrenzter Dispatch
- **Lage:** (gemessen 2026-10-10) `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md`
  (401 Z., 51 offene Marker, `--orphan-docs`) trägt die globale LiDAR-Deckung; der Riss ist
  korrigiert (die Mycelium-296-Zeile „kein LiDAR registriert" war falsch). Bester nächster Bau
  (Z. 373–378): open-lidar-data-Bucket `?list-type=2` = 999 COPC-Keys, ein Asset-Link HTTP 206.
- **Blockade:** keine.
- **Braucht:** den **einen** nächsten Bau aus der Survey (Z. 373–378): Reader gegen die echte
  COPC-Datei `…/BE/EODaS/LiDAR_DHMV_II-2013-2015/copc/LiDAR_DHMV_2_P1_ATL12104_ES_52500_217000.copc.laz`,
  dann Block + `open-lidar-data-cdn.yml` (weitere Länder über `continuation-token`); GEDI/ICESat-2
  (Earthdata-Login) als `blocked account`-Klasse registrieren.

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

### Planetary Radar — Manifestation (`sb-radar-cdn`)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Lauf `sb-radar-cdn` Abschluss
- **Lage:** (gemessen 2026-10-10) `--emit delay` gebaut + `--ci-mode`-Upload verdrahtet
  (`tools/harvest/src/bin/sb_radar_compiler.rs`; `upload_release`). Emit ohne Query = volle API:
  `origin https://ssd-api.jpl.nasa.gov/sb_radar.api` → **2761** Delay-Records, **1953** gehalten
  (Doppler), **574 296 B**, sha256 `442cf01c1c5af7fc3952fb020086441aac7f623612c836299ab658a18d7d85b9`;
  je Record 208 B (26×f64, Container `SBRD`): `val=value·1e-6 s` (nur `units=="us"`/`value>0`, sonst
  `None`; 0 honored), `epoch=lsk.unix_to_tdb(...)` (TDB s past J2000), `freq/bin_width/phase=0`
  (kein Band), `kernel=inverse-square`, `force=em`, `presence=1.0`; Test
  `a_delay_row_emits_one_wire_record_or_none`. Registriert in `phi/sources.φ` (`format sb_radar`,
  `at earth`, `ttl 604800`, `field sb_radar_delay_s`), `register_sort --write` → canonical; Workflow
  `.github/workflows/sb-radar-cdn.yml` (`# auto-dispatch: manual`).
- **Blockade:** CDN-Asset bis zum Workflow-Lauf (`sb-radar-cdn` nicht dispatchet).
- **Braucht:** `sb-radar-cdn.yml` dispatchen; danach Asset/sha am Release prüfen (einmalig
  `ci_manage view`).
- **Riss:** die API-Zeile trägt das Ziel (`des`) und das Stationspaar (`xmit`/`rcvr`), der
  26-f64-Wire hat keinen Ziel-/Stations-Slot — der Emit deklariert `xmit`/`rcvr` nur auf stderr,
  das Ziel geht in den einen `at`-Frame (hier `at earth` = Empfänger-Weltlinie). Doppler-Vorzeichen/
  `phase`, `sigma`, `bp` bleiben benannte Risspunkte `pending` — nie zwei Skalare in ein `val`.

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
- **Status:** wartend | **Bindung:** eigen · Teile linie:mountain/river
- **Trigger:** grüner `ci-check`-Lauf auf einem green tree
- **Lage:** (gemessen 2026-10-10 via `ci_manage jobs 38048598777` + Actions-API) die drei
  Shard-Läufe (2187–2189) brachen im **ersten** `cargo nextest run --release`-Step beim
  Kompilieren ab (`-D warnings` auf rotem Baum) → **0 s Test-Zeit**; der Build-vs-Test-Anteil
  ist **ungemessen**, bis ein grüner `ci-check`-Lauf existiert. Strukturell gemessen: kein
  separater `cargo build`-Step (der erste nextest-Step *ist* der Build); `rust-cache`-Key
  `v0-rust-test-Linux-arm64-…` trägt **keine** Shard-Nummer → alle vier Shards teilen einen
  Cache (voll hit, ~117 MB); Runner-Overhead 16–34 s/Shard (shard-1 apt ~24 s). Maßnahme 3
  (4-Shard-nextest) in `ci-check.yml:47-49`; kein `cargo nextest archive`/`--archive-file` im Baum.
- **Blockade:** der Shard-Build-vs-Test-Anteil ist erst an einem green ci-check messbar.
- **Braucht:** (a) grünen `ci-check`-Lauf abwarten, dann `ci_manage jobs <id>` —
  Build- vs. Test-Anteil je Shard; (b) dominiert der Build: **ein** Dispatch
  `cargo nextest archive` + vier Shards per `--archive-file` (Cache-Key shard-blind gegenprüfen);
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

Origin: mycelium-302 (2026-10-10).

- **KNMI Open Data API-Key** (`state/mail/mail_ledger.φ:270`, `opendata@knmi.nl`) —
  **Rotation nötig:** beim Zitieren der Ledger-Zeile wurde der Key-**Wert** in den
  Modell-Transcript gelesen (Secret-Hygiene-Incident, diese Session). Der Wert gilt damit
  als exponiert; Rotation = Operator-Hand, danach Hinterlegung in `.secrets.local`.


## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
