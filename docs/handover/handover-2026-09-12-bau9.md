<!--
  title: Handover — Bau & Code (Stand 2026-09-12, Bau9)
  session: Bau-Folge
  class: handover
  date: 2026-09-12
  sha256: f769acc1fdcaabc838eb448fb86e3a1de817c3f768972123643ca6b1f0c5015c
  status: live
-->
# Handover — Bau & Code (2026-09-12, Bau9)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

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

## Serien-τ — der verbliebene Registrierungs-Dienst

Die τ-Spalte lebt jetzt in der Registry (`sensor_config`), getrennt vom ttl;
der Draht trägt τ weiter als `Option` (absent bleibt absent); battery trägt
ihr τ (level/voltage/charging 60.0, current 10.0) aus der Registry — ein Ort,
ein Gesetz. Die Nicht-Battery-Serien-Sensoren (temperature, pressure,
humidity, wind, …) tragen noch kein τ → nicht feldfähig, pending: die
gemessene Relaxation fehlt (Registrierungs-Dienst, keine Fabrication).

## Benannte Eigenschaften — nicht gebaut (dieses Atom)

- **Kernel-Konfund electric** — die electric-Familie (force 8) trägt in
  `sensor_config` kernel 5 (Linien-Zuordnung), die Kraft-Tabelle kennt
  `kernel_id_for_force(8) = 1` (Feld-Erzeuger) und `default_kernel_for("electric")
  = None`. Zwei elektrische Wahrheiten im Baum; benannt, nicht umgebaut.
- **Seismische Apertur-Mitführung** — `acoustic_sample` speist acoustic und
  seismic; die Apertur dämpft beide Strahler. Ein Feld, eine Permeabilität,
  zwei Strahler — der Name trägt jetzt zwei Kanäle. Benannt, nicht umbenannt.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check (`/abschluss`).
