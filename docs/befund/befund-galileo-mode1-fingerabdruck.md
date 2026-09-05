<!--
  title: Befund — Galileo Mode-1-Fingerabdruck: Stärke-Split (Oszillator vs. Schwachsignal-PLL)
  class: befund
  date: 2026-09-05
  sha256: eb0ce767198e75aed069f8ed671a5c5e39367fc1b8bfcb1fb16ddc7bdcd8a9b9
  status: done
  antwortet-auf: docs/befund/befund-galileo-rausch-kurve.md
  see-also: docs/auftrag/auftrag-quiet-zone-uebertragung.md docs/paper/probe-front-dark-matter.md docs/befund/befund-galileo-mode1-snr-kurve.md docs/TODO.md
-->

# Befund: Galileo Mode-1-Fingerabdruck — Stärke-Split: Oszillator vs. Schwachsignal-PLL

## Frage & Bindung

Die Rausch-Kurve (`befund-galileo-rausch-kurve`, done) ließ offen, ob die Mode-1-Flachheit
(Einweg-Doppler, 8,2 Hz α-nah vs 7,5 Hz α-fern) eine Oszillator-Eigenschaft ist oder
Schwachsignal-PLL-Rauschen: in Einweg ist die Sonde der Sender, nach dem HGA-Verlust ist das
LGA-Signal schwach, und PLL-Rauschen ist empfangsstärke-abhängig, nicht
solarwinkel-abhängig.
Vorab gebunden: nur Mode 1; Lock-Übergänge (|resid| > 1000 Hz) vor dem Rauschen getrennt;
Rausch-Metrik = Median der Tages-RMS (Rausch-Kurve-Metrik); Stärke-Split in Quartile
(Signalstärke-Slot [7], Pioneer-Stärkekonstanz-Technik, Deduktion-24-Muster);
Verdikt-Protokoll: Rauschen stärkeabhängig → Schwachsignal-Term vorhanden; Rauschen konstant
über Stärke und flach auf beiden Achsen → Oszillator. Stations-Split 14/43/63, wenn n trägt.
Datenkette: `data/galileo_resid.bin` (GASR, Slot [5]=doppler_ref/10, [7]=signal_strength) +
`galileo_daily`/`earth`-Ephemeride. Probe `tools/measure/src/bin/galileo_mode1_strength_split.rs`
(`cargo run -p omegaflow-measure --bin galileo_mode1_strength_split`, Report
`reports/galileo_mode1_strength_split.txt`); Achsen-Probe `tools/measure/src/bin/galileo_mode1_elongation.rs`
(Report `reports/galileo_mode1_elongation.txt`). Beide `cargo check` 0/0.

## n zuerst (0 geehrt)

Mode 1: 9 743 574 Proben, 1 568 246 Lock-Übergänge, 8 175 328 Proben nach Lock-Ausschluss,
163 Tage, 1990-11-29 .. 1997-02-28. Stärke-Feld: 8 171 124 Proben ≠ 0, 4 204 Proben = 0
(getrennt als `s=0` geführt, nie in ein Quartil gelegt), 1108 diskrete Werte.
Die Stärke-Skala ist zweistufig: Q1 = exakt −2560 (AGC-Boden, ~26 % der Proben), die
Quartilschnitte liegen bei b1 = −2560, b2 = −1753, b3 = −1743; Q2..Q4 füllen das Plateau
−1760 .. −1727. Die Referenzfrequenz (Slot 5) ist über Stationen und Quartile konstant
(p50 = 22,013672 MHz je Quartil) — eine Konfiguration, kein Band-Mix; der Stärke-Gradient
ist Empfangsleistung, kein Band-Artefakt. Die Einheit des Stärke-Feldes ist nicht kalibriert;
nur die Ordnung wird benutzt.

## Tabelle 1 — Rauschen vs Stärke (gebundene Metrik: Median Tages-RMS je (Tag, Bin)-Zelle, ≥ 30 Proben)

| Bin | Stärke | Proben | Tage | Zellen | Median Tages-RMS |
|---|---|---|---|---|---|
| Q1 (schwach) | −2560 | 2 100 971 | 113 | 100 | 13,33 Hz |
| Q2 | −1760 | 2 046 405 | 105 | 102 | 4,56 Hz |
| Q3 | −1748 | 2 187 589 | 95 | 92 | 0,14 Hz |
| Q4 (stark) | −1729 | 1 836 159 | 114 | 114 | 1,46 Hz |
| alle | — | 8 175 328 | 163 | 163 | 8,20 Hz |
| s=0 | 0 | 4 204 | 16 | 3 | 10,62 Hz |

Die gebundene Tag-Metrik reproduziert die Rausch-Kurven-Flachheit (α 0–30°: 8,20 Hz, 12 Tage;
α 0–60°: 12,22 Hz, 17; α 120–180°: 7,47 Hz, 138; α 150–180°: 7,5 Hz, 128) — trägt aber ein
Artefakt, das unten benannt wird.

## Tabelle 2 — Stations-Split (Zellen (Tag, Station, Bin) ≥ 30 Proben) — die Säule des Verdikts

| Station | Q1 | Q2 | Q3 | Q4 |
|---|---|---|---|---|
| 14 (2,25 M Proben, 129 d) | 1,81 Hz (62) | 0,16 Hz (78) | 0,10 Hz (72) | 0,13 Hz (81) |
| 43 (3,78 M, 122 d) | 2,16 Hz (64) | 0,28 Hz (76) | 0,10 Hz (71) | 0,10 Hz (78) |
| 63 (2,13 M, 139 d) | 1,63 Hz (68) | 1,93 Hz (74) | 0,09 Hz (71) | 0,15 Hz (76) |

## Der Befund

**1. Das Rauschen ist empfangsstärke-abhängig.** An jeder der drei Stationen ist das schwächste
Bin (AGC-Boden, −2560) ~10- bis ~20-mal lauter als das starke Plateau: Station 14: 1,81 vs
0,10–0,16 Hz; Station 43: 2,16 vs 0,09–0,28 Hz; Station 63: 1,63 vs 0,09–0,15 Hz
(Stations-Anomalie benannt: Station 63 Q2 = 1,93 Hz, 74 Zellen — nicht geglättet). Ein
freilaufender Bordoszillator rauscht unabhängig von der Empfangsleistung; einen solchen
Gradienten trägt er nicht. Das Mode-1-Residuum ist daher **kein reines Oszillator-Rauschen**:
ein empfangsstärke-abhängiger Schwachsignal-Term ist gemessen (Deduktion-24-Muster: Rauschen
wächst bei schwachem SNR; hier als Bodenanstieg am AGC-Boden). Die Stärke-Skala ist grob
(Boden vs Plateau, kein feines SNR-Kontinuum) — die Aussage ist „Boden-Zustand ≈ 10–20× lauter
als Plateau-Zustand", nicht eine durchgängige ∝-Kurve.

**2. Der „8-Hz-Boden" von Mode 1 ist ein Tag-Pooling-Artefakt; die Pass-Wahrheit ist gemessen.**
Kontrolle (Station-Tag-Zellen, alle Bin): Station 14 Median 0,98 Hz (129), Station 43 2,14 Hz
(122), Station 63 3,69 Hz (136) — der gepoolte Tag-Median liegt bei 8,20 Hz (163). Das starke
Station-Tag-Rauschen liegt bei ~0,1 Hz (Q3/Q4). Die Tag-Metrik mischt mehrere Pässe/Stationen
desselben Tages. Auf Pass-Ebene (`befund-galileo-pass-segmentierung`) ist das Pooling-Artefakt
bestätigt (Pass-Boden 0,79–3,69 Hz, ruhigstes Pass-Zehntel 0,04 Hz); die Pass-Ebene widerlegt den
Stärke-Gradienten **nicht** (Pass-Median mischt Stärke-Zustände). Die Übertragung auf das
Mode-2-1,5-Hz-Fenster ist gemessen (`befund-galileo-mode2-station-split`): der 1,5-Hz-Wert ist
fragil (All-Lock-Tag-Zählung + Stations-Pooling), der Konjunktions-Station-Tag-Boden fällt auf
0,28–0,58 Hz.

## Die Achse: der Rausch-Kurven-„SEP" ist der Winkel am Sonnenort, nicht die Elongation

Gemessen (Elongations-Report, 163/163 Tage geometrisch aufgelöst): der SEP der Rausch-Kurve ist
der Winkel α am Sonnenort zwischen Erde- und Sonden-Vektor. Die solare Elongation ε ist der
Winkel an der Erde. Für eine äußere Sonde sind beide gegenläufig: α≈0 ⇔ Opposition (ε 150–180),
α≈150–180 ⇔ Konjunktions-Geometrie (ε 1–30). Gemessene Instanzen: 1990-11-30 α 1,0°, ε 157,8°
(1,03 AU); 1995-11-23 α 154,9°, ε 21,2° (5,27 AU); 1995-12-19 α 179,5°, ε 0,44° (5,30 AU);
1997-01-20 α 179,6°, ε 0,36° (5,12 AU). Der „Nah-Sonne-Arm“ der Rausch-Kurve (α 0–30, Mode 1:
12 Tage 1990 + 1996-06) steht auf der ε-Achse **nicht** bei ε≈0: die 1990-Tage tragen gemessen
ε ≈ 30–158° (Opposition/Nah-Erde), die 1996-06-Tage (α 8°, 5,19 AU) die Oppositions-Seite
(ε groß). Konsequenz für die Rausch-Kurve: der Mode-2/3-Trend und das 1,5-Hz-Fenster (α 150–180
bei 5–6 AU = Konjunktionsseite) sind auf der ε-Achse neu zu ziehen — `pending`, in diesem Blatt
nicht entschieden (Mode 2/3 nicht vermessen).

## Die Rat-Frage (Nah-Sonne 8,2 Hz, Lock-/Akquisitions-Selektion)

Auf der korrigierten Achse gemessen (Elongations-Report): ε 0–10°: 51 Tage, Median 5,76 Hz;
ε 10–30°: 82 Tage, 8,51 Hz; ε 30–60°: 12 Tage, 8,61 Hz; ε 60–120°: 8 Tage, 22,91 Hz
(Jupiter-Ära 1996-09/11, Stärke −2560); ε 120–150°: 0 Tage; ε 150–180°: 10 Tage, 8,20 Hz.
Das sonnennahe Einweg-Fenster (ε < 10°, 51 Tage, inklusive der Konjunktionen 1995-12 und
1997-01) trägt den ruhigsten Band-Median (5,76 Hz), nicht ein „zu ruhiges 8,2". Das 8,2 des
Einweg-Nah-Sonne-Arms war ein α-Label-Artefakt (Opposition/Nah-Erde-Tage). Eine
Lock-/Akquisitions-Selektion am schwachen Signal wird zur Erklärung der Flachheit nicht
gebraucht; die Flachheit selbst (keine Solargeometrie im Einweg-Kanal) bleibt auf der ε-Achse
bestehen (5,8–8,6 Hz über ε; die Ausnahme 60–120° ist Jupiter-Ära/Schwachsignal, keine
Sonnengeometrie). Laute Einzeltage (bis 277 Hz) stehen bei jeder Elongation und jeder Stärke —
Tag-/Pass-Ereignisse, unsegmentiert.

## Grenzen

- Stärke-Feld ohne Kalibrierung, grobe Skala (Boden vs Plateau); Q1 könnte neben SNR eine
  Empfänger-/Epochen-Klasse tragen — ref konstant und Q1 über 83 anti-Tage verteilt schließt
  Band-Mix aus, eine Epochen-Rest-Verwebung bleibt benannt.
- Tag-Metrik mischt Pässe; Pass-Segmentierung ist gemessen
  (`befund-galileo-pass-segmentierung`): Pass-Boden 0,79–3,69 Hz, ruhigstes
  Pass-Zehntel 0,04 Hz; der Stärke-Gradient ist auf Pass-Ebene nicht entscheidbar
  (Pass-Median mischt Stärke-Zustände), der Fingerabdruck-Befund bleibt
  unwiderlegt.
- α/ε-Inversion: Mode-2/3-Rausch-Kurve auf der ε-Achse neu zu ziehen — `pending`.
- Q2/Q3-„Nah"-Zellen (α-Metrik) sind leer (0 geehrt): der α-Split ist in den mittleren Bins
  ohne n; die SEP-Trennung trägt nur Q1 und Q4 (Tabelle 1 der Bins, n in den Reports).

## Register-Satz

*Das Einweg-Residuum trägt einen empfangsstärke-abhängigen Term: am AGC-Boden ist das Rauschen
je Station etwa zehn- bis zwanzigmal lauter als im starken Plateau. „Oszillator" ist damit
nicht belegt, sondern widerlegt; der Schwachsignal-Term ist gemessen. Die Achse der Rausch-Kurve
ist der Winkel am Sonnenort, nicht die Elongation — für die äußere Sonde invertiert; der
Nah-Sonne-Arm war kein Nah-Sonne-Sample. Auf der Elongations-Achse bleibt die Einweg-Flachheit
bestehen, und das „zu ruhige 8,2" löst sich in ein Label-Artefakt auf.*

## Status

`done` — der Rat hat das Verdikt gehalten (2026-09-05). Die Proben und Reports sind lokale
Lauf-Artefakte, nicht committet.
