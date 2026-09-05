<!--
  title: Befund — Galileo Tag-gepaarte Boden/Stark-Messung: der AGC-Boden ist auch am selben Tag lauter als das starke Plateau (kein Tag-Kompositions-Artefakt)
  class: befund
  date: 2026-09-05
  sha256: 0ecc26397f8de224923047d562ec791335c804f45dad99f3c348cdf8c8751815
  status: done
  antwortet-auf: docs/befund/befund-galileo-mode2-staerke-split.md
  see-also: docs/befund/befund-galileo-mode1-fingerabdruck.md docs/befund/befund-galileo-inpass-staerke-rampe.md docs/TODO.md
-->
# Befund: Galileo Tag-gepaarte Boden/Stark-Messung — der AGC-Boden ist auch am selben Tag lauter als das starke Plateau

## Frage & Bindung

Der Mode-2-Stärke-Split (`befund-galileo-mode2-staerke-split`, done) hat den AGC-Boden (−2560)
im ruhigen Konjunktions-Fenster lauter gemessen als das starke Quartil (Station 14 Faktor 2,3,
Station 43 Faktor 5,6, Station 63 Faktor 14,6). Als Grenze blieb: Boden- und starke Zellen lagen
auf verschiedenen Kalendertagen innerhalb des ruhigen Fensters — ein Tag-gepaarter Vergleich
(derselbe Tag, Boden vs stark) war nicht gemessen (`pending`, Tag-Komposition). Diese Probe misst
genau diesen Kontroll-Vergleich: auf einem Tag, der an derselben Station BEIDE Zustände trägt
(Boden −2560 UND starkes Plateau ≥ −1750, je ≥ 30 Proben), ist die Boden-Proben-Streuung lauter
als die starke Proben-Streuung? Der Tag (und mit ihm seine Ära und Geometrie) ist damit je Paar
konstant.

Gebunden: nur Modus-2-Proben (Slot [3], primär, die F1-Frage); Modus 1 sekundär berichtet, wenn
vorhanden; Lock-Übergänge (|resid| > 1000 Hz) vor dem Rauschen getrennt; Stärke 0 (Slot [7]) als
`0` getrennt geführt, nie klassiert (0-Kanon). Klassen: Boden = signal_strength exakt −2560
(AGC-Klemmwert); stark = signal_strength ≥ −1750 (starkes Plateau-Ende, wie F1 spezifiziert).
Paar-Einheit: (Kalendertag, Station), das beide Klassen mit ≥ 30 Proben trägt. Rausch-Zelle =
RMS um den Klassen-Mittelwert innerhalb des Paars; diff = Boden-RMS − stark-RMS (gleicher Tag,
gleiche Station). Geometrie: Elongation an der Erde (Sonne/Sonde), Konjunktion ε < 30°; je Paar
Tag-konstant, Ära/Geometrie geteilt. Datenkette: `data/galileo_resid.bin` (GASR). Probe
`tools/measure/src/bin/galileo_sameday_floor_strong.rs` (additiv, neu), Report
`reports/galileo_sameday_floor_strong.txt` (alle 22 Mode-2- und 62 Mode-1-Zellen als Tabelle).
`cargo check` 0/0.

## n zuerst (0 geehrt)

Mode 2: 3 110 045 Proben, 157 784 Lock-Übergänge, Boden-Klasse 1 879 516 Proben (75 Tage),
starke Klasse 707 263 Proben (65 Tage), 708 Proben Stärke 0 getrennt — die Klassen reproduzieren
die Split-Zählungen exakt (Q1 1 879 516/75 d). Tag-gepaarte Zellen (Tag, Station) mit beiden
Klassen ≥ 30: **22** (Stationen 14/43/63, **21 distinkte Tage**), Spanne 1995-11-24 ..
1997-02-21. Modus 1: 9 743 574 Proben, 1 568 246 Lock-Übergänge, Boden 2 100 971 Proben
(113 Tage), stark 3 647 904 (120 Tage); gepaarte Zellen: **62** (Stationen 14/43/63,
**48 distinkte Tage**), Spanne 1995-11-24 .. 1997-02-28.

## Tabelle 1 — Tag-gepaarter Vergleich, Modus 2 (primär): Zellen (Tag, Station) mit Boden −2560 UND stark ≥ −1750, je ≥ 30 Proben; diff = Boden-RMS − stark-RMS am selben Tag/Station

| Ebene | n | Median Boden-RMS | Median stark-RMS | Median diff | Zeichen (lauter/leiser) |
|---|---|---|---|---|---|
| Zellen (Tag, Station) | 22 | 2,878 Hz | 0,126 Hz | +2,620 Hz | 19 Boden laut / 3 stark laut |
| Konjunktions-Untermenge (ε < 30°) | 20 | — | — | +2,620 Hz | 18 / 2 |
| Tag-Ebene (gepaarte Stationen je Tag gepoolt) | 21 Tage | — | — | +2,530 Hz | 18 / 3 |
| Station 14 | 8 | — | — | +27,297 Hz | 8 / 0 |
| Station 43 | 7 | — | — | +2,156 Hz | 5 / 2 |
| Station 63 | 7 | — | — | +2,287 Hz | 6 / 1 |

## Tabelle 2 — Tag-gepaarter Vergleich, Modus 1 (sekundär, gleiche Definition)

| Ebene | n | Median Boden-RMS | Median stark-RMS | Median diff | Zeichen (lauter/leiser) |
|---|---|---|---|---|---|
| Zellen (Tag, Station) | 62 | 15,734 Hz | 0,206 Hz | +3,551 Hz | 47 Boden laut / 14 stark laut / 1 Gleichstand |
| Konjunktions-Untermenge (ε < 30°) | 55 | — | — | +6,121 Hz | 42 / 12 / 1 Gleichstand |
| Tag-Ebene (gepaarte Stationen je Tag gepoolt) | 48 Tage | — | — | +8,342 Hz | 38 / 9 / 1 Gleichstand |
| Station 14 | 20 | — | — | +0,102 Hz | 12 / 8 |
| Station 43 | 24 | — | — | +6,121 Hz | 20 / 4 |
| Station 63 | 18 | — | — | +52,984 Hz | 15 / 2 / 1 Gleichstand |

Gleichstand benannt (0 geehrt): in Tabelle 2 ist 47 + 14 = 61, nicht 62 — die 62. Zelle ist ein
Gleichstand (diff = 0) an Station 63 (dort 15 / 2 / 1 = 18 Zellen). Dieselbe Zelle liegt in der
Konjunktions-Untermenge (dort 42 / 12 / 1 = 55) und auf der Tag-Ebene (dort 38 / 9 / 1 = 48 Tage).
Ein Gleichstand ist ein reales Messergebnis, kein fehlendes Zeichen und keine dritte Richtung.

## Der Befund

**1. Auf demselben Tag ist der AGC-Boden lauter als das starke Plateau — der Mode-2-Split ist kein
Tag-Kompositions-Artefakt.** An 21 Tagen, die an einer Station sowohl Boden- als auch
starke-Proben tragen, ist der Boden in 19 von 22 Zellen lauter als das starke Plateau desselben
Tages (Zeichen-Test gegen p = 0,5: P(X ≥ 19 | n = 22) ≈ 4,3e-4; Tag-Ebene 18/21, P ≈ 7,4e-4).
Der Median der Tag-gepaarten Differenz liegt bei +2,620 Hz (Konjunktions-Untermenge +2,620 Hz,
18/2); die Median-Zellen liegen bei 2,878 Hz (Boden) gegen 0,126 Hz (stark) — ein Faktor ~23 am
selben Tag. Die F1-Grenze „verschiedene Kalendertage" ist damit gemessen und als Träger des
Boden-Rauschterms ausgeschlossen: die Differenz verschwindet nicht und flippt nicht, wenn der Tag
konstant gehalten wird.

**2. Alle drei Stationen zeigen die Richtung am selben Tag (Modus 2); Modus 1 trägt die
Station-14-Ausnahme.** Modus 2: Station 14 8/0 Zellen (Median +27,3 Hz), Station 43 5/2
(+2,16 Hz), Station 63 6/1 (+2,29 Hz). Modus 1: Station 43 20/4 (+6,12 Hz), Station 63 15/2
(+52,98 Hz) — Station 14 dagegen nur 12/8 (+0,10 Hz), die schwächste der drei. Diese
Mode-1-Station-14-Schwäche wiederholt das Stations-14-Negativ des In-Pass-Tests
(`befund-galileo-inpass-staerke-rampe`); in Modus 2 ist Station 14 dagegen 8/0 — die
Tag-gepaarte Messung trägt die Stations-14-Frage als offene Spur in die Grenzen.

**3. Die drei Gegen-Zellen sind Zustands-Zellen, kein Plateau-Widerspruch.** Die Zellen, in denen
das starke Plateau am selben Tag lauter war (Modus 2: 1995-11-28 Station 63 diff −15,99 Hz bei
stark-RMS 17,9 Hz; 1996-06-28 Station 43 diff −120,7 Hz bei stark-RMS 200,0 Hz und Boden-RMS
79,3 Hz — ein Nicht-Konjunktions-Tag; 1996-01-03 Station 43 diff −0,03 Hz) tragen starke RMS,
die weit über dem Plateau-Median 0,126 Hz liegen: dort war der starke Zustand selbst laut
(Pass-/Erwerbungs-Struktur), nicht das Plateau. Die Plateau-Ruhe (0,05–0,13 Hz) ist in den 19
Boden-lauten Zellen die Regel; die Tag-gepaarte Aussage ist eine Aussage über die beiden
Zustände, nicht über jede Pass-Struktur.

**Auf den Richtungs-Befund geantwortet:** Floor = genuiner angehobener-Rausch-Zustand (diese
Messung), kein Floor→Rauschen-Pfeil (G1, befund-galileo-inpass-richtung) — die Lautheit ist ein
Zustands-Marker, kein Treiber.

## Grenzen

- Pass-Struktur: die Paarung hält Tag und Station konstant, nicht die Pass-Identität. Boden- und
  starke Proben eines Tages können in verschiedenen Pässen desselben Tages liegen; die
  Tag-gepaarte Messung kontrolliert Ära/Geometrie, nicht die Pass-Grenze. Der In-Pass-Test bei
  Pass-Identität (`befund-galileo-inpass-staerke-rampe`) bleibt die feinere Kontrolle — an
  Stationen 43/63 positiv, an Station 14 negativ.
- Boden-Zellen sind oft kurz (30–100 Proben): der Tag-gepaarte Boden-Rauschterm wird häufig aus
  kurzen Ausflügen auf −2560 an Tagen gemessen, die sonst auf dem Plateau liegen. Ob diese
  lauten Boden-Proben an Pass-Kanten oder Pass-Enden kleben (zeitliche Struktur innerhalb des
  Tages), löst diese Probe nicht — `pending`.
- Station-14-Modus-1: 12/8 Zellen bei Median +0,10 Hz reproduziert den In-Pass-Befund (dort keine
  echte Kovarianz); in Modus 2 dagegen 8/0 (+27,3 Hz). Die Station-14-Diskrepanz zwischen den
  Modi ist benannt, nicht entschieden — `pending`.
- Richtung: −2560 ist der AGC-Klemm-Registerwert; ob die Klemmung das laute Rauschen begleitet
  oder einen akquisitionsnahen Zustand markiert, trennt auch die Tag-gepaarte Messung nicht —
  `pending`.
- Die 2 Nicht-Konjunktions-Tage in Modus 2 (1996-06-27/28, Tiefraum-Ära) tragen auf beiden
  Seiten sehr laute RMS (Boden 20,3/+79,3 Hz, stark 9,2/200,0 Hz); sie sind in den Zellen- und
  Tag-Zahlen enthalten, nicht geglättet.

## Register-Satz

*Der Mode-2-Stärke-Split ist kein Tag-Kompositions-Artefakt: an 21 Tagen, die an einer Station
Boden (−2560) und starkes Plateau (≥ −1750) gemeinsam tragen, ist der Boden in 19/22 Zellen
lauter als das starke Plateau desselben Tages (Median-Differenz +2,62 Hz, Boden-RMS ~23× das
starke RMS am selben Tag; Tag-Ebene 18/21). Die Tag-gepaarte Kontrolle des F1-pending ist damit
gemessen: der Boden-Rauschterm ist ein echter Stärke-Zustands-Effekt bei konstantem Tag, keine
Tages-Komposition. Modus 1 bestätigt (47/62 Zellen, 1 Gleichstand an Station 63, Median +3,55 Hz;
Station 14 dort schwach, 12/8).*

## Status

`done` — der Rat hat den Entwurf gehalten (Gleichstand benannt, G1-Querbezug) und die
Korrekturen eingearbeitet. Probe und Report sind additive lokale Lauf-Artefakte (neue Dateien),
nichts Bestehendes verändert. sha256 über den Körper ohne Kopf ist im Kopf gesetzt.
