<!--
  title: Handover — Mycelium-Folge 137 (erste Mycelium-Übergabe; Ernte-Alt-Slug abgelöst; quaoar-Datumsvalidierung + Babamul-0-honored gebaut) (Stand 2026-09-22)
  session: Mycelium-Folge 137
  class: handover
  date: 2026-09-22
  sha256: 3a4f56d3f8643bf3793297d2cf565772b2c8eb3e6ed09b41081f8c68ec918773
  status: live
-->
# Handover — Mycelium-Folge 137 (2026-09-22)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt **aufgeschlüsselt**: **Lage** / **Blockade** / **Braucht**;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Die Mycelium-Linie ist die Nachfolgerin der Ernte-Linie (Operator-Wort 2026-09-21:
„Die Ernte-Linie existiert nicht mehr — Quellen-/Registerarbeit läuft über die
Owner-Tags mycelium/mountain"). Diese ist die **erste** Übergabe unter dem
Mycelium-Slug; die Vorgängerin `handover-2026-09-21-ernte-folge136.md` wurde
konsumiert und ins Archiv gelegt.

## Stehender Pass (gemessen 2026-09-22)

- **HEAD** beim Start `e10c6dd3` (mountain folge131); während der Session zog eine
  fremde Linie nach: `d73fe70c` (PII-Untrack `docs/zustand/external-state.md`).
- **`git_safety --snapshot`** → `refs/safety/1790044611` (Start).
- **Postfach** (`state/mail/mail_ledger.φ`): letzter Eingang `1790043573` (Exa
  API-Key-Welcome, Maschine; Future-Folge 89 hat Tavily/Exa/Linkup bereits in
  `.secrets.local` registriert). `post.md` trug bei Start fünf Zeilen: zwei an
  mycelium (Free-Model-Bench, SuperDARN/Globus), eine an ernte (quaoar-Test),
  `An mountain` (Such-API-Modi), `An mycelium` (solar-system-Key) + `An mycelium`
  (Alt-Slug-Hinweis) — die eigenen gefaltet, die fremde mountain-Zeile belassen.
- **CI** (`/tmp/opencode/ci_status.md` + `ci_manage list`, 2026-09-22): `ci-check`
  `35675988557` **rot** in drei Jobs — `dropped-gate` (Baseline 2647 | current 2665
  | delta 18), `test` (`quaoar_occlt::tests::date_midnight_unix_reads_the_calendar_date`
  rot an `quaoar_occlt.rs:397`), `format` (`cargo fmt --check`: `fai_kz.rs:47`,
  `hdf5.rs:3967,4007`, `ia2_tap.rs:96`, `te.rs:5692…5896`, `tests.rs:641`,
  `hyperscanning_group_te.rs:1624,1645`). `quake-feeds-cdn` `35676543413`
  **success**; `ps1-cdn` `35670499832`, `allwise-cdn` `35671243834`, `ned-cdn`
  `35676548807` success; `te-gate`/`hyperscanning-te`/`ci-check` `35676032092`
  pending/in_progress.
- **`register_lookup --open`**: mycelium-getaggt — `blocked_sources` 5, `ledger`
  3→2 (DEMETER parser-gap gestrichen), `index` 31, `sources` 1, `witnesses` 4,
  `footprints` 2, `nrs` 1.
- **`open_points_check ernte-folge136`**: 10 Pfad-Refs, 0 absent.
- **`register_lookup --dropped --count`**: 2665; die Baseline wurde von Sensory-Folge
  145 bereits auf 2665 nachgezogen (delta 18 aus den Handover-Archivierungen) — kein
  eigener Nachzug nötig.

## Was diese Session tat (ein Atom)

- **quaoar_occlt-Datumsvalidierung gebaut** (grind-pro): `date_midnight_unix`
  (`src/archivar/quaoar_occlt.rs:131`) prüft jetzt Monat (1–12) und Tag gegen die
  Monatslänge inkl. Schaltjahr, bevor `ymd_to_days` gerufen wird — der rote Test
  `quaoar_occlt.rs:393` wird grün; der einzige Produktions-Caller
  (`quaoar_occlt_compiler.rs:53`) überspringt bei `None` ohnehin. `ymd_to_days`
  bleibt unverändert (~20 Caller mit ungeprüften externen Monats-/Tagesfeldern).
- **Babamul-0-honored gebaut** (grind-pro): `src/archivar/babamul.rs` `parse_alerts`
  liefert vierwertig (`Alerts`/`Empty`/`NotJson`/`NoData`/`Unplaced`);
  `babamul_compiler.rs` gibt bei `Empty` (wahrhaftiger 0-Alert) exit 0 aus
  (0 honored, bin ungeschrieben), bei Parser-Regression exit 2. `cargo check -p
  omegaflow-harvest` sauber.
- **Register gemessen und gezogen:** `ledger.φ` Babamul-note auf den gebauten
  Stand + DEMETER-`parser-gap`-Block gestrichen (`demeter.rs:61` akzeptiert
  `ISL SURVEY|ISL BURST`); `blocked_sources.φ` DEMETER-Gate-Note + EPA RadNet
  (`35649300152` success: 157180 records, 32693448 B, unjoined 153, agol 140);
  `sources.φ:14016` Babamul; `footprints.φ:19` PS1 re-gemessen (Release 392860039
  assets [] — Asset absent, Lauf `35670499832` success ohne final-combine).
- **`post.md`:** die zwei eigenen mycelium-Zeilen + die ernte-Zeile gefaltet;
  `An mountain` (Such-API-Modi) wiederhergestellt (versehentlich mitgelöscht).

## Offen (aufgeschlüsselt)

### Babamul CDN-Manifestation
- **Status:** offen | **Bindung:** eigen
- **Lage:** Compiler gebaut (`Empty`→exit 0, Regression→exit 2); CI-Lauf
  `35649296996` war rot (exit 1: 0 honored); CDN `babamul_alerts.bin` 404;
  `sources.φ:14016`, `ledger.φ:18`.
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

### EPA RadNet AGOL
- **Status:** wartend | **Bindung:** eigen
- **Lage:** CI `35649300152` success (157180 records, 32693448 B, unjoined 153,
  agol 140 bad 0); `blocked_sources.φ:56`, `sources.φ:1164`.
- **Blockade:** RadNet-Features tragen keine Koordinaten; FRS-Weg geschlossen.
- **Braucht:** Koordinaten-Route (Site/County-Join) oder descope mit Messung.

### DEMETER ISL Download
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Parser-Gate gebaut (`demeter.rs:61`); Order 18387 (34,71 GB) läuft,
  Download `availableFilesCount=0`, ZIP 204; `blocked_sources.φ:52`.
- **Blockade:** CDPP stellt die Dateien nicht bereit.
- **Braucht:** Download bei `availableFilesCount>0`.

### SuperDARN MAP
- **Status:** offen | **Bindung:** eigen
- **Lage:** Globus-Gruppen `rawacf`/`fitacf_30`/`fitacf_25`/`MAP` gewährt;
  Transfer-Task `af68c4f1-b601-11f1-b9a2-0affd5e180af` ACTIVE (6.561 Dateien,
  21,93 GB) → `data/superdarn/map/`; `blocked_sources.φ:16`. FITACF bereits
  gebaut (`sources.φ:9465`).
- **Blockade:** Transfer läuft im Hintergrund.
- **Braucht:** bei Transfer-Abschluss MAP-Compiler bauen + in `phi/sources.φ`
  registrieren (CDN-Manifestation; 2-GB-Grenze → ein kompiliertes Record).

### NOAA-S3 noaa-goes16 L1B
- **Status:** offen | **Bindung:** eigen
- **Lage:** `sources.φ:8040` — im offenen Bucket nur `GLM-L2-LCFA/`, kein L1B;
  L1B-Origin bleibt GHRC-EDL-protected; token-freie L2-Route gebaut
  (`glm_l2_compiler.rs`, `sources.φ:8042`).
- **Blockade:** L1B nur hinter EDL.
- **Braucht:** L1B-Route messen oder mit Messung descopen.

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

### Gaia-DR3-Alerts
- **Status:** offen | **Bindung:** eigen
- **Lage:** `witnesses.φ:61` pending — Gaia-DR3-Alerts-Index (2026-09-13 HTTP 200),
  je Alert RA/Dec + G-Band; S²-Richtung, τ=0; kompiliert.
- **Blockade:** keine.
- **Braucht:** Alerts-Kanal in die Zeugen-Flotte verdrahten.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin (Dienst)
- **Lage:** `ledger.φ:10-12`; `/tap` HTTP 200, `/tap/tables` HTTP 500 (PostgreSQL
  localhost:5432 refused).
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei `/tap/tables` 200.

### SSDC Limadou
- **Status:** wartend | **Bindung:** termin (PI)
- **Lage:** `ledger.φ:14-16`; Portal 200, CAS-Login funktioniert, „Permission
  Denied" für omegaflow; PI Sotgiu: CSES-02-Umbau, warten.
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

### CI format-Job (cross-line)
- **Status:** offen | **Bindung:** linie
- **Lage:** `ci-check` `format` rot: mycelium-Dateien `src/archivar/fai_kz.rs:47`,
  `src/archivar/ia2_tap.rs:96`; ferner `hdf5.rs`, `te.rs`, `tests.rs`,
  `hyperscanning_group_te.rs` (mountain/sensory).
- **Blockade:** `cargo fmt` ist lokal strukturell verweigert.
- **Braucht:** je Linie die eigenen Dateien formatieren (mycelium: fai_kz.rs,
  ia2_tap.rs).

## Benchmark

- Die Routine-Klasse ist geschlossen (flash-Sieger, 2026-09-16) — zitiert, kein
  Doppel-Lauf. Die zwei gebauten Atome liefen `grind-pro` (Urteilsklasse:
  Datums-Contract bzw. 0-honored-Semantik), kein flash-Gegenlauf.

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/quaoar_occlt.rs`, `src/archivar/babamul.rs`,
  `tools/harvest/src/bin/babamul_compiler.rs`
- `phi/pipeline/ledger.φ`, `phi/blocked_sources.φ`, `phi/sources.φ`,
  `phi/footprints.φ`
- `docs/handover/post.md`, neues Handover
  `handover-2026-09-22-mycelium-folge137.md`, Move
  `handover-2026-09-21-ernte-folge136.md` → `archiv/`
- **Fremd (nicht angetastet):** `AGENTS.md`, `docs/concepts/4d-membrane.md`,
  `src/gate/commit_gate.rs`, `src/gate/commit_gate_vocab.json`,
  `tools/register/src/bin/{number_audit,open_points_check,path_reference_scan}.rs`,
  diverse `archiv/`-Handover.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
