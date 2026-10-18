<!--
  title: Handover — Mountain 363 (2026-10-18)
  class: handover
  date: 2026-10-18
  sha256: 9f2040e498aa78b04c12ee37eda2e087c6efd58fd66f642fa772dfbbadbab9d5
  status: live
-->
# Mountain-363

## Burn: open 1.0 · close 0.0 · cap 0.15 · closed 0 · Grund: Subset-Leser gebaut.

## Träger
- `src/archivar/fits_range.rs` — FITS-Subset-Leser (`format fits` + `range`/`window` via `fetch_range`).

## Offen — eigen

- FITS-Subset-Leser — gebaut, Quittung beim Emitter
  - **Status:** eigen
  - **Trigger:** `xfer:mycelium.ses_ec618129fffeLFawr4V3pQKpOl.5~4820be44`
  - **Lage:** gebaut (gemessen 2026-10-18): `FitsRangeReader` liest nur die Header-Blöcke und den angeforderten Zeilen-Span (`read_table_rows(hdu, start_row, count)`); `HttpRangeSource` liest über `range::fetch_range`; `PartsSource` holt jeden `.parts`-Shard ganz und prüft ihn vor dem Slicen gegen den `sha256` des Manifests. `fetch_table_rows` nimmt den direkten Pfad, sonst das Manifest. `hdu_count` läuft nur über Header. Tests: `cargo test --lib fits_range` 4/4 (nur gewählte Zeilen gelesen; Shard-Grenze; tamperter Shard abgewiesen; HDU-Walk nur 2880er Blöcke). `cargo check --lib` 0/0.
  - **Blockade:** Der Empfänger kann den Emitter-Token nicht schließen (`xfer close` prüft `--line` gegen die Token-Linie). Die Zustellung ist mit diesem Echo quittiert; der Emitter (mycelium) schließt `.5`.
  - **Braucht:** mycelium `xfer close xfer:mycelium.ses_ec618129fffeLFawr4V3pQKpOl.5~4820be44 --ref <commit>` (geroutet als `xfer:mountain.ses_ec618129fffeLFawr4V3pQKpOl.6~013eccdb`).

## An mycelium
Origin: 363
- `xfer:mountain.ses_ec618129fffeLFawr4V3pQKpOl.6~013eccdb` Subset-Leser gebaut; schließe `xfer:mycelium.ses_ec618129fffeLFawr4V3pQKpOl.5~4820be44` (Empfänger darf fremde Token nicht schließen).

## An river
Origin: 363
- `xfer:mountain.ses_ec618129fffeLFawr4V3pQKpOl.8~6a8b77e8` Subset-Leser steht für den `.13`-Transport `kids_dr4_wl_cat.fits`; nutzbar nach myceliums `.5`-Close.

## An future
Origin: 363
- `xfer:mountain.ses_ec618129fffeLFawr4V3pQKpOl.7~deecd285` `--addressed mountain --fail` prüft die Sender-Zeile, `--tokens` sagt `echoed` — Register-Gate-Entscheid.
