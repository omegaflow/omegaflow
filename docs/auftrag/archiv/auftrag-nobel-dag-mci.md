<!--
  title: Auftrag — Nobel-DAG-Atom: iterative Eltern-Selektion, MCI-Phase, FDR innen (der Runge-Einbau in Rust)
  class: auftrag
  date: 2026-09-09
  sha256: 045f19d2709b99f6a25d1304f3d8fac7b3abce65372daaa2e95de364507bdb22
  status: archived
  see-also: docs/TODO.md docs/befund/befund-klassen-benchmark-pcmci.md docs/handover/handover-2026-09-07-nobel-dag-bz-laic.md
-->

# Auftrag — Nobel-DAG-Atom: iterative Eltern-Selektion, MCI-Phase, FDR innen (der Runge-Einbau in Rust)

## Der Auftrag

Der gemessene Rückstand der Maschine hat eine Adresse: `pcmci_links`
(src/mathematikerin/te.rs) trägt die Vorwärts-Elternsuche, aber (a) der zweite
Durchgang ist tot — `tested` blockt jeden Re-Test; (b) keine MCI-Phase — es
wird nur gegen die Eltern des Ziels konditioniert, nie gegen die des Treibers
(das ist genau die Kontrolle, die Runge für den Common-Driver-Fall braucht);
(c) FDR (`benjamini_hochberg`) läuft außerhalb der Suche. Der Atom baut die
echte PCMCI-Struktur nach — Methode ja, Code nein — auf dem Fundament der
Atome 1+2 (Block-Null, n_surr=100, KSG-kNN daneben): PC-Phase mit
Subset-Re-Test (iterative Eltern-Selektion), MCI-Phase (Eltern von Treiber
und Ziel), FDR innen (Benjamini-Hochberg über die MCI-p-Werte, α = 0.05 —
der publizierte Satz). p_max = 2 (benannte Konditionierungstiefe — schützt
die KSG-Dimension).

## Ausgangslage (gemessen — nicht neu suchen)

- Atome 1+2 geschlossen: Block-Null (n^(1/3)=5 am Anker), n_surr=100,
  KSG-kNN (Frenzel–Pompe-Vollraum, k=4); Zug 5 als Dauer-Tor grün für
  (Block, Binned) und (Block, Ksg).
- Befund-Zahlen als Zielscheibe (befund-klassen-benchmark-pcmci.md,
  Betriebspunkt damals Residual/10/Binned — die Zielscheibe ist die
  Fläche, der Betriebspunkt ist jetzt Block/100/Ksg, benannt):
  Anker N=10/T=150/c=0.287: 0/30 Links >70 % Power bei FPR 6,4 %;
  publiziert: 99 % der Links >70 % bei FP ≈/unter 5 %. Chaos-§VII.B:
  FPR wächst mit a bis 17,5 % (publiziert: a-unabhängig kontrolliert).
- Gemessen nach Atom 2 (ksg/Block/100): Anker 0/30 unverändert bei FPR
  3,88 %; der Zug-6-Floor beweist, dass kNN die Anker-Kopplung mit den
  wahren Eltern findet (≥3/30 über 70 %) — die Lücke ist die Suche, nicht
  der Schätzer. Das ist die Dringlichkeit dieses Atoms.
- Der Handover-Plan (2026-09-07) skizziert genau diesen Einbau: iterative
  Eltern-Suche, dann MCI, dann FDR.

## Spielregeln der Sitzung (bindend)

1. **Eine abgeschlossene Sitzung ist ein Atom.** Keine `pending`s — jeder
   geöffnete Punkt endet gebaut und gemessen.
2. **Zug 5 ist das Dauer-Tor:** die neue Maschine läuft durch dasselbe
   FPR-unter-Autokorrelation-Kriterium (FPR ≤ 8 % je a, beide D_Z, Anstieg
   ≤ 2 pp) — für Binned und Ksg. Ein Tor, das die neue Suche nicht trägt,
   ist rot, und Rot ist das Ergebnis, nicht der Skandal.
3. **Die Batterie ist das Urteil:** Erfolg heißt FPR kontrolliert (Zug 5)
   und TPR-Fläche messbar über dem heutigen Blatt (ksg/Block/100) — gegen
   das eigene Blatt, nicht gegen Runge. PCMCI-Parität ist explizit kein
   Kriterium.
4. A/B-Rahmen: SEED 0x9E3779B97F4A7C15, Anker N=10/T=150/c=0.287, fix.
   Jede Zahl im Nachher-Blatt unter denselben Bedingungen.
5. Registerzeile + Commit im selben Schluss, nur die eigenen Dateien.

## Verifikation

- `cargo check` null Warnungen; gezielte Tests (`pcmci_recovers_known_dag`,
  `binned_null_*`, `calibration_*`) grün.
- Zug 5 (Block, Binned) und Zug 5 (Block, Ksg) grün mit der neuen Suche.
- Nachher-Batterie (release, ksg/Block/100, p_max 2, α 0.05): das Blatt
  gegen die Befund-Zielscheibe, Zahlen in die TODO-Registerzeile.
