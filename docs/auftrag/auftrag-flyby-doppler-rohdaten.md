<!--
  title: Auftrag — Roh-Doppler-Daten der historischen Flybys (NSSDC/PDS)
  class: auftrag
  date: 2026-09-03
  status: pending
  see-also: docs/paper/flyby-path-1-cold-cases.md docs/paper/flyby-path-2-preregistration.md docs/auftrag/auftrag-flyby2-addendum.md docs/TODO.md
-->

# Auftrag: die Roh-Doppler-Daten der historischen Flybys

## Zweck

Der Kleinpass hat die mm/s-Anomalie der sieben historischen Erd-Flybys in
`flyby-path-1-cold-cases.md` aus den **publizierten Literaturwerten**
(Turyshev & Toth 2011; Acedo & Bel 2016; Nieto & Anderson 2009) messungswahr
auf einen vorzeichenbehafteten Span korrigiert. Was bleibt, ist ein ehrlich
benanntes, ungelöstes Item: die **Roh-Doppler-Messreihe** selbst — die
zweifach-Doppler Range-Rate nahe dem Perigäum —, aus der der vorzeichenbehaftete
Residuum selbsttragend re-derivierbar wäre statt aus Tabellen zu stammen.

Turyshev & Toth (2011) geben an, die DSN-Doppler seien prinzipiell im
NSSDC-Archiv hinterlegt. Dieser Auftrag sucht und beschafft diese Rohdaten.

## Kernregel (0 honored)

Keine Zahl ohne Tabellen-/Registermarke. Der Residuum wird aus der **geladenen
Rohmessung** re-deriviert, nicht aus Erinnerung oder aus einer Sekundärquelle
abgeschrieben. Fehlt die Messung für einen Flyby, bleibt er `pending` — nie
eine erfundene mm/s-Zahl. Ein nur prinzipiell verfügbares, aber nicht
auffindbares Datenset ist `open` (Registerpflicht), kein Haken.

## Der Gegenstand

Sieben historische Erd-Flybys, je Perigäum (UTC, aus `flyby-path-1`):

- Galileo Ⅰ — 1990-12-08T20:34 (7481 km)
- Galileo Ⅱ — 1992-12-08T15:09 (6969 km)
- NEAR — 1998-01-23T07:24 (6934 km)
- Cassini — 1999-08-18T03:29 (7567 km)
- Rosetta — 2005-03-04T22:09 (8332 km)
- Messenger — 2005-08-02T19:14 (8721 km)
- Juno — 2013-10-09T19:24 (7185 km)

Gesucht wird je Flyby die zweiwege DSN-Range-Rate (Tracking-Pass durch das
Perigäum, Stunden vor/nach dem nächsten Ansatz) als Roh-Zeitreihe, aus der ein
vorzeichenbehafteter Residuum gegen ein N-Körper-Modell ableitbar ist.

## Der Weg (Quellen, in dieser Reihenfolge)

1. **NSSDC** (NASA Space Science Data Coordinated Archive) — die von Turyshev &
   Toth genannte Heimat der alten DSN-Daten; gezielt nach dem Radio-Science-
   Bestand der Missions-Sonden (Galileo, NEAR, Cassini, Rosetta, Messenger,
   Juno) suchen, je Mission eine Tracksammlung um das Perigäum.
2. **PDS** (Planetary Data System) — besonders die Radio-Science-Knoten; das
   Galileo-Radio-Science-Datenset (GLL) ist in PDS vermerkt — prüfen, ob es den
   Perigäum-Tracking des 1990/1992-Erd-Vorbeiflugs trägt.
3. **Daten-Archive der Raumfahrt-Dynamik** — Schwerpunkt der Geodäsie/DSN-
   Re-Analysen der Flyby-Anomalie; jede Quelle, die eine maschinenlesbare
   Range-Rate-Zeitreihe bereitstellt.
4. Führen die Archive zu keiner sauber ladbaren Reihe, ist je Flyby die
   tatsächliche Auffindbarkeit gemessen zu benennen (`pending` mit Ort), nie
   still.

## Abnahme (die Blatt-Zeile)

Ein geladener Tracking-Pass → N-Körper-Residuum → vorzeichenbehafteter
mm/s-Wert, gegen den publizierten ΔV∞ desselben Flybys gelegt. Die
`flyby-path-1`-Zeile trägt erst dann einen neuen belegten Wert, wenn er aus der
Rohmessung stammt.

## Register

Der Auftrag schließt erst, wenn je Flyby entweder die Rohmessung geladen und
re-deriviert **oder** ihre Nicht-Auffindbarkeit mit Ort gemessen ist. Jeder
Fortschritt (gefundener Pass, gelesener Katalog) wird im TODO/Ledger geführt —
dieser Auftrag ist der Ledger seines eigenen Wegs.
