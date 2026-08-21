<!--
  title: Handover: Ausgabe-Flächen & Sensoren — M01–M07, native Sensoren, Browser-Rest
  class: handover
  date: 2026-08-21
  sha256: 09765b5e18a9ca4d33cce8670c094ba0abde240fe252c5fdf390d47f428bae66
  status: live
  see-also: TODO.md docs/omegaflow_sense_hardware.yaml docs/concepts/wetterstation.md docs/concepts/search-command-palette.md
-->

# Handover: Ausgabe-Flächen & Sensoren

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext. Der Auftrag
ist nicht die Ausführung; ausgeführt wird erst auf das Wort des Operators.

Register-Quellen: TODO.md, Abschnitte „Ausgabe-Flächen & Sensoren",
„Browser-Relay", „Membran & Wahrnehmung" (M01–M07, Wetterstation).
Ein Fenster trägt EINE Einheit (M-Nummern sind einzelne Fenster).

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # sauber oder fremde Arbeit nennen
cargo check                      # 0/0
grep -n "sensor_config" src/archivar.rs src/relay.rs   # die Sensor-Registry
```

Referenzen (stehend): `docs/omegaflow_sense_hardware.yaml` (35 Sensoren/
Aktuatoren), `docs/concepts/wetterstation.md`,
`docs/concepts/search-command-palette.md`, `docs/concepts/4d-membrane.md`
(ARCHIVED — M01 referenziert sie), `static/index.html`,
`static/constants.js`, TODO.md.

## Die Einheiten

### A. SurfaceRadiator-Implementierungen

Offen: Bluetooth (Smartwatch) und HID (Force-Feedback); Vibration hängt am
ESP32-Prototyp. Serial-TX lebt (OMEGAFLOW_SERIAL_OUT, 115200, eine Zeile
je Tick).

### B. Kamera/Mikro/IMU nativ

Die Daten existieren, der Sensor-Pfad fehlt (Batterie + Zustimmungs-Gate
leben). Der Sensor fragt, bevor er aufzeichnet.

### C. Gamepad-Oszillator

Die gilrs-Steuerung lebt hinter `--features gamepad` (Navigation:
fold/jump/Rotation); das Gamepad als Sensor-Oszillator ist offen — die
serielle Ingress-Vokabel deckt ESP32, HID-Gamepad steht aus.

### D. Browser-Relay-Rest

`refused-else` ohne body-Deklaration (SurfaceFlow für spd/hdg lebt:
static/index.html 236-249, frame_motion in src/archivar.rs:10114). Der
eingefrorene index.html/fieldShader-Snapshot trägt die tote Rotation noch
(GRID_TO_ANGLE = 2^62, static/index.html 42/1245) — B1, bleibt
registriert, falls der Relay wieder auflebt.

### E. M01 WebSerial-flow-Protokoll

Zwei Spezifikationen konsolidieren: `docs/concepts/4d-membrane.md`
(`flow <force_name> <force_id> <|Ω|> 1 <tick_ms> <t> <x> <y> <z>`) vs.
`docs/omegaflow_sense_hardware.yaml` (`flow <channel> <mode> <value>
<unit> <duration_ms> <t> <x> <y> <z>`). SeismicOscillator schreibt heute
die rohe f32-Σω-Intensität (4 B/Frame) an den Port
(src/mathematikerin.rs:909, KineticRadiator::vibrate).

### F. M02 ESP32-Mantis-Shrimp-Firmware

no_std-Rust-Firmware offen; Browser-Seite (actuate) + M01.

### G. M03 Audio-Gain ohne tanh

static/index.html windowMedianExtent() → tanh(Ω·median) — Median mit
∞-Extents ungelöst; die Normalisierung auf die reine Messung steht aus.

### H. M04 Navigation (Nebra-Kalibrierung)

Wheel-Divisor 128 im Hauptpfad (Touch-Pfad 512); Initial-Scale:
gridStep = 2**31 → 2³⁷; die native Parität (−/= ×4, keine
Wheel-Kalibrierung) ist offen. (Der 2-Finger-Zeitschub läuft in der
Archivar-Übergabe — NICHT anfassen.)

### I. M05 Station-Sensoren als SI-4-Token

recordSample(name, value, force, unit) + convert_to_si im Archivar
(Mikrofon→Pa, Kamera→lx, Accelerometer→m/s², Magnetometer→µT). „biotic"
kollidiert mit der Force-Registry — klären.

### J. M06 Wetterstation-Debug-Konsole

Konsole als 4-Token-Spiegel `name [force, unit]: SI-Wert`.

### K. M07 Command Palette ⌘K

SIMBAD-TAP-Objektsuche (Presence-Jump), lokaler Source-Index,
Force-Filter. Konzept: search-command-palette.md (AUSSTEHEND — nie gebaut).

### L. Wetterstation

Der 4-Token-HUD („wind_speed [advective, m/s]") fehlt nativ — kommt mit
der Messreihe. Konzept: wetterstation.md.

### M. Advective per-Quelle

Wind in tm.w (Kanal verdrahtet, Messquelle fehlt).

### N. OPeNDAP-Integration

Ausstehend — kein Code, kein Block.

### O. Camera

~19k Pixel-Quellen (4×4-Raster) → WS-Traffic-Hotspot; der Weg (Raster,
Drosselung, Rauschen) ist die Einheit.

### P. Konstanten ohne Herleitung

- `sensor_config`/`probe_classify`-τ/TTL (60/300/0.01/3600) —
  Draft-Konvention A6: Sensor-Registry-Kadenzen (serial 60 s, battery
  300 s) und Quellen-TTL-Familie (86400) — KEINE Messungen der Quelle;
  die τ-Gate beim Einbau entscheidet. `sensor_config` lebt in
  src/archivar.rs:11289, Konsumenten relay.rs:555/614.
- `getRto` min/max 100/5000 ms — nicht 2ⁿ/Φ-hergeleitet
  (static/constants.js:12).
- Probe: `coordinates.2` als alt vs. Tiefe bei Seismik — Vorzeichen offen.

## Verifizierter Kontext (2026-08-21)

- Alle Tests laufen kopf-los (Operator-Limit: keine Membrane-Fenster);
  `OMEGAFLOW_HIDDEN=1` für Lauf-Gates.
- Die Aktuatoren sind Oszillatoren (Atom 9): AcousticOscillator
  (stdout-PCM), SeismicOscillator (serial f32), EMOscillator (Fenster).
  M01/M02 bauen darauf, sie ersetzen nichts.
- Consent: jeder neue Sensor trägt sein Zustimmungs-Gate; ein ungefragter
  Sensor ist eine Verletzung (Ethik).

## Gates

- cargo check 0/0 (vier Kombis), cargo test komplett.
- Hardware-Einheiten: kein Test darf Hardware berühren (seriell/vibrierend)
  — Protokolle als reine Funktionen testen.
- Ein Commit je Einheit; TODO.md-Register im selben Commit.
- Diese Datei nach dem Abschluss archivieren (Regel in AGENTS.md).

## Nicht anfassen

2-Finger-Zeitschub (Archivar-Übergabe), Audio-Ausgang-Entscheidung
(Archivar-Übergabe), Source-Port, Validation/CI, 4D-Wahrheit.
