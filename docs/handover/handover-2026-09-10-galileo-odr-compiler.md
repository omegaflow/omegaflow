<!--
  title: Handover — Galileo-ODR-Compiler: die 10 ODR-Dateien zu galileo_odr.bin verpackt
  class: handover
  date: 2026-09-10
  sha256: c0cd672180ff24031c5f2f84070ea8d74c631816e83b11334751de42667effae
  status: live
  see-also: docs/handover/handover-2026-09-10-autonom.md
-->
# Handover — Galileo-ODR-Compiler (2026-09-10)

## Angenommen

- Das erste Galileo-Floor-Atom des Autonom-Handovers: `galileo_odr_compiler`
  + `galileo-odr-cdn.yml` — die 10 lokalen ODR-Dateien
  (`data/pds-ppi.igpp.ucla.edu/galileo_goj_odr/`) als CDN-Asset manifestieren,
  origin-verbatim, kein Regal.

## Geleistet

- `tools/harvest/src/bin/galileo_odr_compiler.rs` gebaut (std-only): liest die
  10 ODR-Dateien (lokal `--dir` oder fetch aus `GO-J-RSS-1-ODR-V1.0` /
  `GO-JS-RSS-1-ODR-V1.0`), prüft die Provenance (sha256-Gate gegen die
  gemessenen 10 Digests aus `sha256.txt`), verpackt origin-verbatim in
  `galileo_odr.bin` (Magic `GODR`, Datei-Tabelle mit Name + sha256 +
  Record-Zahl + Sample-Rate + Offset/Länge), Roundtrip-Parse verifiziert die
  Treue, `--ci-mode` lädt auf den CDN-Release `pds-ppi.igpp.ucla.edu`.
- `.github/workflows/galileo-odr-cdn.yml` gebaut (Vorbild
  `galileo-receiver-cdn.yml`): Idempotenz-Gate, Release-Sicherung, Compile+Upload.
- Lokal verifiziert: `cargo check` 0 Warnungen; 3 Gate-Tests grün
  (hex32-Roundtrip, Sample-Rate-Wort, Binary-Roundtrip); Lauf über die 10
  Dateien — Provenance hält für alle 10, Roundtrip-Fidelity hält.
- Messergebnis: `70580900.ODR` trägt 13 864 Records + 880 Folgebytes — kein
  ganzes Record-Multiple; die Folgebytes sind im Asset origin-verbatim
  erhalten (nicht verworfen, nicht aufgefüllt). Asset: 10 Dateien,
  312 272 126 B.

## Offen

- **CDN-Dispatch** — der Workflow manifestiert `galileo_odr.bin` erst nach
  dem Push (`gh workflow run galileo-odr-cdn.yml`); der Dispatch ist der
  Folge-Schritt (CI).
- **880 Folgebytes von `70580900.ODR`** — erhalten, aber nicht gedeutet: ob
  PDS-Fußstruktur oder abgeschnittener Record, ist ungemessen; ein Probe kann
  die Bytes inspizieren (Folge-Atom, keine Eile).
- Die fünf übrigen Galileo-Floor-Atome stehen unverändert im Autonom-Handover
  (die erste Zeile wurde dort durch das Ergebnis ersetzt).

## Archiv

- Keine Übergabe archiviert: das Autonom-Handover bleibt das stehende
  Tagesregister; die Galileo-Floor-Zeile wurde dort in-place gearbeitet.
