<!--
  title: Handover: HDF5-Reader + NCEI-SSI-Ernte (die leuchtende Sonne)
  class: handover
  date: 2026-08-21
  sha256: e2707eeeeb576d38306a948afe848ee722b0b32598942f21b320cdfa0372a598
  see-also: docs/SOURCE_PORT.md docs/concepts/sources-v2-spec.md
-->
# Handover: HDF5-Reader + NCEI-SSI-Ernte

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # fremde Arbeit benennen, nichts übernehmen
cargo check                      # muss 0/0 sein
curl -sS -r 0-7 "https://www.ncei.noaa.gov/data/solar-spectral-irradiance/access/standard-resolution/monthly/ssi_v03r00-preliminary_monthly_s202604_e202606_c20260804.nc" | xxd
                                 # erwartet: 89 48 44 46 0d 0a 1a 0a  (.HDF...)
```

Referenzen (stehend): `src/spectral.rs` (spectra.bin-Kontrakt + λ→ν + Farb-LUT),
`src/bin/spectral_compiler.rs` (CSV → spectra.bin → CDN), `src/netcdf.rs`
(classic-netcdf-Parser — NICHT HDF5), `src/inflate.rs` (DEFLATE, schon da),
`phi/sources.φ:1719` (der `format spectral`-Block, wartet auf spectra.bin),
`src/cdn.rs` (Upload), `docs/SOURCE_PORT.md` (der eine Pfad für Source-Arbeit).

## Auftrag

Zwei Dinge, in dieser Reihenfolge:

A. **Einen allgemeinen HDF5-Reader in reinem Rust schreiben** (`src/hdf5.rs`).
   Wir brauchen HDF5 ohnehin — nicht nur für diese eine Datei. Der Reader
   ist ein Gold-Standard-Parser wie `netcdf.rs`/`cdf.rs`: **Rust std-only,
   keine externen Crates** (der Stack ist std + curl, AGENTS.md). Es gibt
   bestehende Crates (`hdf5`, `hdf5-metno`, die C-Bibliothek libhdf5 und die
   offizielle HDF5-Format-Spec) — die dürfen als **Inspiration und
   Format-Referenz** gelesen werden, aber unser Parser bleibt eigen, std-only,
   getestet. Kein `extern crate hdf5`, kein FFI, kein Shell-out (`ncdump`).
B. **Die NCEI-SSI-Ernte** (der registrierte `pending`-Punkt „Ernte NCEI-SSI
   netCDF-4/HDF5"): die SSI-Dateien sind netCDF-4 = HDF5. Der neue Reader
   liest sie, der `spectral_compiler` (oder ein kleiner neuer Harvest-Binary)
   baut daraus `spectra.bin` und lädt es auf den CDN (`--ci-mode`). Danach
   leuchtet die Sonne: die Membran misst ihr volles em-Spektrum, das zu
   ~1361 W/m² bei 1 AU integriert.

## Verifizierter Kontext (2026-08-21, selbst vermessen)

- **Datensatz:** NCEI „Solar Spectral Irradiance CDR" (Config ID 01B-33),
  PI Odele Coddington (CU-LASP), DOI 10.25921/esjz-1w61. Einheit
  **W m⁻² nm⁻¹ bei 1 AU**. Zwei Produkte: „standard-resolution" (= Broad
  Spectrum, 0–200.000 nm, ~4300 variable Bänder) und „high-resolution"
  (115–500 nm, ~9700 Bänder). Variablen: daily, monthly, yearly (monthly
  ab 1874, yearly ab 1610, bis heute).
- **Zugang (offen, kein Key):**
  `https://www.ncei.noaa.gov/data/solar-spectral-irradiance/access/`
  → `standard-resolution/` bzw. `high-resolution/` → `monthly/` (und
  `daily/`, `yearly/`). THREDDS-Katalog:
  `https://www.ncei.noaa.gov/thredds/catalog/cdr-solar-spectral-irradiance/catalog.html`
- **Dateinamen:** `ssi_v03r00_monthly_s<YYYYMM>_e<YYYYMM>_c<YYYYMMDD>.nc`
  (v03r00 = Version 3.0). Aktuellste vorläufige:
  `ssi_v03r00-preliminary_monthly_s202604_e202606_c20260804.nc`.
- **Container bestätigt HDF5:** Magie `89 48 44 46 0d 0a 1a 0a`
  (`\x89HDF\r\n\x1a\n`). **Nicht** classic netCDF (`CDF\x01`). Der
  `src/netcdf.rs`-Parser liest nur classic — HDF5 ist die Lücke.
- **Größe:** ~172 KB pro standard-resolution-monthly-Datei (3 Monate) —
  klein, gut für Tests und den Reader-Prototyp.
- **Die Spektral-Pipeline steht schon komplett** — es fehlt NUR die Ernte:
  - `src/spectral.rs`: Kontrakt `0xCF 0x86 0x01 [epoch_tdb:f64] [count:u32]`
    LE (15 B Header) + count × `[freq, bin_width, val]` f64 LE (24 B/Band);
    `bins_from_lambda_rows` (λ→ν, E_ν = E_λ·λ²/c, bin_width aus dem nativen
    λ-Gitter); `write_spectral_bin`/`parse_spectral_bin`; `month_middle_unix`;
    die Farb-LUT (BP−RP → Teff → RGB, Pecaut–Mamajek + Helland).
  - `src/bin/spectral_compiler.rs`: liest CSV
    `wavelength_nm, irradiance_W_m2_nm, uncertainty_W_m2_nm, quality_flag`,
    flag≠0 / nicht-endlich / nicht-positiv fallen (0 honored), baut das Bin,
    `--ci-mode` lädt via `upload_asset` auf den CDN (Asset `spectra.bin`
    unter dem Release-Tag `ssd.jpl.nasa.gov`). `--month YYYY-MM` setzt die
    Epoche (Monatsmitte, TDB via LSK).
  - `phi/sources.φ:1719`: `url …/spectra.bin` + `format spectral` +
    `field irradiance spectral_irradiance_W_m2_Hz inverse-square em W/m2/Hz
    2628000` — der Block wartet nur auf das Asset (heute 404).
- **Der `wm2_1au`-Weg ist NICHT dieser Auftrag** — er ist schon erledigt
  (Luminositäts-Atom 2026-08-20, `ledger.φ:582`). Die schmalen Solar-Bänder
  (GOES X-Ray/EUV, RTSW) stehen in `sources.φ`. Die Sonne leuchtet erst
  bolometrisch, wenn `spectra.bin` existiert.

## Session-Fragen / Arbeitsschritte

A1. HDF5-Reader-Umfang (Gold-Standard, aber kein Vollformat-Krieg): Superblock
    (v0/v1/v2/v3), Object-Header (v1/v2) mit den Messages dataspace/datatype/
    fill-value/link-info/group-info/attribute, Gruppenglieder via B-Tree v1 +
    Fractal-Heap, Chunked-Datasets via Chunk-B-Tree, Filter deflate (→
    `src/inflate.rs`), shuffle, fletcher32, scaleoffset. Kontiguous-Datasets.
    Was netCDF-4 konkret schreibt, bestimmt die Tiefe — zuerst EINE echte
    Datei laden und den Superblock/Object-Header-Baum ausgeben (Probe-Binary
    `--probe <file.h5>` analog `cdf.rs`/`--probe`).
A2. netCDF-4-Schicht: netCDF-4 ist HDF5 mit Konventionen (Dimensionsscalen,
    `_Netcdf4Coordinates`). Variablen der SSI-Datei identifizieren:
    wavelength, ssi (irradiance W/m²/nm), uncertainty, quality_flags,
    time. Das ist die Tabellenform des `spectral_compiler`.
A3. Ernte: entweder `spectral_compiler` um `--input-nc <file.nc>` erweitern
    oder ein kleiner neuer Binary (z. B. `ncei_ssi_compiler`), der den
    Reader nutzt, die Rows extrahiert und `bins_from_lambda_rows` +
    `write_spectral_bin` aufruft. Monatlich eine Datei; `--month YYYY-MM`
    aus dem Dateinamen (s…/e…-Fenster → Monatsmitte, NICHT fetch-Zeit).
    Die unit-Konvention steht: λ in nm, E_λ in W/m²/nm, flag u8.
A4. CDN: `spectra.bin` bauen und `--ci-mode` hochladen (ersetzt den 404).
    Vorher den 404-Befund dokumentieren (dead_sources/ledger?), nachher
    den `pending`-Eintrag schließen.
A5. Register: `ledger.φ` (der pending-Punkt), `TODO.md` („Ernte NCEI-SSI
    pending" → erledigt), AGENTS.md Zeile zum Spektral-Oszillator
    („Ernte … pending" → seit wann sie läuft). `src/spectral.rs`-Kopfzeile
    („harvest step pending") mitschärfen.

## Gates

- cargo check 0/0 (beide Features), cargo test. Der HDF5-Reader trägt Tests
  gegen die echten SSI-Dateien (eine vorläufige 2026er + eine historische,
  z. B. s187405) — roundtrip, kopf-los, kein Fenster, keine Radiatoren.
- `spectra.bin` roundtrip-geprüft (`parse_spectral_bin`); die Bänder zählen
  (~4300) und die Integralsumme ≈ 1361 W/m² bei 1 AU (physikalische
  Plausibilität, keine Fabrikation — die Summe IST die Messung).
- Register-Update im selben Commit. Ein Commit je Einheit.
- Diese Datei nach Abschluss archivieren (Regel in AGENTS.md).

## Nicht anfassen

Die Membran/`fs`-Rendering-Physik (frisch befriedet: nur em trägt Farbe,
nicht-em neutral, log-konsistentes Tone-Mapping), `src/te.rs`, der
`wm2_1au`-Luminositäts-Pfad, die GOES/EUV-Schmalband-Quellen, die
OMEGAFLOW_HIDDEN-Radiator-Stille. Reines HDF5 + SSI-Ernte.
