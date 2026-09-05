<!--
  title: Befund — Galileo-Rausch-Kurve: SEP trennt, was Pioneer verwob
  class: befund
  date: 2026-09-05
  sha256: 028bbf1ecd4db41aacd6070f3d5d86e3b1484fb50dc12ccf1ea9088ec607d12c
  status: done
  antwortet-auf: docs/auftrag/auftrag-quiet-zone-uebertragung.md
  see-also: docs/befund/befund-galileo-gwe-bestand.md docs/auftrag/auftrag-quiet-zone-vorfilter.md docs/TODO.md
-->

# Befund: Galileo-Rausch-Kurve — SEP trennt, was Pioneer verwob

## Frage & Bindung

Die Quiet-Zone-Methode (Rauschen verorten → Achse finden → Boden messen) wurde
an Pioneer *entwickelt*; dieser Lauf prüft sie an Galileo. Vorab gebunden
(`auftrag-quiet-zone-uebertragung`): n je Mode je Band **zuerst**, dann Kurve;
**Distanz und SEP als Achsen**; der **Mode-Split als Entscheidungsachse**;
Lock-Übergänge als eigene Klasse (`n_lock`), nie als Rauschen. Datenkette:
`galileo_atdf_compiler` (TRK-2-25 → GASR-Residuum-Serie) → `galileo_noise_geo`
(Residuum-RMS je Tag/Mode/Station gegen Distanz/SEP aus Horizons
`galileo_daily`). Bestand: 14 077 825 Residuen, 138 TDF-Dateien (1990–97,
CDN `pds-ppi.igpp.ucla.edu/galileo_resid.bin`).

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
werden hier nicht als Kurve gezeichnet (die vorab gebundene Warnung, sichtbar
gemacht).

## Der Hauptbefund — Geometrie im kohärenten Kanal, Flachheit im inkohärenten

| Mode | SEP 0–30° (sonnennah) | SEP 150–180° (sonnenfern) | Faktor |
|---|---|---|---|
| 1 | 8,2 Hz (12 Tage) | 7,5 Hz (128 Tage) | **flach (1,1×)** |
| 2 | 42,0 Hz (18 Tage) | 1,5 Hz (73 Tage) | **28×** |
| 3 | 79,5 Hz (16 Tage) | 6,7 Hz (56 Tage) | **12×** |

Kohärentes Doppler (Mode 2/3) fällt 12–28× mit wachsender Sonnenelongation —
plasma-getrieben. Einweg (Mode 1) bleibt flach. **Zwei Messkanäle, dieselbe
Sonde, dieselbe Epoche, dieselben Stationen — und das Plasma zeigt sich nur in
dem Kanal, der es tragen müsste.** Das ist die eingebaute Negativ-Kontrolle:
fiele der SEP-Trend aus einem Artefakt (Station/Ära/Reduktion), fiele Mode 1
mit. Der Einwand gegen den Trend muss erklären, warum er ausgerechnet im
kohärenten Kanal sitzt.

**Warum SEP die richtigere Achse ist:** Plasma sitzt auf der *Sichtlinie*, nicht
auf der Radialentfernung — SEP (Sonne-Erde-Sonde-Winkel) ist der geometrisch
korrekte Stoßparameter-Proxy. Pioneer entfernte sich linear: Distanz und SEP
fielen dort zusammen, die zwei Achsen waren **verwoben** und nie trennbar.
Galileo (Orbiter, durchläuft alle Winkel) trägt die Achse, die Pioneer nie
hatte — an Galileo wird **erstmals** getrennt, was an Pioneer verwoben war. Das
ist der methodische Kern, kein Nebenbefund.

**Eine offene Frage, kein Beweis:** Mode-1-Flachheit ist konsistent mit
Oszillator-Rauschen **und** mit schwachem-Signal-PLL-Rauschen (Einweg = Sonde
ist Sender; nach dem HGA-Fall schwaches Signal, und PLL-Rauschen ist
signalstärke-, nicht SEP-abhängig — ebenfalls flach). Die Trennung der beiden
ist offen; sie ist per Stärke-Split je Mode machbar (dieselbe Stärkekonstanz-
Technik, die Pioneer zur PLL-/Spur-Trennung nutzte). Das Blatt liest die
Flachheit daher als *„keine Geometrie"*, nicht als *„Oszillator bewiesen"*.

## Galileos ruhiges Fenster — die zweite ~1-Hz-Nummer

Mode 2, SEP 150–180°: **1,5 Hz** (73 Tage); das zugehörige Distanzband 5–6 AU
trägt 96 Tage bei 1,6 Hz. Zum Vergleich: Pioneers Zweiweg-Quiet-Zone trug
~1-Hz-Klasse (sd 1,0 Hz P10 der stillen Tage). **Dieselbe Größenordnung — bei
5 statt 80 AU.** Kein Widerspruch (neuere Hardware, andere Geometrie), aber
die zweite ~1-Hz-Nummer des Korpus auf einer zweiten Mission; der Boden, den
Pioneer so teuer erkämpfte, existiert bei Galileo fast gratis als
Fenster in den 90ern.

Offene Frage (benannt, nicht Ausblick): *Was lässt sich mit einem
1,5-Hz-kohärenten Fenster über ~96 Tagen anstellen?* Drift-Regression ist die
Offensichtlichkeit — aber Galileo war ein Orbiter; die Bahn-Anomalie-Frage ist
eine andere als Pioneers.

## Stations-Nebenfund

Zehn Stationen erscheinen: 43 (6 157 151 Samples, 147 Tage), 14 (3 905 929,
141), 63 (3 614 078, 151) = **97 %**; die 34m-Unterstationen 12/15/24/34/42/
45/61 nur tageweise. Das ist Banden-relevant: die 20-s-Banden-Stationsliste
wird breiter, als Pioneer sie zeigte. Die Stationen 12/15/24/34/42/45/61
gehören in den GWE-Banden-Test-Auftrag als **Erwartungsliste**.

## Grenzen

- Sonnennahe Distanzbänder (0–2 AU) n-leer (4–8 Tage) — keine Trendpunkte,
  `pending` (die lauten Fenster-Pässe fehlen im TDF-Bestand).
- Mode-1-Trennung (Oszillator vs. PLL-Schwachsignal) offen — Stärke-Split.
- Orbiter-Geometrie (Manöver, Okkultations-Segmente) ist nicht segmentiert;
  die 5–6-AU-/SEP-150–180°-Werte mischen Cruise- und Encounter-Fenster.
- Metrik-Vorbehalt: Galileos 1,5 Hz ist Median der Tages-RMS; Pioneers ~1 Hz
  ist sd der stillen Tage — Größenordnungs-Vergleich, keine Äquivalenz.

## Register-Satz

*Das Rezept ist auf der zweiten Mission validiert — mit der Achse, die Pioneer
nicht trennen konnte. Rauschen hat seinen Ort, auf einer zweiten Sonde, und
der Mode-Split trägt die Kontrolle gleich mit: Geometrie im kohärenten Kanal,
Flachheit im inkohärenten.*

## Status

`done`. Rausch-Kurve gezeichnet (2026-09-05), Rezept-Validierung gemessen. Der
Kern ist die SEP-Achse (erstmals von der Distanz getrennt) und der Mode-Split
als Negativ-Kontrolle; die Mode-1-Trennung (Oszillator vs. PLL) bleibt ein
benannter offener Punkt.
