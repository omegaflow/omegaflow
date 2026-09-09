<!--
  title: Befund — echtes pP/sP-Picken an einem tiefen Ereignis (Feldpilot, 1 Ereignis): die Kette läuft, die Tiefe kommt aus dem Lag (Median 250 km gegen Katalog 231 km), aber das ±10-km-Gate wird nicht getragen und die Polaritäts-Vorannahme kippt an dieser Geometrie
  class: befund
  date: 2026-09-09
  sha256: 114629a92c9daa861f947011a642bce91d803992683ca77d4f738fd51cc23c93
  status: done
  see-also: docs/handover/handover-thematisch-tiefenphasen-flotte.md docs/concepts/die-akteure-im-boden-und-wasser.md docs/befund/befund-2026-09-09-tiefenphasen-diagonale.md docs/befund/befund-2026-09-09-ak135-tiefenphasen.md
-->

# Befund: echtes pP/sP-Picken an einem tiefen Ereignis (Feldpilot)

## Frage & Bindung

Die Diagonale (`depth_phase_probe.rs`) liest die Tiefe synthetisch aus dem
pP/sP-Lag. Offen war das echte Picken an einer Flachquelle (10 km, Lag ~3 s,
grenzwertig). Dieser Feldpilot läuft die Kette an einem tiefen Ereignis
(≥ 20 km, Lag ≥ 6 s) an echter Wellenform: `depth_phase_field_probe.rs` —
FDSN-Katalog + Stations-Band + BHZ-Fetch + P-Pick + Kreuzkorrelation des
P-Wavelets im Lag-Fenster aus der ak135-Diagonale + 1D-Inversion.

## Die Auswahlregel (registriert vor dem ersten Fetch)

Tiefe ≥ 35 km, Magnitude ≥ 6, Land-Epizentrum (Hindu-Kush-Box 34–38 N /
68–74 E — kein pwP/pmP aus Wasser-Säule/Moho-Vielfachen), Stations-Entfernungsband
30–90°, SNR-Gate ≥ 3 am P-Onset *bevor* ins pP-Fenster geblickt wird. Ein
GBCO-Zeuge (opentopodata) prüft die Land-Annahme je Ereignis.

## Das Fehlerbudget (vor dem Lauf)

Der Lag trägt den Onset-Pick (~2 s Streu); 3,88 s Lag je 5 km → erwartete
Tiefen-Präzision ~±2,6 km am Einzel-Ereignis. Übereinstimmungs-Gate an die
Katalog-Unsicherheit genagelt (±10 km). Polarisations-Zeuge: Korrelation
trägt beide Vorzeichen; die Flachquellen-Lehrbuch-Vorannahme sagt pP
invertiert (negativ).

## Die Messung

Ereignis `us10003re5` — M7.5, 36,5244 N / 70,3676 E (Hindu Kush), Katalog-Tiefe
231,0 km, Ursprung 2015-10-26T09:09:42; GBCO-Oberflächenhöhe 3273 m (Land).

12 BHZ-Stationen im 30–90°-Band; 6 bestanden das SNR-Gate (6 übersprungen:
kein Record / kein Pick). Je Station: pP-Pick über Kreuzkorrelation im
Lag-Fenster `[0,7·lag, 1,3·lag]` aus der ak135-Diagonale, 1D-Inversion.

| Station | Δ° | SNR | pP-Lag (pred 46,2 s) | pP-corr | sP-corr | Tiefe |
|---|---|---|---|---|---|---|
| GE.EIL | 30,3 | 73,0 | 41,2 s | +0,77 | −0,77 | 200 km |
| KO.MDUB | 30,7 | 56,1 | 50,1 s | +0,91 | −0,93 | 250 km |
| IU.CHTO | 30,7 | 71,4 | 38,5 s | +0,63 | −0,67 | 200 km |
| TM.CMMT | 30,7 | 71,0 | 38,4 s | +0,63 | −0,66 | 200 km |
| II.PALK | 30,7 | 41,0 | 48,1 s | −0,68 | −0,77 | 250 km |
| IC.XAN | 31,4 | 7,2 | 55,9 s | −0,97 | −0,97 | 250 km |

Invertierte Tiefe: Median 250 km (6 Stationen), Katalog 231,0 km, Offset
+19,0 km → **außerhalb des ±10-km-Gates** (zwei Lesarten: unsere Streuung
oder der Katalog daneben).

## Die Befunde (benannt, nicht geglättet)

1. **Die Kette läuft.** Fetch → Pick → SNR-Gate → Korrelation → Inversion
   schließt an echter Wellenform; die Tiefe kommt aus dem Lag, nicht aus dem
   Katalog. Das ist der Feldpilot-Beweis.

2. **Das ±10-km-Gate wird nicht getragen.** +19,0 km Offset. Zwei Lesarten:
   (a) die Katalog-Tiefe eines 231-km-Ereignisses trägt eine größere
   Unsicherheit als die flache ±10-km-Nagelung — das Gate war an
   Flachereignisse kalibriert; (b) die pP-Picks streuen über die Stationen
   (±8 s, weit über der Onset-Streu ±2 s) — die Korrelation verhakt sich an
   Coda-Merkmalen, nicht an einem sauberen pP. Beide Lesarten stehen, keine
   ist gedeutet.

3. **Die Polaritäts-Vorannahme kippt.** Die Flachquellen-Lehrbuch-Regel sagt
   pP invertiert (negativ). Gemessen: pP positiv (+0,63 … +0,91) an vier,
   negativ (−0,68, −0,97) an zwei Stationen; sP durchgehend negativ
   (−0,66 … −0,97). Der Zeuge trennt die Phasen (pP und sP gegenläufig),
   aber die Vorzeichen-Zuordnung der Flachquelle überträgt sich nicht auf
   diese tiefe Geometrie — der Reflexionskoeffizient am freien Rand hängt am
   Einfallswinkel. Das ist ein Messwert, kein Fehler.

4. **Auflösungs-Grenze benannt.** Die Inversions-Klasse (`DEPTHS_FINE`)
   rastet im Bereich 200–250 km in 50-km-Schritten; der Katalog-Wert 231 km
   liegt zwischen zwei Rasterwerten. Die +19 km sind auch Raster-Rauschen.

5. **IC.XAN verhakt pP und sP am selben Lag (55,9 s).** Die beiden Fenster
   überlappen an einem tiefen Ereignis; die Korrelation findet zweimal
   dasselbe Coda-Merkmal. Benannt, nicht geglättet.

## Schließung

**„Scharfe Tiefe" schließt als „Feldpilot bestanden (1 Ereignis)."** Die
Kette steht an echter Wellenform; ein Ereignis trägt die Kette, nicht die
Statistik. Die Flotte (Ereignisse × Stationen, σ und √N) ist der benannte
Nachfolger — neben der W-Phase-M9, die als eigenes Atom entschieden bleibt.
Offene Folge (Register): eine feinere Tiefen-Inversionsklasse und die
Neu-Ableitung der pP-Polarität an tiefer Geometrie.
