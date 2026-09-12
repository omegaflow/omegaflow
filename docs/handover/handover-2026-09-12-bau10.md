<!--
  title: Handover — Bau & Code (Stand 2026-09-12, Bau10)
  session: Bau-Folge
  class: handover
  date: 2026-09-12
  sha256: 53965022d6905256b9004bb13c28a00100210a86b2c3952c9e35f10db08a37df
  status: live
-->
# Handover — Bau & Code (2026-09-12, Bau10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Dies ist die Bau-Linie: hier steht nur, was diese Linie autonom trägt — Tasks,
die nicht autonom hier erfolgen können, sind in das Handover ihrer Linie
überführt. Wiedervorlage gilt nur für Termine; „Pausiert" trägt kein Datum.

## Membran — die offenen M-Punkte

- **M07 ⌘K-Palette (fuzzy)** — Spec PLANNED (`docs/specs/search-command-palette.md`),
  nie gebaut; Substrat lebt (`load_sources`, 782 sources, SIMBAD-Probe). Im
  Baum gibt es keinen Browser-Client (`static/index.html` fehlt; nur
  `landing.html`) — die Palette hat kein Zuhause; offen, bis die Client-Frage
  steht.
- **M02 ESP32-S3-Radiatorium-Firmware (no_std)** — der ESP32 ist ein Peer
  unter sieben: rohe Intensität wie `SeismicOscillator`. Firmware bleibt
  pending (Hardware); ethischer Puls/HRV-Filter pending (nur Spec,
  `omegaflow-sense-hardware.yaml.md:128-137`).
- **M01 WebSerial-flow-Protokoll** — die zwei Specs widersprechen sich und
  wurden nie gebaut; die Wire-Wahrheit ist die rohe Intensität. Auflösung
  erst, wenn die Architekturfrage per-Pixel-Membran (Legacy) vs heutige
  Punktwolke gemessen werden kann.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check (`/abschluss`).
