<!--
  title: Thematisches Handover — TE-Atom-4-Folge (der n=1000-Shift trägt keinen Null-Umzug)
  class: handover
  date: 2026-09-09
  sha256: 3ef51fc7ea0819403abf1f7b9c988970bfa794f0c5c7381d190b15450bc8d115
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-te-atom-4.md docs/auftrag/auftrag-galileo-nsurr-20.md
-->
# Thematisches Handover — TE-Atom-4-Folge (der n=1000-Shift trägt keinen Null-Umzug)

Nachfolger des thematischen Registers `handover-2026-09-09-te-atom-4.md`. Diese
Sitzung hat die abarbeitbaren Punkte der TE-Linie geschlossen und zwei Sheets
gemessen; das Offene folgt.

## Geschlossen und gemessen (diese Sitzung)

- `pcmci_recovers_known_dag` grün (te > thr, B→A verworfen): Generator trägt die
  direkte A→B-Kante (0.6·a[t−1]) statt des Latent-Innovations-Pfads
  (0.5·a_ind[t−1]); der Test läuft am Betriebspunkt (Block/KSG/100, max_lag 2,
  null_lag 12, bins 4, block n^(1/3)). 250 s Debug gemessen — lokal; die
  CI-Re-Verifikation des main-roten Tests ist pending.
- n=1000-Shift gemessen (Run 34386750688): Shift trägt keinen Null-Umzug —
  IDTxl FPR 75,64 % → 66,18 % (block → shift), Tigramite 31,88 % → 32,50 %.
  Die n=1000-Leckage ist keine Null-Familien-Frage: beide Nullen lecken. Die
  Adresse ist die Block-Länge n^(1/3)=10 (zu kurz) / die KSG-Dimension bei
  n=1000. Die Null bleibt Block.
- Bz/LAIC n_surr=100 (Run 34389010206): LAIC gemessen — laic.bin vom CDN,
  1846 Fenster (1726 Ereignisse); die Stille hält unter Common-Cause-Kontrolle,
  Bz→F 0,204 über dem Floor 0,072 (unter n_surr=10 war die Zahl 0,27).
  Bz `unmeasured`: `omni2_indices.bin` fehlt auf dem CI-Runner, `load_indices`
  hat keinen CDN-Fallback (nur Cache) — AE/Dst/SYM-H tragen keine Records. Die
  4,3×/2,4× bleiben, bis die Bz-Hälfte mißt (Register-Duty: omni2_indices.bin
  manifestieren oder CDN-Fallback bauen).
- T=600 (Run 34387500520): in Flug bei Sitzungsende.
- Galileo N_SURR 20-vs-10: Auftrag steht (`docs/auftrag/auftrag-galileo-nsurr-20.md`)
  — kein stiller Shift; die Erhebung (additive `_n`-Varianten, `--n-surr`-Flag,
  CI 10-vs-20) ist erteilt, nicht gebaut.

## Offen (an die nächste Sitzung)

- n=1000: das Blatt steht (kein Null-Umzug) — der Riß selbst (Block-Länge bei
  n=1000) ist die nächste Adresse, kein Shift.
- T=600-Sheet → Blatt (Run 34387500520).
- Bz-Hälfte: omni2_indices.bin manifestieren (oder CDN-Fallback in load_indices)
  → Bz-Re-Messung n_surr=100.
- Galileo-Erhebung (Auftrag steht) — Parameterisierung + CI + Blatt.
- Nadel Ⅲ: Richtung TIAW vs Nanoflares offen; 613-Ereignis-Satz
  (`data/jsoc.stanford.edu/aia2015_fullyear.bin`, CDN-Monats-Assets);
  Richtungs-Nachmessung via `aia_ladder_probe`/`aia_three_year_probe`;
  Kaskaden-Stärke × Sonnenstruktur offen.
- Baupunkte: `cycle_phase_shift_surrogate`-Nutzung, bedingte Multi-Force-TE
  (Phasenraum) — pending-Instrumente.
- Desktop-Fork (GTX 970): 30-Jahres-Lauf bleibt Operator-seitig.

## Nebenfund

- `ephemeris_structure_probe` war herrenlos auf der Platte; committet (toter
  `fmt_jd` entfernt, abwesende Nutation druckt `-` statt fabrizierter 0).
