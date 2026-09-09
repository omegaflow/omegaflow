<!--
  title: Thematisches Handover — TE-Atom-4-Folge (ein Punkt geschlossen, drei CI-Sheets dispatched, ein Auftrag erteilt)
  class: handover
  date: 2026-09-09
  sha256: 64e04051fc8499598a5357453c17485cf7c4320d8065e8759a78b5639d8bbeae
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-te-atom-4.md docs/auftrag/auftrag-galileo-nsurr-20.md
-->
# Thematisches Handover — TE-Atom-4-Folge (ein Punkt geschlossen, drei CI-Sheets dispatched, ein Auftrag erteilt)

Nachfolger des thematischen Registers `handover-2026-09-09-te-atom-4.md`. Diese
Sitzung hat die abarbeitbaren Punkte der TE-Linie geschlossen; das Offene folgt.

## Geschlossen (diese Sitzung)

- `pcmci_recovers_known_dag` grün (te > thr, B→A verworfen): Generator trägt die
  direkte A→B-Kante (0.6·a[t−1]) statt des Latent-Innovations-Pfads
  (0.5·a_ind[t−1]); der Test läuft am Betriebspunkt (Block/KSG/100, max_lag 2,
  null_lag 12, bins 4, block n^(1/3)). 250 s Debug gemessen — lokal; die
  CI-Re-Verifikation des main-roten Tests ist pending.
- n=1000-Shift: Workflow `te-n1000-shift.yml` (Block/Shift × IDTxl §5 /
  Tigramite §6), Release am OP. Run 34386750688. Das Blatt (hält Shift die FPR,
  wo Block leakt 75,64/31,88 %) entsteht aus dem Sheet — pending.
- T=600: Probe-Flag `--t600` (spiegelt `--anchor`) + Workflow `te-t600.yml`.
  Run 34387500520. Blatt pending.
- Bz/LAIC n_surr=100: `nobel_probe_bz`/`nobel_probe_laic` tragen die 100
  (835ee03); Workflow `te-bz-laic-nsurr100.yml`. Run 34389010206. Die unter
  n_surr=10 geborenen Zahlen (4,3×/2,4×) bleiben, bis dieses Sheet mißt.
  `laic.bin` kommt vom CDN, ist aber nicht in `phi/sources.φ` registriert
  (anders als `omni2_serie_1h.bin`; manifestiert über `laic-cdn.yml`) — das
  Asset ist vor dem Blatt zu prüfen; fehlt es, ist die LAIC-Hälfte `unmeasured`,
  nicht eine Messung.
- Galileo N_SURR 20-vs-10: Auftrag steht (`docs/auftrag/auftrag-galileo-nsurr-20.md`)
  — kein stiller Shift; die Erhebung (additive `_n`-Varianten, `--n-surr`-Flag,
  CI 10-vs-20) ist erteilt, nicht gebaut.

## Offen (an die nächste Sitzung)

- Drei Sheets → je ein Blatt (n=1000-Shift, T=600, Bz/LAIC); die Blätter
  entscheiden über Null-Umzug, Betriebspunkt und die n_surr=10-Zahlen.
- Galileo-Erhebung (Auftrag steht) — Parameterisierung + CI + Blatt.
- Nadel Ⅲ: Richtung TIAW vs Nanoflares offen; 613-Ereignis-Satz (AIA-2015-
  Volljahr, `data/jsoc.stanford.edu/aia2015_fullyear.bin`, CDN-Monats-Assets);
  Richtungs-Nachmessung via `aia_ladder_probe`/`aia_three_year_probe`;
  Kaskaden-Stärke × Sonnenstruktur offen.
- Baupunkte: `cycle_phase_shift_surrogate`-Nutzung, bedingte Multi-Force-TE
  (Phasenraum) — pending-Instrumente.
- Desktop-Fork (GTX 970): 30-Jahres-Lauf bleibt Operator-seitig.

## Nebenfund

- `ephemeris_structure_probe` war herrenlos auf der Platte; committet (toter
  `fmt_jd` entfernt, abwesende Nutation druckt `-` statt fabrizierter 0).
