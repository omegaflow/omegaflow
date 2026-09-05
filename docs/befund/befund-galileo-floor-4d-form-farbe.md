# Befund — Galileo AGC-Floor: 4D-Form (Weltlinien-Hülle) und Farbe (PSD-Steigung) — der Floor ist eine Fern-Schalen-Erscheinung, seine Lautheit keine 4D-Feld-Eigenschaft, seine Farbe nahe-weiß wie das Plateau

Messung ausschließlich mit der additiven Sonde `tools/measure/src/bin/galileo_floor_4d.rs`
(neu, einzige Repo-Änderung; `cargo check` 0/0). Vollständige Zell-Tabellen:
`/tmp/opencode/galileo_floor_4d_report.txt`. Daten: `data/galileo_resid.bin` (GASR),
Geometrie je Tag aus `galileo_daily`/`earth`-Ephemeride (baryzentrisch, ICRS, AU).

Bindung: Modus 1 primär (Floor-Träger), Modus 2 sekundär; Lock-Übergänge (|resid| > 1000 Hz)
vor dem Rauschen getrennt; Stärke 0 nie klassiert. Klassen: Boden = Stärke exakt −2560
(AGC-Klemmwert); stark = Stärke ≥ −1750. Rausch-Zelle = (Tag, Station, Klasse)-RMS um den
Zellen-Mittelwert (wie die Vorgänger-Blätter). laut = Zell-RMS ≥ 1 Hz. Geometrie am
TDB-Tagesanfang: r = heliozentrischer |p|, eps = Elongation an der Erde (Sonne-Sonde), alpha
= Winkel an der Sonne, ICRS x/y/z in AU. Farbe = PSD-Steigung (log10 PSD vs log10 f) über
Mode-1-resid bei ~1-s-Kadenz, je (Station, Klasse, Fenster), zusammenhängende Läufe,
DFT je Lauf-Abschnitt (linear-detrended und mean-subtrahiert), log10-PSD in 0.1-Dekaden-Bins
gepoolt, LSQ über Bins mit ≥ 20 Zählungen; Band untere Kante durch den längsten Lauf.

## n zuerst (0 geehrt)

| Klasse | Modus | Proben | Zellen (Tag,Station) | Tage | Spanne | laut (≥1 Hz) | Median Zell-RMS |
|---|---|---|---|---|---|---|---|
| Boden | 1 | 2 100 971 | 268 | 113 | 1994-12-18 .. 1997-02-28 | 143 | 1,125 Hz |
| stark | 1 | 3 643 700 | 262 | 115 | 1990-11-29 .. 1997-02-28 | 89 | 0,130 Hz |
| Boden | 2 | 1 879 516 | 124 | 75 | 1995-11-23 .. 1997-02-25 | 60 | 0,867 Hz |
| stark | 2 | 706 555 | 104 | 65 | 1990-11-29 .. 1997-02-21 | 55 | 2,495 Hz |

## Form — die 4D-Hülle (gemessen)

### 1. Der Boden ist eine Fern-Schalen-Erscheinung: r ≥ ~4,99 AU, nie auf dem Erd-nahen Weltlinien-Segment

Modus 1: 267 von 268 Boden-Zellen liegen im heliozentrischen r-Bin [5,0, 5,5) AU, die 268.
Zelle (die einzige 1994-12-Boden-Zelle, r ≈ 4,99–5,00 AU) im Bin [4,5, 5,0). Modus 2: alle
124 Boden-Zellen in [5,0, 5,5). **Keine Boden-Zelle existiert unter r = 4,99 AU.** Die
Erd-nahe Weltlinie (1990-11/12, r ≈ 0,98–1,5 AU) trägt in beiden Modi Proben, aber nur den
starken Zustand — dort ist der starke Zustand laut (Modus 1 r 0,5–1,0: 4 Zellen/3 Tage, Median
26,6 Hz, 4 laut; Modus 2 r 0,5–1,0: 15 Zellen/5 Tage 13,2 Hz, 14 laut; r 1,0–1,5: 25 Zellen/8
Tage 39,2 Hz, 23 laut). Die r-Achse ist entlang der einen Weltlinie mit der Zeit monoton
verwoben; das Weltlinien-Segment 1,5–4,9 AU ist in beiden Modi unbesetzt (n = 0, 1991–1994-11)
— der Schalenrand unter 4,99 AU ist damit nicht als Abwesenheit messbar, nur die Präsenz ab
4,99 AU und die Abwesenheit auf dem Erd-nahen Segment (0 geehrt).

### 2. Innerhalb der Schale ist der laute Boden nicht räumlich/geometrisch lokalisiert — Lautheit ist Zellen-/Epochen-gebunden, nicht 4D-Feld

Boden-Eps-Verteilung (Modus 1): Konjunktion ε<30° 209 Zellen/91 Tage/Median 1,119 Hz/113 laut;
ε 30–90° 37 Zellen/13 Tage/0,662 Hz/17 laut; ε 90–150° 11 Zellen/5 Tage/32,1 Hz/9 laut;
Opposition ε>150° 11 Zellen/4 Tage/**0,068 Hz**/4 laut. Der laute Boden besetzt also Konjunktion
und beide mittleren Bänder (139 von 143 lauten Zellen bei ε<150°), die Opposition ist im Modus 1
überwiegend ruhig (Median 0,068 Hz, 4 laute Zellen im 1996-06-Oppositions-Fenster). Modus 2
dagegen ist an Opposition laut (9 Zellen/5 Tage, 11,3 Hz, 6 laut, 1996-06).

**Entscheidend (gemessen): Dieselbe 4D-Position liest an verschiedenen Stationen um Größenordnungen
verschieden.** Gleicher Tag, gleiche ICRS-Position, gleiche Geometrie (r 5,272 AU, ε 20,4°, α
155,9°, ICRS −0,433/−4,831/−2,065 AU), 1995-11-24 Modus 1: Station 43 Boden-RMS **0,031 Hz**
(39 991 s), Station 14 **25,85 Hz** (7 463 s), Station 63 0,091 Hz (5 847 s). 1996-06-26 (r
5,191 AU, ε 169,9°, Opposition): Station 14 0,021 Hz (19 083 s), Station 43 0,021 Hz (25 792 s),
Station 63 **20,6 Hz** (24 201 s). Der Boden-Zustand selbst (die −2560-Klemmung) ist damit eine
Fern-Schalen-Erscheinung; seine *Lautheit* ist keine 4D-Feld-Größe (keine Schale/Kegel/
Elongations-Region), sondern Station+Tag-gebunden (Empfänger-/Pass-Zustand) innerhalb derselben
Schale — 0,02–500 Hz an identischer Position.

### 3. Boden vs stark in derselben Schale: 8,8–9,1× Median-Zell-RMS

In der gemeinsamen Schale (r 5,0–5,5, Modus 1): Boden 267 Zellen/Median 1,125 Hz gegen stark 242
Zellen/Median 0,124 Hz → Faktor **9,1×**; auf der Konjunktions-Geometrie (ε<30°): 1,119 gegen
0,127 Hz → **8,8×**. Der starke Zustand ist in derselben Schale in den meisten Monaten ruhig
(0,05–0,13 Hz); seine lauten Zellen (Modus 1, 89) liegen teils auf dem Erd-nahen Segment (1990-12,
r ≈ 0,98–1,03 AU, ε 155–161°) und teils in der Schale selbst (1994-12, r ≈ 5,0 AU, ε 11–21°,
5–240 Hz — die ersten fernen Verfolgungstage *vor* dem Boden-Einsatz; dazu 1995-12/1996-01,
Modus 2 1996-06-Opposition 200 Hz).

### 4. Epochen-/Monats-Struktur des Bodens (Modus 1, Zell-RMS-Median / laut von Zellen)

1994-12 1 Zelle 0,032 Hz (ruhig) · 1995-11 17 Zellen/0,112 Hz/3 laut (ruhig, bis auf Station 14
ab 11-24) · 1995-12 27 Zellen/0,269 Hz/13 laut · 1996-01 2 Zellen/564 Hz (n klein) · 1996-06
11 Zellen/0,068 Hz/4 laut · 1996-09 11 Zellen/32,1 Hz/9 laut · 1996-11 31 Zellen/1,73 Hz/16 laut
· 1996-12 42 Zellen/0,73 Hz/19 laut · 1997-01 70 Zellen/15,7 Hz/48 laut · 1997-02 56
Zellen/1,06 Hz/30 laut. Der ruhige Boden (1994-12, 1995-11, 1996-06-Modus 1) liegt in derselben
Schale und Geometrie wie der laute — die Boden-Ruhig/laut-Trennung ist Monats-/Stations-gebunden,
nicht räumlich.

## Farbe — die PSD-Steigung (gemessen, Modus 1)

| Station | Klasse | Fenster | n Läufe (max Lauf) | Steigung detrend ± SE (r2, Bins, Band) | Steigung mean-subtr. | log10 PSD @0,01 / @0,1 Hz |
|---|---|---|---|---|---|---|
| 14 | Boden | laut 1996-06..97-02 | 122 793 (26 622 s) | **−0,490** ± 0,028 (0,91, 30, 4,8e-4..0,448 Hz) | −0,531 | −2,08 / −2,24 |
| 43 | Boden | laut | 87 195 (29 717 s) | **−0,510** ± 0,022 (0,95, 31, 2,4e-4..0,448 Hz) | −0,545 | −2,03 / −2,24 |
| 63 | Boden | laut | 31 873 (10 007 s) | **−0,432** ± 0,019 (0,95, 28, 4,9e-4..0,448 Hz) | −0,482 | −1,81 / −2,02 |
| 14 | stark | laut | 29 814 (29 832 s) | −0,414 ± 0,020 (0,93, 32, 2,4e-4..0,448 Hz) | −0,441 | −2,72 / −2,95 |
| 43 | stark | laut | 31 782 (41 257 s) | −0,620 ± 0,015 (0,98, 35, 1,2e-4..0,448 Hz) | −0,679 | −2,71 / −3,20 |
| 63 | stark | laut | 49 672 (25 424 s) | −0,514 ± 0,021 (0,95, 32, 2,4e-4..0,448 Hz) | −0,579 | −2,92 / −3,27 |
| 14 | Boden | ruhig 1995-11/12 | 5 272 (2 332 s) | −0,396 ± 0,033 (0,88, 22, 1,9e-3..0,447 Hz) | −0,854 | −1,84 / −2,11 |
| 43 | Boden | ruhig | 2 368 (3 315 s) | −0,400 ± 0,064 (0,70, 19, 3,9e-3..0,447 Hz) | −0,601 | −1,15 / −1,68 |
| 63 | Boden | ruhig | 1 789 (4 740 s) | −0,234 ± 0,065 (0,45, 18, 7,3e-3..0,447 Hz) | −0,474 | −1,83 / −2,41 |
| 14 | stark | ruhig | 2 882 (4 959 s) | −0,707 ± 0,051 (0,90, 23) | −0,739 | −2,38 / −3,10 |
| 43 | stark | ruhig | 6 234 (39 931 s) | −0,651 ± 0,025 (0,96, 28) | −0,670 | −2,82 / −3,38 |
| 63 | stark | ruhig | 3 479 (8 735 s) | −0,747 ± 0,030 (0,96, 25) | −0,799 | −2,50 / −3,04 |

Befund Farbe: Im aufgelösten Band ~2e-4..0,45 Hz ist die Boden-Rausch-Reihe **nahe-weiß, leicht
rosa** — Steigung −0,43..−0,51 (Median über die drei Stationen ≈ −0,48), weit von weiß=0, von
1/f=−1 und von Kolmogorov=−5/3. Sie ist **farbgleich** zum starken Plateau derselben Ära
(−0,41..−0,62) — die Zustände tragen kein unterscheidbares Spektralzeichen; der Boden hebt das
PSD-Niveau im Band nur um ~0,6–1,1 dex bei 0,01 Hz (Faktor ~4–13 in Leistung) gegenüber dem
starken Zustand derselben Ära, nicht die Steigung. Die Boden-Lautheit im Zellen-RMS (Median ~9×
stark, Zellen bis 10²–10⁴×) übersteigt das im Band gemessene Niveau um Größenordnungen — ihr Sitz
liegt unterhalb der aufgelösten Bandkante (Zeitskalen länger als der größte Lauf-Abschnitt,
~Stunden-Pass-Skala, f < ~2e-4 Hz); die Farbe dieses Langzeit-Anteils ist mit dieser Messung
**nicht aufgelöst** (`pending`), die Kurzzeit-Farbe (f > 2e-4 Hz) ist gemessen nahe-weiß.

## Verdict

**Form:** Der AGC-Boden hat eine distinkte 4D-Form auf der Präsenz-Ebene — er ist eine
**Fern-Schalen-Erscheinung** der Weltlinie (heliozentrisch r ≥ ~4,99 AU, 1994-12-18..1997-02-28,
Modus 1; 1995-11-23..1997-02-25, Modus 2), die nie auf dem Erd-nahen Weltlinien-Segment (1990,
r ~1 AU) erscheint, wo der starke Zustand laut war. Seine **Lautheit ist keine räumliche Hülle**:
bei identischer (x,y,z,t) liest der Boden an verschiedenen Stationen 0,02–500 Hz — die laute/
ruhige Boden-Trennung ist Station-/Epochen-gebunden, nicht Position/Geometrie (G4-bestätigt:
Boden nur in der Jupiter-Ära-Geometrie; Opposition 1991–95 und Konjunktion vor 1994 unbesetzt,
n = 0).

**Farbe:** Der Boden trägt **keine distinkte Farbe** — im Band 2e-4..0,45 Hz nahe-weiß/leicht rosa
(Steigung ≈ −0,43..−0,51), statistisch gleich zum starken Zustand derselben Ära; der Boden
unterscheidet sich vom starken Zustand in Niveau (PSD ~4–13×, Zellen-RMS ~9× in derselben
Schale), nicht in Spektralcharakter. Sein Lautheits-Sitz unter ~2e-4 Hz bleibt `pending`.
