<!--
  title: Auftrag — Atom 4: Betriebspunkt-Sweep (bins, null_lag, τ, Block-Länge, n_surr)
  class: auftrag
  date: 2026-09-09
  sha256: 2cfa9d2dcc7593d09f06c0e2c62f1669ceb8f6b5cd4bc7aa8506670d17243bd7
  status: live
  see-also: docs/TODO.md docs/handover/handover-2026-09-09-te-atome-blocknull-ksg-pcmci.md docs/befund/befund-klassen-benchmark-pcmci.md docs/auftrag/auftrag-nobel-dag-mci.md
-->

# Auftrag — Atom 4: Betriebspunkt-Sweep (bins, null_lag, τ, Block-Länge, n_surr)

## Der Auftrag

Die drei TE-Atome (Block-Null + n_surr=100, KSG-kNN, PCMCI-Suche) stehen auf
einem Betriebspunkt (τ=2, null_lag=12, bins=4, n_surr=100, Null Block
n^(1/3), Schätzer KSG k=4, p_max=2, α=0.05, Seed 0x9E3779B97F4A7C15). Atom 4
mißt die Achsen: bins, null_lag, τ, Block-Länge, n_surr — beidseitig um den
OP — auf der CI-Matrix, sheet-grade (Anker R3×S20 + Zug-5-Gate-Batterie je
Punkt). Wo ein Punkt strikt gewinnt, zieht der Atom den Betriebspunkt um
(Probe-Defaults + Zug-5-Test auf dem Punkt eingefroren) — im selben Atom.

## Ausgangslage (gemessen — nicht neu suchen)

- OP-Batterie (Handover 2026-09-09): Anker 3/30 Links >70 % bei FPR 5,19 %;
  a-set2 9/20; c-Sweep bis 11/20. Benannte Risse: n=1000-Null leakt
  (Block-Länge zu kurz), Chaos-Logistik σ=0.2 FPR 20,4 % (MCI-Dim 4 auf
  quasi-deterministischen Serien), T=600 ungemessen (Myzel).
- Gate-Kosten benannt: Zug 5 Binned 429 s, Zug 5 Ksg 6733 s, Zug 6 912 s
  (Debug) — CI-/Myzel-Repertoire.

## Das Gitter — 16 Punkte (ehrlich gezählt)

17 Achsen-Zeilen − 2 OP-Koinzidenzen (τ/Block/n_surr fallen am OP zusammen)
+ Shift als 16. Punkt. Die bins-OP-Zeile läuft `--est binned --bins 4`, die
null_lag-OP-Zeile `--null residual --null-lag 12` — beide sind nicht die
OP-Konfiguration, sondern eigene Punkte (Binned-/Residual-Baseline).

| Achse | Punkte (OP fett) | Flags |
|---|---|---|
| τ (max_lag) | 1, **2**, 3, 5 | `--max-lag` |
| Block-Länge | 3, **5 (n^(1/3))**, 8, 12 | `--block` |
| n_surr | 50, **100**, 200 | `--n-surr` |
| bins (Sub-Achse Binned) | 3, **4**, 8 | `--est binned --bins` |
| null_lag (Sub-Achse Residual) | 6, **12**, 24 | `--null residual --null-lag` |
| Shift (16. Punkt, am OP) | — | `--null shift --anchor --gate` |

Inertheits-Tabelle (Lesart steht vor der Messung): Block-Länge inert unter
Shift, null_lag inert unter Block, bins trägt unter Binned keine Power
(Zug-6-Blindheit — bins kann nur durchs Gate fallen oder stehen, nie
gewinnen). Das Null-Dreieck (Residual/Block/Shift) ist ein T=150-Dreieck;
der n=1000-Shift-Riß bleibt eine eigene Register-Duty (pending), kein Schluß
dieses Blatts.

## Die Gewinner-Regel (arithmetisch — Power primär, kein ODER)

1. **Gate-Pass zwingend:** alle 6 Zellen FPR ≤ 8 % + Anstieg ≤ 2 pp je D_Z —
   die zweite, unabhängige Waage.
2. **Power primär:** Δ = count_pt − count_op (>70 %-Links, gepaart je ri —
   der Sweep ist gepaart durch Konstruktion, gleiche Seed-Ströme je ri über
   alle Punkte). Gewinn nur bei Δ > 2·σ_paired, σ aus realisierter
   Binomial-Streu je Punkt (σ² = n_links·p̂(1−p̂)).
3. **FPR:** Nicht-Schlechter-Schranke fpr_pt ≤ fpr_op + 2·σ_fpr
   (σ_fpr = √(p̂(1−p̂)/neg)); Gleichstands-Brecher nur bei Power-Gleichstand
   in der Streu.
4. **Kein „Sieger"-Wort:** emittiert wird „achsen-lokaler Gewinner unter den
   gemessenen Punkten (τ×p_max-Interaktion ungemessen)" — OFAT benannt,
   p_max bleibt 2 bei allen Punkten (τ=5 mißt Such-Tiefe bei Konditionierungs-
   tiefe 2). Greift nichts → der OP bleibt als bestes gemessenes Blatt.
5. **Verifikations-Kette benannt:** Gate-Pass + eingefrorenes Zug-5 am
   Umzugs-Punkt gegen die Auswahl-Vorspannung.

## Spielregeln der Sitzung (bindend)

1. Eine Sitzung ist ein Atom — jeder geöffnete Punkt endet gebaut und
   gemessen.
2. Die Seed-Strom-Reparatur (`top_redraws` per-ri statt global) ist eine
   benannte Änderung: das Sheet wird unter dem neuen Strom geboren; die
   Handover-Zahlen bleiben das historische Blatt. Byte-Vergleich split/unsplit
   der OP-Zeile (`--quick`) vor dem Dispatch.
3. CI-`--gate`-Zahlen am OP müssen byte-gleich den Zug-5-Zahlen sein — der
   Beweis, daß GateCfg-Umstellung und Batterie-Replikation nichts bewegten.
4. Registerzeile + Commit im selben Schluß, nur die eigenen Dateien.

## Verifikation

- `cargo check` null Warnungen; schnelle Tests + Zug-5-Binned +
  `pcmci_recovers_known_dag` lokal grün; Zug-5-Ksg via Welle.
- 16 Matrix-Jobs, `fail-fast: false`, Release-Build je Job, Kappe 300 min
  (5-h-Kappe, benannt); längster Punkt (n_surr=200) lokal getimt und gegen
  die Kappe gerechnet vor Dispatch; Split über `--r a:b` (byte-identisch).
- Sweep-Sheet: Zeilen tragen Punkt + Flags + Commit-SHA + Kostenspalte.
- Befund `docs/befund/befund-betriebspunkt-sweep.md` antwortet auf diesen
  Auftrag; TODO-Zielzeile trägt die Fläche.
