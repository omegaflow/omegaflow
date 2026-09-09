<!--
  title: Handover — die drei TE-Atome: Block-Null + n_surr=100, KSG-kNN, PCMCI-Suche (geschlossen)
  class: handover
  date: 2026-09-09
  sha256: 934917937c02ed844ecaac58a35b2000d39e45b2e84e63abe0111ca860f5caad
  status: archived
  see-also: docs/TODO.md docs/befund/befund-klassen-benchmark-pcmci.md docs/auftrag/auftrag-klassen-benchmark-pcmci.md docs/auftrag/auftrag-nobel-dag-mci.md docs/concepts/te-literatur-matrix.md
-->

# Handover — die drei TE-Atome: Block-Null + n_surr=100, KSG-kNN, PCMCI-Suche (geschlossen)

Nachfolger-Übergabe der drei Atome, die den gemessenen Rückstand der
Klassen-Batterie geschlossen haben. Ausgangspunkt war der Befund
`befund-klassen-benchmark-pcmci.md` (der Superlativ „fortschrittlichste
TE-Maschine" ist gestrichen); die drei Atome bauen aufeinander auf: erst die
Null, dann der Schätzer, dann die Suche.

## 1. Was gebaut ist

- **Atom 1 — die Null:** `TeNull` (Residual/Block/Shift) als Parameter in
  `conditional_te_surrogates_n` und `pcmci_links`; Block-Länge
  `block_len_from_n` = round(n^(1/3)) = 5 am Anker T=150 (Galileo BLOCK=5 und
  die Faustregel konvergieren); n_surr=100 im PCMCI-Pfad, der Parameter
  durchzieht die 0..10-Schleifen mit Default 10 (kein stiller Shift).
- **Atom 2 — der Schätzer:** `transfer_entropy_ksg_conditional_n` neben der
  Binning-TE — Frenzel–Pompe-Vollraum (ε = k-ter Nachbar im
  (x', x, y, C)-Raum, TE = ψ(k) + ⟨ψ(n_xC+1) − ψ(n_x'xC+1) − ψ(n_xyC+1)⟩),
  std-only-digamma, k = 4. Formel-Anker: JIDT-Quellcode
  (ConditionalMutualInfoCalculatorMultiVariateKraskov1); Literatur-Anker KSG
  2004 (PRE 69, 066138) und Frenzel–Pompe 2007 (PRL 99, 204101), crossref-
  verifiziert in der Literatur-Matrix. `TeEstimator` (Binned/Ksg) mit
  Selektionsregel: dünne Stichproben → kNN, reiche → Bins (nobel-Proben Binned).
- **Atom 3 — die Suche:** `pcmci_links` trägt jetzt die PC-Phase (p=0
  unkonditioniert, dann Subset-Re-Tests gegen die Level-Snapshot-Eltern,
  Entfernung nach dem Level) + die MCI-Phase (Eltern von Ziel UND Treiber,
  dedupliziert) + `benjamini_hochberg` innen (`fdr_pass`-Flag, α = 0.05);
  p_max = 2. Der tote zweite Durchgang ist ersetzt. Auftrag:
  `docs/auftrag/auftrag-nobel-dag-mci.md`.
- **Probe:** `pcmci_class_benchmark` trägt `--null residual|block|shift`,
  `--n-surr`, `--block`, `--est binned|ksg`, `--section 1..6`, `--quick`.
- **Das Dauer-Tor:** Zug 5 in `te.rs #[cfg(test)]` ist parameterisiert über
  (Null, Schätzer) — Kriterium FPR ≤ 8 % je a (beide D_Z) + Anstieg ≤ 2 pp;
  Zug 6 ist der Miss-Funktions-Floor (≥ 3/30 am Anker mit wahren Eltern).

## 2. Der Betriebspunkt der Maschine

max_lag 2, null_lag 12, bins 4, n_surr 100, Null Block (n^(1/3)), Schätzer
KSG (k=4), p_max 2, α 0.05, Seed 0x9E3779B97F4A7C15 — der Punkt, unter dem
die Batterie gemessen ist.

## 3. Die gemessenen Befunde (Vorher → Nachher, gegen das eigene Blatt)

- Anker N=10/T=150/c=0.287: 0/30 → **3/30 Links >70 %** bei FPR 6,4 % → **5,19 %**
  (publiziert ≈5 % — die FPR-Disziplin am Anker ist publiziert-Niveau).
- a-set2 (starke Autokorrelation): 0/20 → **9/20**, FPR 7,7 → 9,68 (benannt).
- max_lag=5: 0/20 → 4/20, FPR 5,4 → 9,42 (benannt).
- c-Sweep: 0/0/1/3 → **2/3/5/11** von 20 >70 % — Power steigt mit c
  (Fig. 6 qualitativ reproduziert).
- nonlinear N=5: 1/10 → 7/10 (publiziert „höchste Power"), FPR 18,43 (benannt).
- Common-Driver: FPR 5,0/8,8/17,5 % → 1,25–4,67 % je Zelle; TPR D_Z=4 2/6/9 →
  5/5/9. Das Zug-5-Kriterium hält auch auf der Batterie.
- IDTxl-MuTE: 0/0/0/5/1 → **5/5 auf allen fünf Kanten**; Tigramite: 5/5, 5/5,
  0/5, 5/5 → 5/5, 5/5, 5/5, 5/5.

## 4. Benannte Risse (gemessen, nicht geschlossen)

- **n=1000-Null leakt laut:** IDTxl FPR 75,64 %, Tigramite 31,88 % — die
  Block-Länge n^(1/3)=10 ist auf n=1000-Serien zu kurz; unter PCMCI wird die
  Inflation lauter (die MCI-p-Werte sind optimistisch). Die Shift-Null (ganze
  Serie rotiert, erhält die volle Autokorrelation) ist gebaut und verdrahtet —
  **ihr Zug-5-Nachlauf ist die dringlichste Folgepflicht.**
- **Chaos-Logistik σ=0.2: FPR 20,4 %** (war 2,4) — die MCI-Konditionierung
  (dim 4) auf nahezu deterministischen Serien bias't die echten TEs über die
  Block-Null hinaus; σ=0 bleibt der 50/50-Münzwurf. Die Grenz-Klasse ist
  benannt: Determinismus trägt keine der beiden Nullen.
- **T=600 ungemessen:** PC+MCI×ksg×100 bei T=600 ≈ Stunden je Punkt (gemessen
  abgebrochen) — Myzel-Duty.

## 5. Gemessene Korrekturen (keine Vermutungen)

- `pcmci_recovers_known_dag` prüfte die Konfound-Leckage: im bbcf2c4-Worktree
  gemessen, der alte ab-te == die unkonditionierte Binning-TE (die Kante lief
  über den gemeinsamen Treiber z, nicht über die Innovation). Der Generator
  trägt jetzt eine direkte A→B-Kante (0.6·a[t−1]), der Test läuft am
  Betriebspunkt (ksg/Block/100, 115 s).
- Galileo-Phasen/Block: das Papier dachte N_SURR=20, der Baum trägt 10 — der
  Ist-Zustand ist benannt und bleibt, bis ein eigener Auftrag die Erhebung
  prüft (kein stiller Shift).

## 6. Offene Pflichten (an die nächste Sitzung)

1. **Shift-Null-Zug-5-Nachlauf** — dringlich, die n=1000-Zahlen tragen die
   Adresse. Danach ggf. den n=1000-Betriebspunkt auf Shift umstellen.
2. **Atom 4 — Betriebspunkt-Sweep** (bins, null_lag, τ, Block-Länge, n_surr)
   als eigener Auftrag unter der neuen Null.
3. **Bz/LAIC-Wiederholung unter n_surr=100** — Myzel/Desktop (die
   Residual-OLS-Null kostet dort Stunden je Lauf, gemessen).
4. **T=600-Batteriepunkt** — Myzel.
5. Die Gate-Kosten sind benannt (Zug 5 Binned 429 s, Zug 5 Ksg 6733 s, Zug 6
   912 s — Debug) und gehören ins CI-/Myzel-Repertoire, wenn das Haus die
   lokale Suite entlasten will.

## 7. Die Zielzeile

„Empfindlichkeitsgrenze der eigenen Jagd gemessen und geschlossen bis X":
X steht heute bei Anker 3/30, a-set2 9/20, c-Sweep bis 11/20, Anker-FPR
5,19 % — gegen das eigene Blatt. PCMCI-Parität war explizit kein Kriterium;
die n=1000-Null ist die nächste gemessene Adresse.
