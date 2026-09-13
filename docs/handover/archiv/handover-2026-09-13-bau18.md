<!--
  title: Handover — Bau & Code (Stand 2026-09-13, Bau18)
  session: Bau-Folge
  class: handover
  date: 2026-09-13
  sha256: 498f5e7ec19bf17a3031cfe500e2bb2656537ad1f8457360f8f454290e5cdd90
  status: live
-->
# Handover — Bau & Code (2026-09-13, Bau18)

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

## archive_search — die offenen Punkte (Operator-Wort, 2026-09-13)

- `--index`-Invalidierung flach — Root-mtime, nicht rekursiv; der Inhalt bleibt
  immer frisch (nie gecacht), die Struktur-Invalidierung ist shallow.
- `JINA_API_KEY` in `.secrets.local` ungenutzt — die freien Endpunkte tragen die
  Leiter (r.jina.ai / s.jina.ai ohne Key).

## DSM/Topo — die offenen Punkte

- Positive-Maske-Feldkonsument (Register-Duty): der Grid→Feld-Evaluator + das
  Volume-bin-Format ist ungebaut. Die 11 Tomographie-Modelle stehen als
  `format reference` in `phi/sources.φ` (sha256-verankert); der HDF5-Parser
  liest sie (mask [13,297,425] an BBNAP19-MASK-3D). Die Membran hat keinen
  Konsumenten.
- LASzip-Chunk-Dekoder: Arithmetic-Coder + Chunk-Tabelle ungebaut.
  `src/archivar/las.rs` liest Header/VLRs/COPC-Info+Hierarchie (127733 Pkt /
  4 Knoten, 19961009ATM2_143020JR.copc.laz) + ept.json (AK_BrooksCamp_2012).
  Die .laz-Punktdaten bleiben komprimiert — `blocked parser-def las-laz` trägt
  den Rest.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`). Kein Push, solange der Baum
nicht ruhig ist.
