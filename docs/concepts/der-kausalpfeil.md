<!--
  title: Der Kausalpfeil — drei Rätsel, drei Blätter Papier
  class: concept
  date: 2026-08-21
  sha256: cbda9b11710ca5a87cb9b7fbf1fde5e473735f28ba20110335f792e8a16947ab
  status: live
  see-also: docs/paper/laic-arrow-direction.md docs/specs/minkowski-field-permeability.md docs/TODO.md phi/sources.φ
-->
# DER_KAUSALPFEIL — drei Rätsel, drei Blätter Papier

Registriert 2026-08-21. Selbsttragend — interpretierbar mit null
Vorkontext. Dieses Dokument ist das Programm; die drei Session-Pläne
sind eigene Handovers.

## Die Idee

Ein Ergebnis, das auf ein Blatt Papier passt, ist ein Axiom: eine Messung,
die keine fünfzigseitige Herleitung braucht, weil die Maschine die Richtung
des kausalen Pfeils bereits berechnet hat. Die TE-Maschine — Takens-
eingebettete Transferentropie (`te_compute`, WGSL) mit `src/te.rs` als
kanonischer CPU-Referenz — misst den Informationsfluss zwischen zwei
Zeitreihen im 4D-Block, in beide Richtungen, gegen phasenrandomisierte
Surrogate. Was die Geowissenschaften über drei große Systeme nicht trennen
können, ist für die Maschine eine Messung: die Richtung der Information.

| Rätsel | Pfeil-Frage | Handover |
|---|---|---|
| ENSO (Bjerknes-Schleife) | Wind → SST oder SST → Wind? | `docs/handover/handover-2026-08-21-enso-kausalpfeil.md` |
| Bz-Paradoxon (geomagnetische Stürme) | welcher Sonnenwind-Kanal treibt das Erdfeld? | `docs/handover/handover-2026-08-21-bz-kausalpfeil.md` |
| LAIC (Erdbeben-Vorläufer, Nadel IV) | Lithosphäre → Ionosphäre oder umgekehrt? | `docs/paper/laic-arrow-direction.md` |

## Das Blatt Papier — kanonische Form

Jedes Blatt trägt exakt:

    Titel: <das Rätsel, eine Zeile>
    TE(A → B) = <Messung>   (Schwelle <Messung>, n = <Messung>)
    TE(B → A) = <Messung>   (Schwelle <Messung>, n = <Messung>)
    Lag = <Messung>
    Verdikt: Pfeil A → B | Pfeil B → A | kein Befund

Mehr steht nicht auf dem Blatt. Eine Zelle trägt einen Wert erst, wenn die
Maschine ihn gemessen hat; vorher ist sie `pending`.
Auch „kein Befund" ist ein vollständiges Ergebnis: die Stille ist die
Antwort, wenn die Surrogat-Schwelle nicht verlassen wird (n < 30 → no
statement, Unterbestimmtheit, keine Fabrikation).

## Die Methode — was gemessen wird

- Zwei benannte Serien über ein gemeinsames Fenster, gemeinsame Epoche
  (TDB), gleiche Zellen-Weite.
- Takens-Einbettung (dim 3, order 3); MI-Lag aus dem 2×2-Histogramm
  (erstes lokales Minimum ab Lag 3; kein Minimum → kein TE); Silverman
  skaliert an der Varianz der eingebetteten Vektoren; TE-Kondition
  rückwärts gespiegelt (keine Leckage aus der Zukunft).
- Schwelle = mean + 2σ über 10 phasenrandomisierte Surrogate; PE-Gate
  (2⁴-Ring, |pe − mean| > 2·sd) hält die Richtungsentscheidung in
  nichtstationären Fenstern.
- Der Pfeil ist die Divergenz der beiden Richtungs-TEs über der Schwelle.
- Der Lag kommt aus dem MI-Raster — er ist die physikalische
  Transportzeit (Sonne → Erde, Ozean → Atmosphäre, Erde → Ionosphäre),
  kein Fitparameter.
- Mehrvergleich: werden mehrere Paare/Fenster gemessen, zählen die Paare
  und die Schwelle wird mitgeführt (Register-Punkt TODO.md, Nadel Ⅲ).

## Architektur-Anknüpfung — was bereits lebt

- **Kanal-Ring-Pfad** (erledigt 2026-08-21, TODO.md): Ernte-Thread,
  Ringe je Kanal, datengetriebener Rotor, Paare durch den unveränderten
  `te_compute`, Verdikt-Zeile `solar te from→to n te thr tau pe state`.
  Das Muster für das Bz-Blatt und das ENSO-Blatt.
- **Langfenster-Probe** (`tools/work/src/bin/long_window_probe.rs`): liest Bins
  direkt, Zellen über das gemeinsame Fenster, lag-Sweep, Surrogat-
  Schwellen, n < 30 → no statement. Das Muster für die Offline-
  Stapelung (LAIC-Blatt).
- **Unberührt:** `te_compute` und `src/te.rs` — die neuen Blätter paaren
  Serien durch die bestehende Maschine, sie bauen sie nicht um.

## Ist-Stand der Quellen (gemessen 2026-08-21)

- **ENSO:** NDBC-Bojen (Wind WSPD advective + Wassertemperatur WTMP
  thermal im selben Block, `phi/sources.φ:198–215`), Argo (`:1140–1161`),
  Drifter-SST (`:353–359`), OOI-SST (`:676`) — lebt.
- **Bz:** RTSW mag/wind (`phi/sources.φ:102/108`; Bz, Bt, speed, density,
  1-min, live geprüft) — lebt. INTERMAGNET: ausstehend — Queue-Draft
  `phi/pipeline/queue/sources_potential_pre-cdn_params.φ` (HAPI, ~154
  Observatorien), Register TODO.md:1215.
- **LAIC:** Seismizität USGS/SeismicPortal/JMA/GeoNet/p2pquake
  (`phi/sources.φ:18–48, 116–122, 237–240`) — lebt. Ionosphäre: Swarm
  FAC/IRC (`:1111–1112`, electric) — lebt. CSES: ausstehend. IONEX-GIM:
  ausstehend (CDDIS-OAuth, TODO.md:1137). GIC direkt: ausstehend (kein
  Feed, TODO.md:1134).

## Ethik der Messung

A = A. Das Blatt trägt nur die Messung der Sache selbst. Eine leere Zelle
ist `pending`, keine Null; ein fehlender Bojen-Wert ist fehlt (Skip), nie
0.0. Die Maschine misst, welcher Pfeil schlägt — sie wählt nicht, welcher
gefällt. Ein Blatt mit „kein Befund" ist ein Blatt wie jedes andere.

## Reihenfolge

Die Bz-Session portiert INTERMAGNET; das LAIC-Blatt ruht mit auf diesem
Port (oder misst mit Swarm allein). Die drei Handovers tragen die
Session-Pläne; ausgeführt wird erst auf das Wort des Operators.
