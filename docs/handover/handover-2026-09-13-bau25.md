<!--
  title: Handover — Bau & Code (Stand 2026-09-13, Bau25)
  session: Bau-Folge
  class: handover
  date: 2026-09-13
  sha256: 2ff8302b9475cccec88ba875f0e0c6f83e9c4b33e62c525a083045b7222b3192
  status: live
-->
# Handover — Bau & Code (2026-09-13, Bau25)

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

## archive_search — die Werkzeug-Naht (Session 2026-09-14)

- Gebaut (gepusht): Exit-Leiter (direct→proton*→socks, on-block) + Rate-Gate;
  `magic`-Sniffer; PDF-Stripper; `--datacite`; `--sniff` (magic+sha256); `--zenodo`;
  `--isc` (FDSN); `--openalex`; Auto-Pagination (crossref/zenodo/openalex);
  EarthData-Token-Hook (401, einmal); `--supermag`; `--heasarc`. Offen: der
  Token-Hook ist live unverifiziert — Schritt: `EARTHDATA_USER`/`EARTHDATA_PASS`
  setzen, `--sniff`/`--isc` gegen eine EarthData-URL proben. ISC-EHB-Grammatik
  (`web-db-v4` `out_format`) bleibt offen/HTML — Schritt: das Format wiegen.

## S3-Reader — die größte Werkzeug-Lücke

- EarthData-Ports (GRACE-FO/SWOT PODAAC, SMAP, CDDIS IONEX) sind token-gated
  (403/307); GES-DISC `/data/` + AppEEARS `/api/product` sind anonym 200. Fehlt:
  `s3://`→HTTPS-Mapping + EDL→S3-Credentials-Exchange (`.../s3credentials`) + SigV4
  in `src/archivar/range.rs`, Header-Injektion in `xml_harvester`. Schritt:
  `range.rs` erweitern, Test gegen `podaac-ops-cumulus-public`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`). Kein Push, solange der Baum
nicht ruhig ist. Fremde uncommittete Arbeit (Parallel-Sessions):
`src/archivar/{bzip2.rs,hdf5.rs,mod.rs,netcdf.rs,parquet.rs}` (+ unversioniert
`hsd.rs,mat5.rs,nexrad.rs`), `tools/measure/src/{mww.rs,rest.rs}`,
`tools/harvest/src/bin/ephemeris_compiler.rs`,
`tools/utils/src/bin/archive_search.rs`, `phi/blocked_sources.φ`,
`.github/workflows/kernel-flatten.yml` — unberührt.
