<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: 65a7a86c221b2f91fdd818cea5b2615aee8911cb4c931915db34aa738288dbea
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
| HEAD | `b17570464` | 2026-10-15 |
| rote CI-Läufe | **279** non-success — Träger: `ci-gate` (170) · `register-coverage` (68) · `tools-build` (23) · `gic-al-network` (4) · `quake-feeds-cdn` (2) · `matrix-rotor` (2) · `m10-mdis-raw-cdn` (2) · `health-check` (2) · `zenodo-raw-cdn` · `ssd-jpl-raw-cdn` · `venus-radar-cdn` · `ci-check` · `themis_asi-fodg-cdn` | 2026-10-15 |

**Gemessener Kern-Befund (nicht geraten).** Der **globale** Blitz-Kanal für die Schumann-TE ist
**frei** erreichbar: **WGLC** (Zenodo concept `10.5281/zenodo.4774528` / Version
`10.5281/zenodo.20277101`, Record-Lizenz `CC-BY-4.0`; der README-Badge `CC BY-SA 4.0` steht als
benannter Riss daneben). Die veröffentlichte Dichte ist **täglich** (`density [5844,360,720]`,
Zeitschritt 1 Tag; monatlich für die Leistung) — `30m`/`05m` im Dateinamen ist die **räumliche**
Auflösung. Der WGLC-Compiler steht (`tools/harvest/src/bin/wglc_compiler.rs`); die Zulassungs-Zeilen
warten auf das Rat-Verdikt zur `field`-Form. Zweiter freier globaler Kanal: **FLASHMAP**
(`10.5281/zenodo.17376946`, CC-BY, aus Vaisala GLD360).

**Venus-Bildträger gespiegelt.** Drei COG-Träger sind registriert (`phi/sources.φ`) und laufen über
die `zenodo-raw`-Matrix byte-für-byte auf das CDN: **Venera-Polar** (194 MB, Zenodo `18176179`,
CC-BY-4.0), das **Provenienz-Klassen-Raster** (89 MB — die Herkunft jeder Basemap-Kachel, die
einzige Stelle, an der die Basemap-`0` als NoData geehrt werden kann) und **Arecibo 1988**
(965 MB). Der Provenienz-Kopf ist gemessen (18 775 × 9 388, 4-bit, 12 Klassen 0–11). Die 80,9-GB-
Basemap bleibt offen: die Rat-Form ist der byte-exakte `.parts`-Split, der Transport braucht noch
einen streamenden Splitter.

**Noch unread:** die tiefen Log-Gründe der fremden roten Läufe (kein Polling); die eigenen
Push-Läufe am HEAD `b17570464` sind queued. Die öffentliche Tafel ist ein Flutstand aus den
parallel laufenden Linien; die Logs liest `ci_manage log <id>`.
