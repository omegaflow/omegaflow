<!--
  title: Befund — Tōhoku 2011 Pegel: der Tsunami ist in sechs NOAA-Pegeln gemessen — die Ankunfts-Staffelung trägt ~750–800 km/h (√(g·d)), der erste Pegel-Zwirn des Hauses
  class: befund
  date: 2026-09-09
  sha256: cff564973d6002b1c58d28f81a635f0a0681b4f1d6fe32ce6ddac3fcced16b1d
  status: done
  see-also: docs/handover/handover-thematisch-tiefenphasen-flotte.md docs/befund/befund-2026-09-09-tohoku-gsn-geblockt.md
-->

# Befund: der Tōhoku-Pegel (NOAA CO-OPS)

Feld-Herkunft: √(g·d) ist die klassische Langwellen-Geschwindigkeit (Airy/Murty);
die Pegel-Staffelung ist das Tide-Gauge-Verfahren der Warnzentren.

## Frage & Bindung

TODO-Registerzeile „Tōhoku als Tsunami-Positivkontrolle": der Pegel-Schenkel
der Kette. Die seismische Ortung ist geblockt (Picker); die Pegel-Quelle
(`api.tidesandcurrents.noaa.gov`) ist registriert und liefert historische
Wasserstände. Probe: `tools/measure/src/bin/tohoku_pegsel_probe.rs` — holt
`product=water_level` und `product=predictions` für 2011-03-11, bildet die
Anomalie (gemessen − Gezeitenvorhersage) und liest den Einsatz.

## Die Messung

Sechs Pazifik-Pegel, 2011-03-11 (Beben 05:46 UTC). Einsatz = erstes
|Anomalie| > 0,3 m nach dem Beben; pre_floor = größtes |Anomalie| vor dem
Beben (die Gezeiten-Residuum-Decke).

| Pegel | Dist [km] | pre_floor [m] | Einsatz [UTC] | Spitze [m] |
|---|---|---|---|---|
| Adak Island | 3495 | 0,05 | 10:36 | 0,98 |
| Midway Island | 3874 | 0,08 | 10:36 | 0,72 |
| Kwajalein | 4153 | 0,22 | 11:06 | 0,62 |
| Honolulu | 5961 | 0,04 | 13:30 | 0,66 |
| Hilo | 6301 | 0,03 | 13:54 | 0,81 |
| Crescent City | 7542 | 0,15 | 15:48 | 2,21 |

Die Anomalie ist sauber: pre_floor 0,03–0,22 m gegen Signal 0,62–2,21 m.

## Die Staffelung — die Ankunfts-Zeiten tragen die Physik

Laufzeit = Einsatz − 05:46; Geschwindigkeit = Dist / Laufzeit:

| Pegel | Laufzeit [min] | v [km/h] |
|---|---|---|
| Adak | 290 | 723 |
| Midway | 290 | 801 |
| Kwajalein | 320 | 779 |
| Honolulu | 464 | 771 |
| Hilo | 488 | 775 |
| Crescent City | 602 | 752 |

Mittel ≈ **766 km/h** — die Flachwasser-Geschwindigkeit √(g·d): d ≈ 4700 m
gibt √(9,81·4700) ≈ 774 km/h. Die gemessene Staffelung trägt das
Flachwasser-Gesetz; der Tsunami ist das, was die Pegel sagen, nicht was
angenommen wurde.

## Verdikt

Der Pegel-Schenkel der Tōhoku-Kette steht: die Maschine hat den Tsunami in
sechs historischen NOAA-Pegeln gemessen, die Ankunfts-Staffelung liest die
~766-km/h-Flachwassergeschwindigkeit, und die Amplitude (bis 2,21 m in
Crescent City) stimmt mit dem historischen Befund. Das ist der erste
Pegel-Zwirn des Hauses — eine gemessene Staffelung, die die Physik begrüßt.

Die Kette ist noch nicht geschlossen: die seismische Ortung (der Picker) und
die Ankündigungs-Rechnung (GEBCO-Tiefenkarte) bleiben pending. Was jetzt
gemessen ist, ist die Antwort-Seite der Kette — die echten Pegel-Ausschläge,
gegen die eine künftige Vorhersage gelegt wird.

## Folge (Register)

- Pegel-Zeile: der Tsunami-Pegel-Abruf ist gebaut; die Ankunfts-Staffelung
  ist gemessen (766 km/h, √(g·d)).
- Offen (unverändert): M9.1-Picker, GEBCO, die volle Kette (orten → ankündigen
  → gegen diese Pegel legen).
