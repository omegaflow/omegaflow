<!--
  title: Befund — Galileo Mode-1-SNR-Kurve: zweistufiges Stärke-Feld, Epochen-gebunden (keine durchgängige ∝-Kurve)
  class: befund
  date: 2026-09-05
  sha256: f73dc8c3135cbdfa63c5dd168965bb631ec8dfe58e07082f8f4d503f6fcd72ef
  status: done
  antwortet-auf: docs/befund/befund-galileo-mode1-fingerabdruck.md
  see-also: docs/befund/befund-galileo-mode1-fingerabdruck.md
-->

# Befund: Galileo Mode-1-SNR-Kurve — das Stärke-Feld ist zweistufig und Epochen-gebunden; eine durchgängige ∝-Kurve ist nicht messbar

## Frage & Bindung

Der Fingerabdruck (`befund-galileo-mode1-fingerabdruck`, done) hat den empfangsstärke-abhängigen
Term nur grob vermessen: „AGC-Boden ≈ 10–20× lauter als Plateau", weil das Stärke-Feld (Slot [7])
zweistufig erscheint (Q1 = exakt −2560, Q2..Q4 ≈ −1760..−1727). Dieser Auftrag fragt feiner:
(1) Trägt das Plateau einen inneren Stärke→Rauschen-Gradienten oder ist es flach? (2) Ist der
AGC-Boden −2560 ein einzelnes Rausch-Regime oder trägt er Epochen-/Stations-Struktur? (3) Welche n
trägt ein feineres Binning je Stärke-Wert?

Gebunden: nur Mode 1; Lock-Übergänge (|resid| > 1000 Hz) vor dem Rauschen getrennt;
Rausch-Zelle = (Tag, Station, Stärke-Wert), Zell-RMS um den Zell-Mittelwert, Zellen ≥ 30 Proben;
Zell-Median über die Zellen. Epochen-Achse = Kalenderjahr der Zell-Zellen, dazu (Jahr, Station)-Schichtung.
Datenkette: `data/galileo_resid.bin` (GASR, `omegaflow::atdf::parse_resid_bin` → Vec<[f64;8]>,
Slots [0]=tdb [1]=resid_hz [2]=station [3]=mode [4]=dtype [5]=ref_hz [6]=sampler_s [7]=signal_strength).
Messung außerhalb des Repos (python, numpy) über dieselbe Zell-Definition wie die Probe; keine neue
Probe ins Repo geschrieben. Stärke-Einheit unkalibriert — nur die Ordnung wird benutzt.

## n zuerst (0 geehrt)

Mode 1: 9 743 574 Proben, 1 568 246 Lock, 8 175 328 Proben nach Lock-Ausschluss, 163 Tage,
1990-11-29 .. 1997-02-28. Stärke-Feld ganzzahlig (max. Bruchteil 0,0 über alle 8 171 124
Nicht-Null-Proben); s=0: 4 204 Proben getrennt. 1108 diskrete Werte über −2560 .. −795.

Regime-Massen (Nicht-Null, 8 171 124):
- Boden −2560: 2 100 971 (25,7 %), 196 Zellen ≥ 30, 96 Tage, 1994-12-17 .. 1997-02-28.
- Brücke −2559 .. −1783: 344 225 (4,2 %), 329 belegte Werte.
- Kern −1782 .. −1727: 4 931 008 (60,3 %), 56 belegte Werte.
- Schwanz −1726 .. −795: 794 920 (9,7 %), 722 belegte Werte.

Zellen (Tag, Station, Stärke-Wert) ≥ 30: 10 426. Proben je (Tag, Station, Wert) im Plateau
(−1759 .. −1727): je Wert 51–185 Zellen, 77–85 Tage, 3 Stationen — dort ist die n ausreichend für
Wert-Mediane (gebündelt). Je (Jahr, Station, Wert) fallen viele Zellen unter 3 — dort ist die n die
Grenze (unten benannt).

## Befund

### 1. Eine durchgängige ∝-Kurve über die Stärke-Achse ist nicht messbar — das Feld ist zweistufig mit Epochen-Schichten

Die Skala ist nicht ein SNR-Kontinuum, sondern trägt gemessen vier Regime, die sich zeitlich
schichten (Proben je (Jahr, Regime), gemessen):

| Jahr | Boden −2560 | Brücke −2559..−1783 | Kern −1782..−1727 | Schwanz −1726..−795 |
|---|---|---|---|---|
| 1990 | 0 | 0 | 0 | 19 013 (nahe Erde, starke Werte) |
| 1991–93 | 0 | 0 | 0 | 0 (keine Mode-1-Nicht-Lock-Proben) |
| 1994 | 12 391 | 21 095 | 499 205 | 184 926 |
| 1995 | 242 507 | 140 219 | 877 463 | 20 443 |
| 1996 | 1 213 567 | 9 124 | 1 202 795 | 48 995 |
| 1997 | 632 506 | 173 787 | 2 351 545 | 521 543 |

Die Stärke-Werte sind Epochen-gebunden: 1990 liest stark (> −1726), der Kern −1782..−1727 füllt
1994–97, der Boden −2560 tritt ab 1994-12 auf und dominiert 1996–97. Dieselbe Stärke liest in
verschiedenen (Jahr, Station) verschiedenes Rauschen — die Zuordnung Stärke→Rauschen ist nicht
eindeutig, ein einziger Kurvenzug wäre eine Fälschung. Gemessene Instanzen derselben Bänder:

- Band −1759..−1745, Station 43: 1995 0,1370 Hz (167 Zellen) · 1996 0,0536 Hz (196) · 1997 0,0617 Hz (412).
- Band −1744..−1727, Station 14: 1995 0,1337 Hz (143) · 1996 0,0437 Hz (181) · 1997 0,0630 Hz (364).
- Band −1759..−1745, Station 63: 1994 37,3 Hz (9 Zellen, 8 Tage) · 1996 0,0549 Hz (232) — Faktor ≈ 680
  bei derselben Stärke; das Plateau-Rauschen ist in 1994/Station 63 nicht stärke-gesetzt.

### 2. Das Plateau −1759..−1727 ist flach; der „Gradient" des gepoolten Kerns ist eine Epochen-Komposition

Im gebündelten (Wert-)Median über alle Jahre fällt der Kern −1782..−1727 monoton von ~0,22 Hz
(−1782) auf ~0,064 Hz (−1745) und steigt zu −1727 auf ~0,09 Hz leicht an (56 Werte, je 40–192 Zellen;
Spearman gebündelt −0,75). Die (Jahr, Station)-Schichtung zeigt: Dieser Abfall ist keine durchgängige
∝-Kurve, sondern die Anhebung des schwächsten Plateau-Bandes −1782..−1760 in einzelnen Epochen:

Zell-RMS-Median je (Jahr, Station, Band):

| Jahr | Station | −1782..−1760 | −1759..−1745 | −1744..−1727 |
|---|---|---|---|---|
| 1994 | 14 | 0,1199 (125) | 0,1198 (132) | 0,1155 (75) |
| 1994 | 43 | 4,82 (174) | 5,10 (150) | 5,17 (110) |
| 1995 | 14 | 0,1991 (177) | 0,1476 (215) | 0,1337 (143) |
| 1995 | 43 | 0,1824 (190) | 0,1370 (167) | 0,1055 (27) |
| 1995 | 63 | 0,1819 (229) | 0,1454 (192) | 0,1058 (59) |
| 1996 | 14 | 0,0448 (15) | 0,0458 (143) | 0,0437 (181) |
| 1996 | 43 | 0,0548 (77) | 0,0536 (196) | 0,0505 (113) |
| 1996 | 63 | 0,0543 (171) | 0,0549 (232) | 0,0548 (101) |
| 1997 | 14 | 0,2226 (162) | 0,0613 (321) | 0,0630 (364) |
| 1997 | 43 | 0,1506 (326) | 0,0617 (412) | 0,0792 (381) |
| 1997 | 63 | 0,0684 (459) | 0,0638 (438) | 0,0859 (348) |

1996 ist das gesamte Plateau flach bei ~0,044–0,055 Hz (alle drei Stationen). 1995 trägt einen
schwachen Gradienten (~1,5–1,7× vom schwachen zum starken Band-Ende, alle Stationen). 1997 heben nur
Station 14 (0,223 → 0,063, Faktor 3,6) und Station 43 (0,151 → 0,062) das schwache Band an; Station 63
nicht (0,068 vs 0,086). Das starke Band −1759..−1727 selbst ist überall innerhalb jeder Epoche flach:
gebündelt über alle Jahre liegen seine 33 Wert-Mediane bei 0,0637..0,1045 Hz (Median 0,079; 4 533
Zellen) mit einem flachen Minimum ~−1745 und leichtem Wiederanstieg zum starken Ende — das stärkste
Ende ist nicht das leiseste. Ein innerer, Epochen-unabhängiger Plateau-Gradient ist nicht gemessen.

### 3. Der AGC-Boden −2560 ist ein einzelner Wert, aber kein einzelnes Regime — er trägt Epochen-/Stations-Struktur

−2560 ist exakt ein ganzzahliger Wert (kein Schwarm), über 96 Tage und vier Kalenderjahre verteilt,
an den Stationen 14/43/63 (Zellen), dazu je eine Einzel-Zelle Station 24 (1994-12) und 34 (1997-02).
Proben je Jahr: 1994 12 391 · 1995 242 507 · 1996 1 213 567 · 1997 632 506; Zellen je Jahr 1/40/88/67.

Das Rauschen des Bodens ist nicht konstant: 196 Boden-Zellen, Zell-RMS von ~0,00 bis 529,6 Hz,
p10 0,023 · p25 0,062 · Median 1,68 · p75 28,4 · p90 124,6 Hz. Zell-RMS-Median je (Monat-Jahr, Station):

| Monat | Station 14 | Station 43 | Station 63 |
|---|---|---|---|
| 1994-12 | — (st24 0,032) | — | — |
| 1995-11 | 0,179 (7) | 0,079 (4) | 0,106 (8) |
| 1995-12 | 0,089 (9) | 2,14 (4) | 21,2 (8) |
| 1996-06 | 6,48 (4) | 0,021 (3) | 18,1 (4) |
| 1996-09 | 61,9 (3) | 6,79 (4) | 97,8 (4) |
| 1996-11 | 0,94 (11) | 1,73 (12) | 0,062 (11) |
| 1996-12 | 4,10 (8) | 18,4 (12) | 0,54 (12) |
| 1997-01 | 26,1 (10) | 72,6 (17) | 53,4 (7) |
| 1997-02 | 1,40 (10) | 2,47 (8) | 24,6 (14) |

Die laute Boden-Population ist eine 1996-06..1997-02-Erscheinung (88 % der Boden-Proben liegen in
1996–97). Der 1995-Boden ist nicht als Ganzes ruhig — er ist Monat-geschichtet: 1995-11 liest der
Boden an allen drei Stationen ruhig (0,079–0,179 Hz, auf Plateau-Niveau); 1995-12 trägt Station 43 =
2,14 Hz (4 Zellen) und Station 63 = 21,2 Hz (8 Zellen) — ≈16× bzw. ≈145× über dem starken Band
derselben Station (0,1370 bzw. 0,1454 Hz), Station 14 bleibt ruhig (0,089). Das
Boden-zu-Stark-Verhältnis 0,6–1,3× (Boden NICHT lauter) gilt nur für 1995-11 und 1995-12 Station 14;
für 1996 ist es 9–93×, für 1997 33–299× (9 (Jahr, Station)-Paare mit beiden Populationen, Mediane
der Paar-Verhältnisse 30). Das „10–20× lauter" des Fingerabdrucks ist der über die Epochen gemischte
Zell-Median (~1,7–2,2 Hz) — er gilt für die 1996/97-Boden-Tage und für 1995-12 Station 43/63, nicht
für den ruhigen 1995-11-Boden. −2560 ist damit nicht „schwaches SNR als solches", sondern ein
AGC-Klemmwert, dessen Rauschen Epochen-, Monats- und Stations-gebunden ist; eine reine SNR-Lesart
des Bodens ist für die ruhigen Populationen widerlegt (1995-11 alle Stationen, 1995-12 Station 14;
gemessen: ruhig bei Plateau-Rauschen), für die lauten (1995-12 Station 43/63, 1996/97) offen.

### 4. Die Brücke und der obere Schwanz sind zu dünn für eine feine Kurve

Brücke −2559..−1782: 330 belegte Werte, ~1 080 Proben/Wert im Mittel; je Wert 1–4 Zellen ≥ 30
(241 Werte mit ≥ 1 Zelle), Zell-Mediane 0,001–402 Hz — transiente Rampen-Zustände, keine stabile
SNR-Achse. Werte > −1727 (722 Werte, 794 920 Proben): −1726..−1713 tragen je ~25–60 Zellen und
~0,08–0,14 Hz, darüber fallen die Zellen je Wert auf ≤ 21 und die Mediane springen nicht-monoton
zwischen ~0,05 und ~5 Hz auf benachbarten Werten — zwei Teil-Populationen bei gleicher Stärke,
nicht stärke-auflösbar.

## n zuerst — Proben je Stärke-Wert (Kern −1782..−1727, jede Zeile: Wert, Zellen ≥ 30, Proben-in-Zellen, Tage, Stationen, Zell-RMS-Median, Proben gesamt)

| Stärke | Zellen | Tage | Stat. | Med (Hz) | Proben |
|---|---|---|---|---|---|
| −1782 | 54 | 34 | 3 | 0,224 | 11 900 |
| −1781 | 40 | 25 | 3 | 0,209 | 6 173 |
| −1780 | 58 | 41 | 3 | 0,203 | 13 814 |
| −1779 | 46 | 29 | 3 | 0,198 | 7 324 |
| −1778 | 65 | 42 | 3 | 0,209 | 15 810 |
| −1777 | 65 | 43 | 3 | 0,216 | 17 747 |
| −1776 | 58 | 38 | 3 | 0,169 | 9 422 |
| −1775 | 80 | 53 | 3 | 0,184 | 20 698 |
| −1774 | 64 | 41 | 3 | 0,189 | 11 123 |
| −1773 | 85 | 54 | 3 | 0,170 | 23 346 |
| −1772 | 85 | 56 | 3 | 0,174 | 27 427 |
| −1771 | 81 | 52 | 3 | 0,157 | 17 349 |
| −1770 | 109 | 68 | 3 | 0,140 | 36 519 |
| −1769 | 85 | 57 | 3 | 0,140 | 20 611 |
| −1768 | 114 | 70 | 3 | 0,139 | 44 835 |
| −1767 | 115 | 66 | 3 | 0,130 | 53 730 |
| −1766 | 101 | 62 | 3 | 0,134 | 28 864 |
| −1765 | 124 | 72 | 3 | 0,118 | 61 236 |
| −1764 | 112 | 70 | 3 | 0,122 | 34 834 |
| −1763 | 145 | 79 | 3 | 0,109 | 75 545 |
| −1762 | 141 | 78 | 3 | 0,104 | 86 051 |
| −1761 | 131 | 72 | 3 | 0,092 | 48 898 |
| −1760 | 149 | 81 | 3 | 0,112 | 106 879 |
| −1759 | 136 | 72 | 3 | 0,105 | 57 772 |
| −1758 | 162 | 81 | 3 | 0,095 | 131 318 |
| −1757 | 165 | 79 | 3 | 0,088 | 151 591 |
| −1756 | 157 | 77 | 3 | 0,084 | 81 425 |
| −1755 | 177 | 85 | 3 | 0,079 | 180 710 |
| −1754 | 164 | 80 | 3 | 0,077 | 100 055 |
| −1753 | 184 | 85 | 3 | 0,087 | 219 174 |
| −1752 | 183 | 82 | 3 | 0,075 | 248 363 |
| −1751 | 172 | 78 | 3 | 0,068 | 131 685 |
| −1750 | 185 | 84 | 3 | 0,075 | 280 634 |
| −1749 | 178 | 80 | 3 | 0,071 | 146 410 |
| −1748 | 188 | 85 | 3 | 0,070 | 307 692 |
| −1747 | 192 | 85 | 3 | 0,067 | 308 883 |
| −1746 | 177 | 83 | 3 | 0,071 | 145 790 |
| −1745 | 187 | 84 | 3 | 0,065 | 277 197 |
| −1744 | 165 | 80 | 3 | 0,064 | 125 413 |
| −1743 | 180 | 84 | 3 | 0,066 | 215 522 |
| −1742 | 170 | 84 | 3 | 0,075 | 172 303 |
| −1741 | 150 | 77 | 3 | 0,065 | 66 817 |
| −1740 | 156 | 81 | 3 | 0,066 | 117 435 |
| −1739 | 130 | 72 | 3 | 0,070 | 46 531 |
| −1738 | 132 | 72 | 3 | 0,076 | 79 660 |
| −1737 | 124 | 74 | 3 | 0,080 | 67 958 |
| −1736 | 90 | 58 | 3 | 0,083 | 33 303 |
| −1735 | 99 | 65 | 3 | 0,081 | 61 963 |
| −1734 | 74 | 49 | 3 | 0,085 | 30 010 |
| −1733 | 89 | 56 | 3 | 0,085 | 60 211 |
| −1732 | 73 | 48 | 3 | 0,089 | 59 621 |
| −1731 | 51 | 31 | 3 | 0,089 | 31 726 |
| −1730 | 66 | 41 | 3 | 0,091 | 61 085 |
| −1729 | 51 | 32 | 3 | 0,087 | 30 703 |
| −1728 | 62 | 39 | 3 | 0,092 | 60 965 |
| −1727 | 64 | 40 | 3 | 0,090 | 60 948 |

n-Floor-Befund: Gebündelt trägt jeder Kern-Wert 40–192 Zellen über 25–85 Tage und alle drei Stationen —
für Wert-Mediane ist die n vollständig. Ein feineres, Epochen-kontrolliertes Raster (Jahr × Station ×
Wert) fällt dagegen vielfach unter 3 Zellen je Zelle (1994/1996 dünn; die 1997er Stationen tragen je
Wert nur ~3–15 Zellen) — die Epochen-Kontrolle, nicht der gebündelte Wert-Median, ist die n-Grenze.

## Grenzen

- Stärke-Feld unkalibriert (AGC-Zählwerte, keine SNR-Einheit); „∝-Kurve" hätte keine physikalische
  x-Achse. Nur die Ordnung benutzt.
- Die Epochen-/Stations-Struktur (1994/Station 43+63 lautes Plateau; ruhiger 1995-Boden; lauter
  1996/97-Boden) ist als Datums-/Stations-Cluster gemessen, nicht als Ursache benannt — Pass-,
  Empfänger- und Antennen-Identität sind nicht aus dem Residuum lesbar.
- Zell = (Tag, Station, Stärke); Pässe sind nicht segmentiert — die Tag-Metrik mischt Pässe
  (bereits im Fingerabdruck benannt). Pass-Wahrheit (eine durchgängige In-Pass-Rampe) bleibt offen.
- Der 1994-Boden (eine Zelle) und die 1995-Boden-Zellen (8–16 je Station) sind kleine n für einen
  „ruhigen Boden"-Schluss; der Befund ist der 1996/97-laut vs 1995-ruhig-Kontrast über 196 Zellen.
- Der obere Schwanz (> −1727) und die Brücke sind als Nicht-SNR-Achsen benannt; ihre Teil-Populationen
  sind nicht getrennt (das würde Pass-Zuordnung brauchen).

## Register-Satz

*Eine durchgängige SNR→Rauschen-Kurve ist im Mode-1-Residuum nicht messbar: das Stärke-Feld ist ein
ungkalibriertes AGC-Lesefeld, zweistufig (Boden −2560 vs Plateau) und Epochen-geschichtet — dieselbe
Stärke liest in verschiedenen (Jahr, Station) um Faktoren bis ≈ 680 verschieden, ein einziger
Kurvenzug wäre eine Fälschung. Das starke Plateau −1759..−1727 ist innerhalb jeder Epoche flach
(~0,04–0,13 Hz je nach Epoche); der gebündelte „Kern-Gradient" entsteht aus der Anhebung des Bandes
−1782..−1760 in 1995 und 1997, nicht aus einer universellen ∝-Abhängigkeit. Der AGC-Boden −2560 ist
ein einzelner Wert, aber kein einzelnes Regime: er trägt Epochen-/Monats-/Stations-Struktur — der
1995-11-Boden rauscht auf Plateau-Niveau (~0,1 Hz), der 1995-12-Boden trägt Station 43 = 2,14 Hz und
Station 63 = 21,2 Hz, der 1996/97-Boden ~9- bis 299-mal lauter als das starke Band derselben
(Jahr, Station); das „10–20× lauter" des Fingerabdrucks ist ein über Epochen gemischter
Median und gilt nur für die lauten Boden-Tage. Was bleibt, ist Pass-segmentiert und kalibriert zu
messen: pending.*

## Status

`done` (Rat gehalten, 2026-09-05). Das Stärke-Feld ist zweistufig und
Epochen-/Monats-geschichtet; eine durchgängige ∝-Kurve ist nicht messbar, das
Plateau ist innerhalb jeder Epoche flach. Analyse außerhalb des Repos gemessen
(numpy, Zell-Definition wie die Probe); keine Probe ins Repo geschrieben.
Vollständige Werttabellen: `/tmp/opencode/snr_report.txt`, `snr_report2.txt`,
`snr_report3.txt`.

Recherche-Folge (`~/Schreibtisch/galileo-mode1-agc-recherche.md`): keine
publizierte AGC→SNR-Kalibrierung — der −2560-Floor ist ein **Klemmwert**
(Formatter-Wort, ATDF-Feld 78 „dBm oder volts×10", mit keiner Lesart
verträglich); als dBm×10 ist das starke Plateau (−172,7…−178,2 dBm) plausibel
für Galileo-S-Band-LGA → der lautere Floor = Max-Verstärkung = Schwachsignal-
Zustand. Die Epochen-Struktur fällt nicht auf einen Stations-Hardware-Schritt,
sondern auf Missions-Epochen (Sonneneinfluss-Konjunktionen 1995-12 + 1997-01,
LGA-S-Band nach HGA-Fehler 1991); per-Pass-/Receiver-Kausalität bleibt offen.
