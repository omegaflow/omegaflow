<!--
  title: Survey — Codestruktur: Archivar, Mathematikerin, Tools (Struktur-Karte + erste Funktions-Messung)
  class: survey
  date: 2026-09-06
  sha256: 6bd8a4aca15d8c446f4218787396ab0a014a774f1174fd7d0867a7abaa32bf65
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
- Randnote: `Failed to create /home/probe for shader cache (Permission
  denied)` — Shader-Cache-Pfad, kein Testfehler (Tests bleiben grün).

## 3. Offen — die restlichen Vermessungs-Dimensionen

Die Struktur-Karte und die ersten zwei Dimensionen sind gemessen. Offen:
- **tools/ unvermessen:** `cargo check`/`cargo test` deckten nur die
  Core-Crate. Die 253 `tools/*`-Dateien brauchen je Crate
  `cargo check -p omegaflow-<fkt>` + Test-Lauf.
- **Konsument** je pub-Fn: lebendig oder tot (Call-Site-Beweis fehlt).
- **Verdrahtung live/offline:** ω-Pfad vs. tools/measure. Die §8-Tabelle in
  `docs/concepts/die-weberin.md` führt beides flach nebeneinander (z. B.
  `direction_distance_join`, `nadel_gate`, `deredden_baseline_probe` sind
  Offline-Bins, keine Membran-Teile) — Aufspaltung ist `pending`.
- **Datenvertrag** je Format-Modul: 26×f64-Wire / GPU-Pack-Offsets gegen die
  WGSL-Zugriffe (das manuelle Verifikationsprotokoll in
  `docs/concepts/archivar-mathematikerin.md`).
