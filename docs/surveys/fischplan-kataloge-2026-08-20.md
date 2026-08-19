# Fischplan: die großen Kataloge (2026-08-20)

Forensik der zweiten Reihe. Exakte Tabellen-IDs, Spalten und Mechanismus je
Katalog — verifiziert gegen den live-TAP (`tapvizier.cds.unistra.fr`) am
2026-08-20. Der Plan ist die Forschungsarbeit; die Ausführung hängt an zwei
bereits offenen Baustellen (Chunk-Kompilation, Unit-Arme).

## Korrektur zur ersten Forensik

- **FIRST ist nicht ungefischt.** `phi/sources.φ:1661` trägt bereits einen
  Live-`TOP 2000`-TAP-Block (`VIII/92/first14`, Fpeak/Fint in mJy). Es fehlt
  nur das Voll-Kompilat.
- GCVS-Muster: ein großer Katalog = Voll-Kompilat (`kernel_flatten.yml` → CDN)
  + optional ein Live-`TOP N`-Query. Die zweite Reihe hat bisher höchstens das
  `TOP N` (nur FIRST), nie das Voll-Kompilat.

## Zwei Schichten je Katalog

1. `kernel_flatten.yml`-Schritt: `tap_compiler` → `.json`/`.bin` → CDN.
2. `sources.φ`-Block: `url` (CDN-Asset) + `field`-Zeilen (Semantik →
   Name/Kernel/Kraft/Einheit/TTL). Format-Muster: `avo_rad.json`/`bzcat5.json`
   (sources.φ:1136–1152) bzw. `gcvs_cat.json` (1284–1297).

## Exakte Tabellen (live-TAP, 2026-08-20)

| Katalog | Tabelle | RA/Dec | Messwert | z/Distanz | Kraft/Einheit | Zeilen | Mechanismus |
|---|---|---|---|---|---|---|---|
| NVSS | `VIII/65/nvss` | RAJ2000/DEJ2000 | S1_4 | — | em mJy | 1,8 M | chunk |
| FIRST | `VIII/92/first14` | RAJ2000/DEJ2000 | Fpeak, Fint | — | em mJy | 1,0 M | chunk (Live-TOP-2000 existiert) |
| GLADE+ | `VII/291/gladep` | RAJ2000/DEJ2000 | Bmag | zhelio/zcmb, dL | em mag | 22 M | chunk |
| Fermi 4FGL | `IX/72/4fgldr4` | RAJ2000/DEJ2000 | EF100 | — | em erg/cm²/s (pending) | 7 k | inline |
| Chandra CSC 2.1 | `IX/70/csc21mas` | RAICRS/DEICRS | Fluxb | z | em erg/cm²/s (pending) | 400 k | async/chunk |
| RAVE DR5 | `III/279/rave_dr5` | RAJ2000/DEJ2000 | Dist, TeffK | — | Sterne (star-bin + crossmatch) | 500 k | async |
| APOGEE | — (SDSS CAS, nicht VizieR) | — | — | — | Sterne | 500 k | SDSS-Root |
| NED | — (`ned.ipac.caltech.edu` TAP) | — | — | z | Galaxien | 100 M | eigener Root + chunk |
| 2MASS | `II/246/out` | RAJ2000/DEJ2000 | Jmag/Hmag/Kmag | — | Sterne | 470 M | Bulk + chunk |

## Blocker

- **`tap_compiler --limit` default 50 000.** Alles > 50 k braucht `--async`
  oder `--chunk-bands`. Genau dafür steht der offene TODO-Punkt
  „CI-Chunk-Kompilation" (chunk_master.py lokal → Rust in CI).
- **Fermi/Chandra**: Energiefluss `erg/cm²/s` ist ein pending Unit-Arm
  (convert_to_si) — erst der Arm, dann der Block.
- **APOGEE/NED**: fremde TAP-Roots (SDSS CAS / NED) — eigene `--root`.

## Reihenfolge

1. CI-Chunk-Kompilation bauen (`tap_compiler --chunk-bands` + Merge, analog
   chunk_master.py) — einmalig, schließt NVSS, FIRST, GLADE+, Chandra, RAVE,
   2MASS, NED.
2. Fermi 4FGL inline (7 k) + der `erg/cm²/s`-Unit-Arm.
3. APOGEE (SDSS-Root) + NED (eigener Root) + 2MASS (Bulk-Harvest).
