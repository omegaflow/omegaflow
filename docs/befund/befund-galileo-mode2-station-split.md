<!--
  title: Befund — Galileo Mode-2-Station-Tag-Split: 1,5-Hz-Wert fragil (All-Lock-Tag-Zählung) + Stations-Pooling senkt auf 0,28–0,58 Hz
  class: befund
  date: 2026-09-05
  sha256: a9ca9078e9068c2d81ff9a258534d9ba75bbd2cfca975ad367f33a4ba7f805f2
  status: done
  antwortet-auf: docs/befund/befund-galileo-rausch-kurve-epsilon.md docs/befund/befund-galileo-mode1-fingerabdruck.md
  see-also: docs/auftrag/auftrag-quiet-zone-uebertragung.md docs/TODO.md
-->

# Befund: Galileo Mode-2-Station-Tag-Split — 1,5-Hz-Wert fragil (All-Lock-Tag-Zählung), Stations-Pooling senkt auf 0,28–0,58 Hz

## Frage & Bindung

Der Mode-1-Fingerabdruck (done) hat den „8,2-Hz-Boden" als Tag-Pooling-Artefakt
gemessen: per (Tag, Station)-Zelle fällt Mode 1 auf 0,98/2,14/3,69 Hz (Stationen
14/43/63); der Tag-Median mischt mehrere Pässe/Stationen desselben Tages. Dieselbe
Frage war für das ruhige Mode-2-Fenster bei 1,5 Hz (ε 0–30°, 77 Tage,
`befund-galileo-rausch-kurve-epsilon`) `pending` — dieser Entwurf misst den
Mode-2-Station-Tag-Split.

Gebunden: nur Mode 2 (Slot [3]); Lock-Übergänge (|resid| > 1000 Hz) vor dem
Rauschen getrennt; Rausch-Zelle = per (Tag) und per (Tag, Station), Zell-RMS um den
Zell-Mittelwert, Zellen ≥ 30 Proben, Median über die Zellen (Mode-1-Zell-Definition,
exakt gespiegelt). Zusätzlich wird die Referenz-Metrik der Rausch-Kurve reproduziert
(Median der Tages-RMS über alle (Mode-2, Tag)-Zellen, ohne Zell-Minimum, inklusive
Tage, deren Mode-2-Aufzeichnung vollständig im Lock liegt) — die Zell-Definition, die
die 1,5 Hz der ε-Kurve trägt. Geometrie pro Tag aus `galileo_daily` + `earth`-Barycenter
wie in den bestehenden Sonden: α = Winkel an der Sonne, ε = Elongation an der Erde.
Datenkette: `data/galileo_resid.bin` (GASR). Probe
`tools/measure/src/bin/galileo_mode2_station_split.rs`, Report
`reports/galileo_mode2_station_split.txt`. `cargo check` 0/0.

## n zuerst (0 geehrt)

Mode 2: 3 110 045 Proben, 157 784 Lock-Übergänge, 2 952 261 Proben nach
Lock-Ausschluss, 109 Tage mit Mode-2-Aufzeichnung, 107 Tage nach Lock-Ausschluss
(≥ 1 Nicht-Lock-Probe), 2 Tage (1997-01-07, 1997-01-21), deren Mode-2-Aufzeichnung
vollständig im Lock liegt — keine Nicht-Lock-Probe, keine Rausch-Zelle (0 geehrt:
absent, kein Wert). Spanne 1990-11-29 .. 1997-02-25. Proben je Tag (nach Lock):
Median 19 454, min 64, max 86 309. Stationen: 12 (56 179 Proben, 7 Tage), 14
(705 113, 52), 42 (98 618, 4), 43 (1 150 165, 53), 61 (55 516, 4), 63 (886 670, 74).
Geometrie: 109/109 Tage aufgelöst. Konjunktionsfenster ε < 30°: 77 Tage mit
Aufzeichnung (75 mit Nicht-Lock-Proben); α ≥ 150°: 73 Tage.

## Tabelle 1 — Gepoolter Tages-RMS, Mode 2 (Hz; Zellen in Klammern)

| Fenster | gebundene Metrik (Zellen ≥ 30) | Rausch-Kurven-Referenz (alle (Mode, Tag)-Zellen) |
|---|---|---|
| alle Mode-2-Tage | 2,79 Hz (107) | 3,84 Hz (109, inkl. 2 All-Lock-Tage) |
| Konjunktion ε < 30° | **0,65 Hz (75)** | 1,45 Hz (77, inkl. 2 All-Lock-Tage) |
| α ≥ 150° | 1,45 Hz (71) | 1,53 Hz (73) |

Die Referenz-Metrik reproduziert die Rausch-Kurve (ε 0–30° = 1,45 Hz, 77 Tage;
α 150–180° = 1,53 Hz, 73 — die ε-Kurve rundet auf 1,5 Hz; die 0,05-Hz-Differenz
1,45/1,53 zur gerundeten 1,5 ist Median-Konvention/Band-Rand, unverifiziert
benannt). Die gebundene Zell-Definition (≥ 30 Proben, All-Lock-Tage absent) senkt
den Konjunktions-Median auf 0,65 Hz. Der Mechanismus ist ein **Median-Index-Effekt**,
kein „Zählen von NaN-Zellen": die Referenz nimmt `list[len/2]` über die 77
Einträge (die 2 All-Lock-Tage sortieren als absent ans Ende); ihr Ausschluss
schiebt den oberen Median-Index von s[37] auf s[38] über eine bimodale Lücke der
Tages-RMS-Verteilung — von 1,46 auf 0,65 Hz (gemessen, unten belegt).

## Tabelle 2 — Stations-Split Mode 2: Median der (Tag, Station)-Zell-RMS (Zellen ≥ 30)

| Station | med über alle Tage (Zellen, Tage) | med Konjunktion ε < 30° (Zellen, Tage) |
|---|---|---|
| 14 | 1,50 Hz (52, 52) | **0,29 Hz (32, 32)** |
| 43 | 1,61 Hz (52, 52) | **0,58 Hz (33, 33)** |
| 63 | 2,87 Hz (74, 74) | **0,28 Hz (56, 56)** |

Zell-Proben (alle Tage, med/min/max): Station 14: 13 562 / 44 / 41 421; Station 43:
25 672 / 277 / 46 296; Station 63: 11 302 / 385 / 33 067. Jede Station trägt je Tag
genau eine Zelle (Zellen = Tage). Station 63 trägt 74 der 107 Tage und 56 der 75
Konjunktions-Tage — die Tag-Zelle ist über weite Strecken Station 63.

## Der Befund

**1. Das 1,5-Hz-Fenster ist kein robuster Boden.** Die 1,5 Hz der ε-Kurve stehen auf
der Referenz-Zell-Definition, die auch die zwei All-Lock-Tage (1997-01-07, ε 10,2°;
1997-01-21, ε 1,0° — keine Nicht-Lock-Probe) als NaN-Tageszellen zählt. Dieselbe
Metrik über die 75 Tage mit Nicht-Lock-Proben ergibt 0,65 Hz (Zellen ≥ 30: identisch,
alle 75 tragen ≥ 568 Proben). Die Tages-RMS-Verteilung im Fenster ist bimodal: 38 der
75 Zellen liegen ≤ 0,65 Hz (0,023–0,647), darüber folgt ein Schwanz bis 293 Hz
(1996-01-01, ε 9,6°; 1997-01-22, ε 1,7° — die tiefste Konjunktions-Geometrie trägt
die lautesten Tage, nicht die ruhigsten). Der Wert „1,5 Hz" ist der obere Median über
eine Lücke dieser Verteilung, verschoben durch zwei absent-Zellen — kein gemessener
Boden.

**2. Der Station-Tag-Split fällt unter den gepoolten Wert — die Pooling-Richtung von
Mode 1 bestätigt sich, metrik-abhängig.** Im Konjunktionsfenster liegen die
(Tag, Station)-Mediane bei 0,29 (14), 0,58 (43), 0,28 Hz (63) — ein Faktor 2,5–5
unter der gepoolten Referenz (1,45 Hz) und ein Faktor **1,1–2,3** unter der
gebundenen Tag-Metrik (0,65 Hz; Station 43: 0,65/0,58 = 1,1×). Wie bei Mode 1 mischt
die Tag-Zelle Stationen desselben Tages; getrennt fällt das Rauschen. Die Richtung
des Mode-1-Befunds (Tag-Pooling bläht auf) ist damit auch für Mode 2 gemessen —
mit Station 43 als schwächstem Faktor.

**3. Aber: der Boden fällt nicht auf das Mode-1-Stark-Signal-Niveau (~0,1 Hz).** Der
Mode-1-Fingerabdruck fand im starken Plateau Q3/Q4 ~0,1 Hz je Station. Mode 2 bleibt im
Konjunktionsfenster bei 0,28–0,58 Hz — ein echter, stationsabhängiger Restboden
(Station 43 ≈ doppelt so hoch wie 14/63), ~3- bis 6-mal über dem Mode-1-Starkwert.
Über alle Tage (nicht nur Konjunktion) liegen die Station-Tag-Mediane (1,50/1,61/2,87)
nahe am gepoolten 2,79 Hz — Station 63 fällt gar nicht (2,87 vs 2,79). Der Mode-2-Tag-
Median ist über weite Strecken die Station-63-Zelle; die lauten Mode-2-Tage verteilen
sich über Station-Tage, nicht über eine schwaches-Signal-Klasse wie Mode 1 (dort
AGC-Boden −2560, ~10–20× lauter). Ein Stärke-Split (Slot [7]) wurde hier nicht
angewandt — `pending`.

**Verdikt:** Zwei Effekte, getrennt benannt — (a) **All-Lock-Tag-Zählung**: der
Referenzwert 1,5 Hz hängt an zwei All-Lock-Tagen (1997-01-07/21, keine Nicht-Lock-
Probe), die die Referenz über 77 Einträge zählt; ohne sie misst dieselbe Metrik
0,65 Hz (Median-Index-Effekt, oben). (b) **Stations-Pooling**: auf (Tag, Station)-
Ebene fällt die Konjunktion weiter auf 0,28–0,58 Hz (14/63 ≈ 0,29, 43 = 0,58). Kein
einzelner genuiner Boden — der Wert „1,5 Hz" trägt die Konjunktions-Wahrheit nicht.
Ein genuiner Stations-Restboden der Konjunktion ist gemessen, aber metrik-abhängig:
~0,3–0,6 Hz je Station-Tag-Metrik (auf Pass-Ebene 0,3–1,2 Hz, siehe
Pass-Segmentierung) — über dem ~0,1-Hz-Niveau des Mode-1-Starksignals und
stationsabhängig.

## Grenzen

- Zwei All-Lock-Tage (1997-01-07/21) tragen keine Nicht-Lock-Probe: in der gebundenen
  Metrik absent (0 geehrt), in der Rausch-Kurven-Referenz als NaN-Tageszelle gezählt —
  beide Zahlen gemessen, die Metrik-Differenz benannt, nicht geglättet.
- Die Konjunktions-Tagesverteilung ist bimodal (0,02–293 Hz); ein Median über diese
  Zellen ist ein Lagemaß einer Mischung, kein Boden. Die lautesten Tage stehen bei der
  tiefsten Konjunktions-Geometrie (ε < 10°) — als Beobachtung gemessen, nicht gedeutet.
- (Tag, Station)-Zelle kann noch Pässe mischen; die Pass-Segmentierung
  (`befund-galileo-pass-segmentierung`) ist getan und misst den Konjunktions-Boden
  auf Pass-Ebene (0,29/0,87/1,20 Hz, Stationen 14/43/63). Stärke-Split (Slot [7])
  für Mode 2 nicht angewandt — `pending`.
- 1,5→0,65-Reconciliation gegen die Pass-Segmentierung offen: die
  Pass-Segmentierung deutet die Differenz als acht in ε 30–60° verschobene laute
  Tage (77→75 schließt arithmetisch nicht); dieser Entwurf als Zählung zweier
  All-Lock-Tage. Die Tagesmengen-Schnitte der beiden Deutungen sind nicht
  gegeneinander gemessen — `pending`.
- Stationen 12/42/61 tragen wenige Tage (4–7) und keine Konjunktions-Zellen ≥ 30 —
  ohne n im Fenster (0 geehrt). Geometrie je Tag einmal (Tagesanfang), keine
  Intra-Tag-Geometrie.

## Register-Satz

*Der Mode-2-Wert 1,5 Hz ist fragil: er hängt an der Zählung zweier All-Lock-Tage
(ohne sie misst dieselbe Metrik 0,65 Hz, Median-Index-Effekt) und am
Stations-Pooling (die (Tag, Station)-Mediane der Konjunktion fallen auf
0,29/0,58/0,28 Hz, Faktor 1,1–2,3). Ein genuiner Stations-Restboden der Konjunktion
ist metrik-abhängig gemessen (~0,3–0,6 Hz je Station-Tag-Metrik; auf Pass-Ebene
0,3–1,2 Hz, Pass-Segmentierung getan), über dem ~0,1-Hz-Niveau des
Mode-1-Starksignals. Der Mode-2-Stärke-Split bleibt ausstehend.*

## Status

`done` (Rat gehalten, 2026-09-05). Messung vollständig, Probe
`galileo_mode2_station_split` (cargo check 0/0), Report
`reports/galileo_mode2_station_split.txt`. Der 1,5-Hz-Wert ist fragil
(All-Lock-Tag-Zählung + Stations-Pooling, Faktor 1,1–2,3); ein einzelner Boden
nicht getragen.
