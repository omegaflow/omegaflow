<!--
  title: Handover — SuperMAG merged-window grain committet (Station-Slot + k-way-merge Compiler)
  class: handover
  date: 2026-09-10
  sha256: f8a9bddd8412a0c8da36fd9d5df79bbb854069c2fceca190020559c8d8062f04
  status: live
  see-also: docs/handover/handover-2026-09-10-supermag.md
-->
# Handover — SuperMAG merged-window grain committet

## Geleistet (trägt Git)

- **geo.rs — Station-Slot** (`station: u32`, `SMG_REC_BYTES = 64`, `pack_iaga`/`iaga_of`,
  `smg_record_bytes`/`smg_record_at`; write_bin/parse_bin verzweigen auf MAGIC_SMG — nur
  supermag trägt 64 Byte, die 8 übrigen Geo-Formate bleiben 60 Byte). Der Slot wurde in die
  noaa-nodd-Atom der parallelen Session absorbiert (deren geo.rs trägt ihn) und trägt hier die
  Compiler-Seite.
- **supermag_compiler.rs — merged-window grain (der Rat)**: `--all` erntet die 194 Stationen,
  k-way-merge der per-Station sortierten Teile (BinaryHeap, 64-Byte-Stream, kein In-Memory-Sort),
  stream-verify. `--station`/`--stations <datei>` bleiben als Einzel-/Listen-Ernte.
- **8 Geo-Compiler** — `station: 0` (absent-Pad) in den GeoRec-Initialisierern — nötig, damit
  der Baum gegen den Station-Slot kompiliert.
- **supermag-cdn.yml** — `--all` (merged-Pilot) statt `--station`.
- **tests.rs** — supermag/iss_lis-Roundtrip trägt die Station.

## Verifikation (gemessen)

- `cargo check -p omegaflow-harvest` im isolierten Worktree: 0 Fehler, 0 Warnungen.
- `cargo test -p omegaflow supermag`: 3/3 grün.
- Commit-Gate (`commit_check`) grün (kein german-in-code, kein zero-fabrication).

## Offen

- **Vollernte der 194 Stationen** — der merged-Pilot (März 2025) als CI-Dispatch; er misst die
  Asset-Größe (CDN-2-GB-Grenze → Fallback per-Station, falls gerissen).
- **CDN-Dispatch** — `gh workflow run supermag-cdn.yml` (Asset `supermag_2025-03.bin`).
- **sources.φ-Url** — der merged-Asset-Name `supermag_2025-03.bin` ist im Baum noch nicht
  committet (die Datei trägt die noaa-Quellen der parallelen Session; eigene Linie beim Dispatch).
- **IAGA-nur-Packung** — `pack_iaga`/`all_stations` nehmen nur 3 ASCII-Großbuchstaben; Codes
  mit Ziffer (z.B. T03) tragen keine pack_iaga-Form (werden übersprungen) — Alternativweg
  (Stationstabellen-Index) ist eine eigene Linie.

## Disposition: die per-Station-Form der parallelen Session

Die parallele Session hatte eine per-Station-Form gebaut (`--emit-stations`/`--stations <tabelle>`
→ ein Bin je Station, Mountains Dissens). Dieser Commit trägt die merged-Form des Rats; die
per-Station-Form liegt als Arbeitskopie unter
`/tmp/opencode/backup/supermag_compiler_tools_harvest_src_bin_supermag_compiler.rs` — falls der
Rat die Körnung doch per-Station entscheidet, ist sie dort wiederverwendbar.
