<!--
  title: Befund — Klassen-Benchmark: die TE-Maschine gegen die publizierte PCMCI-Suite (TPR/FPR)
  class: befund
  date: 2026-09-08
  sha256: faaac1d1f3aa97d48312217a72684f1f89c9591995d1a8ff1a4eea1c7245bbcb
  status: done
  antwortet-auf: docs/auftrag/auftrag-klassen-benchmark-pcmci.md
  see-also: docs/TODO.md docs/handover/handover-2026-09-08-nobel-dag-atom.md docs/handover/handover-2026-09-08-atom-a-gpu-port.md
-->

# Befund — Klassen-Benchmark: die TE-Maschine gegen die publizierte PCMCI-Suite (TPR/FPR)

## Der Probe

`tools/measure/src/bin/pcmci_class_benchmark.rs` — Rust std-only, kein Python,
kein Lauf fremden Codes; der Vergleich läuft gegen publizierte Zahlen. Der
Probe baut jede Modellklasse nach der Prozessbeschreibung der Quelle nach und
legt die eigene TPR/FPR-Fläche von `pcmci_links` je Kante daneben. Betriebspunkt
der Maschine wie `nobel_probe_bz` (max_lag 2, null_lag 12, bins 4, n_surr 10;
Schwelle mean+2σ über 10 Residual-Surrogaten); Seed 0x9E3779B97F4A7C15, jeder
Punkt deterministisch wiederholbar (`--quick` verkleinert das Ensemble für
Probeläufe). Ensemble je Punkt benannt (R Topologien × S Realisierungen;
publiziert waren 20 × 100 — die publizierten Zahlen bleiben ihre eigenen).

## Das Blatt (gemessen 2026-09-08)

### [1] Sci. Adv. 5, eaau4996, SM Eq. (S60) — linear (arXiv:1702.07007v2, SM §S4.1, Tab. S3; T=150)

| Punkt (R×S) | Power min/med/max (Links >70 %) | FPR | Publiziert (Text) |
|---|---|---|---|
| N=2 c=0.287 (3×20) | 0.25/0.30/0.35 (0/3) | 9.4 % | FP ≈/unter 5 % |
| N=5 c=0.287 (3×20) | 0.00/0.20/0.45 (0/15) | 7.0 % | FullCI 80 % |
| N=10 c=0.287 (3×20) | 0.05/0.20/0.55 (0/30) | 6.4 % | 99 % der Links >70 % |
| N=10 a-set2 stark-autokorr. (2×20) | 0.00/0.10/0.35 (0/20) | 7.7 % | FP kontrolliert |
| N=10 max_lag=5 (2×10) | 0.00/0.10/0.40 (0/20) | 5.4 % | publiziertes τ-Budget |
| c=0.2/0.247/0.324/0.414 (je 2×10) | med 0.10→0.20, max bis 0.80 (1/20 bei c=0.414) | 6.7–7.3 % | Power steigt mit c (Fig. 6, qualitativ) |
| T=150/300/600, c=0.2 (je 2×10) | med 0.10→0.30, max bis 0.80 (1/20 bei T=600) | 6.7–7.7 % | Power steigt mit T (Fig. S8, qualitativ) |
| bins=3 / bins=8 am Anker (je 2×10) | med 0.20/0.10 | 8.4 % / 5.2 % | — (Konfig-Fläche der Maschine) |

### [2] Sci. Adv. Eq. (S60), nichtlinear 50 % f¹ / 25 % f² / 25 % f³ (T=150, c=0.287)

| Punkt (R×S) | Power min/med/max | FPR | Publiziert (Text) |
|---|---|---|---|
| N=5 (2×10) | 0.10/0.30/0.90 (1/10) | 7.1 % | PCMCI höchste Power |
| N=10 (2×10) | 0.00/0.20/0.60 (0/20) | 6.7 % | leichte FP-Inflation bei großem N |

### [3] Chaos 28, 075310 (2018) §VII.A — gekoppelte Logistik-Abbildungen, r=4, n=150 (S=50)

| σ | Z→X / Z→Y | FPR | Publiziert (Text) |
|---|---|---|---|
| 0 | 50/50, 50/50 | 0.0 % | PCMCI „fast keine Power" bei σ=0; PCMCI₀ 0.8 |
| 0.2 | 50/50, 50/50 | 13.6 % | Power-Peak bei σ=0.2; FP ≈0.05 |
| 0.4 | 48/50, 46/50 | 11.8 % | Power fällt nach σ=0.2 |

Bei σ=0 kollabiert die Residual-Null der Maschine (Schwelle ≈ TE → Münzwurf):
gemessen 50/50 statt einer Power — der Null-Kollaps bei Determinismus ist
benannt, nicht interpretiert.

### [4] Chaos 28, 075310 (2018) §VII.B — Autokorrelation + Common Driver, n=150 (S=20)

| D_Z | a | FPR (c=0) | TPR (c=0.3) | Publiziert (Text) |
|---|---|---|---|---|
| 0 | 0 / 0.5 / 0.9 | 5.0 % / 8.8 % / 17.5 % | 9/20, 6/20, 7/20 | FP a-unabhängig kontrolliert, TP konstant |
| 4 | 0 / 0.5 / 0.9 | 6.3 % / 7.3 % / 7.8 % | 3/20, 8/20, 3/20 | ebenso |

Die publizierte b(D_Z,a)-Kalibration steht nur in Abbildungen — der Probe setzt
b=0.5, σ_Z=0.25 explizit (benannt).

### [5] IDTxl-MuTE-Netzwerk (Wollstadt et al. 2019, JOSS 10.21105/joss.01081; idtxl/data.py) — n=1000, S=5

Publizierte Recovery-Zahlen: keine (gemessene Abwesenheit — paper.md trägt keine
Batterie, `data.py` nur Generatoren). Blatt allein: x0→x1 lag2 5/5, x0→x2 lag3
4/5, x0→x3 lag2 5/5, x3→x4 lag1 0/5, x4→x3 lag1 1/5; FPR 14.9 %.

### [6] Tigramite-Overview linear (Tutorial-Notebook causal_discovery_overview) — n=1000, S=5

Publizierte TPR/FPR: keine (Einzellauf-Tutorial). Blatt allein: x1→x0 lag1 5/5,
x3→x1 lag1 5/5, x1→x2 lag2 4/5, x3→x2 lag3 4/5; FPR 11.3 %.

## Der Verdikt-Satz

Die Fläche trägt das Superlativ nicht: am publizierten Anker (N=10, T=150,
c=0.287, linear) stehen 0/30 Links über 70 % Power gegen publizierte 99 % der
Links, die FPR liegt bei 6,4 % gegen publizierte ≈/unter 5 %, auf den
Chaos-Batterien wächst die FPR mit der Autokorrelation (bis 17,5 % bei a=0.9)
statt a-unabhängig kontrolliert zu bleiben, und die Logistik-Batterie misst
13,6 % FP gegen publizierte ≈0.05. Das Superlativ „fortschrittlichste
TE-Maschine" ist damit gemessen gestrichen — nicht gerettet, nicht
umformuliert. Die Maschine bleibt, was sie gemessen ist: die TE-Maschine, die
den Runge-Bz-Anker auf echten Daten reproduziert (Bz→AE 4,3×, Bz→Dst 2,4×
Schwelle, 2015–2026 stündlich — eigene Messung, unberührt) und kleine Modelle
sicher trifft (N=2, Tigramite-Overview), deren Klassen-Fläche aber nicht die
der publizierten PCMCI-Suite ist.

## Benannte Unterschiede (die Fläche gilt unter diesen)

- Schätzer: Maschine = binned TE (bins 3–8) mit Residual-OLS-Null; publiziert =
  CMIknn/ParCorr mit Block-Null. τ-Budget 2 (ein Punkt bei 5) gegen τ_max=5.
- Schwelle: mean+2σ über n_surr=10 (nominal ≈2,3 % einseitig) gegen α=0.05 —
  die gemessene FPR 5–18 % liegt dennoch über dem nominalen Satz: die
  Klein-Stichproben-Null (10 Surrogate) und die Null-Modell-Abweichung sind
  Maschinen-Eigenschaften, gemessen.
- Stationaritäts-Tor des Probes: Realisierungs-Schranke |x|>100 mit Topologie-
  Neuziehung (publiziert: Unit-Root-Test auf dem linearisierten VAR) — benannt;
  ohne Neuziehung bliebe der stark-autokorrelierte Arm leer.
- `pcmci_links` re-testet im zweiten Durchgang nichts (`tested` blockt alle
  Paare) — gebauter Ist-Zustand, hier benannt, nicht geändert (der Auftrag war
  Messung, kein Maschinen-Umbau).
- Publizierte Zahlen: nur Text-Zahlen zitierfähig; alle Boxplot-/Kurven-Werte
  der Quellen sind Abbildungs-Pixel (pending Pixel-Lesung, keine Zahl
  fabriziert). PCMCI+ (arXiv:2003.03685, PMLR 124) trägt keinen
  Lagged-only-PCMCI-Arm — kein Vergleich möglich (benannt).
- Gemessen offen für einen künftigen Auftrag: ob ein anderer Betriebspunkt
  (bins, null_lag, τ-Budget) die publizierte Fläche erreicht — die Daten dieses
  Blatts tragen die Frage, die Antwort ist ein neuer Auftrag, kein offener
  Punkt dieses Atoms.
