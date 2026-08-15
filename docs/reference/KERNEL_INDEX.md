# KERNEL_INDEX — ω-Flattener-Quelleninventar

Generiert von `ephemeris_compiler --summarize` aus `phi/sources_index.φ`.
Kanonisch ist `sources_index.φ` (maschinenlesbar); dieses Dokument ist die Lesefassung.

## Wurzeln
- https://ssd.jpl.nasa.gov/ftp/ (JPL SSD — Planeten-/Satelliten-/Kleinkörper-SPKs, PCKs, DASTCOM)
- https://naif.jpl.nasa.gov/pub/ (NAIF SPICE — generic_kernels + Missions-Trees)

Nur HTTPS. Voll rekursiv. CK/IK/SCLK/EK/DBK werden indexiert, aber vom Flattener
nicht geladen (keine Kameras, keine Bordzeit — NAIF-PDF-Bewertung).

## Familienbestand (321022 Dateien, 7092042302065 B)
| Familie | Dateien | Bytes | Neueste mtime (unix) |
|---|---|---|---|
| bpc | 5234 | 779673811 | 1786640820
| ck | 103167 | 3559724308918 | 1786711500
| dastcom | 7 | 258993 | 1786734900
| dsk | 940 | 259142925193 | 1784859060
| fk | 1210 | 775639387 | 1786335480
| gm | 11 | 438272 | 1783549920
| ik | 1245 | 30660076 | 1784875920
| lsk | 259 | 2622222 | 1784249340
| misc | 179630 | 2892774313886 | 1786734900
| mk | 11460 | 484609627 | 1786700400
| pck-text | 1021 | 220731651 | 1786335480
| sclk | 3961 | 128360111 | 1786520520
| spk | 12445 | 342260732631 | 1786688700
| spk-planets | 83 | 2241091584 | 1784078700
| spk-satellites | 349 | 33475935703 | 1783940280

## System-Auflösung (Flattener-Auswahl)
| System | SPK | PCK |
|---|---|---|
| planets | de721_full.bsp | pck00011.tpc |
| jupiter | jup365.bsp | pck.jup365.tpc |
| saturn | sat441.bsp | pck.sat441.tpc |
| mars | mar099s.bsp | pck.mar099.tpc |
| uranus | ura184_part-3.bsp | pck.ura182.tpc |
| neptune | nep097.bsp | pck.nep097.tpc |
| pluto | plu060.bsp | pck.plu060.tpc |

Auswahlregel: erste Ziffernfolge im Namen = Version, höchste gewinnt; bei
Gleichstand bevorzugt die ssd.jpl.nasa.gov-Wurzel, dann die Light-Variante
(`l`/`s`-Suffix), dann der kürzeste Name. Planeten: höchste DE-Basis, bei
Gleichstand volle Präzision (nicht die `s`-Kurzvariante); `_part-N`-Dateien
einer Basis werden vollständig geladen. pck0001*-Dateien werden aufsteigend
geladen (00011 überschreibt 00010, NAIF-Precedence).

## Mond-PCK-Befund (Session 2026-08-14)
Die Mond-Text-PCKs (ssd.jpl.nasa.gov/ftp/misc/pck/
pck.sat441/pck.jup365/pck.mar099/pck.ura182/pck.plu060, je .tpc) tragen
RADII + POLE für die Monde, aber KEINE J2/J4-Werte — die einzigen Treffer sind
Kommentarzeilen (BODYnnn_JCOEF-Dokumentation). Mond-Harmonische aus Text-PCKs
manifestieren daher 0 (keine Fabrikation). Echte Zonal-Terme liegen in
den Binary-PCKs (moon_pa_de440, Satelliten-.bpc) — das ist K02. pck00010.tpc
deckt Phobos/Triton/Charon mit POLE+RADII ab; GM aller Körper kommt aus
gm_Horizons.pck (km³/s², Parser skaliert ×1e9).

## Kraft-Abdeckung (9-Kanal-Matrix)
| Kanal | Was der Index trägt | Rest |
|---|---|---|
| gravity | SPK (Bahnen), PCK (GM/Radii/J2/J4/Pole), DSK (Form, später), DASTCOM (Kleinkörper) | CDDIS (Auth, I03) |
| em | DASTCOM-Albedo; Tycho-2 (K04) liegt nicht auf diesen Wurzeln | HEASARC/IRSA/MAST (TAP, sources-Repo) |
| acoustic | — | GONG/SOHO, NOAA (Kuration) |
| seismic-body | — | USGS/IRIS (live), PDS InSight/Apollo (Auth/API) |
| seismic-surface | — | USGS (live) |
| thermal | DASTCOM H/Albedo | GOES-Thermal (Kuration) |
| diffusion | — | SWPC/OMNI (live) |
| advective | — | DSCOVR/SWPC (live) |
| electric | — | SWPC/OmniWeb/Swarm (Kuration) |

## Flatten-Policy
K01: Planeten + Monde (SPK/PCK) + Sonden (Horizons-Compiler). Kleinkörper sind im
Index registriert (Familie `spk`), der Flatten-Pass liegt am K03-Zweig (DASTCOM+Kepler).
