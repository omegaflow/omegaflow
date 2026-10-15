<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: bd0b91fc057ca817f7cab76e153b610a20834ff510e9898584bc8503868db677
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
| HEAD | `0281ae1a4` | 2026-10-15 |
| rote CI-Läufe | `ci-gate` `38709923691` (**failure** am `1651bc246` — die vier Archivar-Clippy-Lints + `format`/`register`) · `tools-build` `38712632802` (**failure** — `archive_search/net.rs:3882` „unnecessary parentheses", fremder Pfad) · `gic-al-network` `38711782156` (**failure** — River) · `m10-mdis-raw-cdn` `38712632761` (**failure** — Arm `lpsc2024_2663.pdf`, `raw_mirror: the CDN upload returned void`) · neu ausgelöst von `0281ae1a4` und **unread/queued**: `ci-gate` `38714272137`, `harvest-dispatch` `38714272156`, `sources-repo-licence` `38714272150`, `register-coverage` `38714272194` | 2026-10-15 |

**Gemessener Kern-Befund (nicht geraten):** `ci-gate` am `1651bc246` — der frühere
`frame_earth`-Compile-Fehler im Kern-Test-Modul ist geheilt (`224f75f74`); übrig bleiben die
vier Archivar-Clippy-Lints (`extract.rs:4100`; `tle.rs:90/147/341`) plus `format`/`register`.
Träger: Mountain (Parser-Bau) / River (Feld/Kern). Kein Mycelium-Pfad. Der Mycelium-Pfad
`m10-mdis-raw-cdn` `38712632761` ist ein eigener CDN-Upload-Rot (Arm `lpsc2024_2663.pdf`).

**Grün am jüngsten Stand:** `venus-radar-cdn` `38709049909` **success** ·
`wdcb-obn-wayback-cdn` `38704081781` **success** · `wdcb-sep-live-cdn` `38704082015`
**success** · `harvest-dispatch` `38712632810` **success** · `auto-dispatch` `38712632786`
**success**.

Lokal am HEAD `0281ae1a4` gemessen: `register_sort phi/sources.φ` canonical (2 805 Blöcke),
`cdn_reconcile --fail` clean (334 Hosts), `register_lookup --orphans --fail` 0,
`register_lookup --tokens --noecho-fail` exit 0, `cargo check` (Kern) 0/0. Die Athabasca-
ERG-SC-VLF-Manifestation ist gesetzt (der Block trägt `url`/`origin`/`harvest raw_mirror`/`sha256`).

Die Meta-Linie (Mycelium) trägt beide Zeilen in demselben Rund, in dem sie den privaten
Pass schreibt — öffentlich ins öffentliche Haus, privat ins private.
