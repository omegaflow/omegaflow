<!--
  title: Befund — Galileo In-Pass-Stärke-Rampe: der AGC-Boden ist an Station 43/63 eine Boden↔Rauschen-Kovarianz bei Pass-Identität, an Station 14 nicht
  class: befund
  date: 2026-09-05
  sha256: 3a2ce02e44f47d809d9800eab8ed1f8f6a3fe7cf41d7142e676768b225933b5b
  status: done
  antwortet-auf: docs/befund/befund-galileo-pass-segmentierung.md docs/befund/befund-galileo-te-staerke-floor.md
  see-also: docs/befund/befund-galileo-mode1-fingerabdruck.md
-->
# Befund: Galileo In-Pass-Stärke-Rampe — der AGC-Boden ist an Station 43/63 ein echter In-Pass-SNR-Term, an Station 14 nicht (Pass-Identität)

## Frage & Bindung

Die Tages-Achsen-TE (`befund-galileo-te-staerke-floor`) und die Pass-Segmentierung
(`befund-galileo-pass-segmentierung`) liessen die Pass-Wahrheit offen: die Klassifikation eines
Passes über seinen Pass-Median der Signalstärke mischt Stärke-Zustände innerhalb des Passes. Die
ausstehende Messung ist die Stärke-Rampe *bei Pass-Identität*: zerlege einen Pass in durchgehende
Unter-Arcs einheitlicher Stärke (Stärke-Zustand), dann prüfe, ob das Rauschen ansteigt, wenn die
Stärke *innerhalb desselben Passes* auf den AGC-Boden (−2560) fällt. Pass-Identität hält Epoche
und Station exakt konstant; die Geometrie ist pass-gleich, nicht punktgleich (Boden- und
Plateau-Unter-Arcs sitzen im selben Pass, aber nicht auf identischer Elevation) — der stärkste
verfügbare Kontroll-Test gegen die Epochen-/Stations-Kollokation.

Gebunden: Pass = durchgehender Tracking-Arc je (Station, Modus), Pass-Grenze = Zeitlücke zwischen
aufeinanderfolgenden Proben > 600 s (Schwelle im gemessenen dt-Tal, wie im Pass-Blatt); Lock-
Übergänge (|resid| > 1000 Hz) vor dem Rauschen getrennt. Stärke-Zustand je Probe: Boden =
`signal_strength` ≤ −2560 (AGC-Klemmwert), Plateau = ≥ −1900, Werte dazwischen und 0 als
Übergang/Pad ausgeschlossen. Unter-Arc = durchgehender Lauf gleichen Zustands, getrennt bei
Zustandswechsel, bei > 120-s-Lücke zwischen Nicht-Lock-Proben oder bei 60 Proben (lokale
Detrend-Skala); Rauschen = Resid-RMS um den Unter-Arc-Mittelwert, je Zustand innerhalb des Passes
gepoolt; ein Unter-Arc zählt ab 30 Nicht-Lock-Proben. Ein Pass ist *dual*, wenn sein Boden-Pool
und sein Plateau-Pool beide ≥ 30 Proben halten. Kontroll-Variante *interior*: nur Unter-Arcs, die
ganz innerhalb `Pass-Beginn + 120 s` .. `Pass-Ende − 120 s` liegen (Akquisitions-Transienten-
Kontrolle). Stationen 14/43/63, Moden 1 und 2 (Modus 3 nicht vermessen). Datenkette:
`data/galileo_resid.bin` (GASR). Neue additive Sonde `tools/measure/src/bin/galileo_pass_strength_ramp.rs`,
`cargo check` 0/0 Warnings; Report auf stdout, keine Report-Datei.

## n zuerst (0 geehrt)

Modus 1 an 14/43/63: 9 567 949 Proben, 1 413 861 Lock-Übergänge. Modus 2: 2 899 000 Proben,
157 052 Lock-Übergänge. (Die Trio-Anteile der Gesamtdatei; Stations 12/24/34/42/45/61 sind klein
und nicht vermessen.)

Pass-Struktur (dual = Boden-Pool ≥ 30 und Plateau-Pool ≥ 30):

| St | Mode | Pässe | Boden vorhanden | Plateau vorhanden | dual voll | dual interior | nur Boden | nur Plateau | keins |
|---|---|---|---|---|---|---|---|---|---|
| 14 | 1 | 155 | 73 | 96 | 21 | 20 | 49 | 72 | 10 |
| 43 | 1 | 149 | 65 | 92 | 25 | 24 | 39 | 66 | 18 |
| 63 | 1 | 153 | 65 | 88 | 13 | 13 | 50 | 73 | 15 |
| 14 | 2 | 59 | 41 | 23 | 9 | 6 | 32 | 14 | 4 |
| 43 | 2 | 60 | 37 | 26 | 8 | 5 | 29 | 18 | 5 |
| 63 | 2 | 68 | 42 | 31 | 7 | 4 | 33 | 22 | 4 |

Ein Viertel bis ein Drittel der Mode-1-Pässe an jeder Station trägt *beide* Zustände als ≥-30-Proben-
Pools: n(dual) = 59 (Mode 1) und 24 (Mode 2) über die drei Stationen. Der Test trägt.

## Tabelle 1 — gepaartes In-Pass-Rauschen, Modus 1 (Boden-Unter-Arc vs Plateau-Unter-Arc desselben Passes)

Voll (alle Unter-Arcs):

| Station | n | med Boden RMS Hz | med Plateau RMS Hz | med Diff Hz | mean Diff | Boden>Plateau | med Ratio |
|---|---|---|---|---|---|---|---|
| 14 | 21 | 0,777 | 0,869 | −0,004 | 10,60 | 10 / 11 | 0,94 |
| 43 | 25 | 2,080 | 0,092 | 1,828 | 72,51 | 20 / 5 | 9,69 |
| 63 | 13 | 18,902 | 0,471 | 18,827 | 65,86 | 9 / 4 | 35,30 |
| gepoolt | 59 | — | — | 1,147 | 49,01 | 39 / 20 | 6,67 |

Gepoolte Diff-Verteilung: p10 −18,34; p25 −0,09; p50 +1,15; p75 +59,53; p90 +226,00 Hz. Schwere
Schwänze in beide Richtungen; der Median ist positiv (Boden lauter).

Interior (Akquisitions-Transienten ausgeschlossen, 120-s-Rand):

| Station | n | med Boden RMS Hz | med Plateau RMS Hz | med Diff Hz | mean Diff | Boden>Plateau | med Ratio |
|---|---|---|---|---|---|---|---|
| 14 | 20 | 6,458 | 1,847 | −0,005 | 11,12 | 9 / 11 | 0,94 |
| 43 | 24 | 1,258 | 0,092 | 1,162 | 59,15 | 19 / 5 | 10,57 |
| 63 | 13 | 4,784 | 0,472 | 4,757 | 64,47 | 9 / 4 | 11,19 |
| gepoolt | 57 | — | — | 0,760 | 43,51 | 37 / 20 | 6,54 |

## Tabelle 2 — gepaartes In-Pass-Rauschen, Modus 2

Voll:

| Station | n | med Boden RMS Hz | med Plateau RMS Hz | med Diff Hz | mean Diff | Boden>Plateau | med Ratio |
|---|---|---|---|---|---|---|---|
| 14 | 9 | 2,555 | 0,152 | 0,683 | 3,85 | 5 / 4 | 2,82 |
| 43 | 8 | 2,620 | 0,107 | 2,174 | 8,44 | 8 / 0 | 21,31 |
| 63 | 7 | 1,792 | 0,090 | 0,951 | 11,83 | 6 / 1 | 9,75 |
| gepoolt | 24 | — | — | 2,009 | 7,71 | 19 / 5 | 9,92 |

Interior: n 15 (6/5/4), gepoolter med Diff 0,319, Boden>Plateau 9/6, med Ratio 2,05; Station 43
4/1 (med Diff 1,902), Station 63 3/1 (med Diff 36,2), Station 14 2/4 (med Diff −0,03) — das
interior-n (15) ist dünn.

## Der Befund

**1. Bei Pass-Identität steigt das Rauschen an Station 43 und 63, wenn die Stärke auf den
AGC-Boden fällt — eine genuine Boden↔Rauschen-Kovarianz (simultan; Pfeilrichtung nicht gemessen),
nicht Epochen-/Stations-kollokiert.** Station 43: Boden-Unter-Arcs 2,08 Hz (voll) bzw.
1,26 Hz (interior) gegen Plateau-Unter-Arcs 0,092 Hz desselben Passes — Faktor ~10–22, in 20 von
25 Pässen (interior 19/24). Station 63: 18,9 Hz (voll) bzw. 4,8 Hz (interior) gegen 0,47 Hz,
Faktor ~11–35, in 9 von 13 Pässen. Epoche und Station sind über die Pass-Identität konstant; die
Differenz entsteht innerhalb desselben Passes zwischen Stärke-Zuständen. Das ist der gemessene
In-Pass-Gradient, den die Tages-Achse und die Pass-Median-Achse nicht auflösen konnten.

**2. An Station 14 ist der AGC-Boden bei Pass-Identität kein SNR-Term.** Station 14: med Diff
−0,004 Hz (voll) bzw. −0,005 Hz (interior), 10/11 bzw. 9/11 Pässe — das Boden-Rauschen ist nicht
vom Plateau-Rauschen desselben Passes unterscheidbar. Die Boden-Zustände von Station 14 liegen
überwiegend in den lauten Epochen (1995-11..12, 1996-12..1997-02); dort ist auch das Plateau
desselben Passes angehoben (med Plateau 0,87–1,85 Hz gegen 0,09 Hz an 43/63). Das Station-14-Bild
des Fingerabdrucks (Boden lauter) ist damit Epochen-Kollokation, kein Stärke-Effekt — konsistent
mit dem TE-Blatt.

**3. Die gemessene In-Pass-Kovarianz ist an 43/63 stark, an 14 abwesend — der AGC-Boden ist kein
universeller PLL-Treiber.** Gepoolt über alle drei Stationen: 39/59 Pässe (Mode 1), 19/24 (Mode 2)
mit Boden lauter; med Ratio 6,7 bzw. 9,9. Die Richtung wird aber von 43/63 getragen; Station 14
ist gegengerichtet flach (Mode 1 med Diff ≈ 0, 10/11; Mode 2 st14 voll trägt eine schwache
Boden-Lautheit 0,683 Hz, 5/4, die die interior-Kontrolle auflöst, 2/4, −0,03 Hz). Der Boden ist an
zwei Stationen eine Boden↔Rauschen-Kovarianz, an einer Station ein Epochen-Marker. Die
Stations-Anomalie bleibt bestehen und ist jetzt als *Richtungs*-Split benannt.

## Grenzen

- Rausch-Metrik = Resid-RMS um den Unter-Arc-Mittelwert (60-Proben-Detrend); sie misst die
  hochfrequente Streuung, nicht den Tages-/Pass-RMS der Vorgänger. Absolute Werte sind daher nicht
  1:1 mit dem Pass-Blatt vergleichbar; die *Differenz bei Pass-Identität* ist die Aussage.
- Schwere Schwänze (Einzel-Pässe mit Boden-RMS bis ~360 Hz, nahe der Lock-Schwelle) tragen die
  Mittelwerte; Mediane und Vorzeichen-Zählungen sind die tragenden Grössen. Pässe mit sehr kurzem
  seltenem Zustand (Boden- oder Plateau-n ~ 30–100) tragen instabile RMS-Schätzer.
- Interior-Kontrolle entfernt den 120-s-Rand; sie bestätigt 43/63, entkräftet 14 nicht.
- Stärke-Skala unkalibriert; Plateau-Label ab −1900 kann einzelne Zwischenwerte enthalten
  (gemessene Plateau-Mediane 0,09–0,87 Hz bleiben sauber). s = 0 (Pad) und Lock-Proben sind
  ausgeschlossen.
- Modus 3, die kleinen Stationen und eine detrendete Metrik mit kürzerer Skala bleiben pending.
- Boden-Unter-Arcs können auf anderer Elevation sitzen als Plateau-Unter-Arcs desselben Passes
  (Geometrie-Konfundierung innerhalb des Passes — pass-gleich, nicht punktgleich); die
  Elevations-Verteilung der Zustände ist nicht kontrolliert, pending.
- Ein Richtungs-Test (führt der Boden das Rauschen zeitlich innerhalb des Passes an?) ist mit
  dieser Messung nicht geführt — sie misst die Zustands-Kovarianz bei Pass-Identität (simultan),
  nicht die zeitliche Pfeilrichtung.

## Register-Satz

*Bei Pass-Identität ist der AGC-Boden (−2560) an Station 43 und 63 eine genuine
Boden↔Rauschen-Kovarianz (nicht Epochen-/Stations-kollokiert; Boden-Unter-Arcs 1,3–18,9 Hz gegen
Plateau-Unter-Arcs 0,09–0,47 Hz desselben Passes, Faktor ~10–35, in 20/25 bzw. 9/13 Pässen,
interior-Kontrolle bestätigt) und an Station 14 keine (med Diff ≈ 0, 10/21 Pässe; das
Station-14-Boden-Bild ist Epochen-Kollokation) — der Stärke-Gradient des Fingerabdrucks ist damit
an 43/63 als In-Pass-Kovarianz gemessen und an 14 als Epochen-Artefakt getrennt; gepoolt 39/59
Pässe (Mode 1) und 19/24 (Mode 2) mit Boden lauter. Offen (pending): die zeitliche Pfeilrichtung
innerhalb des Passes (diese Messung misst simultane Zustands-Kovarianz), die Geometrie- innerhalb-
Pass (Elevation der Unter-Arcs), die Stations-Asymmetrie (43/63 ja, 14 nein), Modus 3, kleine
Stationen, eine detrendete Metrik mit kürzerer Skala.*

## Status

`draft`. Sonde `galileo_pass_strength_ramp.rs` additiv, `cargo check` 0/0; Report auf stdout.
