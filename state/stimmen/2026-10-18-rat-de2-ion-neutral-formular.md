# Formular — R-2026-10-18-p: DE-2 Ion↔Neutral — coupling oder Abhängigkeit? (2026-10-18)

**Datum:** 2026-10-18 · **Linie:** future (Leitstelle) · **Register:** `R-2026-10-18-p`
**Status:** entschieden — **D** (8/8). Sammlung `2026-10-18-rat-de2-ion-neutral-sammlung.md`.

## Die Frage

Trägt der DE-2-Befund **„ion-neutral coupling"** — oder nur **„Abhängigkeit"**? Und: ist die
**lag-abhängige Umkehr** (lag 1: NVE→IVE stärker; lag 4/16: IVE→NVE stärker) **echt** oder ein
**Vielfach-Test-Artefakt**?

## Der gemessene Befund (Mountain 2026-10-17)

- DE-2 UA_16s (1981, 29 Tage): r(NVE↔IVE) = **+0,256** (Tag +0,358, Nacht +0,185).
- Dichtere Paarung (WATS 2 s + IDM 250 ms, 15 Tage, n = 10 673): roh r = **−0,079**;
  **ost-projiziert** +385 m/s ostwärts, r = **+0,226** — deckungsgleich mit UA.
- TE (Haus-Maschine, lags 1/4/16, 100 phasen-randomisierte Surrogate): **beide Richtungen bei
  jedem Lag signifikant**; Richtung dreht mit dem Lag. **Unverändert** durch die Ost-Projektion.

## Die Wissenschaft — archive_search (2026-10-19)

- **Killeen, Hays, Carignan, Heelis, Hanson, Spencer, Brace (1984), JGR, `10.1029/JA089iA09p07495`**
  — „Ion-neutral coupling in the high-latitude F region": **dieselben DE-2-Instrumente** (IDM +
  RPA + FPI + WATS), **direkte Bestimmung der in-situ Ion-Neutral-Differenzgeschwindigkeit**,
  149 Zitate. Die Kopplung ist dort **eine direkte Messung, keine TE-Ableitung**.
- **Kirkley (2025), arXiv `2506.16215`** — „Transfer entropy for finite data": TE hat bei
  **sparsen Bins** eine **positive Verzerrung** und **keine klare Signifikanz**; schlägt eine
  nichtparametrische Signifikanz ohne Simulation vor.
- **Amornbunchornvej et al. (2020), arXiv `2002.00208`** — **Variable-lag TE**: die Annahme eines
  festen Lags bricht genau beim lag-abhängigen Vorzeichen.

## Der Versuchsplan-Riss (36-Lag-Sweep, gemessen)

| | alte Läufe | dieser Lauf |
|---|---|---|
| Filter | 5 | 6 |
| Schubladen | 1024 | **4096** |
| Zeilen je Schublade (n=23200) | ~23 | **~6** |
| Zeitabstände | 3 | **36** |

Die alten 3/3-Pässe sind **derselbe Schätzer** mit mehr Zeilen je Zelle; bei ~6 Zeilen je
Schublade ist der Null nicht mehr schätzbar, die Schwelle steigt, das Signal bleibt → **0/36**.
**36 Abstände × 2 Richtungen = 72 Tests**, je mit eigener Schwelle, **ohne Vielfach-Test-Korrektur**.
Sauber wird es mit: **gleiche Schubladen, gleiche Abstände, ein Vergleich (mit/ohne Mond), und
eine Schwelle aus dem höchsten Nullwert über alle Abstände** statt aus jedem einzeln.

## Die Optionen

- **A — „coupling" trägt.** Killeen (1984) misst die Differenzgeschwindigkeit direkt; die TE-
  Abhängigkeit ist dieselbe Physik, kein Artefakt.
- **B — nur „Abhängigkeit".** TE misst Abhängigkeit, nicht Reibung; die gerichtete Komponente
  ist erst isoliert mit bedingter TE gegen den gemeinsamen Treiber (Konvektion/Joule).
- **C — erst Methodik fixen.** Der 72-Test-Riss macht jede Aussage über die lag-Umkehr unbelegt;
  Korrektur (Kirkley-nichtparametrisch / höchster Nullwert) zuerst, dann neu messen.
- **D — beide Fragen getrennt.** „coupling" ja (Prior Art), lag-Umkehr offen (C).

## Was der Rat entscheiden soll

1. **A, B, C oder D** — und warum.
2. Wie trägt der Register den Unterschied **Erwerbs-Kanal vs. Kausal-Kanal** (Lumo-Riss)?
3. Ist die lag-Umkehr mit der **höchsten-Nullwert-Schwelle** nachzumessen, bevor sie benannt wird?

## Riss, der bleibt

- Die 36-Lag-Messung ist **nicht methodenfremd** — sie ist derselbe Schätzer am Rand der
  Schätzgrenze; die 3/3-Pässe stehen am selben Rand.
- 72 Tests ohne Korrektur erzeugen **allein durch Zufall** Treffer; auch die 5 rohen „pass" sind
  kein sauberer Beweis.
- Prior Art ist **veröffentlicht** (Coley 1994; Killeen 1984) — die offene Route (CDAWeb, keyless)
  ist der Wert, nicht die Deutung.

## An future

Origin: leitstelle-2026-10-18
- Rat nach Methode: archive_search (erledigt) → Schwarm+UI → paid, mit Stimmen und Axiomen.
