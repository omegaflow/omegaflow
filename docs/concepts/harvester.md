<!--
  title: Adapter-Familie: Harvester vs. Compiler
  class: concept
  sha256: 8f34e957b9a70dc84d2f5515ce7e20fef32ac53528a55cb1676ddcfb35c863cc
-->
# Adapter-Familie: Harvester vs. Compiler

Die Werkzeuge, die externe Datenkataloge in das φ-System ziehen, haben zwei
Rollen — benannt nach dem, was sie tun. Kein Vermischen: ein Harvester ist
kein Compiler, ein Katalog ist kein Werkzeug.

## Harvester — erntet Metadaten → Katalog-Inventar

Ein Harvester liest die Metadaten-Schnittstelle eines Katalogs (OAI-PMH,
Solr, Dataverse, REST) und schreibt eine Inventar-Datei nach `phi/pipeline/katalog/`:
`<identifikator> | <titel>`. Er erntet NICHTS Messwert-artiges — nur den
Bestand (DOI/ID + Titel). Die Messwerte liegen tiefer; die holt der Compiler.

| Harvester | Protokoll | Kataloge |
|---|---|---|
| `oai_harvester` | OAI-PMH (resumptionToken) | PANGAEA-Spiegel, SEANOE, GFZ-DOIDB/IGETS, Zenodo |
| `dataverse_harvester` | Dataverse REST | INRAE, Harvard, Borealis … |
| `solr_harvester` | Solr JSON (`--fields`) | WDCC/CERA, DataONE |
| `deims_harvester` | DEIMS REST | LTER Sites + Sensoren |

## Compiler — kompiliert statische Messwerte → Flat-json + Block

Ein Compiler löst die Daten-Zugriffsstruktur auf (Collection→Member,
TAP-Zeilen→Key-Objekte) und kompiliert die MESSWERTE zu einer Flat-json
(`lat/lon/…` je Zeile) + einem `sources.φ`-Block (CDN-Asset). Er produziert
Oszillator-tragende Daten, kein Inventar.

| Compiler | Zugriff | Kataloge |
|---|---|---|
| `tap_compiler` | TAP/ADQL | VizieR, IRSA, GAVO, ARI, ExoArchive |
| `pangaea_compiler` | Collection→Member→.tab | PANGAEA-Kerne |

## Katalog — das Ergebnis, nicht das Werkzeug

Ein Katalog ist die Inventar-Datei in `phi/pipeline/katalog/*.φ` — der geerntete
Bestand. Er ist die QUEUE: aus jedem Eintrag wird (a) ein Compiler-Aufruf
(statische Messwerte → Flat-json + Block) oder (b) ein Probe-Kandidat
(Live-API → Verdict).

## Die drei Rollen im Trichter

```
Harvester  →  Katalog (phi/pipeline/katalog/)  →  Compiler (Flat-json + Block)  →  sources.φ
                                      →  Probe     (Live-Verdict)       →  sources.φ / dead_sources.φ / blocked_sources.φ
```

## Benennungsregel

- `*_harvester` erntet Metadaten → `phi/pipeline/katalog/*.φ` (Inventar).
- `*_compiler` kompiliert Messwerte → Flat-json (CDN) + `sources.φ`-Block.
- `source_scanner` wiegt Kandidaten (die Linse).
- `--probe` verifiziert Live-APIs (der Richter).
- `*_compiler` der Ephemeriden/Kernel (ephemeris/tycho2/dastcom/dcom5/
  sexagesimal) bleiben — sie kompilieren statische Himmelsmechanik-Daten.

Die Compiler-Flat-jsons (`phi/pipeline/*.json`) sind transiente CDN-Zwischenstufen
und gitignored — der Block referenziert das CDN-Asset, nicht die lokale Datei.
