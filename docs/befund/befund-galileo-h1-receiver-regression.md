<!--
  title: Befund — Galileo-Floor-Laut-Inzidenz je Station gegen die dokumentierten DSN-Empfänger-Meilensteine (H1-Regression): keine Laut-Wahrscheinlichkeits-Stufe an den Meilensteinen — der einzige Zell-Ebenen-Zuwachs (05.12.1995) sitzt auf dem Era-Beginn-/SWS-Experiment-Fenster, die Station-Tag-Wahrscheinlichkeit springt an keinem der vier Daten, kein stations-versetzter Rollout-Sprung gemessen; H1 auf der getesteten Achse widerlegt
  class: befund
  date: 2026-09-06
  sha256: b50c125da1fa9a8eb1fd81a4445ffe6e0baa372d7aae65eeb2b6cf049dccc22b
  status: draft
  antwortet-auf: docs/befund/befund-galileo-ops-aera-floor.md docs/befund/befund-galileo-receiver-floor-ursache.md
  see-also: docs/befund/befund-galileo-floor-stufen-te.md docs/befund/befund-galileo-floor-subtages-recurrenz.md docs/befund/befund-galileo-1996-rest-kontrast.md
-->

# Befund: die Floor-Laut-Inzidenz je Station springt an den dokumentierten DSN-Empfänger-Meilensteinen nicht — H1 („die Receiver-Konfiguration, die der Pass bekam, entschied die Lautheit") auf der Meilenstein-Achse widerlegt

## Frage & Bindung

H1 aus der externen Recherche (Richtung H1): welches Empfangsgerät bzw. welche
Empfangs-Konfiguration ein Galileo-Pass bekam, entschied die Lautheit der
Floor-Zelle. Der hier getestete, an den Meilensteinen messbare Teil der Aussage:
ändert sich an den **dokumentierten, stationsübergreifenden Empfänger-/
Konfigurations-Meilensteinen** die Wahrscheinlichkeit, dass eine Station an
einem Boden-Tag laut ist — und zwar stations-gebunden (versetzte Sprünge im
BVR-Testfenster Okt–Dez 1995) statt global?

Gebunden wie die Vorlagen (Register-Metrik unverändert): Boden = Signalstärke
exakt −2560 (AGC-Klemmwert); Zelle = (Mode, Station, TDB-Tag)-RMS der
Boden-Residuen um den Zellen-Mittelwert; Lock (|resid| > 1000 Hz) und
nicht-endliche Werte vor der Zelle getrennt; robust = Zelle n ≥ 30 Proben;
laut = Zell-RMS ≥ 1 Hz; dünne Zellen (n 1..29) gehen nicht in die Serie ein
(nie klassiert); bodenleere Monate sind 0 geehrt. Tag = Unix-Tag der Probe
aus tdb (rund, Register-Konvention), Zivil-Datum über `civil_from_days`. Ära
1995-11-23..1997-02-28 (Tage 9457..9920), Stationen 14/43/63, Modi 1–3.
Probe `tools/measure/src/bin/galileo_h1_receiver_regression.rs` (neu, einzige
Repo-Änderung außer diesem Blatt; `cargo check` 0/0, RUSTFLAGS `-D warnings`),
Report `/tmp/opencode/galileo_h1_receiver_regression_report.txt`. Daten:
`data/pds-ppi.igpp.ucla.edu/galileo_resid.bin` (GASR, 14 077 825 Residuen;
1 838 157 Lock-/Nicht-endlich-Samples ausgeschlossen, 7 191 614 Nicht-Boden-
Stärken, 0 Boden-Samples außerhalb der Ära).

**Die Meilensteine sind externe Recherche-Daten (TDA Progress Report 42-125,
42-133, Beyer et al. — die Rechercheblätter `…_results.md` / `…_results_2.md`),
keine gemessenen Assets dieser Messung:**

| Datum | dokumentierter Zustand | Quelle |
|---|---|---|
| 1995-09-18 | suppressed-carrier (MI 90°), zusammen mit BVR der Standardmodus des 70-m-Netzes | 42-125, S. 10 |
| 1995-12-05 | Wechsel auf residual-carrier (MI 58°) „to minimize solar degradation in the BVR receivers", Sonder-Konfigurationstabelle an allen 70-m-Sites | 42-125, S. 10–11 |
| 1996-05-23 | DGT (DSN Galileo Telemetry) verarbeitet das Phase-2-Paket-Telemetrieformat operationell | 42-125 (g1-Ära; der Erstfall datiert 24./25.05.1996) |
| 1996-11-01 | Full-Arraying-Routine nur Canberra (C3-Ära) | 42-133, S. 5–7 |

Kontext (nicht testbar hier): BVR an allen DSCC installiert Mitte Mai 1995
(vor der Ära), BVR-Test DSS-63 1995-09-11 („fully implemented throughout the
70-m network"), G1 1996-06-27 FSR-Kanal/BVR→FCD, BVR-Abschluss Aug 1997 (nach
Ära-Ende). Die externe Recherche nennt für H1 das BVR-Testfenster **Okt–Dez
1995** mit stations-versetztem Rollout.

## n zuerst (0 geehrt) — die Register-Reproduktion

| Serie | robuste Tage | laut | ruhig | benachbarte Flips | laut-Fraktion |
|---|---|---|---|---|---|
| M1 st14 | 62 | 32 | 30 | 18 | 0,516 |
| M1 st43 | 64 | 35 | 29 | 14 | 0,547 |
| M1 st63 | 68 | 34 | 34 | 15 | 0,500 |
| M2 st14 | 38 | 17 | 21 | 9 | 0,447 |
| M2 st43 | 35 | 16 | 19 | 12 | 0,457 |
| M2 st63 | 39 | 22 | 17 | 11 | 0,564 |
| M3 st14 | 40 | 25 | 15 | 10 | 0,625 |
| M3 st43 | 33 | 16 | 17 | 9 | 0,485 |
| M3 st63 | 21 | 10 | 11 | 7 | 0,476 |
| **Summe** | **400** | **207** | **193** | **105** | 0,517 |

Die Zahlen reproduzieren das Register exakt (400 robust, 207 laut, 193 ruhig,
105 Flips). Je (Mode, Station) trägt eine Serie höchstens eine robuste Zelle
pro Tag — die Zell-laut-Fraktion der Serie ist zugleich ihre Laut-Tag-Fraktion.
Ära-weit je Station (Modi gepoolt, Zell-Einheit): st14 74/140 (0,529), st43
67/132 (0,508), st63 66/128 (0,516); auf Station-Tag-Ebene (Tag mit ≥ 1
robuster Zelle, laut = ≥ 1 laute Zelle): st14 54/76 (0,711), st43 52/80
(0,650), st63 48/78 (0,615).

## Messung 1 — Laut-Fraktion je Station über die Zeit (Monats-Bins)

Monat je Station: `laut/robust` Zellen in eckiger Klammer nach Mode
(M1/M2/M3), dann die Station-Tag-Fraktion (`laut-Tage/Tage`, laut = ≥ 1 laute
Zelle des Tages). Bodenleere Monate = 0 Zellen (0 geehrt, nicht gefüllt).

| Monat | st14 Zellen (M1/M2/M3) | st14 Tage | st43 Zellen (M1/M2/M3) | st43 Tage | st63 Zellen (M1/M2/M3) | st63 Tage |
|---|---|---|---|---|---|---|
| 1995-11 | 7/20 (2/6,1/8,4/6) | 5/8 | 4/20 (1/4,2/8,1/8) | 4/8 | 8/20 (0/7,3/7,5/6) | 6/8 |
| 1995-12 | 13/25 (4/10,5/9,4/6) | 11/12 | 11/19 (2/4,5/8,4/7) | 8/9 | 7/18 (4/9,1/5,2/4) | 6/9 |
| 1996-01 | 0/1 (0/0,0/0,0/1) | 0/1 | 0/1 (0/0,0/1,0/0) | 0/1 | 0/1 (0/0,0/1,0/0) | 0/1 |
| 1996-02..05 | 0 Zellen (0 geehrt) | — | 0 Zellen (0 geehrt) | — | 0 Zellen (0 geehrt) | — |
| 1996-06 | 6/9 (2/4,1/2,3/3) | 4/4 | 4/9 (0/3,3/5,1/1) | 3/5 | 5/9 (2/4,2/2,1/3) | 4/5 |
| 1996-07/08 | 0 Zellen (0 geehrt) | — | 0 Zellen (0 geehrt) | — | 0 Zellen (0 geehrt) | — |
| 1996-09 | 9/10 (3/3,3/4,3/3) | 3/4 | 6/11 (2/4,1/3,3/4) | 4/5 | 9/10 (4/4,4/4,1/2) | 4/4 |
| 1996-10 | 0 Zellen (0 geehrt) | — | 0 Zellen (0 geehrt) | — | 0 Zellen (0 geehrt) | — |
| 1996-11 | 8/22 (5/10,2/6,1/6) | 7/10 | 10/20 (6/11,1/3,3/6) | 7/11 | 7/15 (5/10,2/4,0/1) | 5/10 |
| 1996-12 | 12/18 (5/9,3/4,4/5) | 7/10 | 12/22 (7/13,3/6,2/3) | 8/14 | 7/20 (5/13,2/3,0/4) | 5/13 |
| 1997-01 | 10/15 (6/10,0/0,4/5) | 10/13 | 13/16 (13/16,0/0,0/0) | 13/16 | 10/13 (5/7,5/6,0/0) | 9/12 |
| 1997-02 | 9/20 (5/10,2/5,2/5) | 7/14 | 7/14 (4/9,1/1,2/4) | 5/11 | 13/22 (9/14,3/7,1/1) | 9/16 |

Die Station-Tag-Laut-Wahrscheinlichkeit schwankt über die Ära zwischen 0,385
und 1,0 ohne anhaltenden Trend; die Zell-Fraktionen schwanken zwischen 0,200
und 0,900. Die bodenleeren Blöcke (Feb–Mai 1996, Okt 1996) und der einzelne
Januar-1996-Tag sind 0 geehrt — zwischen 1995-12 und 1996-06 liegt keine
messbare Serie.

## Messung 2 — Sprung der Laut-Inzidenz an den Meilensteinen

Fisher-exakter zweiseitiger Test auf (laut, ruhig) × (vor, nach), je Seite
n ≥ 3; kleinere Seiten bleiben datendünn (benannt, kein p). Vorher = Zellen
mit Tag < Meilenstein-Tag, nachher = Tag ≥ Meilenstein-Tag.

**18.09.1995 (BVR + suppressed-carrier Standardmodus):** die Vorher-Seite ist
unbesetzt — der 70-m-Floor der Trio-Stationen beginnt erst 1995-11-23, der
Meilenstein liegt 66 Tage vor dem ersten Boden-Tag. Vorher n = 0 an allen
Stationen/Modi (0 geehrt); die Grenze ist auf der Floor-Metrik nicht testbar.
Nachher: die ganze Ära (Stufen 0,516/0,547/0,500 je M1). Eine Laut-Stufe „beim
BVR-Standardmodus" ist damit weder belegt noch widerlegt — `datendünn` an
dieser einen Grenze, weil das Asset erst nach ihr beginnt.

**05.12.1995 (residual-carrier + Sonder-Konfigurationstabelle):**

Zell-Ebene je (Mode, Station) — vorher = die ersten 12 Floor-Tage
(1995-11-23..12-04), nachher = der Rest der Ära:

| Serie | vor n/laut (Fraktion) | nach n/laut (Fraktion) | p |
|---|---|---|---|
| M1 st14 | 10/3 (0,300) | 52/29 (0,558) | 0,176 |
| M1 st43 | 5/1 (0,200) | 59/34 (0,576) | 0,167 |
| M1 st63 | 11/1 (0,091) | 57/33 (0,579) | **0,006** |
| M2 st14 | 12/2 (0,167) | 26/15 (0,577) | **0,034** |
| M2 st43 | 12/5 (0,417) | 23/11 (0,478) | 1,000 |
| M2 st63 | 11/4 (0,364) | 28/18 (0,643) | 0,158 |
| M3 st14 | 9/6 (0,667) | 31/19 (0,613) | 1,000 |
| M3 st43 | 12/3 (0,250) | 21/13 (0,619) | 0,071 |
| M3 st63 | 10/7 (0,700) | 11/3 (0,273) | 0,086 |

Modi gepoolt (Zell-Einheit): st14 0,355→0,578 (+0,223, p 0,041), st43
0,310→0,563 (+0,253, p 0,021), st63 0,375→0,562 (+0,188, p 0,101); alle
Stationen 0,348→0,568 (+0,220, **p 0,0002**). Der Anstieg liegt in den Modi 1
und 2 (die in den ersten 12 Tagen auf einem ruhigen Sockel lasen); Mode 3
st63 fällt gegengleich (0,700→0,273), Mode 3 st14 bleibt flach. Auf der
**Station-Tag-Ebene** (die Wahrscheinlichkeit, dass die Station an einem Tag
laut ist) springt die Grenze nicht: st14 0,667→0,719 (+0,052, p 0,736), st43
0,667→0,647 (−0,020, p 1,000), st63 0,750→0,591 (−0,159, p 0,352).

**23.05.1996 (DGT-Phase-2):** die Vorher-Seite ist im Wesentlichen die
Nov–Dez-1995-Saison (Feb–Mai 1996 bodenleer), die Nachher-Seite die
1996er/97er-Rest-Ära ab 1996-06-26. Zell-Ebene: nur M1 st63 (p 0,043) und
M2 st63 (p 0,039) unter 0,05; gepoolt st14 +0,140 (p 0,150), st43 +0,190
(p 0,058), st63 +0,188 (p 0,057); alle Stationen +0,171 (p 0,002). Station-
Tag-Ebene: st14 −0,071 (p 0,778), st43 −0,022 (p 1,000), st63 −0,067
(p 0,784) — alle drei leicht fallend, keiner signifikant.

**01.11.1996 (Full-Array-Routine):** keine Serie unter p 0,05; gepoolt st14
−0,018 (p 0,866), st43 +0,167 (p 0,080), st63 +0,029 (p 0,859). Station-Tag-
Ebene: st14 −0,134 (p 0,299), st43 −0,044 (p 0,808), st63 −0,192 (p 0,142) —
alle drei leicht fallend, keiner signifikant (gepoolt über Stationen p 0,054).

Über die 27 Zell-Tests der drei testbaren Daten (9 Serien je Datum) liegen
4 unter p 0,05 (2 am 05.12.1995, 2 am 23.05.1996) — in der Größenordnung des
Zufalls bei dieser Multiplizität; keiner der 12 Station-Tag-Tests erreicht
p 0,05. Der gepoolte Zell-Anstieg an den zwei Grenzen 1995-12-05 und
1996-05-23 ist derselbe Kontrast: die ersten 12 Floor-Tage (05.12-Grenze) bzw.
die ganze Nov–Dez-1995-Saison (DGT-Grenze) gegen die Rest-Ära — er sitzt auf
dem Era-Beginn-/Konjunktions-/SWS-Experiment-Fenster, nicht auf den
Meilenstein-Tagen selbst, und verschwindet auf der Station-Tag-Ebene.

**Der Zell-Tag-Unterschied ist benannt:** in den frühen Fenstern tragen laute
Tage nur wenige laute Zellen (Zell-Fraktion deutlich unter der Tag-Fraktion);
nach der Rest-Ära ist ein lauter Tag meist in mehreren Modi laut. Die
Laut-Wahrscheinlichkeit je Tag ändert sich dadurch nicht, die Zell-Zählung
der Register-Metrik schon — die Register-Zählung misst Pass-Zellen, nicht
Station-Tage.

## Messung 3 — stations-spezifisch oder global? Der versetzte Rollout-Test

**Station-Konkordanz (Station-Tag-Delta, alle Modi):**

| Meilenstein | Stationen | st14 | st43 | st63 |
|---|---|---|---|---|
| 1995-09-18 | Vorher-Seite unbesetzt | — | — | — |
| 1995-12-05 | 1 auf / 2 ab | +0,052 | −0,020 | −0,159 |
| 1996-05-23 | 0 auf / 3 ab | −0,071 | −0,022 | −0,067 |
| 1996-11-01 | 0 auf / 3 ab | −0,134 | −0,044 | −0,192 |

An den zwei Daten mit voller Vorher-Seite (DGT, Full-Array) bewegen sich alle
drei Stationen in dieselbe Richtung — **leicht abwärts**, nie eine Stufe;
kein Test signifikant. Ein globaler, stations-gemeinsamer Sprung existiert
nicht; eine station-gebundene Stufe (H1) ebenso wenig.

**Versetzter Rollout (BVR-Testfenster Okt–Dez 1995):** die Tages-Zustände der
dichten Saison 1995-11-23..12-07 je (Mode, Station) — L = laut, q = ruhig,
. = kein robuster Boden-Tag (0 geehrt):

| Serie | 11-23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 12-01 | 02 | 03 | 04 | 05 | 06 | 07 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| M1 st14 | . | L | q | L | q | q | . | q | q | q | L | q | q | q | q |
| M1 st43 | . | q | q | . | . | L | . | q | . | q | . | . | L | q | . |
| M1 st63 | q | q | q | q | q | . | q | q | q | q | q | L | q | q | . |
| M2 st14 | q | L | q | q | q | q | q | q | q | L | q | q | q | L | L |
| M2 st43 | L | q | q | q | L | q | q | q | L | L | L | q | q | q | . |
| M2 st63 | . | L | q | L | q | L | q | q | q | q | q | L | q | q | . |
| M3 st14 | . | . | L | L | q | L | q | L | L | L | . | q | L | q | . |
| M3 st43 | q | L | q | q | q | q | q | q | q | L | q | L | L | q | . |
| M3 st63 | L | L | q | L | L | . | L | . | L | q | L | q | L | q | . |

Die Zustände wechseln tage-scharf innerhalb weniger Tage an jeder Station,
ohne anhaltende Stufe und ohne versetzte Stationen-Kaskade um den 05.12.1995:
am Meilenstein-Tag selbst sind M1 st43 laut und M1 st14, M2 st14, M2 st43,
M2 st63 ruhig; die ersten zwei Tage danach sind M2 st14 laut, M2 st43/st63
ruhig.

Die datengetriebenen besten Ein-Schnitte der Serien (beide Seiten n ≥ 3,
ganze Ära) fallen für fünf Dec-95-Serien zwischen 1995-12-01 und 1995-12-07 —
über die Stationen um bis zu 6 Tage gestreut (M3 st43/st63 nach 12-01,
M1 st63 nach 12-03, M2 st14 nach 12-05, M1 st14 nach 12-07); die übrigen
Serien (M1 st43, M2 st43, M2 st63, M3 st14) legen ihren besten Schnitt auf
einzelne Tage 1996-12-19 bis 1997-02-20 (48–111 Tage vom nächsten Meilenstein)
mit nur 3–5 Zellen rechts (dünn). Kein versetzter Sprung in der Ordnung eines
BVR-Ausroll-Zeitfensters ist gemessen; die tage-scharfen Flips des Registers
(105 auf der robusten Serie) sind die dominante Struktur.

## Verdikt H1

Die getestete Aussage lautete: *an den dokumentierten Empfänger-Konfigurations-
Meilensteinen ändert sich die Wahrscheinlichkeit, dass eine Station an einem
Boden-Tag laut ist, als Stufe, stations-gebunden.* Gemessen (n je Zelle/Bin
oben, 0 geehrt):

- Die Station-Tag-Laut-Wahrscheinlichkeit springt an **keinem** der vier
  Meilensteine: von den 12 Station-Tag-Tests (9 je Station, 3 gepoolt über
  Stationen) erreicht keiner p < 0,05; die Je-Station-Deltas liegen zwischen
  +0,052 und −0,192. An den zwei Grenzen mit voller Vorher-Seite (DGT,
  Full-Array) fallen alle drei Stationen leicht und gemeinsam, ohne Stufe.
- Die Zell-Ebenen-Zuwächse über der Null (gepoolt p 0,0002 am 05.12.1995,
  p 0,002 am 23.05.1996, beide getragen von Modi 1/2) liegen auf demselben
  Kontrast: die ersten 12 Floor-Tage (05.12-Grenze) bzw. die ganze
  Nov–Dez-1995-Saison (DGT-Grenze, Feb–Mai 1996 bodenleer) gegen die Rest-Ära.
  Dieses Vorher-Fenster ist das extern dokumentierte SWS-Experiment-Fenster
  24.11.–04.12.1995 im Konjunktions-Eintritt — die Zuwächse sind an die
  Era-Öffnung gebunden, in Mode 3 st63 gegengleich und verschwinden auf der
  Station-Tag-Ebene; an den Wechsel-Tagen selbst sitzen sie nicht.
- Ein stations-versetzter Sprung (BVR-Rollout Okt–Dez 1995) ist nicht
  gemessen; die Serien wechseln tage-scharf.
- Die Grenze 18.09.1995 ist auf der Floor-Metrik unbesetzt (Vorher-Seite
  beginnt 66 Tage nach ihr) — dort `datendünn`, nicht widerlegt.

**H1 wird widerlegt** auf der gemessenen Meilenstein-Achse: die dokumentierten
Receiver-Konfigurations-Meilensteine tragen die Laut-Inzidenz nicht als Stufe.
Die Messung grenzt zugleich ein, was sie nicht schließt: die per-Pass-
Geräte-Zuweisung (welcher parallele Receiver den Pass bekam) ist im
Resid-Record nicht kodiert (Receiver-Identitäten absent, siehe
Receiver-Identitäts-Befund) — ein tage-scharfer Geräte-Wechsel innerhalb
konfigurations-konstanter Ären bleibt auf diesem Asset unbeobachtbar
(`absent`). Was gemessen ist: die dokumentierten Netz-Konfigurations-Wechsel
sind keine Laut-Schalter.

## Register-Satz

- Neue Probe `tools/measure/src/bin/galileo_h1_receiver_regression.rs`
  (Rust, `cargo check` 0/0): Floor-Register reproduziert (400/207/193/105),
  Monats-Bins je Station, Meilenstein-Regression (Fisher exakt), Station-
  Konkordanz, Saison-Zustandsreihen.
- Neuer Befund-Entwurf `docs/befund/befund-galileo-h1-receiver-regression.md`
  (status: draft): H1 auf der Meilenstein-Achse widerlegt; 18.09.1995-Grenze
  unbesetzt (`datendünn`); per-Pass-Geräte-Zuweisung auf diesem Asset absent.
- Nichts an `phi/`, `docs/TODO.md` oder fremden Dateien angefasst.
