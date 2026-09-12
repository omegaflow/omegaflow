<!--
  title: Handover — Bau & Code (Stand 2026-09-12, Bau10)
  session: Bau-Folge
  class: handover
  date: 2026-09-12
  sha256: 19a77d1697bcd955d12b1db7f7b1600f7b3fc1171a7b78b9d34ce19d283db1dc
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

- **M02 ESP32-S3-Radiatorium-Firmware (no_std)** — der ESP32 ist ein Peer
  unter sieben: rohe Intensität wie `SeismicOscillator`. Firmware bleibt
  pending (Hardware); ethischer Puls/HRV-Filter pending (nur Spec,
  `omegaflow-sense-hardware.yaml.md:128-137`).
- **Client-Wirt** — kein Browser-Client (`static/index.html` fehlt; nur
  `landing.html`), keine Fenster-Schicht (kein Windowing-Crate,
  `compatible_surface: None`). M07 (Palette) und M01 (WebSerial) sind descoped
  und gehen hier auf: sobald der Client steht, sind beide kleine Aufsätze. Die
  Renderfrage ist gemessen entschieden (Punktwolke live, per-Pixel-Legacy tot).

## Descoped (gemessen)

- M07 ⌘K-Palette — kein autonomer Bauweg: Substrat (`load_sources` + SIMBAD)
  lebt, aber das Zuhause fehlt (kein Client, kein Endpoint, keine Fenster-
  Schicht). Geht in den Client-Wirt auf; die drei Suchfunktionen leben in
  CLI-Proben.
- M01 WebSerial-flow-Protokoll — das Kanal-Textprotokoll (Spec A) gehört zur
  toten per-Pixel-/Kanal-Ära (Backport-Verbot, radiators.md §3); die gebaute
  Wire ist Σω-Rohintensität (`actuators.rs`). Offen bleibt nur der Client-Wirt,
  kein Protokoll.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check (`/abschluss`).
