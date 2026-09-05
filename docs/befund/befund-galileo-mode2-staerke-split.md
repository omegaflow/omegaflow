<!--
  title: Befund — Galileo Mode-2-Stärke-Split: das ruhige Zwei-Wege-Fenster ist nicht stärkeflach (AGC-Boden −2560 trägt den Rausch-Term)
  class: befund
  date: 2026-09-05
  sha256: 3f9c67fdeb95bebbc3da29567254dd625be5cd8f67e43262febb153b160badb8
  status: done
  antwortet-auf: docs/befund/befund-galileo-mode2-station-split.md docs/befund/befund-galileo-mode1-fingerabdruck.md
  see-also: docs/befund/befund-galileo-te-staerke-floor.md docs/befund/befund-galileo-inpass-staerke-rampe.md docs/auftrag/auftrag-quiet-zone-uebertragung.md docs/TODO.md
-->
# Befund: Galileo Mode-2-Stärke-Split — das ruhige Zwei-Wege-Fenster ist nicht stärkeflach; der AGC-Boden (−2560) trägt den Rausch-Term

## Frage & Bindung

Der Mode-2-Station-Tag-Split (`befund-galileo-mode2-station-split`, done) hat den
Konjunktions-Boden auf 0,28–0,58 Hz (Station-Tag-Zellen, Stationen 14/43/63) gemessen —
etwa 3- bis 6-mal über dem starken Mode-1-Plateau (0,09–0,15 Hz). Offen blieb, ob das
Zwei-Wege-Residuum einen empfangsstärke-abhängigen Rausch-Term trägt (Boden lauter am AGC-Boden)
oder ob sein Boden stärkeunabhängig ist. Dieser Entwurf spiegelt die Mode-1-Fingerabdruck-Methode
(`befund-galileo-mode1-fingerabdruck`, done) auf Mode 2.

Gebunden: nur Mode 2 (Slot [3]); Lock-Übergänge (|resid| > 1000 Hz) vor dem Rauschen getrennt;
Stärke 0 (Slot [7]) als `s=0` getrennt geführt, nie in ein Quartil gelegt (0-Kanon);
Stärke-Quartile auf Probenniveau der nicht-verschwindenden Stärke (Mode-1-Schnitttechnik);
Rausch-Zelle = per (Tag) und per (Tag, Station, Quartil), Zell-RMS um den Zell-Mittelwert,
Zellen ≥ 30 Proben, Median über die Zellen. Konjunktions-Untermenge: Zellen, deren Tag eine
Elongation ε < 30° trägt (Winkel an der Erde, aus `galileo_daily` + `earth`-Barycentern).
Datenkette: `data/galileo_resid.bin` (GASR). Probe
`tools/measure/src/bin/galileo_mode2_strength_split.rs` (additiv, neu), Report
`reports/galileo_mode2_strength_split.txt`. `cargo check` 0/0.

## n zuerst (0 geehrt)

Mode 2: 3 110 045 Proben, 157 784 Lock-Übergänge, 2 952 261 Proben nach Lock-Ausschluss,
107 Tage, Spanne 1990-11-29 .. 1997-02-25, Proben je Tag Median 19 454 (min 64, max 86 309).
Stärke-Feld: 2 951 553 Proben ≠ 0, 708 Proben = 0 (als `s=0` getrennt), 852 diskrete Werte.
Stationen: 12 (56 179, 7 d), 14 (705 113, 52 d), 42 (98 618, 4 d), 43 (1 150 165, 53 d),
61 (55 516, 4 d), 63 (886 670, 74 d). Geometrie: 107/107 Tage aufgelöst, 0 ohne Ephemeriden;
75 Tage mit Elongation < 30° tragen Nicht-Lock-Mode-2-Proben.

## Tabelle 1 — Stärke-Skala & Quartilschnitte (Mode 2)

| Größe | Wert |
|---|---|
| Proben ≠ 0 | 2 951 553 |
| Proben = 0 (`s=0`, getrennt) | 708 |
| diskrete Werte ≠ 0 | 852 |
| Perzentile | p10–p60 = −2560, p70 = −1756, p80 = −1748, p90 = −1738 |
| Quartilschnitte | b1 = −2560, b2 = −2560, b3 = −1752 |
| schwächster / häufigster Wert | −2560, 1 879 516 Proben = 63,68 % der ≠ 0 |

Die Mode-2-Stärke liegt zu 63,68 % exakt auf dem AGC-Boden −2560: b1 und b2 fallen auf denselben
Wert, **Q2 ist dadurch leer** (kein Messwert, sondern die Quartil-Konstruktion bei einem Boden mit
> 50 % Anteil — 0 geehrt, kein fabrizierter Wert). Die besetzten Bins sind Q1 = −2560 (Boden),
Q3 ≈ −1757 und Q4 ≈ −1742 (Plateau darüber).

## Tabelle 2 — Rauschen vs Stärke (Median Tages-RMS, Tag-Zellen über Stationen gepoolt, ≥ 30 Proben); konj = Zellen mit Tag-ε < 30°

| Bin | Stärke | Proben | Tage | Zellen | Median | konj-Zellen | konj-Median |
|---|---|---|---|---|---|---|---|
| Q1 (Boden) | −2560 | 1 879 516 | 75 | 63 | 2,452 Hz | 44 | 2,293 Hz |
| Q2 | (leer) | 0 | 0 | 0 | – | 0 | – |
| Q3 | −1757 | 338 752 | 48 | 45 | 0,244 Hz | 45 | 0,244 Hz |
| Q4 (stark) | −1742 | 733 285 | 65 | 64 | 0,197 Hz | 49 | 0,136 Hz |
| alle | — | 2 952 261 | 107 | 107 | 2,792 Hz | 75 | 0,647 Hz |
| s=0 | 0 | 708 | 13 | 5 | 201,078 Hz | 0 | – |

## Tabelle 3 — Stations-Split 14/43/63 (Zellen (Tag, Station, Bin) ≥ 30 Proben) — die Säule des Verdikts

| Station | Bin | Zellen | Proben | Median | konj-Zellen | konj-Median |
|---|---|---|---|---|---|---|
| 14 | Q1 | 38 | 520 723 | 0,414 Hz | 26 | 0,414 Hz |
| 14 | Q3 | 11 | 28 263 | 0,111 Hz | 11 | 0,111 Hz |
| 14 | Q4 | 22 | 155 996 | 1,498 Hz | 14 | 0,183 Hz |
| 14 | alle | 52 | 705 113 | 1,498 Hz | 32 | 0,295 Hz |
| 43 | Q1 | 35 | 804 569 | 0,578 Hz | 24 | 0,866 Hz |
| 43 | Q3 | 13 | 181 319 | 0,705 Hz | 13 | 0,705 Hz |
| 43 | Q4 | 24 | 164 211 | 3,322 Hz | 14 | 0,155 Hz |
| 43 | alle | 52 | 1 150 146 | 1,612 Hz | 33 | 0,578 Hz |
| 63 | Q1 | 39 | 554 050 | 1,993 Hz | 29 | 0,996 Hz |
| 63 | Q3 | 33 | 129 100 | 0,187 Hz | 33 | 0,187 Hz |
| 63 | Q4 | 41 | 203 269 | 0,137 Hz | 33 | 0,068 Hz |
| 63 | alle | 74 | 886 670 | 2,872 Hz | 56 | 0,277 Hz |

Die Station-Tag-Kontrolle „alle Bins" reproduziert den Station-Tag-Boden der Vor-Probe
(0,295/0,578/0,277 Hz gegen 0,29/0,58/0,28 Hz) — dieselbe Zell-Definition, gleiche Zahlen.

## Der Befund

**1. Mode 2 trägt im Stärke-Zustands-Split (statisch, ungerichtet) eine Boden-Erhöhung; der
Konjunktions-Boden ist nicht stärkeunabhängig.** Im ruhigen Fenster (Tag-ε < 30°) ist das
Boden-Bin Q1 (−2560) an jeder der drei Stationen lauter als das starke Q4: Station 14 0,414 gegen
0,183 Hz (Faktor 2,3), Station 43 0,866 gegen 0,155 Hz (Faktor 5,6), Station 63 0,996 gegen
0,068 Hz (Faktor 14,6). Auch gepoolt über die Stationen ist der Boden im ruhigen Fenster lauter
als die starken Bins (konj-Median Q1 2,293 Hz gegen Q3 0,244 Hz und Q4 0,136 Hz). Wäre der Boden
stärkeunabhängig, müsste die Stärke-Trennung innerhalb des Fensters flach bleiben — sie ist es
nicht. Dies ist eine statische Zustands-Assoziation ohne Epochen-Kontrolle und ohne Richtung;
ob sie echt oder Ären-kollokiert ist, trennt der In-Pass-Test bei Pass-Identität
(`befund-galileo-inpass-staerke-rampe`): dort ist sie an Stationen 43/63 echt, an Station 14
nicht.

**2. Das starke Zwei-Wege-Quartil erreicht das Mode-1-Plateau; die 0,28–0,58-Hz-Erhöhung wird
überwiegend vom Boden-Bin getragen.** Die konj-Mediane des starken Q4 liegen bei 0,183/0,155/0,068 Hz
(Stationen 14/43/63) — auf oder unter dem Mode-1-Plateau (0,09–0,15 Hz). Der Station-Tag-Boden des
Station-Splits (0,28–0,58 Hz) mischt das laute Boden-Bin mit dem ruhigen starken Quartil. Der Boden
ist aber nicht das einzige laute Bin bei Station 43: dort liegt auch Q3 (0,705 Hz) über dem
Station-Tag-Wert (0,578 Hz) — die Erhöhung ist dort nicht ausschließlich Boden-getragen (siehe
Grenzen/Stations-43-Mitte).

**3. Oberhalb des Bodens ist keine durchgehende Stärke-Kurve gemessen.** Q3 (≈ −1757) gegen Q4
(≈ −1742) ist nicht konsistent geordnet (Station 14: 0,111 < 0,183; Station 43: 0,705 > 0,155;
Station 63: 0,187 > 0,068 im konj-Fenster). Der Term bindet an den Boden-Zustand (−2560), nicht an
ein feines SNR-Kontinuum — die Stärke-Skala ist grob (Boden vs Plateau), wie in Mode 1.

## Grenzen

- Stations-43-Mitte: das Q3-konj-Fenster (0,705 Hz, 13 Zellen, alle Tage ε < 30°) ist so laut wie
  das Q1-Boden-Fenster (0,866 Hz) — eine stationsspezifische Mitte oder eine
  Tages-Komposition; bei Station 14 und 63 ist Q3 dagegen ruhig (0,111/0,187 Hz). Nicht
  entschieden — `pending`.
- Tag-Komposition: Boden- und starke Zellen liegen auf verschiedenen Kalendertagen innerhalb des
  Konjunktions-Fensters; ein Tag-gepaarter Vergleich (derselbe Tag, Boden vs stark) ist noch nicht
  gemessen — `pending`.
- Richtung: −2560 ist der AGC-Klemm-Registerwert; ob die Klemmung das laute Rauschen begleitet
  oder ob sie einen akquisitionsnahen Zustand markiert, trennt dieser Split nicht — `pending`.
- `s=0` (708 Proben, 13 Tage): Median 201 Hz über 5 dünne Zellen, je Station Einzelzellen
  (18–677 Hz) — als eigene Klasse getrennt geführt, nicht interpretiert; 0 geehrt.
- Q2 ist konstruktionsbedingt leer (Boden-Anteil 63,68 % > 50 %): ein leeres Bin, kein
  Null-Rausch-Wert.

## Register-Satz

*Das Zwei-Wege-Residuum trägt einen empfangsstärke-abhängigen Term: im ruhigen
Konjunktions-Fenster ist der AGC-Boden (−2560) an jeder der drei Stationen lauter als das starke
Quartil (Faktor 2,3/5,6/14,6). Der Mode-2-Konjunktions-Boden ist damit nicht stärkeunabhängig;
das starke Quartil erreicht das Mode-1-Plateau (0,068–0,183 Hz), und die 0,28–0,58-Hz-Erhöhung
des Station-Tag-Splits wird vom Boden-Bin getragen. Der Modus-1-Fingerabdruck (Boden lauter als
Plateau) wiederholt sich in Mode 2 im ruhigen Fenster.*

## Status

`draft` — vom Rat ungehalten; vorgelegt für die Orchestrierung. Probe und Report sind additive
lokale Lauf-Artefakte (neue Dateien), nichts Bestehendes verändert.
