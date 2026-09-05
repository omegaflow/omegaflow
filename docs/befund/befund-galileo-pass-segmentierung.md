<!--
  title: Befund — Galileo-Pass-Segmentierung: der Pass-Boden der GASR-Residuen
  class: befund
  date: 2026-09-05
  sha256: 248b05e9064d020e7757c1935342d3aa6367ad83bc0719f4a9be334100940eb8
  status: done
  antwortet-auf: docs/befund/befund-galileo-mode1-fingerabdruck.md docs/befund/befund-galileo-rausch-kurve-epsilon.md
  see-also: docs/befund/befund-galileo-rausch-kurve.md docs/TODO.md
-->

# Befund: Galileo-Pass-Segmentierung — der Pass-Boden der GASR-Residuen

## Frage & Bindung

Beide Vorgänger-Blätter markierten die Pass-Segmentierung als `pending`: die Tages-Zellen-Metrik
(Median der Tages-RMS je (Modus, Tag), bei der Rausch-Kurve über alle Stationen gepoolt) mischt
mehrere Pässe und Stationen desselben Tages; die Pass-Wahrheit braucht die Zerlegung der Residuen
in durchgehende Tracking-Arcs. Dieser Entwurf baut die Segmentierung und misst den ersten
in-pass-Boden.

Gebunden: Pass = durchgehender Tracking-Arc je (Station, Modus); Pass-Grenze = Zeitlücke zwischen
aufeinanderfolgenden Proben derselben (Station, Modus) größer als die Schwelle; Lock-Übergänge
(|resid| > 1000 Hz) vor dem Rauschen getrennt (gezählt, nicht gerauscht); Rausch-Metrik = RMS um
den Pass-/Zellen-Mittelwert; RMS-Listen mit ≥ 30 Nicht-Lock-Proben. Die Schwelle wird aus der
gemessenen dt-Verteilung begründet, nicht blind gesetzt (Vorgabe ~ Minuten). Datenkette:
`data/galileo_resid.bin` (GASR, 14 077 825 Proben, `sha256 2375b309…cbe783`). Neue Sonde
`tools/measure/src/bin/galileo_pass_segment.rs` (`cargo check` 0/0), Report auf stdout — keine
Report-Datei, keine Änderung an Bestandsdateien.

## n zuerst — die Probe und die dt-Verteilung

| Modus | Proben | Lock-Übergänge | Tage |
|---|---|---|---|
| 1 | 9 743 574 | 1 568 246 | 163 |
| 2 | 3 110 045 | 157 784 | 109 |
| 3 | 1 224 206 | 268 480 | 89 |

Die Proben-Klassifikation reproduziert die Vorgänger exakt (Mode 1: 163 Tage; 2: 109; 3: 89).

dt-Verteilung zwischen aufeinanderfolgenden Proben derselben (Station, Modus):
p50 = p90 = p99 = 1,00 s; p99,9 = 60 s; Maximum 156,6·10⁶ s (Lücken über die Mission).

| dt-Bin | Lücken |
|---|---|
| < 1 s | 14 048 920 |
| 1–60 s | 27 896 |
| 1–2 min | 100 |
| 2–5 min | 31 |
| 5–10 min | 43 |
| 10–30 min | 27 |
| 30–60 min | 20 |
| 1–2 h | 39 |
| 2–6 h | 507 |
| 6–24 h | 98 |
| 1–3 d | 121 |

(Bins 1–60 s aggregiert; die Probe emittiert feinere dt-Bins, die Tabelle fasst die
Arcs-internen 1-s-Kadenzen zusammen.)

Die Kadenz der Tracking-Arcs ist 1 s. Zwischen 60 s und 2 h liegt ein gemessenes Tal (insgesamt
~260 Lücken der Mission über alle Station-Modus-Serien); die inter-Pass-Trennungen beginnen bei
~2 h. Die Schwelle 600 s sitzt in diesem Tal: 60-fach über p99 der Kadenz (60 s), unterhalb der
inter-Pass-Trennungen. Empfindlichkeit (Pass-Zahl gesamt bzw. Modus 1/2 an 14/43/63):

| Schwelle | Pässe gesamt | Modus 1 @14/43/63 | Modus 2 @14/43/63 |
|---|---|---|---|
| 300 s | 909 | 477 | 195 |
| 600 s | 877 | 457 | 187 |
| 900 s | 860 | 447 | 183 |

Die Pass-Zahl ändert sich über 300–900 s nur langsam — der Boden ist gegen die Schwelle stabil.

## Pass-Struktur je Station und Modus (14/43/63; ≥ 30 Nicht-Lock-Proben je Pass)

| Station | Modus | Proben | Tage | Pässe | Pässe ≥30 | med. Länge min | med. Pass-n | med. Pass-Residuum Hz |
|---|---|---|---|---|---|---|---|---|
| 14 | 1 | 2 678 813 | 131 | 155 | 146 | 265 | 14 179 | 0,19 |
| 43 | 1 | 4 385 787 | 127 | 149 | 131 | 451 | 33 082 | 0,23 |
| 63 | 1 | 2 503 349 | 140 | 153 | 138 | 196 | 13 231 | 0,17 |
| 14 | 2 | 749 711 | 54 | 59 | 55 | 230 | 13 465 | 0,08 |
| 43 | 2 | 1 190 102 | 53 | 60 | 55 | 428 | 20 932 | 0,08 |
| 63 | 2 | 959 187 | 75 | 68 | 64 | 312 | 17 404 | 0,10 |
| 14 | 3 | 477 405 | 60 | 70 | 65 | 123 | 5 974 | 0,14 |
| 43 | 3 | 581 262 | 50 | 54 | 43 | 163 | 7 400 | 0,11 |
| 63 | 3 | 151 542 | 33 | 35 | 26 | 96 | 4 362 | 0,11 |

Pass-Längen: Modus 1 im Median 3–7,5 h; Modus 2 4–7 h; Modus 3 1,5–3 h. Die Pass-Residuen
(Median je Pass, nur Nicht-Lock) liegen bei ~0,1–0,2 Hz ohne systematischen Offset.

## Tabelle 1 — RMS-Boden: Tages-Zelle (gepoolt) → Station-Tag → Pass

Modus 1 (Stationen 14/43/63):

| Metrik | 14 | 43 | 63 |
|---|---|---|---|
| (Modus, Tag) gepoolt, alle Stationen | 8,20 Hz (163) | — | — |
| Station-Tag-Boden | 0,98 Hz (129) | 2,14 Hz (122) | 3,69 Hz (136) |
| Pass-Boden (Median je Pass) | 0,79 Hz (146) | 1,54 Hz (131) | 3,69 Hz (138) |
| Pass p10 / p90 | 0,04 / 36,1 | 0,04 / 25,4 | 0,04 / 46,6 |

Modus 2:

| Metrik | 14 | 43 | 63 |
|---|---|---|---|
| (Modus, Tag) gepoolt, alle Stationen | 2,79 Hz (107) | — | — |
| Station-Tag-Boden | 1,50 Hz (52) | 1,61 Hz (52) | 2,87 Hz (74) |
| Pass-Boden (Median je Pass) | 1,50 Hz (55) | 1,61 Hz (55) | 3,37 Hz (64) |
| Pass p10 / p90 | 0,03 / 60,1 | 0,03 / 55,1 | 0,04 / 50,2 |

Die Station-Tag-Werte des Mode-1-Fingerabdrucks (0,98 / 2,14 / 3,69 Hz) und der gepoolte
Tages-Wert 8,20 Hz (163) werden reproduziert — die Tages-Metrik selbst ist unverändert, die
Segmentierung ist additiv.

## Tabelle 2 — Modus 1: Pass-Boden nach Stärke-Quartil (Pass-Median der Signalstärke)

| Station | Q1 (−2560) | Q2 | Q3 | Q4 |
|---|---|---|---|---|
| 14 | 0,78 Hz (56) | 1,14 Hz (26) | 0,13 Hz (35) | 2,72 Hz (29) |
| 43 | 0,46 Hz (45) | 1,44 Hz (21) | 1,06 Hz (47) | 5,43 Hz (18) |
| 63 | 0,11 Hz (53) | 4,48 Hz (36) | 4,68 Hz (17) | 6,83 Hz (32) |

## Tabelle 3 — Geometrie-Split (solare Elongation ε): Tages-Zelle und Pass-Boden

Tages-Zellen (gepoolt, alle Stationen — Rausch-Kurven-Reproduktion):

| Modus | ε 0–30° | ε 30–60° | ε 60–120° | ε 150–180° |
|---|---|---|---|---|
| 1 | 7,47 Hz (133) | 8,61 Hz (12) | 20–32 Hz (8) | 8,20 Hz (10) |
| 2 | 0,65 Hz (75) | 16,70 Hz (8) | 10–23 Hz (8) | 41,98 Hz (16) |

Station 14/43/63 im ruhigen Fenster (ε 0–30°, Pass-Mitte) und im lauten Fenster (ε 150–180°):

| Station | Modus | ε 0–30° Pass-Boden | ε 0–30° Station-Tag | ε 150–180° Pass-Boden | ε 150–180° Station-Tag |
|---|---|---|---|---|---|
| 14 | 1 | 0,55 Hz (122) | 0,70 Hz (109) | 0,98 Hz (5) | 6,45 Hz (4) |
| 43 | 1 | 1,45 Hz (107) | 2,24 Hz (99) | 4,41 Hz (7) | 7,51 Hz (6) |
| 63 | 1 | 3,23 Hz (116) | 3,13 Hz (114) | 3,69 Hz (6) | 3,69 Hz (6) |
| 14 | 2 | 0,29 Hz (34) | 0,29 Hz (32) | 60,09 Hz (9) | 55,06 Hz (9) |
| 43 | 2 | 0,87 Hz (36) | 0,58 Hz (33) | 22,59 Hz (14) | 22,65 Hz (13) |
| 63 | 2 | 1,20 Hz (44) | 0,28 Hz (56) | 38,36 Hz (11) | 38,36 Hz (10) |

## Der Befund

**1. Der 8,20-Hz-Tag-Boden von Mode 1 ist ein Pooling-Artefakt — auf Pass-Ebene bestätigt.**
Der gepoolte Tages-Median (8,20 Hz, 163 Zellen) wird reproduziert; der Station-Tag-Boden liegt bei
0,98/2,14/3,69 Hz, der Pass-Boden bei 0,79/1,54/3,69 Hz. Das ruhigste Pass-Zehntel liegt an allen
drei Stationen bei 0,04 Hz (p10). Die Tages-Zelle poolt Stationen und Pässe desselben Tages; die
Segmentierung trennt das Mischen ab.

**2. Das starke Plateau ~0,1 Hz von Mode 1 existiert auf Pass-Ebene als ruhige Pass-Population,
nicht als typischer Pass.** Die ruhigsten Pässe laufen bei 0,04 Hz (p10) und Q3-an-14 bei
0,13 Hz — der Plateau-Boden ist echt und noch unter dem Tages-Wert. Der Pass-Median (0,79–3,69 Hz)
liegt aber über diesem Plateau: laute Pässe (p90 25–47 Hz) tragen den Median. „0,1-Hz-Plateau“
benennt auf Pass-Ebene den ruhigen Modus, nicht den typischen Pass.

**3. Der Stärke-Gradient des Fingerabdrucks ist auf Pass-Ebene nicht entscheidbar —
der Pass-Median mischt Stärke-Zustände.** Der Fingerabdruck maß den Tages-Boden am AGC-Boden
(Q1) 10–20-fach lauter als das Plateau (Q3/Q4) über Proben-Stärke-Bin-Tag-Zellen. Auf Pass-Ebene
(Pass-Median der Stärke) ist das Verhältnis stationenabhängig und nicht monoton: Station 14
Q1 0,78 vs Q3 0,13 Hz (schwach lauter), Station 43 Q1 0,46 vs Q3 1,06 Hz (schwach leiser),
Station 63 Q1 0,11 vs Q3 4,68 Hz (schwach leiser). Die Pass-Klassifikation über den Pass-Median
mischt Stärke-Zustände innerhalb des Passes — sie ist eine andere Achse als die Proben-Stärke-Bin-
Tag-Zellen des Fingerabdrucks, dessen Behauptung damit **nicht widerlegt** ist; die Proben-Bin-Achse
ist auf Pass-Ebene nicht getestet. Ein innerhalb-des-Passes getrennter Stärke-Zustand (Stärke-Zustand
je durchgehendem Unter-Arc) ist die ausstehende Messung.

**4. Das Mode-2-Konjunktions-Fenster trägt echte ruhige Pässe — mit kurzen lauten Pässen an
Station 43/63, kein reines Tages-Misch-Artefakt.** Im ruhigen Fenster (ε 0–30°) liegen die
Pass-Böden bei 0,29/0,87/1,20 Hz (14/43/63) und die Station-Tag-Böden bei 0,29/0,58/0,28 Hz;
im lauten Fenster (ε 150–180°) bei 60,09/22,59/38,36 Hz (Pass) bzw. 55,06/22,65/38,36 Hz
(Station-Tag). Die beiden Regime (ruhig/laut) sind auf Pass-Ebene echt. Aber die Pooling-Richtung
ist stationenabhängig: an Station 14 ist der Pass-Boden gleich dem Station-Tag (0,29 = 0,29) und
unter dem gepoolten Wert (0,65); an Station 43/63 liegt der Pass-Boden (0,87/1,20 Hz) **über** dem
Station-Tag (0,58/0,28) — dort sitzen kurze laute Pässe innerhalb der ruhigen Tage, die Tages-Zelle
glättet sie weg. Der gepoolte Tages-Wert des ruhigen Fensters (in der ε-Kurve 1,5 Hz; hier bei ε am
TDB-Tagesanfang 0,65 Hz über 75 Tage) überschätzt den ruhigen Boden also nur an Station 14/über das
Stationen-Pooling; das Fenster selbst ist echt. Der Unterschied 1,5 vs 0,65 Hz zwischen Vorgänger und
dieser Messung ist gegen E1 (Mode-2-Station-Tag-Split) abzugleichen — dort zwei All-Lock-Tage
(77→75), hier acht laute Tage in ε 30–60° verschoben; die Band-Mitgliedschaft der Rand-Tage
(ε-Klassifikation am Tagesanfang) ist grenzempfindlich und die Tagesmengen-Schnitte der beiden
Deutungen sind nicht gegeneinander gemessen — `pending`.

**5. Die Tages-Metrik der Rausch-Kurve überschätzt den Station-Tag-Boden; die Richtung gilt
nicht auf Pass-Ebene für alle Stationen.** In jedem gemessenen Fenster liegt der gepoolte
(Modus, Tag)-Wert über den Station-Tag-Böden (Mode 1 ε 0–30°: gepoolt 7,47 vs Station-Tag
0,70–3,13 Hz; Mode 2 ε 0–30°: 0,65 vs 0,28–0,58 Hz). Für die Pass-Böden gilt die Richtung nicht
durchgängig: Mode 2 ε 0–30° Pass 0,29/0,87/1,20 Hz vs Station-Tag 0,29/0,58/0,28 Hz — an
Station 43/63 liegt der Pass-Boden über dem Station-Tag. Das Stationen-Pooling der Tages-Zelle
mischt Pässe verschiedener Stationen; die Misch-Stufe des Fingerabdrucks ist auf Station-Tag-Ebene
bestätigt.

## Grenzen

- RMS um den Pass-Mittelwert behält niederfrequente Drift innerhalb des Passes (kein Detrend im
  Pass); der Pass-Boden enthält langsame Trend-Anteile. Eine detrendete Pass-Metrik (Segmentierung
  wie in den Detrend-Sonden, 60-s-Lücke) ist die nächste Stufe.
- Pass-Stärke-Zustand über den Pass-Median klassifiziert; Stärke-Zustände innerhalb eines Passes
  sind gemischt — der Stärke-Gradient ist damit auf Pass-Ebene nicht entscheidbar.
- Station-63-Mode-1-Anomalie bleibt auch auf Pass-Ebene (Q2 4,48; Q3 4,68 Hz), Ursache offen.
- Geometrie: ε am TDB-Tagesanfang (Tag-Zellen) und an der Pass-Mitte (Pass-Zellen); die
  Band-Mitgliedschaft der Rand-Tage ist grenzempfindlich (1,5 vs 0,65 Hz im ruhigen Mode-2-Fenster).
- Nur Modus 1 und 2 an 14/43/63 vermessen; Modus-3-Station-Split bleibt `pending`.

## Register-Satz

*Die Pass-Segmentierung (600-s-Schwelle im gemessenen dt-Tal der 1-s-Kadenz) bestätigt: der
8,20-Hz-Tag-Boden von Mode 1 ist ein Pooling-Artefakt (Pass-Boden 0,79–3,69 Hz, ruhigstes
Pass-Zehntel 0,04 Hz), das ruhige Mode-1-Plateau ~0,1 Hz ist auf Pass-Ebene als ruhige
Pass-Population echt (0,04–0,13 Hz), und das Mode-2-Konjunktions-Fenster löst sich in echte
ruhige Pässe auf (ε 0–30°: Pass-Boden 0,29–1,20 Hz an 14/43/63; ε 150–180°: 22,6–60,1 Hz) — ein
echtes ruhiges Fenster, mit kurzen lauten Pässen an Station 43/63 (Pass-Boden über dem
Station-Tag). Der Stärke-Gradient des Fingerabdrucks ist auf Pass-Ebene **nicht entscheidbar**
(Pass-Median mischt Stärke-Zustände; der Fingerabdruck-Befund bleibt unwiderlegt, die
Proben-Bin-Achse ungetestet); die innerhalb-des-Passes getrennte Stärke-Messung bleibt pending.*

## Status

`done` (Rat gehalten, 2026-09-05). Die Sonde `galileo_pass_segment.rs` ist
`cargo check` 0/0; der Lauf ist ein lokales Artefakt (stdout). Die Pass-Segmentierung ist
gebaut; der Stärke-Gradient ist auf Pass-Ebene nicht entscheidbar, das Mode-2-Fenster trägt
echte ruhige Pässe mit kurzen lauten an 43/63.

Folge-Befunde: die 1,5→0,65-Tagesmengen-Reconciliation ist geschlossen
(`befund-galileo-tagesmengen-reconciliation`, done — E1 hält, exakt 2 All-Lock-Tage);
der Stärke-Gradient ist jetzt auf der Proben-Bin-Achse entschieden
(`befund-galileo-inpass-staerke-rampe`, done — an 43/63 genuine Boden↔Rauschen-Kovarianz
bei Pass-Identität, an 14 Epochen-Kollokation).
