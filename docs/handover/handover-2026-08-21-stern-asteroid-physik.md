<!--
  title: Handover: Stern-/Asteroiden-Physik — abgeleitete Geometrie + Ernte-Folgen
  class: handover
  date: 2026-08-21
  sha256: 1dea70a90c32c47714fcf2e5a2c3dcecf465d9724bfabdb6c827e43d9de5a8f9
  status: live
  see-also: TODO.md phi/pipeline/katalog/ docs/handover/handover-2026-08-21-katalog-luecken.md
-->

# Handover: Stern-/Asteroiden-Physik

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext. Der Auftrag
ist nicht die Ausführung; ausgeführt wird erst auf das Wort des Operators.

Register-Quelle: TODO.md, Abschnitt „Stern-/Asteroiden-Physik — abgeleitete
Geometrie + Ernte-Folgen". Die Daten sind geerntet (Sternkinematik
pmra/pmdec/rv, Farbe Teff/BPmag/RPmag/Gmag via gaiadr3-Crossmatch;
Asteroiden-Größe via NEOWISE/AKARI in `phi/pipeline/katalog/
asteroid_diameters_*.φ`). Offen ist die Nutzung — reine Geometrie, die
sonst nirgends liegt, weil alles einen ICRS-4D-Rahmen teilt.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # sauber oder fremde Arbeit nennen
cargo check                      # 0/0
grep -n "hill_radius_m" src/dastcom.rs src/archivar.rs   # das lebende Gate
```

Referenzen (stehend): `src/dastcom.rs` (der Asteroiden-Record),
`src/archivar.rs` (der SpatialHash), `phi/pipeline/katalog/`,
`phi/pipeline/ledger.φ`, TODO.md.

## Die Einheiten (reine Geometrie)

### A. Hill-Sphäre je Asteroid — Manifestation

Formel repariert: r = a·(1−e)·(m/3M☉)^⅓. `hill_radius_m` lebt
(src/dastcom.rs:187) und ist heute nur Gate — `is_none` im Hash
(src/archivar.rs:10184) — der Wert fließt nirgends. Offen ist die
Manifestation: der Hill-Radius als räumliche Reichweite des Samples
(extent- oder reach-Slot des eigenen Körpers, je nachdem was das Feld
trägt — die Entscheidung ist Teil des Atoms).

### B. Hydrostatische Abplattung aus Rotation

Rotationsperiode (LCDB) + Radius (NEOWISE) + Dichte (Masse) →
Oblatheit im Gleichgewicht. Drei Kataloge übereinander — niemand macht
das systematisch. Die Form-Slots (j2/j4/r_eq) sind seit Atom 7 Pad —
dieses Atom ist ein Weg, sie wahr zu füllen (wo die Messung es trägt).

### C. Co-moving Gruppen / Sternströme

Position + 3D-Geschwindigkeit → Mitgliedschaft als Geometrie des
Geschwindigkeitsfelds.

### D. Sternbegegnungen

Welche Sterne nähern sich der Sonne (Gl-710-Problem) — für JEDEN Stern
live, aus der geernteten Kinematik.

### E. Paarweise 3D-Sternabstände

N² — auf Anfrage, kein Vorab-Feld.

### F. Oberflächengravitation + Fluchtgeschwindigkeit der Asteroiden

g = GM/r², v_esc = √(2GM/r) — mit dem geernteten GM.

### G. LCDB-Rotationsachsen + DAMIT-Formmodelle

Neue Quellen (grind-pro, heikler Join/Parsing): Pol, nicht nur Periode;
3D-Formen → j2/r_eq. Empfohlene Reihenfolge: Hill/Abplattung (A/B) →
LCDB/DAMIT (G). Die Source-Arbeit läuft über den einen Pfad
(docs/SOURCE_PORT.md).

### H. H-Schätzung vs. NEOWISE

Für die Körper, wo DASTCOM einen abgeleiteten (nicht gemessenen) Radius
trägt — registriert, nicht entschieden. Die Entscheidung ist die Einheit.

## Die Ernte-Folgen (Kompilate — CI, kein Hand-Schnitt)

### I. Sternbin-rv-Rekompilation (44-B-Records)

Die Compiler schreiben 44-B-Records (8+8+7×4: ra, dec, pm_ra, pm_de, plx,
mag, flux, farbe, rv in m/s); `parse_star_record` verlangt exakt 44 Byte
(src/archivar.rs:10255), kein rv=0.0-Ersatzwert (0 honored). Offen:
Rekompilation von `dr3_stars.bin` + `bright_stars.json` +
CDN-Remanifestation (CI, kernel_flatten-catalogs). Die Legacy-40-B-Bins
manifestieren nicht — die Sterne bleiben dunkel, bis die 44-B-Binaries
gebaut sind (pending, keine Fabrikation); bis dahin fließt rv nur aus den
JSON-cmap-Quellen (denis `radvel rv`).

### J. CDN-Rekompilat ephemeris v3

Die ephemeris_{body}.bin-Assets sind noch v2 — der nächste
kernel_flatten-Lauf schreibt v3 (0x02 + u16-Präsenz-Maske). Bis dahin
liest der v2-Arm (CI-Reihenfolge eingehalten: Code zuerst, Rekompilat
folgt) und alt-Slot + GM-Slot tragen das benannte Wire-Pad.

### K. kernel_flatten-Neulauf

ephemeris_compiler n_sections 2→3 (rotationslose Körper wurden verworfen,
Rotation abgeschnitten) — CDN-Neukompilat verifizieren (rotationslose
Körper laden, Rotations-Matrizen präsent).

## Verifizierter Kontext (2026-08-21)

- Die Sterne sind gewöhnliche Samples im inertialen SpatialHash
  (Motion::Spherical, Atom 5); MAX_SAMPLES = 1<<22 — das Sample-Budget
  läuft in `handover-2026-08-21-offene-atome.md` (NICHT anfassen; seine
  Messung begrenzt jeden weiteren Katalog-Block).
- Die Katalog-Lücken (GAIA DR4, 2MASS, GLADE+ …) laufen in
  `handover-2026-08-21-katalog-luecken.md` (NICHT anfassen) — dieses
  Dokument trägt nur die abgeleitete Geometrie und die Kompilat-Folgen.

## Gates

- cargo check 0/0 (vier Kombis), cargo test komplett.
- Jede abgeleitete Größe trägt ihre Formel und ihre Quellen — keine
  Schätzung ohne Herkunft.
- Ein Commit je Einheit; TODO.md-Register im selben Commit.
- Diese Datei nach dem Abschluss archivieren (Regel in AGENTS.md).

## Nicht anfassen

Katalog-Lücken/Zeitkritisch (eigene Übergabe), offene-atome
(Sample-Budget/GLADE+), die Sphären, Source-Port-Pipeline.
