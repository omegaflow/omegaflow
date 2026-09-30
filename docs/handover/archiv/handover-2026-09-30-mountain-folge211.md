<!--
  title: Handover — Mountain-Folge 211 (Stand 2026-09-30)
  session: Mountain-Folge 211
  class: handover
  date: 2026-09-30
  sha256: bcb5c1583487fffb37d0b5dbf49ad8f15f57cf41c35347afb82bdad17eaef6f1
  status: live
-->
# Handover — Mountain-Folge 211 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Dieses Atom ist die
**Ausführung der folge210-Tafel** (Phase 2): Parser-Arme gemessen und gebaut, Register-Verdikte
geschrieben, die adressierten Blöcke (mycelium-folge210, river-folge70) gefaltet.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„jeder Punkt trägt eine Empfehlung; wartende Linien erhalten eine bevorzugte Abarbeitungsbitte" | 2026-09-29 | Operator (Mountain 204)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„bitte fetchen und messen pds4_fits BINTABLE" — Netz-Fetch + HDU-Prüfung ausgeführt | 2026-09-30 | Operator (Session, Mountain 211)
„braucht es wirklich pro?" — pro nur mit benanntem Hart-Atom oder gemessener flash-Fehllage | 2026-09-30 | Operator (Session, Mountain 211)
„hast du deinen teil gemacht? pds3/pds4-Registrierung … echt eigen ← mountain; die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — Register-Verdikte + Arme sind Mountains eigener Teil; kein `url`/`format` ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)

## Haus — Mountain (Stand 2026-09-30)

Diese Übergabe **ist** das Haus: jeder offene Punkt, jedes Verdikt, jeder Parser steht hier
mit Zustand, auch um 3 Uhr nachts.

- **Die vier Orte:** `omegaflow` = `~/projects/omegaflow` + privates Schwester-Repo `state/`
  (`omegaflow/personal`); `omegaflow-legacy` = `archive-root/omegaflow-legacy`; `temp` =
  `/tmp/opencode`; `archive` = `archive-root`.
- **Mountain-Fundstellen:** `phi/`, `src/archivar`, `src/mathematikerin`, `src/gate`,
  `tools/harvest`, `tools/measure`, `tools/register`, `docs/specs`, `docs/surveys`,
  `state/zustand`, `state/mountain`.
- **Linien-Preset (privat):** `state/mountain/archive-search-preset.txt`; `state/` immer mit
  `archive_search --root state`.

## Offen (aufgeschlüsselt)

### pds3-Arme decken jetzt IEEE_REAL / PC_REAL — Sample + Asset offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-30, `cargo check` 0/0) `pds3_binary::decode_binary_cell` decodiert
  `IEEE_REAL`/`MSB_REAL` (Big) und `LSB_REAL`/`PC_REAL` (Little) als f32/f64 nach Feldbreite;
  `pds3_img::byte_order_of` bildet `PC_REAL`/`LSB_REAL` auf Little, `MSB_REAL` auf Big;
  `decode_sample` hat den Float-Pfad. Tests: `decode_binary_cell_never_emits_a_fabricated_zero`
  (+1.0 / +inf), `decode_sample_reads_pc_real_and_rejects_non_finite`.
- **Blockade:** keine.
- **Braucht:** Kaguya LRS `.dat` (pds3_binary) bzw. Chandrayaan-1 Mini-RF `.img` (pds3_img)
  als Sample holen, Compiler-Bin + Asset + CDN-Manifestation.

### ExoMars TGO ACS — `Table_Character`, nicht `Table_Binary`
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-30, CI-Log `pds4-binary-cdn 36701710777` + Label-Fetch) die ExoMars
  ACS raw HK `.xml` tragen   `Table_Character` (`Record_Character`/`Field_Character`, CRLF), **nicht** `Table_Binary`.
  `pds4_binary_compiler` routet die File_Area-Klasse jetzt selbst: `Table_Binary` → `pds4_binary`,
  `Table_Character` → `pds4_fixed_width` (geteilte `axis_of`/`assemble` in `src/archivar/pds4.rs`);
  `cargo check` 0/0. Verdikt in `phi/blocked_sources.φ`.
- **Blockade:** keine.
- **Braucht:** Sample holen, Compiler laufen, Asset/Manifestation; Occ-NIR-Spektrum separat messen.
  `pds4-binary-cdn.yml`-Idempotenz prüft `^pds4_binary_` — auf `pds4_fixed_width_` erweitern (Mycelium).

### Chang'e-1/-2 MRM — BINTABLE-Arm gebaut
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-30 per Fetch) `data/ce{1,2}_mrm.fits` = Primär `NAXIS=0`, dann **eine**
  `XTENSION='BINTABLE'` (`TFIELDS=24`, Spalten `orbit`/`utc`/`et`/`ltst`/`t1..t4`); kein `IMAGE`-HDU.
  `pds4_fits::parse_bintable_series` + `parse_named_series` gebaut, `extract.rs`-Dispatch erweitert,
  `pds4_fits_compiler` nimmt BINTABLE an; `cargo check` 0/0.
- **Blockade:** keine.
- **Braucht:** `ce{1,2}_mrm.fits` vollständig holen, Compiler laufen, registrieren (Mountain),
  manifestieren (Mycelium).

### USGS-comcat — Compiler gebaut, Manifestation offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-30) `tools/harvest/src/bin/usgs_comcat_compiler.rs` gebaut
  (monatliche Rate M≥4.5, 1973-01…2026-08, Pagination; Probe Jan 2026 = 618, `cargo check` 0/0);
  Asset `usgs_comcat_m45.bin`, CDN-Tag `earthquake.usgs.gov`. Kein `url`/`format` gesetzt.
- **Blockade:** Manifestation fehlt.
- **Braucht:** Workflow + CI-Lauf, dann Registrierung (Mountain `url`/`format`, Mycelium `origin`/`compiler`/Tag).

### tao_wnd — Fenster geöffnet, Workflow schneidet weiter
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-30) `tao_wnd_compiler.rs` lädt jetzt `d_start=1977-11-06`..`now`
  (Vollabruf 10 398 995 B, kein ERDDAP-Limit), Truncation-Wächter gesetzt; `cargo check` 0/0.
  Der **Workflow** `tao-wnd-cdn.yml:35-38`/`:58` baut weiter ein 127-Tage-Fenster (`--input`).
- **Blockade:** Workflow unverändert.
- **Braucht:** Workflow-Fenster auf das volle Record (Mycelium), Release-Asset `tao_wnd_zonal.csv`
  löschen, `gh workflow run tao-wnd-cdn.yml`, re-manifestieren.

### kernel-flatten `spk_split` — Fix gebaut, CI-Verifikation offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-30, CI-Log `36639779027`) Ursache: Ziel 2174567 hat seine Segmente
  über zwei Summary-Records verteilt (Summary nach Data); der Reader brach ab → `curl (23)`.
  `tools/utils/src/bin/spk_split.rs` auf adressbasierten Record-Puffer + per-Segment-Assemblierung
  umgebaut (Test `records_cover_a_gap_between_summary_records`), `cargo build -p omegaflow-utils
  --bin spk_split` 0/0.
- **Blockade:** keine.
- **Braucht:** `gh workflow run kernel-flatten.yml`, CI-Verifikation am vollen Lauf.

### DAS2 Iowa — HAPI-CSV-Reader gebaut
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-30) `src/archivar/hapi_csv.rs` + `tools/harvest/src/bin/das2_iowa_compiler.rs`
  gebaut (HAPI 1.1 `time.min`/`time.max`, `info`-JSON für Spalten/Einheiten); End-to-End-Proben
  (`Cassini/MAG/Magnitude` 1432, `VectorKSO` 5732 records); `cargo check` 0/0. `hapi_draft_fields_csv`
  (`port.rs:1369`) bleibt Kurations-Textgenerator, kein Datenpfad.
- **Blockade:** Format-Arm fehlt.
- **Braucht:** `extract.rs`-Arm `hapi_csv`; erst dann `url`/`format`-Zeile; Sample/Asset.

### ODF-Flyby — Encounter-Konstanten gebaut, Konsument wartet auf Rohdaten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort zur DSN/JPL-Rohdatenanfrage (Future-Queue).
- **Lage:** (gemessen 2026-09-30) der Shard-Riss ist ein de-dup-Artefakt (River `7e2cefd9f`); die
  sieben Erd-Encounter-Epochen stehen jetzt als `src/archivar/flyby_encounters.rs`
  (`EARTH_FLYBYS`, chronologisch, `earth_flybys_by_spacecraft`/`earth_flyby_unix`, Tests;
  `cargo check` 0/0). Die vier ODF-`url`-Zeilen tragen kein Encounter-Fenster; kein Konsument
  liest eines.
- **Blockade:** die vier ODF-Quellen sind request-only (DSN/JPL) — ohne Rohdaten kein
  Rekonstruktions-Konsument.
- **Braucht:** ODF-Rohdaten über Future; danach den Rekonstruktions-Leser auf `EARTH_FLYBYS`
  aufsetzen.

### `auftrag-flyby2-kette` — σ-Metrik
- **Status:** wartend | **Bindung:** eigener Trigger extern
- **Trigger:** `docs/paper/flyby-path-2-addendum-2026-09-29.md` — JUICE in-situ + Δ publiziert.
- **Lage:** (gemessen 2026-09-29) Addendum trägt die 26-Zellen-Tubus-Registrierung; σ-Metrik pending.
- **Blockade:** externe Publikation.
- **Braucht:** bei Publikation `flyby_ephemeris_gate` (CI) gegen das Addendum.

### DEMETER
- **Status:** termin | **Bindung:** termin:2026-10-05
- **Trigger:** Order-Ablauf 2026-10-05 / Datei-Endpoint 200.
- **Lage:** (gemessen 2026-09-29) Riss `UA-Riss 403/403 vs 403/684 (orderToken)` in
  `phi/blocked_sources.φ`; Träger `state/zustand/wartend.φ`.
- **Blockade:** CDPP-Order.
- **Braucht:** Order-Ablauf abwarten.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte `state/zustand/wartend.φ`)
- **Trigger:** `NED_BYPARAMS_TIMEOUT_TOKEN` per Mail.
- **Lage:** (gemessen 2026-09-30) Dave Cook (NED) bot den Timeout-Token an; Token nicht eingetroffen;
  `ned-byparams-cdn` „success" = 6 s (kein Job-Lauf). **Wort:** „der ned folowup ist nicht lange her" |
  2026-09-30 | Operator — keine erneute Vorlage.
- **Blockade:** Token fehlt.
- **Braucht:** mit Token den ByParams-Job fahren.

### CDSE-CCM STAC — Arm gebaut, Auth-Download bei Mycelium
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mycelium-Auth-Messung (Asset 200).
- **Lage:** (gemessen 2026-09-30) `/stac/collections` 200; Arm `src/archivar/stac.rs`
  (`parse_collection_ids`/`parse_items`/`select_asset`, Tests) + bin `stac_asset_fetch.rs`
  gebaut, `cargo check` 0/0; live gelesen: CCM-Kollektionen
  `ccm-optical`/`ccm-sar`/`ccm-thermal-lst-{hr,mr}`/`ccm-hyperspectral-ref-hr`. Die OData-Route
  `…/odata/v1/Products` liegt `declined` (`declined_sources.φ:1158`).
- **Blockade:** der authentifizierte Asset-Aufruf (Konto) liegt bei Mycelium.
- **Braucht:** Mycelium fährt `stac_asset_fetch --asset <href>` mit `CDSE_TOKEN` (200 = nutzbar);
  danach ttl/frame + `url`/`format`.

## Prosa-Träger (eigene)

- `docs/specs/livefeed-gate.md` | offene Marker = die `pending`-Felder der Ereignis-Tabelle.
- `docs/surveys/survey-raetsel-bestand.md` | Verdikt-Träger (Rats-Konsens 2026-09-29).
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | Marker `:87` astroquery-Gegenprobe.
- `docs/concepts/arxiv-api.md` | Quellen-Zugangsweg `:59`/`:65-67` (sensory-folge207).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | `:28-141`.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | `:52-70`.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | `:56`/`:74`/`:139`.

## An mycelium

Origin: mountain folge211.

- **USGS-comcat Manifestation:** Workflow für `usgs_comcat_m45.bin` (Tag `earthquake.usgs.gov`) —
  Quelle `url https://earthquake.usgs.gov/fdsnws/event/1/query`; Mountain schreibt `url`/`format`.
- **`tao-wnd-cdn.yml` Zeile 35-38/58** auf das volle Record (1977-11-06..now) — der Compiler lädt
  jetzt voll, der Workflow schneidet noch; Release-Asset vor dem Dispatch löschen.
- **`gosat-cdn` Re-Dispatch:** der registrierte Fix (Empty/Overflow getrennt, 2026-09-27) wartet auf
  einen neuen Lauf; `GWT3F_L1B` liefert aktuell 0 Treffer.
- **`pds4-binary-cdn.yml` Quellenzuordnung:** ExoMars ACS HK ist `Table_Character` (Arm
  `pds4_fixed_width`), nicht `Table_Binary` — Quelle/Arm im Workflow korrigieren.
- **CDN-Manifestationen** (Chang'e MRM, DAS2) sobald Arm/Register stehen.
- **CDSE-CCM STAC-Auth-Download:** der Arm steht (`src/archivar/stac.rs` + bin `stac_asset_fetch.rs`,
  `--collections|--items|--asset`, Token aus `CDSE_TOKEN`). Bitte mit dem Konto
  `stac_asset_fetch --asset <href>` fahren (200 = nutzbar) und melden; danach schreibt Mountain
  ttl/frame + `url`/`format`.

## An future (Operator-Queue, private)

Origin: mountain folge211.

**Bitte bevorzugt vorlegen, sobald der Operator spricht:**
- **ODF-Flyby-Fenster fehlt** — DSN/JPL-Rohdaten-Anfrage stellen?
- **Sonden-Download-Session** — Operator-Browser-Session für die fünf `released`-Konten?
- **opencode-Config Secrets** (aus folge210) — Env-Export + Rotation.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
