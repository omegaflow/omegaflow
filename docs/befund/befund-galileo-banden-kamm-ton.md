<!--
  title: Befund — Galileo 20-s-Kamm & Station-42-Ton: Kamm = Abtastgitter-Degeneranz, Ton = Einzellinie (Identität offen)
  class: befund
  date: 2026-09-05
  sha256: 8cc5453178c5bac38defb4a3f65318d91dd41ca7091702353a56818de43bb1ef
  status: done
  antwortet-auf: docs/auftrag/auftrag-bande-split.md
  see-also: docs/befund/befund-galileo-banden-negativ.md docs/befund/befund-galileo-gwe-bestand.md docs/paper/ground-sources-20s-band.md
-->

# Befund: Galileo 20-s-Kamm & Station-42-Ton

## Frage & Bindung

Der Cross-Mission-Test (befund-galileo-banden-negativ, status done) fand die
Pioneer-Linienfrequenzen nicht auf Galileo; zwei neue Linien blieben offen:
(a) ein exakter 20-s-Kamm bei 50/100/150/200 mHz in den dünnen
Dez-1990-Zweiweg-Pässen der 70-m-Stationen (14/43/63), (b) ein Station-42-Ton
bei 52,39 mHz (19,1 s Periode, Dez-1990). Dieser Befund charakterisiert beide
aus den Residuen selbst: Ist der Kamm eine messbare Schwingung oder die
Signatur des Abtast-Reduktionsrasters? Ist der Ton eine isolierte Linie oder
Teil eines Musters?

Probe `galileo_band_probe.rs` (LS 30–70 mHz @ 0,05 mHz, je Station/Mode; Floor
= Band-Median, Mitglieder ≥3×) scannt nur bis 70 mHz — die Kamm-Glieder
100/150/200 mHz liegen außerhalb. Eigene Messung auf `data/galileo_resid.bin`
(GASR: 14 077 825 Sätze, 8 × f64: tdb, resid_Hz, station, mode, dtype,
ref, sampler_s, strength; 1990-11-29..1997-02-28): LS 30–260 mHz @ 0,02 mHz
sowie normiertes Lomb-Scargle (Varianz-Erklärung in [0,1]), je Segment
(Gap 60 s, min 120), Lock-Schnitt |resid| ≤ 1000 Hz wie in der Probe.

## n zuerst

Dez-1990 = Unix-Tage 7638..7670 (1990-11-30..1990-12-31). Zweiweg (Mode 2),
gelockt:

- DSS 14: n = 1592 (davon 5-s-Sampler 120, keine ≥40er-Kette)
- DSS 43: n = 3810 (5-s-Sampler 105, keine Kette)
- DSS 63: n = 4201 (5-s-Sampler 100, keine Kette)

Median-Abtastrate in allen drei Pässen: **60,00 s** (dt-Histogramm: 60-s ×
1468/3699/4091; 5-s nur vereinzelt). Nyquist = 8,33 mHz.

## Der Befund (a): Der 20-s-Kamm ist die Degeneranz des 60-s-Rasters

**Messung 1 — Positionen exakt auf dem Raster.** Im Rohe-Lomb-Scargle liegen
die Kamm-Spitzen auf EXAKT 50,0000/100,0000/150,0000/200,0000/250,0000 mHz
(Offset 0,0000 mHz auf dem 0,02-mHz-Gitter; enger als das 0,05-mHz-Gitter der
Probe), an allen drei 70-m-Stationen, und zwar in den reinen 60-s-Segmenten
(n_detrend st 14: 986–1401, st 43: 3598–3776, st 63: 3709–4051).

**Messung 2 — die Verhältnisse sind numerisch, nicht physikalisch.** Die
Rohe-LS-Verhältnisse sind astronomisch: 10^12–10^29 × Floor bei 50/100/150/200
mHz. Das ist eine Entartung des unnormierten LS: bei einer Frequenz, die ein
ganzzahliges Vielfaches von 1/60 s ist (50 mHz × 60 s = 3,0; 100 × 60 = 6,0;
150 × 60 = 9,0; 200 × 60 = 12,0 Zyklen pro Abtastschritt), ist die Sinus-Spalte über alle
Proben konstant 0 und die Kosinus-Spalte konstant 1 (entartet mit dem
Mittelwert) → Normalengleichung singulär → Leistung explodiert.
**Kontrolle:** weißes Rauschen auf demselben 60-s-Zeitraster reproduziert
dieselben astronomischen Verhältnisse an genau denselben Frequenzen
(10^15–10^18). Die Werte selbst tragen diese Leistung nicht.

**Messung 3 — normiert (Varianz-Erklärung) ist dort nichts.** Das normierte
LS (Press-&-Rybicki, [0,1]) an 50/100/150/200 mHz erklärt: st 43 ≤ 0,6 %,
st 63 ≤ 0,5 % Varianz; st 14 7,75 % an 50,0 mHz UND identisch 7,75 % an
150,0 mHz — 150 mHz ist das Alias von 50 mHz auf dem 60-s-Raster (Differenz
100 mHz = 6 × 16,667 mHz). Gleiche Leistung am Alias-Paar = dieselbe
niederfrequente Struktur, nicht zwei unabhängige Linien. Eine echte kohärente
Linie zeigt ~100 % (siehe Ton unten und injizierte 50-mHz-Sinus-Kontrolle:
18,9 % an 50,0 und identisch 18,9 % an 150,0 — das Alias-Paar).

**Verdikt (a):** Die Linien liegen auf exakten Vielfachen von 50 mHz — das ist
arithmetisch die Signatur einer 1/20-s-Periode. Aber die zugrundeliegenden
Residuen sind auf einem **60-s-Raster** abgetastet (Nyquist 8,33 mHz); jede
Frequenz über 8,33 mHz ist dort nicht auflösbar. Der Kamm ist die
Abtast-/Reduktionsraster-Signatur (die 60-s-Kadenz; ihre 3./6./9./12. Harmonische
fallen auf 50/100/150/200 mHz), keine unabhängig gemessene physikalische
20-s-Schwingung. Ein physikalischer 20-s-Prozess (50-mHz-Fundamental) ist von
dieser Raster-Harmonischen arithmetisch nicht durch die Frequenzlage allein zu
trennen; der Mechanismus (welcher Reduktionsschritt ein 20-s- oder 60-s-Raster
trägt) bleibt offen. **Der Kamm ist als „Reduktions-Periodik" bestätigt —
als unabhängige Linie nicht.**

## Der Befund (b): Station-42-Ton — eine isolierte Einzellinie

Station 42, Mode 2, Dez-1990, 1-s-Sampler (98 213 der 98 702 Sätze), 4 Tage:
**1990-12-07..1990-12-10**. n_locked 1-s = 98 149.

- **Einzellinie bei 52,39 mHz** (Parabel-Interpolation 52,385–52,390 mHz;
  Periode 19,10 s). Varianz-Erklärung in den langen 1-s-Segmenten: **98,9–100 %
  am 1990-12-09/10** (Segmente n 2 150–11 421; RMS der detrendeten Residuen
  5,3–6,7 Hz).
- **Isoliert, kein Muster:** die 2. Harmonische bei 104,77 mHz erklärt 0,0 %,
  die Subharmonische bei 26,2 mHz 0,0 %. Kein Vielfachen-Kamm, keine
  Seitenband-Familie im 45–58-mHz-Fenster (einzige Spitze über 2 % ist die
  52,39-mHz-Linie selbst).
- Station 42 trägt Mode-2-Daten nur 1990 (1990: Mode 1 = 636, Mode 2 = 98 702,
  Mode 3 = 5 848; 1995–97 nur Mode 1). Der Ton ist damit auf das einzige
  Mode-2-Fenster der Station beschränkt.

**Verdikt (b):** Eine einzelne, kohärente, schmale Linie bei 52,39 mHz
(~99–100 % der Segmentvarianz), kein Obertonspektrum, kein Muster. Identität
(rotierende Struktur, Stations- oder Reduktionsperiodik) ist durch diese Messung
nicht entschieden — **pending**.

## Grenzen

- 5-s-Sampler-Sätze der 70-m-Stationen sind zu kurz für eigene Segmente (100–120
  Sätze, keine ≥40er-Kette); die 60-s-Kadenz dominiert die Dez-1990-Pässe. Ein
  möglicher 5-s-Kanal ist in diesem Fenster nicht eigenständig messbar.
- Die Frequenzlage allein trennt „echte 20-s-Periode" nicht von der
  60-s-Raster-Harmonischen (50 mHz = 3/60 s): beides liegt auf demselben Punkt.
  Die Trennung kam aus der normierten Messung (keine Varianz) und der
  Rauschkontrolle (gleiche Explosion auf gleichem Raster).
- Das GWE-ODR (`GO-X-RSS-1-ODR-V1.0`, annex
  `https://pds-ppi.igpp.ucla.edu/annex/GO-X-RSS-1-ODR-V1.0/`) ist **verifizierbar
  erreichbar**: `AAREADME.TXT` (PDS3-Header, 20 160 B) und
  `DOCUMENT/RSC11_11.TXT` antworten 200; die Annex-Wurzel selbst antwortet 500
  (nginx) — ein Server-Listing-Problem, kein Quellen-Mangel. Das
  GWE-ODR-Cross-Check-Item (1994/95, open-loop, DSS 14/43/63) bleibt als nächste
  stärkere externe Validierung offen (nicht in diesem Lauf).

## Register-Satz

*Der exakte 50-mHz-Kamm in den Dez-1990-Zweiweg-Pässen der 70-m-Stationen ist
gemessen die Degeneranz des 60-s-Abtastrasters (normiert ≤ 8 % Varianz, von
weißem Rauschen auf demselben Raster reproduziert) — eine Reduktions-Periodik,
keine unabhängige Linie. Der Station-42-Ton 52,39 mHz ist eine isolierte
Einzellinie über 4 Tage (Dez-1990), Identität offen.*

## Status

`done` (Rat gehalten, 2026-09-05). Charakterisierung gemessen; der Kamm ist
Abtastraster-Degeneranz, der Station-42-Ton eine isolierte Einzellinie
(Identität offen).
