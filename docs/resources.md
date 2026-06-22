# Resources

## 1. AI & Development Infrastructure

| Resource | URL |
|---|---|
| Z.ai Coding Plan | https://z.ai/subscribe |
| Z.ai API Coding Endpoint (Base URL) | `https://api.z.ai/api/coding/paas/v4` |
| Z.ai API General Endpoint (Base URL) | `https://api.z.ai/api/paas/v4` |
| Z.ai API Docs (Core Parameters) | https://docs.z.ai/guides/overview/concept-param |
| Z.ai API Docs (Thinking Mode) | https://docs.z.ai/guides/capabilities/thinking-mode |

## 2. Hardware & omegaflow sense Module (~25 EUR)

| Component | Purpose | Search URL |
|---|---|---|
| ESP32-S3 DevKit (N8R2/N16R8) | Central, Native USB | https://www.aliexpress.com/wholesale?SearchText=ESP32-S3+DevKitC+Native+USB |
| LM358 amplifier module | Telluric current preamplifier | https://www.aliexpress.com/wholesale?SearchText=LM358+amplifier+module |
| Pure copper plates 10x10mm | Telluric electrodes for garden | https://www.aliexpress.com/wholesale?SearchText=pure+copper+plate+10x10mm |
| OPT101 / PT101 sensor | Biophotons (light of living cells) | https://www.aliexpress.com/wholesale?SearchText=OPT101+sensor |
| GL5528 LDR | Civilization flicker (50/60Hz power grid) | https://www.aliexpress.com/wholesale?SearchText=GL5528+LDR |
| PMS5003 sensor | Air interference (PM2.5 dust) | https://www.aliexpress.com/wholesale?SearchText=PMS5003+sensor |
| CJMCU-680 / BME680 | VOC / Odor (Chemical air) | https://www.aliexpress.com/wholesale?SearchText=CJMCU+680+BME680 |
| Dupont cables + Breadboard | Nervous system | https://www.aliexpress.com/wholesale?SearchText=breadboard+jumper+wires+kit+120 |

**Total cost: ~25.20 EUR**

## 3. Tools & Mathematics

| Tool | Purpose | URL |
|---|---|---|
| Wokwi | ESP32 / Arduino simulator | https://wokwi.com/ |
| The Pinouts Book | Microcontroller pin reference | https://pinouts.org/ |
| Datasheet4U | Raw sensor datasheets | https://www.datasheet4u.com/ |
| Fourier Transform Guide | FFT implementation for browser | https://www.jezzamon.com/fourier/ |
| WebGL Fluid Experiment | Point cloud rendering reference | https://www.paveldogreat.com/WebGL-Fluid-Simulation/ |
| WolframAlpha | Certainty formula verification | https://www.wolframalpha.com/ |

## 4. Live APIs — Earth & Atmosphere

| Source | What | Endpoint |
|---|---|---|
| Open-Meteo | Weather, soil, elevation | `https://api.open-meteo.com/v1/forecast?...` |
| Open-Meteo Elevation | On-demand height | `https://api.open-meteo.com/v1/elevation?latitude=X&longitude=Y` |
| USGS Earthquakes | Real-time earthquakes | `https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/all_hour.geojson` |
| NOAA CO-OPS | Tides & currents | `https://api.tidesandcurrents.noaa.gov/api/prod/` |
| NOAA SWPC | Space weather / solar wind | `https://services.swpc.noaa.gov/json/` |
| Blitzortung.org | Real-time lightning | `https://www.blitzortung.org/en/` (WebSocket: `wss://ws.blitzortung.org/...`) |
| IRIS FDSN | Global micro-seismics | `https://service.iris.edu/fdsnws/` |
| OpenSky Network | Global aviation | `https://opensky-network.org/apidoc/` |
| Marine Cadastre | Global shipping (AIS) | `https://marinecadastre.gov/ais/` |

## 5. Live APIs — Biosphere

| Source | What | Endpoint |
|---|---|---|
| iNaturalist | Local animal/plant observations | `https://api.inaturalist.org/v1/observations?lat=X&lng=Y&radius=1` |
| Movebank | Live animal tracking | `https://www.movebank.org/movebank/service/direct-read?...` |
| NASA POWER | NDVI / Vegetation | `https://power.larc.nasa.gov/api/temporal/daily/point?...` |
| GBIF | Global biodiversity | `https://www.gbif.org/developer/summary` |
| NASA Earthdata | SMAP soil temperature/moisture | `https://earthdata.nasa.gov/` |

## 6. Live APIs — Cosmic & Planetary

| Source | What | Endpoint |
|---|---|---|
| JPL Horizons | Planetary positions | `https://ssd.jpl.nasa.gov/api/horizons.api` |
| NMDB | Cosmic radiation (neutron monitors) | `http://www.nmdb.eu/nest/` |
| GCN (NASA) | LIGO/Virgo gravitational waves & IceCube neutrinos | `https://gcn.nasa.gov/circulators` |
| NASA LAMBDA | Planck CMB (echo of the Big Bang) | `https://lambda.gsfc.nasa.gov/` |
| Space Observing System | Schumann resonance | `http://sosrff.tsu.ru/?page_id=7` |
| INTERMAGNET | Raw magnetometer data | `https://intermagnet.github.io/` |
| Open-Meteo | Atmospheric refraction (weather data) | `https://api.open-meteo.com/v1/forecast?...` |

## 7. Deep Research APIs

| Source | What | Endpoint |
|---|---|---|
| RIPE RIS Live | BGP routing (nervous system of the internet) | `https://ris-live.ripe.net/` |
| Folding@Home | Protein folding | `https://foldingathome.org/` |
| Overpass API | OpenStreetMap physical structures | `https://wiki.openstreetmap.org/wiki/Overpass_API` |
| Wikipedia GeoSearch | Human memory | `https://en.wikipedia.org/w/api.php?action=query&list=geosearch&gscoord=X|Y&gsradius=10000` |
| Nextstrain | Pathogen evolution | `https://nextstrain.org/` |
| Ocearch | Marine life GPS tracking | `https://www.ocearch.org/tracker/` |

## 8. Global LiDAR & Topography Sources

| Source | What | URL |
|---|---|---|
| NASA Earthdata Search | ICESat-2 & GEDI | `https://search.earthdata.nasa.gov/search` |
| OpenAltimetry | Visual ICESat-2 selection | `https://openaltimetry.org/` |
| OpenTopography | REST API for point clouds | `https://opentopography.org/` / `https://opentopography.org/developers` |
| USGS 3DEP | LiDAR USA | `https://www.usgs.gov/3d-elevation-program` |
| UK Environment Agency | LiDAR UK | `https://environment.data.gov.uk/DefraDataDownload/?Mode=survey` |
| LOLA | Moon LiDAR | `https://ode.rsl.wustl.edu/moon/index.aspx` |
| MOLA | Mars LiDAR | `https://pds-geosciences.wustl.edu/missions/mgs/megdr.html` |
