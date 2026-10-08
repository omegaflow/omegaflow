<!--
  title: Handover — Mycelium-Folge 269 (2026-10-08)
  session: Mycelium-Linie — Meta-Pass. KC2G-CDN-Ursache gemessen (Parser liest station.lat/lon, die Live-API trägt station.latitude/longitude als String → 0 Stationen; an Mountain). Adressierte Blöcke (future-199/200, mountain-272/273) gefaltet. `--searxng`-Arm gebaut (JSON-fähige Instanzen gefunden) + `awesome-ai-web-search` gemint (5 neue Arme). Rat+Diver+UI zum ci-gate Per-SHA-Verdikt; Per-SHA-Gruppe + subset-Job gebaut. folge268 archiviert.
  class: handover
  date: 2026-10-08
  sha256: 2589bee9288c6c68719f569649e169bf2776f0de41dd2f5b0528113256da8593
  status: live
-->
# Handover — Mycelium-Folge 269 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-08-mycelium-folge268.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An <line>`.

## Burn: open 0.0028 · close 0.46 · cap 0.5 — Grund: Meta-Pass + KC2G + Rat/Diver/UI (ci-gate) + 16 Arme gebaut (searxng/SERP/oeis/hal/wiby/ia/shodan/opencellid/gfw) + FMHY + verdict-Kurzschluss; kein pro/max; gemessen `session_burn`

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
- **Trigger:** Mountains Zulassungs-/Dispositions-Verdikt (`docs/handover/handover-2026-10-08-mountain-folge275.md` §Pipeline-5)
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
- **Trigger:** Mountains Zulassungs-Verdikt (`docs/handover/handover-2026-10-08-mountain-folge275.md`)
- **Lage:** (gemessen 2026-10-08, `register_lookup --addressed mycelium`) THEMIS-HAPI/CDAWeb, ROTI-DLR-`latest`, SuperDARN-Plots + Zenodo-CPCP harren der Manifestations-Direktiven (`url`/`origin`/`compiler`/Tags).
- **Blockade:** Mountains Verdikt zuerst.
- **Braucht:** Mountains Admission; dann schreibt Mycelium die `url`/`origin`/`compiler`/Tags.

### `hadisst-cdn.yml` — Dispatch nach Admission (from mountain-272)
- **Status:** wartend | **Bindung:** eigen (CI-Dispatch)
- **Trigger:** Mountains Register-Admission der HadISST-Zeilen (`docs/handover/handover-2026-10-08-mountain-folge275.md`)
- **Lage:** (gemessen 2026-10-08) Workflow steht, noch nicht dispatcht; Asset im CDN erst nach Admission sichtbar.
- **Blockade:** Mountains Admission.
- **Braucht:** `gh workflow run hadisst-cdn.yml` nach Admission; sha je Asset ins Register.

### Gaia cluster_ka — Pfad-Träger (`phi/blocked_sources.φ:120`, owner mycelium)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Mountain-Admission der VizieR-Route (`phi/blocked_sources.φ:120`)
- **Lage:** (gemessen 2026-10-08) VizieR members table `https://vizier.cds.unistra.fr/viz-bin/VizieR-3?-source=J/A+A/633/A99/members` HTTP 200 (58318 B); Gaia-TAP `cluster_ka` column absent (HTTP 400). Der verlorene Zeiger ist als `pending` re-registriert.
- **Blockade:** Mountain-Admission.
- **Braucht:** Admission → Manifestation/Compiler.

### FMHY Runde 2 — neue legale Quellen/Arme (Operator-Wort 2026-10-08 „nochmal Agenten auf fmhy.net")
- **Status:** eigen | **Bindung:** eigen (tools/utils) → mountain (Admission)
- **Trigger:** Operator-Auswahl / Mountain-Admission
- **Lage:** (gemessen 2026-10-08, 3 Diver über `educational`/`reading`/`developer-tools`/`internet-tools`/`storage`/`misc`/`image`/`text`/`file`/`system-tools`) die FMHY-Erd-/Weltraum-Landschaft ist im Baum weitgehend registriert; **neue, verifizierte Kandidaten (nicht in `phi/` registriert, kein `archive_search`-Modus):** Daten/Research: **USGS NGMDB** (`https://ngmdb.usgs.gov/ArcGIS/rest/services?f=pjson`, 206) · **Internet Archive advancedsearch** (`https://archive.org/advancedsearch.php?q=…&output=json`, 200) · **HAL** (`https://api.archives-ouvertes.fr/search/?q=…&wt=json`, 206) · **OEIS** (`https://oeis.org/search?q=…&fmt=json`, 200) · **OAPEN** (OAI `library.oapen.org/oai/request`, 200) · **deps.dev API** (keyless). Suche/Infra: **Wiby JSON** (`https://wiby.me/json/?q=`, 200) · **RSS-Bridge** (`rss-bridge.org`, 200) · **Kiwix** (`download.kiwix.org/zim/…`, 200) · **web.scraper.workers.dev** (200) · **Shodan/OpenCelliD/Global Forest Watch** (free key). Reader/OCR-Kandidaten: **Tesseract · OCRmyPDF · Marker · Docling · MarkItDown · exifTool · ImageMagick · qsv/xan (Rust)**.
- **Blockade:** —
- **Braucht:** `gh workflow run tools-build.yml` nach dem Push (Rolling-Release trägt die Arme); **gebaut (dieses Atom): `--oeis` · `--hal` · `--wiby` · `--ia-search`** (keyless JSON, end-to-end grün). Ferner offen (Operator/Mountain): `--ngmdb` (ArcGIS-Endpunkt noch nicht sauber gemessen), OAPEN (OAI/XML), Reader/OCR-Sidecars (Tesseract/OCRmyPDF/Marker/Docling/MarkItDown/exifTool/ImageMagick/qsv).

### Key-Arme — Shodan · OpenCelliD · GFW (Operator-Wort 2026-10-08)
- **Status:** teil-gebaut | **Bindung:** eigen (tools/utils) + operator (GFW-Key)
- **Trigger:** ein gültiger GFW-Data-API-Key in `.secrets.local`
- **Lage:** (gemessen 2026-10-08) **`--opencellid` live** (`mcc=.. mnc=.. lac=.. cellid=.. [radio=..]` → lat/lon/range/samples; `OPENCELLID_API_KEY`). **`--shodan` live als Host-Lookup** (`--shodan 8.8.8.8` → ip/ports/org/country/banners); die Suche (`/shodan/host/search`) verlangt **Membership** (HTTP 403 „Requires membership or higher"), free-tier = Host-Lookup. **`--gfw` gebaut**, aber HTTP 403 „Request is missing valid API key" — der gespeicherte `GFW_PI_KEY` ist kein gültiger Data-API-Key (Auth-Token ≠ API-Key).
- **Blockade:** GFW-API-Key fehlt (Okta-Konto existiert; `/auth/apikey` noch nicht gelaufen).
- **Braucht:** `POST /auth/token` → `access_token`; `POST /auth/apikey` → API-Key; diesen als `GFW_PI_KEY` in `.secrets.local`.

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
