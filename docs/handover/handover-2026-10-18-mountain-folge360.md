<!--
  title: Handover — Mountain 360
  class: handover
  date: 2026-10-18
  sha256: 10ffaaa5ee3174d68c081c471fabc03902c40873da553791e1c8dbff77d29afb
  status: live
-->
# Mountain-360

## Burn: open 0.0 · close 0.0 · cap 0.15 · closed 0 · Grund: drei Ratsfragen offen

## An future
Origin: mountain-folge360
- `xfer:mountain.ses_ec618129fffeLFawr4V3pQKpOl.3~ce7e554f` Rat: GRS80 (a, f, GM, ω) + γ₀ ohne Parser-Arm — Formular `state/stimmen/2026-10-18-grs80-skalar-formular.md`; `N` bleibt pending.
- `xfer:mountain.ses_ec618129fffeLFawr4V3pQKpOl.4~d91c67ea` Rat: gfc-Block — Cap 5540/3211/2190/760, Epoche unbelegt, `ttl` NO_CADENCE, 5399-vs-5480; Formular `state/stimmen/2026-10-18-gfc-block-formular.md`.
- `xfer:mountain.ses_ec618129fffeLFawr4V3pQKpOl.5~4d871e64` Rat: P10-Restarme — eionet 276 Emissionen, Infraschall-/Feuer-Intensität, TEC/Pseudorange/`vis_nmi`, Stern-Metadaten, FDSN-Kanalidentität, `kg/(m^2 s)`; Formular `state/stimmen/2026-10-18-p10-restarme-formular.md`.

## Pass-Abschluss (R-2026-10-18-c, 4/4 · 2026-10-18)

Messung (Linie `mountain`): Post 0 (`post/in` nicht im Baum) · Trigger 0
(`register_lookup --fired mountain` = 0) · Transfer 3 offen
(`xfer coverage --line mountain`: emitted 5 · disposed 2 · open 3). Delegiert: 3 freie
Subagenten (done-when vom Pass), Belege geprüft, zwei Beleg-Risse in den eigenen
Formularen geschlossen. Die drei offenen Transport-Token `.3/.4/.5` sind Empfänger-Arbeit
bei `future` (kein `future`-Ledger im Baum) — **kein Ledger-Schluss gesetzt**.

- **GRS80-Formular, Direktiv-Arm** — geschlossen. Zitat `src/archivar/parse.rs`:
  `"gm"` 2282 · `"radius"` 2288 · `"norm"` 2297 · `"tide"` 2303 · `"max_degree"` 2304;
  `"f"`/`"omega"`/`"grs80"` = 0 Treffer, das einzige `"a"` (2027) ist KeplerMap.
  Das Formular trug `2270-2299` (tide/max_degree außerhalb). Korrigiert auf `2282-2304`.
- **P10-Restarme-Formular, Quellen** — geschlossen. Zitat `-p10-em.φ:123-124`:
  „Ende (HEAD dieser Tranche): field 7526 · quantity 382" / „Δ field −241 · Δ quantity +241";
  FDSN-BHZ ×2: `-p10-migration.φ:59`; `kg/(m^2 s)`-Token: `-unit-token.φ:8-12`
  (`channel`-Direktive, `unknown channel unit`, nicht Deskriptor-Parser). Belege nachgezogen.

Geprüft ohne Befund: gfc-Formular (Asset-Größen `-gfc-asset-plan.φ:65-71`, Tail
`-gfc-depth2.φ:69-71` = 5480, `gfc.bin` 404, Epoche durchgehend `absent`).

Nicht geschlossen (Urteil bleibt bei der Linie, nicht delegierbar): die drei Ratsfragen
R-2026-10-18-b/-f/-g (Operator wählt) und die drei offenen Transport-Token `.3/.4/.5`
(Ziel `future`, Empfänger-Arbeit).
