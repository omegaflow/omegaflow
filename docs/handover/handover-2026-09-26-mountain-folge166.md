<!--
  title: Handover — Mountain-Folge 166 (2026-09-26)
  session: Mountain-Folge 166
  class: handover
  date: 2026-09-26
  sha256: 4693da33ff7e83b618eec10787ea8eef5d8cb1764d2e828633588e3b1b072007
  status: live
-->
# Handover — Mountain-Folge 166 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Sortierung: erst Akteur (Linie | Rat | Operator |
Dritter), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
Trigger / Lage / Blockade / Braucht.

## Stehender Pass (gemessen zu Session-Beginn)

- **Postfach:** (gemessen 2026-09-26 via `sread state/mail/mail_ledger.φ`)
  `state/mail/mail_ledger.φ` present, letzte Eingänge 2026-08 (Registrierungs-/
  Verify-Mails, GitHub 2FA). Keine fällige Korrespondenz. Eintrag:
  `docs/zustand/external-state.md`.
- **CI-Status:** Watchdog-Snapshot 2026-09-25T23:12:34+02:00: 6 aktiv
  (`paper-check`, `nh-rex-tnf-cdn`, noaa-*-cdn queued; `supermag-magstid-cdn`
  in_progress), 3 failed (`supermag-cdn`, `radnet-cdn`, `noaa-ghcn-cdn`).
  Ergebnis beim nächsten Pass aus dem Snapshot, nie gepollt.
- **Sicherheitsnetz:** `git_safety --snapshot` → `refs/safety/1790373240`
  (recover: `git_safety --restore refs/safety/1790373240`).
- **`register_lookup --fired`:** 1 fired (`mycelium` pre-cdn ci-grün), 0 Mountain;
  `--stale`: 0; `--orphans`: keine Mountain-Orphans.

## Offen (aufgeschlüsselt)

### Linie handelt (eigen)

#### `phi/blocked_sources.φ::gap:unit-auto-detect ×37`
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `sgrep -c` + grind-flash) 92 der 129
  wurden mit dem vollen Arm (`unit_from_name_suffix` `units.rs:169` +
  `probe_classify`/`probe_classify_raw` `port.rs:1601`/`:1681`) aufgelöst
  (Blöcke entfernt, Feld→(force,unit) bzw. R.5-Drop gemessen). **37 verbleiben**,
  deren Arm `None`/`UNCERTAIN` liefert: `rv_semi_amplitude`, `earth_radius`,
  `transit_duration_hours`, `sz_signal_to_noise`, `exposure_s`, `dm`, `Hp/He/Hn`,
  `au/ae`, `area_millionths`, `wavelength_nm`, `linear_trend_cm_yr`,
  `aerosol_angstrom`, `gamma_dose_uR_h`, `value`, `co_column_mol_cm2`,
  `ohc_0_700m_10e22J`, `significance`, `trend_mm_yr`, `erythemally_weighted_uv`,
  `slip_rate_mm_yr/age_yr`, `vx_au_day`, `x_pole/y_pole/ut1_utc`, `x_pole_mas`,
  `ut1_utc_s`, `geosphere_ut1_utc_iers`, `sigma`, `energy/signalness`,
  `jan..dec`, `insolation_kwh_m2`, `sunspot_number/std_dev`, `kp_value`,
  `solar_sunspot_number`, `geosphere_earth_orientation_raw`.
- **Blockade:** keine
- **Braucht:** den Suffix-Arm für diese Einheiten-Klassen erweitern
  (`probe_classify_raw`/`unit_from_name_suffix`) **oder** per R.5 disponieren;
  Trägerform `phi/blocked_sources.φ::gap:unit-auto-detect ×37`.

#### gll.rss — ATDF-Arm (SFOC-NAV-2-25 Record Format 8)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via grind-pro) Compiler
  `tools/harvest/src/bin/gll_rss_atdf_compiler.rs` steht; `atdf.rs`-TKFORM ist
  pre-1997 TRK-2-25 (Format 4), das Bundle nutzt SFOC-NAV-2-25 (Format 8:
  Sample-Year 73–84, Day 85–100, Minute 109–116, Second 117–124, Station
  145–154, Band 155–162, Data-Type 163–168, SC-ID 177–192) → Arm liest
  DATA_TYPE bei 181–184 → 0 für alle 137155 Records, 0 Samples.
- **Blockade:** kein SFOC-NAV-2-25-Parser in `src/archivar/atdf.rs`.
- **Braucht:** Format-8-Zweig in `atdf.rs` + Registrierung in `phi/sources.φ` +
  `*-cdn`-Workflow, dann CI-Dispatch.

#### gll.rss — RSR-Recordlänge (8260 B / 2000 Samples)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via grind-pro) Compiler
  `tools/harvest/src/bin/gll_rss_rsr_compiler.rs` steht; `cassini_rsr.rs`
  hartkodiert `RECORD_BYTES=4260`/`DATA_WORDS=1000`; gll.rss-RSR = 8260 B /
  2000 I/Q-Paare (1141×8260 B, Label: Data-CHDO-Length Byte 258) → 0 Samples.
- **Blockade:** Recordlänge nicht parametrierbar.
- **Braucht:** Recordlänge aus dem Label lesen (`cassini_rsr.rs`), dann
  Registrierung + `*-cdn` + CI-Dispatch.

#### gll.rss ODR — `year_full` verwirft Records nach 1999
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via grind-pro) `galileo_odr::record_stride`/
  `header` parst RSC-11-11 (30241 Records, 1250 sps, 2666 B/Record); der
  ODR-Compiler shardet bei 1 GB und druckt Shard-URLs. `galileo_odr::year_full`
  akzeptiert nur `90..=99` → ODR-Records 2000–2003 werden verworfen.
- **Blockade:** keine
- **Braucht:** `year_full` um 2000er-Jahre erweitern (`src/archivar/galileo_odr.rs`);
  ODR-Shard-Blöcke nach dem ersten CI-Lauf in `phi/sources.φ` nachtragen.

#### Klasse-5 offene Routen bauen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `archive_search`, siehe
  `docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md`) vier
  Stellen offen: S3-Scheme (200 mit Token), ODF TRK-2-34/TNF (offener Korpus,
  Parser-Arm fehlt), AMS-02 TDAT (HEASARC live, Reader fehlt), Parquet/GRIB-2/
  OPeNDAP (Reader fehlen). In diesem Atom nicht dispatcht.
- **Blockade:** keine
- **Braucht:** TRK-2-34/TNF-Parser (`odf.rs`), TDAT-Reader, Parquet-/GRIB-2-/
  DAP2-Reader; AMS-02 als `live` registrieren.

#### NAIF mariner10 — frame_registry-Route
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** frame_registry-Regeneration (`phi/pipeline/frame_registry.φ`)
- **Lage:** (gemessen 2026-09-25) `ephemeris_mariner10.bin` gebaut
  (`mariner10-ephemeris-cdn 36181036369`; 648 B, sha256 `7ad8b8bf…`, in
  `phi/sources.φ`); `phi/pipeline/frame_registry.φ` trägt keine mariner10-Route.
- **Blockade:** Register-Route fehlt.
- **Braucht:** Eintrag `naif.jpl.nasa.gov/pub/naif/M10/kernels/spk/M10_archive_1.bsp
  | at mariner10`.

#### TAO/TRITON — CI-Dispatch
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** GitHub-API-Rate-Limit erholt.
- **Lage:** (gemessen 2026-09-25 via git) Source registriert (`phi/sources.φ:783`,
  `tao_wnd_zonal`); Compiler + Workflow committed (`974700848`); Dispatch lief in
  `API rate limit already exceeded`.
- **Blockade:** rate limit (transient).
- **Braucht:** `gh workflow run tao-wnd-cdn.yml` (einmal, wenn das Limit erholt ist).

#### Atom D — Beat-Paar
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** eine two-station open-loop Aufnahme eines Trägers (oder ein Asset
  mit zwei kohärenten Tönen in einem Band).
- **Lage:** (gemessen 2026-09-26 via grind-max) Atom D ist gebaut
  (`odf.rs::tnf_phase_series` via `carrier_phase_rad`, `freq=ramp_freq`,
  `bin_width=0.0`; WGSL `beat_pair` hinter presence-/ν-Gates;
  `docs/specs/spectral-oscillator.md:223` `Status: built (2026-09-23)`). Das
  Beat-Paar selbst ist noch nicht gebaut; two-station open-loop gemessen absent.
- **Blockade:** keine Messdaten für ein kohärentes Paar.
- **Braucht:** dual-comb-/two-station-Kandidat an Mycelium; bis dahin `pending`.

### Operator handelt

#### `epochrange` — kein Wire-Slot
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (neuer Wire-Slot).
- **Lage:** (gemessen 2026-09-25 via grind-pro) DECaPS-`epoch`-Arm gebaut
  (`CelestialMap` nimmt `epoch`, MJD→TDB via `mjd_to_tdb`); `epochrange` (MJD-Breite)
  hat keinen Slot im 26×f64-Record noch als `Channel`-Feld.
- **Blockade:** kein Slot; ein neuer Slot ist ein Architektur-Akt.
- **Braucht:** Rat/Operator-Wort, ob die Epochenbreite je ein Feld wird.

#### Membran-Debug — Chrome DevTools MCP anbinden
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (Debugger-Rechte am laufenden Chrome)
- **Lage:** (gemessen 2026-09-25 via `chrome-devtools_list_pages`) der MCP
  antwortet `-32001 timeout` → nicht angebunden; Pfad (i) „noch nicht angebunden"
  (`docs/concepts/tools-map.md`). Telemetrie-Flags `--no-usage-statistics`
  `--no-performance-crux` sind Bedingung.
- **Blockade:** Operator-Wort + Chrome-Start mit den Debug-Flags.
- **Braucht:** Operator startet Chrome mit den beiden Flags/Remote-Debugging;
  dann MCP anbinden und in `docs/concepts/tools-map.md` registrieren.

### Offen nach diesem Atom (CI)

- `gh workflow run gll-rss-odr-cdn.yml` / `gll-rss-tnf-cdn.yml` nach dem Push
  (Assets sind CI-Jobs; ODR shardet bei 1 GB).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
