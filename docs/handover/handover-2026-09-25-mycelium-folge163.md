<!--
  title: Handover — Mycelium-Folge 163 (2026-09-25)
  session: Mycelium-Folge 163
  class: handover
  date: 2026-09-25
  sha256: 1970686a23bdd16b23420219fa2b403556f3c7ed90e0a125ac13ffc862eae6bb
  status: live
-->
# Handover — Mycelium-Folge 163 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge. Sortierung: erst Akteur (**Linie** | **Rat** | **Operator** |
**Dritter**), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
**Trigger** / **Lage** / **Blockade** / **Braucht**. Status-Tag: `autonom` |
`operator-gebunden` | `blockiert` | `wartend` | `termin` | `LOCK`.

Diese Session konsumierte `handover-2026-09-25-mycelium-folge162.md`.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### DECaPS — Per-Stern-Epoch statt `catalog_epoch`
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via grind-pro) Ein uniformer `catalog_epoch` existiert
  nicht: DECaPS2 `decaps_dr2.object` führt `epochmean`/`epochrange` **pro Objekt**
  (TAP, HTTP 200); §6.1 des DECaPS2-Papiers (`10.3847/1538-4365/aca594`) nennt
  heterogene CP-Astrometrie (2MASS/Gaia DR1/eDR3, 2016-03–2019-05). `decaps.rs`
  trägt kein Zeit-Tag (56-B-Stride ra/dec f64 + 10×f32) → der Arm bleibt korrekt bei
  0 Kanälen; ein Einzelwert wäre Fabrication.
- **Blockade:** keine.
- **Braucht:** Per-Stern-Epoch in `src/archivar/decaps.rs` (neuer Stride/Slot aus
  `epochmean`/`epochrange`) + `series_parse_bin`-Arm bauen; alternativ `descoped` mit
  dem Befund. Format der Serien-Arme: `extract.rs`/`voyager_occlt.rs` als Muster.

#### Voyager-Okkultation — Decode offen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via grind-max) `src/archivar/voyager_occlt.rs` +
  `tools/harvest/src/bin/voyager_occlt_compiler.rs` gebaut (Magic `VOCC`, Mediumband
  5056-B-Record, T0 Wörter 1–4, TIMETAGDAYS Wort 5; Narrowband-Kopf 15 complex words);
  `cargo check` 0/0. Offen: Mediumband-Datenwert-Stride (4704 B / 512 = 9,1875, Riss),
  Wort-Byte-Order (T0-D / TIMETAGDAYS-I), Narrowband-Recordlänge (Wort 33 > 15 complex
  words = Riss). Tests frieren die Risse ein.
- **Blockade:** keine.
- **Braucht:** Byte-Order + Datenwert-Stride aus einem gemessenen Beleg lösen; dann
  `parse_series` dekodieren und `phi/sources.φ`-Serien-Arm setzen.

#### gap-Orphan-Einzelträger (15) — Aufenthalt
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via grind-flash) Die Klassen-Träger
  (`unit-auto-detect ×168`, `force-undetermined ×16`, `curation ×17`) stehen bereits
  in `handover-2026-09-25-mountain-folge163.md` (Owner mountain, Counts live);
  `konverter`/`astrometry-reader`/`votable-reader`/`html-parser-arm` ×0 (inert).
  Die 15 `pending`-Orphans ohne `gap` sind mycelium; folge162-Einzelträger:
  `blocked_sources.φ::superdarn.ca/data-download` (Globus `af68c4f1`-Status),
  `::psa.esa.int/psa-tap/tap/` (`psahelp`-Antwort),
  `::nssdc…PSNO-00007` (SDDPT, Voyager Doppler),
  `::nssdc…PSCM-00009` (Mariner-10),
  `::naif…M10_archive_1.bsp` (RISS: note „asset fehlt" vs. State `pending`),
  `::spdf…saturn_occultation_narrow_band/` (RSS-Payload),
  `::nssdc…PSPG-00011` (Viking-Orbiter),
  `::nssdc…PSPA-00605` (Juno-pre-EFB),
  `::dataverse…DVN/K88GFI` (FITS-GZ-Parser-Arm DECaPS2),
  `::toar-data.fz-juelich.de/api/v2/data/timeseries_merged/` (`toar_timeseries_compiler`),
  `::eeadmz1…/ParquetFile/urls` (POST `post_body`),
  `::gavo.aip.de/tap/sync…ravedr4` (HTTP-500-Retry),
  `::padc-tap-rcsed.obspm.fr…rcsed_fibermags` (HTTP-500-Retry),
  `::voparis-tap-astro-m.obspm.fr…hyperleda.galaxies` (HTTP-500-Retry),
  `::skvo.science.upjs.sk/tap/sync…ogle.lightcurves` (HTTP-500-Retry).
- **Blockade:** keine.
- **Braucht:** je Eintrag der obige Schritt; Rissen-State (`naif`/`spdf`) als
  Owner-Korrektur an mountain.

#### health-check `probe-full` — toter Artefaktpfad
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via grind-pro) `probe-full` globt
  `phi/pipeline/stage/*_converted.φ` → 0 Dateien (`stage/` ruhend ausgelagert
  2026-09-17, `SOURCE_PORT.md:24`); `upload-artifact` findet `probe-results/` nicht,
  Job meldet trotzdem grün.
- **Blockade:** keine.
- **Braucht:** Job auf einen lebenden Stage-Pfad zeigen oder mit dem Befund
  `descoped` setzen (`health-check.yml`).

#### OMNI-HAPI `OMNI_HRO_1MIN` registrieren
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via River folge29) CDAWeb-HAPI live,
  `https://cdaweb.gsfc.nasa.gov/hapi/data?id=OMNI_HRO_1MIN&format=csv`; in
  `phi/sources.φ` nicht registriert, kein Compiler (Minuten-Archiv).
- **Blockade:** keine.
- **Braucht:** nach `docs/SOURCE_PORT.md` in `phi/sources.φ` registrieren + Compiler
  bauen.

#### SuperDARN-Mirror rot
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25, Operator-Weiterleitung an River) der SuperDARN-Mirror
  ist rot; welcher Mirror-Workflow/Lauf, ist ungelesen.
- **Blockade:** keine.
- **Braucht:** erster Messschritt `ci_manage list` auf den Mirror-Workflow, dann
  `ci_manage log <id>`; Befund als Register-Zeile.

#### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` am HEAD.
- **Lage:** (gemessen 2026-09-25 via `ci_manage list`) `ci-check 36169873867` pending;
  die Reds `clippy`/`test` sind in `2fc953b4a` committed geheilt; Folgeruns durch
  Concurrency gecancelt.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>`; bei Rot `ci_manage log <id>`.

#### Manifestations-Hashes (TOAR · Zenodo · DECaPS)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Manifest-Lauf (kernel-flatten / CDN).
- **Lage:** (gemessen 2026-09-25 via grind-max) Zenodo `data.zip` 23 506 041 274 B ohne
  Stream-sha256 (nur md5 `86b37400…`); TOAR sha256 `8fd55224…` nur 5-Serien-Sample;
  DECaPS hängt am Per-Stern-Epoch-Punkt.
- **Blockade:** Manifest-Lauf.
- **Braucht:** CI-Full-Range-Manifest trägt die Hashes nach; DECaPS nach der
  Epoch-Entscheidung.

#### PS1 final-combine — 5xx-Retry
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ps1_dr2_coverage.fp01` erreicht / ps1-cdn-Lauf.
- **Lage:** (gemessen 2026-09-25 im Browser) `ps1-cdn` Lauf `36067129156`, Job
  `ps1-shard (2)` rot: `HTTP 500` beim Asset-Combine; kein Logikfehler.
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** Retry mit Backoff auf 5xx beim Asset-Combine.

#### api.sensor.community — CI-Reachability (kein Exit)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `source-census`-Lauf.
- **Lage:** (gemessen 2026-09-25 via grind-flash) direct 403 / Proton 403 (ip-blocked);
  `phi/blocked_sources.φ` um `blocked ip-blocked` ergänzt; `source-census.yml` um
  `--blocked` (proton-freier `source_latency_census`-Puls) erweitert. Beides im
  Atom, `gh workflow run` evaluiert den Remote-Ref → Dispatch erst nach Push.
- **Blockade:** Commit+Push.
- **Braucht:** nach `/commit` `gh workflow run source-census.yml`; Ergebnis via
  `ci_manage view <id>` / `phi/reports/source_latency_census.φ`.

### Operator

#### Fremdmodell-Benchmark — per-Akt-Consent
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (per Akt) + definierte Benchmark-Aufgabe.
- **Lage:** (gemessen 2026-09-25) Chrome-Target verbunden; Rekord
  `docs/surveys/survey-2026-09-24-fremdmodell-bedienung.md`; das Survey selbst
  klassifiziert jeden Prompt als Schreib-Akt bei Dritten (z.ai/claude.ai) →
  per-Akt-Consent.
- **Blockade:** fehlender per-Akt-Consent + kein definierter Benchmark-Auftrag.
- **Braucht:** Operator-Wort auf den präsentierten Akt; danach Prompt/Modell messen,
  Ergebnis als Survey-Zeile.

### Dritter

#### src.pas TAP
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02 (Re-Messung).
- **Lage:** (gemessen 2026-09-25 via research-max) `/tap` 200; `/tap/tables` 500 (PostgreSQL refused).
- **Blockade:** Pithia-Backend.
- **Braucht:** `archive_search --verdict http://pithia.cbk.waw.pl/tap/tables`.

#### BepiColombo PSA
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02.
- **Lage:** (gemessen 2026-09-25) TAP 200; Produkt 403; `release_date 2099-01-01`.
- **Blockade:** ESA-Freigabe.
- **Braucht:** `archive_search --verdict https://psa.esa.int/psa-tap/tap/`.

#### SuperDARN MAP
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02.
- **Lage:** (gemessen 2026-09-25) `data-download` 200; Globus-Task `af68c4f1` ohne anonymen Statuskanal.
- **Blockade:** kein anonymer Statuskanal.
- **Braucht:** `archive_search --verdict https://superdarn.ca/data-download`; Task via Globus-Token.

#### ESA LPF Legacy (D3)
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02.
- **Lage:** (gemessen 2026-09-25) `/lpfsa-sl/data-action` direct 500 / proton 500.
- **Blockade:** ESA-Backend.
- **Braucht:** `archive_search --verdict https://lpf.esac.esa.int/lpfsa-sl/data-action`.

#### DEMETER Order 18387
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-09-28 (Order-Ablauf).
- **Lage:** (gemessen 2026-09-25) `regards.cnes.fr/api/v1/rs-order` direct 403 / proton 403 (F5-ASM-WAF).
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** `archive_search --verdict https://regards.cnes.fr/api/v1/rs-order`; bei Erholung `gh workflow run demeter-cdn.yml`.

#### Lasair-LSST
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02.
- **Lage:** (gemessen 2026-09-25) `api.lasair.lsst.ac.uk/api/` keine Antwort / proton 404; ZTF-Twin 401.
- **Blockade:** Broker-Backend.
- **Braucht:** `archive_search --verdict https://api.lasair.lsst.ac.uk/api/`.

#### SSDC Limadou (CSES-L2)
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02.
- **Lage:** (gemessen 2026-09-25) Portal direct 200 / proton 200; `query.php` CAS-gated.
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** `archive_search --verdict https://limadou.ssdc.asi.it/`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
