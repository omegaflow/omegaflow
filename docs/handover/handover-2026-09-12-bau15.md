<!--
  title: Handover — Bau & Code (Stand 2026-09-12, Bau15)
  session: Bau-Folge
  class: handover
  date: 2026-09-12
  sha256: a12ed2e3233e9896ee1298ba152517af41382c6e04e3ba42475f339923e9724e
  status: live
-->
# Handover — Bau & Code (2026-09-12, Bau15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Membran — die offenen M-Punkte

- **Binding** (Puls-Ankunft über die Firmware → Ton-Gate → Strahlpfad) — wartet
  auf sein Atom: der MAX30102-Lesestrom auf der Firmware (I2C-Leseschleife →
  NN-Intervalle → USB-CDC-TX) und das Host-Routing (Serial → `src/archivar/hrv.rs`
  RMSSD/Ton-Gate → Strahlpfad). Die MAX30102-Leselogik + FIFO-Parse stehen
  lib-getestet; das RMSSD/Ton-Gate steht in `src/archivar/hrv.rs`. Host-seitig
  ist die Serial-Anbindung eine Stack-Entscheidung (std-only + curl).

- **Servo-Kommandopfad** — die Peripherie steht (MCPWM0, 50 Hz, pan GPIO15 /
  tilt GPIO16, Neutral 1.5 ms, host-getestete `servo_ticks`). Offen ist der
  Draht: das Frame-Protokoll (1×f32 Σω → pan/tilt) als eigenes Atom — ein
  Schema-Wechsel ist ein eigener Commit. Operator-Wort: pan/tilt absent-fähig
  bauen, nie 0.0-Ruhe (0.0 ist beim Servo eine echte Richtung).

- **Physik-Gap** — reale Aktoren (Peltier/EM/Piezo) nicht simulierbar; gebaut
  ist die testbare Hälfte (Mock-CDC-Host-Pfad, `presence_frame.test.mjs`).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
