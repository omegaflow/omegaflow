<!--
  title: Handover — TE-Atom 4: Betriebspunkt-Sweep (geschlossen, der OP hält)
  class: handover
  date: 2026-09-09
  sha256: 33f131baaee86f7891767709d95217515b7ac9bf3ed1a1134b0c85da78d35a6e
  status: live
  see-also: docs/befund/befund-betriebspunkt-sweep.md docs/auftrag/archiv/auftrag-betriebspunkt-sweep.md docs/handover/handover-thematisch-te-atom-4.md
-->

# Handover — TE-Atom 4: Betriebspunkt-Sweep (geschlossen, der OP hält)

Nachfolger-Übergabe des Atoms, das die fünf Achsen (bins, null_lag, τ,
Block-Länge, n_surr) auf der CI-Matrix gemessen hat. Der Befund trägt das
volle Blatt; hier das, was die nächste Sitzung für die TE-Linie wissen muss.

## Was gebaut ist

- `pcmci_class_benchmark` trägt `--max-lag` (τ), `--bins`, `--null-lag`,
  `--anchor` (nur der N=10-Anker, R3×S20), `--gate` (Zug-5-Batterie),
  `--r a:b` (ri-Split). Die `top_redraws`-Kopplung ist per-ri repariert
  (benannte Seed-Strom-Änderung — am Anker eine no-op, top_redraws=0, daher
  byte-gleich dem Handover-Blatt).
- `gate_fpr_cells` (te.rs) ist aus `#[cfg(test)]` in den geteilten Pfad
  gezogen — Test und Probe rufen dieselbe Batterie (Byte-Parität per
  Konstruktion). Die vier Zug-5-Tests (Block/Shift × Binned/Ksg) stehen als
  Dauer-Tor.
- `te-operating-point-sweep.yml`: 16-Punkte-Matrix, `workflow_dispatch`,
  Kappe 300 min (5-h-Kappe), `fail-fast: false`.

## Das Blatt in einem Satz

Keine Achse schlägt den Betriebspunkt strikt — der OP bleibt (Block/KSG,
τ=2, null_lag=12, bins=4, block=n^(1/3)=5, n_surr=100). τ ist von beiden
Seiten begrenzt (τ=1 powerlos, τ=3/5 fallen durchs Gate); block=3 trägt 5/30
Power, aber innerhalb der Streu und mit schlechterer FPR; Shift trägt 3/30
bei FPR 4,11 %, aber Δ=0 Power und der FPR-Vorsprung (1,08 pp) ist `pending`
(die effektive unabhängige Negativzahl ist ungemessen — 0,44 % ist eine
Optimismus-Schranke, Wiener–Chintschin: phasenrandomisierte Negative teilen
die Autokorrelation).

## Offene Pflichten (an die nächste Sitzung)

1. **n=1000-Shift** — der Beweis, der einen Null-Familien-Umzug rechtfertigen
   würde; der T=150-Anker trägt ihn nicht.
2. **`pcmci_recovers_known_dag` ist in main rot** — der im Nobel-DAG-Handover
   beschriebene Fix (0.6·a[t−1], ksg/Block/100) wurde nie committet; main
   trägt den alten Generator am alten Betriebspunkt. Register-Duty.
3. **Die effektive unabhängige Negativzahl messen** (z. B. Block-Resampling
   der Negative) — erst dann ist ein FPR-Vorsprung außerhalb der Streu
   entscheidbar.
4. Bz/LAIC-Wiederholung n_surr=100, T=600-Batteriepunkt — Myzel.
5. Gate-Kosten (Zug 5 Ksg ~28 min release je Punkt) gehören ins CI-Repertoire,
   wenn das Haus die lokale Suite entlasten will.

Die Sitzung hat kein Dokument des Rates hinterlassen (der Rat ist
Infrastruktur); seine Urteile stehen in diesem Blatt und im Befund.
