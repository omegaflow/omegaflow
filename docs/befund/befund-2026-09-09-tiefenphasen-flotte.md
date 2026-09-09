<!--
  title: Befund — die Flotte (16 Ereignisse × Stationen): der Flotten-Mittelwert ist unverzerrt (+1,7 km gegen den Katalog, se 4,7 km), aber die Streuung (19 km über Ereignisse, 36 km über Stationen) dominiert das ±10-km-Gate — der Pilot-Offset +19 km war Streuung, kein Bias
  class: befund
  date: 2026-09-09
  sha256: efa4e928e2806c12ee3507c7009c7cb00293e4c7d3ce90fa7d730d7b3eb8a9cd
  status: done
  see-also: docs/handover/handover-2026-09-09-tiefenphasen-flotte.md docs/befund/befund-2026-09-09-tiefenphasen-feld.md docs/befund/befund-2026-09-09-tiefenphasen-inversionsklasse.md docs/befund/befund-2026-09-09-tiefenphasen-polaritaet.md docs/concepts/die-akteure-im-boden-und-wasser.md
-->

# Befund: die Flotte (16 Ereignisse × Stationen) — der Mittelwert ist unverzerrt, die Streuung dominiert

## Frage & Bindung

Der Feldpilot trug ein Ereignis, nicht die Statistik. Der benannte Nachfolger:
die Flotte — Ereignisse × Stationen, σ und √N — damit aus dem Einzelbefund
eine Messreihe wird. Auswahlregel unverändert aus dem Piloten (Hindu-Kush-Box,
Tiefe ≥ 35 km, M ≥ 6, Land-Epizentrum, GBCO-Zeuge je Ereignis, Stationsband
30–90°, SNR-Gate ≥ 3), registriert vor dem ersten Fetch.

## Was gebaut wurde

`tools/measure/src/bin/depth_phase_fleet_probe.rs` — die Ereignis-Schleife über
die gemeinsame Lib (`depthphase.rs`, 1-km-Inversionsklasse aus
`befund-2026-09-09-tiefenphasen-inversionsklasse.md`): je Ereignis Katalog →
GBCO-Zeuge → Stations-Band → BHZ-Fetch → P-Pick → SNR-Gate → pP/sP-Korrelation
→ 1D-Inversion; je Ereignis Median + σ über die Stationen, über die Flotte
Mittelwert ± σ/√N. `mean`/`sample_sd` neu in `stats.rs`.

Nebenbefund (Robustheit): ein BHZ-Körper eines Ereignisses (usp000crt6) war
kein MiniSEED — der Decoder las `data_offset`/Blockette-Offsets ohne
Grenzprüfung und panickte. Fix: `record_meta` prüft die Offsets gegen die
Record-Länge (Fehlkörper → absent, nie Panik; 2 Tests).

## Die Messung

26 registrierte Tiefereignisse in der Box; die 16 größten gemessen (M6.2–7.5,
Katalog-Tiefe 107,7–231,0 km). Je Ereignis 4–11 Stationen über dem SNR-Gate.

Per-Ereignis-Offsets (Median − Katalog) in km:

`[+11, −3, +19, −18, +33, −13, +5, +14, −27, +25, −14, +25, −7, −11, −25, +12]`

| Größe | Wert |
|---|---|
| N (Ereignisse mit invertierter Tiefe) | 16 |
| Mittelwert Offset | +1,7 km |
| σ über die Ereignisse | 19,0 km |
| se = σ/√N | 4,7 km |
| typische Stations-Streuung (Median je Ereignis-σ) | 36,1 km |

## Die Befunde (benannt, nicht geglättet)

1. **Der Flotten-Mittelwert ist unverzerrt.** +1,7 km gegen den Katalog, mit
   se 4,7 km verträglich mit Null. Der Pilot-Offset +19 km (us10003re5, der
   dritte Wert der Reihe) war Streuung, kein systematischer Bias — das
   Einzelbefund-Bild kippt, sobald die Reihe steht.

2. **Die Streuung dominiert das ±10-km-Gate.** 19 km über die Ereignisse,
   36 km über die Stationen — beides weit über dem Gate und über dem
   Pick-Streu-Budget (3,2 km). Der Engpass ist die Korrelation, nicht das
   Modell: die pP-Korrelation verhakt sich an Coda-Merkmalen (und dem
   mehrdeutigen pP-Zweig bei Δ≈30°, Befund `tiefenphasen-polaritaet`), wie
   schon IC.XAN im Piloten pP und sP am selben Lag trug.

3. **√N trägt, aber langsam.** se = 19/√16 = 4,7 km; ein weiteres √N-Viertel
   auf ~2,4 km bräuchte N ≈ 64 — die Streuung zu senken (besseres Picken)
   ist der billigere Hebel als mehr Ereignisse.

## Schließung

Der Nachfolger „die Flotte" schließt als gebaut und gemessen: die Flotte ist
jetzt eine Messreihe (16 Ereignisse, unverzerrt, se 4,7 km), kein Einzelbefund.
Offen bleiben: die Streuung (besseres Picken / den mehrdeutigen pP-Zweig
behandeln), der Quell-Strahlungsterm (CMT), und vor dem Zonen-Lauf die
ak135-Tiefenmodell-Erweiterung über 250 km.
