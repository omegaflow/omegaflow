<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: 993b741dd7cd6cd90b932c425fc8181dfcecb3bae86550be457dbdb9a7a47283
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
| HEAD | `1ccd3f852` | 2026-10-15 |
| rote CI-Läufe | `ci-gate` `38704150827` (HEAD `378359ab3`, **failure** — der Kern-Befund, s. unten) · `ci-gate` `38704269028` (HEAD `f7852a78f`, in Arbeit — **unread**) · `ci-gate` `38704378380` (queued — **unread**) · `ci-gate` `38704992362` (queued — **unread**) · `wdcb-obn-wayback-cdn` `38704081781` (queued — der neue **211er**-Matrix-Rettungslauf) · `zenodo-raw-cdn` `38693040881` (HEAD `b8fd39b49`, unread) | 2026-10-15 |

**Gemessener Kern-Befund (nicht geraten):** `ci-gate` `38704082180` (HEAD `d42e27eb8`,
`failure`) bricht im Kern — `could not compile omegaflow (lib) due to 7 previous errors`
(eine fehlende Funktion im Test-Modul `src/mathematikerin/machines/tests.rs`, dazu
Clippy-Lints in `src/archivar/` und `src/mathematikerin/`). Träger: River (Feld/Kern).
Kein Mycelium-Pfad; der gleiche Befund steht seit HEAD `10b521783` (`38702218068`).

**Grün am jüngsten Stand:** `wdcb-sep-live-cdn` `38704082015` **success** ·
`sources-repo-licence` `38704081473` **success** · `register-coverage` `38704081810`
**success** · `register-coverage` `38704150805` **success** · `harvest-dispatch`
`38704082223` **success**.

Lokal am HEAD `1ccd3f852` gemessen: `register_sort` canonical (2 805 Blöcke),
`cdn_reconcile --fail` clean (334 Hosts), `register_lookup --orphans --fail` 0,
`register_lookup --tokens --noecho-fail` exit 0, `cargo check` 0/0.

Die Meta-Linie (Mycelium) trägt beide Zeilen in demselben Rund, in dem sie den privaten
Pass schreibt — öffentlich ins öffentliche Haus, privat ins private.
