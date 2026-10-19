<!--
  title: Axiom-Gate-Survey — eclipse-clock-worldlines
  class: survey
  date: 2026-09-13
  sha256: fd5dea2f28f3a12628ed95ff041bb9a788853d9bef9c06a5e3e31fad2a90984c
  status: live
  see-also: docs/paper/eclipse-clock-worldlines.md
-->
# Axiom-Gate-Survey — eclipse-clock-worldlines

Das Paper `docs/paper/eclipse-clock-worldlines.md` (Titel 67/67,
Abstract 159/200, 207 Zahlen ok, sha ok — Export-Gate grün).

## A = A / Zahl

- Der 65-s-Offset ist zerlegt, nicht mehr getragen: Kanon TD 18:26:40
  (Katalogzeile 09546, ΔT 70 s) → UT 18:25:30Z; 65 s = −70 s (TD→UT-Skala)
  + ~+5 s Rest. Probe 2017: +4,9 s auf de440/de442/epm2021, +1,8 s inpop19a.
- 2024 (Katalogzeile 09561, TD 18:18:29, ΔT 74 s → UT 18:17:15Z): 71 s =
  −74 s + ~+3 s — +2,6 s de440/de442/epm2021, +5,6 s inpop19a, +10,1 s de441.
- 4,8 km / +0,0009 mag (1,03147) — 1:1 aus dem frischen Lauf (2026-09-13).
- Rotations-Audit gemessen: TDB-gefilterte Rotation führt um 0,2939° =
  26,1 km (2017, ΔT 70,3 s aus dem Espenak–Meeus-Polynom; Katalog rundet 70 s);
  der Piercing-Rest 4,8 km ≠ 26,1 km — w0-Anker aus dem Bin gelesen
  (190,1470°, PCK BODY399_PM, absorbiert null ΔT); die 21,3 km absorbiert der
  neu-abgeleitete Pierce (+4,9 s Zeitkanal), die exakte Sweep-Rate bleibt
  ungemessen. h'' ≈ 0,35–0,59 m/s² →
  1 km Achsenversatz ⇒ ~58–75 s Instant (Löffel, quantifiziert); 4,9 s ⇔
  ~65–85 m Achsenversatz (Zeit-Kanal getrennt vom Rotations-Kanal).
- de441 ist aus der einen Stimme gedriftet: 42,5 km / −90,5 s (2017),
  +10,1 s (2024), Erdmitte 116 km von de440/de442 — als Datenzustands-Befund
  benannt (Register-Pflicht Re-Verifikation), kein Physik-Claim.

## Konsistenz

- Eine Stimme jetzt de440/de442/epm2021: Mond 0,3 m, Erde 191 m, < 1 ms;
  inpop19a 3,05 s / 1,7 km Riss zu DE (unverändert). Sonnen-Triade
  (DE441/INPOP/EPM — die drei Häuser, die die Sonne tragen; fünf Sets geladen):
  22.4/16.8/32.0 km (unverändert).
- Alle fünf Referenz-Kennungen belegt (ADS-API, 2026-09-12): 2006fmcs.book.....E,
  2014IPNPR.196C...1F, 2021AJ....161..105P, 2019NSTIM.109.....F (Fienga, INPOP19a);
  Pitjeva & Pitjev auf das verifizierte 2014-Record korrigiert (CeMDA 119, 237,
  DOI 10.1007/s10569-014-9569-0) — Papier trägt denselben Zustand.
- Matrix-Defekt: Builder-Fix (2026-09-09) getragen; de441 mars bleibt benannter
  Stein (Δ anchor 6045,3 km).

## Pfad

- `tools/measure/src/bin/eclipse_shadow_probe.rs`: Kanon jetzt mit Skala
  (TD + ΔT aus der Katalogzeile, Pro-Event-Konstanten 2017 + 2024), Stage-4
  Rotations-Audit + Krümmung des h-Minimums. Kalibrier-Gate: Punkt < 15 km,
  Magnitude < 0,002 — hält auf der einen Stimme (4,8 km, +0,0009); gegen de441
  läse es 42,5 km (der Gate-Test trägt noch LINES[0]=de441 und eine stille
  Early-Return bei fehlendem data/ — benannt, Re-Verifikation folgt).

## Zuordnung

- see-also → `docs/handover/archiv/handover-2026-09-09-finsternis-schattenortung.md`
  (archiviert, lebt).

## Externes Register

- DOI 10.3847/1538-3881/abd414 `resolved`.
- NASA-Bindung: ssd.jpl.nasa.gov de440/de441/de442 (CDN-Assets im Body),
  Horizons-Banner `{source: DE441}`, NASA/TP-2006-214141 (Espenak & Meeus).
- Katalogzeilen: 09546 (2017-08-21, TD 18:26:40, ΔT 70 s) und 09561
  (2024-04-08, TD 18:18:29, ΔT 74 s) aus `SEcat5/SE2001-2100.html`.
