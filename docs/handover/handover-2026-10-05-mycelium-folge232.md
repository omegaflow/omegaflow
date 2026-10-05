<!--
  title: Handover — Mycelium-Folge 232 (2026-10-05)
  session: Mycelium-Linie — CI-Triage 2026-10-05, adressierte Blöcke gefaltet, Orphan-Carrier, Stehender Pass
  class: handover
  date: 2026-10-05
  sha256: 101de874975a5ed882965a4b52ba47de736c258dd50dedbea6d739e227e005e8
  status: live
-->
# Handover — Mycelium-Folge 232 (2026-10-05)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-04-mycelium-folge231.md` (→ `archiv/`).

## Burn: open 0.0028 · close 0.0429 (session_burn, gemessen; Linie + 3 Taucher: CI-Triage $0.0324 · Listen-Verifikation $0.0340 · EDF-Diagnose)

## Operator-Wort-Register

- Wort | 2026-10-02 | „ich meine glm 5.3 max mit deep search ist echt gut das sollten wir intensiver nutzen" | Quelle: future-folge169.
- Wort | 2026-10-02 | „glm claude und kimi im chat liefern die besten recherchergebnisse" | Quelle: future-folge169 — Recherche-Trio.
- Wort | 2026-10-02 | „nein genug mit den Sondenanfragen. Die Ernte sollten natürlich eingeholt werden." | Quelle: future-folge169.
- Wort | 2026-10-02 | „… ihr macht umfangreiche läufe und dann kastriert ihr sie … so funktioniert forschung nicht" | Quelle: future-folge169 — kein Top-N.
- Wort | 2026-10-02 | „ich kann es mir beim besten willen nicht vorstellen, dass wir nicht an die daten kommen — bitte fahre jetzt starke legale geschütze auf" | Quelle: future-folge169.
- Wort | 2026-10-01 | „stehen lassen aber das wort ist du bist die letzte linie die committed das muss sitzen" | Quelle: Mycelium-Session 216 — Mycelium committet als letzte Linie, nur mit dem `/commit`-Wort.
- Wort | 2026-10-01 | „bitte nicht nur messen und verschleppen sondern bearbeiten messen und bearbeiten ist die prämisse mein dauerhaftes wort" | Quelle: Mycelium-Session 216 — **dauerhaftes Wort**.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213.
- Wort | 2026-10-05 | „du bist mycellium" | Quelle: Operator (Session 2026-10-05) — der Commit-Prefix `mountain` war falsch.
- Wort | 2026-10-05 | „kannst du dir das bitte ansehen?" + zwei Listen (GIC/`field_te_query`) | Quelle: `state/operator-gespraeche/2026-10-05-mycelium.md` — Review-Auftrag; die Listen sind Claims gegen den Baum gemessen (Riss-Befund in der Session).
- Wort | 2026-10-05 | „ja bitte ablegen" | Quelle: `state/operator-gespraeche/2026-10-05-mycelium.md` — die verifizierte Drei-Zustands-Verdrahtungsliste als adressierte Register-Zeile (§ `## An river`).

## Offen — eigen

### register-coverage — 2 Mycelium-Orphans gefaltet (`:166` BGI AGrav, `:170` C9/CEEIN)
- **Status:** eigen
- **Trigger:** nächster `register-coverage`-Lauf auf dem Push-HEAD
- **Lage:** (gemessen 2026-10-05 `ci_manage log 37251844037` / `37249551259`) `register_lookup --orphans --fail` → exit 2, `2 orphan entries [mycelium 2]`: `phi/blocked_sources.φ:166` (BGI AGrav) und `:170` (C9/CEEIN) trugen keinen Träger in einer Live-Mycelium-Übergabe. Mit dieser Übergabe sind beide Träger (s. `### blocked_sources.φ …`).
- **Blockade:** keine
- **Braucht:** Push dieses Handovers; `register_lookup --orphans` muss 0 mycelium zeigen.

### nvss-cdn — Lauf grün, `sha256`/`url`-Rebind offen
- **Status:** eigen
- **Trigger:** `nvss.json` am CDN erreichbar → `archive_search --sniff <url>`
- **Lage:** (gemessen 2026-10-05, grind-flash) Lauf `37233581603` success — `vizier_asu_compiler: 216578 NVSS rows, 669977 SDSS spec-z rows, 10908 matched → kernels/nvss_c315.json`; Release `ssd.jpl.nasa.gov-nvss` angelegt. `nvss.json` veröffentlicht; `sha256` im Log nicht gedruckt.
- **Blockade:** keine
- **Braucht:** `sha256` von `nvss.json` in `phi/sources.φ` nachtragen; `url`-Rebind prüfen.

### SUDEP ds004100 — EDF-Arm überspringt jede Datei (gemessene Ursache)
- **Status:** eigen
- **Trigger:** Fix im `extract_edf`-Pfad
- **Lage:** (gemessen 2026-10-05, grind-flash-Diagnose) `openneuro-cdn 37233586379` failure; jede `.edf` → „carries no contract — skipped (0 honored)". Ursache an `sub-HUP060_..._run-01_ieeg.edf` (22 360 708 B) gemessen: `data_records=378>0`, `header_bytes==expected`, `p_max>p_min` für alle; der Skip kommt aus `tools/harvest/src/bin/openneuro_compiler.rs:355-362` — `signals[59]` (`EDF Annotations`) hat `samples_per_record=57 ≠ 500` der EEG-Kanäle. Der EDF+C-Annotationskanal bricht die Uniformitätsbedingung.
- **Blockade:** keine
- **Braucht:** in `extract_edf` den Annotationskanal (Label `EDF Annotations`) vor der Uniformitätsprüfung herausfiltern (die Wire trägt eine `srate`, der Kanal keine EEG-Spur), dann re-dispatch; bei Grün `format`/`sha256`/`url`-Block in `sources.φ`.

### clpds-cdn Annex — Lauf success, Register nachziehen
- **Status:** eigen
- **Trigger:** `clpds_annex.jsonl`-Asset am CDN → `archive_search --sniff`
- **Lage:** (gemessen 2026-10-05, grind-flash) Lauf `37233584228` success — `assets already present — manifest skipped`; `clpds_catalogue.json` + `clpds_files.jsonl` referenziert, `sha256` nicht gedruckt.
- **Blockade:** keine
- **Braucht:** `clpds_annex.jsonl` mit `sha256`/`url` im Register nachziehen.

### iEEG-Ernte — Backend 503 (Server-Kapazität)
- **Status:** wartend
- **Trigger:** iEEG-Backend erholt sich → `gh workflow run ieeg-cdn.yml -f dataset=09_14_limbic_seizure_374`
- **Lage:** (gemessen 2026-10-04) `37235356150` failure, `ieeg: getId http 503` (Server-Kapazität, nicht Auth); Temp-Pfad-/Env-Fix committet.
- **Blockade:** iEEG-Backend überlastet (Server).
- **Braucht:** bei Kapazität re-dispatch; bei Grün `format ieeg_edf` + `sha256`; 4D-Anker je Elektrode = River/Mountain.

### Exposom-Arme (WQP + EEA-noise) — Läufe cancelled, re-dispatch
- **Status:** eigen
- **Trigger:** `wqp-cdn` / `eea-noise-cdn` re-dispatch → Log lesen
- **Lage:** (gemessen 2026-10-05, grind-flash) `wqp-cdn 37236691679` und `eea-noise-cdn 37236694323` **cancelled** (Log 404, `unread`). Register-Blöcke stehen; `sha256` fehlt.
- **Blockade:** Lauf abgebrochen (Queue/Cancel).
- **Braucht:** beide Workflows neu dispatchen; bei Grün `sha256` in `sources.φ`; WQP-Vokabular-Riss = Mountain.

### ned-byparams — 0/180 Bänder, kein Final-Asset
- **Status:** eigen
- **Trigger:** `ned-byparams-cdn` re-dispatch → Log
- **Lage:** (gemessen 2026-10-05, grind-flash) Lauf `37250626173` success, aber `bands present: 0/180, ned_byparams_redshift.json present: 0`; die per-Band-Zeile `result fetch void at <url>` (`tools/harvest/src/bin/ned_byparams_compiler.rs:780`) — Ursache aus dem Log nicht auflösbar (`unread`).
- **Blockade:** Band-Ergebnis-URL void.
- **Braucht:** einen einzelnen Band-Lauf mit `--band` und voller Log-Ausgabe messen, dann `result_url`/`fetch_body` prüfen.

### Register-Träger `ledger.φ:2`/`:6` — Port-Artefakte
- **Status:** blockiert
- **Trigger:** Port-Runner im Baum
- **Lage:** (gemessen 2026-10-04) `ledger.φ:2` = 825 Blöcke, `:6` = 63; `phi/pipeline/stage/*` leer; der Ausführer war ein nie committeter Working-Tree-Bin; nur der Motor `src/archivar/port.rs`.
- **Blockade:** Port-Runner verloren — nicht im getrackten Baum.
- **Braucht:** Port-Runner als Bin rekonstruieren/committen (Engine `src/archivar/port.rs`; Konverter-Spec = Mountain).

### `phi/blocked_sources.φ` — Mycelium-Klasse (Träger; Stand gemessen 2026-10-05)
- **Status:** je eigen | **Bindung:** eigen
- **Lage** (gemessen 2026-10-05 via `register_lookup --open`), je Eintrag ausgang:
  - `:166` BGI AGrav — `https://api.sedoo.fr/get-agrav-rest/` station/nearto 200 (853 714 B JSON), `point/byStationUuid.observations[].gravity m/s2`; Verdikt inverse-square gravity; **Arm + Manifestation offen** (Orphan-Träger, s. o.).
  - `:170` C9/CEEIN Infraschall — `http://ceein.infp.ro/fdsnws/` station 200, dataselect `C9/BDF` MSEED 200, Archiv endet ~2023-10-30; Verdikt gaussian-inverse-square acoustic Pa; **Arm + Manifestation offen** (Orphan-Träger, s. o.).
  - `:82` ExoMars TGO ACS — Arm steht (`pds4.rs` base16 Group_Field_Character; `ROW_DATA_0000..1279`; End-to-End 200 rows); Asset 543 800 B `sha256 465f3c07…`; **Registrierung (`format pds4_fixed_width`, `at mars`) + Manifestation offen**.
  - `:98` Phobos 2 KRFM / `:102` Hayabusa LIDAR / `:106` Tianwen-1 MoRIC — materialisiert; Feld-/Format-Risse = Mountain.
  - `:110` Danuri ShadowCam — Riss River/Mountain (kein FITS in `/derived/`).
  - `:118` JAXA G-Portal — `sha256` steht; Record-Download (`add_download.json`/SFTP) offen.
  - `:122` CSES materialisiert; `:126` CLPDS Annex (s. o.); `:130` Viking gravity success; `:138`/`:142` externe Hosts down (wartend); `:146` PDS-PPI Kuration; `:150` descoped; `:154` Gaia-RRL Riss Mountain/River; `:158` cluster_ka descoped.
- **Blockade:** je Eintrag (Arm-Bau / Mountain-Disposition / externe Hosts).
- **Braucht:** je eigener Arm `:166`/`:170` bauen + manifestieren; `:118` Download-Route; `:146` Kuration.

### ExoMars TGO ACS — Registrierung + Manifestation
- **Status:** eigen
- **Trigger:** Register-Block in `sources.φ` geschrieben → `acs-nir-cdn` Lauf `37251847669` (queued)
- **Lage:** (gemessen 2026-10-05) `mycelium 231q` (`d83150fa9`) baute den Arm + `acs-nir-cdn.yml`; Block `format pds4_fixed_width`, `at mars`, `field ROW_DATA{...} em count`.
- **Blockade:** Runner-Queue.
- **Braucht:** `37251847669` Log; bei Grün `sha256`/`url`.

### EMM EXI L2a — Workflow angleichen + Frame-Bundle-Arm (mountain-234)
- **Status:** eigen
- **Trigger:** `emm-sdc-cdn.yml` angeglichen → Lauf
- **Lage:** (gemessen, mountain-234) L2a = Bild (256×192, DN), kein `field` → `hips_png`-Form; Compiler-Risiken geheilt. Working tree trägt einen uncommitteten rustfmt-Hunk in `tools/harvest/src/bin/emm_sdc_compiler.rs` (nur Formatierung der `bearer_fetch`-matches, kein Logikwechsel) — eigener Commit-Kandidat.
- **Blockade:** Frame-Bundle-Arm (Loader wie `hips_png` in `main_flow.rs`) noch nicht gebaut.
- **Braucht:** `emm-sdc-cdn.yml` angleichen (l2a, Datums-Range, Keyname, Asset); Loader-Arm; dann `format emm_exi_l2a` + Manifestation.

### `abk_dbdt_1m`-Derivat — harvesten + manifestieren (river-92)
- **Status:** wartend
- **Trigger:** Mountains `--grain minute`-Arm in `intermagnet_dbdt_compiler.rs` steht
- **Lage:** (gemessen 2026-10-05) Roh-Korn `supermag_1m` (`sources.φ:17375-17399`) liegt; Arm noch nicht erweitert.
- **Blockade:** `--grain minute`-Arm (Mountain) fehlt.
- **Braucht:** nach dem Arm `abk_dbdt_1m.bin` ernten + Register-Zeile + CDN.

### superdarn Re-Tag (future-179)
- **Status:** eigen
- **Trigger:** Tag-Korrektur committet
- **Lage:** (gemessen 2026-10-05) `blocked_sources.φ:78` trägt `blocked account [future]`; der Zugang liegt via `GLOBUS_ID_USER/PASS` vor (`wartend.φ:8`).
- **Blockade:** keine
- **Braucht:** den Tag in `blocked_sources.φ:78` korrigieren (Konto liegt vor).

### Orphan-Docs — 2 Surveys ohne Träger (future-179)
- **Status:** eigen
- **Trigger:** Träger-Zeile geschrieben
- **Lage:** (gemessen 2026-10-05) `register_lookup --orphan-docs` = 0 aktuell; future-179 nennt `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` + `survey-2026-09-03-orphan-verdicts.md`.
- **Blockade:** keine
- **Braucht:** Träger in dieser Übergabe (erledigt, s. Orphan-Zensus) oder gemessenes `descoped`.

### Exposom-Quellenmatrix registrieren (future-179)
- **Status:** eigen
- **Trigger:** Matrix-Zeilen in `sources.φ`
- **Lage:** (gemessen 2026-10-05) `state/future/exposom-matrix-2026-10-04.md` (16 Klassen); Domänen ohne Home nicht registriert.
- **Blockade:** keine
- **Braucht:** die Domänen ohne Home als `sources.φ`-Zeilen + Manifestation; Matrix-Lauf (Workflow + `.te` je Klasse) `pending`.

## Adressierte Blöcke — gefaltet (2026-10-05)

- **future-179:** Orphan-Docs (gefaltet als Träger oben); Exposom-Matrix (eigener Punkt oben); Matrix-Lauf `pending`; Tavily-Quota bei 80 % (Fallback `--mwmbl`/`--marginalia`).
- **mountain-234:** EMM-`emm-sdc-cdn.yml`-Block + Frame-Bundle-Arm (eigener Punkt oben); WQP-Re-Harvest (Registrierung = Mountain-Disposition, Ernte = Mycelium); `goes_euvs`-Alt-Asset Orphan → `cdn_reconcile`; BGI AGrav `:168`/C9 `:172` (jetzt `:166`/`:170`) Arm+Manifestation (eigener Punkt oben); Tianwen-1 MoRIC (Riss = Mountain); ExoMars ACS (eigener Punkt oben); 1-min-`dB/dt` (eigener Punkt oben); superdarn Re-Tag (eigener Punkt oben).
- **river-92:** `ci-check` VerdictLine-Scope von River gefixt (`src/mathematikerin/tests.rs`); B-Membran `continue-on-error` auf dem wasm-Build — **an River zurückgegeben** (Transport-Grammatik, nicht Mycelium); `abk_dbdt_1m`-Derivat (eigener Punkt oben).

## Risiken / offene Risse (gemessen, nicht geglättet)

- **Runde 2026-10-05 (1. Umlauf) — Betreiber-Listen gegen den Baum:** Risse — Galileo I bereits gesetzt (`docs/blatt/blatt-anderson-flyby-ephemeridenhaus.md:28`); `goes_xrs`/`eve_lines` verdrahtet; SOI declined; NSRR/TUH descoped; JUICE-Doppler/NAVIO kein Mycelium-Asset; `aia_fullyear` kein Register-Key; Newell gebaut, nicht „in Bau".
- **Runde 2026-10-05 (2. Umlauf, GLM-Korrektur) — gegen den Draht gewogen:** 8 der 15 „verdrahtet"-Zeilen der korrigierten Liste sind nur probe-gelesen, nicht in `.te`/`field_te_query` (RTSW-Quelle · EVE 1032/131 · QBO · D20 · Kp · Swarm HAPI · EEG ds007822/ds007471 · Newell). Verifiziert am Draht: 7. Verifizierte Liste → `## An river`; Detail `state/operator-gespraeche/2026-10-05-mycelium.md`.

## An river

Origin: mycelium-folge232. **Routed — nicht-eigen; deine Disposition (Operator-Wort „ja bitte ablegen" 2026-10-05):**

Verifizierte Drei-Zustands-Verdrahtungsliste (gemessen 2026-10-05 gegen `phi/pipeline/descriptors/*.te` + `field_te_query.rs`):

- **Am Draht (7):** GOES XRS (`solar_hourly_event.te:16-17`) · AIA2013 (`aia_three_year.te:17-18`) · ERBQ-Event (`erbq-event.te:10-16`) · ERSST→NINO3.4 (`enso_blatt.te:17`) · TAO-Wind (`field_te_query.rs:3126`) · OMNI2 (`bz_retro.te:12`) · ABK `dbdt` (`bz_retro.te:13`).
- **Probe-gelesen, nicht am Draht (8):** RTSW/SWPC (`bz_blatt_probe.rs:610`; Bz läuft via OMNI2) · EVE 1032/131 (verdrahtet nur 584/304, `corona_ladder.te:17-18`) · QBO `qbo_30hpa` (`enso_blatt_probe.rs:162`) · D20 `d20_thermocline` (`enso_blatt_probe.rs:193`) · Kp `magnetosphere_kp_3h` (`bz_blatt_probe.rs:606`) · Swarm HAPI (`station_convergence_probe.rs:13`) · EEG ds007822/ds007471 (`hyperscanning_group_te.rs:30`) · Newell `dΦ/dt` (`bz_retro_probe.rs:431`,`:958`).
- **Declined/descoped (5):** SOI (`declined_sources.φ:4711-4713`) · NSRR (`blocked_sources.φ:164`) · TUH (`:160`) · JUICE-Doppler (keine Quelle; NAVIO = Pioneer) · WWP (kein `wwp` im Baum).
- **Reihenfolge:** Blitze (WWLLN/GLM/LIS) → Swarm TEC → GIC Mäntsälä → SuperDARN FITACF → Infraschall BGR hf → IGETS → Pioneer-10/11 → Newell-Runde füllen (n=0).

## LOCK

(kein Eintrag.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Stehende Pass wird
**nach** dem Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
