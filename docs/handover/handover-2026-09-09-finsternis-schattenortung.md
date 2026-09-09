<!--
  title: Handover — Finsternis-Schattenortung (aus den Weltlinien) + die Ephemeriden-Linien (INPOP/EPM-Sonne, drei NASA-Editionen, CI-Regeneration)
  class: handover
  date: 2026-09-09
  sha256: f3e022db97850debc06a15ae9ed916e034340f29ebbfa804d369381aede72a77
  status: live
  see-also: docs/TODO.md tools/measure/src/bin/eclipse_shadow_probe.rs tools/harvest/src/bin/de_compiler.rs .github/workflows/de44-cdn.yml .github/workflows/inpop-epm-cdn.yml phi/sources.φ
-->

# Handover — Finsternis-Schattenortung + die Ephemeriden-Linien

Übergabe für die nächste Sitzung. Gebaut und **committet**: die
Finsternis-Schattenortung (das Rätsel „Punkt größter Bedeckung aus eigenen
Weltlinien"), die INPOP/EPM-Sonnen-Ernte, die drei NASA-Editionen als Linien
und deren CI-Regeneration. Der Arbeitsbaum trägt daneben fremde, nicht
committete Änderungen (parallele Sitzung).

## 1. Was gebaut wurde

### eclipse_shadow_probe (`tools/measure/src/bin/eclipse_shadow_probe.rs`)

- Rechnet die Finsternis rein aus den Sonne/Mond/Erde-Weltlinien; kein
  Finsternis-Katalog in der Rechnung, der Espenak-Kanon liegt erst im Verdikt
  daneben. Fünf Linien: de441, de440, de442 (ssd.jpl.nasa.gov), inpop19a
  (ftp.imcce.fr), epm2021 (ftp.iaaras.ru).
- Stufe 1: geozentrische Syzygie (minimale Elongation). Stufe 2a: größte
  Finsternis (Schattenachse am nächsten zum Geozentrum → Schnittpunkt).
  Stufe 2b: tiefste Bodenbedeckung (der größte-Dauer-Locus). Stufe 3: alle
  Linien nebeneinander, paarweise Riß. `--day-unix` für andere Epochen.
- Oberflächen-Abbildung über `body_fixed_to_icrs_smooth` (nicht
  `body_fixed_to_icrs` — Befund unten). Magnitude = Verhältnis der
  scheinbaren Durchmesser (Espenak-Glossar „strictly a ratio of diameters"
  + der Wikipedia-2024-Abschnitt bestätigen das).
- 8 Tests inkl. Kalibrier-Gate (datengebunden: Punkt < 15 km, Magnitude
  < 0,002 vom Kanon).
- Ergebnis 2017: Punkt 36.9664N 87.6176W = **4,8 km** vom Kanon (36.9667N
  87.6717W); Magnitude 1,03102 (+0,0004); Zeit 18:25:34.9Z = **65 s vor** dem
  Kanon 18:26:40 (flaches h-Minimum, reproduzierbar auch 2024, ~71 s).

### Ephemeriden-Linien (Ernte + Manifestation)

- INPOP/EPM-Sonne: `inpop_compiler`/`epm_compiler` um Sonnen-Scope erweitert;
  beide Kernels tragen target 10 @ SSB (gemessen). `ephemeris_inpop_sun.bin`,
  `ephemeris_epm_sun.bin` manifestiert. Weberin-Sonnen-Triade
  (DE441/INPOP/EPM) united: 22.4/16.8/32.0 km.
- NASA-Editionen: `de_compiler` (netloc-taggbar, `--label`/`--netloc`/
  `--ci-mode`, versagt laut bei 0 geschriebenen Körpern) + `de44-cdn.yml`.
  de440 (NAIF de440.bsp), de441 (ssd `ephemeris_*.bin`, Horizons-Banner
  `{source: DE441}` gemessen), de442 (JPL de442.bsp). Alle drei manifestiert
  und in `phi/sources.φ` registriert.
- **Die Messung:** Die drei NASA-Ausgaben sind eine Stimme — geozentrischer
  Mond 0,3–2,0 m auseinander (Erdmitte 0–191 m), unter 2 ms Finsternis-Zeit.
  Der Riß liegt zwischen den Häusern: inpop19a 3,05 s / 1,7 km, epm2021 ≈ DE.
  Regime-Bild: innen (Mond/Erde, LLR, Meter) geschwister-eng; außen
  (Eisriesen, Kilometer) divergiert sogar die Familie.

## 2. Befunde

1. **`body_fixed_to_icrs` Matrix-Pfad (Kern-Befund, unregistriert):** der
   Nearest-Rotations-Matrix-Zweig dreht die Erdoberfläche 2017 um ~117° Länge
   falsch (Sub-Solar-Punkt: Matrix-Pfad −2.2N/+21.2E; `_smooth` 12.0N/−95.8W
   exakt). Die Probe nutzt `_smooth`; die Membran und andere Proben nehmen den
   Matrix-Pfad, sobald ein Bin `rotation_matrices` trägt. **Gehört als
   TODO-/Befund-Zeile in den Register.**
2. **~65 s Zeit-Offset** der h-Minimum-Instants (flaches Minimum; Position und
   Magnitude exakt). Als Restbefund dokumentiert, optional Nachforschung.
3. **EPM-Stale (benannt):** drei rote `inpop-epm-cdn`-Läufe (Extraktion leer
   im Runner; lokal identisch grün, auch mit rust 1.98.1) → ein frischer
   Re-Dispatch ist grün (EPM 10/10). Root-Cause ungemessen; eine
   sha256-Diagnose war vorbereitet, aber zurückgenommen (blockiert durch den
   fremden te.rs-Edit).

## 3. Offene Steine

1. **Register-Pflicht:** der `body_fixed_to_icrs`-Befund (2.1) — wichtigster
   offener Punkt.
2. sha256-Diagnose für `inpop-epm-cdn.yml` / `de44-cdn.yml` (benennt den
   nächsten Stale sofort) — blockiert durch te.rs.
3. Eikonal (Beugung über das GEBCO-Gitter — von Adak +57 / Hilo +82 bestellt).
4. M9.1-Picker (eigene Ortung, nach dem Eikonal).
5. Fremd: `src/mathematikerin/te.rs:3705` Syntaxfehler (parallele Sitzung) —
   der Commit-Gate kompiliert die Lib und hängt daran.

## 4. Zustand des Arbeitsbaums

- **Committet & gepusht** (diese Sitzung): `e68f86a` (Sonnen-Ernte + Triade),
  `2c71fff` (Probe mit drei Linien + Compiler-Guard), `6df3da5`
  (NASA-Editionen in der Probe), `26ea4ac` (sources.φ de44x), `daa0463`
  (de_compiler + de44-cdn.yml). Nichts von dieser Sitzung liegt als Stray im
  Baum.
- **Fremde uncommittete Änderungen:** `src/mathematikerin/te.rs`
  (Syntaxfehler Z.3705, parallele Sitzung); `docs/TODO.md` (entangled);
  daneben ältere fremde Tracked-Änderungen (docs/specs, teils `src/archivar/*`).
  Der Commit-Gate kompiliert den ganzen Workspace und ist durch te.rs blockiert.
- `cargo check -p omegaflow-measure` und `-p omegaflow-harvest`: diese Sitzung
  warnungsfrei; die Probe trägt 8 grüne Tests.
