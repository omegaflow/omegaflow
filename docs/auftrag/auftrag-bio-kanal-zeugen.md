<!--
  title: Auftrag — Biosignatur-Kanäle: O2/O3, Rotkante, saisonal + Bio-Zeugen lesen
  class: auftrag
  date: 2026-09-05
  status: pending
  sha256: 3cae8a2b85c6dfe40670e2ca158477a866cf43ad689bf81a2399803a0dd2c8c1
  see-also: docs/paper/jwst-disequilibrium-survey.md docs/befund/befund-negativ-fuzzy-techno.md
-->

# Auftrag: die übrigen Biosignatur-Kanäle + die fehlenden Bio-Zeugen

## Zweck

Die Bio-Seite symmetrisch zur Techno-Seite schließen. Gebaut ist bisher nur der
Disequilibrium-Kanal. Offen sind (a) die übrigen Bio-Kanäle — O2/O3-Abundanz,
Vegetations-Rotkante, saisonale/zeitliche Signaturen — und (b) die im
Fuzzy-Befund benannten fehlenden Bio-Zeugen: Stellar-Aktivität/XUV (der
Photochemie-Treiber der SO2/CO2-Hits) und Reservoir [Fe/H]/C/O. Ohne (b) sind
die individuellen Disequilibrium-Hits „Rest über dem Gleichgewicht", aber nicht
„Rest über Gleichgewicht und Aktivität".

## Umfang

- (b) zuerst: die in `pscomppars` vorhandenen, aber ungelesenen Zeugen
  (st_met/[Fe/H], ggf. Aktivitäts-/XUV-Proxy) in den disequilibrium_register_probe
  einlesen und gegen die Hits regressieren — der zweite Reinigungsschritt.
- (a) danach: O2/O3-Abundanz und Rotkante als Kanäle abgrenzen (welche Spektral-
  Merkmale, welche Daten); saisonal = Zeitreihen, als eigener pending-Zweig.

## Kernregel (0 honored)

Zeugen erst lesen, dann ausschließen; wo ein Zeuge fehlt, bleibt der Hit
`pending`, nie „unabhängig". Absenz gegen Instrument-Floor führen, nicht gegen
Modell-Floor.

## Lieferung

Zeugen eingelesen + Reinigungsschritt im Probe, committed; Kanal-Abgrenzung
(O2/O3, Rotkante, saisonal) als Befund.
