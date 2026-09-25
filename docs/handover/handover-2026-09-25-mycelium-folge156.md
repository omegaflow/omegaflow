<!--
  title: Handover — Mycelium-Folge 156 (2026-09-25)
  session: Mycelium-Folge 156
  class: handover
  date: 2026-09-25
  sha256: c2191e9cb64b4f3bb95e15a7eb4d8d9569d338fd6728c92b26bf0ce73056c136
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

### Quellen-Routen aus den Future-Tauchern (gemessen 2026-09-25)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via Future-Taucher, volle Kaskade) fünf Datenzugänge, die als „gated/wartend" galten, sind offen bzw. eigen-verschuldet:
  - **D1 Sonden-Derivate:** Voyager-ODR liegt bereits in `phi/sources.φ:8857–8961` (Heimat `pds-ppi.igpp.ucla.edu`); PDS-Rings RSS-raw, NAIF SPK (Voyager/Mariner 10), Atmo-Okkultations-Doppler offen; die NSSDC-Tapes bleiben request-only.
  - **D3 LPF Δg:** ESA LPF Legacy Archive AIO (`metadata-action`/`data-action`, `ANALYSIS_OBJECT` QUANTITY `Delta-g-x-L1/L2-*`) + MUST-Telemetrie, anonym; Format LTPDA/MATLAB.
  - **D4 GAVO TAP-Async:** own-side Client-Fehler — DaCHS ignoriert `PHASE=RUN` bei Job-Anlage, nötig ist separater `POST /tap/async/<job>/phase`; kein Konto.
  - **D6 Astro Data Lab:** `ls_dr10.tractor` + `decaps_dr2.object` über `datalab.noirlab.edu/tap/sync` anonym; LS DR10 NERSC-FITS offen, DECaPS2 Dataverse `10.7910/DVN/K88GFI`.
  - **D8 TOAR Ozon:** API offen (`toar-data.fz-juelich.de/api/v2/data/timeseries/<id>?format=csv`, kein Login); TOAR-Deposit auf PANGAEA `10.1594/PANGAEA.876108`, Zenodo TOAR Phase II `10.5281/zenodo.21132339`; WOUDC/EBAS/NOAA GML/SHADOZ als Archive.
- **Blockade:** keine.
- **Braucht:** je Route Harvest + Quellen-Registrierung in `phi/sources.φ` (CDN-Manifestation über CI); D4 zusätzlich der Async-Client-Fix.

### Benchmark D2/D8 (flash vs pro, gemessen 2026-09-25)
- **D2 BiSON-Tabelle:** flash `$0.078` (input 125,6 k) vs pro `$0.051` (input 44,4 k) — **Sieger flash** (vollständig: edata1573 + Roh-Reihe 1976–2025; pro blieb bei edata1569/1572), bei 1,5× Kosten.
- **D8 TOAR:** flash `$0.016` vs pro `$0.013` — **Sieger pro** (entscheidender Fund: PANGAEA-TOAR-Deposit + Zenodo), und günstiger.
- **Lehre:** die Siegerklasse ist aufgabenabhängig — Referenz-Vollständigkeit → flash, Deposit-Suche → pro; keine pauschale Tier-Regel.

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
