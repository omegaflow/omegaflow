<!--
  title: Die Köder-Kette — Archäologie (2026-10-21)
  class: concept
  date: 2026-10-21
  status: live
  see-also: docs/concepts/koeder-formen.md docs/concepts/search-channels.md docs/concepts/tools-map.md
-->
# Die Köder-Kette — Archäologie

**Frage:** Was war gebaut, was war verdrahtet, was war nur begonnen — für die
Kette **Köder → Suche → Zwilling/nah → Merge**? Gemessen an der Git-Historie und
am heutigen Binärbaum, nicht an der Erinnerung.

## Zeitachse (2026-10-20, ein Tag)

| Zeit | Commit | Was dazukam |
|---|---|---|
| 02:34 | `4ef21729d` | `sniff`: Bait-Modus, HTTP-Felder, SearXNG-Messung |
| 09:06 | `030dd1e11` | `sniff`: Wert-Köder (`bait values`) — Zahlen und Float-Bytes |
| 09:13 | `df464c33a` | **`weave`**: Anker-Merge über Tabellen, Reihenfolge egal, Teilmengen erlaubt |
| 11:03 | `75f6a51fd` | **`weave`**: Konflikt + DAG + Herkunftskarte; **`bin/find`**: Signatur → Suche → sniff |
| 11:44 | `86b522d75` | `sniff`: Chunk-Hashes, NetCDF/FITS/HEALPix-Signale, Köder aus Prosa |
| 12:07 | `c044cedc1` | `bait`/`find`: Köder aus Bild und PDF, Toleranzprüfung, **Zwillingsurteil** |
| 12:24 | `110f297d6` | `weave`: **mirror-of**-Kante im DAG |
| 14:01 | `083f57127` | `find`: Konjunktion mehrerer Werte und Suchkontext; `bait values` aus Prosa |
| 14:34 | `518a545e6` | `sniff`: FITS-Identität (`TELESCOP`/`INSTRUME`/`DETECTOR`/`FILTER`/`BAND`/`EXPTIME`) |
| 14:58 | `a9f3ae606` | `sniff`: NetCDF-Units aus dem strukturierten Header |
| 15:11 | `a85c34630` | `bait`: Unsicherheitspaare (`0.548 ± 0.009`) |
| 17:40 | `9a7af4e9b` | `sniff`: NetCDF4-Attribute aus HDF5 |
| 18:47 | `74c4ae557` | `sniff`: gepackte NetCDF4-Werte (`--values`) |
| 21:19 | `7d6eb96` | **Ratsdatei gelöscht** (Doppel-ID) — die Formen-/Köder-Analyse verschwand |
| 2026-10-21 06:26 | `6fd3e9945` | `bait`: FITS-Header als Text, Redirects folgen |

## Was die Kette heute kann

| Schritt | Werkzeug | Kann | Verdrahtet? |
|---|---|---|---|
| Köder aus Prosa/Specs | `bin/bait` | Text, PDF (Textschicht/OCR), Bild (OCR), **FITS-Header**; gibt nur `bait …`-Zeilen | ja |
| Köder aus einer Quelle | `bin/find` | `--header`, `--value` (±Toleranz), `--hex`, `--fits`, `--context`, `--pdf`; sucht, snifft jeden Kandidaten, Urteil **MATCH / ZWILLING / NAH / kein-match**; `--near <abs\|N%>` | ja (`bin/find`) |
| Merge | `bin/weave` | `--on <anker>` über ≥2 Tabellen; `__conflict`-Spalte, `.map`-Herkunftskarte, `.edges`-DAG inkl. `mirror-of`, `--terms datei=token` | **erst seit 2026-10-21** (`bin/weave`) |
| Kandidaten-Dedup | `bin/leads_merge` | Leads gegen `phi/sources.φ`; nur unregistrierte Hosts | **erst seit 2026-10-21** (`bin/leads_merge`) |
| Suche | `bin/archive_search` | 100+ Arme, `--all`, `--verdict` (direct → proton → wayback), `--sniff`, `--cc`, `--wayback` | ja |

Beispielmerge (gemessen):

```
$ bin/weave --on id --out /tmp/merged.csv /tmp/t1.csv /tmp/t2.csv
weave: 2 Quellen, 3 Zeilen, 2 Spalten + Anker -> /tmp/merged.csv
       (0 Konfliktzeilen; Karte /tmp/merged.csv.map, Kanten /tmp/merged.csv.edges)
```

`merged.csv` trägt `id,a,b,__conflict`; `.map` nennt je Ausgabespalte Quelle,
Spalte und Lizenz; `.edges` trägt `from → to: contributes|materializes` plus
`mirror-of` für dieselbe Sache an zwei Orten.

## Die Lücke, die blieb — und was davon gebaut ist

1. **`find --jsonl` → `weave --from-find` ist gebaut.** `find` schreibt je
   Kandidat eine JSON-Zeile; `weave --from-find <datei> --out <datei>`
   merged über den Anker `url`. Gemessen 2026-10-21:
   `find: 1 Kandidat, 1 Roh-Link, 1 Treffer` → `weave: 1 Quelle, 2 Zeilen,
   7 Spalten + Anker` (mit `--no-terms`). Die frühere Aussage „nicht
   verkettet“ war Doku-Drift.
2. **`weave` war unsichtbar.** Es lag in `target/release/weave`, hatte keinen
   `bin/`-Wrapper und keinen Eintrag in `docs/concepts/tools-map.md`; wer es
   nicht aus dem Quelltext kannte, fand es nicht. Seit 2026-10-21 gibt es
   `bin/weave` und `bin/leads_merge`.
3. **Die Formen-/Köder-Analyse lag nur in der gelöschten Ratsdatei.** Geborgen
   als `docs/concepts/koeder-formen.md`; ohne das wären die Bedingungen
   (Signal/Anker/blind je Form) nirgends festgehalten.
4. **`weave` prüft `--terms` nur, wenn es angegeben ist.** Ohne `--terms` und
   ohne `--no-terms` bricht es ab; mit `--no-terms` steht in der Karte `unset`
   — die Lizenz ist dann nicht Teil des Merges.
