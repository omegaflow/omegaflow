<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: 16399bd694ac53efb33ba7b4d339b01581203048cddd7a1447fd87c3f7d73503
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
| HEAD | `a75fbc597` | 2026-10-15 |
| rote CI-Läufe | `ci-gate` `38709779846` (in Arbeit — **unread**) · `ci-gate` `38709167601`/`38709103112`/`38709015928` (**failure** — der Kern-Befund, s. unten) · `tools-build` `38709016009` (**failure** — `archive_search/net.rs:3882` „unnecessary parentheses around match scrutinee", fremder Pfad, nicht Mycelium) · `wdcb-obn-wayback-cdn` `38704081781` (queued — der **211er**-Matrix-Rettungslauf) · `venus-radar-cdn` `38709049909` (pending — der neue Venus-Lauf) · `zenodo-raw-cdn` `38693040881` (unread) | 2026-10-15 |

**Gemessener Kern-Befund (nicht geraten):** `ci-gate` `38704082180` (HEAD `d42e27eb8`,
`failure`) bricht im Kern — `could not compile omegaflow (lib) due to 7 previous errors`
(eine fehlende Funktion im Test-Modul `src/mathematikerin/machines/tests.rs`, dazu
Clippy-Lints in `src/archivar/` und `src/mathematikerin/`). Träger: River (Feld/Kern).
Kein Mycelium-Pfad; der gleiche Befund steht seit HEAD `10b521783` (`38702218068`).

**Grün am jüngsten Stand:** `wdcb-sep-live-cdn` `38704082015` **success** ·
`sources-repo-licence` `38704081473` **success** · `harvest-dispatch` `38709015954`
**success** (der neue generische `raw_mirror`-Aktor läuft, kein Block geändert) ·
`register-coverage` `38704081810`/`38704150805`/`38709167540` **success**.

Lokal am HEAD `a75fbc597` gemessen: `register_sort` canonical (2 805 Blöcke),
`cdn_reconcile --fail` clean (334 Hosts), `register_lookup --orphans --fail` 0,
`register_lookup --tokens --noecho-fail` exit 0, `cargo check` 0/0.

Die Meta-Linie (Mycelium) trägt beide Zeilen in demselben Rund, in dem sie den privaten
Pass schreibt — öffentlich ins öffentliche Haus, privat ins private.
