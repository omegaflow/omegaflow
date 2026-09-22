<!--
  title: Handover — Mycelium-Folge 138 (Planungs-Pass: dropped-Prüfung nachgeholt, RAWACF + dirty tree + --dropped-Drift nachgetragen) (Stand 2026-09-22)
  session: Mycelium-Folge 138
  class: handover
  date: 2026-09-22
  sha256: 1697a91023b1cf09e47fec990785988e8b89afff55b77fdc1a992e14a378cbca
  status: live
-->
# Handover — Mycelium-Folge 138 (2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt **aufgeschlüsselt**: **Lage** / **Blockade** / **Braucht**;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session ist der **Planungs-Pass** der Mycelium-Linie. Sie hat die Vorgängerin
`handover-2026-09-22-mycelium-folge137.md` konsumiert und **gegen den Baum gehalten**:
drei Befunde kamen hinzu (dropped-Prüfung, eigener dirty tree, `--dropped`-Drift).

## Stehender Pass (gemessen 2026-09-22)

- **HEAD** beim Start `d2961cb5` (future-folge91). Folge137 wurde bei `e10c6dd3`
  geschrieben und in `6936e640` committet.
- **`git_safety --snapshot`** → `refs/safety/1790083680`.
- **Postfach** (`state/mail/mail_ledger.φ`): `post.md` trägt **1** Zeile —
  `An mountain` (Such-API-Modi, Tavily/Exa/Linkup). **Keine** mycelium-Zeile offen;
  die eigenen aus Folge137 sind gefaltet.
- **CI** (`/tmp/opencode/ci_status.md`, 2026-09-22T14:35): `ci-check 35725878268`
  + `health-check 35718663622` **in_progress**; **13 failed** (viele `ci-check`,
  `hyperscanning-te 35683686776`, `camargo-uranus-cdn 35709963153`,
  `quake-feeds-cdn 35667346490`). Detail bei Bedarf `ci_manage view <id>`.
- **`register_lookup --open`**: 116 Docs, 581 offene Zeilen; mycelium-getaggt
  (`[ernte]`): `blocked_sources` 6, `ledger` 3, `index` 31, `sources` 2,
  `witnesses` 4, `footprints` 2, `nrs` 1; 158 Kandidaten (6 → mycelium).
- **`register_lookup --dropped`** — **Flag existiert nicht mehr.** Der installierte
  `register_lookup` (Binary `4285b09f`, „absent from the tools-latest manifest")
  kennt nur `--open`, `--history`, `<term>` (`--help` gemessen). Die Folge137-Baseline
  „2665" ist am aktuellen Werkzeug unmesbar; `docs/concepts/tools-map.md:233` und
  `_template.md` zitieren den toten Flag. Ersatz ist `--history` (kein Diff-Gate).
- **`register_lookup --history`**: 7969 Treffer in Archiv- + gelöschten Docs.
- **`open_points_check docs/handover/handover-2026-09-22-mycelium-folge137.md`**:
  23 Pfad-Refs, **1 absent** — `tools/register/src/bin/` (Z. 243, Brace-Glob der
  *fremd*-Liste, kein realer Pfad). Kein stale Punkt.
- **`git status`**: 6 **eigene** Dateien modifiziert, **uncommittet** (siehe Punkt
  „Eigener dirty tree").
- **Code gegen den Baum gemessen** (die Folge137-Claims halten): `quaoar_occlt.rs:131`
  `date_midnight_unix` + Test `:403` (`20111301 → None`); `babamul.rs:12-17`
  `enum BabamulParse { Alerts, Empty, NotJson, NoData, Unplaced }`; `demeter.rs:61`
  `ISL SURVEY|ISL BURST`.

## Offen (aufgeschlüsselt)

### Eigener dirty tree (Folge137-Arbeitsbaum)
- **Status:** offen | **Bindung:** eigen
- **Lage:** `git status` zeigt 6 modifizierte, uncommittete Dateien, alle
  mycelium-eigen: `docs/handover/handover-2026-09-22-mycelium-folge137.md`
  (Header-sha `76e65e54` → `3a4f56d3`), `docs/handover/post.md`,
  `phi/blocked_sources.φ`, `phi/footprints.φ`, `phi/pipeline/ledger.φ`,
  `phi/sources.φ`. `6936e640` hatte dieselben Dateien bereits committet; die
  Arbeitsbaum-Version ist die jüngere (Babamul vierwertig, RadNet 157180 records,
  PS1 final-combine, Entfernung der PII-gelöschten `die-vier-schilde.md`/
  `survey-funding-*` aus der Fremd-Liste).
- **Blockade:** keine.
- **Braucht:** im Ausführungs-Pass Autorschaft prüfen (eigene Folge137-Hunks, nicht
  fremd) und die eigenen Hunks pfad-begrenzt committen; danach Folge137 → `archiv/`
  (Move atomar mit dem Commit).

### RAWACF-Bau (wiederhergestellt)
- **Status:** offen | **Bindung:** eigen
- **Lage:** Folge136 nannte RAWACF als „benannter späterer Punkt"; Folge137 trägt
  nur den MAP-Bau, RAWACF fiel heraus (dropped). Globus-Gruppe `rawacf` ist gewährt
  (Mail `1790021001`/`1790020962`), `chroot/sddata/`; FITACF bereits gebaut
  (`sources.φ:9465`).
- **Blockade:** keine.
- **Braucht:** nach MAP-Abschluss RAWACF-Compiler bauen + in `phi/sources.φ`
  registrieren (CDN-Manifestation).

### `--dropped`-Drift / `register_lookup`-Binary
- **Status:** offen | **Bindung:** linie:mountain
- **Lage:** `register_lookup --dropped` existiert am installierten Binary nicht
  (nur `--open`/`--history`/`<term>`); die Folge137-Baseline 2665 ist unmesbar.
  `docs/concepts/tools-map.md:233` und `docs/handover/_template.md:58` zitieren den
  toten Flag. Binary `4285b09f` ist nicht im tools-latest-Manifest.
- **Blockade:** mountain (Register-Werkzeug + tools-map).
- **Braucht:** `--dropped` in `--history` überführen (oder Flag wiederherstellen);
  tools-map/`_template.md` angleichen; Binary-Abgleich gegen tools-latest.

### Babamul CDN-Manifestation
- **Status:** offen | **Bindung:** eigen
- **Lage:** Compiler gebaut (`Empty`→exit 0, Regression→exit 2, `babamul.rs:12-17`);
  CDN `babamul_alerts.bin` 404; `sources.φ:14016`, `ledger.φ:18`.
- **Blockade:** keine.
- **Braucht:** `gh workflow run babamul-cdn.yml` am grünen HEAD, dann sha256 in
  `sources.φ:14016` registrieren.

### solar-system-open-data Key
- **Status:** offen | **Bindung:** eigen
- **Lage:** Key `SOLAR_SYSTEM_OPEN_DATA_KEY` liegt in `.secrets.local`
  (Future-Folge 89); `blocked_sources.φ:47` (`blocked key`).
- **Blockade:** keine.
- **Braucht:** REST `https://api.le-systeme-solaire.net/rest/bodies/` mit
  `Authorization: Bearer` testen, dann Quell-Zeile in `phi/sources.φ` setzen und
  die `blocked key`-Zeile auflösen.

### Free-Model-Bench
- **Status:** offen | **Bindung:** eigen
- **Lage:** kein Ergebnis (`free-model-bench.tsv` existiert nicht); Katalog
  `tools/measure/free_models.tsv` 104 Zeilen, `:72` = `google gemini-2.5-flash`.
- **Blockade:** keine.
- **Braucht:** `gh workflow run free-model-bench.yml -f model=gemini-2.5-flash`.

### Katalog-Inventare (der Wald)
- **Status:** offen | **Bindung:** eigen
- **Lage:** `index.φ` 31 offene Inventar-Zeilen (`tap_index_*`, `erddap_*`,
  `dataverse_*`, `oai_arxiv`, `b2find_*`, `grind_*`, `heliocloud`, `terrapulse`,
  `esa_geomagnetic`, `archeology_gaps`, `arcgis`, `copernicus`, `provider_links`,
  `rpw_datacenter`), 1,32 Mio Zeilen.
- **Blockade:** keine.
- **Braucht:** grind (Blöcke extrahieren).

### Katalog-Lizenz pending
- **Status:** offen | **Bindung:** eigen
- **Lage:** 7 `pending` in `korpora_heim.φ` (dataone terms 401, bcodmo +
  erddap_bcodmo Policy 404, bodc Policy 404, esa_eogateway JS ohne Klausel,
  gfz_igets Terms nicht auffindbar, gaia_swpc gemischt ESA+PD).
- **Blockade:** 404/401 bzw. gemischter Provider.
- **Braucht:** Einzelmessung (dataone Auth, BCO-DMO/BODC-Policy, GFZ-DOIDB,
  Gaia/SWPC trennen).

### pre-cdn Pools
- **Status:** offen | **Bindung:** eigen
- **Lage:** `phi/pipeline/queue/sources_potential_pre-cdn_9k_richest.φ` (824 Zeilen)
  + `..._params.φ` (63 Zeilen), am Datenträger, gitignored.
- **Blockade:** keine.
- **Braucht:** grind/port (Join-Quelle der Lost-Blocks).

### Gaia-DR3-Alerts
- **Status:** offen | **Bindung:** eigen
- **Lage:** `witnesses.φ:61` pending — Gaia-DR3-Alerts-Index (2026-09-13 HTTP 200),
  je Alert RA/Dec + G-Band; S²-Richtung, τ=0; kompiliert.
- **Blockade:** keine.
- **Braucht:** Alerts-Kanal in die Zeugen-Flotte verdrahten.

### NOAA-S3 noaa-goes16 L1B
- **Status:** offen | **Bindung:** eigen
- **Lage:** `sources.φ:8040` — im offenen Bucket nur `GLM-L2-LCFA/`, kein L1B;
  L1B-Origin bleibt GHRC-EDL-protected; token-freie L2-Route gebaut
  (`glm_l2_compiler.rs`, `sources.φ:8042`).
- **Blockade:** L1B nur hinter EDL.
- **Braucht:** L1B-Route messen oder mit Messung descopen.

### CI format-Job (cross-line)
- **Status:** offen | **Bindung:** linie
- **Lage:** `ci-check` `format` rot: mycelium-Dateien `src/archivar/fai_kz.rs:47`,
  `src/archivar/ia2_tap.rs:96`; ferner `hdf5.rs`, `te.rs`, `tests.rs`,
  `hyperscanning_group_te.rs` (mountain/sensory).
- **Blockade:** `cargo fmt` ist lokal strukturell verweigert (nur pfad-begrenzt
  erlaubt seit River-Folge7; CI-`--check` bleibt das Netz).
- **Braucht:** je Linie die eigenen Dateien formatieren (mycelium: `fai_kz.rs`,
  `ia2_tap.rs`).

### EPA RadNet AGOL
- **Status:** wartend | **Bindung:** eigen
- **Lage:** CI `35649300152` success (157180 records, 32693448 B, unjoined 153,
  agol 140 bad 0); `blocked_sources.φ:56`, `sources.φ:1164`.
- **Blockade:** RadNet-Features tragen keine Koordinaten; FRS-Weg geschlossen
  (`get_program_list` 500 ORDS, RADNET/ERM nicht in FRS_PROGRAM_FACILITY).
- **Braucht:** Koordinaten-Route (Site/County-Join) oder descope mit Messung.

### DEMETER ISL Download
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Parser-Gate gebaut (`demeter.rs:61` akzeptiert `ISL SURVEY|ISL BURST`);
  Order 18387 (34,71 GB) läuft, Download `availableFilesCount=0`, ZIP 204;
  `blocked_sources.φ:52`.
- **Blockade:** CDPP stellt die Dateien nicht bereit.
- **Braucht:** Download bei `availableFilesCount>0`.

### SuperDARN MAP
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Globus-Gruppen `rawacf`/`fitacf_30`/`fitacf_25`/`MAP` gewährt;
  Transfer-Task `af68c4f1-b601-11f1-b9a2-0affd5e180af` ACTIVE (6.561 Dateien,
  21,93 GB) → `data/superdarn/map/`; `blocked_sources.φ:16`. FITACF bereits
  gebaut (`sources.φ:9465`).
- **Blockade:** Transfer läuft im Hintergrund.
- **Braucht:** bei Transfer-Abschluss MAP-Compiler bauen + in `phi/sources.φ`
  registrieren (CDN-Manifestation; 2-GB-Grenze → ein kompiliertes Record).

### PS1-Footprint
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `footprints.φ:19` — Release 392860039 assets `[]`,
  `ps1_dr2_coverage.fp01` absent; Lauf `35670499832` success, aber kein
  final-combine/upload (Harvest endet Band 655).
- **Blockade:** final-combine nicht erreicht (Bänder 651–2643 offen).
- **Braucht:** final-combine-Lauf auf dem Slab-Tag.

### NRS SHAPE
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `nrs_stations.φ:17` — NRS02-10,12,13 ohne Spektren-Verankerung;
  Bucket `noaa-passive-bioacoustic` `nrs/products` trägt SHAPE.
- **Blockade:** SHAPE-Format-Anker.
- **Braucht:** SHAPE-Spektren parsen/ankern.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin (Dienst)
- **Lage:** `ledger.φ:10-12`; `/tap` HTTP 200, `/tap/tables` HTTP 500 (PostgreSQL
  localhost:5432 refused).
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei `/tap/tables` 200.

### SSDC Limadou
- **Status:** wartend | **Bindung:** termin (PI)
- **Lage:** `ledger.φ:14-16`; Portal 200, CAS-Login funktioniert, „Permission
  Denied" für omegaflow; PI Sotgiu: CSES-02-Umbau.
- **Blockade:** PI-Portal.
- **Braucht:** neue Anleitung auf dem Limadou-Portal.

### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Lage:** `blocked_sources.φ:3` — `api.lasair.lsst.ac.uk/api` 502 über alle
  Proton-Exits, Frontend 200, Backend down; Token vorhanden.
- **Blockade:** Broker-Backend.
- **Braucht:** Re-Messung (Wiedervorlage).

### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Lage:** `blocked_sources.φ:21` — `bc_mpo_more` release_date 2099-01-01,
  `data?PRODUCT` 403, kein Konto-Gate; Freigabe-Anfrage an `psahelp`.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp` (Anfrage ist eine `future`-Konsequenz).

### EMODNET HFRADAR NADR
- **Status:** wartend | **Bindung:** termin 2026-10-19
- **Lage:** Re-Messung fällig 2026-10-19.
- **Blockade:** Termin.
- **Braucht:** Re-Messung.

## Benchmark

- Kein Doppel-Lauf: die Routine-Klasse ist geschlossen (flash-Sieger, 2026-09-16) —
  zitiert. Diese Session lief ohne Sub-Agenten (Planungs-Pass, read-only); kein
  neuer Sieger.

## Geteilter Baum — eigener Pfad-Satz

- **Diese Session:** neues Handover `docs/handover/handover-2026-09-22-mycelium-folge138.md`.
  Move `handover-2026-09-22-mycelium-folge137.md` → `archiv/` steht mit dem Commit an
  (Folge137 ist die konsumierte Vorgängerin; ihr Arbeitsbaum-Stand ist die jüngere
  Version — siehe Punkt „Eigener dirty tree").
- **Noch offen aus Folge137 (nicht angetastet):** `docs/handover/post.md`,
  `phi/blocked_sources.φ`, `phi/footprints.φ`, `phi/pipeline/ledger.φ`,
  `phi/sources.φ` — eigene uncommittete Hunks, im Ausführungs-Pass zu schließen.
- **Fremd (nicht angetastet):** `AGENTS.md`, `src/gate/commit_gate.rs`,
  `src/gate/commit_gate_vocab.json`,
  `tools/register/src/bin/` (number_audit, open_points_check, path_reference_scan),
  diverse `archiv/`-Handover.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
