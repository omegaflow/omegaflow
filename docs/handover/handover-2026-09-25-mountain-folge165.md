<!--
  title: Handover — Mountain-Folge 165 (2026-09-25)
  session: Mountain-Folge 165
  class: handover
  date: 2026-09-25
  sha256: d1a71b0dfd77a52574c9cb9b5e0ccc9b414a738d894fcb1984e799ac9734446a
  status: live
-->
# Handover — Mountain-Folge 165 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Sortierung: erst Akteur (Linie | Rat | Operator |
Dritter), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
Trigger / Lage / Blockade / Braucht.

## Stehender Pass (gemessen zu Session-Beginn)

- **Postfach:** (gemessen 2026-09-25 via `smail` + `test -f`) `state/mail/` leer,
  `state/mail/mail_ledger.φ` present; keine fällige Post. Eintrag:
  `docs/zustand/external-state.md`.
- **CI-Status:** Watchdog-Snapshot 2026-09-25T21:04 gelesen: 7 aktiv
  (`tools-build`, `bz-retro-probe` queued; `ps1-cdn`, `source-census`,
  `kernel-flatten`, `ci-check`, `allwise-cdn` in_progress), 1 failed
  (`36163930996 ci-check`). Ergebnis beim nächsten Pass aus dem Snapshot, nie
  gepollt.
- **Sicherheitsnetz:** `git_safety --snapshot` → `refs/safety/1790365538`
  (recover: `git_safety --restore refs/safety/1790365538`).

## Offen (aufgeschlüsselt)

### Linie handelt (eigen)

#### `phi/blocked_sources.φ::gap:unit-auto-detect ×327`
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `sgrep -c`) der Arm steht
  (`unit_from_name_suffix`, `units.rs:169`); **live 327** Einträge tragen das
  Token — der Träger nannte ×166 (Count-Drift gemeldet, kein Orphan; die Zahl
  stammt aus einer älteren Messung).
- **Blockade:** keine
- **Braucht:** grind-flash: Re-Port der 327 Einträge über den stehenden Arm; das
  Token fällt pro Eintrag. Trägerform `phi/blocked_sources.φ::gap:unit-auto-detect ×327`.

#### Klasse-5 offene Routen bauen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `archive_search`, siehe
  `docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md`) vier
  Stellen offen: S3-Scheme (200 mit Token), ODF TRK-2-34/TNF (offener Korpus,
  Parser-Arm fehlt), AMS-02 TDAT (HEASARC live, Reader fehlt), Parquet/GRIB-2/
  OPeNDAP (Reader fehlen).
- **Blockade:** keine
- **Braucht:** TRK-2-34/TNF-Parser (`odf.rs`), TDAT-Reader, Parquet-/GRIB-2-/
  DAP2-Reader; AMS-02 als `live` registrieren.

#### gll.rss PDS4-Bundle — ATDF/ODR/TRK-2-34/RSR ernten
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `archive_search --sniff` + grind-pro) das
  PDS4-Bundle `pds-rings.seti.org/pds4/bundles/gll.rss/gll.rss.raw/` ist HTTP 200:
  `data_trk225_atdf/` (240, TRK-2-25 → `atdf.rs`), `data_rsc_11_11_odr/` (676,
  RSC-11-11 → `galileo_odr.rs`, series `galileo_odr`), `data_trk234_trknav/` (2,
  TRK-2-34 → `odf.rs::tnf_rows`/`lro_utf.rs`), `data_0159_sci/` (7, RSR →
  `cassini_rsr.rs`). **Die Arme stehen, die gll.rss-Compiler fehlen:** kein
  Compiler liest das gll.rss-Bundle — `gll_ck_manifestor.rs` (Galileo CK-Attitude,
  `naif.jpl.nasa.gov`), `lro_trk_compiler.rs` (LRO, `imbrium.mit.edu`),
  `maven_tnf_compiler.rs` (MAVEN, `pds-ppi.igpp.ucla.edu`) sind andere Missionen.
  `gll_ionocal_compiler.rs` ist bereits registriert (`sources.φ:8401`).
- **Blockade:** keine
- **Braucht:** je Sammlung einen gll.rss-Compiler (ATDF, ODR, TRK-2-34, RSR) auf
  den stehenden Armen bauen, in `phi/sources.φ` registrieren + `*-cdn`-Workflow,
  dann CI-Dispatch (Asset ist ein CI-Job).

#### TAO/TRITON — Compiler/Workflow committen + CI-Dispatch
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via grind-pro) die Source ist registriert
  (`phi/sources.φ:783`, `tao_wnd_zonal`, per-row `lat 3`/`lon 2`, `field 4
  tao_wnd_zonal_m_s patch-levy advective m/s 86400`); Compiler + Workflow liegen
  als untracked (`tools/harvest/src/bin/tao_wnd_compiler.rs`,
  `.github/workflows/tao-wnd-cdn.yml`); `phi/harvest.φ:203` trägt den Job.
- **Blockade:** keine
- **Braucht:** Compiler + Workflow + `phi/sources.φ`-Eintrag committen (im
  Session-Commit), dann `tao-wnd-cdn.yml` dispatchen (Asset ist ein CI-Job).

#### DECaPS-DR2 — Epoch-Arm (`epoch`-Direktive ohne `CelestialMap`)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via grind-pro) die `epoch`-Direktive hat keinen
  `CelestialMap`-Arm (`src/archivar/parse.rs:1041`: Map/KeplerMap/ProfileMap/Rows);
  der DECaPS-TAP-Block (`format tap` + `cmap`, `phi/sources.φ:9514`) kann
  `epoch_key` nicht setzen; der Konsument steht (`extract.rs:4374` →
  `Channel.epoch`). Register-Eintrag `parser-def tap` / `gap epoch-key` in
  `phi/blocked_sources.φ`; die TAP-Kolonnen HTTP 200 (`epochmean` MJD,
  `epochrange` Tage).
- **Blockade:** keine; `epochrange` hat keinen Wire-Slot, `cmap` kein `epoch_scale`.
- **Braucht:** `CelestialMap` in den `epoch`-Arm aufnehmen; danach `epoch_key
  epochmean` im DECaPS-Block und die MJD→TDB-Skala.

#### NAIF mariner10 — frame_registry-Route
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** frame_registry-Regeneration (`phi/pipeline/frame_registry.φ`).
- **Lage:** (gemessen 2026-09-25) `ephemeris_mariner10.bin` gebaut
  (`mariner10-ephemeris-cdn 36181036369`; 648 B, sha256 `7ad8b8bf…`, in
  `phi/sources.φ`); `phi/pipeline/frame_registry.φ` trägt keine mariner10-Route.
- **Blockade:** Register-Route fehlt.
- **Braucht:** Eintrag `naif.jpl.nasa.gov/pub/naif/M10/kernels/spk/M10_archive_1.bsp
  | at mariner10`.

### Operator handelt

#### Membran-Debug — Chrome DevTools MCP anbinden
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (Debugger-Rechte am laufenden Chrome)
- **Lage:** (gemessen 2026-09-25 via `chrome-devtools_list_pages`) der MCP
  antwortet `-32001 timeout` → nicht angebunden; der Pfad (i) bleibt „noch nicht
  angebunden" (`docs/concepts/tools-map.md`). Telemetrie-Flags
  `--no-usage-statistics` `--no-performance-crux` sind Bedingung.
- **Blockade:** das Operator-Wort + Chrome-Start mit den Debug-Flags.
- **Braucht:** Operator startet Chrome mit den beiden Flags/Remote-Debugging;
  dann MCP anbinden und in `docs/concepts/tools-map.md` registrieren.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
