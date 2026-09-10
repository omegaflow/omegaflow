<!--
  title: Handover — NOAA-NODD-Compiler-Atom (2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: a861133acfc1782623247cfe9705fe5d33a3784d4b7788728eb779724cab8651
  status: archived
  see-also: docs/handover/handover-2026-09-10-autonom.md
-->
# Handover — NOAA-NODD-Compiler-Atom (2026-09-10)

## Erledigt (trägt Git)

- **Drei Compiler gebaut** — `noaa_ghcn_compiler`, `noaa_gsod_compiler`,
  `noaa_isd_compiler` (tools/harvest/src/bin/) mit gemeinsamem Parser
  `src/archivar/noaa_nodd.rs` (csv_fields, parse_ghcn_stations, parse_ghcn,
  parse_gsod, parse_isd, tdb_window, filter_window). Registriert: magic/comp in
  geo.rs (MAGIC_GHCN/GSOD/ISD, comp-Sätze), component names in extract.rs,
  geo-series-Routing in main_flow.rs, mod.rs + lib.rs. Die Parser sind gegen
  echte Bucket-Bytes gemessen (GHCN unquoted `csv/by_station/{id}.csv`, GSOD/ISD
  quoted `{year}/{station}.csv`); 0 honored durchgehend — Sentinel-Sprünge
  (−9999, 9999.9/999.9/99.99, 9000/90000-Codes), Nonneg-Gates nur wo die Physik
  es fordert (PRCP/SNOW/SNWD/WDSP/GUST/SLP), negative Temperaturen passieren,
  fehlend bleibt absent.
- **sources.φ-Blöcke** — formats `noaa_ghcn_d` / `noaa_gsod` / `noaa_isd`
  (at earth, ttl 604800, 9-Token-Fields in comp-Reihenfolge).
- **Drei CDN-Workflows** — noaa-ghcn-cdn.yml, noaa-gsod-cdn.yml,
  noaa-isd-cdn.yml (workflow_dispatch, Idempotenz-Gate, --ci-mode).
- **Tests** — drei (roundtrip, register-field-match, Parser-gegen-Bytes);
  cargo check + cargo test grün.
- **Zwei Re-Litmus descoped** — swpc + swdi (Befunde
  noaa_nodd_disposition.φ:205/208): swpc trägt dieselben Live-Rolling-Windows
  wie sources.φ (kein Bulk-Archiv); swdi trägt Radar-Ableitungen ohne
  Boden-Magnitude (kein tornado-*/wind-*).

## Offen (Register-Duty)

- **CDN-Manifestation** — die drei Pilot-Assets antworten 404 (gemessen); der
  erste Dispatch der drei workflow_dispatch-Workflows steht aus
  (Operator-Wort).
- **All-Stationen-Schleife** — ghcn/gsod/isd über alle Stationen je
  Jahres-Fenster (S3-Listing; GHCN Stations-Walk + csv/by_station/{id}.csv).
  Das Korn ist ein Station-Fenster, die Schleife das nächste Atom.
- **14 Compiler-Lease offen** — ledger.φ `compiler noaa-nodd-14-leases`
  (lidar, dcdb, eri, gk2a, goes16, himawari8, jpss, cors-RINEX, ccor-FITS,
  nexrad, gdp-drifter, keo-papa, ocs-hydrodata, wod).

## Beobachtet (nicht diese Session)

- **Parallele Arbeit im Worktree** — eine gleichzeitige Session trägt
  uncommitted Änderungen in denselben Dateien (geo.rs station-Feld +
  SMG_REC_BYTES, supermag_compiler.rs, number_audit.rs, ps1_coverage,
  antares_loci, vo-tap, job_dashboard, eso-harps). Die drei Compiler lesen
  GeoRec mit `station: 0` (für die 60-Byte-Record-Formate nicht serialisiert).
  Der Commit dieses Atoms ist offen: die eigenen Änderungen liegen mit der
  fremden Arbeit in denselben Dateien (geo.rs, extract.rs, tests.rs, sources.φ)
  verzahnt — ein Commit jetzt nähme fremde Arbeit mit; die Koordination steht
  vor dem Commit.
