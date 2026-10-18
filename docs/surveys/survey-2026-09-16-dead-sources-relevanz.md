<!--
  title: Survey — dead_sources.φ Relevanz-Erstpass + Force-Gate-Verdikt (2026-09-16)
  class: survey
  date: 2026-09-16
  sha256: 1a763ebc30f9e363c798a54ca618ce31a9f49381fdda5ab557e4c4fcfac97c0a
  status: live
  see-also: phi/dead_sources.φ docs/SOURCE_PORT.md phi/sources.φ phi/declined_sources.φ
-->
# Survey — `dead_sources.φ` Relevanz-Erstpass + Force-Gate-Verdikt (2026-09-16)

## Zweck

Der Domaincheck 2026-09-16 (275 Domains, 52 DNS-tot; 6 nie-gelebte Einträge
entfernt) führte zum Relevanz-Erstpass (91 sicher + 35 unsicher „nie
Force-Kanal"). Dieses Update trägt das **Force-Gate-Verdikt** der Ernte-Linie
(SOURCE_PORT §8/§3/§11) und die daraus folgende **Prune**: 118 Einträge aus
`phi/dead_sources.φ` entfernt, 9 behalten.

## Kriterium

Oszillator-Gate (SOURCE_PORT §8): könnte ein nicht-menschlicher Organismus ein
Sinnesorgan für diese Messung evolvieren? Automatisch decline: nackte Zählwerte,
Register/Kataloge ohne Messwert, Modell-Forecasts/Reanalysen, Referenz-Konstanten,
aggregierte Indizes, Text-Warnungen, abgeleitete Satellitenprodukte, geographische
Infrastruktur, position-only. Aktuator-Regel: aktives Objekt (Fahrzeug/Schiff/
Flugzeug) → `decline no-physical-force` (Telemetrie eines Akteurs, kein
propagierendes Feld).

## Verdikt — sichere Kandidaten (91, alle declined + geprunt)

| Kategorie | n | Verdikt |
|---|---|---|
| Transport/Mobilität/Aviation | 11 | `decline no-physical-force` (Telemetrie aktiver Objekte, Fahrpläne) |
| Spaceflight-Logistik/Orbit-Track | 4 | `decline no-physical-force`; `celestrak.com` → `decline superseded-by-integrated` (TLE schon declined_sources.φ derived-orbit-fit, Nachfolger celestrak.org) |
| Biodiversität/Ökologie/Life-Science | 20 | `decline presence-catalog` (Occurrence wie GBIF); `api.marinespecies.org`/`www.addgene.org`/`openneuro.org`/`www.ebi.ac.uk` → `decline registry/katalog`; `www.treetalker.xyz` → `decline no-physical-force` (Bio-Rate-Skalare) |
| Agritech/Agrar/Ernährungsstatistik | 8 | `decline no-physical-force` (Statistik/Indizes; NDVI = abgeleitet) |
| IT/Infrastruktur/Statistik | 48 | `decline no-physical-force` (Netz/Strom/Zeit/Statistik); `decline registry/katalog` (ADS, PRIS, NUCLEUS, OpenInfra, sekitan, ReliefWeb, Humdata, usa.gov); `climada.ethz.ch` → `decline model-forecast`; `api.open-notify.org` → `decline no-physical-force` (position-only) |

## Verdikt — unsichere Kandidaten (35)

**Declined + geprunt (26):**
- `decline no-physical-force` (Land-Cover/abgeleitete Produkte/Infrastruktur):
  esa-worldcover (×4), globalland.vgt.vito.be, jeodpp.jrc DRAXIS (×2),
  modisrest.ornl.gov (NDVI), land.copernicus.eu, landsat.usgs.gov (Produkt-Katalog),
  emergency.copernicus.eu, unosat.org (×2), theoceancleanup.com (×2),
  surveys.coast.noaa.gov, bhuvan-panchayat/bhuvan.nrsc.gov.in (×2),
  catalogue.clms.copernicus.eu, mrdata.usgs.gov, data.nasa.gov, zenodo.org,
  pangaea.de, api.open-elevation.com (statisches DEM), www.transnetbw.de (Netzfrequenz).
- `odlinfo.bfs.de` → `decline superseded-by-integrated` (ODL-Messung lebt unter
  imis.bfs.de, sources.φ:773).

**Force-Kanal — behalten (Re-Check-Pflicht, 3):**
- `remon.jrc.ec.europa.eu` — JRC REM Radioaktivitäts-Monitoring (em).
- `www.irsn.fr` (×2) — IRSN Strahlen-Monitoring (em).

**Gemessen — geschlossen (4, 2026-10-18):**
- `dods.wh.gov` — DNS **NXDOMAIN** (`getent` rc=2, `curl` 6); `--verdict` direkt
  und Proton ohne Antwort, Wayback ohne Snapshot, `--mwmbl` absent. Kein
  Akustik-/OPeNDAP-Nachfolger gefunden. → **absent**.
- `osdr.nasa.gov` — API lebt: `/geode-py/ws/studies` **200, 6360 B**;
  `repo/search?term=dosimetry` **123** Treffer, aber nur
  `experiment`/`hardware`/`payload`/`study`-Records (z. B. `GSE51417`); `DOSTEL` **0**,
  `TEPC` **1** (Transkriptomik, nicht Dosimetrie); alte `/bio/api`-Route **404**.
  → **Registry/Metadaten, kein Roh-Zeitreihen-Feed.** Roh-Dosis liegt außerhalb:
  CDAWeb `ISS_DOSANL_TEPC` (`/pub/data/international_space_station_iss/dos_tepc/`
  **200, 2098 B**; CDF 2017 **200, 13 451 393 B**, `DOSE` µGy/min, SPASE `PT10S`).
- `pskreporter.info` + `reversebeacon.net` — PSKReporter-Query **200, 1 517 299 B**
  `<receptionReports>` (Callsign/Locator/Frequenz); RBN Port 7000 **Timeout (rc=124)**.
  Beides Empfangsreport-**Register**, kein Feld am Punkt. Die TEC-Abdeckung ist da:
  5 `sources.φ`-Blöcke (CHAMP, CDAWeb GPS-TEC, CDDIS IONEX, Swarm), 0 Ionosonde.
  → **`decline registry/katalog` (gedeckt)**.
- `arvo-registry.sci.am` (×2) — **nicht tot**, nur HTTPS: DNS `93.187.165.59`;
  `https://…/tap` **000** (443 refused), `http://arvo-registry.sci.am/` **200, 1452 B**,
  `/tap/sync` ADQL `SELECT TOP 1 * FROM TAP_SCHEMA.tables` **200, 114 B**. Die alte
  `dead unreachable`-Messung (SOURCE_PORT §16.4) traf den TLS-Weg; der HTTP-TAP lebt.
  → **Korrektur: live über HTTP; der HTTPS/Proton-Pfad bleibt geschlossen.**

## Zwei lebende Dienste — keine neuen Quellen

Der Erstpass nannte `api.inaturalist.org` und `api.open-meteo.com` als
Quellen-Kandidaten. Das Force-Gate misst:

- `api.inaturalist.org` → **bereits declined** (declined_sources.φ:27–29,
  `decline presence-catalog`, „wie GBIF"). Der tote `/v1/docs/`-Eintrag war ein
  Duplikat → geprunt.
- `api.open-meteo.com` → **bereits registriert** (sources.φ: forecast :192,
  archive-api :211 ff., air-quality :6194 unter air-quality-api.open-meteo.com).
  Der tote `/v1/air-quality`-Pfad auf dem Haupt-Host ist der alte Weg → geprunt.

Keine neuen `sources.φ`-Zeilen.

## Ergebnis

118 Einträge geprunt (91 sicher + 26 unsicher-declined + open-meteo-Stale-Pfad);
9 behalten (3 Force-Kanal, 4 pending, 2 bereits disponiert).
