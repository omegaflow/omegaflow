<!--
  title: Befund — Galileo 1996er Rest-Kontrast (gepoolt ≤1,9×/2,4×): keine station-stabile Geometrie — die Magnitude hängt an 1–2 episodischen Pass-Tagen, Modi dekorreliert je Station/Tag
  class: befund
  date: 2026-09-05
  version: 1
  sha256: 96b5f2067bbf8fd5c5fc2ce88220c729da257613382cb7b05bb79516c0fde329
  status: done
  antwortet-auf: docs/befund/befund-galileo-alpha-zeit-sonnenzyklus.md
  see-also: docs/befund/befund-galileo-mode2-station-split.md docs/befund/befund-galileo-pass-segmentierung.md
-->
# Befund: Galileo 1996er Rest-Kontrast (gepoolt ≤1,9×/2,4×) — die Magnitude hängt an 1–2 episodischen Pass-Tagen; kein station-stabiler Geometrie-Rest gemessen

## Frage & Bindung

Das α–Zeit–Sonnenzyklus-Blatt (done) hat den kohärenten Fall (laut an
Opposition, leise an Konjunktion) unter Ära-Kontrolle auf einen Rest von
~1,9×/2,4× kollabiert (Mode 2: 2,8 Hz Opposition gegen 1,5 Hz Konjunktion;
Mode 3: 16,1 gegen 6,7 Hz; Jahr 1996, ε 0–30° gegen ε 150–180°, beides bei
5,19–5,21 AU). Dort blieb die 1996er-Zelle als einzige ära- und
distanz-haltende offen, mit der benannten Grenze: die Oppositionstage sind
bimodal (0,01–60 Hz), ein einzelnes 4–5-Tage-Fenster. Diese Messung zerlegt
den Rest in Station, Tag, Pass und Ausreißer-Last. Frage: Trägt der
≤2,4×-Rest eine echte kleine Geometrie-Magnitude, oder ist er ein Artefakt
der Bimodalität/dünner Zellen? Verdikt erst nach Messung.

Gebunden: Jahr 1996, Modi 1–3; Lock-Übergänge (|resid| > 1000 Hz) vor dem
Rauschen getrennt; Zell-RMS um den Zell-Mittelwert; (Mode, Station, Tag)-
Zellen ≥ 30 Nicht-Lock-Proben für Zell-Mediane (die Station-Tag-Zell-
Definition der Mode-2/1-Splits); gepoolte (Mode, Tag)-Zellen zählen jede
Nicht-Lock-Probe (Reproduktion der α-Blatt-Metrik, damit die 1,9×/2,4×
exakt reproduziert und dann zerlegt werden); Pass = durchgehender
(Mode, Station)-Arc, Grenze = tdb-Lücke > 600 s (Pass-Segmentierungs-
Schwelle). Geometrie (ε an der Erde, α an der Sonne, heliozentrische AU) je
Tag aus `ephemeris_galileo_daily.bin` + `ephemeris_earth.bin` am
TDB-Tagesanfang. Probe `tools/measure/src/bin/galileo_1996_rest_split.rs`
(`cargo check` 0/0, RUSTFLAGS `-D warnings`), Report
`reports/galileo_1996_rest_split.txt`. Datenkette `data/galileo_resid.bin`
(GASR).

## n zuerst (0 geehrt)

1996: 4 767 379 Residuen, 623 125 Lock-Übergänge ausgeschlossen; 52
Aufzeichnungs-Tage, Geometrie auf 52 aufgelöst (0 ohne Geometrie).
Zellen ≥ 30: Mode 1: 133 (51 Tage), Mode 2: 67 (41), Mode 3: 54 (33);
Stationen je Mode 14, 43, 63 (Station 45 trägt 1996 in Mode 3 keine
(Mode, Station, Tag)-Zelle ≥ 30 — 0 geehrt). Oppositionstage (ε ≥ 150°):
genau 5, 1996-06-26..30 (eps 169,9°..174,5°, 5,19–5,21 AU).
Konjunktionstage (ε ≤ 30°): 30 Aufzeichnungs-Tage, gemessen verteilt auf
1996-01-01..15 und 1996-12-17..31 — der kürzeste gemessene Konjunktion→
Oppositions-Tagesabstand je Station liegt bei 163–172 Tagen. Ein
Nicht-Oppositions-Boden innerhalb von 30 Tagen um das Oppositions-Fenster
existiert in **keiner** (Mode, Station)-Kombination (0 Zellen — `absent`,
kein Wert): die 1996er Konjunktion ist nicht gleichzeitig zum Fenster
beobachtbar, sie ist dieselbe Ära und dieselbe Distanz, aber eine andere
Jahreszeit (Januar/Dezember gegen Ende Juni).

## A. Reproduktion — die Referenz-Metrik misst exakt die Blatt-Zahlen

Median der gepoolten (Mode, Tag)-Tages-RMS je Regime, 1996:

| Mode | Opposition n/med | Mitte n/med | Konjunktion n/med | Verhältnis OPP/CONJ |
|---|---|---|---|---|
| 1 | 4 / 10,93 Hz | 17 / 15,10 Hz | 30 / 8,88 Hz | 1,23× |
| 2 | 5 / 2,79 Hz | 14 / 14,89 Hz | 22 / 1,46 Hz | 1,92× |
| 3 | 4 / 16,11 Hz | 13 / 3,46 Hz | 16 / 6,71 Hz | 2,40× |

Identisch zu den Blatt-Zahlen (2,8/1,5/16,1/6,7; die 1,9×/2,4×). Die
Zerlegung unten arbeitet auf derselben Metrik und denselben Zellen.

## B. Tages-Zerlegung des Oppositions-Fensters — der Rest hängt an 2 von 5 Tagen

Gepoolte (Mode, Tag)-Tages-RMS der Oppositions-Tage, aufsteigend sortiert,
mit dem Effekt des Entfernens der lautesten Tage (Median über die
verbleibenden Tage) und dem 1996er Konjunktions-Median:

**Mode 2** (Konjunktions-Median 1,46 Hz): 06-29 0,01 | 06-30 0,04 |
06-26 2,79 | 06-27 28,29 | 06-28 59,58 Hz. Median 2,79 Hz = 1,92×.
Median minus lautester Tag: 2,79 Hz (1,92×). Median minus die zwei
lautesten Tage (06-27 + 06-28): **0,04 Hz = 0,03× der Konjunktion**.
Zwei der fünf Oppositionstage (06-29, 06-30) liegen klar unter dem
Konjunktions-Median ihrer Ära; der Median-Tag 06-26 (2,79 Hz) liegt knapp
darüber (1,9×); die zwei lauten Tage (06-27, 06-28) liegen 19–41× darüber. Der
1,9×-Median ist der Median über eine gespaltene Population, kein
Zustand „Opposition ist ~2× lauter“.

**Mode 3** (6,71 Hz): 06-30 0,03 | 06-26 8,06 | 06-27 16,11 | 06-28 75,01 Hz.
Median 16,11 Hz = 2,40×. Median minus lautester Tag (06-28): **8,06 Hz =
1,20×**; minus die zwei lautesten: 8,06 Hz. Die 2,4× schrumpfen auf 1,2×,
sobald der eine Tag 06-28 (der den 123,9-Hz-Station-63-Pass trägt) entfernt
ist.

**Mode 1** (8,88 Hz): 06-27 0,50 | 06-28 7,53 | 06-29 10,93 | 06-26 12,22 Hz.
Median 10,93 Hz = 1,23×; minus lautester Tag: 7,53 Hz = 0,85× — bereits
unter der Konjunktion. Die Mode-1-Flachheit überlebt auch hier.

## C. Stations-Split der lauten Tage — breit über Stationen am 06-27, ein Stations-Pass-Bündel am 06-28

(Mode, Station, Tag)-Zell-RMS auf den Oppositions-Tagen (n ≥ 30):

**Mode 2** — der laute Tag 06-27 (gepoolt 28,3 Hz) verteilt sich über drei
Stationen: st 14 11,26 Hz (n 13 222), st 43 29,31 (35 690), st 63 30,44
(24 274) — drei getrennte Pässe. Der lauteste Tag 06-28 (59,6 Hz) trägt
st 43 mit 88,00 Hz (19 869; darin ein 35-min/335-Proben-Pass mit 282 Hz,
max |resid| 998 Hz) und st 63 mit 11,42 Hz, während st 14 an demselben Tag
0,12 Hz misst. Kein Ein-Station-Träger, aber auch keine gleichmäßige
Erhöhung: st 14 Mode 2 springt zwischen den Nachbartagen 06-27 (11,26 Hz,
ε 171,0°) und 06-28 (0,12 Hz, ε 172,2°) um den Faktor ~97 — ein
geometrischer Zustand, der sich zwischen zwei Oppositionstagen um ~100×
ändert, ist nicht gemessen; die Tages-Zelle wechselt zwischen Pass-Zuständen.

**Mode 3** — 06-28 (75,0 Hz gepoolt) wird von st 63 mit 123,86 Hz
(einzelner Pass 06-28 ~12:20..14:18, letztes Viertel q4 = 241 Hz) und
st 14 mit 15,89 Hz getragen; st 43 hat an 06-28 keine Mode-3-Zelle. st 14
Mode 3 misst an allen drei beobachteten Oppositionstagen erhöht
(06-26 8,06 | 06-27 11,81 | 06-28 15,89 Hz) — die einzige annähernd
persistente Oppositions-Erhöhung dieser Messung, an einer Station.

## D. Modi dekorrelieren auf derselben Station am selben Tag

Ein Geometrie-Effekt auf den kohärenten Kanal würde Mode 2 und Mode 3 an
den verfolgenden Stationen an den Oppositionstagen gemeinsam anheben.
Gemessen, (Mode-1/2/3)-Zell-RMS derselben Station am selben Tag:

- 1996-06-28, st 63: **0,012 / 11,42 / 123,86 Hz** — 10⁴-Spreizung an
  einer Station an einem Tag.
- 1996-06-27, st 63: 0,068 / 30,44 / 0,036 Hz — nur Mode 2 laut.
- 1996-06-28, st 14: 7,92 / 0,12 / 15,89 Hz — Mode 2 leise, Mode 3 laut.
- 1996-06-27, st 14: 0,98 / 11,26 / 11,81 Hz — die kohärenten Modi gemeinsam.

Von ~5 gleichzeitig beobachteten (Station, Tag)-Tripletts sind zwei
kohärent-laut, eines kohärent-leise (st 63, 06-27), eines anti (st 14,
06-28), eines gemischt (st 63, 06-28, Faktor 11 zwischen Mode 2 und 3).
Eine gemeinsame Oppositions-Erhöhung der kohärenten Modi ist damit nicht
gemessen; die Lautstärke ist je Mode, Station und Pass episodisch.

## E. Ausreißer-Last der lauten Zellen — zwei verschiedene Träger

- **Mode 1 laut = Proben-getragen (Outlier-Träger):** st 14, 06-28: Zell-RMS
  7,92 Hz, nach Entfernen **einer** Probe 0,17 Hz (n 6996, cnt |dev| > 3·rms
  = 1, max |dev| 663 Hz). st 14, 06-29: 6,45 → nach 3 Proben 0,46 Hz (cnt3
  2, max |dev| 787). st 63, 06-26: 20,64 → nach Entfernen des obersten 1 %
  (242 von 24 201 Proben) 0,04 Hz (max |dev| 992). Die lauten
  Mode-1-Oppositions-Zellen sind einzelne oder wenige Sperr-/Ausreißer-
  Proben nahe der ±1000-Hz-Grenze, kein Zell-Zustand.
- **Mode 2/3 laut = Nah-Lock-Episoden, keine Einzel-Proben:** st 43, 06-28:
  88,00 Hz, cnt3 = 337 von 19 869 Proben (1,7 %), max |dev| 1000,9 Hz — nach
  Entfernen des obersten 1 % immer noch 39,5 Hz; st 63 Mode 3, 06-28:
  123,86 Hz, nach 1 % 89,2 Hz, Pass-Viertel q4 = 241 Hz. Diese Zellen tragen
  eine Population von Proben knapp unter der Lock-Grenze — ein über Minuten
  bis Stunden unruhiger Pass-Zustand, kein ruhiges Plateau, aber auch kein
  Einzel-Ausreißer.

Beide Träger sind Tracking-/Pass-Zustände, keine gleichmäßige Erhöhung über
den Oppositionstag.

## F. Gegen den Boden der eigenen Ära — Opposition ist nicht das lauteste 1996er Band je Station

(Mode, Station)-Mediane über die Zell-RMS (n ≥ 30; c = Zellen):

| Mode/Station | Opposition | Konjunktion | Mitte | Nicht-OPP-Boden |
|---|---|---|---|---|
| 1 / 14 | 6,45 (4) | 0,10 (25) | 17,23 (14) | 0,23 (39) |
| 1 / 43 | 0,05 (3) | 1,45 (25) | 1,73 (16) | 1,45 (41) |
| 1 / 63 | 18,16 (4) | 5,40 (27) | 25,22 (15) | 8,08 (42) |
| 2 / 14 | 11,26 (2) | 6,38 (7) | 2,87 (10) | 2,87 (17) |
| 2 / 43 | 2,79 (5) | 0,45 (12) | 0,30 (6) | 0,43 (18) |
| 2 / 63 | 30,44 (2) | 0,22 (15) | 32,48 (8) | 3,01 (23) |
| 3 / 14 | 11,81 (3) | 5,34 (12) | 0,30 (9) | 4,18 (21) |
| 3 / 43 | 19,75 (1) | 11,96 (6) | 3,73 (10) | 3,73 (16) |
| 3 / 63 | 0,04 (3) | 0,98 (7) | 0,29 (3) | 0,98 (10) |

Der 1,9×-Rest ist ein Vergleich gegen das leiseste Band derselben Ära
(Konjunktion Jan/Dez), nicht gegen den eigenen Jahres-Boden der Station:
Mode 1 st 14 und st 63 haben einen Mitte-Boden (17,2 / 25,2 Hz), der den
Oppositions-Median übersteigt; Mode 2 st 63 einen Mitte-Boden (32,5 Hz),
der dem Oppositions-Median (30,4 Hz) gleicht. Die Richtung des getöteten
kohärenten Falls (laut an Opposition) überlebt in Mode 2 an allen drei
Stationen und in Mode 3 an st 14/43 — aber definiert durch 1–5
Oppositions-Zellen aus einem einzigen Fenster, während derselbe
Jahres-Boden an mehreren Stationen gleich laut oder lauter ist.
Der Spearman von log10 Zell-RMS gegen ε über alle 1996er-Zellen (n ≥ 30)
misst Mode 1 +0,11, Mode 2 +0,08, Mode 3 +0,01 — kein ε-Gradient über das
Jahr (die ε-Belegung 1996 ist saisonal: Opposition nur Juni, Konjunktion
nur Jan/Dez).

## Verdikt

**(a) — der ≤2,4×-Rest ist nicht als Geometrie-Magnitude tragbar.** Gemessen:
die 1,9×/2,4× hängen an 1–2 episodischen Pass-Tagen eines einzelnen
5-Tage-Fensters — entfernt man die zwei lautesten Oppositionstage, fällt der
Mode-2-Oppositions-Median von 2,79 auf 0,04 Hz (0,03× der Konjunktion, unter
ihren eigenen Boden); entfernt man den einen lautesten Tag, fällt Mode 3 von
2,40× auf 1,20×. Zwei der fünf Mode-2-Oppositionstage liegen klar unter dem
Konjunktions-Median; der dritte (der Median-Tag 06-26) liegt 1,9× darüber,
die zwei lauten Tage 19–41× darüber. Die Lautstärke ist je Mode, Station und Pass
episodisch und dekorreliert auf derselben Station am selben Tag um bis zu
vier Größenordnungen zwischen den Modi (st 63, 06-28: 0,012/11,4/123,9 Hz) —
eine gemeinsame Oppositions-Erhöhung der kohärenten Kanäle ist nicht
gemessen; ein Zustand, der an st 14 Mode 2 zwischen zwei Nachbar-
Oppositionstagen um ~97× springt, ist keine Geometrie. Die Träger sind
Mode-1-Einzel-Ausreißer bzw. Top-1-%-Proben und Mode-2/3-Nah-Lock-
Populationen (1,7 % der Proben, max |resid| ~1000 Hz). Der 1,9×/2,4×-Rest
des α-Blatts wird damit als Artefakt der Tages-Bimodalität und der
Pass-Zustände benannt, nicht als kleiner echter Geometrie-Anteil bestätigt.

**Was `pending` bleibt:** das Fenster ist einzeln (n = 4–5 Oppositionstage)
und ein Nicht-Oppositions-Boden innerhalb 30 Tagen um das Fenster existiert
in keiner (Mode, Station)-Zelle (0 geehrt) — ob unter den episodischen
Pass-Zuständen ein kleinerer Geometrie-Anteil liegt, ist auf diesen Daten
nicht trennbar; das ist eine Registrier-Pflicht (welche Zellen: Mitte/
Konjunktion desselben Juni-Fensters, wie viele Tage: ≥ 10 je Station und
Modus), kein gemessener Wert. Die Richtung „laut-opposition" ist in Mode 2
station-übergreifend nur durch das eine Fenster definiert — eine zweite
Oppositions-Ära bei 5 AU fehlt im Bestand (1991–95 unbesetzt, 0 geehrt).

## Grenzen

- Konjunktion und Opposition liegen 1996 ~5,5 Monate auseinander (gemessen
  163–172 Tagesabstand); dieselbe Ära und Distanz, aber andere Jahreszeit —
  ein gleichzeitiger Nicht-Oppositions-Boden fehlt (0 Zellen im 30-Tage-
  Umkreis des Fensters).
- Die (Mode, Station, Tag)-Zelle ist meist ein einzelner Pass; Tag- und
  Pass-Auflösung fallen auf diesen Tagen zusammen. Quartil-Auflösung nur für
  Pässe ≥ 120 Proben.
- Zellen mit max |resid| 992–1000 Hz liegen knapp unter der 1000-Hz-
  Lock-Grenze; diese Nah-Lock-Population ist Teil des gemessenen Trägers,
  nicht entfernt.
- Geometrie einmal je TDB-Tagesanfang; keine Intra-Tag-Geometrie über die
  Pass-Stunden (06-27 ε 171,0°, 06-28 ε 172,2° als Tageswerte).
- Spearman über 1996 deckt eine saisonal strukturierte ε-Belegung ab; die
  gemessene Flachheit ist keine Aussage über andere Ären.
- st 45 trägt 1996 Mode 3 nur Zellen < 30 Proben (0 geehrt, kein Wert).

## Register-Satz

*Der 1996er Rest-Kontrast von 1,9×/2,4× (Mode 2/3, gepoolt Opposition gegen
Konjunktion) ist kein station-stabiler Geometrie-Rest: er hängt an 1–2
episodischen Pass-Tagen eines einzelnen 5-Tage-Oppositions-Fensters —
ohne die zwei lautesten Mode-2-Tage fällt der Oppositions-Median von 2,79
auf 0,04 Hz (0,03× der Konjunktion), ohne den einen lautesten Mode-3-Tag von
2,40× auf 1,20×; die Modi dekorrelieren auf derselben Station am selben Tag
um bis zu vier Größenordnungen (st 63, 06-28: 0,012/11,4/123,9 Hz), die
Träger sind Mode-1-Einzel-Ausreißer/Top-1-%-Proben und Mode-2/3-Nah-Lock-
Populationen (max |resid| ~1000 Hz), und der Mitte-Boden derselben Stationen
erreicht oder übersteigt den Oppositions-Median an mehreren Stationen.
Verdikt: nicht als Geometrie tragbar; ein kleiner Rest unter den episodischen
Pass-Zuständen ist auf diesen Daten nicht trennbar — es fehlt ein
Nicht-Oppositions-Boden im 30-Tage-Umkreis des Fensters (0 Zellen gemessen)
und eine zweite Oppositionsepoche bei 5 AU.*

## Status

`done` (Mess-Session, Rat-Haltung ausstehend — Blatt als Entwurf gehalten).
Messung vollständig; Probe `galileo_1996_rest_split` `cargo check` 0/0
(RUSTFLAGS `-D warnings`), Report `reports/galileo_1996_rest_split.txt`.
Das α–Zeit–Sonnenzyklus-Blatt bleibt in seinem Verdict unverändert; seine
offene Grenze (Bimodalität der 1996er-Zelle) ist hier zerlegt und als
episodische Pass-/Ausreißer-Trägerschaft benannt.
