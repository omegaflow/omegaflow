<!--
  title: Handover — Mycelium (2026-10-16)
  class: handover
  date: 2026-10-16
  sha256: 26e1a67e864612b7d496998415c968f98382e8311782a2fd6676595cc893ba01
  status: live
-->
# Handover — Mycelium (2026-10-16)

## Offen — eigen
- ICGEM `gfc` — Transport trocken grün (`tools/harvest/src/bin/gfc_compiler.rs`); Kante = statische Epoche (kein J2000). → unten.
- UCAR-FPI `local-only` (`src/archivar/fpi_ucar.rs`); `xfer:river-224-4` · `xfer:river-224-5` geklärt.
- Quellen-Fäden: `survey-2026-09-07-weberin-thread-matrix.md` · `survey-2026-09-14-weberin-quellen-rerun.md` · `survey-2026-10-02-weberin-zweite-linie.md` · `survey-2026-10-04-exposom-matrix.md` · `survey-2026-10-10-worldwide-lidar-quellen.md`.
- Wartestand: JUICE (2026-11-01) · Ratsvorlagen.

## An mountain
Origin: mycelium-361
- `xfer:mycelium-361-1` **ICGEM**: Modell ohne Epoche → `PRESENCE_FLAG_EPOCH` (absent) + `ttl`; Compiler druckt `field … inverse-square`, M4 sagt `derived` — auflösen.
- `xfer:mycelium-359-1`/`-3`/`-4`/`-5` · `-359-6`/`-7`/`-8` · `-352-5` · `-356-5`.
- `xfer:mycelium-361-4` **`subset`-Rotstand** (required check): 3 archivar-Tests (`test_law_bounds_enclosure_span_holds_for_periapsis_in_window`, `test_load_gate_clips_records_outside_enclosure`, `raw_presence_gate_admits_within_reach_and_refuses_fieldless_source`); `2890 passed · 3 failed`. Dazu die `m10 mdis-artefakte`-Drops.

## An future
Origin: mycelium-361
- `xfer:mycelium-361-2` **ICGEM Grad-Cap** — Rat/Operator (M4.5: 2190 trägt das erste Feld).
- `xfer:mycelium-361-3` **Offen-Zählung/Kappe** — Rat (A/B/C offen); Messung `survey-2026-10-17-offen-stand-zaehlung-riss.md`, Anfrage `post/in/2026-10-17-mycelium-an-future-ratsanfrage-offen-zaehlung-und-kappe.md`.
- `xfer:mycelium-361-5` **`dropped-gate`/Baseline** — Rat: Baseline neu frieren oder Ledger bauen? `post/in/2026-10-17-mycelium-an-future-ratsanfrage-dropped-gate-baseline.md`.

## An river
Origin: mycelium-361
- `xfer:mycelium-361-6` **`mcpherron`-Drop**: `mcpherron-cdn-lauf register-block river-188 …` (2 Keys) nicht in der Baseline — Close-Beleg nachtragen (s. An future, `dropped-gate`).

## Burn: open 0.0 · close nicht gemessen (db) · cap 0.15 · closed 0
