<!--
  title: Befund — TE-Serien-Periodizität: V/n tragen die Rotations-Faltung (K 26,2–27,5 d), AE/Dst/SYM-H die semiannuale (185 d), Bz keine stabile — die echte Faltung für cycle_phase_shift_surrogate ist gemessen
  class: befund
  date: 2026-09-10
  sha256: 1fdfff681d7cca15763340ad0c82a697a02fedea03283f23485c43eebf04e9de
  status: done
  antwortet-auf: docs/handover/handover-2026-09-09-te-blatter-bz-laic-tscaling.md (Baupunkte — cycle_phase_shift_surrogate-Nutzung, Teil 1: die Messung)
  see-also: src/mathematikerin/te.rs:1512 docs/handover/archiv/handover-2026-09-09-te-atom-4.md:34
-->

# Befund — TE-Serien-Periodizität (Teil 1 der cycle_phase_shift_surrogate-Nutzung)

Instrument: `tools/measure/src/bin/te_series_periodicity_probe.rs` (neu). Laden wie
`nobel_probe_bz` (OMNI2-1h-CDN-Assets), Lomb-Scargle mit FAP nach der lsst-Vorlage.
Messdesign (aus dem ersten Lauf korrigiert, der drei unvereinbare Antworten je Serie
lieferte): **ein** Scan, **ein** Raster — 0,5-%-Log-Raster von 10/span (≥ 10 Zyklen im
Fenster) bis zur Nyquist-Grenze der 3-h-Unterabtastung (~6 h), lokale Verfeinerung
0,1 % um das Band-Argmax. Zwei versetzte 5-%-Raster (Vollband vs. ≥2-Zyklen-Band) sind
disjunkte Frequenzmengen und messen dieselbe Serie unvereinbar — verworfen, benannt.
Die 3-h-Unterabtastung entfernt die Tages-Alias-Domäne aus dem Band. FAP-Gate 1e-3.

## Das Blatt (gemessen, beide Fenster)

| Reihe | Fenster | K (dominante Faltung) | Z | Zyklen | Befund |
|---|---|---|---|---|---|
| V | 2015–2026 | 26,17 d | 239 | 153 | Rotations-Band, robust |
| V | 2003–2014 | 27,42 d | 518 | 147 | Rotations-Band, robust |
| n | 2015–2026 | 26,14 d | 138 | 154 | Rotations-Band, robust (Feinstruktur benannt) |
| n | 2003–2014 | 27,45 d | 170 | 146 | Rotations-Band, robust |
| \|B\| | 2015–2026 | 26,17 d | 47 | 154 | Rotations-Band |
| \|B\| | 2003–2014 | 381,11 d | 44 | 10,5 | jahresnahes Band, keine Rotation |
| AE | 2015–2026 | 184,79 d | 142 | 21,7 | semiannual, robust |
| Dst | 2015–2026 | 184,79 d | 444 | 21,7 | semiannual, robust |
| SYM-H | 2015–2026 | 185,35 d | 224 | 21,7 | semiannual, robust |
| Bz | 2015–2026 | 1,70 d | 15 | 2362 | keine stabile Faltung |
| Bz | 2003–2014 | 405,81 d | 23 | 9,9 | keine stabile Faltung |
| AE/Dst/SYM-H | 2003–2014 | — | — | — | Indices-Asset deckt das Fenster nicht ab (abwesend, nicht null) |

Alle FAP < 1e-3 über 152 Frequenzen (Gate Z ≈ 11,9).

## Der Verdikt-Satz

Die echte Faltung im TE-Layer existiert — gemessen, nicht hergeleitet. Die
Solarwind-Reihen **V und n** tragen die dominante Rotations-Band-Faltung in beiden
unabhängigen Fenstern (K 26,2–27,4 d, innerhalb 10 % der Carrington-Rotation 27,28 d;
≈ 150 Wiederholungen je Fenster) — die stabilste, wiederholte Periode der Messung.
Die Boden-Indizes **AE, Dst, SYM-H** tragen die semiannuale Faltung (K ≈ 185 d,
21,7 Zyklen, Z bis 444; deckungsgleich über zwei unabhängige Raster). **Bz** — der
Haupt-Treiber-Kanal der TE-Linie — trägt keine stabile Faltung: sein Band-Argmax
wechselt mit dem Fenster (1,7 d vs. 405,8 d) bei niedrigem Z (15/23). **|B|** trägt
die Rotation nur im 2015–26-Fenster (Z 47); im 2003–14-Fenster dominiert ein
jahresnahes Band (381 d, Z 44).

## Benannte Mess-Grenzen (aus drei Instrument-Versionen gemessen)

- Haarfeine (Raster-schmale) Linien erzeugen Raster-Glückstreffer: dieselbe Bz-Serie
  meldete unter zwei versetzten 5-%-Rastern 29,1 d (Z 57) bzw. 418 d (Z 39) — das
  waren keine stabilen Faltungen, sondern Einzelpunkt-Treffer. Das eine 0,5-%-Raster
  + 0,1-%-Verfeinerung ist selbstkonsistent; die Robustheit steht über die
  Fenster-Wiederholung, nicht über den Einzelwert.
- n zeigt Feinstruktur um die 26-d-Linie (ein versetztes Raster traf Z 416, das
  verfeinerte misst 138 — die Linie ist schmaler als die 0,1-%-Verfeinerung an
  dieser Stelle); das aufgelöste Band-Argmax (Z 138–170, stabil über Fenster) trägt
  den Befund.
- Der Refine-Pass (0,1 % um das Argmax) kann bis ~1 % über die Band-Kante
  10/span greifen (Bz 2003–14: 405,8 d bei nominal 401,8 d Grenze) — benannt, kein
  Gate-Verstoß.

## Register-Duty (nächste Adresse)

Die Nutzung der Primitive `cycle_phase_shift_surrogate` (te.rs:1512) ist durch die
Messung geöffnet: eine echte Faltung existiert, ihr K ist gemessen. Der Bau von
`TeNull::CycleShift(K)` gegen ein gemessenes K (V/n: K ≈ 26–27,5 d; AE/Dst/SYM-H:
K ≈ 185 d) bleibt — wie vom Rat entschieden — eine eigene, nächste Sitzung mit
voller Kalibrier-Gate-Batterie (FP/FN/Symmetrie/n-Floor); das Archiv-Gate „Nutzung
pending bis eine echte Faltung im Layer existiert" (atom-4:34) ist mit diesem Befund
als Mess-Gate erfüllt und die Baupunkte-Hälfte (a) als gemessen geführt. Bz trägt
kein K — die Shift-Null am Bz-Betriebspunkt bleibt vom Periodizitäts-Leak unberührt.
