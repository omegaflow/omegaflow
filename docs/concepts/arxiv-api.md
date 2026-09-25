<!--
  title: arXiv — Zugriffswege (API, OAI-PMH, Bulk)
  class: concept
  date: 2026-09-25
  sha256: 7edd1e65f21988e8e5ee9226795f56e9287460168f858f9bd8c5d26883898362
  status: live
  see-also: docs/concepts/tools-map.md docs/handover/handover-2026-09-25-mountain-folge165.md
-->
# arXiv — Zugriffswege (API, OAI-PMH, Bulk)

Gemessen 2026-09-25 aus `info.arxiv.org/help/api/{index,basics}.html` und
`info.arxiv.org/help/bulk_data.html` (Inhalt gelesen via Browser, nicht Pixel).
Dieses Blatt hält die dokumentierten Wege; es behauptet keinen Live-Zustand.

## API (Realzeit)

- Endpunkt: `http://export.arxiv.org/api/query?search_query=<q>&start=<n>&max_results=<m>`
- Methode: HTTP **GET oder POST**; Antwort **Atom 1.0 XML**; kein Client nötig.
- Zweck laut Doku: Echtzeit-Metadaten + Suchmaschine, nicht der Massen-Download.

## Bulk-Metadaten

- **OAI-PMH ist der ausdrücklich bevorzugte Weg** für Massen-Download und
  Fortschreibung: täglich aktualisiert, für Harvesting gebaut.
- RSS-Feeds: neue Updates je Tag, primär für Menschen (XML, maschinenlesbar).

## Bulk-Volltext

- **Kaggle** — der vollständige maschinenlesbare Datensatz (Titel, Autoren,
  Kategorien, Abstracts, PDFs).
- **Amazon S3** — verarbeitete PDFs und Quellen.
- Hinweis: die Default-Lizenz gewährt arXiv kein Copyright-Weiterverteilungsrecht;
  Indizes/Tools müssen auf arXiv zurückverlinken.

## Custom Harvesting

- Dediziertes **`export.arxiv.org`** für programmatischen Zugriff (schont den
  interaktiven Haupt-Host).
- Rate laut Doku: Bursts mit **4 Requests/Sekunde, 1 s Schlaf je Burst**;
  robots-Seite beachten.

## Terms / Pflichten

- Danksagung in jedem Produkt: *„Thank you to arXiv for use of its open access
  interoperability."*
- Namen/Logos/URLs/Farben von arXiv nicht als Endorsement verwenden.
- Kommerzielle Projekte: Affiliate-/Review-Pflicht.

## Beobachteter Zustand (omegaflow)

- (gemessen 2026-09-25) die Query-API `export.arxiv.org/api/query` antwortet
  **HTTP 406 genau dann, wenn `start + max_results > 2`** — unabhängig von
  User-Agent, `http`/`https`, `sortBy`; reproduzierbar. `start=0&max_results=2`
  → 200, `start=0&max_results=3` → 406, `start=2&max_results=2` → 406. Über die
  Query-API sind also **nur die ersten zwei Treffer** abrufbar; Paging ist nicht
  möglich.
- Die drei Doku-Seiten nennen **keine API-Migration/Deprecation**. Der 406 ist
  ein Edge-/WAF-Fenster, keine angekündigte Ablösung.
- **OAI-PMH ist der offene Bulk-Weg:** `https://export.arxiv.org/oai2`
  (`verb=Identify`, `verb=ListMetadataFormats`, `verb=ListRecords&metadataPrefix=arXiv&set=…`)
  → HTTP 200, keine Fensterkappung.
- Werkzeug: `archive_search --arxiv` klemmt das Fenster auf `ARXIV_QUERY_WINDOW = 2`
  (`tools/utils/src/bin/archive_search/net.rs`) und benennt den Zustand; die alte
  „server-side, fresh queries rejected"-Meldung war ungemessen und ist korrigiert.
- Der offene Punkt `arxiv HTTP 406` lebt in `handover-2026-09-25-mountain-folge165.md`.
