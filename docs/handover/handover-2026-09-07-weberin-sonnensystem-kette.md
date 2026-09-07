<!--
  title: Handover — Weberin: das Weben im Sonnensystem (die zweite Linie der Körper-Kette)
  class: handover
  date: 2026-09-07
  sha256: 3b3cba12f4fc096f994b053a6ade466a4a6f0c9b01c03885d649116edbb57663
  status: live
  see-also: docs/concepts/die-weberin.md docs/handover/handover-2026-09-07-weberin-zeugen-faden-matrix.md docs/TODO.md
-->

# Handover — Weberin: das Weben im Sonnensystem

Übergabe für die nächste Sitzung. Die Aufgabe: die **Körper-Kette (Body)** im
Sonnensystem so zu flechten, dass sie der **Richtungs-Kette (Direction/S²)**
nachkommt — die außerhalb schon viel mehr Fäden trägt.

## 1. Der gemessene Befund (warum der Eindruck stimmt)

Außerhalb des Sonnensystems webt die Weberin reich: die S²-Richtungs-Ereignisse
(IceCat-1 348, Antares 8754, GW-Skymaps), vier lebende TAP-Legs (ALeRCE, SIMBAD,
Gaia-ARI, NED), das S²-Feld auf der Einheitskugel (`S2_LMAX 64`). Das ist die
Kette „Direction" — Tausende unabhängige Fäden.

Innerhalb webt sie dünn: der Körper-Verdict (`src/weberin.rs`, SPK gegen
DASTCOM-Kepler) trägt **zehn** Kleinkörper (`BODY_NUMBER`), davon vier
transneptunisch (Pluto, Eris, Makemake, Haumea). Planeten, Monde und Raumsonden
stehen als **eine** Weltlinie im Ephemeriden-Bestand, aber ohne zweite
unabhängige Linie — für sie liefert die Weberin `Absent { line: Dastcom }`,
keinen Zwirn, keinen Riss.

Die Ursache der Asymmetrie ist strukturell: die zweite Linie der Kleinkörper ist
die DASTCOM-Kepler-Bahn (MPC). Für Planeten/Monde/Raumsonden gibt es **keine**
zweite Linie derselben Klasse. Die Lücke ist benannt, nicht gefüllt.

## 2. Das Material (was schon daliegt)

- **Kette Body:** 72 `ephemeris_binary`-Blöcke + 1 `orbit_bin` (Wind) — Sonne,
  Planeten, 28 Monde, Zwerge/KBO/Asteroiden, und Raumsonden als Weltlinien
  (Parker Solar Probe, Solar Orbiter, Juno, JWST, ISS, New Horizons, Voyager 1/2,
  Wind). `die-weberin.md` §1.
- **Zweite Linie der Kleinkörper:** `AsteroidRec` (DASTCOM-Kepler), gebaut in
  `weberin.rs` (`state_at`, `body_number`, `BODY_NUMBER`).
- **Offener Live-Block:** `mpcorb_extended.json.gz` (MPC, TODO.md:1343) — die
  Kepler-Elemente Tausender TNOs; die erste Linie für eine breite TNO-Kette.
- **Sonden-Doppler:** die Pioneer/Voyager/New-Horizons-Linie (`auftrag-quiet-zone-*`)
  — Signal-gegen-Modell, das Gegenstück zur SPK-Linie der Sonden.
- **INPOP25c-Asteroidenmassen** (gravity-Katalog-Route) — Massen, keine
  Positions-Linien; als zweite Positions-Linie ungeprüft.

## 3. Der Kern: die zweite Linie je Klasse

Die Weberin flicht erst, wenn **zwei unabhängige Linien dieselbe Weltlinie
treiben** („Ein Faden ist keine Identität", die-weberin §7). Die offenen Fragen
sind genau die zweiten Linien — je Klasse eine eigene:

- **Planeten/Monde:** was ist die zweite unabhängige Positions-Linie? Eine zweite
  Ephemeriden-Abstammung (INPOP vs DE) oder eine gemessene Astrometrie-Linie —
  im Bestand ungeprüft, `pending`.
- **Raumsonden:** die NAVIO/Doppler-abgeleitete Bahn gegen die SPK-Linie. Das
  verbindet die Weberin mit der Deep-Space-Anomalie-Arbeit (Pioneer-Anomalie,
  Voyager-Quiet-Zone, New-Horizons-Vorfilter).
- **Breite TNO-Kette:** `mpcorb_extended` liefert die erste Linie (Kepler) für
  Tausende; die zweite (SPK-Route oder zweiter Katalog) öffnet den weiten
  Kuipergürtel, die Streuscheibe, die Sedna-Klasse — das transneptunische
  Gegenstück zum S²-Feld außerhalb.

## 4. Nächste Schritte (Sitzungs-Plan)

1. **Bestands-Inventar:** die 72 Ephemeriden-Blöcke listen — welcher Körper trägt
   schon eine zweite unabhängige Positions-Linie (nicht nur Masse)?
2. **TNO-Erweiterung:** `mpcorb_extended` harvesten → Kepler-Linie; die zweite
   Linie suchen (SPK-Route oder zweiter Katalog).
3. **Sonden-Zwirn:** die Doppler-Linie der Sonden als zweite Linie neben SPK —
   der Schritt, der die Weberin in den tiefen Raum öffnet.
4. **Der Riss als Instrument:** der Blink-Komparator (Tombaughs Pluto, Bessels
   Entfernung) ist die Methode; der TNO/Planeten-Zwirn ist genau diese Methode,
   angewandt auf das Sonnensystem.

## 5. Register

Die Lücke „zweite Linie je Körper-Klasse" ist im Register zu halten — nicht als
verdeckte Asymmetrie, sondern als benannte Pflicht: `pending`, bis jede Klasse
eine zweite Linie trägt.
