<!--
  title: Handover — Mycelium-Folge 162 (2026-09-25)
  session: Mycelium-Folge 162
  class: handover
  date: 2026-09-25
  sha256: 82357904446fc43ff902011bfb7c6882cafd3821515ed5530a8f691a93cb120d
  status: live
-->
# Handover — Mycelium-Folge 162 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Geteilter externer Zustand lebt in `docs/zustand/external-state.md`, nie als
Kopie hier. Keine Rangfolge, kein „härtester Punkt". Sortierung: erst Akteur
(**Linie** | **Rat** | **Operator** | **Dritter**), dann chronologisch nach
`Lage`-Datum. Jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** / **Blockade** /
**Braucht**. Status-Tag: `autonom` | `operator-gebunden` | `blockiert` | `wartend` |
`termin` | `LOCK`. Eine Zeile ohne externen Trigger ist ungültig —
`nächster Dispatch`/`nächste Session` ist kein Trigger (der lesende Lauf IST der
nächste); Regel in AGENTS.md, Gate-Fixture `commit_gate_vocab.json::deferral_markers`.

Diese Session konsumierte `handover-2026-09-25-mycelium-folge161.md`.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### DECaPS-Feld-Gate — catalog_epoch
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via grind-flash/grind-pro) `src/archivar/decaps.rs`
  (Magic `0xCF860500`, 56-B-Stride ra/dec f64 + 10×f32, kein Zeit-Tag) + `main_flow`-Route
  + `geo.rs`-Magic stehen; `phi/sources.φ` trägt 7 Feld-Arme
  (parallax arcsec gravity; r/i/z/extinction/g mag em; rv km/s advective);
  `catalog_epoch` fehlt → der Arm emittiert 0 Kanäle; `dist_pc`/`logt_yr`/`mini`
  ausgelassen (kein `pc`/Alter-Unit bzw. keine Physik). `cargo check` 0/0; der Parser
  gegen ein reales Asset ist ungeprüft.
- **Blockade:** keine.
- **Braucht:** DECaPS-DR2-Referenz-Epoch messen (Dataverse K88GFI) und die
  `catalog_epoch`-Direktive in `phi/sources.φ` setzen; dann Manifestation messen.

#### gap-Klassen-Träger + Orphan-Aufenthalte
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via grind-flash/`register_lookup --orphans`) 15
  mycelium-`ORPHAN_COMMITTED` in `phi/blocked_sources.φ`, **alle `pending` ohne
  `gap`** → keine Klasse, nur Einzel-Träger (unten). Gap-Token-Kanon in den
  `note gap-Token`-Zeilen 3–8 (`unit-auto-detect`, `force-undetermined`, `curation`,
  `konverter`, `astrometry-reader`, `votable-reader`, `html-parser-arm`). Klassen-Count
  live gemessen als **Vorkommen** `sgrep -c`: unit-auto-detect 330, force-undetermined
  18, curation 19 (Vorkommen ≠ Einträge — opener und note doppeln); letzte
  Eintragsmessung folge161: ×168 / ×16 / ×17.
- **Blockade:** keine.
- **Braucht:** Klassen-Einträge live zählen (parser-def-Block je Token) und
  Klassen-Träger `phi/blocked_sources.φ::gap:<token> ×N` setzen. **Orphan-Einzelträger (15):**
  - `phi/blocked_sources.φ::https://superdarn.ca/data-download` — Globus-Task `af68c4f1`-Status messen (kein anonymer Kanal)
  - `phi/blocked_sources.φ::https://psa.esa.int/psa-tap/tap/` — `psahelp`-Antwort (BepiColombo release 2099)
  - `phi/blocked_sources.φ::https://nssdc.gsfc.nasa.gov/nmc/dataset/display.action?id=PSNO-00007` — SDDPT-Anfrage-Antwort (Voyager Doppler)
  - `phi/blocked_sources.φ::https://nssdc.gsfc.nasa.gov/nmc/dataset/display.action?id=PSCM-00009` — Mariner-10-Anfrage-Antwort
  - `phi/blocked_sources.φ::https://naif.jpl.nasa.gov/pub/naif/M10/kernels/spk/M10_archive_1.bsp` — `ephemeris_mariner10.bin` + frame_registry (asset fehlt)
  - `phi/blocked_sources.φ::https://spdf.gsfc.nasa.gov/pub/data/voyager/voyager1/radio_science_rss/saturn_occultation_narrow_band/` — RSS-Payload messen, dann voyager_occlt_compiler
  - `phi/blocked_sources.φ::https://nssdc.gsfc.nasa.gov/nmc/dataset/display.action?id=PSPG-00011` — Viking-Orbiter-Anfrage-Antwort
  - `phi/blocked_sources.φ::https://nssdc.gsfc.nasa.gov/nmc/dataset/display.action?id=PSPA-00605` — Juno-pre-EFB-Anfrage-Antwort
  - `phi/blocked_sources.φ::https://dataverse.harvard.edu/dataset.xhtml?persistentId=doi:10.7910/DVN/K88GFI` — FITS-GZ-Parser-Arm DECaPS2
  - `phi/blocked_sources.φ::https://toar-data.fz-juelich.de/api/v2/data/timeseries_merged/` — `toar_timeseries_compiler.rs` bauen
  - `phi/blocked_sources.φ::https://eeadmz1-downloads-api-appservice.azurewebsites.net/ParquetFile/urls` — POST-Pfad (`post_body`) statt GET
  - `phi/blocked_sources.φ::http://gavo.aip.de/tap/sync?...ravedr4...` — HTTP-500-Retry
  - `phi/blocked_sources.φ::http://padc-tap-rcsed.obspm.fr/tap/sync?...rcsed_fibermags...` — HTTP-500-Retry
  - `phi/blocked_sources.φ::http://voparis-tap-astro-m.obspm.fr/tap/sync?...hyperleda.galaxies...` — HTTP-500-Retry
  - `phi/blocked_sources.φ::https://skvo.science.upjs.sk/tap/sync?...ogle.lightcurves...` — HTTP-500-Retry
- **Randnotiz (Owner-Riss):** die Einträge `naif.jpl.nasa.gov/...M10...` (Z. 56) und
  `spdf.../saturn_occultation_narrow_band/` (Z. 60) tragen in der `note` „asset fehlt",
  stehen aber als `pending` (kein `asset`-State) → der Scanner liest sie als
  mycelium-pending; die State-Korrektur ist ein Owner-Akt (mountain), nicht gesetzt.

#### Voyager-Okkultation (D1)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (Spec jetzt gemessen).
- **Lage:** (gemessen 2026-09-25 via research-max + vision) die DSC_0629-Doku liegt unter
  `https://spdf.gsfc.nasa.gov/pub/documents/old/documentation_from_nssdc/old_data_doc/all_catalogs/dsc_0629.pdf`
  (503 865 B, PDF-Scan ohne Text-Layer, sha256 `d41d11fa449aedfbb1dc203692521e67ac120a6effc7bb7d921074d7f956d6e4`,
  Wayback `20170226131533`); OCR (19 S.): **Mediumband** Kopf = 88 complex words
  (dann 512 Datenwerte), Recordlänge 2528 int-Wörter, word 1–4 `T0` (D, „time of first
  sample in record, µs from start of day"), word 5 `TIMETAGDAYS` (I, Tag-Nummer);
  **Narrowband** Kopf = 15 complex words, word 29–33 HOUR/MINUTE/SECOND/DAY
  („time of first sample in file"). Der Scan trägt **nicht** die 122-B-Frame-,
  4098- oder 2-Byte-Delimiter-Konstanten aus der folge161-Notiz.
- **Blockade:** keine.
- **Braucht:** `src/archivar/voyager_occlt.rs` + Magic + `voyager_occlt_compiler.rs`
  aus dem gemessenen Byte-Map bauen.

#### Manifestations-Hashes (TOAR · Zenodo · DECaPS)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Manifest-Lauf (kernel-flatten / CDN).
- **Lage:** (gemessen 2026-09-25 via grind-max) Zenodo `data.zip` 23 506 041 274 B — kein
  Stream-sha256 (Record nur md5 `86b37400…`); TOAR sha256 `8fd55224…` nur 5-Serien-Sample;
  DECaPS Partial-Hash bewusst nicht eingetragen. Zusätzlich (neu): DECaPS-Arm emittiert
  0 Kanäle ohne `catalog_epoch` (siehe oben).
- **Blockade:** Manifest-Lauf.
- **Braucht:** CI-Full-Range-Manifest trägt die Hashes nach; DECaPS erst nach `catalog_epoch`.

#### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` am HEAD.
- **Lage:** (gemessen 2026-09-25 via `ci_manage list` + GitHub-UI) HEAD `1cfe54e76` (river
  folge29); `ci-check 36168588465` @HEAD **pending**; `ci-check 36163930996` (mountain
  folge163 `2fc953b4a`) in_progress; die Reds `clippy`/`test` sind in `2fc953b4a`
  committed geheilt. Die jüngsten abgeschlossenen ci-check-Läufe sind durch
  Concurrency (`cancel-in-progress: false`) gecancelt, nicht rot.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>`; bei Rot `ci_manage log <id>`.

#### PS1 final-combine — 5xx-Retry
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ps1_dr2_coverage.fp01` erreicht / ps1-cdn-Lauf.
- **Lage:** (gemessen 2026-09-25 im Browser) `ps1-cdn` Lauf `36067129156`, Job
  `ps1-shard (2)` rot: nach `band combine 1150 on ps1-dr2-1120` →
  `HTTP 500 (api.github.com/repos/omegaflow/sources/releases/assets/586988453)` → exit 1;
  kein Logikfehler im Shard, ein transienter API-500.
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** Retry mit Backoff auf 5xx beim Asset-Combine; Re-Messung bei Ernte-Fortschritt.

#### health-check Laufzeit/Concurrency
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 im Browser) `health-check #119` `36140394227` rot — `verify (4)`
  lief 1 h 58 min in `cargo run -- --verify phi --shard 4/8`, dann `Error: The operation
  was canceled.` (kein Panic; Log endet sauber nach ETOPO1/sciencebase `reference ok`, dazwischen
  `ndbc.noaa.gov/.../realtime2/*.txt JSON parse void`); Matrix kaskadiert. `health-check.yml`:
  8 Shards, `timeout-minutes: 240`, Schedule `0 */3 * * *`, `concurrency: health-check`
  `cancel-in-progress: false` → Stau (`36132614446` lief > 5 h, `36160983844` wartet);
  `probe-full` hinterließ kein `probe-results/`.
- **Blockade:** keine.
- **Braucht:** Shard-Laufzeit senken (mehr Shards / NDBC-Voids überspringen / Cache) und
  Timeout/Schedule prüfen; `probe-full`-Artefaktpfad prüfen.

#### Fremdmodell-Benchmark
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** gefeuert — Browser-Target verbunden (`browser_targets` nicht leer: chrome).
- **Lage:** (gemessen 2026-09-25) Rekord `docs/surveys/survey-2026-09-24-fremdmodell-bedienung.md`.
- **Blockade:** keine.
- **Braucht:** Benchmark gegen die verbundene Chrome-Session fahren; Ergebnis als Survey-Zeile.

### Operator

#### api.sensor.community — Operator-Exit-Wort
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum Exit/Route.
- **Lage:** (gemessen 2026-09-25 via grind-flash) `api.sensor.community/v1/data/measurements`
  direct 403 / proton 403 (ip-blocked).
- **Blockade:** IP-Blockade; ein Exit-Wechsel berührt Terms/§ 95a UrhG.
- **Braucht:** Operator-Wort; danach `archive_search --verdict` erneut.
- **Vorbereitung (autonom, erledigt):** `bin/proton-wg.sh suggest api.sensor.community`.

### Dritter

Geteilter Zustand → `docs/zustand/external-state.md`; alle Endpunkte am 2026-09-25
frisch gemessen, Re-Messung als `termin` gesetzt (die Session misst selbst — auf eine
Report-Antwort eines Dritten wird nicht gewartet).

#### src.pas TAP
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02 (Wochen-Re-Messung).
- **Lage:** (gemessen 2026-09-25 via research-max) `/tap` 200 (13342 B); `/tap/tables` 500 (PostgreSQL refused).
- **Blockade:** Pithia-Backend.
- **Braucht:** `archive_search --verdict http://pithia.cbk.waw.pl/tap/tables`.

#### BepiColombo PSA
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02 (Re-Messung).
- **Lage:** (gemessen 2026-09-25 via research-max) TAP 200; Produkt 403; `release_date 2099-01-01`.
- **Blockade:** ESA-Freigabe.
- **Braucht:** `archive_search --verdict https://psa.esa.int/psa-tap/tap/`.

#### SuperDARN MAP
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02 (Re-Messung).
- **Lage:** (gemessen 2026-09-25 via research-max) `data-download` 200 (39090 B); Globus-Task `af68c4f1` ohne anonymen Statuskanal (OAuth2 nötig).
- **Blockade:** kein anonymer Statuskanal.
- **Braucht:** `archive_search --verdict https://superdarn.ca/data-download`; Task via Globus-Transfer-Token.

#### ESA LPF Legacy (D3)
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02 (Re-Messung).
- **Lage:** (gemessen 2026-09-25 via research-max) `/lpfsa-sl/data-action` direct 500 / proton 500.
- **Blockade:** ESA-Backend.
- **Braucht:** `archive_search --verdict https://lpf.esac.esa.int/lpfsa-sl/data-action`.

#### DEMETER Order 18387
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-09-28 (Order-Ablauf).
- **Lage:** (gemessen 2026-09-25 via research-max) `regards.cnes.fr/api/v1/rs-order` direct 403 / proton 403 (F5-ASM-WAF).
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** `archive_search --verdict https://regards.cnes.fr/api/v1/rs-order`; bei Erholung `gh workflow run demeter-cdn.yml`.

#### Lasair-LSST
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02 (Re-Messung).
- **Lage:** (gemessen 2026-09-25 via research-max) `api.lasair.lsst.ac.uk/api/` direct keine Antwort / proton 404; ZTF-Twin `lasair-ztf.lsst.ac.uk/api/objects` 401.
- **Blockade:** Broker-Backend.
- **Braucht:** `archive_search --verdict https://api.lasair.lsst.ac.uk/api/`.

#### SSDC Limadou (CSES-L2)
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02 (Re-Messung).
- **Lage:** (gemessen 2026-09-25 via research-max) Portal direct 200 / proton 200; `query.php` CAS-gated.
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** `archive_search --verdict https://limadou.ssdc.asi.it/`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
