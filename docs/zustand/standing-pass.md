<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-17
  sha256: a07028bdd8dd0bb409c470eccfcf02ff56be5cc443c0885ad730a8284f61f1c0
  status: live
  see-also: AGENTS.md
-->
# Standpass — öffentliche Projektion (privat↔öffentlich, A)

Die öffentliche Fassung des Stehenden Passes (Operator-Wort A, 2026-10-14). Sie trägt
**nur, was öffentlich ist** — HEAD und rote CI-Läufe. Mailstand, Operator-Akte und
Personenbezug bleiben im privaten Haus (`state/standing-pass.md`, Linie Mycelium).
Die öffentlichen Linien zitieren **diese** Fassung, nie die private.

| Feld | Wert | gemessen |
|---|---|---|
| HEAD | `f53afdffc` (== `origin/main`) | 2026-10-17 |
| rote CI-Läufe | **611** non-success über 12 Seiten — Träger: `ci-gate` (358) · `register-coverage` (151) · `tools-build` (44) · `quake-feeds-cdn` (13) · `matrix-rotor` (10) · `m10-mdis-raw-cdn` (8) · `health-check` (5) · `zenodo-raw-cdn` (4) · `gic-al-network` (4) · `venus-radar-cdn` (2) · `timed-orbit-cdn` (2) · `geofon-waveform-cdn` (2) · `ci-check` (2) · je 1: `wdcb-obn-wayback-cdn` · `themis_asi-fodg-cdn` · `ssd-jpl-raw-cdn` · `hips-png-cdn` · `de2-cdn` · `.github/workflows/gic-al-network.yml` | 2026-10-17 |

Der `ci-gate`-Kern am HEAD `f53afdffc`: `build`, `register`, `clippy`, **`format`** grün;
`subset` (3 archivar-Tests) und `dropped-gate` (sechs neue Drops) rot.
