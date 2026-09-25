<!--
  title: Handover — Mycelium-Folge 155 (Artefakt-Frische generalisiert, tools-build publiziert die Session-Crates, lokaler Einzel-Bin-Build erlaubt) (Stand 2026-09-25)
  session: Mycelium-Folge 155
  class: handover
  date: 2026-09-25
  sha256: e1a27791e3832360d9a87054c6b5f81ad51851c1b4ff1a059ccdc6a0f9a6e611
  status: live
-->
# Handover — Mycelium-Folge 155 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Trigger** / **Lage** / **Blockade** / **Braucht**;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).
Sortierung von Handlungsfähigkeit zu Nicht-Handlungsfähigkeit: `autonom` →
`operator-gebunden` → `blockiert` → `wartend` → `termin` → `LOCK`.
**Vorbereitung ≠ Akt:** diese Linie führt derzeit keinen operator-gebundenen Punkt.

Diese Session hat `handover-2026-09-25-mycelium-folge154.md` konsumiert.

## Stehender Pass (gemessen 2026-09-25)

- **HEAD** `5a546d80d` == `origin/main` beim Start (Mycelium-Folge 154); der
  Baum trug nur die eigenen A/B/C-Änderungen.
- **Postfach** — `state/mail/` ohne Dateien (gemessen via `glob`); kein
  handlungsrelevanter Neueingang.
- **CI am HEAD** — (gemessen via `ci_manage list`) `tools-build 36103377735`
  (**success** @`5a546d80d`, Push-Trigger); `tools-build 36103383167`
  (in_progress, der in folge154 dispatchte register_sort-Lauf); `ci-check
  36103377842` **pending**; `allwise-cdn` in_progress; `register-dropped`
  in_progress.
- **Artefakt-Frische** — der Start-Pass misst ab jetzt **alle** erzeugten
  Klassen (Tools, Core-Bin, CDN-Assets, WGSL-Kernel, Firmware) gegen HEAD
  (`_template.md`; Eintrag `docs/zustand/external-state.md`). In folge154 war
  `tools-latest=0a0ce96d` vs HEAD `4a183979c`; nach dem Push liegt ein neuer
  `tools-build` success vor.
- **`register_lookup --open`:** pipeline `ledger` 2, `index` 9, `sources` 0,
  footprints 0; 1 candidate.

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

### pre-cdn Merge-Atom (Eingangsmaterial vorhanden, Review offen)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via `read`) Queue-Eingänge vorhanden
  (`phi/pipeline/queue/sources_potential_pre-cdn_9k_richest.φ` 9045 Z,
  `..._params.φ` 561 Z; gitignored), Stage-Ausgänge unter `phi/pipeline/stage/`.
  folge153 maß 887 Blöcke: **310** parse, **494** pending review, **36** declined.
- **Blockade:** keine.
- **Braucht:** URL-Dedupe der 310 gegen `phi/sources.φ` → Neue nach
  `docs/SOURCE_PORT.md` §1.0 **sortiert** einschreiben; 494 einzeln reviewen →
  `blocked_sources.φ` `parser-def` oder Review; 36 declined verifizieren →
  `declined_sources.φ`.

#### Stufe 2 — operator-gebunden

keiner.

#### Stufe 3 — blockiert

### DEMETER Order 18387 (WAF, nicht Workflow)
- **Status:** blockiert | **Bindung:** dritter
- **Trigger:** F5-ASM-WAF erholt ODER Order-Ablauf 2026-09-28.
- **Lage:** (gemessen 2026-09-24 via `demeter_harvest.rs`/`ci_manage log`)
  `rs-order`-Erzeugung scheitert an `F5 ASM: Request Rejected` → Exit 137;
  `blocked_sources.φ:49`-Notiz korrigiert.
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** Wiedervorlage; bei Erholung `gh workflow run demeter-cdn.yml`.

### Fremdmodell-Benchmark (kein Browser-Target)
- **Status:** blockiert | **Bindung:** eigen (braucht Browser-MCP)
- **Trigger:** Browser-Target verbunden (`browser_targets` nicht leer).
- **Lage:** (gemessen 2026-09-25 via `browser_targets` = `[]`) kein Browser
  verbunden. Rekord `docs/surveys/survey-2026-09-24-fremdmodell-bedienung.md`.
- **Blockade:** kein Browser-Target verbunden.
- **Braucht:** Browser verbinden, dann Benchmark fahren.

#### Stufe 4 — wartend

### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-check`-Lauf (36103377842 pending).
- **Lage:** (gemessen 2026-09-25 via `ci_manage list`) der `register_sort`-Red
  ist geheilt und `register_sort` wird jetzt publiziert; der `ble.rs`-Fix ist
  sensory-committet (`4a183979c`). Der `dropped-gate` ist durch mountain folge155
  geheilt (`dropped-baseline 989`).
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36103377842`; Ergebnis ins nächste Handover.

### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ps1_dr2_coverage.fp01` erreicht (`all_present`).
- **Lage:** (gemessen 2026-09-25 via `ci_manage view 36082231862`) `ps1-cdn`
  success @`0a0ce96d`; Final-Combine `all_present` **ungemessen** (Note
  `footprints.φ:19` weiter 404/`band_max 2643`).
- **Blockade:** Ernte-Fortschritt; Final-Combine ungemessen.
- **Braucht:** Asset messen (`archive_search --verdict`); bei Vorhandensein Note
  finalisieren.

### src.pas TAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** `/tap/tables` 200.
- **Lage:** (gemessen 2026-09-24 via `archive_search --verdict`) `ledger.φ:10`;
  `/tap/tables` 500 (`http://pithia.cbk.waw.pl/tap`).
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
  `pending`; Zugang gewährt; MAP 6561 Dateien/21,93 GB.
- **Blockade:** Globus-Task-Status ungemessen (kein CLI/Token am Host).
- **Braucht:** Task-Status messen; bei Abschluss die Note schließen.

### SSDC Limadou (CSES-L2)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur / Sotgiu-Antwort.
- **Lage:** (gemessen 2026-09-24 via `ledger.φ`/`archive_search`) Operator-Wort
  **nein** (2026-09-23); `ledger.φ:14` „Permission Denied", Host 200.
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** wartend lassen.

#### Stufe 5 — termin

keiner. Der EMODNET-Termin (2026-10-19) lebt in `docs/zustand/external-state.md`.

#### Stufe 6 — LOCK

keiner.

## In diesem Atom geschlossen (Register)

- **A — Artefakt-Frische generalisiert** — `docs/handover/_template.md`:
  der Start-Pass misst jetzt **alle erzeugten Klassen** (Session-Tools,
  Core-Bin, CDN-Daten-Assets, WGSL-Kernel, Firmware) gegen HEAD; eine
  Verhaltens-Aussage aus einem Artefakt hinter HEAD ist `pending`.
  `docs/zustand/external-state.md` (lokal): Zeile `Artefakt-Frische (alle
  Klassen)`.
- **B — `tools-build` publiziert die Session-Crates ganz** — die Register-,
  Utils- und Gate-Crates werden mit `--bins` gebaut und in `tools-latest`
  publiziert (Manifest/Publish aus einer `TOOLS`-Liste, 56 statt 22 Tools;
  `register_sort`, `sgrep`, die Register-Scans `pii_exposure`/`path_reference_scan`/
  `number_audit`/`home_scan`/… sind damit frisch auf PATH). `cargo check --bins
  -p omegaflow-register -p omegaflow-utils -p omegaflow-gate` 0/0 vorab gemessen.
  Die 243 measure-/227 harvest-Probe-Bins bleiben CI-Körper, nicht Session-Tools.
- **C — lokaler Einzel-Bin-Build erlaubt** — `AGENTS.md`: lokal sind jetzt
  `cargo check`, `cargo fmt -- <eigene Pfade>` **und** der gezielte
  Einzel-Bin-Lauf `cargo build -p <crate> --bin <name>` / `cargo run -p <crate>
  --bin <name>` (still, `OMEGAFLOW_HIDDEN=1`); whole-crate/`--release`/test/bench
  bleiben CI. `opencode.json`: `cargo build -p *`/`cargo run -p *` in der
  `line`- und der Top-Level-Permission ergänzt (JSON validiert). **Braucht
  opencode-Neustart**, damit die Config greift.

## Katalog-Pool (Register, nicht handlungsfähig)

- `phi/pipeline/ledger.φ` 2 offen = src.pas TAP + SSDC Limadou (beide oben, wartend).
- `phi/pipeline/index.φ` 9 offen = Tür-Kataloge (Adapter-Route, keine Quelle);
  Zähllinie, kein eigener Schritt.
- `candidates` 1 = `pipeline/catalog/archeology_gaps_index.φ`.

## Benchmark

- A/B/C in der Haupt-Linie (Config/Workflow/Doc); Vorab-Messung `cargo check
  --bins` der drei Crates (0/0). Kein Sub-Dispatch, kein `pro`/`max`.
- **Befund:** die 22/576-Kluft war die Wurzel des „Hinterherhinkens"; sie ist
  mit B auf die Session-Crates geschlossen und mit C für jeden Einzel-Bin
  lokal aufholbar. Die `--release`- und whole-crate-Builds bleiben CI.

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien:** `AGENTS.md` (C-Regel), `opencode.json` (C-Permissions),
  `.github/workflows/tools-build.yml` (B), `docs/handover/_template.md` (A),
  `docs/handover/handover-2026-09-25-mycelium-folge155.md`.
- **Move mit dem Commit:** `handover-2026-09-25-mycelium-folge154.md` → `archiv/`.
- **Lokal (gitignored, nicht getrackt):** `docs/zustand/external-state.md`.
- **Fremd im geteilten Baum:** keiner — der Baum trägt nur diese Session.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Nach dem Push
`gh workflow run tools-build.yml` dispatchten; `AGENTS.md`/`opencode.json`
werden erst nach einem opencode-Neustart wirksam.
