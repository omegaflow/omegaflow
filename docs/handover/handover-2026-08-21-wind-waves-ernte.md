<!--
  title: Handover: Wind/WAVES-Ernte 1994–2021 + Frame at wind
  class: handover
  date: 2026-08-21
  sha256: de61df15f612d7e1268eedf4c88a5a7f5b3a19ca3cb335f3e82078b7b357e95e
-->
# Handover: Wind/WAVES-Ernte 1994–2021 + Frame at wind

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # fremde Arbeit nennen (ephemeris-Refactor)
cargo check                      # meine Dateien 0/0; fremde Warnungen nennen
cargo run --bin wind_waves_compiler -- --probe /tmp/opencode/wav_h1_20210101.cdf
```

Referenzen (stehend): `src/bin/wind_waves_compiler.rs` (die Ernte),
`src/wind.rs` (Bin magic WAV1), `src/cdf.rs` (der Parser), `src/rpw.rs`
(Referenz-Muster RPW1), `phi/pipeline/research/agent_output/berkeley_wind_2026-08-21.φ`
(Befunde der Vorgänger-Session, selbsttragend). Der cdf_reader steht seit
2026-08-20; die wav_h1-Ernte ist seit 2026-08-21 Prototyp-stabil.

## Verifizierter Kontext (2026-08-21)

- **wind_waves_compiler erntet und funktioniert.** Prototyp 2021-01: 31 Tage,
  0 void, 18848 Records (608 Bins/Tag = RAD2 256 + RAD1 256 + TNR 96),
  Tages-Median je Frequenzbin, Bin `wind_waves.bin` (magic WAV1, Record
  [t_tdb f64, freq_hz f64, bin_width_hz f64, val_v f64, receiver u32]),
  Roundtrip ✓. CLI: `--year --month --lsk src/kernels/naif0012.tls --out`.
- **Layout (1994 + 2021 geladen):** beide CDF 3.x (3.6.3 / 3.8.0) — KEIN
  CDF 2.6, `src/cdf.rs` bleibt unverändert. Encoding 1 (BE), majority 2,
  keine Kompression (VVR kind 7). 11 zVariables: Epoch (CDF_EPOCH, 1-min-
  Kadenz, erste Probe 00:00:30), E_VOLTAGE_RAD1/RAD2 (REAL4 f32[256]),
  E_VOLTAGE_TNR (REAL4 f32[96]), Minimum_voltage_*, Frequency_RAD1/RAD2
  (INT2 kHz), Frequency_TNR (REAL4 kHz), Epoch2. Frequenzachsen: RAD1
  20–1040 kHz (4 kHz Raster), RAD2 1075–13825 kHz (50 kHz), TNR 4–245 kHz
  (log). ~3,58 MB/Tag.
- **Force-Gate-Urteil steht:** E_VOLTAGE = „normalized receiver average
  voltage" V — force em (die Radio-Welle ist der Messwert selbst, nicht V/m).
  freq/bin_width speisen die Spektral-Oszillator-Achse.
- **SPDF-Tree:** `spdf.gsfc.nasa.gov/pub/data/wind/waves/wav_h1/` lebt, direkt
  erreichbar (kein Jina nötig). Jahresverzeichnisse 1994–2021, Dateien
  `wi_h1_wav_YYYYMMDD_v01.cdf`.

## Auftrag

Zwei Folge-Atome des geschlossenen kraft-kanal electric:

A. **Volle Ernte 1994–2021** (1994-11-10 → 2021-12-31, ≈ 9900 Tage):
   Compiler auf Ganzjahr/Mehrjahr + Parallelität erweitern (Muster:
   `bia_efield_compiler` mit `--jobs` + Atomic-Counter), CI-Job in
   `kernel_flatten.yml` (Asset-Guard, `--ci-mode` Upload als
   `wind_waves.bin` in den Release `spdf.gsfc.nasa.gov`), dann CDN. Masse:
   ≈ 9900 Tage × 608 Bins ≈ 6,0 M Records ≈ 216 MB Bin; Download ≈ 33 GB
   (sequentielle Ernte ~4 s/Tag → parallelisieren). Erst 1994 (das früheste
   Jahr) als Stichprobe über alle 12 Monate laufen lassen — der 1994-11-10-
   Befund deckt bereits, dass auch 1994 CDF 3.x ist.
B. **Frame at wind:** BODY_REGISTRY kennt wind nicht, kein SPK. Befund
   (2026-08-21): NAIFs PDS-Archiv (`naif.jpl.nasa.gov/pub/naif/pds/data/`)
   trägt KEIN WIND-Bundle (Liste komplett — kein Wind). NAIF-ID −485.
   Kandidaten prüfen: Missions-SPK (Wind-MFI/-WAVES-Team), SSCWeb-Bahn,
   generische NAIF-`archived`-Kernels. Ohne Fund: `at wind` bleibt pending
   (0 honored — keine fabrizierte L1-Position), die Ernte trägt dann
   t_tdb/freq/val ohne Raumposition. Erst nach SPK-Fund: Block in sources.φ
   (format wind_waves, force em, V, gaussian-inverse-square).

## Gates

- cargo check 0/0 (meine Dateien; fremde Warnungen der ephemeris-Arbeit
  benennen, nicht flicken), cargo test gegen die echten wav_h1-Dateien
  (1994 + 2021, kopf-los) — sobald der fremde Baum wieder test-grün ist.
- Volle Ernte roundtrip-geprüft; CDN-Upload byte-identisch verifiziert.
- Register: ledger.φ (kraft-kanal electric wind-waves → erledigt/verifiziert),
  TODO.md. Befund in `phi/pipeline/research/agent_output/` fortgeschrieben.
- Ein Commit je Einheit; diese Datei nach Abschluss archivieren (AGENTS.md).

## Nicht anfassen

Im Baum läuft fremde, teils uncommitete Arbeit am Ephemeriden-Refactor
(`src/ephemeris.rs`, `src/bin/spk_split.rs`, `src/bsp_reader/*`,
`src/bin/ephemeris_compiler.rs`, `src/bin/horizons_compiler.rs`,
`.github/workflows/kernel_flatten.yml`, `phi/pipeline/katalog/asteroid_gm_sb441.φ`
u. a.) — der lib-test-Build bricht daran (archivar.rs/mathematikerin.rs).
Erst `git status` lesen; nicht deren Dateien anfassen; `kernel_flatten.yml`
erst editieren, wenn der Refactor committet ist.
