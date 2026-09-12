<!--
  title: Handover — Bau & Code (Stand 2026-09-12, Bau14)
  session: Bau-Folge
  class: handover
  date: 2026-09-12
  sha256: 7cf2d6d8bd8e349f65bf0e9c20b36cb599f4e4d9d18ae0789fd5ed238e958be5
  status: archived
-->
# Handover — Bau & Code (2026-09-12, Bau14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Membran — die offenen M-Punkte

- **M02 ESP32-S3-Radiatorium-Firmware (no_std)** — gebaut in diesem Atom:
  `firmware/radiatorium-lib/` (reine no_std-Logik: f32-LE-Σω-Rahmen-Parser,
  ein-Gain-Duty-Gesetz, TCA9548A-Mux-Wahl, MAX30102-FIFO-Parse — 20 stille
  Host-Tests) und `firmware/radiatorium/` (esp-hal-1.2.1-no_std-Bin: USB-CDC-RX
  → Σω → LEDC auf den 8 MOSFET-Kanälen). Host-Pfad testbar:
  `static/presence_frame.js` + `static/presence_frame.test.mjs` (13 stille
  node:test, Mock-CDC). CI: `.github/workflows/esp32-firmware.yml` (espup-Build
  → Host-Tests → Wokwi-`.bin`-Artifact).

  Offen bleibt:
  - **Build-Verifikation (Toolchain-Gap, Rest):** das Xtensa-Bin ist gegen
    esp-hal 1.2.1 geschrieben, aber auf dieser Maschine unkompiliert (Toolchain
    absent); der erste `esp32-firmware`-Dispatch nach Push ist die Messung
    (grün/rot). Kein Cargo.lock committet — CI löst frisch auf.
  - **Servo-Peripherie:** ESP32-S3-LEDC trägt 8 Kanäle = genau die 8 MOSFETs;
    die 2 Servos (pan/tilt, GPIO15/16) brauchen ein zweites PWM-Peripheriegerät
    (MCPWM/RMT). Das 50-Hz-Servo-Gesetz ist lib-getestet (`servo_pulse_ms`); die
    Peripherie-Verdrahtung ist pending.
  - **Binding** (Puls-Ankunft über die Firmware → Ton-Gate → Strahlpfad) bleibt
    pending; die MAX30102-Leselogik + FIFO-Parse stehen lib-getestet, der
    Firmware-Lesestrom + die Host-Routing warten auf ihr Atom. Das RMSSD/Ton-Gate
    steht in `src/archivar/hrv.rs`.
  - **Physik-Gap** (reale Aktoren Peltier/EM/Piezo nicht simulierbar) bleibt
    gemessen; gebaut ist die testbare Hälfte (Mock-CDC-Host-Pfad).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
