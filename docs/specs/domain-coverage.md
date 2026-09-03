<!--
  title: domain-coverage
  class: concept
  sha256: 14298c8d2f83e9f1214d60708c3e0793700263afc4c5a163c5e9a5fd9b0dbb45
-->
  # HOST                                   SRC PROVIDER TOTALS                                 OUR USE                           FORCES          
================================================================================================================================================================
  1 github.com                             232 515 Release-Assets (v1.0 + live-data)               1025 fields                              diffusion,em,g
  2 api.worldbank.org                      228 29,544 indicators                                   3 indicators, 1 field                  diffusion,em
  3 imag-data.bgs.ac.uk                    162 ~130 INTERMAGNET observatories                       4 fields                                 em,gravity
  4 api.open-meteo.com                     120 8 API endpoints, ~50 measurement parameters                   163 parameters, 95 fields                 acoustic,advec
  5 gea.esac.esa.int                       108 GAIA DR3 (1.8 billion stars, cmap CDN + TAP)            26 fields                                em,gravity
  6 services.swpc.noaa.gov                  84 ~100 SWPC products                                   60 fields                                advective,diff
  7 api.tidesandcurrents.noaa.gov           80 3,499 CO-OPS stations                               116 stations, 21 fields                 acoustic,advec
  8 tapvizier.cds.unistra.fr                83 ~20,000 CDS-VizieR tables (partly CDN)             39 tables, 19 fields                   em,gravity,sei
  9 heasarc.gsfc.nasa.gov                   51 ~600 NASA-HEASARC tables                           13 fields                                diffusion,em
 10 api.adsb.lol                             4 16,284+ aircraft (real time, keyless)                20 fields                                em
 11 earthquake.usgs.gov                     48 USGS FDSN Event Service (global)                     43 fields                                acoustic,em,se
 12 api.gbif.org                            42 2.2 billion GBIF occurrences (GBIF workflow CDN WIP)    54 fields                                acoustic,diffu
 13 www.ndbc.noaa.gov                       39 1,351 NDBC stations → 364 live (+333 new)          200 fields                               acoustic,diffu
 14 www.ncei.noaa.gov                       38 ~10,000 paleoclimate datasets + OISST/CFSR             34 fields                                acoustic,diffu
 15 nrt.cmems-du.eu                          0 DOMAIN RETIRED (all 13 → NOAA/EMODnet)              —                                       —
 16 pdgapi.lbl.gov                          38 ~300 elementary particles                               38 sources                               em
 17 irsa.ipac.caltech.edu                   34 ~100 IRSA/IPAC tables                              4 fields                                 diffusion,em,g
 18 services6.arcgis.com                    32 1k features                                          3 fields                                 acoustic,em
 19 services9.arcgis.com                    28 1k features                                          104 fields                               advective,diff
 20 raw.githubusercontent.com               27 27 source URLs (no total derivable)          35 fields                                em,gravity,sei
 21 ssd-api.jpl.nasa.gov                    26 26 source URLs (no total derivable)          31 field                                acoustic,em,gr
 22 waterservices.usgs.gov                  21 7 keys                                               8 fields                                 acoustic,diffu
 23 gml.noaa.gov                            21 ~30 NOAA-GML feeds                                   23 fields                                diffusion,em
 24 aeronet.gsfc.nasa.gov                   17 ~500 AERONET stations (7-wavelength spectrum)     1 field                                 diffusion,em
 25 eutils.ncbi.nlm.nih.gov                 18 ~40 NCBI databases                                 18 sources                               em,gravity
 26 api.inaturalist.org                     17 1,420,180 taxa, 200M+ obs.                           1 station, 4 fields                    diffusion,em
 27 inspirehep.net                          17 2 keys                                               13 fields                                em
 28 pegelonline.wsv.de                       8 786 gauges (V2 API fix: single measurement + list)       5 fields                                 advective,gravity
 29 aviationweather.gov                     13 ~9,000 ICAO airports                                 36 fields                                acoustic,em
 30 erddap.emodnet-physics.eu                7 CMEMS replacement (SLA, eddy, oxygen) + 676 tabledap      13 fields                                diffusion,gravity
 31 exoplanetarchive.ipac.caltech.edu       13 ~5,500 exoplanets                                   15 fields                                em,gravity
 32 api.obis.org                            10 ~150,000 marine species                                3 stations, 8 fields                    diffusion,em
 33 services.arcgis.com                      9 2k features                                          21 field                                advective,diff
 34 mast.stsci.edu                           9 ~20 MAST tables (JWST/HST/TESS/Kepler)             8 fields                                 em
 35 service.earthscope.org                   8 67,962 seismic stations (global, GeoCSV)          12 fields                                acoustic,em,se
 36 stationview.raspberryshake.org           2 2,775 stations (API path fix)                       2 fields                                 seismic-surface
 37 api.waqi.info                            0 WORKFLOW→CDN (live data, no key in the server)         —                                       —
 38 celestrak.org                            0 CDN (v1.0, GeoBlock bypassed)                        —                                       —
 39 query1.finance.yahoo.com                 0 CDN (v1.0, 429 bypassed)                             —                                       —
 40 export.arxiv.org                         0 CDN (v1.0, 500 bypassed)                             —                                       —
 41 api.weather.gov                          8 254 NWS Gridpoints                                   35 fields                                acoustic,diffu
 42 tle.ivanstanojevic.me                    7 212 TLE satellites                                   2 fields                                 em
 43 api.crossref.org                         7 4 keys                                               7 sources                                em
 44 noaadata.apps.nsidc.org                  7 ~100 NSIDC cryosphere datasets                      4 fields                                 em
 45 cmr.earthdata.nasa.gov                   7 7 source URLs (no total derivable)           7 fields                                 diffusion,em,t
 46 volcanoes.usgs.gov                       6 5 keys                                               5 fields                                 em,seismic-sur
 47 gtnp.arcticportal.org                    6 not reachable                                     6 sources                                diffusion,ther
 48 services3.arcgis.com                     5 5 source URLs (no total derivable)           3 fields                                 em,gravity,sei
 49 gdacs.org                                5 GDACS disasters (324)                             1 field                                 diffusion,em,s
 50 gracedb.ligo.org                         5 ~100 gravitational wave events                       9 fields                                 em,gravity
 51 eonet.gsfc.nasa.gov                      5 EONET Events (200)                                   8 fields                                 em,seismic-bod
 52 api.coral.tsr.lol                        5 9 keys                                               1 field                                 em,thermal
 53 lasp.colorado.edu                        5 not reachable                                     5 sources                                em
 54 power.larc.nasa.gov                      5 not reachable                                     5 parameters, 1 field                    em,thermal
 55 en.wikipedia.org                         5 6.8 million articles                                     1 field                                 em
 56 www.cpc.ncep.noaa.gov                    5 918 lines                                            7 fields                                 em,thermal
 57 ghoapi.azureedge.net                     5 GHO health indicators (2.5k)                   2 fields                                 diffusion,em
 58 webservices.volcano.si.edu               5 not reachable                                     5 sources                                seismic-surfac
 59 rest.isric.org                           5 not reachable                                     3 fields                                 diffusion,em
 60 api.neotomadb.org                        5 78 lines                                             5 sources                                diffusion,em
 61 skyserver.sdss.org                       5 ~500 million SDSS DR18 objects                           3 fields                                 em
 62 www.sidc.be                              5 SIDC sunspots (76k)                             4 fields                                 em
 63 api.adsb.lol                             4 OpenSky replacement (keyless, 16k+ aircraft real time)    1 field                                 em
 64 www.seismicportal.eu                     4 not reachable                                     12 fields                                em,seismic-bod
 65 api.geonet.org.nz                        4 GeoNet NZ earthquakes (100/query)                       11 field                                em,seismic-bod
 66 services2.arcgis.com                     4 2k features                                          9 fields                                 advective,diff
 67 cdaweb.gsfc.nasa.gov                     4 not reachable                                     14 parameters                             advective,em,g
 68 seismic-api.science.unimelb.edu.au       4 not reachable                                     15 fields                                em
 69 hub.docker.com                           4 26 keys                                              4 fields                                 em
 70 data.pmel.noaa.gov                       4 not reachable                                     4 fields                                 diffusion,em
 67 ec.europa.eu                             4 10 keys                                              4 sources                                em
 68 www.ebi.ac.uk                            4 3 keys                                               1 field                                 diffusion,em
 69 ned.ipac.caltech.edu                     4 NED/IPAC galaxies (500/query)                        3 fields                                 em,gravity
 70 api.fda.gov                              4 2 keys                                               4 sources                                em
 71 woudc.org                                4 9 lines                                              1 field                                 diffusion
 72 opendata.cern.ch                         4 3 keys                                               1 field                                 em
 73 api.opentopodata.org                     4 not reachable                                     5 fields                                 em
 74 api.github.com                           3 3 source URLs (no total derivable)           3 fields                                 em
 75 api.wolfx.jp                             3 24 keys                                              3 sources                                em
 76 webservices.ingv.it                      3 not reachable                                     4 fields                                 seismic-body,s
 77 gcn.nasa.gov                             3 119 lines                                            1 field                                 em
 78 www.minorplanetcenter.net                3 32 lines                                             3 sources                                gravity
 79 www.nhc.noaa.gov                         3 3 source URLs (no total derivable)           6 fields                                 em,thermal
 80 cddis.nasa.gov                           3 196 lines                                            3 sources                                em,gravity
 81 bhuvan-app1.nrsc.gov.in                  3 557 lines                                            3 sources                                diffusion,seis
 82 coastwatch.pfeg.noaa.gov                 3 not reachable                                     4 fields                                 em,thermal
 83 hpiers.obspm.fr                          3 267 lines                                            6 fields                                 em
 84 v4.boldsystems.org                       3 494 lines                                            1 field                                 diffusion
 85 gist.githubusercontent.com               3 15 features                                          3 fields                                 em
 86 ssd.jpl.nasa.gov                         3 78 solar system bodies                               3 sources                                gravity
 87 zenodo.org                               3 7k lines                                             1 field                                 diffusion,grav
 88 musicbrainz.org                          3 2.9 million MusicBrainz entries                        3 sources                                em
 89 www.ngdc.noaa.gov                        3 900 totalItems                                       2 fields                                 em,seismic-bod
 90 ws.pangaea.de                            3 3 source URLs (no total derivable)           1 field                                 diffusion
 91 api.wheretheiss.at                       2 13 keys                                              2 sources                                em
 92 stat.ripe.net                            2 15 keys                                              1 field                                 em
 93 transport.opendata.ch                    2 2 keys                                               1 field                                 em
 94 services1.arcgis.com                     2 150 features                                         2 sources                                seismic-body,s
 95 services5.arcgis.com                     2 2k features                                          5 fields                                 acoustic,em
 96 api.energy-charts.info                   2 2 source URLs (no total derivable)           2 sources                                em
 97 api.carbonintensity.org.uk               2 2 source URLs (no total derivable)           2 sources                                em,thermal
 98 www.tsunami.gov                          2 2 source URLs (no total derivable)           1 field                                 gravity
 99 kp.gfz.de                                2 not reachable                                     2 sources                                em
100 geofon.gfz-potsdam.de                    2 101 lines                                            7 fields                                 seismic-body
101 firms.modaps.eosdis.nasa.gov             2 FIRMS fire data (NASA)                             7 fields                                 thermal
102 epic.gsfc.nasa.gov                       2 2 source URLs (no total derivable)           8 fields                                 em
103 www.imis.bfs.de                          2 1,679 IMIS radiation measurement stations                    7 fields                                 em
104 simbad.cds.unistra.fr                    2 not reachable                                     4 fields                                 em
105 geofon.gfz.de                            2 not reachable                                     7 fields                                 em
106 meta.icos-cp.eu                          2 2 keys                                               1 field                                 diffusion
107 erddap.ifremer.fr                        2 not reachable                                     2 fields                                 diffusion,ther
108 gong2.nso.edu                            2 not reachable                                     1 field                                 em
109 openlittermap.com                        2 32 lines                                             1 field                                 diffusion
110 earth-search.aws.element84.com           2 Earth-Search Sentinel-2 (10/query)                   2 sources                                em
111 api.woudc.org                            2 477 features                                         6 fields                                 em
112 clinicaltrials.gov                       2 3 keys                                               2 sources                                em
113 api.openalex.org                         2 3 keys                                               2 fields                                 em
114 www.orfeus-eu.org                        2 1k lines                                             3 fields                                 seismic-body
115 hydro1.gesdisc.eosdis.nasa.gov           2 45 lines                                             3 fields                                 diffusion,ther
116 data.humdata.org                         2 3 keys                                               4 fields                                 diffusion,em
117 www.isc.ac.uk                            2 20 lines                                             2 sources                                seismic-body
118 epqs.nationalmap.gov                     2 not reachable                                     1 station                              em,seismic-sur
119 g6goyz4w56.execute-api.us-west-2.ama     2 2 source URLs (no total derivable)           1 field                                 em
120 oderest.rsl.wustl.edu                    2 2 source URLs (no total derivable)           1 field                                 em
121 www.physics.mcgill.ca                    2 32 lines                                             2 sources                                em
122 data.rcsb.org                            2 31 keys                                              2 sources                                em
123 climate-api.open-meteo.com               2 9 keys                                               4 parameters, 1 field                    em,thermal
124 openlibrary.org                          2 650 lines                                            2 sources                                em
125 pubchem.ncbi.nlm.nih.gov                 2 not reachable                                     2 fields                                 em
126 rapid.ac.uk                              2 196 lines                                            2 sources                                diffusion
127 api.stackexchange.com                    2 24.1 million StackExchange posts                        1 field                                 em
128 macrostrat.org                           2 2 source URLs (no total derivable)           1 station, 1 field                    em
129 qrng.anu.edu.au                          1 4 keys                                               1 source                                em
130 api.blitzortung.org                      1 447 lines                                            3 fields                                 em
131 www.inex.ie                              1 8 keys                                               1 station                              em
132 api.irail.be                             1 not reachable                                     1 field                                 em
133 api.irishrail.ie                         1 2 lines                                              1 field                                 em
134 www.wienerlinien.at                      1 2 keys                                               1 field                                 em
135 data.blitzortung.org                     1 11 lines                                             1 station, 1 field                    acoustic
136 api.drand.sh                             1 4 keys                                               1 source                                em
137 irsc.ut.ac.ir                            1 not reachable                                     1 source                                seismic-body
138 api.p2pquake.net                         1 1 source URLs (no total derivable)           1 field                                 seismic-body
139 beacon.nist.gov                          1 1 source URLs (no total derivable)           1 source                                em
140 service.scedc.caltech.edu                1 not reachable                                     1 source                                seismic-body
141 earthquake.tmd.go.th                     1 59 lines                                             1 field                                 seismic-body
142 api.rainviewer.com                       1 5 keys                                               1 source                                acoustic
143 reg.bom.gov.au                           1 1 source URLs (no total derivable)           1 field                                 diffusion
144 www.emsc-csem.org                        1 not reachable                                     1 source                                seismic-body
145 www.pegelonline.wsv.de                   1 Pegelonline 10.9k stations                          2 fields                                 gravity
146 environment.data.gov.uk                  1 355 items                                            2 fields                                 em
147 www.ionosonde.iap-kborn.de               1 114 lines                                            1 source                                em
148 tadas.afad.gov.tr                        1 29 lines                                             1 field                                 seismic-body
149 api.weather.bom.gov.au                   1 not reachable                                     4 fields                                 em
150 dataserver-coids.inpe.br                 1 498 lines                                            7 fields                                 em
151 bmkg-restapi.vercel.app                  1 15 data                                              5 fields                                 em
152 www.ceic.ac.cn                           1 304 lines                                            1 field                                 seismic-surfac
153 www.jma.go.jp                            1 1 source URLs (no total derivable)           1 source                                seismic-surfac
154 api.waterdata.usgs.gov                   1 not reachable                                     1 source                                advective
155 impc.dlr.de                              1 2 keys                                               1 source                                em
156 almascience.org                          1 not reachable                                     2 fields                                 em
157 www.amsmeteors.org                       1 284 lines                                            1 field                                 gravity
158 argovis-api.colorado.edu                 1 991 lines                                            6 fields                                 diffusion
159 www.hamqsl.com                           1 43 lines                                             1 source                                em
160 api.sunrise-sunset.org                   1 3 keys                                               5 fields                                 em
161 disease.sh                               1 21 keys                                              1 source                                em
162 api.exchangerate-api.com                 1 7 keys                                               1 source                                em
163 open.er-api.com                          1 11 keys                                              1 source                                em
164 data.gdeltproject.org                    1 GDELT news (17.4k)                                   1 field                                 em
165 www.govtrack.us                          1 not reachable                                     1 field                                 em
166 ll.thespacedevs.com                      1 366 SpaceDevs launches                               1 field                                 em
167 onionoo.torproject.org                   1 9.742 Tor Relays                                     1 source                                em
168 wikimedia.org                            1 not reachable                                     1 source                                em
169 ws.cadc-ccda.hia-iha.nrc-cnrc.gc.ca      1 not reachable                                     3 fields                                 em
170 geoserver.efas.eu                        1 not reachable                                     1 source                                diffusion
171 sky.esa.int                              1 not reachable                                     1 source                                em
172 network.satnogs.org                      1 1 source URLs (no total derivable)           1 field                                 em
173 fireball.fripon.org                      1 FRIPON fireballs (20k)                             6 fields                                 em
174 www.globalcmt.org                        1 8 lines                                              1 source                                seismic-body
175 archive.gemini.edu                       1 not reachable                                     1 field                                 em
176 catalogue.dataspace.copernicus.eu        1 not reachable                                     2 fields                                 em
177 ies-ows.jrc.ec.europa.eu                 1 6 lines                                              1 source                                em
178 www.globalfloods.eu                      1 84 lines                                             4 fields                                 em
179 erddap.aoml.noaa.gov                     1 1 source URLs (no total derivable)           2 fields                                 em
180 erddap.emodnet-physics.eu                1 EMODnet Physics ERDDAP (2.1M)                        1 field                                 em
181 dap.oceannetworks.ca                     1 1 source URLs (no total derivable)           1 field                                 diffusion
182 pegelonline.wsv.de                       1 not reachable                                     1 source                                gravity
183 records-ws.nbnatlas.org                  1 not reachable                                     1 field                                 acoustic
184 archive.nrao.edu                         1 97 lines                                             2 fields                                 em
185 services.nvd.nist.gov                    1 7 keys                                               1 field                                 em
186 eyes.nasa.gov                            1 112 lines                                            1 field                                 em
187 api.safecast.org                         1 1 source URLs (no total derivable)           2 fields                                 em
188 www.temis.nl                             1 202 lines                                            1 source                                em
189 terrabrasilis.dpi.inpe.br                1 65 lines                                             1 source                                diffusion
190 db.satnogs.org                           1 1 source URLs (no total derivable)           6 fields                                 em
191 data.cdc.gov                             1 1 source URLs (no total derivable)           1 source                                em
192 weather.uwyo.edu                         1 not reachable                                     1 station, 1 field                    thermal
193 vsx.aavso.org                            1 1 source URLs (no total derivable)           1 field                                 em
194 newton.spacedys.com                      1 169 lines                                            1 field                                 gravity
195 opendata.dwd.de                          1 27 content                                           1 source                                em
196 api.met.no                               1 not reachable                                     1 source                                em
197 opendata.ecdc.europa.eu                  1 ECDC health data (51k)                          1 field                                 em
198 epoch.ai                                 1 6k lines                                             1 field                                 em
199 www.peeringdb.com                        1 2 keys                                               2 fields                                 em
200 modis.ornl.gov                           1 not reachable                                     1 source                                em
201 restcountries.com                        1 3 keys                                               1 source                                em
202 www.submarinecablemap.com                1 718 features                                         1 field                                 em
203 api.unhcr.org                            1 5 keys                                               1 field                                 em
204 www.star.nesdis.noaa.gov                 1 3 lines                                              1 field                                 em
205 xmart-api-public.who.int                 1 WHO health data (120k)                          1 field                                 em
206 www.bodc.ac.uk                           1 not reachable                                     2 fields                                 diffusion
207 www.chime-frb.ca                         1 199 lines                                            1 source                                em
208 mars.nasa.gov                            1 4k soles                                             1 source                                em
209 crates.io                                1 2 keys                                               1 source                                em
210 dasch.rc.fas.harvard.edu                 1 55 lines                                             1 field                                 em
211 re.jrc.ec.europa.eu                      1 not reachable                                     1 source                                em
212 vires.services                           1 not reachable                                     2 parameters                              gravity
213 services.terrascope.be                   1 Terrascope (100/Query)                               1 source                                diffusion
214 catalogue.clms.copernicus.eu             1 not reachable                                     8 fields                                 em
215 usdmdataservices.unl.edu                 1 not reachable                                     1 source                                em
216 ows.emodnet-humanactivities.eu           1 not reachable                                     1 field                                 em
217 erddap.dataexplorer.oceanobservatori     1 OOI ERDDAP (28k)                                     2 fields                                 em
218 datacenter.iers.org                      1 not reachable                                     1 source                                gravity
219 maia.usno.navy.mil                       1 181 lines                                            1 source                                em
220 network.igs.org                          1 20 data                                              1 field                                 seismic-surfac
221 www.tng-project.org                      1 79 lines                                             2 fields                                 gravity
222 www.ioc-sealevelmonitoring.org           1 1 source URLs (no total derivable)           1 field                                 gravity
223 ds.iris.edu                              1 not reachable                                     1 source                                seismic-body
224 landsatlook.usgs.gov                     1 10 features                                          1 source                                em
225 tess.mit.edu                             1 not reachable                                     1 source                                em
226 www.crystallography.net                  1 not reachable                                     1 source                                em
227 aa.usno.navy.mil                         1 not reachable                                     1 source                                em
228 data.giss.nasa.gov                       1 149 lines                                            1 source                                thermal
229 modis.gsfc.nasa.gov                      1 not reachable                                     1 source                                em
230 physics.nist.gov                         1 360 lines                                            1 source                                em
231 psl.noaa.gov                             1 89 lines                                             3 fields                                 thermal
232 gis.ngdc.noaa.gov                        1 1 source URLs (no total derivable)           1 source                                gravity
233 registry.npmjs.org                       1 not reachable                                     1 source                                em
234 api.astrocats.space                      1 1 source URLs (no total derivable)           1 source                                em
235 my.cmems-du.eu                           1 257 lines                                            4 fields                                 em
236 world.openfoodfacts.org                  1 4.6 million OpenFoodFacts products                      4 fields                                 em
237 stac.openlandmap.org                     1 1 source URLs (no total derivable)           1 source                                diffusion
238 paleobiodb.org                           1 2 keys                                               1 source                                em
239 atlas.ripe.net                           1 RIPE Atlas 14.4k probes                              7 fields                                 em
240 gis1.servirglobal.net                    1 1 source URLs (no total derivable)           1 source                                diffusion
241 tevcat2.tevcat.org                       1 1 source URLs (no total derivable)           6 fields                                 em
242 data.cosmic.ucar.edu                     1 9 lines                                              1 source                                diffusion
243 rest.uniprot.org                         1 not reachable                                     1 source                                em
244 nwis.waterdata.usgs.gov                  1 not reachable                                     1 source                                diffusion
245 www.sciencebase.gov                      1 not reachable                                     1 field                                 diffusion
246 developer.uspto.gov                      1 99 lines                                             1 source                                em
247 www.wikidata.org                         1 2 keys                                               1 source                                em
248 api.openbrewerydb.org                    1 1 source URLs (no total derivable)           2 fields                                 em
249 services7.arcgis.com                     1 360 features                                         1 source                                diffusion
250 www.glims.org                            1 not reachable                                     1 source                                thermal
251 stationview.raspberryshake.org           1 1 source URLs (no total derivable)           1 field                                 seismic-surfac
252 egg.astro.cornell.edu                    1 ALFALFA galaxies (15.9k)                             15 fields                                em
253 www.metoffice.gov.uk                     1 178 lines                                            1 source                                em
254 data-api.globalforestwatch.org           1 2 keys                                               1 source                                em
255 esgf.ceda.ac.uk                          1 145 lines                                            1 source                                em
256 stac.eoapi.dev                           1 41 features                                          1 field                                 em
257 icgem.gfz-potsdam.de                     1 713 lines                                            1 field                                 em
258 api.open-elevation.com                   1 not reachable                                     2 fields                                 em
259 planetarycomputer.microsoft.com          1 136 features                                         1 field                                 em

────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
  TOTAL: 259 hosts, 2199 sources
  PROVIDER TOTALS = researched + API-derived total scope
  OUR USE = tables, stations, parameters, fields we extract
  "X source URLs" = no total derivable (single API / single source)
