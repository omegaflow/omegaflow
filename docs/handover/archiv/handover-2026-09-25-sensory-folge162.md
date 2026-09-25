<!--
  title: Handover — Sensory-Folge 162 (Stand 2026-09-25)
  session: Sensory-Folge 162
  class: handover
  date: 2026-09-25
  sha256: ec54742b89df72f6011ddeec90c5b15fda48eae8dbd33259fa00f1a7737e2619
  status: live
-->
# Handover — Sensory-Folge 162 (2026-09-25)

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

### BLE GFDI-StartNotify-Schleife reißt die Bus-Verbindung ab
- **Status:** eigen (autonom) | **Bindung:** eigen
- **Trigger:** nächste Session (kein externer Trigger)
- **Lage:** (gemessen 2026-09-25 via `OMEGAFLOW_BLE_HR=AA:BB:CC:DD:EE:FF
  OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow`) Bus-Auth, `Hello`, `AddMatch`,
  `GetManagedObjects`, Geräteauflösung und `StartNotify` auf `0x2a37` laufen jetzt
  fehlerfrei; die anschließende GFDI-Schleife über
  `service0014/char0015…char002e` liefert je `reply void`, danach `the system bus
  connection closed — reconnecting`; `sensor: 0 samples` (Uhr sendet HR, per
  Operator-Wort bestätigt).
- **Blockade:** keine.
- **Braucht:** isolieren, warum `bus.call` für die GFDI-`StartNotify`-Aufrufe
  `None` (read/EOF) liefert — Kandidat: BlueZ lehnt `StartNotify` auf den
  Vendor-Charakteristiken ab bzw. eine nicht parsebare `PropertiesChanged`-Antwort
  schließt die Verbindung; Messung: einen einzelnen GFDI-`StartNotify` frame-genau
  protokollieren (`next_message`-Rohdaten) mit `./target/debug/omegaflow`.

### BLE-HR-Live-Messung FR945 (Transport geheilt — Beat noch nicht geflossen)
- **Status:** eigen (autonom, Messung) | **Bindung:** eigen
- **Trigger:** nächste Session
- **Lage:** (gemessen 2026-09-25) FR945 `Connected: yes`, HR-Broadcast am Gerät an;
  die fünf Bus-Bugs in `src/archivar/ble.rs` sind geheilt (EXTERNAL-Auth =
  Hex-Kodierung der Dezimal-uid; uid aus `/proc/self/status`; Message-Gesamtgröße =
  `header_end + body_len`, Body nicht auf 8 gepolstert; Array-Länge ab dem ersten
  Element nach dem Alignment; Grundtypen `n/q/i/x/t/d`). Der Lauf erreicht Gerät +
  `0x2a37`-`StartNotify`, empfängt aber keine Notification (`sensor: 0 samples`).
- **Blockade:** hängt am Punkt oben (GFDI-Schleife reißt die Verbindung).
- **Braucht:** erst die GFDI-Schleife heilen/überspringen, dann
  `OMEGAFLOW_BLE_HR=AA:BB:CC:DD:EE:FF OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow`
  (~45 s) und die `sensor:`-/`gfdi_line`-Zeilen lesen; der funktionale Lauf gehört
  in CI, nicht auf den Operator-Rechner.

**Wartend**

### ble-Tests grün — CI-Bestätigung (Auth/Framing/Array/Typen)
- **Status:** wartend | **Bindung:** eigen (CI)
- **Trigger:** Lauf-Ende des nächsten `ci-check` auf einem HEAD mit diesem Atom
- **Lage:** (gemessen 2026-09-25 via `cargo check --tests`) 0/0; die
  `managed_objects`-/`properties_changed`-Fixtures wurden auf die
  Elemente-only-Array-Länge (Spezifikation + realer Daemon) korrigiert. Der Lauf
  `ci-check 36104792061` @`64b4bd205` trug die Auth-Heilung nicht.
- **Blockade:** keine.
- **Braucht:** nach Commit+Push `gh workflow run ci-check.yml`, dann `ci_manage log
  <id>` einmal lesen; bei Rot denselben Punkt erneut.

### Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** operator (Ankunft) + Rat (ZigBee)
- **Trigger:** Ankunft (`LZ473049629CN`) + ZigBee-Stack-Wort
- **Lage:** BL808-Port ungebaut; kein Wertmaß ohne Protokollwahl (gemessen 2026-09-23).
- **Blockade:** Ankunft + Protokollwort.
- **Braucht:** Ankunft abwarten; Rat für ZigBee-Protokoll, dann Daemon-Port.

**Operator-gebunden**

### FIT-Verifikation FR945 — Akt (echte Datei)
- **Status:** operator-gebunden | **Bindung:** operator (Datei)
- **Trigger:** Operator liefert den Pfad einer echten FR945-`.fit`-Datei
- **Lage:** (gemessen 2026-09-25 via `grind-flash`) `emit_nn` über Message-Grenzen
  geheilt, `archivar::fit::tests::real_fit_sample_parses` (`#[ignore]`) gebaut;
  Developer-Fields-Pfad auf echtem File unverifiziert.
- **Blockade:** kein `.fit`-File im Baum.
- **Braucht:** `OMEGAFLOW_FIT_SAMPLE=/abs/path/FR945.fit cargo test -p omegaflow --lib archivar::fit::tests::real_fit_sample_parses -- --ignored --nocapture`

**LOCK**

### HRV/Puls→Strahlung (physischer Träger)
- **Status:** LOCK | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-24 via `sgrep`) `sgrep -i hrv|pulse|rmssd` in
  `firmware/` = 0 Treffer; HRV/Puls-Bindung `pending` per AGENTS.md.
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-23) Firmware Mux-Sweep + SpO2/GNSS-`sensor_config`
  gebaut; GNSS fehlt im BOM (`docs/specs/mantis-shrimp-bom.md`).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
