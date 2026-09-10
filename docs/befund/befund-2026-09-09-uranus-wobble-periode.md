<!--
  title: Befund — Uranus-Wobble-Periode: die Wobble ist km-groß (24.77 km Mittel, 44.27 km max) und trägt die Uranus-Mond-Umläufe (Oberon 13.46 d dominant, Titania 8.71 d, Umbriel 4.14 d, Ariel 2.52 d) — nicht primär 1,4 d
  class: befund
  date: 2026-09-09
  sha256: aa87841e39d40e5642d8e979dfb9b40daa18b5ff4d97930ce21766b6381c46c0
  status: done
  see-also: docs/befund/befund-2026-09-09-uranus-zentrum-kopplung.md docs/befund/befund-2026-09-09-uranus-zentrum-versionenstruktur.md docs/handover/archiv/handover-2026-09-09-mechanische-reste.md
-->

# Befund: Uranus-Wobble-Periode

## Frage & Bindung

Die Übergabe (handover-2026-09-09-uranus-zentrum-kopplung.md) und der
Zentrum-Kopplung-Befund stellten die offene Zeile: die Wobble-Periode
(„1,4-d-Mond-Signatur") als physikalische Form der Wobble separat zu prüfen —
`pending`. Probe: `tools/measure/src/bin/uranus_wobble_period_probe.rs`.

## Das Instrument

Der Probe tastet die Wobble (Planetenzentrum 799 − System-Baryzentrum 7, DE441)
auf einem regulären Gitter ab (365 d, Schritt 0.05 d, 7300 Epochen) und mißt
die Perioden mit einem Least-Squares-Periodogramm (Frequenzgitter, parabolische
Peak-Verfeinerung).

## Die Messung

Wobble-Betrag: **Mittel 24.77 km, max 44.27 km** — physikalisch korrekt für die
Uranus-Mond-Baryzentrums-Wobble (die Monde heben das System-Baryzentrum um
~10–45 km aus dem Planetenmittelpunkt). Dominante Perioden der Komponenten:

| Komponente | Oberon | Titania | Umbriel | Ariel |
|---|---|---|---|---|
| x | 13.463 d / 20.3 km | 8.707 d / 16.8 km | 4.144 d / 3.9 km | — |
| y | 13.458 d / 6.9 km | 8.706 d / 5.8 km | 4.143 d / 1.3 km | 2.521 d / 0.9 km |
| z | 13.460 d / 20.0 km | 8.708 d / 16.6 km | 4.143 d / 3.8 km | 2.520 d / 2.7 km |

Der Betrag |c−b| trägt die Schwebungs-Perioden der Monde (24.6 d =
Oberon−Titania, 3.10 d = Oberon−Umbriel, 5.98 d …), keine eigenen Perioden —
der Betrag ist positiv-definit, die physikalische Form lebt in den Komponenten.

## Zwei Korrekturen an der Vorlage

1. **Die dominante Form ist nicht 1,4 d.** Die vier äußeren Monde tragen die
   Wobble; Miranda (1.413 d) trägt ~0.1 km — unter der Meldeschwelle, kein
   1,4-d-Peak erscheint. Die „1,4-d-Signatur" der Vorlage war ein pars pro toto
   für die Mond-Wobble insgesamt; die gemessene Form ist 13.5/8.7/4.1/2.5 d.
2. **Die Wobble ist km-groß, nicht m.** Der Zentrum-Kopplung-Befund notiert
   „Mittel 21.653 m, max 42.627 m"; dieser Probe mißt 24.77 km / 44.27 km. Der
   Unterschied ist 1000× — die Vorlage trägt die Einheit km als „m" (21.653 km
   über 1992–2011 gegen 24.77 km über J2000+365 d sind stichproben-konsistent).
   Die Korrektur gehört ins Register (TODO), nicht stillschweigend übernommen.

## Kalibrier-Gate

Synthetische Reihe injiziert (7.7 d / 5000 m, eine Linie): zurückgewonnen
**7.700 d / 4999 m** — exakt (0.02 %). Das Periodogramm trägt injizierte
Perioden und Amplituden zurück.

## Verdict

Die offene Zeile ist geschlossen: die Wobble-Periode ist gemessen — die
Uranus-Mond-Umläufe, dominiert von Oberon (13.46 d) und Titania (8.71 d); die
Wobble trägt die Mond-Massen auf km-Skala. Die „1,4-d-Signatur" war die
schwächste, nicht die dominante, Komponente.

## Register-Zeilen

- Der physikalische Ursprung der vier de441-Trägerjahre (1999/2001/2004/2009) —
  `pending`.
- (c) Neptun als zweiter Planet desselben Baus — `pending`.
