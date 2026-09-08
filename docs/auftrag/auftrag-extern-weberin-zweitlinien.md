<!--
  title: Auftrag (extern) — Weberin-Zweitlinien: dritte Linie Eisriesen, breite TNO-Linie, Sonden-Linie
  class: auftrag
  date: 2026-09-08
  sha256: 638d5bd8503699faa23cc1d1d2fdb1588fa9940f7f36513b7c6fd42665ce94cb
  status: pending
  see-also: docs/handover/handover-2026-09-07-weberin-sonnensystem-kette.md docs/concepts/die-weberin.md docs/TODO.md
-->

# Rechercheauftrag (extern): Weberin-Zweitlinien — drei offene Positions-Linien

Die Weberin flicht erst bei zwei unabhängigen Linien derselben Weltlinie. Drei
Klassen tragen heute nur eine Linie oder einen ungeschlichteten Riss. Für jede
ist zu finden: eine maschinenlesbare Route einer **zweiten unabhängigen
Positions-Linie** (gemessene Astrometrie ODER eine unabhängige Ephemeriden-
Abstammung), ihr Zugangscharakter und ein gemessenes HTTP-Verdikt. Diese Datei
ist selbsttragend; wer sie ausführt, liest vorab `docs/SOURCE_PORT.md` (§7–§9,
§11, §13) und `docs/concepts/die-weberin.md` §4/§7 (Abstammungs-Feld, „Ein
Faden ist keine Identität"). Es wird KEINE Datei geschrieben, kein Register
editiert — es wird nur benannt, was die Messung IST, und Befunde gemeldet.

## Status-Vokabular (bindend)

- `live` — 200, offen, maschinenlesbar.
- `blocked` — lebt, Zugang gesperrt (key/account/ip-blocked; reg-Zeile).
- `declined` — lebt, aber keine unabhängige Messung (Modell/Vorhersage/dieselbe Abstammung).
- `pending` — Route bekannt oder geahnt, Verdikt ausstehend.
- `not-published` — kein offener maschinenlesbarer Weg nach abgeschlossener Suche. Ein abwesender Zugang wird IMMER als `not-published` gemeldet — nie durch einen Ersatz gefüllt (zweite Kopie derselben Linie statt zweiter Linie, Modell statt Messung).

Regel: Ein toter Endpoint ist kein Endzustand — erst die Kaskade direkt curl →
r.jina.ai-Präfix → WebArchive → Websuche, dann ein Verdikt mit note, die den
Recherche-Stand nennt. Spekulationswörter sind verboten; ein ungemessener
Befund heißt `pending`. Jeder Befund trägt das Datum der Messung.

## Ausgangslage je Linie (gemessen 2026-09-07/08, nicht neu suchen)

### 1. Eisriesen-Schlichtung — dritte Ephemeriden-Abstammung (Uranus/Neptun)

- Befund: DE-vs-INPOP trägt einen Riss — uranus 1.59e6 m, neptun 1.07e6 m
  (JD 2461291), wachsend mit Extrapolations-Abstand (2000→2026: uranus
  5.6e5→1.6e6 m, neptun 4.3e5→1.1e6 m). Zwei Linien = DE (JPL) und INPOP19a
  (IMCCE, via `_spice.tar.gz`-SPK). Die innere 7-Körper-Konvergenz liest
  1.9e4–3.05e4 m; die Eisriesen sind die Ausreißer.
- Offen: Existiert eine dritte offene planetare Ephemeriden-Abstammung, die
  Uranus/Neptun trägt — EPM (Ephemerides of Planets and the Moon, IAA RAS,
  Pitjeva/Pitjeva & Pavlov), eine frühere/neuere INPOP- oder DE-Version als
  unabhängiger Fit, oder eine gemessene Astrometrie-Linie der Eisriesen-
  Positionen? Gemessen werden: Route, Format (SPK/DE-Binär/Text), Abdeckung
  Uranus+Neptun, Zugangscharakter, HTTP-Status. Nur eine **andere Abstammung**
  schlichtet — eine zweite Kopie von DE/INPOP zählt nicht (Abstammungs-Feld).

### 2. Breite TNO-Zweitlinie — unabhängige Astrometrie/Orbits für die 8.082

- Befund: `mpcorb_distant.bin` (MPC, 8.082 „Distant Object") = die erste
  Kepler-Linie. Gaia DR3 SSO = die einzige gemessen unabhängige Linie, aber nur
  14 helle TNOs (pluto 230 … varda 278 Transits; Sedna/Gonggong absent). Jede
  breite Katalog-Alternative (JPL SBDB/DASTCOM 7287, Horizons-SPK, AstDyS,
  Lowell astorb, Johnston) ist MPC-root — dieselbe Abstammung, keine zweite.
- Offen: Gibt es eine offene, von der MPC-Astrometrie unabhängige Orbit-Linie
  für die breite TNO-Menge — die DES-Orbit-Lösungen (Deep Ecliptic Survey,
  Buie/SWRI) als eigene Fits mit eigenen CCD-Positionen, OSSOS/CFEPS publizierte
  Orbit-Kataloge, oder ein anderer unabhängiger Astrometrie-Bestand (eigene
  Durchmusterung, nicht MPC)? Gemessen: Route, Format (Orbit-Tabelle mit
  Position ja/nein), Umfang (wie viele TNOs), Zugang, HTTP-Status.

### 3. Sonden-Zweitlinie — offene Positions-Astrometrie (VLBI/ΔDOR/Range)

- Befund: iss, juno, jwst, new_horizons, parker_solar_probe, solar_orbiter,
  voyager1/2, atlas_3i, wind tragen nur die SPK/Horizons-Linie. Doppler/NAVIO
  ist ein Range-Rate-Residuum in Hz (Signal-gegen-Modell), keine Position. ISS
  = TLE-basiert (gleiche Abstammung), Solar Orbiter = ESA-OD (bereits die
  gehaltene Linie). Kein offenes VLBI/ΔDOR/Range gemessen.
- Offen: Existiert eine offene **Positions**-Astrometrie für Sonden — PRIDE/EVN
  VLBI-Kampagnen (Jupiter-/Venus-Sonden, ESA PRIDE-Datensätze) als Plane-of-Sky-
  Winkellinie, oder eine offen publizierte zweite Ephemeriden-Abstammung
  (ESA-SPICE unabhängig von der gehaltenen Horizons-Linie)? Gemessen: Route,
  Format, welche Sonde, Positionscharakter (Plane-of-Sky ja/nein), Zugang,
  HTTP-Status.

## Abschluss je Linie (ein Befund-Block)

Für jede der drei Linien ein fester Befund:
1. Was die Messung IST (Positions-Linie: Astrometrie oder Ephemeriden-Abstammung).
2. Die gefundene Route (URL, Format, Positionscharakter) mit gemessenem HTTP-Status und Datum.
3. Das Verdikt aus dem Status-Vokabular.
4. note: was IST (Herkunft, Umfang, offene Restfrage). Absent/geschlossen heißt
   `not-published` — nie ein Ersatz, nie eine zweite Kopie derselben Linie.
