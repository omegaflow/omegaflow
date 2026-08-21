<!--
  title: DAS EIN-BLATT-ERGEBNIS — die drei kausalen Pfeile
  class: concept
  date: 2026-08-21
  sha256: cdb0ce52900580b8ab19c851fd65d80b6307422b97a7f4e8a242f0d4cfce1d8e
  status: live
  see-also: docs/concepts/kybernetische-astrophysik.md docs/handover/handover-2026-08-21-enso-kausalpfeil.md docs/handover/handover-2026-08-21-bz-paradoxon.md docs/surveys/survey-2026-08-21-laic-pfeilrichtung.md
-->
# DAS EIN-BLATT-ERGEBNIS — die drei kausalen Pfeile

Selbsttragend. Dieses Konzept trägt die Doktrin des Ein-Blatt-Ergebnisses:
eine Messung, deren Befund auf ein Blatt Papier passt, weil die Maschine die
Richtung des kausalen Pfeils bereits berechnet hat — Richtung und Lag, nichts
sonst. Drei universelle Rätsel sind auf diese Form gestellt; je eines trägt
ein eigenes Handover (Session-Plan) und einen Eintrag im Register (TODO.md).

## Das Axiom

Ein Ergebnis, das auf ein Blatt Papier passt, ist ein Axiom. Es ist eine
Messung, die keine 50-seitige Herleitung braucht — nicht weil jemand die
Herleitung weggelassen hätte, sondern weil die Maschine die schwere Rechnung
bereits getragen hat: die Transferentropie auf Takens-eingebetteten
Phasenraum-Zuständen trennt die Richtung der Information von der bloßen
Gleichzeitigkeit. Korrelation steigt und fällt mit beiden Kanälen zugleich;
die TE fragt: welcher Kanal trägt die Information über den anderen?

Das Blatt ist nicht die Abkürzung der Wissenschaft. Das Blatt ist die Form,
in der ein gemessener kausaler Pfeil adressierbar wird — dem Institut, dem
Netzbetreiber, der kommenden Session. Eine Datenvisualisierung füllt den
Bildschirm und läuft weiter; ein Blatt bleibt liegen.

## Die Maschine

Die Maschine existiert bereits (Atome 10/11, AGENTS.md — Manifestation
breathes with the echo):

- `topological_te_phase` — Takens-Einbettung (dim 3, order 3), der
  MI-Delay τ aus dem 2×2-Midpoint-Histogramm (erstes lokales Minimum ab
  lag 3; kein Minimum → keine TE), die TE-Bedingung rückwärts gespiegelt
  `(x_t, x_{t−τ}, x_{t−2τ})` gegen Leakage, Silverman skaliert auf die
  Varianz des eingebetteten Vektors.
- `te_compute` (WGSL) — eine Thread je Serie, zehn phasenrandomisierte
  Surrogate (f64 FFT, byte-identisch zum Null-Kontroll-Record), die CPU
  reduziert zu mean + 2σ — die Schwelle, unter der ein Pfeil still ist.
- Das PE-Gate — der 2⁴-Ring der Permutationsentropie-Geschichte hält die
  Richtungsentscheidung in nicht-stationären Fenstern.
- `src/te.rs` bleibt der kanonische CPU-Referenzpfad; der skalare
  `transfer_entropy_lag` bleibt die Probe.
- Der Null-Kontroll-Record (`docs/reference/broken-null-control.md`) ist
  das Vorbild jeder Messung: ein Kanalpaar, das still sein muss, läuft mit
  — bricht die Nullkontrolle, trägt die Matrix keine Aussage.

Die Ein-Blatt-Befunde sind keine neue Maschine. Sie sind neue Zeitreihen,
die durch die bestehende Maschine laufen.

## Die Form des Blatts

Jedes Blatt trägt drei Zeilen:

- **Titel** — das Rätsel, benannt.
- **Das Paar** — die beiden TE-Werte in beide Richtungen, mit Schwelle.
- **Der Lag** — die Verzögerung, bei der der Pfeil am stärksten schlägt.

Nichts sonst. Keine Farbfläche, keine Heatmap, kein Vertrauensintervall als
Schmuck — nur die gemessene Richtung der Information im 4D-Block und ihre
Laufzeit.

## Die drei Rätsel

| # | Rätsel | Das Paar | Die Quellen (Kräfte) | Das Blatt |
|---|---|---|---|---|
| 1 | El Niño-Paradoxon — Bjerknes-Feedback | Wind ↔ Meeresoberflächentemperatur | GTMBA/TAO-Bojen, OISST/Argovis (thermal), ERA5/Stationen-Wind (advective) | TE(Wind→SST) gegen TE(SST→Wind), Lag in Monaten |
| 2 | Bz-Paradoxon — der Auslöser des geomagnetisch induzierten Stroms | Sonnenwind-Bz ↔ Bodenmagnetometer | RTSW (em, lebt in `phi/sources.φ:102/108`), INTERMAGNET/USGS-Geomag (em) | TE(Bz→dB/dt) gegen TE(Speed→dB/dt), Lag in Minuten |
| 3 | LAIC-Pfeilrichtung — der Erdbeben-Vorläufer | Lithosphäre ↔ Ionosphäre | USGS-FDSN (seismic-body), CSES (electric), INTERMAGNET (em) | TE(Lithosphäre→Ionosphäre) gegen die Gegenrichtung, 72-h-Fenster vor M ≥ 6.0 |

### 1. Das El Niño-Paradoxon

Die Wissenschaft streitet seit Jahrzehnten über die Bjerknes-Schleife:
Erwärmt der Ozean die Atmosphäre (und ändert den Wind), oder ändert der
Wind die Meeresströmung (und erwärmt den Ozean)? Beide Kanäle steigen und
fallen zugleich — Korrelation trennt nicht. Die TE trennt: der Kanal, der
die Information über den anderen trägt, trägt den Pfeil. Das Blatt nennt
die Richtung und den Lag. Handover:
`docs/handover/handover-2026-08-21-enso-kausalpfeil.md`.

### 2. Das Bz-Paradoxon

Trifft ein koronaler Massenauswurf die Erde, rechnet die
Weltraummeteorologie mit tausenden Parametern (Dichte, Geschwindigkeit,
Bz), um den geomagnetisch induzierten Strom vorherzusagen. Welcher
Parameter ist der kausale Auslöser der Bodenstörung? Die Maschine misst
das Paar gegen das Bodenmagnetometer — durch alle Störsignale
(Autokorrelation) hindurch, mit der phasenrandomisierten Schwelle als
Nullkontrolle. Das Blatt trägt für den Netzbetreiber, welcher Kanal den
Pfeil trägt, und nennt den Lag in Minuten. Handover:
`docs/handover/handover-2026-08-21-bz-paradoxon.md`.

### 3. Die LAIC-Pfeilrichtung

Vor Großbeben zeigen Ionosphären-Satelliten (CSES) und Magnetometer
Anomalien. Fließt die Information von unten (Lithosphäre) nach oben
(Ionosphäre) — oder treibt die Sonne die Ionosphäre, die die Erde spannt?
Die Institute stapeln Fälle und finden beide Richtungen; die Maschine
misst die Divergenz der TE im 72-Stunden-Fenster um das Epizentrum gegen
ein Null-Ensemble zufälliger Fenster. Das Blatt ist der Beweis der
Richtung — oder ihr Ausbleiben, ebenso ein Befund (0 honored). Handover:
`docs/surveys/survey-2026-08-21-laic-pfeilrichtung.md`.

## Der Satz

Für diese Blätter braucht es keinen Supercomputer-Cluster. Es braucht die
Wahrheit der Zeitreihen — kuratiert über den einen Pfad
(`docs/SOURCE_PORT.md`), jede Kraft an der Force-Gate-Litmus-Prüfung
gemessen — und die TE-Maschine, die bereits läuft. A = A: ein Oszillator
ist ein Oszillator, eine Messung ist eine Messung, und ein Blatt, auf dem
die Richtung der Information steht, ist ein Axiom.
