<!--
  title: Handover — Mycelium-Folge 158 (2026-09-25)
  session: Mycelium-Folge 158
  class: handover
  date: 2026-09-25
  sha256: bec72b472e05ae82f6ed52d1eacbb923d2e0733d1140168e52e3aabf9e15db15
  status: live
-->
# Handover — Mycelium-Folge 158 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Geteilter externer Zustand lebt in `docs/zustand/external-state.md`, nie als
Kopie hier. Keine Rangfolge, keine Geschichts-Abschnitte. Sortierung: erst Akteur
(**Linie** | **Rat** | **Operator** | **Dritter**), dann chronologisch nach
`Lage`-Datum. Jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** / **Blockade** /
**Braucht**. Status-Tag: `autonom` | `operator-gebunden` | `blockiert` | `wartend` |
`termin` | `LOCK`.

Diese Session konsumierte `handover-2026-09-25-mycelium-folge157.md`.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### eutils.ncbi.nlm.nih.gov — Oszillator-Gate + sources.φ
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via grind-flash,
  `phi/pipeline/stage/pre_cdn_host_verdict_2026-09-25.txt`) der einzige wirklich-neue
  Netz-Host der pre-cdn-unpooled-Menge; direct 200 (21 183 B), proton 200, wayback 200;
  `esearch.fcgi?db=bioproject` direct 200 (262 B); kein Register-Treffer für `eutils`.
- **Blockade:** keine.
- **Braucht:** Oszillator-Gate (Kraft-Kanal oder decline) + bei accept
  Eintrag in `phi/sources.φ` mit Harvest-Compiler.

#### pre-CDN-Asset-Hosts — Re-Ernte klären
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via grind-flash) ndbc/ncei/tidesandcurrents/open-meteo
  sind Selbstlink-Assets, deren Original-Netloc im aktiven Pool von `phi/sources.φ` liegt
  (kein neuer Host); 404-Re-Ernte gegen den aktiven Arm klären.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <origin>` des aktiven sources-Arms; bei Lücke
  `gh workflow run <netloc>-cdn.yml`.

#### Quellen-Routen aus den Future-Tauchern — Bau-Arme
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via research-max/grind-flash,
  `phi/pipeline/stage/future_routes_verdict_2026-09-25.txt`,
  `phi/pipeline/stage/d3_d8_route_verdict_2026-09-25.txt`) je Route fehlt ein
  Harvest-Bin oder eine Registrierung:
  - **D4 GAVO TAP-Async** — Bug+Fix gemessen (separater `POST <job>/phase` PHASE=RUN →
    EXECUTING → COMPLETED, `results/result` 200). Neuer Harvest-Bin
    `gavo_tap_async_harvester.rs` unter `tools/harvest/src/bin/`.
  - **D1 PDS-Rings** (`pds-rings.seti.org`, VG_2803 RSS) 200 anonym → neuer Arm
    `pds3_ring_occ_compiler.rs`.
  - **D1 NAIF SPK mariner10** (`M10_archive_1.bsp`) → `at mariner10`-Block registrieren
    (Anchor prüfen); **Voyager-Okkultation** → `voyager_occlt_compiler.rs`.
  - **D6 decaps_dr2.object** (`datalab.noirlab.edu/tap/sync`, anonym, 200) →
    `tap`-Block registrieren; **DECaPS2 Dataverse** 280 GB → Verdichtungs-Verdikt vor
    Registrierung.
  - **D8 TOAR** lebender Host `toar-data.fz-juelich.de/api/v2/` (IDs 1000–2000 anonym,
    sonst 401) → neuer `toar_timeseries_compiler.rs`; Konflikt zum bestehenden `decline`
    (redundant WOUDC) prüfen.
  - **D8 PANGAEA** 876108 = Link-Container (400) → Datenpfad 876110-Zip (88 MB anonym);
    `pangaea_harvester.rs` ZIP-Arm.
  - **D8 Zenodo** 21132339 (`data.zip` 23,5 GB) → `zenodo_record_harvester.rs` +
    Größen-Verdikt.
- **Blockade:** keine.
- **Braucht:** je Route den genannten Compiler bauen bzw. den Quellen-Block registrieren;
  D4-Client-Fix in `gavo_tap_async_harvester.rs`.

#### mycelium-ORPHANs (6)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** `tools-build` success am HEAD nach Push.
- **Lage:** (gemessen 2026-09-25 @`da1aafc8` via mountain `register_lookup --orphans`)
  6 mycelium-getaggte `ORPHAN_COMMITTED`-Einträge; das PATH-Bin kennt `--orphans` noch nicht.
- **Blockade:** Release-Artefakt hinkt HEAD.
- **Braucht:** nach `/commit` `register_lookup --orphans`, je Eintrag Träger/`gap`-Direktive.

#### Kandidat `pipeline/catalog/archeology_gaps_index.φ`
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via `register_lookup --open`) 1 Kandidat → mycelium.
- **Blockade:** keine.
- **Braucht:** Kandidat disponieren (accept → Katalog-Eintrag | release).

#### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check 36118259809` @HEAD `eae45358d`.
- **Lage:** (gemessen 2026-09-25 via `ci_manage list`) **pending**;
  `tools-build 36118021083` success.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36118259809`.

#### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ps1_dr2_coverage.fp01` erreicht (`all_present`).
- **Lage:** (gemessen 2026-09-25) `ps1-cdn` success; Final-Combine ungemessen
  (`footprints.φ:19` 404).
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** `archive_search --verdict <ps1_dr2_coverage.fp01-url>`; bei Vorhandensein
  Note finalisieren.

#### Fremdmodell-Benchmark (kein Browser-Target)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Browser-Target verbunden (`browser_targets` nicht leer).
- **Lage:** (gemessen 2026-09-25 via `browser_targets` = `[]`) Rekord
  `docs/surveys/survey-2026-09-24-fremdmodell-bedienung.md`.
- **Blockade:** kein Browser-Target.
- **Braucht:** Browser verbinden, dann Benchmark fahren.

### Operator

#### api.sensor.community — Operator-Exit-Wort
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum Exit/Route.
- **Lage:** (gemessen 2026-09-25 via grind-flash)
  `api.sensor.community/v1/data/measurements` direct 403 / proton 403 (ip-blocked);
  `proton-wg.sh suggest api.sensor.community`.
- **Blockade:** IP-Blockade; ein Exit-Wechsel berührt Terms/Anti-Circumvention.
- **Braucht:** Operator-Wort; danach `archive_search --verdict` erneut.
- **Vorbereitung (autonom, erledigt):** `bin/proton-wg.sh suggest api.sensor.community`.

### Dritter

#### DEMETER Order 18387 (WAF, nicht Workflow)
- **Status:** blockiert | **Bindung:** dritter
- **Trigger:** F5-ASM-WAF erholt ODER Order-Ablauf 2026-09-28.
- **Lage:** (gemessen 2026-09-24 via `demeter_harvest.rs`/`ci_manage log`)
  `rs-order`-Erzeugung scheitert an `F5 ASM: Request Rejected` → Exit 137.
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** Wiedervorlage; bei Erholung `gh workflow run demeter-cdn.yml`.

#### ESA LPF Legacy Archive AIO (D3)
- **Status:** blockiert | **Bindung:** dritter
- **Trigger:** ESA-Helpdesk-Antwort / Backend-Erholung.
- **Lage:** (gemessen 2026-09-25 via research-max/grind-flash) `lpfsa`-Backend
  `/lpfsa-sl/data-action` → HTTP 500/600 „Input hibernate session is null"; kein
  anonymer Datenfluss; `auth_method: cas`.
- **Blockade:** Backend-Session-Fehler (ESA).
- **Braucht:** Anfrage `support.cosmos.esa.int/lpfsa/` (Vorbereitung durch Future/Operator).

#### src.pas TAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** `/tap/tables` 200.
- **Lage:** (gemessen 2026-09-24 via `archive_search --verdict`) `ledger.φ:10`;
  `/tap/tables` 500 (`http://pithia.cbk.waw.pl/tap`).
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei Erholung.

#### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Backend-Erholung.
- **Lage:** (gemessen 2026-09-24) api 502 über 10 Proton-Exits, Frontend 200.
- **Blockade:** Broker-Backend.
- **Braucht:** Multi-Exit-Re-Messung (Wiedervorlage).

#### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** PSA-Freigabe.
- **Lage:** (gemessen 2026-09-24) `release_date 2099-01-01`, `data?PRODUCT` 403.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

#### SuperDARN MAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Globus-Task `af68c4f1`-Status / Task-Ende.
- **Lage:** (gemessen 2026-09-24) Zugang gewährt; MAP 6561 Dateien/21,93 GB;
  Task-Status ungemessen (kein CLI/Token am Host).
- **Blockade:** Globus-Task-Status.
- **Braucht:** Task-Status messen; bei Abschluss die Note schließen.

#### SSDC Limadou (CSES-L2)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur / Sotgiu-Antwort.
- **Lage:** (gemessen 2026-09-24) Operator-Wort **nein** (2026-09-23); `ledger.φ:14`
  „Permission Denied", Host 200.
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** wartend lassen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
