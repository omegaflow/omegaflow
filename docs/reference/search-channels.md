<!--
  title: Suchkanäle — gemessener Stand (2026-10-19)
  class: ref
  date: 2026-10-19
  sha256: d73f3cac6cf89148ecaa75af9a9d21336937ffad7f18c423e181f327ae95d93e
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
(200, aber intermittierend 429), `--ads`, `--scopus`, `--datacite` (mit **Jahr** seit `763adf39e`).

## Account-Problem, kein fehlender Arm

Key liegt vor, Provider verweigert trotzdem:

| Kanal | HTTP | Ursache |
|---|---:|---|
| `--brave` | **402** | Abrechnung/Konto |
| `--tavily` | **432** | Plan-Limit |
| `--searchapi` / `--serpapi` / `--consensus` | **429** | Quota |
| `--googlecse` | **400** | „API key not valid" (Flag heisst `--googlecse`) |
| `--gemini` | **429** | Quota |

**Nachgemessen 2026-10-20** (Binary `target/release/archive_search`, sha256
`50f002ee7b16…`, 01:27): unverändert 402/432/429/429/429/429/400. Der Zustand
ist ein Konto-, kein Code-Problem.

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

## Werkzeug-Riss (2026-10-20)

`bin/archive_search` meldet beim Start einen Manifest-Riss:

```
remote c580b311… vs downloaded 5b4c0b53…, keeping installed d6d8f744…
```

Die installierte Release-Binary ist älter als der veröffentlichte Manifest-Eintrag
(`tools-build` läuft). Der Suchlauf selbst ist davon nicht betroffen — die oben
gemessenen Codes kommen aus dem lokal gebauten Binärbaum. Sobald `tools-build`
landet, verschwindet der Riss; bis dahin ist `bin/archive_search` für Messungen
nicht die Wahrheit, `target/debug/archive_search` schon.

## Mit Key, lebend — 2026-10-20 nachgemessen (Korrektur)

Die frühere Zeile „die breiten Engines sind tot" war **falsch**. Nicht gemessen
waren vier Key-Arme, die antworten:

| Kanal | Befund | Charakter |
|---|---|---|
| `--serper` | **200**, Treffer mit `position` | **Google SERP** (serper.dev), nicht serpapi |
| `--exa` | **200**, Treffer mit `url`/`title` | semantische Suche |
| `--perplexity` | **200**, Antworttext | Antwort + Quellen |
| `--linkup` | **200**, `url`/`title`/`description` | Web-Suche |

Damit ist die Behauptung „es gibt keine breite Suche" widerlegt: `--serper` ist
Google, `--exa` ist ein anderer Index. Beide wurden im Alternativen-Bericht
**nicht benutzt** — der Fehler lag nicht am Konto, sondern an der Auswahl.

## Browser-Route

`https://html.duckduckgo.com/html/?q=…` im Browser liefert **CAPTCHA**
(„Select all squares containing a duck"). Die Browser-Route taugt für
**Seiten** (JS, Login, PDF), nicht als Suchmaschine. Wer breit suchen will,
nimmt `--serper`/`--exa`, nicht den Browser.

## Was die Köder-Suche damit fand

| Köder | Kanal | Fund |
|---|---|---|
| `"bright_t31,frp,daynight"` | searxng | GitHub, Pastebin, Kaggle, EUMETSAT, pyronear |
| `DHM Nepal river discharge data download csv` | serper | `dhm.gov.np/hydrology/river-watch`, **ICIMOD Streamflow Prediction Tool Nepal**, UN-Hydromet-Diagnostik |
| `FUGIN Nobeyama CO survey FITS cube archive download` | exa | `jvo.nao.ac.jp/portal/nobeyama/fugin.do`, `nro-fugin.github.io/release/`, NRO-Datenrelease |

## SearXNG-Rotation: gemessen, nicht erhofft (2026-10-20)

Das Muster ist `https://searx.space/data/instances.json` (352 KB, 92 Instanzen,
davon 80 `network_type: normal` mit HTTP 200). Alle 75 Kandidaten wurden gegen
`/search?q=test&format=json` geprüft:

| Befund | Anzahl |
|---|---:|
| **JSON, 200, `results`** | **1** (`search.lumy.live`) |
| HTTP 200, aber **HTML** (JSON abgeschaltet) | 8 |
| 429 (Rate-Limit, auch bei serieller Probe) | 53 |
| 403 / 418 / 404 / 302 / 000 | 13 |

**Ergebnis: Rotation über öffentliche SearXNG-Instanzen ist so nicht möglich.**
Aktuelle Versionen liefern `format=json` nur nach ausdrücklicher Freigabe; die
HTML-Route ist rate-limitiert oder zeigt Bot-Prüfungen. Der Arm trägt deshalb
weiter nur `search.lumy.live` — und `OMEGAFLOW_SEARXNG` akzeptiert jetzt eine
**kommagetrennte** Liste, damit eine eigene Instanz (mit `json` in
`search.formats`) als Rotationskette dienen kann.

Korrektur: die frühere Zeile „vier Instanzen antworten mit JSON" war falsch —
nur eine antwortet mit JSON, die anderen mit HTML.

## Marginalia — Spec gelesen, Arm pfadrichtig, Dienst hängt (2026-10-20)

Spec: `https://www.marginalia.nu/marginalia-search/api/` —

```
GET https://api.marginalia.nu/{key}/search/{query}?index=0&count=10
```

Key ist ein **Pfadsegment** (`public`), 503 heißt geteiltes Rate-Limit. Der Arm
nutzt `https://api.marginalia-search.com/public/search/{query}?count=10` — dieselbe
Pfadform, neuer Host, `index` fehlt (Default 0).

**Messung:** der Such-GET antwortet auf **beiden** Hosts nicht innerhalb von 40 s;
mit 90 s Frist kommt nach **60,17 s** ein **HTTP 504 Gateway Timeout** (167 B).
Der Wurzel-HEAD antwortet sofort. Das ist kein 429 und kein „kein Netz",
also läuft der Arm als **pending**, mit der ehrlichen Meldung
„no answer within 40 s (timeout or no network)".

Nachgemessen 2026-10-20: unverändert **HTTP 504** nach **60,37 s**
(167 B); der Wurzel-Host antwortet weiter sofort, der Such-GET nicht.
Der Arm bleibt extern blockiert, nicht ungebaut.

**Lizenz:** die Ergebnisse stehen laut Spec unter **CC-BY-NC-SA 4.0** — non-commercial.
Damit ist Marginalia auch im Erfolgsfall **kein CDN-Kanal**; eigene Keys gibt es
per Mail an `kontakt@marginalia.nu`.
