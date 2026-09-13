<!--
  title: Handover — Bau & Code (Stand 2026-09-13, Bau19)
  session: Bau-Folge
  class: handover
  date: 2026-09-13
  sha256: f70f24e47e1b42c62c9d3c1e9820331ff257947b1f0e090518c89f200d37806a
  status: live
-->
# Handover — Bau & Code (2026-09-13, Bau19)

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
  vorhanden; `cargo build --release` mit gesourctem `export-esp.sh`). Offen, wenn
  das Modul an der Reihe ist: `espflash` installieren und das Gerät anstecken
  (heute kein ESP32 enumeriert — `lsusb` ohne Espressif/CP210x/FTDI); dann läuft
  der nn-Strom als `nn=<ms>` am ttyACM. Die kuratierte BOM
  (`docs/specs/mantis-shrimp-bom.md`) und der AliExpress-Warenkorb (45 Artikel)
  stehen bereit.

## DSM/Topo — die offenen Punkte

- **WGSL-Membran-Konsument des volume-bin-Felds** — ungebaut (nächstes Atom,
  Operator-Wort): Force-Branch + Draht + `constants.js` + Dispatcher. Die Membran
  hat weiter keinen Konsumenten für die 11 Tomographie-Modelle (`format reference`,
  sha256-verankert in `phi/sources.φ`).
- **volume_builder verifizieren** — `tools/utils/src/bin/volume_builder.rs` kompiliert,
  lief aber gegen kein Modell: die NetCDF-Arbeitskopie (BBNAP19-MASK-3D) ist nicht
  lokal gemessen. Der Builder verweigert bei nicht-depth-lat-lon-Achsenordnung.

## LASzip — die offenen Punkte

- **LASzip-Chunk-Dekoder** ungebaut: Arithmetic-Coder + Chunk-Tabelle (VLR
  `laszip encoded`, record 22204) + Attribut-Modelle. `src/archivar/las.rs` liest
  Header/VLRs/COPC-Info+Hierarchie + ept.json; die .laz-Punktdaten bleiben
  komprimiert — `blocked parser-def las-laz` trägt den Rest. Die Arbeitskopie
  (19961009ATM2_143020JR.copc.laz, 127733 Pkt / 4 Knoten, Punktformat 6) lag unter
  `/tmp/opencode/copc_hier.bin` — beim Neustart verloren; vor dem Bau die Kopie neu
  ziehen (s3://noaa-nos-coastal-lidar-pds, `phi/blocked_sources.φ`).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`). Kein Push, solange der Baum
nicht ruhig ist.
