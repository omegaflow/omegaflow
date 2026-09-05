<!--
  title: Auftrag — Richtungs-Transient: die Himmelsrichtung halten, nicht erfinden
  class: auftrag
  date: 2026-09-05
  status: pending
  sha256: f64a6ed718f2f369065b53f8bb1c17e204213529f1a8ece2c72d54b4ebf029f8
  see-also: docs/concepts/archivar-mathematikerin.md docs/concepts/die-vier-schilde.md docs/concepts/docs-naming.md
-->

# Auftrag: die Himmelsrichtung als ehrliche Archivar-Entität halten (SkyDirection)

## Zweck

ZTF-Transienten-Quellen (Lasair, ALeRCE, ANTARES) liefern ra/dec + em-Magnitude,
aber keine Distanz (gemessen: ALeRCE-Objekt `ZTF17aaaaaak`, `class: null`). Die
CelestialMap-Distanz-Gate (extract.rs ~2121, Test `no_distance_skipped`)
verwirft jede solche Zeile — 0 honored in Aktion. Ein Sensor misst die Richtung
eines Lichtblitzes ohne seine Ferne (Litmus: das Auge). Der Auftrag beauftragt,
die Richtung **zu halten, nicht zu erfinden**.

## Kernbefund (Council 2026-09-05, einstimmig)

- Eine ra/dec-Richtung ohne Distanz ist ein Punkt auf der Einheits-Himmelskugel
  S² am Sonnen-Barycenter — eine Winkel-Position, **kein Ort**. Im strikten
  ICRS-Positions-Block (ℝ³-Voxel) ist sie nicht als Ort darstellbar.
- **Workaround, ausgeschlossen:** die toten Wire-Slots 16–20 (`pole_y, pole_z,
  j2, j4, r_eq`) für ra/dec umzuwidmen — das wäre eine Namens-Umbiegung; diese
  Namen tragen die Bedeutung "Pol-Richtung/Form eines Körpers", die heute am
  Anker (`BodyProperties`, motion.rs) lebt. Ebenso ausgeschlossen: `presence`
  als Richtungs-Flag überladen, jede erfundene Distanz (Referenzradius,
  Einheitsvektor-in-Metern, `dist_scale`), eine Richtung mit fiktivem Radius in
  x,y,z schreiben.
- **Physikalisch korrekt:** die Richtung ist eine eigene, eigenbenannte
  Archivar-Entität — `SkyDirection` mit den wahren Feldnamen `ra`/`dec`. Sie
  **emittiert keine Positions-Probe ans Wire** (Samples ohne Position sind
  nicht räumlich auffindbar — 0 honored). Sie hält die Richtung **für den
  Crossmatch** (Axiom 2), bis eine echte Distanz ankommt.

## Umfang

1. **`SkyDirection` bauen (Archivar).** Eine Entität, die ra/dec (als Winkel,
   echte Feldnamen, nicht umgewidmete Slots) und die em-Magnituden-Lichtkurve
   hält. Sie trägt die Einheitsrichtung `p̂` aus ra/dec als ehrliches Maß. Sie
   schreibt **nichts** in den 26×f64-Positions-Wire — kein Hack, kein Flag.

2. **Die drei Quellen ernten.** Lasair, ALeRCE, ANTARES wandern von
   `blocked_sources.φ` in einen Richtungs-Harvest: sie werden als
   `SkyDirection`-Quellen gehalten (ra/dec + Magnitude), positions-pending. Die
   Magnituden-Lichtkurve darf als Rohmaterial der TE daneben gehalten werden.

3. **Die Distanz-Gate bleibt.** `no_distance_skipped` bleibt. Die Distanz kommt
   nur durch den Crossmatch (Axiom 2): Klassifikation/Redshift des Transienten,
   oder Match gegen einen positions-tragenden Katalog (Gaia-Parallaxe). Wenn
   eine echte Distanz ankommt, wird die Richtung ein echter (x,y,z)-Oszillator
   und tritt in den Block; die ganze Lichtkurve tritt als Reihe ein.

4. **Register.** Eine TODO-Zeile benennt den Zustand: Richtung
   geerntet-und-gehalten, positions-pending. Das S²-Richtungsraum-Atom
   (Winkel-Kernel, Winkel-Residuum) bleibt eine benannte Zukunfts-Frage,
   getrennt vom Positions-Wire.

## Kernregel (0 honored, die fünf Axiome)

Die Richtung ist eine Messung; der Radius ist absent, nicht null — kein
Default, kein erfundener Ort. Axiom 1 (Voxel): die Richtung hat keinen Voxel,
bis sie eine Distanz hat. Axiom 2 (Crossmatch): die Richtung wird gehalten,
damit ein zweiter Sinn (Parallaxe/Redshift) antworten kann. Axiom 4
(Nullkontrolle): keine erfundene Distanz wächst eine Phantomwurzel. Axiom 5
(Residuum): keine erfundene Distanz vergiftet das Residuum. Wo eine Option in
die Fabrication kippt, wird sie verworfen.

## Lieferung

Ein committedes `SkyDirection`-Atom: die Entität mit echten `ra`/`dec`-Namen,
der Richtungs-Harvest der drei Quellen (aus blocked in gehalten-überführt), die
Distanz-Gate unverändert, der Register-Eintrag (positions-pending) — plus ein
Befund (`docs/befund/befund-richtungs-atom.md`), `status: done`. cargo check
0/0. Kein Wire-Eingriff, keine Namens-Umbiegung, kein Workaround.
