<!--
  title: Handover — Bau-Folge 41 (Stand 2026-09-16)
  session: Bau-Folge 41
  class: handover
  date: 2026-09-16
  sha256: 16cbff7950e5b43c3f278b62cb9c9974cd9b56d90b93e9130c8750444a26b018
  status: live
-->
# Handover — Bau-Folge 41 (2026-09-16)

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

## TE-Gate — Fix im Baum, n=1000-Gate + residual/ksg-Tail offen

- Der Fix `gate_fpr_cells_from` (ganze z→y-Familie als causal candidate) liegt
  uncommittet in `src/mathematikerin/te.rs` — innerhalb der fremden WIP-Grenze
  (s. nächster Abschnitt). Offen: den Fix landen und die n=1000-Gates in
  `te-gate.yml` laufen lassen; bleibt residual/ksg > 8 %, den residual-Null-Tail
  als eigenes Atom bauen — NICHT den 8-%-Schnitt senken. CI-Issue 13 offen.
  (Schritt: `te-gate.yml` CI-Lauf, dann Commit der te.rs-Hunks nach WIP-Klärung.)
- Benchmark flash vs. max auf diesem Atom: der max-Lauf wurde mid-Atom
  abgebrochen (nur `diag_*`-Scaffolding, kein Fix) → nicht gewertet.
  (Schritt: grind-max auf dieselbe Wurzel-Diagnose neu ansetzen.)

## Arbeitsbaum-Grenze — fremde uncommittete WIP blockiert src/-Commits

- Gemessen 2026-09-16: 114 Dateien uncommittet (`src/archivar/*`,
  `src/mathematikerin/*` inkl. `te.rs`, `src/gate/commit_gate.rs`, `tools/harvest`,
  `tools/measure`). `cargo check -p omegaflow` ist grün (0 Fehler, 0 Warnungen),
  `cargo fmt --all --check` ist rot (Import-Ordnung in vielen Dateien). Offen: die
  WIP fertigstellen/committen oder verwerfen lassen (Urheber-Linie/Operator), sonst
  ist kein pfad-begrenzter src/-Commit möglich. (Schritt: Urheber-Linie benennen,
  dann `git add <Datei>` je Atom.)

## Parser-Gap A — DRS-Registrierung entblockt; TNF/ODF-Konsument offen

- DRS-FITS: sources.φ-Feldabbildung für das 3-Vektor-Δg. (Schritt:
  `phi/blocked_sources.φ` `drs-fits` entblocken → `phi/sources.φ`-Block mit dem
  Vektor-Feld-Schema → CI-CDN.)
- TNF: `tnf_compiler.rs` gebaut; offen echter Lauf + Registrierung +
  Membran-Konsument (Operator).
- ODF Juno/Magellan/MGS/MRO/Odyssey/MESSENGER/Mars Express/Rosetta + ODR Voyager:
  gebaut, Konsument offen (Operator).

## Register-Digest — Bau-Linie

- `--live`-Namensstimme (Operator: `--open` oder bleibt); der sichtbare
  Überseh-Korpus gehört je in seine Linie.
- Legacy-Repo-Run gemessen: `register_lookup --history --legacy
  <archive-root>/omegaflow-legacy` → 1159 Treffer.
- Gate-Fixture `measure again without a due` + Gate-Test fehlen; `commit_gate.rs`
  liegt in fremder WIP. (Schritt: nach WIP-Klärung Fixture in
  `commit_gate_vocab.json` + Test in `commit_gate.rs`.)

## CI — rot auf b2e5ac7

- `format` (fmt-Drift), `clippy`, `test` (3: `src/archivar/omni2.rs:83`,
  `src/archivar/tests.rs:2805`, `src/mathematikerin/te.rs:3723`),
  `esp32-firmware` (`xtensa_lx`), `te-gate` #13 (n=1000 FPR), `number_audit`
  (`docs/specs/bekannt-schlecht-korpus.md`). `cargo fmt --all` erst nach Klärung
  der fremden WIP. (Schritt: je Punkt das genannte Kommando/Datei.)
- `archive_search`-Flags (`--count`/`--case`/`--path`): lokaler
  `cargo check -p omegaflow-utils --bin archive_search` grün (2026-09-16); offen ist
  die CI-Verifikation des Release-Binärs (Session-Build lokal verboten). (Schritt:
  CI-Lauf, dann liegt das Binär mit den Flags vor.)

## Code/Infra

- HRV/Puls-Oszillator-Bindung: physischer ESP32-Träger (on hold) + ein
  End-zu-End-Test.
- 40-byte bins recompilation; mirror-research ~2.300-Quellen-Migration; GLO-30 DEM;
  4d-membrane-Archäologie. (Schritt: `docs/concepts/archivar-mathematikerin.md` bzw.
  `mirror-research.md`.)

## RINEX/CORS, Binding, Mail-Fang, Membran, Papier (Operator)

- RINEX/CORS: Konsument benennen (Parser grün, `cors_compiler`/`cors-cdn.yml` stehen).
- Binding: Konsument-Benennung; kein sources.φ-Block ohne Membran-Konsument.
- Mail-Fang: `wrangler kv namespace create MAIL_QUEUE` → Id eintragen → `wrangler deploy`.
- ESP32-Modul: on hold; BOM `docs/specs/mantis-shrimp-bom.md`.
- 20-s-Bande-Papier: per-Papier-Release-Tag und Welt-Fassung-Branch absent.
  (Schritt: `git tag` + `git branch` anlegen.)

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
