<!--
  title: Befund — pP/sP-Polarität an tiefer Geometrie: die Freiflächen-Reflexion ist im Pilotband überall invertierend (R_pp −0,65 bei 27,5°, Nullstelle 53,9°, R_sp ≈ −1); die gemessene pP-Mischung (4×+, 2×−) trägt nicht die Freifläche, sondern den Quell-Strahlungsterm
  class: befund
  date: 2026-09-09
  sha256: e28f59453e9cee829cfe803f0d8d5ff8cb556ee9b4fc1c2ded0c0023826d8e20
  status: done
  see-also: docs/TODO.md docs/befund/befund-2026-09-09-tiefenphasen-feld.md docs/concepts/die-akteure-im-boden-und-wasser.md
-->

# Befund: pP/sP-Polarität an tiefer Geometrie — die Freiflächen-Reflexion ist im Pilotband überall invertierend

## Frage & Bindung

Der Feldpilot maß an us10003re5 (231 km) pP positiv an vier, negativ an zwei
Stationen, sP durchgehend negativ — die Flachquellen-Regel („pP invertiert")
kippte; als Diagnose stand „der Reflexionskoeffizient hängt am Einfallswinkel"
(`befund-2026-09-09-tiefenphasen-feld.md`, Punkt 3). Der benannte Nachfolger:
die pP-Polarität an tiefer Geometrie ableiten, nicht aus der Flachquelle
übernehmen.

## Was abgeleitet wurde

Die Freiflächen-Reflexionskoeffizienten aus der ak135-Oberflächenschicht, die
der Code trägt (α = 5,80 km/s, β = 3,46 km/s — aus `ak135.dat` gelesen, nicht
aus dem Lehrbuch): R_pp(p) = (4p²ηξ − D²)/(4p²ηξ + D²), R_sp(p) = −4ξpD/(4p²ηξ + D²),
η=(α⁻²−p²)^½, ξ=(β⁻²−p²)^½, D=1/β²−2p². Die Vorzeichen sind durch die
Energie-Erhaltung gebunden (η = η|R_pp|² + ξ|R_ps|² bzw. ξ = ξ|R_ss|² + η|R_sp|²) —
der Test `free_surface_reflection_conserves_energy` hält das fest.

## Die Messung (Derivation gegen die Pilot-Tafel)

| Einfall i_surf | R_pp | R_sp |
|---|---|---|
| 0° | −1,00 | 0,00 |
| 10° | −0,95 | −0,41 |
| 20° | −0,80 | −0,79 |
| 30° | −0,59 | −1,10 |
| 45° | −0,21 | −1,43 |
| 60° | +0,11 | −1,68 |

R_pp kreuzt die Null bei ~53,9° (Test
`free_surface_pp_crosses_zero_at_an_oblique_angle`). Der Pilot liegt bei
Einfall ~27,5° (der pP-Abstiegszweig ist eine teleseismische P bei Δ≈30,7°):
R_pp = −0,65, R_sp = −1,03.

## Die Befunde (benannt, nicht geglättet)

1. **Die Freifläche invertiert pP im ganzen Pilotband.** R_pp ist negativ für
   jeden Einfall unter ~54°; der Pilot sitzt bei ~27,5°, weit davor. Die
   Winkel-Abhängigkeit ist real, trägt den Piloten aber nicht — der Einfall
   bleibt steil, das Vorzeichen bleibt negativ.

2. **R_sp ≈ −1 über das ganze Band:** die S→P-Konversion invertiert sP an
   jeder Station. Das gemessene „sP durchgehend negativ" wird von der
   Freifläche allein getragen.

3. **Die pP-Mischung trägt nicht die Freifläche.** Der Freiflächen-Faktor ist
   an allen sechs Stationen negativ (gleiches Vorzeichen; Δ-Streuung 1,1°);
   die gemessene Mischung (4×+, 2×−) muss aus dem Quell-Term kommen — dem
   Vorzeichen-Verhältnis der auf-/absteigenden P-Strahlung am Herd. Der Term
   ist `pending` ohne CMT-Lösung, kein erfundener Wert.

4. **Nebenbefund: der pP-Zweig ist bei Δ≈30,7° mehrdeutig.** Die pP-Laufzeitkurve
   ist dort nicht-monoton (eine Triplikation des tiefen pP-Zweigs), ein
   einzelner pP-Strahlparameter je Station ist kein sauberer Wert — ein
   Kandidat für einen Teil der Pick-Streuung des Piloten (±8 s, Punkt 2 des
   Feld-Befunds). Benannt, nicht gedeutet.

## Schließung

Der Nachfolger „pP-Polarität an tiefer Geometrie" schließt als abgeleitet und
gegen die Pilot-Tafel geprüft. Die Flachquellen-Annahme ist **verfeinert**:
pP-invertiert gilt, weil die Freifläche bei steilem Einfall invertiert — aber
das gemessene Vorzeichen ist sign(R_pp) × sign(Quell-Strahlung); der
Quell-Term bleibt `pending`. Offen bleiben: die Flotte und der Quell-Term
(CMT-Lösung).
