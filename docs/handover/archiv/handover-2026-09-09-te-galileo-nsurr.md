<!--
  title: Thematisches Handover — Galileo N_SURR 20-vs-10: das Instrument steht, das Blatt folgt
  class: handover
  date: 2026-09-09
  sha256: 5f46eba3979186f94fe59338f72a991106ff961d8098c69b397f6e76ecb5f029
  status: archived
  see-also: docs/handover/archiv/handover-2026-09-09-te-atom-4-folge.md docs/befund/befund-galileo-nsurr-20.md
-->
# Thematisches Handover — Galileo N_SURR 20-vs-10: das Instrument steht, das Blatt folgt

Nachfolger des thematischen Registers `handover-2026-09-09-te-atom-4-folge.md`
(archiviert). Diese Sitzung hat die Erhebung gebaut, die der Auftrag
`auftrag-galileo-nsurr-20.md` erteilt hatte; gemessen (CI-Sheet) und das Blatt
folgen.

## Geschlossen (diese Sitzung)

- Additive `_n`-Varianten gebaut: `surrogate_stats_phase_n` /
  `surrogate_stats_block_n` in `src/mathematikerin/te.rs`; die kanonischen
  `surrogate_stats_phase` / `surrogate_stats_block` rufen die `_n`-Form mit 10
  — der 10-Pfad bleibt byte-identisch (Test
  `phase_block_null_stays_byte_identical_at_ten` grün, `cargo check -p
  omegaflow-measure` 0 Warnungen).
- `--n-surr`-Flag (Default 10) durch die fünf Galileo-Proben
  (`galileo_te_floor_direction`, `galileo_spec_te`, `galileo_floor_external_te`,
  `galileo_floor_stair_te`, `galileo_mode3_s1_repl`); die era-bedingte
  Residual-Null (cThr) behält ihre `N_SURR = 20`.
- CI-Sheet-Workflow `galileo-nsurr-20.yml` steht: 10 Punkte (5 Proben ×
  `--n-surr` 10/20), Release, gleiche Seeds; zieht `galileo_resid.bin` +
  `ephemeris_earth.bin` + `ephemeris_galileo_daily.bin` vom CDN.

## Offen (an die nächste Sitzung)

- Galileo-Sheet → Blatt: `galileo-nsurr-20.yml` dispatchen, das Sheet ziehen,
  `docs/befund/befund-galileo-nsurr-20.md` schreiben (hält der FPR-Vorsprung /
  halten die Schwellen unter 20, wo unter 10 gemessen wurde — oder bleibt 10
  das gemessene Blatt).
- n=1000: das Blatt steht (kein Null-Umzug) — der Riß selbst (Block-Länge bei
  n=1000) ist die nächste Adresse, kein Shift.
- T=600-Sheet → Blatt (Run 34387500520).
- Bz-Hälfte: omni2_indices.bin manifestieren (oder CDN-Fallback in
  `load_indices`) → Bz-Re-Messung n_surr=100.
- Nadel Ⅲ: Richtung TIAW vs Nanoflares offen; 613-Ereignis-Satz
  (`data/jsoc.stanford.edu/aia2015_fullyear.bin`, CDN-Monats-Assets);
  Richtungs-Nachmessung via `aia_ladder_probe`/`aia_three_year_probe`;
  Kaskaden-Stärke × Sonnenstruktur offen.
- Baupunkte: `cycle_phase_shift_surrogate`-Nutzung, bedingte Multi-Force-TE
  (Phasenraum) — pending-Instrumente.
- Desktop-Fork (GTX 970): 30-Jahres-Lauf bleibt Operator-seitig.
