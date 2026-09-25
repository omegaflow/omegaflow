<!--
  title: Handover — Sensory-Folge 166 (Stand 2026-09-25)
  session: Sensory-Folge 166
  class: handover
  date: 2026-09-25
  sha256: e157ee77da85a4b94957068e5334fddda9ebde9d7dea81cb1b794bf03d458c30
  status: live
-->
# Handover — Sensory-Folge 166 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage**
(mit Messstempel) / **Blockade** / **Braucht**. Sortierung: **erst logisch nach
Akteur (wer handelt) — Linie | Rat | Operator | Dritte —, dann chronologisch
(Messdatum)** (Operator-Wort 2026-09-25). Ein Punkt trägt genau einen Akteur:
verschiedene Verantwortlichkeiten werden bis zur Kante getrennt, nie gebündelt.

Die FR945 ist das persönliche Gerät des Operators; ihre Kennung (MAC) und ihre
Daten bleiben lokal (`.secrets.local`, `data/`), nie getrackt, nie am CDN. Der
getrackte Baum trägt nur die Rolle „Träger", nie die Kennung (MAC-Scrub
2026-09-25: `ble.rs`-Fixture auf den Platzhalter `AA:BB:CC:DD:EE:FF`).

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### HRV/Puls→Strahlung — Kette nach dem Live-Lauf beobachten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der BLE-Live-Lauf (Operator) hat `rr` geliefert
- **Lage:** die Kette ist gebaut (gemessen 2026-09-25 via `sgrep`): `src/archivar/hrv.rs:24-87` → `feed_beat_to_hrv` (`src/archivar/main_flow.rs:79-102`) → `tone_code` → `tone_scale` 0.25 (`src/mathematikerin/omega.rs:1670-1678`) → `aperture = field_permeability*tone_scale` (`omega.rs:349`) → Strahlung `Σω*aperture` (`src/mathematikerin/actuators.rs:29`).
- **Blockade:** hängt am BLE-Live-Fluss (Operator).
- **Braucht:** nach dem Live-Lauf `tone_code`→`tone_scale`→`aperture` im Frame / perm-Log lesen.

### Rat handelt

#### ZigBee-Stack-Weg für den BL808 (Ox64-Zweitknoten)
- **Status:** wartend | **Bindung:** Rat
- **Trigger:** M1 beantwortet (2026-09-25 erfüllt) + Ox64 angekommen
- **Lage:** M1 beantwortet (gemessen 2026-09-25 via `general`/flash): das BL808-Datasheet (`bl_docs`, `BL808_DS` v1.2, „Zigbee / IEEE 802.15.4") trägt 802.15.4 im 2,4-GHz-Transceiver — in `docs/specs/mantis-shrimp-bom.md` umgesetzt. Kein öffentliches BL808-ZigBee-SDK/NCP (`ncp-blz`/`zigpy-blz`/`bl_iot_sdk` = BL702/BL706-scoped; `bl_mcu_sdk` BL808 = 0 Treffer). Der Rat trägt die Wahl: BL70x-NCP-Dongle vs. Espressif-RCP (ESP32-H2/C6 `ot_rcp`) vs. eigener Stack.
- **Blockade:** kein Gerät + kein Stack-Beleg.
- **Braucht:** Ratssitzung (Architektur); danach M2 (nach Ankunft: ZigBee-Beispiel für den BL808 bauen).

### Operator handelt

#### BLE-HR-Live-Messung FR945
- **Status:** operator-gebunden (Akt: Hardware/Radio) | **Bindung:** operator
- **Trigger:** Operator startet den verdeckten Lauf am gekoppelten Gerät (945 am Arm)
- **Lage:** der GFDI-Bus-Abriss ist geheilt und **CI-bestätigt** (gemessen 2026-09-25 via `ci_manage log 36116592391`): alle `archivar::ble::tests` grün auf `45b4b2eb1`; Fix `src/archivar/ble.rs:1360/1418`. Live-Bestätigung offen (kein BlueZ/Gerät in der Session).
- **Blockade:** Hardware/Radio — der verdeckte Lauf braucht das Operator-Wort.
- **Braucht:** `OMEGAFLOW_BLE_HR=<FR945-MAC aus .secrets.local> OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow` (~45 s); `sensor:`-/`gfdi_line`-Zeilen lesen.

#### FIT-Verifikation eigene FR945-Datei (lokal-only)
- **Status:** operator-gebunden (Akt) | **Bindung:** operator
- **Trigger:** Operator startet den Lauf mit seiner Datei
- **Lage:** (gemessen 2026-09-25 via `cargo run -p omegaflow --bin omegaflow`) der Dump-Pfad ist gebaut (`src/main.rs:1-56`: liest `OMEGAFLOW_FIT_SAMPLE`, ruft `parse_fit`, druckt records/nn/min/max/all-finite, `exit(2)` bei Refusal); am SDK-Sample verifiziert. `emit_nn` (`src/archivar/fit.rs:264`) begrenzt `nn` auf `[NN_MIN_MS, NN_MAX_MS]` — eine Event-Timestamp-Diskontinuität ist `absent`, kein Riesen-`nn`; Test `hr_event_timestamp_discontinuity_is_absent`.
- **Blockade:** keine (Vorbereitung gebaut).
- **Braucht:** `OMEGAFLOW_FIT_SAMPLE=/abs/pfad/FR945.fit cargo run -p omegaflow --bin omegaflow` — Datei bleibt auf diesem Gerät.
- **Wort:** „meine fits datei verlässt niemals dieses gerät" | 2026-09-25 | Operator (Session)
- **Wort:** „die 945 von anderer Hardware trennen; meine Daten bleiben lokal" | 2026-09-25 | Operator (Session)

#### Onboard-/CIQ-Bedarf benennen
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort
- **Lage:** (gemessen 2026-09-24 via `sgrep`) der Host-Reader `parse_fit` (`src/archivar/fit.rs:78`) ist verdrahtet; CIQ hat keinen Reader (`sgrep -i ciq src tools` leer). Ein-Quellen-Regel: solange FIT läuft, sind BLE/Serial über die Arbitrierung ausgeschlossen.
- **Blockade:** keine — bewusst `pending` (Ein-Quellen-Regel).
- **Braucht:** Bedarf Ja/Nein — Nein → der Punkt ist released, `FIT_DIR` bleibt der FIT-Kanal.

#### Beat-Arbitrierung — verdeckter Lauf mit zwei Beat-Quellen
- **Status:** operator-gebunden (hidden) | **Bindung:** operator
- **Trigger:** Operator startet einen verdeckten Lauf mit zwei gesetzten Beat-Quellen
- **Lage:** die Spawn-Arbitrierung steht (`src/archivar/main_flow.rs:504`); kein Lauf hat die Verdict-Zeile (`beat source: …`) gemessen erzeugt (gemessen 2026-09-24 via `sgrep`). Die reine Funktion ist CI-getestet.
- **Blockade:** Heavy compute (CI/Operator) — eine Live-Messung braucht CI oder Operator-Wort.
- **Braucht:** `OMEGAFLOW_HIDDEN=1`-Lauf mit zwei Quellen, der genau eine `beat source:`-Zeile zeigt.

#### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-23) Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut. Die GNSS-BOM-Lücke ist geschlossen: `docs/specs/mantis-shrimp-bom.md` trägt ATGM336H GNSS (GPS+BDS, UART, EEPROM) `1005009361234427`, 3,01 CHF ≈ 3,20 €* (gemessen 2026-09-25 via `archive_search --playwright`; UART/3,3 V statt Mux-Weg).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** Operator-Wort (LOCK-Aufhebung), dann Bestellung der BOM.

#### ESP32-Puls-Knoten (Träger ohne Uhr)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25 via `sgrep`) Code-Eingang `BeatSource::Serial` steht; dedizierter Puls-Knoten unbestellt.
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —
- **Wort:** ESP32 separat als eigener LOCK | 2026-09-25 | Operator (Session)

### Extern handelt (Dritte)

#### Ox64-Lieferung
- **Status:** wartend | **Bindung:** termin (Carrier)
- **Trigger:** Ankunft (`LZ473049629CN`)
- **Lage:** PINE64 versandte zwei Ox64 (gemessen 2026-09-25 via `state/mail/mail_ledger.φ`, Mail `1790046330`) — Ankunft offen.
- **Blockade:** Carrier.
- **Braucht:** Ankunft quittieren; dann M2 + der Rat-Stack-Weg.

#### Postfach
- **Status:** wartend | **Bindung:** extern (Mail)
- **Trigger:** neuer Eingang
- **Lage:** (gemessen 2026-09-25 via `read`/`awk` auf `state/mail/mail_ledger.φ`) Ledger **vorhanden** (153 Zeilen), jüngster Eingang `1790328682` (Exa-Login-Code); `sgrep` relay/sensor/beat/mantis/ble/hrv = 0 Treffer. Der `mail_digest`-Befund „ledger absent" ist ein Pfad-Artefakt (Tool lokal nicht gebaut; `state/` = privates Repo `omegaflow/personal`).
- **Blockade:** keine.
- **Braucht:** `smail_recv` bzw. `state/mail/mail_ledger.φ` bei Trigger.

### Released (kein offener Punkt)

#### ble-Tests grün — CI-bestätigt
- **Status:** released | **Bindung:** eigen
- **Lage:** (gemessen 2026-09-25 via `ci_manage log 36116592391`) alle `archivar::ble::tests` grün auf `45b4b2eb1`; die Reds des Laufs sind fremd (`tools/utils/src/discovery.rs:197`, `register_sort.rs`-Format).
- **Braucht:** —

#### FIT-SDK-Sample → CI/CDN
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-25 via `archive_search --playwright`/`--verdict`) die Garmin „FIT Protocol License Agreement" §2c verbietet „distribute, publish, transfer or otherwise make available the Licensed Technology … to any third party"; es gibt keine Sample-Ausnahme (das Wort „sample" fehlt im Volltext). Quelle: `garmin/fit-cpp-sdk/LICENSE.txt` (20 035 B). Der shared-SDK-Sample-Pfad (CDN + CI) wird **nicht gebaut** — das persönliche FR945-File läuft weiter über `OMEGAFLOW_FIT_SAMPLE` (lokal-only), `garmin_sdk_sample_parses` bleibt `#[ignore]` lokal.
- **Blockade:** keine — durch Messung released.
- **Braucht:** —

**PII (im Auftrag):** die FR945-MAC ist aus HEAD entfernt (Wert nur lokal in `.secrets.local`); der History-Rewrite (die Historie trägt sie weiter) + die umgesetzte MAC-Gate-Klasse (Platzhalter ausgenommen) leben in `docs/auftrag/auftrag-pii-history-rewrite.md` (Nachtrag 2026-09-25).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
