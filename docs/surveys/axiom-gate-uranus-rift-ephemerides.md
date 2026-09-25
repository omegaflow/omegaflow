<!--
  title: Axiom-Gate-Survey — uranus-rift-ephemerides
  class: survey
  date: 2026-09-12
  sha256: 10d90ff3220fe411bbb42b5f467f418e027b0ae6d2225c5972efddbdd14bdacf
  status: live
  see-also: docs/paper/uranus-rift-ephemerides.md
-->
# Axiom-Gate-Survey — uranus-rift-ephemerides

Das Paper `docs/paper/uranus-rift-ephemerides.md` (Titel 59/59,
Abstract 130/200, 119 Zahlen ok, sha ok — Export-Gate grün).

## A = A / Zahl

- DE−INPOP 32.1, DE−EPM 39.5, INPOP−EPM 47.0 mas (se ~1.3 mas); diurnale
  Reduktion RMS 242.9/222.6/228.9 → 77.5/73.6/73.3 mas unter ⟨σ⟩ = 87.8 mas;
  Planetenzentrum 0.36–1.57e6 m (0.048″ median) — 1:1 getragen.

## Konsistenz

- Der Innenplanet-Kontrast fehlt in beiden Quellen-Handovers — im Paper
  `absent` benannt, nicht fabriziert.
- Die Absolut-Offset/Aberrations-Zerlegung ist gemessen (EPM2021 am nächsten
  bei Null, |c0| 11,8 mas). Die Neptun-Bau-Linie war am 2026-09-12 `pending`;
  sie ist gebaut (Compiler `neptune_ephemeris_compiler`, Registrierung
  `phi/sources.φ:3456` `ephemeris_de440_neptune.bin`/`:3232`
  `ephemeris_neptune_c.bin`, kernel-flatten `neptune-de440s-cdn.yml`) — das
  `pending` ist eingelöst, kein Riss (gemessen 2026-09-25 via
  `sgrep ephemeris_neptune phi/sources.φ`).

## Pfad

- Echte CDN-Asset-Namen im Body (gemessen, keine Platzhalter):
  `ephemeris_uranus.bin`, `ephemeris_uranus_c.bin`,
  `ephemeris_de440_earth.bin`, `ephemeris_de442_earth.bin`,
  `ephemeris_inpop_uranus.bin`, `ephemeris_epm_uranus.bin`.

## Zuordnung

- see-also → `docs/handover/archiv/handover-2026-09-09-uranus-riss-kontur.md`
  (archiviert, lebt).

## Externes Register

- Der Body trägt keine DOI/arXiv-Kennung — `reference_verify` meldet den
  Zustand („no arXiv id, no DOI in body"); benannt, nicht beschönigt.
- Die vier ADS-Bibcodes sind belegt (ADS-API, 2026-09-12, alle resolved):
  2014IPNPR.196C...1F, 2021AJ....161..105P, 2011CeMDA.111..363F,
  2014CeMDA.119..237P.
