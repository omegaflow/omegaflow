<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-14
  sha256: 53f2562b163a8962efa082fbc3306ac955523997e7d0ab2113b659f6f92d6572
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
| HEAD | `d4fdb76ee` | 2026-10-14 |
| rote CI-Läufe | `ci-gate` `38641123673` (am HEAD neu gestartet; gemessen lokal: `cargo fmt --check` bricht an `tools/service/src/bin/llm_gateway.rs:1265` — der Ganz-Crate-Format-Riss, kein Linien-Eigentum; register: `cdn_reconcile --fail` am HEAD-Baseline = 5 ungebundene CDN-Netlocs → Mountain) · `register-coverage` `38641123689` (am HEAD neu gestartet; `register_lookup --tokens --noecho-fail` = 7 XFER-MISSING in fremden Blöcken, `mountain-folge346`/`347` und `river-folge211`; `--orphans --fail` = 0) | 2026-10-14 |

Die Meta-Linie (Mycelium) trägt beide Zeilen in demselben Rund, in dem sie den privaten
Pass schreibt — öffentlich ins öffentliche Haus, privat ins private.
