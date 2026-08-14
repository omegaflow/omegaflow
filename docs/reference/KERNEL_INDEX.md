# KERNEL_INDEX — ω-Flattener-Quelleninventar

Dieses Dokument ist die Lesefassung von `phi/sources_index.φ` (maschinenlesbar,
kanonisch). Beide entstehen im `kernel_flatten.yml`-Index-Job:
`ephemeris_compiler --index` (voller rekursiver HTTPS-Crawl von
`ssd.jpl.nasa.gov/ftp/` und `naif.jpl.nasa.gov/pub/`, resumabel) und
`ephemeris_compiler --summarize` (diese Lesefassung). Der Job überschreibt
beide Dateien monatlich (+ workflow_dispatch) und committet sie.

Der Stand unten dokumentiert die Session-Befunde, bis der erste CI-Lauf die
vollen Trees trägt (Smoke-Crawl 2026-08-14: Tiefe 3, 2658 Dateien — Mechanismus
verifiziert).

## Befund Session 2026-08-14 (K01)

- Mond-Text-PCKs liegen auf `ssd.jpl.nasa.gov/ftp/misc/pck/pck.{sat441,jup365,
  mar099,ura182,plu060}.tpc`. Sie tragen RADII + POLE für die Monde, aber KEINE
  J2/J4-Werte — die einzigen Treffer sind Kommentarzeilen (BODYnnn_JCOEF-Doku).
  Mond-Harmonische aus Text-PCKs manifestieren 0 (0 honored, keine Fabrikation).
  Binary-PCKs (moon_pa_de440_200625.bpc) tragen laut pck.req NUR Orientierung
  (Typen 2/3/20, „orientation only") — die Präzisions-Röhre ist die stype-4-
  Nutationssektion (K02), nicht J2/J4. Echte Mond-Zonal-Terme liegen außerhalb
  der Flattener-Wurzeln (GRAIL-Modelle, Kanal-Forschung).
- `gm_Horizons.pck` (ssd.jpl.nasa.gov/ftp/xfr/) trägt GM aller Körper in km³/s²
  (Parser skaliert ×1e9); es zerlegt Pluto korrekt in 999 + 901 (Charon).
  `pck00010.tpc` deckt Phobos/Triton/Charon mit POLE+RADII ab.
- Flattener-Auswahl (erste Ziffernfolge = Version, höchste gewinnt; Gleichstand:
  SSD-Wurzel, dann Light-Variante, dann kürzester Name; `_part-N`-Dateien einer
  Basis werden vollständig geladen):
  | System | SPK | PCK |
  |---|---|---|
  | planets | de442.bsp | pck00011_n0066.tpc (pck0001*-Folge aufsteigend) |
  | jupiter | jup365.bsp | pck.jup365.tpc |
  | saturn | sat480.bsp | pck.sat441.tpc |
  | mars | mar099.bsp | pck.mar099.tpc |
  | uranus | ura184_part-1/2/3.bsp | pck.ura182.tpc |
  | neptune | nep105.bsp | pck.nep097.tpc |
  | pluto | plu060.bsp | pck.plu060.tpc |
- Dev-Beweis (Smoke, lokal): de440s.bsp + mar099s.bsp + gm_Horizons + pck00010
  + geophysical → 13 Binaries (dynamische Segment-Enumeration). Gegen die
  28 verifizierten /tmp-Referenzen: 11 Körper koeffizienten-identisch
  (relΔ = 0.0 über 3425 t0-gematchte Granulen); phobos/deimos-Referenzen sind
  km-Legacy (Δ ≈ 10³ — exakt der K01-Ersatz); Pluto/GM-Δ = gm_Horizons-
  Systemzerlegung. Data-Contract (Granulen-Stride 448 B = 18 Koeffizienten,
  stype-1 12 Params, stype-3 Rotationen) intakt.
- Kraft-Abdeckung (9-Kanal-Matrix): gravity = SPK/PCK/DSK/DASTCOM vollständig
  auf beiden Wurzeln; em/thermal = DASTCOM-Parameter (K03) bzw. Tycho-2 (K04,
  nicht auf diesen Wurzeln); acoustic/seismic/diffusion/advective/electric
  bleiben Kurations-APIs (siehe TODO-Kraftmatrix).
- CK/IK/SCLK/EK/DBK werden indexiert, vom Flattener nicht geladen (keine
  Kameras, keine Bordzeit — NAIF-PDF-Bewertung). Kleinkörper-SPKs sind im
  Index registriert (Familie `spk`); ihr Flatten-Pass liegt am K03-Zweig
  (DASTCOM+Kepler).
- K02-Befund (2026-08-14): src/bpc.rs (DAF, Binary-PCK Typ 2) + stype-4-
  Nutationssektion (additiv, RA/DEC/PM) in Compiler/Runtime/Protokoll-Doku
  (FORCE_SYSTEM.md). Mond-Text-PCKs tragen echte NUT_PREC-Reihen → die Monde
  bekommen die stype-4-Nutation aus dem Text-Kanal. moon_pa_de440_200625.bpc:
  Leser verifiziert (Fenster 1550–2650, Grad-9-Fits), aber die Winkel folgen
  nicht der IAU-Pol-Konvention (J2000-Pol ≈ RA −0,05°/DEC +0,43°, W-Drift
  0,23°/Tag) — der Moon-PA-Merge läuft über die FK-Kette MOON_PA_DE440 +
  TKFRAME_31009 (moon_de440_250416.tf), das ist K05.
