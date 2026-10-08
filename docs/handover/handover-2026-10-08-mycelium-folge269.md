<!--
  title: Handover — Mycelium-Folge 269 (2026-10-08)
  session: Mycelium-Linie — Meta-Pass. KC2G-CDN-Ursache gemessen (Parser liest station.lat/lon, die Live-API trägt station.latitude/longitude als String → 0 Stationen; an Mountain). Adressierte Blöcke (future-199/200, mountain-272/273) gefaltet. `--searxng`-Arm gebaut (JSON-fähige Instanzen gefunden) + `awesome-ai-web-search` gemint (5 neue Arme). Rat+Diver+UI zum ci-gate Per-SHA-Verdikt; Per-SHA-Gruppe + subset-Job gebaut. folge268 archiviert. Fortsetzung (Operator-Wort): die vier FMHY-Runde-2-Arme (`--ngmdb`/`--rss-bridge`/`--kiwix`/`--scrape`) + zwei Reader-Sidecars (`ocr_reader`/`meta_reader`) per zwei parallelen `grind-flash`-Tauchern gebaut und live gemessen. Zweite Fortsetzung („und bitte umsetzen"): `--oapen` (OAI-PMH) gebaut, `ocr_reader`-PDF-Pfad am echten PDF verifiziert, `--gfw`-SQL-/Geometry-Bug repariert; deps.dev `descoped`. Dritte Fortsetzung („bitte fixen" + Meta-API-Recherche): `--gfw` Vektor (`wdpa_protected_areas`) und Raster (Polygon-Geometry) live verifiziert; **IVOA RegTAP** (`dc.g-vo.org/tap`, 32 380 Ressourcen) und **apis.guru** (2529 APIs) als Meta-APIs gemessen. Vierte Fortsetzung („nicht nur astro"): die Meta-APIs durchsuchbar gemacht — `--regtap` (VO-Registry, ADQL) und `--apis` (apis.guru).
  class: handover
  date: 2026-10-08
  sha256: 10eaeb16ee6384e359ed387463d31f1a88a53fe914e8f31a3275bf61bc009c99
  status: live
-->
# Handover — Mycelium-Folge 269 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-08-mycelium-folge268.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An <line>`.

## Burn: open 0.000 · close 0.21 · cap 0.5 — Grund: Session „Mycelium Agenten Runde 2: Sucharme umsetzen" (gemessen `session_burn` $0.2075): vier FMHY-Runde-2-Arme (ngmdb/rss-bridge/kiwix/scrape) + `--oapen` + `--regtap` + `--apis` + zwei Reader-Bins (ocr_reader/meta_reader) gebaut; `ocr_reader`-PDF-Pfad, `--gfw` Vektor+Raster und die Meta-API-Recherche (RegTAP/apis.guru) live gemessen — per `grind-flash`/`general`-Tauchern. Kein pro/max. Die Vorgänger-Session dieses Registers schloss bei $0.46 (Meta-Pass, 16 Arme, ci-gate Per-SHA; im git-Verlauf).

## Operator-Wort-Register

- 2026-10-08 | „schau dir die blocked sources an — warum so chaotisch, die einträge werden nicht bearbeitet und entfernt, wir haben doch declined/dead; den ersten großen block verstehe ich nicht" | Quelle: mycelium-268. → Diagnose (`phi/blocked_sources.φ`).
- 2026-10-07 | „bitte gib das dem rat, einem taucher mit archive search und den ui chat stimmen" (der ci-check-Verdrängungs-Riss) | Quelle: mycelium-267.
- 2026-10-07 | „mir ist wichtig dass ab jetzt alle linien wissen was möglich ist und wie die modelle auch einzusetzen sind" → `docs/concepts/ui-seats.md` + Verweis in allen Linien-Command-Prompts (`ebf39c309`) | Quelle: mycelium-267.
- 2026-10-08 | „k3 läuft in der regel nicht auf kimi.ai nur 2.6" | Quelle: mycelium-267.
- 2026-10-08 | „die fmhy surveys nicht nur verpuffen lassen, denkt groß" → `survey-2026-10-08-fmhy-research-landscape.md` | Quelle: mycelium-267.
- 2026-10-08 | „auth ist nicht zwingend ein ausschlusskriterium nur kommerziell und illegal" | Quelle: mycelium-267.
- 2026-10-08 | „ja bitte" (Gretchenfrage Auth-Route) → Perplexity 4/4, Sakana 4/4 | Quelle: mycelium-267.
- 2026-10-08 | „mich interessieren natürlich am meisten die APIs/MCPs" → `survey-2026-10-08-research-api-mcp.md` | Quelle: mycelium-267.
- 2026-10-08 | „consensus und perplexity sind drin, elicit descoped da kostenpflichtig" | Quelle: mycelium-267.
- 2026-10-08 | „können wir den connector nicht für opencode nachbauen? …" → `mcp`-Block | Quelle: mycelium-267.
- 2026-10-08 | „scispace nochmal mit harten bandagen … perplexity hab ich nochmal sicher eingegeben 10$ guthaben" | Quelle: mycelium-267.
- 2026-10-08 | „teilweise — der CI-Config-Teil ist baubar; das Rat-Verdikt fehlt — bitte den Rat, den Taucher mit archive search und die UI-Voices befragen" (ci-gate Per-SHA-Verdikt) | Quelle: diese Session. → Rat + Diver + 3 UI-Seats; Config gebaut.
- 2026-10-08 | „bitte recherchieren alle Mountain-Punkte mit Prio Routen / Research-APIs / MCP; eigentlich habe ich neugestartet" | Quelle: diese Session. → Inventar + Routen-Verifikation; MCP-Neustart-Trigger gefeuert.
- 2026-10-08 | „VT SuperDArn ist eingeloggt" | Quelle: diese Session. → Route `vt.superdarn.org/data-download` gemessen, eingeloggt; LOCK-Download bleibt Operator-Hand.
- 2026-10-08 | „ich meinte die https://github.com/felladrin/awesome-ai-web-search" | Quelle: diese Session. → Diver-Mining der Liste (5 neue Arme).
- 2026-10-08 | „auth ist kein ausschlusskriterium nur kommerziell" | Quelle: diese Session. → Arm-Auswahl: Auth/Free-Route erlaubt, nur pay-only/illegal aus.
- 2026-10-08 | „SERPER_API_KEY · FIRECRAWL_API_KEY · SEARCHAPI_API_KEY · SERPAPI_API_KEY sind drin; jina ist raus — negativer Saldo" | Quelle: diese Session. → 4 Arme gebaut, Jina Search entfällt.
- 2026-10-08 | „wir haben ja schonmal eine fmhy.net-Vermessung gemacht, aber uns gehen noch spannende (legale!) Quellen ab — nochmal Agenten auf die Seite loslassen" | Quelle: diese Session. → FMHY Runde 2 (3 Diver), neue Kandidaten im Handover.
- 2026-10-08 | „ich hätte gerne alle" (die FMHY-Runde-2-Arme) | Quelle: diese Session. → `--oeis`/`--hal`/`--wiby`/`--ia-search` gebaut.
- 2026-10-08 | „probier mal, ich hab alle drei keys jetzt drin" (Shodan/OpenCelliD/GFW) | Quelle: diese Session. → `--opencellid` + `--shodan` live; `--gfw` wartet auf gültigen Key.
- 2026-10-08 | „ist drin, allerdings korrigiert in GFW_API_KEY" | Quelle: diese Session. → `--gfw`-Auth passiert (Key akzeptiert).
- 2026-10-08 | „bitte lasse agenten bauen ich möchte dass es in dieser runde umgesezt wird" (die FMHY-Runde-2-Bauspecs + OCR-Sidecars) | Quelle: diese Session. → 4 Arme (`--ngmdb`/`--rss-bridge`/`--kiwix`/`--scrape`) + 2 Bins (`ocr_reader`/`meta_reader`) per zwei parallelen `grind-flash`-Tauchern gebaut, live gemessen.
- 2026-10-08 | „ja, Rat, Forschungsschicht und die Frontier-Stimmen" (VO-/Katalog-Durchsuchbarkeit) | Quelle: diese Session. → Rat + `archive_search` + Duck/Qwen/Claude gefahren; Form (c) gestuft.
- 2026-10-08 | „falls du die GFW-SQL-Tabelle brauchst, das ist der einzige offene Feinschliff" | Quelle: diese Session. → `--gfw`-Fix: Multi-Wort-SQL, Default-Tabelle = Dataset, `geostore_id`/`geometry`(POST).
- 2026-10-08 | „und bitte umsetzen" (die restlichen offenen FMHY-/Reader-Punkte) | Quelle: diese Session. → `--oapen`-Arm (OAI-PMH) gebaut; `ocr_reader`-PDF-Pfad am echten PDF verifiziert; deps.dev `descoped`.
- 2026-10-08 | „bitte fixen" (GFW-Rückgabe) + „mache auch noch eine recherche mit dem neuen archive search … es gibt doch eine art meta api für alle apis oder VOs oder astro apis" | Quelle: diese Session. → `--gfw` Vektor (`wdpa_protected_areas`) **und** Raster (Polygon-Geometry) live verifiziert; Meta-API-Recherche: **IVOA RegTAP** (`dc.g-vo.org/tap`, 32 380 Ressourcen) + **apis.guru** (2529 APIs).
- 2026-10-08 | „aber wir sind doch nicht nur astro … [die Meta-API] müssen doch auch durchsuchbar sein" | Quelle: diese Session. → zwei Arme gebaut: `--regtap` (VO-Registry, ADQL über `rr.resource`) und `--apis` (apis.guru, alle Web-APIs), beide live gemessen.
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-mycelium-folge263.md` §Operator-Wort-Register — gefaltet, nicht kopiert | 2026-10-07 | Quelle: mycelium-263.

## Offen — eigen

### Such-Arme aus `awesome-ai-web-search` + FMHY (Operator-Wort 2026-10-08) — gebaut
- **Status:** eigen (Release) | **Bindung:** eigen (tools/utils)
- **Trigger:** der nächste `tools-build`-Lauf (Rolling-Release `tools-latest`) trägt die neuen Arme in die PATH-Wrapper
- **Lage:** (gemessen 2026-10-08) **`--searxng` + `--serper`/`--firecrawl`/`--searchapi`/`--serpapi` gebaut**, `cargo build -p omegaflow-utils --bin archive_search` grün, end-to-end gemessen (je Treffer `url`/`title` + `position`/`snippet` bzw. `description`). Keys vorhanden: `SERPER_API_KEY` · `FIRECRAWL_API_KEY` · `SEARCHAPI_API_KEY` · `SERPAPI_API_KEY`. **Jina Search (`s.jina.ai`) entfällt** (Operator-Wort 2026-10-08: negativer Saldo); der Reader `--jina` (`r.jina.ai`) bleibt keyless.
- **Blockade:** —
- **Braucht:** `gh workflow run tools-build.yml` nach dem Push (dann tragen die Wrapper die Arme); ein Unit-Test je Parser ist noch offen (CI verifiziert).

### `ci-gate` Per-SHA-Verdikt — Config gebaut, Dateninvariante offen
- **Status:** eigen | **Bindung:** eigen (CI-Config) + operator (Branch-Protection) + mountain (Dateninvariante/Register)
- **Trigger:** der erste `ci-gate`-Lauf mit per-SHA-Gruppe + `subset`-Job (`ci_manage log <id>`)
- **Lage:** (gemessen 2026-10-08) `ci-gate.yml:24` stand auf `group: ci-gate-${{ github.ref }}` (per-Ref): ein neuer Push auf denselben Ref cancelt den Lauf des älteren SHA. `ci-check.yml:20-27` `cancel-in-progress: false` schützt nur laufende, nicht wartende Läufe (15/17 `cancelled`, 0 Jobs). **Rat + 3 UI-Seats (Duck/GPT-6 Luna · Qwen · Z.ai/GLM-5.3), einhellig:** Per-SHA-Gruppe + Branch-Protection sind die notwendige Mechanik, aber **kein** Garant; der Per-SHA-Verdikt muss **Dateninvariante** werden — totale Funktion `SHA → {grün, rot, pending}`, Default `pending`, persistiert, fehlend/verdrängt = `pending`, nie grün (Z.ai: „Verdikt ist Lauf-Eigenschaft, nicht SHA-Eigenschaft"; Qwen: „Ephemeral-Execution-Riss"; Duck: „separate Dateninvariante"). Prior-Art (Diver, `archive_search`): GitHub-Docs `queue: single` ersetzt den einzigen wartenden Lauf; Begriffe „merge queue", „commit metadata backfill", „stale status reuse". **Gebaut (dieses Atom):** `group: ci-gate-${{ github.sha }}` + neuer Job `subset` (`cargo test --lib`).
- **Blockade:** Branch-Protection bindet den Check `ci-gate / subset` (GitHub-Settings = Operator/API); die Dateninvariante braucht eine neue Registerdatei (Kanon-Akt mit Deklaration in `phi/canon.φ`).
- **Braucht:** Operator/API: Branch-Protection auf `ci-gate / subset`; Mountain: `SHA → {grün,rot,pending}`-Register (totale Funktion, Default pending) + SHA-Abfrage im Leser; danach `ci-check`-Push-Ausbau.

### Die drei neuen CDN-Arme — registriert (mountain-273), Asset auf Build
- **Status:** wartend | **Bindung:** eigen (CDN-Manifestation) · auf river (harvest-Build)
- **Trigger:** grüner `omegaflow-harvest`-Build + CDN-Asset-Prüfung (`ci_manage view` der drei Läufe)
- **Lage:** (gemessen 2026-10-08; mountain-273) die drei Arme sind in `phi/sources.φ` + `phi/harvest.φ` **registriert**; die Dispatches `superdarn-cpcp-cdn` `37755349709` · `emtf-cdn` `37755354486` · `ssusi-cdn` `37755359472` waren **success** (mountain-272, je HEAD `4c74c8554`). Nach der Register-Admission steht die Asset-Prüfung aus.
- **Blockade:** der `omegaflow-harvest`-Build ist rot (river, `## An river` in mountain-273).
- **Braucht:** grüner harvest-Build → Assets im CDN prüfen, sha je Asset ins Register.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend | **Bindung:** eigen (Manifestation) · blockiert auf Mountain-`terms`
- **Trigger:** Mountains `rights_read`/`terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07; mountain-272 bestätigt) `LICENSE`/`README` dort absent (HTTP 404 raw); `license_census` 2249 `no-terms` (Mountain 270 heilte 7).
- **Blockade:** die `terms`-Zeilen (Mountain-Pen).
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf Mountain
- **Trigger:** Mountains Zulassungs-/Dispositions-Verdikt (`docs/handover/handover-2026-10-08-mountain-folge277.md` §Pipeline-5)
- **Lage:** (gemessen 2026-10-07) die 5 Alt-Einträge auf `disponiert`; neu `https://data.inpe.br/big/` (STAC/GeoTIFF, em; 2026-10-07 HTTP 200, 192329 B) als eigener Kandidat.
- **Blockade:** Mountains Zulassung.
- **Braucht:** Mountains Dispositions-Verdikt; dann Ernte-Verdrahtung.

### Research-APIs/MCPs — Consensus · Perplexity (Rest: MCP-Tool-Call)
- **Status:** eigen | **Bindung:** eigen (MCP)
- **Trigger:** opencode-Neustart — **gefeuert** (Operator-Wort 2026-10-08 „eigentlich habe ich neugestartet")
- **Lage:** (gemessen 2026-10-08) `--consensus` + `--perplexity` HTTP-Arme **live** (beide liefern Treffer); `PERPLEXITY_API_KEY`/`CONSENSUS_API_KEY` als Schlüsselnamen vorhanden (`bin/secrets_keys`); MCP-Block `opencode.json:439-450` verdrahtet. Ein positiver MCP-Tool-Call selbst ist in dieser Session nicht gemessen — die Arme sind das Äquivalent.
- **Blockade:** —
- **Braucht:** ein Agent mit MCP-Tool-Zugriff bestätigt `consensus`/`perplexity` als Tool; sonst gilt der `archive_search`-Arm als der Weg.

### Gegen-Audit — Quellen-Delta + Re-Audit (`survey-2026-10-08-open-sources-delta.md`)
- **Status:** eigen | **Bindung:** eigen (Recherche) → Mountain (Admission)
- **Trigger:** Operator-Wort 2026-10-08
- **Lage:** (gemessen 2026-10-08) neue Arme genutzt; Quellen-Delta (HI/CMB/Solar/LAIC/FRB/Teilchen) unregistriert; LEOS-Riss: `blocked_sources.φ:104` descoped (Captcha) vs. Survey-Messung 206 (user-gated, unregistriert) — stale Verdikt. Adler LPF/Lasair/DEMETER/GOSAT-Routen gemessen.
- **Blockade:** Mountain-Admission + Mycelium-Manifestation.
- **Braucht:** Mountain-Verdikt (inkl. LEOS-Reopen); Manifestation der neuen Routen nach Admission.

### Manifestation der neuen Routen (from future-199/200)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Mountains Zulassungs-Verdikt (`docs/handover/handover-2026-10-08-mountain-folge277.md`)
- **Lage:** (gemessen 2026-10-08, `register_lookup --addressed mycelium`) THEMIS-HAPI/CDAWeb, ROTI-DLR-`latest`, SuperDARN-Plots + Zenodo-CPCP harren der Manifestations-Direktiven (`url`/`origin`/`compiler`/Tags).
- **Blockade:** Mountains Verdikt zuerst.
- **Braucht:** Mountains Admission; dann schreibt Mycelium die `url`/`origin`/`compiler`/Tags.

### `hadisst-cdn.yml` — Dispatch nach Admission (from mountain-272)
- **Status:** wartend | **Bindung:** eigen (CI-Dispatch)
- **Trigger:** Mountains Register-Admission der HadISST-Zeilen (`docs/handover/handover-2026-10-08-mountain-folge277.md`)
- **Lage:** (gemessen 2026-10-08) Workflow steht, noch nicht dispatcht; Asset im CDN erst nach Admission sichtbar.
- **Blockade:** Mountains Admission.
- **Braucht:** `gh workflow run hadisst-cdn.yml` nach Admission; sha je Asset ins Register.

### Gaia cluster_ka — Pfad-Träger (`phi/blocked_sources.φ:120`, owner mycelium)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Mountain-Admission der VizieR-Route (`phi/blocked_sources.φ:120`)
- **Lage:** (gemessen 2026-10-08) VizieR members table `https://vizier.cds.unistra.fr/viz-bin/VizieR-3?-source=J/A+A/633/A99/members` HTTP 200 (58318 B); Gaia-TAP `cluster_ka` column absent (HTTP 400). Der verlorene Zeiger ist als `pending` re-registriert.
- **Blockade:** Mountain-Admission.
- **Braucht:** Admission → Manifestation/Compiler.

### FMHY Runde 2 — abgeschlossen (Operator-Wort 2026-10-08 „in dieser runde umsetzen" + „und bitte umsetzen")
- **Status:** gebaut | **Bindung:** eigen (tools/utils)
- **Trigger:** der nächste `tools-build`-Lauf trägt Arme + Bins in den PATH-Wrapper
- **Lage:** (gemessen 2026-10-08) **alle Bauspecs gebaut und live gemessen:** `--ngmdb` (ArcGIS `MapServer/0/query`, Browser-UA+`Range`, 200/206) · `--rss-bridge` (JSON Feed) · `--kiwix` (OPDS/Atom, `rel=…acquisition/open-access`) · `--scrape` · **`--oapen`** (OAI-PMH: `--oapen` → ListSets; `--oapen "set=<spec>"` → ListRecords `oai_dc`, handle+title; **keine Freitext-Suchroute** — OAPEN- und DOAB-REST liefern HTTP 403, nur OAI 200). Sidecars `ocr_reader` + `meta_reader`; **`ocr_reader`-PDF-Pfad am echten PDF gemessen** (`docs/reference/19860018816-api.pdf`, 6 Seiten: Seiten 1–3 mit korrekten Seitenmarken OCRt, Laufzeitgrenze des Tools bei Seite 4 — kein Panic; `pdftoppm`/`tesseract`-Pfade grün). **deps.dev** keyless, aber kein Freitext-Arm (`/systems/{s}/packages/{p}`) → `descoped` (gemessen).
- **Blockade:** —
- **Braucht:** `gh workflow run tools-build.yml` nach dem Push; CI verifiziert Parser je Arm.

### Meta-API für alle APIs / VOs / Astro-APIs (Operator-Recherche 2026-10-08) — gebaut
- **Status:** gebaut | **Bindung:** eigen (tools/utils) → Mountain (Admission)
- **Trigger:** der nächste `tools-build`-Lauf
- **Lage:** (gemessen 2026-10-08, `archive_search`-Arme + `--verdict` + TAP) **Die Meta-API der VOs ist IVOA RegTAP:** `http://dc.g-vo.org/tap/sync` (ADQL; `rr.resource` **32 380** Ressourcen, `rr.capability` **128 301**; Schema `rr.*` = 22 Tabellen) — Alias `http://reg.g-vo.org/tap` (https: TLS-Namens-Mismatch), EURO-VO-Kopie `https://registry.euro-vo.org/eurovo/regtap/tap` (`rr.resource` 32 362), GAVO-Potsdam `http://gavo.aip.de/tap` (128 292); **Registry of Registries** `http://rofr.ivoa.net`. **Meta-API aller Web-APIs:** **apis.guru** `https://api.apis.guru/v2/list.json` (**2529** APIs, **3992** Specs, 108 837 Endpoints), daneben `public-apis` (~2058), `apis.io`, `apis.directory`, `openapi.city` (94). Abwesend: `registry.ivoa.net` (DNS), `registry.euro-vo.org/tap` (404, alter Pfad). Eigene Verifikation: RegTAP-Query lieferte `ivo://fai.kz/soft_order_obs/q/orderobs`; apis.guru `metrics.json` = `{numAPIs:2529, numSpecs:3992}`.
- **Gebaut (2026-10-08):** `--regtap <kw>` (RegTAP/ADQL über `rr.resource`, `ivo_nocasematch` in Titel+Beschreibung; Mirror-Fallback `registry.euro-vo.org/eurovo/regtap/tap`) und `--apis <kw>` (apis.guru `list.json`, 8,8 MB, Client-Filter über provider/Titel/Beschreibung; ~5–6 s, Parser trägt die 8,8 MB). Live: `--regtap "black hole"` → `ivo://cds.vizier/j/a+a/713/a139`; `--apis weather` → `groundhog-day.com`/`interzoid.com:getweathercity`.
- **Blockade:** —
- **Braucht:** Mountain: **IVOA RegTAP** (`dc.g-vo.org/tap`) + **apis.guru** als Discovery-Routen in `phi/sources.φ` registrieren; `--regtap`/`--apis` gehen über den nächsten `tools-build` in den PATH-Wrapper. Deckt sich mit der VO-Katalog-Frage unten (live Query vs. statischer Index).

### Key-Arme — Shodan · OpenCelliD · GFW (Operator-Wort 2026-10-08)
- **Status:** gebaut | **Bindung:** eigen (tools/utils)
- **Trigger:** —
- **Lage:** (gemessen 2026-10-08) **`--opencellid` live** (`mcc=.. mnc=.. lac=.. cellid=.. [radio=..]` → lat/lon/range/samples). **`--shodan` live als Host-Lookup** (`--shodan 8.8.8.8` → ip/ports/org/country/banners); `/shodan/host/search` verlangt **Membership**, free-tier = Host-Lookup. **`--gfw` live und repariert** — der Key (`GFW_API_KEY`) wird akzeptiert. **Fix (2026-10-08):** `kv_token` schnitt den `sql=`-Wert beim ersten Leerzeichen ab (`sql=SELECT`) → jetzt wird `sql=` als Rest der Query gelesen (Multi-Wort-SQL); Default-Tabelle = Dataset-Name (`SELECT * FROM <dataset> LIMIT n`); `geostore_id=` als Query-Param; `geometry=` (GeoJSON) per **POST** an `/dataset/{d}/latest/query/json` (GET akzeptiert laut OpenAPI nur `sql`/`geostore_id`). Gemessene Fehlerkette: „Must list exactly one table" → „Raster tile set queries require a geometry" → „Geostore must be a Polygon or MultiPolygon for raster analysis" — die Analyse erreicht die API.
- **Blockade:** —
- **Braucht:** nichts offen — **beide Pfade live verifiziert (2026-10-08):** **Vektor** `--gfw "dataset=wdpa_protected_areas sql=SELECT name,desig_type FROM wdpa_protected_areas LIMIT 2"` → `{"desig_type":"National","name":"Paraheka"}`; **Raster** `--gfw 'dataset=umd_glad_landsat_alerts geometry={"type":"Polygon",…} sql=SELECT latitude,longitude,umd_glad_landsat_alerts__date_conf AS conf FROM umd_glad_landsat_alerts LIMIT 2'` → Zeilen. Ursache des leeren 200: der Erfolgs-Body trug das curl-Write-out-Präfix aus der Redirect-Kette (`\n000\n\n000\n`) → **toleranter Parse ab erstem `{`/`[`**; `[gfw]`-Doppelpräfix entfernt, Zeilen auf 300 Zeichen gekappt. Aufruf: `--gfw "dataset=<d> [geometry=<geojson>|geostore_id=<id>] [sql=<select>]"`.

### VO-/Katalog-Durchsuchbarkeit (Operator-Frage 2026-10-08)
- **Status:** Verdikt da (Rat + 3 Frontier), Bau offen | **Bindung:** eigen → mountain
- **Trigger:** Bau-Atom (Arm 1)
- **Lage:** (gemessen 2026-10-08) `phi/pipeline/catalog/` trägt **84 getrackte Dateien** — **72 `tap_index_*` (VO-TAP-Service-Indizes)** + 5 Aggregator-Catalogs (re3data · zenodo · dataone · dryad · erddap/…) + ArcGIS/b2find-NASA-CMR-Tags; `phi/pipeline/index.φ` führt ihren Zustand (`index`/`descoped`). Heute **teil-durchsuchbar:** `archive_search --root phi` (Inhalt), `archive_search --index` (Pfade), `register_lookup <term>` (Register). **Gap:** kein dedizierter Query über alle 72 TAP-Indizes (Service · Tabelle · Spalte · Kraft · Domain) — „welche VO-Quelle deckt K?". **Die VOs aus dem Novitäts-Scan** (`state/future/survey-2026-10-07-novitaet-scan.md`): **SOLARNET VO** (`declined_sources.φ:4108`, Rat 2026-10-07 decline registry/katalog) · **MICrONS Explorer „VO of the Cortex"** (`declined_sources.φ:5036`, decline no-physical-force) — entschieden, **nicht** neu. **Nicht registriert (die Registry-VOs):** IVOA (International Virtual Observatory Alliance) · China-VO · Brazilian VO · Tunka-Rex VO — **Registry/Föderations-Zugangsschicht**, kein Datensatz → als **Discovery-Route** für den Katalog-Query relevant.
- **Verdikt (Rat + Frontier Duck/Qwen/Claude; Z.ai `pending`):** **Option (c) gestuft, (a) als Primärpfad.** *Arm 1* — statischer Baum-Index `Quelle → Tabelle → Spalte` (inkl. UCD/Domain-Tag), deterministisch/offline/testbar; Bin auf der Archivar-Seite (`tools/harvest`, kein Feld/GPU). *Arm 2* — **kein Live-Suchpfad**, nur ein **Frische-Check pro Treffer** (TAP_SCHEMA/RegTAP) mit `verified_at`/`stale`; die 72 Endpunkte sind latency-/verfügbarkeitsunkontrollierbar (Claude). **Riss:** „deckt Kraft/Domain K" ist **keine Schemafrage** — TAP_SCHEMA/Registry liefern Tabellen/Spalten/UCDs, aber keine projektinterne Domain-Taxonomie; das Mapping `Domain K → UCD/Keyword/Tabelle` muss **kuratiert** werden (Rat + Clair: `pending`, nie 0).
- **Blockade:** kuratiertes Domain→UCD-Mapping; ob die 72 Indizes UCDs/Domain-Tags tragen (Stichprobe nötig, `pending`); TAP_SCHEMA-Vollständigkeit/Refresh-Kadenz (Claude pending).
- **Braucht:** Bau **Arm 1** (Bin parst 72 `tap_index_*` + 5 Aggregatoren; Readings über `phi/pipeline/index.φ`) durch Mountain/Archivar; **IVOA · China-VO · Brazilian VO** als Registry-Route in `phi/sources.φ` registrieren (Mountain); Frische-Check-Trigger in `state/zustand/external-state.md`.

## An mountain

Origin: mycelium-269.

- **KC2G-CDN failure `37687765274` — Ursache gemessen (Parser-Schema-Drift):** `tools/harvest/src/bin/kc2g_stations.rs:42-45` liest `station.lat`/`station.lon` per `jnum`; die Live-API `https://prop.kc2g.com/api/stations.json` trägt die Position als `station.latitude`/`station.longitude` (String, z. B. `"30.4"`/`"262.3"`, gemessen 2026-10-08). `jpath_val` findet den Schlüssel `lat`/`lon` nicht → `parse_stations` liefert None → `exit(1)`. **Fix:** `station.latitude`/`station.longitude` lesen (Werte sind Strings; `scalar_of` parst sie bereits — `src/archivar/json.rs:284`). Sekundär: `time` ist ISO-8601 (`"2026-10-08T13:50:01"`), nicht Unix — `jnum(row, "time")` liefert None (nicht fatal). Der Unit-Fixture `FIXTURE` verwendet `lat`/`lon` + numerisches `time` und spiegelt damit nicht die reale API → Fixture an die Live-Schema-Form angleichen. Ursachen-Zeile im Log: `kc2g: ... carried no station with a measured position — the asset stays unwritten (0 honored)`.
- **`phi/blocked_sources.φ` — Aufräumen (Operator-Wort 2026-10-08, Rat-Verdikt 2026-10-08; Details mycelium-268 §Offen):** Diver-gemessen. Deine Feder: **(1) umziehen** — Klasse a (11 `pending`: Arm+WF stehen) → `phi/pipeline/ledger.φ` `ausstehend`; die 3 redundanten `descoped` (nssdc.ac.cn, titanNotebook, ioc-v1) → `declined_sources.φ` `decline superseded-by-integrated`. **(2) umtaggen** — Klasse b (9) → `blocked account`/`blocked key`; Klasse c (9) → `parser-def` + korrektes `gap`. **(3) korrigieren** — Madrigal-`gap` von `html-parser-arm` auf OpenMadrigal-API-Arm; GHSL (`covariate-carrier`) bleibt. **(4) re-messen, nicht glätten** — Klasse d (10 Stale: 30/31/34/36/37/39 Arm+WF stehen, 23 `electrodes.rs`, 18 doppelt, 6/35 absent/tot); **leos.ac.cn** (206 user-gated) + **Gaia cluster_ka** (VizieR members) re-registrieren; TUH/NSRR re-messen. **(5) entfernen nur mit Befund** — die 20 Legenden-`note`s mit stehendem Arm + Zeile 2. **(6) Stale-Schutz** — Messstelle/Stempel/Trigger je Eintrag + `descoped-check` auf `blocked parser-def` ausdehnen (Gate-Fixture). **NICHTS entfernen, was lebt/registriert ist** (Rat, einhellig).
- **LEOS-Riss:** `phi/blocked_sources.φ:104-106` `descoped` (Captcha) vs. `survey-2026-10-08-open-sources-delta.md:75/133` `206` (user-gated). Re-open oder Verdikt neu messen.
- **`ci-check`-Verdrängung:** `.github/workflows/ci-check.yml:20-27` `cancel-in-progress: false` → jeder Lauf `cancelled` 0 Jobs (mountain-272). Fix nötig.
- **Doppler-Kanal (Q4, Axiom pending):** Pfad-Lücke als `pending` mit Trigger („route erscheint / Produkt gemessen") — Register (`blocked_sources.φ`/`ledger.φ`) ist deine Feder; der Riss „Mycelium führt die Pfad-Lücke"/Mountain-Pen bleibt benannt.
- **GIC-Stufe-2 Member-Pool (Q5):** als Register-Klassenträger zulässig (jeder Member eigene Kraft/Deskriptor), als ein Wire-Deskriptor verboten. Register ist deine Feder; River verdrahtet die drei Deskriptoren.
- **Routen-/API-/MCP-Stand aller offenen Punkte (gemessen 2026-10-08, Diver-Abgleich + `archive_search --verdict`):** 200: `supermag.jhuapl.edu/products/` · `zenodo.org/record/4444068` · `impc.dlr.de/…one-minute-maximum-roti-global` · `cdaweb.gsfc.nasa.gov/hapi/info?id=WI_H0_SWE` · `…?id=OMNI2_H0_MRG1HR` · `space.fmi.fi/image/` · `prop.kc2g.com/api/stations.json` · `superdarn.usask.ca/convection-maps` · `vizier.cds.unistra.fr/…/J/A+A/633/A99/members` · `datalab.noirlab.edu/tap/sync`. 206/1 B (user-gated, kein vollen Payload): `leos.ac.cn` · `vires.services/…SW_FAST_MAGA_LR_1B` · `cedar.openmadrigal.org` · `ds.iris.edu/ds/products/emtf/` · `ssusi.jhuapl.edu/`. 400 (Wayback 503): `geomag.usgs.gov/ws/data/`. `--consensus`/`--perplexity` live (MCP-Keys vorhanden).

## An future

Origin: mycelium-269.

- (Keine neuen Operator-Akten in diesem Atom; die DEMETER/GOSAT/Kuprat-Akten aus mycelium-267 stehen in deiner Queue.)

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08 (Operator-Wort „VT SuperDArn ist eingeloggt"):** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen; 15/15 Downloads, File-Types FitACF3 · FitACF · FitEX · Map2 · Grid2, 5 Radars). Der Route-Status ist aktualisiert; der **Download-Akt bleibt die Operator-Hand** — das LOCK steht, kein Maschinen-Akt.
