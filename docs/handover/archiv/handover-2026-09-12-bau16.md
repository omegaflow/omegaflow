<!--
  title: Handover — Bau & Code (Stand 2026-09-12, Bau16)
  session: Bau-Folge
  class: handover
  date: 2026-09-12
  sha256: 73e8c99ed6dfed924d079c381667d2f2fd4db0df868c83b2e4712aa6873c6085
  status: archived
-->
# Handover — Bau & Code (2026-09-12, Bau16)

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
