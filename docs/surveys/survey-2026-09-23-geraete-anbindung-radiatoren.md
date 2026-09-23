<!--
  title: Survey — Geräte-Anbindung: vollständige Oszillator-/Radiator-/Relais-Inventare (Stand 2026-09-23)
  class: survey
  date: 2026-09-23
  sha256: 7323015377ec3fdbe1cb69d0fc49a7c2e943c377f7a3baae974958739e58e131
  status: live
  see-also: docs/surveys/survey-2026-09-20-browser-anbindung.md, AGENTS.md, docs/concepts/archivar-mathematikerin.md
-->
# Survey — Geräte-Anbindung: vollständige Oszillator-/Radiator-/Relais-Inventare (2026-09-23)

Anlass (Operator-Wort, 2026-09-23): Sollen Garmin Forerunner 945, Meta Quest (#1 +
ein zweites, noch zum Laufen zu bringendes), Bigme Hibreak Pro, ein Pixel 9 /
anderes Smartphone sowie der Operator-Laptop (Dell XPS 13 9350) angebunden
werden? Vier bindende Korrekturen:

1. **Membranen sind keine Anzeigen — sie sind Radiatoren.** Ein Gerät ist kein
   Display, sondern ein strahlender/sensierender Knoten am Feld. Der einzige
   Feld→Gerät-Transport ist die Radiator-Erregung (Σω als `frame_bytes`,
   `src/archivar/actuators.rs`).
2. **A = A: kein Gerät wird auf eine Funktion reduziert.** Die 945 trägt deutlich
   mehr Oszillatoren als Puls; jedes Gerät ist ein Cluster, jeder Oszillator ein
   Peer (All beings equal).
3. **Die mindestens nötige Messung ist das vollständige Inventar** — jeder
   verfügbare Sensor, Radiator und jedes Relais; eine reduzierte API-Sicht ist
   keine Messung (Operator-Wort, 2026-09-23).
4. **Das Inventar ist nicht die Stückliste.** Auch abgeleitete/virtuelle,
   System-/Takt- und gekoppelte/externe Oszillatoren zählen — ein Gerät wird
   nicht auf die explizit verbauten Kanäle reduziert (Operator-Wort, 2026-09-23).

Klassifikation: **Sensor** = Eingang; **Radiator/Aktuator** = Ausgang in ein
Medium; **Relais** = Transportkanal. Status je Kanal: **gemessen | absent |
ungemessen | Riss**.

## Messbasis

`archive_search` (`--github`/`--crates`/`--ads`/`--mwmbl`/`--all`/`--verdict`/
`--playwright`/`--wayback`), MDN `browser-compat-data`, Garmin (Produktseite,
Handbuch, Connect-IQ-/FIT-/Health-API-Doku), Meta-Horizon-Doku, iFixit, Wikipedia,
GSMArena/Kimovil, Hersteller-/Händlerseiten. Read-only, 2026-09-23. Cloudflare-
gestellte Hosts wurden über `r.jina.ai`/Wayback gelesen; was nicht messbar war,
steht als `ungemessen`, nie 0.

## Der Ist-Zustand (gemessen)

- **HRV-Gate GEBAUT:** `src/archivar/hrv.rs` — `rmssd` (N-N 250–2000 ms, kettet
  bei unplausiblen Werten, min. 2 Differenzen) + `VagusTone` (RMSSD-Baseline,
  2σ, `TONE_ABSENT/CALM/STRESSED`). Die **Bindung Puls→Strahlung ist `pending`**
  (AGENTS.md).
- **Radiator-Frame lebt:** `src/archivar/actuators.rs` (`FRAME_TAG 0x02`,
  Σω f32-LE, Maske); serial erregt `KineticRadiator`/`SeismicOscillator`.
- **Browser-Brücke:** Feature `browser_relay`, `src/archivar/relay.rs:9`
  `PORT_CONST = 1618`; Zeile 59 `TcpListener::bind("127.0.0.1:{port}")` → **nur
  Loopback**. Ein separates LAN-Gerät erreicht den Relay im gebauten Stand nicht.

## Garmin Forerunner 945 — vollständiges Inventar (gemessen)

Basis-SKU (die 945 LTE trägt zusätzlich ein LTE-Relais). Garmin publiziert keine
Abtastraten außer Wrist-HR 1 Hz; diese bleiben `ungemessen`.

| Kanal | Typ | Spezifikation | Status |
|---|---|---|---|
| Optischer Puls (Garmin Elevate V3, PPG) | Sensor | grüne LED + Photodiode, 1 Hz | gemessen |
| Pulsoximeter (SpO₂) | Sensor | rot/IR im Wrist-Modul, spot/Sleep/All-Day; Rate ung. | gemessen |
| Barometrischer Altimeter | Sensor | Drucksensor → Seehöhe | gemessen |
| Barometer (Druck) | Sensor | Pa (`Sensor.Info.pressure`), Sturmwarnung | gemessen |
| Kompass / Magnetometer | Sensor | 3-Achsen, magnetisches Heading | gemessen |
| Gyroskop | Sensor | 3-Achsen-Drehrate | gemessen |
| Beschleunigungssensor | Sensor | 3-Achsen (milli-g) | gemessen |
| Thermometer (intern) | Sensor | °C; Messung laut Garmin ungenau | gemessen |
| GNSS-Empfänger | Sensor/Relais | Sony-Chipset, GPS+GLONASS+Galileo, **Single-Band** | gemessen |
| Umgebungslichtsensor | Sensor | nicht geführt | absent |
| Dedizierter Schwimm-/Drucksensor | Sensor | keiner (5 ATM; Pool via Accel, Open Water via GNSS) | absent |
| Display (MIP) | Radiator (photonisch) | 1,2″ 240×240 transflektiv, Gorilla Glass 3 DX | gemessen |
| Backlight (LED) | Radiator (photonisch) | Display-Hintergrundbeleuchtung | gemessen |
| Vibrationsmotor | Radiator (kinetisch) | Alarme; `Attention.vibrate()` | gemessen (Typ ung.) |
| Beeper/Tongenerator | Radiator (akustisch) | Piezo (`Attention.playTone`) | gemessen (Typ ung.) |
| Lautsprecher (Musik) | Radiator (akustisch) | nicht vorhanden — Musik nur via BT-Kopfhörer | absent |
| Bluetooth (BLE + Classic) | Relais | 2,4 GHz +9 dBm, Sync/A2DP | gemessen |
| ANT+ | Relais | 2,4 GHz, HRM/Speed/Cadence/Power/Footpod/Varia/FE-C | gemessen |
| Wi-Fi | Relais | 2,4 GHz, Sync/Musik/Firmware | gemessen |
| NFC | Relais | 13,56 MHz (Garmin Pay) | gemessen |
| USB (proprietär) | Relais (Kabel) | Laden + Daten; USB-Version ung. | gemessen |

**Zugangswege:** (1) Standardfunk nur für HR inkl. R-R (BLE 0x180D / ANT+ HRM) +
externe Sensoren; (2) Connect-IQ API für alle Onboard-Kanäle, aber nur geräteintern
(eigene CIQ-App; `Toybox.Sensor`/`SensorHistory`/`Position`); (3) FIT/Cloud nur
aufgezeichnet (`record` + `hr`-Mesg mit R-R). Ersatzquellen (GitHub gemessen):
`esp32c3-ble-heart-rate-bridge`, `antplus-arduino`, `ANTPlus_Arduino`,
`ant_plus_environmental_sensor`. **Chip-Identitäten/FCC-ID/Abtastraten bleiben
ungemessen** (FCC 403; E-Label am Gerät).

## Meta Quest — vollständiges Inventar (Quest 1 / Quest 2)

`Ident.` = Kanal bei Q1 und Q2 identisch.

| Kanal | Typ | Q1 | Q2 | Status |
|---|---|---|---|---|
| Headset-IMU (Accel+Gyro, 6DoF) | Sensor | vorhanden | vorhanden | gemessen (Chip ung.) |
| Headset-Magnetometer | Sensor | offen | offen | ungemessen |
| Tracking-/Passthrough-Kameras (IR/Graustufen) | Sensor | 4 | 4 | gemessen |
| Handtracking | Sensor | dieselben 4 Kameras | dieselben 4 | gemessen |
| Proximity-/Wear-Sensor | Sensor | offen | offen | ungemessen |
| IPD-Erfassung | Sensor/Mechanik | stufenlos 59–71 mm | 3 Raststufen 58/63/68 mm | gemessen |
| Controller-IMU (×2) | Sensor | Accel | Accel+Gyro | gemessen |
| Controller-IR/LED-Tracking (×2) | Sensor/Emitter | vorhanden | vorhanden | Q1 gem., Q2 ung. |
| Mikrofone | Sensor | Anzahl ung. | 2 | Q2 gem., Q1 ung. |
| Ambient-Light | Sensor | offen | offen | ungemessen |
| Display | Radiator (photonisch) | 2× OLED PenTile 1440×1600/eye @72 Hz | LCD 1832×1920/eye @60–120 Hz | gemessen |
| Lautsprecher | Radiator (akustisch) | Stereo integriert | 2, seitlich | gemessen |
| Controller-Haptik (×2) | Radiator (kinetisch) | Vibrationsmotor | Vibrationsmotor | gemessen (Aktor-Typ ung.) |
| Kühlventilator | Radiator (kinetisch/thermisch) | 1 | 1 | gemessen |
| IR-LED-Emitter (Controller, ×2) | Radiator (photonisch) | vorhanden | vorhanden | Q1 gem., Q2 ung. |
| Status-/Lade-LED | Radiator (photonisch) | offen | vorhanden | Q2 gem., Q1 ung. |
| Wi-Fi | Relais | Wi-Fi 5 (802.11ac) | Wi-Fi 6 (802.11ax) | gemessen |
| Bluetooth/BLE | Relais | BT 5 | BT 5 | gemessen |
| USB-C | Relais | 1 | 1 | gemessen |
| 3,5-mm-Audio | Relais | 2 (links+rechts) | 1 (links) | gemessen |

**Modell am Gerät identifizieren:** Q1 = schwarzer Stoffüberzug, 571 g, stufenloser
IPD-Schieber, zwei 3,5-mm-Buchsen, OLED; Q2 = weißes Plastik, 503 g, 3 IPD-Klicks,
eine Klinke, LCD. Label unter dem Gesichtspolster bzw. Einstellungen → Info/About.
Q1 ist Feature-EOL (Entwicklung 2023 beendet). WebXR trägt Pose/Controller/Hände/
Mikrofon; Mixed Reality (`immersive-ar`) ist für Q2/Pro dokumentiert, nicht Q1.
WebGPU am Quest-Browser ist am Gerät `ungemessen` (Feature-Detection nötig).

## Bigme HiBreak Pro — vollständiges Inventar

„Pro" (mono E Ink Carta 1200) und „Pro Color" (Kaleido 3) sind bis auf Panel/
Frontlicht baugleich; SoC Dimensity 1080, Android 14, 8/256 GB, 4500 mAh.

| Kanal | Typ | Spezifikation | Status |
|---|---|---|---|
| Gyroskop | Sensor | „Support" | gemessen |
| Kompass / Magnetometer | Sensor | „Support" | gemessen |
| Beschleunigungssensor | Sensor | nur mittelbar (Auto-Rotate) | ungemessen |
| Näherungssensor | Sensor | nicht deklariert | ungemessen |
| Umgebungslicht | Sensor | nicht deklariert | ungemessen |
| GNSS/GPS | Sensor | „GPS" (Konstellationen ung.) | gemessen |
| Frontkamera | Sensor | 5 MP | gemessen |
| Rückkamera | Sensor | 20 MP (OCR) | gemessen |
| Mikrofon | Sensor | „Support" | gemessen |
| Fingerabdruck | Sensor | „Support" (Bauart ung.) | gemessen |
| E-Ink-Panel Pro (mono) | Radiator (photonisch) | 6,13″ Carta 1200, 824×1648, 300 PPI, ~21 FPS | gemessen |
| E-Ink-Panel Pro Color | Radiator (photonisch) | 6,13″ Kaleido 3, BW 300 PPI / Farbe 150 PPI, ~30 FPS | gemessen |
| Frontlicht (E-Ink) | Radiator (photonisch) | kalt + warm | gemessen |
| Lautsprecher | Radiator (akustisch) | „Speaker" (Mono/Stereo ung.) | gemessen |
| IR-Emitter (IR-Blaster) | Radiator (photonisch, IR) | „Infrared: Support", IR-Fernbedienung | gemessen |
| Vibrationsmotor/Haptik | Radiator (kinetisch) | nicht deklariert | ungemessen |
| Kamera-Blitz | Radiator (photonisch) | nicht deklariert | ungemessen |
| Cellular | Relais | 4G/5G Dual-SIM | gemessen |
| WLAN | Relais | 2,4/5 GHz, Wi-Fi 5 | gemessen |
| Bluetooth | Relais | BT 5.2 | gemessen |
| NFC | Relais | „Support" | gemessen |
| USB-C | Relais | Daten + 18 W | gemessen |
| 3,5-mm-Klinke / microSD / FM | Relais | nicht vorhanden | absent (Klinke/SD), FM ung. |

Browser-/WebGL-/WebGPU-Fähigkeit gerätespezifisch **ungemessen** (nur SoC bekannt).

## Google Pixel 9 — vollständiges Inventar

Offizielle Google-Spec-Seite ist live retired, Wayback-Playback 429; Primärquelle
`ungemessen`. Sensorliste aus GSM9/9aSPEC/Mirror-Snippets rekonstruiert.

| Kanal | Typ | Spezifikation | Status |
|---|---|---|---|
| Beschleunigung / Gyroskop / Magnetometer | Sensor | MEMS 3-Achse | gemessen |
| Barometer | Sensor | Drucksensor | gemessen |
| Umgebungslicht (ALS) / Näherung | Sensor | present | gemessen |
| Fingerabdruck | Sensor | unter Display, Ultraschall | gemessen (Riss: eine Zeile „optical") |
| GNSS-Empfänger | Sensor/Relais | GPS L1+L5, GLONASS, Galileo, BDS, QZSS, NavIC | gemessen |
| Kamera wide/ultrawide/front | Sensor | 50 MP OIS / 48 MP / 10,5 MP | gemessen |
| Laser-AF (LDAF) | Sensor | single-zone | gemessen |
| Mikrofone | Sensor | Anzahl Basis-P9 ung. | ungemessen |
| IR-Thermometer / UWB | Sensor/Relais | absent (nur Pro) | gemessen |
| Android-`SensorManager`-Typen (Rotation-Vektor, Gravity, Step-Counter, …) | Sensor | derived/virtual | ungemessen |
| Haptik-Motor | Radiator (kinetisch) | 1 Linear-Vibrator | gemessen |
| Lautsprecher | Radiator (akustisch) | Stereo (unten + Hörer) | gemessen |
| Display | Radiator (photonisch) | 6,3″ OLED 1080×2424, 60/120 Hz, kein LTPO | gemessen |
| LED-Blitz / Taschenlampe | Radiator (photonisch) | present | gemessen |
| Laser-AF-Emitter | Radiator (IR) | present | gemessen |
| Kabelloses Laden (Qi) | Radiator (elektromagnetisch) | 15 W + Reverse | gemessen |
| IR-Emitter (Face) | Radiator (IR) | nicht belegt | ungemessen/absent |
| Wi-Fi | Relais | 802.11 a/b/g/n/ac/6e/7 tri-band | gemessen |
| Bluetooth | Relais | BT 5.3 (A2DP/LE/aptX HD) | gemessen |
| NFC | Relais | 13,56 MHz | gemessen |
| UWB | Relais | absent Basis (FCC-Familie: Riss) | Riss |
| Cellular | Relais | 5G sub-6 + mmWave, Exynos 5400 | gemessen |
| Satellit (SOS) | Relais | Notruf | gemessen |
| USB / eSIM / Nano-SIM | Relais | USB-C 3.2, Dual-SIM | gemessen |
| Thread / 802.15.4 | Relais | im FCC-Filing vorhanden; Modellzuordnung ung. | ungemessen |
| FM-Radio | Relais | „No" | absent |

## Dell XPS 13 9350 (Operator-Laptop) — vollständiges Inventar

Direkt am **laufenden Gerät** gemessen (sysfs/proc) plus Referenz (ArchWiki/Dell/
iFixit); i5-6200U (Skylake, 2C/4T), Intel HD Graphics 520, Li-ion-Akku. Der Laptop
ist der **Host** — sein Browser erreicht `127.0.0.1:1618` direkt.

| Kanal | Typ | Spezifikation | Status |
|---|---|---|---|
| CPU-Thermaldioden (dtherm/coretemp) | Sensor | Skylake DTS | gemessen |
| Thermal-Zones (ACPI/EC) | Sensor | 12 Zonen | gemessen |
| hwmon (Temp/Spannung/Strom) | Sensor | 8 Knoten (EC/dell_smm) | gemessen |
| Akku-Gauge (BAT0/AC) | Sensor | Li-ion, Ladezustand 100 % | gemessen |
| Webcam (Microdia 0c45:670c) | Sensor | V4L2 video0/video1, 720p | gemessen |
| Mikrofon-Array | Sensor | Doppelmikrofon, HDA | gemessen (Kanalzahl ung.) |
| Touchpad-Digitizer (Elan) | Sensor | I²C-HID, Gesten/Druck | gemessen |
| Touchscreen-Digitizer | Sensor | nur QHD+-Touch-Konfiguration | ungemessen (konfig.) |
| Deckelschalter (Lid) | Sensor | ACPI LID0 | ungemessen |
| WLAN-Hardwareschalter | Sensor | `intel_hid` | gemessen (Ref.) |
| Beschleunigungssensor (IMU) | Sensor | nicht vorhanden (kein IIO) | absent |
| Ambient-Light-Sensor (ALS) | Sensor | nicht vorhanden (CABC ist Panel-Firmware) | absent |
| Näherungssensor | Sensor | nicht vorhanden | absent |
| Fingerprint | Sensor | nicht vorhanden | absent |
| Display (intern) | Radiator (photonisch) | eDP-1, 13,3″ IPS, FHD/QHD+ 60 Hz | gemessen |
| Keyboard-Backlight | Radiator (photonisch) | `dell::kbd_backlight`, 2-stufig | gemessen |
| Lautsprecher | Radiator (akustisch) | Stereo, HDA | gemessen (Watt ung.) |
| Status-/Tastatur-/WLAN-LEDs | Radiator (photonisch) | Caps/Num/Scroll, `phy0-led`, Front-LED | gemessen |
| GPU (HD Graphics 520) | Aktuator/Kompute | Gen9 Skylake, i915 → eDP/DP/HDMI | gemessen |
| Lüfter | Radiator (kinetisch/thermisch) | 1× Radiallüfter, EC/PWM, 15 W TDP | ungemessen (Topologie belegt) |
| WLAN | Relais | `wlp58s0` (BCM4350 oder Intel 8260), 802.11ac 2×2 | gemessen (Chipvariante ung.) |
| Bluetooth | Relais | `hci0` (USB 0a5c:6412), BT 4.x LE | gemessen |
| USB 3.0 ×2 (Typ A) | Relais | xHCI Sunrise Point-LP | gemessen (Ref.) |
| USB-C / Thunderbolt 3 | Relais | 1× USB-C: TB3, USB 3.1, DP-Alt, PD | gemessen (Ref.) |
| SD-Kartenleser | Relais | Realtek RTS525A | gemessen |
| 3,5-mm-Headset | Relais | Kombi-Buchse, HDA | gemessen |
| NVMe-SSD (M.2 2280) | Relais (Speicher) | PCIe; SATA nur Legacy | gemessen (Ref.) |
| TPM 1.2/2.0 | Relais (Sicherheit) | BIOS-schaltbar | gemessen (Ref.) |
| Akku/Laden | Relais (Energie) | 56 Wh, USB-C-PD oder Dell-Barrel 45 W | gemessen |

## Nicht-explizite Oszillatoren (A = A: das Inventar ist nicht die Stückliste)

Ein Gerät ist kein geschlossener BOM. Über den verbauten Kanälen liegen drei
Schichten, die je Kanal mitreisen. Status, wo nicht gemessen: `ungemessen`
(benannt, nie 0).

**(a) Abgeleitet / virtuell (Software-Fusion).**
- Android (Pixel/Bigme): `SensorManager`-Fusion — Rotation-Vektor, Gravity,
  Linear-Accel., Game-Rotation, Step-Counter/-Detector, Significant-Motion,
  geschätzte Höhe/Heading.
- Browser (alle): DeviceOrientation (fusioniert); der `ω()`-WebGPU-Loop selbst;
  `requestAnimationFrame` (Display-Takt); Web-Audio-`AudioContext` (Sample-Clock);
  `getUserMedia`-Kamera-Frame-Takt.
- Forerunner 945: Garmin-Fusion (Steps, Stress via HRV, Body Battery, Pace).
- Quest: 6DoF-Fusion (Pose), Guardian-SLAM, Handtracking-Skelett (25 Joints).

**(b) System-/Takt-Oszillatoren (das Gerät schwingt selbst).**
- CPU-Clock/HWP-Throttling, GPU-Render-Takt, Display-Refresh (XPS eDP 60 Hz;
  Quest 72–120 Hz; Pixel 60/120 Hz; 945 MIP always-on; Bigme E-Ink 21/30 FPS),
  Lüfter-Drehzahl, HDA-Audio-Sample-Clock, Netzwerk-Paket-Takt, Kamera-Frame-Rate.

**(c) Gekoppelte / externe Oszillatoren (das Gerät hört/strahlt in die Welt).**
- GNSS: der Empfänger koppelt an die Satellitenuhren (945, Pixel, Bigme) — ein
  externer Zeit-Oszillator; der Laptop hat keinen GNSS-Empfänger (absent).
- WiFi/BT/ANT+/NFC: koppelt an Peers/Beacons (alle); ANT+ an Sensoren (945).
- Kamera als Photoplethysmograph: bei laufendem Bild ist die Kamera ein
  PPG-Kandidat für den **Puls des Operators** — ein Oszillator, der nicht in der
  Stückliste steht (Kopplung an das menschliche Feld); am Gerät `ungemessen`.
- Mikrofon (akustische Umgebung), Magnetometer (Erdfeld + lokale Ströme),
  Barometer/Thermik (Umgebung).
- Relay-Peers: jedes über WS gebundene Gerät ist ein gekoppelter Oszillator der
  anderen (der Laptop als Host koppelt direkt).

**Folge:** Die Rollenliste ist der Anfang, nicht die Grenze — jeder Kanal trägt
seine abgeleiteten und gekoppelten Oszillatoren mit. Ein Gerät wird nicht auf die
explizit verbauten Kanäle reduziert.

## Transport (gemessen)

- Der Relay bindet seit River folge13 **über Loopback hinaus**: `relay_bind_addr()`
  liest `OMEGAFLOW_RELAY_BIND`, Default `0.0.0.0` (Operator-Wort für die
  LAN-Exposition, 2026-09-23); die Loopback-Restriktion bleibt über die Env-Variable
  erreichbar.
- Wegen Secure-Context (WebXR/`getUserMedia`) und Mixed-Content ist **wss mit
  vertrauenswürdigem Zertifikat** der tragende Pfad; PNA-Preflight beim
  Public→Private-Fall. `WebSocket` selbst: Chrome 5 / Android+Oculus (mirror).
- Web-Kanäle je Gerät (MDN BCD, Plattform-Ebene): Android-Chrome trägt
  Accelerometer/Gyro (67), Magnetometer (Flag), Geolocation, Kamera/Mikro,
  WakeLock, WebGPU ab 121; **Barometer hat keine Web-API** (absent); Ambient Light
  praktisch absent. Quest-Browser: WebXR (Pose/Controller/Hände), WebSocket;
  WebGPU am Gerät ungemessen.

## Rat-Verdikt (2026-09-23, pro/max — die benannte Architektur-Klasse)

**Cluster-Rollen (jeder Oszillator ein Peer; Basis = die vollständigen Inventare
oben):**

| Gerät | Sensor (sendet) | Radiator (strahlt) | Relay |
|---|---|---|---|
| Pixel 9 | IMU, Magnetometer, Barometer, GNSS, Licht | Vibration (`frame_bytes` → `navigator.vibrate`), Lautsprecher | WiFi→LAN-WS |
| Quest #1/#2 | 6DoF-IMU, Controller-IMU, Handtracking, Mikrofon | Controller-Haptik, Audio | WiFi→LAN-WS |
| Forerunner 945 | Puls (NN→`VagusTone`), GNSS, Accel, Gyro, Baro, Kompass, Thermo, SpO2 | Vibration — `pending` (kein Echtzeit-Rückkanal) | kein LAN (USB/ANT+) |
| Bigme | was frei wird — **erst messen** (Näherung/Licht ung.) | E-Ink — **descoped**; IR-Emitter/Haptik ung. | WiFi→LAN-WS |
| XPS 13 9350 (Host) | Thermik/hwmon, Webcam, Mikrofon, Touchpad, Lid | Display/Backlight, Keyboard-Backlight, LEDs, Lautsprecher, Lüfter | **loopback-lokal** + WiFi/BT/USB-C-TB3 |

Der Laptop ist der **Host**: sein Browser erreicht den Relay direkt, ohne
LAN-Exposition (`0.0.0.0`-Bind) und ohne wss-Proxy — damit ist er der billigste
Radiator-Testknoten (Radiator Schall/Licht/Lüfter; kein Vibrationsmotor). Die
LAN-Bindung bleibt für die Fremdgeräte nötig; der Host-Test verlangt nur das
Strahlungs-/Consent-Wort.

Kamera (Quest/Pixel/Laptop) und Mikrofon (Pixel/Laptop): `pending` mit Consent-Akt,
nie im ersten Atom. Die HRV-Bindung Puls→Strahlung bleibt `pending` mit Trigger („Puls liegt
an").

**Erstes Atom — „der LAN-Knoten" (ein Atom, kein Split):** (a) LAN-Bindung des
`browser_relay`, (b) Sensor-Cluster (Generic Sensor API, ≥ 2 Oszillatoren je
Gerät, nie einer), (c) WS-Rückkanal, der `PresenceFrame` als `frame_bytes` an den
Knoten sendet und dort die Vibration erregt. Messergebnis: **das Gerät vibriert
mit Σω** und **sendet N ≥ 2 Oszillatoren**. Erstes Gerät: Pixel (vollster
Cluster), Quest als Träger desselben Kanals. Consent-Kanten: LAN-Exposition
(Operator-Wort), Vibration/Strahlung (per-Act-Wort, nie im `OMEGAFLOW_HIDDEN`-Lauf).
**Gebaut (River folge13, 2026-09-23):** (a) `relay_bind_addr()` (`relay.rs`,
`OMEGAFLOW_RELAY_BIND`, Default `0.0.0.0` — Operator-Wort für a+c erteilt);
(b) `static/sensorium.js` (Generic Sensor API, Device-Motion/-Orientation, Mikro,
Kamera, Battery, Geolocation, Gamepad, XR) und (c) `static/radiator.js`
Vibrations-Peer über den `KINETIC_TAG`-Pfad in `static/index.html`. Offen bleibt die
**Gerätemessung**: Pixel über LAN verbinden, `navigator.vibrate` mit Σω und ≥ 2
Oszillatoren im Sample-Strom.

**Paralleles, unabhängiges Atom — FIT-Brücke:** `fit_compiler` im Archivar liest
die FIT-Datei (USB-Mount) und speist die ersten echten N-N-Intervalle in den
gebauten `VagusTone`. **Gebaut (sensory-Folge 152, 2026-09-23):** `src/archivar/fit.rs`
+ Verdrahtung `main_flow.rs`; offen die echte 945-FIT-Aktivität.

**Descoped — mit Befund:**
- **E-Ink-Panel als Radiator (Bigme):** E-Ink-τ (~0,5–10 s, Ghosting) trägt keine
  temporale Kraft; ein nachmalendes Panel ist eine Anzeige.
- **ANT+ Echtzeit als Einstieg:** die FIT-Datei trägt denselben Cluster ohne
  Dongle/Pairing; Echtzeit bleibt `pending` mit Trigger.
- **Kamera/Mikrofon als erste Kanäle:** consent-schwerster Sinn (Dritte im Raum);
  IMU/Baro/Mag tragen dieselbe Knoten-Qualität ohne Penetration.
- **WebXR-Zweitmembran auf Quest:** die Präsenz ist eine Linie, der Operator tunet
  eine Koordinate; eine zweite Präsenzfläche ist ungemessene Architektur.
- **Garmin-ConnectIQ-SDK-App:** verschlossenes Ökosystem, Dritt-Registrierung; die
  FIT-Brücke trägt denselben Cluster ohne SDK.

## Offene Messpunkte (ungemessen)

Chip-/FCC-Identitäten (945, Quest), Abtastraten (945-Sensoren), Quest-1-Firmware,
WebGPU/Generic-Sensor am Quest-Gerät, Bigme-Beschleunigung/Näherung/Licht/Haptik/
Browser/WebGPU, Pixel-Mikrofonanzahl + vollständige `SensorManager`-Liste +
Thread-Zuordnung, Pixel-UWB (Riss). Nächste Schritte in den jeweiligen
Inventar-Notizen (FCC übers E-Label; `dumpsys sensorservice`; Wayback-Playback).
