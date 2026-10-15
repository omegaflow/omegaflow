<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: 372b58b3c6220f8704ed445225c26d491e9f3f13d2c86c35b8b965ba162d1440
  status: live
  see-also: AGENTS.md
-->
# Standpass — öffentliche Projektion (privat↔öffentlich, A)

Die öffentliche Fassung des Stehenden Passes (Operator-Wort A, 2026-10-14). Sie trägt
**nur, was öffentlich ist** — HEAD und rote CI-Läufe. Mailstand, Operator-Akte und
Personenbezug bleiben im privaten Haus (`state/zustand/standing-pass.md`, Linie Mycelium).
Die öffentlichen Linien zitieren **diese** Fassung, nie die private.

| Feld | Wert | gemessen |
|---|---|---|
| HEAD | `8a7b48e0e` | 2026-10-15 |
| rote CI-Läufe | **56** non-success — Träger: `register-coverage` (Fehler-Job `orphans`) · `ci-gate` (`dropped-gate`/`register`/`clippy`/`format`/`subset`) · `tools-build` · `matrix-rotor` · `quake-feeds-cdn` | 2026-10-15 |

**Gemessener Kern-Befund (nicht geraten):** Der WWLLN-Stundenstoßdatensatz — der von der
Frontier-Runde gewählte Blitz-Kanal für die Schumann-TE — ist **zugangsgebunden** (Archiv-Verkauf
ab 2004-08-15, Host-Abo, Forschungszugang via University of Washington, kommerzieller Reseller).
Öffentlich sind nur die WWLLN-**Klimatologien** (Monat×Stunde, keine Stundenreihe). Bis zu einem
Zugang bleibt die bedingte TE blockiert. LIS/OTD (NASA GHRC) war von dieser Maschine nicht
erreichbar (`status 0`) → `pending`.

**Noch unread:** die Push-Läufe an `8a7b48e0e` (`ci-gate`, `register-coverage`) — queued.

Die Tafel ist ein Flutstand aus den parallel laufenden Linien; die Logs liest `ci_manage log <id>`.
