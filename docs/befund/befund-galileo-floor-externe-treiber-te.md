<!--
  title: Befund — Galileo-Mode-1-Boden: externe Treiber (Konjunktion, Empfangs-Ära, Epoche) per TE gegen die Tages-Rauschreihe
  class: befund
  date: 2026-09-05
  sha256: bd64b956e2611822fda9a308835763b606ec614bbd563707971e93f14ce94f8b
  status: draft
  antwortet-auf: docs/befund/befund-aera-treiber-solar.md
-->
# Befund: Galileo-Mode-1-Boden — externe Treiber (Konjunktion, Empfangs-Ära, Epoche) gegen die Tages-Rauschreihe per TE

## Frage & Bindung

befund-aera-treiber-solar (done) mass die Ära-/Monats-Struktur des Galileo-Mode-1-
AGC-Bodens: laute Fenster ~1995-12 und 1996-06..1997-02 (die −2560-Klemmwert-
Population), ruhig ~1995-11; die Struktur folgt qualitativ dem ~13-monatlichen
Sonnenkonjunktions-Zyklus, nicht der Sonnenaktivität (f107 flach im Zyklus-22-
Minimum). Die dortige Bindung war der Mediane je (Monat, Station) — kein TE, und die
Feld-TE konnte externe Treiber nicht prüfen, weil sie nicht in den aufgezeichneten
Feldern stehen.

Dieser Befund TEt die Tages-Bodenreihe gegen drei **recherchierte externe** Treiber
(nicht aus den Feldern, sondern aus den dokumentierten Ereignissen + Geometrie):

- **D1 Konjunktions-Phase** — die Sonnen-Elongation ε (Winkel Sonne–Erde–Sonde, Grad),
  je Tag aus `ephemeris_galileo_daily.bin` + `ephemeris_earth.bin` (ICRS,
  `body_barycenter_position`), am Tagesanfang.
- **D2 Empfangs-Ären-Stufe** — dokumentierte Daten: 0 vor BVR-voll (1995-09-18),
  1 bis DGT/Arraying (1996-05-01), 2 danach.
- **D3 Epochen-/Ära-Datum** — der Monatsindex (Jahr·12+Monat) als Kontroll-Treiber.

Geprüft wird TE(Treiber → Tages-Boden) **unkonditional** und **konditional** auf die
jeweils anderen Treiber, mit den omegaflow-Surrogat-Nullen. Der additive Sonden-Bin
`tools/measure/src/bin/galileo_floor_external_te.rs` (der eine neue Sonden-Pfad;
Repo sonst read-only, kein git) schreibt die Roh-Tabelle nach
`/tmp/opencode/galileo_floor_external_te.txt`.

TE-Konvention: `transfer_entropy(target = Tageswert, source = Treiber)` misst die
treiber-gerichtete Kopplung; Rückrichtung = reverse TE. Nullen: phasen-randomisiert
(10) und Block-Bootstrap Block=5 (10); konditional Residuen-Surrogate (20).
* = TE über der Null. Fenster 1994-12-01..1997-02-28, Stationen 14/43/63, Mode 1,
|resid| ≤ 1000 Hz (lock ausgeschlossen); Tageszelle ≥ 30 Samples; Reihe ≥ 30 Tage;
Lags 1/3/5/7 Tage.

## n zuerst (gemessen)

Mode-1-Samples 9 743 574; lock 1 568 246; Boden-Population (−2560) 2 088 475;
stark (≥ −1750) 3 624 697 (Stationen 14/43/63, Fenster). 153 verschiedene Tage mit
Boden-/stark-Zellen, 1994-12-16..1997-02-28.

Tageszellen-Monate der Boden-Population (Samples über alle drei Stationen):
1995-11 (135 526), 1995-12 (106 981), 1996-01 (34), 1996-06 (177 573), 1996-09
(70 110), 1996-11 (494 387), 1996-12 (471 463), 1997-01 (11 864), 1997-02
(620 537). Die starke Population beginnt 1994-12-16 (381 413 im Dez 1994) und läuft
durch. Der AGC-Boden (−2560) trägt im Fenster erst ab 1995-11 Tageszellen.

Netz-Bodenreihe n = 194 Tage (1995-11-23..1997-02-28, Lücke max 185 Tage);
je Station n = 62–68 (Lücke max ~190 Tage). Netz-stark n = 246 (1994-12-16..,
Lücke max 331 Tage). D1-Elongation spannt 0,36°..173,38° (Konjunktions-Senke in der
Reihe), die Boden-Reihe deckt nur die Stufen 1..2 von D2 (keine Vor-BVR-Tage im
Boden — die Stufe 0 liegt vor dem ersten Boden-Tag), die starke Reihe 0..2.

## Boden-Population (−2560): Ergebnis

Unkonditional, Netz (n = 194), fwd TE gegen Phasen-/Block-Null:

| Treiber | lag 1 | lag 3 | lag 5 | lag 7 | Urteil |
|---|---|---|---|---|---|
| D1 elong | 0,0317 | 0,0259 | 0,0320 | 0,0325 | unter beiden Nullen (thrPh 0,0435–0,0573) |
| D2 Stufe | 0,0131 | 0,0122 | 0,0135 | 0,0139 | unter beiden Nullen |
| D3 Epoche | 0,0252 | 0,0245 | 0,0287 | 0,0288 | unter beiden Nullen (thrBl 0,0317–0,0490) |

Kein Treiber übersteigt die Surrogat-Null. Rückrichtung klein (rev TE ≤ 0,027).
Je Station (n = 62–68) nur zwei vereinzelte Grenz-Überschreitungen gegen die
Phasen-Null (st14 D3 lag7 0,1045 > 0,1032; st63 D3 lag3 0,0642 > 0,0587) — bei
96 Vergleichen Zufalls-Niveau, ohne Lag-/Station-/Treiber-Konsistenz.

Konditional, Netz (n = 194), cTE gegen cThr (Residuen-Surrogate, 20), Lags 1/3/5/7:

| Kopplung | cTE (lag1/3/5/7) | cThr (lag1/3/5/7) | Urteil |
|---|---|---|---|
| D1\|D3 Epoche | 0,0219 / 0,0164 / 0,0222 / 0,0236 | 0,0512 / 0,0446 / 0,0597 / 0,0648 | unter Null |
| D1\|D2 Stufe | 0,0308 / 0,0236 / 0,0300 / 0,0302 | 0,0511 / 0,0419 / 0,0545 / 0,0586 | unter Null |
| D2\|D3 Epoche | 4·10⁻⁵ / 4·10⁻⁵ / 6·10⁻⁵ / 6·10⁻⁵ | 0,0297–0,0434 | ≈ 0 |
| D2\|D1 elong | 0,0123 / 0,0098 / 0,0116 / 0,0115 | 0,0389–0,0459 | unter Null |
| D3\|D1 elong | 0,0154 / 0,0150 / 0,0189 / 0,0199 | 0,0393–0,0473 | unter Null |
| D3\|D2 Stufe | 0,0121 / 0,0124 / 0,0153 / 0,0150 | 0,0257–0,0370 | unter Null |

Urteil Boden: **Null über der Surrogat-Null für alle drei Treiber, unkonditional und
konditional.** Die Tagesreihe des AGC-Bodens (n = 194 gepoolt mit 185-Tage-Lücke,
n = 62–68 je Station) ist die gemessene Grenze: Sie kann weder die
Konjunktions-Phase noch die Empfangs-Ären-Stufe noch den Epochen-Index als
gerichtete Tages-Kopplung über der Null tragen. Die frühere Monats-Median-
Assoziation an den Konjunktions-Zyklus bleibt die aufgezeichnete Messung; die
gerichtete Tages-Trennung „Konjunktions-Zyklus gegen Epochen-Datum" ist auf diesen
dünnen Fenstern **nicht messbar**.

## Starke Population (≥ −1750): Ergebnis

Unkonditional, Netz (n = 246), fwd TE:

| Treiber | lag 1 | lag 3 | lag 5 | lag 7 | Urteil |
|---|---|---|---|---|---|
| D1 elong | 0,0186 | 0,0229 | 0,0283 | 0,0294 | unter Phasen-Null (0,0258–0,0335) an allen Lags; über Block-Null an 3/4 |
| D2 Stufe | 0,0464 | 0,0420 | 0,0409 | 0,0428 | über Block-Null (0,0187–0,0288) an allen; unter Phasen-Null (0,0554–0,0647) |
| D3 Epoche | 0,0465 | 0,0425 | 0,0425 | 0,0439 | **über beiden Nullen an lag 1/3/5** (thrPh 0,0362–0,0387) |

Nur D3 (Epochen-Index) räumt beide Nullen — die spektrum-erhaltende Phasen-Null ist
für glatte langsame Reihen die strenge; D1 und D2 räumen nur die kurzsichtige
Block-Null. Rückrichtung klein (rev TE ≤ 0,014) → gerichtet.

Konditional, Netz (n = 246):

| Kopplung | cTE (lag1/3/5/7) | cThr (lag1/3/5/7) | Urteil |
|---|---|---|---|
| D1\|D3 Epoche | 0,0185 / 0,0245 / 0,0315 / 0,0350 | 0,0113–0,0198 | **über Null an allen Lags** |
| D1\|D2 Stufe | 0,0185 / 0,0246 / 0,0320 / 0,0346 | 0,0116–0,0197 | **über Null an allen Lags** |
| D2\|D3 Epoche | 5·10⁻⁴ / 5·10⁻⁴ / 2·10⁻⁴ / 5·10⁻⁴ | 0,0029–0,0078 | ≈ 0 |
| D2\|D1 elong | 0,0462 / 0,0438 / 0,0446 / 0,0480 | 0,0107–0,0126 | **über Null an allen Lags** |
| D3\|D1 elong | 0,0464 / 0,0441 / 0,0456 / 0,0495 | 0,0109–0,0133 | **über Null an allen Lags** |
| D3\|D2 Stufe | 0,0006 / 0,0009 / 0,0018 / 0,0016 | 0,0015–0,0071 | ≈ 0 (lag 7 0,0016 marginal) |

Die D2/D3-Seite ist eine Achse: Stufe und Monatsindex sind im Fenster nahezu
deterministisch verklammert — konditioniert man die eine auf die andere, fällt die
Kopplung auf ≈ 0. D1 (Elongation) überlebt die Konditionierung auf Epoche **und**
auf Stufe; D3 überlebt die Konditionierung auf D1. Beide Achsen sind also als
gerichtete Kopplungen in die starke Tagesreihe gemessen und gegenseitig nicht auf
eine reduzierbar.

## Urteile (a) (b) (c)

(a) Koppelt die Konjunktions-Phase (D1) über die Epoche (D3) hinaus in den Boden?
**Boden-Population: nein — D1\|D3 liegt an allen Lags unter der Null.**
Starke Population: ja, aber die Gegenrichtung D3\|D1 bleibt ebenfalls über der Null
(verstrickt, siehe (c)).

(b) Trägt die Empfangs-/DGT-Ären-Stufe (D2) über Konjunktion/Epoche hinaus etwas bei?
**Nein, in beiden Populationen.** D2\|D3 ≈ 0 (Boden ~5·10⁻⁵, stark ~3–5·10⁻⁴): die
Stufe ist im Monatsindex vollständig enthalten. Im Boden liegt auch D2\|D1 unter der
Null.

(c) Wird die Boden-Ära-Struktur vom Konjunktions-Zyklus (D1) oder vom Epochen-Datum
(D3) getrieben? **Die Boden-Tagesreihe kann das nicht trennen: kein Treiber
übersteigt die Null.** Die Daten können die Trennung nicht leisten — benannt: die
dünnen, zerrissenen Boden-Fenster (zwei Cluster mit ~5-Monats-Lücke, D2 nur Stufe
1..2 im Boden). Für die starke Population messen beide Achsen (Epoche und
Elongation) als gegenseitig nicht reduzierbare gerichtete Kopplungen; die grobe
Monatsindex-Konditionierung (Residuen-Null entfernt nur einen linearen
Epochen-Trend) kann die gemeinsame glatte langsame Struktur nicht vollständig
auseinanderlegen — auch dort bleibt die Trennung „Konjunktions-Zyklus gegen
Epochen-Datum" verhakt.

## Gemessene Antwort

Die Ära-/Monats-Boden-Struktur (AGC-Fläche −2560), deren laute Fenster der
Befund-aera-treiber-solar an den ~13-Monats-Konjunktions-Zyklus anschloss, trägt
**keine messbare gerichtete Tages-Kopplung** von Konjunktions-Phase, Empfangs-Ären-
Stufe oder Epochen-Index über die Surrogat-Null — weder unkonditional noch
konditional. 0 geehrt: Die Konjunktions-Zyklus-Attribution bleibt die gemessene
Monats-Median-Assoziation der Vor-Blätter; die gerichtete Tages-Trennung gegen das
Epochen-Datum ist auf den vorhandenen Boden-Fenstern nicht messbar (nicht „kein
Effekt", sondern die Messung trägt ihn nicht). In der starken Population ist die
Epochen-/Empfangs-Achse eine echte gerichtete Kopplung (D3 über beide Nullen), und
die Elongation addiert eine zweite, nicht auf Epoche reduzierbare Komponente; die
Zerlegung „Konjunktions-Zyklus gegen bloßes Datum" bleibt dort verhakt.

## Grenzen

- Boden-Reihe n = 194 gepoolt (62–68 je Station) mit Lücken bis 185/190 Tagen; die
  Lags laufen in anwesenden Tagen, nicht Kalendertagen. Die Phasen-Null ist für
  glatte langsame Tagesreihen konservativ hoch.
- D2 hat im Boden-Fenster nur die Stufen 1→2 (1996-05); die BVR-Stufe 0 liegt vor
  dem ersten Boden-Tag — die Empfangs-Voll-Inbetriebnahme 1995-09 ist für die
  Boden-Population nicht prüfbar.
- Konditionale Null = Residuen-Surrogat (OLS-Entfernung eines linearen
  Epochen-Trends aus dem Ziel); für den groben, stufenförmigen Monatsindex ist das
  eine schwache Entfernung — die D1/D3-Nichtreduzierbarkeit der starken Population
  steht unter dieser Grenze.
- D1 (Elongation) und D3 (Monatsindex) sind geometrisch verstrickt: jeder Monat
  trägt eine weite Elongations-Spanne, die Konjunktions-Senken wiederholen sich im
  ~13-Monats-Takt. Welchen Anteil die Konditionierung nicht trennen kann, ist
  benannt, nicht geglättet.

## Register-Satz

*Die AGC-Boden-Tagesreihe (−2560) trägt keine messbare gerichtete Kopplung der
externen Treiber — Konjunktions-Phase (D1), Empfangs-Ären-Stufe (D2), Epochen-Index
(D3) bleiben im Boden unkonditional wie konditional unter der Surrogat-Null (n = 194
gepoolt, 62–68 je Station, Lücken bis 185 Tage): die Monats-Median-Assoziation an
den Konjunktions-Zyklus bleibt die gemessene Aussage, die gerichtete Tages-Trennung
gegen das Epochen-Datum ist auf den Boden-Fenstern nicht messbar. In der starken
Population ist die Epochen-/Empfangs-Achse eine echte gerichtete Kopplung (D3 über
Phasen- und Block-Null, 3/4 Lags), die DGT-Stufe trägt nichts über den Monatsindex
hinaus (D2\|D3 ≈ 0), und die Elongation addiert eine zweite, nicht auf Epoche
reduzierbare Komponente; die Zerlegung Konjunktions-Zyklus gegen Epochen-Datum
bleibt dort verhakt.*

## Status

`draft` (2026-09-05). Zahlen aus dem additiven Sonden-Bin
`galileo_floor_external_te.rs` (cargo build 0 Warnungen, Lauf reproduziert, Tabelle
`/tmp/opencode/galileo_floor_external_te.txt`).
