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
| Juno | PSPA-00605/OCRU post-EFB (ab DOY 284); Earth-EDR nicht registriert | nein | `open` (nur intern JPL-NAV) |

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
- **Juno** — PSPA-00605 (Outer Cruise, `JUNO-J-RSS-1-OCRU-V1.0`) geladen und
  gemessen: enthält den Earth-Pass **nicht** (2013-ODF beginnt 2013-10-11T15:27,
  Quelldatei `juno_merged_odf_2013_postefb.dat`, post-EFB). Earth-Encounter-EDR
  `JNO-E/J-RSS-1-EDR-V1.0` **existiert nicht** in der PDS-Registry (dsid löst zu
  keiner Datensatz-Seite auf); nur `JUNO-J-RSS-1-OCRU`/`JUGR` sind registriert.
  Der Earth-Pass (2013-10-09, DOY 282) ist in **keinem** erreichbaren PDS-
  Juno-Archiv — gemessen über PDS-Registry, Volume-Census (nur DOY 284-ODF) und
  lokalen TRK-2-34-Parser (631 327 Records, scid 61, Span 2013-10-11→12-31).
  Auch außerhalb PDS (Vektoren 1–3, unten) ist er nicht erhältlich.
  → `open`, Ort: internes JPL-NAV-Archiv (pre-EFB-Merged-ODF, nicht öffentlich).

### Nächster Schritt (Juno, konkret)

Der Earth-Pass (2013-10-09, DOY 282) ist in den PDS-Juno-Archiven gemessen
**abwesend** (kein Earth-Encounter-EDR registriert; nur post-EFB Outer-Cruise-
ODF ab DOY 284). Auch **außerhalb PDS** (DSN/open-data, NAIF/SPICE, NTRS,
arXiv, Zenodo, figshare) ist kein Roh-Doppler-Datenset erhältlich — gemessen.
Der Pass wurde dicht verfolgt (DSN + 2 ESA-Stationen; JPL AAS 14-435), die
Rekonstruktion fand kein anomales ΔV am Perigäum, aber die Roh-Tracking-Reihe
ist weder archiviert noch an die Analysen angehängt. Einzig erreichbar:
NAIF-SPICE-Trajektorien-Rekonstruktion `spk_rec_131005_131014_131101.bsp`
(1,66 MB, OD006) — Trajektorie, kein Doppler → nur die Modell-Seite, keine
Messreihe zum Treiben. Juno damit vollständig gemessen `open` mit Ort.

**Vektoren 1–3 (gemessen, 2026-09-03):** (1) NAIF-JUNO-Baum = nur kernels/ +
misc/ — kein odf/tracking; misc = CK-Rekonstruktion, kein Roh-Tracking;
`/pub/jpl-missions-support` = 404. (2) NSSDC/SCID-61/2011-040A = NMC retired,
SPDF nur Heliophysik, data.nasa.gov nur PDS-Science-Mirrors — keine SCID-61-
Sammlung für 2013-10. (3) Die „Beweis"-Behauptung eines 2026-Repository
kollabiert: `matthewsmawfield/TEP-EFA` hostet in
`data/raw/dsn_tracking/Juno_2013/` nur **Request-Formulare** (DSN-Anfragen
2026-05-17, Level-1-Doppler ±48 h) + `DOWNLOAD_INSTRUCTIONS.txt` auf das
PDS-OCRU-Mirror; sein eigener Text erwartet beim Ingest einen `RuntimeError`
(kein Overlap mit dem Flyby-Fenster). Kein öffentliches Repository trägt rohe
Juno-2013-Tracking-Daten. Kern-Erkenntnis: der Label-Name
`juno_merged_odf_2013_postefb.dat` impliziert eine intern beim JPL-NAV-
Conditioning gehaltene **pre-EFB-Merged-ODF, die nicht öffentlich archiviert
ist** — die einzige plausible Heimat des Earth-Passes ist damit das interne
JPL-NAV-Archiv, nicht öffentlich erreichbar. Eine Fortführung wäre eine
formelle DSN-Datenanfrage an JPL-NAV (wie die im TEP-EFA-Repo beantragten).

**Bestätigung + Modell-Seiten-Anker (2026-09-03):** Die Flyby-Lücke ist
archivseitig **hart belegt**, nicht nur vermutet — frühester öffentlicher
Juno-ODF = `GRV_OCRU_2013284_1527XMMMC005V01` (DOY 284 = 2013-10-11, ~2 Tage
nach C/A); TNF-Verzeichnis beginnt erst 2015 (Ka-Band-Ära). Kein ±7-Tage-Bogen
um C/A im PDS-Baum. Für die (noch unmögliche) Re-Derivations-Modell-Seite
nützlich: (a) NAIF `misc/sff/` — Small-Forces/Manöver-Historie (Residuen-
Modell); (b) OPAG-Präsentation `juno_efb_plans.pdf` — belegt das Daten-Backbone
(7 Tage kontinuierliches DSN-Tracking um Periapsis, Lücke C/A−2,25 h bis
C/A+1,5 h durch ESA-Stationen geschlossen, IMU aktiv); (c) JPL-OD-Lösung
„Reconstruction of Earth flyby by the Juno spacecraft" (AAS 14-435 /
ResearchGate 290573685) — Trajektorien-/Kraft-Seite, kein Roh-Range-Rate.
Roh-Doppler (DOY 275–289) bleibt ausschließlich über eine DSN-Datenanfrage
(tda.jpl.nasa.gov / JPL-Data-Request) oder die Paper-Autoren erhältlich —
kein öffentlicher Direkt-Download.

Kein Residuum wurde re-deriviert, keine mm/s-Zahl in `flyby-path-1` verändert.
Für die übrigen sechs Flybys bleibt der Auftrag `open`/`pending` mit den oben
benannten Orten — kein Haken.
