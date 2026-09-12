<!--
  title: Handover — Ernte-Folge VIII (Stand 2026-09-12)
  session: Ernte-Folge VIII
  class: handover
  date: 2026-09-12
  sha256: 09b53451560ceb0824157468a7c1983ea9246d970e5065ee0c053ac8e06faa5b
  status: live
-->
# Handover — Ernte-Folge VIII (2026-09-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird erst, wenn der Baum ruhig ist.

## TAP-Klassifikation

- 3 ausstehend — DaCHS-Backend down (gemessen 2026-09-12): `dachs.fai.kz/tap`
  (HTTP 500, PostgreSQL connection refused localhost:5432),
  `vo.lmd.jussieu.fr/tap` (VOTable-Fehler, Backend down),
  `pithia.cbk.waw.pl/tap` (connection pool is closed). Der Re-Probe-Takt trägt
  sie — kein Bau-Punkt.

## CDN-Manifestation (Duty)

- igets.bin — Wächter rot gemessen: Run 34684040604 nach 360 min gecancelled
  (Compiler-Timeout — `igets_compiler` läuft an der 6-h-Grenze noch, killt als
  orphan; Download cache-resumable über `data/igetsftp.gfz.de`, save-always).
  Asset 404. Fix offen: Laufzeit-Strategie (per-Jahr-Loop oder Scope-Reduktion).
- eso-harps — Dispatch + Push warten: der Baum war beim Abschluss nicht ruhig.
  Nach dem Push dispatcht `eso-harps-rvcat-cdn`; Wächter: harps_rvcat.json auf
  Release `ssd.jpl.nasa.gov` (HTTP 200).

## Ernte

- Hi-net — `HINET_PASS` weiter absent (.secrets.local gemessen) — Operator.
- noaa-jpss — Compiler-Lease (`noaa_jpss_compiler`, NETLOC
  noaa-jpss.s3.amazonaws.com) offen: Register-Duty in
  `phi/pipeline/catalog/noaa_nodd_disposition.φ`.
- pioneer10_skyfreq.bin — pending (Tor 1, kein Membran-Konsument; 14-Feld-Serie
  ist kein Skalar-Oszillator) — Sitz `phi/blocked_sources.φ`.
- ESO tap_obs — pending (Tor 1, Discovery-Metadaten, Messung liegt in den FITS
  hinter access_url) — Sitz `phi/blocked_sources.φ`.

## Votable-TAP

- Compiler-Tranche (8 Kataloge) — pending, Tor 1 (kein gebauter Konsument je
  Katalog); Disposition vollständig in `phi/blocked_sources.φ`. Kein Bau-Punkt
  ohne Konsument.

## Abdeckung

- XRISM (`darts.isas.jaxa.jp`) und Fermi (`fermi.gsfc.nasa.gov`) gemessen live
  (HTTP 200), in keinem Register — Source-Draft offen.
- NuSTAR/NICER/IXPE via HEASARC und der MAXI-Host gemessen live; HEASARC-Xamin-TAP
  tot (`dead_sources.φ`). Kein Draft ohne verifizierte Daten-API.

## Katalog-Linse

- Probe-Kette: 30 Kandidaten → 3 Survivors, alle schon kanonisch — null neue
  Quellen; der Katalog-Kandidaten-Pool ist erschöpft.
- 7 Unter-Floor-Kandidaten mit Force-Signal (terrapulse 1 em, archeology_gaps
  3 em, grind_arcgis 3 em/thermal) — Re-Weigh offen.
- `b2find_science_catalogs` (167.037 Zeilen) — `source_scanner`-Timeout bei 850 s,
  kein Output; Nachlauf offen.

## Ledger

- Der 75a2bbe-Stand (381 Blöcke) bleibt bewusst untrackt (`0ed6c02`, „gitignored,
  local-only"); keine Negation in `.gitignore`. Der Arbeitsbaum-Ledger ist das
  lebende Register, die 381 Blöcke sind Historie.

## Abschluss

- Nur eigene Hunks committet (lokal); fremde uncommittete Arbeit (`phi/sources.φ`
  22 Insertions, `src/`, `tools/`, `static/`, Handover bau11/forschung-folge6)
  blieb unberührt; Push wartet, bis der Baum ruhig ist.
