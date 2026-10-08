<!--
  title: Handover — Mycelium-Folge 268 (2026-10-08)
  session: Mycelium-Linie — Meta-Pass. `blocked_sources.φ`-Drift diagnostiziert (48 Einträge: 39 pending · 7 descoped · 2 blocked, plus 24-Zeilen-gap-Legende); adressierte Blöcke (future-199, mountain-272) gefaltet; JAXA-gportal + KC2G-CDN gemessen (beide failure, Ursache aus dem Log); die drei neuen CDN-Arme success; folge267 archiviert.
  class: handover
  date: 2026-10-08
  sha256: dc41cdbb685844b12b71cbf893a0fb0682d5633f31110ee8922982cb46a300f9
  status: live
-->
# Handover — Mycelium-Folge 268 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-07-mycelium-folge267.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An <line>`.

## Burn: open 0.000 · close 0.081 · cap 0.5 — Grund: Meta-Pass + Drift-Fix (Move+Commit); blocked_sources-Diagnose (2 Diver), Rat (council), folge267 archiviert; kein pro/max; gemessen `session_burn`

## Operator-Wort-Register

- 2026-10-08 | „schau dir die blocked sources an — warum so chaotisch, die einträge werden nicht bearbeitet und entfernt, wir haben doch declined/dead; den ersten großen block verstehe ich nicht" | Quelle: diese Session. → Diagnose unten (`phi/blocked_sources.φ`).
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
- 2026-10-08 | „nochmal prüfen ob du quellen findest die future nicht gefunden hat" → `survey-2026-10-08-open-sources-delta.md` | Quelle: mycelium-267.
- 2026-10-08 | „nicht nur die Top irgendwas (LLM-Seuche) — alle speichern; harte Taucher …" | Quelle: mycelium-267.
- 2026-10-08 | **Rat-Vorbereitung (dauerhaft):** vor jeder Ratssitzung `archive_search --all` + gewichtete UI-Chats im stärksten Modus | Quelle: mycelium-267.
- 2026-10-08 | „kannst du dafür keine token wrapper bauen …" → `--lasair`/`--gosat` live | Quelle: mycelium-267.
- 2026-10-08 | „ich hab keinen nerv darauf zu warten — bitte vertage CDPP" → `wartend.φ:demeter-refresh` | Quelle: mycelium-267.
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-mycelium-folge263.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Offen — eigen

### `phi/blocked_sources.φ` — Register-Drift (Operator-Wort 2026-10-08)
- **Status:** blockiert | **Bindung:** linie:mountain
- **Trigger:** Mountains Dispositions-/Register-Verdikt
- **Lage:** (gemessen 2026-10-08, `grep`/`read`/`register_lookup`; Diver A/B via `archive_search --verdict`/`--sniff` + Arm-/Workflow-Abgleich; Rat-Verdikt) **221 Zeilen, 48 Einträge: 39 `pending`, 7 `descoped`, 2 `blocked parser-def` — 0 `blocked account`/`key`/`ip-blocked`.** Davor 24 `note`-Zeilen Legende (22 gap-Tokens; Arme fehlen WIRKLICH nur bei `konverter`, `covariate-carrier`). `docs/SOURCE_PORT.md:36` definiert das Register als `key-needed`/`parser-def`. Diver-Befunde: **7 `descoped`** = 3 messbar redundant (nssdc.ac.cn→CDS-HiPS, titanNotebook→cassini_odf, ioc-v1→service.php; ENTFERNEN ohne Verlust) + 4 Riss/verlorener Zeiger (**leos.ac.cn** steht descoped/Captcha, neu 206 user-gated unregistriert; **Gaia cluster_ka** → VizieR J/A+A/633/A99/members nirgends registriert; TUH EEG Zugang live/Feld descoped; NSRR Gate=Operator-HIPAA/Arm gebaut). **2 parser-def:** Madrigal-`gap html-parser-arm` ist FALSCH (Arm steht, echte Lücke = OpenMadrigal-API-Arm); GHSL-`covariate-carrier` echt. **39 `pending`** Klassen a/b/c/d = **11/9/9/10** (a: Arm+WF stehen → ledger; b: Konto/Key → blocked account/key; c: Arm/Feld fehlt → parser-def/Bau; **d: 10 Stale-Verdikte** — Note sagt „kein Arm"/„neu", Compiler+WF liegen im Baum).
- **Blockade:** die Verdikt-Zeilen sind Mountains Feder (`AGENTS.md` — Mountain alleiniger Schreiber der Dispositions-Verdikte); `--orphans` = 0 (Einträge sind getragen).
- **Braucht:** **Mountains Ausführung des Rat-Verdikts (2026-10-08):** *Umziehen* — Klasse a (11) → `phi/pipeline/ledger.φ` `ausstehend`; die 3 redundanten `descoped` → `declined_sources.φ` `decline superseded-by-integrated`. *Umtaggen* — Klasse b (9) → `blocked account`/`blocked key`; Klasse c (9) → `parser-def` mit korrektem `gap`; Madrigal-`gap` korrigieren; GHSL bleibt. *Re-messen* — Klasse d (10) einzeln (Arm steht → ledger; tot → dead; absent → pending+Trigger); leos+Gaia re-registrieren; TUH/NSRR re-messen. *Entfernen* — die 20 Legenden-`note`s mit stehendem Arm + Zeile 2 („19 Tokens" falsch); die 2 echten `gap`s bleiben am Block. *Stale-Schutz* — Register-Eintrag trägt künftig Messstelle/`gap` + Mess-Stempel + Trigger; `register_lookup --descoped-check` auf `blocked parser-def` ausdehnen (Gate-Fixture). **UI-Zweiter-Kanal (Operator-Wort) noch offen** — Frage: „Register-Schnitt ohne Verlust: pending→ledger, descoped→declined, blocked bleibt — trägt, mit welcher Bedingung?" (Gruppe `mycelium-ui` + `open-weight-ui`).

### JAXA G-Portal-CDN — Lauf `37676047864` failure
- **Status:** wartend | **Bindung:** eigen (Compiler-Riss)
- **Trigger:** neue Lauf-Ergebnis-Zeile
- **Lage:** (gemessen 2026-10-08 via `ci_manage view`/`log`) **failure**; Ursache aus dem Log: `jaxa_gpm_ku: FS/navigation/scLat stays unread` → `exit 1`. Der Download lief (`HTTP 200`, 153785337 B); der Compiler verweigert, weil das GPM-Ku-Granule das Navigationsfeld `scLat` nicht liest.
- **Blockade:** der Compiler-Arm liest `FS/navigation/scLat` nicht (Granule-Struktur).
- **Braucht:** `ci_manage log 37676047864` → Arm um `scLat` (bzw. die Ku-Navigation) erweitern; dann `gh workflow run jaxa-gportal-cdn.yml`.

### KC2G `prop.kc2g.com` — Lauf `37687765274` failure
- **Status:** wartend | **Bindung:** eigen (Quelle leer)
- **Trigger:** neue Lauf-Ergebnis-Zeile
- **Lage:** (gemessen 2026-10-08 via `ci_manage view`/`log`) **failure**; Ursache aus dem Log: `kc2g: https://prop.kc2g.com/api/stations.json carried no station with a measured position — the asset stays unwritten (0 honored)`. Die Station-API liefert keine Position (0 honored im Arm; kein Fabrikat).
- **Blockade:** `stations.json` trägt am CI-Standort keine gemessene Position (Host/Standort-Drift).
- **Braucht:** `archive_search --verdict https://prop.kc2g.com/api/stations.json` + Payload-Form prüfen; alternativen kc2g-Endpunkt oder Standort-Route messen; dann Re-Dispatch.

### Die drei neuen CDN-Arme — success
- **Status:** wartend | **Bindung:** eigen (Mountain-Admission)
- **Trigger:** Mountains Register-Admission der Arme (`gh workflow run hadisst-cdn.yml`)
- **Lage:** (gemessen 2026-10-08 via `ci_manage view`) `superdarn-cpcp-cdn` `37755349709` **success** · `emtf-cdn` `37755354486` **success** · `ssusi-cdn` `37755359472` **success** (mountain-272), je HEAD `4c74c8554`.
- **Blockade:** Asset im CDN erst nach Admission sichtbar; `hadisst-cdn.yml` noch nicht dispatcht.
- **Braucht:** `gh workflow run hadisst-cdn.yml` nach Push; sha je Asset ins Register.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend | **Bindung:** eigen (Manifestation) · blockiert auf Mountain-`terms`
- **Trigger:** Mountains `rights_read`/`terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07, Mycelium 255/260/263/270; mountain-272 bestätigt) `LICENSE`/`README` dort absent (HTTP 404 raw); `license_census` 2249 `no-terms` (Mountain 270 heilte 7).
- **Blockade:** die `terms`-Zeilen (Mountain-Pen).
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf Mountain
- **Trigger:** Mountains Zulassungs-/Dispositions-Verdikt (`docs/handover/…mountain-folge270.md` §Pipeline-5)
- **Lage:** (gemessen 2026-10-07) die 5 Alt-Einträge auf `disponiert`; neu `https://data.inpe.br/big/` (STAC/GeoTIFF, em; 2026-10-07 HTTP 200, 192329 B) als eigener Kandidat.
- **Blockade:** Mountains Zulassung.
- **Braucht:** Mountains Dispositions-Verdikt; dann Ernte-Verdrahtung.

### `ci-gate` trägt ein leichtes Testsubset (Bedingung des ci-check-Beschlusses)
- **Status:** wartend | **Bindung:** eigen (CI-Config)
- **Trigger:** der erste `ci-gate`-Lauf mit dem neuen Test-Job (`ci_manage log`)
- **Lage:** (gemessen 2026-10-07, Rat + 11 UI-Seats; mountain-272 Riss bestätigt) `ci-check` behauptet `cancel-in-progress: false`, wird aber als `cancelled` mit 0 Jobs verdrängt (`.github/workflows/ci-check.yml:20-27`). Riss (Qwen3.8): „Diagnose trägt, Fix trägt nicht" — der per-SHA-Verdikt muss Dateninvariante werden (`gap-fill`), nicht Scheduler-Nebenwirkung; `queue:max` verworfen.
- **Blockade:** der grüne/schnelle Subset-Job + Branch-Protection; lokaler Testlauf ist kein Nachweis.
- **Braucht:** `cargo test --lib`-Subset-Job in `ci-gate.yml` (+ per-SHA-Gruppe `ci-gate-${{ github.sha }}`); Branch-Protection auf ci-gate **vor** dem ci-check-push-Ausbau; Rat-Verdikt zum Riss (Dateninvariante `gap-fill` + Bisect vs. Nachtlauf).

### FMHY/Awesome-Mining — Research-Landschaft (Operator-Wort 2026-10-08)
- **Status:** eigen | **Bindung:** eigen (tools/utils + Roster)
- **Trigger:** Operator-Wort 2026-10-08
- **Lage:** (gemessen 2026-10-08, `survey-2026-10-08-fmhy-research-landscape.md`) kein neues Science-Daten-Arm; `--jina` gebaut (`0e644437d`); Auth-Route-Gretchenfrage: Perplexity 4/4, Sakana Chat 4/4 → `ui-seats.md`.
- **Blockade:** die offenen Arme sind Code; AI2/Elicit/Consensus brauchen die Gretchenfrage.
- **Braucht:** `archive_search --searxng <query>` bauen; AI2-Playground-Seat testen; Sci-Bot via Rat.

### Research-APIs/MCPs — Consensus · Perplexity · SciSpace (`survey-2026-10-08-research-api-mcp.md`)
- **Status:** wartend | **Bindung:** eigen (MCP-Config) + operator (Neustart)
- **Trigger:** opencode-Neustart mit exportierten Keys (`set -a; source .secrets.local; set +a`)
- **Lage:** (gemessen 2026-10-08) `--consensus`/`--perplexity` live; MCP-Harness (`opencode.json` `mcp`, `consensus`+`perplexity` `type: remote`) verdrahtet; Elicit `descoped`; SciSpace cookie-interne API → kein Arm.
- **Blockade:** opencode-Neustart mit `set -a; source .secrets.local; set +a`.
- **Braucht:** Neustart (MCP-Tools für alle Agenten).

### Gegen-Audit — Quellen-Delta + Re-Audit (`survey-2026-10-08-open-sources-delta.md`)
- **Status:** eigen | **Bindung:** eigen (Recherche) → Mountain (Admission)
- **Trigger:** Operator-Wort 2026-10-08
- **Lage:** (gemessen 2026-10-08) neue Arme genutzt; Quellen-Delta (HI/CMB/Solar/LAIC/FRB/Teilchen) unregistriert; **LEOS-Riss**: `blocked_sources.φ:104` `descoped` (Captcha) vs. Survey-Messung `206` (user-gated, unregistriert) — stale Verdikt. Adler LPF/Lasair/DEMETER/GOSAT-Routen gemessen.
- **Blockade:** Mountain-Admission + Mycelium-Manifestation.
- **Braucht:** Mountain-Verdikt (inkl. LEOS-Reopen); Manifestation der neuen Routen nach Admission.

## An mountain

Origin: mycelium-268.

- **`phi/blocked_sources.φ`-Aufräumen (Operator-Wort 2026-10-08, Rat-Verdikt 2026-10-08):** Diver-gemessen (Details im Offen-Block). Deine Feder: **(1) umziehen** — Klasse a (11 `pending`: Arm+WF stehen) → `phi/pipeline/ledger.φ` `ausstehend`; die 3 redundanten `descoped` (nssdc.ac.cn, titanNotebook, ioc-v1) → `declined_sources.φ` `decline superseded-by-integrated`. **(2) umtaggen** — Klasse b (9) → `blocked account`/`blocked key`; Klasse c (9) → `parser-def` + korrektes `gap`. **(3) korrigieren** — Madrigal-`gap` von `html-parser-arm` auf OpenMadrigal-API-Arm; GHSL (`covariate-carrier`) bleibt. **(4) re-messen, nicht glätten** — Klasse d (10 Stale: 30/31/34/36/37/39 Arm+WF stehen, 23 `electrodes.rs`, 18 doppelt, 6/35 absent/tot); **leos.ac.cn** (206 user-gated) + **Gaia cluster_ka** (VizieR members) re-registrieren (verlorene Zeiger!); TUH/NSRR re-messen. **(5) entfernen nur mit Befund** — die 20 Legenden-`note`s mit stehendem Arm + Zeile 2. **(6) Stale-Schutz** — Messstelle/Stempel/Trigger je Eintrag + `descoped-check` auf `blocked parser-def` ausdehnen (Gate-Fixture). **NICHTS entfernen, was lebt/registriert ist** (Rat, einhellig).
- **LEOS-Riss:** `phi/blocked_sources.φ:104-106` `descoped` (Captcha) vs. `survey-2026-10-08-open-sources-delta.md:75/133` `206` (user-gated). Re-open oder Verdikt neu messen.
- **`ci-check`-Verdrängung:** `.github/workflows/ci-check.yml:20-27` `cancel-in-progress: false` → jeder Lauf `cancelled` 0 Jobs (mountain-272). Fix nötig.

## An future

Origin: mycelium-268.

- (Keine neuen Operator-Akten in diesem Atom; die DEMETER/GOSAT/Kuprat-Akten aus mycelium-267 stehen in deiner Queue.)

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
