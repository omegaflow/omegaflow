<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: 72a18ca5a9d70df16e6d0aa4d73228d558bbac2fc4069f4bf411691152df75cd
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
| HEAD | `20e8d7e36` | 2026-10-15 |
| rote CI-Läufe | `ci-gate` `38645988368` (am HEAD neu gestartet; gemessen lokal: der Format-Job bricht an `tools/service/src/bin/llm_gateway.rs:1265` — der Ganz-Crate-Format-Riss, kein Linien-Eigentum; der register-Job ist **clean**: `cdn_reconcile --fail` → `cap+tag contract clean (324 registry hosts)`, `register_sort` canonical) · `register-coverage` `38645988529` (am HEAD neu gestartet; `register_lookup --tokens --noecho-fail` = 4 XFER-MISSING im `## An mycelium`-Block von `mountain-folge347`; `--orphans --fail` = 0) | 2026-10-15 |

Die Meta-Linie (Mycelium) trägt beide Zeilen in demselben Rund, in dem sie den privaten
Pass schreibt — öffentlich ins öffentliche Haus, privat ins private.
