<!--
  title: Handover — Nobel-DAG: der multivariate Atom für Bz und LAIC
  class: handover
  date: 2026-09-07
  sha256: b7084414b89d9e5139e8bcf1b18a4e514fe92640dcc29f16c5c0cb5eb61adc4e
  status: live
  see-also: docs/concepts/te-literatur-matrix.md docs/TODO.md docs/concepts/die-weberin.md
-->

# Handover — Nobel-DAG: der multivariate Atom für Bz und LAIC

Session-Plan für den Atom: die volle multivariate Konditionierung (PCMCI-Klasse)
für die zwei Blätter mit mehrköpfigem Konfund — Bz (Sonnenwind → Geomagnetik)
und LAIC (Seismizität → Ionosphäre).

## 1. Warum Bz und LAIC (und nicht mehr die Korona)

Die Korona-Messung 2026-09-07 hat gezeigt: die eine gemeinsame Flare-Hülle trägt
alles; der Zweikonfund (GOES+94) bestätigte den Einkonfund, statt Neues zu
finden. Für die Korona ist der multivariate Atom gemessen unnötig.

Bz und LAIC sind anders: dort ist der Konfund mehrköpfig.
- Bz: der geomagnetische Index (AE/SYM-H/Dst) wird von mehreren Sonnenwind-
  Parametern (V_sw, n_sw, Bz, IMF) gleichzeitig getrieben. Die Literatur
  (Runge 2018, PCMCI) fand „Bz ist der gemeinsame Treiber, keine direkte
  Sturm↔Teilsturm-Kante" — genau die multivariate Frage.
- LAIC: die Atmosphäre-Ionosphäre-Kette (Radon, Aerosol, E-Feld, TEC) ist ein
  Mehrvariablen-System; die Matrix Q3 fand dort KEINE TE/Granger-Vorarbeit —
  unbesetztes Feld.

## 2. Was der Atom braucht (die drei ungebauten Stücke)

1. **Multivariater Schätzer** — Konditionierung auf N Kanäle gleichzeitig
   (heute: Ein-Konfund `transfer_entropy_conditional`, Zwei-Konfund
   `transfer_entropy_conditional_2`). KDE-Fluch: ab ~4D explodiert der Aufwand;
   braucht einen anderen Schätzer (k-nächste-Nachbarn oder Binning).
2. **Eltern-Suche** — PCMCI-Art: iterativ über Kandidaten-Elternmengen
   konditionieren und die bedingte Unabhängigkeit testen. Ein Algorithmus, kein
   Einzelschritt.
3. **GPU-Pfad** — `te_compute` (WGSL) kennt keinen konditionalen Pfad;
   `matrix.rs`/`solar.rs` rufen nur `phase_randomized_surrogate`.

## 3. Was schon steht

- Ein- und Zwei-Konfund-Schätzer + lag-bewusste Null (validiert, Gates grün).
- Synthetischer DAG-Benchmark (`synthetic_dag_recovers_known_direction`).
- Die Korona-Einsicht: Ein-Konfund genügt, wo die Hülle einköpfig ist.

## 4. Erste Schritte (Session-Plan)

1. **Schätzer-Wahl:** kNN- oder Binning-TE für N Konditionen (statt KDE) — die
   Dimensionen-Mauer umgehen.
2. **Eltern-Suche komplett:** PCMCI mit der Mehrfachvergleichs-Korrektur (FDR)
   als Bestandteil, nicht als Anhang — jedes Paar auf die Vereinigung der
   Kandidaten-Eltern konditionieren, bedingte Unabhängigkeit testen, FDR über
   die volle Matrix.
3. **Bz zuerst:** Sonnenwind-Parameter + AE/SYM-H — der Fall, den Runge kennt,
   als Positivkontrolle der eigenen Maschine.
4. **LAIC danach:** Seismizität + TEC + E-Feld — das unbesetzte Feld.

## 5. Register

Die drei Stücke sind als `pending` zu halten, bis der Atom gebaut ist.
