<!--
  title: Befund — Galileo-Floor-Boden gegen die Schwellen-Physik-Frage (Richtung E2, billige Nachprüfungen): laute (Station, Tag)-Zellen sind nicht die flachen Pässe (Elevations-Decken-Verteilung je Station überlappend, Nachbar-Tages-Flips bei Decken-Differenz ~0,03°), und die Ausbruchs-Episoden sind zweipopulig — ~42 % unter ~2 s (einzelne Samples, Cycle-Slip-/Neugreif-Transienten) neben anhaltenden Episoden bis Stunden
  class: befund
  date: 2026-09-06
  version: 1
  sha256: 5c239f02a926fac12cdb89555e88fe91be7b853985a8ff5134237d40eef04f36
  status: draft
  antwortet-auf: docs/befund/befund-galileo-ops-aera-floor.md docs/befund/befund-galileo-floor-stufen-te.md
  see-also: docs/befund/befund-galileo-simultan-intrapass-trk225.md docs/befund/befund-galileo-inpass-elevation.md
-->
# Befund: Galileo-Floor-Boden gegen die Schwellen-Physik-Frage (Richtung E2) — Elevations-Decke und Ausbruchs-Dauer der lauten Tage

## Frage & Bindung

Zwei billige Nachprüfungen (Richtung E2, aus der externen Recherche) gegen die
Schwellen-Physik-vs-Geräte-Frage des Galileo-Floor-Bodens: (1) sind die lauten
Pässe die **flachen** (niedriger Maximal-Stand der Sonde über dem Horizont je
Station → schwaches Signal → Schwellen-Physik)? (2) wie lang sind die
**Ausbrüche** (Regelkreis-Neugreifen = Sekunden vs Störer/Kalibrierung =
anders)?

Gebunden wie die Referenz-Linie: Floor-Sample = Stärke exakt −2560
(AGC-Klemmwert), Stationen 14/43/63, ground mode 1..3; Lock (|resid| > 1000 Hz)
vor dem Rauschen getrennt; Zell-RMS um den Zellen-Mittelwert; laut = Zell-RMS ≥
1 Hz; robuste Zelle = n ≥ 30. Die (Mode, Station, Tag)-Zellen reproduzieren die
Registratur **exakt** (robust 400 | laut 207 | ruhig 193). Für die
Elevations-/Ausbruchs-Messung werden die Mode-Zellen eines (Station, Tag) zu
**einer** (Station, Tag)-Zelle zusammengelegt (derselbe Pass-Bogen der Station,
die Mode-Trennung ist zeitlich, nicht geometrisch): robust 237 | laut 158 |
ruhig 79. Elevations-Proxy unverändert vom Elevation-Match-Befund:
Lehrbuch-GMST-topozentrische Elevation der Sonde (galileo_daily − earth
Baryzentrum) über den Stations-Horizont (`odp::dsn_station`-Geodäsie),
validiert dort über Pass-Gating; Decke (ceil) = Maximal-Stand über das
Tages-Intervall (90-s-Raster). Probe
`tools/measure/src/bin/galileo_floor_elevation_burst.rs` (neu, einzige
Repo-Änderung außer diesem Blatt; `cargo check` 0/0, RUSTFLAGS `-D warnings`),
Report `/tmp/opencode/galileo_floor_elevation_burst_report.txt`.

## n zuerst (0 geehrt)

4 647 387 Floor-Samples an den 70-m-Stationen (Mode 1..3, non-lock);
1 994 510 Lock-Cut-Samples getrennt; 0 non-finite. Elevation-Void über alle 237
robusten (Station, Tag)-Zellen: 0 (jede Zelle trägt die Ephemeriden-Elevation).

Mode-Ebene (Registratur, robust | laut | ruhig): M1 st14 62|32|30 · M2 st14
38|17|21 · M3 st14 40|25|15 · M1 st43 64|35|29 · M2 st43 35|16|19 · M3 st43
33|16|17 · M1 st63 68|34|34 · M2 st63 39|22|17 · M3 st63 21|10|11. (Station,
Tag)-Ebene: st14 robust 76 (laut 54, ruhig 22) · st43 80 (53/27) · st63 81
(51/30).

## Messung 1 — Elevations-Decke der lauten gegen die ruhigen (Station, Tag)-Zellen

Die Decke (Pass-Gipfel über dem Horizont) ist je Station über die Ära fast
konstant (Sonden-Deklination ~ −23°, daraus geometrisch konsistent: st14-Decke
31–36°, st63 26–31°, st43 73–78°); die laut/ruhig-Verteilung der Decke je
Station überlappt vollständig:

| St | laut n | Decke (°) med (p25/p75) | ruhig n | Decke (°) med (p25/p75) | Δ med (laut−ruhig) | Permutation p |
|---|---|---|---|---|---|---|
| 14 | 54 | 31,82 (31,57/33,92) | 22 | 32,93 (31,62/35,12) | −0,61° | 0,121 |
| 43 | 53 | 76,55 (75,27/77,60) | 27 | 77,00 (75,66/77,54) | +0,11° | 0,744 |
| 63 | 51 | 27,67 (26,61/29,43) | 30 | 27,62 (26,59/28,94) | +0,02° | 0,964 |

Keine Station trägt eine Decken-Trennung der laut/ruhig-Tage (Permutation des
Differenz-der-Mittel über 9999, zweiseitig: p 0,12–0,96). Spearman
log10(Zell-RMS) gegen Decke über alle robusten Zellen je Station: st14 −0,12
(75), st43 −0,14 (80), st63 −0,01 (81). **Quer über die Stationen** liest die
Decken-Achse keinen Laut-Gradienten: st43 mit nahe-zenitalen Pässen (Decke
~77°) ist auf 53/80 Tagen (66 %) laut, st14 (Decke ~32°) auf 54/76 (71 %) und
st63 (Decke ~28°) auf 51/81 (63 %) — die flachsten Pässe der Ära (st63) sind
nicht lauter als die zenitalen (st43).

Die **Nachbar-Tages-Flips** (laut↔ruhig an kalendarisch benachbarten robusten
Tagen derselben Station, das dominante Muster des Ops-Ära-Blatts) laufen bei
identischer Decke ab: st14 23 Flips, Median |Δ-Decke| 0,03° (alle 23 ≤ 2°);
st43 18 Flips, 0,02° (alle ≤ 2°); st63 20 Flips, 0,03° (alle ≤ 2°). Der laute
Tag des Flip-Paars hat nur 10/23, 8/18 bzw. 11/20-mal die höhere Decke — die
Lautheit wechselt bei festem Pass-Gipfel in beide Richtungen.

**Innerhalb der lauten Tage** sitzen die lauten Samples (|resid| > 1 Hz) auf
derselben Elevation wie die ruhigen desselben Tages (gepoolt je Station, Median
der lauten gegen alle Samples): st14 26,4° gegen 26,5° (n 112 637 / 940 150),
st43 49,3° gegen 49,3° (n 124 669 / 1 351 605), st63 22,6° gegen 22,4° (n
142 522 / 748 474). Laut-Samples unter 5° Elevation: 2 von 379 828 über alle
drei Stationen (die DSN-Maske schneidet den Horizont ab). Die gepaarte
Tag-interne Lesart zeigt eine schwache Neigung, dass die lauten Samples ~0,7–1,9° unter dem Tages-Median der ruhigen
Samples liegen (Median der Tag-Differenz
med_laut − med_ruhig: st14 −0,98° in 35/52 Tagen, st43 −1,89° in 32/52, st63
−0,69° in 31/49) — ein Flanken-Effekt an der Bogen-Kante, klein gegen die
laut/ruhig-Tages-Kontraste der Referenz und ohne jede Masse an der
Horizont-Kante.

**Messung 1 gemessen: die lauten (Station, Tag)-Zellen sind nicht die flachen
Pässe.** Weder innerhalb einer Station (Decken-Verteilungen überlappen, p
0,12–0,96) noch quer über die Stationen (die zenitale st43 ist so oft laut wie
die flachen st14/st63) trägt der Pass-Gipfel die Lautheit; die tages-scharfen
Flips laufen bei Decken-Differenzen um 0,03° ab, und die lauten Samples eines
lauten Tages verteilen sich wie der Pass selbst, ohne Massierung an der
Horizont-Kante. Die Grenze der Messung ist benannt: die Sonden-Deklination
steht über die Ära fest, so dass die Decke je Station kaum variiert — die
inner-stationäre Decken-Achse ist kurz; das Gewicht tragen der Quer-Stationen-
Vergleich, die Flip-Paare und die Tag-interne Elevations-Verteilung.

## Messung 2 — Dauer der Ausbruchs-Episoden auf den lauten Tagen

Episode = zusammenhängende Samples mit |resid| über der Schwelle (Lücke
zwischen über-Schwellen-Samples ≤ gap_s, innerhalb eines Laufs mit ≤ 600-s-
Lücken; die 1-s-Abtastung der Referenz); Dauer = Zeitspanne letztes − erstes
Sample. Ein einzelnes über-Schwellen-Sample trägt Spanne 0 s (keine zeitliche
Ausdehnung auf dem Abtast-Gitter, gezählt, nicht in der Spannen-Verteilung).

**Kanonische Zählung (Schwelle 10 Hz, gap 30 s):** auf 151 der 158 lauten Tage
(7 laute Tage tragen kein |resid| > 10 Hz — dort trägt das Band 1–10 Hz die
Lautheit) 500 Episoden: 165 einzel-Sample (33 %), Spannen-Verteilung der
Mehr-Sample-Episoden (n 335): min 0, p25 12 s, **Median 70 s**, p75 211 s, max
33 880 s (~9,4 h). Histogramm der 500 Episoden inklusive Einzel-Samples: < 2 s:
208 (42 %; davon 165 Einzel-Samples und 43 Spannen 0–2 s), 2–10 s: 24 (→ 46 %
≤ 10 s), 10–60 s: 87 (→ 64 % ≤ 60 s), 60–600 s: 140, 600–3600 s: 23, ≥ 1 h:
18. Die Bestätigungsläufe: Schwelle 1 Hz/gap 60 s → alle 158 lauten Tage, 687
Episoden, Median 68 s; Schwelle 100 Hz/gap 2 s → 137 laute Tage, 334 Episoden
(105 Einzel-Samples), Median 21 s, nur 4 Episoden ≥ 10 min (max 5 984 s).

**Je Anker-Pass (Schwelle 10 Hz, gap 30 s) die Episoden-Längen:** M1 st14
1995-11-24 (Decke 31,7°) 15 Episoden, 9 einzel-Sample, Mehr-Sample-Spannen
31–1 235 s (Median 176 s, Gesamtspanne 2 579 s — das mittlere Lärmband des
Passes); M2 st14 1995-11-24 identisch (derselbe Station-Tag); M3 st14
1995-12-05 eine Episode von 63 s; M3 st63 1995-11-27 4 Episoden (2
einzel-Sample, 1 s, 381 s); M1 st63 1996-06-26 2 Episoden (1 einzel-Sample,
89 s); M1 st43 1996-11-04 6 Episoden (3 einzel-Sample, Spannen 0–59 s); M3 st43 1995-12-04 eine Episode von 0 s Spanne (die ~±900-Hz-Samples am
Laufende, die den Tages-RMS tragen, teilen einen Zeit-Takt; keine Einzel-Sample-
Episode).

**Messung 2 gemessen: die Ausbruchs-Dauern sind zweipopulig, keine Klasse
dominiert allein.** Eine große Population sehr kurzer Episoden (42 % der
Episoden unter ~2 s, davon 165 einzelne Samples — das Cycle-Slip-/
Neugreif-Sekunden-Muster der H3-Familie) steht neben einer anhaltenden
Population: Median der Mehr-Sample-Spannen 70 s, 140 Episoden (28 %) dauern
1–10 min, 41 Episoden (8 %) über 10 min bis ~9,4 h (die Lärmbänder wie M1 st14 1995-11-24
und einzelne Bursts wie M3 st63 1995-11-27). Ein reines
Cycle-Slip/Acquisition-Sekunden-Modell trägt höchstens die zahlenmäßig große,
zeitlich kleine Population; die lauten Tage tragen zusätzlich anhaltende
Episoden der Minuten- bis Stunden-Klasse.

## Verdikt

**Die Elevations-Nachprüfung unterstützt die Schwellen-Physik-Lesart über den
Pass-Gipfel nicht; die Ausbruchs-Dauer-Nachprüfung zeigt eine Zweipopulation,
keine dominante Sekunden-Klasse.** (1) Die lauten (Station, Tag)-Zellen sind
nicht die flachen Pässe: die Decken-Verteilungen der lauten und ruhigen Tage
überlappen je Station vollständig (Permutation p 0,12–0,96), die Station mit
den zenitalen Pässen (st43, Decke ~77°) ist auf 66 % ihrer Tage laut — nicht
seltener als die flachen st14/st63 (63–71 % bei Decken 28–32°), die
Nachbar-Tages-Flips der Referenz laufen bei Decken-Differenzen mit Median
0,02–0,03° ab, und die lauten Samples eines lauten Tages verteilen sich über
die Elevation wie der Pass selbst (gepoolte Mediane der lauten gegen alle
Samples identisch auf ~0,1–0,3°; nur 2 von 379 828 lauten Samples unter 5°).
Die einzige gemessene Elevations-Neigung ist eine schwache Tag-interne (die
lauten Samples ~0,7–1,9° unter dem ruhigen Tages-Median in 31–35 von 49–52 lauten
Tagen) — ein Flanken-Effekt an der Bogen-Kante, klein gegen die Laut-Kontraste
der Referenz. (2) Die Ausbruchs-Dauer misst 500 Episoden auf 151 lauten Tagen
(Schwelle 10 Hz, gap 30 s): 42 % unter ~2 s (165 einzel-Samples eingeschlossen,
das Sekunden-Muster), aber Mehr-Sample-Median 70 s, 140 Episoden 1–10 min und
41 Episoden > 10 min (bis ~9,4 h); die Anker-Pässe tragen Spannen von 0 s
(einzelne Samples) bis 1 235 s (Lärmband M1 st14). Die Cycle-Slip/Neugreif-
Klasse ist zahlenmäßig groß, zeitlich klein; eine zweite, anhaltende Klasse
(Minuten–Stunden) trägt den Großteil der Ausbruchs-Zeit — konsistent zur
H3-Familie (Transienten) plus Lärmband-Struktur des Intra-Pass-Blatts.

**Was `pending` bleibt:** der Sitz der anhaltenden Episoden-Klasse
(Regelkreis-Zustand, Störer, Kalibrierung — die Dauer allein trennt sie nicht);
die Ursache der tages-scharfen Flips bei fester Decke; die Receiver-Identität
je Pass (`galileo_receiver.bin`, CI/CDN-bereit laut Handover); die
Elevations-Decke kann die laut/ruhig-Tage nicht erklären, und die Tag-interne
Flanken-Neigung ist als solche benannt, nicht als Mechanismus belegt.

## Grenzen

- Die Sonden-Deklination steht über die Boden-Ära nahe fest (~ −23°), darum
  variiert die Pass-Decke je Station nur wenig (st14 31–36°, st43 73–78°, st63
  26–31°) — die inner-stationäre Decken-Achse ist kurz; die belastbaren
  Vergleiche sind der Quer-Stationen-Vergleich, die Flip-Paare und die
  Tag-interne Elevations-Verteilung.
- Die (Station, Tag)-Zellen legen die Mode-Zellen eines Tages zusammen
  (derselbe Pass-Bogen); die Mode-Ebene der Registratur (400/207/193) ist
  separat und exakt reproduziert. Zell-RMS-Werte einzelner Anker weichen damit
  von der Mode-Zell-Referenz ab (z. B. M3 st43 1995-12-04 merged 5,32 Hz gegen
  10,52 Hz Mode 3), benannt, nicht geglättet.
- Einzel-Sample-Episoden haben auf dem 1-s-Gitter keine aufgelöste Spanne (τ =
  0 gezählt, nicht als Sekundenwert gefüllt); ihre wahre Dauer liegt unter der
  Abtast-Kadenz (~1–1,3 s).
- Elevations-Proxy: Lehrbuch-GMST-Sphärik (tdb ≈ UT1 ~0,25°, RA gegen
  Äquinoktium ~0,5°), validiert über Pass-Gating im Elevation-Match-Befund;
  keine Refraktion, keine Antennen-Maske über 0°.
- Episode-Gap und Schwelle sind gewählte Parameter (10 Hz/30 s kanonisch,
  Bestätigung bei 1 Hz/60 s und 100 Hz/2 s); die Verteilungsform (große
  Kurz-Population + anhaltender Schwanz) ist über alle drei Parameter stabil.

## Register-Satz

*Die lauten Galileo-Floor-(Station, Tag)-Zellen sind nicht die flachen Pässe:
die Elevations-Decken der lauten und ruhigen Tage überlappen je Station
vollständig (st14 med 31,82 gegen 32,93°, p 0,121; st43 76,55 gegen 77,00°, p
0,744; st63 27,67 gegen 27,62°, p 0,964), die zenitale st43 (Decke ~77°) ist
auf 53/80 Tagen laut — nicht seltener als die flachen st14/st63 (54/76 und
51/81 bei Decken 28–32°), die 61 Nachbar-Tages-Flips laufen bei
Decken-Differenzen mit Median 0,02–0,03° ab (alle ≤ 2°), und die lauten
Samples eines lauten Tages verteilen sich wie der Pass selbst (gepoolte Mediane
laut/alle identisch auf 0,1–0,3°; 2 von 379 828 lauten Samples unter 5°; nur
eine schwache Tag-interne Neigung ~0,7–1,9° tiefer in 31–35 von 49–52 Tagen). Die
Ausbruchs-Dauern auf 151 lauten Tagen (Schwelle 10 Hz, gap 30 s) sind
zweipopulig: 500 Episoden, 42 % unter ~2 s (165 einzel-Samples, das Cycle-Slip-/
Neugreif-Sekunden-Muster), aber Mehr-Sample-Median 70 s, 140 Episoden 1–10 min
und 41 Episoden > 10 min bis ~9,4 h (Anker: M1 st14 Lärmband 31–1 235 s, M3 st63
Burst bis 381 s, M1 st43/M1 st63/M3 st14 ≤ 90 s, M3 st43 0-s-Episode) — die
kurze Klasse ist zahlenmäßig groß, zeitlich klein; die anhaltende Klasse trägt
den Großteil der Ausbruchs-Zeit. Schwellen-Physik über den Pass-Gipfel ist
damit nicht unterstützt (Decken-Achse stationär, pending bleibt der Sitz der
anhaltenden Episoden und die Ursache der Flips bei fester Decke).*
