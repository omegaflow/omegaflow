<!--
  title: Handover — Bau-Folge 46 (Stand 2026-09-16)
  session: Bau-Folge 46
  class: handover
  date: 2026-09-16
  sha256: 45d34ae09b3113187fceff0a3dfcdbb4bf7038349c1e50b2da233af033f0d0ba
  status: live
-->
# Handover — Bau-Folge 46 (2026-09-16)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

## ODR-Roh-Sample-Dekodierung — härtester undatierter Punkt

- `voyager_odr` (`src/archivar/voyager_odr.rs`) und `galileo_odr`
  (`src/archivar/galileo_odr.rs`) tragen rohe Sample-Wörter (`VODR` 16-bit;
  `ad: [[u8; 4]; 625]`). Fehlender Fakt: das RSC-11-6 Sample-Wort-Layout bzw. die
  AD-Byte→physikalischer-Wert-Kanal-Semantik + Epochen-Anker (Jahrfeld 7-bit).
  `sample_count`-Semantik offen (Spec ≤ 299999 vs. gemessen ~4,29e9, Offset 52).
  (Schritt: RSC-11-6 / PDS3-Label lesen — kein 0.0.)

## Epochen-Anker-Formate (Compiler muss den Anker tragen)

- `GED1`/`AT31`/`SWS1` tragen `delta_time` (anker-relativ); der absolute
  Granule-Anker steht nicht im Bin. `NCDO` trägt Unix-Sekunden, nicht TDB.
  Ein Reader ohne Anker bliebe `pending`. (Schritt: Compiler um den absoluten
  Anker / `unix_to_tdb` erweitern.)

## TE-Gate — Verifikation (CI, dispatcht)

- `te-gate` run `35092997862` war `in_progress` (Schritt 4, shift-null sweep);
  residual/ksg noch nicht gemessen. (Schritt: Lauf lesen; bleibt residual/ksg
  > 8 %, den gemessenen FPR-Boden als Register-Zeile tragen. Kein Polling.)

## CI — Rest

- `ci-check` run `35093194650` test-Job `in_progress`; der jüngste abgeschlossene
  Fehlschlag (`35090593186`, @b109f105) war der `src/archivar/eea.rs:69`
  i128-Shift-Compile-Fehler, vom Ernte-Commit `625452e5` gefixt.
  `archive_search --count/--case/--path` am Release-Binär verifiziert
  (2026-09-16). (Schritt: test-Job lesen.)

## 40-byte bins — Star-Katalog

- `dr3_stars.bin` am CDN ist 40 B; Loader/Compiler verlangen 44 B (40 B + f32 rv),
  sonst bleibt der Katalog dunkel (`src/archivar/spatial.rs:246`,
  `tools/harvest/src/bin/tap_compiler.rs:403`). (Schritt: `tap_compiler --ci-mode`
  mit rv-Spalte rekompilieren + manifestieren.)

## mirror-research — gemessen

- „~2.300-Quellen" ist ungemessen; gemessen (`docs/specs/cdn_reconciliation.json`):
  412 Quellen, 203 Live-Releases, 67 Source-Netlocs, 200 Release-Netlocs. Die
  25-Mirror-Frage ist geschlossen (1/25). (Schritt: Umfang gegen
  `docs/specs/cdn-ziel-schema.md` §1 messen; das `mirror-research.md`-Zitat ist
  für die breite Migration falsch.)

## DRS-FITS — Arm steht, Asset offen

- Der Arm (`src/archivar/drs_fits.rs`, flat series, UTC-Epoche im Bin-Header) ist
  gebaut; `cargo check` 0/0. Offen: kein DRSF-Bin am CDN (Dispatch
  `.github/workflows/drs-fits-cdn.yml`), Index-Zeit-Drift bei Reihen-Lücken
  (24-B-Stride trägt keine per-Row-Zeit), Anker `at earth` (kein LPF-Ephemeridenkörper).
  Benchmark: `grind-pro` fabrizierte `t/lat/lon = 0.0`, `grind-max` erkannte und
  entfernte — nach den zwei Messungen (TIME = UTC, keine Orbit-Spalten) ist der
  flat-series-Arm der gemessene Weg. (Schritt: CDN-Dispatch mit gemessenem Granulat.)

## Code/Infra (gemessen — 12×-Wortgleich-Block aufgelöst)

- GLO-30 DEM: CDN-Dispatch 2026-09-14 gemessen fertig (Asset HTTP 200); offener
  Konsument = S1-Layover-Korrektur. 4d-membrane-Archäologie: PCK/GM/presence_probe/
  WGCCRE gebaut bzw. entfernt — gestrichen. `docs/concepts/positive-maske.md:51`
  trägt noch die veraltete „pending"-Zeile (Stand 09-12).

## HRV/Puls-Oszillator-Bindung

- Physischer ESP32-Träger (on hold) + ein End-zu-End-Test.
  (Schritt: `src/archivar/hrv.rs`.)

## Mail-Fang, Membran, Papier (Operator)

- Mail-Fang: `wrangler kv namespace create MAIL_QUEUE` → Id eintragen →
  `wrangler deploy`.
- ESP32-Modul: on hold; BOM `docs/specs/mantis-shrimp-bom.md`.
- 20-s-Bande-Papier: `git tag` + Welt-Fassung-Branch absent. (Schritt: anlegen.)

## Register-Digest — Bau-Linie

- `--live`-Namensstimme (Operator: `--open` oder bleibt). Legacy-Repo-Run:
  `register_lookup --history --legacy <archive-root>/omegaflow-legacy` → 1159 Treffer.

## Post / Format-Gate

- `docs/handover/post.md` trägt fremde uncommittete Hunks („An entscheid"); meine
  gefalteten „An bau"-Zeilen (Serien-Leser, ODR) und die DRS-FITS-Zeile wurden
  **nicht** gelöscht, um fremde Arbeit nicht zu überschreiben. (Schritt: nach dem
  fremden Commit die drei Zeilen entfernen.)
- CI-`format` rot @625452e5 — fremde unformatierte Dateien
  (`src/gate/commit_gate.rs:540`, `tools/measure/src/bin/aia_ladder_probe.rs`,
  `trishuli_gauge_probe.rs`, `tools/utils/src/bin/archive_search.rs`).
  (Schritt: rustfmt-Diff aus `ci-check` run `35091175017`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
