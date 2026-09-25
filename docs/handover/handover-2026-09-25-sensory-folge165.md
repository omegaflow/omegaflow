<!--
  title: Handover — Sensory-Folge 165 (Stand 2026-09-25)
  session: Sensory-Folge 165
  class: handover
  date: 2026-09-25
  sha256: 60bed93f293b31102251ae65eef225f056aa38c3dc34aad744cb0121b654e481
  status: live
-->
# Handover — Sensory-Folge 165 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage**
(mit Messstempel) / **Blockade** / **Braucht**.

## Offen (aufgeschlüsselt)

**Descoped (Messung)**

### FIT-SDK-Sample → CI/CDN
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-25 via `archive_search --playwright`/`--verdict`) die Garmin „FIT Protocol License Agreement" §2c verbietet „distribute, publish, transfer or otherwise make available the Licensed Technology … to any third party"; es gibt keine Sample-Ausnahme (das Wort „sample" fehlt im Volltext). Quelle: `garmin/fit-cpp-sdk/LICENSE.txt` (20 035 B). Der shared-SDK-Sample-Pfad (CDN + CI) wird **nicht gebaut** — das persönliche FR945-File läuft weiter über `OMEGAFLOW_FIT_SAMPLE` (lokal-only), `garmin_sdk_sample_parses` bleibt `#[ignore]` lokal.
- **Blockade:** keine — durch Messung released.
- **Braucht:** —

**FR945 — persönliches Gerät, von Projekt-Hardware getrennt, Daten lokal**

Die FR945 ist das persönliche Gerät des Operators. Ihre Kennung (MAC) und ihre Daten bleiben lokal (`.secrets.local`, `data/`), nie getrackt, nie am CDN. Der getrackte Baum trägt nur die Rolle „Träger", nie die Kennung (MAC-Scrub 2026-09-25: `ble.rs`-Fixture auf den Platzhalter `AA:BB:CC:DD:EE:FF`).

### BLE-HR-Live-Messung FR945 (Wire geheilt — Beat noch nicht geflossen)
- **Status:** operator-gebunden (Akt: Hardware/Radio) | **Bindung:** operator
- **Trigger:** Operator startet den verdeckten Lauf am gekoppelten Gerät
- **Lage:** (gemessen 2026-09-25 via `cargo check --tests` + Code) der GFDI-Bus-Abriss ist geheilt: `characteristic_matches` liefert `(path, uuid)`, die GFDI-Schleife destrukturierte vertauscht `(uuid, path)` und sandte die UUID als OBJECT_PATH → dbus-daemon verwarf die Verbindung (`src/archivar/ble.rs:1360/1418`); Fix + zwei Fixture-Tests, `cargo check --tests` 0/0, `./target/debug/omegaflow` gebaut. Live-Bestätigung offen (kein BlueZ/Gerät in der Session).
- **Blockade:** Hardware/Radio — der verdeckte Lauf braucht das Operator-Wort.
- **Braucht:** `OMEGAFLOW_BLE_HR=<FR945-MAC aus .secrets.local> OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow` (~45 s); `sensor:`-/`gfdi_line`-Zeilen lesen.

### FIT-Verifikation eigene FR945-Datei (lokal-only)
- **Status:** operator-gebunden (Akt) | **Bindung:** operator
- **Trigger:** Operator startet den Lauf mit seiner Datei
- **Lage:** (gemessen 2026-09-25 via `cargo run -p omegaflow --bin omegaflow`) der Dump-Pfad ist gebaut (`src/main.rs:1-56`: liest `OMEGAFLOW_FIT_SAMPLE`, ruft `parse_fit`, druckt records/nn/min/max/all-finite, `exit(2)` bei Refusal); am SDK-Sample verifiziert. Seit demselben Atom begrenzt `emit_nn` (`src/archivar/fit.rs:264`) `nn` auf `[NN_MIN_MS, NN_MAX_MS]` — eine Event-Timestamp-Diskontinuität ist `absent`, kein Riesen-`nn`; Test `hr_event_timestamp_discontinuity_is_absent`.
- **Blockade:** keine (Vorbereitung gebaut).
- **Braucht:** `OMEGAFLOW_FIT_SAMPLE=/abs/pfad/FR945.fit cargo run -p omegaflow --bin omegaflow` — Datei bleibt auf diesem Gerät.
- **Wort:** „meine fits datei verlässt niemals dieses gerät" | 2026-09-25 | Operator (Session)
- **Wort:** „die 945 von anderer Hardware trennen; meine Daten bleiben lokal" | 2026-09-25 | Operator (Session)

**Wartend**

### ble-Tests grün — CI-Bestätigung
- **Status:** wartend | **Bindung:** eigen (CI)
- **Trigger:** Lauf-Ende des `ci-check` auf dem Commit dieses Atoms
- **Lage:** (gemessen 2026-09-25 via `cargo check --tests`) 0/0; Fixtures gepinnt (`gfdi_match_names_the_path_before_the_uuid`, `error_reply_is_a_decline_for_the_calling_serial`). Der Push triggert `ci-check` selbst (`on: push`, Pfade `src/**`/`docs/**`): Lauf `36113209167` auf `d406c597c` (gemessen 2026-09-25 via `ci_manage view`, `pending`).
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36113209167` einmal lesen, sobald der Lauf endet; bei Rot derselbe Punkt erneut.

### HRV/Puls→Strahlung (Träger FR945)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der BLE-Live-Lauf liefert `rr`
- **Lage:** (gemessen 2026-09-25 via `sgrep`) die Kette ist gebaut: `src/archivar/hrv.rs:24-87` → `feed_beat_to_hrv` (`src/archivar/main_flow.rs:79-102`) → `tone_code` → `tone_scale` 0.25 (`src/mathematikerin/omega.rs:1670-1678`) → `aperture = field_permeability*tone_scale` (`omega.rs:349`) → Strahlung `Σω*aperture` (`src/mathematikerin/actuators.rs:29`); `BeatSource::Ble` → `ble_ingress` → `sensor_rx` (`main_flow.rs:502-513/1031-1033`).
- **Blockade:** hängt am BLE-Live-Fluss.
- **Braucht:** nach dem Live-Lauf `tone_code`→`tone_scale`→`aperture` beobachten (Frame / perm-Log).
- **Wort:** LOCK aufgehoben für den Träger FR945 | 2026-09-25 | Operator (Session)

### Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** operator (Ankunft) + Rat (ZigBee)
- **Trigger:** Ankunft (`LZ473049629CN`) + ZigBee-Stack-Wort
- **Lage:** (gemessen 2026-09-25 via Rat/`archive_search`) BL808-Port ungebaut; der Rat trägt **keine** belastbare Stack-Wahl: `bouffalolab/bl_iot_sdk` trägt ZigBee auf der **BL70X**-Familie, für den **BL808** kein Beleg; der ESP32-S3 hat kein 802.15.4-Radio → ohne Zusatzradio (ESP32-C6/H2 als RCP, `esp-zigbee-sdk` v2.x/ZBOSS) trägt die BOM-Zeile „Funk: … ZigBee" kein Mesh. Die BOM-Zeile ist entsprechend als ungemessen korrigiert.
- **Blockade:** Ankunft + Protokollwort + Zusatzradio-Entscheid.
- **Braucht:** M1 (ohne Gerät) — trägt das BL808-Radio 802.15.4? via `archive_search --playwright` auf PINE64-Ox64-Wiki/BL808-Datenblatt; dann Ankunft (M2: `bl_iot_sdk` ZigBee-Beispiel für BL808 bauen); Zusatzradio-Zeile in die BOM, falls Mesh.

### Beat-Arbitrierung: funktionale Verifikation im Betrieb
- **Status:** wartend | **Bindung:** eigen (hardware/versteckter Lauf; Arbitration in River `src/archivar/main_flow.rs`)
- **Trigger:** ein Beat-Quellen-Satz ist am Host gesetzt (Serial-Gerät + `OMEGAFLOW_BLE_HR`/`FIT_DIR`)
  und ein sichtbarer/hidden Lauf steht.
- **Lage:** die Spawn-Arbitrierung steht (`src/archivar/main_flow.rs:504`) (gemessen 2026-09-24 via
  sgrep), aber kein Lauf hat die Verdict-Zeile (`beat source: …`) gemessen erzeugt; `tests.rs` deckt
  nur die reine Funktion.
- **Blockade:** Heavy compute (Regel: CI, nie lokal) — eine Live-Messung braucht CI oder Operator-Wort.
- **Braucht:** `cargo test` in CI (Funktion) bzw. ein versteckter Lauf mit zwei gesetzten Quellen,
  der genau eine `beat source:`-Zeile zeigt.

### 945-FIT-Datei (Onboard nur FIT/CIQ)
- **Status:** wartend | **Bindung:** eigen (Ein-Quellen-Regel)
- **Trigger:** ein Onboard-FIT/CIQ-Auslesepfad wird gemessen nötig.
- **Lage:** der Host-Reader ist gemessen (gemessen 2026-09-24 via sgrep) — `parse_fit`
  (`src/archivar/fit.rs:78`), verdrahtet in `main_flow.rs`; CIQ hat **keinen** Reader
  (`sgrep -i ciq src tools` = leer); die Beat-Quellen-Arbitrierung liegt in River;
  `OMEGAFLOW_SERIAL_IN` steht (`src/archivar/ingress.rs:4`, `ble.rs:931`).
- **Blockade:** keine — bewusst `pending` (Ein-Quellen-Regel).
- **Braucht:** erste Messung bleibt: wird ein Onboard/CIQ-Pfad gebraucht? Sonst bleibt
  `FIT_DIR` der FIT-Kanal; BLE (`OMEGAFLOW_BLE_HR`) und Serial sind über die Arbitrierung
  ausgeschlossen, solange FIT läuft.

**LOCK — Projekt-Hardware**

### ESP32-Puls-Knoten (Träger ohne Uhr)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25 via `sgrep`) Code-Eingang `BeatSource::Serial` steht; dedizierter Puls-Knoten unbestellt.
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —
- **Wort:** ESP32 separat als eigener LOCK | 2026-09-25 | Operator (Session)

### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-23) Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut. Die GNSS-BOM-Lücke ist geschlossen: `docs/specs/mantis-shrimp-bom.md` trägt ATGM336H GNSS (GPS+BDS, UART, EEPROM) `1005009361234427`, 3,01 CHF ≈ 3,20 €* (gemessen 2026-09-25 via `archive_search --playwright`; UART/3,3 V statt Mux-Weg).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** Operator-Wort (LOCK-Aufhebung), dann Bestellung der BOM.

**PII (im Auftrag):** die FR945-MAC ist aus HEAD entfernt (Wert nur lokal in `.secrets.local`); der History-Rewrite (die Historie trägt sie weiter) + die umgesetzte MAC-Gate-Klasse (Platzhalter ausgenommen) leben in `docs/auftrag/auftrag-pii-history-rewrite.md` (Nachtrag 2026-09-25).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
