<!--
  title: Handover — Bau-Folge 43 (Stand 2026-09-16)
  session: Bau-Folge 43
  class: handover
  date: 2026-09-16
  sha256: e27ce0cbad5a13c4fe23f70992c175586b000df2ff9c86c780a0759fdb2c1a51
  status: live
-->
# Handover — Bau-Folge 43 (2026-09-16)

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

## TE-Gate — residual-Null-Tail (härtester undatierter Punkt)

- Der n=1000-Lauf `35079194274` läuft noch (Start 2026-09-16, timeout 360 min).
  Lokal mit dem z→y-Fix (`04dcbdc5`) gemessen (`/tmp/opencode/n1000_new.log`,
  6180 s): residual/ksg 25,77 %, shift/ksg 10,20 %, restricted/binned 8,93 % —
  alle > 8 %. (Schritt: den Lauf `35079194274` lesen; bleibt der Tail > 8 %, den
  residual-Null-Tail als eigenes Atom bauen — NICHT den 8-%-Schnitt senken.)

## Parser — Frame-Anker-Fabrikation in `fetch.rs`

- `frame_anchor` (`src/archivar/fetch.rs:1151`) fällt für `Manifest`/`Barycenter`
  auf `_ => (0.0, 0.0)` — ein fabrizierter Koordinaten-Anker (Golf von Guinea),
  gelesen in `src/archivar/port.rs:2064`/`:2129`. Die Gate-Fixture
  `_ => (0.0, 0.0)` + Test stehen (`src/gate/commit_gate_vocab.json`,
  `commit_gate.rs`); der Code ist noch nicht geheilt. (Schritt: `frame_anchor` auf
  `Option<(f64, f64)>` umstellen und die zwei `port.rs`-Aufrufer den Anker-losen
  Fall überspringen lassen — kein Default.)

## Parser-Gap A — DRS-FITS

- DRS-FITS: sources.φ-Feldabbildung für das 3-Vektor-Δg. (Schritt:
  `phi/blocked_sources.φ` `drs-fits` entblocken → `phi/sources.φ`-Block mit dem
  Vektor-Feld-Schema → CI-CDN.)

## esp32-firmware — Linker: Root-Cause gemessen, Fix in CI-Verifikation

- Root-Cause (gemessen 2026-09-16): die `setup-rust-toolchain`-Action setzt
  `RUSTFLAGS="-D warnings"` und verdrängt damit die `build.rustflags` aus
  `firmware/radiatorium/.cargo/config.toml` (`-Tlinkall.x -nostartfiles`) — der
  Link fällt auf die ROM-Symbole zurück. Fix: `rustflags: ""` in
  `.github/workflows/esp32-firmware.yml`. (Schritt: `gh workflow run
  esp32-firmware.yml` und den Lauf lesen; bleibt der Link rot, die verbose
  Link-Zeile auf `-Tlinkall.x` prüfen.)

## CI — Rest

- `clippy` grün; der `test`-Job des Laufs `35081527096` wurde von der Concurrency
  abgebrochen (gemessen 2026-09-16), nicht rot. (Schritt: den `test`-Job des
  jüngsten `ci-check`-Laufs lesen.)
- `archive_search`-Flags (`--count`/`--case`/`--path`): offen ist die
  CI-Verifikation des Release-Binärs. (Schritt: CI-Lauf.)

## Register-Digest — Bau-Linie

- `--live`-Namensstimme (Operator: `--open` oder bleibt); der sichtbare
  Überseh-Korpus gehört je in seine Linie.
- Legacy-Repo-Run gemessen: `register_lookup --history --legacy
  <archive-root>/omegaflow-legacy` → 1159 Treffer.

## Code/Infra

- HRV/Puls-Oszillator-Bindung: physischer ESP32-Träger (on hold) + ein
  End-zu-End-Test.
- 40-byte bins recompilation; mirror-research ~2.300-Quellen-Migration; GLO-30 DEM;
  4d-membrane-Archäologie. (Schritt: `docs/concepts/archivar-mathematikerin.md` bzw.
  `mirror-research.md`.)

## Mail-Fang, Membran, Papier (Operator)

- Mail-Fang: `wrangler kv namespace create MAIL_QUEUE` → Id eintragen → `wrangler deploy`.
- ESP32-Modul: on hold; BOM `docs/specs/mantis-shrimp-bom.md`.
- 20-s-Bande-Papier: per-Papier-Release-Tag und Welt-Fassung-Branch absent.
  (Schritt: `git tag` + `git branch` anlegen.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
