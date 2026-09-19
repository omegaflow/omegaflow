<!--
  title: Handover — Bau-Folge 94 (Stand 2026-09-19)
  session: Bau-Folge 94
  class: handover
  date: 2026-09-19
  sha256: f8a9a87435b2d7f5842d690f0abe07256c580ed357825c8b719bdb7ff1a87f65
  status: live
-->
# Handover — Bau-Folge 94 (2026-09-19)

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

- **HEAD** Session-Beginn `d9dc64e7` (== `origin/main`); während der Session von
  fremden Linien auf `d1750fe0` (research folge97) gezogen (== `origin/main`).
  `git_safety` Snapshot `refs/safety/1789849746`.
- **Postfach** leer (`post.md` nur Header, keine Bau-Zeile); Zustand-Eintrag
  zitiert: letzter Ledger-Eingang `1789795811`, kein neuer seit Folge 54.
- **CI** (`ci_manage list`/`view`, ~20:41Z): `ci-check 35468161696` pending
  @`d1750fe0`, `hyperscanning-te 35468144989` pending @`5219db7e`; der folge93-Gate-
  Lauf `ci-check 35467445389` @`d9dc64e7` wurde **cancelled** (Konkurrenz-Push),
  nicht ausgeführt; `te-gate 35462518676` pending @`3d2e6adb`.
- **Arbeitsbaum** fremd uncommittet (nicht angefasst): ernte98-Pfade
  (`docs/handover/handover-2026-09-19-ernte-folge98.md`, `post.md`, die fünf
  `phi/*.φ`, `openneuro*`, die drei `handover-2026-09-16-*`-Renames).

## Offen

Kein offener undatierter Punkt: der Cache-Punkt aus folge93
(`read_chunk_diag` ↔ `LazyHdf5`-Reader-Cache) ist gebaut — der Gate-Test
`lazy_chunk_read_reuses_the_index_window` und die Vereinheitlichung stehen im
Baum; die CI-Verifikation ist Wartestellung.

## Wartestellungen (kein Auswahlpunkt)

- **Gate `lazy_chunk_read_reuses_the_index_window` + folge93-Gates** —
  `lazy_resolution_fetches_only_the_requested_path`,
  `lazy_rollback_retries_a_failed_datatype_chase`, die drei Pagination-Tests und
  der neue Cache-Gate laufen im `test`-Job von `ci-check` (`cargo test --release`)
  am Push dieses Commits. Auslöser: der `ci-check`-Lauf am Push. · `wartend`
- **TE-Gates Multi-Seed-Umbau CI-Verifikation** (aus folge92) — derselbe
  `ci-check`-Lauf. · `wartend`
- `te-gate 35462518676` pending @`3d2e6adb` · `wartend`
- planetary-odf-cdn `35351411938` · `wartend`
- Scanned-/bild-only-PDFs → `vision`-OCR · `pending`

## Benchmark

- **hdf5 Chunk-Index-Cache** (harte Klasse: Struktur-Urteil + Schreiben):
  Messung `grind-flash` — `synthetic_chunked_image`-Helfer + Gate-Test, der den
  B-Tree-Fetch über zwei Chunk-Reads zählt; vor dem Fix rot. Vereinheitlichung
  `grind-max` — `chunk_index_cache: HashMap<u64, (Vec<ChunkRec>, bool)>` in
  `LazyHdf5`, `read_chunk_resolved` in `chunk_read_plan` + `chunk_values_from`
  zerlegt; der persistente `self.reader` bleibt dem Objekt-Header vorbehalten
  (Fetch-Budget-Parität), nur die kleinen Index-Records werden gecacht. Dazu
  head-first-Traversal (v1/v2): ein Knoten-Start wird einmal geholt statt
  überlappend. `cargo check -p omegaflow --all-targets` 0 Fehler / 0 Warnungen.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/archivar/hdf5.rs`,
  `docs/handover/handover-2026-09-19-bau-folge94.md` (neu), Move
  `handover-2026-09-19-bau-folge93.md` → `archiv/`,
  `docs/zustand/external-state.md` (nur die CI-Status-Zeile).
- **Fremd (nicht anfassen):** ernte98-Pfade, die drei
  `handover-2026-09-16-*`-Renames, `.github/workflows/hyperscanning-te.yml`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
