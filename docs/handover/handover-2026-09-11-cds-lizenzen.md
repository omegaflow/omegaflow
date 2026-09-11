<!--
  title: Handover — CDS-Lizenzen + NOIRLab-TAP (Stand 2026-09-11)
  session: Quellen-Browser
  class: handover
  date: 2026-09-11
  sha256: 289345a3187c420a6cd021cfec8b21f2c4910b924890d9bbc9f743a4c679648b
  status: live
-->
# Handover — CDS-Lizenzen + NOIRLab-TAP (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## CDS Copernicus — Compiler

- Die 5 datensatzspezifischen Lizenzen (GNSS, GRUAN, IGRA, surface-land,
  WOUDC) sind im ECMWF-Konto `code@omegaflow.space` angenommen (Browser,
  gemessen 2026-09-11). Die `blocked account`-Einträge sind aus
  `phi/blocked_sources.φ` ausgetragen. Offen: die `copernicus_cdm_obs`-Compiler
  für die 5 + `insitu-observations-surface-marine` +
  `insitu-comprehensive-upper-air-observation-network` (alle `compiler-lease`
  in `phi/pipeline/catalog/copernicus_disposition.φ`, Compiler pending).
- US-CRN (`insitu-observations-near-surface-temperature-us-climate-reference-network`):
  Download-Form provider-seitig geschlossen (Re-Check 2026-09-11: CC-BY +
  USCRN-Datenpolitik angenommen, `Submit form` disabled). Bleibt `pending` in
  `phi/blocked_sources.φ`; nur die Form-Closure offen.

## NOIRLab Astro Data Lab — Frage (Rat 2026-09-11)

- Registrierung abgelehnt (Nikutta 2026-09-11), aber TAP public + LS DR10 +
  DECaPS offen: anonym gemessen (`ls_dr10.tractor`, `decaps_dr2.object`,
  FORMAT=csv). `phi/blocked_sources.φ`: `blocked account` → `blocked parser-def
  csv-header`, mit der Frage-Referenz im Eintrag.
- **Keine Ernte heute** (Rat, einmütig): Tor 1 — keine registrierte Frage
  frisst reine Photometrie; das 44-Byte-Sternfeld verlangt plx>0 + endliches
  rv, LS DR10/DECaPS tragen rv nicht (rv=0.0 wäre Fabrication).
- **Die Frage benennen** — „Frisst das Feld die tiefe Photometrie zwischen den
  Gaia-Sternen — als Richtungs-Stern mit Farbe (Distanz absent) und als
  3D-Stern, wo der Gaia-Crossmatch die Distanz trägt?" Konsument = eine
  photometrische Record-Klasse (Richtung + mag/Flux + Farbe,
  Distanz-Präsenz-Bit, niemals rv=0.0) plus die plx>0-Teilmenge als
  Sternproben-Erweiterung. Wer die Zeile annimmt: Klasse bauen oder gemessen
  freigeben (descoped mit Befund), nie ein Regal.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check (`/abschluss`).
