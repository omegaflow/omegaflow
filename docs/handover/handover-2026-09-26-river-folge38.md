<!--
  title: Handover — River-Folge 38 (2026-09-26)
  session: River-Folge 38
  class: handover
  date: 2026-09-26
  sha256: 977c701d4d73f4e4631a9cf856e78b77a7fcb2b86021a21245c2d3a5fdc5251a
  status: live
-->
# Handover — River-Folge 38 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** abgearbeitet. Sortierung: **erst logisch nach Akteur (wer handelt) —
Linie | Rat | Operator | Dritter**, innerhalb der Gruppe ohne Rang. Jeder Punkt
trägt **Trigger** / **Lage** (mit Messstempel) / **Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-26-river-folge37.md` (nach
`docs/handover/archiv/`).

**Wort | Datum | Quelle**
- Punkt 1 Chrome-Debugger aktivieren: **ja** | 2026-09-26 | Operator-Wort (diese Session).
- Funk-Sensor über HTTPS: **ja** | 2026-09-26 | Operator-Wort folge36.
- FIT-Brücke 945: **ja** — Garmin liefert .FIT-Dateien | 2026-09-26 | Operator-Wort folge36.
- Puls-Weg: **(b) Live-BLE nach dem Membran-Fix** | 2026-09-26 | Operator-Wort folge36.
- Geräte-Inventar: **Einzelbefehle liefern** | 2026-09-26 | Operator-Wort folge36.
- Mantis Shrimp (BOM): **LOCK — zuletzt** | 2026-09-26 | Operator-Wort folge36.
- Harte Läufe: **LOCK — verboten**, bis die Membran sauber lädt | 2026-09-26 | Operator-Wort folge36.
- Chrome-Debugger andocken: **ja** | 2026-09-26 | Operator-Wort folge36 („6 ja du darfst").
- Browser-Extension: **forken statt Dritten fragen** | 2026-09-26 | Operator-Wort folge36.
- vC-Permeabilität: **945 und Mantis Shrimp getrennt führen** | 2026-09-26 | Operator-Wort folge36.
- Session-Consent (Delegation) | 2026-09-26 | Operator-Wort: alles außer Harte Läufe; Commit trägt `/commit`.
- Sensory-Eigentum `src/archivar/llnl_g3d.rs`: **ja** — die untracked Datei gehört der Sensory-Linie | 2026-09-26 | Operator-Wort.

## Offen

### Linie handelt (eigen)

#### star-dmax-probe dispatchten + Artefakt falten (RISS-Zeuge)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Run `36252229964` liefert das Artefakt.
- **Lage:** (gemessen 2026-09-26 via git/SHA/`cargo check`) Probe-Bin `star_dmax_probe.rs` + Workflow `star-dmax-probe.yml` auf `main` (`f9ea3284`); alter Run `36240676551` = failure an der inzwischen **gefallenen** Blockade. `src/archivar/llnl_g3d.rs` ist jetzt **getrackt** (HEAD, blob `7d3fab1f`, sensory folge176), `main` kompiliert; `ci-check`-clippy-River-Lints (`spatial.rs:502`, `main_flow.rs:219/:857`) gefixt (`3b4c08d22`). Neu dispatcht: `36252229964`. Lokaler `--span`-Pass steht: `dr3_stars.bin` → COUNT 1.704.587, SPAN_M 1.798012e21, EPOCH_MIN=EPOCH_MAX=0.0; `cell_size_star ≈ 2.57e19 m`, `rho_star ≈ 8.2 pc`.
- **Blockade:** keine — `llnl_g3d` getrackt.
- **Braucht:** `ci_manage view 36252229964`, Artefakt falten (RISS-Zeile per Schwelle schließen). Der Harte-Läufe-LOCK fällt erst, wenn der Folgelauf die drei Umgehungen (`main_flow.rs:191-241`, `:1227-1258`, `:1299-1364/1758-1833`) auf derselben Hülle verifiziert hat.

#### health-check — Verdikt am Lauf 36237216821
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `36237216821` ist beendet.
- **Lage:** (gemessen 2026-09-26 via `ci_manage view`) `in_progress`, head `07d4b899`, `updated_at 11:58:15Z` — seither kein Fortschritt.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36237216821`, bei Rot `ci_manage log 36237216821`.

#### Sonnenfarbe color:measured — compute-only Renderpfad
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** lavapipe-ICD in CI vorhanden.
- **Lage:** (gemessen 2026-09-26 via `general`) Route (A) gescopt: Extraktor `browser_field_shader()` in `src/mathematikerin/tests.rs` herausziehen, neuer `#[ignore]`-Test, Pipeline `static/index.html:285-313` offscreen rekonstruieren (`Rgba8Unorm`, `compatible_surface: None`, Bind-Layout `:285-289`, Blend `one/one`+`add` `:299-313`).
- **Blockade:** lavapipe-ICD fehlt auf ubuntu-latest (Crosscheck-Tests skippen, `tests.rs:110-113`); `llnl_g3d` getrackt → cargo-CI grün.
- **Braucht:** nach llnl-Fix `mesa-vulkan-drivers`/lavapipe apt-Zeile + Test in `tests.rs` bauen; Verifikation, dass der measured-Zweig (LUT-Sampling) ausgeführt wird.

#### Ruhe-Ort als Hüllen-Zentrum — Lese-Kante bauen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Code-Kante gebaut.
- **Lage:** (gemessen 2026-09-26 via `council`) Rat einmütig **Ja** — der Ruhe-Ort (SSB-Origin) zählt als Hüllen-Zentrum; die ruhende Presence ist ein voll realisierter Zustand, keine Abwesenheit. Der Wert muss aus dem stehenden Slot gelesen werden (`presence_slot`, `main_flow.rs:693-696`), nie hartkodiert `[0,0,0]`.
- **Blockade:** keine (`llnl_g3d` getrackt).
- **Braucht:** Code-Kante bauen — Hüllen-Zentrum aus dem stehenden Slot statt aus `archive.presence`, so dass der Hidden-Lauf ohne Browser die Sterne trägt; CI-Test.

#### RX100 V5A — optische Quelle (Geräte-Anbindung)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** definierter Messbegriff (was misst die Kamera?) + Parser.
- **Lage:** (gemessen 2026-09-26 via Operator-Wort) Sony RX100 V (DSC-RX100M5A) vorhanden; optischer Sensor, kein Parser/SDK im Bestand. Operator-Prinzip: jedes Gerät ist anbindbar — Sensor **oder** Radiator.
- **Blockade:** Messbegriff und Parser fehlen (Sony Camera Remote API / PTP über WLAN/USB).
- **Braucht:** Messbegriff festlegen, dann Source-Port nach `docs/SOURCE_PORT.md` + `phi/sources.φ`.

#### Quellen-Verwerfung am Start — `#body` fehlt + Shard-Overlaps
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Messung, welche Einträge in `phi/sources.φ` überlappen.
- **Lage:** (gemessen 2026-09-26 via `bin/omegaflow`-Ausgabe + sread) zwei bewusste Verwerfungs-Pfade: (1) `src/archivar/main_flow.rs:566` „native body undeclared" — ohne `#body=<body>,<lat>,<lon>,<alt>` werden **alle** Stations-Samples verworfen; (2) `refuse_shard_overlaps` (`src/archivar/parse.rs:1529-1558`) — überlappende ODF/PODF-Shards werden pro Format **nicht gemergt**, jeder weitere wird „never merged" verworfen (drei Rosetta-ODF-Shards `[1080341864,1431699247)`, `[1431692248,1464772941)`, `[1464772941,1475187437)`). Der Code ist korrekt; Ursache ist die fehlende `#body`-Deklaration + überlappende Einträge im Register. **Ungemessen:** welche/wie viele Einträge konkret überlappen.
- **Blockade:** offene Messung (Overlap-Enumeration).
- **Braucht:** `#body=`-Übergabe prüfen; überlappende Rosetta-ODF-Einträge in `phi/sources.φ` auflisten und im Register entdoppeln (Oder durch Dispatch eines `explore`-Agenten).

### Rat handelt

#### Akustischer Radiations-Kanal — JBL-Kopfhörer, Bose SoundLink Mini
- **Status:** blockiert | **Bindung:** Rat
- **Trigger:** Rats-Verdikt über Form/Apertur des akustischen Radiators.
- **Lage:** (gemessen 2026-09-26 via Operator-Wort) beide Aktuatoren (Ton-Ausgabe) vorhanden; Ton im Vordergrund ist consent-gebunden und derzeit gesperrt (kein Test darf Audio emittieren). Operator-Prinzip: Aktuatoren sind so wichtig wie die Radiatoren — jedes Gerät anbindbar.
- **Blockade:** kein akustischer Radiations-Kanal im System; Foreground-Audio per Operator-Wort gesperrt.
- **Braucht:** Rats-Verdikt (Σω → Ton, Apertur/TE-Kopplung), dann Code-Kante.

### Operator handelt

#### Chrome DevTools MCP — Remote-Debugging aktivieren
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator aktiviert `chrome://inspect/#remote-debugging` und startet die opencode-Session neu.
- **Lage:** (gemessen 2026-09-26 via git diff) `opencode.json` MCP `--autoConnect` (`chrome-devtools-mcp@1.9.0`), `jq empty` grün.
- **Blockade:** keine.
- **Braucht:** in Chrome `chrome://inspect/#remote-debugging` einschalten, Session neu starten.
- **Wort:** Punkt 1 Chrome-Debugger aktivieren | 2026-09-26 | Operator-Wort („Ja").

#### Browser-Extension unpacked laden
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator führt den Load aus (Artefakt `chrome-mv3/` liegt vor).
- **Lage:** (gemessen 2026-09-26 via `ci_manage`) CI-Build grün — Lauf `36243288134` success (npm install --legacy-peer-deps + build + **85/85 Tests**), Artefakt `chrome-mv3/` hochgeladen (`head 39cfa06c3`).
- **Blockade:** keine.
- **Braucht:** `chrome://extensions` → Developer mode → „Load unpacked" → Fork-Unpacked-Verzeichnis.
- **Wort:** Extension forken | 2026-09-26 | Operator-Wort folge36.

#### Geräte-Inventar — Einzelbefehle ausführen
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator führt die Befehle am Gerät aus.
- **Lage:** (gemessen 2026-09-26 via `adb` + `chrome://gpu`) **Bigme HiBreak, Android 14**, adb autorisiert, `dumpsys sensorservice`: 21 Sensoren — ACCELEROMETER 12,5–400 Hz, GYROSCOPE (UNCALI_GYRO 12,5–400 Hz), MAGNETOMETER 5–50 Hz, LIGHT (on-change, 1 Hz), PROXIMITY (on-change, 1 Hz, wakeUp), GRAVITY/LINEARACCEL/ROTATION_VECTOR/GEOMAG 50–200 Hz, STEP_DETECTOR/COUNTER, TILT, WAKE_GESTURE, SIGNIFICANT_MOTION, DEVICE_ORIENTATION. Haptik (`vibrator_manager`, mId=1): Prebaked CLICK/DOUBLE_CLICK/TICK/HEAVY_CLICK/TEXTURE_TICK, **keine** Amplitude/Frequenz-Steuerung. **WebGPU: verfügbar** (`chrome://gpu` Chrome 153: „WebGPU: Hardware accelerated", Dawn-Vulkan-Backend Mali-G68 MC4 + ANGLE-OpenGLES beide „Available"; Features u. a. `core-features-and-limits`, `timestamp-query`, `shader-f16`, `subgroups`, `bgra8unorm-storage`). Display 440×879, sRGB, 8-bit, SDR-Weiß 203 nits. **Pixel 10a** (adb `stallion`, Android 17; Operator-Wort nannte „Pixel 9" — gemessen ist 10a): 38 Sensoren — ACCELEROMETER 1,5–400 Hz (ICM45631), GYROSCOPE 1,5–400 Hz (ICM45631), MAGNETOMETER 1,25–100 Hz (MMC5616), LIGHT (TMD3743, on-change 1 Hz), PROXIMITY (TMD3743, wake-up, 10 Hz), BAROMETER 1–25 Hz (SPL07003), GRAVITY/ROTATION_VECTOR/GEOMAG 5–200 Hz, LINEARACCEL 5–50 Hz; **Haptik reich** (Frequenz 30–300 Hz, Resonanz 171,2 Hz, 1081 Amplituden — Amplitude+Frequenz steuerbar). **WebGPU: verfügbar** (`chrome://gpu` Chrome 153, GPU Mali-G715, Vulkan 1.4.343 Treiber 54.3.0; Dawn-Vulkan + ANGLE-OpenGLES beide „Available", Features u. a. `core-features-and-limits`, `timestamp-query`, `shader-f16`, `subgroups`, `dual-source-blending`) — Display 412×924, scale 2,625. **Forerunner 945** (`GarminDevice.xml` im Backup `data/garmin-945/2026-09-26/`): SoftwareVersion **13.70**, PartNumber 006-B3113-00, UnitId 3312659436; **237 `.FIT`** (gemessen via `find`; die folge37-Zählung „134" betraf einen anderen Schnitt). FCC/IC nur am Case-Label — ungemessen. Aus Operator-Wort vorhanden: Laptop, Meta Quest 1 (Existenz unklar); Pixel-Mikrofonanzahl = 3. Offen (Quest): **Quest 2** (`hollywood`, `1WMHHB60SN1463`, Android 14, Build `UP1A.231005.007.A1`) — Dev-Mode aktiv, adb autorisiert (gemessen 2026-09-26 via `adb`): Android-Sensoren = **Syncboss IMU**, **Syncboss Double Tap**, **IAD** (`oculus.sensor.iad`); Controller-Pfad = `dumpsys OVRRemoteService` (rechter Controller `JEDI`, FW 1.9.2, IMU ICM42686, Battery 100 %, TrackingStatus POSITION) + VR-Dienste `oculus.internal.tracking.IControllerTrackingService`/`IHapticsService`/`IHandTrackingService`/`ITrackedObjectService`/`ITrackingFidelityService`. **WebGPU: verfügbar** (`chrome://gpu`, OculusBrowser 149.1.0.10 / Chrome 149; GPU Qualcomm Adreno 650, Dawn-Vulkan + ANGLE-OpenGLES beide „Available", Features u. a. `core-features-and-limits`, `timestamp-query`, `shader-f16`). Meta-Konto bestätigt (Code 400630, `code@omegaflow.space`), Developer-Organisation `omegaflow` angelegt, SideQuest v1.2.3 AppImage verifiziert (`~/Schreibtisch/SideQuest-1.2.3.AppImage`). **Quest 1** — im Android-Recovery („open belly"), lädt; Factory-Reset via Boot-Menü (Power+Volume-Down) oder Firmware-Reset ausstehend.
- **Blockade:** Restmessungen nur am physischen Gerät.
- **Braucht:** Quest `adb devices` + `dumpsys sensorservice`.
- **Wort:** Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.

#### Puls-Pfad 945 — Weg (b): Live-BLE-HR nach dem Membran-Fix
- **Status:** wartend | **Bindung:** operator
- **Trigger:** Harte-Läufe-LOCK gefallen (verifiziertes `star-dmax-probe`-Artefakt + Folgelauf).
- **Lage:** (gemessen 2026-09-26 via `bin/omegaflow` + Backup) 945 gesichert (`data/garmin-945/2026-09-26/`, 36 MB); alle 134 `.fit`/`.FIT` tragen `nn: 0` (RR nur live über BLE; `decode_hr_measurement`, `src/archivar/ble.rs:1220`).
- **Blockade:** Harte-Läufe-LOCK.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 bin/omegaflow` mit 945-BLE-HR; `tone_code`-Stresswechsel + `tone_scale`-Relaxation messen.
- **Wort:** Puls-Weg (b) | 2026-09-26 | Operator-Wort folge36.

#### vC-Permeabilität — 945-Zweig (ohne Platine)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** 945 liefert den Puls-Arrival.
- **Lage:** (gemessen 2026-09-26 via sread/sgrep) Pfad steht (`src/mathematikerin/omega.rs:200/349`), HRV-Reader gebaut (`src/archivar/ble.rs:715/926`); absent ist der Puls-Arrival.
- **Blockade:** kein Puls-Arrival.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`, dann `perm_target_probe --live <pfad>` — unter dem Harte-Läufe-LOCK.
- **Wort:** vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36.

#### HRV/Puls-Arrival → Radiations-Pfad (Quelle 945 per BLE)
- **Status:** wartend | **Bindung:** operator
- **Trigger:** der Puls-Arrival liegt am Kanal (nach verifiziertem `star-dmax-probe`-Lauf).
- **Lage:** (gemessen 2026-09-26 via sread/sgrep) Code-Pfad steht: `feed_beat_to_hrv` (`main_flow.rs:79`), ω-Loop `tone_scale` (`omega.rs:1670-1678`), Apertur = `field_permeability * tone_scale` (`omega.rs:349`); Test `tests.rs:547`. Absent ist der physische Puls-Arrival.
- **Blockade:** kein Puls-Arrival am Kanal.
- **Braucht:** Puls-Arrival auf `nn`/`rr`/`ibi` liefern, dann `tone_code`-Stresswechsel + `tone_scale`-Relaxation messen.
- **Wort:** Puls-Weg (b) | 2026-09-26 | Operator-Wort folge36.

#### TLS im Relay (wireless)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator installiert `ca.pem` am Pixel und startet `stunnel`.
- **Lage:** (gemessen 2026-09-26 via `openssl`/`ss`/stunnel-Lauf) Material in `state/tls/`; **Leaf mit `openssl x509 -req` neu ausgestellt, SAN = `IP:192.168.178.26`** (war Platzhalter `192.168.0.0`; LAN-IP = `wlp58s0`). `ca.pem` aufs Pixel gepusht (`/sdcard/Download/ca.pem`, 1139 B). **Port 1619 ist doppelt belegt:** `smail_recv` (Mail, pid 1207, `127.0.0.1:1619`) **und** `bin/relay-tls.stunnel.conf` (`0.0.0.0:1619`) → stunnel bindet nicht. Test-Config `/tmp/opencode/relay-tls-1620.conf` (Port 1620) bereit; `/tmp` wird beim Neustart geleert → ggf. neu anlegen. Relay `bin/omegaflow` (1618) muss nach Neustart neu gestartet werden.
- **Blockade:** Port 1619 doppelt belegt (smail_recv ↔ Relay-TLS); **Chrome/Android vertraut Nutzer-CAs nicht** → HTTPS im Chrome scheitert voraussichtlich (Client mit User-CA-Trust nötig).
- **Braucht:** freien Port (1620) oder smail_recv-Port trennen; `ca.pem` am Pixel installieren (Einstellungen → Sicherheit → Weitere Sicherheitseinstellungen → Verschlüsselung & Anmeldedaten → Zertifikat installieren → CA-Zertifikat); `stunnel /tmp/opencode/relay-tls-1620.conf`; `https://192.168.178.26:1620/consent?ja`.
- **Wort:** HTTPS ja | 2026-09-26 | Operator-Wort folge36.

#### Mantis Shrimp — Sensor-Hardware beschaffen (BOM)
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator hebt das LOCK auf.
- **Lage:** (gemessen 2026-09-26 via Operator-Wort) BOM bestellfertig (`docs/specs/mantis-shrimp-bom.md`); Platine nicht bestellt.
- **Blockade:** LOCK.
- **Braucht:** Operator-Wort; dann BOM bestellen.
- **Wort:** Mantis Shrimp LOCK | 2026-09-26 | Operator-Wort folge36.

#### Harte Läufe (sichtbar/hidden) — LOCK
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Membran verifiziert (star-dmax-probe-Artefakt + verifizierender Folgelauf).
- **Lage:** (gemessen 2026-09-26 via sgrep/git) Stern-Gitter (`StarCellKey`), `enclosure_rho`, Sprung-Vektor `jump_residual_breached` und die drei Umgehungen sind gebaut; die Verifikation fehlt.
- **Blockade:** Verifikation (siehe Linie-Punkt star-dmax-probe).
- **Braucht:** Probe-Artefakt lesen + verifizierenden Folgelauf; dann LOCK aufheben (Operator-Wort).
- **Wort:** Harte Läufe LOCK | 2026-09-26 | Operator-Wort folge36.

### Dritter handelt

#### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via sread) präregistrierte JUICE-Kette wartet auf das Perigäum; RTSW-Retention ~24 h (`docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** keine.
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
