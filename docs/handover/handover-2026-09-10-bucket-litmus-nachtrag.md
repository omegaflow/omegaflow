<!--
  title: Handover — Bucket-Litmus-Nachtrag (Stand 2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: 6c13f82a460bedbd6bba25711d1d0d14686134f0bdad855e205f7685f91a4f4a
  status: live
  see-also: docs/handover/handover-2026-09-10-autonom.md
-->
# Handover — Bucket-Litmus-Nachtrag (2026-09-10)

## Erledigt (trägt Git)

- **Pending-Bucket verifiziert** — der Registry-Name
  `s3://-noaa-ufs-gefsv13replay-pds` ist ein Tippfehler: der führende
  Bindestrich löst DNS 000, der echte Name `noaa-ufs-gefsv13replay-pds`
  antwortet HTTP 200 (gemessen). Kette auf dem verifizierten Namen: Frage 1
  nein (UFS GEFS v13 Replay = Modell-Output; der Bucket trägt
  Forcing/Restart-netCDF runoff, salt_restore, interpolate_zgrid), Frage 4
  kein Konsument → descoped. Disposition + Inventar korrigiert
  (`noaa_nodd_disposition.φ:217`, `noaa_nodd_inventory.φ:83`).
- **19 Compiler-Lease registriert** — ein gruppierter `ausstehend`-Block in
  `phi/pipeline/ledger.φ` (`compiler noaa-nodd-19-leases`) benennt die 19
  Compiler samt Bucket und Manifestations-Duty (CDN via --ci-mode); die
  Disposition bleibt die Quelle, der Ledger-Eintrag macht die Pflichten
  findbar.
- **Ledger-Nachtrag** — `bucket-litmus-nachtrag` (verifiziert) hält das
  Messergebnis; Zählung jetzt 0 pending, 79 descoped.

## Offen (Register-Duty, nichts gebaut/geerntet)

- **19 Compiler bauen** — benannte Pflichten für die Nachfolge-Atome
  (ledger.φ `compiler noaa-nodd-19-leases`); je Lease Manifestations-Duty.
  In diesem Atom wurde kein Compiler gebaut oder geerntet.

## Beobachtet (nicht diese Session, uncommitted im Worktree)

- `tools/harvest/src/bin/ephemeris_compiler.rs` (M),
  `tools/measure/src/bin/te_series_periodicity_probe.rs` (??) — unangetastet.
