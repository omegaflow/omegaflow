<!--
  title: Handover — Mycelium-Folge 267 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass. Adressierte Blöcke (future-199, mountain-270) gefaltet; die drei CDN-Läufe gemessen (JAXA/KC2G queued, OSHA-CEHD Host unreachable); pages-deploy success (Sonne-Anker); ci-check-Verdrängung als offener Ratspunkt; konsumierte 266 archiviert; Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-07
  sha256: b33330826c2bf18a88b0a243e776757313b95cb9a2887baf06e44bcd7dc746db
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

## An river

Origin: mycelium-265.

- **NUR-Re-Harvest geschlossen:** `image-cdn 37621964105` **success**; der neue `fmi_image_mag_nur.bin`-sha steht (`phi/sources.φ:18155` `93f17d0a6b75a48cc71a5b09f8ecfbf209f9381c692cdb15db0e5f658db6a139`). **Die River-Probe + Zahl Paper §4/§6 können laufen.**

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
