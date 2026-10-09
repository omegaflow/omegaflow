<!--
  title: Survey — Freie Alternativen für redistributions-abgelehnte Quellen (2026-10-09)
  class: survey
  date: 2026-10-09
  sha256: eada6e3541c7575db78e41bea55e82ca4f284cdf738907856fce2ff1400273b4
  status: live
-->
# Survey — Freie Alternativen für redistributions-abgelehnte Quellen (2026-10-09)

**Anlass:** Operator-Frage — „wir haben ca. 25 Quellen von CDN genommen, weil deren
Lizenzen die Redistribution untersagt; gibt es freie Alternativen?" Gemessen trägt
`phi/declined_sources.φ` **41** `decline redistribution`-Blöcke (44 String-Treffer,
3 davon Note-Prosa). Erstmals systematisch mit `archive_search --all/--verdict`
durchsucht (ein `general`-Taucher, 2026-10-09).

**Das Finding ist eine Messung, kein Verdikt:** die Lizenz jedes Alternativ-Kandidaten
ist nur dort „gemessen", wo die Lizenzseite geprüft wurde. „ungemessen" heißt
`pending`, nie frei. Die Re-Admission (declined → sources) ist Mountains Verdikt.

## Sofort CDN-fähig (offene Lizenz **gemessen** + HTTP 2xx + maschinenlesbar)

| Deckt (decline) | Kandidat | Lizenz | HTTP | Format |
|---|---|---|---|---|
| Meteostat (1610, 2771), WeatherXM (3127), OGIMET (5539) | `https://api.open-meteo.com/v1/forecast` | **CC-BY 4.0** (open-meteo.com/en/terms) | 200 | JSON |
| MapTiler (593), Protected Planet (825) | `https://tile.openstreetmap.org/{z}/{x}/{y}.png` | **ODbL** (openstreetmap.org/copyright) | 206 | PNG |

## HTTP-gemessen, Lizenz `ungemessen` (Shortlist — nächster Schritt Lizenzseite)

US-Government / offene Knoten mit 2xx, Lizenz nicht am Quelltext gemessen:
`opensky-network.org` (403 blockiert), `api.carbonintensity.org.uk` (200),
`archive.routeviews.org` / `ris.ripe.net` (206/200), `tevcat.uchicago.edu` (200),
`intermagnet.org` (206), `icgem.gfz-potsdam.de` (200), `services.swpc.noaa.gov` (206),
`globalfloods.eu` / `grdc.bafg.de` (206), `dataspace.copernicus.eu` (200),
`ssd.jpl.nasa.gov` (206), `ndbc.noaa.gov` (200), `giro.uml.edu/didbase` (206),
`ds.iris.edu` (206), `www.goes-r.gov` (200), `data.sensor.community` (403),
`api.openaq.org` (401), `waterisotopesDB.org` (206), `data.mendeley.com` (200).

**Nächster Schritt:** je Kandidat `archive_search --jina <lizenz-url>` + `--verdict`
— dann steigt die zertifizierte Liste über zwei Einträge.

## Die 41 Blöcke (Quelle | Datenart | Grund | Alternative | Lizenz | HTTP)

| Quelle | Datenart | Alternative(n) gemessen | Lizenz | HTTP |
|---|---|---|---|---|
| almascience.org | ALMA TAP obscore | kein Kandidat | ungemessen | – |
| api.airplanes.live | ADS-B | opensky-network.org | ungemessen | 403 |
| api.blitzortung.org | Blitzortung | goes-r.gov (GLM) | ungemessen | 200 |
| api.electricitymap.org | CO₂-Intensität | api.carbonintensity.org.uk | ungemessen | 200 |
| api.maptiler.com | Map-Tiles | tile.openstreetmap.org | **ODbL** | 206 |
| api.protectedplanet.net | WDPA | protectedplanet.net · OSM | ODbL (OSM) | 200/206 |
| api.purpleair.com | PM2.5 | data.sensor.community · api.openaq.org | ungemessen | 403/401 |
| api.resourcewatch.org | WRI Geostore | data-api.globalforestwatch.org | ungemessen | 206 |
| api.sncf.com | SNCF Transit | transport.data.gouv.fr | ungemessen | 200 |
| api.sofarocean.com | Spotter-Wellen | ndbc.noaa.gov | ungemessen | 200 |
| api.z.ai (×2) | AI-Modell (kein Messwert) | keine | – | – |
| bgp.tools | BGP-Routing | archive.routeviews.org · ris.ripe.net | ungemessen | 206/200 |
| casdc.china-vo.org | LHAASO | tevcat.uchicago.edu | ungemessen | 200 |
| data.blitzortung.org | Blitzortung strikes | goes-r.gov (GLM) | ungemessen | 200 |
| data.lsst.cloud/api/tap | LSST TAP | kein Kandidat | ungemessen | – |
| dev.meteostat.net | Meteostat | api.open-meteo.com | **CC-BY 4.0** | 200 |
| developer.amentum.io | Geomag/Grav/Rad | intermagnet.org · icgem.gfz-potsdam.de · swpc.noaa.gov | ungemessen | 206/200/206 |
| dhm.gov.np (×2) | Nepal Hydrologie | globalfloods.eu · grdc.bafg.de | ungemessen | 206/206 |
| docs.sentinel-hub.com | Sentinel-Hub | dataspace.copernicus.eu | ungemessen | 200 |
| ftp.imcce.fr InPOP19a | Planeten-Ephemeriden | ssd.jpl.nasa.gov | ungemessen | 206 |
| ftp.imcce.fr NOE-4 | Satelliten-Ephemeriden | ssd.jpl.nasa.gov | ungemessen | 206 |
| gateway.api.globalfishingwatch.org | AIS Fischerei | aishub.net · dma.dk | ungemessen | 200/200 |
| meteostat.net/api | Meteostat | api.open-meteo.com | **CC-BY 4.0** | 200 |
| nucleus.iaea.org/wiser | GNIP Isotope | data.mendeley.com · waterisotopesDB.org | ungemessen | 200/206 |
| pradan.issdc.gov.in | Chandrayaan-2 L1 | earthdata.nasa.gov | ungemessen | 200 |
| pro.weatherxm.com | WeatherXM | api.open-meteo.com | **CC-BY 4.0** | 200 |
| services.sentinel-hub.com | Sentinel-Hub Catalog | dataspace.copernicus.eu | ungemessen | 200 |
| supermag.jhuapl.edu (×3) | Geomag Substorms/Indizes | intermagnet.org · swpc.noaa.gov | ungemessen | 206/206 |
| sws-data.sws.bom.gov.au | Space Weather (BoM) | services.swpc.noaa.gov | ungemessen | 206 |
| wwlln.net (×2) | WWLLN Blitz | goes-r.gov (GLM) | ungemessen | 200 |
| ctbto.org | CTBTO Hydroakustik | ds.iris.edu | ungemessen | 206 |
| dxpredictor.com | Ionosonde fof2 | giro.uml.edu/didbase | ungemessen | 206 |
| hamqsl.com/solarxml.php | Solar-Indizes | services.swpc.noaa.gov | ungemessen | 206 |
| ogimet.com | SYNOP Wetter | ncei.noaa.gov · api.open-meteo.com | CC-BY 4.0 (Open-Meteo) | 200/200 |
| reddit.com/dev/api | Social-Text (kein Messwert) | keine | – | – |
| Vaisala GLD360 | Blitzortung proprietär | goes-r.gov (GLM) | ungemessen | 200 |

**Grenze des Laufs:** nur 2 Alternativen sind lizenz-zertifiziert; der Rest ist
HTTP-gemessen mit `Lizenz ungemessen`. `frbcat.org`/Blinkverse und der Mariner-
Tracking-Subpfad blieben `pending`.
