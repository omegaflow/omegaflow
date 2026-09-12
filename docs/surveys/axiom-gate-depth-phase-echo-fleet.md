<!--
  title: Axiom-Gate-Survey — depth-phase-echo-fleet
  class: survey
  date: 2026-09-12
  sha256: 05744af4cf8a2086d2c7da3ce420eb4fe7d1494c893464811fae34b422317e3b
  status: live
  see-also: docs/paper/depth-phase-echo-fleet.md
-->
# Axiom-Gate-Survey — depth-phase-echo-fleet

Das Paper `docs/paper/depth-phase-echo-fleet.md` (Titel 59/59,
Abstract 159/200, 184 Zahlen ok, sha ok — Export-Gate grün).

## A = A / Zahl

- Flotte +1.7 km, se 4.7 km gegen das ±10-km-Gate; Streuung 19 km über
  Ereignisse, 36 km über Stationen; Feldpilot us10003re5: 250 km gegen
  Katalog 231 km (+19 km, außerhalb); Positivkontrolle 14.6 km, rms
  1.887 s, ak135 P+S < 0.15 s — 1:1 getragen.

## Konsistenz (Divergenz aufgelöst)

- Die 700/250-Divergenz ist aufgelöst (2026-09-12): der Code trug die
  Wahrheit — `MAX_DEPTH_KM = 700.0` (`src/archivar/ak135.rs:7`), die
  Tiefenmodell-Erweiterung über 250 km ist seit 2026-09-09 geschlossen
  (250 → 700, `DEPTH_KM`-Raster bis 700, Inversion bis 700 km), das Modell
  `ak135.dat` trägt Tiefenzeilen bis 6371 km. Die Konzept-/Register-Zeile
  („Erweiterung > 250 km") war veraltet und ist korrigiert; das Paper trägt
  die Erweiterung als geschlossen.
- Kennett & Engdahl 1991 (Methode) und Kennett, Engdahl & Buland 1995
  (ak135-Modell) beide zitiert — der ak135.dat-Kopf trägt 1995.

## Pfad

- `tools/measure/src/bin/depth_phase_fleet_probe.rs`,
  `depth_phase_field_probe.rs`, `tools/measure/src/depthphase.rs`,
  `src/archivar/ak135.rs` — stimmen.

## Zuordnung

- see-also → `docs/concepts/die-akteure-im-boden-und-wasser.md` (lebt).
- Benannte Pendings im Paper: CMT-Quell-Strahlungsterm, Kalibrier-Gate
  (sechs Stationsazimute in keinem Register), Mehrdeutigkeits-Zweig bei
  Δ≈30°.

## Externes Register

- 5/5 DOIs `resolved` (doi.org), darunter Kennett-1991, KEB-1995,
  ISC-GEM 2013 (+ Datensatz-DOI), USGS ANSS ComCat.
- Datenquellen ehrlich benannt: IRIS/EarthScope FDSN, USGS, GEOFON —
  keine NASA-Fassade; NASA-Anbindung = Literatur-Crosscheck (ADS).
