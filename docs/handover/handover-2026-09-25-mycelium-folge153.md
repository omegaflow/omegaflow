<!--
  title: Handover — Mycelium-Folge 153 (Bayestar-sha manifestiert, Dropped-Audit klassifiziert, pre-cdn Stage regeneriert, witnesses-`absent` terminal geheilt) (Stand 2026-09-25)
  session: Mycelium-Folge 153
  class: handover
  date: 2026-09-25
  sha256: 2bf9b5224f2efb8a30b69a1c0f693e82de39e744ef1bb4e6cfefba6847c0817d
  status: live
-->
# Handover — Mycelium-Folge 153 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Trigger** / **Lage** / **Blockade** / **Braucht**;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).
Sortierung von Handlungsfähigkeit zu Nicht-Handlungsfähigkeit: `autonom` →
`operator-gebunden` → `blockiert` → `wartend` → `termin` → `LOCK`.
**Vorbereitung ≠ Akt:** diese Linie führt derzeit keinen operator-gebundenen Punkt.

Diese Session hat `handover-2026-09-24-mycelium-folge152.md` konsumiert.

## Stehender Pass (gemessen 2026-09-24/25)

- **HEAD** `de76fda6a` (== `origin/main`) beim Abschluss; der geteilte Baum rückte
  vor (fremder style-Commit `de76fda6a`) — eigener Commit pfad-begrenzt,
  Fast-Forward geprüft.
- **Postfach** — `state/mail/mail_ledger.φ`; keine handlungsrelevante
  Neueingang für diese Linie (Watchdog-Snapshot 2026-09-24T22:54).
- **CI am HEAD** — Watchdog: aktiv `te-ncurve 36052804297`, `health-check
  36027534328`. `ci_manage list` (2026-09-24T21:57Z @ `de76fda6a`): jüngster
  `tools-build 36064053749` success; jüngste `ci-check 36064053750` pending, davor
  nur cancelled — kein frischer success/failure; der alte `tools-build
  35844365704` (release-asset 404) ist **nicht mehr aktuell** (Folgeläufe success).
- **`open_points_check` folge152:** 20 Pfad-Refs, 0 absent, 0 guardians, 0
  format-gaps, 0 owner-drift.
- **`register_lookup --open`:** 116 Docs, 584 offen; pipeline: `ledger` 2,
  `index` 9, witnesses 4 (vor dem Fix — der auf PATH liegende Alt-Bin kennt die
  neue Semantik noch nicht; CI-`tools-build` regeneriert), `footprints` 0,
  `sources` 0; 1 candidates.

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

### pre-cdn Merge-Atom (Stage regeneriert, Review offen)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-24 via `bin/omegaflow --port`) 887 Blöcke konvertiert
  (richest 824 + params 63): **310** parse-Kandidaten, **494** pending review,
  **36** declined. Stage-Dateien (gitignored) unter `phi/pipeline/stage/`:
  `sources_potential_pre-cdn_9k_richest_converted.φ` (7010 Z),
  `…_params_converted.φ` (439 Z). Der `--port`-Treiber ist der Core-Bin-Modus
  (`src/archivar/main_flow.rs:386` → `port.rs`), baufrei über `bin/omegaflow`.
- **Blockade:** keine.
- **Braucht:** URL-Dedupe der 310 parse-Blöcke gegen `phi/sources.φ` (13917 Z) →
  echte Neue nach `docs/SOURCE_PORT.md` §1.0 einschreiben; 494 pending einzeln
  reviewen (unit/cadence/force absent) → `blocked_sources.φ` `parser-def` oder
  Review; 36 declined verifizieren → `declined_sources.φ`.

### Register `absent` in `footprints.φ` / `nrs.φ` (gleiche Achse, ungemessen)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Plan-Pass.
- **Lage:** (gemessen 2026-09-24 via Read `register_lookup.rs:962`,`:987`) Das
  Council-Verdikt (a) ist für `phi/witnesses.φ` umgesetzt (open_markers
  `["pending","absent"]`→`["pending"]`, Gate-Test). `footprints.φ` und `nrs.φ`
  tragen **dasselbe** Muster `["pending","absent"]` — hier **ungemessen**.
- **Blockade:** keine.
- **Braucht:** je Register messen, ob `absent` dort terminal (0 honored) oder
  Ernte-Schuld (`pending`) ist; analog entscheiden, Gate-Test im selben Atom.

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
- **Trigger:** nächster `ci-check`-Lauf nach dem Red-Heil-Commit.
- **Lage:** (gemessen 2026-09-24 via `ci_manage list` @ `de76fda6a`) jüngste
  `ci-check 36064053750` pending, davor nur cancelled — kein frisches Urteil.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>`; Ergebnis ins nächste Handover.

### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ps1-cdn`-Lauf bis `all_present`.
- **Lage:** (gemessen 2026-09-24 via `footprints.φ`/`ps1-cdn`)
  `ps1_dr2_coverage.fp01` HTTP 404; Band-Parts 637–671, `band_max 2643`;
  Note in `footprints.φ:19`.
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** bei `all_present` PS1-Note finalisieren.

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

- **Bayestar19-Manifestation** — `bayestar-cdn.yml` Lauf `36062991395` **success**
  (2026-09-24T21:40Z); Asset `bayestar2019.be19` manifestiert (HTTP 200). Der
  `archive_search --sniff`-Weg bricht > ~96 MB ab (kein Voll-Hash); sha256
  `970cd3a2a5f1f66859f93b615d1c203512d430203c2cb73c0476495d3f77b41f` über die
  GitHub-Releases-API (`assets[].digest`), eingetragen in `phi/sources.φ`
  (bayestar-Block).
- **Dropped-Audit** — `ci_manage log 36029814914 --all`: 3402 dropped / 2718
  commit-resolved. Alle 13 Kandidaten der folge151-Tabelle klassifiziert:
  `resolve`/`rename`/`descope` mit Baum-/Register-Beleg, **kein echter Drop**.
- **witnesses-`absent`** — Council-Verdikt (a): `absent` ist ein terminaler
  Feldzustand (0 honored), keine Ernte-Schuld. `register_lookup.rs:951`
  open_markers `["pending","absent"]`→`["pending"]` (released bleibt `declined`);
  Gate-Test `witness_absent_is_terminal_not_an_open_duty`. `cargo check -p
  omegaflow-register` 0/0. Die vier Notizen bleiben unangetastet.
- **Zustand-DUE** — `external-state.md` (lokal, gitignored): `:22` CI-Status,
  `:29` GitHub-Release-Asset-Cap (Umbau umgesetzt, `cdn.rs:42`), `:33` Lokales
  Release-Binär nachgezogen; `:26`/`:27`/`:28` (CDN-Assets) gültig; EMODNET-Zeile
  ergänzt.

## Katalog-Pool (Register, nicht handlungsfähig)

- `phi/pipeline/ledger.φ` 2 offen = src.pas TAP + SSDC Limadou (beide oben, wartend).
- `phi/pipeline/index.φ` 9 offen = Tür-Kataloge (Adapter-Route, keine Quelle);
  Zähllinie, kein eigener Schritt.
- `candidates` 1 = `pipeline/catalog/archeology_gaps_index.φ` (ledger `verifiziert`,
  54 Kandidaten → 35 live/18 dead/1 blocked key).

## Benchmark

- Planungs-Pass ohne Dispatch. Ausführung: 4× `grind-flash` (Bayestar, Dropped-Audit,
  Release-Digest, Zustand-DUE), 1× `grind-pro` (pre-cdn `--port`), 1× `council`
  (witnesses-Register-Semantik), Haupt-Linie (Register-Fix, zustand-Edits, Handover).
- **Befund:** `bin/omegaflow --port` ist der baufreie Weg für die Stage-Regeneration
  (der Core-Bin-Modus; kein lokaler Build). `archive_search --sniff` trägt keinen
  Voll-Hash > ~96 MB — der Release-Asset-Digest ist die Messung (`assets[].digest`).

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien:** `tools/register/src/bin/register_lookup.rs`,
  `phi/sources.φ` (bayestar sha256), `docs/handover/handover-2026-09-25-mycelium-folge153.md`.
- **Move mit dem Commit:** `handover-2026-09-24-mycelium-folge152.md` → `archiv/`.
- **Lokal (gitignored, nicht getrackt):** `docs/zustand/external-state.md`
  (4 Zeilen), `phi/pipeline/stage/*_converted.φ`.
- **Fremd im geteilten Baum (uncommittet, andere Linien):** `src/archivar/ble.rs`
  (sensory), `src/archivar/fit.rs` (sensory) — nicht Teil dieses Commits.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, nie das Commit-Wort).
