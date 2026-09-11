<!--
  title: Handover — Ernte & Register (Stand 2026-09-11)
  class: handover
  date: 2026-09-11
  sha256: 222158883f21d24150c780fd6c7e4a1b9deff2072537c6ca7d7ff334573a8bcb
  status: live
-->
# Handover — Ernte & Register (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Ernte (Harvest)

- SuperMAG per-Station-Fallback — die merged-Vollernte (203 Stationen, 3,23 GB)
  reißt die GitHub-2-GB-Asset-Grenze (HTTP 422 gemessen); die Rückfallebene ist
  der gebaute `supermag-magstid-cdn.yml` (599 Stationen, `--list-stations`).
- SuperMAG magstid Vollernte — Dispatch `supermag-magstid-cdn.yml`.
- noaa-nodd All-Stationen — Dispatch `noaa-ghcn/gsod/isd-allstations-cdn.yml`
  (gebaut; Stationen gemessen: GHCN 132 501, GSOD 11 656, ISD 12 815).
- Katalog-Lücken-Ernte — klassifiziert (RAVE DR6, APOGEE DR17, TGSS ADR1
  akzeptiert; HyperLEDA dead; VLASS parser-def; GLADE+ decline; AMS-02
  akzeptiert, Daten-Endpoint pending). Verdicts in
  `phi/pipeline/research/agent_output/catalog_gaps_2026-09-11.φ` — Merge in die
  Register ist der nächste Schnitt.
- ESO-TAP — Ernte + Duty (kein Register-Eintrag).
- Discovery-Download-Workflow (Korpora).
- Pioneer-10-ATDF-Restbestand — 238 Dateien / 77 Tage ernten (SPDF
  `ATDF_Data-Files_CMarkwardt_Readable`).
- IGETS/GGP supraleitende Gravimeter — Compiler gebaut (`igets_compiler`,
  IGT1, `src/archivar/geo.rs`; Arm in `src/archivar/extract.rs`; Quellblock in
  `phi/sources.φ`). SFTP `igetsftp.gfz.de` (Loginname
  `johannes.tyroller_at_proton.me`, `IGETS_USER`/`IGETS_PASS` in
  `.secrets.local`), Zugang gemessen 2026-09-11. Produkt: **Level 2** `g_fil`
  (kalibrierte Gravitation in nm/s²; Level 1 trägt in neuen Dateien keinen
  Gravitations-Kalibrierfaktor — die älteren tragen `Gravity Cal (nm.s-2/V)`,
  die 2011er `gravity(nm/s**2)`, beide gemessen) auf 60-min-Mittel
  (`--bucket-min 60`); volle Level-1-Rohdaten sind ~50 GB (Wettzell allein
  1,5 GB). Pilot Membach Level 2: 6 Dateien → 4234 Records, roundtrip grün.
  Vollernte offen: CI-Dispatch `igets-cdn.yml` (Asset `igets.bin`).
  Katalog `phi/pipeline/catalog/gfz_igets_catalog.φ`.

## CDN-Manifestation (Duty)

- jup365 — kernel-flatten bodies-Job bricht 3× ab (103/103 Downloads, dann
  „canceled" vor dem Flatten; kein jup365-Asset am CDN). Wächter offen.
- igets — `gh workflow run igets-cdn.yml` (Asset `igets.bin`); braucht
  `IGETS_USER`/`IGETS_PASS` als Repo-Secrets.

## Compiler-Lease (Register-Duty)

- 6 Copernicus-Lease offen — surface-land, US-CRN, IGRA, WOUDC, GRUAN, GNSS:
  5 licence-`blocked account` (Reg `?tab=download#manage-licences`), US-CRN
  `pending` (Form geschlossen). ICOADS + CUON gebaut (`copernicus_*_compiler`).
- 11 noaa-nodd-Lease parser-def — lidar (LAS/GeoTIFF), eri (GeoTIFF), gk2a
  (Geostationär-Nav), goes16 (HDF5-v2), himawari8 (bzip2/HSD), jpss (Grain),
  cors-RINEX (RINEX-2), nexrad (bzip2), gdp-drifter (14 GB netCDF-4), wod
  (netCDF-4 v2), hydrodata (GeoTIFF/GeoPackage). Verdicts in
  `phi/pipeline/research/agent_output/lease_noaa_{a,b,c}_2026-09-11.φ` — Merge
  in `blocked_sources.φ`.

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

- TAP-Klassifikation gemessen (117 Blöcke in
  `phi/pipeline/research/agent_output/tap_klassifikation_2026-09-11.φ`):
  13 Source-Drafts akzeptiert, 31 registry-decline, 39 dead (cefca-404-Familie,
  JVO-Skynode×25, neocc 502, koa 404), 6 parser-def (ALMA×3, Euclid, ESO×2),
  26 live-with-query, 2 decline. Merge in die Register ist der nächste Schnitt.
- Speisekammer-Fragen — aia2014, planck, eve, omni2, goes15, gebco.
- Proton-VPN-Recheck — arvo-registry (endgültiges dead), cadc.argus (TLS-Reset).

## Dead-Source-Nachlese

- 3 Verdicts gemessen (alle decline): tsunami.incois.gov.in (Event-Katalog),
  services5.arcgis.com „parameter" (Tsunami-Gefährdungs-Zonenkarte),
  Current_lightning_view/CA_Lightning (generisch, kein Spitzenstrom-Feld).
  In `phi/pipeline/research/agent_output/deadsource_2026-09-11.φ` — Merge in
  `dead_sources.φ`.
