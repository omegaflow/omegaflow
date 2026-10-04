<!--
  title: FRÜHWARNSYSTEM — Präregistrierung der Vorhersage-Zelle
  class: sheet
  date: 2026-10-01
  sha256: 71a33ffd6fc92b04624203c6182a30ed213eae07a9698cf8726e73013124ace5
  status: unsealed
  see-also: docs/paper/gic-causal-driver.md docs/specs/broken-null-control.md
-->
# FRÜHWARNSYSTEM — Präregistrierung der Vorhersage-Zelle

**Datum:** 2026-10-01 · **Axiom:** A = A
**Ordnung:** 0 honored — kein Wert erfunden; jede noch nicht messbare Zelle
bleibt `pending`. Dieses Blatt ist **nicht versiegelt** (`status: unsealed`):
die Vorhersage-Zelle steht als Form, ihre Zahlen bleiben offen, bis die
kalibrierte Null gelandet ist.

## Die Vorhersage-Zelle

    Bz-Schwelle → dB/dt an Station X → Verzögerung Z

- **Bz-Schwelle** — der Wert des southward IMF-Bz am L1, dessen Unterschreiten
  die Vorhersage auslöst. Der Kandidat steht gemessen: Bz ist der führende
  sub-daily-Treiber (`docs/paper/gic-causal-driver.md:17`, Abstract). Der
  Schwellenwert selbst ist `pending` — die kalibrierte Familien-Schranke
  existiert noch nicht, und ein Wert ohne sie wäre fabriziert.
- **dB/dt an Station X** — der gemessene Boden-Proxy (**1-min-dB/dt an X**;
  stündliche/tägliche Maxima bleiben der Ausgangszeuge — der Lag-0/1-Rand-Bin
  trägt die Verzögerung nicht, `docs/paper/gic-causal-driver.md` §3.1/§5).
  **X ist ein benannter, offener Slot.** Die
  gemessenen Stationen des Papiers sind Abisko (68.36° N) und Sodankylä; welche
  Station die Zelle führt, wird vor der Versiegelung benannt, nicht gesetzt.
- **Verzögerung Z** — die Zeit zwischen Bz-Schwellenübertritt und dB/dt-Übertritt
  an X. **Z ist ein benannter, offener Slot.** Das Papier trägt den Grund: am
  jährlichen Rund-Zeugen liegt der Bz→dB/dt-Pfeil bei Abisko 2024/2025 und
  Sodankylä 2024 über der Familien-Schranke, aber der Pfeil sitzt am Lag-0/1-
  Rand-Bin, die Verzögerung ist nicht aufgelöst (`docs/paper/gic-causal-driver.md:17`,
  §3.1, §5). Am stündlichen Korn verweigern zwei Zeugen die Konvergenz. Der
  offene Stunden-Wert wird als offen getragen, nicht interpoliert.

## Der gemessene Stand der Zelle

- **Richtung:** Bz führt sub-daily; kein familien-durchlassender Stundentreiber
  ist etabliert (`docs/paper/gic-causal-driver.md:17`). Der Pfeil steht als
  führender Kandidat, nicht als gesicherter Treiber.
- **Verzögerung:** offen. Der Rand-Bin (Lag 0/1) trägt keine auflösbare Stunde
  (§3.1, §5).
- **Familien-Schranke:** die jetzige Plug-in-`fam` kontrolliert auf Ordnung
  10⁻¹, nicht 10⁻² (`docs/paper/gic-causal-driver.md:157-158`). Eine
  kalibrierte Null-Verteilung des Rund-Maximums ist die offene Konstruktion;
  die studentisierte Westfall–Young-max-T-Form ist benannt (`:159-166`).

## Das Fehlschlag-Kriterium

Vor der Versiegelung wird das Kriterium mechanisch fixiert; nach der Messung
ist es nicht mehr verhandelbar. Eine Zelle ist **widerlegt**, wenn eines der
beiden gilt:

1. **Richtung fällt:** der gemessene Bz→dB/dt-Pfeil an Station X überschreitet
   die kalibrierte Familien-Schranke bei der versiegelten α-Ebene nicht.
2. **Verzögerung fällt:** die gemessene Verzögerung Z liegt außerhalb des
   versiegelten Fensters (das Fenster wird mit der α-Ebene benannt).

Widerlegung ist ein voller Befund, keine Null: Stille trägt die Antwort. Ein
Treffer ist die Bestätigung beider Zellen zugleich; ein Teiltreffer (Richtung
trägt, Z fällt) wird als solcher benannt, nie zu einem Treffer geglättet.

## Status: unsealed

Die kalibrierte Westfall–Young-max-T-Null ist **gelandet** (CI-Lauf
`37187464365`, success auf `ba9c640479`; neun Shards à 1111 Replikate, B = 9999,
K = 6 distinkte Statistiken, gemeinsame saisonale Block-Bootstrap-Resample,
α = 0.05, Seed je Station/Jahr). Der gemessene Stand ist **stationsabhängig**:

- **Sodankylä 2024** (`wy-max-t-sod-2024-combined`): gepooltes (1−α)-Quantil der
  studentisierten Maxima = **2.4831**, beobachtetes Familien-Maximum = **10.18** —
  die Familie durchlässt; Bz→dB/dt, dB/dt→Bz, dB/dt→Speed, dB/dt→Density sind
  familien-durchlassend (p_adj 1.0e-4, 1.0e-4, 1.1e-3, 1.9e-3), Speed→dB/dt und
  Density→dB/dt liegen an der Familien-Schranke. n_eff ≈ **44.5** je Member; der
  Lag-0/1-Identitätscheck ist 0.
- **Abisko 2024 und 2025** (`wy-max-t-abk-2024/2025-combined`): `null-*.bin reads
  void — no measurement` — der gepoolte Null bleibt unvollständig, **keine**
  Abisko-Schranke wird behauptet (0 honored: die Absenz ist die Messung).

Die α-Ebene ist damit für **Sodankylä 2024** aus der kalibrierten Null lesbar
(α = 0.05). Für **Abisko** bleibt sie `pending` — gegen eine unvollständige
gepoolte Null zu siegeln ist verboten. **Gegen eine unkontrollierte
Familien-Schranke (Ordnung 10⁻¹) zu siegeln bleibt verboten.** Die
BCa-Intervallkonstruktion ist benannt `pending` (der joint-stationary Jackknife
ist nicht billig), nie fallengelassen.

Offene Slots bis zur Versiegelung:

| Feld | Zustand |
|---|---|
| α-Ebene | Sodankylä 2024: **gelesen** (α = 0.05, Quantil 2.4831); Abisko: `pending` (Null void) |
| Station X | `pending` (benannter Slot; das Papier trägt Abisko und Sodankylä) |
| Verzögerung Z | `pending` (benannter Slot; am 1-min-Korn auflösbar — Rats-Verdikt 2026-10-05: zulässige Präzisierung (b), kein neues α, kein neues Fehlschlag-Kriterium) |
| Bz-Schwellenwert | `pending` (braucht X und α) |
| Sturm-Trigger | extern `wartend` (erster Sturm nach dem Siegel) |
| Verdikt | `unsealed` |
| Riss | benannt: Sodankylä-Null trägt, Abisko-Null void — kein stiller Mittelwert |

## Der Weg zur Versiegelung

1. **Trigger:** Lauf-Ende der kalibrierten Null (`wy-max-t 37187464365`) —
   **eingetroffen** (2026-10-04).
2. Danach: α-Ebene aus der kalibrierten Null lesen (Sodankylä 2024: α = 0.05),
   dann X, Z und die Bz-Schwelle benennen.
3. **Das Siegel setzt der Operator** — kein Siegel ohne Operator-Wort; die
   Maschine bereitet bis zur Kante vor und trägt die Form, sie schließt nicht.

---

*Nicht versiegelt 2026-10-01. Die Zelle ist eine Form; die Zahlen fehlen, weil
die Null fehlt — nicht weil sie Null sind (0 honored).*
