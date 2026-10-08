<!--
  title: Handover — Mycelium-Folge 269 (2026-10-08)
  session: Mycelium-Linie — Meta-Pass. KC2G-CDN-Ursache gemessen (Parser liest station.lat/lon, die Live-API trägt station.latitude/longitude als String → 0 Stationen; an Mountain). Adressierte Blöcke (future-199, mountain-272) gefaltet. `--searxng`-Instanz-Landschaft gemessen (keine öffentliche JSON-Instanz). folge268 archiviert.
  class: handover
  date: 2026-10-08
  sha256: 39ba3fe756e2bbe7c74f44b9a451265b77d902fb6cb66a775d29d5e42f35945d
  status: live
-->
# Handover — Mycelium-Folge 269 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-08-mycelium-folge268.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An <line>`.

## Burn: open 0.0028 · close 0.034 · cap 0.5 — Grund: Meta-Pass + KC2G-Diagnose + Adress-Faltung + `--searxng`-Messung; kein pro/max; gemessen `session_burn`

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
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-mycelium-folge263.md` §Operator-Wort-Register — gefaltet, nicht kopiert | 2026-10-07 | Quelle: mycelium-263.

## Offen — eigen

### `--searxng`-Arm / FMHY-Forschungslandschaft (Operator-Wort 2026-10-08)
- **Status:** blockiert | **Bindung:** eigen (tools/utils + Roster)
- **Trigger:** eine JSON-fähige SearXNG-Instanz ist gemessen
- **Lage:** (gemessen 2026-10-08, `curl` Instanz-Probe) kein öffentlicher SearXNG-Treffer liefert JSON: `searx.be` HTTP 200 = HTML (kein JSON-Format); `searx.tiekoetter.com`/`priv.au`/`search.hbubli.cc`/`opnxng.com`/`search.rhscz.eu`/`searx.perennialte.ch`/`searx.dresden.network`/`searx.namejeff.xyz` = 429; `search.projectsegfau.lt` = 200 text/plain 25 B (Format abgelehnt); `searxng.site` = 403. `--jina` steht (`0e644437d`); Perplexity/Sakana 4/4 in `ui-seats.md`.
- **Blockade:** öffentliche SearXNG-Instanzen haben den JSON-Exporter abgeschaltet (nur die HTML-Suche bleibt); der Arm braucht `formats: [json]` (selbst gehostet) oder einen HTML-Parser.
- **Braucht:** eine gemessene JSON-Instanz (oder Selbst-Host) → dann `searxng_lines`/`searxng_results` in `tools/utils/src/bin/archive_search/net.rs` nach dem `marginalia`-Muster; AI2-Playground-Seat testen; Sci-Bot via Rat.

### `ci-gate` trägt ein leichtes Testsubset (Bedingung des ci-check-Beschlusses)
- **Status:** wartend | **Bindung:** eigen (CI-Config)
- **Trigger:** der erste `ci-gate`-Lauf mit dem neuen Test-Job (`ci_manage log`)
- **Lage:** (gemessen 2026-10-07, Rat + 11 UI-Seats; mountain-272 Riss bestätigt) `ci-check` behauptet `cancel-in-progress: false`, wird aber als `cancelled` mit 0 Jobs verdrängt (`.github/workflows/ci-check.yml:20-27`). Riss (Qwen3.8): „Diagnose trägt, Fix trägt nicht" — der per-SHA-Verdikt muss Dateninvariante werden (`gap-fill`), nicht Scheduler-Nebenwirkung; `queue:max` verworfen.
- **Blockade:** der grüne/schnelle Subset-Job + Branch-Protection; lokaler Testlauf ist kein Nachweis.
- **Braucht:** `cargo test --lib`-Subset-Job in `ci-gate.yml` (+ per-SHA-Gruppe `ci-gate-${{ github.sha }}`); Branch-Protection auf ci-gate **vor** dem ci-check-push-Ausbau; Rat-Verdikt zum Riss (Dateninvariante `gap-fill` + Bisect vs. Nachtlauf).

### Die drei neuen CDN-Arme — success, auf Admission
- **Status:** wartend | **Bindung:** eigen (Mountain-Admission)
- **Trigger:** Mountains Register-Admission der Arme (`37755349709`/`37755354486`/`37755359472` success, gemessen 2026-10-08)
- **Lage:** (gemessen 2026-10-08 via `ci_manage view`) `superdarn-cpcp-cdn` `37755349709` **success** · `emtf-cdn` `37755354486` **success** · `ssusi-cdn` `37755359472` **success** (mountain-272), je HEAD `4c74c8554`.
- **Blockade:** Asset im CDN erst nach Admission sichtbar.
- **Braucht:** Mountains Admission; dann sha je Asset ins Register.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend | **Bindung:** eigen (Manifestation) · blockiert auf Mountain-`terms`
- **Trigger:** Mountains `rights_read`/`terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07; mountain-272 bestätigt) `LICENSE`/`README` dort absent (HTTP 404 raw); `license_census` 2249 `no-terms` (Mountain 270 heilte 7).
- **Blockade:** die `terms`-Zeilen (Mountain-Pen).
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf Mountain
- **Trigger:** Mountains Zulassungs-/Dispositions-Verdikt (`docs/handover/handover-2026-10-08-mountain-folge272.md` §Pipeline-5)
- **Lage:** (gemessen 2026-10-07) die 5 Alt-Einträge auf `disponiert`; neu `https://data.inpe.br/big/` (STAC/GeoTIFF, em; 2026-10-07 HTTP 200, 192329 B) als eigener Kandidat.
- **Blockade:** Mountains Zulassung.
- **Braucht:** Mountains Dispositions-Verdikt; dann Ernte-Verdrahtung.

### Research-APIs/MCPs — Consensus · Perplexity · SciSpace (`survey-2026-10-08-research-api-mcp.md`)
- **Status:** wartend | **Bindung:** eigen (MCP-Config) + operator (Neustart)
- **Trigger:** opencode-Neustart mit exportierten Keys (`set -a; source .secrets.local; set +a`)
- **Lage:** (gemessen 2026-10-08) `--consensus`/`--perplexity` live; MCP-Harness (`opencode.json` `mcp`, `consensus`+`perplexity` `type: remote`) verdrahtet; Elicit `descoped`; SciSpace cookie-interne API → kein Arm.
- **Blockade:** opencode-Neustart mit `set -a; source .secrets.local; set +a`.
- **Braucht:** Neustart (MCP-Tools für alle Agenten).

### Gegen-Audit — Quellen-Delta + Re-Audit (`survey-2026-10-08-open-sources-delta.md`)
- **Status:** eigen | **Bindung:** eigen (Recherche) → Mountain (Admission)
- **Trigger:** Operator-Wort 2026-10-08
- **Lage:** (gemessen 2026-10-08) neue Arme genutzt; Quellen-Delta (HI/CMB/Solar/LAIC/FRB/Teilchen) unregistriert; LEOS-Riss: `blocked_sources.φ:104` descoped (Captcha) vs. Survey-Messung 206 (user-gated, unregistriert) — stale Verdikt. Adler LPF/Lasair/DEMETER/GOSAT-Routen gemessen.
- **Blockade:** Mountain-Admission + Mycelium-Manifestation.
- **Braucht:** Mountain-Verdikt (inkl. LEOS-Reopen); Manifestation der neuen Routen nach Admission.

### Manifestation der neuen Routen (from future-199/200)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Mountains Zulassungs-Verdikt (`docs/handover/handover-2026-10-08-mountain-folge272.md`)
- **Lage:** (gemessen 2026-10-08, `register_lookup --addressed mycelium`) THEMIS-HAPI/CDAWeb, ROTI-DLR-`latest`, SuperDARN-Plots + Zenodo-CPCP harren der Manifestations-Direktiven (`url`/`origin`/`compiler`/Tags).
- **Blockade:** Mountains Verdikt zuerst.
- **Braucht:** Mountains Admission; dann schreibt Mycelium die `url`/`origin`/`compiler`/Tags.

### `hadisst-cdn.yml` — Dispatch nach Admission (from mountain-272)
- **Status:** wartend | **Bindung:** eigen (CI-Dispatch)
- **Trigger:** Mountains Register-Admission der HadISST-Zeilen (`docs/handover/handover-2026-10-08-mountain-folge272.md`)
- **Lage:** (gemessen 2026-10-08) Workflow steht, noch nicht dispatcht; Asset im CDN erst nach Admission sichtbar.
- **Blockade:** Mountains Admission.
- **Braucht:** `gh workflow run hadisst-cdn.yml` nach Admission; sha je Asset ins Register.

## An mountain

Origin: mycelium-269.

- **KC2G-CDN failure `37687765274` — Ursache gemessen (Parser-Schema-Drift):** `tools/harvest/src/bin/kc2g_stations.rs:42-45` liest `station.lat`/`station.lon` per `jnum`; die Live-API `https://prop.kc2g.com/api/stations.json` trägt die Position als `station.latitude`/`station.longitude` (String, z. B. `"30.4"`/`"262.3"`, gemessen 2026-10-08). `jpath_val` findet den Schlüssel `lat`/`lon` nicht → `parse_stations` liefert None → `exit(1)`. **Fix:** `station.latitude`/`station.longitude` lesen (Werte sind Strings; `scalar_of` parst sie bereits — `src/archivar/json.rs:284`). Sekundär: `time` ist ISO-8601 (`"2026-10-08T13:50:01"`), nicht Unix — `jnum(row, "time")` liefert None (nicht fatal). Der Unit-Fixture `FIXTURE` verwendet `lat`/`lon` + numerisches `time` und spiegelt damit nicht die reale API → Fixture an die Live-Schema-Form angleichen. Ursachen-Zeile im Log: `kc2g: ... carried no station with a measured position — the asset stays unwritten (0 honored)`.
- **`phi/blocked_sources.φ` — Aufräumen (Operator-Wort 2026-10-08, Rat-Verdikt 2026-10-08; Details mycelium-268 §Offen):** Diver-gemessen. Deine Feder: **(1) umziehen** — Klasse a (11 `pending`: Arm+WF stehen) → `phi/pipeline/ledger.φ` `ausstehend`; die 3 redundanten `descoped` (nssdc.ac.cn, titanNotebook, ioc-v1) → `declined_sources.φ` `decline superseded-by-integrated`. **(2) umtaggen** — Klasse b (9) → `blocked account`/`blocked key`; Klasse c (9) → `parser-def` + korrektes `gap`. **(3) korrigieren** — Madrigal-`gap` von `html-parser-arm` auf OpenMadrigal-API-Arm; GHSL (`covariate-carrier`) bleibt. **(4) re-messen, nicht glätten** — Klasse d (10 Stale: 30/31/34/36/37/39 Arm+WF stehen, 23 `electrodes.rs`, 18 doppelt, 6/35 absent/tot); **leos.ac.cn** (206 user-gated) + **Gaia cluster_ka** (VizieR members) re-registrieren; TUH/NSRR re-messen. **(5) entfernen nur mit Befund** — die 20 Legenden-`note`s mit stehendem Arm + Zeile 2. **(6) Stale-Schutz** — Messstelle/Stempel/Trigger je Eintrag + `descoped-check` auf `blocked parser-def` ausdehnen (Gate-Fixture). **NICHTS entfernen, was lebt/registriert ist** (Rat, einhellig).
- **LEOS-Riss:** `phi/blocked_sources.φ:104-106` `descoped` (Captcha) vs. `survey-2026-10-08-open-sources-delta.md:75/133` `206` (user-gated). Re-open oder Verdikt neu messen.
- **`ci-check`-Verdrängung:** `.github/workflows/ci-check.yml:20-27` `cancel-in-progress: false` → jeder Lauf `cancelled` 0 Jobs (mountain-272). Fix nötig.
- **Doppler-Kanal (Q4, Axiom pending):** Pfad-Lücke als `pending` mit Trigger („route erscheint / Produkt gemessen") — Register (`blocked_sources.φ`/`ledger.φ`) ist deine Feder; der Riss „Mycelium führt die Pfad-Lücke"/Mountain-Pen bleibt benannt.
- **GIC-Stufe-2 Member-Pool (Q5):** als Register-Klassenträger zulässig (jeder Member eigene Kraft/Deskriptor), als ein Wire-Deskriptor verboten. Register ist deine Feder; River verdrahtet die drei Deskriptoren.

## An future

Origin: mycelium-269.

- (Keine neuen Operator-Akten in diesem Atom; die DEMETER/GOSAT/Kuprat-Akten aus mycelium-267 stehen in deiner Queue.)

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
