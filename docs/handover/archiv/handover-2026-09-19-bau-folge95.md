<!--
  title: Handover — Bau-Folge 95 (Stand 2026-09-19)
  session: Bau-Folge 95
  class: handover
  date: 2026-09-19
  sha256: c0b445087bdc1c6ec21a7439f7c7b6006cc49dfde403f5c9f3df5bcd2e2b9420
  status: live
-->
# Handover — Bau-Folge 95 (2026-09-19)

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

- **HEAD** `7aa5c23e` (== `origin/main`). `git_safety` Snapshot
  `refs/safety/1789850913`.
- **Postfach** — `state/mail/mail_ledger.φ` lokal abwesend (kein lokaler Ledger
  in diesem Checkout); kein neuer Eingang messbar. Zustand-Eintrag zitiert:
  letzter Ledger-Eingang `1789795811`, zuletzt gemessen von Entscheid-Folge 55.
- **CI** — Zustand-Eintrag `docs/zustand/external-state.md` zitiert (Ernte-Folge
  99, `ci_manage list` @`7aa5c23e`): **pending** `ci-check` `35468441157`
  @`7aa5c23e`, `hyperscanning-te` `35468144989` @`5219db7e`; **in_progress**
  `openneuro-cdn` `35468606830`, `harvest` `35467466676`, `placebo-ave-cdn`
  `35468313922`, `ned-cdn` `35468572091`, `ps1-cdn` `35468039021`.
- **Arbeitsbaum** fremd uncommittet (nicht angefasst): die drei
  `handover-2026-09-16-*`-Renames (alt gelöscht, neu in `archiv/`).

## Offen

- **Scanned-/bild-only-PDFs → `vision`-OCR** (aus folge94 getragen). Ein
  bild-only-PDF trägt keinen Textlayer; die Figuren/Zahlen fehlen dem
  Paper-Bestand. (Schritt: die bild-only-PDFs im `docs/paper/`-Bestand messen —
  `archive_search --sniff` auf die PDFs, kein Textlayer ⇒ `vision` liest Figur/
  Tabelle als Text.) · `pending`

## Wartestellungen (kein Auswahlpunkt)

- **`archive_search --sniff` Partial-Hash-Fix** — der Gate-Test
  `sniff_lines_marks_a_partial_download` (`tools/utils/src/bin/archive_search/net.rs`)
  läuft im `test`-Job von `ci-check` `35468441157` @`7aa5c23e` am Push dieses
  Commits. Auslöser: der `ci-check`-Lauf am Push. · `wartend`
- **Gate `lazy_chunk_read_reuses_the_index_window` + folge93-Gates** — derselbe
  `ci-check`-Lauf. · `wartend`
- **TE-Gates Multi-Seed-Umbau CI-Verifikation** (aus folge92) — derselbe
  `ci-check`-Lauf. · `wartend`
- `te-gate 35462518676` pending @`3d2e6adb` · `wartend`
- planetary-odf-cdn `35351411938` · `wartend`

## Benchmark

- **`--sniff` Partial-Hash** (Klasse: Routine-Mechanik — Struct-Feld, Branch,
  Gate-Test): `grind-flash`. Die Routine-Agent-Klasse ist gemessen geschlossen
  (2026-09-16: flash $0.0008–0.0017 gegen pro/max $0.0041–0.0090 bei identischem
  Ergebnis); kein Doppellauf. `cargo check -p omegaflow-utils --all-targets`
  0 Fehler / 0 Warnungen. Der `Fetch` trägt `complete: bool` aus dem curl-Exit;
  `sniff_lines_from` meldet `sha256 partial <hash> (download incomplete, N bytes)`,
  nie einen Teil-Hash als vollen.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `tools/utils/src/bin/archive_search/net.rs`,
  `docs/handover/handover-2026-09-19-bau-folge95.md` (neu), Move
  `handover-2026-09-19-bau-folge94.md` → `archiv/`.
- **Fremd (nicht anfassen):** die drei `handover-2026-09-16-*`-Renames,
  `docs/zustand/external-state.md` (die CI-Zeile trägt bereits Ernte-Folge 99
  @`7aa5c23e`), ernte99-Pfade.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
