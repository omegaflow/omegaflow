<!--
  title: Handover — River-Folge 38 (2026-09-26)
  session: River-Folge 38
  class: handover
  date: 2026-09-26
  sha256: a66a118b4ef1d995ad547dc8b394c6c27a7b96b2927e9db5fe9d53c64427e2dc
  status: live
-->
# Handover — River-Folge 38 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt`. **Sortierung: umsetzbar →
nicht umsetzbar** (Operator-Wort, 2026-09-26) — erst was die Maschine jetzt bis
zur Kante arbeiten kann, dann was auf Operator/LOCK/termin wartet. Innerhalb der
Gruppen ohne Rang. Jeder Punkt trägt **Trigger** / **Lage** (mit Messstempel) /
**Blockade** / **Braucht**.

Diese Session konsumierte `handover-2026-09-26-river-folge37.md` (nach
`docs/handover/archiv/`).

**Wort | Datum | Quelle**
- Sortierung der Tafel: **umsetzbar → nicht umsetzbar** | 2026-09-26 | Operator-Wort — `AGENTS.md` trägt sie bereits (Zeile 206, „umsetzbar zuerst"); `docs/handover/_template.md` war der stale Träger (Akteur) und ist in diesem Atom korrigiert.
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
- Feld über **alle Radiatoren gleichberechtigt** ausgegeben | 2026-09-26 | Operator-Wort.
- Radiator-Transportmedien (Code-Archäologie, hier + legacy): **acoustic** (`AcousticOscillator`) | **visual** (`TcpRadiator`/`StderrRadiator`) | **serial** (`SeismicOscillator`/`KineticRadiator`, `frame_bytes 0x02`); physikalische Feintaxonomie = 9 Kräfte (legacy `docs/specs/omegaflow-sense-hardware.yaml.md`); jeder Radiator und Sensor gleichberechtigt | 2026-09-26 | Operator-Wort + Archäologie.
- **Keine Kraft→Kraft-Mappung:** jeder Radiator erhält **alle neun Kräfte** (Σω kanonisch, `frame.omega.iter().sum()`); das Transportmedium (acoustic/visual/serial) ist nur Transport, keine Kraft-Zuordnung — die verlorene Kanal-Zuweisung (audio→2, haptics→4) kehrt nicht zurück | 2026-09-26 | Operator-Wort (Doktrin `docs/specs/radiators.md`, Satz 2).
- Membran: **natürlich fixen** (die Defaults + tycho-Divergenz) | 2026-09-26 | Operator-Wort.
- BT-Geräte (Box + Kopfhörer) **angeschlossen** | 2026-09-26 | Operator-Akt.
- Quest 2 autorisiert (`1WMHHB685V1462`) | 2026-09-26 | Operator-Akt.
- Geräte-Zugriff: **vor jedem Zugriff fragen** (adb/BT), damit der Operator zulassen kann | 2026-09-26 | Operator-Wort.

## Stehender Pass (gemessen 2026-09-26)

- **HEAD:** `7e2cefd9f` == `origin/main` (fast-forward); dieser Atom gepusht.
- **Safety-Snapshot:** `refs/safety/1790443358`.
- **Arbeitsbaum:** nur Fremdarbeit (andere Linien: `AGENTS.md`, `_template.md`, mountain-Rename, `fits.rs`/`hdf4.rs`/`uvfits.rs`, `blocked_sources.φ`, planck-psz2, `skydirection.rs`) — nichts Eigenes offen.
- **`open_points_check`:** 0 format-gaps (Stand folge37-Tafel).
- **`register_lookup`:** keine register-eigenen River-Einträge; Orphans/`--orphan-docs` fremd (mycelium, `positive-maske`).
- **Postfach:** `state/mail/mail_ledger.φ` absent (privates `state/`, CI-Build zuständig) — Lücke benannt.
- **CI (diese Session dispatched):** `star-dmax-probe 36252229964` = success (keine Widerlegung); `membrane-hull-probe 36260828234` = success (Prozess), Artefakt misst **3 von 4 Pfaden REFUTED**; `ci-check 36260830080` = cancelled (überholt).
- **Geräte (gemessen):** Quest 2 (Oculus, Android 12, `1WMHHB685V1462`) autorisiert — Syncboss IMU `ICM42688`, Accel/Gyro 20–100 Hz, Fusion 200 Hz; BT-Box/Kopfhörer angeschlossen.
- Shared external state: `docs/zustand/external-state.md` (nicht kopiert).

## Offen

### Umsetzbar (jetzt, autonom bis zur Kante)

#### membrane-hull-probe — Fix verifizieren (LOCK-Gate)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Run `36263190452` (nach Commit neu dispatched) liefert das grüne Artefakt.
- **Lage:** (gemessen 2026-09-26 via Artefakt `36260828234`) `3 von 4` Pfaden divergent: bootstrap `anchor_items 10/82` ohne Gate (`main_flow.rs:243`), per-tick `in_hull` defaultet `true` (75/82, `main_flow.rs:1261,1263`), catalog_tycho `f_excl 0.999940`. Fix gebaut (noch uncommittet): `spatial.rs law_bounds` analytische Schranke, `fetch.rs body_in_enclosure` mit Record-Epoche, per-tick `_ => false`; lokal **0 von 4** divergent; 4 Gate-Fixtures.
- **Blockade:** Fix uncommittet → Lauf `36263190452` trägt den alten Stand.
- **Braucht:** nach `/commit` `gh workflow run membrane-hull-probe.yml`; bei grün dem Operator das Harte-Läufe-LOCK zur Aufhebung vorlegen.

#### Akustik-Sink `OMEGAFLOW_ACOUSTIC` — BT-Radiatoren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Run `36263190452` grün (nach Commit) + Operator-Wort zum LOCK-Aufheben.
- **Lage:** (gemessen 2026-09-26) Sink in `main_flow.rs` gebaut (`acoustic_sink()`): `-`/`stdout`, freies Player-Kommando, oder **`auto`** — liest die live Sinks (`pactl list short sinks`, keine hartkodierten Namen) und fächert den Frame via `AcousticFanout` gleichberechtigt auf alle; `cargo check` 0/0, Test `parse_sink_names_reads_the_live_sink_list`; noch uncommittet.
- **Blockade:** Harte-Läufe-LOCK.
- **Braucht:** `OMEGAFLOW_ACOUSTIC=auto bin/omegaflow` — Box + JBL + jeder weitere Sink empfangen denselben Frame.

#### Geräte-Inventar — alle Wege je Gerät
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator paart/verbindet (Kopfhörer, Quest-Browser).
- **Lage:** (gemessen 2026-09-26 via `adb`/`bluetoothctl`/`lsusb -t`/`pactl`)
  - **RX100M5A** (`054c:0cb0`) — USB **Mass Storage** (Karte lesbar) + WiFi **Camera Remote API**.
  - **Forerunner 945** (`091e:4c29`) — USB **Vendor/`usbfs`** (FIT) + BLE **Heart Rate `0x180d` + RSC `0x1814`** (live RR/HR) + Garmin-vendor.
  - **Quest 2 #1** (`1WMHHB685V1462`) — USB adb (Syncboss IMU `ICM42688`, Accel/Gyro 20–100 Hz) + WiFi Relay-Display (`adb` aktuell leer).
  - **Quest 2 #2** (BLE ManufacturerData `1WMHHB60SN1463`) — BLE (Facebook `0xfeb8`) + WiFi Relay.
  - **Pixel 10a** (BT verbunden) — A2DP/Audio-Source + **PAN/NAP** (Tethering) + BLE GATT + MAP/Phonebook.
  - **HiBreak pro** — A2DP + PAN/NAP + HFP.
  - **Bose Mini II** (BT verbunden, Default-Sink, Batterie 50 %) — A2DP Audio-Sink + HFP.
  - **JBL TUNE500BT** (`b8:f6:53:5f:27:18`, BT verbunden) — A2DP-Sink `bluez_output.B8_F6_53_5F_27_18.1` (Card 661).
- **Blockade:** keines.
- **Braucht:** `OMEGAFLOW_ACOUSTIC=auto` (live fan-out auf alle Sinks, kein hartkodierter Sink); 945 **BLE-HR** verbinden (Puls-Arrival); Quest-Browser `http://<host>:1618`; RX100 MSC mounten (USB) oder WiFi-Remote.
- **Wort:** Geräte-Zugriff vorher fragen | 2026-09-26 | Operator-Wort.

#### `#body`-Deklaration fehlt — alle Stations-Samples verworfen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eigen.
- **Lage:** (gemessen 2026-09-26 via explore) `#body=<body>,<lat>,<lon>,<alt>` ist ein CLI-Arg (`main_flow.rs:550-565`), kein Registerfeld; es steht in `phi/` 0×, `main_flow.rs:566` verwirft ohne es alle Stations-Samples. 10 Stations-Quellen betroffen (u. a. `phi/sources.φ:428`, `:583`, `:715`).
- **Blockade:** keine.
- **Braucht:** `#body`-Übergabe im Lauf sicherstellen (CLI/Doku).

#### Sonnenfarbe — `#[ignore]`-Test in CI ausführen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** `--ignored`-Dispatch vorhanden.
- **Lage:** (gemessen 2026-09-26) Test `browser_field_pipeline_offscreen_runs_the_measured_branch` + `mesa-vulkan-drivers`-apt in `ci-check.yml` (committed `7e2cefd9f`); `#[ignore]` → läuft nicht im plain `cargo test`.
- **Blockade:** kein `--ignored`-Dispatch.
- **Braucht:** CI-Schritt `cargo test --release --features browser_relay -- --ignored browser_field_pipeline_offscreen_runs_the_measured_branch` (Muster `te-gate.yml`).

#### RX100 — Restmessungen (Kalibrierung, Band, Live-Zugang)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kamera im Smart-Remote-LAN; Messung via `rx100_compiler`.
- **Lage:** (gemessen 2026-09-26) Parser/Compiler/Register gebaut (`7e2cefd9f`; `src/archivar/rx100.rs`, `phi/sources.φ`); `L_v = K·N²/(t·S)`, K=12.5 (ISO 2720, ungemessen); Band `freq`/`bin_width` noch nicht in die Wire-Slots verdrahtet.
- **Blockade:** physische Kamera + Referenzmessung.
- **Braucht:** K gegen ein kalibriertes Luminanzmeter messen; Kamera in Smart-Remote ins LAN, `cargo run -p omegaflow-harvest --bin rx100_compiler`.

### Nicht umsetzbar (wartet)

#### Chrome DevTools MCP — Remote-Debugging aktivieren
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator aktiviert `chrome://inspect/#remote-debugging` und startet die Session neu.
- **Lage:** (gemessen 2026-09-26 via git diff) `opencode.json` MCP `--autoConnect` (`chrome-devtools-mcp@1.9.0`), `jq empty` grün.
- **Blockade:** geräteseitige Aktivierung.
- **Braucht:** `chrome://inspect/#remote-debugging` einschalten, Session neu starten.
- **Wort:** Punkt 1 Chrome-Debugger aktivieren | 2026-09-26 | Operator-Wort („Ja").

#### Browser-Extension unpacked laden
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator führt den Load aus (Artefakt `chrome-mv3/` liegt vor).
- **Lage:** (gemessen 2026-09-26 via `ci_manage`) CI-Build grün — Lauf `36243288134` success (npm install --legacy-peer-deps + build + 85/85 Tests), Artefakt `chrome-mv3/` hochgeladen (head `39cfa06c3`).
- **Blockade:** keine.
- **Braucht:** `chrome://extensions` → Developer mode → „Load unpacked" → Fork-Unpacked-Verzeichnis.
- **Wort:** Extension forken | 2026-09-26 | Operator-Wort folge36.

#### Geräte-Inventar — Quest-Rest
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator führt die Befehle am Gerät aus.
- **Lage:** (gemessen 2026-09-26 via `adb` + `chrome://gpu`) Bigme HiBreak (Android 14) + Pixel 10a (Android 17) + Forerunner 945 (SW 13.70, 237 `.FIT`) vollständig vermessen; WebGPU auf Bigme/Pixel/Quest 2 verfügbar. Offen: Meta Quest (1?) `adb devices` + `dumpsys sensorservice`.
- **Blockade:** Messung nur am physischen Gerät.
- **Braucht:** Quest `adb devices` + `dumpsys sensorservice`.
- **Wort:** Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.

#### TLS im Relay (wireless)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator installiert `ca.pem` am Pixel und startet `stunnel`.
- **Lage:** (gemessen 2026-09-26 via `openssl`/`ss`/stunnel-Lauf) Material in `state/tls/`; Leaf mit `openssl x509 -req` neu ausgestellt, SAN = `IP:192.168.178.26`; `ca.pem` aufs Pixel gepusht (`/sdcard/Download/ca.pem`). **Port 1619 doppelt belegt:** `smail_recv` (127.0.0.1:1619) und `bin/relay-tls.stunnel.conf` (0.0.0.0:1619) → stunnel bindet nicht. Test-Config `/tmp/opencode/relay-tls-1620.conf` (Port 1620).
- **Blockade:** Port 1619 doppelt belegt; Chrome/Android vertraut Nutzer-CAs nicht → HTTPS im Chrome scheitert voraussichtlich (Client mit User-CA-Trust nötig).
- **Braucht:** freien Port (1620); `ca.pem` am Pixel installieren (Einstellungen → Sicherheit → … → Zertifikat installieren → CA); `stunnel /tmp/opencode/relay-tls-1620.conf`; `https://192.168.178.26:1620/consent?ja`.
- **Wort:** HTTPS ja | 2026-09-26 | Operator-Wort folge36.

#### vC-Permeabilität — 945-Zweig (ohne Platine)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** 945 liefert den Puls-Arrival.
- **Lage:** (gemessen 2026-09-26 via `sread`/`sgrep`) Pfad steht (`src/mathematikerin/omega.rs:200/349`), HRV-Reader gebaut (`src/archivar/ble.rs:715/926`); absent ist der Puls-Arrival.
- **Blockade:** kein Puls-Arrival.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`, dann `perm_target_probe --live <pfad>` — unter dem Harte-Läufe-LOCK.
- **Wort:** vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36.

#### Puls-Pfad 945 — Weg (b): Live-BLE-HR nach dem Membran-Fix
- **Status:** wartend | **Bindung:** operator
- **Trigger:** Harte-Läufe-LOCK gefallen (verifiziertes `star-dmax-probe`-Artefakt + Folgelauf).
- **Lage:** (gemessen 2026-09-26 via `bin/omegaflow` + Backup) 945 gesichert (`data/garmin-945/2026-09-26/`, 36 MB); alle `.fit`/`.FIT` tragen `nn: 0` (RR nur live über BLE; `decode_hr_measurement`, `src/archivar/ble.rs:1220`).
- **Blockade:** Harte-Läufe-LOCK.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 bin/omegaflow` mit 945-BLE-HR; `tone_code`-Stresswechsel + `tone_scale`-Relaxation messen.
- **Wort:** Puls-Weg (b) | 2026-09-26 | Operator-Wort folge36.

#### HRV/Puls-Arrival → Radiations-Pfad (Quelle 945 per BLE)
- **Status:** wartend | **Bindung:** operator
- **Trigger:** der Puls-Arrival liegt am Kanal (nach verifiziertem `star-dmax-probe`-Lauf).
- **Lage:** (gemessen 2026-09-26 via `sread`/`sgrep`) Code-Pfad steht: `feed_beat_to_hrv` (`main_flow.rs:79`), ω-Loop `tone_scale` (`omega.rs:1670-1678`), Apertur = `field_permeability * tone_scale` (`omega.rs:349`); Test `tests.rs:547`. Absent ist der physische Puls-Arrival.
- **Blockade:** kein Puls-Arrival am Kanal.
- **Braucht:** Puls-Arrival auf `nn`/`rr`/`ibi` liefern, dann `tone_code`-Stresswechsel + `tone_scale`-Relaxation messen.
- **Wort:** Puls-Weg (b) | 2026-09-26 | Operator-Wort folge36.

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
- **Lage:** (gemessen 2026-09-26 via `sgrep`/git) Stern-Gitter (`StarCellKey`), `enclosure_rho`, Sprung-Vektor `jump_residual_breached` und die drei Umgehungen sind gebaut; die Verifikation fehlt.
- **Blockade:** Verifikation (siehe Linie-Punkt star-dmax).
- **Braucht:** Probe-Artefakt lesen + verifizierenden Folgelauf; dann LOCK aufheben (Operator-Wort).
- **Wort:** Harte Läufe LOCK | 2026-09-26 | Operator-Wort folge36.

#### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via `sread`) präregistrierte JUICE-Kette wartet auf das Perigäum; RTSW-Retention ~24 h (`docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** keine.
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
