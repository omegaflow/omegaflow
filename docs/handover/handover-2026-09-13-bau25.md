<!--
  title: Handover — Bau & Code (Stand 2026-09-13, Bau25)
  session: Bau-Folge
  class: handover
  date: 2026-09-13
  sha256: ab318597ff09857613a184d8c1b4ee0b3494abc3242e2941b9d59df07ce5bdc0
  status: live
-->
# Handover — Bau & Code (2026-09-13, Bau25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Membran — die offenen M-Punkte

- **ESP32-Modul — on hold** (Operator-Wort, 2026-09-13): das Gerät und sein
  Flash kommen zuletzt — zuerst laufen Software und Membranen. Der Binär baut
  (Xtensa-Toolchain in `~/.rustup/toolchains/esp/`, Linker `xtensa-esp32s3-elf-gcc`
  vorhanden). Offen, wenn das Modul an der Reihe ist: `espflash` installieren
  und das Gerät anstecken (kein ESP32 enumeriert — `lsusb` ohne
  Espressif/CP210x/FTDI); dann läuft der nn-Strom als `nn=<ms>` am ttyACM. Die
  kuratierte BOM (`docs/specs/mantis-shrimp-bom.md`) und der AliExpress-Warenkorb
  (45 Artikel) stehen bereit.

## LASzip — die offenen Punkte

- **Format-7/8/9/10-Layer (Rgb14/RgbNir14/Wavepacket14) unabhängige Referenz
  pending** — kein unkomprimiertes .las-Gegenstück existiert. Gemessen
  (2026-09-13): laspy und laz-rs führen .las/.laz-Paare nur für Format 0–6;
  `fullwave.laz` (Format 10) trägt nur das `fullwave.wdp`-Begleitfile, kein
  .las. Der Layered-POINT14-Kern inkl. GPS-Time ist unabhängig verifiziert
  (Format-6-Paar `1_4_w_evlr.las/.laz`, laspy, Kreuz-Digest gegen die
  unkomprimierte Referenz). Offen bleibt die unabhängige Referenz für die
  Format-10-spezifischen Reader Rgb14/RgbNir14/Wavepacket14 — ihre Digests
  bleiben Selbst-Pins, weil der durchsuchte Korpus kein Format-8/10-.las führt.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`). Kein Push, solange der Baum
nicht ruhig ist. Fremde uncommittete Arbeit (Parallel-Sessions):
`src/archivar/{bzip2.rs,hdf5.rs,mod.rs,netcdf.rs}` (+ unversioniert
`hsd.rs,mat5.rs,nexrad.rs`), `tools/measure/src/{mww.rs,rest.rs}`,
`tools/harvest/src/bin/ephemeris_compiler.rs`,
`tools/utils/src/bin/archive_search.rs`, `phi/blocked_sources.φ`,
`.github/workflows/kernel-flatten.yml` — unberührt.
