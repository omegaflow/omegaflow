<!--
  title: Auftrag — Galileo N_SURR 20-vs-10: die Phasen-/Block-Null unter der Zählung, die das Papier dachte
  class: auftrag
  date: 2026-09-09
  sha256: c163ca0ad5c0c576eee7548eb10420dac9c866a26e03a93d0bb5e7f03844a5f1
  status: live
  see-also: docs/handover/handover-2026-09-09-te-atom-4.md docs/handover/archiv/handover-2026-09-09-te-atome-blocknull-ksg-pcmci.md
-->
# Auftrag — Galileo N_SURR 20-vs-10: die Phasen-/Block-Null unter der Zählung, die das Papier dachte

## Der Auftrag

Das Papier dachte N_SURR=20 für die Phasen-/Block-Null der Galileo-TE-Blätter;
der Baum trägt 10. Die fünf Galileo-Proben (`galileo_te_floor_direction`,
`galileo_spec_te`, `galileo_floor_external_te`, `galileo_floor_stair_te`,
`galileo_mode3_s1_repl`) tragen zwar `const N_SURR = 20`, geben die 20 aber nur
an die era-bedingte Residual-Null (`conditional_te_stats`, Spalte cThr); die
Phasen-/Block-Schwellen (Spalten Ph*/Bl*) entstehen in `surrogate_stats_phase` /
`surrogate_stats_block` (`src/mathematikerin/te.rs`), die intern hart 10
Surrogate tragen.

Der Ist-Zustand (10) bleibt. Dieser Auftrag erteilt die Erhebung, die den
Ist-Zustand gegen 20 prüft — kein stiller Shift.

## Die Erhebung

1. Additive `_n`-Varianten: `surrogate_stats_phase_n` / `surrogate_stats_block_n`
   (Parameter n_surr); die alten Funktionen rufen die `_n`-Form mit 10 — der
   10-Pfad bleibt byte-identisch, die kanonische Null wird nicht bewegt.
2. `--n-surr`-Flag durch die fünf Galileo-Proben (Default 10 — der Ist-Zustand;
   20 ist der Erhebungs-Punkt, kein neuer Default).
3. CI-Sheet: je Probe ein Punkt `--n-surr 10` (Reproduktion) und `--n-surr 20`
   (der Papier-Wert) — Release, gleiche Seed-Ströme.
4. Blatt: hält der FPR-Vorsprung / halten die Schwellen unter 20, wo unter 10
   gemessen wurde — oder bleibt 10 das gemessene Blatt.

## Spielregeln

- Kein stiller Shift: der Probe-Default bleibt 10, bis das Blatt einen Umzug mißt.
- Jede Änderung an der kanonischen Null muß die vier Kalibrier-Gates bestehen;
  die additive `_n`-Form läßt die 10-Pfad-Null unberührt (Byte-Parität per
  Konstruktion).
- Registerzeile + Commit im selben Schluß.

## Verifikation

- `cargo check` null Warnungen; `surrogate_stats_phase`/`_block` byte-gleich der
  `_n`-Form bei 10 (Test).
- CI-Sheet trägt Punkt + Probe + n_surr + Commit-SHA.
- Befund `docs/befund/befund-galileo-nsurr-20.md` antwortet auf diesen Auftrag.
