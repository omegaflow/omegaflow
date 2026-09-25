<!--
  title: Handover — Mountain-Folge 165 (2026-09-25)
  session: Mountain-Folge 165
  class: handover
  date: 2026-09-25
  sha256: 75de6c59ae1b64acc4a882c5e25be6de50040ab568cdd391e25e0ad4550ed386
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

**Fortschreibung 2026-09-25 (Fold-Session):** Watchdog-Snapshot 22:08Z: 5 aktiv
(`source-census` in_progress 19:28, `ci-check` 19:01, `bz-retro-probe` 18:56,
`ps1-cdn` 18:35, `health-check` queued 18:22), 2 failed (`galileo-ionocal-cdn`,
`ci-check`); Postfach: `state/mail/mail_ledger.φ` absent, `mail_digest` pending
(Build gehört CI `tools-build`); `git_safety --snapshot` → `refs/safety/1790371511`.
`register_lookup --orphans`: keine Mountain-Orphans (alle `parser-def`-Einträge
gehalten; 9 fremde: future 4, mycelium 5).

## Offen (aufgeschlüsselt)

### Linie handelt (eigen)

#### `phi/blocked_sources.φ::gap:unit-auto-detect ×129`
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `sgrep -c` + grind-flash) der Arm steht
  (`unit_from_name_suffix`, `src/archivar/units.rs:169`). Korrektur der Vorgabe:
  **×327 war die Token-Vorkommenszahl** (166 `gap` + 161 `note`), nicht die
  Eintragszahl — live waren **166** `gap`-Einträge. 37 Einträge wurden disponiert
  (Block entfernt), **129 verbleibend**. Der volle Klassen-Arm umfasst laut
  Register-Notiz auch `probe_classify` (`src/archivar/port.rs:1681`; löst u.a.
  `mag_*`, `magnitude`, `_mpc`, `_mw`, `_kt`, `_f`, `redshift`) — unter dem vollen
  Arm wäre die Zahl kleiner.
- **Blockade:** keine
- **Braucht:** Re-Port der 129 mit dem **vollen** Arm (`unit_from_name_suffix` +
  `probe_classify`); Trägerform `phi/blocked_sources.φ::gap:unit-auto-detect ×129`.

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

#### TAO/TRITON — CI-Dispatch
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** GitHub-API-Rate-Limit erholt (nächster Pass).
- **Lage:** (gemessen 2026-09-25 via git) die Source ist registriert
  (`phi/sources.φ:783`, `tao_wnd_zonal`); Compiler + Workflow sind bereits
  committed (`974700848`: `tools/harvest/src/bin/tao_wnd_compiler.rs`,
  `.github/workflows/tao-wnd-cdn.yml`, auf `origin/main`). Der Dispatch
  `gh workflow run tao-wnd-cdn.yml` lief in `API rate limit already exceeded`
  (GraphQL, user 295896184).
- **Blockade:** rate limit (transient).
- **Braucht:** `gh workflow run tao-wnd-cdn.yml` (einmal, wenn das Limit erholt ist).

#### `epochrange` — kein Wire-Slot
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (neuer Wire-Slot).
- **Lage:** (gemessen 2026-09-25 via grind-pro) der DECaPS-`epoch`-Arm ist gebaut
  (`CelestialMap` nimmt `epoch` an, MJD→TDB via `mjd_to_tdb`); `epochrange` (Tage,
  MJD-Breite der Epoche) hat keinen Slot im 26×f64-Record noch als `Channel`-Feld.
- **Blockade:** kein Slot; ein neuer Slot ist ein Architektur-Akt.
- **Braucht:** Rat/Operator-Wort, ob die Epochenbreite je ein Feld wird.

#### NAIF mariner10 — frame_registry-Route
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** frame_registry-Regeneration (`phi/pipeline/frame_registry.φ`).
- **Lage:** (gemessen 2026-09-25) `ephemeris_mariner10.bin` gebaut
  (`mariner10-ephemeris-cdn 36181036369`; 648 B, sha256 `7ad8b8bf…`, in
  `phi/sources.φ`); `phi/pipeline/frame_registry.φ` trägt keine mariner10-Route.
- **Blockade:** Register-Route fehlt.
- **Braucht:** Eintrag `naif.jpl.nasa.gov/pub/naif/M10/kernels/spk/M10_archive_1.bsp
  | at mariner10`.

#### Atom D — phase/presence-Konsum (Route B)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-23 im Rat,
  `survey-2026-09-17-verlorene-diskussionen.md:85`) die Prämisse „kein Asset trägt
  Phase" ist widerlegt: vier phase-tragende Klassen am CDN (`cassini_rsr` I/Q
  `sources.φ:8081`, cassini/maven TNF-Trägerphase `:8099/:8819` inkl. `ramp_freq`
  `odf.rs:1653-1666`, fdsn BHZ `:110`, RAWACF nur Power); die Slots fahren seit v9
  mit, nichts liest sie.
- **Blockade:** keine
- **Braucht:** Producer schreibt `phase: Some(fract(cycles)·2π)` + `freq=ramp_freq`
  (nie hartkodiert) auf der TNF-Route, `bin_width=0.0` (null-echt point source);
  WGSL-Beat-Term nur für ein Paar; ein Atom, `grind-max`. Das Beat-Paar bleibt
  `pending` mit Trigger.

#### field absorption — per-force_type-Absorptionsgesetz
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-23, `survey-2026-09-17-verlorene-diskussionen.md:86`)
  verifiziert: `src/mathematikerin/shaders.rs:53` `alpha = clamp(absorption, 0.0, 1.0)`
  blendet nur Kernel 5 (`:52-57`, Gradient `:90-97`); jeder andere Kernel ignoriert den
  Parameter; `src/archivar/channels.rs:1020` schreibt `sensor.absorption`,
  `:1061/:1088` Pad `0.0`; ein per-force_type-Absorptionsgesetz ist ungebaut — WP10
  `docs/concepts/remove-bias.md:1738` ist Plan, kein `absorbs`-Symbol im Baum.
- **Blockade:** keine
- **Braucht:** per-force_type-Absorptionsgesetz in WGSL + Producer je Kraftkanal.

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

## Träger (Prosadokumente) — orphan-docs-Fold (2026-09-25)

Je Zeile ein trägerloses Dokument der Linie: `Pfad | offene Marker: N | nächster
Schritt: <tool/file/url>`. Der Dateiname in dieser Übergabe ist der Träger
(`register_lookup --orphan-docs`). `archivar-mathematikerin.md` fiel mit dem
`pending`-Definitions-Fix aus der Liste (der Marker war ein definierter Begriff).
Kein `git mv`: alle acht sind lebende Referenzen/offene Messlinien, keine
abgeschlossenen Dokumente.

**Gemessen 2026-09-25:** `register_lookup --orphan-docs` **53 → 10**; der
Mountain-Anteil ist 0. Der Rückgang trägt (a) den Präzisions-Fix
(`strip_inline_code` + `blocked`-Statuskontext in `register_lookup.rs` und
`commit_gate.rs`: Definitionen in Backticks/Kursiv zählen nicht mehr als offener
Marker — die vier Identitäts-/Governance-Konzepte `die-weberin.md`,
`docs-naming.md`, `kybernaut-native-methodology.md`, `the-counter-slope.md` sind
Falsch-Positive gewesen) und (b) die Träger-Zeilen unten plus die Träger-Folds
der anderen Linien. Werkzeug-Fix + Tests im selben Atom
(`inline_code_terms_are_not_open_markers`, `blocked_counts_only_as_a_status_token`,
`fp_doc_open_marker_gate_contract`); Commit-Wort `/commit`.

- `docs/concepts/arxiv-api.md` | offene Marker: 2 | nächster Schritt: den im Blatt genannten 406-Punkt (`arxiv HTTP 406`) gegen `docs/handover/handover-2026-09-25-mountain-folge165.md` abgleichen — tragen oder als gemessen schließen (`sread`).
- `docs/concepts/pfeiler-der-architektur.md` | offene Marker: 2 | nächster Schritt: keine — beide Marker Prosa (`seit Jahrzehnten offenes Problem`, `vorgewartet`), Lebend-Referenz ohne offenen Zustand.
- `docs/concepts/zeugnis.md` | offene Marker: 5 | nächster Schritt: §14-Bau-Linie Stufe 2 (Footprints) → `phi/footprints.φ` (PS1+AllWISE gebaut, SDSS/2MASS refused); Stufe 1/3/4 folgen.
- `docs/surveys/survey-2026-09-06-codestruktur.md` | offene Marker: 8 | nächster Schritt: `ci_manage view 36172029908` (measure-gates) / `36172034181` (service-build), dann die Deckung der tools-Crates re-messen; tote pub-Fns in `974700848` entfernt.
- `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md` | offene Marker: 2 | nächster Schritt: TDA/Betti-0 + Delay-Spectrum als measure-Probe in tools/measure bauen; Silence-Map (`silence_map_probe`) bereits gebaut, Minkowski-Delta-Probe.
- `docs/surveys/survey-messpunkt-verteilung.md` | offene Marker: 6 | nächster Schritt: R_struct-Verlauf je Kernel-Form als Fixture (`src/mathematikerin/shaders.rs`), Multipol-Fehler-Fixture.
- `docs/surveys/survey-fortschritt.md` | offene Marker: 1 | nächster Schritt: §C-Punkte gegen den Baum messen (Relay-Trailer, `deep_dirty`, Rgba8Unorm); bei geschlossenem Befund nach `docs/surveys/archiv/`.
- `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` | offene Marker: 9 | nächster Schritt: Mountain-Anteile Atom D + field absorption (oben, gebaut-zu-bauen); HRV/ESP32 → `handover-2026-09-25-river-folge32.md`, sources-Repo → `handover-2026-09-25-mycelium-folge165.md` (direkt getragen 2026-09-25).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
