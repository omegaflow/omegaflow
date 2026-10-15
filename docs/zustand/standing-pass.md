<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: 01ba2cbc07a602e67982ae16f96f2811decce60e614cf2345172c91968617a88
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
| HEAD | `6290535e5` | 2026-10-15 |
| rote CI-Läufe | **56** non-success — Träger: `register-coverage` (Fehler-Job `orphans`) · `ci-gate` (`dropped-gate`/`register`/`clippy`/`format`/`subset`) · `tools-build` · `matrix-rotor` · `quake-feeds-cdn` | 2026-10-15 |

**Gemessener Kern-Befund (nicht geraten):** Der Blitz-Kanal für die Schumann-TE ist **offen**.
Die weite Leiter (`bin/all_wide`) und die Direktmessung (alle HTTP 200) zeigen: **WGLC** (WWLLN
Global Lightning Climatology + time series, CC-BY, global-monatlich, Zenodo/Pangaea) ·
WWLLN-Klimatologie-Datenverzeichnisse · ein **stündliches CONUS**-Set (UNC Dataverse
`10.15139/s3/nk3ncm`) · WWLLN eigener offener KML-Feed (1 h, 6 h Verzögerung) · offene
Satelliten-Blitze **MTG-LI** (Europa/Afrika) und **GLM** (Amerikas). Nur der globale stündliche
WWLLN-Rohdatensatz bleibt zugangsgebunden. Die erste enge Messung („nur Monatsmittel") war
unvollständig.

**Noch unread:** die Push-Läufe an `8a7b48e0e` (`ci-gate`, `register-coverage`) — queued.

Die Tafel ist ein Flutstand aus den parallel laufenden Linien; die Logs liest `ci_manage log <id>`.
