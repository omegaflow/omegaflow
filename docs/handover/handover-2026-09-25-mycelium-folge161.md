<!--
  title: Handover — Mycelium-Folge 161 (2026-09-25)
  session: Mycelium-Folge 161
  class: handover
  date: 2026-09-25
  sha256: 468355e8ceb9ff1b019c4bc6495f0cc5dbcb3b241fe565c6ff39cddb94476ce1
  status: live
-->
# Handover — Mycelium-Folge 161 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Geteilter externer Zustand lebt in `docs/zustand/external-state.md`, nie als
Kopie hier. Keine Rangfolge, kein „härtester Punkt". Sortierung: erst Akteur
(**Linie** | **Rat** | **Operator** | **Dritter**), dann chronologisch nach
`Lage`-Datum. Jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** / **Blockade** /
**Braucht**. Status-Tag: `autonom` | `operator-gebunden` | `blockiert` | `wartend` |
`termin` | `LOCK`. Eine Zeile ohne externen Trigger ist ungültig — `nächster
Dispatch`/`nächste Session` ist kein Trigger (der lesende Lauf IST der nächste);
Regel in AGENTS.md, Gate-Fixture `commit_gate_vocab.json::deferral_markers`.

Diese Session konsumierte `handover-2026-09-25-mycelium-folge160.md`.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### DECaPS-Loader-Arm
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via grind-max) `phi/sources.φ` trägt
  `decaps_dr2_stars.bin` (format decaps_dr2_stars, ttl 31536000, origin Dataverse
  K88GFI); `src/archivar/geo.rs` `magic_of`/`comp_max` hat keinen Arm, und ein
  GeoRec-Arm wäre falsch: der Record ist 56 B (ra/dec f64 + 10×f32; 16-B-Header
  `0xCF860500`+count+rec_bytes+reserved), ohne Zeitachse/`t` — `parse_bin`
  (60-B-GeoRec) fehlparst.
- **Blockade:** keine.
- **Braucht:** `src/archivar/decaps.rs` (gaia_sso-Muster) + Dispatch in
  `main_flow`/`extract.rs` + Magic-Registrierung; dann gemessen manifestieren.

#### TOAR-Komponenten-Arm
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via grind-max) `geo.rs` `MAGIC_TOAR`/`COMP_TOAR_O3`
  stehen; `src/archivar/extract.rs::geo_series_component_name` hat keinen
  `"toar_surface_o3"`-Arm → `field toar_surface_o3_ppb` matcht keinen Kanal; der
  Block lädt, manifestiert aber nichts.
- **Blockade:** keine.
- **Braucht:** 3-Zeilen-Arm `"toar_surface_o3"` in `extract.rs`.

#### Manifestations-Hashes (TOAR · Zenodo · DECaPS)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Manifest-Lauf (kernel-flatten / CDN).
- **Lage:** (gemessen 2026-09-25 via grind-max) Zenodo `data.zip` 23 506 041 274 B —
  kein Stream-sha256 (Record trägt nur md5 `86b37400…`), Block ohne sha256-Zeile;
  TOAR sha256 `8fd55224…` nur 5-Serien-Sample (IDs 1000–2000 ersetzt); DECaPS
  Partial-Hash bewusst nicht eingetragen.
- **Blockade:** Manifest-Lauf.
- **Braucht:** CI-Full-Range-Manifest trägt die Hashes nach (Register-Duty bei der
  Manifestation).

#### gap-Klassen-Träger fortschreiben
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via sgrep) `gap konverter 4→0` geschlossen: 4
  Gaia-Blöcke portiert (`field phot_g_mean_mag gaia_dr3_g_mag inverse-square em mag
  604800 0.0 0.0`); RR Lyrae + cluster_ka als `gap curation` zurück (HTTP 400,
  Spalte fehlt); live: `unit-auto-detect ×168`, `force-undetermined ×16`,
  `curation ×17`; gap-Token-Kanon (Header-Notizen) steht.
- **Blockade:** keine.
- **Braucht:** die `unit-auto-detect`-/`force-undetermined`-Klassen je Klassen-Träger
  `phi/blocked_sources.φ::gap:<token> ×N` weiterführen (Port-Kandidaten einzeln
  messen).

#### Voyager-Okkultation (D1)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (Spec gemessen).
- **Lage:** (gemessen 2026-09-25 via research-max) Kollektion PSPA-00217;
  `FORMAT_IDENTIFIER NSSD1395`, `MACHINE_REPRESENTATION Data General Eclipse`,
  `RECORD_FORMAT variable`, `STREAM_RECORD_DELIMITER 2-BYTE HEADER`,
  `MAXIMUM_RECORD_LENGTH_BYTES 4098`; Frame 122 B + 3908×4098 B = 16 015 106 B je
  `.DAT`; 120-B-Metadatum Byte-Map (F1/F2) gemessen; Zeit-Tag-Slot („time of first
  sample…") ohne Offset.
- **Blockade:** Zeit-Tag-Encoding ohne Spec-Text (DSC_0629-Scan als PDF, Text
  ungemessen).
- **Braucht:** Vision-OCR der DSC_0629-Seiten ODER NSSDC-CRUSO-Anfrage; dann
  `src/archivar/voyager_occlt.rs` + Magic + `voyager_occlt_compiler.rs`.

#### mycelium-ORPHANs (10)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `tools-build` Lauf `36161207629` success (dispatcht 2026-09-25).
- **Lage:** (gemessen 2026-09-25 @`93c4bb773` via `register_lookup --orphans`) 10
  ORPHAN_COMMITTED in `phi/blocked_sources.φ` (Zeilen driften:
  31/36/41/45/49/53/57/61/74/78); Release-Artefakt hinkt HEAD.
- **Blockade:** Lauf-Ergebnis.
- **Braucht:** bei success `register_lookup --orphans`; je Eintrag Träger /
  `gap`-Direktive.

#### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` am HEAD.
- **Lage:** (gemessen 2026-09-25) der HEAD-Lauf `36132607670` ist pending; der
  jüngste abgeschlossene rote `36124590241` @`1bebe6dad` liegt **vor** Mountain
  `c4592e347` — überholt.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>`; bei Rot `ci_manage log <id>`.

#### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ps1_dr2_coverage.fp01` erreicht.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) Upload-URL
  `.github/workflows/ps1-cdn.yml:200` direct 404 / proton 404 / Wayback kein
  Snapshot → absent.
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** Re-Messung bei Ernte-Fortschritt; dann Note finalisieren.

#### Fremdmodell-Benchmark
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Browser-Target verbunden (`browser_targets` nicht leer).
- **Lage:** (gemessen 2026-09-25 via `browser_targets` = `[]`) Rekord
  `docs/surveys/survey-2026-09-24-fremdmodell-bedienung.md`.
- **Blockade:** kein Browser-Target.
- **Braucht:** Browser verbinden, dann Benchmark fahren.

**Geschlossene Benchmark-Klasse (Operator-Wort 2026-09-25):** Register-/Konverter-Port
— `grind-flash` gegen `grind-max`: der max-Arm registrierte 4 Gaia-Quellen live, die
flash-Messung fand 2 davon als HTTP 400 (Spalte fehlt); Sieger `grind-flash` (billiger,
fand den Defekt). Die 2 sind als `gap curation` zurückgeführt.

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
  Denied), POST 403; Lauf `35851193831` failure (8× `F5 ASM: Request Rejected`).
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** Wiedervorlage; bei Erholung `gh workflow run demeter-cdn.yml`.

#### ESA LPF Legacy Archive AIO (D3)
- **Status:** blockiert | **Bindung:** dritter
- **Trigger:** ESA-Helpdesk-Antwort / Backend-Erholung.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) `/lpfsa-sl/data-action`
  500 (87 B, „Malformed retrieval request"); Param-Route 502; Portal `/lpfsa/` 200;
  `auth_method: cas`.
- **Blockade:** Backend-Session-Fehler (ESA).
- **Braucht:** Anfrage `support.cosmos.esa.int/lpfsa/` (Vorbereitung durch
  Future/Operator).

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
- **Braucht:** Task-Status via Globus-Transfer-Token/Kanal messen; bei Abschluss Note
  schließen.

#### SSDC Limadou (CSES-L2)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur / Sotgiu-Antwort.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) Portal 200;
  `query.php` anonym 302 → CAS-Login; Operator-Wort **nein** (2026-09-23).
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** wartend lassen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
