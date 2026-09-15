<!--
  title: Das Blatt Papier — das axiomatische Messergebnis (BLATT_PAPIER_RESULTAT)
  class: concept
  date: 2026-08-21
  sha256: 438769592ef7832082ae5f7a28e581e21f13921eba68ae12ad06a8bdeb2a6fed
  status: live
  see-also: docs/paper/laic-arrow-direction.md
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
TE(Bz → dB/dt)     = 3.10e-1  | Pfeil  (Schwelle 2.26e-1, mean+2σ)
TE(speed → dB/dt)  = 8.20e-2  | still  (Schwelle 2.24e-1)
Lag                = 119 min (Bz) / 60 min (speed) — Sweep 0–120 min
n, Schwelle        = n 1301 (Bz) / 1254 (speed), 1-min-Grid
```

Gemessen 2026-09-15 (`bz_blatt_probe`, live, 22-h-Fenster, ABK 68,36° N):
Bz trägt den Pfeil, Speed bleibt still; die Null-Kontrolle Density→dB/dt
= 2.85e-1 liegt ebenfalls über ihrer Schwelle (Pfeil) — die Familien-Schwelle
(fam = max Surrogat-TE der Runde, max-T) muss das Verdikt schließen und steht
als CI-Lauf offen; die retro OMNI2-PCMCI-Zeile (FDR) ist CI-gebunden.

### Blatt 3 — Die Richtung der Lithosphäre-Atmosphäre-Ionosphäre-Kopplung

```
TE(Lithosphäre → Ionosphäre) = still — 71/1400 Fenster (5,1 %), mean excess −4.6e-2
TE(Ionosphäre → Lithosphäre) = still — 101/1400 Fenster (7,2 %), mean excess −2.2e-2
Lag                          = 1–2 h (PCMCI max_lag 2, Stundenzellen)
n (Ereignisse), Schwelle     = 1400 Fenster; Schwelle mean+2σ, FDR (Benjamini-Hochberg)
```

Gemessen 2026-09-15 (`nobel_probe_laic`, laic.bin, PCMCI + common-cause über
1400 Ereignis-Fenster): beide Richtungen liegen am Boden, keine dominante
Lithosphäre↔Ionosphäre-Richtung; die Solar-Kontrolle Bz→F (285/1400, 20,4 %)
liegt über dem Boden — der gemeinsame Treiber trägt den einzigen gemessenen
Pfeil, nicht die Lithosphäre.

Nur der Lauf füllt das Blatt. Was die Maschine nicht misst,
steht nicht auf dem Blatt — auch nicht als 0.0 (fehlt ≠ null; Bz = 0
dagegen ist eine Messung).

## 4. Die Disziplin des Blatts

- **A = A:** nur gemessene Werte. Jede Zahl trägt n, Fenster, Schwelle und
  den Lag-Sweep-Bereich, aus dem der Lag stammt.
- **Surrogate:** jede Richtungsaussage gegen das phasenrandomisierte
  Null-Ensemble geprüft; die Mehrfachvergleichskorrektur läuft — Blatt 3 trägt
  FDR + common-cause (PCMCI), Blatt 2 die paarweise Surrogat-Schwelle
  (mean+2σ); die Familien-Schwelle (fam/max-T) und die retro
  OMNI2-PCMCI-Zeile stehen als CI-Lauf offen.
- **Lag:** der Lag-Sweep ist Pflicht. Blatt 2 trug den Sweep 0–120 min
  (Lag 119 min Bz / 60 min speed), Blatt 3 1–2 h — der Sweep steht, „Lag 0
  als Default" ist geschlossen.
- **KDE-Bandbreite:** die Sensitivität des Verdikts gegen h (Faktor 2)
  gehört auf das Blatt oder ins Register; gemessen ist sie noch nicht —
  `laic_probe --analyze --kde-scale` trägt den Knopf, die lokale laic-Ernte
  fehlt, also CI.
- **0-Kanon:** Quelle ausgefallen → fehlt, kein fabrizierter Wert. Stille in
  beiden Richtungen ist ein Befund, kein leerer.
- **Der gemeinsame Treiber:** wo die Sonne beide Serien antreiben könnte
  (Blatt 3), wird die Kontrollrichtung TE(Solar → Ziel) mitgemessen — der
  Pfeil der Sache selbst muss über der Schwelle liegen, während die
  Kontrollrichtungen still bleiben.
- **Multi-Force-TE:** die Blätter laufen auf der paarweisen TE; die bedingte
  Multi-Force-TE (alle Kräfte im Phasenraum) ist gemessen
  (`multi_force_te_probe`): 216 konditionale Zellen, 1 FDR-Pass
  (seismic-body→seismic-surface, p 9.5e-7), die drei gepflanzten Pfeile
  werden bei n=512 nicht rekonstruiert — der Surrogat-Boden schluckt die
  schwache Kopplung.
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
