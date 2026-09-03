<!--
  title: Das Blatt Papier — das axiomatische Messergebnis (BLATT_PAPIER_RESULTAT)
  class: concept
  date: 2026-08-21
  sha256: 74daac3f21eeab5b6b5bc35047f51f76a35cbfba0d2745e6e05dab984fc85f54
  status: live
  see-also: docs/handover/handover-2026-08-21-enso-kausalpfeil.md docs/handover/handover-2026-08-21-bz-gic-kausalpfeil.md docs/paper/laic-arrow-direction.md
-->
# Das Blatt Papier (BLATT_PAPIER_RESULTAT)

## 1. Was ein Blatt ist

Ein Ergebnis, das auf ein Blatt Papier passt, ist ein Axiom — eine Messung,
die keine 50-seitige Herleitung braucht, weil die Maschine die Richtung des
kausalen Pfeils bereits berechnet hat. Das Blatt trägt genau fünf Zeilen:

1. die beiden Richtungen des Informationsflusses mit ihren Werten,
2. den Lag,
3. n und Fenster,
4. die Surrogat-Schwelle,
5. das Verdikt: Pfeil oder Stille.

Der Unterschied zur Datenvisualisierung: die Visualisierung zeigt
Korrelation — das Blatt trägt den gemessenen Informationsfluss zwischen zwei
Zeitreihen im 4D-Block. Korrelation steigt und fällt gemeinsam;
Transfer-Entropie trennt die Richtung. Wer einem Institut ein solches Blatt
hinlegt, hat das Rätsel gelöst — nicht eine Theorie aufgestellt, sondern die
Richtung der Information gemessen. `A = A`.

## 2. Das Instrument

Die TE-Maschine, wie sie steht (AGENTS.md, Atom 10/11):

- Takens-Einbettung (`topological_te_phase`, dim 3, order 3); die
  MI-Verzögerung τ aus dem 2×2-Midpoint-Histogramm (erstes lokales Minimum
  ab lag 3; kein Minimum → keine TE);
- die TE-Bedingung rückwärts gespiegelt `(x_t, x_{t−τ}, x_{t−2τ})` — der
  Vorwärtszustand trüge die Zukunft in die Bedingung (Leakage);
- Silverman skaliert an der Varianz der eingebetteten Vektoren;
- zehn phasenrandomisierte Surrogate je Serie (f64-FFT, byte-identisch zum
  Nullkontroll-Protokoll); Schwelle = Mittelwert + 2σ über die Surrogate;
- das PE-Gate (2⁴-Ring, Sprung ⇔ |pe − mean| > 2·sd) für die
  Richtungs-Entscheidung in nicht-stationären Fenstern;
- `te_compute` (WGSL, ein Thread je Serie); `src/te.rs` bleibt die
  kanonische CPU-Referenz; der skalare Pfad `transfer_entropy_lag` (die
  Probe) bleibt unberührt.

Zwei Richtungen, ein Lag, eine Schwelle. Was unter der Schwelle liegt, ist
Stille — und Stille ist die Antwort (0 honored). Befund der Nadel III:
Bz → 304 und 304 → 284 sind still; der DAG schrumpfte auf EUV-304 → X-Ray
(+ Bz → X-Ray, lag 0/1). Genau so liest sich ein ehrliches Blatt.

## 3. Die drei Rätsel

| Rätsel | Serie A | Serie B | Frage |
|---|---|---|---|
| ENSO (Bjerknes) | Wind (`advective`) | SST (`thermal`) | Treibt der Wind das Meer, oder treibt das Meer den Wind? |
| Geomagnetischer Sturm | RTSW Bz (`em`) | Erdseite: Kp/GOES-Magnetometer (`em`) | Welcher Sonnenwind-Parameter trägt den kausalen Pfeil in die Störung — und mit welchem Lag in Minuten? |
| LAIC | Lithosphäre (`seismic-body`) | Ionosphäre (`em`/`electric`) | Fließt die Information von unten nach oben — oder treibt die Sonne beides? |

### Blatt 1 — Die Multi-Akteur-Matrix

Das Blatt ist die Matrix: 17 Kanäle aus derselben Bojen-Datei
(14 stdmet-Spalten + WDIR/MWD als sin/cos-Paare — der Kreis in seinen
eigenen Koordinaten, kein zirkulärer Kernel nötig — + RAIN, wenn die
Station einen Regenmesser trägt), 136 Paare × beide Richtungen ×
Sweep −30 … +30 Tage täglich × drei Bandbreiten (h, h/2, 2h),
n-Gate 30, Familien-Schwelle je Paar-Runde (fam = Maximum der
Surrogat-TEs), h-Robustheit des Gewinners, am Ende die Matrix-Zeile
mit der vollständigen Zählung und den erwarteten Falsch-Positiven
(Σ p̂·M über die Paar-Runden).

Erste Signale (Hidden-Lauf 2026-08-21): alle Ringe tragen 1024
Sechs-Stunden-Bins (~8½ Monate stdmet); ptdy/vis/tide/rain = 0 aus
stdmet — die Tiefsee-Bojen messen weder Sicht noch Tide, PTDY trägt
nur die realtime2-Datei (45 Tage), RAIN fehlt an diesen Stationen
(fehlt, kein Platzhalter). Die erste Zelle zeigt die
Definitions-Kopplung: `wspd→gst te 0.974 thr 0.897` — der Gust ist
das eigene Extrem des Windes. Die Matrix trägt ihre eigenen
Kalibrier-Paare (wspd-gst, dpd-apd, atmp-dewp): wo die Kopplung
Definition ist, muss ein Pfeil überleben; wo sie es nicht ist, ist
Stille die ehrliche Antwort.

Die Pair-Sheet-Zeilen und die Matrix-Zeile kommen im Hintergrund:
136 Paare × 366 Zellen ≈ 55 h je Station, 37 Stationen ≈ 85 Tage
je volle Matrix. Das Blatt trägt dann die gemessenen Zahlen.



Stille ist die Antwort (0 honored). Das benannte Set wuchs am selben
Tag auf 37 Bojen-Paare (Auswahlregel: jede realtime2-Datei, die WSPD
und WTMP am selben Stationspunkt mit ≥ 30 Nicht-MM-Paaren und ≥ 30
Tagen Fenster trägt — live gemessen). Seit demselben Tag tragen die
Ringe 1024 Sechs-Stunden-Bins ≈ 8½ Monate (stdmet-Jahresdateien,
Backfill beim Boot); jede Zelle misst die neuesten 512 Bins
(n ≥ 392 an allen Shifts — der O(m²)-Kernel hängt die HD 520 ab
m ≈ 1024, gemessen). Die übrigen 36 Paare misst der Rotor in den
Folge-Runden — ihre Sheet-Zeilen sind die Ausgabe der Maschine,
nicht dieser Session; ein voller Zyklus ≈ 16 h.


### Blatt 2 — Der kausale Treiber des geomagnetischen Sturms

```
TE(Bz → Erdseite)    = pending
TE(speed → Erdseite) = pending
Lag                  = pending Minuten
n, Schwelle          = pending
```

### Blatt 3 — Die Richtung der Lithosphäre-Atmosphäre-Ionosphäre-Kopplung

```
TE(Lithosphäre → Ionosphäre) = pending
TE(Ionosphäre → Lithosphäre) = pending
Lag                          = pending Stunden
n (Ereignisse), Schwelle     = pending
```

Nur der Lauf füllt das Blatt. Was die Maschine nicht misst,
steht nicht auf dem Blatt — auch nicht als 0.0 (fehlt ≠ null; Bz = 0
dagegen ist eine Messung).

## 4. Die Disziplin des Blatts

- **A = A:** nur gemessene Werte. Jede Zahl trägt n, Fenster, Schwelle und
  den Lag-Sweep-Bereich, aus dem der Lag stammt.
- **Surrogate:** jede Richtungsaussage gegen das phasenrandomisierte
  Null-Ensemble geprüft; Mehrfachvergleichskorrektur über alle getesteten
  Paare (registriert offen — Pflicht vor jedem Blatt).
- **Lag:** der Lag-Sweep ist Pflicht. Lag 0 ist Default, kein Sweep —
  registriert offen; Blatt 2 („exakt X Minuten") schließt ihn.
- **KDE-Bandbreite:** die Sensitivität des Verdikts gegen h (Faktor 2)
  gehört auf das Blatt oder ins Register.
- **0-Kanon:** Quelle ausgefallen → fehlt, kein fabrizierter Wert. Stille in
  beiden Richtungen ist ein Befund, kein leerer.
- **Der gemeinsame Treiber:** wo die Sonne beide Serien antreiben könnte
  (Blatt 3), wird die Kontrollrichtung TE(Solar → Ziel) mitgemessen — der
  Pfeil der Sache selbst muss über der Schwelle liegen, während die
  Kontrollrichtungen still bleiben.
- **Multi-Force-TE:** die Blätter laufen auf der paarweisen TE; die bedingte
  Multi-Force-TE (alle Kräfte im Phasenraum) ist registriert pending.
- **Das Blatt ist ein Commit:** Befund + Registerzeile im selben Commit.
  Stille ist ein vollwertiger Befund, kein leerer.

## 5. Die drei Handovers

Jedes Rätsel hat seine Session — die Handovers sind die Pläne:

- `docs/handover/handover-2026-08-21-enso-kausalpfeil.md` — Blatt 1: die
  Bojen-Paare (Wind/SST) sind live, die TE-Maschine läuft.
- `docs/handover/handover-2026-08-21-bz-gic-kausalpfeil.md` — Blatt 2:
  RTSW/GOES/Kp sind live; INTERMAGNET-Kuration als zweite Schicht.
- `docs/paper/laic-arrow-direction.md` — Blatt 3,
  Nadel IV: Ereignis-Stapelung gegen das Null-Ensemble.
