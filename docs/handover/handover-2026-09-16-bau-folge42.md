<!--
  title: Handover — Bau-Folge 42 (Stand 2026-09-16)
  session: Bau-Folge 42
  class: handover
  date: 2026-09-16
  sha256: d7371c7dd0ea99cab25da1f36182fe13282c7fbaca7426c17df135cee21fadd6
  status: live
-->
# Handover — Bau-Folge 42 (2026-09-16)

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

## Clippy gegen den 0-Kanon — 36 Funde unter `-D warnings`

- Der WIP-Lande-Commit `ee4db3aa` hat `format` und `build` gegrünt (ci-check
  `35079185066`), `clippy` bleibt rot. Gemessen (`gh run view 35080299174
  --log-failed`): 36 clippy-Fehler unter rust 1.98 + `-D warnings` (die
  setup-Action setzt RUSTFLAGS), lokal ist 1.96. Die Klassen: `manual_unwrap_or`
  auf den `match { Some(v) => v, None => 0.0 }`-Formen, die das Gate erzwingt
  (`src/archivar/fits.rs:540,566,792`, `src/mathematikerin/te.rs:1908` u. a.);
  `chunks_exact_to_as_chunks` (`src/archivar/netcdf.rs:459`,
  `src/archivar/tiff.rs:1562`); `manual_memcpy` (`src/archivar/las/laszip.rs:144,715,728`);
  byte-str (`src/archivar/netcdf.rs:3`). Offen ist die Form: benannte
  quellen-definierte Default-Konstante (FITS TZERO/BZERO = 0.0, TIFF = 1) statt
  Literal, Option-Plumbing, oder eine Projekt-Lint-Politik — die Gate-Fixture
  bleibt unberührt. (Schritt: `gh run view 35080299174 --log-failed`, je Fund die
  Form wählen, `cargo clippy --all-targets -- -D warnings` lokal grün, committen.)

## TE-Gate — Fix committet; n=1000-Lauf offen

- Der z→y-Familien-Fix ist committet (`04dcbdc5`); der n=1000-Lauf läuft als
  te-gate `35079194274` (workflow_dispatch). Lokal gemessen
  (`/tmp/opencode/n1000_new.log`, 6180 s): `residual/ksg` 25,77 %,
  `shift/ksg` 10,20 %, `restricted/binned` 8,93 % — alle > 8 %. (Schritt: den
  Lauf `35079194274` lesen; bleibt der Tail > 8 %, den residual-Null-Tail als
  eigenes Atom bauen — NICHT den 8-%-Schnitt senken.)

## esp32-firmware — Linker statt xtensa_lx

- Der Workflow baut jetzt aus `firmware/radiatorium`; der Lauf `35079185142`
  scheitert am Linker: undefined `CACHE_CORE0_ACS`, `CACHE_CORE1_ACS`,
  `USB_DEVICE`, `PERI_BACKUP`, `DMA_EXTMEM_REJECT` — esp-hal/ROM-Symbole fehlen
  (Versions-/Feature-Mismatch esp-hal ↔ esp-15.2.0). (Schritt:
  `firmware/radiatorium/Cargo.toml` esp-hal-Version/Features gegen
  `firmware/radiatorium/rust-toolchain.toml` prüfen.)

## Parser-Gap A — DRS-Registrierung; TNF/ODF-Konsument offen

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
- `number_audit` wird von keiner CI-Workflow aufgerufen (im Vorhandover als
  CI-Punkt geführt) — lokales Werkzeug, kein CI-Gate.

## CI — Rest

- `test`: der Lauf `35079185066` wurde durch `concurrency.cancel-in-progress`
  (zweiter Push) abgebrochen, kein Testfehler; der Lauf `35080299174` trägt den
  Test-Job. (Schritt: `gh run view 35080299174`.)
- `archive_search`-Flags (`--count`/`--case`/`--path`): offen ist die
  CI-Verifikation des Release-Binärs. (Schritt: CI-Lauf.)

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

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
