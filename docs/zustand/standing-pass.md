<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: b57de18f6b29a2c98e241b5028f4f52f91d0e77bf079b04bc84a4ad65e5eaa8b
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
| HEAD | `22310fbc6` | 2026-10-15 |
| rote CI-Läufe | `ci-gate` `38694956019` (HEAD `fc2221143`, queued — **unread**) · `register-coverage` `38694955977` (queued — **unread**) · `sources-repo-licence` `38694955997` (queued — **unread**) · `harvest-dispatch` `38694955990` (queued — **unread**) · `wdcb-obn-wayback-cdn` `38694956011` (queued — der neue 32er-Matrix-Rettungslauf) · `zenodo-raw-cdn` `38693040881` (HEAD `b8fd39b49`, in Arbeit — der Matrix-Lauf über gps_cmonoc/gps_campaign/cedc_catalog) | 2026-10-15 |

**Grün am jüngsten Stand:** `register-coverage` `38693094427`/`38694122270` **success** ·
`sources-repo-licence` `38693040838` **success** · `harvest-dispatch` `38693040865` **success**.
Lokal am HEAD gemessen: `register_sort` canonical (2 614 Blöcke), `cdn_reconcile --fail` clean
(328 Hosts), `register_lookup --tokens --noecho-fail` exit 0, `cargo check` 0/0.

Die Meta-Linie (Mycelium) trägt beide Zeilen in demselben Rund, in dem sie den privaten
Pass schreibt — öffentlich ins öffentliche Haus, privat ins private.
