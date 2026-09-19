<!--
  title: Handover — Bau-Folge 96 (Stand 2026-09-19)
  session: Bau-Folge 96
  class: handover
  date: 2026-09-19
  sha256: 1ca2e28df18f65bc8b4c933b354e456dce884c89c134e3a6f3222d2b696c3914
  status: live
-->
# Handover — Bau-Folge 96 (2026-09-19)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder offene
Punkt trägt seinen nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt. Das Handover wird **vor allem anderen gegen den Baum gehalten**.

## Stehender Pass (gemessen 2026-09-19, Session-Beginn)

- **HEAD** `7aa5c23e` bei Start (== `origin/main`, „bau folge94"); während der
  Session von fremden Linien auf `f6b10df0` gezogen (ernte folge100). Safety-Net
  `refs/safety/1789851206` (Start), `refs/safety/1789851255` (fremd).
- **Postfach** — `state/mail/mail_ledger.φ` lokal abwesend (kein lokaler Ledger in
  diesem Checkout); kein neuer Eingang messbar. Zustand-Eintrag `external-state.md:20`
  zitiert: letzter Ledger-Eingang `1789795811`.
- **CI** — `external-state.md:22` (ernte folge100, HEAD-Wechsel) zitiert:
  **failure** `openneuro-cdn 35468606830` @`7aa5c23e` (ds007822, jetzt Fix gebaut);
  **in_progress** `ci-check 35468441157`, `harvest 35467466676`, `placebo-ave-cdn`,
  `ps1-cdn`; **success** `quake-feeds-cdn 35468653918`, `harvest 35467467726`.
- **Post** — die `An bau`-Zeile (gedi_l2a + icesat2 + ds007471 + ds008192 +
  ds007822) **gefaltet** in dieses Handover; `post.md` geleert.

## Offen

- **ds007822 EEGLAB-.set Parser-Gap** `phi/pipeline/ledger.φ:122-124`. Gemessen:
  `chanlocs.labels` liegt als **miUTF8 (Typ 16, Kleinformat-Tag)** vor; `matfile.rs`
  `read_tag`/`read_data` hatten keinen Typ-16-Arm → `labels` Empty → `extract_eeg`
  None. **Fix gebaut**: `MI_UTF8/16/32` + `small_type 16..=18`, Tests
  `a_utf8_char_matrix_decodes_to_utf8`, `a_chanlocs_struct_with_utf8_labels_decodes_each_label`.
  (Schritt: `openneuro_compiler --local <eine .set>` in CI, dann `--probe`.) · `wartend`
- **ds007471 BrainVision-Ingest-Arm** `phi/pipeline/ledger.φ:126-128`. Arm gebaut:
  `src/archivar/brainvision.rs` (vhdr/vmrk/eeg → `OpenNeuroEeg`, kanalmajor µV) +
  `tools/harvest/src/bin/brainvision_compiler.rs` (GraphQL, Roundtrip, `--local`,
  CDN hinter `--ci-mode`), check 0/0. (Schritt: `brainvision_compiler --local
  <eine Triple>` in CI, dann Voll-Ernte.) · `wartend`
- **ds008192 SNIRF-Ingest-Arm** `phi/pipeline/ledger.φ:130-132`. Arm gebaut:
  `src/archivar/snirf.rs` (neues `SNIR`-Format, HDF5-Reader) +
  `tools/harvest/src/bin/snirf_compiler.rs` (Roundtrip, `--probe`, CDN hinter
  `--ci-mode`), check 0/0. (Schritt: `snirf_compiler --probe <eine .snirf>` in CI.) · `wartend`
- **gedi_l2a HDF5-Hang** `phi/harvest.φ:57-65`. Range-Guard gebaut: globales
  Byte-Budget `MAX_TRAVERSAL_BYTES=1<<26` im `Hdf5WindowReader`, `Hdf5Note::TraversalBudget`,
  Test `object_header_traversal_is_range_bounded`. (Schritt: Harvest-Run in CI →
  `gedi_l2a.bin`.) · `wartend`
- **icesat2_atl03 Budget** `phi/harvest.φ:75-83`. `--limit 1 --skip 0` gesetzt;
  Compiler trug `--limit`/`--skip` bereits (`window_slice`, Test
  `window_slice_applies_offset_then_limit`). (Schritt: nächster Harvest-Lauf,
  `--skip` je Lauf fortschreiben.) · `wartend`

## Wartestellungen (kein Auswahlpunkt)

- **`--sniff` Partial-Hash-Fix CI-Verifikation** (`net.rs`, aus bau folge95
  übernommen) — der `test`-Job des `ci-check`-Laufs am Push dieses Commits.
  · `wartend`
- **`lazy_chunk`- + folge93-Gates** — derselbe `ci-check`-Lauf. · `wartend`
- **TE-Gates Multi-Seed-Umbau** (aus folge92) — derselbe `ci-check`-Lauf. · `wartend`
- `te-gate 35462518676` @`3d2e6adb` · `wartend`
- `planetary-odf-cdn 35351411938` · `wartend`
- **allwise-cdn** — 127/304 Spans, Final `allwise_coverage.fp01` absent. · `wartend`

## Benchmark

- **Bau-Folge 96**: gedi_l2a → `grind-pro`; icesat2 → `grind-flash`;
  BrainVision + SNIRF → `grind-max` (novel parser); ds007822-Diagnose → `grind-pro`;
  PDF-Messung → `vision`. Alle lieferten vollständig; `cargo check -p omegaflow
  --all-targets` + `-p omegaflow-harvest --all-targets` 0 Fehler / 0 Warnungen.
  Kein Doppellauf, kein neuer Klassen-Sieger (Routine-Klassen gemessen geschlossen).
- **PDF-Klasse**: 7 `docs/paper/`-PDFs gemessen — 6 digital (echter Textlayer),
  1 scanned (Armstrong/Woo/Estabrook 1979, raster + unsichtbarer OCR-Layer);
  Figuren transkribiert. Punkt geschlossen.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/archivar/hdf5.rs`, `src/archivar/matfile.rs`,
  `src/archivar/brainvision.rs` (neu), `src/archivar/snirf.rs` (neu),
  `src/archivar/mod.rs`, `src/lib.rs`,
  `tools/utils/src/bin/archive_search/net.rs` (bau95-Fix, übernommen),
  `tools/harvest/src/bin/icesat2_atl03_compiler.rs`,
  `tools/harvest/src/bin/brainvision_compiler.rs` (neu),
  `tools/harvest/src/bin/snirf_compiler.rs` (neu),
  `phi/harvest.φ`, `phi/pipeline/ledger.φ`, `docs/handover/post.md`,
  `docs/paper/armstrong-woo-estabrook-1979-interplanetary-scintillation.md` (neu),
  `docs/handover/handover-2026-09-19-bau-folge96.md` (neu), Move
  `bau-folge-95.md` → `archiv/`, Move `bau-folge-94.md` → `archiv/` (aus bau95).
- **Fremd (nicht anfassen):** `tools/measure/src/bin/hyperscanning_group_te.rs`,
  `docs/zustand/external-state.md`, die drei `handover-2026-09-16-*`-Moves.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
