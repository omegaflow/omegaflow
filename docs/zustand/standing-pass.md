<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: 83a0676d29ea7729a34418057d193fb5b30d95a578c2a7286d35c8ddbac425c4
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
| HEAD | `51bcd7fe1` | 2026-10-15 |
| rote CI-Läufe | **271** non-success — Träger: `ci-gate` (164) · `register-coverage` (64) · `tools-build` (23) · `gic-al-network` (4) · `quake-feeds-cdn` (3) · `health-check` (3) · `matrix-rotor` (2) · `m10-mdis-raw-cdn` (2) · `zenodo-raw-cdn` · `ssd-jpl-raw-cdn` · `venus-radar-cdn` · `ci-check` · `themis_asi-fodg-cdn` | 2026-10-15 |

**Gemessener Kern-Befund (nicht geraten).** Der **globale** Blitz-Kanal für die Schumann-TE ist
**frei** erreichbar: **WGLC** (Zenodo concept `10.5281/zenodo.4774528` / Version
`10.5281/zenodo.20277101`, Record-Lizenz `CC-BY-4.0`; der README-Badge `CC BY-SA 4.0` steht als
benannter Riss daneben). Die veröffentlichte Dichte ist **täglich** (`density [5844,360,720]`,
Zeitschritt 1 Tag; monatlich für die Leistung) — `30m`/`05m` im Dateinamen ist die **räumliche**
Auflösung (30 bzw. 5 Bogenminuten). Der WGLC-Compiler steht (`tools/harvest/src/bin/wglc_compiler.rs`);
die Zulassungs-Zeilen warten auf das Rat-Verdikt zur `field`-Form. Zweiter freier globaler Kanal:
**FLASHMAP** (`10.5281/zenodo.17376946`, CC-BY, aus Vaisala GLD360).

**Neu gespiegelt.** Der **Venera-Polar**-Rückstreu-GeoTIFF (Zenodo `18176179`, CC-BY-4.0) ist
registriert (`phi/sources.φ`) und wird über die `zenodo-raw`-Matrix byte-für-byte auf das CDN
gespiegelt (nur der Träger, client-seitig per Range gelesen — kein erfundenes Feld, Gaps `pending`).

**Noch unread:** die tiefen Log-Gründe der fremden roten Läufe (kein Polling); die eigenen
Push-Läufe am HEAD `51bcd7fe1` sind queued. Die öffentliche Tafel ist ein Flutstand aus den
parallel laufenden Linien; die Logs liest `ci_manage log <id>`.
