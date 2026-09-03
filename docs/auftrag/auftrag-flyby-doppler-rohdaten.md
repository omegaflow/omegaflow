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

## Suchlauf-Befund (2026-09-03) — reine Web-Recherche

Vorbemerkung, ehrlich benannt: Dieser Suchlauf verfügte nur über Web-Recherche —
kein Datei-Download, kein ODF-Parser, kein N-Körper-Solver. Der in der Abnahme
geforderte Schritt „geladener Tracking-Pass → Residuum → mm/s-Wert" ist daher
**nicht** vollzogen. 0 honored: keine einzige mm/s-Zahl wird hier genannt oder
re-deriviert. Es folgt der Auffindbarkeits-Befund je Flyby mit Ort.

| Flyby | Archiv-Ort gefunden | Doppler-Reihe geladen | Status |
|---|---|---|---|
| Galileo Ⅰ | POS/MAG/SSI ja, RS/ODF nein | nein | `open` |
| Galileo Ⅱ | POS/MAG ja, RS/ODF nein | nein | `open` |
| NEAR | PSPG-00721 (RS-Set existiert, Earth-Pass fehlt in Auflistung) | nein | `pending` |
| Cassini | RSS-Infrastruktur ja, Earth-ODF nein | nein | `open` |
| Rosetta | nur RPC-ICA (Plasma), kein RSI | nein | `open` |
| Messenger | keine Fundstelle | nein | `open` |
| Juno | PSPA-00605 (ODF, deckt Okt. 2013 Earth-Flyby ab) | nein | `found`, Laden aussteht |

### Je Flyby, gemessen

- **Galileo Ⅰ/Ⅱ** — PDS/NSSDC führen Trajektorien- (Earth-1/2, GSE/GSM) und
  Magnetometer-Sets, **kein** RS/ODF-Doppler-Set. Sekundärquelle: Galileo-1
  zweiwege Doppler sei nahe Perigäum gut beobachtet, aber in der Ursprungsquelle
  nur **graphisch** publiziert, schlechte zeitliche Auflösung — die Rohreihe
  wurde womöglich nie als maschinenlesbare Zeitreihe veröffentlicht, nur als
  Plot. Ort der Negativ-Prüfung: `data.nasa.gov`/`catalog.data.gov` (Tag
  „galileo"), `pds-ppi.igpp.ucla.edu/mission/Galileo`.
- **NEAR** — RS-Set PSPG-00721 existiert (ODF, closed-loop zweiwege Doppler +
  sequenzielles Range; Volume `NREROS_2001`); die Teilsets nennen Mathilde/Eros,
  **nicht** den 1998-01-23-Erd-Vorbeiflug. Ob der Earth-Pass im Volume unter
  anderem Namen liegt, ist ungeprüft (Volume nicht geladen). → `pending`.
- **Cassini** — RSS-Infrastruktur (S/X/Ka-Band zweiwege) beschrieben; einziges
  den Erd-Vorbeiflug nennendes Set ist RPWS (Plasmawellen, keine Doppler-Range-
  Rate). Kein Earth-ODF. Orte: `pds-atmospheres.nmsu.edu/.../Cassini/inst-rss`,
  `pds-rings.seti.org/cassini`.
- **Rosetta** — nur RPC-ICA (Ionen-Plasma) zum dritten Erd-Swingby; kein
  RSI-Doppler-Set.
- **Messenger** — keine Fundstelle (weder NSSDC-ID noch PDS-RS-Knoten-Eintrag).
- **Juno** — **einziger positiver Treffer:** NSSDC PSPA-00605 (Outer Cruise
  Gravity Science Raw Data Archive), X-Band-ODF, Fenster 2013-10 (Earth-Flyby)
  bis 2016-07 (JOI); ODF-Felder umfassen Einweg-/Zweiweg-/Dreiweg-Doppler (Hz).
  → `found`, Lade-/Re-Derivations-Schritt offen.

### Nächster Schritt (Juno, konkret)

Download PSPA-00605-ODF → Extraktion des Perigäum-Fensters (2013-10-09T19:24) →
ODF-Parser (der vorhandene `odf_census_probe` liest TRK-2-34) → N-Körper-Fit →
vorzeichenbehafteter Residuum gegen den publizierten ΔV∞. Dieser Schritt benötigt
File-Download + Verarbeitungswerkzeug und steht noch aus.

Kein Residuum wurde re-deriviert, keine mm/s-Zahl in `flyby-path-1` verändert.
Für die übrigen sechs Flybys bleibt der Auftrag `open`/`pending` mit den oben
benannten Orten — kein Haken.
