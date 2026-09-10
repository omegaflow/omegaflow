<!--
  title: Handover — Ernte & Register (Stand 2026-09-11)
  class: handover
  date: 2026-09-11
  sha256: 541c26e97a24d82c833d8660e54d341dcd4dc8a743d64bdbfd0cc1fbb362233f
  status: live
-->
# Handover — Ernte & Register (2026-09-11)

Dedupliziert aus den archivierten Atom-Übergaben des 2026-09-10. Jede Zeile ist
eine Sitzungs-Arbeit; erledigt = trägt Git.

## Ernte (Harvest)

- SuperMAG Vollernte der 194 Stationen — merged-Pilot (März 2025) als
  CI-Dispatch; per-Station-Fallback, falls die CDN-2-GB-Grenze reißt.
- SuperMAG magstid Vollernte-Workflow — Station-Schleife über die 599 Stationen.
- noaa-nodd All-Stationen-Schleife — ghcn/gsod/isd über alle Stationen je
  Jahres-Fenster.
- Katalog-Lücken-Ernte — RAVE DR6, APOGEE/GALAH, HyperLEDA, TGSS ADR, VLASS,
  AMS-02, GLADE+.
- ESO-TAP — Ernte + Duty (kein Register-Eintrag).
- Discovery-Download-Workflow (Korpora).

## CDN-Manifestation (Duty)

- supermag — `gh workflow run supermag-cdn.yml` (Asset `supermag_2025-03.bin`).
- noaa ghcn/gsod/isd — erster Dispatch der drei Workflows (Pilot-Assets 404 gemessen).
- jup365 — kernel-flatten-Dispatch mit Wächter (bodies-Job grün + jup365-Release am CDN).
- `sources.φ`-Url für das merged-Asset `supermag_2025-03.bin` (eigene Linie beim Dispatch).

## Compiler-Lease (Register-Duty)

- 8 Copernicus-Lease — insitu-observations-* (ICOADS, surface-land, US-CRN,
  IGRA, WOUDC, GRUAN, CUON, GNSS-Delay); je Lease Manifestations-Duty via --ci-mode.
- 19 noaa-nodd-Lease (ledger.φ).
- 14 weitere Lease — lidar, dcdb, eri, gk2a, goes16, himawari8, jpss,
  cors-RINEX, ccor-FITS, nexrad, gdp-drifter, keo-papa, ocs-hydrodata, wod.

## Register

- bucket_litmus auf weitere Inventare (Copernicus u. a.).
- Step-5-Folge — destruktiver Schnitt (verifiziert).
- R2 — Archiv-Zählung (archive-root + lokales Backup) als Grundwahrheit in
  `number_audit.rs` verdrahten.
- docs-reference-verteilung — Bewegung + Referenz-Rewiring, Seeds → Survey-Heimat.
- Probe-Einheit-Autoableitung.
- Korpora-Verdikt cmr + dataone messen (CMR aggregiert Partner-Metadaten;
  dataone terms-Seite broken).

## Klassifikation (aus dem tap-Pass)

- Source-Drafts der 44 überlebenden TAP-Endpoints (field/force/τ je §8).
- http-Klassifikation der 67 (POST-only/CSV-only/Redirect, cefca-404-Familie,
  neocc 502, koa 404).
- Speisekammer-Fragen — aia2014, planck, eve, omni2, goes15, gebco.
- Proton-VPN-Recheck — arvo-registry (endgültiges dead), cadc.argus (TLS-Reset).

## Dead-Source-Nachlese

- tsunami.incois.gov.in past90days.json — Tsunami-Ereignis vs. Pegeldaten.
- services5.arcgis.com „parameter" — generischer Layer-Name.
- Verify: services.arcgis.com Current_lightning_view, services5 CA_Lightning —
  Spitzenstrom (Feld) vs. Strike-Punkte.
