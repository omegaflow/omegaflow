<!--
  title: Handover — Bau & Code (Stand 2026-09-12, Bau16)
  session: Bau-Folge
  class: handover
  date: 2026-09-12
  sha256: 5361f540bb1eaffb6fdd8f8892ab39801607c5f24678f1fdbdd2c53eb2bf0947
  status: live
-->
# Handover — Bau & Code (2026-09-12, Bau16)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Membran — die offenen M-Punkte

- **Flashen + Live-Messen (nn-Strom)** — der Sensor-Strom ist gebaut und
  lib-getestet (53 Tests), der Binär typgeprüft (`cargo check --release` sauber);
  der Xtensa-Linker `xtensa-esp32s3-elf-gcc` fehlt lokal (`~/.espressif` abwesend),
  also kein Binär, kein Flash, kein Live-Messen. CI (`esp32-firmware.yml`) baut
  beim Push. Pending auf die Toolchain.

- **Feld→pan/tilt-Ableitungsgesetz** — das Schema (getaggtes Frame, pan/tilt
  absent-fähig) steht, die Firmware behandelt pan/tilt-Frames (Bit klar = Position
  halten, nie 0.0); die Host-Ableitung (Feld → pan/tilt-Winkel) ist pending — ein
  Skalar Σω trägt zwei Achsen nicht, abgeleitete tilt wäre Fabrikation. Bis das
  Gesetz steht senden die Schreiber nur bit0 (Intensität).

- **Physik-Gap** — reale Aktoren (Peltier/EM/Piezo) nicht simulierbar; die
  testbare Hälfte (Mock-CDC-Host-Pfad, presence_frame.test.mjs) ist gebaut.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`). Kein Push, solange der Baum
nicht ruhig ist (eine andere Session arbeitet parallel).
