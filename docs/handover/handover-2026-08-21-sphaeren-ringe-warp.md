<!--
  title: Handover: Die Sphären — Atom 2 (Ringe), Atom 3 (Warp), Feld-Absorption
  class: handover
  date: 2026-08-21
  sha256: 12be38a5e31e46fc9555fb2f0a85a63bd238134fabe72604a3b47ac7335a4ec9
  status: live
  see-also: TODO.md docs/concepts/the-seven-spheres.md docs/handover/handover-2026-08-21-archivar-arbeitsliste.md
-->

# Handover: Die Sphären des Unsichtbaren

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext. Der Auftrag
ist nicht die Ausführung; ausgeführt wird erst auf das Wort des Operators.

Register-Quelle: TODO.md, Abschnitt „Die Sphären des Unsichtbaren". Atom 2
und Atom 3 sind als eigene Sessions benannt — je ein Fenster, je ein Atom.
Die Feld-Absorption ist ein eigenes Atom (aus Atom 8 heraus registriert).

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # sauber oder fremde Arbeit nennen
cargo check                      # 0/0
grep -n "absorption" src/mathematikerin.rs | head    # der Slot lebt
```

Referenzen (stehend): `docs/concepts/the-seven-spheres.md`,
`docs/concepts/lost-concepts.md` (Okklusions-Ära), `TODO.md` (Sphären +
Okklusions-Reste), das Wire-Protokoll `docs/reference/BINARY_PROTOCOL.md`
(falls vorhanden — der absorption-Slot, Slot 10 des 24×f64-Records).

## Der Kontext (verifiziert)

- Atom 1 der Sphären (die Membrane als Wahrnehmungs-Sphäre) ist ERLEDIGT —
  er deckt den Weg für Ringe/Warp. Ein Konzept-Dokument für Ringe/Warp
  existiert noch nicht (das Register benennt das — es ist Teil von Atom 2/3,
  den Weg zu dokumentieren, bevor er gebaut wird).
- Die geometrische Okklusion (Ephemeriden-Barrieren, OccIndex/OccReport)
  starb in Atom 8 (2026-08-20). Der `absorption`-Slot lebt im Protokoll:
  Sample.absorption (src/archivar.rs:32), der Wire trägt ihn, und die WGSL
  hat bereits den Alpha-Pfad: `field_spatial` nimmt absorption und
  `alpha = clamp(absorption, 0.0, 1.0)` (src/mathematikerin.rs:99/115).
  Was fehlt, ist die Füllung — kein Sample trägt heute einen Wert ≠ 0.0.

## Die Atome

### Atom 2 — Ringe

Eigener rings-Buffer + WGSL `ring_transmission`; Literatur-τ mit
Provenienz (die Ring-Parameter der Körper kommen aus gemessenen
Literaturwerten, jeder Wert mit Herkunft). `ring_transmission` existiert
nicht im Code (grep: kein Treffer — der Bau ist das Atom). Ringe sind
Feld-Dämpfung, keine Objekt-Geometrie — sie manifestieren über den
absorption-Kanal.

### Atom 3 — Warp

Linsen-Kompiler: Gaia-BH-Kandidaten + ATNF-Pulsare mit gemessener Masse
werden zu Gravitationslinsen-Parametern kompiliert; WD-Modell-Massen stehen
aus; das f64-Fold-Muster aus Atom 1 ist die Vorlage (der Fold rechnet in
f64, wie der Rest der Linsen-Physik). Der Warp ist die Gravitationslinse
im Feld — eine Kraft, die das Feld krümmt.

### Atom 4 — Feld-Absorption (Okklusions-Reste → Feld)

Die Rückkehr der Verdeckung als Feld-Eigenschaft: kontinuierliche Opazität
(Partial-Transmission), atmosphärische Dämmerung, kleine Skala (Terrain/
Bauten — der Mechanismus ist skalenfrei, die Daten fehlen), Oszillator-
Eigenradius als Rekord-Slot, Transits als Feld-Dämpfung. Der
absorption-Slot lebt im Protokoll — das Atom ist die Manifestation:
welche Samples tragen Werte, wie faltet der Shader sie (der Alpha-Pfad
steht schon).

### Registrierte Grenzen (Atom 1 — kein Schnitt ohne Wort)

- Der 3D-Orbit des Planetenpunkts bleibt ausstehend — Ω (Azimut im
  Sky-Frame) ist ungemessen, der Schatten ist Ω-frei, ein Punktorbit wäre
  geraten.
- pscomppars trägt mehrere Parametersätze je Planet und keinen default_flag
  — der erste Satz je Planetenname zählt; fehlt ein Element → kein
  Schatten (0 honored).
- LuckyStar: decline (Vorhersagen sind Modell, keine Messung). Der rohe
  em-Lichtkurven-Kanal der Fresnel-Sphäre bleibt ausstehend — ein
  Quellen-Fund, kein Bau.

## Gates

- cargo check 0/0 (vier Kombis), cargo test komplett, naga-Validierung.
- Jeder Literatur-τ trägt seine Provenienz (Quelle + Messung) — ohne
  Herkunft kein Wert (0 honored).
- Ein Commit je Atom; TODO.md-Register im selben Commit.
- Diese Datei nach dem Abschluss archivieren (Regel in AGENTS.md).

## Nicht anfassen

Atom C des spektralen Oszillators, die Archivar-Arbeitsliste, Source-Port,
die 4D-Wahrheit (läuft), offene-atome (läuft).
