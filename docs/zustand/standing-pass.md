<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: 2943165c06d4e3973749fa7762f08004a793a299e55d520f0601fe9daf6803a7
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
| HEAD | `db83de03b` | 2026-10-15 |
| rote CI-Läufe | **56** non-success — Träger: `register-coverage` (Fehler-Job `orphans`) · `ci-gate` (`dropped-gate`/`register`/`clippy`/`format`/`subset`) · `tools-build` · `matrix-rotor` · `quake-feeds-cdn` | 2026-10-15 |

**Gemessener Kern-Befund (nicht geraten):** Der **globale** Blitz-Kanal für die Schumann-TE ist
**frei** erreichbar. Die weite Quellen-Leiter über die Zenodo/OpenAlex-Register fand **WGLC** (WWLLN
Global Lightning Climatology and time series, Kaplan & Lau, **CC-BY-4.0**, Zenodo
`10.5281/zenodo.4774528`): global, **stündlich** (0.5°/5′) plus **30-Minuten**-Stoßleistung,
**2010–2025**, detection-efficiency-korrigiert. Zweiter freier globaler Kanal: **FLASHMAP**
(`10.5281/zenodo.17376946`, CC-BY, aus Vaisala GLD360 CG+IC). Die zwei früheren Messungen
(„Monatsmittel" bzw. „regional") waren zu eng.

**Noch unread:** die Push-Läufe an `8a7b48e0e` (`ci-gate`, `register-coverage`) — queued.

Die Tafel ist ein Flutstand aus den parallel laufenden Linien; die Logs liest `ci_manage log <id>`.
