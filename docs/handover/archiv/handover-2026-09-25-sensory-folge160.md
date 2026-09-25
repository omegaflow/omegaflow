<!--
  title: Handover — Sensory-Folge 160 (Stand 2026-09-25)
  session: Sensory-Folge 160
  class: handover
  date: 2026-09-25
  sha256: c58ac87472e2eef39e530c3275e82fddd42f43a0d4617e44d06b2fd86d6898fb
  status: live
-->
# Handover — Sensory-Folge 160 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

Jeder offene Punkt wird aufgeschlüsselt geführt: **Trigger** / **Lage** (mit
Messstempel) / **Blockade** / **Braucht**. Keine Rangfolge — die parallel
abarbeitbaren Punkte wurden in diesem Atom dispatcht (Operator-Wort 2026-09-21).

## Stehender Pass (gemessen 2026-09-25, Sensory-Folge 160)

- **HEAD** zu Session-Beginn `de76fda6a` == `origin/main`, Arbeitsbaum sauber
  (gemessen 2026-09-25 via `git rev-parse HEAD`/`git status --short`); während der
  Session landete der fremde Commit `f2d84b124` (Mountain folge154) — HEAD jetzt
  `f2d84b124`. Eigene uncommittete Arbeit: `src/archivar/ble.rs`, `src/archivar/fit.rs`.
- **Postfach** — keine Mail an sensory; die sechs jüngsten `mail_ledger.φ`-Eingänge
  sind Maschinen-News (Rubin LSST forum ×2, STScI News, GitHub-Support ×3)
  (gemessen 2026-09-25 via `mail_digest --last 6`).
- **CI** — `ci-check 36064053750` @`de76fda6a` **in_progress** (gestartet 21:51Z,
  Stand 21:58Z); der rote `ci-check 36030755250` @`e05f8a419` liegt **vor** dem
  folge159-Fix `c015aa9b2` (gemessen 2026-09-25 via `ci_manage view`).
- **Zustand** — `docs/zustand/external-state.md:35` (BLE-HR-Reader, sensory) bleibt
  lokal, seit `7cd9aedd1` gitignored (`.gitignore:134`) — kein Commit-Pfad.

## Offen (aufgeschlüsselt)

**Autonom**

### GFDI Protobuf-Reassembly über Frames
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** — (jetzt handlungsfähig)
- **Lage:** der Frame-Splitter + std-only Protobuf-Walk ist gebaut (gemessen 2026-09-25 via `grind-max`, `src/archivar/ble.rs`); `protobuf_payload` deckt nur den Ein-Frame-Fall — `payload_len > avail` (gechunkte Proto-Messages) wird nicht reassembliert (gemessen 2026-09-25 via `sread`).
- **Blockade:** keine
- **Braucht:** `grind-flash` — in `src/archivar/ble.rs` einen Proto-Reassembly-Puffer über aufeinanderfolgende 5043/5044-Frames bauen (payload_len-basiert), durch die `v > 0.0`-Plausibilität; ohne Live-Daten sauber `pending`, kein Fabrikat.

**Operator-gebunden**

### BLE-Live-Bring-up FR945 — Akt (Transport COBS/MLR mitmessen)
- **Status:** operator-gebunden | **Bindung:** operator + Hardware
- **Trigger:** Operator-Wort für den Live-Lauf
- **Lage:** Vorbereitung kantenfertig, Preflight gemessen (gemessen 2026-09-24 via `bluetoothctl`/`busctl`): FR945 gekoppelt/gebondet, `AA:BB:CC:DD:EE:FF`, `00002a37-…`, `notify`. Offen ungemessen: ob die FR945-Notification rohe `size|type|payload|crc`-Frames oder COBS+Multi-Link-Handle trägt (beide Host-Implementierungen rahmen COBS/MLR; der Splitter findet bei COBS-Wire nichts, kein Fabrikat) (gemessen 2026-09-25 via Rat/`grind-max`).
- **Blockade:** Operator-Kontext.
- **Braucht:** `OMEGAFLOW_BLE_HR=AA:BB:CC:DD:EE:FF OMEGAFLOW_HIDDEN=1 cargo run`
  (hidden = körperlesend, strahlungsstill; sichtbar nur auf ausdrückliches Wort).
  Kantenzeile: `omegaflow-Core-Bin BLE-Beat-Quelle (src/archivar/ble.rs; Auswahl src/archivar/main_flow.rs:11) | OMEGAFLOW_BLE_HR=… OMEGAFLOW_HIDDEN=1 cargo run | Wort erwartet`.

### FIT-Verifikation FR945 — Akt (echte Datei)
- **Status:** operator-gebunden | **Bindung:** operator (Datei)
- **Trigger:** Operator liefert den Pfad einer echten FR945-`.fit`-Datei
- **Lage:** `emit_nn` über Message-Grenzen geheilt und mit zwei Message-Grenzen-Tests versehen (`hr_event_timestamp_links_across_messages`, `hr_event_timestamp_12_links_across_messages`); der env-gated Test `archivar::fit::tests::real_fit_sample_parses` (`#[ignore]`) ist gebaut (gemessen 2026-09-25 via `grind-flash`, `src/archivar/fit.rs`). Developer-Fields-Pfad (`parse_definition` ~189-205) auf echtem File unverifiziert.
- **Blockade:** kein `.fit`-File im Baum (`glob **/*.fit` leer).
- **Braucht:** `OMEGAFLOW_FIT_SAMPLE=/abs/path/FR945.fit cargo test -p omegaflow
  --lib archivar::fit::tests::real_fit_sample_parses -- --ignored --nocapture`
  (der funktionale Lauf gehört in CI, nicht auf den Operator-Rechner).

**Wartend**

### `ble`-Accessor-Heil — CI-Bestätigung
- **Status:** wartend | **Bindung:** eigen (CI)
- **Trigger:** Lauf-Ende von `ci-check 36064053750` @`de76fda6a` (trägt den Fix `c015aa9b2`)
- **Lage:** Accessor-Shape-Mismatch geheilt (`device_by_address` `ble.rs:579`, `characteristic_matches` `ble.rs:641`; Fixture `managed_characteristic` `ble.rs:1181` auf die wahre Wire-Form `Dict`) — `cargo check --tests` 0/0 (gemessen 2026-09-24 via `grind-max`); der Lauf ist noch nicht beendet (gemessen 2026-09-25 via `ci_manage view 36064053750`).
- **Blockade:** keine.
- **Braucht:** Lauf-Ende abwarten, Ergebnis einmal lesen (`ci_manage log 36064053750`); bei Rot denselben Punkt erneut.

### Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** operator (Ankunft) + Rat (ZigBee)
- **Trigger:** Ankunft (`LZ473049629CN`) + ZigBee-Stack-Wort
- **Lage:** BL808-Port ungebaut; kein Wertmaß ohne Protokollwahl (gemessen 2026-09-23).
- **Blockade:** Ankunft + Protokollwort.
- **Braucht:** Ankunft abwarten; Rat für ZigBee-Protokoll, dann Daemon-Port.

**Termin**

keiner.

**LOCK**

### HRV/Puls→Strahlung (physischer Träger)
- **Status:** LOCK | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** `sgrep -i hrv|pulse|rmssd` in `firmware/` = 0 Treffer; HRV/Puls-Bindung `pending` per AGENTS.md; physische Teile fehlen (gemessen 2026-09-24 via `sgrep`).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut; GNSS fehlt im BOM (`docs/specs/mantis-shrimp-bom.md`) (gemessen 2026-09-23).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

## Benchmark

- **GFDI-Frame-Decode — Rat + `grind-max`:** der Rat verdiktete den 26×f64-Vertrag
  (String bleibt Archivar-Sache/`Sample.name`, kein Kraftkanal trägt
  Geräte-Metadaten, FileSync→Field-Wire per Messung `descoped` — kein Konsument);
  `grind-max` baute CRC-16/ARC-Splitter + std-only Protobuf-Walk + FileSync-Records
  in den `tx`-Kanal, 11 Tests an der wahren Wire-Form (3 Referenz-Frames aus der
  Zig-Quelle), `cargo check --tests` 0/0. Harte Klasse → `grind-max` gerechtfertigt.
- **FIT `emit_nn` — `grind-flash`:** Message-Grenzen-Fehler bestätigt und geheilt
  (+74/−19), zwei Grenzen-Tests synthetisch; `cargo check --tests` 0/0. Routine-
  Extraktion → flash genügte.

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/ble.rs` (GFDI CRC-16/ARC-Frame-Splitter + std-only Protobuf-Walk + FileSync-Records in den `tx`-Kanal)
- `src/archivar/fit.rs` (`emit_nn`/`cumulative_timestamp_12` über Message-Grenzen, `HrStream`-Zustand)
- `docs/handover/handover-2026-09-25-sensory-folge160.md` (neu)
- Move `handover-2026-09-24-sensory-folge159.md` → `archiv/` (eigene Linie, atomar)

`docs/zustand/external-state.md` (Zeile 35) lokal, gitignored (`.gitignore:134`) — kein Commit-Pfad.

Fremd uncommittet am Baum, nicht angefasst (gemessen 2026-09-25 via `git status --short`):
`phi/sources.φ`, `tools/register/src/bin/register_lookup.rs`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
