<!--
  title: Befund — Galileo-Rausch-Kurve: Mode-Split als Kontrolle, Plasma auf der Sichtlinie
  class: befund
  date: 2026-09-05
  sha256: b7d9bde6c4d57b1abfc887c02bc32710c825af7c7d1bd076cd5241c33a9fb9cc
  status: done
  antwortet-auf: docs/auftrag/auftrag-quiet-zone-uebertragung.md
  see-also: docs/befund/befund-galileo-gwe-bestand.md docs/auftrag/auftrag-quiet-zone-vorfilter.md docs/TODO.md
-->

# Befund: Galileo-Rausch-Kurve — Mode-Split als Kontrolle, Plasma auf der Sichtlinie

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
werden nicht als Kurve gezeichnet (die vorab gebundene Warnung, sichtbar
gemacht).

## Der Hauptbefund — Geometrie im kohärenten Kanal, Flachheit im inkohärenten

| Mode | SEP 0–30° (sonnennah) | SEP 150–180° (sonnenfern) | Faktor |
|---|---|---|---|
| 1 | 8,2 Hz (12 Tage) | 7,5 Hz (128 Tage) | flach (1,1×) |
| 2 | 42,0 Hz (18 Tage) | 1,5 Hz (73 Tage) | **28×** |
| 3 | 79,5 Hz (16 Tage) | 6,7 Hz (56 Tage) | **12×** |

Der 28×-Fall (Zweiweg) und der 12×-Fall (Dreiweg) sind **zwei getrennte Daten**,
nicht ein Band: der Unterschied selbst ist ein Hinweis auf den Mechanismus
(kohärenter Rundlauf vs. gesplitteter Downlink). Kohärentes Doppler fällt steil
mit wachsender Sonnenelongation; Einweg bleibt flach. **Zwei Messkanäle,
dieselbe Sonde, dieselbe Epoche, dieselben Stationen — das Plasma zeigt sich
nur in dem Kanal, der es tragen müsste.** Fiele der Trend aus einem Artefakt
(Station/Ära/Reduktion), fiele Mode 1 mit.

**Die Achse, ehrlich benannt:** Plasma sitzt auf der *Sichtlinie*, gesetzt von
Stoßparameter **b = r·sin(SEP)** — nicht von SEP allein und nicht von Distanz
allein. SEP ist ein Proxy, Distanz ist ein Proxy. Die Orbiter-Bahn trägt eine
Kapazität, die Pioneers lineare Bahn nicht trug — mehr wird hier nicht
behauptet. Die **Trennung von Distanz und SEP ist nur teilweise** erreicht:
der sonnennahe Arm (SEP 0–30°) ist Innen-Cruise (0–2 AU), wo beide Achsen noch
verwoben sind; die breite Jupiter-Stichprobe (5–6 AU) erreicht nur das
sonnenferne Regime. „Erstmals getrennt" wäre ein ungemessener Superlativ.

**Zwei offene Fragen, nicht der Befund selbst:**
1. *Mode-1-Flachheit* ist konsistent mit Oszillator-Rauschen **und** mit
   schwachem-Signal-PLL-Rauschen (Einweg = Sonde sendet schwach nach dem
   HGA-Fall; PLL-Rauschen ist stärke-, nicht SEP-abhängig). „Keine Geometrie"
   ist bewiesen; „Oszillator" ist es nicht. Trennung per Stärke-Split je Mode
   (Pioneer-Stärkekonstanz-Technik) — `pending`.
2. *Der Einweg-Nah-Sonne-Arm ist eine Stichprobenfrage, noch keine Kontrolle.*
   Ein geteiltes Einweg-Plasma sonnennah läge bei ~20–30 Hz (Zweiweg 42 Hz ≈
   zwei Beine); ein flaches 8,2 Hz sonnennah ist damit unverträglich, *falls*
   die 12 Mode-1-Tage mit den kohärenten Nah-Sonne-Tagen in Epoche/Geometrie
   überlappen. Überlappen sie → die Flachheit braucht eine Erklärung
   (Lock-/Akquisitions-Selektion am schwachen LGA-Signal). Überlappen sie
   nicht → die Negativ-Kontrolle ist ein Stichproben-Artefakt. Benannt, nicht
   geglättet.

## Galileos ruhiges Fenster — die zweite ~1-Hz-Nummer

Mode 2, SEP 150–180°: **1,5 Hz** (73 Tage); das zugehörige Distanzband 5–6 AU
trägt 96 Tage bei 1,6 Hz. **Metrik exakt:** Median der Tages-RMS. Pioneers
~1-Hz-Klasse war die sd der stillen Tage — die zwei Zahlen sind nicht
äquivalent, nur in derselben Größenordnung (neuere Hardware, andere Geometrie).
Offene Frage (benannt): *Was lässt sich mit einem 1,5-Hz-kohärenten Fenster
über ~96 Tagen anstellen?* Drift-Regression ist die Offensichtlichkeit — aber
Galileo war ein Orbiter; die Bahn-Anomalie-Frage ist eine andere als Pioneers.

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
- Einweg-Nah-Sonne-Arm (8,2 Hz vs. erwartet ~20–30 Hz geteiltes Plasma) —
  Überlappungs-Prüfung gegen die kohärenten Nah-Sonne-Tage steht aus.
- SEP–Zeit–Sonnenzyklus verwoben: 1990–97 reicht vom Sonnenmaximum (früh) ins
  Minimum (spät); die Nah-Sonne-Bins sitzen nahe dem Maximum, die Sonnenferne
  nahe dem Minimum — gegen die Datei-Epochen zu verifizieren.
- Orbiter-Geometrie (Manöver, Okkultations-Segmente) ist nicht segmentiert;
  die 5–6-AU-/SEP-150–180°-Werte mischen Cruise- und Encounter-Fenster.

## Register-Satz

*Das Rezept ist auf der zweiten Mission validiert — das kohärente Rauschen
fällt mit der Sichtlinie, das inkohärente nicht. Die Orbiter-Bahn trägt eine
Kapazität, die Pioneers lineare Bahn nicht trug; mehr ist nicht gemessen.*

## Status

`done`. Rausch-Kurve gezeichnet (2026-09-05), Rezept-Validierung gemessen,
Rat-Verdikt eingearbeitet. Der Kern ist der Mode-Split als Negativ-Kontrolle
(kohärent steil, inkohärent flach) und der Stoßparameter b = r·sin(SEP) als
ehrliche Achse; die Trennung Distanz/SEP ist teilweise, „erstmals" gestrichen.
Drei benannte offene Punkte: Mode-1-Trennung (Oszillator vs. PLL), der
Einweg-Nah-Sonne-Arm, die SEP–Zeit–Sonnenzyklus-Verwebung.
