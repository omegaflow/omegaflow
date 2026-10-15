<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: b2576fa8d98df2986af5ca841bf407c16bdbe5d0addcfb8ecb6e315b468319be
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
| HEAD | `88e051c65` | 2026-10-15 |
| rote CI-Läufe | `ci-gate` `38693094480` (HEAD `88e051c65`, queued — **unread**) · `register-coverage` `38693094427` (HEAD `88e051c65`, queued — **unread**) · `ci-gate` `38693040722` (HEAD `b8fd39b49`, in Arbeit `subset → cargo test --lib` — **unread**) · `sources-repo-licence` `38693040838` (HEAD `b8fd39b49`, in Arbeit — **unread**) · `zenodo-raw-cdn` `38693040881` (HEAD `b8fd39b49`, queued — der neue Matrix-Lauf über die drei neuen Assets) | 2026-10-15 |

Am HEAD `b8fd39b49` (Ware-Commit) ist `register-coverage` `38693040719` **success** und
`harvest-dispatch` `38693040865` **success**. Lokal am HEAD gemessen: `register_sort`
canonical, `cdn_reconcile --fail` clean (327 Hosts), `register_lookup --tokens --noecho-fail`
exit 0.

Die Meta-Linie (Mycelium) trägt beide Zeilen in demselben Rund, in dem sie den privaten
Pass schreibt — öffentlich ins öffentliche Haus, privat ins private.
