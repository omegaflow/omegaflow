<!--
  title: Handover — Bau & Code (Stand 2026-09-13, Bau20)
  session: Bau-Folge
  class: handover
  date: 2026-09-13
  sha256: 6d7c99f9a8021426a1f4a440ce4454aa35a3d493b265f1ac71c9420d8e03a820
  status: live
-->
# Handover — Bau & Code (2026-09-13, Bau20)

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

- **CDN-Manifestation der 11 volume.bin-Assets** (Register-Duty): der
  WGSL-Membran-Konsument ist gebaut (`Buffer.volumes` → GPU-Buffers → `sample_volume`
  in `presence_probe`, Summe auf force 3 seismic-body; CPU-seitig die geodätische
  Presence-Konvertierung `icrs_to_body_geodetic` pro Frame; WGSL offline-validiert
  via `field_wgsl_validates_offline`). Der Konsument bleibt ohne Daten, solange die
  `.volume.bin` nicht als `format volume` am CDN stehen. Offen: `volume_builder`
  `--ci-mode` (NetCDF-Quelle laden → volume.bin → Upload), ein CI-Schritt (eigener
  Workflow, nie kernel-flatten häufen) und die `phi/sources.φ`-Zeilen (11 Modelle,
  sha256 je Asset — gemessen bisher nur BBNAP19-MASK-3D: `88f3552d…`). Die
  NetCDF-Originale bleiben `format reference`. Die geodätische Konvertierung
  verankert die Presence auf „earth" (alle 11 Modelle sind Erdmodelle); außerhalb
  der Domäne trägt das Grid 0.0 (null-echt).

## LASzip — die offenen Punkte

- **LASzip-Chunk-Dekoder** ungebaut: Arithmetic-Coder + Chunk-Tabelle (VLR
  `laszip encoded`, record 22204) + Attribut-Modelle. `src/archivar/las.rs` liest
  Header/VLRs/COPC-Info+Hierarchie + ept.json; die .laz-Punktdaten bleiben
  komprimiert — `blocked parser-def las-laz` trägt den Rest. Die Arbeitskopie
  (19961009ATM2_143020JR.copc.laz, 127733 Pkt / 4 Knoten, Punktformat 6) lag unter
  `/tmp/opencode/` — beim Neustart verloren; vor dem Bau die Kopie neu ziehen
  (s3://noaa-nos-coastal-lidar-pds, `phi/blocked_sources.φ`).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`). Kein Push, solange der Baum
nicht ruhig ist. Fremde uncommittete Arbeit beim Schreiben dieser Übergabe:
`.github/workflows/kernel-flatten.yml`, `phi/sources.φ`,
`src/archivar/{bsp_reader/spk.rs,ephemeris.rs,pck.rs}`,
`tools/harvest/src/bin/ephemeris_compiler.rs`,
`tools/measure/src/{bin/depth_phase_fleet_probe.rs,bin/placebo_pair_eeg_probe.rs,eeglab.rs}`,
`.github/workflows/placebo-ave-cdn.yml`,
`docs/handover/handover-2026-09-13-forschung-folge13.md`,
`src/archivar/kernels/naif_spacecraft_ids.tsv`, `.playwright-mcp/`,
`tools/utils/src/bin/archive_search/playwright*` — unberührt.
