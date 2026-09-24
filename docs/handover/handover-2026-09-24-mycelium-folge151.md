<!--
  title: Handover — Mycelium-Folge 151 (Planungs-Pass: Tafel nach Handlungsfähigkeit, Fremdmodell-Benchmark + Vorbereitung≠Akt gefaltet) (Stand 2026-09-24)
  session: Mycelium-Folge 151
  class: handover
  date: 2026-09-24
  sha256: 68f0145a09a1f7e0ddd8855433b292609a42d387e905ae7ca38a127d4d6da3e2
  status: live
-->
# Handover — Mycelium-Folge 151 (2026-09-24)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Trigger** / **Lage** / **Blockade** / **Braucht**;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).
Sortierung von Handlungsfähigkeit zu Nicht-Handlungsfähigkeit: `autonom` →
`operator-gebunden` → `blockiert` → `wartend` → `termin` → `LOCK`.
**Vorbereitung ≠ Akt** (Post `An mycelium`, 2026-09-24): ein `operator-gebundener`
Punkt wird in **zwei** Zeilen geführt — Vorbereitung autonom/dispatchbar, allein
der Akt operator; diese Linie führt derzeit keinen operator-gebundenen Punkt.

Diese Session hat `handover-2026-09-24-mycelium-folge150.md` konsumiert.

## Stehender Pass (gemessen 2026-09-24)

- **HEAD** `0f7a6b0de`; Arbeitsbaum sauber (`git status` leer, die in folge150
  genannten fremden Mods sind committet); `git_safety --snapshot` = „working tree
  equals HEAD — nothing to record".
- **Postfach** — `state/mail/mail_ledger.φ` absent (`mail_digest` pending, Bau
  gehört zu CI); kein handlungsrelevanter Eingang.
- **CI am HEAD** — Watchdog-Snapshot 2026-09-24T17:34:03: aktiv = `ps1-cdn
  36017540577`, `te-ncurve 35994664173`, `health-check 35990890566`; `failed`
  attempt 1: `ci-check 36000930169` (jüngste), `…35998482967`, `…35989139086`.
- **`open_points_check` folge151:** 12 Pfad-Refs, 0 absent (der frühere
  Backtick-Fehlalarm `canon.φ` entfällt); die 15 `format-gap lage unstamped`
  entfallen, weil jeder Lage-Stempel auf der `- **Lage:**`-Zeile steht.
- **`register_lookup --open`:** 116 Docs, offene Zeilen; pipeline: `ledger` 2,
  `index` 9, `witnesses` 4, `candidates` 1; Post offen.
- **Post gefaltet:** `An mycelium` (Fremdmodelle) → neuer Punkt
  `Fremdmodell-Benchmark`; `An mycelium` (Vorbereitung ≠ Akt) → Intro-Regel. Der
  `An future`-Rest bleibt bei future. `post.md` trägt die Zeilen noch — die Datei
  ist zum Pass-Zeitpunkt fremd-uncommittet (Mountain/River), darum nicht
  angefasst; die Löschung fällt mit dem post-eigenen Commit.

## Diese Session (git trägt es)

- **Planungs-Pass** (read-only) + **Post-Faltung**: Fremdmodell-Benchmark als
  offenen Punkt aufgenommen; Vorbereitung≠Akt-Regel übernommen; Lage aller Punkte
  gestempelt; keine Messung außerhalb des Passes, kein Bau.
- **CI-Dispatch:** `register-dropped 36029814914` (`gh workflow run
  register-dropped.yml`, HEAD `0f7a6b0de`) — full-history `--dropped`-Sweep. Der
  lokale Lauf ist am Compute-Ceiling abgebrochen; Ergebnis wird am nächsten Pass
  **einmal** via `ci_manage log 36029814914` gelesen (`DROPPED\tmycelium`-Zeilen
  gefiltert), kein Polling.

## Offen (aufgeschlüsselt)

### GitHub-Release-Asset-Cap
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Bau des Folge-Atoms.
- **Lage:** (gemessen 2026-09-24 via Council/Register) 1000 Assets/Release erreicht
  (`src/archivar/cdn.rs:41` `CAPPED_RELEASE`); Council-Verdikt 2026-09-24:
  **per-caller family tag, Rotation vollenden** — Rest ~24 Writer-Sites, deren
  `CDN_TAG` auf das gekappte Release zeigt (RPW/ps1/eve bereits rotiert).
- **Blockade:** keine (Architektur entschieden).
- **Braucht:** `grind-pro`-Atom „the capped release carries no writer": je Site
  Familien-Tag setzen, `cargo check` 0/0, CI-Manifestation verifizieren.

### Bayestar19-Asset-Manifestation
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Workflow-Netloc-Fix + CI-Lauf.
- **Lage:** (gemessen 2026-09-24 via archive_search --verdict/sgrep) Rust-Kette
  gebaut (`Buffer.bayestar` `src/archivar/spatial.rs:38`, Loader `main_flow.rs:2653`,
  `sightline_ebv` `membrane.rs:133`); CDN-Asset
  `…/dataverse.harvard.edu/bayestar2019.be19` **404** in allen 3 Stufen;
  `src/archivar/bayestar.rs:352` trägt `chunks_exact` (clippy) und
  `archivar::bayestar::tests::load_map_leaf_record_finds_the_pixel` rot;
  `bayestar-cdn.yml:23` prüft fälschlich `ssd.jpl.nasa.gov` (soll
  `dataverse.harvard.edu`). (Post `An mycelium` 2026-09-24 gefaltet.)
- **Blockade:** keine.
- **Braucht:** `bayestar.rs:352` auf `as_chunks` heilen + Leaf-Test grün;
  Workflow-Netloc fixen; `gh workflow run bayestar-cdn.yml`; sha256 in `phi/sources.φ`
  nachtragen.

### sources.φ dr3_stars netloc-Drift
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Register-Korrektur.
- **Lage:** (gemessen 2026-09-24 via archive_search --verdict) `sources.φ:8327`
  registriert `dr3_stars.bin` unter Tag `gea.esac.esa.int` → HTTP 404; dieselbe
  Datei (75 001 828 B, sha256 `fb9a1408…`) liefert `ssd.jpl.nasa.gov/dr3_stars.bin`
  200 (auch `frame_registry.φ:157`, `ztf-cdn.yml:22`).
- **Blockade:** keine.
- **Braucht:** `url`-Zeile auf den tragenden Tag umstellen (Netloc-Konvention prüfen).

### pre-cdn Stage-Regeneration
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Merge-Atom.
- **Lage:** (gemessen 2026-09-24 via Register) Queue gitignored → CI sieht sie
  nicht; 887 Blöcke, 141 keyless HAPI.
- **Blockade:** keine.
- **Braucht:** `--port` über `queue/sources_potential_pre-cdn_9k_richest.φ` +
  `…_params.φ`, gebunden im Merge-Atom.

### Witnesses absent (4)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Konsument-/Release-Entscheid.
- **Lage:** (gemessen 2026-09-24 via witnesses.φ) `witnesses.φ:7/13/37/91` `absent`
  (0 honored, regeneriert 2026-09-24): S²-Richtung/Energie-Feld real nicht geführt.
- **Blockade:** keine.
- **Braucht:** `declined`/`descoped`-Entscheid oder `absent` als Endzustand führen.

### Beat-Paar Datenquelle
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** eine gemessene Zwei-Ton-Rohquelle oder ein `descoped`-Befund.
- **Lage:** (gemessen 2026-09-24 via sgrep/Register) `beat_pair` (`shaders.rs:306`)
  verlangt zwei Oszillatoren im selben Kraft-Kanal, `df·dt<0.5`; Dual-Comb/Maser
  nur Paper bzw. `df·dt≥0.5`; kein Registry-Treffer.
- **Blockade:** keine.
- **Braucht:** Zwei-Ton-Rohquelle finden oder `descoped mit Befund`.

### DEMETER Order 18387
- **Status:** autonom | **Bindung:** eigen (Tooling)
- **Trigger:** CI-Workflow/Binary verfügbar.
- **Lage:** (gemessen 2026-09-24 via blocked_sources.φ) Operator-Wort ja
  (2026-09-23); `demeter_harvest.rs` (BATCH=100, `DMT_N1_1144`) vorhanden, aber kein
  Binary (`tools-latest` absent) und kein CI-Workflow. Order RUNNING, Ablauf 09-28;
  Neuordnung = 578 Orders, ~7 h CI-skalig. Register-`note` (55 218 / 34 %) weicht von
  der Handover-Zahl (56,9 %) ab — beim Bau nachmessen.
- **Blockade:** keine (Workflow-Bau ist ein Edit, kein lokaler Bau).
- **Braucht:** `demeter`-Workflow bauen (`demeter_harvest.rs` dispatchbar), dann
  `ci_manage view`.

### Fremdmodell-Benchmark
- **Status:** autonom | **Bindung:** eigen (braucht `chrome-devtools`-MCP)
- **Trigger:** Post `An mycelium` 2026-09-24.
- **Lage:** (gemessen 2026-09-24 via Post/Survey) GLM-5.2/5.3 (z.ai) + Claude
  Sonnet 5 (claude.ai) über `chrome-devtools`-MCP als unabhängige Reviewer des
  `docs/paper/flyby-path-2-preregistration.md` finden alle drei unabhängig das
  Verdikt als tautologisch (Vorhersage = Messung, nicht falsifizierbar); Claude
  ergänzt den Common-cause-Confound (Kp/Bz → Drag, TEC/Plasma → Doppler) und einen
  Fix (Residuenbudget + Entscheidungsregel, Pipeline hashen). Methode: z.ai Anhang
  via `upload_file` (Drop-Zone), claude.ai synthetischer Paste. Vollrekord
  `docs/surveys/survey-2026-09-24-fremdmodell-bedienung.md`.
- **Blockade:** keine.
- **Braucht:** kontrollierter Benchmark N ≥ 5 harte Artefakte, identischer Prompt,
  blind bewertet, €/Qualität je Modell. Der `An future`-Rest (Claude- vs.
  DeepSeek-Kosten) bleibt bei future.

### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-check`-Lauf.
- **Lage:** (gemessen 2026-09-24 via Watchdog/ci_manage) `ci-check` mehrfach rot
  (attempt 1, jüngste `36000930169`); folge150-Lauf `35924695548` zuletzt pending.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>`; Ergebnis ins nächste Handover.

### dropped-gate-Baseline
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-check`-Lauf.
- **Lage:** (gemessen 2026-09-24 via register-dropped) `register-dropped
  35922188109` success; lokal `register_lookup --dropped --count` = 3258;
  `docs/zustand/dropped-baseline.md` Baseline 678 @`48ae5e734`.
- **Blockade:** keine.
- **Braucht:** prüfen, ob der nächste `ci-check` `dropped-gate` grün ist; sonst
  Baseline im annehmenden Commit heben.

### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ps1-cdn`-Lauf bis `all_present`.
- **Lage:** (gemessen 2026-09-24 via footprints.φ/ps1-cdn) `ps1_dr2_coverage.fp01`
  HTTP 404; Band-Parts 637–671, `band_max 2643`; Note in `footprints.φ:19`
  aktualisiert.
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** bei `all_present` PS1-Note finalisieren.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin
- **Trigger:** `/tap/tables` 200.
- **Lage:** (gemessen 2026-09-24 via archive_search --verdict) `ledger.φ:10`
  ausstehend; `/tap` 200, `/tap/tables` 500 (`http://pithia.cbk.waw.pl/tap`).
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei Erholung.

### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Backend-Erholung.
- **Lage:** (gemessen 2026-09-24 via blocked_sources.φ/archive_search)
  `blocked_sources.φ:3`; api 000 (direct) / 5xx (Proton), Frontend 200;
  Ein-Exit-Stichprobe 500 vs. notiertem 502.
- **Blockade:** Broker-Backend.
- **Braucht:** Multi-Exit-Re-Messung (Wiedervorlage).

### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** PSA-Antwort / Freigabe.
- **Lage:** (gemessen 2026-09-24 via blocked_sources.φ/archive_search)
  `blocked_sources.φ:21`; `release_date 2099-01-01`, `data?PRODUCT` 403.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

### SuperDARN MAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Globus-Task-`af68c4f1`-Status / Task-Ende.
- **Lage:** (gemessen 2026-09-24 via blocked_sources.φ) `blocked_sources.φ:16`
  `pending`; Zugang gewährt (Mail `1790021001`/`1790020962`); MAP 6 561 Dateien/
  21,93 GB → `data/superdarn/map` (Task `af68c4f1`); FITACF `sources.φ:9465`,
  RAWACF `:8032` registriert. In folge137–149 offener Punkt, in folge150/151
  nicht mehr getragen → **Dropped-Audit** (unten).
- **Blockade:** Globus-Task-Status ungemessen (kein CLI/Token am Host, folge150).
- **Braucht:** Task-Status messen; bei Abschluss die Note schließen, sonst weiter
  tragen.

### SSDC Limadou (CSES-L2)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur / Sotgiu-Antwort.
- **Lage:** (gemessen 2026-09-24 via ledger.φ/archive_search) Operator-Wort **nein**
  (2026-09-23); `ledger.φ:14` „Permission Denied" (Konto `omegaflow`), Host 200.
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** wartend lassen.

### EMODNET HFRADAR NADR
- **Status:** termin | **Bindung:** termin 2026-10-19
- **Trigger:** Datum 2026-10-19.
- **Lage:** (gemessen 2026-09-24 via external-state.md) Re-Messung fällig
  2026-10-19.
- **Blockade:** Termin.
- **Braucht:** Re-Messung.

## Dropped-Audit (gemessen 2026-09-24, Kandidaten)

Beim Überarbeiten (folge137–151) hand-gemessene Kandidaten für still gedroppte
Punkte. Die **commit-aufgelöste** Wahrheit liefert `register_lookup --dropped
mycelium`; lokaler Lauf am Compute-Ceiling abgebrochen, CI-Sweep `register-dropped
36029814914` dispatcht (Ergebnis noch nicht gelesen, ein `ci_manage log`-Call am
nächsten Pass). Klassifikation (resolve/rename/descope vs. echter Drop) offen bis
zum Sweep — die Tabelle ist die Messung, nicht das Urteil.

| Punkt | erschien | fehlt ab | vorläufige Bewertung |
|---|---|---|---|
| **SuperDARN MAP** | 137–149 | 150 (nur „Diese Session" → `post.md`), 151 | stillster Drop — jetzt als offener Punkt getragen |
| **Sicherheits-Befund** | 144–149 | 150 (→ `post.md`), 151 | geroutet; ob geschlossen, offen |
| dropped-Zähler-Wurzel (→ mountain) | 148–149 | 150 | an mountain geroutet (benannt) |
| Free-Model-Bench | 137–145 | 146 | gemessen geschlossen (`3ab8fdc00`) — kein Drop |
| SSDC Limadou | 137–143 | 144–149, wieder 150 | Lücke 144–149 |
| 13. Korpus | 144–146 | 147 | resolve/rename offen |
| Pipeline-Rest (50 Domains) | 147 | 148 | dito |
| 7 Feld-Kandidaten (13. Korpus) | 147–148 | 149 | dito |
| Rosetta ODF (CDN) | 147–148 | 149 | dito |
| DataONE Katalog-/Terms-Lizenz | 141–148 | 149 | dito |
| Katalog-Wald (Rest) | 137–145 | 146 | dito |
| NRS-Register-Lücke | 142 | 143 | dito |
| Gaia-DR3-Alerts / NOAA-S3 L1B / CI format-Job | 137–138 | 139 | dito |

## Benchmark

- Planungs-Pass ohne Dispatch (read-only). Vorgeschlagene Ausführung: parallel —
  3× `grind-flash` (Bayestar19-Heilung, dr3_stars-Netloc, pre-cdn-Stage),
  3× `grind-pro` (Asset-Cap, DEMETER-Workflow, Witnesses-Entscheid),
  1× `research-max` (Beat-Paar), 1× Haupt-Linie (Fremdmodell-Benchmark, MCP).
- **Befund (fortgeschrieben aus folge150):** zwei Operator-Worte (DEMETER,
  SuperDARN) waren trotz Wort **maschinell nicht ausführbar**; die Infrastruktur ist
  der zweite Gatter. DEMETER trägt jetzt einen eigenen Schritt (Workflow bauen).

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Datei dieser Session:** `docs/handover/handover-2026-09-24-mycelium-folge151.md`
  (überarbeitet: Post gefaltet, Lage gestempelt).
- **Move mit dem Commit:** `handover-2026-09-24-mycelium-folge150.md` → `archiv/`.
- **Fremd im geteilten Baum:** `docs/handover/_template.md`, `docs/handover/post.md`
  und die Mountain/River-Übergaben (uncommittet, andere Linien) — nicht Teil dieses
  Commits; `post.md` wird wegen der fremden Hunks nicht mitcommittet.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
