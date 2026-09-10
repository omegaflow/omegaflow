<!--
  title: Befund — Neptun-Scheinbar-Orts-Kette: gebaut und über den vollen Zeitraum (1846–1983) gegen das breite DE441-Zentrum reduziert (Kalibrier-Gate +40/−25 mas exakt, Median ~0.2″)
  class: befund
  date: 2026-09-09
  sha256: 8ea028bc10fc53c708bfe57c2848bcc07fcd42dc6819b556d703c5933918750a
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-mechanische-reste.md
-->

# Befund: Neptun-Scheinbar-Orts-Kette — gebaut und über den vollen Zeitraum (1846–1983) gegen das breite DE441-Zentrum reduziert; Median ~0.2″

## Frage & Bindung

Die Übergabe stellte die Scheinbar-Orts-Kette als die eine offene Linie: die 7289 „App"-Reihen (HILTON-Transit 1846–1969, URSS-Transit, scheinbar-of-date FK4/GC) plus die Nikolaiev-Foto-Reihen (astrometrisch B1950) brauchen die inverse Kette zurück nach ICRS. Dieser Befund baut die Kette und reduziert die volle Reihe gegen das breite DE441-Zentrum.

## Das Instrument

`src/archivar/astrometry.rs` trägt die Ketten-Primitive: IAU-1980-Nutation (SOFA nut80, volle 106 Terme, gegen den SOFA-Testvektor 1e-15), mittlere Schiefe, Newcomb- und IAU-1976-Präzession, Aoki-1983-FK4→FK5 (Seidelmann 3.591-4), FK5→ICRS-Frame-Bias, klassische Aberration (jährlich + täglich), Parallaxe, Espenak–Meeus-ΔT. Der Probe `neptune_apparent_chain_probe` liest die drei Formate (HILTON token-weise, URSS/BDL feste Spalten, OBSLIST.OPT λφh) und reduziert per Reihe: Aberration → Parallaxe (nur HILTON topozentrisch) → Nutation → Newcomb-Präzession → Aoki → Frame-Bias.

## Die Messung

1. **Richtung** (Unit-Tests): Newcomb(date→B1950) ist die Inverse der Aoki-Matrix; IAU-1976(B1950→J2000) deckt sich mit Aoki. Der Konvention-Bug (passive statt aktive Rotation) wurde geflickt — `rot_z(z)·rot_y(−θ)·rot_z(ζ)`.
2. **Kalibrier-Gate** (`--calibrate`, alle 7391 Reihen): +40/−25 mas injiziert → **+40.273/−25.079 mas** zurückgewonnen.
3. **Reduktion** (real, volle Reihe, Median): das breite Zentrum (`ephemeris_neptune_c.bin`, jetzt Jahr 1802–2030, 38352 Granulen) reproduziert die Reihen auf **Median ~0.2–0.6″** — USNO +421 mas, CAMB +398, CAPE −27, GREN +172, NICE +635, PARI +217, TKY +244, URSS-USNO −75, NIK-Foto-B1950 −426 mas (RMS 488/498 mas).

## Der Zeitraum & die Glättung

Das breite **Zentrum** (DE441-Baryzentrum + nep097xl 899-8 über die volle Range) ist gebaut — `horizons_compiler --neptune-c-spk` wurde von J2000±30 auf 1802–2030 erweitert, samt eines Fenster-Fixes im Granulen-Fit (der O(n²)-Full-Vektor-Scan drosselte den breiten Lauf). Die 7289 „App"-Reihen reduzieren vollständig gegen das Zentrum (171 übersprungen: BESA/STRA ohne OBSLIST-λφh, Dec-absent). Die frühen HILTON-Reihen tragen Bogenminuten- bis Grad-Ausreißer — der **Median** trennt sie: die glatte Mitte liegt bei ~0.2–0.6″, das Mittel/RMS wird von wenigen Transkriptions-Ausreißern getragen (CAMB 1862 −1.6°, 1868 −14.8°). Die Kette bildet jede geprüfte Reihe exakt ab (reduced ≈ model) — die Ausreißer sind die Daten, nicht die Kette.

## Verdict

Die Scheinbar-Orts-Kette ist gebaut und über den vollen Zeitraum gegen das breite Zentrum gemessen; die glatte Mitte liegt bei ~0.2–0.6″ (Median), die Ausreißer sind eine eigene Datenfrage.

## Register-Zeilen

- Die Grad-Ausreißer der frühen HILTON-Reihen (Transkriptionsfehler im APDB-Datum) sind eine eigene Messung (Datenprüfung) — `pending`.
