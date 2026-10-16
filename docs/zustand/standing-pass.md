<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-17
  sha256: 7562c1235133f84983e05c29080f6e50df73e600e6fd8cc5219cefba864d25c0
  status: live
  see-also: AGENTS.md
-->
# Standpass — öffentliche Projektion (privat↔öffentlich, A)

Die öffentliche Fassung des Stehenden Passes (Operator-Wort A, 2026-10-14). Sie trägt
**nur, was öffentlich ist** — HEAD und rote CI-Läufe. Mailstand, Operator-Akte und
Personenbezug bleiben im privaten Haus (`state/zustand/standing-pass.md`, Linie Mycelium).
Die öffentlichen Linien zitieren **diese** Fassung, nie die private.

| Feld | Wert | gemessen |
|---|---|---|
| HEAD | `4e2ff67d8` (== `origin/main`) | 2026-10-17 |
| rote CI-Läufe | **612** non-success über 12 Seiten — Träger: `ci-gate` (353) · `register-coverage` (155) · `tools-build` (44) · `quake-feeds-cdn` (14) · `matrix-rotor` (11) · `m10-mdis-raw-cdn` (8) · `health-check` (6) · `zenodo-raw-cdn` (4) · `gic-al-network` (4) · `timed-orbit-cdn` (2) · `geofon-waveform-cdn` (2) · `ci-check` (2) · je 1: `wdcb-obn-wayback-cdn` · `venus-radar-cdn` · `themis_asi-fodg-cdn` · `ssd-jpl-raw-cdn` · `hips-png-cdn` · `de2-cdn` | 2026-10-17 |

Der `ci-gate`-Kern am HEAD `4e2ff67d8` (Lauf `38975908313`): `build`, `register`, `clippy` grün;
`format`, `subset`, `dropped-gate` rot. Der `format`-Drift ist in diesem Rund geräumt.
