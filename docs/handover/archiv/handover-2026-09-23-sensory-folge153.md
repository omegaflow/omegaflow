<!--
  title: Handover — Sensory-Folge 153 (Stand 2026-09-23)
  session: Sensory-Folge 153
  class: handover
  date: 2026-09-23
  sha256: 7304ea5ab2df303ac3d909a2f9d673f1a931d86c60405e7912cdde3888849fcf
  status: live
-->
# Handover — Sensory-Folge 153 (2026-09-23)

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
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung.

## Stehender Pass (gemessen 2026-09-23, Sensory-Folge 153)

- **HEAD** `e0a0e52a5` == `origin/main`; eigener Arbeits-Satz uncommittet bis `/commit`: `src/archivar/fit.rs`
  (neu), `src/archivar/main_flow.rs` (+Fit-Spawn, +Routing-Helper),
  `src/archivar/mod.rs` (+`fit`), `docs/auftrag/auftrag-flyby2-kette.md` (Retention),
  `docs/handover/post.md` (2 eigene Zeilen),
  `docs/handover/handover-2026-09-23-sensory-folge153.md` (neu), Move folge152→`archiv/`.
  Fremd, in dieser Session von der Mountain-Linie committet: river folge11-Move,
  `river-folge12`, der Geräte-Survey. `post.md` wird von der Mountain-Linie
  **parallel** geschrieben (fremde `An mycelium`/`An research`-Zeilen) → **nicht in
  meinem Commit** (nur meine Zeilen stehen dort).
- **Postfach** — keine an sensory (`register_lookup --open`: 0 `owner=sensory`,
  0 `An sensory`); `post.md` trug fremde `An future:`×3 + `An river:` (nicht
  angefasst), um 2 eigene Zeilen ergänzt. Mailbox-Ledger nicht neu gemessen
  (letzte Messung folge152, gleicher Tag, A = A).
- **CI** — `te-gate 35834155918` @`7ed618d5a` **in_progress**: 11 Jobs success,
  `fpr-ksg` offen (404), `flare`-Job **rot**, `issue` pending (gemessen 2026-09-23
  via `ci_manage log --all`). `ci-check 35861931781` u.a. fremd.
- **`git_safety --snapshot`** → `refs/safety/1790171695`.

## Geschlossen in dieser Session (git trägt sie)

- **FIT-Brücke gebaut** (Rat F1; Operator-Wort „alle Oszillatoren"): `src/archivar/fit.rs`
  (std-only FIT-Leser: Header/Definition/Data/CRC/compressed-timestamp; `hr`-Mesg →
  `nn`, `record` → erkannte Keys), `fit_ingress` (env `FIT_DIR`), verdrahtet
  `main_flow.rs:435`, Modul `mod.rs:51/175`. 10 Tests; gegen echte Garmin-Fixtures
  (`fitparse-rs hrv-activity.fit`) verifiziert; `cargo check` 0/0.
  `event_timestamp_12` (Feld 10) nicht decodiert — benannt. Register-Akt:
  `post.md` An river (Faltung von Rivers folge12-Punkt).
- **Routing-Schluss HRV quellen-agnostisch** (Rat F2): Helper `feed_beat_to_hrv`
  (`main_flow.rs:55`) — beat-to-beat-Keys `nn`/`rr`/`ibi` erreichen
  `rmssd`/`VagusTone` aus **beiden** Kanälen (Sensor-Tupel + Browser-`Sample`);
  keine bpm→NN-Synthese.
- **HRV-Lage korrigiert**: der folge152-Satz „der Puls-Ankunfts-Pfad fehlt" ist
  **widerlegt** — die Kette ist gebaut (Firmware `nn=` → `serial_ingress` → `rmssd`
  → `VagusTone` → `tone_code` → `tone_scale`/Apertur; CI `esp32-firmware.yml` grün;
  DS18B20/`one_wire` getrackt). Es fehlen nur die **physischen Teile** + Bring-up.
- **Flyby-Retention gemessen** (`research-max`): vollständige Tabelle in
  `docs/auftrag/auftrag-flyby2-kette.md` §Retention; RTSW **~24 h** ist die harte
  Frist, OMNI2 lagt gemessen ~6 d (Auftrag-Annahme „~1–2 d" widerlegt).
- **te-gate-Teilernte** (`grind-flash`): 11 Jobs success; n=1000-Zahlen +
  `te_fn_probe` (skalar 10/10, topo 0/10 bei φ=0) + `ksg-k-gate` (`sweep void` →
  K bleibt 0) in `docs/zustand/external-state.md:25` + unten.

## Offen (aufgeschlüsselt)

### te-gate n=1000 — Lauf offen, `flare` rot
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des Laufs `35834155918`
- **Lage:** 11 Jobs **success**; `fpr-ksg` läuft, `issue` pending. Der `flare`-Job
  war rot (`src/mathematikerin/te.rs:6239`: 13/30 < 50 %); die **Mountain-Linie hat
  ihn übernommen** — per Rat-Verdikt den assert-Step entfernt, die power-probe
  (n ∈ {400,600,1000}) läuft allein, Auswertung an `research` (`post.md` An research)
  (gemessen 2026-09-23 via `ci_manage log --all`).
- **Blockade:** Lauf nicht abgeschlossen.
- **Braucht:** `ci_manage log 35834155918 --all` nach Abschluss → `fpr-ksg` + `issue`
  (die n=1000-KSG-Gates; `flare` ist geroutet, nicht dieser Punkt).

### FIT-Verifikation 945 + `event_timestamp_12`
- **Status:** wartend | **Bindung:** operator (Datei) / eigen (Decoder)
- **Trigger:** eine echte 945-FIT-Aktivität + CI-Lauf der neuen Tests
- **Lage:** Parser gegen öffentliche Garmin-Fixtures grün; 945-spezifisch
  unverifiziert; Feld 10 (`event_timestamp_12`) ungedecodiert (gemessen 2026-09-23).
- **Blockade:** keine Datei; CI-Testlauf steht aus.
- **Braucht:** `ci-check`-Lauf der neuen Tests; echte `GARMIN/Activity/*.FIT`
  (`post.md` An future).

### HRV/Puls→Strahlung (physischer Träger)
- **Status:** operator-gebunden | **Bindung:** operator (Beschaffung)
- **Trigger:** ESP32-Träger verfügbar / Operator-Wort
- **Lage:** Kette gebaut (s.o.); es fehlen ESP32-S3/MAX30102/DS18B20-Teile +
  Bring-up (gemessen 2026-09-23).
- **Blockade:** Beschaffung (`post.md` An future).
- **Braucht:** Teile bestellen/zusammenbauen; dann Live-Puls via ESP32.

### Live vom Forerunner 945 (Descope wieder offen — Operator-Bedürfnis)
- **Status:** operator-gebunden | **Bindung:** operator (Wort) + Rat
- **Trigger:** Operator-Wort für eine Live-Route (CIQ+ANT vs. BLE-Port)
- **Lage:** gemessen 2026-09-23.
  - *BLE-Sondierung (Operator-Wort „erst B", `bluetoothctl`/`busctl` an `hci0`):* die
    gekoppelte FR945 (`AA:BB:CC:DD:EE:FF`) verbindet sich mit dem Linux-BlueZ-Stack
    (`ServicesResolved: yes`) und exponiert HR `0x180D` (char `0x2A37`), RSC `0x1814`,
    Garmin **`6a4e2800`** (GFDI, ~11 Charakteristiken `6a4e28xx`, mehrere notify) +
    Garmin `6a4e8022` (`6a4e4c80`/`6a4ecd28`). → **BLE-Route auf der 945 bestätigt**;
    sie trägt (per `garmin-ble`) HR/HRV-RR/SpO2/Resp/Stress/BodyBattery/Accel, aber
    kein GNSS/Baro/Mag/Temp.
  - *Route-Übergabe (River → sensory, `post.md` An sensory, gefaltet 2026-09-23):* der
    945-BLE-Live-Reader ist per Operator-Wort sensorys; river hatte ihn mit `bluer`
    gebaut (zieht libdbus → `tools-build` rot) und nahm ihn zurück; std-only BlueZ-D-Bus
    trägt ihn. Haken `OMEGAFLOW_SERIAL_IN` (`src/archivar/ingress.rs:4`) liest einen
    tty-Pfad als nn-Quelle. **Ein-Quellen-Regel:** nur eine Beat-Quelle gleichzeitig.
  - *Route CIQ+ANT (voller Cluster, cloudfrei; `research-max`):* FR945 ist CIQ **3.3.0**;
    `Sensor.registerSensorDataListener` (Accel ab 2.3.0, Gyro/Mag 3.3.0),
    `Sensor.Info` (Baro/Temp/Höhe), `Position.enableLocationEvents` (GNSS),
    SpO2/HR onboard (3.2.0); TX via `Ant.GenericChannel.sendBroadcast` auf einem
    **public/private**-Netz (Master auf ANT+ verboten); Sideload `GARMIN/APPS`
    über USB (kein Store; Dev-Key RSA-4096, SDK-Login). Empfänger: Dynastream
    ANTUSB2/m (libusb — **nicht** `/dev/ttyACM*`) + `openant`, oder nRF52840+S340.
    **Kritisch ungemessen:** die TX-Kanalakquise der FR945.
  - *Route BLE (`gwerneckp/garmin-ble`, GFDI-V2-Protobuf):* live HR/HRV-RR/SpO2/
    Respiration/Stress/BodyBattery/Steps/Calories/Intensity/**Accelerometer** über
    Linux-BLE (`hci0`), Uhr vom Telefon entkoppelt; **kein** GNSS/Baro/Mag/Temp;
    FR945-Support ungemessen; Python → Rust-Port nötig (Python verboten) + BLE-Dep.
  - *Route LiveTrack-Cloud:* Position/Speed/Altitude/HR (~Sekundentakt), Garmin-
    Account + Telefon-App → `future`/Operator.
  - Gadgetbridge = Android-Sync/FIT (kein Linux-Live); python-garminconnect/openScale
    = kein Live.
- **Blockade:** CIQ = Monkey C + Garmin-SDK-Login/Dev-Key + ANT-Hardware (ANTUSB =
  libusb, **kein** tty); die **BLE-Route ist std-only baubar** — BlueZ spricht GATT
  über D-Bus, ein selbsttragender std-D-Bus-Client (Auth/Marshalling/`PropertiesChanged`)
  + GFDI-V2-Protobuf hand-dekodiert → **kein Crate**, Stack bleibt `std + curl +
  serialport`; nur größerer Protokoll-Aufwand. Empfangs-Allowlist bindend.
- **Braucht:** Bau `src/archivar/ble.rs` (std-D-Bus-Client + HR `0x2A37`→`nn`,
  GFDI als Folgeschritt) — der Bau läuft. Für den vollen Cluster CIQ+ANT
  (hardware-gebunden). Lizenz: `src/` bleibt PolyForm/NC (Operator-Wort 2026-09-23);
  OSI-Frage an `future`.

### Live-Sensor-Cluster (eigener Knoten)
- **Status:** wartend | **Bindung:** operator (Beschaffung) + eigen (Firmware)
- **Trigger:** BOM-Teile vorhanden / Operator-Wort für den Live-Cluster
- **Lage:** Die 945 live nur HR/R-R (BLE 0x180D/ANT+ HRM); Onboard-Cluster nur
  FIT/CIQ (Survey-Tabelle). **Live kommt über den eigenen Knoten** — und die BOM
  (`docs/specs/mantis-shrimp-bom.md`) trägt die Module schon: IMU **MPU6050**
  (`:32`), Magnetometer **QMC5883L** (`:22`), Druck/Baro **BME680**+**MS5803**
  (`:24,28`), Temperatur **DS18B20**/MLX90614, SpO2 **MAX30102** (`:34`, rot heute
  verworfen `main.rs:222`). **GNSS fehlt im BOM** — einziges Kanal-Modul ohne Teil.
  Der I2C-Mux TCA9548A hat 16 Wege, heute nur Weg 0 selektiert (`main.rs:114`).
  `sensor_config` (`membrane.rs:383`) löst temperature/pressure/magnet/accelerometer/
  gyro schon auf; SpO2 + GNSS brauchen neue Einträge (GPS/GNSS → heute `None`,
  `membrane.rs:444`). Firmware emittiert heute nur `nn=` (`main.rs:286`).
- **Blockade:** physische Teile; kein GNSS-Modul im BOM.
- **Braucht:** BOM bestellen (+1 GNSS-Modul); dann Firmware-Bau: Mux-Sweep +
  je Sensor eine `key=value`-Zeile (die `serial_ingress` schon parst); SpO2/GNSS in
  `sensor_config` ergänzen. Alternativen: 945-CIQ-App (onboard, descoped) /
  Browser-Cluster (River-LAN-Knoten).

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
- **Lage:** Route offen, kein Dateieingang (gemessen 2026-09-23 via Übergabe-Ref).
- **Braucht:** eingehende Datei lesen.

### Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** operator (Ankunft) + Rat (ZigBee)
- **Trigger:** Ankunft (`LZ473049629CN`) + ZigBee-Stack-Wort
- **Lage:** BL808-Port ungebaut; kein Wertmaß ohne Protokollwahl (gemessen 2026-09-23).
- **Braucht:** Ankunft abwarten; Rat für ZigBee-Protokoll, dann Daemon-Port.

### Stale Spec-Zeile (ds18b20)
- **Status:** wartend | **Bindung:** unbestimmt (kein Live-Handover nennt den Spec)
- **Trigger:** nächster Firmware-/Spec-Pass
- **Lage:** `docs/specs/mantis-shrimp-build.md` trägt eine „Measured gap: ds18b20"-Zeile;
  `firmware/radiatorium-lib/src/ds18b20.rs` + `radiatorium/src/one_wire.rs` sind
  getrackt, `main.rs:143` bindet GPIO7 (gemessen 2026-09-23).
- **Braucht:** Spec-Zeile korrigieren (Owner beim nächsten Pass).

## Geroutet / fremd (nicht sensory)

- **LAN-Knoten** (Pixel/Quest-Cluster, WS-Rückkanal → Vibration) — Rivers Rat-Atom,
  **operator-gebunden** (LAN-Exposition + Strahlung); sensorys Anteil = die
  Sensor-Cluster-Seite (Koordination via `post.md`).
- **Quest/Bigme/Pixel** — kein Puls-Hardware-Sensor (Quest IMU/Kameras; Handys kein
  HR) → nicht an HRV gebunden; Kamera-PPG = eigenes Atom, Operator-Wort nötig.
  Quest-Modell ablesen: Einstellungen → Über / `adb devices -l` (operator-domain).

## Benchmark

- **FIT-Parser (novel parser):** `grind-pro` (pro/high) — geliefert, `cargo check`
  0/0, gegen echte Fixture verifiziert; kein Gegenlauf (die Klasse „novel parser"
  ist als hart gelistet — pro trug sie; kein max-Doppel-Lauf nötig).
- **Sensor-Ingest-Map:** `explore`/flash — Routine, kein Gegenlauf.
- **te-gate-Ernte:** `grind-flash` — Routine, kein Gegenlauf.
- **Flyby-Retention:** `research-max` — mehrstufig (12 Kanäle, `--verdict`+`curl`),
  kein Gegenlauf.
- **Rat Sensor-Ingest/Transport:** `council` (pro/max) — Architektur, kein Gegenlauf.

## Geteilter Baum — eigener Pfad-Satz

- `src/archivar/fit.rs` (neu)
- `src/archivar/mod.rs` (+`pub mod fit`/`pub use`)
- `src/archivar/main_flow.rs` (+Fit-Spawn, +`feed_beat_to_hrv`/Routing)
- `docs/auftrag/auftrag-flyby2-kette.md` (Retention-Abschnitt)
- `docs/handover/post.md` (2 eigene Zeilen)
- `docs/handover/handover-2026-09-23-sensory-folge153.md` (neu)
- Move `handover-2026-09-23-sensory-folge152.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (gitignored; eigene TE-Gate-Zeile)

Fremd uncommittet: river folge11-Move (staged `R`), `?? river-folge12`,
`?? survey-…-radiatoren.md` — nicht angefasst, nicht gestaged.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent, nie das Commit-Wort.
