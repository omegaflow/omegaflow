<!--
  title: Handover — Mycelium-Folge 159 (2026-09-25)
  session: Mycelium-Folge 159
  class: handover
  date: 2026-09-25
  sha256: 2ad27c501d6bbaac23a67aa2f852187838ff38988192ee5fd9a79f20e4318168
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
- **Lage:** (gemessen 2026-09-25 via grind-flash/research-max,
  `phi/pipeline/stage/future_routes_verdict_2026-09-25.txt`,
  `d3_d8_route_verdict_2026-09-25.txt`) D4 GAVO TAP-Async und D1 PDS-Rings sind in
  diesem Atom gebaut; offen:
  - **D1 NAIF SPK mariner10** (`M10_archive_1.bsp`) → `at mariner10`-Block registrieren
    (Anchor prüfen); **Voyager-Okkultation** → `voyager_occlt_compiler.rs`.
  - **D6 decaps_dr2.object** (`datalab.noirlab.edu/tap/sync`, anonym 200) → `tap`-Block
    registrieren; **DECaPS2** Dataverse 280 GB → Größen-Verdikt vor Registrierung.
  - **D8 TOAR** lebender Host `toar-data.fz-juelich.de/api/v2/` (IDs 1000–2000 anonym,
    sonst 401) → `toar_timeseries_compiler.rs`; Konflikt zum bestehenden `decline`
    (redundant WOUDC) prüfen.
  - **D8 PANGAEA** 876108 = Link-Container; Datenpfad 876110-Zip (gemessen 2026-09-25
    via HEAD `content-length` = 347 455 312 B, ~347 MB — **nicht** 88 MB der folge158-Zeile);
    `pangaea_harvester.rs` ZIP-Arm.
  - **D8 Zenodo** 21132339 (`.../files/data.zip/content` 200 anonym, `content-length`
    23 506 041 274 B = 23,5 GB) → `zenodo_record_harvester.rs` + Größen-Verdikt.
- **Blockade:** keine.
- **Braucht:** je Route den genannten Compiler bauen/den Block registrieren; die zwei
  gebauten Arme nach Push über `gh workflow run` manifestieren.

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
- **Lage:** (gemessen 2026-09-24 via `demeter_harvest.rs`/`ci_manage log`)
  `rs-order`-Erzeugung scheitert an `F5 ASM: Request Rejected` → Exit 137.
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** Wiedervorlage; bei Erholung `gh workflow run demeter-cdn.yml`.

#### ESA LPF Legacy Archive AIO (D3)
- **Status:** blockiert | **Bindung:** dritter
- **Trigger:** ESA-Helpdesk-Antwort / Backend-Erholung.
- **Lage:** (gemessen 2026-09-25 via research-max) `lpfsa`-Backend
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
- **Lage:** (gemessen 2026-09-24) `release_date 2099-01-01`, `data?PRODUCT` 403.
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
- **Lage:** (gemessen 2026-09-24) Operator-Wort **nein** (2026-09-23); `ledger.φ:14`
  „Permission Denied", Host 200.
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** wartend lassen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
