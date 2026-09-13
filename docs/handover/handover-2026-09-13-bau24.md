<!--
  title: Handover — Bau & Code (Stand 2026-09-13, Bau24)
  session: Bau-Folge
  class: handover
  date: 2026-09-13
  sha256: beef2667ec17ed28d1a22d7198aee920ae5360affc0adffb5f02c45c30ada299
  status: live
-->
# Handover — Bau & Code (2026-09-13, Bau24)

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

- **Pointwise BYTE (Extra-Bytes) Item unbuilt** — blockiert die
  Format-4/5-WAVEPACKET13-Verifikation. Gemessen (2026-09-13): `pdrf4-1.3.laz`
  (Format 4, 1024 Pkt, loaders.gl) trägt neben
  POINT10/GPSTIME11/WAVEPACKET13 ein pointwise BYTE-Item (Typ 0);
  `decode_chunk_pointwise` refused es als `LasNote::LazItem { item: 0 }`. Der
  WAVEPACKET13-Reader ist gebaut, teilt aber keine verifizierte Fixture — der
  einzige Format-4/5-Fund im durchsuchten Korpus trägt das ungebaute BYTE-Item.
  Der unkomprimierte Format-4-Reader ist verifiziert (`pdrf4-1.3.las`, 1024 Pkt,
  Waveform + GPS-Time, Digest gepinnt).

- **fullwave.laz unabhängige Referenz pending** — kein unkomprimiertes
  .las-Gegenstück existiert (gemessene Absenz; die Waveform braucht das
  `fullwave.wdp`-Begleitfile). Provenienz benannt: byte-identisch zu
  `laspy/laspy` master (blob-sha256 `8d49c1c4…`). Der Dekode-Digest bleibt ein
  Selbst-Pin.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`). Kein Push, solange der Baum
nicht ruhig ist. Fremde uncommittete Arbeit (Parallel-Sessions):
`src/archivar/{ck.rs,fk.rs,hdf5.rs,mod.rs}`, `tools/measure/src/{lib.rs,mww.rs,rest.rs}`,
`tools/harvest/src/bin/ephemeris_compiler.rs`, `phi/blocked_sources.φ`,
`docs/paper/depth-phase-echo-fleet.md`, `docs/handover/handover-2026-09-13-forschung-folge14.md`
— unberührt.
