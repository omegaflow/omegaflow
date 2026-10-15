<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: f0717fc75b723635f920a089ce63f73b20a653b547ba1e90e6854e36eb0a36e3
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
| HEAD | `378359ab3` | 2026-10-15 |
| rote CI-Läufe | `ci-gate` `38704082180` (HEAD `d42e27eb8`, **failure** — der gemessene Kern-Befund, s. unten) · `ci-gate` `38704269028` (HEAD `f7852a78f`, queued — **unread**) · `ci-gate` `38704150827` (unread) · `register-coverage` `38704150805` (queued — **unread**) · `sources-repo-licence` `38704081473` (in Arbeit) · `wdcb-obn-wayback-cdn` `38704081781` (queued — der neue **211er**-Matrix-Rettungslauf) · `wdcb-sep-live-cdn` `38704082015` (in Arbeit — der **7er**-Lauf des lebenden `www.wdcb.ru/sep`-Baums) · `zenodo-raw-cdn` `38693040881` (HEAD `b8fd39b49`, unread) | 2026-10-15 |

**Gemessener Kern-Befund (nicht geraten):** `ci-gate` `38704082180` (HEAD `d42e27eb8`,
`failure`) bricht im Kern — `could not compile omegaflow (lib) due to 7 previous errors`
(eine fehlende Funktion im Test-Modul `src/mathematikerin/machines/tests.rs`, dazu
Clippy-Lints in `src/archivar/` und `src/mathematikerin/`). Träger: River (Feld/Kern).
Kein Mycelium-Pfad; der gleiche Befund stand schon am HEAD `10b521783` (`38702218068`).

**Grün am jüngsten Stand:** `register-coverage` `38704081810` **success** ·
`harvest-dispatch` `38704082223` **success** · `register-coverage` `38702218028` **success**.

Lokal am HEAD `378359ab3` gemessen: `register_sort` canonical (2 805 Blöcke),
`cdn_reconcile --fail` clean (334 Hosts), `register_lookup --orphans --fail` 0,
`register_lookup --tokens --noecho-fail` exit 0, `cargo check` 0/0.

Die Meta-Linie (Mycelium) trägt beide Zeilen in demselben Rund, in dem sie den privaten
Pass schreibt — öffentlich ins öffentliche Haus, privat ins private.
