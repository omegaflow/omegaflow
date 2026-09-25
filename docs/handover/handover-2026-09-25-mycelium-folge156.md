<!--
  title: Handover — Mycelium-Folge 156 (2026-09-25)
  session: Mycelium-Folge 156
  class: handover
  date: 2026-09-25
  sha256: 8ff3eea6552dec63dfba8fe1a08b70dbcf9aa7695aae34af84a9d25a981c109d
  status: live
-->
# Handover — Mycelium-Folge 156 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Geschichts-Abschnitte (Stehender-Pass-Ergebnis, geschlossen-Register,
Benchmark, Geteilter Baum): sie leben in git. Geteilter externer Zustand lebt in
`docs/zustand/external-state.md`, nie als Kopie hier. Keine Rangfolge — die offenen
Punkte werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** / **Blockade** /
**Braucht**. Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session hat `handover-2026-09-25-mycelium-folge155.md` konsumiert.

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

### Gate-vs-Bestand — Riss (Ratsverdikt „angleichen", Konsument widerspricht)
- **Status:** autonom | **Bindung:** eigen (Rats-/Operator-Wort)
- **Trigger:** Rats-/Operator-Wort zum konsumentengestützten Kohort.
- **Lage:** (gemessen 2026-09-25 via Rat + Taucher-Inventar) der Rat ist einstimmig
  „angleichen"; Inventar: **99 disponierbar** (93 model-forecast, 4 aggregated-index,
  1 no-physical-force, 1 catalog), **43 stehen gelassen** (31 `format reference`, 2 EOP,
  2 Neutronenmonitor, 2 DONKI, 2 TNO-Kataloge, 1 TNS, 3 global-mean GHG). **Riss:** die
  90 `archive-api.open-meteo.com`-Blöcke (ERA5-Reanalyse) tragen eine **CDN-Release**
  (`archive-api.open-meteo.com`, 90 Assets = 30×3) und die Tibet-Flut-Papiere
  (`docs/blatt/blatt-kreuz-screening-kollab.md:128`, `.github/workflows/trishuli-pfeil.yml:25`);
  die Taucher-Datensatzprüfung hakte sie fälschlich als „kein Datensatz" ab. Der Austrag
  wurde **zurückgesetzt** — `phi/sources.φ` trägt die 90 Blöcke weiter (canonical, 1422 Blöcke).
- **Blockade:** Riss — Gate (Reanalyse ≠ Messung) vs. gebauter Konsument (Papiere + CDN-Release).
- **Braucht:** Rats-/Operator-Wort: (a) die open-meteo-Kohorte **behalten** (Riss tragen) und
  die übrigen 9 disponieren, oder (b) die CDN-Release-Assets zuerst sichern/dokumentieren,
  dann auch die 90 disponieren. Belege: `git show refs/safety/1790320430:phi/sources.φ`
  (Vor-Zustand), `git log --oneline -1` für den Ausgangs-HEAD.

### `phi/declined_sources.φ` — Ordnung + Duplikate
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via `register_sort`/`awk`) **1283 Blöcke**, 116
  benachbarte `url`-Inversionen, 1282/1283 Positionen unsortiert; **2 Duplikat-URLs**
  (`https://zenodo.org/records/8401262`, `https://zenodo.org/records/8427755`) — §1.0
  verlangt url-Ordnung und „keine Duplikate pro URL". `register_sort` unterstützt das
  Register **nicht** (`block does not open with 'url'` → exit 2; verlangt `url`-Öffner + `ttl`).
- **Blockade:** keine (Werkzeug-Lücke).
- **Braucht:** `tools/utils/src/bin/register_sort.rs` um Dispositions-Register erweitern
  (Blocköffner `decline`/`dead`/`key-needed`/`parser-def`, Sortierschlüssel `url`, `ttl`
  optional) + Gate-Test; dann anwenden und die 2 Duplikate entscheiden.

### pre-cdn verlorene Blöcke (Pipeline-Korpus, nicht leer)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via `wc`/`pre-cdn_join_report.txt`)
  `phi/pipeline/stage/pre_cdn_lost_blocks_unpooled.φ` **51089 Z / ~4877 Blöcke** ohne
  Pool-Eintrag („own extraction, blockade"); die richest-/params-Kohorte (887 Blöcke) ist
  disponiert. `phi/pipeline/index.φ` 9 offen = Tür-Kataloge (Adapter-Route, keine Quelle).
- **Blockade:** keine.
- **Braucht:** eigenen Pool-Join-/Dispositions-Pass starten (`docs/SOURCE_PORT.md` §5);
  Blockade je Netloc messen.

### `phi/pipeline/index.φ` — Verweise auf generierte/orphane Dateien
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via `sread`) `phi/sources_index.φ` (39,7 MB, gitignored,
  CI-Artefakt aus `kernel-flatten.yml`) ist in `phi/pipeline/index.φ:123` als `register`
  geführt; `phi/pipeline/prompt.φ` (gitignored) in `index.φ:127` als infra. Beide sind
  keine Register.
- **Blockade:** keine.
- **Braucht:** Marker in `index.φ` auf generiert/infra umstellen (Register-Edit,
  SOURCE_PORT-konform).

#### Stufe 2 — operator-gebunden

keiner.

#### Stufe 3 — blockiert

### DEMETER Order 18387 (WAF, nicht Workflow)
- **Status:** blockiert | **Bindung:** dritter
- **Trigger:** F5-ASM-WAF erholt ODER Order-Ablauf 2026-09-28.
- **Lage:** (gemessen 2026-09-24 via `demeter_harvest.rs`/`ci_manage log`)
  `rs-order`-Erzeugung scheitert an `F5 ASM: Request Rejected` → Exit 137.
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** Wiedervorlage; bei Erholung `gh workflow run demeter-cdn.yml`.

### Fremdmodell-Benchmark (kein Browser-Target)
- **Status:** blockiert | **Bindung:** eigen (braucht Browser-MCP)
- **Trigger:** Browser-Target verbunden (`browser_targets` nicht leer).
- **Lage:** (gemessen 2026-09-25 via `browser_targets` = `[]`) kein Browser
  verbunden. Rekord `docs/surveys/survey-2026-09-24-fremdmodell-bedienung.md`.
- **Blockade:** kein Browser-Target.
- **Braucht:** Browser verbinden, dann Benchmark fahren.

#### Stufe 4 — wartend

### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`-Lauf am HEAD.
- **Lage:** (gemessen 2026-09-25 via `ci_manage list`) `ci-check 36104225522`
  **pending**; `tools-build 36103747012` success. `phi/sources.φ` ist canonical
  (1422 Blöcke, `register_sort` 0 Verstöße).
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36104225522`.

### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ps1_dr2_coverage.fp01` erreicht (`all_present`).
- **Lage:** (gemessen 2026-09-25 via `ci_manage view 36082231862`) `ps1-cdn`
  success; Final-Combine **ungemessen** (`footprints.φ:19` 404).
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** `bin/archive_search --verdict <ps1_dr2_coverage.fp01-url>`; bei
  Vorhandensein Note finalisieren.

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
  api 000 (direct) / 5xx (Proton), Frontend 200.
- **Blockade:** Broker-Backend.
- **Braucht:** Multi-Exit-Re-Messung (Wiedervorlage).

### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** PSA-Freigabe.
- **Lage:** (gemessen 2026-09-24 via `blocked_sources.φ`) `release_date 2099-01-01`,
  `data?PRODUCT` 403.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

### SuperDARN MAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Globus-Task `af68c4f1`-Status / Task-Ende.
- **Lage:** (gemessen 2026-09-24 via `blocked_sources.φ`) Zugang gewährt; MAP
  6561 Dateien/21,93 GB; Task-Status ungemessen (kein CLI/Token am Host).
- **Blockade:** Globus-Task-Status.
- **Braucht:** Task-Status messen; bei Abschluss die Note schließen.

### SSDC Limadou (CSES-L2)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur / Sotgiu-Antwort.
- **Lage:** (gemessen 2026-09-24 via `ledger.φ`) Operator-Wort **nein**
  (2026-09-23); `ledger.φ:14` „Permission Denied", Host 200.
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** wartend lassen.

#### Stufe 5 — termin

keiner. Der EMODNET-Termin (2026-10-19) lebt in `docs/zustand/external-state.md`.

#### Stufe 6 — LOCK

keiner.

## Katalog-Pool (Register, nicht handlungsfähig)

- `phi/pipeline/ledger.φ` 2 offen = src.pas TAP + SSDC Limadou (beide oben, wartend).
- `phi/pipeline/index.φ` 9 offen = Tür-Kataloge (Adapter-Route, keine Quelle);
  Zähllinie, kein eigener Schritt.
- `candidates` 1 = `pipeline/catalog/archeology_gaps_index.φ`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
