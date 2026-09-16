<!--
  title: Survey — dead_sources.φ Relevanz-Erstpass (2026-09-16)
  class: survey
  date: 2026-09-16
  sha256: b509990570167ec036a41e575e6b78347c27bbcf5ce539ff1e3b6580d92163c1
  status: live
  see-also: phi/dead_sources.φ docs/SOURCE_PORT.md phi/sources.φ
-->
# Survey — `dead_sources.φ` Relevanz-Erstpass (2026-09-16)

## Zweck

Der Domaincheck vom 2026-09-16 (`getent hosts`, 275 eindeutige Domains aus 362
Einträgen) fand **52 DNS-tote Domains**; alle tragen in ihrer `note` bereits eine
Dienst-Identität + Nachfolger (Browser-Check 2026-09-10), also gelebt. Sechs
Einträge, die **nie gelebt** haben, wurden entfernt: die vokal-ausgedünnten
`p.ntrlst.rg` (= `api.inaturalist.org`) und `p.pn-mt.cm` (= `api.open-meteo.com`)
— beide Dienste leben —, `lightning.gld` (keine Identität) und
`masie_ice.apps.nsidc.org` (Unterstrich-Host; echter Host `masie_web`, behalten).

Dieses Survey trägt den **ersten Relevanz-Pass** (flash): Kandidaten, die nie ein
Force-Kanal sein können — die Grundlage für ein Force-Gate-Verdikt der
Ernte-Linie. Es ist ein Erst-Pass ohne Re-Fetch; die Klassifikation stützt sich
auf Domain + `note` (Dienst-Identität).

## Kriterium

Force-relevant = das System trägt Gravitation, em, seismisch, ionosphärisch,
magnetosphärisch, solar, atmosphärisch, hydrologisch, ozeanographisch, geodätisch,
Radio-Science, planetarisch, astrophysikalische Transienten oder LAIC/Bio-Kopplung
(`docs/SOURCE_PORT.md` §3, `phi/sources.φ`).

## Sichere Kandidaten (91)

**Transport / Mobilität / Aviation (11)** — kein Feld:
`api.digitransit.fi` (×2), `api.entur.io` (×2), `gateway.apiportal.ns.nl`,
`www.mvg.de`, `www.trafiklab.se`, `aisstream.io`, `api.adsb.lol`,
`soa.smext.faa.gov` (×2).

**Spaceflight-Logistik / Orbit-Track (4)** — Logistik/abgeleitete Bahn, kein Feld:
`api.spacexdata.com` (×2), `www.n2yo.com`, `celestrak.com`.

**Biodiversität / Ökologie / Life-Science (20)**:
`api.inaturalist.org`, `api.boldsystems.org` (×2), `portal.boldsystems.org`,
`api.ebird.org`, `api.ebeard.org`, `api.eurobis.org`, `api.marinespecies.org`,
`api.fisheries.noaa.gov`, `mangrove-atlas.sei.org` (×2), `www.eurobats.org`,
`www.invasivesnet.org` (×2), `www.xeno-canto.org`, `xeno-canto.org`,
`www.addgene.org`, `www.treetalker.xyz`, `openneuro.org`, `www.ebi.ac.uk`.

**Agritech / Agrar / Ernährungsstatistik (8)**:
`fenixservices.fao.org` (×2), `faostat4.fao.org` (×2),
`world.openagritechdata.org`, `data.apps.fao.org` (×2), `www.fao.org`.

**IT / Infrastruktur / Statistik / generische Web-Dienste (48)**:
`api.ioda.caida.org`, `api.ioda.ioda.caida.org`, `ris.ripe.net` (×2),
`poweroutage.us`, `mempool.space`, `openquantumsafe.org` (×2),
`overpass-api.de`, `overpass.kumi.systems`, `nwis.waterdata.usgs.gov`,
`localhost:5000`, `time.nist.gov`, `worldtimeapi.org`, `api.reliefweb.int`,
`data.humdata.org`, `wikimedia.org` (×2), `api.usa.gov`, `apidev.uis.unesco.org`,
`sdmx.oecd.org`, `washdata.org` (×2), `www.govtrack.us`, `www.who.int`,
`ebolaintel.com`, `openeo.org`, `example.com`, `api.open-notify.org`,
`api.farmsense.net` (×2), `astromap.app`, `resonanceone.app`,
`quantumrandomnumbergenerator.net` (×2), `api.adsabs.harvard.edu` (×2),
`cartocdn-gusc.global.ssl.fastly.net`, `sekitan.jp` (×2),
`api.ember-climate.org`, `openinfra.io` (×2), `pris.iaea.org` (×2),
`nucleus.iaea.org` (×2), `climada.ethz.ch`.

## Unsicher (35) — zweiter Pass

`api.open-elevation.com`, `mrdata.usgs.gov`, `data.nasa.gov`, `zenodo.org`,
`bhuvan-panchayat.nrsc.gov.in`, `bhuvan.nrsc.gov.in`,
`catalogue.clms.copernicus.eu`, `esa-worldcover.s3…` (×2),
`s3…wasabisys.com/esa-worldcover` (×2), `globalland.vgt.vito.be`,
`jeodpp.jrc.ec.europa.eu` DRAXIS (×2), `land.copernicus.eu`, `landsat.usgs.gov`,
`modisrest.ornl.gov`, `dods.wh.gov`, `emergency.copernicus.eu`, `odlinfo.bfs.de`,
`osdr.nasa.gov`, `pskreporter.info`, `remon.jrc.ec.europa.eu`,
`surveys.coast.noaa.gov`, `theoceancleanup.com` (×2), `unosat.org` (×2),
`www.irsn.fr` (×2), `pangaea.de`, `reversebeacon.net`, `transnetbw.de`,
`arvo-registry.sci.am` (×2).

Grenze: Land-Cover, Strahlungs-Monitoring, Amateurfunk-Propagation, generische
Repositories und VO-Registries sind die schwache Kante — deshalb unsicher, nicht
gestrichen.

## Nächster Schritt

Force-Gate-Verdikt der Ernte-Linie nach `docs/SOURCE_PORT.md`: sichere Kandidaten
prüfen, `declined`/Streichen je Eintrag, unsichere in einem zweiten Pass messen.
Die zwei lebenden Dienste (`api.inaturalist.org`, `api.open-meteo.com`) sind
Kandidaten für eine Quellen-Registrierung, keine Toten.
