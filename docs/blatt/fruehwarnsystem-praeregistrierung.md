<!--
  title: FRÜHWARNSYSTEM — Präregistrierung der Vorhersage-Zelle
  class: sheet
  date: 2026-10-01
  sha256: 0a197d6ac81877fdb90728b3622ca6b7ef0d5b1e21a65dab49f169804c686da7
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

Die kalibrierte Westfall–Young-max-T-Null ist **gelandet** (neun Shards à 1111
Replikate, B = 9999, K = 6 distinkte Statistiken, gemeinsame saisonale
Block-Bootstrap-Resample, Seed je Station/Jahr). Der erste α = 0.05-Lauf
(`37187464365` auf `ba9c640479`) trug einen Matrix-Kollaps: die `wy-shards`-Jobs
liefen nur für `sod-2024`, die Abisko-Kombines lasen `null-*.bin reads void — no
measurement`. Der Matrix-Fix (`point` als echter Matrix-Key) trug den Wiederlauf
(`37235150270`, success auf `f266a292`, 2026-10-05); jetzt kombinieren alle drei
Punkte. Der gemessene Stand:

- **α = 0.05** (`wy-max-t`): gepooltes (1−α)-Quantil der studentisierten Maxima =
  **2.4831** (Sodankylä 2024, Familien-Maximum **10.18**), **2.4467** (Abisko
  2024, **12.698**) und **2.4433** (Abisko 2025, **10.864**) — die Familie
  durchlässt an allen drei Punkten, Bz→dB/dt ist das Familien-Maximum. An
  Sodankylä 2024 sind Bz→dB/dt, dB/dt→Bz, dB/dt→Speed, dB/dt→Density
  familien-durchlassend (p_adj 1.0e-4, 1.0e-4, 1.1e-3, 1.9e-3), Speed→dB/dt und
  Density→dB/dt liegen an der Familien-Schranke. n_eff ≈ **44.5** je Member; der
  Lag-0/1-Identitätscheck ist 0.
- **α = 0.01** (`bz-yearly-maxt`, CI-Lauf `37187466569`, success auf `ba9c640479`;
  zehn Shards à 1000 Replikate, B = 10000, m = 6, block = 24 h): gepooltes
  (1−α)-Quantil der studentisierten Maxima = **3.0964** (SOD 2024), **3.0490**
  (ABK 2024) und **3.0036** (ABK 2025), beobachtetes Familien-Maximum =
  **10.181** / **12.698** / **10.864** — die Familie durchlässt an allen drei;
  der Rückkanal dB/dt→Bz liegt tiefer (6.091 / 5.684 / 2.324).

Die α-Ebene ist damit für **alle drei Punkte** bei α = 0.05 und α = 0.01 aus der
kalibrierten Null lesbar. Die zwei Ebenen sind
distinkt und werden nicht gemittelt; welche Ebene das Siegel trägt, benennt das
Operator-Wort. **Gegen eine unkontrollierte
Familien-Schranke (Ordnung 10⁻¹) zu siegeln bleibt verboten.** Die
BCa-Intervallkonstruktion ist benannt `pending` (der joint-stationary Jackknife
ist nicht billig), nie fallengelassen.

Offene Slots bis zur Versiegelung:

| Feld | Zustand |
|---|---|
| α-Ebene | alle drei Station-Jahre **gelesen** (α = 0.05: Quantile 2.4831/2.4467/2.4433; α = 0.01: 3.0964/3.0490/3.0036); welche Ebene siegelt: Operator-Wort |
| Station X | `pending` (benannter Slot; das Papier trägt Abisko und Sodankylä) |
| Verzögerung Z | `pending` (benannter Slot; am 1-min-Korn auflösbar — Rats-Verdikt 2026-10-05: zulässige Präzisierung (b), kein neues α, kein neues Fehlschlag-Kriterium). Deskriptiv gemessen 2026-10-05 (`bz_dbdt_delay_probe`, ABK 2024): Z = 142 min, r = 0.0223, n = 251048 — ohne kalibrierte Minute-Null, nicht gesiegelt |
| Bz-Schwellenwert | `pending` (braucht X und α) |
| Sturm-Trigger | extern `wartend` (erster Sturm nach dem Siegel) |
| Verdikt | `unsealed` |
| Riss | geschlossen: beide Stationen tragen eine kalibrierte Null auf beiden Ebenen (α = 0.05 und α = 0.01) — die Ebenen bleiben getrennt, kein stiller Mittelwert |

## Der Weg zur Versiegelung

1. **Trigger:** Lauf-Ende der kalibrierten Null (`wy-max-t 37187464365` und
   `bz-yearly-maxt 37187466569`) — **eingetroffen** (2026-10-04/05).
2. Danach: α-Ebene aus der kalibrierten Null lesen (alle drei Punkte auf beiden
   Ebenen gelesen), dann X, Z und die Bz-Schwelle benennen.
3. **Das Siegel setzt der Operator** — kein Siegel ohne Operator-Wort; die
   Maschine bereitet bis zur Kante vor und trägt die Form, sie schließt nicht.

---

*Nicht versiegelt 2026-10-01. Die kalibrierte Null liegt für alle drei
Station-Jahre auf beiden Ebenen (α = 0.05 und α = 0.01); X, Z und die Bz-Schwelle
bleiben benannte offene Slots — der Schwellenwert ohne versiegelte α-Ebene wäre
fabriziert (0 honored).*
