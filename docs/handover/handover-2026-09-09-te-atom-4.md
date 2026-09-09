<!--
  title: Thematisches Handover — TE-Atom 4
  class: handover
  date: 2026-09-09
  sha256: 5a757c10c0e79d1cdd531a596e79e54bfdac42d9ecaf2519a250cd78ddd4ec8b
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-te-atome-blocknull-ksg-pcmci.md docs/auftrag/archiv/auftrag-betriebspunkt-sweep.md docs/concepts/te-literatur-matrix.md
-->
# Thematisches Handover — TE-Atom 4

Stehendes Register der offenen Transfer-Entropie-Linie. Die drei Atome (Null,
Schätzer, Suche) sind geschlossen; der Betriebspunkt steht (max_lag 2,
null_lag 12, bins 4, n_surr 100, Null Block, Schätzer KSG k=4, p_max 2,
α 0.05). Das Offene folgt.

- **n=1000-Shift** — pending. Die n=1000-Zahlen tragen die Adresse (Block-Länge
  n^(1/3)=10 leakt auf n=1000: IDTxl FPR 75,64 %, Tigramite 31,88 %); die
  Shift-Null (ganze Serie rotiert) ist gebaut, verdrahtet, und ihr Zug-5 hält
  jetzt bei T=150 (Atom 4: Shift-Punkt 3/30 Power bei FPR 4,11 %, Gate PASS;
  Zug 5 Shift Binned + Ksg grün). Der n=1000-Shift-Beweis selbst bleibt die
  offene Messung — der T=150-Anker trägt keinen Null-Familien-Umzug.
- **`pcmci_recovers_known_dag` ist in main rot** (te 0,0127 < thr 0,0208):
  TODO-Zeile Nobel-DAG + Handover §5 beschreiben den Fix (Generator
  0.6·a[t−1], ksg/Block/100, 115 s), aber main trägt noch den alten Generator
  (0.5·a_ind[t−1]) am alten Betriebspunkt (Residual/Binned/10, max_lag=1) —
  der Worktree-Fix wurde nie committet. Register-Duty, kein stiller Fix.
- **Bz/LAIC-Wiederholung unter n_surr=100** — Myzel/Desktop (die Residual-OLS-
  Null kostet dort Stunden je Lauf, gemessen). Die Befund-Zahlen (Bz→AE 4,3×,
  Bz→Dst 2,4×) wurden unter n_surr=10 geboren und bleiben, bis gemessen.
- **T=600-Batteriepunkt** — Myzel (PC+MCI×ksg×100 bei T=600 ≈ Stunden je Punkt).
- **Galileo N_SURR 20-vs-10** — das Papier dachte 20, der Baum trägt 10; der
  Ist-Zustand bleibt, bis ein eigener Auftrag die Erhebung prüft (kein stiller
  Shift).
- **cycle_phase_shift_surrogate** — getestete Primitive in `te.rs`; Nutzung
  `pending` bis eine echte Faltung im Layer existiert.
- **Bedingte Multi-Force-TE (Phasenraum)** — `pending`-Instrument; nie
  stillschweigend auslassen (Pflicht vor jedem Blatt).
- **Pflicht vor jedem Blatt** — Mehrfachvergleichskorrektur, Lag-Sweep,
  KDE-h-Sensitivität, Kontrollrichtung des gemeinsamen Treibers, bedingte TE.
- **Nadel Ⅲ Richtung** — TIAW vs Nanoflares offen; Korrelation Kaskaden-Stärke
  × Sonnenstruktur; Richtungs-Nachmessung auf dem vollen 613-Ereignis-Satz.
- **Desktop-Fork (GTX 970)** — der 30-Jahres-Lauf braucht die GPU; ~80–90 min
  gemessen.

Gemessen und geschlossen (bleibt im Befund, nicht hier): Anker 0→3/30 bei FPR
5,19 %, a-set2 9/20, c-Sweep bis 11/20 gegen das eigene Blatt. **Atom 4
(Betriebspunkt-Sweep) geschlossen:** 16 Punkte auf CI, keine Achse schlägt den
OP strikt — der OP bleibt (Block/KSG τ=2 blk=5 ns=100), τ von beiden Seiten
begrenzt (τ=1 powerlos, τ=3/5 fallen durchs Gate); siehe
`docs/befund/befund-betriebspunkt-sweep.md`.
