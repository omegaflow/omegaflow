<!--
  title: Handover — Ernte-Folge 13 (Stand 2026-09-13)
  session: Ernte-Folge 13
  class: handover
  date: 2026-09-13
  sha256: 6be102ac371023fe28e7a9cb7ef5e0eefd2a8e7436c3a60cd65fc621074f18e4
  status: live
-->
# Handover — Ernte-Folge 13 (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Duty)

- hinet.bin — Wächter: Run 34746164903 (2026-09-13T07:49Z) in Flug; der
  Status-Token-Fix wirkt (Build/Auth/Stationswahl grün), der Patience-Fix
  (cont-Poll 8×240s) trägt die gemessene Upstream-Varianz (Prep einmal 40–90s,
  einmal >5,5min). Erfolg: `hinet.bin` im Release `hinetwww11.bosai.go.jp`.

## Weberin — offene Fäden (Ernte)

- Phobos/Deimos-Zweitlinie — NOE-4-2020.bsp (ftp.imcce.fr/pub/ephem/satel/NOE/MARS/2020/,
  244 MB, NAIF 401/402) live; die Planeten tragen die INPOP/EPM-Zweitlinie schon.
  Kein NOE-Compiler/Format → pending (Bau).
- WWLLN — Nachfolger-Host `wwlln.net/climate/` (anonym, monthly `WWLLN_th_YYYY.nc.zip`,
  0,05°-Gitter, netCDF); Realtime-Roh weiter Mitgliedschaft. Compiler+CDN-Duty
  pending (wie iss_lis).
- NRS-Tiefen — lat/lon der NRS02–10/12/13 stehen in `phi/nrs_stations.φ`; die
  Wasser-/AUH-Tiefe aus der Netz-Tabelle (NOAA PMEL ONSN) fehlt → pending
  (Tiefe in den Register).
- INPOP25c-Asteroidenmassen — VizieR `J/A+A/705/A189` tablea1 (196 Asteroiden, kg)
  live; natives .header unpubliziert (pending); `phi/pipeline/catalog/asteroid_gm_inpop25c.φ`
  trägt nur 4/196 → pending (Voll-Ernte).
- MPC — `mpcorb_distant.bin` registriert (TNO-Teilmenge); Vollkatalog
  (Distant-Filter lösen) + Observations-Linie (MPCAT-OBS 404, `get-obs`-JSON
  per-Designation) → pending.
- Broker/GW-Positionen — ANTARES-Lichtkurve (`/v1/loci/{id}`), Fink-conesearch
  ohne `columns`, GW-bayestar (graceDB, anonym) gemessen; der Compiler trägt nur
  die Richtung → pending (Positions-Ernte).

## Offene Pendings

- Eclipse-2024-Kanon-Punkt — getragen im Paper (Δ 793,3 km auf allen fünf Linien,
  Kanon-vs-Algorithmus-Punktdefinition, keine Ephemeriden-Drift).

## Abschluss

- Baum beim Sessionsstart: HEAD == origin/main == d8a17f68 (nicht 9395452 wie in
  Folge 12 notiert — der Baum lief weiter), mit fremder uncommitteter Arbeit.
  Fremde uncommittete Arbeit beim Schreiben (bau18-Linie, disjunkt von dieser
  Session): `docs/handover/handover-2026-09-13-bau18.md`,
  `docs/handover/archiv/handover-2026-09-13-bau17.md` (Rename, unstaged),
  `docs/handover/handover-2026-09-12-entscheid-folge3.md`,
  `docs/paper/uranus-rift-ephemerides.md`, `phi/sources.φ`,
  `phi/blocked_sources.φ`, `src/lib.rs`, `src/archivar/{mod,netcdf,odf,zeuge}.rs`,
  `tools/harvest/src/bin/galileo_odf_compiler.rs`,
  `tools/utils/src/bin/{archive_search,netcdf_reader}.rs`,
  `.github/workflows/{copernicus-dem-90m-cdn,openneuro-cdn}.yml`,
  `src/archivar/las.rs`, `tools/harvest/src/bin/copernicus_dem_90m_compiler.rs`,
  `tools/utils/src/bin/archive_search/`, `tools/utils/src/bin/las_reader.rs` —
  unberührt. Gepusht wird erst mit dem Wort.
