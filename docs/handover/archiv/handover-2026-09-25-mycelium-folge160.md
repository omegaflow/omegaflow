<!--
  title: Handover — Mycelium-Folge 160 (2026-09-25)
  session: Mycelium-Folge 160
  class: handover
  date: 2026-09-25
  sha256: 9ed7a71b9c76baebe0980645d0ff4d612714a48ad8669aae9f3f3b9ddea532d9
  status: live
-->
# Handover — Mycelium-Folge 160 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Geteilter externer Zustand lebt in `docs/zustand/external-state.md`, nie als
Kopie hier. Keine Rangfolge, kein „härtester Punkt". Sortierung: erst Akteur
(**Linie** | **Rat** | **Operator** | **Dritter**), dann chronologisch nach
`Lage`-Datum. Jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** / **Blockade** /
**Braucht**. Status-Tag: `autonom` | `operator-gebunden` | `blockiert` | `wartend` |
`termin` | `LOCK`.

Diese Session konsumierte `handover-2026-09-25-mycelium-folge159.md`.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### Register-Manifestation der gebauten Compiler (TOAR · Zenodo · DECaPS)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via grind-pro/grind-max) drei Compiler gebaut und
  `cargo check -p omegaflow-harvest` 0/0: `toar_timeseries_compiler.rs` (121 920
  Records/7 315 208 B, sha256 `8fd55224…`, geo.rs `MAGIC_TOAR/COMP_TOAR_O3`),
  `zenodo_record_harvester.rs` (record 21132339, environment.yml sha256
  `f3de3f99…`), `decaps_dr2_compiler.rs` (27 469 Records/1 538 280 B sha256
  `0967edeb…`). Die hinzuzufügenden Register-Blöcke sind gemessen, aber **nicht** eingefügt:
  `(ttl,url)`-Sortierung ist hartes CI-Gate, `register_sort`/`commit_check` lokal
  nicht auf PATH → Insertion unsicher (Insertionsstellen: TOAR ttl 604800,
  `toar-data.fz-juelich.de`; Zenodo ttl 86400 `zenodo.org/api/records/21132339/files/data.zip/content`).
- **Blockade:** kein lokaler Sort-Verifier.
- **Braucht:** die drei gemessenen Blöcke sort-korrekt einfügen und über den CI-Sort
  prüfen; DECaPS braucht zuvor einen Loader-Arm (`format decaps_dr2_stars` hat keinen
  `geo.rs`-Eintrag → ehrlich `pending`), Zenodo `data.zip`-Innenlayout ungemessen
  (`format reference`).

#### Voyager-Okkultation (D1) — neuer Archivar-Parser fehlt
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** NSSD1395-Zeit-Tag-Spec gefunden / Modul-Freigabe.
- **Lage:** (gemessen 2026-09-25 via grind-pro/`sfetch`) Verzeichnis 200 (1657 B);
  5 PSPA-Tars (DD059825–829, 13-NOV-80) mit `DD05982x_F{1,2}.DAT`. Frame gemessen:
  2-Byte-Big-Endian-Längenheader, Record 1 = 120-Byte-Metadatum (`SA01`/`VA634`),
  Records 2…N = 4096-Byte = 1024 × BE-float32. Kein bestehender Parser
  (`mariner_occlt` hard-codet `data_len 4106`/int8) passt.
- **Blockade:** Metadatum-Zeit-Tag braucht NSSD1395; neu zu bauen: Archivar-Modul
  voyager_occlt.rs + Magic im Format-Register, außerhalb des Agenten-Schreibsets.
- **Braucht:** NSSD1395-Zeit-Tag messen, das Modul + Magic/comp bauen,
  dann `voyager_occlt_compiler.rs`.

#### Port der `gap`-Quellen nach `phi/sources.φ` (183 offen)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch.
- **Lage:** (gemessen 2026-09-25 via grind-pro) live zählt `unit-auto-detect` ×168
  (working tree, +1 fremdes uncommitted), `force-undetermined` ×16, `konverter` ×4.
  Der `konverter`-Gap ist **ein Unit-Bug im `--port`-Konverter**: Gaia
  `phot_g_mean_mag` bekam `nT` statt `mag` (korrekt:
  `field phot_g_mean_mag gaia_dr3_g_mag inverse-square em mag 604800 0.0 0.0`);
  `bp_rp` hat keinen Parser-Arm → DROP. 4 `konverter`-Einträge gemessen portierbar
  (`blocked_sources.φ` gap-lines 237/257/272/357), URLs HTTP 200.
- **Blockade:** keine.
- **Braucht:** die 4 `konverter`-Blöcke nach `phi/sources.φ` portieren, `gap konverter`
  streichen; die `unit-auto-detect`-/`force-undetermined`-Klassen je Klassen-Träger
  `phi/blocked_sources.φ::gap:<token> ×N` weiterführen.

#### mycelium-ORPHANs (10)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `tools-build` success am HEAD.
- **Lage:** (gemessen 2026-09-25 @`93c4bb773` via `register_lookup --orphans`) 10
  mycelium-getaggte `ORPHAN_COMMITTED` in
  `phi/blocked_sources.φ:31/36/41/45/49/53/57/61/74/78` (superdarn.ca, psa.esa.int,
  4× nssdc, naif M10, spdf voyager, dataverse K88GFI, toar); das PATH-Bin kennt
  `--orphans` (Manifest-Sha `7ae650d5c`).
- **Blockade:** Release-Artefakt hinkt HEAD (93c4bb773).
- **Braucht:** je Eintrag Träger / `gap`-Direktive in dieses Register.

#### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` am HEAD `93c4bb773`.
- **Lage:** (gemessen 2026-09-25 via `ci_manage view`) der HEAD-Lauf `36132607670`
  ist **pending**; der jüngste abgeschlossene rote `36124590241` @`1bebe6dad` liegt
  **vor** Mountain `c4592e347` (in HEAD) — dessen Reds (`register_sort.rs` Format +
  `disposition_register_sorted_and_duplicate_detected`) sind dort geheilt, also
  fremd/überholt.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36132607670`; bei Rot `ci_manage log <id>`.

#### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ps1_dr2_coverage.fp01` erreicht (`all_present`).
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) Upload-URL
  `.github/workflows/ps1-cdn.yml:200` → `ssd.jpl.nasa.gov-ps1/ps1_dr2_coverage.fp01`
  direct 404 / proton 404 / Wayback kein Snapshot → **absent**.
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** Re-Messung bei Ernte-Fortschritt; dann Note finalisieren.

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
- **Lage:** (gemessen 2026-09-25) `regards.cnes.fr/api/v1/rs-order` 403 (Jetty Access
  Denied), POST 403; Lauf `35851193831` failure (8× `F5 ASM: Request Rejected`, Exit 137).
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** Wiedervorlage; bei Erholung `gh workflow run demeter-cdn.yml`.

#### ESA LPF Legacy Archive AIO (D3)
- **Status:** blockiert | **Bindung:** dritter
- **Trigger:** ESA-Helpdesk-Antwort / Backend-Erholung.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) `/lpfsa-sl/data-action`
  500 (87 B, „Malformed retrieval request"); Param-Route 502; Portal `/lpfsa/` 200;
  `auth_method: cas`.
- **Blockade:** Backend-Session-Fehler (ESA).
- **Braucht:** Anfrage `support.cosmos.esa.int/lpfsa/` (Vorbereitung durch Future/Operator).

#### src.pas TAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** `/tap/tables` 200.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) `/tap` 200 (13342 B);
  `/tap/tables` 500 (PostgreSQL localhost refused); `ledger.φ:10`.
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei Erholung.

#### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Backend-Erholung.
- **Lage:** (gemessen 2026-09-25) `api.lasair.lsst.ac.uk/api` direct keine Antwort,
  proton **HTTP 500**; Frontend 200; ZTF-Zwilling `lasair-ztf.lsst.ac.uk/api/objects`
  **401** (Token nicht angehängt).
- **Blockade:** Broker-Backend.
- **Braucht:** Token-Probe mit `LASAIR_LSST_TOKEN` am ZTF; Multi-Exit-Re-Messung.

#### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** PSA-Freigabe.
- **Lage:** (gemessen 2026-09-25) TAP 200; `release_date 2099-01-01`; Produkt 403.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

#### SuperDARN MAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Globus-Task `af68c4f1`-Ende.
- **Lage:** (gemessen 2026-09-25 via research-max) `superdarn.ca/data-download` 200;
  Globus-Task-API **HTTP 400** `ClientError.AuthenticationFailed`; Activity-SPA 200
  (Status hinter Login).
- **Blockade:** kein anonymer Statuskanal (OAuth2-Bearer nötig).
- **Braucht:** Task-Status via Globus-Transfer-Token/Kanal messen; bei Abschluss Note schließen.

#### SSDC Limadou (CSES-L2)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur / Sotgiu-Antwort.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) Portal 200; `query.php`
  anonym 302 → CAS-Login; „Permission Denied" hinter dem Credential-Login
  (operator-gebunden); Operator-Wort **nein** (2026-09-23).
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** wartend lassen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
