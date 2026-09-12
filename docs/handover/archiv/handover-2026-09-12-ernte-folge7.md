<!--
  title: Handover — Ernte-Folge VII (Stand 2026-09-12)
  session: Ernte-Folge VII
  class: handover
  date: 2026-09-12
  sha256: 844f535d4c8ea4d68308d6db4af19f35ef441c8c3254cc2f9d59cba9d653fde3
  status: live
-->
# Handover — Ernte-Folge VII (2026-09-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Duty)

- igets.bin — Wächter rot gemessen: Run 34684040604 nach 360 min gecancelled
  (Compiler-Timeout — `igets_compiler` läuft an der 6-h-Grenze noch, killt als
  orphan; Download cache-resumable über `data/igetsftp.gfz.de`, save-always).
  Asset 404. Ursache ist die 360-min-Grenze (public-repo-Maximum), kein
  Download-404. Kein erneuter Blind-Dispatch; Fix offen: Laufzeit-Strategie
  (per-Jahr-Loop oder Scope-Reduktion).
- eso-harps — Fehlschlag gemessen: der Workflow trug kein `actions/checkout`
  (Cargo.toml fehlte → exit 127). Behoben (checkout + rust-setup), committet —
  aber Dispatch + Push warten: der Baum war beim Abschluss nicht ruhig (fremde
  uncommittete Arbeit). Nach dem Push dispatcht `eso-harps-rvcat-cdn`; Wächter:
  harps_rvcat.json auf Release `ssd.jpl.nasa.gov` (HTTP 200).

## Ernte

- Hi-net — `HINET_PASS` weiter absent (.secrets.local gemessen) — Operator.
- noaa-jpss — EULA-Operator-Akt ist durch: Granule-URL-Probe gemessen HTTP 200
  (EDL-Token, VNP02IMG .nc, HDF5-Magic) — keine 403/EULA mehr. Offen: die
  Compiler-Lease (noaa_jpss_compiler, NETLOC noaa-jpss.s3.amazonaws.com) —
  Register-Duty in phi/pipeline/catalog/noaa_nodd_disposition.φ.
- pioneer10_skyfreq.bin — pending (Tor 1, kein Membran-Konsument; 14-Feld-Serie
  ist kein Skalar-Oszillator) — Sitz phi/blocked_sources.φ.
- ESO tap_obs — pending (Tor 1, Discovery-Metadaten, Messung liegt in den FITS
  hinter access_url) — Sitz phi/blocked_sources.φ.

## Votable-TAP

- Compiler-Tranche (8 Kataloge) — pending, Tor 1 (kein gebauter Konsument je
  Katalog); Disposition vollständig in phi/blocked_sources.φ. Kein Bau-Punkt
  ohne Konsument.

## TAP-Klassifikation

- 7 ausstehend — weiter absent (sync-GET 2026-09-12: Basis 200, tap_schema-sync
  500 bzw. kein TAP-JSON); der Re-Probe-Takt trägt sie — kein Bau-Punkt.

## Abschluss

- Nur eigene Hunks committet (lokal); fremde uncommittete Arbeit im Baum blieb
  unberührt; Push + eso-harps-Dispatch warten, bis der Baum ruhig ist.
