<!--
  title: Handover — Mycelium-Folge 157 (2026-09-25)
  session: Mycelium-Folge 157
  class: handover
  date: 2026-09-25
  sha256: f9564502483afd7895d36e29940148217fb991ea93a50d60b6e5ae85cca5a027
  status: live
-->
# Handover — Mycelium-Folge 157 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Geschichts-Abschnitte (Stehender-Pass-Ergebnis, geschlossen-Register,
Geteilter Baum): sie leben in git. Geteilter externer Zustand lebt in
`docs/zustand/external-state.md`, nie als Kopie hier. Keine Rangfolge — die offenen
Punkte werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** / **Blockade** /
**Braucht**. Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session hat `handover-2026-09-25-mycelium-folge156.md` konsumiert. Der
Archive-Move steht aus — folge156 trägt **fremde uncommittete** Hunks (siehe unten);
sie zu verschieben hieße, fremde Arbeit mitzuschleppen.

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

### `phi/declined_sources.φ` — verbleibende 2 Verdikt-Duplikate
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via `register_sort`) die Datei ist canonical (1288
  Blöcke, `ttl` asc / `url` asc, 0 exakte Duplikate); zwei urls tragen je zwei Blöcke
  mit **verschiedenem Verdikt/`note`**: `zenodo.org/records/8401262`
  (`decline analysis-dataset` vs `decline count`) und `zenodo.org/records/8427755`
  (`decline single-event` vs `decline event-record`). Die frühere Lage-Angabe
  (1283 Blöcke / 116 Inversionen) war falsch; die Messung ergab 1288 / 963.
- **Blockade:** keine.
- **Braucht:** je url **einen** Block wählen (das zutreffendere Verdikt), den anderen
  entfernen — Register-Edit, danach `register_sort` (exit 0).

### pre-cdn unpooled — Rest-Disposition
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via `grind-flash`,
  `phi/pipeline/stage/pre_cdn_unpooled_netloc_tafel.txt`) 4877 Blöcke; 2095 exakt im
  Pool (Duplikat → verwerfen), 1743 eigen-Katalog-Artefakte, **1002 Blöcke echter
  Hosts** ohne exakten Pool-Eintrag, **24 Blöcke / 11 externe GitHub-Repos** wirklich
  neu. Selbstlink-Assets (`ndbc.noaa.gov`, `open-meteo.com`) je 404 — Original-Netloc
  ist im Pool.
- **Blockade:** keine.
- **Braucht:** `--verdict`-Batch je pending-Host (`ndbc.noaa.gov`, `worldbank.org`,
  `open-meteo.com`, `tidesandcurrents.noaa.gov`, `gbif.org`, `ncei`, `eutils` …) +
  Register-Abgleich (`phi/sources.φ` / `phi/pipeline/ledger.φ`); die 11 Repo-Tags durch das
  Oszillator-Gate und disponieren.

### Quellen-Routen aus den Future-Tauchern
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via Future-Taucher, volle Kaskade; committet
  `d34b5347e`) fünf Datenzugänge galten als „gated/wartend", sind aber offen bzw.
  eigen-verschuldet: **D1** PDS-Rings RSS-raw, NAIF SPK (Voyager/Mariner 10),
  Atmo-Okkultations-Doppler (Voyager-ODR liegt bereits in `phi/sources.φ`); **D3**
  ESA LPF Legacy Archive AIO (`Delta-g-x-L1/L2-*`) + MUST-Telemetrie, anonym; **D4**
  GAVO TAP-Async — own-side Client-Fehler (DaCHS ignoriert `PHASE=RUN`; separater
  `POST /tap/async/<job>/phase` nötig); **D6** Astro Data Lab `ls_dr10.tractor` +
  `decaps_dr2.object` über `datalab.noirlab.edu/tap/sync`, anonym, DECaPS2 Dataverse
  `10.7910/DVN/K88GFI`; **D8** TOAR Ozon-API offen, Deposit PANGAEA `10.1594/PANGAEA.876108`
  + Zenodo `10.5281/zenodo.21132339`.
- **Blockade:** keine.
- **Braucht:** je Route Harvest + Quellen-Registrierung in `phi/sources.φ`
  (CDN-Manifestation über CI); D4 zusätzlich der Async-Client-Fix.

### parser-def-Gap-Audit — Owner-Entscheid
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zur Owner-Zuordnung.
- **Lage:** (gemessen 2026-09-25 via `grind-max`) `phi/blocked_sources.φ` trägt **189**
  `parser-def`-Blöcke. Primärklassen (disjunkt, Präzedenz V>M>N>F>U): „unit absent"
  **152**, „force undetermined" **21**, `mag_g`→nT **4**, `parser-def votable` **4**,
  Multi-Token **1**; **7** außerhalb (1 `parser-def html` AEC-FDSN + 6
  unit/cadence-Varianten). 14 Blöcke tragen zwei Signaturen. Register-Tag:
  `[mountain] parser-def`; die Mountain-Tafel nannte sie `linie:mycelium`. Die
  Klassen-Labels `Force-undetermined` / `Multi-Token` (mit Bindestrich) existieren
  nirgends im Tree.
- **Blockade:** widersprüchliche Owner-Zuordnung — Register (`parser-def` → mountain)
  gegen Mountain-Tafel (`linie:mycelium`).
- **Braucht:** Operator-Wort. **mountain** → per direktem Edit in Mountains Übergabe
  routen (halten-vor-reichen). **mycelium** → in folge158 als Stufe-1-Punkte
  übernehmen: generische Unit-Tabelle (`src/archivar` `convert_to_si`), 9-Kraft-
  Zuordnung (Astrometrie/Farbindex), VOTable-Reader-Arm, `mag_g`-Konverter-Fix.

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
- **Trigger:** `ci-check`-Lauf am HEAD `61e272ab0`.
- **Lage:** (gemessen 2026-09-25 via `ci_manage view`) `ci-check 36114466560`
  @`61e272ab0` **pending**; `tools-build 36113209201` success. `phi/sources.φ`
  canonical (`register_sort` 0 Verstöße).
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36114466560`.

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
- **Lage:** (gemessen 2026-09-24 via `blocked_sources.φ`/`archive_search`) api 502
  über 10 Proton-Exits, Frontend 200.
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

## Fremd uncommittet (gemessen 2026-09-25, `git status --short`)

Während dieser Session war eine **parallele Mycelium-Session** im geteilten Baum
aktiv: sie committete `d34b5347e` (Future-Routen in folge156). Eine Delegation dieser
Session berührte die folgenden Pfade **nicht**:

- `tools/register/src/bin/register_lookup.rs` (`M`; OrphanCandidate-Feature, +189 Z.)
  — bleibt unangetastet und uncommittet; der Commit gehört der verursachenden Session.

`docs/handover/archiv/handover-2026-09-25-mycelium-folge156.md` ist committet
(`d34b5347e`, clean) und von dieser Session archiviert; die gerouteten Routen sind
oben nach folge157 übernommen.

## Benchmark

parser-def-Gap-Audit (`blocked_sources.φ`, 189 Blöcke, read-only) `grind-flash` gegen
`grind-max`: flash zählte 158 K1 (Überlapp nicht abgezogen, Klassen nicht disjunkt)
und verpasste die 7 Außenblöcke; max lieferte disjunkte Primärklassen (V>M>N>F>U),
legte die 14 Doppelsignaturen offen und fand die Außenblöcke inkl. `parser-def html`.
**Sieger: `grind-max`.**

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
