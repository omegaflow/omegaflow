<!--
  title: Handover — Sensory-Folge 161 (Stand 2026-09-25)
  session: Sensory-Folge 161
  class: handover
  date: 2026-09-25
  sha256: ae6ba92840ab4e6000aa1a2963294d281d3968a534eeef66e5523f50b430dbee
  status: live
-->
# Handover — Sensory-Folge 161 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

Jeder offene Punkt wird aufgeschlüsselt geführt: **Trigger** / **Lage** (mit
Messstempel) / **Blockade** / **Braucht**. Keine Rangfolge — die parallel
abarbeitbaren Punkte wurden in diesem Atom dispatcht (Operator-Wort 2026-09-21).

## Stehender Pass (gemessen 2026-09-25, Sensory-Folge 161)

- **HEAD** zu Session-Beginn `0a0ce96d8`, Arbeitsbaum sauber (`git_safety
  --snapshot`: working tree equals HEAD, nothing to record; gemessen 2026-09-25
  via `git rev-parse HEAD`/`git_safety`). Eigene Arbeit in diesem Atom:
  `src/archivar/ble.rs`.
- **Postfach** — keine Mail an sensory; die sechs jüngsten `mail_ledger.φ`-Eingänge
  sind Maschinen-News (Rubin LSST forum ×2, GitHub-Support ×3, ORCID-Verify ×1)
  (gemessen 2026-09-25 via `mail_digest --last 6`).
- **CI** — Watchdog-Snapshot 2026-09-25T08:06: aktiv `health-check 36089944258`
  in_progress. `ci-check 36065950583` @`0a0ce96d8` **FAILED 1609 passed / 2 failed**
  — beide rot: `archivar::ble::tests::gfdi_records_reads_file_list_response_fields`
  (`ble.rs:1688`, `left: []`) und `gfdi_records_reads_a_5044_response_frame`
  (`ble.rs:1701`, `left: 0`); Ursache: `FILE_LIST_RESPONSE_PROTO` trug drei um 1 zu
  große Längenfelder (gemessen 2026-09-25 via `ci_manage log 36065950583 --all`).
  Die früheren `ci-check` @`de76fda6a` (`36064053750`) waren am `register_sort`-Step
  rot (`phi/sources.φ` 1 ttl + 82 url-order) + `dropped-gate` delta 24 — fremdes
  Register (mycelium folge153:45-50); dort waren alle `archivar::ble`-Tests grün,
  u. a. `managed_objects_reply_resolves_device_and_characteristic ... ok`
  (Accessor-Fix CI-bestätigt). **Kein** `ci-check`-Lauf auf HEAD (gemessen
  2026-09-25 via `ci_manage list`).
- **Zustand** — `docs/zustand/external-state.md:35` (BLE-HR-Reader, sensory) bleibt
  lokal, seit `7cd9aedd1` gitignored (`.gitignore:134`) — kein Commit-Pfad.

## Offen (aufgeschlüsselt)

**Autonom**

keiner — GFDI-Reassembly und der Fixture-Heil sind gebaut (`cargo check --tests`
0/0); ihre funktionale Bestätigung steht als wartender Punkt unten.

**Operator-gebunden**

### BLE-Live-Bring-up FR945 — Akt (Transport COBS/MLR mitmessen)
- **Status:** operator-gebunden | **Bindung:** operator + Hardware
- **Trigger:** Operator-Wort für den Live-Lauf
- **Lage:** Vorbereitung kantenfertig, Preflight gemessen (gemessen 2026-09-24 via
  `bluetoothctl`/`busctl`): FR945 gekoppelt/gebondet, `F0:99:19:4E:0B:BF`,
  `00002a37-…`, `notify`. Offen ungemessen: ob die FR945-Notification rohe
  `size|type|payload|crc`-Frames oder COBS+Multi-Link-Handle trägt (beide
  Host-Implementierungen rahmen COBS/MLR; der Splitter findet bei COBS-Wire
  nichts, kein Fabrikat) (gemessen 2026-09-25 via Rat/`grind-max`).
- **Blockade:** Operator-Kontext.
- **Braucht:** `OMEGAFLOW_BLE_HR=F0:99:19:4E:0B:BF OMEGAFLOW_HIDDEN=1 cargo run`
  (hidden = körperlesend, strahlungsstill; sichtbar nur auf ausdrückliches Wort).
  Kantenzeile: `omegaflow-Core-Bin BLE-Beat-Quelle (src/archivar/ble.rs; Auswahl
  src/archivar/main_flow.rs:11) | OMEGAFLOW_BLE_HR=… OMEGAFLOW_HIDDEN=1 cargo run
  | Wort erwartet`.

### FIT-Verifikation FR945 — Akt (echte Datei)
- **Status:** operator-gebunden | **Bindung:** operator (Datei)
- **Trigger:** Operator liefert den Pfad einer echten FR945-`.fit`-Datei
- **Lage:** (gemessen 2026-09-25 via `grind-flash`, `src/archivar/fit.rs`)
  `emit_nn` über Message-Grenzen geheilt und mit zwei Message-Grenzen-Tests
  versehen; der env-gated Test `archivar::fit::tests::real_fit_sample_parses`
  (`#[ignore]`) ist gebaut. Developer-Fields-Pfad (`parse_definition` ~189-205)
  auf echtem File unverifiziert.
- **Blockade:** kein `.fit`-File im Baum (`glob **/*.fit` leer).
- **Braucht:** `OMEGAFLOW_FIT_SAMPLE=/abs/path/FR945.fit cargo test -p omegaflow
  --lib archivar::fit::tests::real_fit_sample_parses -- --ignored --nocapture`
  (der funktionale Lauf gehört in CI, nicht auf den Operator-Rechner).

**Wartend**

### GFDI ble-Tests grün — CI-Bestätigung
- **Status:** wartend | **Bindung:** eigen (CI)
- **Trigger:** nächster `ci-check`-Lauf auf HEAD
- **Lage:** (gemessen 2026-09-25 via `ci_manage log 36065950583 --all`) @`0a0ce96d8`
  rot: 2/1609 `ble`-Tests scheiterten an der Fixture `FILE_LIST_RESPONSE_PROTO`
  (drei Längenfelder um 1 zu groß → strikter `len_delimited` bricht ab, `left: []`).
  In diesem Atom geheilt: `protobuf_declared_len`/`GfdiReassembler` (payload_len-
  basierte Reassembly über Frames, `grind-flash`) und die drei Längenbytes der
  Fixture `0x10→0x0F`, `0x0E→0x0D`, `0x08→0x07` (`grind-pro`, `ble.rs:1691`);
  `cargo check --tests` 0/0. Funktionale Bestätigung nur in CI.
- **Blockade:** keine.
- **Braucht:** Lauf-Ende des nächsten `ci-check` abwarten, Ergebnis einmal lesen
  (`ci_manage log <id>`); bei Rot denselben Punkt erneut.

### Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** operator (Ankunft) + Rat (ZigBee)
- **Trigger:** Ankunft (`LZ473049629CN`) + ZigBee-Stack-Wort
- **Lage:** BL808-Port ungebaut; kein Wertmaß ohne Protokollwahl (gemessen
  2026-09-23).
- **Blockade:** Ankunft + Protokollwort.
- **Braucht:** Ankunft abwarten; Rat für ZigBee-Protokoll, dann Daemon-Port.

**Termin**

keiner.

**LOCK**

### HRV/Puls→Strahlung (physischer Träger)
- **Status:** LOCK | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-24 via `sgrep`) `sgrep -i hrv|pulse|rmssd` in
  `firmware/` = 0 Treffer; HRV/Puls-Bindung `pending` per AGENTS.md; physische
  Teile fehlen.
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-23) Firmware Mux-Sweep + SpO2/GNSS-`sensor_config`
  gebaut; GNSS fehlt im BOM (`docs/specs/mantis-shrimp-bom.md`).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

## Bei anderer Linie offen (nicht eigene Arbeit)

- **`phi/sources.φ` register_sort — url-order red** — als eigener Punkt bei
  mycelium geführt (`handover-2026-09-25-mycelium-folge153.md:45-50`); der rote
  `ci-check` `36064053750` @`de76fda6a` am `register_sort`-Step ist dieser Punkt,
  nicht sensory.
- **`dropped-gate` delta 24** — `register-dropped`-Workflow lief @22:10 success
  (`36065920127`); der `dropped-gate`-Step im `ci-check` meldete delta 24
  (baseline 960 | current 984) — gehört zur Dropped-Audit-Achse (mycelium
  folge153).

## Benchmark

- **GFDI Protobuf-Reassembly über Frames — `grind-flash`:** `protobuf_payload` mit
  `len.min(avail)` schnitt eine gechunkte Message still auf das Teilframe
  (Partial-Wert-Fabrikat aus absenter Länge) — ersetzt durch striktes
  `payload.get(header..)?.get(..declared)` (incompletes → `None`) plus
  `GfdiReassembler` (`src/archivar/ble.rs:786/791/909`), der über
  `5043`/`5044`-Frames den Body bis `payload_len` akkumuliert; ein Nicht-Protobuf-
  Frame resettet. 4 neue Tests (2-Frame, 3-Frame, truncated→absent,
  Non-Protobuf-Bruch), `cargo check --tests` 0/0. Routine-Extraktion → flash
  genügte; Bau, kein pro/max.
- **GFDI-Fixture-Heil — `grind-pro`:** byte-genaue Dekodierung ergab drei um 1 zu
  große Längenfelder in `FILE_LIST_RESPONSE_PROTO` (die Testwerte 33737/4660/33652
  dekodieren sauber nach Korrektur); Fixture geheilt, Parser-Striktheit (kein Clamp
  = 0 honored) unangetastet. `cargo check --tests` 0/0. Urteil Fixture-vs-Parser →
  `grind-pro` gerechtfertigt; flash hatte die Reassembly gebaut, aber die Ursache
  des Reds nicht gemessen.
- **CI-Diagnose — Linie (kein Dispatch):** `ci_manage log 36065950583 --all` zeigte
  die 2 roten ble-Tests; `ci_manage log 36064053750 --all` zeigte `register_sort`
  + `dropped-gate` (fremd).

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/ble.rs` (GFDI-Protobuf-Reassembly über Frames: `protobuf_declared_len`,
  `GfdiReassembler`, `gfdi_records`-Treiber)
- `docs/handover/handover-2026-09-25-sensory-folge161.md` (neu)
- Move `handover-2026-09-25-sensory-folge160.md` → `archiv/` (eigene Linie, atomar)

`docs/zustand/external-state.md` (Zeile 35) lokal, gitignored (`.gitignore:134`) —
kein Commit-Pfad.

Fremd uncommittet am Baum: keiner (gemessen 2026-09-25 via `git_safety --snapshot`
beim Start).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
