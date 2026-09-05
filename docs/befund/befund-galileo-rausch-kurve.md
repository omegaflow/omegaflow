<!--
  title: Befund — Galileo-Rausch-Kurve: Achsen-Revision (α statt ε), Plasma-Deutung unverifiziert
  class: befund
  date: 2026-09-05
  version: 2
  sha256: b3b91c16c14bc74e9e36862d863fa86deb4cba99467da58bf9009aa1a3fcbfc4
  status: done
  antwortet-auf: docs/auftrag/auftrag-quiet-zone-uebertragung.md
  see-also: docs/befund/befund-galileo-gwe-bestand.md docs/befund/befund-galileo-mode1-fingerabdruck.md docs/befund/befund-galileo-rausch-kurve-epsilon.md docs/auftrag/auftrag-quiet-zone-vorfilter.md docs/TODO.md
-->

# Befund: Galileo-Rausch-Kurve — Achsen-Revision (α statt ε), Plasma-Deutung unverifiziert

## Frage & Bindung

Die Quiet-Zone-Methode (Rauschen verorten → Achse finden → Boden messen) wurde
an Pioneer *entwickelt*; dieser Lauf prüft sie an Galileo. Vorab gebunden
(`auftrag-quiet-zone-uebertragung`): n je Mode je Band **zuerst**, dann Kurve;
**Distanz und die solare Achse**; der **Mode-Split als Entscheidungsachse**;
Lock-Übergänge als eigene Klasse (`n_lock`), nie als Rauschen. Datenkette:
`galileo_atdf_compiler` (TRK-2-25 → GASR-Residuum-Serie) → `galileo_noise_geo`
(Residuum-RMS je Tag/Mode/Station gegen Distanz/Achse aus Horizons
`galileo_daily`). Bestand: 14 077 825 Residuen, 138 TDF-Dateien (1990–97,
CDN `pds-ppi.igpp.ucla.edu/galileo_resid.bin`).

**Achsen-Revision (Rat 2026-09-05, im Code gemessen):** Der von
`galileo_noise_geo.rs` gerechnete Winkel war **nicht** die solare Elongation ε
(Winkel an der Erde — Armstrong-Woo-Estabrook 1979; Asmar 2005:
„Sun-Earth-spacecraft angle"), sondern **α, der Winkel am Sonnenort**
(Erde–Sonne–Sonde). Für die äußere Sonde sind α und ε ~komplementär. Die
Zahlen dieses Blatts sind dieselben Messungen; nur die Achse war falsch
benannt. Die Folge ist keine Umbenennung, sondern eine **Inversion des
Kernbefunds** — Abschnitt „Die Achsen-Revision".

## n-Tabellen (zuerst, wie gebunden)

| Mode | Tage | Samples | Lock-Übergänge |
|---|---|---|---|
| 1 (Einweg) | 163 | 9 743 574 (69 %) | 1 568 246 |
| 2 (Zweiweg) | 109 | 3 110 045 (22 %) | 157 784 |
| 3 (Dreiweg) | 89 | 1 224 206 (9 %) | 268 480 |

Distanz-Achse — n je Mode je Band:

| Mode | 0–1 AU | 1–2 AU | 4–5 AU | 5–6 AU |
|---|---|---|---|---|
| 1 | 4 Tage | 4 | 3 | **152 Tage** |
| 2 | 5 | 8 | — | **96 Tage** |
| 3 | 4 | 8 | — | **77 Tage** |

Die sonnennahen Bänder (0–2 AU) tragen 4–8 Tage — **zu dünn für Trendpunkte.**
Die Distanz-Kurve ist nur am fernen Ende (5–6 AU) getragen; die 4–8-Tage-Werte
werden nicht als Kurve gezeichnet.

## Der Hauptbefund — Geometrie im kohärenten Kanal, Flachheit im inkohärenten

| Mode | α 0–30° (Opposition) | α 150–180° (Konjunktion) | Faktor |
|---|---|---|---|
| 1 | 8,2 Hz (12 Tage) | 7,5 Hz (128 Tage) | flach (1,1×) |
| 2 | 42,0 Hz (18 Tage) | 1,5 Hz (73 Tage) | **28×** |
| 3 | 79,5 Hz (16 Tage) | 6,7 Hz (56 Tage) | **12×** |

Der 28×-Fall (Zweiweg) und der 12×-Fall (Dreiweg) sind **zwei getrennte Daten**,
nicht ein Band. Das kohärente Doppler fällt steil von Opposition zu Konjunktion;
Einweg bleibt flach. **Zwei Messkanäle, dieselbe Sonde, dieselben Stationen** —
der Unterschied selbst ist ein Hinweis auf den Mechanismus (kohärenter Rundlauf
vs. gesplitteter Downlink). Der Mode-Vergleich ist epochal geteilt; der
α-Vergleich (Opposition vs. Konjunktion) ist es nicht — die α-Bins sitzen in
verschiedenen Ären (siehe Grenzen).

## Die Achsen-Revision — was der erste Befund behauptete und was die Korrektur ist

**Die Fehlbezeichnung.** `galileo_noise_geo.rs` rechnete
`cos α = dot(e→, p→) / (r_earth · r_probe)` — den Winkel **am Sonnenort** α
(Erde–Sonne–Sonde). Die Blätter nannten ihn „SEP". Die solare Elongation ε ist
der Winkel **an der Erde** (Sonne–Erde–Sonde). Für die äußere Sonde sind α und ε
komplementär: α≈0 ⇔ Opposition (ε 150–180°), α 150–180° ⇔ Konjunktion
(ε 1–30°). Gemessene Instanzen: 1990-11-30 α 1,0° / ε 157,8° / 1,03 AU;
1997-01-20 α 179,6° / ε 0,36° / 5,12 AU.

**Die invertierte Deutung.** „α 0–30° = 42 Hz (sonnennah)" ist α 0–30° =
**Opposition** (Erd-Cruise 0–2 AU; die Opposition wiederholt sich auch auf der
fernen Seite — 1996-06 trägt α 8° bei 5,19 AU). „α 150–180° = 1,5 Hz
(sonnenfern)" ist α 150–180° = **Konjunktion** = 5–6 AU Jupiter. Der kohärente
Kanal ist **laut an Opposition, leise an Konjunktion — das Gegenteil von
Plasma-Szintillation** (die Konjunktion laut, Opposition leise macht). **Die
Plasma-Schlussfolgerung des ersten Blatts hält nicht** — der 28×/12×-Fall ist
ein Distanz-/Ära-Confound, der Elongation nicht zuschreibbar. Die Achsen sind
**stark verwoben** (Opposition ↔ 0–2 AU, Konjunktion ↔ 5–6 AU), aber nicht
identisch — die Opposition überspannt beide Distanzen.

**Der Stoßparameter.** Die Formel „b = r·sin(SEP)" galt für α und war falsch.
Der Stoßparameter der Sichtlinie ist **b ≈ 1 AU · sin ε** (Abstand Erde–Sonne ×
Sinus der Elongation), nicht r·sin α.

**Was bleibt (unberührt von der Revision).** Die Mode-1-Flachheit (flach auf
beiden Achsen — gemessen im `befund-galileo-mode1-fingerabdruck`), die
Distanz-n-Tabellen, die Lock-Übergangs-Klasse.

**Mode-2/3 auf der ε-Achse neu ziehen — `pending`.** Der Mode-2/3-Trend und das
1,5-Hz-Fenster sind auf der ε-Achse neu zu ziehen; das bestätigt oder tötet die
Plasma-Deutung. Register-Pflicht, kein Befund dieser Revision.

## Galileos ruhiges Fenster — die zweite ~1-Hz-Nummer

Mode 2, α 150–180°: **1,5 Hz** (73 Tage); das zugehörige Distanzband 5–6 AU
trägt 96 Tage bei 1,6 Hz. Geometrie korrekt benannt: **Konjunktion, 5–6 AU**
(auf der ε-Achse ist Konjunktion ε≈0, also sonnennah — das alte Label
„sonnenfern" war der α/ε-Fehler). Metrik exakt: Median der Tages-RMS. **Der
Boden steht unter Frage**: Tag-Pooling. Das Pooling ist für Mode 1 gemessen
(8,2 Hz gepoolt → 0,98/2,14/3,69 Hz Station-Tag, siehe
`befund-galileo-mode1-fingerabdruck`); der Mode-2-Wert 1,5 Hz ist **nicht**
station-tag-gesplittet — nicht extrapolieren, der Mode-2-Station-Tag-Split ist
`pending`.

## Stations-Nebenfund

Zehn Stationen erscheinen: 43 (6 157 151 Samples, 147 Tage), 14 (3 905 929,
141), 63 (3 614 078, 151) = **97 %**; die 34m-Unterstationen 12/15/24/34/42/
45/61 nur tageweise. Das ist Banden-relevant: die 20-s-Banden-Stationsliste
wird breiter, als Pioneer sie zeigte. Die Stationen 12/15/24/34/42/45/61
gehören in den GWE-Banden-Test-Auftrag als **Erwartungsliste**.

## Grenzen

- Sonnennahe Distanzbänder (0–2 AU) n-leer (4–8 Tage) — keine Trendpunkte,
  `pending` (die lauten Fenster-Pässe fehlen im TDF-Bestand).
- Mode-2/3-Rausch-Kurve auf der ε-Achse neu ziehen — `pending` (diese Revision
  hat die Achse korrigiert, nicht neu gezogen; Mode 1 ist auf ε gezogen,
  Mode 2/3 nicht).
- Mode-2-Station-Tag-Split — `pending` (das 1,5-Hz-Fenster ist gepoolt, nicht
  station-tag-gesplittet).
- α–Zeit–Sonnenzyklus verwoben: 1990–97 reicht vom Sonnenmaximum (früh) ins
  Minimum (spät); die α-niedrigen Bins (Opposition/Cruise) sitzen früh, die
  α-hohen (Konjunktion/Jupiter) spät — gegen die Datei-Epochen zu verifizieren.
- Orbiter-Geometrie (Manöver, Okkultations-Segmente) ist nicht segmentiert;
  die 5–6-AU-/α-150–180°-Werte mischen Cruise- und Encounter-Fenster.

## Register-Satz

*Die Rausch-Kurve stand auf einer invertierten Achse: der gemessene Winkel war
α am Sonnenort, nicht die solare Elongation ε. Der kohärente Kanal ist laut an
Opposition und leise an Konjunktion — das Gegenteil von Plasma-Szintillation;
die Plasma-Deutung ist damit unverifiziert, der 28×/12×-Fall ein
Distanz-/Ära-Confound. Die Mode-1-Flachheit bleibt, die Mode-2/3-Kurve ist auf
der ε-Achse neu zu ziehen.*

## Status

`done` (Revision v2). Die Achse ist korrigiert (α statt ε), die Nahe/Fern-Labels
invertiert, die Plasma-Deutung auf `unverifiziert` gestuft. Der Mode-2/3-Trend
auf der ε-Achse ist `pending`, ebenso der Mode-2-Station-Tag-Split. Der
ursprüngliche `done`-Kernbefund („Plasma auf der Sichtlinie") ist revidiert —
er war ein Distanz-/Ära-Confound auf einer falsch benannten Achse.

Folge-Befund (`befund-galileo-rausch-kurve-epsilon`, done): die Mode-2/3-Kurve
ist auf der ε-Achse neu gezogen — der kohärente Fall ist invers zur
Plasma-Erwartung (leise an Konjunktion, laut an Opposition); die
Plasma-Deutung ist dort auf `getötet` gestuft. Der Distanz-/Ära-Confound ist
zerlegt (`befund-galileo-alpha-zeit-sonnenzyklus`, done): die Fall-Magnitude ist
Ära/Sonnenzyklus-konfundiert (kollabiert unter Ära-Kontrolle auf ~2×), die
Richtung (kein laut-bei-kleinem-ε) hält.
