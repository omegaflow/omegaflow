# Handover: Die Dreiteilung — main · archivar · mathematikerin

Selbsttragendes Handover für eine frische Session mit null Vorkontext.
Stand: commit `fef2974` (2026-08-19). Zeilennummern können driften —
der Compiler führt, diese Liste ist der Kompass.

## Auftrag

Den verworrenen Zustand auflösen: „archivar" existiert zweimal
(der Archivar-Kern als lib in `src/archivar.rs` + eine dünne
Laufzeit-Hülle `mod archivar` in `src/main.rs`), und die Mathematikerin
steckt noch als `mod mathematikerin` in main.rs. Ziel ist die
Dreiteilung mit drei ehrlichen Namen:

```
src/main.rs            dünner Einstieg: fn main, ω()-Loop, Radiatoren, Relay
src/archivar.rs        der ganze Archivar (lib): holen, parsen, extrahieren, cachen
src/mathematikerin.rs  die ganze Mathematikerin: auswerten, WGSL, manifestieren
```

Danach bedeutet „archivar" wieder genau eines. Die zwei Domänen aus
AGENTS.md (Archivar = CPU, Mathematikerin = GPU) werden sichtbar.

## Warum jetzt (der Befund)

- Der doppelte Name ist ein Stolperstein für jedes frische LLM und jeden
  Menschen: `mod archivar` in main.rs ist keine Archivar, sondern eine
  Hülle, die `use omegaflow::archivar::*` importiert.
- Die Dreiteilung war schon immer das Ziel (AGENTS.md: „The system is
  strictly separated into two domains … Archivar … Mathematikerin").
- Die Split-Vorgeschichte (Atom 1, commit `0f1eefe`) hat den Archivar
  wegen des Probes gespalten — Kern in die lib, Laufzeit blieb in
  main.rs. Diese Halbheit wird jetzt geheilt.

## IST-Zustand (commit `fef2974`)

`src/main.rs` — 16 344 Zeilen, das Herz:
- `mod archivar { … }` — Zeile 996–11 882: die Laufzeit-Hülle. Enthält
  u. a. `main_flow`, `fetch_one` + CDN-Maschine (`cdn_fresh`/`cache_fresh`),
  `build_buffer`, `query_hash`, `sense_*`, `surface_motion`/`frame_motion`,
  `leap_seconds`/`system_now`, `resolve_asset`, `sensor_config`,
  `OriginState`, `build_netcdf_channels`, `build_finals_channels`,
  `build_ionex_channels`, `anchor`, `law_bounds`, `body_pole_at`,
  `gravity_manifest`, `kernel_extent`, die Radiator-Typen
  (AudioRadiator, SurfaceRadiator, SerialSurface, TcpRadiator,
  MathematikerinRadiator), `render_url`/`render_source_url`, die
  port-*-Maschine, `fanout_fetch`, `parse_station_entries`, die
  probe_*-Helfer.
- `mod mathematikerin { … }` — Zeile 11 883–15 418: `star_position_at`,
  `system_now`, `pack_window`, `window_median_extent`,
  `build_curve_set`, `build_planet_set`, die TE-Rufstellen
  (→ `omegaflow::te`), `Record = OscRecord`, Aberration.
- `mod relay { … }` — Zeile 15 419 ff.: der WebSocket-Schreibkreis,
  benutzt `crate::archivar::*` + `omegaflow::archivar::OscRecord`.
- Die WGSL-Strings liegen als Top-Level-Konstanten am Dateianfang.
- `fn main` ganz am Ende.

`src/archivar.rs` (lib, 5 314 Zeilen): der Archivar-Kern — Typen
(Motion, Oscillator, Position, Extract, …), SI-Block, Konstanten,
`parse_sources`, `extract`, `extract_series`, `fetch_raw`(+Bytes/Probe/Post),
Ephemeriden-Auswertung + `impl Motion`, `jlast`/`jfirst`/`kernel_id_of`,
`csv_to_json`/`text_to_json`/`tap_to_json`/`universal_auto_detect`,
`is_drop_key`/`is_unit_name`/`is_time_key`, Tests.

`src/te.rs` (lib, 467 Zeilen): Transfer-Entropie + phasenrandomisierte
Surrogate (FFT) + Block-Bootstrap + Tests.

`src/lib.rs`: `#![allow(mixed_script_confusables)]` + 19 Module
(archivar, bpc, bsp_reader, cdn, dastcom, fit, fits, fk, force, inflate,
json, kepler, lsk, mat, netcdf, pck, sexagesimal, spectral, te).

`src/bin/` (25 Dateien): CI-Compiler/Harvester — für diesen Schnitt
irrelevant, nicht anfassen.

## Der eine Faden (die einzige echte Kopplung)

`mod archivar` in main.rs beginnt mit:

```rust
use crate::mathematikerin::{build_curve_set, build_planet_set};
```

`fetch_one` ruft diese beiden für Transit/Lightcurve-Quellen auf. Das
ist Archivar → Mathematikerin — der einzige Faden, der den sauberen
Schnitt bisher verhindert hat.

**Entscheidung, die die Session trifft (Survey vor dem Schnitt):**
Sind `build_curve_set`/`build_planet_set` rein (nur lib-Typen +
Mathematik, kein WGSL/relay/IO)? Wenn ja → in die lib ziehen
(`archivar.rs` oder ein kleines Modul), dann ist `fetch_one` frei und
wandert mit in die lib. Wenn nein → der Council entscheidet die Naht
(z. B. die beiden Funktionen ebenfalls nach `mathematikerin.rs` und die
lib ruft sie über eine injizierte Funktion, oder ein kleines
gemeinsames Modul).

## Ziel-Schnitt (drei ehrliche Namen)

1. **`src/mathematikerin.rs` (neu):** `mod mathematikerin`
   (11 883–15 418) + die WGSL-Strings vom main.rs-Anfang. Publiziert,
   was main.rs braucht (`pub fn` für `star_position_at`, `system_now`,
   `pack_window` etc.). `mod mathematikerin;` in main.rs wird zu
   `mod mathematikerin;` mit `#[path]` ODER main.rs bekommt
   `mod mathematikerin;` und das File heißt passend (Cargo-Konvention:
   `src/mathematikerin.rs` + `mod mathematikerin;` in main.rs).

2. **`src/archivar.rs` (lib, wächst):** die Hülle aus main.rs wandert
   hierher — alles Archivar: `fetch_one` + CDN-Maschine,
   `resolve_asset`, `build_buffer`, `query_hash`, `sense_*`,
   `build_netcdf_channels`/`build_finals`/`build_ionex`, `anchor`,
   `law_bounds`, `body_pole_at`, `gravity_manifest`, `kernel_extent`,
   `OriginState`, `sensor_config`, `parse_station*`, `fanout_fetch`,
   die port-*-Maschine, die probe_*-Helfer — **sofern** der eine Faden
   (build_curve_set/build_planet_set) in die lib gezogen wurde.
   Der Hauptpfad bleibt: der Probe erbt weiter `omegaflow::archivar`.

3. **`src/main.rs` (dünn):** bleibt `fn main`, der ω()-Loop
   (`main_flow`), die Radiatoren, `mod relay`, die Verdrahtung.
   `mod archivar` verschwindet; `use omegaflow::archivar::*;` kommt an
   die Stelle. `mod mathematikerin` wird zum eigenen File.

## Vorgehen (die Session)

1. **Baseline:** `cargo check` 0/0 + `cargo test` grün (lib + bin).
2. **Council hält die Schnittliste** (einmal, vor dem Schnitt —
   Architektur-Entscheid). Die Stimmen lesen zuerst die drei Regionen.
3. **Faden prüfen:** `build_curve_set`/`build_planet_set` lesen —
   rein oder nicht? Entscheidung + Registratur in TODO.
4. **mathematikerin.rs extrahieren** (erst der GPU-Teil, da abgegrenzt).
5. **Hülle in die lib ziehen** (der größte Schritt — `cargo check` nach
   jedem Stück).
6. **main.rs schrumpfen:** Streichungen + Importzeilen — dieselbe
   Gate-Disziplin wie Atom 1.
7. **Gates.**

## Gates

- `cargo check` 0 Fehler, 0 Warnungen (default **und**
  `--features browser_relay` — relay lebt in main.rs).
- `cargo test --lib` + `cargo test --bin omegaflow` grün.
- `git diff src/main.rs` Zeile für Zeile: nur Streichungen + Importzeilen,
  keine Logik-Änderung (Zeilen-Mengen-Differenz gegen HEAD verifiziert,
  wie Atom 1).
- `cargo run` → nicht-schwarzes Fenster (0 honored: schwarz nur, wenn
  beabsichtigt).
- TODO.md im selben Commit aktualisiert.

## Regeln, die der Schnitt respektiert

- Die deutschen Eigennamen bleiben (Archivar, Mathematikerin) — sie
  benennen das Handwerk, nicht den Beruf. Kein „archiver", kein
  „mathematician".
- „membran" ist im Vokabular verboten — die Hülle heißt Laufzeit, nicht
  Membran.
- Kein Kommentar, kein `#[allow]` als Schweige-Instrument, `cargo check`
  bleibt 0/0.
- Ein Commit ist ein Häkchen: TODO und Code im selben Commit.

## Der Satz (Betriebsverfassung)

Vorschlag vor Schnitt: Befund, Abweichung vom Auftrag, kleinster wahrer
Schnitt, Verifikation. Der Operator entscheidet; ohne sein Wort kein
Schnitt.
