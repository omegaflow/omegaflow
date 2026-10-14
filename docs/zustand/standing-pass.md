<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-14
  sha256: 724067219e62c3c3a2ea42d52dafc76313f41c75d2717a3264a74408c33f9a43
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
| HEAD | `523c03063` | 2026-10-14 |
| rote CI-Läufe | `ci-gate` `38642537410` (am HEAD neu gestartet; gemessen lokal: der Format-Job bricht an `tools/service/src/bin/llm_gateway.rs:1265` — der Ganz-Crate-Format-Riss, kein Linien-Eigentum; der register-Job ist jetzt **clean**: `cdn_reconcile --fail` → `cap+tag contract clean (325 registry hosts)`, exit 0) · `register-coverage` `38642537331` (am HEAD neu gestartet; `register_lookup --tokens --noecho-fail` = 3 XFER-MISSING im `## An mycelium`-Block von `mountain-folge347`; `--orphans --fail` = 0) | 2026-10-14 |

Die Meta-Linie (Mycelium) trägt beide Zeilen in demselben Rund, in dem sie den privaten
Pass schreibt — öffentlich ins öffentliche Haus, privat ins private.
