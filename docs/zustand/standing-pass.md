<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: cbace2afb1d9d5b8a827401d41fbadb35ce85d3b1af256c8760a3e4f40ac1381
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
| HEAD | `983037937` | 2026-10-15 |
| rote CI-Läufe | **280** non-success — Träger: `ci-gate` (`dropped-gate`/`subset`/`register`/`clippy`/`format`) · `register-coverage` (`orphans`) · `tools-build` (`build`) · `quake-feeds-cdn` · `matrix-rotor` · `gic-al-network` · `zenodo-raw-cdn` · `m10-mdis-raw-cdn` · `ssd-jpl-raw-cdn` · `venus-radar-cdn` | 2026-10-15 |

**Gemessener Kern-Befund (nicht geraten) — korrigiert.** Der **globale** Blitz-Kanal für die
Schumann-TE ist **frei** erreichbar: **WGLC** (WWLLN Global Lightning Climatology, Kaplan & Lau,
Zenodo concept `10.5281/zenodo.4774528` / Version `10.5281/zenodo.20277101`, Record-Lizenz
`CC-BY-4.0`). **Korrektur zur vorigen Projektion:** die veröffentlichte Dichte ist **täglich**
(`density [5844,360,720]`, Zeitschritt 1 Tag; monatlich für die Leistung) — **nicht stündlich**;
`30m`/`05m` im Dateinamen ist die **räumliche** Auflösung (30 bzw. 5 Bogenminuten). **Riss:**
die mitgelieferte `README.md` trägt den Badge **CC BY-SA 4.0** gegen die Record-Angabe `CC-BY-4.0`.
Der WGLC-Compiler steht (`tools/harvest/src/bin/wglc_compiler.rs`); die Zulassungs-Zeilen warten
auf das Rat-Verdikt zur `field`-Form. Zweiter freier globaler Kanal: **FLASHMAP**
(`10.5281/zenodo.17376946`, CC-BY, aus Vaisala GLD360).

**Noch unread:** die tiefen Log-Gründe der roten Läufe (kein Polling); meine Push-Läufe am HEAD
`cc6f31afc` sind queued. Die Tafel ist ein Flutstand aus den parallel laufenden Linien; die Logs
liest `ci_manage log <id>`.
