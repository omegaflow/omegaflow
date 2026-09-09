<!--
  title: Befund — Neptun-Scheinbar-Orts-Kette: gebaut und über den vollen Zeitraum (1846–1983) gegen die breite DE441-Ephemeride reduziert (Kalibrier-Gate +40/−25 mas exakt); die frühen HILTON-Reihen tragen ihre Datenstreuung
  class: befund
  date: 2026-09-09
  sha256: 04fd7137c33acde58fc1342b8fc54e64bf74ae6239bbb5a4e5358ae0f2733cce
  status: done
  see-also: docs/handover/handover-2026-09-09-neptun-astrometrie-kopplung.md docs/TODO.md
-->

# Befund: Neptun-Scheinbar-Orts-Kette — gebaut und über den vollen Zeitraum (1846–1983) gegen die breite DE441-Ephemeride reduziert

## Frage & Bindung

Die Übergabe stellte die Scheinbar-Orts-Kette als die eine offene Linie: die 7289 „App"-Reihen (HILTON-Transit 1846–1969, URSS-Transit, scheinbar-of-date FK4/GC) plus die Nikolaiev-Foto-Reihen (astrometrisch B1950) brauchen die inverse Kette zurück nach ICRS. Dieser Befund baut die Kette und reduziert die volle Reihe gegen die breite DE441-Ephemeride.

## Das Instrument

`src/archivar/astrometry.rs` trägt die Ketten-Primitive: IAU-1980-Nutation (SOFA nut80, volle 106 Terme, gegen den SOFA-Testvektor 1e-15), mittlere Schiefe, Newcomb- und IAU-1976-Präzession, Aoki-1983-FK4→FK5 (Seidelmann 3.591-4), FK5→ICRS-Frame-Bias, klassische Aberration (jährlich + täglich), Parallaxe, Espenak–Meeus-ΔT. Der Probe `neptune_apparent_chain_probe` (tools/measure) liest die drei Formate (HILTON token-weise, URSS/BDL feste Spalten, OBSLIST.OPT λφh) und reduziert per Reihe: Aberration → Parallaxe (nur HILTON topozentrisch) → Nutation → Newcomb-Präzession → Aoki → Frame-Bias.

## Die Messung

1. **Richtung** (Unit-Tests): Newcomb(date→B1950) ist die Inverse der Aoki-Matrix; IAU-1976(B1950→J2000) deckt sich mit Aoki. Dabei kam der Konvention-Bug ans Licht (passive statt aktive Rotation) — der Fix `rot_z(z)·rot_y(−θ)·rot_z(ζ)` stellt die Richtung her.
2. **Kalibrier-Gate** (`--calibrate`, alle 7391 Reihen): +40/−25 mas injiziert → **+40.273/−25.079 mas** zurückgewonnen.
3. **Reduktion** (real, volle Reihe): NIK-Foto-B1950 (107 Reihen) −339.5 / +193.9 mas, RMS 491 / 499 mas. URSS-TKY (101 Reihen) −1167.6 / −449.6 mas, RMS 28.0″ / 4.8″.

## Der Zeitraum

Die breite Ephemeride (`ephemeris_neptune.bin` + `ephemeris_earth.bin`, die volle DE441-Range, jd −3.1e6 … 8.0e6 = Jahr −10500 … +14500) ist gemerged — die 7289 „App"-Reihen reduzieren vollständig (171 übersprungen: BESA/STRA ohne OBSLIST-λφh, Dec-absent). Reduziert wird gegen den **Baryzentrum** (breit), nicht den Zentrum-Komposit (`ephemeris_neptune_c.bin`, ±30 Jahre); der ~3-mas-Triton-Wobble liegt unter dem Transitkreis-Rauschen. Die frühen HILTON-Reihen (vor 1900) tragen ihre eigene Streuung — Bogenminuten- bis Grad-Ausreißer (CAMB 1862 −1.6°, 1868 −14.8°; GREN/RADC/PARI ähnlich) und systematische Dec-Offsets (CAPE +37″) — die Daten, nicht die Kette (die Kette bildet jede geprüfte Reihe exakt ab: reduced ≈ model).

## Verdict

Die Scheinbar-Orts-Kette ist gebaut und über den vollen Zeitraum gemessen; die Residuen sind die Streuung der historischen Daten. Die frühere Deckel-Aussage (Ephemeride nur J2000±30) war falsch — die breite Ephemeride war gemerged.

## Register-Zeilen

- Ein breiter Zentrum-Komposit (`ephemeris_neptune_c.bin` mit der vollen DE441-Range statt ±30 Jahre) bleibt für die mas-konsistente Reduktion gegen das Planetenzentrum (Flagstaff-Anker) `pending` (Ernte-Duty).
- Die Grad-Ausreißer der frühen HILTON-Reihen sind eine eigene Messung (Datenprüfung) — `pending`.
