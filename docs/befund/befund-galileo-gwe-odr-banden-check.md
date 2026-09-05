<!--
  title: Befund — 20-s-Bande extern: Galileo GWE-ODR open-loop (gezielte Ein-Pass-Stichprobe) negativ
  class: befund
  date: 2026-09-05
  sha256: b1550f4a445a56a7cc1d95bec4e734427f6e588f60fa33b449e15b7d19e1ce4b
  status: done
  antwortet-auf: docs/auftrag/auftrag-bande-split.md
  see-also: docs/befund/befund-galileo-banden-negativ.md docs/befund/befund-galileo-gwe-bestand.md docs/befund/befund-galileo-banden-kamm-ton.md docs/TODO.md
-->

# Befund: 20-s-Bande extern — Galileo GWE-ODR Open-Loop-Check (gezielte Ein-Pass-Stichprobe, negativ)

## Frage & Bindung

Die 20-s-Bande (Pioneer) trägt stationsfixe, kohärente Linien bei 45,75 / 51,55 /
47,35 mHz (DSS 14/43/63). Der Galileo-Zweiweg-Resid-Cross-Check war negativ
(`befund-galileo-banden-negativ`). Die stärkste offene externe Validierung ist das
GWE-ODR-Volumen `GO-X-RSS-1-ODR-V1.0` (PDS3 `GORS_9110`, PPI/UCLA): offene
(open-loop) Aufzeichnungen der Gravitationswellen-Experimente GWE1 (1994) und GWE2
(1995) an genau denselben drei 70-m-Stationen DSS 14/43/63. Frage dieser gezielten
Ein-Pass-Stichprobe: erscheint die 20-s-Bande (45,75/51,55/47,35 mHz) in den
open-loop ODR-Reihen derselben Stationen?

Bindung: open-loop ODR ist ein roher Spannungs-/Amplitudenstrom, kein
Closed-Loop-Doppler-Residuum. Die Bande wurde in Doppler-**Residuen** gefunden
(`doppler_resid` upstream). Ob eine 45,75/51,55/47,35-mHz-Struktur in open-loop ODR
überhaupt erscheinen müsste, ist selbst die Transfer-Frage — gemessen wird ohne
Transfer-Annahme an zwei Reihen, die der open-loop Datenlage entsprechen: (a) der
**Träger-Frequenzreihe** (Momentanfrequenz der dominanten S-RCP-Komponente, das
open-loop Analogon des Doppler-Residuums) und (b) der **Amplitudenhüllkurve**
desselben Kanals.

## n zuerst

Vollständige Einzel-Pass-Dateien, eine je Station, aus beiden GWE-Fenstern
(1994 = GWE1, 1995 = GWE2), 566-Byte-Records (166 B Header + 400 B 8-bit-Samples),
S-RCP (Kanal 2, Galileo-Signal), Header-Station == LBL-Station (gemessen):

| Datei | DSS | Fenster (UTC) | Dauer | Records | Frequenzreihe n (10 sps) | Träger | Ton-SNR |
|---|---|---|---|---|---|---|---|
| `51560410.ODR` | 14 | 1995-06-05 04:10–08:29 (GWE2) | 4,33 h | 31 192 | 155 959 | 54,38 Hz | 25,0 dB |
| `41241454.ODR` | 43 | 1994-05-04 14:54–21:19 (GWE1) | 6,42 h | 46 202 | 231 009 | 82,52 Hz | 11,8 dB |
| `51711933.ODR` | 63 | 1995-06-20 19:33 – 06-21 03:11 (GWE2) | 7,64 h | 55 000 | 274 999 | 48,19 Hz | 25,1 dB |

Methode gespiegelt von `galileo_band_probe.rs`: Lomb-Scargle (floating mean)
30–70 mHz @ 0,05 mHz (801 Bins), lineare Detrend je Segment (Gap 60 s, min 120),
Floor = Band-Median, Mitglieder ≥3×, Spiegel-Zahlen 44–56 mHz, Suchbreite ±0,5 mHz
um jede Pioneer-Referenz. Zusätzlich: empirische Ranzahl (wie viele der 801 Bins
erreichen/übersteigen die Referenz-Leistung) — die Probe selbst prüfte nur das
Verhältnis; die Ranzahl setzt es in die Band-Verteilung.

## Der Befund — NEGATIV (n = 3 Pässe, eine je Station)

**Die Pioneer-Linienfrequenzen 45,75 / 51,55 / 47,35 mHz erscheinen in keiner der
drei gemessenen GWE-ODR-open-loop-Reihen — weder in der Träger-Frequenzreihe noch
in der Amplitudenhüllkurve.** Die LS-Leistung am exakten Referenz-Bin verhält sich
in beiden Reihen statistisch exakt wie weißes Rauschen (gemessen gegen die
Exponential-Erwartung: die Anzahl der 801 Bins, die die Referenz-Leistung
erreichen, stimmt mit 801·2^−r überein, r = Verhältnis/Floor):

| Station | Reihe | Referenz | am Ref (× Floor) | Bins ≥ Ref (gemessen) | Bins ≥ Ref (Rausch-Erwartung) |
|---|---|---|---|---|---|
| DSS 14 | Frequenz | 45,75 mHz | 1,23× | 348 / 801 | ≈348 |
| DSS 43 | Frequenz | 51,55 mHz | 3,18× | 85 / 801 | ≈88 |
| DSS 63 | Frequenz | 47,35 mHz | 2,31× | 163 / 801 | ≈162 |
| DSS 14 | Amplitude | 45,75 mHz | 0,47× | 601 / 801 | ≈580 |
| DSS 43 | Amplitude | 51,55 mHz | 0,52× | 552 / 801 | ≈552 |
| DSS 63 | Amplitude | 47,35 mHz | 4,14× | 38 / 801 | ≈45 |

Kein Referenz-Bin liegt über seiner Rausch-Erwartung; die scheinbar „näheren"
Werte (DSS 43 Frequenz 3,18× bei 51,55; DSS 63 Amplitude 4,14× bei 47,35) sind
exakt die Rausch-Schwänze, nicht Linien. Auch die ganze 30–70-mHz-Bande verhält
sich in der Frequenzreihe wie weißes Phasenrauschen: stärkstes Band-Mitglied je
Pass 7,7×/9,8×/10,6× Floor — die Rausch-Erwartung für das Maximum aus 801 Bins ist
ln(801)/ln 2 ≈ 9,6×; die Anzahl der ≥3×-Mitglieder (67/66/68) liegt bei der
Erwartung 801·2^−3 ≈ 100. Ein kohärenter Ton (wie der Galileo-Station-42-Ton
52,39 mHz mit ~99 % Varianz) erscheint nirgends.

**Empfindlichkeits-Kalibrierung (Injektion, kein Datenwert):** eine injizierte
Sinusschwingung der Stations-Referenzfrequenz mit 0,3× der RMS der detrendeten
Frequenzreihe erscheint bei 3 700–8 700× Floor (DSS 14/43/63). Die Sonde ist
gegen eine kohärente 20-s-Frequenzlinie von ~0,03 Hz Amplitude empfindlich — die
drei Referenzfrequenzen sind in diesen Pässen darunter gemessen **absent**, nicht
unerreichbar.

## Grenzen

- **Gezielte Ein-Pass-Stichprobe, kein Vollscan:** 3 von 241 physisch vorhandenen
  `.ODR`-Dateien (75 MB von ~4,2 GB), je ein zusammenhängender Pass pro Station
  (4,3/6,4/7,6 h), über GWE1 und GWE2. Der Restbestand (77 Tracking-Tage) ist
  ungescannt.
- **Transfer unentschieden:** die 20-s-Bande wurde in Zweiweg-Doppler-Residuen
  gefunden; ob dieselbe Stationsperiodik in open-loop ODR überhaupt sichtbar sein
  müsste, beantwortet dieser Scan nicht — er misst nur, dass sie in den beiden
  open-loop Reihen (Trägerfrequenz und Amplitude) dieser Pässe nicht erscheint.
- Ein Einzel-Pass pro Station bindet die Aussage auf diesen Tag/diese Epoche;
  stationsfeste Linien, die nur intermittierend aktiv sind, können in anderen
  Pässen liegen.
- DSS 43 1994: Ton-SNR 11,8 dB (schwächster Träger); die Frequenzreihe ist dort
  phasenrausch-lastiger — der Null ist dennoch gegen die Rausch-Erwartung
  konsistent.
- Die Amplitudenreihe trägt einzelne Strukturen über der Rausch-Erwartung des
  Maximums (DSS 14: 13,8× bei 42,8 mHz; DSS 63: 15,8× bei 32,5 mHz) — außerhalb
  des Pioneer-Bandes-Fensters der drei Referenzen, Identität ungeprüft (pending).
- Grid-Abdeckung: Referenzen 45,75/51,55/47,35 mHz liegen exakt auf dem
  0,05-mHz-Gitter (gemessen); ±0,5 mHz Suchbreite eingehalten.

## Register-Satz

*Die Pioneer-Linienfrequenzen der 20-s-Bande (45,75/51,55/47,35 mHz) erscheinen in
einer gezielten GWE-ODR-open-loop-Ein-Pass-Stichprobe (je ein voller DSS-14/43/63-Pass,
GWE1+GWE2) weder in der Träger-Frequenzreihe noch in der Amplitudenhüllkurve: die
LS-Leistung an den drei Referenzfrequenzen ist gemessen ununterscheidbar vom
Weißrauschen der Bande (Ranzahl = Rausch-Erwartung), die Sonde ist injektions-
kalibriert empfindlich. Die open-loop-ODR-Übertragung der Bande bleibt als Frage
offen; der Restbestand (238 Dateien, 77 Tage) ungescannt.*

## Status

`done` (Rat gehalten, 2026-09-05). Gemessen: Datensatzstruktur
`GO-X-RSS-1-ODR-V1.0` (566-Byte-Records, 4 Kanäle à 200 sps, S-RCP = Galileo-Signal,
Station im Header); gezielte Ein-Pass-Stichprobe 3 Pässe/3 Stationen (3 von 241);
Scan 30–70 mHz @ 0,05 mHz auf Trägerfrequenz- und Amplitudenreihe → negativ an den
drei Referenzen, Bande in der Frequenzreihe statistisch weiß. Transfer-Frage und
Restbestand (238 Dateien, 77 Tage) offen.
