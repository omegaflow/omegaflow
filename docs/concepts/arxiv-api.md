<!--
  title: arXiv — Zugriffswege (API, OAI-PMH, Bulk)
  class: concept
  date: 2026-09-25
  sha256: a621c580ea041b1d5ddc96031f31c7c89c01736145acdf16d141ae5638e7dfbd
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

- (gemessen 2026-09-25) der ungecachte Query-Pfad `export.arxiv.org/api/query`
  antwortet mit **HTTP 406, leerem Body**, UA-unabhängig; gecachte Queries 200.
- Die drei Doku-Seiten nennen **keine API-Migration/Deprecation** — der
  dokumentierte Endpunkt bleibt bestehen. Der 406 ist damit Edge-/Anti-Crawl-
  Verhalten, keine angekündigte Ablösung. Der belastbare Zweitkanal für den
  Massen-Zugriff ist **OAI-PMH**.
- Der offene Punkt `arxiv HTTP 406` lebt in der Mountain-Übergabe (folge164).
