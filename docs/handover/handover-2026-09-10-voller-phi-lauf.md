<!--
  title: Handover — Voller phi-Lauf (Stand 2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: efa0109244d20e7e6db4d2e3d2190a7efd2a8897efa2ef7e3a3ea132fcdba572
  status: live
  see-also: docs/handover/handover-2026-09-10-autonom.md
-->
# Handover — Voller phi-Lauf (2026-09-10)

## Erledigt (trägt Git)

- **Voller Lauf über den bereinigten phi-Bestand** — Linse (`source_url_candidates`)
  → `probe_sweep` → Review (Schritt 4) auf der kartierten Struktur
  (`MANIFEST.φ`), lokal, Review in der Session. Kette gemessen: 30 Kandidaten
  (25 bekannt, 3 bare DONKI-Präfixe, 2 neu zu `master_urls`) → 30 checked,
  19 live, 0 jina-json, 15 drafted, 13 Blöcke nach Totfilter, 7 Source-Blöcke
  → 3 verifiziert + 4 declined.
- **Review-Ergebnis: 0 neue Quellen.** Alle 7 Endpoints sind bereits in
  `phi/sources.φ` mit korrekter Einheit registriert — density kg/m3, crosswind
  m/s, F nT, Distance m, Absolute_Ne m-3, Absolute_VTEC TECu. Die 2
  `master_urls`-neuen Kandidaten — das imag-data.bgs.ac.uk
  GIN-HAPI-`{station}`-Template (sources.φ:3823) und vires `GF_OPER_NE__KBR_2F`
  (sources.φ:15) — stehen beide schon im Register. Der Bestand trägt keine
  unregistrierte Quelle; die Null ist die Wahrheit der Messung (0 honored).
- **Probe-Autoableitung benannt (nicht gebaut)** — zwei Survivor-Felder waren
  falsch abgeleitet: crosswind als kg/m3 (registriert m/s), density Num(inf).
  Die registrierten Einträge tragen bereits die korrekte Einheit; die
  Einheit-Autoableitung der Probe bleibt offen.
- **Ledger** — `verifiziert kandidat voller-phi-lauf` hält das Messergebnis;
  die Reports `phi/reports/probe_sweep_survivors.φ` + `probe_sweep_void.txt`
  sind versioniert (Commit = Häkchen).

## Offen (Register-Duty, nichts gebaut/geerntet)

- **Probe-Einheit-Autoableitung** — die Probe leitet crosswind als kg/m3 ab
  (die registrierte Quelle trägt m/s); die Autoableitung ist eine Register-Duty,
  kein Befund.
- **bucket_litmus auf weitere Inventare** — aus `phi-struktur` übernommen:
  Copernicus-Inventar u.a. liegen im Katalog; `bucket_litmus
  phi/pipeline/decline_lens.φ <inventar.φ> [--calibrate <disposition.φ>]`.
- **19 Compiler-Lease** — aus `bucket-litmus-nachtrag` unverändert offen
  (ledger.φ `compiler noaa-nodd-19-leases`).

## Beobachtet (nicht diese Session, uncommitted im Worktree)

- `tools/service/src/assets/job_dashboard.html` (M), `tools/vo-tap/src/lib.rs`
  (M) — unangetastet (andere Linie).
