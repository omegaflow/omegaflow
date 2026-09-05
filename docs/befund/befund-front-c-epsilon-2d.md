<!--
  title: Befund — P11-Konjunktions-Spitze (ε 0–10°, 8013 Hz) unter ε×Distanz×Ära-Entzerrung: Ären-Zelle, keine Konjunktions-Geometrie
  class: befund
  date: 2026-09-05
  sha256: 269b4202489312a14b8a512cd086615dc4b2e34759d8d91f3e6489473c78c5e0
  status: done
  antwortet-auf: docs/befund/befund-front-c-noise-vs-epsilon.md (offene ε×Distanz×Ära-2D-Entzerrung der P11-ε0-Spitze)
  see-also: docs/befund/befund-front-c-noise-vs-epsilon.md
-->

# Befund: P11 ε 0–10° = 8013 Hz — 2D-Entzerrung (Ära × Distanz × Station)

## Frage & Bindung

Der ε-Recheck (befund-front-c-noise-vs-epsilon) ließ offen, ob die P11-Konjunktions-
Spitze (ε 0–10°, 8013 Hz, n=43) eine echte Geometrie-Struktur (Sichtlinie nahe der
Sonne) oder ein Ären-/Stations-Kluster ist — die ε-Bänder mischen Ären. Dieses
Blatt zerlegt die 43 ε0-Tage nach Distanz, Ära (Kalenderjahr) und Station und
kontrolliert ihre Lautheit gegen Nicht-Konjunktions-Tage derselben Ära und
derselben Distanz.

Bindung: wiedergegeben wird nur, was das Werkzeug druckt — Median des
per-Day-Resid-RMS (Front-C-Metrik, ≥ 30 Residuen/Tag), n je Zelle. Metrik und
Tagesauswahl sind identisch zum ε-Recheck; die vier Reproduktions-Zellen
(P11 ε0–10 8013 Hz n43, ε10–20 4506 n150, d0–5 4248 n42, d5–10 5820 n754; P10
ε0–10 1487 n127) stimmen mit dem gedruckten Abdruck überein — die Entzerrung
rechnet auf denselben Tagen wie der Befund.

## Messung

Werkzeug-Lauf `cargo run -p omegaflow-measure --bin pioneer_navio_epsilon_2d`
(additives Messwerkzeug), 2026-09-05. Die 43 P11-ε0-Tage einzeln (Ledger):
Ära 1978 n=6 (1978-08-18…09-02, 7,47–7,54 AU), Ära 1979 n=23 (1979-08-31…
09-23, 9,36–9,38 AU), Ära 1980 n=14 (1980-09-26…10-11, 9,67–9,70 AU). Distanz:
7–8 AU n=6, 9–10 AU n=37. Stations-Mix über die Tage: Goldstone DSS 11/12/14,
Canberra DSS 42/43/44, Madrid DSS 61/63; der laute 1979-09-Block wurde von allen
drei Komplexen getrackt (drei-Wege-Anteil ≈ 30–60 %).

### 2D-Kontrolle: Konjunktions-Tage gegen Nicht-Konjunktions-Tage gleicher Ära × Distanz (P11)

| Kontrolle | n Tage mit Pool | conj med (Hz) | gematchte Nicht-Konj med (Hz) | conj lauter als eigener Pool |
|---|---|---|---|---|
| ±1 AU, gleiches Jahr, ε ≥ 10° | 43/43 | 8013 | 9958 | 31/43 |
| ±0,4 AU, gleiches Jahr, ε ≥ 30° | 43/43 | 8013 | 15379 | 21/43 |

### ε-Bänder innerhalb des Ära×Distanz-Fensters der Konjunktions-Tage (P11)

| Jahr (Fenster AU) | ε 0–10 | ε 10–30 | ε 30–60 | ε 60–90 | ε 90–120 | ε 120–150 | ε 150–180 |
|---|---|---|---|---|---|---|---|
| 1978 (6,8–8,2) | 5229 n6 | 4443 n27 | 3682 n38 | 4435 n24 | 1692 n23 | — | — |
| 1979 (8,7–10,1) | 11242 n23 | 7176 n32 | 7632 n65 | 29574 n34 | 15835 n31 | 12923 n17 | — |
| 1980 (9,0–10,4) | 7101 n14 | 8088 n34 | 5842 n59 | 7480 n53 | 4304 n35 | 3289 n50 | 3571 n55 |

### Ära-Profil und jährliches Geometrie-Minimum

P11, alle getrackten Tage je Jahr — n, min ε, Anzahl ε<10, Median-RMS:
1973 n4 minε 91,6° (0) 4700 Hz; 1974 n38 73,6° (0) 4111; 1975–77: keine Tage;
1978 n118 minε 4,8° (6) 2950; 1979 n302 minε 2,0° (23) 8891; 1980 n300 minε 7,7°
(14) 5425; 1981 n303 minε 11,7° (0) 6040; 1982 n241 minε 14,0° (0) 5560; 1983
n277 minε 15,1° (0) 2113; 1985 n88 16,2° (0) 996; 1986 n23 16,8° (0) 835; 1987
n128 15,9° (0) 1202; 1988 n167 16,4° (0) 1041; 1989 n112 15,7° (0) 1422; 1990
n98 28,6° (0) 1879.

P10-Kreuzprobe (ε0–10 n=127, med 1487): die P10-ε0-Tage sind nur in den Jahren
1979–82 laut (med 7890 n5 / 12047 n14 / 20586 n2 / 6925 n18, bei 18–28 AU), ab
1983 ruhig (2039 → 196 Hz, bei 30–66 AU). P10-2D-Kontrolle: ε0-Tage nicht lauter
als ihre Ära×Distanz-Nachbarn (gematcht ±0,4 AU ε≥30: 1588 Hz, lauter 61/126;
±1 AU: 1716 Hz, lauter 62/127). P10 bleibt in der Ekliptik: min ε < 10° in fast
jedem Jahr 1979–1996; sein ε0-Band ist trotzdem über die Mission ruhig.

## Befund

**1. Die 43 P11-ε0-Tage sind eine einzige Ära×Distanz-Zelle, kein Früh-Missions-
Kluster.** Die Zähl-Ähnlichkeit zum Distanz-Band 0–5 AU (n=42) war ein Zufall der
Zahlen, nicht dieselben Tage: die ε0-Tage liegen bei 7,47–9,70 AU in den Jahren
1978 (n6), 1979 (n23), 1980 (n14); die 42 Tage des 0–5-AU-Bandes sind die
Start-Ära 1973–74 (1–5 AU). Die ε0-Tage sind die jährlichen Konjunktionsfenster
der Saturn-Begegnungs-Ära.

**2. Die Konjunktions-Spitze überlebt die 2D-Kontrolle nicht.** Gegenüber
Nicht-Konjunktions-Tagen derselben Ära und derselben Distanz (±0,4 AU, ε≥30) sind
die Konjunktions-Tage nicht lauter — gematchter Median 15379 Hz ≥ Konjunktions-
Median 8013 Hz, und nur 21 von 43 ε0-Tagen sind lauter als ihr eigener Pool. Auch
die weite Kontrolle (±1 AU, ε≥10: 9958 Hz, 31/43) trägt keinen Konjunktions-
Überschuss. Innerhalb der Ära×Distanz-Fenster trennt ε nicht: 1979 ist das laute
Band ε 60–90° (29574 Hz), nicht die Konjunktion (11242 Hz); 1980 liegt die
Konjunktion (7101 Hz) unter ε 10–30 (8088 Hz) und ε 60–90 (7480 Hz). Die
Distanz-Ära-Zelle 1978–1980 bei 7–10 AU ist über alle ε laut.

**3. Kein Stations-Kluster.** Die lauten ε0-Tage wurden über alle drei
DSN-Komplexe getrackt (Goldstone 11/12/14, Canberra 42/43/44, Madrid 61/63), der
lauteste Block (1979-09) von allen dreien gleichzeitig mit Zwei-/Drei-Wege-Mix.
Die Nicht-Konjunktions-Tage derselben Zelle tragen denselben Stations-Mix und sind
gleich laut — die Zelle ist über Stationen laut, nicht durch eine Station.

**4. Warum ε<10 nur 1978–80 existiert — Geometrie und Datenlücken, gemessen.**
Nach der Saturn-Exkursion (1979-09) verlässt P11 die Ekliptik: das jährliche
ε-Minimum bleibt ab 1981 bei 11,7–16,8° (1990: 28,6°) — ε<10 ist geometrisch
nicht mehr möglich, die 43 Tage sind nicht „übersehene" späte Konjunktionen.
1975–77 trägt das Residuum keine qualifizierenden Tage (Datenlücke); die
1973/74-Tage (n 4/38) liegen außerhalb ihrer Konjunktionssaison (min ε 91,6°/73,6°).

**5. Die laute Ära ist kalendarisch, nicht distanz- oder winkelgebunden.**
Beide Sonden sind in denselben Kalenderjahren 1978–82 laut (P11-Missionsmedian
2950–8891 Hz bei 7–13 AU; P10 6204–7106 Hz bei 18–28 AU) und ab ~1983–85 ruhig
(P11 835–2113 Hz bei 13–35 AU; P10 365–1761 Hz bei 30–66 AU) — bei stark
verschiedenen heliozentrischen Distanzen je Ära. Die 1D-„Distanz"-Monotonie des
Front-C-Abdrucks (P11 5820 → 4578 → 993 Hz) ist der Schatten dieser Ären-Ordnung:
die P11-Distanzbänder 5–10 AU sind vollständig die laute Ära 1978–80 (7–10 AU),
der sub-kHz-Boden beginnt mit dem Ende der lauten Ära (~15 AU / ab 1984).

**6. Urteil.** Die P11-ε0-10°-Spitze (8013 Hz, n=43) ist **keine genuine
Konjunktions-Geometrie-Wirkung** — sie überlebt die ε×Distanz×Ära-Kontrolle nicht.
Sie ist eine **Ären-Zellen-Struktur**: die 43 Tage liegen vollständig in der
lauten Ära 1978–80 bei 7,5–9,7 AU, deren ganze Distanz-Ära-Zelle über alle ε laut
ist. Das „lauteste Band über jedem Distanz-Median" ist ein Artefakt der
1D-Randbildung: ε-Achse wie Distanz-Achse mischen die laute Ära 1978–82 mit
ruhigen Nachbarjahren; jede achsenweise Druckform erbt die Ären-Zelle. P10 —
ekliptikständig, ε<10 in fast jedem Jahr — bestätigt quer: seine Konjunktions-
Tage sind nur in derselben lauten Ära 1979–82 laut und nie lauter als ihre
Ära×Distanz-Nachbarn.

## Grenzen

(1) **Treiber der lauten Ära 1978–82 nicht zerlegt.** Kalendarisch messbar,
mechanisch offen: ein Sonnenwind-/Aktivitäts-Anteil (Sonnenzyklus-21-Maximum
≈ 1979–82) oder ein Begegnungs-/Betriebs-Anteil (Saturn-Vorbeiflug 1979-09,
Modell-Residuen) ist mit diesem Werkzeug nicht getrennt; die Solarwind-Daten
(omni2/f107-Bestände) existieren für einen direkten Test — `pending`. (2) Der
sub-kHz-Quiet-Zone-Befund (andere Metrik, Tagesmedian-Felder, `--zone`) ist
unberührt; Distanz ≠ Winkel ≠ Ära, die Zonen-Ergebnisse ändern sich durch dieses
Blatt nicht. (3) n=43 an der entscheidenden P11-Zelle, n=23 davon in 1979; die
Zelle ist eine Bahnpassage ohne zweite Realisierung — die Ären-Zuordnung stützt
sich auf die P10-Kreuzprobe und die 2D-Kontrolle, nicht auf eine P11-Wiederholung.

## Register-Satz

Die offene ε×Distanz×Ära-2D-Entzerrung der P11-Konjunktions-Spitze
(befund-front-c-noise-vs-epsilon) ist gemessen: die 43 ε0-Tage liegen vollständig
in einer Ära×Distanz-Zelle (1978–80, 7,47–9,70 AU, die Saturn-Begegnungs-Ära) und
sind gegen Nicht-Konjunktions-Tage gleicher Ära × ±0,4 AU nicht lauter
(gematchte Nicht-Konj 15379 Hz ≥ 8013 Hz; 21/43 lauter); die Spitze ist eine
Ären-Zellen-Struktur über alle ε (1979 lauteste ε-Zelle 60–90° = 29574 Hz), kein
Konjunktions-Geometrie-Effekt und kein Stations-Kluster (alle drei DSN-Komplexe);
beide Sonden sind in denselben Kalenderjahren 1978–82 bei stark verschiedenen
Distanzen laut (P11 7–13 AU, P10 18–28 AU) und ab ~1983–85 ruhig — die 1D-
„Distanz"-Monotonie des Front-C-Abdrucks ist der Schatten dieser Ären-Ordnung;
der Treiber der lauten Ära (Sonnenwind-/Zyklus vs Begegnungs-/Betriebs-Anteil)
bleibt pending.

## Status

`done` (Rat gehalten, 2026-09-05). Die ε0-10°-Spitze von P11 (8013 Hz, n=43) ist
unter ε×Distanz×Ära-Kontrolle gemessen: Ären-Zelle (1978–80, 7,5–9,7 AU), keine
Konjunktions-Geometrie, kein Stations-Kluster. Offen bleibt die Treiber-Zuordnung
der lauten Ära 1978–82 — `pending`.
