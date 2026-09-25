<!--
  title: Handover — Sensory-Folge 163 (Stand 2026-09-25)
  session: Sensory-Folge 163
  class: handover
  date: 2026-09-25
  sha256: 2a294dd5b51a67420b5a271a70bd9b200a2c1fb8139b77228280122aac9acacd
  status: live
-->
# Handover — Sensory-Folge 163 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage**
(mit Messstempel) / **Blockade** / **Braucht**.

## Offen (aufgeschlüsselt)

**Autonom**

### FIT-Parser: unplausible `nn` aus Event-Timestamp-Diskontinuitäten
- **Status:** eigen (autonom) | **Bindung:** eigen
- **Trigger:** nächste Session
- **Lage:** (gemessen 2026-09-25 via `OMEGAFLOW_FIT_SAMPLE=… cargo run -p omegaflow --bin omegaflow`) das Garmin-SDK-Sample `activity_poolswim_with_hr.fit` parst zu 7570 `nn`, alle finit, aber `nn min 123.0 ms` (< `NN_MIN_MS` 250) und `nn max 2119005.9 ms` (~35 min) — `parse_fit` führt die Lücke als ein `nn`; das HRV-Gate (`src/archivar/hrv.rs:29`) filtert sie.
- **Blockade:** keine.
- **Braucht:** prüfen, ob `emit_nn` in `src/archivar/fit.rs` eine Event-Timestamp-Diskontinuität (Pause/Reset/12-bit-Rollover) als Kettenbruch statt als Riesen-`nn` führen soll; Test in `fit.rs` `#[cfg(test)]`.

### FIT-Sample: Lizenz + CI-Verdrahtung
- **Status:** eigen (autonom, Recherche) | **Bindung:** eigen
- **Trigger:** nächste Session
- **Lage:** (gemessen 2026-09-25 via `archive_search --playwright`) das Sample stammt aus `tormoder/fit` `testdata/fitsdk/` (MIT-Repo), die Dateien selbst sind Garmin-FIT-SDK-Beispiele; Garmin-SDK-Weiterverbreitung `unverified`. Lokal unter `data/fit-sample/` (gitignored, nie getrackt), nicht auf dem CDN. Der `#[ignore]`-Test `real_fit_sample_parses` (`src/archivar/fit.rs:555`) läuft nicht in CI.
- **Blockade:** Lizenz unverifiziert.
- **Braucht:** Garmin-FIT-SDK-Lizenz messen (`archive_search --verdict`/`--playwright` auf die SDK-EULA) — bei erlaubter Weiterverbreitung CDN-Asset + CI-Verdrahtung des `#[ignore]`-Tests; sonst lizenzfreies Fixture oder `descoped` per Messung.

**Operator-gebunden**

### BLE-HR-Live-Messung FR945 (Wire geheilt — Beat noch nicht geflossen)
- **Status:** operator-gebunden (Akt: Hardware/Radio) | **Bindung:** operator
- **Trigger:** Operator startet den verdeckten Lauf am gekoppelten Gerät
- **Lage:** (gemessen 2026-09-25 via `cargo check --tests` + Code) der GFDI-Bus-Abriss ist geheilt: `characteristic_matches` liefert `(path, uuid)`, die GFDI-Schleife destrukturierte vertauscht `(uuid, path)` und sandte die UUID als OBJECT_PATH → dbus-daemon verwarf die Verbindung (`src/archivar/ble.rs:1360/1418`); Fix + zwei Fixture-Tests, `cargo check --tests` 0/0. Live-Bestätigung offen (kein BlueZ/Gerät in der Session).
- **Blockade:** Hardware/Radio — der verdeckte Lauf braucht das Operator-Wort.
- **Braucht:** `OMEGAFLOW_BLE_HR=F0:99:19:4E:0B:BF OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow` (~45 s); `sensor:`-/`gfdi_line`-Zeilen lesen.

### FIT-Verifikation eigene FR945-Datei (lokal-only)
- **Status:** operator-gebunden (Akt) | **Bindung:** operator
- **Trigger:** Operator startet den Lauf mit seiner Datei
- **Lage:** (gemessen 2026-09-25 via `cargo run -p omegaflow --bin omegaflow`) der Dump-Pfad ist gebaut (`src/main.rs:1-56`: liest `OMEGAFLOW_FIT_SAMPLE`, ruft `parse_fit`, druckt records/nn/min/max/all-finite, `exit(2)` bei Refusal); am SDK-Sample verifiziert.
- **Blockade:** keine (Vorbereitung gebaut).
- **Braucht:** `OMEGAFLOW_FIT_SAMPLE=/abs/pfad/FR945.fit cargo run -p omegaflow --bin omegaflow` — Datei bleibt auf diesem Gerät.
- **Wort:** „meine fits datei verlässt niemals dieses gerät" | 2026-09-25 | Operator (Session)

**Wartend**

### ble-Tests grün — CI-Bestätigung
- **Status:** wartend | **Bindung:** eigen (CI)
- **Trigger:** Lauf-Ende des `ci-check` auf dem Commit dieses Atoms
- **Lage:** (gemessen 2026-09-25 via `cargo check --tests`) 0/0; Fixtures gepinnt (`gfdi_match_names_the_path_before_the_uuid`, `error_reply_is_a_decline_for_the_calling_serial`). Der Lauf trägt den Fix erst mit dem neuen Commit.
- **Blockade:** keine.
- **Braucht:** nach Commit+Push `gh workflow run ci-check.yml`, dann `ci_manage log <id>` einmal lesen; bei Rot denselben Punkt erneut.

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
- **Lage:** BL808-Port ungebaut; kein Wertmaß ohne Protokollwahl (gemessen 2026-09-23).
- **Blockade:** Ankunft + Protokollwort.
- **Braucht:** Ankunft abwarten; Rat für ZigBee-Protokoll, dann Daemon-Port.

**LOCK**

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
- **Lage:** (gemessen 2026-09-23) Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut; GNSS fehlt im BOM (`docs/specs/mantis-shrimp-bom.md`).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
