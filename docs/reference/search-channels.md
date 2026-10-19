<!--
  title: Suchkanäle — gemessener Stand (2026-10-19)
  class: ref
  date: 2026-10-19
  sha256: f166832bfd8940b99a67881de896f252e7b7438c423755e8d0e9bbfdadcb9dee
  status: live
  see-also: docs/concepts/tools-map.md docs/surveys/survey-2026-10-19-paper-forschungslage.md
-->
# Suchkanäle — gemessener Stand

Eine Zeile je Kanal, gemessen am 2026-10-19 (`archive_search`, Begriff „solar wind"). Kein
Kanal wird als „geht" geführt, weil er im Hilfetext steht — nur, weil er geantwortet hat.

## Keyless, laufen

| Kanal | Befund |
|---|---|
| `--openalex` | 200, kappt bei 252 |
| `--crossref` | 200, kappt bei 26 — **Jahr seit `1c569e8e1` in der Ausgabe** |
| `--mwmbl` | 200 |
| `--marginalia` | 200 über das Tool (JSON); **HTML-Route 0 organische Treffer** |
| `--wikipedia` | 200 |
| `--github` | 200 |
| `--crates` | 200 |
| `--internet-archive` | 200, mit Jahr |
| `--wayback` / `--cc` | 200 (Common Crawl seit `d6a6524f0`: collinfo-Index + `*.host`) |
| `--oaipmh` (arXiv) | **200 durchgehend**, auch während arXiv-429 |

## Mit Key (Key vorhanden, gemessen 200)

`--base` (400-Mio-Index, **mit Jahr**), `--core`, `--openaire`, `--semanticscholar`
(200, aber intermittierend 429), `--ads`, `--scopus`, `--datacite`
(**Riss:** `publicationYear` steht im Body, wird aber nicht ausgegeben).

## Account-Problem, kein fehlender Arm

Key liegt vor, Provider verweigert trotzdem:

| Kanal | HTTP | Ursache |
|---|---:|---|
| `--brave` | **402** | Abrechnung/Konto |
| `--tavily` | **432** | Plan-Limit |
| `--searchapi` / `--serpapi` / `--consensus` | **429** | Quota |
| `--google-cse` | **400** | ungültige Konfiguration |
| `--gemini` | **429** | Quota |

Das ist **keine Code-Arbeit**. Ein Key öffnet Zugang, keine Lizenz — und ein 402 ist kein
leerer Treffer.

## Bewusst verworfen

- **Mojeek**: 200 mit **Captcha**, 0 Treffer.
- **Marginalia-HTML**: 302→200, 0 organische Treffer (JSON bleibt).
- **Google Books**: Arm existiert nicht; keyless, aber Tagesquota (429 in 2/2 Läufen) —
  erst bauen, wenn eine Aufgabe ihn braucht.

## arXiv

`--arxiv` war in **35 von 35** Paper-Läufen 429 — aber transient: gemessen 19:16 429,
19:19 vier× 200, 19:20 429, nach ~15 min wieder 200. Kein `Retry-After` (das
`Retry-After: 0` gehört zum 301 http→https). Seit `1c569e8e1`: 3-s-Takt,
`Retry-After` respektiert (Deckel 30 s). **Bulk über `oaipmh.arxiv.org`.**

## Mehrkanal

Seit `46b82ec95` fahren mehrere Flags in einem Aufruf wirklich mehrere Kanäle
(`Vec<Mode>`, je Kopfzeile). Vorher gewann die letzte Flag — ein gemessener Defekt,
kein Bedienfehler.
