<!--
  title: Axiom-Gate-Survey — neptune-rift-ephemerides
  class: survey
  date: 2026-09-16
  sha256: 21c08362c6c5a55d7f381531060e4d3d4505bb494c4c4a9a0ba863f3cfdb6f93
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
- Der Uranus-Survey benannte am 2026-09-12 „die Neptun-Bau-Linie bleibt
  `pending`"; der Bau (Compiler `neptune_ephemeris_compiler`, Registrierung
  `phi/sources.φ:3456` `ephemeris_de440_neptune.bin`/`:3232`
  `ephemeris_neptune_c.bin`, kernel-flatten `neptune-de440s-cdn.yml`) und
  diese Messung vom 2026-09-16 lösen es ein — kein Riss, die frühere
  `pending`-Zeile ist abgelöst (gemessen 2026-09-25 via
  `sgrep ephemeris_neptune phi/sources.φ`).
- Die frühere `resolved`-Linie ist geschlossen (`descoped`): die Neptun-Bau-Linie
  ist gebaut (`phi/sources.φ:3456` `ephemeris_de440_neptune.bin`, `:3232`
  `ephemeris_neptune_c.bin`), kein unerledigter Schritt.

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
