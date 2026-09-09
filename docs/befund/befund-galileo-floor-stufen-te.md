<!--
  title: Befund — Galileo-Floor je (Station, Mode): die 105 tages-scharfen Flips (n ≥ 30) sind isolierte 1–3-Tage-Gipfel auf einem wiederkehrenden ruhigen Sockel, keine anhaltenden Stufen; gerichtete Station→Station-TE ist nur in einem Fenster messbar (Mode 2, Nov–Dez 1995) und liegt unter der Null; Treiber-TE (ε, konstruierter 5.12.1995-Schritt) ohne Kopplung über der Null
  class: befund
  date: 2026-09-05
  version: 1
  sha256: c0417084f584255c0085bf98d985df2ff73c71575c2e1e666299e2d98b1e3510
  status: draft
  antwortet-auf: docs/befund/befund-galileo-ops-aera-floor.md docs/befund/befund-galileo-1996-rest-kontrast.md
  see-also: docs/befund/befund-galileo-alpha-zeit-sonnenzyklus.md docs/handover/archiv/handover-2026-09-05-galileo-tiefe-rotor-spin-receiver.md docs/handover/archiv/handover-2026-09-05-galileo-epsilon-revision.md
-->

# Befund: Galileo-Floor je (Station, Mode) — Stufen- und Treiber-TE-Lesart der station-gebundenen Tages-Lautheit

## Frage & Bindung

Der Ops-Ära-Befund (draft) misst 114 Nachbar-Tages-Flips (laut↔ruhig) auf der
Floor-Tages-Reihe der drei 70-m-Stationen (Boden = Stärke exakt −2560) und
lässt den Sitz der station-gebundenen Lautheit als per-Pass-Zustand je
(Station, Tag) `pending`. Dieser Lauf (Richtung C) prüft die Struktur dieser
Flips mit drei Messungen: (1) sind die tages-scharfen Zustandswechsel
**anhaltende Stufen** (Treppe: die Serie verharrt mehrere Tage auf einem
Niveau) oder **isolierte Einzel-Tag-/2–3-Tag-Gipfel**? (2) treibt der Zustand
einer Station gerichtet den einer anderen (TE station→station)? (3) wird der
Floor (die Gipfel) gerichtet von einem Kandidaten-Treiber angetrieben (ε,
konstruierter Betriebs-Schritt)?

Gebunden wie die Vorlagen: Tages-RMS je (Mode, Station, Tag) = RMS der
Boden-Residuen der Tages-Zelle um den Zellen-Mittelwert; Lock
(|resid| > 1000 Hz) vor dem Rauschen getrennt; laut = Zell-RMS ≥ 1 Hz. Die
Tages-Serie wird auf Zellen mit n ≥ 30 Proben geführt (die robuste Konvention
des Ops-Ära-Blatts, dort als n ≥ 30-Teilset) — dünne Zellen (1..29) werden
ausgewiesen, nicht klassiert; Lücken (n = 0-Tage) und dünne Tage werden nie
mit Werten gefüllt. Die TE läuft auf **zusammenhängenden Läufen**
(kalendarisch aufeinanderfolgende Tage), nie über Lücken; Serienwert für TE
und Segmentierung ist log10(Tages-RMS) (genannt; die TE-Maschine
`transfer_entropy_lag` in der Konvention der Vorlage). TE-Schwellen:
Surrogat-Null `surrogate_stats_phase` (Phasen-Surrogat, zerstört jede
glatte/Stufen-Struktur des Treibers → anti-konservativ für glatte Treiber)
und `surrogate_stats_block` (Block-Bootstrap, block 5, erhält lokale
Struktur); Verdikt nur über beide Nullen, sonst `pending`. Zusätzlich
`transfer_entropy_conditional`/`conditional_te_stats` mit Ära (Monatsindex)
als Bedingung, wie `galileo_te_floor_direction`. Richtung X→Y = der Zustand
von X (Quelle) am Tag t sagt Y am Tag t+lag über Y_t hinaus voraus. Die
Segmentierung ist eine binäre Zerlegung auf log10-Werte (Mittelwert-Lücke je
Schnitt, beide Seiten ≥ 2 Tage), angenommen nur wenn der beste Schnitt die
Permutations-Null (199 Wiederholungen, α = 0,05) übersteigt; die
Klassifikation Stufe gegen Gipfel nutzt K = 4 Tage (Segment ≥ 4 Tage auf
beiden Seiten eines Wechsels = anhaltende Stufe; kurze Segmente < 4 Tage =
Gipfel). Probe
`tools/measure/src/bin/galileo_floor_stair_te.rs` (neu, einzige
Repo-Änderung außer diesem Blatt; `cargo check` 0/0, RUSTFLAGS `-D
warnings`), Report `/tmp/opencode/galileo_floor_stair_te_report.txt`. Daten:
`data/galileo_resid.bin`, Geometrie je Zelle aus `ephemeris_galileo_daily` +
`ephemeris_earth`.

## n zuerst (0 geehrt) — die Läufe je (Station, Mode)

Floor-Tages-Zellen n ≥ 30 über die Boden-Ära; in Klammern die ausgeschlossenen
dünnen Tage (1..29 Proben; je Serie auch n = 0-Lücken, unbesetzt):

| Serie | n-Tage (n≥30) | dünn | Läufe (zusammenhängende Tage) |
|---|---|---|---|
| M1 st14 | 62 | 20 | 1995-11-24..28 (5), 11-30..12-07 (8), 12-09, 12-18, 12-24, 1996-06-26..29 (4), 09-06..08 (3), 11-03..06 (4), 11-08..10 (3), 11-28..12-01 (4), 12-18..22 (5), 12-25..27 (3), 13 Einzel-Tage 1997 |
| M1 st43 | 64 | 20 | max 8 d (1996-11-03..10); sonst Läufe 1–6 d |
| M1 st63 | 67 | 30 | 1995-11-23..27 (5), 12-01..06 (6), 1996-06-26..29 (4), 09-06..09 (4), 11-03..06 (4), 11-08..10 (3), 11-28..12-01 (4), **12-17..26 (10)**, 1997-02-12..15 (4), 02-19..21 (3), 02-25..28 (4); 11 Einzel-Tage |
| M2 st14 | 38 | 1 | **1995-11-23..12-07 (15)**, 1996-06-27..28 (2), 09-05..08 (4), 11-03..05 (3), 12-19..20 (2); 9 Einzel-Tage |
| M2 st43 | 35 | 0 | **1995-11-23..12-06 (14)**, 1996-06-26..30 (5), 09-06..08 (3), 12-17..19 (3), 12-21..23 (3) |
| M2 st63 | 39 | 11 | **1995-11-24..12-05 (12)**, 1996-06-27..28 (2), 09-06..09 (4), 11-04..06 (3), 12-19..20 (2), 1997-01-30..31 (2), 02-19..22 (4) |
| M3 st14 | 39 | 5 | 1995-11-25..12-02 (8), 12-04..06 (3), 1996-06-26..28 (3), 09-06..08 (3), 11-04..06 (3), 12-18..21 (4) |
| M3 st43 | 33 | 2 | **1995-11-23..12-06 (14)**, 1996-06-27 (1), 09-05..08 (4), 11-03..05 (3) |
| M3 st63 | 21 | 1 | max 5 d (1995-11-23..27); 10 Läufe 1–4 d |

Die Boden-Ära ist damit eine **Serie kurzer Inseln**: der längste
zusammenhängende Floor-Lauf über die ganze Ära misst 15 Tage (M2 st14,
1995-11-23..12-07); acht Läufe erreichen ≥ 8 Tage (segmentierbar), fünf Läufe
≥ 9 Tage (TE-fähig, da `transfer_entropy_lag` für lag 1 mindestens 9 Werte
braucht). Die Lücken (Feb–Mai 1996, Okt 1996) und die dünnen Tage sind
ausgewiesen, nicht gefüllt — ein Lücken-Tag ist kein Wert. Für die
Laut-Zählung reproduziert die n ≥ 30-Serie die Ops-Ära-Zahlen exakt (Flips
M1 18/14/15, M2 9/12/11 = 79, identisch zum n ≥ 30-Teilset des Ops-Ära-Blatts;
dazu M3 10/9/7 → **105 Flips gesamt** auf der robusten Serie).

## Messung 1 — Stufen gegen isolierte Gipfel

**Laut-Runs (Zustand ≥ 1 Hz, über die n ≥ 30-Tages-Serie):** über alle neun
Serien 137 laute Episoden; **133 (97 %) dauern 1–3 Tage, nur 4 dauern ≥ 4
Tage** (je eine in M1 st14/st43/st63 und M2 st63); die ruhigen Runs sind
ebenfalls kurz (max 3–7 Tage), weil schon die Floor-Präsenz der Stationen in
1–6-Tage-Blöcken kommt. Die Zählung ist durch die Läufe begrenzt (ein lauter
Block kann nicht länger als der Lauf selbst sein) — die direkte, von dieser
Begrenzung freie Messung ist die Segmentierung der acht Läufe ≥ 8 Tage:

**Segmentierung (acht Fenster, 8–15 Tage):** *kein einziges Fenster trägt
eine anhaltende Stufe.* Jedes der acht Fenster fällt auf **ein** Niveau
zurück (0 Stufen, 0 kurze Segmente); die besten Schnitt-Kandidaten liegen
unter der Permutations-Null (p 0,10–0,77). Die zwei st14-Fenster von
Nov–Dez 1995 platzieren ihren besten Schnitt exakt auf dem
5.12.1995-Modulationswechsel-Datum (p 0,405 in M1, p 0,100 in M2) — nicht
signifikant; rechts des Schnitts liegen nur 2–3 laute Einzel-Tage
(12-06/12-07 bzw. 12-03). Die Fenster-Werte zeigen die Struktur direkt
(Tages-RMS, Hz): M2 st14 1995-11-23..12-07 = 0,03 | **31,8** | 0,02 | 0,41 |
0,03 | 0,08 | 0,03 | 0,03 | 0,04 | **36,7** | 0,10 | 0,05 | 0,07 | **13,1** |
**31,9** — vier laute Einzel-Tage auf einem ruhigen Sockel ~0,03–0,08 Hz, kein
Plateau. M1 st63 1996-12-17..26 = 0,06 | 0,11 | **28,3** | **119,2** | 0,02 |
**25,1** | **119,5** | 0,07 | 0,04 | 0,02 — zwei 2-Tage-Ausbrüche auf ruhigem
Sockel. M1 st43 1996-11-03..10 = 0,05 | **23,1** | 0,02 | **25,2** | 0,02 |
**24,5** | 3,07 | 4,09 — laut an jedem zweiten Tag (04/06/08), eine gemessene
2-Tage-Periode, keine Stufe. Ein Kontrolllauf der Segmentierung auf einer
synthetischen persistenten Stufe (7 ruhige + 8 laute Tage) detektiert diese
(p = 0,0005) — die Methode ist für echte Treppen nicht blind; das gemessene
Fehlen ist ein Negativ mit Aussagekraft. **Verdikt Messung 1: die Flips sind
in den längsten zusammenhängenden Fenstern isolierte 1–3-Tage-Gipfel auf
einem wiederkehrenden ruhigen Sockel; anhaltende Stufen (beide Seiten ≥ 4
Tage) sind in keinem Fenster gemessen.**

**Wiederkehrende Niveau-Klassen:** die Tages-RMS-Verteilung je Serie ist
bimodal (ruhiger Sockel + laute Population), aber die laute Population ist
**kontinuierlich gestreut, nicht in diskrete Sprossen** geteilt: Serien-
Quantile der lauten Tage p25/p50/p75 (Hz) — M1 st14 7,9/25,8/61,9 (max 291);
M1 st43 4,1/26,3/120,6 (max 440); M1 st63 23,1/32,1/97,8 (max 530); M2 st14
11,3/18,1/31,8; M2 st43 2,3/3,8/22,2; M2 st63 3,0/22,1/32,5; M3 st14
8,0/11,9/31,3 (max 253). Die Spanne der mittleren 50 % der lauten Tage reicht
je Serie über Faktor 3–30 — keine besetzten, wiederkehrenden Laut-Sprossen.
Der einzige wiederkehrende Sockel ist der ruhige: Median der ruhigen Tage je
Serie 0,044–0,085 Hz (acht Serien) bzw. 0,14/0,28 Hz (M3 st14/st63). M3 st14
weicht ab: dort ist der Boden meist laut (25 von 39 Tagen, Tages-Median
7,4 Hz) — eine Station-Mode-Asymmetrie, gemessen, nicht erklärt.

## Messung 2 — Transfer-Entropie Station → Station

Gemeinsame Läufe (beide Stationen tragen am selben Tag Boden, gleicher Mode,
gleiche Tag-Achse): nur **ein** Fenster der gesamten Ära erreicht die
TE-Mindestlänge ≥ 9 zusammenhängende gemeinsame Tage — **Mode 2,
1995-11-23/24..12-05/06** (st14×st43 n 14; st14×st63 n 12; st43×st63 n 12).
Alle übrigen (Mode, Paar)-Kombinationen tragen keine gemeinsamen Läufe ≥ 9
Tage (Mode 1-Paare maximal 6 zusammenhängende gemeinsame Tage; die
Floor-Tage der Stationen überlappen nur in kurzen Blöcken — die Stationen
tragen den Boden wechselweise, gemessen, nicht ergänzt). TE in dem einen
Fenster, Richtungen beidseitig, lags 1–3 (Werte in nats, Null = Mittel+2σ der
Surrogate):

| Richtung (Quelle→Ziel) | n | beste lag-1..3 TE | Phasen-Null | Block-Null | Verdikt |
|---|---|---|---|---|---|
| st14→st43 | 14 | 0,197 / 0,208 / 0,317 | 0,408/0,449/0,511 | 0,321/0,327/0,405 | unter beiden Nullen (cte 0,360 > cThr 0,311 nur lag 3, Bedingungs-Null) |
| st43→st14 | 14 | 0,168 / 0,208 / 0,165 | 0,256/0,333/0,356 | 0,243/0,243/0,404 | unter beiden Nullen |
| st14→st63 | 12 | 0,371 / 0,306 / 0,222 | 0,491/0,398/0,480 | 0,354/0,427/0,302 | lag 1 über der Block-Null, unter der Phasen-Null — kein Verdikt über beide |
| st63→st14 | 12 | 0,156 / 0,232 / 0,154 | 0,211/0,364/0,378 | 0,245/0,357/0,277 | unter beiden Nullen |
| st43→st63 | 12 | 0,310 / 0,333 / 0,318 | 0,444/0,432/0,497 | 0,415/0,482/0,454 | unter beiden Nullen |
| st63→st43 | 12 | 0,384 / 0,397 / 0,395 | 0,442/0,410/0,586 | 0,424/0,505/0,489 | unter beiden Nullen |

**Verdikt Messung 2: keine gerichtete Station→Station-Kopplung über der
Null.** Der eine markierte Wert (st14→st63, lag 1, über der Block-Null allein)
steht unter der Phasen-Null und ist nicht wiederholbar (n = 12, ein Fenster);
der Konditional-TE-Treffer (st14→st43, lag 3, gegen die Bedingungs-Null)
ebenso ein einzelner Lag. Messbar ist dieser Test nur im Mode-2-Fenster
Nov–Dez 1995 — dem Fenster des Floor-Beginns, des 5.12.1995-Wechsels und des
Konjunktions-Eintritts (der im Ops-Ära-Blatt benannte Confound); alle anderen
(Mode, Paar)-Achsen sind auf diesen Daten n-leer (0 geehrt).

## Messung 3 — Kandidaten-Treiber in den Floor (Gipfel gegen TE)

Treiber auf den fünf TE-fähigen Serien-Läufen (je Station/Mode ≥ 9 Tage):
M2 st14 (15 d), M2 st43 (14 d), M3 st43 (14 d), M2 st63 (12 d), M1 st63
(10 d, 1996-12-17..26). Drei Treiber-Lesarten:

**(a) ε (solare Elongation an der Erde, gemessene Geometrie je Zelle):** die
ε-Spanne über die Fenster misst 6–11° (M1-st63-Fenster Dez 1996 nur
19,4–25,8°). Einschränkung benannt: ε ist innerhalb eines Fensters glatt und
monoton, und die Phasen-Null zerstört genau diese Struktur (anti-konservativ
für glatte Treiber) — die Block-Null ist hier die belastbare Schwelle. TE
ε→Floor: **kein lag über beiden Nullen in irgendeinem Fenster**; nur
Phasen-Null-Überschreitungen an M2 st14 lag 2/3 (te 0,329/0,355 gegen Phasen-
Null 0,314/0,319, unter der Block-Null 0,359/0,388) und eine Block-Null-
Überschreitung an M3 st43 lag 1 (te 0,344 gegen 0,339, unter der Phasen-Null
0,414). Die Kontrolle Floor→ε liegt in vier der fünf Fenster unter beiden
Nullen; ein Einzel-Lag (M2 st43, lag 3) überschreitet Block- und Bedingungs-
Null (unter der Phasen-Null) — eine gemessene Grenze der Null-Kalibrierung
bei n = 12–14, kein Kopplungs-Beleg (Floor→ε ist physikalisch unmöglich;
der Einzel-Lag wird als Kalibrier-Rauschen benannt, nicht geglättet).
**Gipfel-Tor:** die ε-Mediane der lauten gegen die ruhigen Tage je Fenster
sind im Vorzeichen gemischt — M2 st14 13,5 gegen 15,9°, M2 st43 14,0 gegen
15,7°, M3 st43 13,4 gegen 16,8° (laut bei kleinerem ε), aber M2 st63 18,6
gegen 15,3° und M1 st63 24,1 gegen 22,5° (laut bei größerem ε): **kein
konsistentes ε-Tor für die Gipfel-Tage gemessen.**

**(b) konstruierter Betriebs-Schritt 5.12.1995** (0 vor / 1 ab dem
Modulationswechsel-Anker, markiert als konstruiert, kein gemessenes Asset):
nur ein Lauf trägt den Anker innen mit Seiten ≥ 3 (M2 st14, 1995-11-23..12-07,
Seiten n 12/3); an M2 st43 und M3 st43 liegt der Anker 2 Tage vor Laufende
(Seiten 12/2, nicht ausgewertet, benannt). TE Schritt→Floor unter beiden
Nullen in allen lags (beste 0,258 gegen Phasen-Null 0,352/Block 0,327); die
Kontrolle Floor→Schritt ebenfalls unter der Null.

**(c) ε-Band-Lage der lauten Tage über die Ära** (die einzige Gipfel-TE-
fokussierte Messung, die über die kurzen Läufe hinaus trägt): Die lauten
Tage aller neun Serien sind über die ε-Achse verteilt (Konjunktions-Fenster
1995/1996/1997, Oppositions-Fenster Juni 1996, Mitte-Fenster Sep/Nov 1996
tragen alle laute Tage) — die Gipfel folgen keiner ε-Selektion (konsistent
zur ε-Flachheit der α–Zeit-Blätter).

**Verdikt Messung 3: keine gerichtete Treiber-Kopplung über der Null.**
Weder ε noch der konstruierte Betriebs-Schritt treiben den Floor gerichtet
über beide Surrogat-Nullen; die einzigen Überschreitungen sind Einzel-Lags
gegen je eine der Nullen und die anti-konservative Phasen-Null. Die
Einschränkung der Glattheit (ε wenig Variiert und monoton in den Fenstern,
Phasen-Null anti-konservativ) ist benannt; die Block-Null und die
Reverse-Kontrolle tragen den Null-Befund.

## Verdikt

**Die station-gebundene Floor-Lautheit ist in ihrer Tages-Struktur gemessen:
kein Telegraph, keine Treppe, keine gerichtete Kopplung — die Flips sind
isolierte Gipfel auf einem wiederkehrenden ruhigen Sockel.** In den acht
längsten zusammenhängenden Boden-Fenstern der Ära (8–15 Tage) findet die
Segmentierung **keine anhaltende Stufe** (jedes Fenster fällt auf ein Niveau
zurück, p der besten Schnitte 0,10–0,77), obwohl dieselbe Methode eine
synthetische 7/8-Tage-Stufe sicher detektiert (p 0,0005). Über alle Serien
sind 133 von 137 lauten Episoden 1–3 Tage lang, nur 4 erreichen 4 Tage; die
laute Amplitude ist kontinuierlich gestreut (Serien-Mediane 4–32 Hz, Spanne
der mittleren 50 % Faktor 3–30, bis 530 Hz) — keine wiederkehrenden
Laut-Sprossen, der einzige wiederkehrende Sockel ist der ruhige (~0,04–0,09 Hz
in acht Serien). Die 105 robusten Flips reproduzieren exakt das n ≥ 30-Teilset
des Ops-Ära-Blatts (79 in M1+M2) und sind damit als isolierte
Einzel-Tag-/2–3-Tage-Gipfel benannt — der per-Pass-/per-Episode-Zustand bleibt
die getragene Lesart, eine Stufen-Lesart auf Wochen-Skala ist auf diesen Daten
nicht vorhanden (Fenster länger als 15 Tage existieren nicht, 0 geehrt). TE
station→station ist nur in einem einzigen Fenster messbar (Mode 2, Nov–Dez
1995, n 12–14 — das konfundierte Floor-Beginn-/Modulationswechsel-/
Konjunktions-Fenster) und liegt dort unter beiden Nullen; alle übrigen Paar-
Achsen sind auf diesen Daten n-leer. Treiber-TE (ε, konstruierter
5.12.1995-Schritt) zeigt keine Kopplung über der Null; die Glattheits-
Einschränkung der ε-Treiber und die unvollkommene Null-Kalibrierung bei
n = 12–14 (ein Floor→ε-Einzel-Lag über Block-/Bedingungs-Null) sind benannt.

**Was `pending` bleibt:** die Ursache der isolierten Gipfel-Tage selbst — ihr
Sitz (per-Pass-Verschaltung, Schleifen-/AGC-Zustand, Array-Teilnahme,
Betriebs-Kadenz) ist weiterhin nicht benannt. Eine gerichtete Lesart über
mehrere Stationen (Netz-Koordination) ist auf der Tages-Achse mangels
gemeinsamer Läufe nicht messbar — eine Sub-Tages-Achse (Pass-Beginn, Episoden-
Anfang innerhalb des Tages) oder die Receiver-Identität (`galileo_receiver.bin`,
CI/CDN-bereit laut Handover) sind die offenen, ungemessenen Pfade.

## Grenzen

- Die Tages-Serie ist über weite Strecken dünn besetzt: 20–30 % der
  Boden-Tage sind dünn (n < 30) und werden ausgewiesen, nicht klassiert; die
  Läufe sind kurz (längster 15 Tage), so dass Stufen länger als die
  Fensterlänge und die meiste Ära (Okt 1996 n = 0, Feb–Mai 1996 n = 0) nicht
  prüfbar sind. M3 st63 hat keinen Lauf ≥ 8 Tage (0 geehrt).
- Die Segmentierung ist ein deskriptiver Changepoint-Scan mit
  Permutations-Null (α 0,05, nicht über die Rekursion adjustiert); sie
  detektiert persistente Zwei-Niveau-Strukturen (Kontrolllauf), nicht aber
  periodische Muster (die 2-Tage-Periode an M1 st43 Nov 1996 wird als solche
  gemessen, nicht als Kopplung klassiert).
- TE bei n = 12–14 ist schwach; die Null-Kalibrierung zeigt einen
  Reverse-Kontroll-Einzel-Lag über Block-/Bedingungs-Null (M2 st43,
  Floor→ε) — ein Maß der Grenze, nicht geglättet.
- Der einzige TE-fähige station→station-Fenster (Mode 2, Nov–Dez 1995) ist
  das im Ops-Ära-Blatt benannte konfundierte Fenster (Floor-Beginn,
  5.12.1995-Wechsel, Konjunktions-Eintritt); eine Ursachen-Trennung ist dort
  nicht möglich.
- Serienwert für TE/Segmentierung ist log10(Tages-RMS) (genannt); die
  laut/ruhig-Zählungen nutzen die Referenz-Schwelle 1 Hz unverändert.

## Register-Satz

*Der Floor je (Station, Mode) ist in seiner Tages-Struktur kein Telegraph:
in den acht längsten zusammenhängenden Fenstern (8–15 Tage) findet die
Permutations-Segmentierung keine anhaltende Stufe (jedes Fenster fällt auf
ein Niveau, beste Schnitte p 0,10–0,77; Kontrolllauf detektiert eine
synthetische 7/8-Tage-Stufe mit p 0,0005), 133 von 137 lauten Episoden dauern
1–3 Tage und die laute Amplitude ist kontinuierlich gestreut (Mediane
4–32 Hz, bis 530 Hz) — die 105 robusten Flips (n ≥ 30, M1+M2 exakt das
Ops-Ära-n≥30-Teilset) sind isolierte Gipfel auf einem wiederkehrenden
ruhigen Sockel (~0,04–0,09 Hz), keine Stufen; TE station→station ist nur im
Mode-2-Fenster Nov–Dez 1995 messbar (n 12–14) und liegt dort unter beiden
Surrogat-Nullen (eine Block-Null-Überschreitung st14→st63 lag 1 unter der
Phasen-Null), alle übrigen Paar-Achsen sind n-leer; TE ε→Floor und
konstruierter 5.12.1995-Schritt→Floor ohne Kopplung über beiden Nullen
(Glattheits-/Null-Kalibrierungs-Grenzen benannt). Der Sitz der isolierten
Gipfel-Tage bleibt pending — die Sub-Tages-Achse und die Receiver-Identität
(galileo_receiver.bin) sind die offenen Pfade.*

## Status

`draft` (Entwurf für die Haupt-Session/Rat; TODO-Registerzeile ergänzt die
Haupt-Session). Probe `galileo_floor_stair_te` committet (`cargo check` 0/0,
RUSTFLAGS `-D warnings`), Report `/tmp/opencode/galileo_floor_stair_te_report.txt`.
Messung 1 vollständig auf den längsten Fenstern; Messungen 2/3 tragen die
n-Grenze der Boden-Abdeckung aus (ein TE-Fenster überhaupt, dort unter der
Null) — der Operator-Wortlaut „nichts done, wir finden ihn" bleibt gehalten.
