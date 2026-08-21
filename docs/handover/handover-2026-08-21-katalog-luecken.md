<!--
  title: Handover: Katalog-Lücken — Wellen I–III, Zeitkritisch, Unit-Arme
  class: handover
  date: 2026-08-21
  sha256: 1178425036102c7dfd38007950b770533a1bda0a2fc8c3cae1d7d82d5cd0e5b1
  status: live
  see-also: docs/SOURCE_PORT.md TODO.md phi/pipeline/ledger.φ docs/handover/handover-2026-08-21-source-port-pipeline.md
-->

# Handover: Katalog-Lücken

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext. Der Auftrag
ist nicht die Ausführung; ausgeführt wird erst auf das Wort des Operators.

Register-Quellen: TODO.md, Abschnitte „Source-Port" (Katalog-Lücken,
Zeitkritisch) und „Curation & Quellen". Die Pipeline-Mechanik (Queue,
Grind, Zustandsmaschine) trägt die eigene Übergabe
(`handover-2026-08-21-source-port-pipeline.md`); dieses Dokument trägt
die konkreten Lücken mit ihren verifizierten Tabellen/Spalten. Der
eine Pfad ist `docs/SOURCE_PORT.md`; das Detail-Register
`phi/pipeline/ledger.φ`.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # sauber oder fremde Arbeit nennen
cargo check                      # 0/0
grep -n "2MASS\|GLADE\|NED" phi/pipeline/ledger.φ | head
```

Referenzen (stehend): Archivierte Übergaben (mit exakten Tabellen-IDs +
Spalten + Mechanismus):
`/home/johannes/projects/archive/handover/handover-2026-08-20-fischplan-
kataloge.md` und `handover-2026-08-20-chunk-kataloge.md`.

## Die Lücken (genuin, verifiziert gegen alle drei Register)

### A. Photometrie/Spektroskopie

2MASS PSC (siehe N — Bulk-Atom), RAVE DR6 (RAVE DR5 erledigt — III/279/
rave_dr5, 472845 Zeilen, 24 RA-Slices à 15°, rv-Gate HRV, Asset +
sources.φ-Block leben), APOGEE/GALAH.

### B. Extragalaktisch

NED (läuft in offene-atome — NICHT anfassen), HyperLEDA/PGC.

### C. Radio-Kontinuum (Achse leer)

TGSS ADR, SUMSS, RACS, LoTSS, VLASS. NVSS/FIRST erledigt (Workflow +
Blöcke) — die Achse hat Vorbilder, keine Einträge.

### D. High-Energy

Fermi 4FGL-DR4 (Unit-Arm pending), AMS-02. Chandra CSC 2.1 erledigt.

### E. Sonnensystem

PDS (Instrumentendaten), MPC-Live (mpcorb_extended.json.gz).

### F. TAP-Indexe

MAST, CADC, ESASky, NOIRLab Data Lab, NED (siehe B).

### G. Terrestrisch

EarthScope-FDSN, EPOS, SeaDataNet, Smithsonian GVP, Natural Earth.

### H. Welle II (Recherche 2026-08-17)

Diffusion/Chemorezeption unbesetzt — TCCON (verifiziert, tccondata.org,
Registrierung); pending Verifikation: AGAGE, NDACC, WDCGG, GLODAP, EBAS.
electric: WWLLN (registriert/restringiert) — Force-Gate klären, sonst
refused. em terrestrisch: NSRDB/BSRN (Bodensolar fehlt) — NSRDB pending.
gravity: BGI/GGP-Bodengravimetrie (IGETS nur indexiert) — pending
Verifikation.

### I. Welle III (genuin, Zugriffsarten unverified)

electric — AMPERE, GloCAEM, USArray-MT; diffusion — EMEP/CCC, WDCRG,
European Waterbase; em — NEUBrew (UV), THEMIS/ASI (Polarlicht, CDF),
COSMOS2025/COSMOS-Web, INTEGRAL, ATLAS-RefCat2, Subaru HSC-SSP, TIC;
kosmisch/Neutrino — CREDO, KM3NeT; Geodäsie — ILRS, IVS-EOP,
DORIS-Live, GRACE-FO-Mascons (L2/L3); Atmosphäre/Ozean — E-GVAP,
Wyoming-Soundings, BGC-Argo-live, IOOS-HFRNet, NOAA-NRS (Ozean-Lärm),
MIROVA.

### J. Crossmatch indexiert → live heben

GALEX-GUVcat (UV), SkyMapper DR4, UKIDSS/VISTA/VIKING (NIR), DES DR2 /
Legacy Surveys DR10.

### K. ESA/Geomagnetik

Swarm TCT-E-Feld (keyless), VirES-Aeolus, SMOS, MERIS/SAR/Landsat
Kandidaten.

## Zeitkritisch (die Termine sind die Messungen)

- Gaia DR4 (2. Dez 2026) — dr4_stars.bin + DR4-Schema im tap_compiler
  (5,5 a, halbierte Parallaxenfehler, Gaia-Exoplaneten).
- Rubin LSST DR1 (Ende Juni 2028), Alerts live (Broker declined).
- GCVS-Stand prüfen (HEASARC-Update Juni 2026 vs. gcvs_cat.json).
- Euclid DR1 (Okt 2026); SDSS-V.
- eROSITA-DR2 (Juli 2026 erschienen — prüfen ob via HEASARC-tap_index
  erreichbar).
- SPHEREx (IRSA VOAPI + AWS S3 + FITS, Quick-Release live, Voll-Katalog
  2026 — verifiziert).
- DESI DR1 (NOIRLab Astro Data Lab TAP, ~18 Mio Spektren — verifiziert).
- Roman (2027), 4MOST/WEAVE (2026) — unverified.

## N. Die gemessenen Blocker (keine Wiederholung der Messung)

- 2MASS-Befund: sync-COUNT auf II/246/out > 60 s (gemessen 2026-08-20) —
  der --mag-bands-Bander ist für 470 M nicht CI-tragfähig; Bulk-Route
  (cdsarc-ftp) braucht einen Kompilator (eigenes Atom), ein erklärter
  heller Schnitt bleibt Kurationsfrage.
- VizieR-async-Befund: --async + gaiadr3-JOIN hängt PENDING — UWS-Jobs
  sind IP-gebunden: stirbt der Runner, verwaist der Job. RA-Slices sind
  der Weg für Crossmatch-Kompilate.
- GLADE+/NED: laufen in `handover-2026-08-21-offene-atome.md` — NICHT
  anfassen (Sample-Budget zuerst).
- Chandra-Drift benannt: der Block trägt erg/cm2, CSC-Fluxb ist
  physikalisch erg/cm²/s — gehört zum Unit-Arm, Block-Label prüfen.

## O. Unit-Arme (Curation & Quellen)

Pending: F (Fahrenheit, CHPL-Lufttemperatur), μg/L (Chlorophyll,
CREST-Boje), mg/L (Sauerstoff, CREST-Boje) — die Felder existieren in
den Quellen, manifestieren erst mit dem convert_to_si-Arm.
Kurationsfrage: ein Live-`vectors`-Block (Horizons) in sources.φ —
dead_sources.φ:3090 deklariert Horizons als Compiler-Eingang, keine
Live-Quelle; die Entscheidung ist die Einheit.

## Gates

- Force-Gate je Lücke (Litmus); τ-Gate: ohne deklariertes τ kein Sample.
- Jede Lücke endet mit Register-Zeilen (sources.φ / blocked / dead) +
  ledger.φ im selben Commit.
- cargo check 0/0 für alles, was src/ berührt.
- Diese Datei nach dem Abschluss archivieren (Regel in AGENTS.md).

## Nicht anfassen

Source-Port-Pipeline (eigene Übergabe), offene-atome (Sample-Budget/
GLADE+/NED/2MASS/grüner Lauf), die Sonnen-Handovers, berkeley-wind,
Stern-/Asteroiden-Physik (die 44-B-Rekompilation läuft dort).
