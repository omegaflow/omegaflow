<!--
  title: Handover — Mycelium-Folge 154 (register_sort url-order red geheilt, footprints/nrs `absent` terminal, `sgrep --all` gegen die gitignore-Sichtbarkeitslücke) (Stand 2026-09-25)
  session: Mycelium-Folge 154
  class: handover
  date: 2026-09-25
  sha256: fbc4b58871161a8569dce0ec73ac0a2ab9d86d3eddfed335d3469133528b4038
  status: live
-->
# Handover — Mycelium-Folge 154 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Trigger** / **Lage** / **Blockade** / **Braucht**;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).
Sortierung von Handlungsfähigkeit zu Nicht-Handlungsfähigkeit: `autonom` →
`operator-gebunden` → `blockiert` → `wartend` → `termin` → `LOCK`.
**Vorbereitung ≠ Akt:** diese Linie führt derzeit keinen operator-gebundenen Punkt.

Diese Session hat `handover-2026-09-25-mycelium-folge153.md` konsumiert.

## Stehender Pass (gemessen 2026-09-25)

- **HEAD** `0a0ce96d8` beim Start; der geteilte Baum rückte während des Atoms vor
  (`bc38f0a88` mountain folge155, `9e894f368` river folge23) — eigener Commit
  pfad-begrenzt, Fast-Forward geprüft.
- **Postfach** — `state/mail/` trägt keine Dateien (gemessen via `glob`);
  `mail_digest` liest den Ledger als absent. Kein handlungsrelevanter Neueingang.
- **CI am HEAD** — Watchdog-Snapshot 2026-09-25T08:06 + `ci_manage list`:
  jüngster `ci-check 36065950583` (@`0a0ce96d`) **failure** am `test`-Job
  (`ble.rs`-Fixture, sensory); `register_sort`-Step dort latent (Test-Job bricht
  vorher ab). `ci-check 36064053750` (@`de76fda`) **failure** an `register_sort`
  (1 ttl + 82 url) und `dropped-gate` (960→984). `dropped-gate` ist durch mountain
  folge155 geheilt (`dropped-baseline 989`). `ps1-cdn 36082231862` **success**
  (@`0a0ce96d`, 2026-09-25T01:29) überholt den roten `36067129156` (HTTP 500 der
  GitHub-Assets-API). `tools-build` success.
- **`open_points_check` folge153:** 21 Pfad-Refs, 1 absent =
  `phi/pipeline/stage/sources_potential_pre-cdn_9k_richest_converted.φ` — eine
  **Glob-Artefakt-Meldung**; die Datei existiert. `phi/pipeline` trägt **289
  Dateien** (gemessen via `find`), fast alle gitignored (`.gitignore:67`
  `phi/pipeline/*` + Whitelist); `glob` (gitignore-aware) und `sgrep`
  (`git ls-files`) blenden sie systematisch aus. Kein stale Punkt, sondern eine
  Werkzeug-Lücke — in diesem Atom geschlossen (`sgrep --all`).
- **`register_lookup --open`:** pipeline `ledger` 2, `index` 9, footprints 0,
  witnesses-`absent` terminal, `sources` 0; 1 candidate.

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

### pre-cdn Merge-Atom (Eingangsmaterial bestätigt, Review offen)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via `read`) Die Queue-Eingänge **existieren**:
  `phi/pipeline/queue/sources_potential_pre-cdn_9k_richest.φ` (9045 Z) und
  `..._params.φ` (561 Z) — untracked/gitignored, deshalb von `glob`/`sgrep`
  unsichtbar. Stage-Ausgänge unter `phi/pipeline/stage/` (mtime 2026-09-24).
  folge153 maß 887 Blöcke (richest 824 + params 63): **310** parse-Kandidaten,
  **494** pending review, **36** declined. Der `--port`-Treiber ist
  `bin/omegaflow --port <in> <out>` (`src/archivar/main_flow.rs:386` → `port.rs`),
  baufrei.
- **Blockade:** keine.
- **Braucht:** URL-Dedupe der 310 parse-Blöcke gegen `phi/sources.φ` → echte Neue
  nach `docs/SOURCE_PORT.md` §1.0 einschreiben (**sortiert**, `register_sort`-Kanon);
  494 pending einzeln reviewen (unit/cadence/force absent) → `blocked_sources.φ`
  `parser-def` oder Review; 36 declined verifizieren → `declined_sources.φ`.

#### Stufe 2 — operator-gebunden

keiner.

#### Stufe 3 — blockiert

### DEMETER Order 18387 (WAF, nicht Workflow)
- **Status:** blockiert | **Bindung:** dritter
- **Trigger:** F5-ASM-WAF erholt ODER Order-Ablauf 2026-09-28.
- **Lage:** (gemessen 2026-09-24 via `demeter_harvest.rs`/`ci_manage log`
  35851193831/REGARDS-API) `demeter-cdn.yml` + `demeter-aggregate-cdn.yml`
  existieren, Secrets gesetzt. Katalog-Scan erfolgreich (57760 URNs), aber jede
  `rs-order`-Erzeugung scheitert an `F5 ASM: Request Rejected` → Exit 137.
  `blocked_sources.φ:49`-Notiz korrigiert.
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** Wiedervorlage; bei Erholung `gh workflow run demeter-cdn.yml`.

### Fremdmodell-Benchmark (kein Browser-Target)
- **Status:** blockiert | **Bindung:** eigen (braucht Browser-MCP)
- **Trigger:** Browser-Target verbunden (`browser_targets` nicht leer).
- **Lage:** (gemessen 2026-09-25 via `browser_targets` = `[]`) kein Browser
  verbunden. Kontrollierter Benchmark N ≥ 5 harte Artefakte, identischer Prompt,
  blind bewertet, €/Qualität je Modell. Vollrekord
  `docs/surveys/survey-2026-09-24-fremdmodell-bedienung.md`: GLM-5.2/5.3 + Claude
  Sonnet 5 finden das Verdikt von `flyby-path-2-preregistration.md` unabhängig als
  tautologisch. Der `An future`-Rest (Claude- vs. DeepSeek-Kosten) bleibt bei future.
- **Blockade:** kein Browser-Target verbunden.
- **Braucht:** Browser verbinden, dann Benchmark fahren.

#### Stufe 4 — wartend

### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-check`-Lauf nach dem `register_sort`-Heil-Commit.
- **Lage:** (gemessen 2026-09-25 via `ci_manage list`/`log`) Die zwei Reds sind
  getrennt: `register_sort` (@`de76fda`) **von dieser Session geheilt**;
  `ble.rs`-Fixture (@`0a0ce96`, `src/archivar/ble.rs:1632`, ein fehlendes
  `0x00`-Byte) gehört **sensory** und ist dort uncommittet in Arbeit. Bei
  `0a0ce96` bricht der `test`-Job vor dem `register_sort`-Step ab.
- **Blockade:** sensory-`ble.rs`-Fix.
- **Braucht:** `ci_manage log <neuer ci-check> --all`; `register_sort`-Step muss
  nun grün sein.

### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ps1_dr2_coverage.fp01` erreicht (`all_present`).
- **Lage:** (gemessen 2026-09-25 via `ci_manage view 36082231862`) `ps1-cdn`
  **success** @`0a0ce96d`; ob der hierarchische Final-Combine `all_present` erreicht
  hat, ist **ungemessen** — die Note `footprints.φ:19` führt weiter `404`/`band_max
  2643` (Stand 2026-09-24). Der rote `36067129156` war ein HTTP 500, kein Code.
- **Blockade:** Ernte-Fortschritt; Final-Combine-Ergebnis ungemessen.
- **Braucht:** Asset `ps1_dr2_coverage.fp01` messen (`archive_search --verdict`);
  bei Vorhandensein PS1-Note finalisieren.

### src.pas TAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** `/tap/tables` 200.
- **Lage:** (gemessen 2026-09-24 via `archive_search --verdict`) `ledger.φ:10`
  ausstehend; `/tap` 200, `/tap/tables` 500 (`http://pithia.cbk.waw.pl/tap`).
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei Erholung.

### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Backend-Erholung.
- **Lage:** (gemessen 2026-09-24 via `blocked_sources.φ`/`archive_search`)
  `blocked_sources.φ:3`; api 000 (direct) / 5xx (Proton), Frontend 200.
- **Blockade:** Broker-Backend.
- **Braucht:** Multi-Exit-Re-Messung (Wiedervorlage).

### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** PSA-Antwort / Freigabe.
- **Lage:** (gemessen 2026-09-24 via `blocked_sources.φ`/`archive_search`)
  `blocked_sources.φ:21`; `release_date 2099-01-01`, `data?PRODUCT` 403.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

### SuperDARN MAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Globus-Task-`af68c4f1`-Status / Task-Ende.
- **Lage:** (gemessen 2026-09-24 via `blocked_sources.φ`) `blocked_sources.φ:16`
  `pending`; Zugang gewährt (Mail `1790021001`/`1790020962`); MAP 6561 Dateien/
  21,93 GB → `data/superdarn/map`. FITACF `sources.φ:9660`, RAWACF `:8318`.
- **Blockade:** Globus-Task-Status ungemessen (kein CLI/Token am Host).
- **Braucht:** Task-Status messen; bei Abschluss die Note schließen.

### SSDC Limadou (CSES-L2)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur / Sotgiu-Antwort.
- **Lage:** (gemessen 2026-09-24 via `ledger.φ`/`archive_search`) Operator-Wort
  **nein** (2026-09-23); `ledger.φ:14` „Permission Denied" (Konto `omegaflow`),
  Host 200.
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** wartend lassen.

#### Stufe 5 — termin

keiner. Der EMODNET-Termin (2026-10-19) ist in `docs/zustand/external-state.md`
eingetragen und lebt dort.

#### Stufe 6 — LOCK

keiner.

## In diesem Atom geschlossen (Register)

- **`phi/sources.φ` register_sort url-order red** — kanonisch sortiert
  (`ttl asc, url asc`): 1416 Blöcke, **0 ttl- und 0 url-order-Verletzungen**
  (vorher 1 ttl + 82 url; `register_sort`-Step `ci-check 36064053750`). Der Sort
  lief als exakte Reimplementierung von `register_sort.rs` in `awk`
  (`/tmp/opencode/reg_sort.awk`), weil `register_sort` nicht im
  `tools-latest`-Manifest stand; idempotent verifiziert, Zeilen-Multiset gleich,
  17/17 Zeilen (16 Blockmoves). **Wurzel behoben:** `register_sort` ist jetzt in
  `tools-build.yml` (Build + Manifest + Publish) — ab dem nächsten `tools-build`
  baufrei via `bin/.tools_ensure register_sort`; der hand-gerollte `awk`-Weg
  entfällt.
- **Werkzeug-Lücke `phi/pipeline` geschlossen** — `sgrep --all` walkt den
  Arbeitsbaum und schließt **gitignorierte** Dateien ein (überspringt
  `target`/`data`/`cache`/`.git`/`node_modules`); Tests `all_flag_parses` und
  `walk_all_lists_ignored_and_skips_heavy_dirs`; `docs/concepts/tools-map.md`
  (Interface + Modustabelle) aktualisiert; `cargo check -p omegaflow-utils --bin
  sgrep` 0/0.
- **Start-Pass gegen geerbte Alt-Info gehärtet** — `docs/handover/_template.md`:
  der CI-Watchdog-Snapshot wird nur gelesen, wenn er jünger ist als der letzte
  HEAD-Wechsel, sonst `ci_manage list` erzwungen; neue Pflichtzeile
  **Werkzeug-Frische** (Manifest-`git_sha` gegen HEAD, stale = `pending`); ein
  CDN-Claim ohne `archive_search --verdict`/`--sniff` im selben Atom = ungemessen.
  `docs/zustand/external-state.md` (lokal): neue Zeile `tools-latest Manifest`.
- **Register `absent` in `footprints.φ` / `nrs_stations.φ`** — Council-Verdikt (a)
  auf die Schwester-Register übertragen: `open_markers` `["pending","absent"]` →
  `["pending"]` (`register_lookup.rs:962`/`:987`); Gate-Tests
  `footprint_absent_is_terminal_not_an_open_duty` und
  `nrs_absent_is_terminal_not_an_open_duty`. `footprints.φ:24` nennt `absent`
  selbst „nie pending"; `nrs_stations.φ` trägt kein `absent` (inertes Token).
  `cargo check -p omegaflow-register --bin register_lookup` 0/0 in isoliertem
  Worktree (der geteilte Baum trägt fremdes, uncommittetes `ble.rs` mit E0308).

## Katalog-Pool (Register, nicht handlungsfähig)

- `phi/pipeline/ledger.φ` 2 offen = src.pas TAP + SSDC Limadou (beide oben, wartend).
- `phi/pipeline/index.φ` 9 offen = Tür-Kataloge (Adapter-Route, keine Quelle);
  Zähllinie, kein eigener Schritt.
- `candidates` 1 = `pipeline/catalog/archeology_gaps_index.φ` (ledger `verifiziert`,
  54 Kandidaten → 35 live/18 dead/1 blocked key).

## Benchmark

- Ausführung dieses Atoms: 3× `grind-flash` parallel (CI-Diagnose $0.0467,
  pre-cdn-Pfad-Recherche $0.0353, Register-`absent` $0.0268), Haupt-Linie
  (`register_sort`-awk, `sgrep --all`, `tools-build`, Handover, Abschluss). Kein
  `pro`/`max` nötig — Routine.
- **Befund:** Zwei Werkzeug-Wurzeln, nicht nur Symptome: (1) `register_sort` war
  nie publiziert → hand-gerollter `awk`-Sort; jetzt in `tools-build`. (2)
  `glob`/`sgrep`/`git ls-files` blenden das gitignorierte `phi/pipeline` (289
  Dateien) aus → `sgrep --all`. Nach dem Push `gh workflow run tools-build.yml`,
  damit `tools-latest` den neuen Stand trägt.

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien:** `phi/sources.φ` (Kanon-Sort),
  `tools/register/src/bin/register_lookup.rs` (absent-Marker + 2 Gate-Tests),
  `tools/utils/src/bin/sgrep.rs` (`--all` + 2 Tests),
  `.github/workflows/tools-build.yml` (`register_sort` publiziert),
  `docs/concepts/tools-map.md` (`sgrep --all`),
  `docs/handover/_template.md` (Start-Pass: Snapshot-Alter, Werkzeug-Frische,
  CDN-Claims), `docs/handover/handover-2026-09-25-mycelium-folge154.md`.
- **Move mit dem Commit:** `handover-2026-09-25-mycelium-folge153.md` → `archiv/`.
- **Lokal (gitignored, nicht getrackt):** `docs/zustand/external-state.md`,
  `phi/pipeline/queue/sources_potential_pre-cdn_9k_richest.φ`,
  `phi/pipeline/queue/sources_potential_pre-cdn_params.φ`,
  `phi/pipeline/stage/sources_potential_pre-cdn_9k_richest_converted.φ`,
  `phi/pipeline/stage/sources_potential_pre-cdn_params_converted.φ`.
- **Fremd im geteilten Baum:** keiner mehr — sensory committete `ble.rs` +
  die Handover-Moves (`4a183979c`); der Baum trägt nur noch diese Session.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, nie das Commit-Wort).
