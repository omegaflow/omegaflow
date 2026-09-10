<!--
  title: Handover — Bucket-Litmus-Anwendung (Stand 2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: 9b86748ed855013b7ab84dcbd65704eb5f72f299c2b580b518f1d720c809b39f
  status: live
  see-also: docs/handover/handover-2026-09-10-autonom.md
-->
# Handover — Bucket-Litmus-Anwendung (2026-09-10)

## Erledigt (trägt Git)

- **Bucket-Litmus-Anwendung** — die 101 NOAA-NODD-Datasets disponiert nach der
  4-Fragen-Kette (Rats-Verdikt `ledger.φ:937`). Register:
  `phi/pipeline/catalog/noaa_nodd_disposition.φ`. 99 Datasets nach
  Duplikat-Kollaps (sea-ice ×2, goes16 ×2): 19 compiler-lease (Compiler pending
  = benannte Register-Duty), 1 pending (unverifizierter Bucket-Name
  `-noaa-ufs-gefsv13replay-pds`), 79 descoped, 0 konsument, 0 url-line.
- **Zwei Rats-Sitzungen**: (1) Architektur — Register-Ort = eigene Datei neben
  dem Inventar; Zeilenformat zwei Zeilen je Dataset (Kopfwort + note mit der
  Kette); (2) Abschluss — das Blatt gehalten, sechs Zeilen korrigiert
  (MRMS / S-102 / Nationalbathymetry → descoped; CORS → compiler-lease;
  SWPC / SWDI-Noten geschärft).
- **Anschluss-Punkte**: Inventar-Kopf (`noaa_nodd_inventory.φ`) verweist auf das
  Register; die offene Klammer in `ledger.φ:937` geschlossen (neuer
  verifiziert-Eintrag `bucket-litmus-anwendung`).

## Offen (Register-Duty, nichts gebaut/geerntet)

- **19 Compiler-Lease mit Compiler pending** — benannte Pflichten für das
  Nachfolge-Atom: noaa_nos_lidar, noaa_dcdb_bathymetry, noaa_eri, noaa_ghcn,
  noaa_gk2a, noaa_goes, noaa_gsod, noaa_himawari, noaa_isd, noaa_jpss,
  noaa_cors, noaa_ccor, noaa_nexrad, noaa_gdp_drifter, noaa_keo_papa,
  noaa_ocs_hydrodata, noaa_swpc, noaa_swdi, noaa_wod. Jeder Lease trägt
  Manifestations-Duty (CDN-Asset via --ci-mode).
- **1 pending** — `-noaa-ufs-gefsv13replay-pds` (unverifizierter Bucket-Name,
  führender Bindestrich): Register-Duty verifizieren, dann Kette.
- Die übrigen Zeilen des Autonom-Handovers bleiben live
  (`handover-2026-09-10-autonom.md`); der Gegen-Handover unberührt.

## Kontext

- Agenten-Rebind (global, `~/.config/opencode/opencode.jsonc`): build → pro/high,
  plan → pro/max, council → pro/max, grind-pro → pro/high, grind-flash →
  flash/low; Backups daneben (`.bak-2026-09-10-litmus`). Gilt ab opencode-Neustart;
  betrifft die Delegation des Nachfolge-Atoms.
- `.gitignore` trägt eine Ausnahme für `noaa_nodd_disposition.φ` (Registrierungs-
  Urteil, versioniert wie dead/blocked — der Rat wählte „ein Dokument in catalog/",
  das Register trägt daher eine eigene Ausnahme).

## Beobachtet (nicht diese Session, uncommitted im Worktree)

- `tools/harvest/src/bin/ephemeris_compiler.rs` (M), `tools/measure/src/bin/te_series_periodicity_probe.rs`
  (??) — TE-Serie-Periodizität,
  steht uncommitted aus einer früheren Session; dieses Atom fasst sie nicht an.
