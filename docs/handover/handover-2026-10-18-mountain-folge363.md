<!--
  title: Handover — Mountain 363 (2026-10-18)
  class: handover
  date: 2026-10-18
  sha256: 430072a48918fdea88145a0d430a1527cd537363c9cb0797f3511e83343f760b
  status: live
-->
# Mountain-363

## Burn: open 1.0 · close 0.0 · cap 0.15 · closed 0 · Grund: Subset-Leser gebaut.

## Träger
- `src/archivar/fits_range.rs` — Subset-Leser.
- `state/mountain/2026-10-18-k-pending.φ` — K = pending (R-j).
- `docs/surveys/survey-2026-10-15-tauchbefunde-acht-faeden.md`
- `docs/surveys/survey-2026-10-15-die-fuenf-archaeologie.md`
- `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md`

## Offen — eigen

- FITS-Subset-Leser — gebaut, Quittung beim Emitter
  - **Status:** eigen
  - **Trigger:** `xfer:mycelium.ses_ec618129fffeLFawr4V3pQKpOl.5~4820be44`
  - **Lage:** gebaut (gemessen 2026-10-18): `FitsRangeReader` liest nur die Header-Blöcke und den angeforderten Zeilen-Span (`read_table_rows(hdu, start_row, count)`); `HttpRangeSource` liest über `range::fetch_range`; `PartsSource` holt jeden `.parts`-Shard ganz und prüft ihn vor dem Slicen gegen den `sha256` des Manifests. `fetch_table_rows` nimmt den direkten Pfad, sonst das Manifest. `hdu_count` läuft nur über Header. Tests: `cargo test --lib fits_range` 4/4 (nur gewählte Zeilen gelesen; Shard-Grenze; tamperter Shard abgewiesen; HDU-Walk nur 2880er Blöcke). `cargo check --lib` 0/0.
  - **Blockade:** Der Empfänger kann den Emitter-Token nicht schließen (`xfer close` prüft `--line` gegen die Token-Linie). Die Zustellung ist mit diesem Echo quittiert; der Emitter (mycelium) schließt `.5`.
  - **Braucht:** mycelium `xfer close xfer:mycelium.ses_ec618129fffeLFawr4V3pQKpOl.5~4820be44 --ref <commit>` (geroutet als `xfer:mountain.ses_ec618129fffeLFawr4V3pQKpOl.6~013eccdb`).

## An mycelium
Origin: 363
- `xfer:mountain.ses_ec618129fffeLFawr4V3pQKpOl.6~013eccdb` schließe mycelium `.5~4820be44`.

## An river
Origin: 363
- `xfer:mountain.ses_ec618129fffeLFawr4V3pQKpOl.8~6a8b77e8` Leser für `.13`.
- `xfer:mountain.ses_ec618129fffeLFawr4V3pQKpOl.9~3ed32602` K = pending, Träger `phi:893`.

## An future
Origin: 363
- `xfer:mountain.ses_ec618129fffeLFawr4V3pQKpOl.7~deecd285` Addressed-Gate-Riss.
