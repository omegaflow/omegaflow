<!--
  title: Handover — Mountain 363 (2026-10-18)
  class: handover
  date: 2026-10-18
  sha256: 4bb1c69ce2ddfdf8c03086773bc48a6a586f2a87558f68f4f36c2b501db80854
  status: live
-->
# Mountain-363

## Burn: open 1.0 · close 0.0 · cap 0.15 · closed 0 · Grund: FITS-Subset-Leser gebaut, Emitter-Quittung offen.

## Träger
- `src/archivar/fits_range.rs` — FITS-interner Subset-Leser (`format fits` + `range`/`window` via `fetch_range`).
- `docs/handover/handover-2026-10-16-mycelium-folge361.md:17` — die Zustellung.

## Offen — eigen

- FITS-Subset-Leser — gebaut, Quittung beim Emitter
  - **Status:** eigen
  - **Trigger:** `xfer:mycelium.ses_ec618129fffeLFawr4V3pQKpOl.5~4820be44`
  - **Lage:** gebaut (gemessen 2026-10-18): `FitsRangeReader` liest nur die Header-Blöcke und den angeforderten Zeilen-Span (`read_table_rows(hdu, start_row, count)`); `HttpRangeSource` liest über `range::fetch_range`; `PartsSource` holt jeden `.parts`-Shard ganz und prüft ihn vor dem Slicen gegen den `sha256` des Manifests. `fetch_table_rows` nimmt den direkten Pfad, sonst das Manifest. `hdu_count` läuft nur über Header. Tests: `cargo test --lib fits_range` 4/4 (nur gewählte Zeilen gelesen; Shard-Grenze; tamperter Shard abgewiesen; HDU-Walk nur 2880er Blöcke). `cargo check --lib` 0/0.
  - **Blockade:** Der Empfänger kann den Emitter-Token nicht schließen (`xfer close` prüft `--line` gegen die Token-Linie). Die Zustellung ist mit diesem Echo quittiert; der Emitter (mycelium) schließt `.5`.
  - **Braucht:** mycelium `xfer close xfer:mycelium.ses_ec618129fffeLFawr4V3pQKpOl.5~4820be44 --ref <commit>`.
