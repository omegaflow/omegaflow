<!--
  title: Handover — CDS-Lizenzen + NOIRLab-TAP (Stand 2026-09-11)
  session: Quellen-Browser
  class: handover
  date: 2026-09-11
  sha256: 5545f3e3679fcf753b5d56e3dbbd4b5c7554f07014076d976b7572d24717ca5b
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
  Download-Form weiterhin geschlossen (Re-Check 2026-09-11); CC-BY angenommen,
  USCRN-Datenpolitik nicht. Bleibt `pending` in `phi/blocked_sources.φ`.

## NOIRLab Astro Data Lab — Parser

- Registrierung abgelehnt (Nikutta 2026-09-11), aber TAP public + LS DR10 +
  DECaPS offen: anonym gemessen (`ls_dr10.tractor`, `decaps_dr2.object`,
  FORMAT=csv). `phi/blocked_sources.φ`: `blocked account` → `blocked parser-def
  csv-header`. Offen: CSV-Header-Parser (derselbe Gap wie ESO `tap_obs`/
  `tap_cat`) oder ein dedizierter Compiler wie `des_coverage_compiler`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check (`/abschluss`).
