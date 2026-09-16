<!--
  title: Survey — fremde Parser-/Compiler-Sammlungen (Stand 2026-09-16)
  class: survey
  date: 2026-09-16
  sha256: 82d6c010522e8904c334d56362664186870b2982d46afeea8b44d467fec239d5
  status: live
  see-also: phi/sources.φ phi/declined_sources.φ phi/blocked_sources.φ phi/dead_sources.φ
-->
# Survey — fremde Parser-/Compiler-Sammlungen (Stand 2026-09-16)

Frage: Gibt es eine **komplettere** Sammlung an Compilern/Parsern als unsere — und
was lohnt sich abzuschauen? Gemessen per `archive_search` (brave/github) und
GitHub-API am 2026-09-16. Kein Verdikt über Herkunft, nur die Messung.

## Teil 1 — Vergleichstabelle

| Sammlung | Domäne | Umfang (gemessen) | Ausgabe/Vertrag | Lizenz | Verhältnis zu uns |
|---|---|---|---|---|---|
| **omegaflow (wir)** | domänenübergreifend: Astro · Helio · Geo · Klima · Ozean · Medizin | **918** Live-Quellen, **1655** Oszillatoren, **424** Körper-Anker; **197** harvest-Bins + **233** measure-Bins | φ **26×f64** + 9 Kraftmedien, ICRS/J2000 | PolyForm / CC BY-NC-SA | Referenz |
| **astroquery** (`astropy/astroquery`) | Astronomie | **69** Module (`astroquery/`-Einträge, GitHub-API) | astropy `Table` | BSD-3-Clause | schmaler (eine Domäne) |
| **SunPy / Fido** (`sunpy/sunpy`) | Solar | **19** Subpakete | `Map`/`Table` | BSD-2-Clause | schmaler |
| **pyvo** (`astropy/pyvo`) | Virtual Observatory (TAP/SIA/SSA) | **11** Subpakete | astropy/VOTable | BSD-3-Clause | schmaler |
| **HAPI** (+ `hapi-server/data-specification`) | Heliophysik | föderierte Server (AMDA, CCMC, CDAWeb, DAS2, **INTERMAGNET**, LISIRD, SSCWeb, **ViRES**, …) | REST-JSON/CSV + Streaming | Spec, kein Lizenz-Assert | **nutzen wir** (ViRES, INTERMAGNET) |
| **GDAL/OGR** (`OSGeo/gdal`) | Geo-Formate | **117** Raster- + **79** Vektor-Format-Treiber = **196** (GitHub-API) | Raster-/Vektor-Datasets | MIT/X11 (API: `NOASSERTION`) | Formate, nicht Quellen; C-Trade-off |
| **Airbyte** (`airbytehq/airbyte`) | Business-ETL | **700+** Connectoren (airbyte.com/connectors) | Warehouse/DB-Sinks | MIT/ELv2 (API: `NOASSERTION`) | falsche Domäne |
| **Meltano / Singer** (`meltano/meltano`) | Business-ETL | **550+** Connectoren (meltano.com) | JSON-Streams (taps/targets) | MIT | falsche Domäne |
| **Frictionless Data** (`frictionlessdata/frictionless-py`) | tabulare Specs | Spec + Bibliothek | Data Package | MIT | unser φ-Vertrag ist stärker |

> **Fußnote Einheiten & Schnittmenge.** Die Umfänge sind in **verschiedenen
> Maßen** gezählt: astroquery/SunPy/pyvo in *Modulen/Subpaketen* (je Modul viele
> Dienste), wir in *Live-Quellen* und *Oszillatoren*. **Gemessene Schnittmenge
> (2026-09-16):** astroquery erreicht **199 unserer 918 Quellen** und damit
> **147 von 1655 Oszillatoren (≈8,9 %)** — die astronomische Minderheit
> (v. a. `ssd.jpl.nasa.gov` 141, Vizier 20, IMCCE 12, IRSA 5, ESAC 4). Das
> Kreuz-Domänen-Ergebnis überlebt die Frage: ~91 % der Oszillatoren liegen
> außerhalb jeder Fremd-SDK.

## Teil 2 — Domänen-Coverage (unsere Quellen, gemessen aus `phi/sources.φ`)

**Umfang.** `phi/sources.φ`: **918** `url`-Zeilen (Live), **1655** `field`-Zeilen
(Oszillatoren), **424** `at`-Zeilen (Körper-Anker), 390 `format`, 303 `epoch`,
266 `lat`/`lon`. Die `url`-Zeilen enthalten den **CDN**
(`github.com/omegaflow/sources/releases/download/<host>/…`) — der **echte
Quellhost** steht im Pfad; für die Schnittmenge wurde er aufgelöst. Registries
gesamt: **918 live · 912 declined · 12 blocked ·
244 dead** (~2086 katalogisierte Quellen). Tools: **197** harvest-Bins + **233**
measure-Bins.

**Krafttypen** (1655 Oszillatoren, `field $5`):

| Kraft | Anzahl |
|---|---|
| acoustic | 426 |
| advective | 399 |
| em | 338 |
| thermal | 301 |
| gravity | 97 |
| diffusion | 62 |
| seismic-surface | 14 |
| electric | 11 |
| seismic-body | 7 |

**Nicht mitgezählt: Kernel.** ak135, GEBCO, ETOPO1 u. a. sind **Werkzeuge/Kernel**
(Erdmodelle, Bathymetrie), keine Oszillator-Quellen — `seismic-body` = 7 zählt nur
Quellen, nicht die seismische Arbeit. Die Seismik-Front (Tiefenphasen, Flotte,
Eikonal) lebt in Kerneln und Proben, nicht in dieser Zeile; sie ist kein
Widerspruch zur Zahl.

**Körper-Anker** (`at`, Auszug): sun 224, earth 111, parker_solar_probe 4,
solar_orbiter 3, moon 3, wind 2, phobos 2, neptune 2, mars 2, deimos 2,
voyager1/voyager2/vesta/venus je 1. Schwerpunkt: Sonne + Erde.

**Standards, die wir bereits nutzen** (im Baum gemessen): HAPI (`vires.services`,
`BGS-INTERMAGNET-HAPI`), IVOA/VO (TAP), OAI-PMH (`oai_harvester.rs`), STAC
(CDSE/Copernicus Data Space, Planetary Computer, PDS-STAC), OPeNDAP
(`opendap_reader.rs`), ERDDAP (`erddap_harvester.rs`).

## Methode & Grenzen

- **Suchraum:** `archive_search` (brave/github) + GitHub-API, 2026-09-16 — unser
  Werkzeug, unser Fensterausschnitt; kein vollständiger Markt-Scan.
- **Aus Lehrbuch-Wissen ergänzt (ungemessen):** **pySPEDAS** (Heliophysik,
  NASA-Ökosystem) fiele in die SDK-Klasse und ändert das Verdikt nicht — eine
  Vermutung, keine Messung.
- **Schnittmenge gemessen:** astroquery ↔ unsere Quellen, per Host-Mapping über
  die aufgelösten Quellhosts (siehe Fußnote Teil 1) — 147/1655 Oszillatoren.
- **Gegenprobe offen (nicht meßpflichtig):** 199/918 ≈ 21,7 % — welche
  *astroquery*-Module das abdecken und welche unserer astronomischen Quellen
  astroquery **nicht** erreicht, ist nicht gezählt; relevant nur, falls das Blatt
  je als „wir sind keine astroquery-Teilmenge" zitiert wird.
- **Einseitiger Spiegel:** gewogen wurden *unsere* Quellen gegen astroquery,
  **nicht** astroquerys Gesamtreichweite (es erreicht Quellen, die wir bewußt
  declined oder nie gesucht haben). „Kein Superset" trägt; die Lesart „wir decken
  astroquerys Domäne ab" wäre stärker als die Messung.

## Verdikt

**Kein Superset.** Die drei Fremd-Klassen decken je nur eine Achse:

- **Domänen-SDKs** (astroquery 69, SunPy 19, pyvo 11) — je **eine** Domäne
  (Astronomie/Solar/VO); wir sind domänenübergreifend breiter.
- **Format-Parser** (GDAL 196 Treiber) — Formate, nicht Quellen; als
  C-Abhängigkeit gegen den „Rust std + curl + serialport"-Stack.
- **Generische ETL** (Airbyte 700+, Meltano 550+) — Business/SaaS/DB, nicht
  wissenschaftliches Quell-Parsing.

**Unser Alleinstellungsmerkmal:** domänenübergreifende Quellbreite unter **einem**
Vertrag (φ 26×f64 + 9 Kraftmedien, ICRS/J2000) **und** einem physikalischen
Modell (Kraft/Oszillator). Kein fremdes Projekt vereinigt beides.

**Was wir nicht haben** (bewusst nicht übernommen): Per-Quelle-Query-Tiefe und
Community (astroquery/GDAL) sowie Format-Breite (GDAL). Ein Ab-Schauen lohnt
**nur als Muster** (Auth/Retry/Pagination je Quelle), nicht als Abhängigkeit.
**Ehrlich dazu:** astroquery kennt die Auth-Macken einzelner Dienste tiefer als
jedes Muster. Wo die Macken einer Quelle das Muster übersteigen, wird das Muster
zur **privaten** Abhängigkeit, die nur dieses Haus wartet — der Unterhaltspreis
der Unabhängigkeit: klein heute, wachsend mit jeder Sonderquelle.

**Lizenz (die stillgestellte Rechnung).** Die Sammlung steht unter **PolyForm /
CC BY-NC-SA — non-commercial**. Sie kann so nicht verkauft werden; jede
Monetarisierung beginnt mit einer Lizenz-Entscheidung. Keine Kritik, eine offene
Zeile.

**Standards statt Sammlung:** Wo eine Quelle einen Standard anbietet
(HAPI/VO/STAC/OAI-PMH/OPeNDAP/ERDDAP), nutzen wir ihn — das ersetzt die
Fremd-SDKs.
