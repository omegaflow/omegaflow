<!--
  title: Handover — Bau & Code (Stand 2026-09-13, Bau25)
  session: Bau-Folge
  class: handover
  date: 2026-09-13
  sha256: 3726edcb3d674d46c1bd8c70c3cdb72ed644626cd173d1b7ecdfdd240e124f7a
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

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`). Kein Push, solange der Baum
nicht ruhig ist. Fremde uncommittete Arbeit (Parallel-Sessions):
`src/archivar/{bzip2.rs,hdf5.rs,mod.rs,netcdf.rs,parquet.rs}` (+ unversioniert
`hsd.rs,mat5.rs,nexrad.rs`), `tools/measure/src/{mww.rs,rest.rs}`,
`tools/harvest/src/bin/ephemeris_compiler.rs`,
`tools/utils/src/bin/archive_search.rs`, `phi/blocked_sources.φ`,
`.github/workflows/kernel-flatten.yml` — unberührt.
