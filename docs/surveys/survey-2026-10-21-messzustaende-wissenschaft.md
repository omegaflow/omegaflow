<!--
  title: Messzustände in Wissenschaft und Forschung — was null-echt, pending, absent und riss tragen (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: c5504f2a80a93ff8d9e8cea1904c98685f8c6d91a4959414b6dbd2aeb5744504
  status: live
  owner: future
  ack: future 2026-10-21T14:05Z
  next: Rat der 13 — Frage nach Vollständigkeit und Trennschärfe der zwei Achsen
  orphan-after: none
  see-also: docs/concepts/messzustaende.md docs/concepts/glossar.md
-->
# Messzustände in Wissenschaft und Forschung

**Frage.** Trägt das Zustandsmodell (`pending` / `gemessen` mit `null-echt`,
`absent`, `riss`) vor der Wissenschaft? Und was lehren Metrologie,
Erkenntnistheorie, offene Wissenschaft und Astronomie über die Grenzen?

**Grundlage.** Vier belegte Notizen, DOIs gegen Crossref, Zahlen gegen
arXiv/PDF geprüft: `research/messzustaende/notes/2026-10-21-{metrologie,
erkenntnistheorie,open-science,astronomie}.md`.

## 1 · Metrologie — „0 ist ein Wert, kein Zustand"

- **`null-echt`** = gemessene 0 über n (n genannt), deren PDF die 0 einschließt und die eine
  dokumentierte „kompatibel mit 0"-Regel erfüllt (α berichtet). Exakt 0 ist bei
  kontinuierlichen Größen nicht beweisbar; bei Zählgrößen ist „0 Ereignisse"
  exakt (VIM 2.9/2.10/2.11; GUM D.3.5).
- **Unter der Nachweisgrenze** ist **nicht** 0 und **nicht** `absent`: es ist
  „nicht nachgewiesen", berichtet als `< L_D` (Currie 1968: L_C/L_D/L_Q;
  ISO 11929-1:2025). Das ist eine Form von `pending`.
- **`absent`** ist **kein Messergebnis**: VIM 2.3 verlangt eine spezifizierte
  Messgröße; ohne sie gibt es keinen Wert (`None`), nie 0, nie `< L_D`.
  Nichtnachweis stützt nur „kein Signal > L_D im untersuchten Bereich".
- **`riss`** = Inkompatibilität, die eine vollständige Budget-Revision überlebt
  (VIM 2.47/2.48). Geteilte Kalibrierung wirkt als Kovarianz (GUM 5.2.2) und
  erklärt eine Differenz **nicht** — ein gemeinsamer Kettenfehler ist keine
  Erklärung für einen Riss.

## 2 · Erkenntnistheorie — `absent` braucht Reichweite

`absent` ist eine **Allaussage** und nur belegt, wenn vier Bedingungen zusammen
erfüllt sind:

1. **Falsifizierbarkeit** — ein Fund widerlegt sie, Aufzählung beweist sie nie.
2. **Nachweisgrenze** — `absent < L`, nie als Null; fehlt L, ist es `pending`.
3. **Macht** — Nicht-Signifikanz ≠ kein Effekt; ohne Power bleibt `pending`.
4. **Prior** — P(absent | Daten) wird nie 0; ein Bayes-Faktor muss H0 stützen.

**Regel:** `absent` = dokumentierte Reichweite (L, Regime, Instrument) + Beleg;
`pending` = Evidenz fehlt **ohne** Reichweiten-Angabe.

Für den **Riss** gilt: gleiche Definition + gleiches Regime + unabhängige
Wurzeln + Widerspruch überlebt die Kettenreparatur. Sonst ist es ein Vergleich
(Messinvarianz, Simpson) oder ein Kettenfehler (Duhem–Quine).

## 3 · Offene Wissenschaft — die Lücke ist der Normalfall

- „Nicht in der Literatur" ist **kein** Beleg für Nicht-Existenz:
  File-Drawer/Reporting Bias (Rosenthal 1979; Ioannidis 2005; Turner 2008).
- **`pending` hat drei Formen:** nicht gemessen; gemessen-aber-nicht-berichtet;
  berichtet-aber-nicht-zugänglich (FAIR-F ohne A).
- **`absent`** nur bei **positiver** Nicht-Existenz-Messung — z. B. ein vorab
  registrierter Endpunkt, der nachweislich nie erhoben wurde und die Studie ist
  geschlossen. Publikationsbias ist eine Populationsaussage, kein
  Einzelfall-Beweis.
- **MNAR** ist aus beobachteten Daten nicht widerlegbar (Rubin 1976; Sterne
  2009): aus fehlenden Daten folgt **nie** `absent`.
- **`declined`** ist unsere benannte Entscheidung — kein Urteil über fremdes
  Nicht-Berichten; das ist für uns `pending`.

## 4 · Astronomie — Riss ist der Rest nach zwei Filtern

**Filter 1 — Vergleichbarkeit:** Rahmen (ICRS-Realisierung), Epoche
(propagiert), Einheit, Auswahl.
**Filter 2 — Kettenausschluss:** Kalibrierung, Zero-point, Kovarianzen,
Modell, Systematik.

- ICRF3 (Radio, 2015.0) vs. Gaia-CRF3 (optisch, 2016.0): <0,03 mas Verformung,
  0,5 mas Median-Offset → **kein Riss**; Epoche/Auswahl.
- Gaia-Parallaxen-Bias, DR2-Hellrahmen-Spin → **Kette**.
- Hubble 67,4 / 73,04 / 69,8 km/s/Mpc → **Spannung zwischen Ketten**, kein
  Datenriss. „Tension" ist epistemischer Status, kein Datenbefund.
- DE440/441, INPOP, EPM → **Modellvergleich**, kein Riss. DE440 vs. DE441:
  dieselben Daten, anderes Modell (Mondkern-Dämpfung) → Methode.

## Synthese — was das Modell bestätigt und schärft

| Zustand | Wissenschaftliche Bedingung | Hausregel |
|---|---|---|
| `pending` | nicht gemessen; `< L_D`; gemessen-nicht-berichtet; berichtet-nicht-zugänglich | Default; drei Formen |
| `null-echt` | gemessen 0 über n, PDF schließt 0 ein, α berichtet | Qualifier von `gemessen` |
| `absent` | Reichweite L + Power + Prior + positiver Nicht-Existenz-Beleg | selten; nie aus „nichts gefunden" |
| `riss` | Vergleichbarkeit + Kettenausschluss + überlebt Budget-Revision | Qualifier von `gemessen` |
| `declined` | unsere Entscheidung mit Owner/Datum/Grund | Achse 2 |
| `wartend` | äußerer Trigger | Achse 2 |
| `LOCK` | Operator-Wort / Autolock (private/rechtlich) | Achse 2 |

**Was sich ändert:**

1. **`< L_D` wird ein benannter Unterfall von `pending`** — „nicht nachgewiesen",
   nicht 0, nicht absent.
2. **`absent` wird strenger:** Reichweite, Power, Prior — sonst `pending`.
3. **`riss` wird strenger:** zwei Filter, Budget-Revision; „Spannung" ist kein
   Riss.
4. **`declined` bleibt Achse 2** — keine Aussage über die Welt, nur über uns.
5. **`null-echt` braucht die 0-Kompatibilitätsregel** — exakt 0 nur bei Zählern.

## Die Frage an den Rat der 13

> Sind die zwei Achsen — **Messzustand** (`pending` | `gemessen` mit
> `null-echt`/`absent`/`riss`) und **Prozess-Tag** (`offen`/`wartend`/`LOCK`/
> `declined`) — vollständig und trennscharf? Wo bricht das Modell: welcher
> Zustand fehlt, welcher ist überflüssig, welche Grenze ist nicht prüfbar?

Jeder Sitz: **ein** Mechanismus, **ein** Owner, **ein** Artefakt, **eine**
Zahl, **ein** erster Bruch. Keine Aufsätze. Die Synthese geht nach
`rat/entscheidungen.md`.
