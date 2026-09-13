<!--
  title: Handover — Bau & Code (Stand 2026-09-13, Bau23)
  session: Bau-Folge
  class: handover
  date: 2026-09-13
  sha256: ac99f0524275526e4a3aee9b727c9cdc7b5514ba67a7bbbb7b4c1fe48385e5bd
  status: live
-->
# Handover — Bau & Code (2026-09-13, Bau23)

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

- **CDN-Lauf 34774414830 offen**: `volume-cdn.yml` (10 Jobs) dispatched
  (2026-09-13T18:22Z, workflow_dispatch); die 10 Assets landen über den Lauf auf
  `omegaflow/sources` — offen, bis der Wächter die Landung je Asset misst
  (Idempotenz-Skip je Asset; LITHO1.0 descoped, Befund am Read-Site in
  `phi/sources.φ`).

## LASzip — die offenen Punkte

- **Pointwise WAVEPACKET13 (Formate 4/5) unverified**: gebaut, keine echte
  Fixture in den durchsuchten Korpora — gemessene Absenz; der verifizierte
  WAVEPACKET14-Pfad (fullwave.laz, Format 10) teilt keinen Code mit dem
  pointwise Item.
- **Layered RGB14 (Format 7) als Standalone-Item unverified**: der
  RGB-Kanal-Codec (`decode_rgb`) ist über RGBNIR14/fullwave.laz (Format 10)
  verifiziert; ungeprüft bleibt der Standalone-Pfad (POINT14+RGB14-Komposition,
  eigene `Rgb14Reader`-Kontextführung) — keine Format-7-Fixture, gemessene
  Absenz.
- **Layered BYTE14 (Extra-Bytes) unverified — im Bau22-Umfang gebaut**: keine
  Fixture mit Extra-Bytes in den durchsuchten Korpora; der eine reale Fund
  (USGS-3DEP-OriginId, BYTE=0/size 4) nimmt die typisierte Refusal
  (`LasNote::LazItem`), nicht den BYTE14-Pfad — gemessene Absenz.
- **Digest-Provenienz benannt**: die drei Pins (simple.laz Format 3,
  autzen_trim.laz Format 3 110k, fullwave.laz Format 10) sind Selbst-Pins —
  Regressions-Pins, keine unabhängige Referenz-Decodierung; die unabhängige
  Referenz-Decodierung (laspy) bleibt eine Register-Duty.
- **Pointwise Kompositionen Format 0/2 ohne Pin**: POINT10 allein und
  POINT10+RGB12 teilen die verifizierten Reader (Format 1/3), die Item-Listen
  selbst sind nicht gepinnt — gemessene Absenz.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`). Kein Push, solange der Baum
nicht ruhig ist. Fremde uncommittete Arbeit (Parallel-Sessions): unberührt.
