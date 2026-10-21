<!--
  title: Zwillinge per Signatur — acht key-gebundene Arme (2026-10-20)
  class: survey
  date: 2026-10-20
  sha256: 7dc663bfd3230fa7cd29f81658d266aa1feb680c8f1d49cb3d10c425d3f4bdd7
  status: live
  see-also: docs/blatt/blatt-2026-10-19-entscheidung-17-lizenzarme.md docs/concepts/search-channels.md
-->
# Zwillinge per Signatur — acht key-gebundene Arme

**Methode:** nicht nach Arm-Namen suchen, sondern nach der **Signatur** — Spaltenkopf,
Variablenname, Produkt-ID, Dateiendung, Byte-Muster. Werkzeuge: `bin/sniff <datei> bait`
(Suchkette aus einer Datei), `archive_search --serper|--exa|--searxng` (die drei
lebenden Kanäle), `bin/sniff <url>` und `pattern=<hex>` für den Kandidaten.

**Regel:** Ein Zwilling ist nur dann eine Öffnung, wenn er (a) dieselbe Datei bzw.
dasselbe Produkt trägt **und** (b) die Zugangswand nicht erbt. Trägt er nur dieselbe
**Messung** in anderen Daten, ist er ein **neuer Arm**, kein Ersatz.

## Ergebnis je Arm

| Arm | Signatur | Kandidat | Wand | Verdikt |
|---|---|---|---|---|
| `substorm` | CSV-Kopf `Date_UTC,MLT,MLAT,GLON,GLAT` | SuperMAG selbst, `list=newell` | **keine** (keyless, 200/3 439 B) | **Token-Fehler**: `auth-required` → `no-redistribution` |
| `icrf3` | `ICRF3-SX`, Spalten 1–15 | `deepspace.jpl.nasa.gov/…/catalog-fixed.txt` | keine (200/637 566 B) | **freigegeben**: 4 063 Zeilen, `PD-USGov` |
| `ieeg` | iEEG/BIDS | OpenNeuro **ds004100** (HUP) | keine; `"License":"CC0"` | **neuer Arm** `hup_ieeg` — andere Daten |
| `pradan_ch2` | Chandrayaan-2 CH2 | Mendeley **10.17632/6mmd96j5bg** | keine; `rightsIdentifier: cc-by-4.0` | **neuer Arm** — Teilmenge, Datei ungeprüft |
| `babamul` | `candid/ra/dec/magpsf` | ALeRCE-API | keine (200) | **neuer Arm**; **kein** Lizenztext in den Docs gefunden |
| `demeter` | 6-Float-Record | IRAP-Quicklook | keine (200/1 651 155 B) | gleiche Messung, anderer Container; kein Lizenztext |
| `tns` | Objektliste | TNS-Web-CSV | keine | **Scrape**; API bleibt 401 — keine Lizenz |
| `ilrs_npt` | NPT-CRD | CDDIS / EDC | **Earthdata-Login** bzw. 403 | Zwilling erbt die Wand |

## Belegte Lizenzen (am Datensatz selbst, nicht an einer Landing-Page)

- **OpenNeuro ds004100:** `dataset_description.json` aus dem Snapshot →
  `"License": "CC0"` (13,2 GB, 43 Subjekte).
- **Mendeley 10.17632/6mmd96j5bg:** DataCite-Record → `rightsIdentifier: cc-by-4.0`,
  `rightsUri: …/licenses/by/4.0/legalcode`.

Beides sind **neue** Arme: die HUP-Daten sind nicht die iEEG-Rohdaten der UPenn, und
die Mendeley-Datei ist eine Teilmenge (zwei Tage), nicht der PRADAN-Bestand.

## Nicht belegt

- **ALeRCE:** API antwortet (200, 647 B), aber weder `alerce.online` (403 für curl)
  noch `alerce.readthedocs.io` nennen eine Lizenz. Ohne Token kein Arm.
- **IRAP-Quicklook (DEMETER):** Container passt (33-B-Records), aber die Seite trägt
  keinen Lizenztext.
- **TNS-Web-CSV:** liefert ohne Key Zeilen, ist aber eine HTML-Oberfläche, kein Release.

## Was daraus folgt

1. `substorm` war der einzige echte **Token-Fehler** — behoben (`no-redistribution`).
2. `icrf3` ist über JPL freigegeben (siehe Blatt, Nachtrag 8).
3. `ieeg` und `pradan_ch2` bekommen je einen **neuen** freigegebenen Arm (CC0 bzw. CC-BY-4.0),
   ohne den alten Arm zu ersetzen.
4. `babamul`, `demeter`, `tns`, `ilrs_npt` bleiben, wie sie sind — mit benanntem Grund.
