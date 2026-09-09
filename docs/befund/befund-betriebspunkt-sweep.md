<!--
  title: Befund — Atom 4: Betriebspunkt-Sweep (bins, null_lag, τ, Block-Länge, n_surr) — der OP hält, keine Achse schlägt ihn strikt
  class: befund
  date: 2026-09-09
  sha256: e88a1d44003d19a7e8e57a304cf6d684ace04cf2c182044a8c66d5b556addcd7
  status: done
  antwortet-auf: docs/auftrag/archiv/auftrag-betriebspunkt-sweep.md
  see-also: docs/handover/handover-2026-09-09-te-atom-4.md docs/handover/archiv/handover-2026-09-09-te-atome-blocknull-ksg-pcmci.md docs/befund/befund-klassen-benchmark-pcmci.md
-->

# Befund — Atom 4: Betriebspunkt-Sweep (bins, null_lag, τ, Block-Länge, n_surr)

## Das Blatt (16 Punkte, CI-Matrix, sheet-grade — Commit dbbaa0c)

Anker N=10/T=150/c=0.287 (S60 linear), R3×S20, Zug-5-Gate-Batterie je Punkt.
`links>70%` = Links über 70 % Power von 30; Anker-FPR über neg=10200
(60 Realisierungen × 170 Falschkanten-Slots); Gate-Kriterium ≤ 8 % je Zelle
+ Anstieg ≤ 2 pp.

| Punkt | Achse / Betriebspunkt | links>70% | Anker-FPR | Gate |
|---|---|---|---|---|
| 01-op | **Block/KSG τ=2 blk=n^(1/3)=5 ns=100** | 3/30 | 5,19 % | PASS |
| 02-tau-1 | τ=1 | 0/30 | 4,79 % | PASS |
| 03-tau-3 | τ=3 | 3/30 | 6,76 % | FAIL |
| 04-tau-5 | τ=5 | 5/30 | 8,12 % | FAIL |
| 05-block-3 | Block-Länge 3 | 5/30 | 7,03 % | PASS |
| 06-block-8 | Block-Länge 8 | 3/30 | 4,78 % | PASS |
| 07-block-12 | Block-Länge 12 | 2/30 | 4,48 % | PASS |
| 08-nsurr-50 | n_surr=50 | 3/30 | 5,65 % | PASS |
| 09-nsurr-200 | n_surr=200 | 3/30 | 4,97 % | PASS |
| 10-bins-3 | Binned, bins=3 | 6/30 | 8,82 % | FAIL |
| 11-bins-4 | Binned, bins=4 (Binned-Baseline) | 1/30 | 4,43 % | PASS |
| 12-bins-8 | Binned, bins=8 | 0/30 | 1,01 % | PASS |
| 13-nulllag-6 | Residual, null_lag=6 | 0/30 | 7,04 % | FAIL |
| 14-nulllag-12 | Residual, null_lag=12 (Residual-Baseline) | 0/30 | 6,78 % | PASS |
| 15-nulllag-24 | Residual, null_lag=24 | 0/30 | 6,15 % | FAIL |
| 16-shift | Shift-Null (am OP) | 3/30 | 4,11 % | PASS |

## Der Verdikt-Satz

Keine Achse schlägt den Betriebspunkt strikt. Der OP ist der
**achsen-lokale Gewinner unter den gemessenen Punkten** (τ×p_max-Interaktion
ungemessen, p_max=2 fix) — der OP bleibt (Block/KSG, τ=2, null_lag=12,
bins=4, block=n^(1/3)=5, n_surr=100). Der OP-Anker ist byte-gleich dem
Handover (3/30 links >70 %, FPR 5,19 %) — die per-ri-Redraw-Reparatur war
am Anker eine no-op (top_redraws=0), das Blatt ist unter demselben Ensemble
geboren.

## Die Achsen (gemessen, nicht interpretiert)

- **τ (max_lag) — von beiden Seiten begrenzt:** τ=1 verliert die Power
  (0/30 — die wahren Lags sind 1–2); τ=3 und τ=5 fallen durchs Gate (FPR
  6,76 % / 8,12 %). τ=5 trägt die meiste Power (5/30), aber seine FPR 8,12 %
  verletzt das Tor — die Vorhersage („τ=5 trägt ~9,4 % FPR") ist der Richtung
  nach eingetroffen, gemessen 8,12 %. Der OP τ=2 ist das begrenzte Optimum.
- **Block-Länge:** block=3 trägt 5/30 Power (gegen 3/30), aber der Vorsprung
  (Δ=2) liegt innerhalb der realisierten Binomial-Streu (2σ_paired≈5,24) und
  die FPR ist schlechter (7,03 % > 5,63 %-Schranke) — kein strikter Gewinn.
  block=8/12 reinigen die FPR (4,78/4,48) bei gleicher/niedriger Power.
  n^(1/3)=5 ist der mittlere, tragende Punkt.
- **n_surr:** flach — 50/100/200 alle 3/30, FPR 5,65/5,19/4,97. 200 kauft
  nichts, das außerhalb der Streu liegt.
- **bins (Binned — nur das Gate spricht):** bins=3 trägt 6/30 Power, fällt
  aber durchs Gate (FPR 8,82 %); bins=4 → 1/30; bins=8 → 0/30 bei sauberster
  FPR (1,01 %). Weniger Bins = mehr Power, mehr FPR-Leak. Die Binned-Baseline
  (bins=4) ist nicht der OP — der OP läuft KSG, die bins-Achse ist die
  Schätzer-Variante (benannt, kein Namens-Überladen).
- **null_lag (Residual):** die Residual-Null trägt am Anker keine Power
  (0/30 bei allen drei null_lag), und nur null_lag=12 besteht das Gate
  (6 und 24 fallen durch den a-Anstieg bei D_Z=4). Block > Residual am
  T=150-Anker mit KSG — gemessen, nicht behauptet.
- **Shift (16. Punkt, T=150-Dreieck):** 3/30 Power, FPR 4,11 %, Gate PASS.
  Aber Δ=0 auf der Primärachse; die Gate-Zellen sind gemischt (Shift schlechter
  bei kleinem a — a=0 D_Z=0 3,75 gegen Block 2,75 —, besser bei großem a);
  und die FPR-Differenz 1,08 pp ist **pending**, kein strikter Gewinn (siehe
  unten). Ein Null-Familien-Wechsel wäre der Bodenwechsel unter der ganzen
  Messung; sein Zweck (der n=1000-Leak, Block leakt 75,64 % IDTxl) ist bei
  T=150 ungemessen. **n=1000-Shift bleibt eine Register-Duty (pending).**

## Die Gewinner-Regel (angewendet, nicht umformuliert)

- block=3: Δ=2 < 2σ_paired=5,24 (innerhalb der Streu) UND FPR 7,03 % > 5,63 %
  (verletzt die Nicht-Schlechter-Schranke fpr_op + 2σ_fpr) — kein Gewinn.
- Shift: Δ=0 (Power-Gleichstand) — der FPR-Gleichstands-Brecher trägt den
  einzigen Kandidaten. Die σ-Frage ist eine **ungemessene Größe**, nicht zwei
  Regeln: die Auftrags-Schranke 2σ_fpr=0,44 % (neg=10200) und die
  Operator-Band 2,2 pp (100 Trials je Zelle) differieren um sqrt(10200/100)≈10 —
  genau die Annahme „Surrogat-Tests unabhängig". Die 10200 Negative sind
  phasenrandomisierte Kopien, die das Amplitudenspektrum und damit die
  Autokorrelation teilen (Wiener–Chintschin) — 0,44 % ist eine obere
  Optimismus-Schranke, nicht das gemessene σ. Das wahre σ (die effektive
  unabhängige Negativzahl) liegt dazwischen, ungemessen. Bis sie gemessen ist
  (z. B. Block-Resampling der Negative), ist der Shift-Vorsprung 1,08 pp
  `pending`, kein strikter Gewinn.

## Verifikations-Kette (benannt)

- OP-Anker byte-gleich dem Handover (3/30, 5,19 %) — die Maschine reproduziert
  sich unter demselben Seed.
- Zug-5-Batterie geteilt: `gate_fpr_cells` (te.rs) ist der eine Pfad für Test
  und Probe — Byte-Parität per Konstruktion; CI-OP-Gate (KSG) PASS
  (Zellen 2,75/2,75/1,50/2,86/1,67/3,81 %).
- Zug 5 Binned (Block + Shift) lokal grün; Shift-Gate T=150 PASS.
- `--r a:b`-Split byte-identisch am ri=0 (Flag + Redraw-Reparatur).

## Gemessener Riss (nicht dieses Atoms)

`pcmci_recovers_known_dag` ist in main rot (te 0,0127 < thr 0,0208): die
TODO-Zeile des Nobel-DAG-Atoms und Handover §5 beschreiben den Fix
(Generator 0.6·a[t−1], ksg/Block/100, 115 s), aber main trägt noch den alten
Generator (0.5·a_ind[t−1]) am alten Betriebspunkt (Residual/Binned/10,
max_lag=1). Der Worktree-Fix wurde nicht committet — Register-Duty, kein
stiller Fix in diesem Atom.
