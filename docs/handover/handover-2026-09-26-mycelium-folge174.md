<!--
  title: Handover — Mycelium-Folge 174 (2026-09-26)
  session: Mycelium-Folge 174
  class: handover
  date: 2026-09-26
  sha256: 1be549c81271334fb68b3a1e524cdc5c5167d4950dee8c1bb35688620d832281
  status: live
-->
# Handover — Mycelium-Folge 174 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `operator-gebunden` |
`blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-26-mycelium-folge173.md`.

## Operator-Wort-Register

- Wort | 2026-09-26 | „all" — session-weiter Consent (`mycelium_go`), Delegation an alle Taucher.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### GOSAT-GW GWT3F_L1B — format-Arm (GeoRec re-pack)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** `src/archivar/main_flow.rs` frei von fremden Hunks.
- **Lage:** (gemessen 2026-09-26 via grind-pro + Rat) Compiler `gosat_tanso3` packt 6×f64 (48 B) `[t,lat,lon,val,band,product]`, GeoRec erwartet 60 B `[t,lat,lon,alt,freq,bin_width,val,comp]`; Rat: Option 1 re-pack, `comp = product·3 + (band−1)` ∈ 1..6, `(freq,bin_width)=(0,0)` (Band nie in Hz); radiance-`units`-Attribut ungelesen.
- **Blockade:** `main_flow.rs` trägt einen fremden, uncommitteten fmt-Hunk (catalog_charm2, `:4013-4040`) — ein Commit würde ihn mitschleppen.
- **Braucht:** Re-pack `pack()` (`tools/harvest/src/bin/gosat_tanso3_compiler.rs:711`) auf GeoRec; `MAGIC_GOSAT`+`COMP_GOSAT_*` (`geo.rs`), `magic_of`/`comp_max` (`:205`/`:248`), `geo_series_component_name` (`extract.rs:450`), `"gosat_tanso3"` in `main_flow.rs:3238`; radiance-`units` messen (Accessor wie Compiler `:541`); dann `blocked_sources.φ:323-325` → released.

#### modis-cdn — Hosted-Runner-Shutdown
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `modis-cdn`-Lauf grün (geshardet).
- **Lage:** (gemessen 2026-09-26 via grind-flash/`ci_manage log 36267998951`) MYD11C3/MOD11C2 nutzt **nur** coder 4 (DEFLATE) über alle 17 SDS — kein NBIT-Arm nötig (Handover-Prämisse widerlegt); Abbruch `The runner has received a shutdown signal` mitten im Harvest (17 M Records je Granule × Serie).
- **Blockade:** Lauf-Zeitbudget des Hosted-Runners.
- **Braucht:** `modis-cdn.yml` sharden (Jahr/Produkt-Split), dann `gh workflow run modis-cdn.yml`.

#### Pre-CDN params — Keyer-Re-Run über korrigierte Queue
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `source_keyer`-Re-Run grün (keine Riss-Wiederholung).
- **Lage:** (gemessen 2026-09-26 via grind-flash) MCQG-Riss = Koordinaten-Kollision: MCG-Block (`queue/sources_potential_pre-cdn_9k_richest.φ:6030`) trug MCQ-Koordinate −54.5/158.95; SuperMAG MCG = 72.599998/−38.350006 (`phi/supermag_stations.φ:329`), MCQ = −54.5/158.949997 (`:332`); Korrektur gesetzt.
- **Blockade:** keine.
- **Braucht:** `cargo run -p omegaflow-utils --bin source_keyer` über die korrigierte Queue; MCQG-Direktive bestätigen.

#### arvo-registry.sci.am — erholt (http)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `archive_search --verdict http://arvo-registry.sci.am/tap` 200 (2026-09-26).
- **Lage:** (gemessen 2026-09-26 via grind-flash `--verdict`) `http://arvo-registry.sci.am/tap` stage 1 **200 (9 337 B)** (recovered); `https://` tot; 8 übrige Hosts der Relevanz-Liste weiter tot (5×404, 1×503, dns.wh.gov no-response).
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict http://arvo-registry.sci.am/tap/tables`; bei Samples Quelle registrieren.

#### kernel-flatten de441-cdn — Asset-Größe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `de441-cdn`-Lauf (≥183 MB `ephemeris_{sun,earth}.bin`).
- **Lage:** (gemessen 2026-09-26 via `ci_manage`) `kernel-flatten` success, `spk_split` lief; `de441-cdn` wartet auf ≥183 MB Bins.
- **Blockade:** Asset-Größe.
- **Braucht:** `ci_manage list` filter de441-cdn; Bins prüfen.

#### IRIS/EarthScope EMC netCDF-4 — volume-Extract
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Architektur-Wort `volume`-Extract.
- **Lage:** (gemessen 2026-09-26 via grind-pro) `src/archivar/hdf5.rs` liest netCDF-4; Gap `volume`-Extract (`types.rs:136`) + Kanalarm; Lizenz keine, EarthScope-ToS (Attribution + Non-human-Visitor).
- **Blockade:** Architektur-Akt (neue `volume`-Direktive).
- **Braucht:** Rat/Operator-Wort `volume` + `parse.rs`/`channels.rs`/`main_flow.rs`.

#### Voyager 1/2 closed-loop Doppler — request-only
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** NSSDC-Antwort (Anfrage `:207`).
- **Lage:** (gemessen 2026-09-26 via `archive_search`) PDS V2 nur open-loop `.ODR`; `radio_science_rss=2` dirs kein Cruise ODF/ATDF; SPDF `cruise/` 404.
- **Blockade:** kein offener ODF/ATDF-Endpunkt.
- **Braucht:** NSSDC-Antwort; sonst descope.

#### src.pas → esc.pithia.eu — Backend down
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** TAP-Backend erholt (`/tap/tables` 500→200).
- **Lage:** (gemessen 2026-09-26 via grind-flash) Host lebt (TAP 200); PostgreSQL `:5432` refused; 11 `epn_core`-Tabellen; `access_url`→`:8081/data/*` 404; Gap `epncore-spatial`.
- **Blockade:** Server-Backend (dritter).
- **Braucht:** `archive_search --verdict http://pithia.cbk.waw.pl/tap/tables` bei Trigger.

#### DEMETER — Download serverseitig zu
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** CDPP `online:true` (`phi/blocked_sources.φ:76`) oder Operator-Wort `restart`.
- **Lage:** (gemessen 2026-09-26) `PUT /orders/18387/retry` 200 (wirkungslos); Status `DONE_WITH_WARNING`, 0 verfügbar / 96 978 Fehler, alle `online:false`, Download 500/0 B.
- **Blockade:** CDPP-seitiges `online:false`.
- **Braucht:** Operator `PUT /orders/18387/restart` (Vorbereitung steht).

#### EMODNET HFRADAR NADR — Termin-Re-Messung
- **Status:** termin | **Bindung:** termin:2026-10-19
- **Trigger:** 2026-10-19.
- **Lage:** (gemessen 2026-09-24 via `external-state.md:43`) Asset registriert; Re-Messung offen.
- **Braucht:** `archive_search --sniff` bei Fälligkeit.

### Dritter

#### BepiColombo bc_mpo_more — Termin
- **Status:** termin | **Bindung:** termin:2027-04-01
- **Trigger:** 2027-04-01 (Science-Phase-Beginn).
- **Lage:** (gemessen 2026-09-26) PSA (Bentley) + PI (Iess): MORE-Cruise nicht öffentlich, Freigabe April 2027; Ticket `YYM-342-97327`.
- **Braucht:** Wiedervorlage 04/2027, dann MORE-`data_raw`/`calibration_raw`.

#### SSDC / Limadou — wartend auf CSES-02-Prozedur
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur `limadou.ssdc.asi.it`.
- **Lage:** (gemessen 2026-09-26 via `sfetch`) `query.php` → SSDC-CAS-Login (Auth-Redirect), neue Prozedur noch nicht live. Sotgiu (`mail_ledger.φ:79`): „wait a few weeks".
- **Blockade:** Portal-Umbau (dritter).
- **Braucht:** neue Prozedur abwarten; beim Trigger `query.php` re-messen.

#### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** 2026-09-28 (DEMETER) / 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-26 via `--verdict`) keine Erholung bei `regards.cnes.fr` (403) / `pithia.cbk.waw.pl` (500/TAP down) / `api.lasair.lsst.ac.uk` (404); Ersatzrouten gefunden (esc.pithia.eu, lasair-ztf).
- **Braucht:** `archive_search --verdict <url>`; bei Erholung den `*-cdn.yml`-Lauf dispatchen.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | nächster Schritt: `mariner-occlt-cdn` Lauf `36272916043` lesen (`ci_manage view 36272916043`).
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | nächster Schritt: `state/mail/mail_ledger.φ` auf MPI-FKF/LAB_A-Antwort (`smail`).
- `docs/surveys/survey-2026-09-26-secrets-inventar.md` | Dispositionen committet; Namens-Disposition trägt die Future-Übergabe.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | ausstehend nur Wiedervorlage 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending weiter tot; `arvo-registry.sci.am` http erholt → Quellenpunkt oben; nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | 55-disposition erledigt (2026-09-26); offen nur Step 4 (CI-Dedupe) + Step 5 (CDN-kanonisch) | nächster Schritt: Step 4 in `docs/auftrag/archiv/auftrag-saubere-datenbank.md` fassen.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | offen: Ziel-Layout `knowledge/`+`backups/` | nächster Schritt: Operator-Wort zum Layout.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen: NOAA-NRS passive-bioacoustic Quell-Entscheidung | nächster Schritt: Register-Eintrag + Compiler.

## Abschluss

Commit-Wort (`/commit`) steht aus.
