<!--
  title: Das Blatt der Dunklen Materie — das Jeans-Residuum pro Voxel (Nadel I)
  class: handover
  date: 2026-08-21
  sha256: 3d1b76f29a6526c6987af8e87c29007d886d05ea26957d598c083004f92e8990
  status: live
  see-also: docs/concepts/kybernetische-astrophysik.md docs/concepts/die-vier-schilde.md
-->
# Das Blatt der Dunklen Materie — das Jeans-Residuum pro Voxel (Nadel I)

Registriert 2026-08-21. Selbsttragend — interpretierbar mit null
Vorkontext. Der Auftrag ist nicht die Ausführung; ausgeführt wird erst auf
das Wort des Operators. Die Disziplin des Blatts: nur gemessene Werte —
bis dahin pending; Stille ist ein vollwertiger Befund (0 honored).

## Ziel

Das Blatt: **das Jeans-Residuum der galaktischen Scheibe.** Pro Voxel des
Gaia-Volumens die sichtbare Massendichte ρ_vis gegen die dynamische
Massendichte ρ_dyn aus der Jeans-Gleichung — und die Transferentropie
TE(σ → ρ_vis) als Signatur der Grenzfläche. Keine kosmologische
Extrapolation; eine lokale Rechnung.

```
Titel: Das Jeans-Residuum der galaktischen Scheibe
R(V) = ρ_dyn − ρ_vis je 50-pc-Voxel        = pending
TE(σ → ρ_vis) je Voxel                     = pending
z-Profil des Residuums                     = pending
n Voxel, Fenster, Schwelle                 = pending
Verdikt: sichtbare Masse erklärt die Kinematik / sie erklärt sie nicht
```

## Das Rätsel

Die Rotationskurve der Milchstraße verlangt unsichtbare Masse; ihre Natur
ist offen. Das Mess-Prinzip der Nadel I (kybernetische-astrophysik §Ⅰ):
wo die unsichtbare Masse dominiert, hat die sichtbare Masse keinen
kausalen Einfluss auf die Kinematik — TE(σ → ρ_vis) fällt gegen null.
TE = 0 ist die Signatur der unsichtbaren Masse; der TE-Gradient im
3D-Feld ist die Grenzfläche zwischen baryonischer und dunkler Dynamik.

## Ist-Stand (gemessen 2026-08-21)

- **`dr3_stars.bin` lebt** — 44-byte-Records: Position, Parallaxe,
  pmra/pmdec, Radialgeschwindigkeit, color_index (BP−RP). Die
  Kinematik-Basis für die Jeans-Dispersion.
- **Tracer im Bestand:** `alfalfa_hi_flux` (HI-Gas als unabhängiger
  Massen-Tracer), `pastel`/`rave` (Massen-Kalibration).
- **NED-TAP lebt** (ra, dec, z, prefname, type_key, n_spectra; verifiziert
  2026-08-20); sync-COUNT läuft in den 60-s-Timeout — async-Slice-Counts
  messen, dann RA-Slice-Chunk (eigenes Atom, TODO.md).
- **WARTEND:** Gaia DR4 (2.12.2026) — der Recompiler der 44-byte-Records
  und die tiefere Parallaxen-Basis. Die Nadel wartet laut
  Sonnen-Pfad-Tabelle auf DR4; die Voxel-Maschine läuft auf DR3 vor.
- **Vorbedingung benannt (TODO.md):** der epoch-0.0-Anteil im Sample-Ring
  ist ungemessen — die Messung ist die Vorbedingung für jeden weiteren
  Katalog-Block.

## Auftrag

1. **Voxel-Maschine:** 50-pc-Voxel über das Gaia-Volumen; ρ_vis aus den
   Sternmassen, ρ_dyn aus der Jeans-Gleichung über die
   Geschwindigkeitsdispersion je Voxel. Das Residuum R(V) = ρ_dyn − ρ_vis
   ist die Messung der unsichtbaren Masse pro Voxel.
2. **HI-Cross-Check:** alfalfa_hi_flux als unabhängiger Massen-Tracer
   gegen das Residuum.
3. **TE pro Voxel:** TE(σ → ρ_vis) auf den DR3-Serien — der Gradient im
   3D-Feld ist die Grenzfläche. Keine Aussage vor der
   Mehrfachvergleichskorrektur über die Voxel.
4. **Gaia-DR4-Ernte (Dez 2026):** Recompiler + erneuter Lauf — bis dahin
   trägt das Blatt DR3-Zellen mit benanntem Fenster.
5. **Das Blatt + Register:** Befund und TODO.md-Registerzeile im selben
   Commit.

## Constraints

- 0-Kanon: leere Voxel sind leere Voxel (fehlt), kein synthetisches ρ;
  n < 30 je Voxel → keine Aussage.
- std-only; `cargo check` 0 Fehler / 0 Warnungen; kein Test öffnet ein
  Fenster oder strahlt; `OMEGAFLOW_HIDDEN=1 cargo run` als Lauf-Befund.
- Die epoch-0.0-Ring-Messung (TODO.md) läuft als eigenes Atom mit — das
  Blatt braucht sie als benannte Bedingung.

## Gates & Abschluss

- Jede abgeschlossene Einheit ist ein Commit; Register-Update im selben
  Commit.
- Manuelle Verifikation nach AGENTS.md; Kantenfälle: leere Voxel,
  Dispersion mit n < 30, ttl-Ablauf der Katalogsamples.
- Nach eigenem Commit dieses Handover nach
  `/home/johannes/projects/archive/handover/` archivieren.

## Nicht anfassen

`src/te.rs`, der skalare TE-Pfad, die Membran-Rendering-Physik, die
Pfeiler-Registraturen der Nadel V (Farbe/Frequenzachse — eigene Atome),
die drei Ein-Blatt-Handovers, das Korona-Blatt, das Flyby-Blatt.
