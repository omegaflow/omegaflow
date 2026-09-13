<!--
  title: Handover — Bau & Code (Stand 2026-09-13, Bau21)
  session: Bau-Folge
  class: handover
  date: 2026-09-13
  sha256: 45d107e2738402da7b1da058c182234e3ff854c06646f0fb42d904126513c1a2
  status: live
-->
# Handover — Bau & Code (2026-09-13, Bau21)

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
  und das Gerät anstecken (heute kein ESP32 enumeriert — `lsusb` ohne
  Espressif/CP210x/FTDI); dann läuft der nn-Strom als `nn=<ms>` am ttyACM. Die
  kuratierte BOM (`docs/specs/mantis-shrimp-bom.md`) und der AliExpress-Warenkorb
  (45 Artikel) stehen bereit.

## CDN-Manifestation — die offenen Punkte

- **3 volume.bin-Assets refused** (gemessen, nie fabriziert): der
  `volume_builder`-Kontrakt (rank-3-float + depth/lat/lon-Koordinatenvariablen)
  trägt drei der 11 Tomografie-Modelle nicht — LITHO1.0 (kein rank-3-float-Dataset),
  AFRP20-MASK-3D und AFRP22-MASK-3D (Achse 1 matcht mehr als eine
  Koordinatenvariable). Die 8 gebauten Modelle stehen in `phi/sources.φ`
  (`format volume`, sha256 je Asset gemessen — BBNAP19-Parität `88f3552d…` hält)
  und in `volume-cdn.yml`. Die 3 stehen `pending`: der Builder braucht eine
  Achsen-Disambiguierung bzw. das LITHO1.0-Layout.
- **CI-Lauf pending**: `volume-cdn.yml` (workflow_dispatch, Idempotenz-Skip je
  Asset) manifestiert die 8 Assets zum CDN — der Lauf ist der Schritt des
  Manifestators, nicht der Session.

## LASzip — die offenen Punkte

- **byte14-Extra-Bytes ungebaut** (RGB/NIR/Waveform-Formate 2/3/5/7/8/9/10):
  der POINT14-Layered-Dekoder (`src/archivar/las/laszip.rs` — Arithmetic-Coder,
  Integer-Compressor, Chunk-Tabelle aus der Trailing-Struktur) dekodiert
  Punktformat 6; die Fixture `cache/copc_hier.copc.laz` (127733 Pkt / 4 Knoten)
  läuft mit Parität (Punktzahl, Min/Max in den Header-Grenzen). Layouts mit
  Zusatz-Items tragen `LasNote::LazItem` (named, nie fabriziert). Die
  byte14-Folgepunkt-Dekodierung steht offen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`). Kein Push, solange der Baum
nicht ruhig ist. Fremde Arbeit beim Schreiben dieser Übergabe (Parallel-Session,
gestaged + untracked): `.github/workflows/kernel-flatten.yml`,
`phi/sources.φ`-Hunks (mpcorb, aia2014_lines), `src/archivar/bsp_reader/spk.rs`,
`src/archivar/ephemeris.rs`, `tools/harvest/src/bin/{hinet_win32_compiler,mpcorb_compiler}.rs`,
`tools/utils/src/bin/archive_search.rs/json.rs/net.rs`, Handover-Umzüge und
`docs/handover/handover-2026-09-13-entscheid-folge5.md` — unberührt.
