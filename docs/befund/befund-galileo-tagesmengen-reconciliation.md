<!--
  title: Befund — Tagesmengen-Reconciliation Mode 2: der 1,5→0,65-Hz-Tag-Satz (All-Lock-Tag-Zählung gemessen; die 30–60°-Deutung trägt nicht)
  class: befund
  date: 2026-09-05
  sha256: 958576e43e08d3f37b71e71756355f1086d60f9246808fd20ba62cdd5f533f63
  status: done
  antwortet-auf: docs/befund/befund-galileo-mode2-station-split.md docs/befund/befund-galileo-pass-segmentierung.md
  see-also: docs/befund/befund-galileo-rausch-kurve-epsilon.md
-->

# Befund: Tagesmengen-Reconciliation Mode 2 — der 1,5→0,65-Hz-Tag-Satz

## Frage & Bindung

Beide Vorgänger messen das ruhige Mode-2-Konjunktionsfenster (ε < 30°, Mode 2) und
widersprechen sich in der Deutung der Tagesmengen-Differenz: die ε-Kurve
(`befund-galileo-rausch-kurve-epsilon`) trägt 1,5 Hz über 77 Tage, die
Pass-Segmentierung 0,65 Hz über 75 Tage. E1 (Mode-2-Station-Tag-Split) deutet die
Differenz als Zählung zweier All-Lock-Tage (1997-01-07, ε 10,2°; 1997-01-21,
ε 1,0°) über einen Median-Index-Effekt an einer bimodalen Lücke. E2
(Pass-Segmentierung) deutet sie als acht in ε 30–60° verschobene laute Tage
(77→75 schließe arithmetisch nicht). Beide registrierten den Abgleich der
Tagesmengen-Schnitte als `pending`. Dieser Entwurf misst die tatsächlichen
Tag-Mengen beider Konventionen gegeneinander.

Gebunden: ε (solare Elongation an der Erde) am TDB-Tagesanfang, Band =
`floor(ε/30)` — dieselbe Klassifikation, die beide Sonden und die ε-Kurve
verwenden; Lock-Übergänge (|resid| > 1000 Hz) vor dem Rauschen getrennt;
Tages-RMS um den Tages-Mittelwert über die Nicht-Lock-Proben; die Referenz-Metrik
der ε-Kurve zählt Tage mit Mode-2-Aufzeichnung ohne Nicht-Lock-Probe als
NaN-Zelle (Sortierung ans Ende, Median-Index `n/2`); die Zell-Metrik der
Pass-Segmentierung zählt nur Tage mit ≥ 1 Nicht-Lock-Probe. Datenkette:
`data/galileo_resid.bin` (GASR) + `data/ephemeris_galileo_daily.bin` +
`data/ephemeris_earth.bin`. Additive Sonde `galileo_mode2_dayrecon.rs`
(`cargo check` 0/0), Report auf stdout, keine Bestandsdatei geändert.

## n zuerst

Mode 2: 109 Tage mit Aufzeichnung, 107 mit ≥ 1 Nicht-Lock-Probe — 2 All-Lock-Tage
gesamt. Konjunktionsfenster ε < 30°: **77 Tage mit Aufzeichnung (die
ε-Kurven-Zählung), 75 mit Nicht-Lock-Proben (die Zell-Zählung), 2 All-Lock-Tage.**

## Messung 1 — die All-Lock-Tage (E1-Zählung)

Beide All-Lock-Tage liegen im ruhigen Fenster (Band 0) und tragen keine einzige
Nicht-Lock-Probe (0 geehrt: absent, kein Wert):

| Tag | ε | α | Mode-2-Aufzeichnungen | Nicht-Lock-Proben |
|---|---|---|---|---|
| 1997-01-07 | 10,2° | 167,8° | 10 650 | 0 |
| 1997-01-21 | 1,0° | 178,8° | 11 044 | 0 |

Die ε-Werte reproduzieren E1 exakt (10,2° / 1,0°). 77 − 75 = 2 = genau diese Tage;
die Referenz-Menge ist die Zell-Menge plus genau diese zwei Tage.

## Messung 2 — Mengen-Arithmetik beider Konventionen (E2-Test)

| Schnitt | n |
|---|---|
| ruhiges Fenster, Aufzeichnung (ε-Kurve/Referenz) | 77 |
| ruhiges Fenster, Nicht-Lock (Zell/Pass-Seg) | 75 |
| ruhig (Aufzeichnung) ∩ ε 30–60° (Aufzeichnung) | **0** |
| ε 30–60° (Nicht-Lock) ∩ ruhig (Nicht-Lock) | **0** |
| ruhig (Aufzeichnung) − ruhig (Nicht-Lock) | 2 (die All-Lock-Tage) |

Die ε-30–60°-Tage (8, alle mit Nicht-Lock-Proben, Median 16,701 Hz — die 16,70 Hz
der Pass-Segmentierung reproduziert) sind **disjunkt** von der ruhigen Menge unter
genau derselben Klassifikation, die beide Sonden und die ε-Kurve verwenden. Die
ε-Kurve hat sie nie in Band 0 gezählt (Band = `floor(ε/30)` = 1; die ε-Kurve
unterdrückt das Band nur wegen n = 8 < 10). Eine „Band-Kanten-Verschiebung"
existiert als Mechanismus nicht: kein ruhiger Tag liegt näher als 2,0° an der
30°-Kante (größtes ruhiges ε = 28,00°, 1997-02-25; kleinstes 30–60°-ε = 30,57°,
1990-12-11) — die Band-Mitgliedschaft jedes Tages ist in diesen Daten stabil, kein
Rand-Tag grenzempfindlich.

## Messung 3 — die Median-Arithmetik 77 → 75 → 0,65 Hz

Sortierte Nicht-Lock-Tages-RMS der 75 Zellen, Positionen um die Median-Lücke:

| Index | Tag | ε | Tages-RMS |
|---|---|---|---|
| 37 | 1996-01-05 | 12,7° | 0,647 Hz |
| 38 | 1996-01-03 | 11,1° | 1,455 Hz |

Referenz-Metrik (ε-Kurven-Konvention, 77 Einträge inkl. 2 NaN-All-Lock-Zellen ans
Ende sortiert): `n/2` = Index 38 → **1,455 Hz** (die ε-Kurve rundet auf 1,5 Hz).
Zell-Metrik (75 reale Zellen): `n/2` = Index 37 → **0,647 Hz**. Die zwei
NaN-Zellen am Listenende schieben den Median-Index von 37 auf 38; der Ausschluss
schiebt ihn zurück von 38 auf 37 — über die bimodale Lücke 0,647 → 1,455 Hz. Die
1,5 Hz der ε-Kurve sind **der erste Wert des oberen (lauten) Lobus** (Index 38 =
1996-01-03, 1,455 Hz), nicht ein Boden.

(E1 formuliert die Index-Richtung im Text mit vertauschten Indizes — „von s[37]
auf s[38]" bei genannten Werten 1,46 → 0,65 Hz; gemessen ist die Richtung 38 →
37, die Zahlenangabe 1,46 → 0,65 korrekt. Die Wirkung, nicht die Index-Beschriftung,
trägt.)

## Messung 4 — die lauten Tage und der obere Lobus

Die Verteilung der 75 Zellen ist bimodal: 38 Zellen ≤ 0,647 Hz (ruhiger Lobus,
0,023–0,647 Hz), darüber 37 Zellen (oberer Lobus, 1,455–293,373 Hz). 37 Tage sind
laut (> 1,0 Hz). Die zwei absoluten Maxima stehen bei der tiefsten Konjunktion:
1996-01-01 (293,373 Hz, ε 9,6°), 1997-01-22 (283,540 Hz, ε 1,7°) — beide ε < 10°,
beide mit reichlich Nicht-Lock-Proben (3468 / 2064). Von den 37 lauten Tagen haben
11 ε < 10°; die lauten Tage spannen ε 1,7°–25,7° — die tiefe Konjunktion trägt die
Extreme, nicht die gesamte laute Menge. Die lauten Tage sind der obere Lobus; sie
verschwinden beim Ausschluss der All-Lock-Tage nicht (sie bleiben in den 75), der
Median fällt unter sie, weil 38 ruhige Zellen die Hälfte der 75 tragen.

## Der Befund — die Reconciliation

**E1 (All-Lock-Tag-Zählung) hält arithmetisch exakt.** Die Referenz zählt 77
Einträge = 75 reale Zellen + 2 All-Lock-Tage (1997-01-07 ε 10,2°; 1997-01-21
ε 1,0°, keine Nicht-Lock-Probe). Ihr Ausschluss ergibt exakt die Zell-Menge der
Pass-Segmentierung (75); der Median fällt über den Index-Schritt 38 → 37 von
1,455 Hz auf 0,647 Hz über der bimodalen Lücke. 77 → 75 → 0,65 Hz schließt.

**E2 (acht laute Tage in ε 30–60° verschoben) trägt als Zähl-Deutung nicht.** Die
acht ε-30–60°-Tage (Median 16,701 Hz) sind unter der in beiden Sonden und der
ε-Kurve identischen Klassifikation disjunkt von der ruhigen Menge (Schnitt 0); sie
waren nie in den 77. Ein ε-Band-Kanten-Unterschied zwischen den Konventionen ist
nicht messbar (größtes ruhiges ε 28,00°, kleinstes 30–60°-ε 30,57° — kein Tag in
der Nähe der Kante). E2s Beobachtung des 30–60°-Fensters ist echt, aber
fenster-extern: es erklärt die 1,5→0,65-Differenz nicht.

**Verdikt: E1 (All-Lock-Tag-Zählung + Median-Index-Effekt) ist die gemessene
Reconciliation; E2s Laut-Tag-Verschiebung ist widerlegt.** Beide Vorgänger messen
dieselben Tag-Mengen (75 Zellen, 0,647 Hz) — die Differenz ist ausschließlich die
Zähl-Konvention der zwei All-Lock-Tage in der ε-Kurven-Referenz, die den
oberen-Mittleren über eine Wertelücke auf den ersten lauten Tag (1996-01-03,
1,455 Hz) legt.

## Grenzen

- Die zwei All-Lock-Tage sind Aufzeichnungs-Tage mit 0 Nicht-Lock-Proben
  (vollständig im Lock) — als Zellen absent, in der ε-Kurven-Referenz als NaN
  gezählt; beide Zahlen gemessen, nicht geglättet.
- Warum die lautesten Tage (ε < 10°, Konjunktions-Tiefe) so laut sind, ist hier
  nicht gedeutet (Stationen-/Pass-Mix: siehe Station-Split und Pass-Segmentierung).
- Mode 3 (ε-Kurve 4,9 Hz / 60 Tage): All-Lock-Tag-Struktur im ruhigen Fenster
  nicht gegengezählt — `pending`.
- ε einmal je Tag (TDB-Tagesanfang), keine Intra-Tag-Geometrie; die 30°-Kante ist
  in diesen Daten unbesetzt (kein Rand-Tag), die Aussage zur Band-Stabilität gilt
  für diesen Tag-Satz.

## Register-Satz

*Die Tagesmengen-Reconciliation des Mode-2-Konjunktionsfensters ist gemessen: die
ε-Kurven-Zählung 77 = 75 reale Zellen + 2 All-Lock-Tage (1997-01-07 ε 10,2°;
1997-01-21 ε 1,0°, keine Nicht-Lock-Probe); ihr Ausschluss ergibt exakt die
Zell-Menge der Pass-Segmentierung, und der Median fällt über den Index-Schritt
38 → 37 von 1,455 Hz auf 0,647 Hz über der bimodalen Lücke (E1 hält). Die
ε-30–60°-Tage (8, Median 16,701 Hz) sind unter der identischen Klassifikation
disjunkt von der ruhigen Menge — E2s Laut-Tag-Verschiebung erklärt die Differenz
nicht. Die 1,5 Hz sind der erste Wert des oberen Lobus (1996-01-03, 1,455 Hz),
kein Boden.*

## Status

`done`. Additive Sonde `galileo_mode2_dayrecon` (cargo check 0/0), Lauf auf stdout.
E1 (All-Lock-Tag-Zählung) ist die gemessene Reconciliation; E2 (Laut-Tag-Verschiebung)
widerlegt. Mode-3-All-Lock-Tag-Gegenzählung im ruhigen Fenster `pending`.
