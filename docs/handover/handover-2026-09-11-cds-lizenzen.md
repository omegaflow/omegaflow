<!--
  title: Handover — CDS-Lizenzen + NOIRLab-TAP (Stand 2026-09-11)
  session: Quellen-Browser
  class: handover
  date: 2026-09-11
  sha256: 7ffec77aacc90af51e2fc59902a08b7b023437c81587bba11b26efb66ff21fa7
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
- **Die Frage benennen** — „Braucht die Weberin die tiefe Photometrie als
  Zeugin (S²-Richtung mit Farbe, Distanz absent) für eine Nadel?" Gemessen:
  Nadel Ⅴ (Technosignatur) frisst Lichtkurven + 10–60-μm-IR-Exzess, nicht die
  statischen Kataloge; Biosignatur frisst JWST-Spektren; dunkle Materie hat
  keinen photometrischen Kanal. Kein gebauter Konsument. Kandidat: ein
  Farbe-Exzess-Zeuge der Nadel-Ⅴ-Chromatizität oder ein Mikrolensing-/Photo-z-
  Kanal. Wer die Zeile annimmt: den Konsumenten benennen + Klasse bauen oder
  gemessen freigeben (descoped mit Befund), nie ein Regal.
- 2026-12-02 (Gaia DR4) — die Frage neu wiegen: trägt DR4 die astrometrischen
  Binaries, RVS und Epoch-Photometrie selbst, ist die dunkle-Materie-/Photo-z-
  Linie descoped und nur die Tiefe-/Plane-Nische (DECaPS-Y, LS-DR10-z) bliebe
  als Konsument; dann bauen oder gemessen freigeben.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check (`/abschluss`).
