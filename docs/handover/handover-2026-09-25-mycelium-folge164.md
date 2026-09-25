<!--
  title: Handover — Mycelium-Folge 164 (2026-09-25)
  session: Mycelium-Folge 164
  class: handover
  date: 2026-09-25
  sha256: d61387b67ddae5aab801ddb9e589ae47849da3514c337345bd4f39dbf7278c26
  status: live
-->
# Handover — Mycelium-Folge 164 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge. Sortierung: erst Akteur (**Linie** | **Rat** | **Operator** |
**Dritter**), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
**Trigger** / **Lage** / **Blockade** / **Braucht**. Status-Tag: `autonom` |
`operator-gebunden` | `blockiert` | `wartend` | `termin` | `LOCK`.

Diese Session konsumierte `handover-2026-09-25-mycelium-folge163.md`.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### DECaPS TAP-object Epoch-Quelle
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via grind-pro) Der FITS-Binary-Weg kann die
  Per-Stern-Epoch nicht tragen: `decaps_stars_batch_*.fits.gz` (24 Spalten) hat keine
  `epochmean`/`epochrange`, die TAP-`decaps_dr2.object` hat kein `decaps_id`/`gaia_id`
  (TAP ERROR „Column does not exist") → kein Join-Key. `decaps.rs` bleibt korrekt ohne
  Zeit-Slot. Die Epoch liegt pro Objekt in `decaps_dr2.object` (`epochmean` MJD,
  `epochrange` Tage, HTTP 200).
- **Blockade:** keine.
- **Braucht:** die TAP-Quelle `phi/sources.φ:9506` um `epochmean,epochrange` erweitern
  (`format tap`, `epoch_key epochmean`, MJD→TDB) — oder `descoped` mit dem Befund.

#### Voyager-Okkultation — T0-Epoch-Anker + Narrowband-Rate
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via grind-max) Decode aus dem PSPA-Payload selbst
  gelöst (2-B-BE-Längenpräfix; Mediumband 704-B-Kopf + 512 BE-f32-(I,Q)-Paare,
  Narrowband 120-B-Kopf + 1024 Samples; alles Big-Endian, Univac 1108).
  `src/archivar/voyager_occlt.rs` + `voyager_occlt_compiler.rs` +
  `voyager-occlt-cdn.yml` + der Block in `phi/sources.φ` stehen, `cargo check` 0/0. Offen:
  absoluter UTC-Anker des T0-ms-Zählers; Narrowband-Abtastrate (Reihen bleiben 0).
- **Blockade:** keine.
- **Braucht:** OP2S-Formatfiguren (`VG2-S-RSS-1-ROCC-V1.0/DOCUMENT/OP2SF*.PS`,
  gescannt) oder Stanford-Cn-Doku via `archive_search --ntrs`/`--playwright`; dann
  Anker + Rate setzen.

#### gap-Orphans — Rest
- **Status:** wartend / blockiert (per Eintrag) | **Bindung:** eigen / dritter
- **Trigger:** je Eintrag (HTTP-Retry / Anfrage-Antwort / Manifest).
- **Lage:** (gemessen 2026-09-25)
  - 4× HTTP-500 persistiert (direct+socks5h, Wayback CDX ohne Snapshot):
    `gavo.aip.de…ravedr4`, `padc-tap-rcsed…rcsed_fibermags`,
    `voparis-tap-astro-m…hyperleda.galaxies`, `skvo.science.upjs.sk…ogle.lightcurves`.
  - `naif…M10_archive_1.bsp`: asset fehlt (ephemeris_mariner10.bin + frame_registry).
  - `nssdc…PSNO-00007` (SDDPT), `PSCM-00009` (Mariner-10), `PSPG-00011` (Viking),
    `PSPA-00605` (Juno pre-EFB): Anfrage-Antwort offen.
  - `dataverse…K88GFI`: FITS-GZ-Parser steht; Größen-Verdikt (280 GB) bleibt pending.
- **Blockade:** Server / Anfrage / Größe.
- **Braucht:** je Eintrag der Register-Schritt (`archive_search --verdict <url>` /
  Anfrage / CDN-Manifest).

#### OMNI-HRO / Voyager CDN-Manifest
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gh workflow run omni-hro-cdn.yml` / `voyager-occlt-cdn.yml` nach Push.
- **Lage:** (gemessen 2026-09-25) Compiler + Workflows + die Blöcke in `phi/sources.φ`
  stehen (`omni_hro` Zeile 746, `voyager_occlt` nach dem mariner-Block); Assets noch
  nicht auf dem CDN, sha256 fehlt.
- **Blockade:** Commit+Push.
- **Braucht:** nach `/commit` die zwei Workflows dispatchen; `ci_manage view <id>`;
  sha256 in die Blöcke in `phi/sources.φ` nachtragen.

#### api.sensor.community — CI-Reachability (kein Exit)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `source-census`-Lauf.
- **Lage:** (gemessen 2026-09-25) direct 403 / Proton 403 (ip-blocked);
  `phi/blocked_sources.φ` um `blocked ip-blocked` ergänzt; `source-census.yml` um
  `--blocked` (proton-freier `source_latency_census`-Puls) erweitert. Push steht aus.
- **Blockade:** Commit+Push.
- **Braucht:** nach `/commit` `gh workflow run source-census.yml`; Ergebnis via
  `ci_manage view <id>` / den Report `source_latency_census.φ`.

#### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` am HEAD.
- **Lage:** (gemessen 2026-09-25 via `ci_manage list`) `ci-check 36173241029` @HEAD
  pending; `source-census 36172999495` in_progress; die Reds `clippy`/`test` sind in
  `2fc953b4a` geheilt.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36173241029`; bei Rot `ci_manage log <id>`.

#### Manifestations-Hashes (TOAR · Zenodo)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Manifest-Lauf (kernel-flatten / CDN).
- **Lage:** (gemessen 2026-09-25 via grind-max) Zenodo `data.zip` 23 506 041 274 B ohne
  Stream-sha256 (nur md5 `86b37400…`); TOAR sha256 `8fd55224…` nur 5-Serien-Sample.
- **Blockade:** Manifest-Lauf.
- **Braucht:** CI-Full-Range-Manifest trägt die Hashes nach.

#### PS1 final-combine — 5xx-Retry
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ps1_dr2_coverage.fp01` erreicht / ps1-cdn-Lauf.
- **Lage:** (gemessen 2026-09-25) `ps1-cdn 36174412773` in_progress; früherer Lauf
  `36067129156` Job `ps1-shard (2)` rot: `HTTP 500` beim Asset-Combine, kein Logikfehler.
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** Retry mit Backoff auf 5xx beim Asset-Combine.

### Operator

#### Fremdmodell-Benchmark — per-Akt-Consent
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (per Akt) + definierte Benchmark-Aufgabe.
- **Lage:** (gemessen 2026-09-25) Chrome-Target verbunden; Rekord
  `docs/surveys/survey-2026-09-24-fremdmodell-bedienung.md`; jeder Prompt ist ein
  Schreib-Akt bei Dritten (z.ai/claude.ai) → per-Akt-Consent.
- **Blockade:** fehlender per-Akt-Consent + kein definierter Benchmark-Auftrag.
- **Braucht:** Operator-Wort auf den präsentierten Akt.

### Dritter

#### src.pas TAP
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02 (Re-Messung).
- **Lage:** (gemessen 2026-09-25) `/tap` 200; `/tap/tables` 500 (PostgreSQL refused).
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
- **Lage:** (gemessen 2026-09-25) `data-download` 200; Globus-Task `af68c4f1` ohne
  anonymen Statuskanal.
- **Blockade:** kein anonymer Statuskanal.
- **Braucht:** `archive_search --verdict https://superdarn.ca/data-download`.

#### ESA LPF Legacy (D3)
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02.
- **Lage:** (gemessen 2026-09-25) `/lpfsa-sl/data-action` direct 500 / proton 500.
- **Blockade:** ESA-Backend.
- **Braucht:** `archive_search --verdict https://lpf.esac.esa.int/lpfsa-sl/data-action`.

#### DEMETER Order 18387
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-09-28 (Order-Ablauf).
- **Lage:** (gemessen 2026-09-25) `regards.cnes.fr/api/v1/rs-order` direct 403 / proton
  403 (F5-ASM-WAF).
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** `archive_search --verdict https://regards.cnes.fr/api/v1/rs-order`; bei
  Erholung `gh workflow run demeter-cdn.yml`.

#### Lasair-LSST
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02.
- **Lage:** (gemessen 2026-09-25) `api.lasair.lsst.ac.uk/api/` keine Antwort / proton
  404; ZTF-Twin 401.
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
