<!--
  title: Befund — Ruck/Stufe der ruhigen Galileo-Floor-Resid-Basis über die Floor-Ära: genau ein abrupter, anhaltender Mode-1-Niveau-Sprung am Era-Übergang 1995-11-30/12-01 (das einwöchige +0,7…+0,85-Hz-Plateau am Era-Anfang endet mit einem −0,6…−0,7-Hz-Segment- bzw. −0,8-Hz-Nachbar-Tag-Sprung; keine Rückkehr in den Folge-Ruhig-Tagen, t −4,9…−11,7, p_perm ≤ 0,01); die drei kohärenten Versatz-Tage sind isolierte Einzel-/Zwei-Tag-Transienten, keine Stufen-Beginne; die Modi 2/3 tragen keinen Ruck
  class: befund
  date: 2026-09-06
  sha256: e5d95c87c777acf76f7576fd489618d25f3abba3f3980dc1bab0e354eb9e05b3
  status: draft
  see-also: docs/befund/befund-galileo-ruhige-basis-drift.md docs/befund/befund-galileo-floor-pass-episodisch.md
-->
# Befund: Ruck/Stufe der ruhigen Galileo-Floor-Resid-Basis — genau ein abrupter, anhaltender Mode-1-Niveau-Sprung am Era-Übergang 1995-11-30/12-01 (das einwöchige +0,7…+0,85-Hz-Plateau am Era-Anfang endet mit einem −0,6…−0,7-Hz-Segment- bzw. −0,8-Hz-Nachbar-Tag-Sprung; keine Rückkehr in den Folge-Ruhig-Tagen, t −4,9…−11,7, p_perm ≤ 0,01); die drei kohärenten Versatz-Tage sind isolierte Einzel-/Zwei-Tag-Transienten, keine Stufen-Beginne; die Modi 2/3 tragen keinen Ruck

## Frage & Bindung

Dieser Lauf (Richtung D3, aufbauend auf dem Drift-Befund `fad752b`) prüft, ob die
ruhige Basis der Galileo-Floor-Residuen über die Floor-Ära (1995-11-23..1997-02-28)
**einen „Ruck" trägt** — eine plötzliche, anhaltende Niveau-Stufe
(Diskontinuität/Change-Point im Resid-Niveau), getrennt vom linearen Trend (der
bereits gemessen ist: nur Mode-1-Senkung ≈ −0,4 Hz/27 Monate, sonst flach).
Gebunden wie die Vorlagen: Tages-Zelle (Mode, Station, Tag) über die
in-track-Boden-Residuen (Stärke exakt −2560, |resid| ≤ 1000 Hz; Lock vor dem
Rauschen getrennt); robust = Zell-n ≥ 30; ruhig = Zell-RMS < 1 Hz, laut = ≥ 1 Hz
(Out-of-Lock-Boden-Rauschen, separat gehalten); Tag = gerundeter TDB-Zivil-Tag;
Tages-Niveau = Tages-Median/Tages-Mittelwert der Residuen der ruhigen Zelle.
Die Niveau-Reihe je (Mode, Station) speist eine Stufen-Suche: Ein-Schnitt-
Zwei-Segment-Modell (stückweise konstant, mindestens 3 Tage je Segment) auf der
chronologischen Ruhig-Tag-Reihe; Permutations-Null = die Niveau-Blöcke (Läufe
aufeinander folgender ruhiger Tage) werden in der Zeit neu gemischt (1999 Züge),
Statistik = max |Segment-Mittel-Sprung|; zusätzlich die Nachbar-Tag-Sprünge in den
Läufen und die Rückkehr-Messung (ruhige Tage nach dem Schnitt mit Niveau ≥
Segment-Median davor). 0 geehrt: bodenleere Monate bleiben Lücken, kein Wert
gefüllt. Probe `tools/measure/src/bin/galileo_floor_basis_ruck.rs` (neu,
`cargo check` 0/0, RUSTFLAGS `-D warnings`), Report
`/tmp/opencode/galileo_floor_basis_ruck_report.txt`. Daten:
`data/pds-ppi.igpp.ucla.edu/galileo_resid.bin`.

## n zuerst (0 geehrt) — die Reihen und ihre Dichte

Der Zensus reproduziert das Register exakt: **400 robuste (Mode, Station, Tag)-
Zellen, 207 laut / 193 ruhig** (n ≥ 30); 4 647 387 in-track-Floor-Samples der
Ära, 1 815 098 Lock-Samples ausgeschlossen. Die Niveau-Analyse läuft auf den
ruhigen robusten Tagen je Serie:

| Serie | ruhige Tage (n) | Läufe | Monate mit Ruhig-Tagen | Spanne d |
|---|---|---|---|---|
| M1 st14 | 30 | 18 | 7 | 461 |
| M1 st43 | 29 | 21 | 8 | 462 |
| M1 st63 | 34 | 16 | 7 | 461 |
| M2 st14 | 21 | 13 | 7 | 460 |
| M2 st43 | 19 | 11 | 7 | 394 |
| M2 st63 | 17 | 12 | 7 | 455 |
| M3 st14 | 15 | 14 | 7 | 453 |
| M3 st43 | 17 | 11 | 6 | 460 |
| M3 st63 | 11 | 9 | 6 | 393 |

Die ruhigen Läufe sind kurz (1–5 Tage). Die längsten zusammenhängenden ruhigen
Läufe der Ära liegen im Nov/Dez-1995-Block der M1-Serien (M1 st63
1995-11-23..27 und 1995-11-29..12-03, je 5 Tage; M1 st14 1995-11-30..12-02,
3 Tage, und 1995-12-04..07, 4 Tage) — die Stelle, an der „anhaltend auf einem
Niveau verharren" und ein Sprung zwischen benachbarten Messtagen innerhalb der
Daten prüfbar ist. Der Rest der Ära ist durch bodenleere Monate (Feb–Mai 1996,
Jul–Aug 1996, Okt 1996) und laut-Tage in 1–2-Tage-Ruhig-Inseln getrennt; auch in
M1 st43 liegen die +0,7-Hz-Plateau-Tage als einzelne Ruhig-Inseln (24., 25., 30.11.).

## Messung 1 — die Niveau-Reihe im Ruhig-Tag-Raster (Tages-Median)

Der Report führt jeden ruhigen Tag mit Datum, Zell-n, Tages-Median/-Mittel/-RMS
und Lauf. Die drei Mode-1-Serien beginnen die Ära auf einem gemeinsamen hohen
Plateau und fallen dort ab:

| Serie | Era-Öffnungs-Plateau (ruhige Tage) | Niveau Hz | erster niedriger Ruhig-Tag | Niveau Hz |
|---|---|---|---|---|
| M1 st14 | 1995-11-25..30 (4 Tage, med +0,775) | +0,708…+0,847 | 1995-12-01 | +0,028 |
| M1 st43 | 1995-11-24..30 (3 Tage, med +0,719) | +0,703…+0,833 | 1995-12-02 | +0,033 |
| M1 st63 | 1995-11-23..29 (6 Tage, med +0,741) | +0,702…+0,848 | 1995-12-01 | +0,046 |

Danach liegen die M1-Tages-Niveaus über den Rest der Ära (Dez 1995 bis Feb 1997)
zwischen −0,26 und +0,49 Hz (st43 max +0,406; st63 max +0,390; st14 max +0,487
neben dem Versatz-Tag 1997-01-22) — sie kehren auf das +0,7…+0,85-Hz-Plateau
nicht zurück. Die Modi 2/3 beginnen die Ära nicht auf einem erhöhten
Plateau: ihre 1995-11-Niveaus (M2 +0,08…+0,43 Hz, M3 +0,08…+0,26 Hz) liegen in
der Größenordnung ihrer ganzen Ära-Lage (Serien-Mitglieds-Mittel +0,02…+0,29 Hz
im Drift-Befund).

## Messung 2 — Change-Point: der eine abrupte, anhaltende M1-Sprung am 30.11./1.12.1995

Die Ein-Schnitt-Zwei-Segment-Suche findet in **drei der neun Serien** einen
einzelnen dominanten Schnitt, und alle drei liegen an derselben Kalender-Stelle —
dem Übergang November→Dezember 1995, unmittelbar nach dem Era-Öffnungs-Plateau:

| Serie | bester Schnitt | nL/nR | Sprung Segment-Mittel Hz | gepoolte σp Hz | t | p_perm (Lauf-Block) | Rückkehr ≥ Plateau-Median |
|---|---|---|---|---|---|---|---|
| M1 st14 | 1995-11-30 ⇄ 12-01 (Lücke 1 d, im Lauf) | 4/26 | −0,615 | 0,236 | −4,86 | **0,0030** | 1/26 (nur 1997-01-22) |
| M1 st43 | 1995-11-30 ⇄ 12-02 (Lücke 2 d) | 3/26 | −0,637 | 0,152 | −6,88 | **0,0095** | 0/26 |
| M1 st63 | 1995-11-29 ⇄ 11-30 (Lücke 1 d, im Lauf) | 6/28 | −0,700 | 0,133 | −11,73 | **0,0045** | 0/28 |

Der **Nachbar-Tag-Sprung** (die tatsächliche, beobachtete Diskontinuität zwischen
letztem hohem und erstem niedrigem ruhigem Tag) beträgt: M1 st14 −0,819 Hz
(1995-11-30 +0,847 → 12-01 +0,028, beide Zellen ruhig, n 1055/1602), M1 st63
−0,802 Hz über 1995-11-29 (+0,848) → 12-01 (+0,046; dazwischen der
Zero-Spread-Tag 1995-11-30, med 0,000 bei n 334, rms 0,0 — die Drift-Befund-Nennung
„M1 st63 trägt einen Tag mit Tages-RMS exakt 0"), M1 st43 −0,800 Hz über
1995-11-30 (+0,833) → 12-02 (+0,033; 12-01 ist kein ruhiger robuster Tag).
In M1 st14 und st63 liegt der Sprung **innerhalb eines Laufs aufeinander folgender
ruhiger Tage** (1995-11-30..12-02 bzw. 1995-11-29..12-03) — er ist dort nicht über
eine Datenlücke hinweg gemessen, sondern zwischen zwei direkt benachbarten
ruhigen Messtagen.

**Getrennt vom linearen Trend:** die OLS-Senkung der M1-Serien ist ≈ −0,4 Hz über
27 Monate (≈ 9×10⁻⁴ Hz/Tag); der Sprung ist −0,8 Hz an einem Tag. Am selben
Schnitt bleibt der Sprung auch nach Abzug der gemessenen globalen OLS-Geraden
bestehen (detrended: M1 st14 −0,512 Hz, M1 st43 −0,337 Hz, M1 st63 −0,427 Hz) —
die Diskontinuität ist nicht die Trend-Linie, sondern eine zusätzliche Stufe.
Die inneren OLS-Steigungen der Segmente sind klein (links +0,02 Hz/Tag auf 3–6
Tagen = Plateau-Breite, rechts ≈ 0,000) — die Segmente sind Plateau bzw. flacher
Körper, kein Rampen-Paar.

**Über der internen Streuung:** der Sprung (−0,61…−0,70 Hz Segment-Mittel,
−0,80…−0,82 Hz Nachbar-Tag) ist 2,6–5,3× die gepoolte inner-Segment-Streuung σp
(0,13–0,24 Hz), ≈ 8–20× die ruhige Decke (Tages-RMS 0,04–0,09 Hz) und 13–50× den
Median der ruhigen Nachbar-Tag-Niveau-Sprünge (0,016–0,060 Hz in den M1-Serien).
Die Lauf-Block-Permutations-Null (1999 Züge) verwirft die Zufalls-Anordnung der
Niveau-Blöcke für alle drei M1-Schnitte (p_perm 0,0030/0,0095/0,0045).

**Anhaltend (keine Rückkehr):** nach dem Schnitt erreicht kein ruhiger Tag der
M1-Serien wieder den Plateau-Median (+0,719…+0,775), außer dem einen isolierten
Versatz-Tag 1997-01-22 in M1 st14 (+1,076 Hz, Einzel-Transient): M1 st43 0/26, M1
st63 0/28, M1 st14 1/26 (nur dieser Tag). Die späteren moderaten Niveaus
Mitte/Ende 1996 (+0,2…+0,49 Hz) bleiben alle unter dem Plateau (0,70–0,85 Hz).

**Mode-1-spezifisch:** an denselben Kalender-Tagen zeigen die Mode-2/3-Serien
keinen Sprung — z. B. M2 st14 1995-11-30 +0,190 → 1995-12-01 +0,172 (−0,018 Hz),
M3 st43 1995-11-30 +0,189 → 1995-12-01 +0,185 (−0,004 Hz). Der Sprung sitzt auf
dem one-way-Downlink-Pfad (Mode 1), nicht auf den uplink-tragenden Modi 2/3.

## Messung 3 — die Modi 2/3 und die übrigen Serien: keine weitere Stufe

Die besten Ein-Schnitt-Sprünge der übrigen sechs Serien:

| Serie | bester Schnitt | Lücke | nL/nR | Sprung Hz | σp Hz | t | p_perm |
|---|---|---|---|---|---|---|---|
| M2 st14 | 1996-12-27 ⇄ 1997-02-02 | 37 d | 18/3 | −0,320 | 0,126 | −4,08 | **0,0025** |
| M2 st43 | 1995-12-06 ⇄ 1996-01-03 | 28 d | 9/10 | −3,321 | 7,344 | −0,98 | 0,9595 |
| M2 st63 | 1997-02-16 ⇄ 02-19 | 3 d | 14/3 | −0,296 | 0,523 | −0,89 | 0,8955 |
| M3 st14 | 1995-12-04 ⇄ 12-06 | 2 d | 3/12 | −0,113 | 0,168 | −1,04 | 0,7975 |
| M3 st43 | 1996-11-30 ⇄ 12-19 | 19 d | 14/3 | −0,176 | 0,090 | −3,07 | 0,0805 |
| M3 st63 | 1996-06-30 ⇄ 09-07 | 69 d | 5/6 | +0,220 | 0,292 | +1,24 | 0,7855 |

Keine dieser Strukturen ist ein beobachteter abrupter Sprung zwischen benachbarten
ruhigen Messtagen: M2 st43/M2 st63 werden von den Einzel-Versatz-Tagen (siehe
Messung 4) getragen (p_perm 0,96 bzw. 0,90, σp so groß wie der Sprung), M3 st43
ist ein Era-Schwanz-Differenz über 19 d mit nR 3 (p 0,08). M2 st14 trägt die
einzige kleine signifikante Zwei-Segment-Struktur außerhalb M1: der Schnitt
trennt den 1997-02-Schwanz (3 ruhige Tage, Mittel −0,18 Hz, darunter der
Einzel-Tag 1997-02-02 −0,516 Hz) vom 1996er-Körper (+0,14 Hz) über eine
37-Tage-Datenlücke (p_perm 0,0025, Sprung nur −0,32 Hz) — ein Niveau-Unterschied
über eine Lücke am Era-Ende, kein beobachteter plötzlicher Sprung. Die ruhigen
M3-Serien sind für Stufen-Aussagen dünn (n 11–17, davon fast nur Einzel-Tage;
M3 st63 mit der höchsten ruhigen Decke 0,28 Hz trägt den Nachbar-Sprung
1996-12-17→18 +0,677 Hz als Ausreißer-Insel, die im Niveau am 12-21 auf −0,240
zurückfällt).

## Messung 4 — die drei kohärenten Versatz-Tage: isolierte Transienten, keine Stufen-Beginne

Die drei im Drift-Befund genannten ruhigen Tage mit |Tages-Niveau| > 1 Hz bei
Tages-RMS < 1 Hz (alle drei reproduziert, Datum exakt):

| Versatz-Tag | Serie | n | Tages-Median Hz | Tages-Mittel Hz | Tages-RMS Hz | Nachbar-Ruhig-Tage | Klasse |
|---|---|---|---|---|---|---|---|
| 1996-01-03 | M2 st43 | 45 | −31,910 | −31,928 | 0,085 | vorher 12/95 +0,010…+0,178; nachher 06/96 −0,096/−0,008, 09/96 +0,179 | isolierter Einzel-Tag |
| 1996-01-08 | M2 st63 | 91 | +2,219 | +1,606 | 0,996 | vorher 12/95 +0,047…+0,146; nachher 11/96 +0,121…+0,259 | isolierter Einzel-Tag |
| 1997-01-22 | M1 st14 | 1820 | +1,076 | +1,113 | 0,905 | Lauf 21.–23.01: +0,020, +1,076, +0,495; danach 08.02 +0,042 | Zwei-Tag-Transient |

Alle drei sind **keine Stufen-Beginne**: kein Versatz-Tag hat einen Folge-Ruhig-Tag,
der auf dem versetzten Niveau verharrt. M2 st43 1996-01-03 und M2 st63 1996-01-08
stehen als einzige ruhige Tage in ihren Monaten zwischen bodenleeren/laute
Nachbarschaft (vorher 12/95, nachher erst 06 bzw. 11/96 — beide auf dem
Basis-Niveau ≈ 0,0…+0,3 Hz, nicht auf dem Versatz). M1 st14 1997-01-22 ist ein
Zwei-Tag-Transient innerhalb eines 3-Tage-Laufs (+0,020 → +1,076 → +0,495) und fällt
bis zum nächsten ruhigen Tag (08.02, +0,042) auf das Basis-Niveau zurück. Sie sind
**auch nicht der laut-Out-of-Lock-Klasse zuzuordnen**: ihr Tages-RMS (0,085/0,996/
0,905 Hz) liegt unter der 1-Hz-Laut-Schwelle — die ganze Zelle sitzt einen Tag lang
kohärent auf einem versetzten Niveau bei kleiner Streuung (die „Versatz-Klasse bei
ruhigem RMS" des Drift-Befunds, n = 3 von 193 ruhigen Tagen). Der größte
Nachbar-Tag-Niveau-Sprung in den Ruhig-Reihen ist denn auch ein Versatz-Transient
(M1 st14 1997-01-21→22, +1,056 Hz), nicht eine Stufe.

## Größenordnung & Einordnung

Der gemessene Ruck ist **Mode-1-spezifisch, abrupt und anhaltend**: ≈ −0,8 Hz
Nachbar-Tag (−0,61…−0,70 Hz Segment-Mittel) am 30.11./1.12.1995, nach einem
+0,7…+0,85-Hz-Plateau, das nur die erste Woche der Ära (23.–30.11.1995) umfasst,
mit keiner Rückkehr in den 26–28 folgenden Ruhig-Tagen (außer dem einen
Einzel-Transient 1997-01-22 in st14). Größenordnung: 8–20× der ruhigen Decke
(0,04–0,09 Hz), 2,6–5,3× der Segment-Streuung, und ≈ 2× die gesamte über 27 Monate
gemessene Mode-1-Trend-Senkung (≈ −0,4 Hz) — der Sprung ist damit die größte
einzelne Niveau-Bewegung der ruhigen Basis, und er ist ein Sprung, keine Rampe.
Der gleichzeitige, gleich große Sprung in allen drei unabhängigen M1-Stations-
Serien bei ausbleibendem Sprung in den Mode-2/3-Serien derselben Stationen
benennt den Sitz als den one-way-Downlink-Pfad (gemeinsamer Signal-/Modell-/
Reduktions-Referenz-Term), nicht als Stations- oder Rausch-Eigenschaft. Ob der
Sprung ein Reduktions-/Modell-Wechsel (Referenz-Term auf dem S/C-Oszillator-Pfad)
oder ein Signal-Effekt ist, trennt die Tages-Achse nicht (pending); eine
Pioneer-artige unmodellierte Rampen-Beschleunigung ist es nicht — Ruck und
langsame Senkung zusammen tragen keine monotone Rampe (die Drift-Befund-Struktur
Saison-Lage + dieser eine Sprung decken die Niveau-Bewegung ab).

## Verdikt

**Die ruhige Basis trägt genau einen Ruck: den abrupten, anhaltenden Mode-1-
Niveau-Sprung am Era-Übergang 1995-11-30/12-01.** Gemessen an der Tages-Niveau-
Reihe (ruhige robuste Tage) springt das Niveau in allen drei M1-Serien von einem
+0,7…+0,85-Hz-Plateau (nur die erste Woche der Ära, 4–6 ruhige Tage) um
−0,80…−0,82 Hz Nachbar-Tag (−0,61…−0,70 Hz Segment-Mittel, t −4,86…−11,73,
p_perm ≤ 0,01) auf ein Niveau, das in den 26–28 folgenden ruhigen Tagen nicht
zurückkehrt (st43 0/26, st63 0/28, st14 1/26 — nur der Einzel-Transient
1997-01-22); auch nach Abzug der gemessenen linearen Trend-Linie bleibt der
Sprung (−0,34…−0,51 Hz). In M1 st14 und st63 liegt er zwischen direkt benachbarten
ruhigen Messtagen (in einem Lauf), nicht über einer Datenlücke. Die Modi 2/3
tragen keinen solchen Sprung: der größte Mode-2/3-Schnitt ist an den −31,9-Hz-
Versatz-Tag gebunden (M2 st43, |Δ| 3,3 Hz, aber σp 7,3, t −0,98, p_perm 0,96 — ein
Einzelwert, keine Stufe), alle übrigen sind klein (|Δ| ≤ 0,32 Hz) und liegen an
Datenlücken/Era-Rändern (M2 st14 1997-02-Schwanz p_perm 0,0025 über 37 d Lücke
mit nR 3; M3 st43 −0,18 Hz p_perm 0,08; M2 st63, M3 st14, M3 st63 p_perm ≥ 0,78).
Die drei kohärenten Versatz-Tage (|Niveau| 1,08–31,9 Hz bei ruhigem RMS) sind
**isolierte Einzel-/Zwei-Tag-Transienten**, keine Stufen-Beginne und nicht die
laut-Out-of-Lock-Klasse: ihre Folge-Ruhig-Tage liegen auf dem Basis-Niveau, nicht
auf dem versetzten. Damit ist die Antwort: ein Ruck (M1, Era-Anfang), aber nicht
als mid-era-Instrument-/Stations-Stufe und nicht in den Modi 2/3 — die ruhige
Basis der Ära ist sonst der langsame M1-Trend + isolierte Einzel-Tage. 0 geehrt:
die bodenleeren Monate (Feb–Mai 1996, Jul–Aug 1996, Okt 1996) liegen als Lücken
in der Reihe, nie als Werte; die Versatz-Tage zählen als das, was sie sind
(3 von 193 ruhigen Tagen, eigene Transient-Klasse), nicht als Stufe.

**Was `pending` bleibt:** der Sitz des M1-Sprungs (Reduktions-/Modell-Referenz-
Wechsel auf dem one-way-Pfad, S/C-Oszillator-Effekt oder Geometrie — die Tages-
Achse trennt nicht); ob das Era-Öffnungs-Plateau (+0,75 Hz) ein Anfangstransient
der ersten Messwoche oder ein zuvor stehendes Niveau war (die Ära beginnt
1995-11-23, vor-Ära-Niveau ungemessen); die Sub-Tages-Struktur des Sprungs
(innerhalb welcher Pass-Stunde er liegt); die Ursache der drei Versatz-Tage. Der
Verdikt bleibt `draft` (Entwurf für die Haupt-Session/den Rat).

## Grenzen

- Die Niveau-Reihen sind dünn und geklumpt (11–34 ruhige Tage je Serie, fast nur
  Einzel-Tage außerhalb der M1-Läufe Nov–Dez 1995); die Lauf-Block-Permutation
  erhält die Lauf-Längen, aber die Anzahl der Blöcke ist klein (9–21), und eine
  Anordnung mit dem Hoch-Plateau am Anfang ist selten — die p_perm sind damit
  konservativ gehalten, nicht exakt.
- Der Ein-Schnitt-Sprung von M1 st14/st43 hat links nur 4 bzw. 3 ruhige Tage
  (das Plateau ist kurz, weil die Ära dort beginnt und weil Nov-Tage laut/leer
  sind); die Plateau-Breite ist gemessen, nicht geglättet.
- M1 st63: der Zero-Spread-Tag 1995-11-30 (alle 334 Samples identisch, med/rms 0,0)
  liegt im Lauf zwischen den Plateau-Tagen und dem niedrigen Niveau; der Sprung
  besteht auch ohne diesen Tag (29.11 +0,848 → 1.12 +0,046, −0,802 Hz).
- Der M2-st14-Era-Schwanz (1997-02, 3 ruhige Tage, −0,32 Hz unter dem 1996er-
  Körper) ist über eine 37-Tage-Lücke gemessen; „plötzlich" ist dort nicht
  beobachtbar — benannt als Lücken-Differenz, nicht als Ruck.
- Die Modi-2/3-Versatz-Tage 1996-01-03 (n 45) und 1996-01-08 (rms 0,996, knapp
  unter der Laut-Schwelle) stehen an den Rändern ihrer Klassen; die Nähe ist
  benannt, nicht verschoben.
- Die Einordnung gegen den one-way-Downlink-Pfad ist strukturell (Mode-1-spezifisch,
  stations-übergreifend simultan), kein Fit eines Referenz-Terms.

## Register-Satz

*Die ruhige Basis der Galileo-Floor-Residuen über 1995-11-23..1997-02-28 trägt
genau einen Ruck: einen abrupten, anhaltenden Mode-1-Niveau-Sprung am
Era-Übergang 1995-11-30/12-01 — das einwöchige +0,7…+0,85-Hz-Plateau am Era-Anfang
(M1 st14 n 4 med +0,775, st43 n 3 med +0,719, st63 n 6 med +0,741) endet in einem
Nachbar-Tag-Sprung von −0,80…−0,82 Hz (Segment-Mittel −0,61…−0,70 Hz, t −4,86…
−11,73, Lauf-Block-p_perm 0,0030/0,0095/0,0045; detrended −0,34…−0,51 Hz) auf ein
Niveau, das in den 26–28 Folge-Ruhig-Tagen nicht zurückkehrt (st43 0/26, st63 0/28,
st14 1/26 = nur der Einzel-Versatz-Tag 1997-01-22); in M1 st14/st63 liegt der
Sprung in einem Lauf benachbarter ruhiger Messtage, in st43 über die eine Lücke
1.12.; die Modi 2/3 tragen keinen Sprung (der größte Mode-2/3-Schnitt hängt am
−31,9-Hz-Versatz-Tag, M2 st43 p_perm 0,96; die übrigen |Δ| ≤ 0,32 Hz an
Lücken/Era-Rändern: M2 st14 p_perm 0,0025 über 37-d-Lücke nR 3, M3 st43 0,08,
übrige ≥ 0,78); die drei
kohärenten Versatz-Tage 1996-01-03 M2 st43 −31,9 Hz, 1996-01-08 M2 st63 +2,2 Hz,
1997-01-22 M1 st14 +1,08 Hz (alle bei ruhigem RMS 0,085/0,996/0,905) sind
isolierte Einzel-/Zwei-Tag-Transienten mit Rückkehr aufs Basis-Niveau — keine
Stufen-Beginne, nicht die laut-Klasse; der Ruck ist die größte Niveau-Bewegung der
ruhigen Basis (8–20× ruhige Decke, ≈ 2× die ganze M1-Trend-Senkung), sitzt
Mode-1-spezifisch und stations-übergreifend simultan auf dem one-way-Downlink-
Pfad; Sitz (Reduktions-/Modell-/Signal-Term) bleibt pending.*
