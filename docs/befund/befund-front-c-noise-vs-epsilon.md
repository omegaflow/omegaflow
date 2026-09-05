<!--
  title: Befund — Pioneer-Rauschen auf der ε-Achse: Front-C-Urteil „Distanz, nicht Winkel" — Distanz reproduziert, Winkel-Aussage auf ε präzisiert (P11-Konjunktions-Spitze, P10 anti-Plasma)
  class: befund
  date: 2026-09-05
  sha256: b35a607039b4bc09436936193825cc1fd6bdee1806bbb86569f327201a113172
  status: done
  antwortet-auf: docs/auftrag/auftrag-dunkle-materie-front-c.md (Commit 0932cae, Rausch-Räumliche Verortung) docs/TODO.md (ε-Achsen-Pflicht der Pioneer-Quiet-Zone-Rezept-Blätter)
  see-also: docs/befund/befund-galileo-rausch-kurve-epsilon.md docs/befund/befund-galileo-rausch-kurve.md
-->

# Befund: Pioneer-Rauschen auf der ε-Achse — Front C nachgemessen

## Frage & Bindung

Front C (`auftrag-dunkle-materie-front-c`, Commit 0932cae, Werkzeug
`pioneer_navio_noise_geo`) legte das per-Day-Resid-RMS der Pioneer-NAVIO-Residuen
gegen die heliozentrische Distanz und gegen einen als „SEP" benannten Winkel aus
den Horizons-Ephemeriden und befand: das Rauschen fällt MONOTON mit der Distanz
(P10 7412 Hz bei 25 AU → 651–805 Hz bei 60–65 AU, ~10×; P11 5820 Hz bei 5–10 AU →
993–1181 Hz bei 15–30 AU), die SEP-Korrelation sei schwach — **„Distanz, nicht
Winkel, ist der Treiber."**

Die Register-Korrektur (TODO.md, 2026-09-05) hat benannt: der gemessene Winkel
war **α (Winkel am Sonnenort**, Erde–Sonne–Sonde), nicht **ε (solare Elongation,
Winkel an der Erde**, Sonne–Erde–Sonde). Für die äußere Sonde sind α und ε
komplementär. Der physikalisch tragende Winkel der Sichtlinien-Plasma-Szintillation
ist ε — der Stoßparameter der Sichtlinie ist b ≈ 1 AU·sin ε: kleines ε = Konjunktion
= Sichtlinie nah an der Sonne. Das korrigierte Werkzeug druckt heute α-, ε- und
Distanz-Bänder je Sonde. Dieser Entwurf legt die ε-Kurve gegen das Front-C-Urteil.

Bindung: Es wird nur wiedergegeben, was das Werkzeug druckt — der Median des
per-Day-Resid-RMS je Band, mit n (Bänder n ≥ 10). Keine Kausal-Aussage; die
Distanz/Ära-Kovarianz bleibt offen, wie Front C sie selbst benannt hat (0932cae).

## n zuerst

P10: 2904 Tage (per-Day-Resid-RMS, ≥ 30 Residuen/Tag). P11: 2201 Tage. Alle Bänder
unten tragen n. Klein-n-Extreme: P11 ε 0–10° n=43, ε 170–180° n=32, α 170–180° n=36;
P10 ε 160–170° n=179, α 0–10° n=181. Offene Lücke (nicht übertüncht): P10 α 0–10°
(n=181) hat im gedruckten ε-Band kein 170–180°-Gegenstück — das Werkzeug druckt
ε 160–170° als äußerstes P10-ε-Band; ob ε 170–180° für P10 leer ist, bleibt als
`pending` benannt (2D-Entzerrung).

## Messung

Werkzeug-Lauf `cargo run -p omegaflow-measure --bin pioneer_navio_noise_geo`,
2026-09-05, unveränderter Baum.

### Pioneer 10 — Distanz-, α- und ε-Bänder

| Distanz [AU) | n | med RMS (Hz) | α [°) | n | med RMS (Hz) | ε [°) | n | med RMS (Hz) |
|---|---|---|---|---|---|---|---|---|
| 0–5 | 11 | 8782 | 0–10 | 181 | 2073 | 0–10 | 127 | 1487 |
| 5–10 | 46 | 5994 | 10–20 | 179 | 1695 | 10–20 | 138 | 1632 |
| 15–20 | 18 | 7059 | 20–30 | 148 | 1725 | 20–30 | 140 | 1753 |
| 20–25 | 362 | 7412 | 30–40 | 164 | 1814 | 30–40 | 152 | 1900 |
| 25–30 | 399 | 5079 | 40–50 | 163 | 1744 | 40–50 | 170 | 1580 |
| 30–35 | 201 | 1496 | 50–60 | 181 | 1598 | 50–60 | 162 | 1579 |
| 35–40 | 84 | 1014 | 60–70 | 176 | 1666 | 60–70 | 154 | 1703 |
| 40–45 | 408 | 1487 | 70–80 | 177 | 1264 | 70–80 | 175 | 1439 |
| 45–50 | 408 | 1683 | 80–90 | 175 | 1412 | 80–90 | 170 | 1401 |
| 50–55 | 138 | 870 | 90–100 | 176 | 1462 | 90–100 | 172 | 1327 |
| 55–60 | 326 | 1021 | 100–110 | 169 | 1457 | 100–110 | 176 | 1433 |
| 60–65 | 306 | 651 | 110–120 | 160 | 1639 | 110–120 | 183 | 1577 |
| 65–70 | 188 | 805 | 120–130 | 159 | 1516 | 120–130 | 156 | 1744 |
|  |  |  | 130–140 | 157 | 1528 | 130–140 | 161 | 1812 |
|  |  |  | 140–150 | 145 | 1860 | 140–150 | 149 | 1675 |
|  |  |  | 150–160 | 139 | 1841 | 150–160 | 170 | 1695 |
|  |  |  | 160–170 | 138 | 1632 | 160–170 | 179 | 2073 |

### Pioneer 11 — Distanz-, α- und ε-Bänder

| Distanz [AU) | n | med RMS (Hz) | α [°) | n | med RMS (Hz) | ε [°) | n | med RMS (Hz) |
|---|---|---|---|---|---|---|---|---|
| 0–5 | 42 | 4248 | 0–10 | 41 | 3327 | 0–10 | 43 | **8013** |
| 5–10 | 754 | 5820 | 10–20 | 200 | 3535 | 10–20 | 150 | 4506 |
| 10–15 | 781 | 4578 | 20–30 | 178 | 2866 | 20–30 | 91 | 2963 |
| 15–20 | 98 | 993 | 30–40 | 151 | 2807 | 30–40 | 115 | 3379 |
| 20–25 | 158 | 1181 | 40–50 | 133 | 2659 | 40–50 | 123 | 3975 |
| 25–30 | 273 | 1166 | 50–60 | 146 | 2751 | 50–60 | 131 | 2872 |
| 30–35 | 95 | 1879 | 60–70 | 139 | 2744 | 60–70 | 112 | 4855 |
|  |  |  | 70–80 | 128 | 3857 | 70–80 | 122 | 4435 |
|  |  |  | 80–90 | 129 | 3709 | 80–90 | 132 | 4374 |
|  |  |  | 90–100 | 126 | 4981 | 90–100 | 135 | 3629 |
|  |  |  | 100–110 | 115 | 3765 | 100–110 | 121 | 3685 |
|  |  |  | 110–120 | 111 | 4065 | 110–120 | 136 | 2579 |
|  |  |  | 120–130 | 125 | 3023 | 120–130 | 138 | 2735 |
|  |  |  | 130–140 | 116 | 3661 | 130–140 | 132 | 2659 |
|  |  |  | 140–150 | 101 | 2963 | 140–150 | 143 | 2954 |
|  |  |  | 150–160 | 86 | 3328 | 150–160 | 172 | 2866 |
|  |  |  | 160–170 | 140 | 4760 | 160–170 | 173 | 3434 |
|  |  |  | 170–180 | 36 | **8013** | 170–180 | 32 | 3327 |

## Befund

**1. Distanz-Trend — reproduziert (Front-C-Zahlen unverändert).** P10 fällt von
7412 Hz (20–25 AU, n=362) auf 651 Hz (60–65 AU, n=306) und 805 Hz (65–70 AU,
n=188) — 11,4×; die Zwischen-Werte (1496 bei 30–35 AU, 1014 bei 35–40 AU, 870/1021
bei 50–60 AU) tragen die Monotonie mit einer mittleren Aufwölbung bei 40–45 AU
(1487/1683). P11 fällt von 5820 Hz (5–10 AU, n=754) auf 993–1181 Hz (15–30 AU,
n=98/158/273) — 4,9–5,9×. Die Distanz-Achse trägt die monotone Struktur beider
Sonden. Die ε- und α-Bänder mischen Tage über alle Distanzen/Ären (ε oszilliert
jährlich mit der Erdbahn, die Distanz wächst monoton mit der Ära), die
Distanz-Bänder sind fast ära-kontiguiert.

**2. α-Achse (die alte, als „SEP" fehlbenannte Achse) — schwach, reproduziert.**
P10: alle α-Bänder zwischen 1264 und 2073 Hz, keine Monotonie, α-Spanne 1,6×.
P11: alle α-Bänder zwischen 2659 und 4981 Hz außer dem Extrem α 170–180° = 8013 Hz
(n=36); keine Monotonie, α-Spanne ohne Extrem 1,9×. Front C las genau diese
Streuung als „SEP-Korrelation schwach".

**3. ε-Achse (korrigierte Achse) — kein monotoner Gang, aber eine sonden-
spezifische Extrem-Struktur.**
P10: ε-Bänder flach zwischen 1327 und 2073 Hz (Spanne 1,6×); das Minimum liegt bei
ε 90–100° (1327 Hz, n=172), das Maximum bei ε 160–170° (**Opposition**, 2073 Hz,
n=179); das Konjunktions-Ende ε 0–10° ist mittel (1487 Hz, n=127), nicht laut.
P10 zeigt auf ε **keine** laut-an-Konjunktion-Struktur; die lauteste Form sitzt an
der Opposition — die anti-Plasma-Form, die der Galileo-kohärente Kanal auf seiner
ε-Achse zeigt (`befund-galileo-rausch-kurve-epsilon`).
P11: das Konjunktions-Extrem **ε 0–10° = 8013 Hz (n=43)** ist das lauteste aller
P11-Bänder — lauter als jeder Distanz-Band-Median (5–10 AU 5820 Hz, 10–15 AU
4578 Hz). ε 10–20° = 4506 Hz (n=150), ε 20–30° = 2963 Hz (n=91); zwischen 30° und
170° oszilliert die Kurve nicht-monoton zwischen 2579 und 4855 Hz. Die einzige
starke ε-Struktur ist die Konjunktions-Spitze am Extrem; ein monotoner ε-Gang
existiert nicht.

**4. Komplementäre Geometrie (α+ε≈180°), Mediane gleich; Tag-Mengen nicht
identisch (n differiert).** Die sauberen Komplement-Paare tragen gleiche Mediane:
P11 α 170–180°/ε 0–10° = Konjunktion, beide 8013 Hz; P11 α 0–10°/ε 170–180° =
Opposition, beide 3327 Hz. Für P10 gilt **keine** Komplementaritäts-Aussage: die
lauteste P10-ε-Bande (160–170°, 2073 Hz, Opposition-nahe) hat als exaktes
Komplement α 10–20° = 1695 Hz — das paart nicht. Die von Front C als Streuung
gelesene laute P11-Bande war, unter der Fehlbenennung, „SEP 170°" — als nahe der
Opposition laut gelesen und unter der Plasma-Lesart physikalisch sinnlos
verworfen. Auf der korrigierten Achse ist dieselbe Bande die **Konjunktions-
Spitze (ε 0°) — die plasma-tragende Richtung** (kleiner Stoßparameter
b ≈ 1 AU·sin ε).

**5. Urteil über „Distanz, nicht Winkel, ist der Treiber".** Der Satz gilt auf ε
für den monotonen Gang — er ist **präzisiert, nicht gekippt**: der Distanz-Anteil
hält (die monotone Struktur ab 20–25 AU, mit mittlerer Aufwölbung bei 40–45 AU,
lebt auf der Distanz-Achse, in Front-C-Zahlen reproduziert). Der Winkel-Anteil
der Formulierung („SEP-Korrelation schwach / Winkel nicht der Treiber") ist auf ε
**lokal präzisiert**: P11 trägt eine echte Konjunktions-Extrem-Bande
(ε 0–10°, 8013 Hz, n=43), die über jedem Distanz-Median liegt — die α-Achse hatte
sie als wertlose 170°-Streuung getarnt. P10 trägt diese Spitze nicht (Konjunktion
mittel, Opposition laut). Beide Sonden zusammen ergeben **keinen globalen
ε-Treiber** und **keine universelle Plasma-Signatur**; P11s Konjunktions-Spitze
ist eine lokale, sonden-spezifische Struktur, P10s Opposition-Spitze eine zweite,
entgegengesetzte. Lokal zeigt der Winkel eine Information, die die Distanz-Achse
nicht trägt — ob die P11-Spitze Plasma oder Ära ist, entscheidet erst die
ε×Distanz×Ära-Entzerrung (pending).

## Grenzen

(1) **ε × Distanz × Ära nicht entzerrt.** Das Werkzeug druckt jede Achse separat,
keine 2D-Zellen. Die P11-ε0-Tage (n=43) zählen fast identisch zur Distanz-Bande
0–5 AU (n=42) — konsistent mit einem Früh-Missions-/Ären-Kluster in
Konjunktions-Geometrie, genau das, was die 2D-Entzerrung testet; ihre
Distanz-/Era-Verteilung druckt das Werkzeug nicht. Die ε-Bänder mischen Ären, die
Distanz-Bänder nicht — der ε-Test ist im Abdruck verdünnt (Ära-Varianz überwiegt),
die P10-Flachheit ist deshalb ein schwächerer Gegenbefund, als sie aussieht. (2) Die Konjunktions-Spitze ist eine
Geometrie-Korrelation, kein Plasma-Beweis — die Iess-Plasma-Zuordnung bleibt
Argument, wie Front C sie selbst eingeschränkt hat. (3) n=43 an der
entscheidenden P11-Bande. (4) Metrik = per-Day-Resid-RMS (Front-C-Metrik); die
sub-Hz-Zonen-Metrik der Quiet-Zone-Kette (Tagesmedian-Felder, `--zone`) ist
unberührt — Distanz ≠ Winkel, die Zonen-Ergebnisse (Quiet-Zone, sub-Hz-Boden,
leeres Netz) ändern sich durch diesen Befund nicht.

## Register-Satz

Die ε-Nachmessung der Front-C-Rausch-Verortung (TODO.md, ε-Achsen-Pflicht für die
Pioneer-Quiet-Zone-Rezept-Blätter, `pioneer_navio_noise_geo` α-benannt + ε-ergänzt)
ist gemessen: der Front-C-Distanz-Befund reproduziert sich (P10 7412 → 651–805 Hz,
P11 5820 → 993–1181 Hz), die „Winkel nicht der Treiber"-Aussage gilt auf ε nur
global (kein monotoner ε-Gang), nicht lokal — P11 ε 0–10° = 8013 Hz (n=43,
Konjunktion) ist das lauteste P11-Band und liegt über jedem Distanz-Median, P10 ist
auf ε flach mit anti-Plasma-Form (Opposition 2073 Hz lauter als Konjunktion
1487 Hz); die ε×Distanz×Ära-Entzerrung (2D-Zellen) bleibt pending und trennt erst
die P11-Konjunktions-Spitze von einer Ära-Kovariate.

## Status

`done` (Rat gehalten, 2026-09-05). Der ε-Recheck der Front-C-Rausch-Verortung ist
gemessen: Distanz reproduziert, Winkel-Aussage auf ε präzisiert. Offen bleibt die
ε×Distanz×Ära-2D-Entzerrung der P11-Konjunktions-Spitze (ε 0–10°, 8013 Hz, n=43)
— `pending`.
