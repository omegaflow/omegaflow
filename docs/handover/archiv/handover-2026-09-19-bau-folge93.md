<!--
  title: Handover — Bau-Folge 93 (Stand 2026-09-19)
  session: Bau-Folge 93
  class: handover
  date: 2026-09-19
  sha256: 598e4f23690714bfe9684c163acd05d3167c4fbdb3d3ecfa61fba78524c5fdfb
  status: live
-->
# Handover — Bau-Folge 93 (2026-09-19)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt. Jeder offene
Punkt trägt seinen nächsten Schritt in derselben Zeile; Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`). Wartestellungen sind
kein Auswahlpunkt.

Das Handover wird **vor allem anderen gegen den Baum gehalten**.

## Stehender Pass (gemessen 2026-09-19, Session-Beginn)

- **HEAD** Session-Beginn `eb2e414e` (== `origin/main`); während der Session von
  fremden Linien auf `60d2c2fb` (ernte folge97) gezogen (== `origin/main`).
  `git_safety` Snapshot `refs/safety/1789848003`.
- **Postfach** leer (`post.md` nur Header; `state/mail/mail_ledger.φ` ohne Bau-Zeile).
- **CI** (`ci_manage list`, ~20:12Z): `ci-check 35465331299` @`eb2e414e` in_progress,
  `ci-check 35466644178` pending @`60d2c2fb`; `te-gate 35462518676` pending @`3d2e6adb`;
  `fmt-apply 35464780631` success (→ `d63e2a11`, die baumweite fmt-Drift); `hyperscanning-te` aktiv.
- **Arbeitsbaum** fremd uncommittet (nicht angefasst): die drei
  `handover-2026-09-16-*`-Renames nach `archiv/` und
  `.github/workflows/hyperscanning-te.yml`.

## Offen

- **`read_chunk_diag` ↔ `LazyHdf5`-Reader-Cache** — der Council hielt die
  Cache-Vereinheitlichung bewusst zurück (Verhaltensparität heute). (Schritt: den
  Fetch-Zähler messen, ob `read_chunk_diag` B-Tree-Windows doppelt holt, die der
  gespeicherte Reader schon hält.) · `pending`

## Wartestellungen (kein Auswahlpunkt)

- **CI-Verifikation der neuen Gates** — `lazy_resolution_fetches_only_the_requested_path`
  und `lazy_rollback_retries_a_failed_datatype_chase` (hdf5.rs) sowie die drei
  Pagination-Tests (icesat2_atl03) laufen im `test`-Job von `ci-check`
  (`cargo test --release`). Auslöser: der `ci-check`-Lauf am Push. · `wartend`
- **TE-Gates Multi-Seed-Umbau CI-Verifikation** (aus folge92) — dieselben zwei
  Tests laufen im `ci-check`-`test`-Job. Auslöser: der `ci-check`-Lauf am Push. · `wartend`
- `te-gate 35462518676` pending @`3d2e6adb` · `wartend`
- planetary-odf-cdn `35351411938` · `wartend`
- Scanned-/bild-only-PDFs → `vision`-OCR · `pending`

## Benchmark

- **hdf5 Demand-Resolution** (harte Klasse: Struktur-Urteil + Schreiben): Council
  (pro/max) lieferte das Design — neuer `LazyHdf5`-Typ statt API-Änderung, `&mut self`
  statt `RefCell`, `Hdf5Access`-Trait als Brücke, Rollback gegen den halb-aufgelösten
  Platzhalter; die Implementierung lief in `grind-max` (ein Kontext). Kein
  flash-Vergleich für diese Klasse; der Sieger steht noch aus (die Gate-Tests im
  CI entscheiden).
- **icesat2-Pagination** (mechanisch): `grind-flash`. Die erste Fassung war inert
  (`page_size = window + 8` → nie mehr als eine Seite); die Bau-Session hat die
  Seitengröße vom Fenster entkoppelt (`CMR_PAGE_MAX`/`S3_PAGE_MAX = 2⁷`), damit die
  Pagination real greift.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/archivar/hdf5.rs` (`LazyHdf5`, `Hdf5Access`,
  `parse_object_header`, `chunk_records_with`, `ensure_object`-Rollback, zwei
  Gate-Tests), `src/archivar/netcdf.rs` (`nc4_group` über `Hdf5Access`),
  `tools/harvest/src/bin/icesat2_atl03_compiler.rs` (Pagination + Migration),
  `tools/harvest/src/bin/gedi_l2a_compiler.rs`,
  `tools/harvest/src/bin/swot_l2_lr_ssh_compiler.rs`,
  `tools/harvest/src/bin/cosmic_ro_compiler.rs`,
  `tools/harvest/src/bin/wod_compiler.rs`,
  `docs/handover/handover-2026-09-19-bau-folge93.md` (neu), Move
  `handover-2026-09-19-bau-folge92.md` → `archiv/`.
- **Fremd (nicht anfassen):** die `handover-2026-09-16-*`-Renames,
  `.github/workflows/hyperscanning-te.yml`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
