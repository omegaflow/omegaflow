<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: 20239e391383e2f1556471d122f6ad93f5d1ff37474d3a62e7a134c2bc667628
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
| HEAD | `03178cb3f` | 2026-10-15 |
| rote CI-Läufe | `ci-gate` `38712632776` (queued am `03178cb3f` — **unread**) · `tools-build` `38712632802` (queued am `03178cb3f` — **unread**) · `ci-gate` `38709923691` (**failure** am `1651bc246` — die vier Archivar-Clippy-Lints + `format`/`register`) · `tools-build` `38712323866` (**failure** — `archive_search/net.rs:3882` „unnecessary parentheses", fremder Pfad) · `gic-al-network` `38709132540` (**failure** — runner shutdown; neu `38711782156`) · `zenodo-raw-cdn` `38712632760` (queued am `03178cb3f` — der Fix-Lauf) | 2026-10-15 |

**Gemessener Kern-Befund (nicht geraten):** `ci-gate` am `1651bc246` — der frühere
`frame_earth`-Compile-Fehler im Kern-Test-Modul ist geheilt (`224f75f74`); übrig bleiben die
vier Archivar-Clippy-Lints (`extract.rs:4100`; `tle.rs:90/147/341`) plus `format`/`register`.
Träger: Mountain (Parser-Bau) / River (Feld/Kern). Kein Mycelium-Pfad.

**Grün am jüngsten Stand:** `venus-radar-cdn` `38709049909` **success** ·
`wdcb-obn-wayback-cdn` `38704081781` **success** · `wdcb-sep-live-cdn` `38704082015`
**success** · `harvest-dispatch` `38709015954` **success** · `register-coverage`
`38712324163` **success** · `gic-station-sweep` `38709029080` **success**.

Lokal am HEAD `03178cb3f` gemessen: `register_sort phi/sources.φ` canonical (2 805 Blöcke),
`cdn_reconcile --fail` clean (334 Hosts), `register_lookup --orphans --fail` 0,
`register_lookup --tokens --noecho-fail` exit 0, `cargo check` (Kern) 0/0.

Die Meta-Linie (Mycelium) trägt beide Zeilen in demselben Rund, in dem sie den privaten
Pass schreibt — öffentlich ins öffentliche Haus, privat ins private.
