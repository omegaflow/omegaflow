<!--
  title: Handover — Mycelium-Folge 159 (2026-09-25)
  session: Mycelium-Folge 159
  class: handover
  date: 2026-09-25
  sha256: 93eb4ab0b41269f616cef7329b889027950618a12eba15fad622fd35438d7e00
  status: live
-->
# Handover — Mycelium-Folge 159 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Geteilter externer Zustand lebt in `docs/zustand/external-state.md`, nie als
Kopie hier. Keine Rangfolge, kein „härtester Punkt". Sortierung: erst Akteur
(**Linie** | **Rat** | **Operator** | **Dritter**), dann chronologisch nach
`Lage`-Datum. Jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** / **Blockade** /
**Braucht**. Status-Tag: `autonom` | `operator-gebunden` | `blockiert` | `wartend` |
`termin` | `LOCK`.

Diese Session konsumierte `handover-2026-09-25-mycelium-folge158.md`.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### pre-CDN-Asset-Hosts — NDBC-Stationslücke
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) ndbc/ncei/
  tidesandcurrents/open-meteo sind Selbstlink-Assets, deren Original-Netloc im aktiven
  Pool von `phi/sources.φ` liegt; die 404 pre-CDN-Assets lösen sich gegen den aktiven
  Arm. Offen bleibt `www.ndbc.noaa.gov/data/realtime2/` (200, 547334 B, 946 Dateien):
  die Stationen `42003`/`42019`/`42020` sind im Listing **404**; kein `ndbc-cdn.yml`
  in `gh workflow list --all` (286 Workflows) → die Quelle ist live-`url` ohne CDN-Asset.
- **Blockade:** keine.
- **Braucht:** Stations-IDs in `phi/sources.φ:383–440` gegen das live Listing erneuern.

#### Quellen-Routen der Future-Taucher — Rest-Arme
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via grind-max/grind-pro) D4 GAVO TAP-Async und D1
  PDS-Rings sind in diesem Atom gebaut **und gemessen lauffähig**: GAVO
  (`RESPONSEFORMAT=votable/td` + BINARY-`<STREAM>`-Arm) liefert td- und
  BINARY-Pfad byte-identisch, Exit 0; PDS3 (Spalten-Sammlung gefixt) schreibt
  375 050 Samples / 9 001 312 B, Roundtrip hält, Exit 0. Offen:
  - **D8 TOAR** — Verdikt **accept** (diffusion, Oberflächen-O3 nmol/mol = ppb,
    CC BY 4.0; nicht redundant zu WOUDC, das die Gesamtsäule trägt):
    `toar_timeseries_compiler.rs` fehlt (`phi/blocked_sources.φ:71`).
  - **D6 DECaPS** — `tap`-Block `decaps_dr2.object` registriert (`phi/sources.φ:9478`);
    Dataverse `doi:10.7910/DVN/K88GFI` ~280 GB pending (FITS-GZ-Parser fehlt,
    `phi/blocked_sources.φ:74`); CDN-Manifestation offen.
  - **D1 NAIF mariner10** — `M10_archive_1.bsp` 200 (51200 B); Anchor `at mariner10`
    fehlt (kein `ephemeris_mariner10.bin`, kein `frame_registry.φ`-Eintrag) →
    `ephemeris_compiler.rs` + Anchor (`phi/blocked_sources.φ:49`).
  - **D1 Voyager-Okkultation** — PSPA-Katalog 200; RSS-Payload ungemessen →
    `voyager_occlt_compiler.rs` (`phi/blocked_sources.φ:53`).
  - **D8 PANGAEA** 876110-Zip (347 455 312 B) ZIP-Arm + **D8 Zenodo** 21132339
    (`data.zip` 23 506 041 274 B) `zenodo_record_harvester.rs` pending.
- **Blockade:** keine.
- **Braucht:** die genannten Compiler/Anchors bauen; die zwei gebauten Arme nach Push
  über `gh workflow run` manifestieren.

#### mycelium-ORPHANs (6)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `tools-build` success am HEAD nach dem Push `e3b8e598e`.
- **Lage:** (gemessen 2026-09-25 @`da1aafc8` via mountain `register_lookup --orphans`)
  6 mycelium-getaggte `ORPHAN_COMMITTED` in `phi/blocked_sources.φ:31/36/41/45/49/53`
  (superdarn.ca, psa.esa.int, 4× nssdc); das PATH-Bin kennt `--orphans` noch nicht.
- **Blockade:** Release-Artefakt hinkt HEAD.
- **Braucht:** nach `tools-build` success `register_lookup --orphans`, je Eintrag
  Träger/`gap`-Direktive.

#### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` am HEAD `e3b8e598e`.
- **Lage:** (gemessen 2026-09-25 via `ci_manage view 36116592391`) der Rot war
  `omegaflow-utils` `discovery::tests::fold_replaces_placeholder_with_wildcard`
  (`host*y*` erwartet, Rinde liefert `host/*y*`); in diesem Atom korrigiert. Der neue
  Lauf am Push `e3b8e598e` ist ungemessen (pending).
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage list` / `ci_manage view <run-id>` des HEAD-Laufs; bei erneutem
  Rot `ci_manage log <id>`.

#### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ps1_dr2_coverage.fp01` erreicht (`all_present`).
- **Lage:** (gemessen 2026-09-25) `ps1-cdn` success; Final-Combine ungemessen
  (`phi/footprints.φ:19` 404).
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

#### Port der 187 `gap`-Quellen nach `phi/sources.φ` (von Mountain getragen)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via `sgrep` über `phi/blocked_sources.φ`) die 5
  Parser-Arme stehen; die `gap`-Direktiven zählen `unit-auto-detect` ×167,
  `force-undetermined` ×16, `konverter` ×4 = 187. `votable-reader` (ALMA) entfiel mit
  dem `decline redistribution`-Verdikt (2026-09-25); AEC trägt kein `gap` mehr
  (`parser-def json`, JSON-API). Unit-Feld-Verdikte: 27 Felder
  (`src/archivar/port.rs`/`units.rs`), 9 DROP (Metadatum/String-Enum), 2 außerhalb der
  ~30 (`declination_deg`, `sz_mass_10e14_msun`, Quelle noch nicht portiert).
- **Blockade:** keine.
- **Braucht:** Port je Klassen-Träger `phi/blocked_sources.φ::gap:<token> ×N` nach
  `phi/sources.φ` + CDN; die `gap`-Direktive fällt erst mit dem Port.

### Operator

#### api.sensor.community — Operator-Exit-Wort
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum Exit/Route.
- **Lage:** (gemessen 2026-09-25 via grind-flash)
  `api.sensor.community/v1/data/measurements` direct 403 / proton 403 (ip-blocked).
- **Blockade:** IP-Blockade; ein Exit-Wechsel berührt Terms/§ 95a UrhG.
- **Braucht:** Operator-Wort; danach `archive_search --verdict` erneut.
- **Vorbereitung (autonom, erledigt):** `bin/proton-wg.sh suggest api.sensor.community`.

### Dritter

#### DEMETER Order 18387 (WAF, nicht Workflow)
- **Status:** blockiert | **Bindung:** dritter
- **Trigger:** F5-ASM-WAF erholt ODER Order-Ablauf 2026-09-28.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`/`ci_manage log`)
  `regards.cnes.fr/api/v1/rs-order` 403 (358 B, Jetty Access Denied), `user/orders/18387`
  403, POST 403; Lauf `35851193831` failure (8× `F5 ASM: Request Rejected`, Exit 137);
  Order-Ablauf 2026-09-28 liegt in der Zukunft.
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** Wiedervorlage; bei Erholung `gh workflow run demeter-cdn.yml`.

#### ESA LPF Legacy Archive AIO (D3)
- **Status:** blockiert | **Bindung:** dritter
- **Trigger:** ESA-Helpdesk-Antwort / Backend-Erholung.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) `/lpfsa-sl/data-action`
  500 (87 B, „Malformed retrieval request: none data identifiers"); Param-Route
  `?ProductType=…&data=1` 502 Proxy Error; Portal `/lpfsa/` 200; `auth_method: cas`.
- **Blockade:** Backend-Session-Fehler (ESA).
- **Braucht:** Anfrage `support.cosmos.esa.int/lpfsa/` (Vorbereitung durch Future/Operator).

#### src.pas TAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** `/tap/tables` 200.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) `/tap` 200 (13342 B);
  `/tap/tables` 500 (1683 B, `connection to "localhost" (127.0.0.1), port 5432 failed:
  Connection refused`); `ledger.φ:10`.
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei Erholung.

#### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Backend-Erholung.
- **Lage:** (gemessen 2026-09-25 via research-max/`archive_search --verdict`)
  `api.lasair.lsst.ac.uk/api` direct keine Antwort, proton **HTTP 500** (Register trug
  502 vom 2026-09-20); Frontend `lasair.lsst.ac.uk` 200; ZTF-Zwilling
  `lasair-ztf.lsst.ac.uk/api/objects` **HTTP 401** mit JSON
  `Authentication credentials were not provided` (Token nicht angehängt).
- **Blockade:** Broker-Backend.
- **Braucht:** Token-Probe mit vorhandenem `LASAIR_LSST_TOKEN` am ZTF (401 *mit* Token
  qualifiziert Key/Scope); Multi-Exit-Re-Messung der API.

#### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** PSA-Freigabe.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) TAP 200 (1562 B);
  `release_date 2099-01-01T00:00:00.0`; `data?PRODUCT` 403 (1121 B, PSA
  `DataRetrieval forbidden`).
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

#### SuperDARN MAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Globus-Task `af68c4f1`-Ende.
- **Lage:** (gemessen 2026-09-25 via research-max) `superdarn.ca/data-download` 200;
  `transfer.api.globusonline.org/v0.10/task/af68c4f1` **HTTP 400** JSON
  `ClientError.AuthenticationFailed` (kein Transfer-Token/CLI am Host; `.secrets.local`
  trägt `GLOBUS_ID_USER`/`GLOBUS_ID_PASS`); `app.globus.org/activity/af68c4f1` 200
  (SPA-Shell, Status hinter Login).
- **Blockade:** kein anonymer/hostbarer Statuskanal (OAuth2-Bearer nötig).
- **Braucht:** Task-Status über einen Globus-Transfer-Token/Kanal messen; bei Abschluss
  die Note schließen.

#### SSDC Limadou (CSES-L2)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur / Sotgiu-Antwort.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) Portal 200 (60804 B);
  `query.php` anonym 302 → `tools.ssdc.asi.it/cas/login`; „Permission Denied" liegt
  hinter dem Credential-Login (operator-gebunden). Operator-Wort **nein** (2026-09-23);
  `ledger.φ:14`.
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** wartend lassen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
