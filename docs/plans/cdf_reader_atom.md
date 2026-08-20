# Session-Plan: cdf_reader-Atom — die CDF-Ernte (LIRA + Berkeley)

Registriert 2026-08-20. Träger zweier Payloads: das LIRA-BIA-E-Feld
2023–2025 (die solare electric-Lücke) und PSP-VSC (Berkeley).

## Einstieg (neue Session, null Vorkontext)

1. `docs/SOURCE_PORT.md` — das eine Arbeitsprotokoll für alle Source-Arbeit.
2. `phi/pipeline/research/agent_output/solar_akteure_probe.φ` — trägt alle
   verifizierten Befunde (AMDA/RPW, CDAWeb-Katalog, LIRA-Fund, TS1/TS2).
3. Ledger: `kraft-kanal electric` (die LIRA-Ernte wartet dort auf den
   cdf_reader), die cdf_reader-pending-Zeilen in TODO.md.
4. `cargo check` als Null-Referenz (muss 0/0 bleiben).
5. Test-Artefakt laden (eine echte LIRA-Datei):
   `https://rpw-lira.obspm.fr/roc/data/pub/solo/rpw/data/L3/lfr_efield/2025/12/solo_L3_rpw-bia-efield_20251231_V01.cdf`

## Verifizierter Kontext (aus der Solar-Akteure-Session)

- **LIRA-Baum** (das französische Primärzentrum, LESIA→LIRA-Redirect):
  `https://rpw-lira.obspm.fr/roc/data/pub/solo/rpw/data/L3/` mit
  `lfr_efield/` (Jahre 2020→2025, Tagesdateien
  `solo_L3_rpw-bia-efield_YYYYMMDD_V01.cdf` bis 2025-12-31),
  `lfr_density/` (bis 2025-04), `lfr_scpot/`, `lfr_vht/`,
  `mag-scm_merged/`, `thr_flux/`, `tnr_fp/` + `db.csv`
  (Katalog-Index: `relative_filepath,start_datetime,stop_datetime`).
- **Das Bin-Format existiert**: `rpw_efield.bin` (magic RPW1, Record
  `[t_tdb f64, val f64, comp u32]`, comp 1=Ey 2=Ez) + der Block
  `format rpw_efield at solar_orbiter` in `phi/sources.φ`. Der Compiler
  (`src/bin/rpw_compiler.rs`) trägt `--merge` (merge in die bestehende
  Bin) und `--window-start/--window-end`; Position = Solar-Orbiter-
  Ephemeride am Sample-Epoch (der Archivar rechnet sie selbst, kein
  Positions-Baking nötig).
- **CDAWeb-BIA endet 2022-12-01** — LIRA trägt 2023–2025; die
  „2022-2026-Lücke" ist eine Host-Lücke, keine Mess-Lücke.
- **PSP-VSC**: kalibriertes mV/m-Feld nur bei Berkeley (CDF),
  CDAWeb-HAPI 1406 — Route ist die erste Session-Frage.
- Referenzen: cdf.gsfc.nasa.gov (CDF-3-Spezifikation), RPW-DPDD
  (ROC-PRO-DAT-NTT-00075, rpw-datacenter.obspm.fr).

## Bausteine

1. **`src/cdf.rs`** (std-only): CDF-3-Parser — CDR (Compression/Rekord-
   Sätze), Variable Descriptor Records (vDR), zVDR-Datenrecords,
   `CDF_EPOCH`/`CDF_EPOCH16`/`CDF_TIME_TT2000`, big-endian, row-major.
   Scope: uncompressed + RLE; gzip über das vorhandene `src/inflate.rs`;
   Huffman/Adaptive als parser-gap registriert (0 honored — kein
   fabrizierter Partial-Parse). Tests gegen die echte LIRA-Datei aus
   Schritt 5 (Roundtrip: Zeitbasis + EDC_SRF-Werte plausibel, mV/m,
   endlich, fill-Skips).
2. **`src/bin/bia_efield_compiler.rs`**: `db.csv` (oder Baum-Crawl)
   indexieren → Tages-CDFs → `EDC_SRF [3]` mV/m → 10-min-Mediane
   (Statistik der 10-s-Reihe, wie das AMDA-Kompilat — benannt) →
   `--merge` in `rpw_efield.bin` → 2023–2025 ans Kompilat,
   `--ci-mode`-Upload. Der bestehende Block trägt dann 2020–2025,
   ohne neuen Block. Erste Session-Frage: Kompressionsart +
   EDC_SRF-Record-Layout (beim ersten echten File notieren).
3. **PSP-VSC**: Berkeley-Route suchen (erste Session-Frage). Gefunden →
   derselbe Parser erntet (eigenes Bin oder Merge, je nach Frame —
   `at parker_solar_probe`); nicht gefunden → Registrierpflicht bleibt
   (pending, kein Default).
4. **Register**: Ledger (cdf_reader-Einträge schließen/narrowen), TODO,
   `solar_akteure_probe.φ`. Offen bleibt: TRACERS-SPK (eigenes Atom —
   kein SPK bei NAIF, verifiziert), TS1/TS2-Block.

## Gates

- `cargo check` 0 Fehler + 0 Warnungen (alle Feature-Kombinationen).
- CDF-Parser-Tests gegen die echte LIRA-Datei + Roundtrip-Tests.
- Compiler-Lauf: Bin wächst um die 2023–2025-Masse, `--merge`-Integrität.
- Smoke: `cargo run` → `rpw …: N oscillators` (N ≈ 147148 + Neuernte).
- Ein Commit je in sich geschlossener Einheit; der letzte schließt die
  TODO-Items. CDN-Upload via gh (Login steht, OMEGAFLOW_TOKEN hat
  read:org/repo/workflow).

## Offene Session-Fragen

1. Kompressionsart + EDC_SRF-Layout der bia-efield-CDFs.
2. Berkeley-VSC-Route (URL-Baum oder API).
3. Ob die 2020–2022-LIRA-CDFs mit dem AMDA-Kompilat deckungsgleich sind
   (sonst Merge-Dedupe prüfen — `dedup_by (t, comp)` existiert im
   Compiler).
