<!--
  title: Handover — Mycelium-Folge 313 (2026-10-11)
  session: Mycelium-Linie — Meta-Pass in einem Atom: PCN-sha256 aus dem CDN-Release-Digest, usgs_lidar_coverage (Block + coverage-manifest-Matrix-Arm) und glofas_ewds_coverage (Compiler + Workflow + Block) registriert, GRDC-1h-curl-Cap geheilt (7.06 GB / ~86 min) auf den Streaming-Helper, GloFAS-EWDS-Route gebaut, SOURCE_PORT-Sidecar-Regel geschärft, Prosa-Marker (free-voices/tools-map) + Frame/Epoche-Wire-Doc; Stehender Pass folgt am neuen HEAD.
  class: handover
  date: 2026-10-11
  sha256: 9a0b97d0241a80f5fb5276b5d71d03a9cdbc80fe03e7d20273e69b3d774aa9cd
  status: live
-->
# Handover — Mycelium-Folge 313 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-11-mycelium-folge312.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0519 · cap 0.20 deklariert — Grund: Meta-Pass in einem Atom (flash-first; 3 Taucher: 3×grind-flash); kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-11 | Operator (Session 313) · `state/operator-gespraeche/2026-10-11-mycelium.md` |
| „auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl" | 2026-10-11 | Operator (Session 310) |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-11 | Operator (Session 312) · `state/operator-gespraeche/2026-10-11-mycelium.md` |
| Sensory: `{cmb_planck,mla,ola}_compiler.rs` = Referenz, kein Träger (a) | 2026-10-11 | Operator/Sensory (Session 310) |

## Offen — eigen

### open-lidar / coverage-manifest — EA-4xx-End geheilt; USGS-Arm ergänzt
- **Status:** wartend · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** neuer `coverage-manifest`-Lauf endet
- **Lage:** (gemessen 2026-10-11) Lauf `38169560006` **queued** (Matrix `ea_lidar_index`); das 4xx→`End`-Fix ist gebaut. Der `usgs_lidar_coverage`-Arm ist jetzt registriert (`phi/sources.φ:20944`, `format usgs_lidar_coverage`) und als Matrix-Zeile in `.github/workflows/coverage-manifest.yml` (`--bucket usgs-lidar-public --jobs 8`) ergänzt — der Arm hat jetzt einen Writer.
- **Blockade:** Lauf fehlt; Mountain-Verdikt NL-Basis offen (`state/mycelium/lidar_id_diff_measured.md`).
- **Braucht:** `gh workflow run coverage-manifest.yml` nach Push; Lauf-Log via `ci_manage log`; Mountain-Verdikt NL.

### LiDAR-Inventar / Epochen — swisstopo+ign re-open; ahn4 2 Kacheln
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11, Taucher-Kachel-ID-Diff) `swisstopo` flai 39514/41982 (94,1 %) und `ign` 337207/524687 (64,3 %) → **re-open**; `ahn4` 1373/1375 (2 Kacheln `08bz1`/`40hz1` fehlen); `linz` ohne flai-Überlappung.
- **Blockade:** Parser/Register-Blöcke bei Mountain.
- **Braucht:** die 2 fehlenden ahn4-Kacheln + swisstopo/ign-Epochen nachziehen ODER die Descopes neu messen; Mountain-Verdikt.

### THEMIS-ASI getimetes Keogramm — Harness gebaut; Asset-Name-Riss aufgelöst
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** neuer `themis_asi-timed-keogram-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-11) `themis_asi_timed_keogram_compiler.rs` + Workflow + Block (per-Nacht-Asset `themis_asi_timed_keogram_<YYYYMMDD>_<station>.bin`); `cargo check -p omegaflow-harvest` 0/0. Läufe `38170044789` pending / `38169557688` queued. `format themis_asi_timed_keogram` registriert.
- **Blockade:** Läufe laufen; Station-Koordinaten-Tabelle + `terms` (UCalgary) fehlen.
- **Braucht:** Log via `ci_manage log`; Station-Koordinaten/`terms` (Mountain).

### GRDC-Caravan — 1h-curl-Cap geheilt; Reader-Arm + field/terms offen
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** neuer `grdc-caravan-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-11) Lauf `38156642621` **failure** — `fetch void` nach exakt 3600 s: der ad-hoc-`curl -m 3600` traf den Cap (Content-Length **7 575 685 313 B**, ≈86 min bei gemessenen 1,46 MB/s). **Geheilt:** `grdc_caravan_compiler.rs` nutzt jetzt `fetch_raw_to_file_with` (Streaming auf Platte, `GRDC_TRANSFER_BOUND_S = 3·3600`, bounded retries); `cargo check -p omegaflow-harvest` 0/0. Block registriert (`phi/sources.φ:21518`, `format grdc_caravan`). Riss bleibt: kein `field` — Einheit **mm/d**.
- **Blockade:** Lauf fehlt; `src/archivar/extract.rs`-Reader-Arm `grdc_caravan` fehlt; `field` mm/d + `terms` (Mountain).
- **Braucht:** `gh workflow run grdc-caravan-cdn.yml` nach Push; Reader-Arm; `field` mm/d + `terms` (Mountain); sha256 nach grünem Lauf.

### MLA-Register — repointet; mla-cdn-Ergebnis + sha256 offen
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** Lauf `mla-cdn` endet
- **Lage:** (gemessen 2026-10-11) Block `phi/sources.φ:12100` repointet (`pds3_fixed_width_mlaradr0801141902.bin`); `field RANGE mla_range` steht (Mountain 319).
- **Blockade:** `mla-cdn`-Ergebnis; NAME-Riss (`.lbl` trug kein `NAME`).
- **Braucht:** Lauf-Log lesen; sha256 nachtragen; NAME-Riss (Mountain).

### SPT-3G D1 `cmap` — Fix v2 grün; sha256 offen
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) Fix v2 in `8f23c7a71`; Lauf `38150991206` **success**.
- **Blockade:** keine.
- **Braucht:** sha256/Länge aus dem Release-Digest in `phi/sources.φ:11644` (Mountain-Aufenthalt).

### McPherron-Onset-Liste — Block + sha256 registriert; MCON-Bau (Mountain)
- **Status:** wartend · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** Mountains `MCON`-Format-Verdikt
- **Lage:** (gemessen 2026-10-11) `mcpherron_compiler.rs` + `.github/workflows/mcpherron-cdn.yml`; **Lauf `38168682154` success** → Asset `mcpherron.bin` 12 896 968 B, sha256 `0c99e7d8…`; **Block registriert** (`phi/sources.φ:21542`, `format substorm`, sha256 gesetzt).
- **Blockade:** das MCON-Format (Mountain) für den `SubstormOnset`-Konsum.
- **Braucht:** Mountains `MCON`-Bau; `terms` (Zenodo CC-BY-4.0) (Mountain).

### ENTSO-E A78 — Secret gesetzt; erster Lauf queued
- **Status:** wartend · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** erster `entsoe-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-11) `ENTSOE_API_TOKEN` als Repo-Secret gesetzt; Lauf `38169562211` **queued**.
- **Blockade:** Lauf läuft.
- **Braucht:** Log via `ci_manage log`; bei Erfolg Register-Block (url/origin/format/at earth/ttl); `terms`/`ttl` (Mountain).

### Re-Admission — GloFAS über EWDS (Route gebaut + registriert)
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** erster `glofas-ewds-cdn`-Lauf endet (push-getriggert)
- **Lage:** (gemessen 2026-10-11) GloFAS liegt im **EWDS** `https://ewds.climate.copernicus.eu`, **nicht** CADS. STAC `…/api/catalogue/v1/collections/cems-glofas-historical` = **200** (12 Kollektionen, 5 GloFAS); `…/items` = **501** (nicht implementiert); `…/retrieve/v1/processes/cems-glofas-historical` = 200 (Prozess-Inputs), `…/execution` = POST-only, **401** (auch mit `PRIVATE-TOKEN: $CDS_API_KEY` gemessen). **Gebaut:** `tools/harvest/src/bin/glofas_ewds_compiler.rs` (keyless Katalog-Walk → `shard`-Zeilen, `format coverage_manifest`, `--ci-mode` → `upload_release("ewds.climate.copernicus.eu", …)`) + `.github/workflows/glofas-ewds-cdn.yml` + Block (`phi/sources.φ:11295`); `cargo check -p omegaflow-harvest` 0/0. Riss im paper `docs/paper/sturzflut-tibet-pfeil.md:450` (behauptet GloFAS „declined, 404" — Host falsch).
- **Lage-Nachtrag (gemessen 2026-10-11 im eingeloggten Download-Tab):** der offizielle Weg ist die **EWDS API (Python `cdsapi`-Stil)** — `dataset = "cems-glofas-historical"`, `client = cdsapi.Client()`, `client.retrieve(dataset, request).download()` — nicht der rohe STAC-`execution`-POST. **Request-Änderung 30.07.2026** (`confluence.ecmwf.int` pageId 699325478): `hyear→year`, `hmonth→month`, `hday→day`; **neuer Pflichtparameter `timespan`** (`instantaneous` | `time_mean`); Variable `river_discharge_in_the_last_24_hours → average_river_discharge_in_the_last_24_hours`. Der 401 ist damit **ECMWF-Account-Token** (Konto `code@omegaflow.space` eingeloggt), plus einmalige Lizenz-Annahme („Accept terms").
- **Lage-Nachtrag 2 (gemessen 2026-10-11, `glofas_ewds_compiler --probe`):** `POST https://ewds.climate.copernicus.eu/api/retrieve/v1/processes/cems-glofas-historical/execution` mit `PRIVATE-TOKEN: $EWDS_API_TOKEN` und Body `{"inputs":{system_version:"version_4_0", hydrological_model:"lisflood", product_type:"consolidated", timespan:"time_mean", variable:"average_river_discharge_in_the_last_24_hours", year:"2020", month:"01", day:"01", data_format:"grib2", download_format:"unarchived"}}` → **HTTP 201**, `jobID bda23140-8862-4fde-b6ff-c60d4848e0b2`. `timespan:"instantaneous"` → **400** („no valid combination"); `time_mean` trägt. Der Token authentifiziert. Prozess-Inputs (keyless, `…/processes/cems-glofas-historical` 200): `system_version` v4_0/v5_0 · `hydrological_model` lisflood · `product_type` consolidated/intermediate · `timespan` instantaneous/time_mean · `variable` 4 (s. o.) · `year` "1979".."2026" · `data_format` grib2/netcdf · `download_format` zip/unarchived. **Gebaut:** `--probe` + `KEY_NAME="EWDS_API_TOKEN"`; Workflow materialisiert das Repo-Secret (`bin/secrets-sync.sh --set`).
- **Lage-Nachtrag 3 (gemessen 2026-10-11, `--fetch` + `--download`):** der volle Daten-Arm läuft — `POST …/execution` → **201** + `jobID`; `GET …/retrieve/v1/jobs/{jobID}` → `status:"successful"`; `GET …/retrieve/v1/jobs/{jobID}/results` liefert **JSON** `{"asset":{"value":{"type":"application/x-grib","href":"<object-store-url>","file:size":21366009,…}}}` — **nicht** die Datei. Der `href` (ECMWF-Object-Store) wird gefolgt; `--download 5641493b-…` → `data/ewds.climate.copernicus.eu/glofas_20200101_average_river_discharge_in_the_last_24_hours.grib` **21 366 009 B** (== `file:size`), sha256 `5d117a46…`, Magic `GRIB` Edition 2. `--out`-Default `data/ewds.climate.copernicus.eu/glofas_<y><m><d>_<variable>.grib`; `--download <jobID>` wiederholt ohne Neu-Submit; `--ci-mode` → `upload_release`.
- **Secret-Hygiene-Incident (gemessen 2026-10-11):** die `how-to-api`-Seite zeigt den CDS/EWDS-Key inline; ein `browser_get_text` las ihn in den Transcript. Operator hat den Key **rotiert** (Fingerprint `sha=4e893caa73f8`); die Key-tragende Spill-Datei ist gelöscht. Profil-/Key-Seiten werden nicht mehr per Browser gelesen.
- **Blockade:** der grib2→φ-Konsum fehlt (Grid → Sample-Punkte); die getestete Auswahl ist ein 1-Tag-Slice (kein Harvest-Umfang).
- **Braucht:** grib2-Konsum (Grid → Samples in den 26×f64-Vertrag) + eine echte Auswahl; Register-Block-Scharfschaltung für den Daten-Arm (der bestehende `format coverage_manifest`-Block bleibt für den keyless Katalog-Walk); `terms` (CEMS FLOODS) (Mountain).

### NIST NCNR NSE — Route gebaut + registriert; Lauf grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`-Verdikt
- **Lage:** (gemessen 2026-10-11) `nist_ncnr_nse_compiler.rs` + Workflow + Block (`phi/sources.φ`, `format coverage_manifest`); Lauf `38170359508` **success** → Asset `nist_ncnr_nse_coverage.manifest` 3149 B, sha256 `eee1e943…` (in den Block geschrieben). Ersetzt das descopte `trisp-keller` als offene NSE-Route für `kuprat-complex-te`.
- **Blockade:** `field`-Riss der coverage-Klasse.
- **Braucht:** `terms` (US-PD) + `field` (Mountain); Riss `format opentopography_coverage` (`phi/sources.φ:20023`) vs Loader-Arm (`main_flow.rs:5106`).

### Coverage-Manifest-Fetch-Loop — USGS-Manifest registriert
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** erster `usgs_lidar_coverage`-Matrix-Lauf endet
- **Lage:** (gemessen 2026-10-11) `shard <url>`-Form steht; Arm `format coverage_manifest`/`usgs_lidar_coverage` in `src/archivar/main_flow.rs:5106`. `usgs_lidar_coverage` jetzt registriert (`phi/sources.φ:20944`) + Matrix-Zeile (`coverage-manifest.yml`).
- **Blockade:** `field`-Riss der coverage-Klasse (`main_flow.rs:5173-5180`) — Mountain-Zeilenklasse; Lauf operator-getriggert.
- **Braucht:** `gh workflow run coverage-manifest.yml`; `field` (Mountain); 104 Nationalportale enumerieren.

### Offene-Prosa-Träger (future-235) — 3 Dokumente, je ein nächster Schritt
- **Status:** eigen · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** nächste Berührung je Dokument
- **Lage:** (gemessen 2026-10-11) `register_lookup --orphan-docs` = 0 (diese Zeile trägt sie). `free-voices.md:87` (Falsch-Positiv) und `tools-map.md:374` (Falsch-Positiv) sind **rewortet** — Marker weg; `tools-map.md:349` **operator-gebunden** (Debugger-Rechte am live Chrome). Offen: `survey-2026-09-03-daten-holdings-inventur.md:96` (operator-gated) · `survey-2026-09-03-orphan-verdicts.md:106` (Step 5 CDN-kanonisch vs 13-netloc, destruktiv) · `survey-2026-10-08-fmhy-research-landscape.md:53` (Seat-Re-Probe MiMo).
- **Blockade:** je Dokument ein eigener Akt (operator-gated / destruktiv / Seat).
- **Braucht:** die drei surveys den benannten nächsten Schritt.

### Mariner-Rohband-CDN (future-236) — wartet Mountain
- **Status:** wartend · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** Mountains `url`-Zeilen + Parser-Arme für SPDF-Roh-Tracking-Images (M4/6/7) + ASU-M10-Imaging-Rohbänder
- **Lage:** (gemessen 2026-10-11) die zwei genutzten Produkte (SPK `phi:31596`, Occultation `phi:20261`) sind geerntet; `mariner-rst` = descoped. NSSDCA-Auth (`NSSDC_USER`/`NSSDC_PASS`) ist die Kante für `PSCM-00009`.
- **Blockade:** Mountains Parser/`url`-Zeilen fehlen.
- **Braucht:** Mountains Arme; dann Mycelium `url`/`origin` + Manifest-Lauf.

### MCP no-leak — Runtime-Hälfte offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** MCP-Session-Restart → `state/mycelium/mcp_allowlist_capture_measured.md`
- **Lage:** (gemessen 2026-10-11) `opencode.json` parst; Wrapper Pin `1.9.0`; Allowlist konsistent.
- **Blockade:** Live-Seat-Prüfung braucht einen MCP-Session-Restart.
- **Braucht:** nach dem Restart die Seats live prüfen.

### de441-de442-sha
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** erster Merge-Lauf je NAIF-Anker (`phi/sources.φ:4321-4369`)
- **Lage:** (gemessen 2026-10-11) NAIF-Assets registriert; sha256 erst nach dem Merge-Lauf.
- **Blockade:** wartet auf den Lauf.
- **Braucht:** Merge-Lauf-Ergebnis.

### dropped-gate — Match-Fix gebaut; Event-Fold offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) `ci-gate`-Job `dropped-gate` rotete auf `new 5` (`38165122495`); die 5 Namen sind Handover-Churn, in `docs/zustand/dropped-legacy-baseline.txt` gepinnt; Match-Fix `offer_word` lowercase + Test (312). Riss: der future-235-Block behauptet 924 Keys; die getrackte Baseline trägt **78**.
- **Blockade:** die Q2/Q3-Event-Fold-Verdrahtung (`dropped_gate.rs`) ist ein eigener CI-only-Atom.
- **Braucht:** Q2/Q3-Event-Fold.

### SWPC/RTSW-Archivreihe — wartet Mountain
- **Status:** wartend · **Teile linie:mountain** | **Bindung:** eigen
- **Trigger:** Mountains Quelle + `ttl` genannt
- **Lage:** (gemessen 2026-10-11, Lauf `38119733631` Job `matrix-vlies`) 126 `alignment pending`-Zellen schließen erst mit einem Compiler auf die **Archivreihe**.
- **Blockade:** Mountains Quelle/`ttl` fehlen.
- **Braucht:** Mountains Quelle+`ttl`, dann Compiler + `*-cdn.yml`.

### Architektur — CI-Optimierung
- **Status:** wartend · **Teile linie:mountain/river** | **Bindung:** eigen
- **Trigger:** grüner `ci-check`-Lauf
- **Lage:** (gemessen 2026-10-11, `ci_manage log 38166097283 --all`) `register` failure = `cdn_reconcile: fingrid-cdn.yml:48` (auf `1bf37f2e6`, inzwischen behoben); `clippy` failure = 3 Fehler in `src/archivar/odf.rs` (`:179`/`:213`/`:2222`, Träger Mountain); `format` failure = Diffs in `src/mathematikerin/{mod,receiver,tropo}.rs` (fremde uncommittete Hünke im Baum).
- **Blockade:** Shard-Build-vs-Test-Anteil erst an grüner `ci-check` messbar.
- **Braucht:** **Mountain:** `odf.rs` clippy-clean + `src/mathematikerin/*` fmt committen; dann grüner `ci-check`.

### matrix-rotor
- **Status:** wartend · **Teile linie:river** | **Bindung:** eigen
- **Trigger:** nächster `matrix-rotor`-Lauf (externer Akt)
- **Lage:** (gemessen 2026-10-11) letzter rot = transienter Runner-Shutdown.
- **Blockade:** nur der persistente t420 ist dauerhaft.
- **Braucht:** —

### Domain→UCD — pending (Operator-Wort 2026-10-11)
- **Status:** pending | **Bindung:** eigen
- **Trigger:** eine messbare Domain-Quelle
- **Lage:** (gemessen 2026-10-11) keine `domain`-Achse; das Mapping bleibt `pending` (`state/mycelium/quantitykind-ucd-mapping.md`, 5/10 gemessen).
- **Blockade:** keine messbare Domain-Quelle.
- **Braucht:** eine messbare Domain-Quelle (kein Bau).

## An mountain

Origin: mycelium-313 (2026-10-11).

- **THEMIS-ASI-Keogramm** — der 310 gebaute `themis_asi_keogram_compiler.rs` spiegelt stream2; der getimete Pfad ist stream0 (neu gebaut). Braucht Station-Koordinaten (`on earth <lat> <lon> <alt>`) + `terms` (UCalgary).
- **GRDC-Caravan** — Block registriert (`phi/sources.φ:21518`); Braucht `field` **mm/d** (nicht cfs) + `terms`.
- **McPherron** — Block + sha256 registriert (`phi/sources.φ:21542`); Braucht `MCON`-Bau + `terms` (Zenodo CC-BY-4.0).
- **GloFAS-EWDS** — Route + Block; Daten-Arm läuft (`--fetch` 201 → `successful` → `results`-JSON → `href` → GRIB2 21 366 009 B, Magic `GRIB` Ed. 2); offen: grib2→φ-Konsum + echte Auswahl; Braucht `terms` (CEMS FLOODS).
- **usgs_lidar_coverage** — Block (`phi/sources.φ:20944`) + coverage-manifest-Matrix-Arm; Braucht den `field`-Riss der coverage-Klasse.
- **ENTSO-E** — Braucht `terms`/`ttl` nach dem ersten Lauf.
- **MLidar/Epochen** — swisstopo/ign re-open + ahn4 2 Kacheln + NL-Verdikt.
- **MLA-NAME-Riss** — `.lbl` trägt kein `NAME`; die `NAME = RANGE`-Zeile widerlegen oder belegen.
- **odf.rs clippy (`:179`/`:213`/`:2222`) + `src/mathematikerin/*` fmt** — blockt `ci-check`; die fmt-Hunks liegen uncommitted im Baum.
- **NIST NCNR NSE** — Route registriert (`phi/sources.φ:11711`); braucht `terms` (US-PD) + `field`. Riss: `format opentopography_coverage` (`phi/sources.φ:20023`) nicht dispattcht.

## An river

Origin: mycelium-313 (2026-10-11).

- **Getimetes THEMIS-ASI-Keogramm** — Harness + Workflow + Block stehen; der stream0-`format` ist registriert. Offen: erster per-Nacht-Lauf + Station-Koordinaten (Mountain).
- **GRDC-Caravan** — der 1h-curl-Cap ist geheilt (Streaming-Helper, 7.06 GB / ~86 min); Re-Dispatch nach Push.
- **ENTSO-E** — Secret gesetzt; erster Lauf queued.
- **GloFAS-EWDS** — Route + Workflow + Block stehen; push-getriggert.
- **McPherron-CDN** — Lauf `38168682154` success, Block + sha256 registriert.
- **SWPC/RTSW-Archivreihe** — unverändert wartend auf Mountain.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
