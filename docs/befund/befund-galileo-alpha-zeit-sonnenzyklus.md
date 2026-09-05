<!--
  title: Befund — Galileo α–Zeit–Sonnenzyklus: der kohärente Fall kollabiert unter Ära-Kontrolle (Ära-/Distanz-Confound, Rest ~2× datendünn)
  class: befund
  date: 2026-09-05
  version: 1
  sha256: b5c2bdd8872dc9afa4d1f987438908236475d7aa36141924628e1e00dc5bdabb
  status: done
  antwortet-auf: Grenzen „α–Zeit–Sonnenzyklus verwoben“ aus docs/befund/befund-galileo-rausch-kurve.md und docs/befund/befund-galileo-rausch-kurve-epsilon.md
  see-also: docs/befund/befund-galileo-rausch-kurve.md docs/befund/befund-galileo-rausch-kurve-epsilon.md docs/befund/befund-galileo-mode2-station-split.md
-->

# Befund: Galileo α–Zeit–Sonnenzyklus — der kohärente Fall kollabiert unter Ära-Kontrolle

## Frage & Bindung

Beide Vorgänger-Blätter (Rausch-Kurve v2, ε-Kurve) benennen dieselbe offene
Grenze: die α-niedrigen Bins (Opposition, ε 150–180°) sitzen früh, die
α-hohen (Konjunktion, ε 0–30°) spät; 1990–97 reicht vom Sonnenmaximum ins
Minimum. Dieser Lauf zerlegt die zwei verwobenen Achsen, wo die Daten es
zulassen. Frage: Ist der kohärente Fall (laut an Opposition, leise an
Konjunktion) Geometrie oder Ära/Sonnenzyklus?

Gebunden: n je (Geometrie-Regime, Ära)-Zelle **zuerst**; die Ära-Achse ist das
Kalenderdatum als gemessener Sonnenzyklus-Stellvertreter. Eine monatliche
SSN-/Sonnenflecken-Zahlreihe über 1990–97 liegt offline in `data/` **nicht vor**
(gemessene Abwesenheit — kein SSN-File, kein f107, kein OMNI im Bestand); die
bekannte Zyklus-22-Lage (Maximum ~1989–91, Minimum ~1996) ist Rahmen, nicht
Messung dieses Blatts. Die Distanzachse ist mit der Ära verwoben (Cruise ~1 AU
früh, Jupiter ~5–6 AU spät) und wird als solche benannt, nicht als dritte
unabhängige Achse verkauft.

Metrik unverändert zum Referenz-Befund: Median der Tages-RMS je (Mode, Tag),
Residuen gepoolt über Stationen, Lock-Übergangs-Samples (`|resid| > 1000 Hz`)
ausgeschlossen; Geometrie (ε an der Erde, α an der Sonne, heliozentrische AU)
je Tag aus `ephemeris_galileo_daily.bin` + `ephemeris_earth.bin` am
TDB-Tagesanfang. Additive Sonde: `galileo_era_cycle_probe.rs`. Bestand:
14 077 825 Residuen, 1 994 510 Lock-Samples ausgeschlossen; Tag-Zellen Mode 1:
163, Mode 2: 107, Mode 3: 87.

## n zuerst — (Regime, Jahr)-Zellen: n Tage und Median der Tages-RMS (Hz)

Regime ε 0–30° = Konjunktion (leises Ende), ε 150–180° = Opposition (lautes
Ende), ε 30–150° = Mitte. Ära-Zelle = Kalenderjahr.

| Mode 2 (Zweiweg) | 1990 | 1994 | 1995 | 1996 | 1997 |
|---|---|---|---|---|---|
| Opposition ε150–180 | **11, med 46,9** | — | — | **5, med 2,8** | — |
| Mitte ε30–150 | 2, med 60,9 | — | — | 14, med 14,9 | — |
| Konjunktion ε0–30 | — | 1, med 37,1 | 21, med 2,3 | 22, med 1,5 | 31, med 0,3 |

| Mode 3 (Dreiweg) | 1990 | 1994 | 1995 | 1996 | 1997 |
|---|---|---|---|---|---|
| Opposition ε150–180 | **11, med 93,8** | — | — | **4, med 16,1** | — |
| Mitte ε30–150 | 1, med 20,7 | — | — | 13, med 3,5 | — |
| Konjunktion ε0–30 | — | 1, med 0,2 | 21, med 8,0 | 16, med 6,7 | 20, med 0,3 |

| Mode 1 (Einweg) | 1990 | 1994 | 1995 | 1996 | 1997 |
|---|---|---|---|---|---|
| Opposition ε150–180 | 6, med 8,2 | — | — | 4, med 10,9 | — |
| Mitte ε30–150 | 2, med 260,0 | — | — | 17, med 15,1 | — |
| Konjunktion ε0–30 | — | 13, med 10,3 | 38, med 3,1 | 30, med 8,9 | 52, med 7,9 |

Parität mit den Vorgänger-Blättern (alle Ären gepoolt): Mode 2 Opposition
42,0 Hz (16 Tage) und Mode 3 Opposition 79,5 Hz (15) reproduzieren die
ε-Kurve exakt; Mode 2 Konjunktion misst hier 0,6 Hz über 75 Nicht-Lock-Tage —
die 1,5-Hz-Zahl des ε-Blatts hängt an zwei All-Lock-Tagen (dort registriert),
die Station-Split-Zahl 0,65 Hz über 75 Tage bestätigt dieselbe Bevölkerung.
Die Mode-3-Konjunktion dagegen summiert hier 58 Nicht-Lock-Tage (1+21+16+20) gegen 60 im
ε-Blatt und misst gepoolt 4,8 Hz gegen 4,9 Hz dort; anders als der Mode-2-Defizit (75 gegen 77,
dort als zwei All-Lock-Tage im ε-Blatt registriert) ist der Mode-3-Defizit von zwei Tagen nirgends
als All-Lock-Tage registriert — die Differenz ist damit benannt, nicht reconciliert (`pending`,
nicht geglättet).
Mode 1 Konjunktion 7,5 Hz (133), Opposition 8,2 Hz (10).

## Der Befund — drei gemessene Zerlegungen

**1. Unter fester Ära kollabiert der kohärente Kontrast.** Das Jahr 1996 ist
die einzige Ära, die für einen Mode beide Regime trägt — Opposition (fern,
5,19 AU) und Konjunktion (5–6 AU) im selben Minimum-Zeitfenster, Distanz und
Ära damit gehalten:

| Mode | gepoolt Opposition/Konjunktion | 1996, gleiche Ära | 1996, gleiche Distanz |
|---|---|---|---|
| 2 | 42,0 / 0,6 Hz ≈ **70×** | 2,8 / 1,5 Hz ≈ **1,9×** | 5,19 AU / 5–6 AU |
| 3 | 79,5 / 4,8 Hz ≈ **17×** | 16,1 / 6,7 Hz ≈ **2,4×** | 5,19 AU / 5–6 AU |

(Unter der ε-Befund-Metrik mit 1,5 Hz wäre der gepoolte Mode-2-Fall 28× —
die Kollapsrichtung ist dieselbe.) Der 70×/17×-Fall ist also ein
**Zwischen-Ären-Kontrast** (1990er-Cruise bei ~1 AU gegen 1995–97 bei 5–6 AU),
kein Geometrie-Kontrast bei fester Ära.

**2. Die laute Oppositions-Zahl überlebt die Versetzung in die späte Ära
nicht.** Alle Oppositionstage der kohärenten Modi liegen in zwei Fenstern:
11 Tage 1990-11-29…12-09 (Erd-Cruise, ~1 AU, α 0–1°) und 4–5 Tage 1996-06-26…
06-30 (fern, 5,19 AU, α 4–8°). Die Mediane derselben Geometrie-Band:

- Mode 2: 1990 **46,9 Hz** → 1996 **2,8 Hz** (Spearman Tages-RMS gegen Tag innerhalb
  Opposition −0,69 — über nur zwei Cluster: 11 Tage 1990 gegen 5 Tage 1996; der Wert misst den
  Ära-Abstand, keinen Trend innerhalb eines Regimes).
- Mode 3: 1990 **93,8 Hz** → 1996 **16,1 Hz** (−0,56, ebenso zwei Cluster: 11 Tage 1990 gegen
  4 Tage 1996 — Ära-Abstand, kein Regime-Trend).

Die ferne Opposition im Minimum sitzt auf dem Niveau des leisen Bodens ihrer
eigenen Ära — Mode 2 misst dort 2,8 Hz gegen 1,5 Hz der gleichzeitigen
Konjunktion. Die Tages-Zeilen zeigen zusätzlich, dass die 1996er Opposition
**bimodal** ist (Mode 2: 2,8 / 28,3 / 59,6 / 0,01 / 0,04 Hz über fünf Tage) —
nicht ein stabiler lauter Geometrie-Zustand.

**3. Die leise Konjunktion ist nur in der späten Ära beobachtbar.** Für alle
drei Modi existieren ε 0–30°-Tage erst ab 1994 (Mode 2: ein Einzel-Tag 37 Hz;
Mode 3: ein Einzel-Tag 0,2 Hz; dann 1995–97 dicht). Die Konjunktions-Stille
(1997 Mode 2: 0,3 Hz) ist damit **nicht ära-entwirrbar**: frühe
Konjunktions-Zellen fehlen (n = 0 vor 1994). Innerhalb der späten Ära ist der
Konjunktions-Trend schwach abfallend (Mode 2 −0,13, Mode 3 −0,28, Mode 1
−0,00 über die Tages-RMS gegen Tag) — kein Beleg für eine Sonnenzyklus-Bewegung
im leisen Regime, aber auch kein Ausschluss.

**Mode 1 bleibt die flache Referenz** — auch unter Ära-Kontrolle: 1996
Opposition 10,9 Hz (4 Tage) gegen Konjunktion 8,9 Hz (30), Faktor 1,2×,
identisch zur gepoolten Flachheit (8,2/7,5). Der 1990er-Cruise-Lärm (Mode 2/3
47–94 Hz an Opposition) tritt in Mode 1 an denselben Tagen **nicht** auf
(8,2 Hz) — die lauten Cruise-Tage sind ein kohärent-Kanal-Phänomen.

## Verdict

**Ära-/Sonnenzyklus-Confound — der kohärente Fall überlebt die Ära-Kontrolle
nicht.** Unter fester Ära (1996, zugleich gleiche Distanz 5–6 AU) schrumpft
der Kontrast von 70×/17× (gepoolt) auf 1,9×/2,4×. Die laute
Oppositions-Seite des Falls (42/79 Hz) wird vollständig von den 11
Cruise-Tagen 1990 (Sonnenmaximum-Ära, ~1 AU) getragen; dieselbe
Geometrie-Band im Minimum misst 2,8/16,1 Hz — auf dem Niveau des leisen
Bodens ihrer Ära. Ein reiner Geometrie-Effekt in der gemessenen Stärke wird
nicht getragen. Ein Rest-Kontrast ≤ ~2,4× in der einen Ära, die beide Regime
besucht (1996), bleibt als möglicher kleiner Geometrie-Anteil stehen — aber
datendünn (ein 4–5-Tage-Fenster). Die Plasma-Todes-Richtung (kein laut-bei-kleinem-ε) überlebt
die Ära-Kontrolle; nur die Magnitude des kohärenten Falls kollabiert (Ära/Sonnenzyklus-konfundiert).

**Nicht besetzbare Zellen (0 geehrt):** Opposition (ε 150–180°) in 1991–1995
n = 0 für alle Modi; Konjunktion (ε 0–30°) in 1990–1993 n = 0 für alle Modi.
Die ära-haltende Geometrie-Prüfung existiert damit nur im Jahr 1996 (ein
Oppositions-Fenster, n = 4–5). Ob Konjunktion im Maximum ebenso leise wäre,
ist auf diesen Daten nicht messbar; ob Opposition 1991–95 laut wäre, ebenfalls
nicht.

## Grenzen

- Ära-Stellvertreter ist das Kalenderdatum; keine monatliche SSN-Serie offline
  (gemessen abwesend). Die Distanzachse bleibt mit der Ära verwoben; die
  1996er-Zerlegung hält beide zugleich, die frühe Ära nicht.
- Die 1996er-Zelle trägt ein einziges 5-Tage-Oppositions-Fenster; die
  Bimodalität der Tages-RMS dort (0,01–60 Hz) ist nicht erklärt und begrenzt
  die Aussagekraft jedes Mittelwerts dieser Zelle.
- Station-Pooling wie im Referenz-Befund; ein Stations-Split der späten
  Konjunktions-Zellen ist im `befund-galileo-mode2-station-split` als `pending`
  registriert.
- Die 1994er-Einzel-Tage (Mode 2 Konjunktion 37 Hz) sind zu dünn für einen
  Trendpunkt.

## Register-Satz

*Unter fester Ära (1996, gleiche Distanz) schrumpft der kohärente Kontrast von
70×/17× auf 1,9×/2,4×: die laute Oppositions-Zahl ist ein 1990er-Cruise-Wert
(Sonnenmaximum-Ära, ~1 AU), die ferne Opposition im Minimum misst 2,8/16,1 Hz
und liegt auf dem Niveau des leisen Bodens ihrer Ära. Der kohärente Fall ist
ein Ära-/Distanz-Confound, kein überlebender Geometrie-Effekt; ein Rest
≤ ~2,4× steht datendünn (ein Fenster). Opposition 1991–95 und Konjunktion
1990–93 sind unbesetzt (n = 0).*

## Status

`done` (vom Rat gehalten; die Mode-3-Konjunktions-Tage-Differenz und der Zwei-Cluster-Spearman
sind benannt). Zerlegt die aus beiden Vorgänger-Blättern übernommene
α–Zeit–Sonnenzyklus-Verwebung, wo die Daten es zulassen; der einzige
ära-haltende Vergleich (1996) kollabiert den Fall. Mode 2/3-Kurven auf der
ε-Achse und der Station-Split bleiben wie registriert `pending`.
