<!--
  title: Handover — Sensory-Folge 154 (Stand 2026-09-23)
  session: Sensory-Folge 154
  class: handover
  date: 2026-09-23
  sha256: 513b9919deae5345f2db247c128069f8698bd14be3ec940cf52a15e5323f846c
  status: live
-->
# Handover — Sensory-Folge 154 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt — kein Register-Kürzel: **Trigger** (das
Ereignis/Datum/Wort/der Lauf, dessen Eintreffen den Punkt kippt — Status =
f(Trigger)) / **Lage** (der Zustand, gemessen, mit Messstempel) / **Blockade**
( woran es hängt, oder „keine") / **Braucht** (was es löst: der wörtliche,
kopierbare Schritt — Werkzeug/Datei/URL/Befehl/Operator-Wort). `operator-gebunden`,
`blockiert` und `wartend` werden benannt, nie dispatcht. Jeder Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin` | `LOCK`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung.

## Stehender Pass (gemessen 2026-09-23, Sensory-Folge 154)

- **HEAD** `5d4655e24` == `origin/main`; eigener Arbeits-Satz uncommittet bis
  `/commit`: `src/archivar/ble.rs` (neu), `mod.rs` (+`ble`), `main_flow.rs`
  (+`ble_ingress`-Spawn), `fit.rs` (+`event_timestamp_12`), `membrane.rs`
  (+SpO2/GNSS-`sensor_config`), `firmware/radiatorium/src/main.rs` (Mux-Sweep +
  `spo2=`), `firmware/radiatorium-lib/src/max30102.rs` (`spo2_from_samples`),
  `firmware/radiatorium-lib/src/ds18b20.rs` (rustfmt), `docs/specs/mantis-shrimp-build.md`
  (ds18b20-Zeile), neues Handover folge154 + Move folge153→`archiv/`. Fremd, nicht
  angefasst: `docs/handover/post.md`, `phi/sources.φ`, `src/archivar/ingress.rs`
  (clippy-if-let, unowned), river folge16-Move + `river-folge17`.
- **Postfach** — keine an sensory (Mail-Ledger `state/mail/mail_ledger.φ`, letzte
  Eingänge fremd/Maschine; `mail_digest absent`); `post.md` ohne `An sensory`.
- **CI** — `te-gate 35875025486` @`122d36ef2` **in_progress**; Neulauf
  `te-gate 35893882101` **pending** (gemessen 2026-09-23 17:49 via `ci_manage list`).
  Fremd: zahlreiche `ci-check`/`tools-build`/CDN-Läufe.
- **`git_safety --snapshot`** → Arbeitsbaum während der Session von Fremdlinien
  committet; eigener Satz bleibt offen bis `/commit`.

## Operator-Queue

Keine offene Frage. **Hardware-Bestellung ist LOCK** (Operator-Wort 2026-09-23,
für alle drei Beschaffungs-Punkte: HRV-Träger, Sensor-BOM, 945-ANT-Hardware) —
die Punkte werden nicht erneut vorgelegt.

## Geschlossen in dieser Session (git trägt sie)

- **BLE-Live-Reader gebaut** (`grind-max`): `src/archivar/ble.rs` — std-only
  BlueZ-D-Bus-Client (EXTERNAL-Auth, v1-Header-Marshalling, `Hello`/`AddMatch`/
  `GetManagedObjects`, Signal-Empfang; Gerät per MAC → `Connect` → GATT `0x180D`/
  `0x2A37` → `StartNotify`), `0x2A37`-Decode (HR 8/16-bit, RR 1/1024 s → `rr`);
  Ein-Quellen-Regel über `OMEGAFLOW_BLE_HR` (Konflikt mit `OMEGAFLOW_SERIAL_IN`
  benannt, BLE weicht). 14 Fixture-Tests, `cargo check` 0/0. Verdrahtet:
  `mod.rs` (+`pub mod ble`/`pub use`), `main_flow.rs:462` (+`ble_ingress`-Spawn).
  **Live-Bring-up gegen die FR945 bleibt hardware-gebunden `pending`.**
- **FIT `event_timestamp_12`** (`grind-flash`): Feld 10 decodiert (12-bit-Unpack +
  18-bit-Rollover-Carry, Garmin-Delta-Akkumulation) in `fit.rs`, Test
  `hr_event_timestamp_12_unpacks_and_carries_rollover`, `cargo check` 0/0.
- **Firmware-Sensor-Knoten** (`grind-pro`): TCA9548A-Mux-Sweep + je Sensor eine
  `key=value`-Zeile (`firmware/radiatorium/src/main.rs`); MAX30102-`spo2_from_samples`
  (`max30102.rs`, no-std `sqrt`, `0 < spo2 ≤ 100`-Gate); `sensor_config`-Einträge
  `spo2` (%/force 6/kernel 3) + GNSS `speed`/`track` (force 7/0), τ = `pending`.
  `cargo check` 0/0 (core + `radiatorium-lib` + esp-Firmware).
- **ds18b20-Spec-Zeile korrigiert** (`grind-flash`): `docs/specs/mantis-shrimp-build.md`
  — Treiber getrackt (`ds18b20.rs`/`one_wire.rs`), `main.rs:143` bindet GPIO7;
  physisches Bring-up `pending` (Hardware LOCK).

## Offen (aufgeschlüsselt)

### BLE GFDI-V2-Protobuf (Folgeschritt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Bedarf für Roh-Cluster-Daten über BLE
- **Lage:** BLE-HR-Route gebaut (s.o.); GFDI `6a4e2800` nur benannt, nicht gebaut
  (gemessen 2026-09-23).
- **Blockade:** keine.
- **Braucht:** `src/archivar/ble.rs` GFDI-Charakteristiken (`6a4e28xx`) +
  hand-dekodiertes Protobuf; Force-Gate-Klassifikation.

### BLE-Live-Bring-up (FR945)
- **Status:** operator-gebunden | **Bindung:** operator + Hardware (LOCK)
- **Trigger:** Operator-Wort für einen Live-Lauf am gekoppelten Gerät
- **Lage:** Codec + Session-Logik frame-getestet; Funkstrecke ungemessen
  (gemessen 2026-09-23).
- **Blockade:** Hardware/Operator-Kontext.
- **Braucht:** Live-Lauf mit `OMEGAFLOW_BLE_HR=F0:99:19:4E:0B:BF` am gekoppelten 945.

### FIT-Verifikation 945
- **Status:** wartend | **Bindung:** operator (Datei) / eigen (CI-Lauf)
- **Trigger:** echte 945-FIT-Aktivität + CI-Lauf der neuen Tests
- **Lage:** Parser inkl. `event_timestamp_12` gegen Garmin-Fixtures grün;
  945-spezifische Aktivität unverifiziert (gemessen 2026-09-23).
- **Blockade:** keine 945-Datei.
- **Braucht:** `ci-check`-Lauf der neuen Tests; echte `GARMIN/Activity/*.FIT`
  (`post.md` → future/Operator).

### te-gate n=1000 — `fpr-ksg`/`issue`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des Laufs `35875025486` (bzw. Neulauf `35893882101`)
- **Lage:** `flush`/`flare` nun success (Mountain folge143, `external-state.md:25`);
  `fpr-ksg`/`issue` offen; Läufe in_progress/pending (gemessen 2026-09-23 via
  `ci_manage list`/`view`).
- **Blockade:** Lauf nicht abgeschlossen.
- **Braucht:** `ci_manage log 35875025486 --all` (bzw. `35893882101`) nach Abschluss.

### HRV/Puls→Strahlung (physischer Träger)
- **Status:** LOCK | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** Firmware-Kette inkl. BLE gebaut (s.o.); physische Teile fehlen
  (gemessen 2026-09-23).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK (Hardware) | **Bindung:** operator (Beschaffung, LOCK)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut (s.o.); GNSS
  fehlt im BOM (`docs/specs/mantis-shrimp-bom.md`); Verifikation ohne Teile
  ungemessen (gemessen 2026-09-23).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —

### Flyby-Path-2-Kette
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** Perigäum 2026-09-28
- **Lage:** Retention gemessen — RTSW ~24 h (kritisch), ACE ~31 d, Kp ~7,4 d
  (→GFZ-Backfill), OMNI2 ~6 d Lag, Swarm frisch — `docs/auftrag/auftrag-flyby2-kette.md`
  §Retention (gemessen 2026-09-23).
- **Blockade:** externer Termin.
- **Braucht:** Fill-Run ≤ 24 h nach der ersten Perigäum-Zelle (RTSW-1m-Vorrat).

### NSE/Haug
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Dateieingang (Mail 2026-09-17)
- **Lage:** kein Dateieingang (gemessen 2026-09-23).
- **Braucht:** eingehende Datei lesen.

### Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** operator (Ankunft) + Rat (ZigBee)
- **Trigger:** Ankunft (`LZ473049629CN`) + ZigBee-Stack-Wort
- **Lage:** BL808-Port ungebaut; kein Wertmaß ohne Protokollwahl (gemessen 2026-09-23).
- **Braucht:** Ankunft abwarten; Rat für ZigBee-Protokoll, dann Daemon-Port.

## Benchmark

- **BLE-Reader (hartes Atom, std-D-Bus + GFDI):** `grind-pro` **leer/incomplete**
  (zweimal, kein `ble.rs`) → eskaliert auf **`grind-max`** — geliefert, `cargo check`
  0/0, 14 Tests. Verdikt: harte Protokoll-Atome tragen `grind-pro` nicht, `grind-max` trägt.
- **Firmware Sensor-Knoten:** `grind-pro` — geliefert, kein Gegenlauf.
- **FIT `event_timestamp_12`:** `grind-flash` — Routine, kein Gegenlauf.
- **ds18b20-Spec-Zeile:** `grind-flash` — Routine, kein Gegenlauf.

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/ble.rs` (neu)
- `src/archivar/mod.rs` (+`pub mod ble`/`pub use`)
- `src/archivar/main_flow.rs` (+`ble_ingress`-Spawn)
- `src/archivar/fit.rs` (+`event_timestamp_12`)
- `src/archivar/membrane.rs` (+SpO2/GNSS)
- `firmware/radiatorium/src/main.rs` (Mux-Sweep + `spo2=`)
- `firmware/radiatorium-lib/src/max30102.rs` (`spo2_from_samples`)
- `firmware/radiatorium-lib/src/ds18b20.rs` (rustfmt)
- `docs/specs/mantis-shrimp-build.md` (ds18b20-Zeile)
- `docs/handover/handover-2026-09-23-sensory-folge154.md` (neu)
- Move `handover-2026-09-23-sensory-folge153.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (gitignored; eigene BLE-Zeile)

Fremd uncommittet: `docs/handover/post.md`, `phi/sources.φ`, `src/archivar/ingress.rs`
(clippy-if-let, unowned), river folge16-Move + `river-folge17` — nicht angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent, nie das Commit-Wort.
