<!--
  title: Befund — Galileo-Pass-Detrend-Metrik: der Pass-Boden nach intra-pass linearem Detrend
  class: befund
  date: 2026-09-05
  sha256: cc838b2dc05bffd182fb0555d5ac76983b252a7f50a23b607289c63cf431ff3c
  status: done
  antwortet-auf: docs/befund/befund-galileo-pass-segmentierung.md
-->

# Befund: Galileo-Pass-Detrend-Metrik — der Pass-Boden nach intra-pass linearem Detrend

## Frage & Bindung

Die Pass-Segmentierung (Befund vom 2026-09-05) mass den Pass-Boden als RMS um den Pass-Mittelwert
und benannte die Grenze selbst: dieser RMS behaelt niederfrequente Drift innerhalb des Passes; eine
detrendete Pass-Metrik (Segmentierung wie in den Detrend-Sonden, 60-s-Luecke) ist die naechste Stufe.
Dieser Entwurf legt die Detrend-Sonde daneben und misst, ob der intra-pass lineare Detrend den
Pass-Boden senkt (die Drift hat ihn aufgeblaeht) oder unveraendert laesst (der Boden ist
Hochfrequenz-Streuung, keine Drift).

Gebunden: Pass = durchgehender Tracking-Arc je (Station, Modus), Pass-Grenze = Zeitluecke > 600 s
(Vorgabe der Pass-Segmentierung); Lock |resid| > 1000 Hz vor dem Rauschen getrennt; Detrend-Konvention
der Rausch-Sonden: Segmentgrenze = Luecke > 60 s zwischen Nicht-Lock-Proben, Segment-Mindestlaenge
120 Proben, linearer LS-Fit je Segment. Metriken je Pass auf den behaltenen Proben (Proben in
Segmenten >= 120): const = RMS um den Pass-Mittelwert der behaltenen Proben, block = RMS um jeden
Segment-Mittelwert, detr = RMS um den linearen LS-Fit jedes Segments. Boden = Median ueber die Paesse
mit >= 30 Proben (MIN_CELL). Datenkette `data/galileo_resid.bin` (GASR,
`sha256 2375b309…cbe783`). Neue Sonde `tools/measure/src/bin/galileo_pass_detrend_floor.rs`
(`cargo check` 0/0), Report auf stdout.

## Tabelle 1 — Pass-Boden: non-detrended (Befund) → matched → detrended

Spaite pass_floor_hz reproduziert den Befund-Pass-Boden (RMS um den Pass-Mittelwert, alle
Nicht-Lock-Proben). matched = die Paesse mit >= 30 Proben, die mindestens ein Segment >= 120 haben
(die Bevoelkerung, fuer die ein Detrend definiert ist); matched_pass_floor_hz = deren
nicht-detrendeter Boden (RMS um den Pass-Mittelwert, alle Nicht-Lock-Proben). const/block/detr sind
auf derselben behaltenen Probe identisch gemessen.

| Modus | St | Proben | Tage | Paesse ge30 | Pass-Boden (Befund) | matched nicht-detr | const | block | detr |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 14 | 2 678 813 | 131 | 146 | 0,79 Hz | 0,70 Hz (138) | 0,44 | 0,44 | 0,39 |
| 1 | 43 | 4 385 787 | 127 | 131 | 1,54 Hz | 1,45 Hz (129) | 1,03 | 1,03 | 0,97 |
| 1 | 63 | 2 503 349 | 140 | 138 | 3,69 Hz | 4,47 Hz (133) | 2,64 | 2,64 | 2,63 |
| 2 | 14 | 749 711 | 54 | 55 | 1,50 Hz | 0,41 Hz (52) | 0,31 | 0,31 | 0,31 |
| 2 | 43 | 1 190 102 | 53 | 55 | 1,61 Hz | 1,61 Hz (55) | 1,17 | 1,17 | 0,91 |
| 2 | 63 | 959 187 | 75 | 64 | 3,37 Hz | 3,37 Hz (61) | 1,86 | 1,59 | 1,59 |

## Tabelle 2 — Der Detrend-Anteil auf identischen Proben (Drift-Zerlegung)

Fuer jeden Pass mit const- und detr-Metrik ist detr/const <= 1 (der LS-Fit minimiert die
Residual-Streuung). Die Zerlegung zaehlt die Paesse mit detr < 0,9·const und die Mediane.

| Modus | St | detr/const p50 | Paesse mit detr < 0,9·const | med const → med detr | Anteil der langsamen Trend-Bewegung |
|---|---|---|---|---|---|
| 1 | 14 | 1,00 | 13/138 | 0,44 → 0,39 Hz | 0,05 Hz |
| 1 | 43 | 1,00 | 12/129 | 1,03 → 0,97 Hz | 0,06 Hz |
| 1 | 63 | 1,00 | 9/133 | 2,64 → 2,63 Hz | 0,01 Hz |
| 2 | 14 | 1,00 | 1/52 | 0,31 → 0,31 Hz | 0,00 Hz |
| 2 | 43 | 0,99 | 8/55 | 1,17 → 0,91 Hz | 0,26 Hz |
| 2 | 63 | 1,00 | 8/61 | 1,86 → 1,59 Hz | 0,27 Hz |

block == const an fuenf der sechs Zellen: die behaltenen Segmente liegen auf gleichem Niveau; an
Modus 2 / Station 63 senkt erst die block-Metrik (1,86 → 1,59 Hz), der lineare Fit innerhalb der
Segmente nicht weiter (block == detr) — dort tragen Niveau-Versaetze zwischen Segmenten, keine
lineare Drift.

## Der Befund

**1. Der intra-pass lineare Detrend senkt den Pass-Boden nicht substantiell — der Boden ist
Hochfrequenz-Streuung, keine langsame Pass-Drift.** Auf identischen behaltenen Proben liegt der
Detrend-Boden (detr) an allen Zellen bei oder dicht unter dem const-Boden: Mode 1 0,39/0,97/2,63 Hz
gegen 0,44/1,03/2,64 Hz, Mode 2 0,31/0,91/1,59 Hz gegen 0,31/1,17/1,86 Hz. Der Median von detr/const
ist an fuenf Zellen 1,00 und an einer 0,99; die Mehrheit der Paesse wird durch den Detrend um weniger
als 10 % gesenkt (13/138 bis 1/52 der Paesse unter 0,9). Die langsame Trend-Bewegung im Pass traegt
0,00–0,27 Hz.

**2. Nur an Mode-2-Station 43/63 traegt der Detrend messbar ab (0,26/0,27 Hz), und auch dort bleibt
die Streuung dominant.** Mode 2 / Station 43 senkt der lineare Fit innerhalb der Segmente
(block 1,17 = const 1,17 → detr 0,91); an Station 63 senkt die block-Metrik (1,86 → 1,59 Hz), weil
Segmente auf verschiedenen Niveaus liegen. An fuenf der sechs Zellen ist block == const, dort existiert
kein Niveau-Versatz zwischen den Segmenten.

**3. Die Luecke zwischen Befund-Pass-Boden und detr-Boden (z. B. Mode 1 / Station 14: 0,79 → 0,39 Hz)
stammt nicht aus dem Detrend, sondern aus der Segment-Zugehoerigkeit.** Der nicht-detrendete Boden
der matched-Bevoelkerung (0,70 Hz an Mode 1 / Station 14) faellt erst durch das Verwerfen kurzer,
lauter Teil-Arcs (Segmente < 120 Proben — Lock-Fragment-Arcs) auf const (0,44 Hz); der lineare Fit
traegt davon nur 0,05 Hz. Die 60-s/120-Proben-Segmentierung selbst ist der groessere Schritt als die
Entfernung der linearen Drift.

Die Verschiebungsrichtung zwischen Befund-Pass-Boden und matched nicht-detrendetem Boden ist
zellenabhaengig und wird vom Mode-1-Station-14-Beispiel (0,79 → 0,70 Hz) nicht verallgemeinert:
Mode 2 / Station 14 faellt deutlich (1,50 → 0,41 Hz), Mode 1 / Station 63 ist die benannte Anomalie —
dort liegt der matched nicht-detrendete Boden (4,47 Hz ueber 133 Paesse) ueber dem Befund-Boden
(3,69 Hz ueber 138 Paesse): die 5 durch die Segment-Anforderung ausgeschlossenen Paesse (Grenzen)
senken den Boden der vollen Bevoelkerung. Die Segment-Zugehoerigkeit kann den matched-Boden damit
auch anheben, nicht nur senken.

**4. Der detr-Boden liegt an jeder Zelle unter dem Station-Tag-Boden** (Mode 1: 0,39/0,97/2,63 gegen
0,98/2,14/3,69 Hz; Mode 2: 0,31/0,91/1,59 gegen 1,50/1,61/2,87 Hz) — die Pass-Zerlegung der
Vorgaenger-Metrik wird durch die Detrend-Segmentierung fortgesetzt, nicht umgekehrt.

## Grenzen

- Der Detrend ist linear je Segment; eine quadratische oder stueckweise langsamere Drift innerhalb
  des Passes (lange Mode-1-Arcs bis 7,5 h) bleibt ungemessen — die naechste Stufe.
- block == const an fuenf Zellen ist auf 2 Dezimalen gemessen; Niveau-Versaetze unter 0,005 Hz sind
  nicht aufgeloest.
- Die detr-Bevoelkerung (matched) schliesst Paesse ohne Segment >= 120 aus (Mode 1: 8/2/5, Mode 2:
  3/0/3 Paesse); deren Boden ist nicht detrendbar und bleibt aussen vor.
- Nur Modus 1 und 2 an Station 14/43/63 vermessen; Modus 3 bleibt `pending`.

## Register-Satz

*Der intra-pass lineare Detrend (60-s-Luecke, 120-Proben-Segmente) laesst den Pass-Boden im Median
unveraendert: detr/const p50 = 0,99–1,00, nur 0,00–0,27 Hz langsame Trend-Bewegung (Mode 2 an
Station 43/63 traegt mit 0,26/0,27 Hz, die uebrigen Zellen < 0,06 Hz). Der Pass-Boden ist damit
Hochfrequenz-Streuung um die Arc-Bahn, keine langsame intra-pass Drift; die Luecke zwischen dem
Befund-Pass-Boden (0,79–3,69 Hz) und dem detr-Boden (0,39–2,63 Hz) traegt die
60-s/120-Proben-Segmentierung (Verwerfen kurzer Lock-Fragment-Arcs), nicht der Detrend.*

## Status

`done` (vom Rat gehalten; die zellenabhängige matched-Richtung mit der Station-63-Anomalie ist
benannt). Die Sonde `galileo_pass_detrend_floor.rs` ist
`cargo check` 0/0. Die Pass-Segmentierung der Vorgaenger wird reproduziert (Pass-Boden und
Station-Tag-Boden identisch); der Detrend-Boden liegt bei 0,39/0,97/2,63 Hz (Mode 1) und
0,31/0,91/1,59 Hz (Mode 2) an Station 14/43/63.
