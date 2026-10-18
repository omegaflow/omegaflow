<!--
  title: Handover — Mycelium (2026-10-16)
  class: handover
  date: 2026-10-17
  sha256: 2eb2853eff0ffab97253d4f32d22e09cf03f197ebe9982732ad565e712c78ce1
  status: live
-->
# Mycelium (2026-10-16)

## Offen — eigen

### UCAR-FPI + MODVOLC — lokal halten; L1 MOD021KM = Mountain
- **Status:** blockiert · **Bindung:** operator (UCAR-DOI/Terms-Mail) · an:mountain (L1 MOD021KM)
- **Trigger:** Operator-Wort 2026-10-17 „lokal halten" (gegeben); Mountain-Übergabe
- **Lage:** (gemessen 2026-10-18) MODVOLC-CDN-Release `nrt3.modaps.eosdis.nasa.gov` existiert nicht mehr (Rückzug wirksam); kein MODVOLC-Block in `phi/sources.φ`; HIGP lokal 89 874 `.dat` unter `data/modis.higp.hawaii.edu/2000/`; UCAR-Athabasca-Koordinate durch river-213 entschieden (54.60282 -113.64442 693); UCAR-Mail ist Entwurf (`mail-2026-10-15-ceic-ucar-re3data.md`)
- **Blockade:** UCAR-DOI/Terms ist ein Operator-Akt; das L1-Feld ist Mountain
- **Braucht:** UCAR-DOI/Terms an `qwu@ucar.edu` (Operator); L1 `MOD021KM` terms/field/ttl + Band-21/22/32-Reader bei Mountain (`xfer:mycelium-361-7`)

### Quellen-Surveys — nächste Route registrieren
- **Status:** wartend · **Bindung:** eigen
- **Trigger:** nächste freie Runde / `bin/where <begriff>`
- **Lage:** (gemessen 2026-10-18) 5 Surveys live; GSRAS (`gsras_teleseismic_catalog_1993.txt`), ISC (bulletin/EHB) und IGETS/CHAMP sind registriert; offen: BGI-Absolutgravimetrie, WDC-B-Obninsk (`cat_OBN`), CMONOC-Zenodo (7325306/7316100/7798617), CEIC-Schwere (7112529/7855090; 6365137 restricted), CB-Metadaten, `equip-g` (Daten ab 2029)
- **Blockade:** keine (Routen messbar; Terms/Verdikt bei Mountain)
- **Braucht:** eine offene Route (z. B. BGI `https://bgi.obs-mip.fr/data-products/gravity-databases/absolute-gravity-data/`) enumerieren und als `url`/`origin`/`compiler` in `phi/sources.φ` registrieren; Terms/field bei Mountain

### JUICE — Termin 2026-11-01
- **Status:** termin · **Bindung:** termin:2026-11-01
- **Trigger:** 2026-11-01
- **Lage:** (gemessen 2026-10-18 via `phi/sources.φ:27245`) Wartestand; der Offen-Riss ist durch `R-2026-10-17-i` (`d3ce6d91e`) entschieden (beide Zähl-Stile) und hier gelöscht
- **Blockade:** Termin
- **Braucht:** nach dem 2026-11-01 den Flyby-Punkt neu messen

## An mountain
Origin: 361
- `xfer:mycelium.ses_ec618129fffeLFawr4V3pQKpOl.5~4820be44` FITS-interner Subset-Leser (`format fits` + `range`/`window` via `fetch_range`, gegen den `.parts`-Shard-`sha256`; Test auf FITS-Fixture).

## Burn: open 0.0 · close nicht gemessen (db) · cap 0.15 · closed 0 · Grund: UCAR/L1 extern, Surveys offen, JUICE Termin; Offen-Riss entschieden.
