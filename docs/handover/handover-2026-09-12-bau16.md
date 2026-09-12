<!--
  title: Handover — Bau & Code (Stand 2026-09-12, Bau16)
  session: Bau-Folge
  class: handover
  date: 2026-09-12
  sha256: 96ec3d0ea28e32ba057c9ece0b701d46d3f528310a344e2060ca4fd8f5dacd31
  status: live
-->
# Handover — Bau & Code (2026-09-12, Bau16)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Membran — die offenen M-Punkte

- **Flashen + Live-Messen (nn-Strom)** — der Binär baut: die Xtensa-Toolchain
  liegt in `~/.rustup/toolchains/esp/` (Linker `xtensa-esp32s3-elf-gcc` vorhanden),
  `cargo build --release` gelingt mit gesourctem `export-esp.sh`. Offen ist der
  Draht zum Gerät: kein ESP32 ist enumeriert (`lsusb` ohne Espressif/CP210x/FTDI,
  `/dev/ttyACM*` und `/dev/ttyUSB*` leer) und `espflash` (der Runner) ist nicht
  installiert. Flashen braucht das angesteckte Gerät + `espflash`; dann läuft der
  nn-Strom als `nn=<ms>` am ttyACM.

## Eclipse-Schattenortung — offene Code-Punkte

- **Kalibrier-Gate-Test** (`eclipse_shadow_probe.rs`): trägt noch `LINES[0]`
  (=de441, gedriftet → läse 42,5 km) und eine stille Early-Return bei fehlendem
  `data/` (`cargo test` läuft aus der Crate-Root → No-op). Gehört zur
  de441-Re-Ernte.
- **2024-Kanon-Punkt:** Katalogzeile 09561 trägt den Punkt gerundet (25N 104W);
  ein präziser Punkt käme aus der TSE2024-Detailseite (nur wenn gewollt).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`). Kein Push, solange der Baum
nicht ruhig ist (eine andere Session arbeitet parallel).
