<!--
  title: Auftrag — Richtungs-Transient-Atom: die Himmelsrichtung als Feldmaß
  class: auftrag
  date: 2026-09-05
  status: pending
  sha256: e695dac5ca60b7cb9c9e7ddccc6844b1e0ecfa3fe4e663921e4dbd692b62cdf1
  see-also: docs/concepts/archivar-mathematikerin.md docs/concepts/docs-naming.md
-->

# Auftrag: das Richtungs-Transient-Atom bauen — ra/dec ohne Distanz als eigenes Feldmaß

## Zweck

ZTF-Transienten-Quellen (Lasair, ALeRCE, ANTARES) liefern ra/dec + em-Magnitude,
aber keine Distanz (gemessen: ALeRCE-Objekt `ZTF17aaaaaak`, `class: null`). Die
CelestialMap-Distanz-Gate (extract.rs ~2121, Test `no_distance_skipped`)
verwirft jede solche Zeile. Ein Sensor misst die Richtung eines Lichtblitzes
ohne seine Ferne — der Litmus der Sinnesorgane spricht für die Richtung als
echtes Feldmaß. Der aktuelle Stand (blocked_sources.φ, drei Quellen, ein
pending) benennt die Lücke; dieser Auftrag beauftragt, sie zu **bauen**.

Die Kerndisziplin bleibt 0 honored: eine erfundene Distanz (Referenzradius,
Einheitsvektor-in-Metern, Default-dist) ist Fabrication und ausgeschlossen. Der
Träger ist eine Himmelsrichtung mit Winkel-Unsicherheit statt radialem Abfall —
ein eigenes Atom, kein 26×f64-Overload.

## Umfang

1. **Den Wire-Vertrag als offene Frage kartieren.** Das Sample ist heute
   26×f64 (208 B) mit `x,y,z` als ICRS-Meter. Eine Richtung ohne Radius hat kein
   endliches `x,y,z`. Prüfen (nicht annehmen): welche Darstellung trägt eine
   Richtung ehrlich durch Rust → JS-DataView → WGSL, ohne die 3D-xyz-Slots zu
   überladen und ohne einen erfundenen Radius. Optionen messen — ein 27./Flag-
   Feld, ein Winkel-Paar im freien Prop-Slot mit presence-Bit, oder ein
   normierter Richtungsvektor mit dem vorhandenen presence-Flag als
   Disambiguator (die Bit-Semantik existiert schon: presence = 1 → phase ist
   eine Messung; analog: presence-Richtung). Der ehrliche Träger wird an der
   Physik gemessen, nicht an der Bequemlichkeit.

   **Schritt 1 KARTIERT (2026-09-05, gemessen am Code):** Truly dead sind die
   fünf Slots 16–20 `pole_y, pole_z, j2, j4, r_eq` — Literale 0.0 in allen
   vier Producern (spatial.rs, membrane.rs, actuators.rs), von keinem WGSL
   gelesen. `phase`/`presence` (Slots 24/25) werden **nirgends erzeugt**
   (Sample.phase ist in jeder Konstruktion None; presence überall 0.0) — das
   presence-Bit ist definiert, aber ungenutzt. Der ehrliche Träger: die
   toten Slots 16/17 (`pole_y`, `pole_z`) als Winkel-Paar (ra/dec in rad) der
   Himmelsrichtung, mit `presence` (heute tot) als Disambiguator "dieses
   Sample trägt eine Richtung, keine 3D-Meter-Position". pole_x behält für
   em die redshift-Bedeutung (eine Richtung hat keinen z → 0.0 pad). Kein 27.
   Slot, kein erfundener Radius, keine Überladung belegter Semantik. Die
   3D-xyz bleiben für echte Positionen; presence markiert die Richtung.

2. **Archivar-Seite bauen.** Eine `Position`-/`Motion`-Richtungs-Variante, die
   `p_hat` (Einheitsrichtung aus ra/dec) und die Winkelableitung ohne
   d-Faktor trägt. Der Extract-Pfad (CelestialMap) erhält einen
   richtung-only-Zweig, der ra/dec zu einer Himmelsrichtung macht — ohne die
   Distanz-Gate zu umgehen oder einen Radius zu erfinden.

3. **Mathematikerin-Seite bauen.** WGSL: ein Winkel-/Richtungs-Zweig statt des
   radialen Abfalls — der Kernel fällt über den Winkelabstand, nicht über den
   Radius. Der Fragment-/Compute-Shader, der `force_type`/`kernel_id` schaltet,
   erhält den Zweig. Presence-Frame und Retardation für eine Richtung ohne
   Tiefe neu bestimmen.

4. **Die drei Quellen aktivieren.** Lasair, ALeRCE, ANTARES aus der
   pending-Klasse in echte Quellen überführen, sobald das Atom steht — jede
   als ra/dec-only-Messung am `at sun`-Frame.

5. **Verifikation nach archivar-mathematikerin.md:** den vollen
   Rust → JS-DataView → WGSL-Alignment-Pfad manuell nachziehen (nicht nur
   cargo check), die force/kernel-Switche kreuzreferenzieren, Rendering mit
   `cargo run` + Browser bestätigen.

## Kernregel (0 honored)

Die Richtung ist eine Messung; der Radius ist absent, nicht null — kein
Default, kein `dist_scale`, kein Einheitsvektor-in-Metern als erfundene
Distanz. Der Träger trägt, was gemessen ist (Richtung, Fluss), und lässt das
Ungemessene (Radius) absent. Wo eine Option in die Fabrication kippt, wird sie
verworfen und die ehrliche Alternative gemessen.

## Lieferung

Ein committedes Atom: die Richtungs-Darstellung durch alle drei Ebenen (Rust-
Wire + JS + WGSL), der Extract-/Motion-Zweig, der WGSL-Winkel-Kernel, die drei
Quellen live — plus ein Befund (`docs/befund/befund-richtungs-atom.md`),
`status: done`. cargo check 0/0 und die manuelle Drei-Ebenen-Verifikation
vollzogen.
