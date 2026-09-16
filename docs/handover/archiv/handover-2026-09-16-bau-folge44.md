<!--
  title: Handover — Bau-Folge 44 (Stand 2026-09-16)
  session: Bau-Folge 44
  class: handover
  date: 2026-09-16
  sha256: 0e6c169b8345934d5377d0daeb551d23ece18110b60fb351967a7bfed52a8763
  status: live
-->
# Handover — Bau-Folge 44 (2026-09-16)

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

- CI-Lauf `35079194274` gelesen (2026-09-16): 6 grün / 3 rot — residual/ksg
  a=0.9 **12,36 % > 8 %**, restricted/binned rise 4,12 pp, block/binned rise
  2,75 pp. Council-Verdikt: nicht die Null wechseln, sondern die
  Entscheidungsregel — empirischer Rangtest `p = (1 + #{s ≥ te})/(B + 1)`,
  Threshold = empirisches α-Quantil über `surr ∪ {te}`, in `pcmci_links`
  (`src/mathematikerin/te.rs`); `n_surr` 100 → 200 (Boden 1/201), 8-%-Kriterium
  und Kalibrier-Gate unangetastet. Gebaut, `cargo check` 0/0. (Schritt: nach dem
  Push `gh workflow run te-gate.yml`, dann den Lauf lesen — bleibt residual/ksg
  > 8 %, den gemessenen FPR-Boden als Register-Zeile tragen, kein weiterer
  Null-Wechsel ohne Messung. Benchmark: hartes Atom → `grind-max`, kein
  flash-Gegenlauf; Mechanik `frame_anchor` → `grind-flash`.)

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
