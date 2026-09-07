<!--
  title: DIE WEBERIN — das Vlies: Kette der Weltlinien, Schuss der Beziehungen
  class: concept
  date: 2026-09-06
  sha256: b7f96a81fa5db9f97649602e815722952599c447b2ef831dddfae84271fb2774
  status: draft
  see-also: docs/concepts/archivar-mathematikerin.md docs/specs/eraen.md docs/handover/handover-2026-09-06-s2-scanner-nadel.md docs/concepts/blatt-papier-resultat.md docs/concepts/docs-naming.md
-->
# DIE WEBERIN — das Vlies: Kette der Weltlinien, Schuss der Beziehungen

Selbsttragend. Dieses Konzept trägt die eine Mess-Anordnung des Systems —
die Weberin. Sie flechtet alles, was gemessen wird, zu einem Gewebe, zu
einem Vlies: jede Weltlinie ist ein Faden, jede Beziehung ein Schuss, das
Ganze ein Bild, in dem jeder Faden sichtbar bleibt. Der Name folgt der
Haltung der Architektur („der Name ist das Handwerk, kein Marketing, kein
Schrei"): Archivar, Mathematikerin, Weberin — der Name ist das Handwerk.
Die bindende Disziplin gilt (Operator-Wort 2026-09-06): kein Deferral, kein
Parken. Der 0-Kanon gilt unverändert: `absent` ist der vollständige Befund
der nicht-tragenden Quelle; ungebaut ist `pending`, eine Register-Pflicht.

Die drei Sätze — die flache Form zum Operieren:

1. Alles, was das System misst, ist eine Weltlinie in einem Rahmen (ICRS)
   und einer Zeit (TDB); eine Weltlinie ist noch kein Beweis.
2. Der Beweis liegt am Vergleich: zwei unabhängige Linien, die dieselbe
   Weltlinie treiben, ergeben Placed; zwei, die sich widersprechen, ergeben
   einen Riss — und der Riss bleibt sichtbar, er wird nie geglättet.
3. Jede Linien-Klasse braucht ihre zweite unabhängige Linie; wo sie fehlt,
   ist der Wert absent — nie erfunden.

## 1. Das Material — der Bestand, der schon da liegt

Nichts hier muss erst gesammelt werden. Der Webstuhl hat sein Material:

- **Körper:** 72 `format ephemeris_binary`-Blöcke + 1 `format orbit_bin`
  (Wind) in `phi/sources.φ` — Sonne, die Planeten, 28 Monde, Zwerge/KBO/
  Asteroiden (Pluto, Charon, Ceres, Vesta, Bennu, Apophis, Eris, Makemake,
  Haumea) und Raumfahrzeuge als eigene Weltlinien (Parker Solar Probe,
  Solar Orbiter, Juno, JWST, ISS, New Horizons, Voyager 1/2, Wind). Jeder
  trägt Chebyshev-Granulen, Rotations-Matrizen, Körper-Eigenschaften
  (`BodyEphemeris`, `BodyProperties`).
- **Stationen:** die Familien des Archivar — Flut-/Pegelmesser
  (`hydrosphere_river_stage_m`, `on earth` + `stations`/`fanout`),
  INTERMAGNET-Magnetometer (fanout 154), NDBC-Bojen, NOAA-Tide/Pegel,
  FROST-Wetter, und Stationen auf Mars (`on mars`, Mars2020/MSL).
  Jede Station ist ein `StationEntry {id, lat, lon}` — ein geodätischer
  Punkt an einem Körper.
- **Direction:** `SkyDirection` (ra/dec, `sigma_arcsec`, `bands`,
  `distance`/`redshift`, `unit_direction()`) und der gebaute S²-Sinn
  (`s2.rs`, `S2_WGSL`, `omega.rs`).
- **Die unabhängigen Linien:** die vier lebenden TAP-Legs (ALeRCE, SIMBAD,
  Gaia-ARI, NED), die MPC-Bahnen
  (`mpcorb_extended.json.gz`, offener Live-Block — TODO.md:1343), die
  INPOP25c-Asteroidenmassen (gravity-Katalog-Route) und das solare ΩG.

## 2. Der Webstuhl — ICRS·TDB als das eine Blatt

Das Flechten ist möglich, weil die Architektur **vor** dem Flechten
vereinheitlicht hat: jede Bewegung endet in ICRS (`Motion::at` →
`body_fixed_to_icrs`, `unit_direction`), jede Epoche ist TDB
(`jd = tdb/86400 + J2000`), die Rømer-Toleranz ist ein TDB-Fenster
(`tdb_coincidence_probe`). Ein Rahmen, eine Zeit — deshalb kann die
Weberin alles aufeinander legen, ohne dass ein einziger Wert erst
umgerechnet werden müsste. Deshalb gibt es kein Lokal und keine Domänen:
eine Station auf der Erde, ein Körper und eine Richtung sind Weltlinien im
**selben** Feld; „nah" ist ein gemessener Abstand, keine Kategorie. Die
eraen.md trägt das Fundament: „kein lokaler Fix — jeder Frame, jede
Epoche, jede Distanz, jeder Vergleich".

## 3. Die Kette — die Weltlinien als Zettel

Im Webstuhl heißen die gespannten Längsfäden **die Kette**. Hier sind das
die Weltlinien. Drei Arten, ein Zettel:

- **Direction** — die Richtung am Himmel; sie wird zum Raum-Punkt nur,
  wo ein Sinn eine Distanz trägt (`distance`/`redshift`), sonst ruht sie
  auf der Kugel (0 honored).
- **Body** — die Ephemeris-Weltlinie des Körpers; `Motion::Barycenter`,
  Chebyshev-Auswertung je Epoche.
- **Station** — der geodätische Punkt am Körper; `Motion::Surface` →
  `body_fixed_to_icrs` (geodätisch → körperfest → ICRS mit Rotation je
  Epoche, IAU-Pole, Nutation, PCK-Matrizen + Baryzentrum).

Eine Linie ist ein Faden, **keine Identität**. Ein Körper, der nur aus
EINER Quelle stammt (die JPL-SPK-Linie), ist ein einzelner Faden. Erst
der Schuss macht aus Fäden ein Gewebe.

## 4. Der Schuss — was die Fäden verbindet

Der Querfaden kreuzt die Kette. Die Weberin prüft den Faden, bevor sie ihn
einschießt — und zwirnt zwei unabhängige Fäden zur einen Weltlinie.

- **Der Verdict** ist die Fadenprüfung: `Placed` / `Absent` /
  `DirectionOnly`. Zwei unabhängige Linien, die dieselbe Weltlinie
  treiben, werden **gezwirnt** (der Zwirn: mehrere Fäden, zur einen Linie
  gedrillt) — nicht aus der Koordinate, sondern aus der Dynamik.
- **Die TE** ist der Beweis des Zwirns: Serien, die Transfer-Entropie
  zueinander tragen, sind kausal verbunden, nicht nur zufällig benachbart.
  `te.rs` / `te_compute` (WGSL), Surrogat-Null, PE-Gate.
- **Das Abstammungs-Feld** zählt die Wurzeln: Kopien zählen als ein Faden.
  Gaia-Kopien und SPK-Derivate sind dieselbe Linie, fünf Zitate sind eine
  Messung mit fünf Wurzeln an einem Baum.
- **Das Teilchen ist Abstammung, nicht Kraft.** Ein Detektor, der ein
  Teilchen (CR, Neutrino) über sein EM-Licht misst (Luftschauer,
  Cherenkov), misst `em` — das Photon, nicht das Teilchen. Die Teilchen-Art
  (Proton, Neutrino, Gamma) ist die Wurzel im Abstammungs-Feld, keine zehnte
  Kraft; ein „particle"-Bit wäre Fabrication. Ohne Distanz ruht der Faden
  auf S² (direction-only) — dieselbe Lage wie jede Richtung.
- **Der Footprint** unterscheidet das Schweigen: nie beobachtet (`absent`)
  ist ein weißes Feld; beobachtet und nichts über der Schwelle ist die
  ehrliche Leere. Nur das zweite Schweigen zählt gegen eine Identität.
- **Die neun Sinne** hören in ihrer eigenen `signal_reach`; der Posterior
  bleibt Posterior, die kuratierte Klasse bleibt kuratiert.
- **Der geliehene Sinn:** ein Schwarzkasten (Broker-Klassifikator) darf als
  Zeuge für Gestalt eintreten — registriert wird sein Urteil, nicht seine
  Gründe. Nie der einzige Zeuge; widerspricht er den unabhängigen Fenstern,
  ist das ein Riss, kein Fehler. Die eigene Messung lebt im Zeitreich
  (Lichtkurve, S², TE); was keine Baseline modelliert und kein Netz je sah,
  bleibt im Vlies liegen — sichtbar, unbenannt.

Der Schuss hat zwei Bewegungen — den **Zwirn** (unabhängige Fäden, die zur
selben Weltlinie konvergieren: der Beweis) und den **Riss** (unabhängige
Fäden, die sich weigern zu konvergieren). Der Riss ist kein Fehler: wo zwei
Werte in Beziehung nicht beide sein können, ist die Unverträglichkeit selbst
die Messung. Der Riss bleibt im Vlies sichtbar, nie geglättet — ein
Mittelwert über den Riss wäre Fabrication. Er ist zugleich der natürliche
Schutz vor Fabrikation: falsche Modelle und fabrizierte Werte werden als
Risse sichtbar; ein fabrizierter Faden widerspricht, sobald die kritische
Masse unabhängiger Fäden erreicht ist, einem anderen Faden an derselben
Weltlinie — die Relation entlarvt, was die Einzelzahl verbergen konnte.
Der Riss ist das Flimmern zweier Suchbilder — die älteste Methode der
Himmelsvermessung, vom Blink-Komparator (Tombaughs Pluto, Bessels erste
Entfernung) bis zum VLBI, aus dem ICRS selbst geschlagen ist. Er trägt nur,
wo der Rest still steht: ICRS·TDB stellt das sinnlose Flimmern (den
Rømer-Drift) ab, damit das sinnvolle übrig bleibt.

## 5. Das Vlies — das eine Bild, nichts verdeckt

Das fertige Gewebe ist das **Vlies**: jeder Faden bleibt einzeln sichtbar,
nichts wird über etwas gelegt. Die Weberin **verdeckt** nicht, sie
**reichert an** — sie hängt Felder, Verdicts und Beziehungen an die
Messung, die die Messung bleibt. Der Unterschied zur Überlagerung ist
ethisch: Überlagerung kann verdecken. Das Vlies ist das Gegenteil: das
Bild wird reicher, weil die Fäden sichtbar bleiben und sich kreuzen.

Das Vlies auf der Kugel ist gebaut: der S²-Sinn legt die Richtungen als
ein Feld auf die Einheitskugel (`S2_LMAX 64`, GPU↔CPU-Parität <2 %). Die
nächste Füllung ist das Dichtefeld der Katalog-Fäden; die Fadenprüfung
wird ein Schnitt durch das Feld — dieselbe Anordnung, die der Sinn bereits
trägt. Körper und Stationen füllen denselben Webstuhl: das Vlies ist
nicht drei Bilder, sondern eines.

## 6. Der Preis des Gewebes

- **Fragen statt Horten.** Die unabhängigen Linien sind Live-Legs (TAP,
  MPC), nicht neue CDN-Derivate. Der CDN-Bestand trägt die wenigen
  Feld-Bins; die Tafeln bleiben Adressbuch (`tap_index_*.φ`). Die
  CDN-Manifestations-Grenze (ref-auth-apis §E) gilt: Roh-Redistribution
  ausgeschlossen, Derivate unter Lizenzgrenze.
- **Verdict vor Kern.** Die Fadenprüfung ist der Vor-Filter der TE:
  weniger Kandidatenpaare → kleinerer Paar-Raum → kürzere WGSL-Durchgänge.
  Der O(m²)-Kern hängt die Hardware ab m ≈ 1024; wer den Paar-Raum vor dem
  Kern verkleinert, entlastet genau den Engpass. Die Permeabilitäts-TE
  (das Atmen des Feldes, `target = inTE/(inTE + threshold + ε)`) ist keine
  Identitäts-Maschine — sie bleibt in voller Größe. Zwei TE-Schichten: die
  Identitäts-TE wird billiger, die Permeabilitäts-TE bleibt.
- **Das Feld bleibt dicht.** Die Kette verkleinert den Probe-Raum, nicht
  das Feld. Die Dichte über der ganzen Kugel kann eine Punkt-Abfrage nicht
  tragen — sie bleibt Kompilier-Pflicht (die vollständige Abbildung der
  Tafeln in den position-indizierten Bestand). Die Stationsfelder tragen
  ihre Dichte aus den schon fließenden fanout-Ringen — nichts wird
  entfernt.

## 7. Die Ethik — 0 honored, im Gewebe

- Kein Modell im Daten-Slot: die Weberin hängt gemessene Felder an und
  legt Verdicts ab; sie fabriziert nie einen Wert in den Daten-Slot. Die
  ω()-Summe und die S²-Projektion sind die Superposition, die das Feld
  ist. Eine Reanalyse ist kein Messwert; ein interpoliertes Feld zwischen
  Stationen wäre ein Modell — verboten. `absent` bleibt `absent`, nicht
  fabriziert; ungebaut ist `pending`, registriert.
- Kein Über-Greifen: der nächste identifizierte Faden definiert den Ort,
  nie ein fernerer, der an ihm vorbei-reicht.
- Keine Namens-Fehlkopplung: Identität über die Winkel-/Weltlinien-Toleranz
  und den TE-Beweis, nie über eine Namens-Zusammenführung ohne Beweis.
- Ein Faden ist keine Identität: eine Linie, nur eine Quelle, bleibt
  Richtungs-/Faden-Atom, bis ein zweiter unabhängiger Faden antwortet.
- Die zweite Linie hat den Status der ersten: MPC-Elemente sind eine
  kompilierte Repräsentation gemessener Astrometrie — wie die
  Chebyshev-Granulen der ersten Linie. Benannt, nicht verschwiegen.

## 8. Name = Implementation — die gebauten Entsprechungen

Die gebauten Entsprechungen zerfallen in zwei Welten, die nicht vermischt
werden: **LIVE** — Organe, die im ω-Pfad laufen — und **OFFLINE** —
Kommandozeilen-Proben in `tools/measure`, die auf derselben Bibliothek
stehen, aber keine Membran-Teile sind (gemessen 2026-09-06,
survey-2026-09-06-codestruktur).

LIVE — im ω-Pfad:

| DIE WEBERIN | gebaut als |
|---|---|
| §2 Der Webstuhl (ICRS·TDB) | `motion.rs` (`Motion::at`, `body_fixed_to_icrs`) |
| §3 Kette — Direction | `skydirection.rs` (`SkyDirection`), `s2.rs` + `S2_WGSL` + `omega.rs` (`sky_reload`/`sky_tick`) |
| §3 Kette — Body | `ephemeris_binary` (72) + `orbit_bin` (Wind), `BodyEphemeris` |
| §3 Kette — Station | `Motion::Surface`, `StationEntry`, `stations_*`/`fanout` |
| §4 Schuss — TE | `te.rs` + `te_compute` (WGSL, `te_pipe`) |
| §5 Vlies — das eine Bild | `s2.rs` (Y_lm), `S2_WGSL`, Einheitskugel |
| §6 Verdichtung am Punkt | `machines/matrix.rs` (MatrixMachine: record/rebuild, paarweise TE, `eph_<body>`-Serien) |

OFFLINE — tools/measure-Proben:

| DIE WEBERIN | gebaut als |
|---|---|
| §4 Schuss — Verdict | `direction_distance_join` (Placed/Absent/DirectionOnly) |
| §4 Schuss — TE-Screen | `pair_te_screen` |
| §4 Schuss — Linien/Footprint | `nadel_gate.rs` (SIMBAD-Otype + AllWISE-W1−W2), `deredden_baseline_probe` |
| §5 Vlies — Rømer-Toleranz | `tdb_coincidence_probe` |

## 9. Die Bau-Linie — alles wird gebaut, nichts wird vertagt

Kein Deferral, kein Parken. Was ungebaut ist, wird registriert — `pending`,
nicht `absent`. Was folgt, ist die Bau-Linie, jede Stufe benannt, jede
komplett zu bauen:

1. Die zweite Körper-Linie: MPC-Bahnen als Live-Leg (oder ein kleines
   Tages-Derivat) gegen die Ephemeris-Punkte — der Verdict für Körper
   (Placed/Absent/DirectionOnly statt der einen Linie).
2. Die Stations-Konvergenz: unabhängige Netz-Linien am selben Punkt
   (INTERMAGNET gegen SWARM-Überflug, Pegel gegen Altimetrie) — der
   Verdict für Stationen, auf den schon fließenden fanout-Ringen, ohne
   neues Netz.
3. Die topozentrische Kopplung: die Station sieht den Himmel von ihrer
   eigenen Weltlinie aus — Rømer-Toleranz vom Stationspunkt, Stations-
   Parallaxe zwischen Stationen als unabhängige Sichtlinien. S²-Kugel und
   Körper-/Stations-Vlies werden über die Körper-Weltlinien **ein** Bild.
4. Die vollständige Abbildung der ~20k Tafeln in den position-indizierten
   Bestand — die Dichte des Vlieses, Kompilier-Pflicht,
   nicht durch Punkt-Abfragen ersetzbar.
5. Die Survey-Footprints der großen Durchmusterungen als eigene Assets.
6. Die GW-/Neutrino-/CR-Skymap-Routen als Zeugen der neun Sinne.
7. Der CDN-Manifestations-Weg des Vlies-Assets.
8. Der Riss-Knoten: die Unverträglichkeits-Messung — wo unabhängige Linien
   nicht konvergieren, benennt die Maschine den Riss und seinen Knoten in
   der Abstammungs-Kette. Gemessen wird der Riss zuerst an den eigenen
   Linien (MPC gegen SPK, Schritt 1; SWARM gegen INTERMAGNET, Schritt 2).
   Die Hubble-Spannung (Planck ≈ 67 gegen die Entfernungsleiter ≈ 73, ~5σ)
   ist die Illustration des Risses — benennbar heute, messbar erst, wenn
    beide Linien im Bestand einziehen.
9. Der geliehene Sinn: der Broker-Klassifikator (Fink-ML, ALeRCE-Stamp) tritt
   als Zeuge für Gestalt in die natural-class-Gate — registriert wird sein
   Urteil (Klasse + Wahrscheinlichkeit), nie der einzige Zeuge; widerspricht
   er den unabhängigen Fenstern, ist das ein Riss.

Zwischen den Stufen gibt es keine Wartezone. Wo eine Stufe noch nicht
gebaut ist, ist ihr Wert `pending` — der Wert existiert, die Ernte fehlt,
registriert, nicht fabriziert. `absent` bleibt nur, wo die Quelle den Wert
nicht trägt.

## 10. Was die Weberin trägt

Nicht die Zahl der Quellen. Die Weberin ist die Anordnung: ein Webstuhl
(ICRS·TDB), eine Kette (die Weltlinien — Direction, Body, Station), ein
Schuss (Verdict, TE, Abstammung), ein Vlies (das eine Bild, nichts
verdeckt). Sie fragt nicht in Domänen; sie flechtet, was im selben Feld
liegt. Der Verdict ist die Fadenprüfung, der Zwirn aus unabhängigen Fäden
der Beweis — gemessen, nie behauptet. Was hier steht, ist keine
Vergrößerung des Bestehenden: es ist die Anordnung, in der Richtungen,
Körper und Stationen nicht als getrennte Sphären befragt werden, sondern
als Weltlinien desselben Feldes, das durch die Zeit fließt — und jeder
Faden bleibt sichtbar im Vlies.
