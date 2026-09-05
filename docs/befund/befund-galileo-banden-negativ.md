<!--
  title: Befund — 20-s-Bande Cross-Mission: Pioneer-Linienfrequenzen missions-spezifisch (negativ)
  class: befund
  date: 2026-09-05
  sha256: 77735df672a76162d7d8b578dccb70f9f91bb5c570d7faca6382b3e1ff5f36da
  status: done
  antwortet-auf: docs/auftrag/auftrag-bande-split.md
  see-also: docs/befund/befund-galileo-gwe-bestand.md docs/befund/befund-galileo-rausch-kurve.md docs/paper/ground-sources-20s-band.md docs/TODO.md
-->

# Befund: 20-s-Bande Cross-Mission — Pioneer-Linienfrequenzen missions-spezifisch (negativ)

## Frage & Bindung

Die 20-s-Bande (Pioneer) trägt stationsfixe, kohärente Linien bei 45,75 /
51,55 / 47,35 mHz (Goldstone/Canberra/Madrid, `ground-sources-20s-band`). Der
Klasse-2-Cross-Mission-Scan fragt: erscheinen dieselben Linienfrequenzen auf
einer anderen Mission an denselben Stationen? Galileo ist die erste andere
Mission mit denselben drei Stationen (DSS 14/43/63 = 97 % der
Galileo-Residuen). Wenn die 20-s-Bande ein DSN-Ketten-Erbe wäre
(stationseigenes Equipment), müssten die Linien auf Galileo an denselben
Stationen erscheinen; sind sie Pioneer-spezifisch, nicht.

Probe `tools/measure/src/bin/galileo_band_probe.rs` (LS 30–70 mHz @ 0,05 mHz,
je (Station, Mode, Ära); Spiegel-Zahlen 44–56 mHz; Floor = Band-Median;
Mitglieder ≥3×). Daten `data/galileo_resid.bin` (GASR, 14 077 825 Proben,
1990-11-29 .. 1997-02-28). Suchbreite ±0,5 mHz um jede Pioneer-Referenz.

## n zuerst

Stationen 14/43/63 tragen 97 % der Galileo-Proben (Rausch-Kurve-Nebenfund).
Die drei Pioneer-Referenzlinien werden je (Station, Mode, Ära) geprüft.

## Der Befund — NEGATIV

**Die Pioneer-Linienfrequenzen 45,75 / 51,55 / 47,35 mHz erscheinen auf
Galileo an denselben Stationen (14/43/63) in keinem Modus und keiner Ära
innerhalb ±0,5 mHz.** Das Verdikt ist „Pioneer-Linienfrequenzen sind
missions-spezifisch" formuliert — **nicht** „die Bande ist weg". Der
Unterschied ist 0 honored: ein negatives Cross-Mission-Ergebnis schließt die
Frequenzen als Galileo-Eigenschaft aus, nicht die 20-s-Bande als
Pioneer-Befund.

**Der exakte 20-s-Kamm (50/100/150/200 mHz, ~100 % Varianz)** tritt in den
dünnen Dez-1990-Zweiweg-Pässen der 70-m-Stationen auf — eine
Reduktions-Periodik (Harmonische von 1/20 s), Mechanismus offen.

**Der Station-42-Ton 52,39 mHz** (Periode 19,1 s = 3,14 U/min, 4 Tage
Dez-1990) ist eine neue Linie, Identität offen.

## Grenzen

- Der Scan deckt 30–70 mHz ab; die drei Pioneer-Linien (45,75/51,55/47,35 mHz)
  liegen im Scan. ±0,5 mHz ist die Suchbreite.
- Mode 1 (Einweg) ist der schwache Kanal; der exakte 20-s-Kamm erscheint nur in
  den dünnen Dez-1990-Zweiweg-Pässen.
- Die GWE-ODR-Quelle (`GO-X-RSS-1-ODR-V1.0`, 1994/95, open-loop, DSS 14/43/63)
  ist die nächste stärkere externe Validierung — nicht in diesem Lauf (das
  Klasse-2-Item bleibt dort offen).

## Register-Satz

*Die Pioneer-Linienfrequenzen der 20-s-Bande sind missions-spezifisch: auf
Galileo erscheinen sie an denselben Stationen in keinem Modus und keiner Ära.
Der exakte 20-s-Kamm (50/100/150/200 mHz) und der Station-42-Ton 52,39 mHz
sind neue, offene Linien.*

## Status

`done`. Der Cross-Mission-Test ist negativ (die Sonde sieht den 20-s-Kamm und
den 52,39-mHz-Ton — sie ist nicht blind); der Mechanismus des 20-s-Kamms und
die Identität des Station-42-Tons bleiben offen.
