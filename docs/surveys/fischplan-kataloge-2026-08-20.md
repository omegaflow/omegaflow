# Fischplan: die großen Kataloge (2026-08-20)

Forensik der zweiten Reihe. Exakte Tabellen-IDs, Spalten und Mechanismus je
Katalog — verifiziert gegen den live-TAP (`tapvizier.cds.unistra.fr`) am
2026-08-20.

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

## Die großen Kataloge können schon gefischt werden

`tap_compiler` hat die Groß-Mechanismen bereits (kein Neubau nötig):

- `--mag-bands <min> <max> <step>` — Magnituden-Banding mit `.part`-Resume und
  `upload_asset`. **Läuft in CI**: kernel_flatten.yml:448 (dr3_stars.bin).
- `--async <poll>` — UWS-Job (`tap_async`) für große Vollabfragen.
- `--where` — WHERE-Bänder (nutzt chunk_master.py lokal für
  pastel/wds/mktypes/denis).

Der einzige offene CI-Rest: pastel/wds/mktypes/denis laufen noch über
chunk_master.py (Python, lokal) statt über den Rust-Weg im monatlichen
Workflow.

## Exakte Tabellen (live-TAP, 2026-08-20)

| Katalog | Tabelle | RA/Dec | Messwert | z/Distanz | Kraft/Einheit | Zeilen | Mechanismus |
|---|---|---|---|---|---|---|---|
| NVSS | `VIII/65/nvss` | RAJ2000/DEJ2000 | S1_4 | — | em mJy | 1,8 M | `--async` |
| FIRST | `VIII/92/first14` | RAJ2000/DEJ2000 | Fpeak, Fint | — | em mJy | 1,0 M | `--async` (Live-TOP-2000 existiert) |
| GLADE+ | `VII/291/gladep` | RAJ2000/DEJ2000 | Bmag | zhelio/zcmb, dL | em mag | 22 M | `--mag-bands` |
| Fermi 4FGL | `IX/72/4fgldr4` | RAJ2000/DEJ2000 | EF100 | — | em erg/cm²/s (pending) | 7 k | inline |
| Chandra CSC 2.1 | `IX/70/csc21mas` | RAICRS/DEICRS | Fluxb | z | em erg/cm²/s (pending) | 400 k | `--async` |
| RAVE DR5 | `III/279/rave_dr5` | RAJ2000/DEJ2000 | Dist, TeffK | — | Sterne (star-bin + crossmatch) | 500 k | `--mag-bands`/`--async` |
| APOGEE | — (SDSS CAS, nicht VizieR) | — | — | — | Sterne | 500 k | SDSS-Root |
| NED | — (`ned.ipac.caltech.edu` TAP) | — | — | z | Galaxien | 100 M | eigener Root + `--where` |
| 2MASS | `II/246/out` | RAJ2000/DEJ2000 | Jmag/Hmag/Kmag | — | Sterne | 470 M | Bulk + `--mag-bands` |

## Blocker (je Katalog, nicht global) — Stand 2026-08-20

- **Fermi**: Energiefluss `erg/cm²/s` = pending Unit-Arm (convert_to_si) —
  erst der Arm, dann der Block. Drift benannt: der chandra_csc-Block
  (sources.φ) trägt `erg/cm2`, CSC-Fluxb ist physikalisch erg/cm²/s —
  Block-Label prüfen.
- **APOGEE**: SDSS-CAS-Root pending.
- **NED**: Root ist KEIN Blocker mehr — `https://ned.ipac.caltech.edu/tap/sync`
  antwortet auf den Standard-Sync-Stil, Tabelle `NEDTAP.objdir`
  (ra, dec, z, prefname, type_key, n_spectra) live verifiziert
  (2026-08-20). Blocker ist die Größe: sync-COUNT läuft in den 60-s-Timeout
  (Server: „use async mode") — async-Slice-Counts messen, dann
  RA-Slice-Chunk-Schritt (eigenes Atom).
- **2MASS**: sync-COUNT auf II/246/out überschreitet das 60-s-Fenster
  (gemessen 2026-08-20) — der adaptive `--mag-bands`-Bander ist für 470 M
  nicht CI-tragfähig. Bulk-Route (cdsarc-ftp II/246) braucht einen
  Kompilator (eigenes Atom); ein erklärter heller Schnitt bleibt
  Kurationsfrage.
- RAVE: wired im chunk_catalogs-Job (kernel_flatten.yml), Spalten live
  verifiziert (RAJ2000/DEJ2000/HRV/TeffK). Asset ausstehend bis zum
  ersten Lauf.
- **GLADE+**: Spalten live verifiziert (RAJ2000/DEJ2000/Bmag/zhelio/zcmb/
  dL[Mpc]), aber pending — drei gemessene Blocker: (1) der
  `--mag-bands`-Bander kappt am Schrittboden still mit TOP limit
  (~180 k Zeilen je 0.25-mag-Band, Peak >1 M); (2) Ein-Asset ≈ 2.4 GB >
  2-GB-Release-Limit; (3) 22 M > MAX_SAMPLES 4.19 M — der Feld-Rebuild
  wirft die ältesten Samples. Detail: docs/surveys/chunk-plan-2026-08-20.md.

## Reihenfolge — Stand 2026-08-20

1. ERLEDIGT: NVSS + FIRST + Chandra über `--async` (Workflow-Schritte +
   sources.φ-Blöcke leben).
2. FERTIG: RAVE DR5 über 24 RA-Slices à 15° + Gaia-Crossmatch
   (HRV-Gate = River) — 472845 Zeilen auf dem CDN, Block in sources.φ.
   Der --async+JOIN-Weg hing PENDING (gemessen verworfen).
3. PENDING: GLADE+ (RA-Slice/async + Quadranten-Assets + Sample-Budget-
   Atom) + Fermi 4FGL (erg/cm²/s-Unit-Arm).
4. PENDING: 2MASS (Bulk-Kompilator-Atom) + NED (async-Counts → Chunks)
   + APOGEE (SDSS-Root).
