<!--
  title: Befund — Neptun-Scheinbar-Orts-Kette: die Kette gebaut und an der in-range Teilmenge gemessen (Kalibrier-Gate +40/−25 mas exakt); die volle 7289-App-Reduktion wartet auf die breitere Ephemeride (derzeit J2000±30 Jahre, die Reihen liegen 1846–1969)
  class: befund
  date: 2026-09-09
  sha256: 7f2ea6917ec274fb6138f53ae16aa323a0a9faeba0ceadbd9fe29b69a53f3c23
  status: done
  see-also: docs/handover/handover-2026-09-09-neptun-astrometrie-kopplung.md docs/befund/befund-2026-09-09-neptun-astrometrie-kopplung.md docs/TODO.md
-->

# Befund: Neptun-Scheinbar-Orts-Kette — gebaut und an der in-range Teilmenge gemessen; die volle 7289-App-Reduktion wartet auf die breitere Ephemeride

## Frage & Bindung

Die Übergabe stellte die Scheinbar-Orts-Kette als die eine offene Linie: die 7289 „App"-Reihen (HILTON-Transit 1846–1969, URSS-Transit, scheinbar-of-date FK4/GC) plus die Nikolaiev-Foto-Reihen (astrometrisch B1950) brauchen die inverse Kette zurück nach ICRS. Dieser Befund baut die Kette und mißt sie an der im Ephemeriden-Zeitraum liegenden Teilmenge.

## Das Instrument

`src/archivar/astrometry.rs` trägt die Ketten-Primitive: IAU-1980-Nutation (SOFA nut80, die volle 106-Terme-Tabelle, gegen den SOFA-Testvektor 1e-15), mittlere Schiefe, Newcomb- und IAU-1976-Präzession, Aoki-1983-FK4→FK5 (Seidelmann 3.591-4 Positionsblock), FK5→ICRS-Frame-Bias, klassische Aberration (jährlich + täglich), Parallaxe, Espenak–Meeus-ΔT. Der Probe `neptune_apparent_chain_probe` (tools/measure) liest die drei Formate (HILTON token-weise, URSS/BDL feste Spalten, OBSLIST.OPT λφh) und reduziert per Reihe: Aberration → Parallaxe (nur HILTON topozentrisch) → Nutation → Newcomb-Präzession → Aoki → Frame-Bias.

## Die Messung

Die Kette ist an drei Toren gemessen:

1. **Richtung** (Unit-Tests): Newcomb(date→B1950) ist die Inverse der Aoki-Matrix; IAU-1976(B1950→J2000) deckt sich mit Aoki (Komposit ≈ Identität). Dabei kam der Konvention-Bug ans Licht: die erste Fassung präzedierte rückwärts (passive statt aktive Rotation) — der Fix `rot_z(z)·rot_y(−θ)·rot_z(ζ)` stellt die Richtung her.
2. **Kalibrier-Gate** (`--calibrate`): +40/−25 mas in die Tangentenebene injiziert → +40.185/−24.851 mas zurückgewonnen. Die 0.2-mas-Differenz ist die Aoki-Rundung (orthogonal bis 1e-7), keine Kette.
3. **In-range-Reduktion** (real): NIK-Foto-B1950 (66 Reihen): mean ΔRA·cosδ −428.6 / ΔDec +167.7 mas, RMS 528.3 / 571.3 mas. URSS-TKY (32 Reihen, 1970–1983, volle Bogensekunden): mean ΔRA·cosδ +914.7 / ΔDec +666.7 mas, RMS 3957.5 / 4894.1 mas — die Streuung trägt die Transitkreis-σ (0.3–3.8″).

## Der Deckel

`ephemeris_neptune_c.bin` und `ephemeris_earth.bin` tragen J2000 ± 30 Jahre (1970–2030, hart in `horizons_compiler --neptune-c-spk`). Die 7289 „App"-Reihen liegen 1846–1969 — außerhalb; sie werden ehrlich übersprungen (0 reduziert), nicht fabriziert. NIK (8) und GOLO (4) Reihen tragen nur Bogenminuten („0.0000 0.0000" an der Bogensekunden-Stelle) — ihre Residuen (+36″ ΔDec, −0.18° ΔRA) sind die Trunkation des Datums, nicht die Kette.

## Verdict

Die Scheinbar-Orts-Kette ist gebaut und an der in-range Teilmenge gemessen; die volle 7289-Reihen-Reduktion wartet auf eine breitere Ephemeride (Ernte-Duty, pending), nicht auf Code.

## Register-Zeilen

- Die volle 7289-App-Reduktion braucht `ephemeris_neptune_c.bin`/`ephemeris_earth.bin` mit J2000 ± 180 Jahre (1846 deckend; derzeit ± 30 Jahre = 1970–2030) — `pending` (Ernte-Duty: `horizons_compiler --neptune-c-spk` mit breiterem Bereich).
- Die NIK/GOLO-Reihen tragen nur Bogenminuten — die Bogensekunde ist im Datum absent (0-Kanon), ihre Reduktion bleibt auf die Bogenminute begrenzt.
