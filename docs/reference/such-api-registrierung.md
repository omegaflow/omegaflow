# Such-APIs: Registrierung und Key-Seiten

Aufbereitete HTML-Seite: [`such-api-registrierung.html`](such-api-registrierung.html).
Sie hat pro Zeile eine Status-Checkbox (`offen → registriert → authentifiziert →
declined`), die den eigenen Stand lokal im Browser (`localStorage`) merkt.

Gemessen 2026-10-21 mit `curl -sL -o /dev/null -w '%{http_code}'` und
Browser-User-Agent. Ein `403`/`429` heißt erreichbar, aber Bot-Schutz —
kein toter Host. Ein `000` heißt TCP/TLS tot.

Die vier neuen keyless Arme in `archive_search` sind:

| Arm | Endpoint | Messwert |
|---|---|---|
| `--hn` | `https://hn.algolia.com/api/v1/search` | 200 |
| `--openverse` | `https://api.openverse.org/v1/images/` | 200 |
| `--nasa-images` | `https://images-api.nasa.gov/search` | 200 |
| `--artic` | `https://api.artic.edu/api/v1/artworks/search` | 200 |

## Web- und SERP-Suche

| Anbieter | Registrierung / Key | Messwert |
|---|---|---|
| Mojeek | https://www.mojeek.com/services/search/web-search-api/ | 200, Key per Plan |
| Yandex Search API | https://console.yandex.cloud/ | 307, Service-Account + IAM |
| Brave Search | https://api.search.brave.com/app/keys | 403 Bot, gueltig |
| Exa | https://dashboard.exa.ai/api-keys | 429 Bot, gueltig |
| Tavily | https://app.tavily.com/home | 200 |
| Serper (auch Scholar) | https://serper.dev/ | 200 |
| SerpAPI | https://serpapi.com/manage-api-key | 200 |
| SearchApi | https://www.searchapi.io/ | 200 |
| Firecrawl | https://www.firecrawl.dev/app/api-keys | 200 |
| Linkup | https://app.linkup.so/ | 200 |
| Consensus | https://consensus.app/ | 200 |
| Perplexity Sonar | https://www.perplexity.ai/settings/api | 200 |
| Gemini | https://aistudio.google.com/app/apikey | 200, Login; Key da, 429 Quota |
| Kagi | https://kagi.com/settings?p=api | 200 |
| You.com | https://api.you.com/ | 200 |
| Naver | https://developers.naver.com/apps/#/register | 200 |
| Wolfram Alpha | https://developer.wolframalpha.com/access | 200 |
| Diffbot | https://www.diffbot.com/dev/home/ | 200 |
| Telnyx Web Search | https://portal.telnyx.com/ | 200 |
| StepFun | https://platform.stepfun.com/ | 200 |
| Parallel AI | https://parallel.ai/ | 200 |
| TinyFish | https://www.tinyfish.ai/ | 200 |
| Bright Data | https://brightdata.com/ | 200 |
| Olostep | https://www.olostep.com/ | 200 |
| Microsoft Graph | https://entra.microsoft.com/ | 200, OAuth-App |
| PipesHub | self-hosted | kein Key |

## Wissenschaft und Bibliothek

| Anbieter | Registrierung / Key | Messwert |
|---|---|---|
| Scopus (Elsevier) | https://dev.elsevier.com/apikey/manage | 200 |
| NASA ADS | https://ui.adsabs.harvard.edu/user/settings/token | 202 |
| CORE | https://core.ac.uk/services/api | 200 |
| BASE | https://www.base-search.net/about/en/contact.php | 200, Antrag |
| Mendeley | https://dev.mendeley.com/myapps.html | 200, OAuth |
| Springer Nature | https://dev.springernature.com/ | 200 |
| Clarivate Web of Science | https://developer.clarivate.com/apis/wos | 200 |
| Dimensions | https://www.dimensions.ai/api/ | 200, Vertrag |
| IEEE Xplore | https://developer.ieee.org/ | 200 |
| ACM Digital Library | https://developer.acm.org (000, TCP tot); https://dl.acm.org/ (403) | kein Self-Serve |
| Lens.org | https://www.lens.org/ | 200, API hinter Login |
| Semantic Scholar | https://www.semanticscholar.org/product/api | 200, Key-Antrag |
| Europeana | https://www.europeana.eu/en/apis | 200, Key-Antrag |
| DPLA | https://dp.la/ | 200 |
| NYPL | https://api.repo.nypl.org/ | 200 |
| Harvard Art Museums | https://harvardartmuseums.org/collections/api | 200 |
| Rijksmuseum | https://data.rijksmuseum.nl/ | 200 |
| Smithsonian / US-Gov | https://api.data.gov/signup/ | 200 |

## Code und Infrastruktur

| Anbieter | Registrierung / Key | Messwert |
|---|---|---|
| GitHub | https://github.com/settings/tokens | 200 |
| GitLab | https://gitlab.com/-/user_settings/personal_access_tokens | 403 Bot, gueltig |
| Azure DevOps | https://learn.microsoft.com/en-us/azure/devops/organizations/accounts/use-personal-access-tokens-to-authenticate | 200 |
| Shodan | https://account.shodan.io/ | 200 |
| OpenCellID | https://opencellid.org/ | 200 |
| Global Fishing Watch | https://globalfishingwatch.org/our-apis/ | 200 |
| AlphaXiv | https://www.alphaxiv.org/ | 200 |
| Marginalia | https://search.marginalia.nu/ | 200 |
| MCP360 Google Scholar | https://www.mcp360.ai/ | 200 |
| Patlytics Patent Search | https://patlytics.ai/ | 200 |
| Bohrium Scholar | https://bohrium.dp.tech/ | 200 |
| ZoomInfo | https://www.zoominfo.com/ | 200, Enterprise/OAuth |

## Standardansicht: kostenlos und noch ohne funktionierenden Key

Die HTML-Seite blendet standardmäßig zwei Gruppen aus (per Toggle wieder sichtbar):

- **kostenpflichtig/vertragsgebunden** (33): Mojeek, Yandex, Brave, Exa, Tavily,
  Serper, SerpAPI, SearchApi, Firecrawl, Linkup, Consensus, Perplexity, Kagi,
  You.com, Wolfram, Diffbot, Telnyx, StepFun, Parallel, TinyFish, Bright Data,
  Olostep, Scopus, Clarivate, Dimensions, IEEE, ACM, Lens, Shodan, MCP360,
  Patlytics, Bohrium, ZoomInfo.
- **bereits funktionierend** (13, am 2026-10-21 mit Treffern gemessen): ADS,
  AlphaXiv, BASE, CORE, Firecrawl, Linkup, Serper, Perplexity, Scopus, Shodan,
  GitHub, Semantic Scholar, PipesHub.
- **Kreditkarte im Signup** (9): Gemini, Yandex, Perplexity,
  Kagi, You.com, Bright Data, Shodan, Telnyx, Parallel AI. Google verlangt für
  das Cloud-Projekt eine Karte; Yandex und die übrigen sind ohnehin
  kostenpflichtig. Der Filter wirkt auch, wenn man „kostenpflichtige aus“ abwählt.

Sichtbar bleiben 15 freie Dienste ohne Kreditkarte und ohne funktionierenden
Key: Naver, Microsoft Graph, Mendeley, Springer Nature, Europeana, DPLA, NYPL,
Harvard Art Museums, Rijksmuseum, api.data.gov, GitLab, Azure DevOps,
OpenCellID, Global Fishing Watch (404 Route), Marginalia (haengt).

Die Keyless-Sektion bleibt sichtbar: sie braucht keine Registrierung.

## Ohne Registrierung

OpenAlex, Crossref, DataCite, Zenodo, arXiv, Europe PMC, PubMed,
HN Algolia, Openverse, NASA Images, Art Institute of Chicago, Wikipedia,
Library of Congress, Internet Archive, StackExchange, GitHub-Suche
(rate-limited ohne Token), Fink/ALeRCE/ANTARES/IRSA.

`archive_search --apis "<begriff>"` durchsucht apis.io/apis.guru und findet
die lange Liste darüber hinaus (ACM, GitLab, Azure DevOps, DSpace/CRIS,
Serper Scholar, MCP360, Patlytics, Bohrium, ZoomInfo, PoetryDB).
