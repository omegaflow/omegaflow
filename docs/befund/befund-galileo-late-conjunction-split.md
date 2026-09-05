<!--
  title: Befund — Galileo late-conjunction Station × Stärke-Split (1997)
  class: befund
  date: 2026-09-05
  sha256: 8bbc48a9be9888fff1eff303731a479804ef9caecd0d309fa610a1c94648e263
  status: done
  antwortet-auf: docs/befund/befund-galileo-alpha-zeit-sonnenzyklus.md docs/befund/befund-galileo-mode2-station-split.md
  see-also: docs/befund/befund-galileo-mode2-staerke-split.md docs/befund/befund-galileo-inpass-staerke-rampe.md
-->

# Befund: Galileo late-conjunction Station × Stärke-Split (1997) — die Ära-Stille ist Station-63-Starkzustand, Station 43 absent

## Frage & Bindung

Der α–Zeit–Sonnenzyklus-Befund (G4, done) misst die leise Konjunktion als nur in der späten
Ära beobachtbar (1997 Mode 2: 31 Tage, Median 0,3 Hz) und registriert den Station-Split dieser
späten Zellen als `pending`. Dieser Entwurf zerlegt das 1997er-ruhige Konjunktionsfenster
(ε < 30°, Kalenderjahr 1997) je Station (14/43/63) und je Stärke-Zustand (AGC-Boden −2560 vs
starkes Plateau ≥ −1900), um zu messen, welche Station und welcher Zustand die Ära-Stille trägt.

Gebunden: Moden 1/2/3 (Mode 2 im Fokus; 1/3 gemeldet, weil anwesend); Lock-Übergänge
(|resid| > 1000 Hz) vor dem Rauschen getrennt; Stärke-Zustand je Probe — Boden = `signal_strength`
≤ −2560 (AGC-Klemmwert), stark = ≥ −1900, s = 0 (Pad) und Zwischenwerte ausgeschlossen;
Rausch-Zelle = je (Tag, Station, Zustand), Zell-RMS um den Zell-Mittelwert, Zellen ≥ 30 Proben,
Median über die Zellen. Fenster = Kalenderjahr 1997 UND Elongation ε < 30° (Winkel an der Erde,
aus `galileo_daily` + `earth`-Barycentern, Tagesanfang). Additive Sonde
`tools/measure/src/bin/galileo_late_conj_split.rs`, Report auf stdout, `cargo check` 0/0 Warnings
(gemessen), nichts Bestehendes verändert. Der Mode-2-Bestand endet am 1997-02-25 — die 31
Fenstertage liegen 1997-01-01 .. 1997-02-25.

## n zuerst (0 geehrt)

Fenster-Tage mit Nicht-Lock-Aufzeichnung an 14/43/63: Mode 2 = 31, Mode 1 = 52, Mode 3 = 20.
Proben (Fenster, je Station, alle Zustände gepoolt): Mode 2 — Station 14: 139 817, Station 43:
34 060, Station 63: 315 481. Mode 1 — 1 002 458 / 1 688 276 / 911 039. Mode 3 — 106 714 /
91 936 / 2 807. Alle Zellen ≥ 30 Proben. Station 43 trägt im Mode-2-Fenster genau einen
Zell-Tag (1997-02-21, Boden, 22,183 Hz); Station 63 trägt 26 der 34 Fenster-Tag-Zellen.

## Tabelle 1 — Mode 2 (Zweiweg), Fenster 1997 ε < 30°: Median der (Tag, Station, Zustand)-Zell-RMS (Hz), n Zellen = n Tage

| Station | Boden n / med | stark n / med | alle Zustände n / med |
|---|---|---|---|
| 14 | 5 / 0,720 | 5 / 0,086 | 7 / 0,086 |
| 43 | 1 / 22,183 | 0 / absent | 1 / 22,183 |
| 63 | 13 / 2,452 | 19 / 0,123 | 26 / 0,277 |
| gepoolt 14/43/63 | 19 / 2,452 | 24 / 0,123 | 34 / 0,277 |

## Tabelle 2 — Mode 1 und Mode 3 (anwesend), Fenster 1997 ε < 30°

Mode 1 (52 Fenster-Tage): Station 14 Boden 2,075 (19) / stark 0,123 (34) / alle 0,273 (41);
Station 43 35,575 (24) / 0,122 (37) / 2,467 (44); Station 63 29,428 (20) / 0,121 (37) / 4,139 (48).
Gepoolt: Boden 15,079 (63), stark 0,122 (108), alle 1,251 (133).

Mode 3 (20 Fenster-Tage): Station 14 Boden 5,025 (10) / stark 0,072 (9) / alle 0,269 (14);
Station 43 29,981 (4) / 0,323 (4) / 0,758 (7); Station 63 Boden 59,221 (1) / stark absent / 59,221 (1).
Gepoolt: Boden 5,025 (15), stark 0,073 (13), alle 0,758 (22).

## Der Befund

**1. Die 1997er-Stille ist eine Station-63-Aufzeichnung.** Der gepoolte
(Tag, Station)-Median des Mode-2-Fensters misst 0,277 Hz über 34 Zellen (31 Tage) — er
reproduziert die 0,3 Hz (31 Tage) des G4-Blattes (Median-Konvention). Station 63 trägt 26 der 34
Zellen (alle-Zustände-Median 0,277 Hz) und deckt den Januar + Mitte Februar; Station 14 trägt
7 Februar-Zellen (Median 0,086 Hz, ruhiger, aber datendünn); Station 43 trägt genau eine
Boden-Zelle am 1997-02-21 (22,183 Hz). Der späte leise Boden ist damit in der
Aufzeichnungs-Dominanz eine Station-63-Zahl.

**2. Die Stille ist der starke Zustand, nicht der AGC-Boden — in jedem Mode und an jeder
Station.** Mode 2: starke Zustände 0,086–0,123 Hz (Station 14/63; Station 43 hat keinen
starken Fenster-Zell-Tag) gegen Boden 0,720–2,452 Hz (Faktor 4,4× an 14, ~20× an 63 gepoolt).
Mode 1: stark 0,121–0,123 Hz an allen drei Stationen gegen Boden 2,1–35,6 Hz. Mode 3: stark
0,072/0,323 Hz (14/43) gegen Boden 5,0–59,2 Hz. Die Richtung des In-Pass-/Stärke-Befunds
(Boden lauter als Plateau) wiederholt sich in der 1997er-Konjunktion exakt.

**3. Aber die Station-63-Tag-Komposition ist zustands-gemischt — die 0,277-Hz-Zahl ist eine
Mischung aus stillem Starkzustand und lauten Tagen.** Die starken Zellen von Station 63
(19) sind bimodal: 14 ruhig (0,050–0,276 Hz, die Januartage ε 1,7–17° und 02-03/04/11), 5 laut
(2,8–61,4 Hz: 01-10, 01-22, 01-23, 01-27, 02-07). Die Boden-Zellen von Station 63 (13) spannen
0,023–360 Hz: ruhig nur bei höherem ε (02-16 0,028, 02-22 0,023, 02-19 0,258, 02-20 0,163 Hz),
laut bei ε < 8° (01-22 360 Hz, 01-23 127 Hz, 01-28 160 Hz) und an 02-14/15/21 (22–29 Hz). Die
ruhigen Einzeltage des Fensters (0,02–0,14 Hz) sind überwiegend Starkzustands-Tage; die
Boden-Tage tragen den lauten Schwanz und heben den alle-Zustände-Median (0,277) auf das ~2,2-fache
des reinen Starkzustands-Medians (0,123). Der G4-Wert 0,3 Hz ist damit kein reiner
Starkzustands-Boden, sondern die Mischung einer von Station 63 getragenen,
starkzustands-dominierten Tag-Verteilung mit einem Boden-/Stark-Schwanz.

## Verdict

**Die 1997er-Konjunktions-Stille trägt Station 63** (26/34 Tag-Zellen; gepoolt 0,277 Hz =
G4 0,3 Hz), und ihr ruhiger Kern ist der **starke Zustand** (Mode 2 stark 0,086–0,123 Hz gegen
Boden 0,720–2,452 Hz; Mode 1 stark 0,12 Hz an allen Stationen; Mode 3 stark 0,07–0,32 Hz) — der
AGC-Boden ist auch in der späten Ära der laute Zustand, konsistent mit den In-Pass-/
Stärke-Befunden. Die Station-63-Aufzeichnung ist jedoch **zustands-gemischt**: 0,3 Hz ist keine
reine Starkzustands-Zahl, sondern der Median einer Mischung aus stillen Starkzustands-Tagen und
lauten Boden-Tagen (0,02–360 Hz). Station 43 ist im späten ruhigen Fenster **absent** (eine laute
Boden-Zelle am 1997-02-21); Station 14 ist ruhig (0,086 Hz) aber datendünn (7 Februar-Zellen).

## Grenzen

- Zell-Definition (Tag, Station, Zustand), ≥ 30 Proben, nur Stationen 14/43/63; die gepoolten
  Werte sind daher nicht 1:1 mit den G4-Tag-Medianen (station-gemischt, ohne Zell-Minimum,
  alle Stationen) identisch — Mode 2 reproduziert G4 (0,277 vs 0,3 Hz), Mode 1/3 weichen metrik-
  bedingt ab (Mode 3 gepoolt 0,758 gegen G4 0,3 Hz; Differenz benannt, nicht reconciliert).
- Station 43: 34 060 Fenster-Proben, aber nur ein Tag mit ≥ 30 Proben — die übrigen Tage sind
  dünn (< 30), absent, nicht null (0 geehrt). Starkzustand an Station 43 im Fenster: n = 0 Zellen.
- Der Zustands-Split ist statisch (Zustands-Assoziation je Tag), ohne Pass-Identität und ohne
  Richtung; die In-Pass-Kontrolle der lauten Tage (01-10/22/23/27, 02-07 an 63) ist nicht geführt.
- Geometrie je Tag einmal (Tagesanfang); Datenende 1997-02-25 — das "1997"-Fenster ist
  Jan/Feb, kein ganzes Jahr.
- Mode-1/3-Starkzustands-n im Fenster trägt den Befund für Mode 2; für Mode 3 an Station 63 ist
  der Starkzustand absent (1 Boden-Zelle).

## Register-Satz

*Die 1997er-Konjunktions-Stille (ε < 30°, Mode 2 0,277 Hz über 34 Tag-Zellen = G4 0,3 Hz über 31
Tage) trägt Station 63 (26/34 Zellen); ihr ruhiger Kern ist der starke Zustand (stark 0,086–0,123 Hz
gegen AGC-Boden 0,720–2,452 Hz, jeder Mode), aber die Station-63-Tag-Verteilung ist zustands-
gemischt — der Boden trägt den lauten Schwanz (0,02–360 Hz), Station 43 ist absent (eine laute
Boden-Zelle), Station 14 ruhig und datendünn (7 Februar-Zellen).*

## Status

`draft` — vorgelegt für die Orchestrierung; Probe additiv, `cargo check` 0/0, Report auf stdout,
kein Report-File, nichts Bestehendes verändert.
