<!--
  title: FRÜHWARNSYSTEM — Präregistrierung der Vorhersage-Zelle
  class: sheet
  date: 2026-10-01
  sha256: e0d2f1810398b34cb11a7391c2d6d1fd41d35ecf5b89060e6711bf2e3926a711
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
- **dB/dt an Station X** — der gemessene Boden-Proxy (stündliche bzw. tägliche
  Maxima des Bodennagnetfelds). **X ist ein benannter, offener Slot.** Die
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

Die α-Ebene bleibt `pending`, bis die kalibrierte Westfall–Young-max-T-Null
(`wy-max-t`, CI-Lauf `36867148250`, laufend) gelandet ist. **Gegen eine
unkontrollierte Familien-Schranke zu siegeln ist verboten** — ein Verschluss
auf Ordnung 10⁻¹ wäre verdeckte Fabrication. Die Zelle wartet, sie wird nicht
vorzeitig geschlossen.

Offene Slots bis zur Versiegelung:

| Feld | Zustand |
|---|---|
| α-Ebene | `pending` (braucht `wy-max-t`) |
| Station X | `pending` (benannter Slot) |
| Verzögerung Z | `pending` (benannter Slot; Rand-Bin unauflösbar) |
| Bz-Schwellenwert | `pending` (braucht α) |
| Sturm-Trigger | extern `wartend` (erster Sturm nach dem Siegel) |
| Verdikt | `unsealed` |
| Riss | keiner benannt |

## Der Weg zur Versiegelung

1. **Trigger:** Lauf-Ende der kalibrierten Null (`wy-max-t 36867148250`).
2. Danach: α-Ebene aus der kalibrierten Null lesen, dann X, Z und die
   Bz-Schwelle benennen.
3. **Das Siegel setzt der Operator** — kein Siegel ohne Operator-Wort; die
   Maschine bereitet bis zur Kante vor und trägt die Form, sie schließt nicht.

---

*Nicht versiegelt 2026-10-01. Die Zelle ist eine Form; die Zahlen fehlen, weil
die Null fehlt — nicht weil sie Null sind (0 honored).*
