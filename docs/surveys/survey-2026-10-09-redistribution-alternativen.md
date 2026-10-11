<!--
  title: Survey — Freie Alternativen für redistributions-abgelehnte Quellen (2026-10-09)
  class: survey
  date: 2026-10-09
  sha256: bc5bd59be7738632592879da3d9328f47f84cab6acbc25668c8b9b2b2fba263e
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

## Zweiter Lauf — Lizenz-Zertifizierung der Shortlist (2026-10-09)

Die HTTP-gemessene Shortlist wurde am Lizenztext zertifiziert (`--jina`):

**Neu CDN-fähig (offene Lizenz + HTTP 2xx):**

| Kandidat | Lizenz (gemessen) |
|---|---|
| NOAA SWPC | US-Government public domain (`weather.gov/disclaimer`) |
| NOAA NCEI | PD / CC0 (NCEI Open Data Policy) |
| NASA Earthdata | CC0 (NASA Science Data License) |
| JPL SSD | CC0 (NASA-led) |
| Copernicus Data Space (Sentinel) | Copernicus Sentinel Data licence (frei/voll/offen + Attribution) |
| RouteViews | **CC-BY-4.0** |
| GOES GLM | PD / CC0 (NCEI/NOAA) |
| NOAA NDBC | US-Government public domain |
| ICGEM GFZ | **CC-BY-4.0** |
| Global Floods (GloFAS/CEMS) | CEMS-Lizenz (offen; restricted subset prüfen) |
| HDX/OCHA · Copernicus Emergency | je Dataset CC-BY/CC0/ODbL (Plattform offen) |

**Redistributions-/klarheits-blockiert (nicht CDN-fähig):**

- **GRDC** — „No redistribution" (der Veto ist die Redistribution, nicht das NC).
- **RIPE RIS** — keine Standard-Open-Lizenz; widerrufliche Default-Erlaubnis, Repository-Terms restriktiv → unklar.

**NC ist kein Block (Korrektur eines Fabrikationsmusters im zweiten Lauf).** Das
Projekt ist selbst NC (PolyForm NC + CC BY-NC-SA); die geschlossene Terms-Vokabel
führt `CC-BY-NC-3.0-IGO`/`CC-BY-NC-4.0`/`CC-BY-NC-SA-4.0` (`license_census.rs:12-14`),
und **181** `sources.φ`-Einträge tragen bereits `CC-BY-NC-3.0-IGO` (ESA). Die einzige
harte Terms-Wache ist `ohne-lizenz` auf einem CDN-Block (`license_census.rs:241-257`).
Damit sind **GIRO DIDBase (CC-BY-NC-SA-4.0) und INTERMAGNET (CC-BY-NC-4.0)
NC-kompatibel → admissibel**, nicht blockiert. Die `decline redistribution`-Verdikte
der 41 Blöcke gründen auf **expliziten Redistributions-Verboten** (kommerzielle ToS),
nicht auf NC.

**Nächster Schritt:** Mountains Re-Admission je Kandidat → dann CDN-Workflow/`sources.φ`-Block.
Divergenz: `earthdata.nasa.gov`-Policy 403 (CloudFront); Lizenz über
`science.data.nasa.gov/about/license` belegt.

## Rückgewinnungs-Karte (2026-10-09)

Für die 41 abgelehnten Quellen wurde gemessen, ob die **Messung** über einen
legalen Weg bereits vorhanden ist (`sgrep -c <alternative> phi/sources.φ`):

- **schon-da (23)** — die Messung läuft bereits über eine zugelassene freie Quelle:
  Blitzortung (×2) + WWLLN (×2) + GLD360 → **GOES GLM** (`glm_l2.bin`);
  PurpleAir → OpenAQ/Sensor.Community; Sofar → **NDBC**; LHAASO → TeVCat;
  Amentum → **INTERMAGNET/SWPC**; Meteostat/WeatherXM → **Open-Meteo**;
  Sentinel-Hub (×2) → **Copernicus**; InPOP19a/NOE-4 → **JPL SSD**;
  SuperMAG (×3) → INTERMAGNET/SWPC; BoM → SWPC; hamqsl → SWPC; ogimet → NCEI.
- **neu-zulassen (13)** — Kandidat gemessen, nicht in `sources.φ` (Mountain-Admission):
  `tile.openstreetmap.org` (ODbL) · `archive.routeviews.org` (**CC-BY-4.0**) ·
  `giro.uml.edu/didbase` (**CC-BY-NC-SA-4.0**, NC-kompatibel) ·
  `data.mendeley.com`/`waterisotopesDB.org` (GNIP PD) · `globalfloods.eu` (GloFAS/CEMS) ·
  `data-api.globalforestwatch.org` · `transport.data.gouv.fr` · `api.carbonintensity.org.uk` ·
  `aishub.net`/`dma.dk` · `ds.iris.edu` · `opensky-network.org` (Key/Account offen).
- **blockiert (5)** — keine legale Route gemessen: `almascience.org` (kein Kandidat),
  `data.lsst.cloud` (proprietäre Frist), `api.z.ai` ×2 + `reddit.com/dev/api` (kein Messwert).

### Feld-Zulassung — die Redistributions-Route ist nicht die Feld-Route

**Korrektur:** die Karte oben ist eine **Redistributions**-Karte. `redistribution` ist nur
**43** von ~1600 Decline-Blöcken; die dominante Ursache ist **`no-physical-force` (250)**,
dann `registry/katalog` (~229), `superseded-by-*` (~160), `model-forecast` (~85),
`variant` (62), `static` (51), `molecular` (48). Ein free-/legaler Kandidat, der keine
Kraft/Quantity trägt, wird am Feld-Gate **`no-physical-force`** abgelehnt.

Unter den 13 `neu-zulassen` ist nur ein Teil **feld-fähig** (9 Kräfte: em · gravity ·
acoustic · seismic-body · seismic-surface · thermal · diffusion · advective · electric):

| Kandidat | Feld? | Träger |
|---|---|---|
| `giro.uml.edu/didbase` (foF2) | **ja** | electric/quantity (Ionosphäre, Frequenz) |
| `ds.iris.edu` (CTBTO Hydroakustik) | **ja** | acoustic/seismic-body |
| `data.mendeley.com`/`waterisotopesDB` (GNIP) | **evtl.** | isotope ratio (dimensionlos — derselbe Kontrakt-Riss wie Keogramm) |
| `globalfloods.eu` (GloFAS) | **evtl.** | advective — aber **model-forecast** (Decline-Muster) |
| `tile.openstreetmap.org` | **nein** | Bild/Karte → `image`/`registry` |
| `archive.routeviews.org` | **nein** | Netz-Infrastruktur → `infrastructure` |
| `data-api.globalforestwatch.org` | **nein** | Landbedeckung → `no-physical-force` |
| `transport.data.gouv.fr` | **nein** | Transit → `no-physical-force` |
| `api.carbonintensity.org.uk` | **nein** | Netz-CO₂ → `aggregate-index` |
| `aishub.net`/`dma.dk` (AIS) | **nein** | Verkehr → `no-physical-force` |
| `opensky-network.org` (ADS-B) | **nein** | Verkehr → `no-physical-force` |

**Damit:** von den 13 sind nur **~2–4 feld-fähig** (GIRO, ds.iris.edu, ggf. GNIP/GloFAS);
die übrigen sind **Daten-Wiederverwendungs**-Alternativen, keine neuen Felder. Die
`schon-da`-Liste trägt ohnehin nur Messungen, die schon als Feld zugelassen sind.

**Konsequenz:** die 23 `schon-da` brauchen keine Zulassung — die Messung ist zurück;
die 13 `neu-zulassen` sind Mountains Re-Admission (dann Mycelium-CDN); die 5
`blockiert` bleiben.

### Weberin-Eignung — der eigentliche Maßstab

Die Weberin trägt **Weltlinien** (Kette: Direction · Body · Station) und einen
**Schuss** (Verdict · TE · Abstammung) — ein Quell ist weberin-fähig, wenn er
(a) eine **Weltlinie mit gemessenem Skalar-Series** trägt (Station/Body/Direction),
die per TE zwirnbar ist, oder (b) ein **Zeuge** ist: S²-Direction (ra/dec +
Energie, τ=0), räumliche Gestalt (Binding, z. B. `dust-maske.φ`) oder Presence.
**Kein Zeuge:** Modell/Forecast, Katalog/Registry, Bild/Tile, position-only,
statisches Raster ohne Messwert (`die-weberin.md:128`; `survey-2026-09-13-weberin-quellen.md`).

Damit sind von den **13 `neu-zulassen`** nur **2 weberin-tauglich**:
- **`giro.uml.edu/didbase`** (foF2) — Ionosonde-Station, Zeitreihe → **Station-TE-Faden**.
- **`ds.iris.edu`** (Hydroakustik) — Station, akustische Zeitreihe → **Station-TE-Faden**.
- `globalfloods.eu` (GloFAS) — **Forecast → kein Zeuge**.
- `data.mendeley.com` (GNIP) — Isotop-Station, dimensionslose Reihe → **Kontrakt-Riss**, kein Kraft-Faden.
- die übrigen (OSM-Tiles, RouteViews, Forest Watch, transport, carbonintensity, AIS, ADS-B) — **keine Weltlinie, kein Zeuge**.

Von den **23 `schon-da`** sind die echten Weberin-Fäden schon da: **INTERMAGNET**
Station-Magnetometer-Reihen (der Exempel-Zwirn `station_convergence_probe`),
**NDBC** (Boje/Station), **JPL SSD** (Body-Kette), **SWPC/NCEI** (Station/Feld),
**Open-Meteo** (Wetter-Station, thermal), **TeVCat** (S²-Direction-Zeuge);
Copernicus/Sentinel = Gestalt-Binding, GOES GLM = Presence-/Event-Zeuge.

**Kern:** der Redistributions-Filter hat viele **Daten**-, aber nur wenige
**Weberin**-Quellen betroffen. Die verlorenen Weberin-Fäden (Magnetometer,
Seismik, Infraschall, Gravimeter) sind über die zugelassenen freien Quellen
  bereits rückgeholt; neu bleibt allein **GIRO** und **ds.iris.edu**.

## Dritter Lauf — Lizenz-Zertifizierung der blockierten Kerne (2026-10-11)

Die verbleibenden **existierenden, aber lizenz-blockierten** Kandidaten wurden mit
`archive_search --all` (voller Katalog) + `--verdict`/`--jina` neu gemessen; jede
Lizenz-Zeile trägt ihre Quell-URL. Mehrere Verdicts wandern von „blockiert" zu
**legal erreichbar**:

| Quelle | Lizenz (gemessen) | Verdikt | Legale Route |
|---|---|---|---|
| IMCCE INPOP19a / NOE-4 | „may not be used or reproduced without the explicit authorization" (`imcce.fr/mentions-legales`) | blockiert | JPL DE440/441 (PD) · EPM2021 (öffentl. FTP, Lizenz ungemessen) · VSOP/ELP via EPH (`zenodo.org/records/17142232`, CC-BY-4.0) |
| GRDC | „no commercial … no redistribution" (`grdc.bafg.de/about/data_policy`) | blockiert | GRDC-Caravan (CC-BY-4.0, `zenodo.org/records/15349031`) · USGS NWIS (PD) · ECCC/HYDAT (OGL-Canada) · Copernicus GloFAS (frei/open) · ADHI (CC-BY-4.0) |
| RIPE RIS | widerrufliche Default-Erlaubnis (`ripe.net/.../routing-information-service-ris/commercial-use/`) | blockiert | RouteViews (CC-BY-4.0; **Riss**: Seitentext nennt BY, Link zielt BY-SA) |
| ALMA (ObsCore) | JAO-Archiv ohne offene Lizenz | teil-blockiert | **ESO Science Archive (CC BY 4.0, Dritte dürfen verteilen**, `archive.eso.org/cms/eso-data-access-policy.html`) |
| LSST / Rubin TAP | „No access … for individuals who do not have Rubin data rights" (`rubinobservatory.org/for-scientists/data-products/data-policy`) | blockiert | Gaia (CC BY-NC 3.0 IGO, NC-kompatibel) |
| TRISP/YBCO-NSE (MPI-FKF) | kein öffentliches Roh | blockiert | NIST NCNR NSE (US-PD, 17 USC §105) · Zenodo 18306252 (CC-BY-4.0) |

**NC-kompatibel (admissibel, kein Block) — verifiziert:** INTERMAGNET
(**CC BY-NC 4.0**, `intermagnet.org/data_conditions.html`) · GIRO DIDBase
(CC BY-NC-SA 4.0) · waterisotopesDB (NC + Policy-Carry).

**Offen gemessen:** NOAA SWPC/NDBC (PD) · ds.iris.edu (frei, Attribution) ·
globalfloods.eu (Copernicus, frei) · Mendeley (je Dataset CC).

**Risse (nicht geglättet):** RouteViews BY-vs-BY-SA; das ESO-CC-BY-4.0 deckt nur die
ESO-bedienten ALMA-Records (JAO-Archiv separat); EPM2021 trägt keinen expliziten
Lizenztext; DE442 im SSD-Baum nicht einzeln bestätigt; VizieR-TAP down (HTTP 500).

**Konsequenz:** die Re-Admission dieser Kandidaten ist **Mountains Verdikt**; danach
**Mycelium-CDN/`sources.φ`-Block**. Rückzeile an beide (Future 233).
