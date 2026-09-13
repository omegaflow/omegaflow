<!--
  title: Handover — Bau & Code (Stand 2026-09-13, Bau22)
  session: Bau-Folge
  class: handover
  date: 2026-09-13
  sha256: 77ce34c56b80f9bbd242987fab1af559ffc8be78f7169006cc0e38306c77e9de
  status: live
-->
# Handover — Bau & Code (2026-09-13, Bau22)

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

## CDN-Manifestation — die offenen Punkte

- **CI-Lauf pending**: `volume-cdn.yml` trägt jetzt 10 Jobs (8 + AFRP20 + AFRP22;
  LITHO1.0 ist descoped mit Befund am Read-Site in `phi/sources.φ`). Der Lauf
  (workflow_dispatch, Idempotenz-Skip je Asset) ist der Schritt des
  Manifestators, nicht der Session.

## LASzip — die offenen Punkte

- **RGB/NIR/Waveform-Extra-Items UNVERIFIED** (Formate 2/3/5/7/8/9/10): beide
  Wurzeln sind gebaut — layered `RGB14/RGBNIR14/WAVEPACKET14/BYTE14` (Formate
  7/8/9/10, auf dem bestehenden POINT14) und pointwise
  `POINT10/GPSTIME11/RGB12/WAVEPACKET13` (Formate 0–5, neuer Pfad
  `decode_chunk_pointwise`, compressor 2). `LasPoint` trägt jetzt
  `waveform: Option<WavePacket>` (Deskriptor/Pointer-Felder, Samples nie
  fabriziert). Was fehlt: eine echte Fixture mit RGB/NIR/Waveform und einem
  festgehaltenen Digest — gemessen nicht gefunden (USGS 3DEP EPT = Format 1/6
  ohne RGB; noaa-coastal = Format 6; der pointwise-Pfad decodierte die
  Format-6-Regression 157710 Pkt sauber, refused aber typisierte
  Extra-Bytes-Items `BYTE/SHORT/INT/LONG/FLOAT/DOUBLE` — gemessen am 3DEP-OriginId
  (`BYTE`=0, size 4) — korrekt via `LasNote::LazItem`, nicht gebaut; das
  layered-`BYTE14` ist gebaut). Ein Legacy-Digest
  (Format 3/5) bleibt die gemessene Verifikations-Duty.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`). Kein Push, solange der Baum
nicht ruhig ist. Fremde uncommittete Arbeit (Parallel-Sessions):
`phi/blocked_sources.φ`, `src/archivar/{fk.rs,ck.rs,mod.rs}`,
`tools/measure/src/{lib.rs,mww.rs,rest.rs}` — unberührt.
