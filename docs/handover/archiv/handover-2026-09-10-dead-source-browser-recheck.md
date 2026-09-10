<!--
  title: Handover — dead-sources Browser-Recheck (2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: f1ece36b3838652fe28ea29a17327af667b2d0d06967fa9a0ec724df1bad8767
  status: archived
  see-also: docs/SOURCE_PORT.md docs/concepts/kybernaut-native-methodology.md
-->
# Handover — dead-sources Browser-Recheck (2026-09-10)

Der Recheck ist getragen: `phi/dead_sources.φ` wurde im opencode-Browser
durchgemessen (nicht curl), die echten Toten per DDG/Google nach Nachfolgern
recherchiert, und die lebenden curl-Artefakte per Force-Gate umdisponiert.
Messung 2026-09-10.

## Disponiert

- **61 tote Hosts** (Browser-Fehlerseite): 57 tragen jetzt den
  Dienst-Nachfolger in der `note`, 3 ohne Nachfolger (`api.ebeard.org`,
  `dods.wh.gov`, `world.openagritechdata.org`), 1 Fehlverdict
  (`jsoc.stanford.edu` lebt via HTTP — HTTPS-Zertifikatfehler war das
  curl-Artefakt).
- **82 lebende Hosts** (curl-Artefakt — DNS/Timeout/Zertifikat/abgeschnittene
  URL): nicht-feldtragende → `decline <grund>` mit Note
  `Host lebt (Browser 2026-09-10), dead war curl-Artefakt.`; bereits
  integrierte Duplikate → `decline superseded-by-integrated`
  (`api.tidesandcurrents.noaa.gov` water_level, `argovis-api.colorado.edu`,
  `data.pmel.noaa.gov` pCO2, `erddap.aoml.noaa.gov` drifter-SST,
  `erddap.ifremer.fr` argo, `mast.stsci.edu` TESS, `geofon.gfz-potsdam.de`
  fdsnws).
- `ledger.φ` unberührt.

## Nächster Atom — Feld-Träger in die Port-Queue

Die lebenden Feld-Träger stehen noch als `dead` und sind der nächste
Port-Pass: je Quelle ein Eintrag in `queue/master.φ`, dann der `dead`-Eintrag
entfernt (Git trägt ihn). Liste:

- api.waterdata.usgs.gov — NWIS streamflow/gage-height/water-temp (hydro/thermal)
- aqs.epa.gov — pollutant concentration (diffusion)
- dap.oceannetworks.ca — salinity ERDDAP scalar (diffusion)
- data.neracoos.org — A01 met (air temp/wind/pressure) + A01 ocean (temp/salinity)
- erddap.gcoos.org — drifter SST/currents
- erddap.marine.ie — SWAN wave height + smartbay acoustic SPL
- waterservices.usgs.gov — NWIS dv discharge/water-level
- geodesy.unr.edu — IGS GNSS velocity (crustal deformation)
- impc.dlr.de — VTEC (ionospheric electron density, em)
- cdaweb.gsfc.nasa.gov — DSCOVR magnetometer (em)
- www.seis-insight.eu — InSight MQS Mars quakes (seismic)
- www.arpansa.gov.au — UV index (em)
- services-eu1.arcgis.com — seismicity catalogue (Mw+depth)
- utility.arcgis.com — lightning recent (em)
- www.ncei.noaa.gov — 53 hourly-precip Blöcke (diffusion/gravity)
- mars.nasa.gov — InSight weather ×2 (thermal/acoustic)
- www.ndbc.noaa.gov — stdmet 15006 (wind/wave/temp/pressure)

## PENDING (unbestimmt, vor Port klären)

- tsunami.incois.gov.in past90days.json — Tsunami-Ereignis vs. Pegeldaten
- services5.arcgis.com "parameter" — generischer Layer-Name

## Verify (Feld vs. punkt-nur)

- services.arcgis.com Current_lightning_view
- services5.arcgis.com CA_Lightning — Spitzenstrom (Feld) vs. reine Strike-Punkte
