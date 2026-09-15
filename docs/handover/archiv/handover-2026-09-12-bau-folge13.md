<!--
  title: Handover — Bau & Code (Stand 2026-09-12, Bau13)
  session: Bau-Folge
  class: handover
  date: 2026-09-12
  sha256: c0edc9a7b7f2ae718eacb10756df326615d7f7f7a2e94f0ef40f322c45e6c393
  status: live
-->
# Handover — Bau & Code (2026-09-12, Bau13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session führt wenn
möglich alle offenen Aufgaben dieser Linie aus — die Delegation an Sub-Agenten
(eigener Kontext) macht die Gesamtzahl handhabbar. Nur eigene Arbeit: bei geteilten
Dateien nur die eigenen Hunks — committet wird nur der eigene Teil, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird erst, wenn der Baum
ruhig ist.

## Membran — die offenen M-Punkte

- **M02 ESP32-S3-Radiatorium-Firmware (no_std)** — der ESP32 ist ein Peer
  unter sieben: rohe Intensität wie `SeismicOscillator`. Der Host-Strahlpfad
  steht (Relay-Σω-Stream, WebSerial-Schreibpfad, Consent-gated); der ethische
  Puls/HRV-Filter steht als RMSSD/Ton-Gate in `src/archivar/hrv.rs` (8 stille
  Tests). Neu gemessen 2026-09-12 (docs.wokwi.com/guides/esp32): Wokwi
  simuliert den ESP32-S3 (Custom-Firmware-Upload `.bin`/`.elf`/`.uf2`, USB-CDC
  `USB_SERIAL_JTAG`, GDB). Drei Trennungen, sauber gehalten:

  - **Hardware-Gap geschlossen** — die Firmware (flow-Parser, I2C-Mux-
    Adressierung der 35 Module, PWM-Outputs, MAX30102-I2C-Leselogik) ist ohne
    Device schreib- und logisch testbar.
  - **Toolchain-Gap bleibt** — Wokwi führt ein kompiliertes Binary aus; der
    no_std-Xtensa-Build (`espup`, Target `xtensa-esp32s3-none-elf`) ist auf
    dieser Maschine gemessen absent. Der Build gehört in CI (heavy compute)
    oder als espup-Setup auf die Maschine.
  - **Physik-Gap bleibt** — die Simulation prüft Signale, nicht Manifestation
    (Peltier-Wärme, Elektromagnetfeld, Piezo-Vibration nicht simulierbar); der
    Host-Pfad (`webserial.js`) ist nur logisch gegen die simulierte CDC-
    Schnittstelle testbar, nicht gegen die realen Aktoren.

  Binding bleibt pending (Puls-Ankunft über die Firmware → Ton-Gate →
  Strahlpfad).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
