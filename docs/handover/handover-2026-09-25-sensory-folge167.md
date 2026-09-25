<!--
  title: Handover — Sensory-Folge 167 (Stand 2026-09-25)
  session: Sensory-Folge 167
  class: handover
  date: 2026-09-25
  sha256: 8236b76dc40f85e1483beaa01f5a3c47892219ff4bccf34f4e59a3148f9e9959
  status: live
-->
# Handover — Sensory-Folge 167 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage**
(mit Messstempel) / **Blockade** / **Braucht**. Sortierung: **erst logisch nach
Akteur (wer handelt) — Linie | Rat | Operator | Dritter —, dann chronologisch
(Messdatum)** (Operator-Wort 2026-09-25). Ein Punkt trägt genau einen Akteur:
verschiedene Verantwortlichkeiten werden bis zur Kante getrennt, nie gebündelt.

Die FR945 ist das persönliche Gerät des Operators; ihre Kennung (MAC) und ihre
Daten bleiben lokal (`.secrets.local`, `data/`), nie getrackt, nie am CDN. Der
getrackte Baum trägt nur die Rolle „Träger", nie die Kennung.

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### ZigBee M2a — Rust-ZNSP-Host-Skelett + `esp_zigbee_host` auf dem S3
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Rat-Verdikt 2026-09-25 (Route 2) — erfüllt
- **Lage:** Route 2 getragen (gemessen 2026-09-25 via research-max `archive_search`): `esp-zigbee-sdk` `examples/esp_zigbee_ncp` (Target esp32h2) ↔ `examples/esp_zigbee_host` (Target esp32s3 — die gebaute Topologie), `esp-ieee802154` 0.8.0 im esp-hal-Ökosystem, Wrapper Apache-2.0; H2-Zeile in `docs/specs/mantis-shrimp-bom.md`, Verdikt in `docs/specs/mantis-shrimp-build.md` (§Zweitknoten).
- **Blockade:** keine
- **Braucht:** `examples/esp_zigbee_host` (Target esp32s3) bauen + Rust-ZNSP-Host-Skelett (`firmware/radiatorium`, esp-hal: UART-Frame-Parser + Netzwerkbildung) als eigenen Bin anlegen — kein Funk, keine Hardware, CI-fähig.

#### HRV/Puls→Strahlung — Kette nach dem Live-Lauf beobachten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der BLE-Live-Lauf (Operator) hat `rr` geliefert
- **Lage:** die Kette ist gebaut (gemessen 2026-09-25 via `sgrep`): `src/archivar/hrv.rs:24-87` → `feed_beat_to_hrv` (`src/archivar/main_flow.rs:79-102`) → `tone_code` → `tone_scale` 0.25 (`src/mathematikerin/omega.rs:1670-1678`) → `aperture = field_permeability*tone_scale` (`omega.rs:349`) → Strahlung `Σω*aperture` (`src/mathematikerin/actuators.rs:29`).
- **Blockade:** hängt am BLE-Live-Fluss (Operator).
- **Braucht:** nach dem Live-Lauf `tone_code`→`tone_scale`→`aperture` im Frame / perm-Log lesen.

#### M2c — Rust-ZNSP-Host auf dem BL808 gegen das H2
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ox64 angekommen
- **Lage:** der Weg steht (Rat-Verdikt 2026-09-25): der Ox64 wird Host-CPU des Coordinators (ZNSP über UART), das H2 das Funkmodul; Ox64-UART-Pins gemessen via PINE64-Wiki (UART0 GPIO14/15 = Pin 1/2, UART1 GPIO16/17 = Pin 32/31). Buildroot-Bring-up ungemessen.
- **Blockade:** Ox64 liegt beim Carrier.
- **Braucht:** Buildroot-Bring-up (Wiki-Flashing-Pfad), dann derselbe Rust-ZNSP-Host (`std` + `serialport`) auf dem BL808 gegen das H2.

#### BL808-eigenes 802.15.4-Radio — registrierter `pending`-Faden
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** öffentlicher 802.15.4-Treiber-/Stack-Fund für den BL808
- **Lage:** kein öffentlicher Treiber (gemessen 2026-09-25 via research-max `archive_search`): `bl_iot_sdk` trägt für BL808 nur `bl808_wifi` (kein 802.15.4), BL808-RM ohne Wireless-Kapitel, PAC+SVD existieren; `openbouffalo/bouffalo_sdk_bl808` (README-Raw 404, Inhalt ungemessen → `pending`). Route 2 nutzt das Radio bewusst nicht (Rat-Verdikt); der Punkt fällt nie auf 0.0.
- **Blockade:** kein Treiber
- **Braucht:** bei Treiber-Fund `sgrep`/`archive_search --github bouffalo_sdk_bl808 802.15.4`; bis dahin keine Arbeit.

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
- **Lage:** (gemessen 2026-09-25) Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut. Die GNSS-BOM-Lücke ist geschlossen: `docs/specs/mantis-shrimp-bom.md` trägt ATGM336H GNSS `1005009361234427`, 3,01 CHF ≈ 3,20 €* (via `archive_search --playwright`). **Neu (Rat 2026-09-25): die BOM trägt die H2-Zeile** `ESP32-H2-DevKitM-1-N4` (NCP-Funkmodul, Coordinator-Radio; `1005008131868631` ≈ 6,25 $ unverified · DigiKey 26282483 9,68 $) — ein Beschaffungsakt, ein LOCK.
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** Operator-Wort (LOCK-Aufhebung), dann Bestellung der BOM (inkl. H2).

#### ESP32-Puls-Knoten (Träger ohne Uhr)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25 via `sgrep`) Code-Eingang `BeatSource::Serial` steht; dedizierter Puls-Knoten unbestellt. Vom Rat unberührt (anderes Gegenüber/Ziel — Route 2 berührt ihn nicht).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —
- **Wort:** ESP32 separat als eigener LOCK | 2026-09-25 | Operator (Session)

### Extern handelt (Dritte)

#### Ox64-Lieferung
- **Status:** wartend | **Bindung:** termin (Carrier)
- **Trigger:** Ankunft (`LZ473049629CN`)
- **Lage:** PINE64 versandte zwei Ox64 (gemessen 2026-09-25 via `state/mail/mail_ledger.φ`, Mail `1790046330`) — Ankunft offen.
- **Blockade:** Carrier.
- **Braucht:** Ankunft quittieren; dann M2c (BL808-Host-Port).

#### Postfach
- **Status:** wartend | **Bindung:** extern (Mail)
- **Trigger:** neuer Eingang
- **Lage:** (gemessen 2026-09-25 via `sread` auf `state/mail/mail_ledger.φ`) Ledger **vorhanden** (154 Zeilen), jüngster Eingang `1790328682`; `sgrep` relay/sensor/beat/mantis/ble/hrv = 0 Treffer. Der `mail_digest`-Befund „ledger absent" ist ein Pfad-Artefakt (Tool lokal nicht gebaut; `state/` = privates Repo `omegaflow/personal`).
- **Blockade:** keine.
- **Braucht:** `smail_recv` bzw. `state/mail/mail_ledger.φ` bei Trigger.

**PII (im Auftrag):** die FR945-MAC ist aus HEAD entfernt (Wert nur lokal in `.secrets.local`); der History-Rewrite (die Historie trägt sie weiter) + die umgesetzte MAC-Gate-Klasse (Platzhalter ausgenommen) leben in `docs/auftrag/auftrag-pii-history-rewrite.md` (Nachtrag 2026-09-25).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
