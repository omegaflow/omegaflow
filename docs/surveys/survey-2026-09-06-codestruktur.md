<!--
  title: Survey — Codestruktur: Archivar, Mathematikerin, Tools (Struktur-Karte + erste Funktions-Messung)
  class: survey
  date: 2026-09-06
  sha256: 62758fff918d2039fd8e59015eb25d259c045b01665d07cb7341a128cad9b15f
  status: live
  see-also: docs/concepts/archivar-mathematikerin.md docs/concepts/die-weberin.md docs/concepts/docs-naming.md
-->
# Survey — Codestruktur: Archivar, Mathematikerin, Tools

Dies ist eine **Struktur-Karte** — sie zählt Namen, nicht Funktion — plus die
ersten zwei Vermessungs-Dimensionen (kompiliert, Tests grün). Was sie nicht
misst, ist als offen benannt (§3). Ein `.rs`, das existiert, ist nicht
„gebaut"; das ist die Stelle, an der diese Karte ihre Grenze benennt.

## 1. Die Struktur (Namen, gemessen)

368 `.rs`-Dateien: `src/` 115, `tools/` 253. Grundmuster: jedes Binärformat
trägt ein `write_bin`/`parse_bin`-Paar; die Compiler-Bins rufen die Parser
aus `src/archivar` auf.

### src/ — Kern-Crate
- **archivar/** (~86 Dateien, ~712 pub-Items) — der Quell-Compiler-Stapel.
  Binär-/Record-Formate: aia_lines, atdf, bayestar, bidsleep, bison_*,
  bl_narrowband, demeter, euvs, eve_lines, exclude, f107, goes, gong(_series),
  hmi_polar, ir, jwst(_equilibrium), kbo, mitdb, movement_monitoring, odf,
  omni2, phonocardiogram, pioneer_telemetry, radio, rixs, rpw, skydirection,
  spectral, suprastrom, tns, twomass, wind, wind_orbit, wso_polar, ztf.
  Dateiformat-Parser: fits, hdf5, netcdf+nc4, cdf+cdf25, matfile, cif, pck,
  ionex, inflate, lzw, json, sexagesimal, dastcom, thermochem. SPICE:
  bsp_reader/{daf,spk}, fk, lsk, ephemeris, motion, spatial, channels, frames.
  Universal-Tools: extract, parse, units, fetch, cdn, naming, render, port,
  main_flow, ingress, relay, membrane.
- **mathematikerin/** (15 + machines/ 5, ~145 pub-Fn): omega.rs (OmegaLoop),
  actuators, s2, shaders (FIELD_WGSL/TE_WGSL/S2_WGSL), te.rs (~30 Fn:
  transfer_entropy, _lag, _conditional, _embedded, topological_te_*,
  permutation_entropy, Surrogate, find_mi_lag, embed_series, silverman,
  hilbert_instantaneous_phase), force, media, doppler, healpix, orientation,
  mat, least_squares, equilibrium; machines/{matrix,solar,verdict,tests}.
- **gate/** (7, ~45 pub-Fn): commit_gate, axioms, friction, handover, state,
  tool_perm.

### tools/
- harvest (83 Bins: ~74 Compiler + 8 generische Harvester + Sonder-Bins)
- measure (137: 134 Proben + lib, deredden, nadel_gate) — Solar/Pioneer/
  Galileo/TE-Prüfungen/sonstige
- register (12), service (5), science (5), gate (2), utils (9)

Kernbeobachtung: die Bibliothek (Parser/Writer/Physik) lebt in
`src/archivar` + `src/mathematikerin`; die `tools/*`-Crates sind fast reine
`bin`-Programme, die diese API aufrufen — harvest schreibt, measure misst,
register/science/service/gate/utils operieren.

## 2. Erste Funktions-Messung (2026-09-06)

- `cargo check`: grün — 0 Fehler, 0 Warnungen (Core-Crate, 0,38 s).
- `cargo test`: **526 passed, 0 failed, 1 ignored** (147,59 s).
- Belegt als bestehend (nicht nur vorhanden): TE-Kalibrier-Gate
  (`calibration_fp_independent_ar1_stays_near_chance`), GPU↔CPU-Parität
  (`s2_gpu_matches_the_cpu_spherical_harmonic_reference`,
  `te_gpu_crosscheck_against_cpu_reference`), WGSL-Offline-Validierung
  (`field/s2/te_wgsl_validates_offline`), GPU-Pack-Offsets
  (`golden_pack_slots_against_wgsl_access`). `te.rs` ist damit die kanonische
  CPU-Referenz **mit** Kalibrier-Gate.
- Randnote: Shader-Cache meldete `Permission denied` (der Cache-Pfad war
  nicht beschreibbar) — kein Testfehler (Tests bleiben grün).

## 3. Offen — die restlichen Vermessungs-Dimensionen

Die Struktur-Karte und die ersten zwei Dimensionen sind gemessen. Offen:
- **tools/ je Crate vermessen** (gemessen 2026-09-25, HEAD `f02171e5`): Es gibt
  **keinen** Workflow, der `cargo check -p omegaflow-<fkt>` fährt — `cargo check`
  (`ci-check.yml:30`) deckt nur die Core-Crate, und einen ganzen-Crate-`cargo
  test -p` gibt es nur für `register` (`ci-check.yml:56`) und `utils`
  (`ci-check.yml:57`). Workflow-Deckung je Crate:

  | Crate | CI-Workflow / Step | Run-ID (2026-09-25) | Stand |
  |---|---|---|---|
  | omegaflow-harvest | `ci-check.yml:58-62` (5 Bin-Tests) + je Compiler-Bin in `*-cdn.yml` | `ci-check` 36171288869 | pending CI |
  | omegaflow-measure | `measure-gates.yml:27` (`--bin silence_map_probe`), `corpus-te.yml:21` (`--bin corpus_te`) | `measure-gates` 36172029908 | pending CI (dispatch 18:13) |
  | omegaflow-register | `ci-check.yml:56` `cargo test --release -p omegaflow-register` | `ci-check` 36171288869 | pending CI |
  | omegaflow-service | `service-build.yml:14` `cargo build --release -p omegaflow-service` | `service-build` 36172034181 | pending CI (dispatch 18:13) |
  | omegaflow-science | `paper-check.yml:37,41` `export_latex --check` | `paper-check` 36169861633 | success |
  | omegaflow-gate | `tools-build.yml:23` `cargo build --release --bins … -p omegaflow-gate` | `tools-build` 36171288887 | success |
  | omegaflow-utils | `ci-check.yml:57` `cargo test --release -p omegaflow-utils` | `ci-check` 36171288869 | pending CI |

  `harvest`/`measure` werden nur bin-weise getestet, `service`/`science`/`gate`
  nur gebaut (`science` zusätzlich `export_latex`-Gate). Neu dispatcht 2026-09-25:
  `measure-gates` 36172029908, `service-build` 36172034181; die übrigen Run-IDs
  stammen von den Push-Runs am HEAD `f02171e5`.
- **Konsument je pub-Fn** (gemessen 2026-09-25 via `sgrep <fn> src`):
  - `src/archivar/port.rs:1498 pub fn find_timestamp` → **tot**: kein
    Call-Site im getrackten Baum; die einzigen weiteren Treffer liegen in
    gitignored `state/zai-export/`.
  - `src/mathematikerin/te.rs:1867 pub fn find_mi_lag` → **lebendig**: 27
    Treffer in `src`, darunter Produktions-Call-Sites `te.rs:2058, 2592-2593,
    3018, 3026` und `ksg_k.rs:82-83, 303`.
  - Zählung über beide Crates: **1 lebendig / 1 tot**. Ein Scan aller pub-Fns
    (`sgrep -c <name> .` je Name) findet im getrackten Baum genau zwei tote
    pub-Fns: `find_timestamp` (`port.rs:1498`) und `number_text`
    (`mpcorb.rs:33`) — beide `src/archivar`; `src/mathematikerin` trägt keine
    tote pub-Fn.
- **Verdrahtung live/offline:** ω-Pfad vs. tools/measure. Die §8-Tabelle in
  `docs/concepts/die-weberin.md` führt beide getrennt: LIVE/ω-Pfad (`:210-220`)
  gegen OFFLINE/`tools/measure` (`:222-233`); `direction_distance_join`,
  `nadel_gate`, `deredden_baseline_probe` stehen in der OFFLINE-Tabelle —
  Aufspaltung ist `erledigt` (gemessen 2026-09-25, getrennt in `5dcce39c0`).
- **Datenvertrag** je Format-Modul: 26×f64-Wire / GPU-Pack-Offsets gegen die
  WGSL-Zugriffe (das manuelle Verifikationsprotokoll in
  `docs/concepts/archivar-mathematikerin.md`).
