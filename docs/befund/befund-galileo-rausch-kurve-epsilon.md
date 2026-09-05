<!--
  title: Befund — Galileo-Rausch-Kurve auf der ε-Achse: kohärenter Fall invers zur Plasma-Erwartung, Plasma-Deutung getötet
  class: befund
  date: 2026-09-05
  sha256: 2a5678da3bb32ef5a6af5f68b140412f1cc74d19ee173528c361e7b0633b6bc3
  status: done
  antwortet-auf: docs/auftrag/auftrag-quiet-zone-uebertragung.md
  see-also: docs/befund/befund-galileo-rausch-kurve.md docs/befund/befund-galileo-mode1-fingerabdruck.md
-->

# Befund: Galileo-Rausch-Kurve auf der ε-Achse — der kohärente Fall ist invers zur Plasma-Erwartung

## Frage & Bindung

Die Revision v2 (`befund-galileo-rausch-kurve`, 2026-09-05) hat die Achse
korrigiert — der gemessene Winkel war α am Sonnenort (Erde–Sonne–Sonde), nicht
die solare Elongation ε — und die Plasma-Deutung auf `unverified` gestuft. Der
dort als `pending` registrierte Schritt war: Mode-2/3-Rausch-Kurve auf der
ε-Achse neu ziehen. Dieser Entwurf zeichnet genau diese Kurve.

Gebunden: n je Band **zuerst**, dann die Kurve. ε ist der Winkel an der Erde
(Sonne–Erde–Sonde); für die äußere Sonde gilt α + ε ≈ 180°. Die
Plasma-Erwartung bindet die Richtung: Plasma-Szintillation ist **laut bei
kleinem ε** (Konjunktion, Plasma auf der Sichtlinie) und **leise bei großem ε**
(Opposition). Laut-bei-kleinem-ε ⇒ konsistent mit Plasma; das Gegenteil oder
eine flache Kurve ⇒ Plasma-Deutung getötet; ein Null-Befund (kein Trend) ist
ein gültiges Ergebnis (0 geehrt).

Datenkette unverändert: `galileo_atdf_compiler` (TRK-2-25 → GASR-Residuum-Serie)
→ `galileo_noise_geo` (korrigiert: ε-Bänder neben α, Median der Tages-RMS,
Bänder nur bei n ≥ 10). Residuum-Bestand: 14 077 825 Samples
(`sha256 2375b30944ed76b08fc50566a9e3eecfc2e4e5adabfeebbd155db3a128cbe783`),
361 (Mode, Tag)-Zellen, Modus-Klassifikation wie in v2 (1: 163 Tage / 2: 109 /
3: 89).

## n zuerst — ε-Band-Bevölkerung (n Tage je Mode, Bänder mit n ≥ 10)

| Mode | ε 0–30° | ε 30–60° | ε 60–90° | ε 90–120° | ε 120–150° | ε 150–180° |
|---|---|---|---|---|---|---|
| 1 | 133 | 12 | — | — | — | 10 |
| 2 | 77 | — | — | — | — | 16 |
| 3 | 60 | — | — | — | — | 15 |

Die mittleren ε-Bänder (30–150°) sind für Mode 2 und 3 **n-leer** (kein Band
erreicht n ≥ 10). Die ε-Kurve der kohärenten Modi trägt nur die zwei
Endbänder. Das ist die gemessene Grenze, kein Kurvenpunkt.

## Der Befund — ε-Kurve (Median der Tages-RMS, Hz; n Tage)

| Mode | ε 0–30° (Konjunktion) | ε 30–60° | ε 150–180° (Opposition) |
|---|---|---|---|
| 1 | 7,5 Hz (133) | 8,6 Hz (12) | 8,2 Hz (10) |
| 2 | **1,5 Hz (77)** | — | **42,0 Hz (16)** |

(Zum 1,5-Hz-Wert: ε am TDB-Tagesanfang über 75 Nicht-Lock-Tage misst 0,65 Hz;
die 1,5-Hz-Differenz hängt an zwei All-Lock-Tagen und der Stations-Pooling-
Metrik — siehe `befund-galileo-mode2-station-split`.)
| 3 | **4,9 Hz (60)** | — | **79,5 Hz (15)** |

Die ε- und α-Bänder sind nicht dieselben Tag-Mengen — α und ε sind für die
äußere Sonde komplementär, aber die Bandgrenzen wählen verschiedene Tage.
Mode 2 hält den Wert: α 150–180° = 1,5 Hz (73 Tage) ↔ ε 0–30° = 1,5 Hz
(77 Tage); die n verschieben sich um die Band-Rand-Tage. Mode 3 ändert den
Wert: α 150–180° = 6,7 Hz (56) ↔ ε 0–30° = 4,9 Hz (60) — die Konjunktions-Tage
auf ε tragen ein ruhigeres Residuum als die auf α eingeordneten. Die
Achsen-Korrektur benennt also nicht nur die Geometrie; wo die Bandgrenzen
andere Tage ziehen, ändert sie die Zahl mit.

**Die Richtung auf ε ist die direkte Inversion der Plasma-Erwartung.** Der
kohärente Kanal ist **leise bei kleinem ε** (Konjunktion: Mode 2 = 1,5 Hz,
Mode 3 = 4,9 Hz) und **laut bei großem ε** (Opposition: Mode 2 = 42,0 Hz,
Mode 3 = 79,5 Hz). Ein laut-bei-kleinem-ε — das einzige Muster, das die
Plasma-Deutung getragen hätte — tritt nicht auf. Die Plasma-Deutung ist damit
**getötet**, nicht nur unverifiziert.

**Der Fall überlebt nicht als Plasma-Elongationseffekt.** Die zwei besetzten
ε-Enden liegen überwiegend in denselben Tag-Mengen wie die Distanz-Extreme:
ε 0–30° ist eine Teilmenge der 5–6-AU-Tage (v2-Distanztabelle: Mode 2 96,
Mode 3 77 Tage), ε 150–180° die 0–2-AU-Cruise plus die Oppositions-Tage der
fernen Seite (1996-06, α 8° bei 5,19 AU). Das mittlere ε (30–150°) — der
Bereich, der die Achsen entwirrt hätte — ist unbesetzt. Die Distanz-/Ära-
Verwebung aus v2 wird durch diese Messung **nicht gebrochen**; der 28×-
(Mode 2, 42,0/1,5) und 16×-Fall (Mode 3, 79,5/4,9 auf ε) bleibt ein
Distanz-/Ära-Confound.

**Mode 1 bleibt auf ε flach über die besetzten Bänder**: 7,5 / 8,6 / 8,2 Hz
(ε 0–30° → 30–60° → 150–180°, Faktor 1,1×). Das ε-60–120°-Band (8 Tage,
22,91 Hz, Jupiter-Ära, im `befund-galileo-mode1-fingerabdruck`) fällt unter
die n≥10-Schwelle und trägt hier keinen Punkt — die Flachheit gilt über die
besetzten Bänder, nicht über die unbesetzte ε-Mitte.

## Grenzen

- Mittlere ε-Bänder 30–150° für Mode 2/3 n-leer (n < 10): die ε-Kurve trägt nur
  zwei Endpunkte, beide mit Distanz/Ära verwoben; die solare Achse ist auf
  diesen Daten nicht unabhängig von der Distanzachse prüfbar.
- Der Mode-2-Station-Tag-Split ist gemessen (`befund-galileo-mode2-station-split`):
  der 1,5-Hz-Wert ist fragil (All-Lock-Tag-Zählung + Stations-Pooling, Faktor
  1,1–2,3); die Konjunktions-Station-Tag-Mediane fallen auf 0,28–0,58 Hz, der
  Stations-Restboden ist metrik-abhängig (Pass-Ebene 0,3–1,2 Hz,
  `befund-galileo-pass-segmentierung`). Der Mode-2-Stärke-Split bleibt `pending`.
- α–Zeit–Sonnenzyklus-Verwebung (1990–97, Maximum → Minimum) aus v2 übernommen;
  die Oppositionstage liegen früh, die Konjunktionstage spät.

## Register-Satz

*Auf der ε-Achse ist der kohärente Fall invers zur Plasma-Erwartung gemessen:
leise bei Konjunktion (ε 0–30°: Mode 2 = 1,5 Hz / 77 Tage, Mode 3 = 4,9 Hz /
60), laut bei Opposition (ε 150–180°: Mode 2 = 42,0 Hz / 16, Mode 3 = 79,5 Hz /
15). Ein laut-bei-kleinem-ε tritt nicht auf — die Plasma-Deutung ist getötet.
Die ε-Endpunkte sind dieselben Distanz-/Ära-Extreme wie die α-Extreme; das
mittlere ε (30–150°) ist unbesetzt, die solare Achse auf diesen Daten nicht
unabhängig prüfbar. Mode 1 bleibt auf ε flach (7,5/8,6/8,2 Hz).*

## Status

`done` (Rat gehalten, 2026-09-05). Zeichnet die in v2 als `pending`
registrierte Mode-2/3-Kurve auf der ε-Achse: der kohärente Fall ist invers zur
Plasma-Erwartung, die Plasma-Deutung getötet. Der Mode-2-Station-Tag-Split ist
nachgemessen (`befund-galileo-mode2-station-split`): der 1,5-Hz-Wert ist fragil
(0,65 Hz über 75 Nicht-Lock-Tage ohne zwei All-Lock-Tage; Station-Tag-Konjunktion
0,28–0,58 Hz); ein einzelner Boden nicht getragen.
