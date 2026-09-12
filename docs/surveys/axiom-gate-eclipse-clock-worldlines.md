<!--
  title: Axiom-Gate-Survey — eclipse-clock-worldlines
  class: survey
  date: 2026-09-12
  sha256: 8c0bc729018483cf41baa108116c67854dae89b901335583d0e791aada6678df
  status: live
  see-also: docs/paper/eclipse-clock-worldlines.md
-->
# Axiom-Gate-Survey — eclipse-clock-worldlines

Das Paper `docs/paper/eclipse-clock-worldlines.md` (Titel 67/67,
Abstract 138/200, 134 Zahlen ok, sha ok — Export-Gate grün).

## A = A / Zahl

- 4.8 km / +0.0004 mag / 65 s vor dem Kanon — 1:1 aus dem
  Finsternis-Handover getragen; der 65-s-Offset steht als Restbefund
  (flaches h-Minimum), kein Physik-Claim.
- Die drei NASA-Editionen eine Stimme: Mond 0.3–2.0 m, < 2 ms; inpop19a
  3.05 s / 1.7 km; Sonnen-Triade 22.4/16.8/32.0 km — unverändert.

## Konsistenz

- Alle fünf Referenz-Kennungen belegt (ADS-API, 2026-09-12): 2006fmcs.book.....E
  (der Erstansatz 2006fmsc war absent — korrigiert), 2014IPNPR.196C...1F,
  2021AJ....161..105P, 2019NSTIM.109.....F (Fienga, INPOP19a); Pitjeva &
  Pitjev auf das verifizierte 2014-Record korrigiert (CeMDA 119, 237, DOI
  10.1007/s10569-014-9569-0) — die 2018-Zeile (CeMDA 130, 57) war eine
  Verschmelzung zweier Records.
- Der Matrix-Defekt im Paper: Builder-Fix (2026-09-09) getragen, vier von
  fünf stale Bins rekompiliert + re-verifiziert (orientation_probe Δ 0,0 km
  auf de440/de442/inpop/epm earth); de441 mars bleibt als benannter Stein.

## Pfad

- `tools/measure/src/bin/eclipse_shadow_probe.rs` (drei Stufen,
  Kalibrier-Gate: Punkt < 15 km, Magnitude < 0.002); Oberflächen-Abbildung
  über `body_fixed_to_icrs_smooth` — stimmen.
- Der `body_fixed_to_icrs`-Matrix-Pfad-Defekt ist im Paper getragen als
  gefixt + re-verifiziert (vier Linien Δ 0,0 km); de441 mars bleibt
  benannter Stein.

## Zuordnung

- see-also → `docs/handover/archiv/handover-2026-09-09-finsternis-schattenortung.md`
  (archiviert, lebt).

## Externes Register

- DOI 10.3847/1538-3881/abd414 `resolved`.
- NASA-Bindung: ssd.jpl.nasa.gov de440/de441/de442 (CDN-Assets im Body),
  Horizons-Banner `{source: DE441}`, NASA/TP-2006-214141 (Espenak & Meeus).
