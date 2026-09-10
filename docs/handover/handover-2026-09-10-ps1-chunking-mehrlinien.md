<!--
  title: Handover — PS1-Chunking + Korpora-Heim + vier weitere Linien (Stand 2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: bb50d314404a04ee5ed8b8f12078a7943675dacc8524514ca004d012df56b5ea
  status: live
  see-also: docs/handover/handover-2026-09-10-nicht-autonom.md, docs/handover/handover-2026-09-10-autonom.md
-->
# Handover — PS1-Chunking + Korpora-Heim + vier weitere Linien (2026-09-10)

Eine Session, ein Atom: die Session nahm fünf Linien der 09-10-Handover auf und
baute nach der Operator-Ratifizierung („du kannst") das Korpora-Heim. Jede Linie
trägt einen gebauten Stand oder eine Messung. Der Korpora-Teil wurde in fünf
isolierten Commits auf origin/main getragen (nur der Korpora-Teil, kein Fremdes);
der Baum trägt daneben ungecommittete Arbeit einer parallelen Session, die diese
Session nicht angefasst und nicht committet hat. Die konsumierten Handover
bleiben `live` — sie tragen viele unberührte Linien weiter.

## PS1-Chunking — gebaut, verifiziert

- `ps1_coverage_compiler.rs` — `--sub-min` (Subcell-Bereich je Chunk) + ehrlicher
  Leer-Chunk-Schrieb (0 Records, kein „0 honored"-Abbruch); `chunked` = Bereich
  enger als 0..99.
- `ps1_coverage_combiner.rs` — `parse_part` erkennt `ps1_part_<band>_<s>_<e>.fp01`;
  Vollständigkeit = Subcell-Kachelung 0..99 je Band; 3 neue Tests (Kachelung,
  ungetilete Verweigerung, Leer-Chunk-Toleranz).
- `.github/workflows/ps1-cdn.yml` — ein Subcell-Chunk je Schritt, Deadline zwischen
  Chunks, `finished_all`-Tor vor dem Combine.
- **Verifikation:** nachdem die Parallel-Session `noaa_nodd` in `src/archivar/mod.rs`
  verdrahtet hat, kompiliert die Lib wieder. `cargo check` (RUSTFLAGS="-D warnings"):
  0 Fehler, 0 Warnungen. `cargo test -p omegaflow-harvest --bin ps1_coverage_combiner`:
  10 passed, 0 failed (inkl. der 3 neuen Chunking-Tests).

## ESO-TAP — gebaut, uncommittet, unmanifestiert

- `.github/workflows/eso-harps-rvcat-cdn.yml` (untracked) + `phi/sources.φ`-Block
  (`harps_rvcat.json`; Pilot `safcat.HARPS_RVCAT_V1`, 277.846 Zeilen mit rv+plx).
  Gemessen: `tap_cat` verlangt FORMAT=json, `tap_obs` FORMAT=csv; Tabellen sind
  schema-qualifiziert (`safcat.*`).
- **Unmanifestiert:** das Release trägt kein `harps_rvcat.json`; das Workflow ist
  untracked, die CI hat es nie gesehen. Der sources.φ-Block zeigt auf ein Asset,
  das erst nach grünem Workflow-Lauf existiert — die url-Zeile ist die
  Registrierung für die Manifestation, nicht ein Anwesend-Sein.
- **Lizenzverdikt offen** (CC BY 4.0 unverifiziert — nicht als Tatsache ins
  Register geschrieben). Register-Duty: das Verdikt messen oder `pending` tragen.

## matrixmachine-Urkunde — re-basiert

- Letzter grüner `ci-check.yml`-Lauf: **253945bf** (2026-09-08). Die
  Geburtsurkunden-Zeile (archive-root) trägt jetzt diesen Lauf — zuvor ein
  lokaler 02a46d3 (gegen „kein lokaler Lauf"). Aktueller HEAD e87894b:
  `in_progress`, nicht grün.

## flyby2-addendum — aufgelöst

- Addendum `status: live` + Operator-Siegel seit 03.09.; die Kalender-Zeile
  „Metrik vor dem 28.09." ist erfüllt. Offen bleibt das AGU-2013-Abstract
  (Anderson) — Operator-Wort.

## abfluss-trishuli — gemessen, Abfluss-Reihe absent

- Das CSV trägt Pegel (m), nicht Abfluss (m³/s). Der Regen→Pegel-Pfeil bei
  präregistriertem Lag 24 h ist Stille (0.2232 nats < Schwelle 0.2310). Die
  Abfluss-Reihe (m³/s) fehlt → der Abfluss-Pfeil bleibt `pending`, nicht 0.0.

## Korpora-Heim — ratifiziert, gebaut, manifestiert

- Operator ratifiziert („du kannst"). Verdikte gemessen (curl + Browser):
  9 `redistribute` (pangaea, re3data, zenodo, dryad, wdcc, dataverse×2, seanoe),
  5 `decline` (b2find/EUDAT), 2 `pending` (cmr — CMR aggregiert Partner-Metadaten;
  dataone — terms-Seite broken). Übrige Korpora: `pending`, nicht 0.0.
- Gebaut: `korpora_heim.φ` (Verdikt-Register), `korpora-cdn.yml` (Workflow),
  `MANIFEST.φ`-Eintrag, `.gitignore`-Ausnahmen.
- Manifestiert: die 9 `redistribute`-Korpora liegen als `.phi`-Assets im
  CDN-Release ssd.jpl.nasa.gov (ASCII-Namen — `gh release upload` legt eine
  `.φ`-Datei sonst als `default.*` ab; der Workflow staged daher unter ASCII-Basename).
- Commits (nur Korpora-Teil, isoliert auf origin/main): dd34893, 432a673,
  4e81143, 7b0400f, 78cd619.
- Offen: cmr + dataone (Verdikt-Messung), der Discovery-Download-Workflow.

## Parallel-Session — benannt, nicht angefasst

- Der Baum trägt ungecommittete Arbeit einer parallelen Session: `noaa_nodd.rs`
  (wurde während dieser Session aktiv editiert), supermag, vo-tap, mehrere
  Harvest-Compiler, `docs/handover/handover-2026-09-10-disposition-hygiene.md`.
  Diese Session hat keine dieser Dateien angefasst; committet wurden nur die
  Korpora-Dateien (isoliert, siehe Korpora-Heim).
