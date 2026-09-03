<!--
  title: Adapter family: Harvester vs. Compiler
  class: concept
  sha256: d28e5a4dec3eca06042bd7a74e233aa9d5c66b6eedb0f53a080aef491c07c8d0
-->
# Adapter family: Harvester vs. Compiler

The tools that pull external data catalogs into the φ system have two
roles — named after what they do. No mixing: a Harvester is
not a Compiler, a catalog is not a tool.

## Harvester — harvests metadata → catalog inventory

A Harvester reads the metadata interface of a catalog (OAI-PMH,
Solr, Dataverse, REST) and writes an inventory file to `phi/pipeline/catalog/`:
`<identifier> | <title>`. It harvests NOTHING measurement-like — only the
holdings (DOI/ID + title). The measurement values lie deeper; the Compiler fetches those.

| Harvester | Protocol | Catalogs |
|---|---|---|
| `oai_harvester` | OAI-PMH (resumptionToken) | PANGAEA mirror, SEANOE, GFZ-DOIDB/IGETS, Zenodo |
| `dataverse_harvester` | Dataverse REST | INRAE, Harvard, Borealis … |
| `solr_harvester` | Solr JSON (`--fields`) | WDCC/CERA, DataONE |
| `deims_harvester` | DEIMS REST | LTER sites + sensors |
| `pangaea_harvester` | Collection→Member→.tab (mirror) | PANGAEA cores |

## Compiler — compiles static measurement values → flat-json + block

A Compiler resolves the data access structure (collection→member,
TAP rows→key objects) and compiles the MEASUREMENT VALUES into a flat-json
(`lat/lon/…` per line) + a `sources.φ` block (CDN asset). It produces
oscillator-carrying data, not inventory.

| Compiler | Access | Catalogs |
|---|---|---|
| `tap_compiler` | TAP/ADQL | VizieR, IRSA, GAVO, ARI, ExoArchive |

## Catalog — the result, not the tool

A catalog is the inventory file in `phi/pipeline/catalog/*.φ` — the harvested
holdings. It is the QUEUE: each entry becomes (a) a Compiler call
(static measurement values → flat-json + block) or (b) a probe candidate
(live API → verdict).

## The three roles in the funnel

```
Harvester  →  catalog (phi/pipeline/catalog/)  →  Compiler (flat-json + block)  →  sources.φ
                                      →  probe     (live verdict)       →  sources.φ / dead_sources.φ / blocked_sources.φ
```

## Naming rule

- `*_harvester` harvests metadata → `phi/pipeline/catalog/*.φ` (inventory).
- `*_compiler` compiles measurement values → flat-json (CDN) + `sources.φ` block.
- `source_scanner` weighs candidates (the lens).
- `--probe` verifies live APIs (the judge).
- The `*_compiler` of the ephemerides/kernels (ephemeris/tycho2/dastcom/dcom5/
  sexagesimal) stays — they compile static celestial mechanics data.

The Compiler flat-jsons (`phi/pipeline/*.json`) are transient CDN intermediate stages
and gitignored — the block references the CDN asset, not the local file.
