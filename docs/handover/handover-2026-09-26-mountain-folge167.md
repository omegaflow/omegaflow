<!--
  title: Handover — Mountain-Folge 167 (2026-09-26)
  session: Mountain-Folge 167
  class: handover
  date: 2026-09-26
  sha256: 24bd82449c73e6e7223ba90b99404e9c966d3f95b15d9b34d5a588eeb53b3d35
  status: live
-->
# Handover — Mountain-Folge 167 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Sortierung: erst Akteur (Linie | Rat | Operator |
Dritter), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
Trigger / Lage / Blockade / Braucht.

## Stehender Pass (gemessen zu Session-Beginn)

- **Postfach:** (gemessen 2026-09-26 via `sread state/mail/mail_ledger.φ`)
  Ledger present (158 Z.), letzter Eingang **2026-09-16** (CSES-Limadou, Sotgiu:
  „wait a few weeks" → Wiedervorlage, kein neuer Akt). `mail_digest` meldet
  „ledger absent" — Pfad-Auflösung des Digest, nicht die Wahrheit. Eintrag:
  `docs/zustand/external-state.md`.
- **CI-Status:** Rate-Limit **erholt** (`ci_manage list` antwortet wieder; zu
  Session-Beginn `list void`/403). In diesem Atom dispatcht:
  `tao-wnd-cdn.yml` → `36225623480`; nach Push `e6b92b605`:
  `gll-rss-odr-cdn.yml` → `36227804843`, `gll-rss-tnf-cdn.yml` → `36227808151`,
  `messenger-tnf-cdn.yml` → `36227810541`, `ams02-tdat-cdn.yml` → `36227812411`.
- **Sicherheitsnetz:** `git_safety --snapshot` → `refs/safety/1790375651`
  (recover: `git_safety --restore refs/safety/1790375651`).
- **`register_lookup --fired`:** 1 fired (`mycelium` pre-cdn), 0 Mountain;
  `--stale`: 0; `--orphans`: keine Mountain-Orphans.
- **Concurrency-Warnung:** Der Arbeitsbaum ist stark fremd-beschrieben (parallele
  Sessions editieren live `src/archivar/extract.rs`, `main_flow.rs`, `mod.rs`,
  `src/gate/*`, `phi/sources.φ`, `phi/blocked_sources.φ`, neue hamqsl/nohrsc/
  ogimet/eri-Dateien). Fremde Hunks nie überschreiben/sweepen.

## Offen (aufgeschlüsselt)

### Linie handelt (eigen)

#### gll.rss + Klasse-5 — Register + Workflows + CI-Dispatch (nach Commit/Push)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eigener Commit steht + `origin/main` Vorfahr (dann `gh workflow run`).
- **Lage:** (gemessen 2026-09-26) Gebaut (committet `e6b92b605`/`bc1d87643`/
  `f1bd268b2`): `atdf.rs`
  SFOC-NAV-2-25-Format-8 + GLL-Dispatch-/Component-Arme (am echten F8-File
  `gll_rss_2002308t0717_dssmm_tdf.dat` verifiziert; Fabrikations-Fix
  `ref_sky`/`xmtr_ref` → `Option`), `cassini_rsr.rs` Recordlänge aus Label
  (GLL 8260 B/2000 I/Q), `galileo_odr.rs` `year_full` 2000–2003 (gemessene
  Spanne 1990..=2003), `opendap.rs` DAP2-Grid-Fix (live verifiziert),
  `tdat.rs` AMS-02 (18340 Zeilen/23 Spezies), neu `messenger_tnf_compiler.rs`
  (am echten TRK-2-34-File verifiziert: 120774 Zeilen) + `ams02_tdat_compiler.rs`.
  Register-Blöcke in `phi/sources.φ` (`gll_rss_rsr`, `gll_rss_atdf`,
  `gll_rss_atdf_x`, `messenger_tnf`, `ams02_spec`, 8445–8564) geschrieben, aber
  im Gridlock **uncommittet**. Neu:
  `.github/workflows/messenger-tnf-cdn.yml` + `ams02-tdat-cdn.yml`. `gll_rss_odr`/
  `gll_rss_tnf` waren bereits registriert.
- **Blockade:** keine
- **Braucht:** dispatcht nach Push (s. Stehender Pass, 4 Runs); offen: Shard-/sha-
  Blöcke nach dem CI-Lauf in `phi/sources.φ` nachtragen (MESSENGER-TNF ~30 GB →
  `--year`-Filter); ein `gll-rss-atdf-cdn.yml` fehlt noch (Register-Hunk im
  Gridlock).

#### Benannt — ungemessen (offene Messungen aus diesem Atom)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (benannt 2026-09-26, **ungemessen**) Die Taucher haben diese Punkte
  als offen markiert; sie gehören gemessen, nicht geglaubt:
  - `atdf.rs` F8-Item-20 **Doppler-Bias-Einheit** (`/1000`): Report „ließ sich
    aus den Quellen nicht abschließend belegen".
  - `cassini_rsr.rs` **SFDU-Magic `NJPL`/`C997`** (Byte 0/8): „falls das
    GLL-Label andere IDs trägt, wäre `read_record` eine eigene Messung".
  - `units.rs` `allowed_units_for_force(0)` ohne `gev`/`gv`: `report_physics_mismatch`
    **kann** für die 46 AMS-02-Felder eine Anomalie melden (blockt nicht) — ob
    real, ungemessen.
  - **Präzedenz-Zuordnungen** unit-auto-detect: `sz_signal_to_noise`/`kp_value`
    → em 1, Wolf-Zahl/σ → descoped „Statistik" — per Präzedenz/Statistik
    zugeordnet, nicht am Feld gemessen.
  - `main_flow.rs` **Fetch-Liste** `gll_rss_atdf[_x]`/`messenger_tnf`/`ams02_spec`:
    per Analogie eingetragen (`cargo check` grün); Lauf fetch→parse→Samples fehlt.
  - **Workflow-Ergebnisse** `messenger-tnf-cdn`/`ams02-tdat-cdn`: dispatcht,
    Ergebnis ungemessen (nicht gepollt).
  - **mariner10-Regeneration**: dass der Eintrag `at mariner10` die Route
    wirklich erzeugt, ist nicht gemessen.
- **Blockade:** keine
- **Braucht:** je ein realer Record/Lauf wie oben (`atdf.rs`-F8-Record mit
  bekanntem Bias; echtes `gll.rss_rsr`-Label; AMS-02-Parse-Lauf; ein
  `gll_rss_atdf`-Bin-Lauf; `gh run view <id>` einmalig).

#### Concurrency-Gridlock — geteilte `phi`-Register (`phi/sources.φ`, `phi/blocked_sources.φ`)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Fremd-Session commit.
- **Lage:** (gemessen 2026-09-26) Die Code-Seite ist committet
  (`e6b92b605`, `bc1d87643` extract.rs, `f1bd268b2` main_flow.rs). Geteilt
  verflochten bleiben nur die `phi`-Register-Hunks: `phi/sources.φ` trägt meine
  5 Blöcke (`gll_rss_rsr/atdf/atdf_x/messenger_tnf/ams02_spec`) **und** einen
  fremden `iras_psc`-TAP-Block unstaged; `phi/blocked_sources.φ` trägt meine
  unit-auto-detect-Block-Entfernungen **und** fremde Query-URL-Fixes
  (mycelium) unstaged.
- **Blockade:** fremde uncommittete Arbeit in denselben Dateien.
- **Braucht:** `git add -p` (nur eigene Hunks) sobald die Fremd-Session
  committet, dann eigener Folge-Commit der `phi`-Hunks.

#### NAIF mariner10 — dauerhafte Route
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** frame_registry-Regeneration / Eigentümer-Wort.
- **Lage:** (gemessen 2026-09-26) `mariner10`-Anchor gültig
  (`phi/sources.φ:3204` `at mariner10`); `phi/pipeline/frame_registry.φ` ist
  gitignored/generiert (`src/archivar/frames.rs::build_frame_registry` aus
  `sources.φ`/`dead_*`/`declined_*`/`blocked_*`/`witnesses.φ`). Der NAIF-Key
  fehlt in der generierenden Quelle: `phi/blocked_sources.φ:58` trägt nur `url`.
- **Blockade:** generierte Datei; `blocked_sources.φ` fremd-modifiziert.
- **Braucht:** `naif.jpl.nasa.gov/pub/naif/M10/kernels/spk/M10_archive_1.bsp |
  at mariner10` in die generierende Quelle, dann Regeneration.

#### Parquet-/GRIB-2-Codec-Grenzen (named gap, kein Verdikt)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) Structure-Reader stehen
  (`parquet.rs`/`grib2.rs`) — Survey-Claims „fehlen" widerlegt. `cora_ar.parquet`
  = 868606056 B (nicht 90,3 MB), Thrift-Compact/PLAIN/UNCOMPRESSED(+SNAPPY);
  NOAA-GFS = Template 5.3, ECMWF-IFS = 42 (CCITT-G4). Nicht getragen: Parquet
  zstd/gzip/delta, GRIB-2 Section-7 (complex/JPEG2000/CCITT).
- **Blockade:** keine
- **Braucht:** je Codec entscheiden (CCITT-G4/SNAPPY nah, JPEG2000/zstd nicht) —
  bauen oder R.5 mit Befund.

#### Atom D — Beat-Paar
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** two-station open-loop Aufnahme eines Trägers (extern) — sonst kein Flip.
- **Lage:** (gemessen 2026-09-26) Atom D gebaut (`odf.rs::tnf_phase_series`,
  WGSL `beat_pair` hinter presence-/ν-Gates; `docs/specs/spectral-oscillator.md:223`
  built 2026-09-23); das Paar selbst absent.
- **Blockade:** keine Messdaten für ein kohärentes Paar.
- **Braucht:** dual-comb-/two-station-Kandidat an Mycelium; bis dahin `wartend`.

### Operator handelt

#### S3-Scheme (Token) + `epochrange` Wire-Slot + Membran-Debug (Chrome MCP)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort.
- **Lage:** (gemessen 2026-09-26) S3-Scheme `blocked key` (Survey: SigV4+Handshake
  gebaut; alter 401 = Register-Umbuchung); `epochrange` (MJD-Breite) hat keinen
  Wire-Slot (Architektur-Akt); Chrome-DevTools-MCP antwortet `-32001 timeout`,
  braucht Chrome-Start mit `--no-usage-statistics --no-performance-crux`.
- **Blockade:** Operator-Wort (bzw. Token).
- **Braucht:** je ein Operator-Wort; MCP danach in `docs/concepts/tools-map.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
