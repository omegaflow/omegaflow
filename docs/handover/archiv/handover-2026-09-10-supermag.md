<!--
  title: Handover — SuperMAG in die Pipeline + cddis-EOP-Disposition (2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: 3774caa771f553e1444f093a16047859fd8c15186f8f0e6a1953d53864128da4
  status: archived
  see-also: docs/handover/handover-2026-09-10-autonom.md docs/handover/handover-2026-09-10-nicht-autonom.md
-->
# Handover — SuperMAG in die Pipeline + cddis-EOP-Disposition (2026-09-10)

## Erledigt (trägt Git)

- **supermag_compiler.rs** — gebaut (std-only + curl, data-api.php → GeoRec-Bin).
  `--station <IAGA> --start <ISO> --days <n> --out <bin> --lsk <naif0012.tls>
  [--ci-mode]`. data-api.php-Pagination (28-d-Takt, `extent`), `parse_json` +
  `jpath` für die sechs Komponenten (N/E/Z × nez/geo), magstid.php-Ernte für
  geolat/geolon (geolon 0..360 → −180..180), `--lsk` unix→TDB, GeoRec-Schreiber
  MAGIC `SMG1`, Roundtrip-Verify. Pilot: TRO 2025-03 → 267486 GeoRecs (16,0 MB),
  roundtrip parses.
- **Format supermag_1m** — registriert an den vier Berührstellen: geo.rs
  (MAGIC_SMG + COMP_SMG_1..6 + magic_of/comp_max), extract.rs
  (geo_series_component_name), main_flow.rs (geo-series-match). Zwei Tests
  (roundtrip + register-field-match) grün.
- **sources.φ-Block** — `supermag_tro_2025-03.bin` (CDN-Asset), `format
  supermag_1m`, `at earth` (Position im Record, nie von Hand im Register),
  sechs `field`-Zeilen (em nT, τ 86400).
- **supermag-cdn.yml** — der Manifestator (workflow_dispatch: station/start/
  days/asset; Idempotence-Skip; naif0012.tls-Fetch; `--ci-mode`).
- **cddis finals2000A** — Review-Frage geschlossen: EOP-Duplikat der maia
  finals.all → `decline duplicate-eop` (dead_sources.φ); der cddis-Block in
  blocked_sources.φ bleibt RINEX-GNSS (nicht EOP). Ledger-Kandidat entfernt.
- **maia-finals2000A-Zweit-Datei** — hält (genau eine maia-Datei,
  sources.φ:3836, keine zweite).

## Gemessen (die API, 2026-09-10)

- data-api.php (`fmt=json&logon=omegaflow&start=<ISO>&extent=<sec>&all&station=<IAGA>`)
  → `OK\n` + JSON-Array: `tval` (epoch s), `ext` (60.0), `iaga`, `N/E/Z` je
  `{nez, geo}` (nT). Daten-Lag: 2026-09-01 leer, 2025-03 voll (Monate).
- Fill-Wert = 999999.0. TRO 2025-03: 40320 Records, 59 voll-absent (alle sechs
  = 999999.0), 0 partiell → absent-Regel: Komponente = 999999.0 → Slot
  übersprungen (0-Kanon); ein echtes 0.0 nT fließt als 0.0.
- magstid.php → Stations-Liste mit geolat/geolon (einzige Koordinaten-Quelle;
  keine Elevation → alt = 0.0 Pad, benannte Abwesenheit).
- inventory.php → 194 Stations-Codes (keine Koordinaten).

## Pending

- **CDN-Manifestation** — das Asset `supermag_tro_2025-03.bin` liegt erst lokal;
  der Manifestator ist `supermag-cdn.yml` (workflow_dispatch). Der Upload ist
  CI-Sache (der Manifestator ist der einzige CDN-Schreiber).
- **ESO** — bleibt offen (kein Eintrag) — Ernte + Duty, eigenes Atom (die
  Review-Frage ist im Ledger benannt).
- **Vollernte der 194 Stationen** — die Körnung (ein Bin je Station vs
  Fenster-Bins) ist eine Bauentscheidung nach der Piloternte; der Pilot (TRO)
  misst den Maßstab (Request-Budget je Station, extent-Pagination).
