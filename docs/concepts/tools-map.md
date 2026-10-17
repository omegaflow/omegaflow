<!--
  title: Tools-Map — was jedes Werkzeug kann, was es kostet, wer es darf
  class: concept
  date: 2026-10-17
  sha256: 932bd609ad46e39be8f9cae8996283d1ebe26ab353500c6523de4255d3951dd3
  status: live
  see-also: AGENTS.md
-->
# Tools-Map — was jedes Werkzeug kann, was es kostet, wer es darf

Eine Karte statt dreier sich widersprechender Stellen (gemessen 2026-09-16,
Release-Binaries, Einzellauf je Messung). `osearch` ist gestrichen — es gehört
nicht zum omegaflow-Werkzeugsatz; seine generischen Fähigkeiten (`--count`,
`--case`, `--path`) sind in `archive_search` eingeflossen; `--hexcolor` bleibt
draußen (spezifische Anwendung, kein Such-Werkzeug).

## Die eine Regel (ersetzt alle früheren Inhaltssuche-Vorschriften)

- Inhalt im lebenden Baum: `archive_search <kws> --root <dir>` (auf PATH) oder
  `sgrep [-i] <pattern> [dir]` — `sgrep` ist case-sensitiv ohne `-i`.
- Pfade/Dateinamen: `archive_search --index [<query>] [--path] [--kind] [--sort]`.
- **Code** durchsuchen (Rust, JS/TS, WGSL, Python): `archive_search --sourcegraph "<pattern>"` — **keyless**, echte Code-Zeilen (`repo/path/line/match`). **`--github` findet nur Repositories, *keinen* Code** (gemessen 2026-10-15); `--grepapp` = 429 + nur Metadaten.
- `grep`, `ls`, `cat` **in bash** sind Verstöße (der Zähler gehört ins Handover).
  Das `Grep`-Tool von OpenCode ist erlaubt; bash-`grep` ist es nicht. Die
  Leitbefehle (`grep`/`ls`/`cat` am Kommandoanfang) sind in `opencode.json`
  **strukturell verweigert** (`"grep *": "deny"`); eine Pipe (`cmd | grep`) fällt
  weiter unter die Regel und den Zähler.
- Neu gemessen 2026-09-16 11:00 (opencode.db, Fenster 2026-09-13 09:49 →
  2026-09-16 11:00, gezählt je bash-Kommando): 206 bash-Aufrufe; `grep` 8 (alle 8
  strukturell verweigert — 0 ausgeführt), `ls` 6, `cat` 3; dagegen `curl` 44,
  `sgrep` 37, `archive_search` 20, `sfetch` 10; Playwright-Tool 14, bash-Playwright
  2. Reibung: 18 verweigerte Aufrufe (17 bash, 1 read) — jeder `grep`/`ls`-Griff
  kostete einen Turn, bevor die Sperre griff.
- `curl` 44 > `archive_search` 20 ist kein Verstoß: die 44 tragen REST-APIs
  (TAP/ERDDAP/GitHub), für die `archive_search` keinen Modus hat. Für Quellen und
  JS-Seiten bleibt `archive_search <quelle>` / `--playwright` der erste Zug.
- Das `--help` des Werkzeugs selbst ist die kanonische Schnittstelle — alle
  Aufrufe und Flags stehen dort (`sgrep --help`, `archive_search --help`,
  `git_safety`, `omega_sh help`, `smail`, `register_lookup`). Die
  Interfaces-Sektion unten spiegelt sie; bei Widerspruch gilt das `--help`.

## Gemessen — Inhaltssuche (Query `purpleair`, `src/`)

| Werkzeug | Zeit | Treffer | Urteil |
|---|---|---|---|
| `archive_search --root src/` | 0,051 s | korrekt | kanonisch |
| `sgrep -i` | 0,041 s | korrekt | schlank |
| `sgrep` (default) | 0,075 s | 0 (case-sensitiv) | ohne `-i` fehlt Großschreibung |
| `grep -rni src/` | 0,105 s | korrekt | auf Repo-Wurzel: Hänger (`target/`+`data/`) |

## Gemessen — `sgrep` Modi

`default` 0,075 s · `-i` 0,071 s · `-l` 0,067 s · `-c` 0,067 s · `-g '*.rs'` 0,052 s.

## Gemessen — `archive_search` lokale Modi

| Modus | Zeit | Bemerkung |
|---|---|---|
| `--root` | 0,051 s | Inhalt, Ranking, Caps |
| `--index` | 0,023 s | Pfade, lebender Walk |
| `--git <query>` | 9,98 s | History-Suche — teuer, gezielt nutzen |
| `--leads <kw>` | 0,85 s | Kandidaten-Homes abzüglich Registern |
| `--mft <device>` | 0,003 s | benannter Exit ohne NTFS-Device |
| `--sniff <url>` | 0,16 s | magic bytes + sha256 |
| `--verdict <url>` | 20,5 s | Reichweiten-Leiter mit Fallbacks — nur für Reichweite; Stufe 1/2 seit 2026-09-28 als 1-Byte-`--range` (erreichbares Asset in ms, vorher 3×30 s Timeout), die Gesamtzeit dominiert Stufe 3/CDX |
| `--playwright <url>` | 2,97 s | Browser-Render |
| `--pdf-image <file\|url>` | — | hebt eingebettete JPEG/PNG/JP2 aus einem PDF (kein Rasterizer); `--out <dir>` sonst Temp-Verzeichnis |
| `--pdf-text <file\|url>` | — | liest den Textlayer eines PDF (FlateDecode-Content-Streams); bild-only/scanned → `pending` |
| `--count` / `--case` / `--path` | — | im `tools-latest`-Binär (gemessen 2026-09-25, sha `bf4656a6`) |

**Handoff — PDF→Bild→`vision`:** `archive_search --pdf-image <pdf|url> [--out <dir>]`
hebt die eingebetteten Bilder (`DCTDecode`→`.jpg`, `FlateDecode`→`.png`,
`JPXDecode`→`.jp2`) und druckt die Pfade; die gereichten Dateien gehen an
`vision` (P6, liest nur, kein bash). Kein Bild gefunden → eine `pending`-Zeile,
kein erfundenes Bild.

**Handoff — PDF→Text:** `archive_search --pdf-text <pdf|url>` liest den Textlayer
direkt (FlateDecode-Content-Streams). Trägt das PDF keinen Textlayer (gescannt/
bild-only) → eine `pending`-Zeile; dann führt der Weg über `--pdf-image` +
`vision` (OCR). Kein erfundener Text.

**Handoff — Legacy-Office (`.ppt`/`.doc`/`.xls`) → PDF → Text/Bild (gemessen 2026-10-12, river-196):**
die alten OLE2-Office-Binärformate (Magic `d0 cf 11 e0`) wandelt **`soffice --headless --convert-to pdf <datei>`**
(lokal vorhanden, LibreOffice 24.2) in ein PDF; danach `--pdf-text` (Textlayer) bzw. `pdftoppm -r 400 -png <pdf> <prefix>`
→ `vision` (Zeichnungen) und `tesseract <bild> - --psm 11` als OCR-Fallback. Beleg: die THEMIS-GBO-CDR-Decks
(`themis.ssl.berkeley.edu/themisftp/…/Reviews/`) trugen so eine **bemaßte** Zeichnung + Gehäusemaße. Kein Python.

## Gemessen — `archive_search` Netz-Modi (Einzelaufruf, kleiner Query)

| Modus | Zeit | | Modus | Zeit |
|---|---|---|---|---|
| `--arxiv` | 0,48 s | | `--crates` | 0,56 s |
| `--wiki` | 0,46 s | | `--datacite` | 0,58 s |
| `--librs` | 0,73 s | | `--github` | 0,76 s |
| `--brave` | 1,04 s | | `--heasarc` | 1,4–2,1 s |
| `--ntrs` | 1,38 s | | `--isc` | 1,70 s |
| `--ads` | 2,05 s | | `--wayback` | 2,72 s |
| `--crossref` | 3,48 s | | `--zenodo` | 11,6 s (101 Zeilen) |
| `--openalex` | 10,8 s (101 Zeilen) | | `--supermag` | 3,4 s Daten / 0,54 s Inventory |
| `--pubmed` | — (neu) | | `--europepmc` | — (neu) |
| `--psychporta` | — (neu, ES-POST) | | `--all` / `--wide` | Σ der Keyword-Modi (88; `--help` = Kanon) — letzte Stufe, nie der erste Zug · `--wide` = dieselbe Leiter mit Stopp **400** statt 20 (die gehaltenen Arme ziehen) |
| `--awmf` | — (neu, API-Key) | | `--cochrane` | — (neu, via Europe PMC) |
| `--mwmbl` | 0,78 s (neu, keyless JSON, kein Gate) | | | |
| `--clinicaltrials` | — (neu) | | `--openfda` | — (neu) |
| `--pubchem` | — (neu) | | `--uniprot` | — (neu) |
| `--pdb` | — (neu, PDBe) | | `--chembl` | — (neu) |
| `--ensembl` | — (neu, EBI Search) | | `--doaj` | — (neu) |
| `--go` | — (neu, QuickGO) | | `--unpaywall` | — (neu, braucht `UNPAYWALL_EMAIL`) |
| `--core` | — (neu, keyless/`CORE_API_KEY`) | | `--materialsproject` | — (neu, braucht `MP_API_KEY`) |
| `--semanticscholar` | — (neu, keyless 429/`S2_API_KEY`) | | | |
| `--reactome` | — (neu, keyless) | | `--interpro` | — (neu, keyless) |
| `--alphafold` | — (neu, keyless, UniProt-Accession) | | | |
| `--entrez` | — (neu, keyless, `db=nuccore\|sra\|gds`) | | | |
| `--ena` | — (neu, keyless, `result=read_run\|study\|sample\|analysis\|assembly`) | | |
| `--cod` | — (neu, keyless, `text=` Freitext + COD-Parameter) | | | |
| `--biomodels` | — (neu, keyless, EBI Search, `text=` Freitext) | | | |

## Interfaces — exakt (damit niemand rät)

- `sgrep [-i] [-l] [-c] [--all] [-g <glob>] <pattern> [dir|file]` — **kein `-n`**; die
  Standardausgabe ist `pfad:zeile:text`; ohne `-i` case-sensitiv; `-l` nur
  Dateipfade, `-c` nur der Zähler. Der Dateisatz kommt aus `git ls-files`
  (tracked + untracked-not-ignored). `--all` walkt den Arbeitsbaum und schließt
  **gitignorierte** Dateien ein (`phi/pipeline/queue`, `stage`, `research`, …),
  überspringt `target`/`data`/`cache`/`.git`/`node_modules` — die Sichtbarkeits-
  Lücke, durch die `glob` und `git ls-files` Pipeline-Material ausblenden.
- `archive_search <kws>... [--root <dir>]... [--lines n] [--files n] [--max-mb n]
  [--skip n] [--binary] [--count] [--case] [--include <glob>]` — default
  case-insensitiv; `--count` druckt `n files, m hits for: …`; `--include '*.rs'`
  filtert auf Dateinamen (der `grep --include`-Ersatz).
- `archive_search --index [<query>] [--path] [--kind any|file|dir] [--sort name|size|mtime]`.
- `archive_search --supermag "station=<code> start=<YYYYMMDDHHMM> end=<YYYYMMDDHHMM>"`
  (Daten) oder `"start=<YYYYMMDDHHMM> extent=<sekunden>"` (Stations-Inventory);
  Logon = `SUPERMAG_USER` aus `.secrets.local` (ohne Konto: Inventory geht, Daten
  nicht). Ein leeres `OK` ist `absent`, kein Fehler.
- `archive_search --heasarc "table=<w3browse-tabelle> rows=<n>"` — echte
  W3Browse-Tabellen, z. B. `table=sao`; `master` existiert nicht (W3Browse sagt
  es wörtlich).
- `archive_search --mwmbl <query>` — offene Community-Suchmaschine
  (`mwmbl.org/api/v1/search/?s=`), **keyless JSON, kein Gate**; Antwort
  `[{url, title:[{value,is_bold}], extract:[{value,is_bold}], source}]` → `url` +
  Titel/Quelle/Auszug. Gemessen 2026-09-21: relevante Treffer (Wikipedia/
  Frontiers/Nagoya), 0,78 s, 200. Der beste keyless Brave-Ersatz.
- `archive_search --brave <query>` — Brave Search API (`BRAVE_API_KEY`);
  gemessen 2026-09-21: **HTTP 402** (free credits erschöpft, Ledger `1789978555`)
  bis zum Monatswechsel — die Kaskade führt darum `--mwmbl` zuerst.
- `archive_search --marginalia <query>` — Marginalia public search
  (`api.marginalia-search.com`), **keyless JSON, kein Gate**; `url` +
  Titel/Beschreibung/`quality`. Gemessen 2026-09-23: relevante Treffer (MPG,
  ESA/sci.esa.int), 200 — die zweite keyless Engine neben `--mwmbl`.
- `archive_search --jina <url>` — Jina Reader (`r.jina.ai/<url>`), **keyless**;
  liefert den sauberen Text einer Seite als `Title / URL Source / Markdown Content`.
  Gemessen 2026-10-08 end-to-end (`--jina https://example.com` → 200, 1465 B,
  Markdown-Text). Die sauberere Text-Extraktion neben `--playwright`/`--sniff`.
- **Breite, nie Vorschau (Operator-Wort 2026-10-10).** `--max <n>` setzt die Ergebniszeilen je
  Netz-Quelle (Default **25**, vorher fest 10); `--all` zeigt inline je Quelle nur die ersten 5
  als **Vorschau** und schreibt das Volle in den `full: <pfad>`-Spill — der wird **vollständig
  gelesen**, die Vorschau ist nie das Ergebnis. Eine stille Kappung („top 5", „excerpt") als
  Grundlage eines Befunds ist eine ungemessene Behauptung = Fabrikation (`AGENTS.md`).
  `--all` hält nach **20** Ergebniszeilen; **`--wide`** hebt die Schwelle auf **400** (die
  gehaltenen Arme ziehen), `OMEGAFLOW_ALL_STOP` überschreibt beide. `bin/all_wide` ist
  gestrichen (2026-10-17) — `--wide` ist der eine Weg, kein zweiter Modus.
- `archive_search --consensus <query>` — Consensus-Literatursuche (`api.consensus.app/v1/search`,
  `CONSENSUS_API_KEY`); `title`/`doi`/`publish_year`/`citation_count`/`study_type`/`takeaway`.
  Gemessen 2026-10-08: liefert echte Literatur (Titel/DOI/Zitat/Studientyp/Takeaway).
- `archive_search --perplexity <query>` — Perplexity Sonar (`api.perplexity.ai`,
  `PERPLEXITY_API_KEY`); `answer`-Text + Quellen-Zitationen. Gemessen 2026-10-08: funktioniert
  (Answer + Zitationen); ein früherer Lauf kam als `HTTP 0` (transient) → `pending`, nie
  stiller Nullwert.
- `archive_search --tavily <query>` — Tavily Search API (`TAVILY_API_KEY`); `url`
  + Titel/`score`/Text.
- `archive_search --exa <query>` — Exa Search API (`EXA_API_KEY`); `url` +
  Titel/Autor/Datum/Text.
- `archive_search --linkup <query>` — Linkup Search API (`LINKUP_API_KEY`); `url`
  + Titel/Text.
  Gemessen 2026-09-23: alle drei lasen zunächst `pending` (`--exa`/`--linkup`
  HTTP 402, `--tavily` „no JSON") — Ursache war `net.rs::post()`, das die
  Auth-Header **ohne `-H`** als nacktes curl-Argument reichte (curl sah die
  Header-Zeile als zweite URL): kein Key gesendet → 402, und die `-w`-Ausgabe der
  Pseudo-URL vor dem JSON → kein Parse. Fix `post_args()` setzt `-H` vor jeden
  Header (Gate-Test `post_args_prefix_every_header_with_dash_h`); im
  `tools-latest`-Binär (gemessen 2026-09-25: `--tavily` liefert Treffer).
- `archive_search --alphaxiv <query>` — alphaXiv MCP `discover_papers`
  (`api.alphaxiv.org/mcp/v1`, `ALPHAXIV_API_KEY`); `url https://www.alphaxiv.org/abs/<id>`
  + Titel/Abstract. Gemessen 2026-10-07 (River 127): für die GIC-Stufe-2-Frage direkt
  einschlägig (`2605.27664` BOOST block-structured strong FWER, `2509.02376`
  multi-resolution maxT, `1106.2068` Westfall-Young). **Verdrahtet 2026-10-07:**
  `archive_search --alphaxiv-researchers <query>` — alphaXiv MCP `find_researchers`
  (derselbe Endpoint/Key); `url https://www.alphaxiv.org/@<slug>` + Name/Position/
  Zitationen/Affiliation (gemessen: liefert reale Profile, z. B. `@michael-wibral`).
  **Ebenso 2026-10-07:** `archive_search --alphaxiv-tools` listet alle alphaXiv-MCP-Arme mit
  ihrer `read`/`write`/`destructive`-Annotation; `archive_search --alphaxiv-call <tool>
  '<json-arguments>'` ruft jeden **Lese**-Arm auf — Schreib-/Lösch-Arme werden verweigert
  (`archive_search` ist read-only; ein Schreibakt ist Operator-Hand). Gemessen 2026-10-07:
  **23 Arme** (14 read, 5 write, 4 destructive).
- `archive_search --pubmed <query>` — NCBI E-utilities (esearch + esummary),
  `url https://pubmed.ncbi.nlm.nih.gov/<pmid>/` + Titel/Journal/Datum/DOI.
- `archive_search --europepmc <query>` — Europe PMC REST search,
  `url https://europepmc.org/article/<source>/<id>` + Titel/Jahr/DOI/Zitate.
- `archive_search --psychporta <query>` — ZPID PsychPorta (PSYNDEX +
  PsychArchives + Tests + Persons), Elasticsearch-POST an `/api/search`;
  `total: N` + `url https://psychporta.org/works/<id>` + Titel/Index.
  Deutschsprachige Psychologie (PubPsych ist seit Juni 2026 offline).
- `archive_search --awmf <query>` — AWMF-Leitlinienregister über
  `leitlinien-api.awmf.org/v1/search?keywords=` (öffentlicher Register-Key im
  Header `Api-Key`; Override `AWMF_API_KEY`); `url .../detail/<id>` + Klasse/
  Release/Beschreibung.
- `archive_search --cochrane <query>` — Cochrane Database of Systematic Reviews
  über den offenen Europe-PMC-Index (`JOURNAL:"Cochrane Database Syst Rev"`);
  die Direktseite ist Cloudflare-blockiert (403), Volltext ist Abo. Baut die
  kanonische `cochranelibrary.com/cdsr/doi/<doi>/full`-URL.
- `archive_search --clinicaltrials <query>` — ClinicalTrials.gov API v2
  (`query.term`), `url .../study/<NCT>` + Titel/Status.
- `archive_search --openfda <query>` — openFDA Arzneimittel-Label
  (`drug/label.json`, `openfda.generic_name`); `url` DailyMed `setid` + Marke/
  Hersteller/Datum.
- `archive_search --pubchem <query>` — PubChem PUG-REST Namens-Lookup
  (`property/MolecularFormula,MolecularWeight,IUPACName`); `url .../compound/<CID>`.
- `archive_search --uniprot <query>` — UniProtKB REST search;
  `url .../uniprotkb/<acc>/entry` + Proteinname/Organismus.
- `archive_search --pdb <query>` — PDBe/Solr (`search/pdb/select`);
  `url .../pdbe/entry/pdb/<ID>` + Titel.
- `archive_search --chembl <query>` — ChEMBL Molekül-Suche; Report-Card-URL +
  `pref_name`/`max_phase`/Formel/Masse.
- `archive_search --ensembl <query>` — EBI Search über `ensembl`;
  `url https://www.ensembl.org/id/<id>` + `source`.
- `archive_search --doaj <query>` — DOAJ Artikel-Suche; DOI-URL + Titel/Journal.
- `archive_search --go <query>` — QuickGO GO-Term-Suche; Term-URL + Name/
  Definition.
- `archive_search --unpaywall <DOI>` — legale OA-Version eines DOI
  (`api.unpaywall.org/v2/<doi>`); braucht `UNPAYWALL_EMAIL` (Kontakt-Adresse,
  Unpaywall-Pflicht), sonst `pending`.
- `archive_search --core <query>` — CORE v3 (`/v3/search/works/?q=`, Trailing-
  Slash); keyless mit 1 Batch / 5 Req pro 10 s, `CORE_API_KEY` (Bearer) hebt
  das Limit. `url https://core.ac.uk/works/<id>` + Titel/DOI/Jahr/Volltext.
- `archive_search --materialsproject <query>` — Materials Project
  (`/materials/summary/search?q=`), Header `X-API-KEY`; braucht `MP_API_KEY`
  (Dashboard), sonst `pending`. `url .../materials/<material_id>` + Formel/
  Bandlücke/Stabilität.
- `archive_search --semanticscholar <query>` — S2 Academic Graph
  (`/graph/v1/paper/search`); keyless 429 (geteilter Pool), `S2_API_KEY` hebt
  das Limit. `url` + Titel/DOI/arXiv/Jahr/Zitate.
- `archive_search --reactome <query>` — Reactome ContentService-Suche
  (`/ContentService/search/query?query=&cluster=true`); keyless.
  `url https://reactome.org/content/detail/<stId>` + Name/Typ/Spezies/Datenbank/
  Referenz.
- `archive_search --interpro <query>` — InterPro-Eintragssuche über
  `/api/entry/all/protein/reviewed/?search=`; keyless.
  `url https://www.ebi.ac.uk/interpro/entry/<source_database>/<accession>` +
  Name/Typ/Quelle/integrierter Eintrag.
- `archive_search --alphafold <acc>` — AlphaFold DB Vorhersage über
  `/api/prediction/<uniprot-accession>`; keyless.
  `url <pdbUrl>` + Eintrag/Accession/Proteinname/Gen/Organismus/pLDDT/
  Sequenzdatum.
- `archive_search --entrez "db=<database> <term>"` — NCBI E-utilities esearch
  (`esearch.fcgi?db=&term=&retmode=json`); keyless. `db=nuccore|sra|gds`
  (GenBank/SRA/GEO), `term=` optional statt des Rests. `count <n>` +
  `query: <querytranslation>` + je Treffer `url https://www.ncbi.nlm.nih.gov/<db>/<id>`
  + `db`/`id` (GEO als `geo/query/acc.cgi?acc=`).
- `archive_search --ena "result=<type> fields=<comma-list> <query>"` — EBI European
  Nucleotide Archive Portal API (`portal/api/search?format=json&limit=`, Gesamtzahl
  über `portal/api/count`); keyless. `result=read_run|study|sample|analysis|assembly`,
  `fields` ist Pflicht (ohne `fields` liefert ENA den vollen Feldsatz, gemessen
  ~70 MB bei `read_run`), `query`/`term` optional (leer = erste Datensätze).
  `count <n>\tresult: <type>` + je Treffer `url https://www.ebi.ac.uk/ena/browser/view/<accession>`
  (aus dem `*_accession`-Feld) + die angeforderten Felder. Kein Eintrag → `absent`.
- `archive_search --cod "text=<free text> [el1=.. el2=.. nel=..] [fields=<comma-list>] [max=<n>]"`
  — Crystallography Open Database REST (`/cod/result?format=json`, keyless). Freitext über
  `text=`, weitere COD-Parameter (`el1`/`el2`/`el3`/`nel`/`formula`/`sg`/`year`/`doi`/`id` …)
  werden als key=value durchgereicht; `fields=` wählt die Anzeigefelder (Default:
  `file,chemname,formula,sg,a,b,c,alpha,beta,gamma,vol,year,doi`), `max=` die lokale
  Zeilengrenze. Je Treffer `url https://www.crystallography.net/cod/<file>.html` + die Felder.
  Kein Eintrag → `absent`.
- `archive_search --biomodels "text=<free text> [fields=<comma-list>] [max=<n>]"`
  — EBI BioModels über die EBI-Search-REST (`/ebisearch/ws/rest/biomodels?format=json&size=`,
  keyless). Freitext über `text=` (auch `query=`/`term=` oder blank), `fields=` wählt die
  Anzeigefelder (Default: `name`), `max=` die lokale Zeilengrenze. Je Treffer
  `url https://www.ebi.ac.uk/biomodels/<id>` + die Felder. Kein Eintrag → `absent`.

**Astronomie/Raumfahrt:**
- `archive_search --arxiv <query>` — arXiv-Keyword-Modus (keine Detailzeile im `--help`).
- `archive_search --arxiv-oai [set] [--pages <n>]` — arXiv OAI-PMH Bulk-Harvest (ListRecords + resumptionToken bis zum Ende; `set` = optionaler setSpec-Filter, `--pages` kappt die Seitenzahl) — Katalog-Record-Format `identifier | title`.
- `archive_search --ads <query>` — ADS-Keyword-Modus (keine Detailzeile im `--help`).
- `archive_search --ntrs <query>` — eine bloße Citation-ID löst über den Citation-Pfad auf, jede andere Query sucht.
- `archive_search --opensn <query>` — Open Supernova Catalog (`github.com/astrocatalogs` sne-*-Repos), keyless; `url` + Name/Typ/Rotverschiebung/RA/Dec/Discoverdate aus dem OSC-Objekt-JSON.
- `archive_search --frbcat <query>` — FRBSTATS Fast-Radio-Burst-Katalog (live gviz CSV, Fallback Repo-`catalogue.csv`), keyless; `url` + FRB/RA/Dec/Rotverschiebung (voll im Temp-File).
- `archive_search --simbad <query>` — SIMBAD-Objektsuche (`simbad.u-strasbg.fr/simbad/sim-tap`, ADQL `ident.id LIKE`), keyless; sim-id-`url` + main_id/RA/Dec.
- `archive_search --vizier <query>` — VizieR-Katalogsuche (`tapvizier.cds.unistra.fr/TAPVizieR`, TAP_SCHEMA.tables), keyless; VizieR-`url` + table/description.
- `archive_search --exoplanet-archive <query>` — NASA Exoplanet Archive Planetensuche (`exoplanetarchive.ipac.caltech.edu/TAP`, pscomppars), keyless; overview-`url` + pl_name/hostname/year.
- `archive_search --ned <query>` — NED-Objektsuche (`ned.ipac.caltech.edu/tap` NEDTAP.objdir, prefname-Präfix), keyless; byname-`url` + main_id/RA/Dec.
- `archive_search --mast <query>` — MAST Name Resolver (`mast.stsci.edu`, Mast.Name.Lookup), keyless; id/name/RA/Dec/type.
- `archive_search --gaia <query>` — Gaia-Archiv TAP source_id-Lookup (`gea.esac.esa.int`), keyless; numerische source_id + RA/Dec/g_mag (Textnamen laufen in den Timeout).
- `archive_search --horizons <query>` — JPL Small-Body Database Lookup (`ssd-api.jpl.nasa.gov/sbdb.api`), keyless; sbdb-`url` + des/a/e/i.
- `archive_search --lasair <query>` — Lasair LSST-Objektabfrage (`api.lasair.lsst.ac.uk`), `LASAIR_LSST_TOKEN`; objectId → diaObject-Felder (direkt, dann socks5h 127.0.0.1:25344).
- `archive_search --gosat <product> [<start> <end>]` — GOSAT-GW TANSO-3 Produktsuche (cookie-authed via `GOSAT_GW_MAIL`/`GOSAT_GW_PASS`; Default-Produkt GWT3F_L1B, Default letzte 30 Tage).
- `archive_search --lpf <start> <end> [hdu]` — LISA-Pathfinder-DRS-Verfügbarkeit aus dem HEASARC-Selektor (start/end als MJD oder ISO; Default hdu=SCI_SCIENCE_1Hz; der Binärbody wird gemessen, nie gedruckt).
- `archive_search --nssdca <query>` — NASA NSSDCA Master Catalog (public; Datenkollektions-Name, key=value spacecraft/experiment/pi, oder bare NSSDCA-ID wie PSCM-00009 für das Dataset-Detail).
- `archive_search --nssdc <query>` — China NSSDC/VSSO coreMetadata-Suche (`vsso.nssdc.ac.cn`, public; der Download ist CAS/Captcha- und application-gated).
- `archive_search --nssdc-auth` — China-NSSDC-CAS-Login (`NSSDC_USER`/`NSSDC_PASS`); misst `ticket present` / `page authenticated`. **Gemessen 2026-10-14:** der CAS antwortet auf einen erfolgreichen Login mit HTTP 200 und einer JS-Weiterleitung im **Antwortkörper** (`…logindo?CURRENTCASUSER=…&ticket=ST-…`, kein `Location`-Header) — der Arm liest Header **und** Body; das Passwort geht als `base64(escape(pass))` (die Seite `encode64`, `/js/commonNew.js:30`).
- `archive_search --ssdc-auth <service-url>` — ASI-SSDC-CAS-Login (`tools.ssdc.asi.it`, `SSDC_USER`/`SSDC_PASS`); misst `ticket present`.

**Geo/Erdwissenschaften:**
- `archive_search --cc <url>` — Common Crawl Index JSON-lines (`index.commoncrawl.org/<index>-index?url=…&output=json`), keyless; Default-Index CC-MAIN-2024-51, Override index=<id>.
- `archive_search --wayback-available <url>` — Availability API, nächster Snapshot (`archive.org/wayback/available?url=…`).
- `archive_search --wayback-timemap <url>` — Memento-Timemap-JSON (`web.archive.org/web/timemap/json/<url>`).
- `archive_search --ckan <query>` — Datenkatalog-Suche (CKAN package_search), keyless; `portal=<host>` (Default ckan.publishing.service.gov.uk = data.gov.uk); `url` + Titel/Organisation/Ressourcen/Notes.
- `archive_search --cmr <query>` — NASA Earthdata CMR Collection-Suche (`cmr.earthdata.nasa.gov`), keyless; `url` + short_name/provider/summary.
- `archive_search --dataeuropa <query>` — EU Open Data Portal (`data.europa.eu/api/hub/search`), keyless; `url` + Titel/Publisher/Land/issued/description.
- `archive_search --dryad <query>` — Dryad research-data (`datadryad.org/api/v2/search`), keyless; doi + Titel/Autoren/Abstract.
- `archive_search --tap "<endpoint=… SELECT …>"` — IVOA TAP sync ADQL, keyless; Zeilen als col=val (Service mit `--regtap` finden).
- `archive_search --pangaea <query>` — PANGAEA earth-science (`pangaea.de/advanced/search.php`), keyless; doi + Titel/Abstract.
- `archive_search --re3data <query>` — re3data Repository-Suche (`re3data.org/api/v1`), keyless; id + name/doi.
- `archive_search --seanoe <query>` — SEANOE ocean-data (`seanoe.org/?q=`), keyless; Dataset-`url` (gescrapte Ergebnislinks).
- `archive_search --arcgis <query>` — ArcGIS Online Item-Suche (`arcgis.com/sharing/rest/search`), keyless; `url` + Titel/Typ/Owner/Snippet.
- `archive_search --dataverse <query>` — Dataverse Dataset-Suche (`entrepot.recherche.data.gouv.fr/api/search`), keyless; `host=<host>`; `url` + Titel/Publisher/Subjects/Description.
- `archive_search --sparql "<endpoint=… SELECT …>"` — generisches SPARQL, keyless; Zeilen als var=value.
- `archive_search --copernicus <query>` — Copernicus CDS/CAMS Collection-Filter (catalogue v1 STAC), keyless; `url` + Titel/Description (Substring-Filter).
- `archive_search --vires <query>` — VirES HAPI Dataset-Katalog-Filter (`vires.services/hapi`), keyless; Dataset-`url` (Substring-Filter).
- `archive_search --oai-pmh "endpoint=<oai-url> [set=<set>]"` — generisches OAI-PMH ListRecords, keyless; id + Titel.
- `archive_search --stac "endpoint=<url> [collection=<id>] [bbox=w,s,e,n] [datetime=a/b]"` — generische STAC-Suche, keyless; `url` + collection/id/datetime.
- `archive_search --erddap "server=<url> searchFor=<kw> [itemsPerPage=N]"` — generische ERDDAP-Dataset-Suche, keyless; `url` + Titel/Institution/id.
- `archive_search --overpass <query>` — OSM Overpass Namenssuche (`overpass-api.de`, Overpass QL), keyless; openstreetmap-`url` + Name/Typ.
- `archive_search --nominatim <query>` — OSM Nominatim Geocoding (`nominatim.openstreetmap.org`, User-Agent nötig), keyless; openstreetmap-`url` + Name/lat/lon.
- `archive_search --open-meteo <query>` — Open-Meteo Geocoding (`geocoding-api.open-meteo.com/v1`), keyless; Name/Land/lat/lon.
- `archive_search --gbif <query>` — GBIF Artensuche (`api.gbif.org/v1/species/search`), keyless; GBIF-Art-`url` + scientific_name/rank/status.
- `archive_search --fdsn <query>` — FDSN Stationssuche (`service.iris.edu/fdsnws/station/1`, Text-Tabelle), keyless; network/station/lat/lon.
- `archive_search --ngmdb <query>` — USGS National Geologic Map DB (`ngmdb.usgs.gov` ArcGIS REST), keyless; `url` + Titel/Bundesstaat/Jahr.
- `archive_search --gfw "dataset=<name> [sql=<select>]"` — Global Forest Watch Data API (`data-api.globalforestwatch.org`), `x-api-key`.
- `archive_search --opencellid "mcc=.. mnc=.. lac=.. cellid=.. [radio=..]"` — OpenCelliD Zell-Lookup (`opencellid.org/cell/get`), Key; lat/lon/range.
- `archive_search --regtap <query>` — IVOA RegTAP Registry-Suche (`dc.g-vo.org/tap`, ADQL über rr.resource), keyless; ivoid + Titel/Description.
- `archive_search --isc "start=.. end=.. [minmag=.. minlat=.. maxlat=.. minlon=.. maxlon=..]"` — ISC-Erdbebenkatalog, key=value.

**Life-Sciences/Daten/Literatur:**
- `archive_search --base <query>` — BASE — Bielefeld Academic Search Engine (~400M offene Dokumente, `api.base-search.net`), **`BASE_API_KEY`** (net.rs:4181; ohne Schlüssel `pending`); `url` + Titel/Autoren/Jahr/doi; Teil von `--all` (`QUERY_MODES`). **Erster Arm für Datensatz-/Repositoriums-Discovery** — findet DataCite-/Repositoriums-DOIs (NASA PDS, GFZ Data Services, VizieR, Hochschul-Repositorien), die arXiv/Crossref/OpenAlex übersehen (gemessen 2026-10-13: Venus-Radar-PDS-Bundle `10.17189/1522443`, ALFALFA-`J/ApJ/768/77`).
- `archive_search --osf <query>` — OSF-Preprints-Titelsuche (`api.osf.io/v2/preprints`), keyless; `url` + Titel/Autoren/doi/Datum/Abstract.
- `archive_search --openaire <query>` — OpenAIRE Graph research-product (`api.openaire.eu/graph/v1`), keyless; `url` (DOI) + Titel/Datum/Publisher.
- `archive_search --figshare <query>` — Figshare research-data (`api.figshare.com/v2/articles/search`, POST), keyless; `url` + Titel/doi.
- `archive_search --dataone <query>` — DataONE Solr (`cn.dataone.org/cn/v2/query/solr`), keyless; search.dataone.org-`url` + Titel/Format.
- `archive_search --huggingface <query>` — HuggingFace Datasets API (`huggingface.co/api/datasets`), keyless; Dataset-`url` + Autor/Downloads.
- `archive_search --dblp <query>` — DBLP-Publikationssuche (`dblp.org/search/publ/api`), keyless; `url` + Titel/Venue/Jahr (aktuell JS-gated: `pending`).
- `archive_search --orcid <query>` — ORCID Forschersuche (`pub.orcid.org/v3.0/expanded-search`), keyless; orcid-`url` + Name/Institution.
- `archive_search --ror <query>` — ROR Organisationssuche (`api.ror.org/organizations`), keyless; ror.org-`url` + Name/Land.
- `archive_search --opencitations <DOI>` — OpenCitations Citation-Count (`opencitations.net/index/api/v1`, DOI-only), keyless; doi + citations.
- `archive_search --scholexplorer <query>` — OpenAIRE Scholexplorer Links (`api.scholexplorer.openaire.eu/v3/Links`, DOI/PMID), keyless; target + Relationstyp.
- `archive_search --mendeley <query>` — Mendeley Data (`data.mendeley.com/api/datasets`), keyless; Dataset-`url` + Titel/doi.
- `archive_search --openlibrary <query>` — Open Library Katalogsuche (`openlibrary.org/search.json`), keyless; `url` + Titel/Autoren/Jahr/Editions.
- `archive_search --stackexchange <query>` — Stack Exchange Q&A (`api.stackexchange.com/2.3`, site=stackoverflow), keyless; `url` + Titel/Score/answered/answers/tags.
- `archive_search --sourcegraph <query>` — Sourcegraph Code-Suche (`sourcegraph.com/.api/search/stream`), keyless; repo/path/line/match.
- `archive_search --grepapp <query>` — grep.app Code-Suche (`grep.app/api/search`), keyless; repo/path (Vercel-Checkpoint kann 429 antworten).
- `archive_search --oeis <query>` — OEIS Integer-Sequenz-Suche (`oeis.org/search?fmt=json`), keyless; `url` + Titel/data.
- `archive_search --hal <query>` — HAL Open Archive (`api.archives-ouvertes.fr/search`), keyless; `url` + Titel/doi/published.

**Web/Misc:**
- `archive_search --wiki <query>` — Wiki-Keyword-Modus (keine Detailzeile im `--help`).
- `archive_search --crossref <query>` — Crossref-Keyword-Modus (keine Detailzeile im `--help`).
- `archive_search --github <query>` — GitHub-Keyword-Modus (keine Detailzeile im `--help`).
- `archive_search --crates <query>` — crates-Keyword-Modus (keine Detailzeile im `--help`).
- `archive_search --datacite <query>` — DataCite-Keyword-Modus (keine Detailzeile im `--help`).
- `archive_search --zenodo <query>` — Zenodo-Keyword-Modus (keine Detailzeile im `--help`).
- `archive_search --openalex <query>` — OpenAlex-Keyword-Modus (keine Detailzeile im `--help`).
- `archive_search --wayback <query>` — Wayback-Keyword-Modus (keine Detailzeile im `--help`).
- `archive_search --duckduckgo-instant <query>` — DuckDuckGo Instant Answer (`api.duckduckgo.com`), keyless; answer/abstract + related-topic url/title.
- `archive_search --searxng <query>` — SearXNG Meta-Suche (Instanz aus `$OMEGAFLOW_SEARXNG`, sonst public JSON-Instanzen; keyless); `url` + Titel/Content/engine.
- `archive_search --serper <query>` — Serper Google SERP (`google.serper.dev/search`, X-API-KEY); `url` + Titel/Position/Snippet.
- `archive_search --firecrawl <query>` — Firecrawl (`api.firecrawl.dev/v1/search`, Bearer; OSS self-host Fallback); `url` + Titel/Description.
- `archive_search --searchapi <query>` — SearchApi.io (`searchapi.io/api/v1/search`, Bearer); `url` + Titel/Position/Snippet.
- `archive_search --serpapi <query>` — SerpApi (`serpapi.com/search.json`, api_key); `url` + Titel/Position/Snippet.
- `archive_search --gemini <query>` — Google Gemini answer+search (`generativelanguage.googleapis.com`), `GEMINI_API_KEY`; Antworttext + Google-Search-Grounding-Quellen.
- `archive_search --wikidata <query>` — Wikidata Entity-Suche (`wikidata.org/w/api.php` wbsearchentities), keyless; id/label/description.
- `archive_search --googlecse <query>` — Google Programmable Search (`googleapis.com/customsearch/v1`), `GOOGLE_CSE_API_KEY` + `GOOGLE_CSE_CX`; `url` + Titel/Snippet.
- `archive_search --wiby <query>` — Wiby unabhängige Websuche (`wiby.me/json`), keyless; `url` + Titel/Description.
- `archive_search --ia-search <query>` — Internet Archive Fulltext/Metadata (`archive.org/advancedsearch.php`), keyless; `url` + Titel/Mediatype.
- `archive_search --rss-bridge "bridge=<name> <key>=<value>..."` — rss-bridge JSON-Feed (`rss-bridge.org`), keyless; `url` + Titel/Content.
- `archive_search --kiwix <query>` — Kiwix OPDS-Katalog (`opds.library.kiwix.org`), keyless; `url` + Titel/Summary.
- `archive_search --scrape "<url> [selector=<css>]"` — scraper.workers.dev Seitentext, keyless; `url` + Text.
- `archive_search --shodan <query>` — Shodan Host-Suche (`api.shodan.io/shodan/host/search`), Key; ip/port/org/country.
- `archive_search --apis <query>` — API-Verzeichnis (`apis.io/api/v1/apis` primary, `api.apis.guru/v2/list.json` Fallback), keyless; spec-`url` + Titel/Provider.
- `archive_search --oapen "set=<setSpec>"` — OAPEN OAI-PMH (`library.oapen.org/oai/request`; keine Freitextsuche — ListSets oder `set=<setSpec>`), keyless; `url` + Titel.

## Gemessen — übrige lokale Werkzeuge

| Werkzeug | Zeit | Bemerkung |
|---|---|---|
| `omega_sh reports` | 0,007 s | 3645 Zeilen (Watchdog-Konkatenat) |
| `omega_sh search` | 0,055 s | `sgrep`-Wrapper |
| `omega_sh fetch` | 0,122 s | `sfetch`-Wrapper |
| `omega_sh jwst` | 0,644 s | CDN-Watch |
| `omega_sh sha <file>` | — | Header-sha256 über den Body ohne `<!-- … -->`-Header (docs-naming) |
| `omega_sh check` | — | `cargo check`-Zusammenfassung (Fehler-/Warnungszahlen) |
| `sfetch` / `curl -s` | 0,113 / 0,073 s | `sfetch` zuerst — **HTML/Text**; für Binärdownloads (PDF/PNG) `sfetch --raw` oder `curl -o`, sonst zerstört (`from_utf8_lossy` + Tag-Strip) |
| `smail --dry-run` | 0,288 s | kein Versand |
| `register_lookup --open` | 0,054 s | 659 Zeilen — Planungs-Pass |
| `register_lookup --dropped [<line>] [--persist <n>]` | — | Diff-Gate: offene Punkte aus Übergabe N, die in N+1 fehlen, je Linie (kein Arg = alle); `--persist <n>` = nur Punkte, die ≥n Übergaben überleben, dann verschwinden |
| `register_lookup --orphans` | — | owner-getaggte offene Register-Einträge, die keine lebende Übergabe des Owners hält: `ORPHAN_COMMITTED` (in HEAD) / `ORPHAN_UNCOMMITTED` (nur Arbeitsbaum); Klassen-Träger `φ/<register>::gap:<token> ×N` hält die Klasse, Count-Drift wird als `CARRIER_DRIFT` gemeldet; `NO_GAP` = parser-Block ohne erklärte Klasse (lauter Einzel-Orphan). Getrennt von `--dropped` (kein Aufenthalt vs. abgerissener Aufenthalt) |
| `register_lookup --orphan-docs` | — | lebende Prosadokumente (`docs/{surveys,specs,auftrag,blatt,concepts,paper}`) mit offenen Markern, die keine lebende Übergabe nennt: `ORPHAN_DOC <path> <marker>`; der Commit-Gate `commit_check` (doc-carrier) blockt neue trägerlose Dokumente |
| `register_lookup --history` | 1,03 s | 3049 Zeilen |
| `register_lookup --compilers [--no-directive]` | — | je φ/sources.φ-Block mit `compiler`-Direktive `<binary> | <format> | <source-url> | at <anchor>` + Domain-Zusammenfassung über das Format-Präfix; `--no-directive` = Baum-Compiler (`tools/*/src/bin/*_compiler.rs`) ohne Direktive, klassifiziert `workflow`/`register:<datei>`/`variant`/`pending`/`unregistered` |
| `register_lookup --tokens [--fail] [--noecho-fail]` | — | je Eintrag jedes `## An <line>`-Blocks `XFER <tok> echoed|unbeglichen` (Echo = Owner-`## Offen`-Token **oder** durable `FOLDED`/`CLOSED` im Ledger), `XFER-MANIFEST ok|open` bei `[manifest n=<N> root=<h8>]`, `XFER-MISSING` (kein Token), `XFER-NOECHO` (Sender hat ohne Echo gelöscht); `--noecho-fail` exit 2 nur auf MISSING/NOECHO (CI-Gate) |
| `xfer mark/list/fold/close/manifest/migrate/coverage` | — | Transport-Token-Ledger (`state/zustand/ledger/<line>/<sess>.log`, append-only, gitignored): `mark --line <L> [--to <line|self>] "…"` mintet `<line>.<sess>.<n>~<h8>`, `fold <tok> --line <L> --into <ziel>`, `close <tok> --line <L> --ref <commit\|pfad>`, `manifest --line <L> --to <ziel> [--file <p>]` druckt `[manifest n=<N> root=<h8>]`, `migrate --line <L> [--handover <p>] [--write]` ersetzt alte `xfer:<sender>-<n>` in den `## An`-Blöcken durch verankerte Tokens (eine Spur, kein `legacy.log`), `coverage --line <L>` exit 2 solange ein Token offen |
| `ci_manage status [--limit N]` | — | Live-Übersicht + aktueller Schritt je laufendem Run (ein Listen-Call + je laufendem Run ein Jobs-Call) |
| `ci_manage jobs <run-id>...` | — | Job-Status + aktueller Schritt je Run (GH-API `/runs/<id>/jobs`); der `updated_at` von `list` ist grob |
| `open_points_check [<handover>] [--root <dir>]` | — | billiger Baum-Abgleich (std, kein Netz/LLM): jeder in den offenen Punkten genannte Pfad wird gegen den Arbeitsbaum geprüft; `ABSENT` = stale Punkt; Default = neueste `docs/handover/*.md` |
| `git_safety --snapshot` | 1,20 s | Planungs-Pass |
| `git_safety --list` | 0,017 s | |
| `git_safety --close [<own-path>…]` | — | Commit-Abschluss-Check in einem Aufruf |
| `session_burn` | — | Burn je Session (opencode.db); PATH-Symlink `~/.local/bin/session_burn` → `bin/session_burn` (Wrapper über `bin/.tools_ensure`), `tools-latest` sha `97f6c252`, gemessen 2026-09-25 |
| `./bin/archive_search` (Wrapper) | 0,009 s nach Stempel | execs `bin/.tools_ensure` (Refresh aus `tools-latest`), baut nie |
| OpenCode-Tools (`read`/`grep`/`glob`) | ein Tool-Round-Trip, kein Prozess | Kosten sind Kontext, nicht CPU |

## Werkzeug → kann → darf

| Werkzeug | Kann | Scope | Profile |
|---|---|---|---|
| `archive_search` (PATH, Symlink auf `bin/archive_search`) | Inhalt, Pfade, NTFS, 106 Netz-Modi (gemessen 2026-10-12 aus `archive_search --help` — `--all` nennt 106 Aufrufe), `--playwright`, `--all`, `--leads`, `--serve`, `--count/--case/--path` | lokal + Netz | P1–P5 |
| `bin/archive_search` (Wrapper) | execs `bin/.tools_ensure`, sonst `exec` | lokal | P1–P5 |
| `sgrep` | Zeilensuche über `git ls-files`; `--all` incl. gitignorierter Dateien | lokal | P1–P5 |
| `sfetch` / `omega_sh` | fetch / reports-status-search-fetch-jwst-sha-check | Netz / lokal | P3 (fetch), P1 (alle) |
| `smail` | Mail senden (Resend), `--dry-run` | Netz | P1 |
| `register_lookup` | `--open`/`--dropped`/`--orphans`/`--orphan-docs`/`--history`/`--tokens` — Register mit OPEN-Zeilen, zustand-/post-Scan, Drop-Diff, Träger-Checks, Transport-Token-Zustand | lokal | P4 |
| `xfer` | Transport-Token-Ledger: `mark`/`list`/`fold`/`close`/`manifest`/`migrate`/`coverage` — mintet Marker am Punkt der Handlung, dauerhaftes Echo für `register_lookup --tokens` | lokal | P1/P4 |
| `ci_manage` | `status`/`list`/`view`/`jobs`/`log`/`cancel`/`rerun` — CI-Läufe mit Schritt-Detail und Fehllogs; nie `gh run` | Netz | P1–P5 |
| `open_points_check` | Pfad-Abgleich der offenen Punkte gegen den Arbeitsbaum (stale Punkt = genannter Pfad absent) | lokal | P4 |
| `git_safety` | `--snapshot/--restore/--list/--watch/--close` | lokal | P1/P4 |
| `session_burn` | Burn je Session (opencode.db) | lokal | P1 |
| OpenCode-Tools | kein Prozess, ein Round-Trip | — | nach Profil |
| `curl` | nur wo `sfetch`/`archive_search` nichts trägt | Netz | P3 |

**CI-Roh-API (`curl` → `api.github.com/.../actions/runs`):** der Filter `conclusion=` wird
nicht unterstützt und still ignoriert (die Liste kommt ungefiltert zurück — gemessen 2026-10-10:
`?status=completed&conclusion=failure` lieferte `cancelled`-Läufe, und zählte fälschlich 0 rote).
Der tragfähige Filter ist `status=failure` / `status=cancelled` / `status=success` (plus `head_sha`),
sonst client-seitig `jaq 'select(.conclusion=="failure")'`; der kanonische Leser bleibt `ci_manage`.

## Session-Checks — lokal, deterministisch/gescoped (2026-09-28)

Vier Werkzeuge für Eingang/Ausgang und die Nachschau auf das Veröffentlichte. Kein
Gate (das bleibt `commit_check`), kein Send. Die `state/`-Daten verlassen die
Maschine nie.

| Werkzeug | Kann | Scope | Profile |
|---|---|---|---|
| `bin/session_check --in <handover>` / `--out` | Ein-/Ausgangs-Check in einem Turn: (in) HEAD/origin-Relation + dirty, `open_points_check`, `--orphan-docs`, `--orphans`, `ci_manage status`; (out) `git show --stat HEAD`, `git log origin/main..HEAD --name-only`, `--history`, `git_safety --snapshot`. Kastriert nichts, keine Gate-Schicht | lokal | P1/P4 |
| `bin/house_audit [--staged] [--spec]` | mechanische Hausregeln des getrackten Baums: `pii` (Adresse/Telefon/Mail, nie getrackt), `ident` (`Archiver`/`Mathematician`), `spec` (opt-in) | lokal | P1 |

## Browser-Anbindung — die vier Pfade (Rat 2026-09-20: nicht konsolidieren; Pfad 4 registriert 2026-09-23)

Vier Pfade sind vier Identitäten (A = A), je mit gemessener Rolle. Messung + Verdikte:
`docs/surveys/survey-2026-09-20-browser-anbindung.md`, Messnachtrag 2026-09-23.

| Pfad | Natur | Rolle | Eintritt |
|---|---|---|---|
| `@vymalo/opencode-browser` (Plugin, Bridge Port 4517, Extension am Operator-Profil) | UI-Automator am echten Profil | (iii) Registrierung, interaktive Seiten | `browser_open`/`click`/`type`/`snapshot`/`get_text`/`get_html`; Fokus nie erzwungen (`focus:false`), Inhalt statt Pixel |
| `@playwright/mcp` (globaler MCP, headless) | Kopflos-Automator | (ii) mehrschrittige Flüsse, Cross-Browser/Tests | MCP-Tools; eigene Session |
| `archive_search --playwright` | Kopflos-Fetcher mit SOCKS/proton + Interstitial-Erkennung; `OMEGAFLOW_COOKIES=<cookie-editor.json>` setzt eine Session vor `goto` | (ii) Recherche, erster Zug | `archive_search --playwright <url> [--headed]` |
| `opencode-chromium` (npm 1.7.2; Extension `hdljmmpfnhojebplbbgdgejoobmjcbml` + Native-Messaging-Host `com.opencode.browser.plugin`; Operator-Profil) | UI-Automator am echten Profil, Batch | (ii) mehrschrittige Ketten (≤20 Steps/Call), opencode-v2-fähig | `browser_run`/`observe`/`session`/`finalize`; `settle` default `dom-quiet` (langsam) → `waitUntil:load` + `settle:{exists,selector}` |

- **(i) Membran-Debug** → **Chrome DevTools MCP** (Konsole/Netz/Performance via CDP auf
  dem Pfad-1-Chrome) — **gebunden** (`opencode.json:172-178`: `chrome-devtools-mcp@1.9.0`
  mit `--no-usage-statistics --no-performance-crux --autoConnect`, Commit `7e79c194d`,
  2026-09-20); das Operator-Wort (Debugger-Rechte am live Chrome) ist **gegeben** (2026-10-12, Operator „mach", `state/operator-gespraeche/2026-10-12-mycelium.md`) — die Bindung ist damit aktiv.
- **(iii) Captcha** — Option (a) gesetzt: die Operatorin löst selbst im sichtbaren
  Browser. Option (b) Buster (dessant/buster, GPL-3.0, reCAPTCHA-Audio, lokal per
  Whisper möglich) ist **descoped mit Befund**, solange das Operator-Wort ausbleibt.
  Bezahlte Solver (Option c) sind ausgeschlossen.
- **Cookie-Editor** (Moustachauve/cookie-editor, GPL-3.0) ist **manuelles**
  Operator-Werkzeug — Cookie-Transfer Operator-Profil ↔ persistentes Profil nur per Akt
  mit Operator-Wort (Cookies = Zugangsdaten). Mechanischer Weg (Sensory-Folge 121,
  Operator-Wort „bau B"): der Cookie-Editor-Export wird als `OMEGAFLOW_COOKIES=<json>`
  an `archive_search --playwright` gereicht (`context.addCookies` vor `goto`, headless
  wie headed); unset = keine Cookies (0 geehrt), gesetzt-aber-kaputt = benannter
  Abbruch `exit(2)`. Die Datei bleibt lokal/ungtracked.
- **Privacy Pass** (Cloudflare) reduziert Challenge-Häufigkeit, löst keine Captcha.
- Pfad-1-Version: die globale Config (`~/.config/opencode/opencode.jsonc`) pinnt
  **`@vymalo/opencode-browser@0.17.0`** (gemessen 2026-09-20, Sensory-Folge 115) —
  die Versionslücke 0.16.1 → 0.17.0 ist geschlossen, kein Zustand mehr.
- **Pfad 4 — `opencode-chromium`** registriert 2026-09-23 (Operator-Wort „beide behalten
  und registrieren"): npm `opencode-chromium@1.7.2` (MIT, Repo `Quindart-com/opencode-chromium`),
  Store-Extension `hdljmmpfnhojebplbbgdgejoobmjcbml`, Native-Messaging-Host
  `com.opencode.browser.plugin` (`~/.config/google-chrome/NativeMessagingHosts/`), Wrapper
  `~/.config/opencode/browser/opencode-browser-host`. Vier grobe Tools; `find` = semantische
  Suche (`query`), kein CSS-Selektor (CSS nur über `target.selector` bei `click`/`assert`).
  Gemessen 2026-09-23 am selben Profil: Transport-Roundtrip ~10–20 ms (gleichauf mit Pfad 1);
  Navigation default `dom-quiet`-Settle läuft in den Timeout (~2,7 s — spürbar langsam), mit
  `waitUntil:load` + `settle:{exists,selector}` ~130 ms (schneller als Pfad 1, das ~180–480 ms
  braucht); Text/Observe ~13–60 ms. Koexistenz beider Brücken am selben Profil gemessen. Der
  Adapter ist in der OpenCode-1.18.x-`{ id, server() }`-Path-Plugin-Form **und** mit dem
  V2-Vertrag gebaut → ein opencode-2.0-Umstieg wird von Pfad 4 nicht blockiert (Pfad 1 ist
  V1-only). CLI-Kante: der globale bin-Symlink triggert `main()` in 1.7.2 nicht — Aufruf über
  `node …/dist/cli/index.js <cmd>`.
- **Kopplung (Pfad 4):** geladen über die globale Config
  (`~/.config/opencode/opencode.jsonc`, `"plugin": [ … , "opencode-chromium" ]`) — beide
  Brücken laden nebeneinander; **kein** Projekt-`opencode.json`-Eintrag (Merge-Semantik
  global↔Projekt vermieden). Rückbau: Extension entfernen + `…/NativeMessagingHosts/com.opencode.browser.plugin.json`
  + `~/.config/opencode/browser/` löschen + `npm uninstall -g opencode-chromium`.

## Stimmen-Befragung — die korrekte Route (gemessen 2026-10-03)

Drei Mechanismen, nie verwechseln. Die **scharfe Recherche** läuft nur über die UI;

die API-Flotte dient der **Masse/Reproduzierbarkeit**, nie dem scharfen Kanal.

1. **Scharfe Recherche → UI-Chats** (die einzige Route für Frontier-Modelle). Roster
   (Operator-Wort `future-folge169`, gefaltet `mycelium-folge225`): `chat.z.ai`
   (GLM-5.3 Geo/Deep Search) · `claude.ai` · `kimi.ai`. **Kimi K3 nur über
   `tryingopen.com`** — die Ausgabe ist auf **~4000 Zeichen** begrenzt: mit `Weiter`/
   `continue` den Rest holen, ggf. mehrfach (der Operator-Hinweis 2026-10-03); für Sonnet 5.5 Search
   `arena.ai/search/direct?model_a=claude-sonnet-5-search`. Prozedur (Selektoren, Senden,
   Antwort-Lesen, Aufräumen): `docs/surveys/survey-2026-09-24-fremdmodell-bedienung.md` +
   `state/stimmen/README.md`; Rohantworten privat nach `state/stimmen/<TS>_<site>_<slug>.md`.
2. **Masse/Reproduzierbarkeit → API-Flotte**: `bin/text_review <draft>` (Modelle in
   `tools/measure/free_models.tsv`); `bash state/stimmen/stimme.sh <zai|gemini|mistral> <prompt.txt>`.
3. **Öffentlich messende Stimme → `voice`-Agent** (`opencode.json`): read-only, bash nur
   `./bin/archive_search_public <öffentlicher-modus> <query>` + `./bin/voice_read_full <spill>`;
   Aufruf `opencode run -m <provider/model> --agent voice`.

Grenzen / gemessene Ausfälle — **nicht** als scharfen Kanal verwenden:
- `nvidia/z-ai/glm-5.3` via `opencode run --agent voice` hing 15 min ohne Output; nur ein
  paar nvidia-LLMs sind nutzbar (River-76-Rang ist alt — der `voice`-Agent trägt die
  Frontier-Modelle **nicht** zuverlässig; die UI ist der Weg).
- `text_review` über die blockierten `zai`-Zeilen (`free_models.tsv` `blocked`) hängt; `--all`
  ist langsam. Für eine kleine Auswahl `--model`/`--provider`/`--timeout` setzen.
- `kimi.ai` liefert **kein K3** (schneller Modus, Kontingent schnell erschöpft); K3 nur
  `tryingopen.com`. Together liefert schwache/keine Antworten.
- **ToS**: `claude.ai`/`kimi.ai`/`arena.ai` untersagen Automatisierung — nur mit Operator-Wort,
  nie still; das Konto-/Sperr-Risiko trägt der Operator (Quelle: `state/stimmen/README.md`).
- **Cloud-Grenze**: nur public/technische Prompts (keine `state/`-, Register- oder Secret-Daten);
  die Antwort ist Rohmaterial, nie Verdikt. Ein UI-Prompt ist ein Dritt-Akt → per-Akt-Consent.

## Profile (aus AGENTS.md) — gemessene Kosten ihres erlaubten Satzes

| Profil | Agenten | Darf | Messung |
|---|---|---|---|
| P1 | build | edit + full bash | alle oben; teure Builds sind CI |
| P2 | explore, council | git-read + `sgrep` | `sgrep` 0,04–0,08 s |
| P3 | general, research-max | `archive_search`/`sfetch`/`curl`/`proton-wg` + git-read + `sgrep`/`omega_sh` | Netz-Modi 0,5–24 s, lokal 0,02–0,11 s |
| P4 | plan | `register_lookup` + `git_safety` + `open_points_check` + git-read + `sgrep` | 0,05 + 1,2 + 0,07 s |
| P5 | grind-* | edit + full bash | wie P1 |
| P6 | vision | nichts (kein edit, kein bash) | — |

## Agenten-Benchmark (2026-09-16, identischer Task: dieselben vier Befehle, alle 8 Profile)

Lauf 3, nach der Werkzeug-Grenze in der System Directive und den Struktur-Sperren.
Werkzeug-Zeiten, identisch über alle Profile: `--root --count` 0,026–0,035 s;
`sgrep -i` 0,038–0,069 s; `--index` 0,020–0,037 s; `--crossref` 0,54–3,14 s
(Netz; bei 8 parallelen Aufrufen 2× HTTP 429 — Rate-Limit, als `pending` gemeldet).

| Agent | Modell | Cost | cache_read | Dauer |
|---|---|---|---|---|
| general | flash | $0.0008 | 51k | 6 s |
| explore | flash | $0.0015 | 25k | 6 s |
| plan | flash | $0.0017 | 47k | 5 s |
| grind-flash | flash | $0.0031 | 35k | 7 s |
| research-max | pro | $0.0115 | 53k | 27 s |
| grind-max | pro | $0.0116 | 28k | 19 s |
| council | pro | $0.0117 | 28k | 27 s |
| grind-pro | pro | $0.0119 | 28k | 26 s |

Anmerkung 2026-10-05 (Operator-Wort): der `council`-Agent läuft **flash/low** — die fünf
Stimmen sind eine Prompt-Struktur, keine Modell-Stufe; der pro-Wert oben bleibt die
historische Messung. Die Politik steht in `AGENTS.md` (cost ladder + Modell-/Thinking-Politik).

Pro/max kostet 4–15× flash bei identischem Ergebnis — flash-first bestätigt,
Sieger flash (`general` $0.0008). **Reibung gemessen:** Lauf 2 (vor der Grenze)
hatte 1–2 verweigerte Werkzeug-Versuche je Profil (plan/general/research-max);
Lauf 3 (nach der Grenze im System-Directive-Text + den Sperren) hatte **0
Versuche** in allen 8 Profilen — kein Agent probiert mehr ein verbotenes Werkzeug.
`vision` (P6, kein bash) ist mit Kommando-Tools nicht benchmarkbar.

## Benchmark (wiederholbar)

Der komplette Lauf ist diese Datei selbst: jede Zeile der Tabellen ist ein
Befehl — `time <befehl>` je Modus, Einzellauf, `src/`-Query `purpleair` bzw.
die genannten kleinen Queries. Das Skript braucht kein `grep` — nur `time` und
`wc -l`.

## archive_search — the divers' research tool

`archive_search` is on PATH (symlink to `bin/archive_search`) — the
self-contained research tool (Rust std + curl + own parse — never
webfetch/websearch). The wrapper `bin/archive_search` never builds — freshness comes from the rolling
release `tools-latest` via `bin/.tools_ensure`; a stale/absent artifact reads
`pending`, named, never a silent zero. One mode per source:
`--arxiv|--ads|--ntrs|--wayback|--crossref|--wiki|--github|--crates|--librs|
--brave|--datacite|--zenodo|--isc|--openalex|--base|--supermag|--heasarc <query>`;
`--all <query>` runs every source; `--playwright <url|query>` renders a page;
`--leads <keyword>` scans un-registered candidate homes; `--verdict <url>` /
`--sniff <url>`. A session that dispatches a diver (`taucher`)
names this tool in the delegation — the standard web tools are the
slow, expensive fallback, not the first move.

### Local search — three modes (do not confuse them)

- **Content in the live tree** (find a string): `archive_search <keyword> --root
  <dir>` — caps, match-ranking, binary handling, `--count`, `--case` — or the
  lean `sgrep` (`-i` for case-insensitive). This is the grep the agents use;
  the canonical map is `docs/concepts/tools-map.md`.
- **Path / filename** (not content): `archive_search --index [<query>] [--path]`.
- **Raw NTFS device / deleted files** (forensics): `archive_search --mft
  <device>` — needs a device path, not the live repo (which is not NTFS).

### The cost ladder — the research cascade

The tool speaks to **more than 50** network modes, not three (`archive_search --help` is the canon). The cascade, in order — and
for source *discovery* the breadth is the answer, not a cost:

1. **Content in the live tree** → `archive_search <kw> --root <dir>` or `sgrep`.
2. **Known URL** → `archive_search --verdict <url>` — the reachability ladder
   itself: stage 1 direct → stage 2 Proton exit → stage 3 Wayback. No manual
   `proton-wg.sh` rotation; the tool rotates on 403/429. Then `--sniff <url>`
   (magic bytes + sha256), then `--playwright <url>` when the plain fetch
   carries no content (JS-rendered).
3. **Known source** → the one mode the question needs: `--ads`, `--arxiv`,
   `--crossref`, `--ntrs`, `--openalex`, `--github`, `--heasarc`, …
   **Dataset/repository discovery → `--base` first** (BASE, Bielefeld Academic
   Search Engine, ~400M open documents, `BASE_API_KEY`): it indexes repository and
   dataset DOIs (NASA PDS, GFZ Data Services, VizieR, institutional repositories)
   that arXiv/Crossref/OpenAlex miss — measured 2026-10-13, it surfaced the open
   Venus-radar PDS bundle and the ALFALFA catalogue DOI where the other arms did not.
4. **Unknown source** → run the web-search engines directly: `--tavily`, `--exa`,
   `--linkup` (keyed — read from `.secrets.local`; the wrapper exports
   `OMEGAFLOW_REPO` and the binary resolves the repo from its own path, so they
   hold from any cwd), plus `--marginalia`/`--mwmbl` (keyless). `--brave` runs
   explicitly (HTTP 402 while its free quota is spent). `--all <query>` runs
   every keyword mode **sequentially** (`net.rs:3559`, `all_lines`) — the wall-clock
   is the sum of every mode's latency (curl + retries + the direct→Proton→Wayback
   ladder), and the output is written only after the whole loop; the count is the
   tool's own (`--help`, filled by `query_mode_count()`; measured 2026-10-10: the
   installed binary reported 62 while `QUERY_MODES` carried 64 in the tree) — the
   number drifts, the canon is the tool's own `--help`, and its full output lies in
   the temp file. Slow, breadth, not the first move. The key=value modes (`--isc`/`--cod`/`--biomodels`/`--entrez`/
   `--ena`/`--supermag`/`--heasarc`) run individually. It prints the
   top 5 per source inline **and writes the full result to a temp file** (the
   paged sources openalex/zenodo run ~100 deep) — read that file; the inline
   summary is not the whole answer. A diver that draws only `--ads`/`--arxiv`
   has left 36 modes unasked.
5. **JS-rendered page** → `--playwright <url|query>` (real browser render).

The three measures are distinct: `--verdict` measures reachability, `--sniff`
measures the file type, `--playwright` measures the rendered content. A stage-3
`503`/absent on `--verdict` is not a dead end — `--brave` finds the mirror,
`--playwright` renders the JS page.

**Recherche wird nie auf ein Minimum gekastet (Operator-Wort 2026-10-10).** A measurement is
read to its measured end. The inline summary is a signpost, never the result:
`archive_search --all` prints at most five lines per source inline and writes the full result
to the `full: <pfad>` temp file — **that file is read to its end**, not the preview (the paged
sources run far deeper than the five shown; a single `--all` run is a breadth measurement, and
its breadth lives in the file). A document is read to the end the claim needs, never the first
window and stop. A selection is a measurement only when the selection itself *is* the
measurement (an explicit criterion stated with the finding); a silent cap — "top 5", "die
wichtigsten", "excerpt", the first page, the first hits — presented as a finding is an
unmeasured claim = fabrication. A diver that returns the preview has not measured. This is not
a licence to inflate context: the *relevant* full extent is read (the source list, the response
file, the register), while grep-hit windows stay deliberate.

**Geo-suspect and Cloudflare.** A blocked direct route or a Cloudflare
interstitial is a measured state, not a reason to bypass silently. The tool
prints `geo-suspect: .<tld> -> proton-wg.sh <cc>` (a suggestion — `bin/proton-wg.sh
suggest <host>` names a country exit if a free config exists) or `bridge: …`
when the interstitial did not clear. The session turns that line into a
**question to the operator** before rotating the exit: a geoblock bypass
touches the source's terms and, for protected works, anti-circumvention law
(§ 95a UrhG / DMCA §1201); the access route's state is registered
(`geo`/`key-needed`/`blocked`), never silently bypassed. The `bin/proton-wg.sh *`
rotation is an `ask` in the permission map — the operator sees the dialog.
Cloudflare hosts run through the browser bridge (the operator's profile, which
passes the managed challenge) or the per-host API/mirror; `--playwright
--headed` (a persistent profile under `~/.cache/omegaflow/playwright-profile`)
is the browser path for a display.

## Local tools — the self-contained path

The project's own tools live on `PATH` (via `~/.local/bin`, built from
`tools/utils`): `sgrep` (grep), `sfetch` (fetch), `omega_sh`
(`reports|status|search|fetch|jwst`), `smail` (mail), `ci_manage` (GitHub Actions
runs: `status`/`list`/`view`/`jobs`/`log`/`cancel`/`rerun` — the CI reading, never `gh run
list`/`gh run view`; `status` shows the current step of every running run,
`jobs <run-id>...` the per-job step detail, `log <run-id> [--all]` the failed job logs),
`sread` (file with offset/limit), `register_lookup`, `open_points_check`,
`git_safety`, `session_burn`. The full map is `docs/concepts/tools-map.md`; at a conflict the
tool's own `--help` holds. They are Rust std + curl,
allowed to every agent — prefer them over the standard `webfetch`/`websearch`
(now denied) and over spawning a fresh process where one of them fits.
Reachability and file type run through `archive_search --verdict <url>` (direct →
Proton → Wayback) and `--sniff <url>` (magic bytes + sha256) — not `curl -sI`;
`curl` stays for binary/Zip content no reader mode carries. Write the step that
way in handovers, so the next session inherits the convention.

**Werkzeug-Nutzung — die Form (Rat + Operator-Wort, 2026-09-27; `--help` ist der Kanon).** The mode catalog lives in the tool itself: read `--help` before a new mode, never trust a prompt list. The forms: `archive_search` — the cascade known URL → `--verdict` → `--sniff` → `--playwright`; known source → the one mode; unknown → the search engines (`--tavily`/`--exa`/`--linkup` keyed, `--mwmbl`/`--marginalia` keyless); local `--root`/`--index`/`--git`/`--leads`; PDF `--pdf-text`, else `--pdf-image` → vision; `--all` is breadth (the tool's own `--help` count; sequential, the sum of every mode's latency), never the first move, and its full output lies in the temp file (`full: <pfad>`) — read it. `sgrep` — `-i` case-insensitive, `-l` paths only, `-c` count, `--all` for gitignored (direct file paths always read). `sread` — always `--offset`/`--limit`. `sfetch` — `--title`/`--links`. `register_lookup` — `<term>` (is X already measured?), `--open`, `--fired`, `--stale`, `--orphans`, `--orphan-docs`, `--addressed <line>` (the `## An <line>` blocks addressed to the own line across the live handovers, sender-named; never a whole foreign-handover read), `--descoped-check`. `git_safety` — `--snapshot` at start/end, `--close [own paths]` before the commit. `ci_manage` — `status`/`list`/`view`/`jobs`/`log`, never `gh run` (the run-level `updated_at` of `list` is coarse — the current step comes from `status`/`jobs`). `omega_sh` — `sha <datei>`, `reports|status|search|fetch|jwst`. `smail` — `--dry-run` only; the send is the operator's hand. `session_burn` — after the atom. The canonical tools run through the `bin/` wrappers (never `./target/release/*` directly): the wrapper enforces freshness via `bin/.tools_ensure` (content-addressed sha256, TTL 2⁶ min; a stale or absent artifact is named `pending`, never a silent zero); the meta pass runs one `.tools_ensure` sweep per round. CI runs are analyzed **in detail, not only success/fail**: the failure reason is read from the job log — `ci_manage log <id>`, the GH-API directly (`https://api.github.com/repos/omegaflow/omegaflow/actions/runs/<id>/jobs`), or the browser (`browser_snapshot`/`browser_get_text` = content, never pixels) — never conjectured; a red run without a measured reason is `unread`, named, never filled. All agents run in the repo root (delegations name absolute paths); the wrappers export `OMEGAFLOW_REPO`, so the keyed `archive_search` arms resolve `.secrets.local` from any cwd.


## The repo — layout and stack

Kybernetic field system — pure Rust, WebGPU point cloud, ICRS block universe. The bins live in the functional crates under `tools/`: `omegaflow-harvest` (the harvesters/compilers), `omegaflow-measure`, `omegaflow-register`, `omegaflow-service`, `omegaflow-science`, `omegaflow-gate`, `omegaflow-utils`; each runs as `cargo run -p omegaflow-<fkt> --bin <name>`. `src/` is the one core crate (Archivar + Mathematikerin + the gate modules). `cargo build` builds only the core. A run is headless by default (no window package); `--features browser_relay` opens the browser membrane at `127.0.0.1:1618`. Stack: Rust `std` + `curl` + `serialport`; Vanilla JS ES modules; WebGPU WGSL; binary φ(x,y,z,t) protocol (`0xCF 0x86`, little-endian); φ suffix for config files.

**Internal vs external tool.** The complete intelligence lives under omegaflow name — an internal tool is self-contained (own fetch, own parse, own logic) and never depends on omegaflow own external release; an external tool is a separate project that carries none of it out.
