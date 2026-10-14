<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-14
  sha256: dc8aca83de4c53302589262f71f11ea384dde994d464cdb51cffe4282e12d3e4
  status: live
  see-also: AGENTS.md
-->
# Standpass — öffentliche Projektion (privat↔öffentlich, A)

Die öffentliche Fassung des Stehenden Passes (Operator-Wort A, 2026-10-14). Sie trägt
**nur, was öffentlich ist** — HEAD und rote CI-Läufe. Mailstand, Operator-Akte und
Personenbezug bleiben im privaten Haus (`state/zustand/standing-pass.md`, Linie Mycelium).
Die öffentlichen Linien zitieren **diese** Fassung, nie die private — so wird der einzige
leck-fähige Faden (das Register-Lesen) geschnitten, nicht nur verschoben.

| Feld | Wert | gemessen |
|---|---|---|
| HEAD | `6192d3206` | 2026-10-14 |
| rote CI-Läufe | `ci-gate` (format: rustfmt-Diff in `tools/service/src/bin/llm_gateway.rs:1244`; register: `cdn_reconcile --fail`, 5 ungebundene CDN-Netlocs → Mountain) · `register-coverage` (`register_lookup --tokens`: 21 unbeglichen, 7 XFER-MISSING) — je Lauf `ci_manage list` am HEAD; die HEAD-Läufe (`38626650325` ci-gate, `38626650196` register-coverage) laufen neu | 2026-10-14 |

Die Meta-Linie (Mycelium) trägt beide Zeilen in demselben Rund, in dem sie den privaten
Pass schreibt — öffentlich ins öffentliche Haus, privat ins private.
