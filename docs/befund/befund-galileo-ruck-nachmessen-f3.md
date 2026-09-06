<!--
  title: Befund — F3-Ruck nachgemessen (Richtung D5): der −0,80…−0,82-Hz-Mode-1-Niveau-Sprung 1995-11-30/12-01 hat einen Versatz BESEITIGT statt erzeugt — das +0,75-Hz-Öffnungs-Plateau (erste Messwoche) war der Versatz, die resid-Lage danach ist ≈ 0 (Dez-95-Folge-Körper +0,026…+0,033 Hz, mean −0,029…+0,024; Ära-Post-Median +0,02…+0,15 Hz mit der Saison-Struktur des Drift-Befunds, keine Rückkehr 0–1 von 26–27 Tagen); M2/M3-Deckung am Übergang ist echt, wo Zellen existieren (ruhig-robust auf beiden Seiten der Grenze: M2 st14 Δ −0,018 Hz, M2 st63 Δ −0,054 Hz, M3 st43 Δ −0,004 Hz, n je 10³) und Lücke/laut, wo nicht (M2 st43 12-01…03 laut, M3 st14 11-30…12-02 laut und 12-03 leer, M3 st63 11-30 leer n 0); der Change-Point ist datengetrieben (Ganz-Ära-Ein-Schnitt-Scan über alle Schnitte, keine Datums-Vorgabe), in drei unabhängigen M1-Serien am selben Kalender-Übergang reproduziert, 6–7 Tage nach dem Daten-Beginn 1995-11-23 und 454–456 Tage vor dem Daten-Ende — nicht an der Daten-/Era-Kante, ohne das Plateau (≥ 12-01) bleibt kein 0,5-Hz-Schnitt; der 23.05.1996 (DGT) trägt keinen M1-Niveau-Sprung — er liegt in der bodenleeren Spanne Feb–Mai 1996 (0 Floor-Samples, 0 geehrt), nächste ruhige Tage 168–169 d davor (Dez-95, ≈ +0,03 Hz) und 34–35 d danach (Jun-96, +0,21…+0,40 Hz); die Übergangs-Liste der F3-Zeile bleibt beim 11-30/12-01-Sprung
  class: befund
  date: 2026-09-06
  sha256: b9b6d1c98f4bfde64a35a8c3b55db704a2401b9d6b3a27f4323c1ebc66f3474d
  status: draft
  see-also: docs/befund/befund-galileo-ruhige-basis-ruck-stufe.md docs/befund/befund-galileo-ruhige-basis-drift.md docs/befund/befund-galileo-ruck-untersuchung-zeugen.md docs/befund/befund-galileo-h1-receiver-regression.md
-->
# Befund: F3-Ruck nachgemessen (Richtung D5) — der −0,8-Hz-Sprung 1995-11-30/12-01 hat den +0,75-Hz-Öffnungs-Versatz beseitigt (resid danach ≈ 0); M2/M3-Deckung am Übergang teils echt (Gegen-Daten) teils Lücke/laut; der Change-Point ist datengetrieben, drei-Stationen-reproduziert und nicht an der Era-Kante; der 23.05.1996 (DGT) trägt keinen M1-Sprung

## Frage & Bindung

Dieser Lauf (Richtung D5) schärft die **F3-Zeile des Befunds `befund-galileo-floor-pass-episodisch`**
(der eine gemessene Ruck: −0,80…−0,82 Hz, Mode 1, simultan st14/43/63, anhaltend)
mit vier gezielten Messungen auf der Tages-Niveau-Reihe der ruhigen robusten
Mode-1-Basiszellen. Gebunden wie die Vorlagen (Ruck-Befund `b8226ab`, Drift-Befund
`fad752b`, Zeugen-Befund `a435c56`): Tages-Zelle (Mode, Station, Tag) über die
in-track-Boden-Residuen (Stärke exakt −2560, |resid| ≤ 1000 Hz); robust = Zell-n ≥ 30;
ruhig = Zell-RMS < 1 Hz, laut = ≥ 1 Hz; Tag = gerundeter TDB-Zivil-Tag (der mit D
gelabelte Tag umspannt UTC D−1 12:00 bis D 12:00); Tages-Niveau = Tages-Median der
Residuen der ruhigen Zelle. Der Sprung liegt pass-zu-pass zwischen 1995-11-30 03:17
und 18:20 UTC (Zeugen-Befund) — er ist damit im Tag-Label 12-01 gemessen (die erste
niedrige Zelle von st14/st63 trägt das Label 12-01, von st43 12-02). „Nach dem
Sprung" heißt hier: Folge-Ruhig-Tage ab dem ersten niedrigen Messtag (Label ≥ 1995-12-01).

Die vier Fragen, gemessen (nicht geraten):

1. **Landet das Niveau nach dem Sprung nahe null?** (GLM-Arithmetik: +0,75 − 0,8 ≈
   −0,05.) Messe die M1-ruhige-Basis je st14/43/63 nach dem 01.12.1995 mit Streuung/n.
2. **M2/M3-Deckung am Übergang (1995-11-28..12-03):** Gegen-Daten oder Lücke? Zeige je
   (Tag, Station, Mode 2/3) die n und das Niveau über die Grenze; n = 0 ehrlich als
   Lücke.
3. **Segmentierungs-Provenienz:** datengetrieben, über die drei Stationen reproduziert,
   nicht an der Daten-/Era-Kante?
4. **23.05.1996 (DGT-Übergang):** trägt er einen M1-Niveau-Sprung? Vorher/Nachher wie
   beim 11-30/12-01-Sprung; die Mitte-1996-Fenster sind datenärmer (Feb–Mai 96
   bodenleer) — 0 geehrt.

Probe `tools/measure/src/bin/galileo_ruck_nachmessen_f3.rs` (neu; `cargo check` 0/0,
RUSTFLAGS `-D warnings`), Report `/tmp/opencode/galileo_ruck_nachmessen_f3_report.txt`.
Daten: `data/pds-ppi.igpp.ucla.edu/galileo_resid.bin`.

## n zuerst (0 geehrt) — Zensus und Reihen

Der Zensus reproduziert das Register exakt: **400 robuste (Mode, Station, Tag)-Zellen,
207 laut / 193 ruhig**; 4 647 387 in-track-Floor-Samples der Ära, 1 815 098
Lock-Samples ausgeschlossen. Die M1-Ruhig-Tage je Serie: st14 30 (Spanne 1995-11-25
.. 1997-02-28), st43 29 (1995-11-24 .. 1997-02-28), st63 34 (1995-11-23 ..
1997-02-26) — exakt die Zahl der Vor-Befunde. Das Q1-Fenster (Dez-95-Folge-Körper,
Label ≥ 1995-12-01 und ≤ 1995-12-31) hält an ruhigen M1-Tagen: st14 n 6, st43 n 2
(12-02, 12-06), st63 n 5. Das Q4-Fenster um den 23.05.1996 ist **bodenleer** (siehe
Messung 4): Februar–Mai 1996 tragen **0** Floor-Samples auf den Trio-Stationen (0
geehrt), Januar 1996 nur 241 (davon 34 Mode-1).

## Messung 1 — das Niveau nach dem Sprung: ≈ 0, der Sprung hat einen Versatz beseitigt

Je M1-Serie: Plateau-Median (ruhige Tage ≤ 1995-11-30, der Zero-Spread-Tag st63
1995-11-30 mit rms 0 ausgeschlossen), der unmittelbare Folge-Körper Dezember 1995 und
der ganze Post-Körper (Label ≥ 1995-12-01):

| Serie | Plateau (n) Median Hz | Dez-95-Folge (n) Median/Mean Hz | Post-Ära ≥ 12-01 (n) Median/Mean/SD Hz | Rückkehr ≥ Plateau-Median |
|---|---|---|---|---|
| M1 st14 | +0,775 (4) | n 6 +0,032 / +0,017 | n 26 +0,043 / +0,153 / 0,248 | 1/26 (nur 1997-01-22) |
| M1 st43 | +0,719 (3) | n 2 +0,033 / −0,029 | n 26 +0,149 / +0,115 / 0,157 | 0/26 |
| M1 st63 | +0,741 (6) | n 5 +0,026 / +0,024 | n 27 +0,021 / +0,045 / 0,145 | 0/27 |

Die **unmittelbaren Folge-Ruhig-Tage (Dezember 1995) liegen bei +0,026…+0,033 Hz
(Median) bzw. −0,029…+0,024 Hz (Mittel) — das ist ≈ 0** innerhalb der ruhigen
Streuung, und es ist genau die GLM-Erwartung +0,75 − 0,8 ≈ −0,05. Der ganze
Post-Körper (Dez 95 bis Feb 97) liegt mit Median +0,02…+0,15 Hz **weit unter dem
+0,7…+0,85-Plateau** und kehrt nicht zurück (0–1 von 26–27 Tagen erreichen den
Plateau-Median; nur der Einzel-Transient 1997-01-22). Die leicht positive Lage des
Ära-Post-Medians (st43 +0,149) ist die Saison-Struktur des Drift-Befunds (Mitte/Ende
1996 um +0,2…+0,4 Hz, 1997 ≈ 0), nicht eine Rückkehr zum Plateau.

**Deutung mit der Zahl: der Sprung hat einen Versatz BESEITIGT, nicht erzeugt.** Das
+0,7…+0,85-Hz-Niveau war ein Versatz der ersten Messwoche (nur die Ära-Öffnung 23.–30.11.
trägt es, in M2/M3 steht es nicht); der −0,8-Hz-Schritt führt die resid-Lage auf ≈ 0
zurück. Die Alternative „der Sprung erzeugt einen +0,75-Hz-Versatz" ist gemessen
widerlegt: kein Folge-Tag (0–1 von 26–27) hält das Plateau-Niveau.

## Messung 2 — M2/M3-Deckung am Übergang 1995-11-28..12-03: echt, wo Zellen existieren; Lücke/laut, wo nicht

Zellen des Übergangsfensters je Serie (n | Tages-Median Hz | rms Hz | Klasse);
„leer" = kein Floor-Sample (0 geehrt):

| Serie | 1995-11-30 | 1995-12-01 | 1995-12-02 | 1995-12-03 | ruhig-robust auf beiden Seiten der Grenze |
|---|---|---|---|---|---|
| M2 st14 | n 13562, +0,190, ruhig | n 13947, +0,172, ruhig | n 10892, rms 36,7 laut | n 10253, +0,077, ruhig | **ja** (11-30 → 12-01, Δ −0,018 Hz) |
| M2 st43 | n 8969, +0,178, ruhig | n 37623, +0,162, laut | n 22114, +0,140, laut | n 44943, +0,069, laut | nein (12-01…03 laut, Zellen vorhanden) |
| M2 st63 | n 17886, +0,200, ruhig | n 3067, +0,146, ruhig | n 19868, +0,139, ruhig | n 13358, +0,102, laut | **ja** (11-30 → 12-01, Δ −0,054 Hz) |
| M3 st14 | n 6155, +0,196, laut | n 12522, +0,180, laut | n 10674, +0,141, laut | n 0, leer | nein (11-30…12-02 laut, 12-03 leer) |
| M3 st43 | n 12464, +0,189, ruhig | n 7303, +0,185, ruhig | n 6266, +0,155, laut | n 13849, +0,084, ruhig | **ja** (11-30 → 12-01, Δ −0,004 Hz) |
| M3 st63 | n 0, leer | n 4859, +0,183, laut | n 5159, +0,107, ruhig | n 5078, +0,071, laut | nein (11-30 leer, Nachbar laut) |

Die benachbarten Tage im Fenster (11-28/11-29) tragen an st14/st43 M2- und M3-Zellen
teils ruhig, teils laut (M2 st14 11-28/29 ruhig; M2 st43 11-28/29 ruhig; M2 st63
11-29 ruhig; M3 st14 11-29 ruhig; M3 st43 11-28/29 ruhig; M3 st63 11-29 laut).

**Antwort: beides, gemessen — echte Gegen-Daten, wo die Zellen existieren, und
Lücke/laut, wo nicht.** Drei Serien tragen **ruhig-robuste Zellen direkt auf beiden
Seiten der 11-30/12-01-Grenze** mit n im Tausend-Bereich und kontinuierlichem Niveau
(M2 st14 −0,018 Hz, M2 st63 −0,054 Hz, M3 st43 −0,004 Hz über die Grenze) — das ist
dichte, flache Gegen-Daten zur −0,8-Hz-M1-Stufe an denselben Kalender-Tagen. Zwei
Serien gehen unmittelbar nach der Grenze in die laut-Klasse (M2 st43 ab 12-01, M3 st14
ab 11-30; Zellen mit n 6 000–45 000 vorhanden, Mediane +0,14…+0,18, **kein**
−0,6-Hz-Ausflug); M3 st63 hat am 11-30 **n 0** (leer, 0 geehrt) mit lautem Nachbar
(12-01) und erst 12-02 ruhig (+0,107). Für diese drei Serien ist die Seite nach bzw.
vor der Grenze als laut/leer benannt — sie ist dort **keine Gegen-Daten**, aber auch
kein Widerspruch: kein M2/M3-Tag des Fensters trägt einen Niveau-Abfall der
Ruck-Klasse (−0,6…−0,8 Hz). Die Mode-1-Spezifität des Rucks steht damit auf den drei
dichten ruhig-ruhig-Paaren; die lauten/leeren Zellen begrenzen die Aussage auf die
Serien, die sie tragen.

## Messung 3 — Segmentierungs-Provenienz: datengetrieben, drei Stationen, nicht an der Era-Kante

Methode (wie im Ruck-Befund): Ein-Schnitt-Zwei-Segment-Modell (stückweise konstant,
mindestens 3 ruhige Tage je Segment) auf der **chronologischen Ruhig-Tag-Reihe der
ganzen Floor-Ära**; der Scan bewertet **jeden zulässigen Schnitt** (Index von 3 bis
n−3) und nimmt den Schnitt mit maximalem |Segment-Mittel-Sprung| — er setzt kein Datum
voraus. Permutations-Null = Lauf-Block-Neuordnung (1999 Züge).

| Serie | bester Ganz-Ära-Schnitt | nL/nR | Sprung Segment-Mittel Hz | t | p_perm | Tage nach Daten-Beginn 23.11. | Tage vor Daten-Ende 28.02.97 |
|---|---|---|---|---|---|---|---|
| M1 st14 | 1995-11-30 ⇄ 12-01 (im Lauf) | 4/26 | −0,615 | −4,86 | 0,0030 | 7 | 455 |
| M1 st43 | 1995-11-30 ⇄ 12-02 (Lücke 12-01) | 3/26 | −0,637 | −6,88 | 0,0095 | 7 | 454 |
| M1 st63 | 1995-11-29 ⇄ 11-30 (im Lauf) | 6/28 | −0,700 | −11,73 | 0,0045 | 6 | 456 |

**Der Schnitt ist datengetrieben und liegt nicht an einer Kante.** Der Daten-/Era-
Beginn ist 1995-11-23; der Change-Point liegt **6–7 Tage danach** (die linke Seite ist
ein aufgelöster Ruhig-Tag-Lauf der ersten Messwoche, nL 3–6, kein Datenrand) und
**454–456 Tage vor dem Daten-Ende**. Der Scan über die ganze Ära findet ihn in drei
**unabhängigen** Station-Serien am **selben Kalender-Übergang** (st14/st43 letzter
Plateau-Tag 11-30, st63 11-29 — st63s bester Schnitt fällt auf den Zero-Spread-Tag
11-30 als rechte Seite, der Niveau-Abfall selbst ist 11-29 +0,848 → 12-01 +0,046,
−0,802 Hz). Die Fensterwahl konstruiert das Datum nicht: die Reihe hat vor dem Schnitt
eine volle Woche und nach dem Schnitt 14 Monate Daten, und der Scan hätte jeden
späteren Tag wählen können.

Zwei Stabilitäts-Messungen stützen die Datums-Zuordnung:

- **Ohne das Plateau (Post-Körper ≥ 1995-12-01)** bleibt kein 0,5-Hz-Schnitt: die
  besten Schnitte des Post-Körpers liegen bei −0,13…−0,21 Hz (st14 1997-02-15 ⇄
  02-20, st43 1997-02-15 ⇄ 02-25, st63 1997-02-12 ⇄ 02-20) — die einzige
  0,5-Hz-Klasse-Stufe der M1-Serie ist der Übergang vom Öffnungs-Plateau zum Körper.
- **Plateau-Tag-Jackknife** (je einen Öffnungs-Ruhig-Tag entfernt, Ära neu gescannt):
  st14 und st63 behalten den Schnitt am Übergang bei jedem Wurf (st14 11-30 ⇄ 12-01
  bzw. beim Entfernen des 11-30: 11-28 ⇄ 12-01; st63 11-29 ⇄ 11-30 bzw. 11-27 ⇄
  11-30); st43 hat nur nL 3 — jeder einzelne Wegfall macht die linke Seite < 3
  (min-Segment-Grenze), der Scan wandert dann auf den kleineren 12-02 ⇄ 12-06-Sprung
  (−0,37…−0,41 Hz). Die st43-Kante ist damit die dünnste (nur drei ruhige
  Plateau-Tage 24./25./30.11.), die Datums-Aussage selbst (der −0,800-Hz-Nachbar-Tag
  11-30 → 12-02) bleibt eine Messung zwischen zwei ruhigen Messtagen über den einen
  leeren 12-01.

## Messung 4 — der 23.05.1996 (DGT) trägt keinen M1-Niveau-Sprung: die Grenze liegt in der bodenleeren Spanne

Monats-Zensus der in-track-Floor-Samples (Trio-Stationen, Mode 1..3; 0 geehrt):

| Monat | alle Modi | nur Mode 1 |
|---|---|---|
| 1995-11 | 636 690 | 135 526 |
| 1995-12 | 598 859 | 106 981 |
| 1996-01 | 241 | 34 |
| **1996-02 … 1996-05** | **0** | **0** |
| 1996-06 | 364 690 | 177 573 |
| 1996-09 | 366 858 | 70 110 |
| 1996-11 | 909 062 | 494 387 |
| 1996-12 | 841 221 | 471 463 |
| 1997-01 | 14 574 | 11 864 |
| 1997-02 | 915 192 | 620 537 |

(Juli–August 1996 und Oktober 1996 ebenfalls 0.) Der **DGT-Start 23.05.1996 liegt
mitten in der bodenleeren Spanne Februar–Mai 1996** — es gibt kein M1-Sample in
±5 Wochen um das Datum, also keine Vorher-Messung nahe der Grenze (0 geehrt).

Je M1-Serie die nächsten ruhigen Tage und die Niveau-Blöcke vor/nach dem Datum:

| Serie | letzte ruhige Zelle vor DGT | Abstand | erste ruhige Zelle nach DGT | Abstand | Block vor (Dez-95) n / Median Hz | Block nach (> 23.05.96) n / Median Hz | bester Schnitt im ±90-d-Fenster bei ±21 d von DGT? |
|---|---|---|---|---|---|---|---|
| M1 st14 | 1995-12-07 (n 53, −0,016) | 168 d | 1996-06-26 (n 19 083, +0,224) | 34 d | n 6 / +0,032 | n 20 / +0,188 | nein (1997-02-15 ⇄ 02-20) |
| M1 st43 | 1995-12-06 (n 23 298, −0,090) | 169 d | 1996-06-26 (n 25 792, +0,209) | 34 d | n 2 / +0,033 | n 24 / +0,169 | nein (1996-06-29 ⇄ 09-06) |
| M1 st63 | 1995-12-06 (n 17 400, +0,001) | 169 d | 1996-06-27 (n 4 557, +0,231) | 35 d | n 5 / +0,026 | n 22 / +0,007 | nein (1996-11-08 ⇄ 11-28) |

Ruhige M1-Tage innerhalb ±40 d um den 23.05.1996: st14 2, st43 3, st63 2 (alle nach
dem Datum, 34–35 d danach); innerhalb ±90 d ebenso nur nachgelagerte. **Kein Schnitt
der M1-Serien liegt innerhalb ±21 d des 23.05.1996.**

**Antwort: der 23.05.1996 trägt keinen M1-Niveau-Sprung — messbar ist an diesem Datum
nichts (n = 0 in Feb–Mai 96, 0 geehrt).** Die vor dem Datum nächstliegende ruhige
Basis ist der Dez-95-Körper (≈ +0,03 Hz, 168–169 d davor), die erste danach der
Jun-96-Körper (+0,21…+0,40 Hz, 34–35 d danach); der Unterschied (grob +0,1…+0,2 Hz,
bei st63 ≈ 0) liegt **über eine sechsmonatige, sample-lose Spanne verteilt** und ist
die Saison-Lage-Struktur des Drift-Befunds (1996-Mitte hoch, 1997 ≈ 0) — kein
beobachteter Sprung an einem Datum und nicht die −0,8-Hz-Klasse. Damit bleibt die
Übergangs-Liste der F3-Zeile beim **einen** gemessenen M1-Sprung 1995-11-30/12-01.

## Größenordnung & Einordnung

Die vier Messungen schärfen die F3-Zeile so: (a) der −0,8-Hz-Sprung **entfernt** den
+0,7…+0,85-Hz-Öffnungs-Versatz (die resid-Lage danach ≈ 0; GLM +0,75 − 0,8 ≈ −0,05
reproduziert den gemessenen Dez-95-Körper +0,03) — die F3-Aussage „anhaltend, keine
Rückkehr" ist damit präzisiert: das Plateau war der transiente Versatz der ersten
Messwoche, kein Dauer-Zustand, zu dem eine Rückkehr zu erwarten wäre; (b) die
M2/M3-Gegen-Daten am Übergang sind **echt** an den drei dicht ruhig-ruhig-Paaren
(M2 st14/st63, M3 st43) und **lücken/laut** an den übrigen (M2 st43, M3 st14, M3 st63
— benannt, nicht als Gegen-Daten gezählt); (c) der Change-Point ist datengetrieben
(ganz-Ära-Scan über alle Schnitte, keine Datums-Vorgabe), reproduziert in drei
unabhängigen M1-Serien am selben Kalender-Übergang, 6–7 Tage nach Daten-Beginn und
454–456 Tage vor Daten-Ende — kein Konstrukt der Fensterwahl, keine Era-Kante; (d) der
DGT-Meilenstein 23.05.1996 trägt keinen M1-Sprung (n = 0 in Feb–Mai 96), die
Übergangs-Liste bleibt beim 11-30/12-01-Event.

## Verdikt

**Der eine gemessene M1-Ruck (1995-11-30/12-01, −0,80…−0,82 Hz) ist ein
Versatz-beseitigender Sprung, kein versatz-erzeugender: das +0,7…+0,85-Hz-Öffnungs-
Plateau war der transiente Versatz der ersten Messwoche, die resid-Lage danach ist
≈ 0** (Dez-95-Folge-Körper je Station n 2–6, Median +0,026…+0,033 Hz bzw. Mittel
−0,029…+0,024 Hz; ganze Post-Ära-Median +0,02…+0,15 Hz mit der Drift-Saison-Struktur;
keine Rückkehr 0–1 von 26–27 Tagen). **Die M2/M3-Deckung am Übergang ist teils echte
Gegen-Daten, teils Lücke:** M2 st14 (Δ −0,018 Hz), M2 st63 (Δ −0,054 Hz) und M3 st43
(Δ −0,004 Hz) tragen ruhig-robuste Zellen (n 10³–10⁴) auf beiden Seiten der
11-30/12-01-Grenze mit flachem Niveau — das Mode-Argument steht dort gemessen; M2 st43
und M3 st14 gehen unmittelbar nach der Grenze in die laut-Klasse (Zellen vorhanden,
Mediane +0,14…+0,18, kein −0,6-Hz-Ausflug) und M3 st63 hat am 11-30 n 0 (0 geehrt) —
dort ist die Seite laut/leer, benannt als Lücke, nicht als Gegen-Daten. **Der
Change-Point ist datengetrieben** (Ein-Schnitt-Zwei-Segment über die ganze Ära, alle
Schnitte bewertet; p_perm 0,0030/0,0095/0,0045), in drei unabhängigen M1-Serien am
selben Kalender-Übergang reproduziert, 6–7 Tage nach dem Daten-Beginn 1995-11-23 und
454–456 Tage vor dem Daten-Ende — **nicht an der Daten-/Era-Kante**; ohne das Plateau
(≥ 12-01) bleibt kein 0,5-Hz-Schnitt (Post-Körper-Maximum −0,13…−0,21 Hz), und der
Plateau-Tag-Jackknife hält den Schnitt (st14/st63 bei jedem Wurf; st43 mit nL 3 an der
min-Segment-Grenze, benannt). **Der 23.05.1996 (DGT) trägt keinen M1-Niveau-Sprung**:
das Datum liegt in der bodenleeren Spanne Feb–Mai 1996 (0 Floor-Samples, 0 geehrt),
nächste ruhige M1-Tage 168–169 d davor (Dez-95, ≈ +0,03 Hz) und 34–35 d danach (Jun-96,
+0,21…+0,40 Hz); kein M1-Schnitt liegt innerhalb ±21 d — die Übergangs-Liste bleibt
beim 11-30/12-01-Sprung. 0 geehrt: Feb–Mai 1996, Jul–Aug 1996, Okt 1996 liegen als
leere Monate, M3 st63 11-30 und M3 st14 12-03 als leere Zellen im Fenster — nie als
Werte.

**Was `pending` bleibt:** die Echt-vs-Modell-Trennung des Sprungs (Zeugen-Befund —
ein realer Empfangs-/S/C-Frequenzschritt vs. eine sub-Hz-Predict-/Modell-Korrektur;
der hier gemessene Befund, dass der Sprung einen Öffnungs-Versatz auf ≈ 0 führt,
benennt die Richtung des Niveau-Wechsels, nicht seinen Sitz); die Ursache des
+0,7…+0,85-Hz-Öffnungs-Versatzes selbst (Anfangs-Predict-/Kalibrier-Fehler der ersten
Messwoche oder vor-Ära-Zustand — die Ära beginnt 1995-11-23, vor-Ära-Niveau ungemessen);
warum die mittlere-1996-Saison-Lage (+0,2…+0,4 Hz) über dem Dez-95-/1997-Niveau steht
(Drift-Befund, Sitz unbenannt); die Ursache der laut-Klasse an M2 st43/M3 st14
unmittelbar nach der Grenze und des st63-Null-Passes 1995-11-30. Der Verdikt bleibt
`draft` (Entwurf für die Haupt-Session/den Rat).

## Grenzen

- Das Q1-„≈ 0" gilt für den unmittelbaren Dez-95-Folge-Körper (n 2–6 je Station, st43
  nur 2 Tage) und für den Ära-Post-Median (+0,02…+0,15 Hz) — die leicht positive
  Ära-Lage ist gemessen und benannt, nicht zu 0 geglättet; der st43-Dec-95-Körper ist
  mit n 2 dünn (Median +0,033, Mittel −0,029).
- Die M2/M3-Aussage ist serienabhängig: nur drei der sechs M2/M3-Serien tragen
  ruhig-ruhig-Paare direkt an der Grenze; M3 st63 hat am 11-30 n 0 und M3 st14 am
  12-03 n 0 — dort ist „kein Sprung" eine Lücke, keine Messung.
- Der Change-Point von st43 ist über den einen leeren 12-01 gemessen (11-30 → 12-02);
  st14/st63 liegen im Lauf benachbarter ruhiger Messtage. Der st43-Jackknife endet an
  der min-Segment-Grenze (nL 3).
- Der DGT-Test (23.05.1996) ist auf der Tages-Achse **nicht durchführbar** (Feb–Mai 96
  = 0 Samples): die „vorher/nachher"-Blöcke liegen 168–169 d bzw. 34–35 d vom Datum —
  ein Niveau-Schritt genau am 23.05. ist damit nicht beobachtbar, die Antwort ist die
  benannte Lücke (0 geehrt) plus die Niveau-Differenz über die leere Spanne.
- Die F3-Zeile selbst liegt in `befund-galileo-floor-pass-episodisch` (fremdes Blatt,
  nicht angefasst); die Schärfung steht in diesem Blatt als Register-Satz.

## Register-Satz

*F3-Ruck nachgemessen (Richtung D5): der −0,80…−0,82-Hz-Mode-1-Niveau-Sprung am
1995-11-30/12-01 hat einen Versatz BESEITIGT statt erzeugt — das +0,7…+0,85-Hz-
Öffnungs-Plateau (nur die erste Messwoche 23.–30.11.) war der transiente Versatz, die
resid-Lage danach ist ≈ 0 (Dez-95-Folge-Körper: st14 n 6 Median +0,032/Mittel +0,017,
st43 n 2 +0,033/−0,029, st63 n 5 +0,026/+0,024 Hz — GLM +0,75 − 0,8 ≈ −0,05
reproduziert; Ära-Post-Median +0,021…+0,149 Hz = Drift-Saison-Struktur, keine Rückkehr
0–1 von 26–27 Tagen). M2/M3-Deckung am Übergang 1995-11-28..12-03 ist teils echte
Gegen-Daten (ruhig-robust auf beiden Seiten der Grenze: M2 st14 Δ −0,018 Hz n
13 562/13 947, M2 st63 Δ −0,054 Hz, M3 st43 Δ −0,004 Hz), teils laut/leer (M2 st43
12-01…03 laut, M3 st14 11-30…12-02 laut und 12-03 leer n 0, M3 st63 11-30 leer n 0 —
benannt, nicht als Gegen-Daten gezählt; kein M2/M3-Tag trägt einen Ruck-Klasse-Abfall).
Der Change-Point ist datengetrieben (Ganz-Ära-Ein-Schnitt-Scan über alle Schnitte,
keine Datums-Vorgabe; p_perm 0,0030/0,0095/0,0045), in drei unabhängigen M1-Serien am
selben Kalender-Übergang reproduziert, 6–7 Tage nach Daten-Beginn 1995-11-23 und
454–456 Tage vor Daten-Ende — nicht an der Era-Kante; Post-Körper ≥ 12-01 ohne
0,5-Hz-Schnitt (Max −0,13…−0,21 Hz), Plateau-Tag-Jackknife hält den Schnitt
(st14/st63; st43 an min-Segment-Grenze nL 3). Der 23.05.1996 (DGT) trägt keinen
M1-Niveau-Sprung: Datum in der bodenleeren Spanne Feb–Mai 1996 (0 Floor-Samples, 0
geehrt), nächste ruhige M1-Tage 168–169 d davor (Dez-95 ≈ +0,03 Hz) und 34–35 d danach
(Jun-96 +0,21…+0,40 Hz), kein M1-Schnitt in ±21 d — Übergangs-Liste bleibt beim
11-30/12-01-Sprung. Pending: Sitz des Sprungs echt-vs-Modell, Ursache des
Öffnungs-Versatzes und der 1996-Saison-Lage, Ursache der laut-Klasse am Übergang.
Probe galileo_ruck_nachmessen_f3 (cargo check 0/0, -D warnings), Report
/tmp/opencode/galileo_ruck_nachmessen_f3_report.txt.*

## Status

`draft` (Entwurf für die Haupt-Session/den Rat; die F3-Zeilen-Schärfung wird der
Haupt-Session als Register-Satz übergeben — `docs/TODO.md` und
`befund-galileo-floor-pass-episodisch` nicht angefasst). Probe
`galileo_ruck_nachmessen_f3` committet (`cargo check` 0/0, RUSTFLAGS `-D warnings`),
Report `/tmp/opencode/galileo_ruck_nachmessen_f3_report.txt`. Richtung D5 (F3-Schärfung)
ist gemessen; der Sitz des Sprungs bleibt pending.
