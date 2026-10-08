<!--
  title: Handover — Mycelium-Folge 267 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass. Adressierte Blöcke (future-199, mountain-270) gefaltet; die drei CDN-Läufe gemessen (JAXA/KC2G queued, OSHA-CEHD Host unreachable); pages-deploy success (Sonne-Anker); ci-check-Verdrängung als offener Ratspunkt; konsumierte 266 archiviert; Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-07
  sha256: f63cd50ff512c59c94d406e63c9cd75baddefa9de16477c8d640f689f14328b2
  status: live
-->
# Handover — Mycelium-Folge 267 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-07-mycelium-folge266.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An <line>`.

## Burn: open 0.000 · close 0.045 · cap 0.5 — Grund: Meta-Pass, adressierte Blöcke gefaltet + Stehender Pass; kein pro/max; gemessen `session_burn` (line, deepseek-flash; Eintrag „Mycelium-Linie in einem Pass starten")

## Operator-Wort-Register

- 2026-10-07 | „bitte gib das dem rat, einem taucher mit archive search und den ui chat stimmen" (der ci-check-Verdrängungs-Riss) | Quelle: diese Session.
- 2026-10-07 | „mir ist wichtig dass ab jetzt alle linien wissen was möglich ist und wie die modelle auch einzusetzen sind" → getrackte Karte `docs/concepts/ui-seats.md` + Verweis in allen Linien-Command-Prompts (`ebf39c309`) | Quelle: diese Session.
- 2026-10-08 | „k3 läuft in der regel nicht auf kimi.ai nur 2.6" (Korrektur zum Seitentitel) | Quelle: diese Session.
- 2026-10-08 | „die fmhy surveys nicht nur verpuffen lassen, denkt groß — wahnsinniges potential gerade im science/research Bereich" → Survey `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` + offene Punkte | Quelle: diese Session.
- 2026-10-08 | „auth ist nicht zwingend ein ausschlusskriterium nur kommerziell und illegal" → Auth-Route-Kandidaten (Perplexity/SciSpace/Sakana/NotebookLM) in der Survey; kein „raus" wegen 403/Login | Quelle: diese Session. (Deckt sich mit AGENTS „Authentifizierung ist kein Ausschlusskriterium", Operator-Wort 2026-10-08.)
- 2026-10-08 | „ja bitte" (Gretchenfrage für die Auth-Route-Kandidaten fahren) → Perplexity 4/4 (3 s), Sakana 4/4 → `ui-seats.md` | Quelle: diese Session.
- 2026-10-08 | „mich interessieren natürlich am meisten die APIs/MCPs" → API/MCP-Survey `docs/surveys/survey-2026-10-08-research-api-mcp.md` + `--consensus`-Arm; Endpunkte gemessen | Quelle: diese Session.
- 2026-10-08 | „consensus und perplexity sind drin, elicit descoped da kostenpflichtig" → Keys in `.secrets.local`; `--consensus` live; Elicit `descoped` | Quelle: diese Session.
- 2026-10-08 | „können wir den connector nicht für opencode nachbauen? und braucht unser taucher nicht einen MCP harness?" → opencode `mcp`-Block ist der Harness; `consensus` + `perplexity` als `type: remote` verdrahtet; SciSpace-MCP gebrokert, nicht nachbaubar | Quelle: diese Session.
- 2026-10-08 | „scispace nochmal mit harten bandagen … perplexity hab ich nochmal sicher eingegeben 10$ guthaben geschenkt" → SciSpace: cookie-interne API (kein Arm); Perplexity: Endpunkt umgezogen → Agent-API, `--perplexity` live (`1569a26d8`) | Quelle: diese Session.
- 2026-10-08 | „nochmal prüfen ob du quellen findest die future nicht gefunden hat … und hast du gerade das tool mit seinen neuen skills genutzt?" → Gegen-Audit `docs/surveys/survey-2026-10-08-open-sources-delta.md` (neue Arme genutzt, LEOS/CSES u. a. neu) | Quelle: diese Session.
- 2026-10-08 | „nicht nur die Top irgendwas (LLM-Seuche) — alle speichern; harte Taucher inkl. Secrets/Proton/`--all` auf die Blockierten, ggf. UI-Chats; wenn ihr die legal knackt wäre der Hammer" → vollständige Tabellen (keine Top-N) + zwei harte Taucher: LPF geknackt (HEASARC), Lasair/DEMETER/GOSAT-GW bis zur Auth-Kante | Quelle: diese Session.
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-mycelium-folge263.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Offen — eigen

### JAXA gportal — `jaxa-gportal-cdn` läuft
- **Status:** wartend | **Bindung:** eigen (Lauf-Ausgang)
- **Trigger:** Lauf-Ergebnis `37676047864`
- **Lage:** (gemessen 2026-10-07T21:5xZ via `ci_manage view`) seit 19:38Z **queued** (self-hosted Runner); unverändert.
- **Blockade:** keine (Runner-Queue).
- **Braucht:** `ci_manage view 37676047864` → Ergebnis einmalig lesen; dann sha ins Register.

### KC2G `prop.kc2g.com` — Manifestation dispatcht
- **Status:** wartend | **Bindung:** eigen (Lauf-Ausgang)
- **Trigger:** Lauf-Ergebnis `37687765274`
- **Lage:** (gemessen 2026-10-07T21:5xZ via `ci_manage view`) **queued**; der Harvest-Block in `phi/harvest.φ` (`format kc2g_stations`/`arm`/`pattern ^kc2g_stations\.csv$`) steht.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 37687765274` → neuen `kc2g_stations.csv`-sha ins Register.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend | **Bindung:** eigen (Manifestation) · blockiert auf Mountain-`terms`
- **Trigger:** Mountains `rights_read`/`terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07, Mycelium 255/260/263/270) `LICENSE`/`README` dort absent (HTTP 404 raw); `license_census` zählt 2249 `no-terms` (Mountain 270 heilte 7 `terms unbestimmt`).
- **Blockade:** die `terms`-Zeilen (Mountain-Pen).
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf Mountain
- **Trigger:** Mountains Zulassungs-/Dispositions-Verdikt (`docs/handover/handover-2026-10-07-mountain-folge270.md` §Pipeline-5)
- **Lage:** (gemessen 2026-10-07) die 5 Alt-Einträge auf `disponiert`; neu aufgenommen `https://data.inpe.br/big/` (STAC/GeoTIFF, em; 2026-10-07 HTTP 200, 192329 B) als eigener Kandidat.
- **Blockade:** Mountains Zulassung.
- **Braucht:** Mountains Dispositions-Verdikt; dann Ernte-Verdrahtung.

### `ci-gate` trägt ein leichtes Testsubset (Bedingung des ci-check-Beschlusses)
- **Status:** wartend | **Bindung:** eigen (CI-Config)
- **Trigger:** der erste `ci-gate`-Lauf mit dem neuen Test-Job (`ci_manage log`)
- **Lage:** (gemessen 2026-10-07) Der Rat (5 Stimmen) beschließt: ci-check ist die schwere Nacht-/Dispatch-Messung (push-Trigger entfernt, `3db5a3ad4`), der per-SHA-Blocker ist ci-gate. **Sieben UI-Stimmen konvergieren auf „trägt, mit Bedingung"** — Z.ai/GLM-5.3, Qwen, DeepSeek V4 Pro, GPT-OSS 120B, MiMo V2.6 Pro, Inkling, Nemotron 3 Ultra: ci-gate muss ein leichtes Testsubset (`cargo test --lib` / Unit-Tests, < ~10 min) als harten **required status check** tragen, sonst ist „Gate" Deklaration (und per-SHA testblind). `queue:max` einhellig verworfen (legalisiert Backlog, Verdicts kommen stale an). Duck.ai (Tageslimit) und Claude (5-h-Nachrichtenlimit) gemessen nicht erreichbar.
- **Riss (Qwen3.8 2.4T):** „Diagnose trägt, Fix trägt nicht." Die Gruppe war nie per-SHA (`github.ref` = Branch, nicht Commit); die statische Gruppe heilt nur selten, nightly misst einen beweglichen HEAD. Der per-SHA-Verdikt muss als **Dateninvariante** entstehen (jeder Run schreibt eine Verdikt-Zeile sha/metric/status; ein Nightly-`gap-fill`-Step misst main-SHAs ohne Verdikt, bounded N), nicht als Scheduler-Nebenwirkung; Merge-Queue max in-flight 1 drosselt an der Quelle.
- **Zwei Risse (MiMo V2.6 Pro):** (1) ci-gates eigener concurrency-Group muss per-SHA (`ci-gate-${{ github.sha }}`) oder ganz ohne concurrency sein — sonst canneln Push-Storms das wartende Gate und der required Check bleibt auf „Expected"; (2) Reihenfolge: erst ci-gate live + exakter Job-Name als required Check, **dann** push aus ci-check entfernen, sonst Fenster ohne Pflichtsignal.
- **Blockade:** die grüne/schnelle Subset-Messung — ein lokaler Testlauf ist CI-Job, kein lokaler Nachweis; der Riss (Dateninvariante vs. Scheduler) ist ein Ratspunkt.
- **Runde 2 — sechs weitere Seats (2026-10-07, Roster vollständig):** Gemini 3.1 Pro („Holds logically, but cracks operationally" — Gap-Fill vieler SHAs verbrennt Compute; Alternative **Nightly-HEAD + O(log N) Bisect**), MiniMax M3 („Riss trägt" — Schema für `cancelled,0 jobs` festnageln, PR-Zeit ≠ Nightly, `cancel-in-progress:false` bleibt falsch für push), DeepSeek Chat („Trägt — aber nur mit der Trennung; reine Scheduler-Lösung ist ein Riss"), Mistral („Trägt — beide zusammen"), Lumo („Riss trägt konzeptionell, löst das Queue-Problem nicht"), Kimi (Login-Wall, gemessen nicht erreichbar). **Roster damit vollständig:** Duck/Claude (Limit, gemessen), Qwen, Z.ai, MiMo, Nemotron, MiniMax, Gemini, DeepSeek Chat, Mistral, Lumo, Kimi (Wall) + die 6 Tryingopen (DeepSeek V4 Pro, GPT-OSS 120B, MiMo, Qwen3.8 2.4T, Inkling, Nemotron).
- **Bedien-Karte (Operator-Wort):** `docs/concepts/ui-seats.md` — Roster, Composer-Selektoren, `open-weight-ui`-Lock, Runden-Disziplin; verlinkt aus allen fünf Linien-Command-Prompts (`ebf39c309`).
- **Braucht:** `cargo test --lib`-Subset-Job in `ci-gate.yml` (+ per-SHA-Gruppe); Branch-Protection auf ci-gate, **vor** dem ci-check-push-Ausbau; Rat-Verdikt zum Riss (Dateninvariante `gap-fill` + Bisect vs. Nachtlauf mit Auto-Bisect).

### FMHY/Awesome-Mining — Research-Landschaft (Operator-Wort 2026-10-08)
- **Status:** eigen | **Bindung:** eigen (tools/utils + Roster)
- **Trigger:** Operator-Wort 2026-10-08 („nicht verpuffen lassen, denkt groß")
- **Lage:** (gemessen 2026-10-08, `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md`) das Mining liefert **kein** neues Science-Daten-Arm (beide Listen out-of-scope). **`--jina` gebaut** (`0e644437d`, `jina.rs` + Wiring; end-to-end gemessen `--jina https://example.com` → 200, sauberer Text) + in `docs/concepts/tools-map.md` eingetragen. **Auth-Route-Gretchenfrage** (`state/stimmen/2026-10-08_gretchenfrage-auth-route.md`): **Perplexity 4/4** (`PHIL: absent,null-echt,pending,nein,b,fabrication`, 3 s) und **Sakana Chat 4/4** (Tempo `pending`) → in `ui-seats.md`; SciSpace Login-Wall, Gemini Notebook Dokument-Bot → `pending`. Offen: `--searxng` (JSON-Instanz), AI2 Playground, Elicit/Consensus, Sci-Bot (Rat), NVIDIA NIM.
- **Blockade:** die offenen Arme sind Code (`tools/utils`, Vorbild `--mwmbl`); AI2/Elicit/Consensus brauchen die Gretchenfrage (Fähigkeit 4/4 + Tempo).
- **Braucht:** `archive_search --searxng <query>` bauen + JSON-fähige Instanz; AI2-Playground-Seat testen; Elicit/Consensus gegen `--openalex`/`--semanticscholar` gegenprüfen; Sci-Bot via Rat.

### Research-APIs/MCPs — Consensus · Perplexity · Elicit(descoped) · SciSpace (`docs/surveys/survey-2026-10-08-research-api-mcp.md`)
- **Status:** wartend | **Bindung:** eigen (tools/utils + MCP-Config)
- **Trigger:** opencode-Neustart mit exportierten Keys (`set -a; source .secrets.local; set +a`)
- **Lage:** (gemessen 2026-10-08) **`--consensus` läuft live** (`CONSENSUS_API_KEY`; 10 Papers am Baum); **`--perplexity` läuft live** (`1569a26d8`) — der 403 war der **umgezogene Endpunkt**: Sonar ist jetzt die **Agent-API** (`POST api.perplexity.ai/v1/agent`, `preset`+`input`); Antwort mit `[web:n]`-Citations. **MCP-Harness verdrahtet** (`opencode.json` `mcp`: `consensus` + `perplexity` als `type: "remote"` mit `{env:VAR}`). **Elicit `descoped`** (nur Pro/Scale/Enterprise). **SciSpace:** `api.scispace.com/api/v1/*` → 403 `{"message":"Missing cookies"}` — cookie-interne Web-API, **keine Key-API**, keine öffentlichen REST-Docs; MCP nur gebrokert → **kein Arm** (`pending`).
- **Blockade:** opencode-Neustart mit exportierten Keys steht aus.
- **Braucht:** `set -a; source .secrets.local; set +a` + opencode neu starten (MCP-Tools für alle Agenten).

### Gegen-Audit — Quellen-Delta + Re-Audit (`docs/surveys/survey-2026-10-08-open-sources-delta.md`)
- **Status:** eigen | **Bindung:** eigen (Recherche) → Mountain (Admission)
- **Trigger:** Operator-Wort 2026-10-08 („Quellen finden, die Future nicht fand" + „hast du das Tool mit den neuen Skills genutzt?")
- **Lage:** (gemessen 2026-10-08) **die neuen Arme endlich genutzt** (`--jina`/`--perplexity`/`--consensus`) — sie liefern: `--perplexity` fand den **CSES-Port `www.leos.ac.cn`** (200, user-gated, unregistriert), `--consensus` CSES-Vorläufer-Papers, `--jina` liest Portal-Seiten. Quellen-Delta: HI-Surveys (LAB/EBHIS/GASS/GALFA-HI, SkyView EBHIS/GASS), CMB (PLA, LAMBDA, ACT/SPT), Solar (IRIS/SDO/VSO/HEK/Solar Orbiter/PSP), LAIC (CSSDC/LEOS/INTERMAGNET/ISGI), FRB (CHIME/Blinkverse), Teilchen (ATLAS/Belle II/GWOSC/Fermilab). **Re-Audit:** SuperMAG jetzt erreichbar (206; `--jina`: `ERROR: No username` → braucht `username`-Param); LEOS neu; PSA/SSDC/Chang'e erreichbar; LPF/Lasair/DEMETER-order/GOSAT-Host blockiert. **Riss:** der Diver zitierte CSES als `sources.φ:17149` — der Baum trägt dort Vega 2 MISCHA (`sgrep leos.ac.cn`=0).
- **Blockade:** die neuen Kanäle brauchen Mountain-Admission (Verdikt-Zeile) + Mycelium-Manifestation; die Kuprat-Rückfrage ist operator-gebunden.
- **Blockierte vier (harte Runde, Kaskade inkl. Proton + neue Arme):** **keine** war geo/Cloudflare — **LPF GEKNACKT** (offenes NASA/HEASARC-Mirror `heasarc.gsfc.nasa.gov/lpf/cgi/selector?start=&end=&hdu=` → 200 `application/fits`; ESA `/lpfsa/` lebt); **Lasair Route geknackt** (`api.lasair.lsst.ac.uk` 401 = am Leben; `cone/query/object/sherlock`, `?token=<TOK>`/`Authorization: Token`); **DEMETER** Block wanderte WAF-403→auth-401 (lokale Metalink-JWTs **abgelaufen** 2026-10-05; Endpunkt `…/orders/public/files/<id>?orderToken=<JWT>&scope=cdpp`); **GOSAT-GW** falscher Host → echter Port `product.gosat-gw.nies.go.jp` 200, API cookie-gated (Login `mail_address`/`password`).
- **Braucht:** Mountain-Verdikt für die neuen Quellen + **LPF-HEASARC**; Lasair mit Token einmalig messen (Probe bauen); **DEMETER/GOSAT-GW = Operator-Akt** (frisches REGARDS-Token / NIES-Cookie); SuperMAG-`username`-Kontrakt messen; Kuprat-/Detektor-Rückfrage an den Operator.

## An river

Origin: mycelium-265.

- **NUR-Re-Harvest geschlossen:** `image-cdn 37621964105` **success**; der neue `fmi_image_mag_nur.bin`-sha steht (`phi/sources.φ:18155` `93f17d0a6b75a48cc71a5b09f8ecfbf209f9381c692cdb15db0e5f658db6a139`). **Die River-Probe + Zahl Paper §4/§6 können laufen.**

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
