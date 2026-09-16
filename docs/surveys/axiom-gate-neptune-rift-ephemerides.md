<!--
  title: Axiom-Gate-Survey — neptune-rift-ephemerides
  class: survey
  date: 2026-09-16
  sha256: 785896ca32de74fbfce8919bc81a0132e725ed432c4ee7882950fa60ea5e16b2
  status: live
  see-also: docs/paper/neptune-rift-ephemerides.md
-->
# Axiom-Gate-Survey — neptune-rift-ephemerides

Das Paper `docs/paper/neptune-rift-ephemerides.md` (Titel 61/75, Abstract 121/200,
Export-Gate: `paper-check.yml` prüft Zahlen/sha im Push-Lauf).

## A = A / Zahl

- Drei Häuser am Neptun-Zentrum, 731 Epochen (1970–2030, 30-d-Schritt):
  de441−inpop19a Mittel 953 km (max 2515), de441−epm2021 2704 (7476),
  inpop19a−epm2021 3208 (9668) — 1:1 getragen.
- Dekaden-Struktur: 1980er–1990er Minimum (0,3–1,5e3 km), 2020er
  2,1e3/5,8e3/7,6e3 km — das Wachstum zum Jetzt ist der Befund.
- Der 899−8-Offset aus `nep097xl-899.bsp` liegt auf allen drei Baryzentren
  gemeinsam und kürzt sich in jeder Paardifferenz.

## Konsistenz

- Modell-gegen-Modell — keine Beobachtung entscheidet; der Uranus-Riss
  (0,36–1,57e6 m = 360–1570 km) bleibt der beobachtungsgestützte Zwilling,
  der Neptun-Riss der größere, rein modellseitige.
- Der Uranus-Survey benannte „die Neptun-Bau-Linie bleibt `pending`" —
  diese Messung löst das Pending ein.

## Pfad

- Echte CDN-Asset-Namen (gemessen, keine Platzhalter):
  `ephemeris_neptune_c.bin`, `ephemeris_inpop_neptune.bin`,
  `ephemeris_epm_neptune.bin`; Center-899-SPK `nep097xl-899.bsp`
  (naif.jpl.nasa.gov).
- Workflow `neptune-center-rift.yml`, Lauf 35145673355 (2026-09-16, success).

## Zuordnung

- see-also → `docs/paper/neptune-rift-ephemerides.md`,
  `docs/paper/uranus-rift-ephemerides.md`.

## Externes Register

- Die vier ADS-Bibcodes des Uranus-Zwillings wiederverwendet (verifiziert
  2026-09-12); keine neue DOI/arXiv-Kennung im Body — `reference_verify`
  meldet den Zustand, benannt statt beschönigt.
