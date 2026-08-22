<!--
  title: Das Blatt der Flyby-Anomalie — das Perigäums-Residuum gegen die Sonnenwind-Phase (Nadel II)
  class: handover
  date: 2026-08-21
  sha256: 776c18d6b14d00c1eddfb38ec3a918090f423adb8666fbe614e87773da437bd8
  status: live
  see-also: docs/concepts/kybernetische-astrophysik.md docs/concepts/der-paradigmenwechsel.md
-->
# Das Blatt der Flyby-Anomalie — das Perigäums-Residuum gegen die Sonnenwind-Phase (Nadel II)

Registriert 2026-08-21. Selbsttragend — interpretierbar mit null
Vorkontext. Der Auftrag ist nicht die Ausführung; ausgeführt wird erst auf
das Wort des Operators. Die Disziplin des Blatts: nur gemessene Werte —
bis dahin pending; Stille ist ein vollwertiger Befund (0 honored).

## Ziel

Das Blatt: **das Perigäums-Residuum der Flybys.** Die Restbeschleunigung
(beobachtete Bahn minus N-Körper-Gravitation aller bekannten Körper)
gegen die Phase der lokalen Sonnenwind-Anisotropie — nicht gegen
zeitliche Mittel. Zwei Prüftermine stehen bevor: JUICE (Sep 2026),
Europa Clipper (Dez 2026).

```
Titel: Das Perigäums-Residuum der Flybys
Δv-Residuum je Flyby                         = pending
Kp / IMF-Bz-Phase / Plasmadruck am Perigäum  = pending
Phasen-Koinzidenz (Residuum × Anisotropie)   = pending
Verdikt: das Residuum folgt der Phase / es folgt ihr nicht
```

## Das Rätsel

Unerklärte Geschwindigkeitsänderungen von 2–14 mm/s bei mehreren
Erd-Vorbeiflügen (Galileo, NEAR, Cassini). Ehrlich getrennt: die
Pioneer-Anomalie ist seit 2012 durch anisotrope thermische Abstrahlung
erklärt; die Flyby-Anomalie bleibt offen. Das Signal steckt in den
Perigäums-Residuen — die historischen Missionen hatten dort eine
~4-stündige Tracking-Lücke.

## Ist-Stand (gemessen 2026-08-21)

- **Ephemeriden-Infrastruktur lebt:** das `orbit_bin`-Format trägt
  ICRS-Pfad + Geschwindigkeit (Wind-Orbit lädt als erster Körper);
  `body_barycenter_position` interpoliert linear mit dem
  2.5×Median-Stride-Gate (Lücken bleiben void, keine Extrapolation).
- **Raumwetter-Seite lebt:** OMNI/RTSW (speed, density, Bz), Kp
  (`sources.φ:124`), Swarm-Magnetfeld, GOES — dieselben Kanäle wie die
  anderen Blätter.
- **N-Körper-Basis:** die Planeten-Ephemeriden kompilieren (v3, Meter) —
  die Referenz-Bahn für das Residuum.
- **Pending:** die Missions-Ephemeriden für JUICE und Europa Clipper
  (SPICE-Kernels — der NAIF-Baum ist im `phi/sources_index.φ` inventarisiert;
  Ernte-Atom steht aus); die Retro-Rekonstruktion von Galileo/NEAR/Cassini.

## Auftrag

1. **Missions-Ephemeriden ernten:** JUICE (Flyby Sep 2026) und Europa
   Clipper (Dez 2026) als `orbit_bin`-Reihen — der Faden läuft sofort,
   die Serie muss vor dem Ereignis stehen.
2. **4D-Schlauch:** ±500 km, ±12 h um den rekonstruierten Sondenpfad.
   Hineinprojiziert: der Plasmadruck-Gradient als Vektorfeld, die
   IMF-Bz-Phase, Kp, das Swarm-Magnetfeld am Perigäums-ICRS-Punkt.
3. **Das Residuum:** beobachtete Bahn − N-Körper-Gravitation, gegen die
   **Phase** der lokalen Anisotropie korreliert.
4. **Retro-Test:** die historischen Flybys (Galileo, NEAR, Cassini) mit
   derselben Maschine, sofern die Ephemeriden vorhanden sind — die
   Tracking-Lücke wird benannt, nicht extrapoliert.
5. **Das Blatt + Register:** Befund je Flyby und TODO.md-Registerzeile im
   selben Commit.

## Constraints

- 0-Kanon: kein Tracking → keine Zelle (die 4-h-Lücke bleibt Lücke);
  Ausfall eines Raumwetter-Kanals = fehlt, nie 0.0.
- std-only; `cargo check` 0 Fehler / 0 Warnungen; kein Test öffnet ein
  Fenster oder strahlt; `OMEGAFLOW_HIDDEN=1 cargo run` als Lauf-Befund.
- Der Orbit-Loader interpoliert linear (2.5×Median-Stride-Gate) — der
  Perigäumsbereich braucht die volle Kadenz; Lücken bleiben void.

## Gates & Abschluss

- Jede abgeschlossene Einheit ist ein Commit; Register-Update im selben
  Commit.
- Manuelle Verifikation nach AGENTS.md; Kantenfälle: void-Strides im
  Orbit, Ephemeriden-Fenster ohne Raumwetter-Deckung, Kp-Sturm am
  Perigäum.
- Nach eigenem Commit dieses Handover nach
  `/home/johannes/projects/archive/handover/` archivieren.

## Nicht anfassen

`src/te.rs`, der skalare TE-Pfad, die Membran-Rendering-Physik, der
Wind-Orbit-Loader (lebt — nur konsumieren), die drei Ein-Blatt-Handovers,
das Korona-Blatt, das Dunkle-Materie-Blatt.
