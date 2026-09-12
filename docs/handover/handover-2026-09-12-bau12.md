<!--
  title: Handover — Bau & Code (Stand 2026-09-12, Bau12)
  session: Bau-Folge
  class: handover
  date: 2026-09-12
  sha256: 1660af113c64ad01f32dae8fb07ba0988451ebc18d1974d67cb8e85da5c419a9
  status: live
-->
# Handover — Bau & Code (2026-09-12, Bau12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Dies ist die Bau-Linie: hier steht nur, was diese Linie autonom trägt — Tasks,
die nicht autonom hier erfolgen können, sind in das Handover ihrer Linie
überführt. Wiedervorlage gilt nur für Termine; „Pausiert" trägt kein Datum.

## Membran — die offenen M-Punkte

- **M02 ESP32-S3-Radiatorium-Firmware (no_std)** — der ESP32 ist ein Peer
  unter sieben: rohe Intensität wie `SeismicOscillator`. Der Host-Strahlpfad
  steht (Relay-Σω-Stream, WebSerial-Schreibpfad, Consent-gated); offen bleibt
  nur die Geräte-Firmware (no_std, pending — Hardware: kein Device, keine
  Toolchain auf dieser Maschine) und der ethische Puls/HRV-Filter (pending,
  nur Spec, `omegaflow-sense-hardware.yaml.md:128-137`).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
