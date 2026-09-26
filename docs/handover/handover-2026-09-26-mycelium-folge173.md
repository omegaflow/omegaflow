<!--
  title: Handover — Mycelium-Folge 173 (2026-09-26)
  session: Mycelium-Folge 173
  class: handover
  date: 2026-09-26
  sha256: 058046699d054f321b7ab42bceb26be1a4acfa00794576b033f70fa407dca354
  status: live
-->
# Handover — Mycelium-Folge 173 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `autonom` | `operator-gebunden` |
`blockiert` | `wartend` | `termin` | `LOCK`.

Diese Session konsumierte `handover-2026-09-26-mycelium-folge172.md`.

## Operator-Wort-Register

- Wort | 2026-09-26 | „Die bis zur Kante abarbeitbaren Dinge abarbeiten; Backlog verkleinern" — session-weiter Consent (`mycelium_go`), Delegation.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### sensor.community-Spiegel — Port-Gap
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `value_type`-Keyed-Selector-Arm gebaut.
- **Lage:** (gemessen 2026-09-26 via `archive_search`) api 403 (direct+Proton); Spiegel `maps.sensor.community/data/v2/data.json` 8 612 211 B sha256 `8cf2dc8c…`; `archive…/2026-09-25_bme280_sensor_141.csv` 36 743 B sha256 `1f64cb94…`; `.csv.gz` 404. JSON `value_type`-Reihenfolge variiert; CSV braucht Station/Tag-Crawl.
- **Blockade:** kein korrekter Eintrag in `phi/sources.φ` (Keyed-Selector/Crawl fehlen).
- **Braucht:** Keyed-Selector-Arm (Feld-Dot-Pfad `.0.value` ist positional → mislabelt interleaved Sensortypen).

#### IRIS/EarthScope EMC netCDF-4 — volume-Extract
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Architektur-Wort `volume`-Extract.
- **Lage:** (gemessen 2026-09-26 via grind-pro) `src/archivar/hdf5.rs` existiert (liest netCDF-4); Gap `volume`-Extract (`types.rs:136`) + Kanalarm; Lizenz keine, EarthScope-ToS (Attribution + Non-human-Visitor).
- **Blockade:** Architektur-Akt (neue `volume`-Direktive).
- **Braucht:** Rat/Operator-Wort `volume` + `parse.rs`/`channels.rs`/`main_flow.rs`.

#### MODIS LST CMG — hdf4-Coder-Arm
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** NBIT-Arm in `hdf4.rs` gebaut.
- **Lage:** (gemessen 2026-09-26 via `ci_manage`/grind-pro) CI `modis-cdn 36267998951`/`36268005691` = Runner-Shutdown (transient, kein Code-Fehler); `hdf4.rs:583` `decode_chunk` kennt nur NONE/RLE/DEFLATE; NBIT(2)/SKPHUFF(3)/SZIP(5) fehlen; `comp`-Header (`:409`) nicht durchgereicht.
- **Blockade:** hdf4-Coder-Arm fehlt.
- **Braucht:** eine Aqua-Granule fetchen (`MYD11C3.061`, CMR, EARTHDATA_EDL_TOKEN), Coder-ID messen, NBIT-Arm (~30 Zeilen) + Fixture.

#### GOSAT-GW GWT3F_L1B — format-Arm
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `format gosat_tanso3`-Arm gebaut.
- **Lage:** (gemessen 2026-09-26 via grind-flash) Compiler `gosat_tanso3` gebaut (`1e3aeebc7`, `G3L1`, 3648 Records); GWT3F_L1B 132 / GWT3W_L1B 434 Dateien; Endpoint `cui-search`/`cui-download` + Cookie; format-Arm fehlt.
- **Blockade:** Reader-Arm + Quellenblock.
- **Braucht:** `format gosat_tanso3`-Reader + Quellenblock in `phi/sources.φ`.

#### CDN nohrsc — Lauf rot
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `nohrsc_snowfall-cdn`-Lauf grün.
- **Lage:** (gemessen 2026-09-26 via `ci_manage`) `nohrsc 36236054115` failure (attempt 2); `eri 36236056155` success.
- **Blockade:** CI-Lauf rot.
- **Braucht:** `ci_manage log 36236054115`.

#### kernel-flatten de441-cdn — Asset-Größe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `de441-cdn`-Lauf (≥183 MB `ephemeris_{sun,earth}.bin`).
- **Lage:** (gemessen 2026-09-26 via `ci_manage`) `kernel-flatten 36233774828` success, `de441 base present`, `spk_split` lief; `de441-cdn` wartet auf ≥183 MB Bins.
- **Blockade:** Asset-Größe.
- **Braucht:** `ci_manage list` filter de441-cdn; Bins prüfen.

#### Pre-CDN params — Keyer-Gegenprüfung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Queue-Pass `phi/pipeline/queue/sources_potential_pre-cdn_params.φ`.
- **Lage:** (gemessen 2026-09-26) `source_keyer.rs` gebaut (`318e456d2`); 34 Direktiven in die Queue; 3 Riss gelöst (TIRM/TLON/VALL), 1 Riss (MCQG) geführt.
- **Blockade:** keine.
- **Braucht:** Keyer-Output gegenprüfen + MCQG-Riss führen.

#### Voyager 1/2 closed-loop Doppler — request-only
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** NSSDC-Antwort (Anfrage `:207`).
- **Lage:** (gemessen 2026-09-26 via `archive_search`) PDS V2 nur open-loop `.ODR`; `radio_science_rss=2` dirs kein Cruise ODF/ATDF; SPDF `cruise/` 404.
- **Blockade:** kein offener ODF/ATDF-Endpunkt.
- **Braucht:** NSSDC-Antwort; sonst descope.

#### src.pas → esc.pithia.eu — Backend down
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** TAP-Backend erholt (`/tap/tables` 500→200).
- **Lage:** (gemessen 2026-09-26 via grind-flash) Host lebt (TAP 200); PostgreSQL `:5432` refused; 11 `epn_core`-Tabellen; `esc.pithia.eu` = Metadaten-Katalog, `access_url`→`:8081/data/*` 404; Gap `epncore-spatial`.
- **Blockade:** Server-Backend (dritter).
- **Braucht:** `archive_search --verdict http://pithia.cbk.waw.pl/tap/tables` bei Trigger.

#### DEMETER — Download serverseitig zu
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** CDPP `online:true` (`phi/blocked_sources.φ:76`) oder Operator-Wort `restart`.
- **Lage:** (gemessen 2026-09-26) `PUT /orders/18387/retry` 200 (wirkungslos); Status `DONE_WITH_WARNING`, 0 verfügbar / 96 978 Fehler, alle `online:false`, Download 500/0 B.
- **Blockade:** CDPP-seitiges `online:false`.
- **Braucht:** Operator `PUT /orders/18387/restart` (Vorbereitung steht).

#### Arbeitsbaum-Formatierung — Autorschaft ungemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Pass `git diff`.
- **Lage:** (gemessen 2026-09-26) reine fmt-Umbauten in `skydirection.rs` + 3 Probe-Dateien (fremd, unangetastet).
- **Blockade:** keine.
- **Braucht:** beim nächsten Pass zuordnen.

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

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | nächster Schritt: `gh workflow run mariner-occlt-cdn.yml`.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | nächster Schritt: `state/mail/mail_ledger.φ` auf MPI-FKF/LAB_A-Antwort (`smail`).
- `docs/surveys/survey-2026-09-26-secrets-inventar.md` | Dispositionen committet; Namens-Disposition trägt die Future-Übergabe.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | ausstehend nur Wiedervorlage 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | offen: Re-Check 3 Force-Kanal + 4 pending + `arvo-registry.sci.am` | nächster Schritt: `archive_search --verdict` je Host.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offen: Disposition der 55 undocumented `stale_pending` | nächster Schritt: `docs/specs/cdn_orphan_verdicts.json` je Netloc disponieren.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | offen: Ziel-Layout `knowledge/`+`backups/` | nächster Schritt: Operator-Wort zum Layout.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen: NOAA-NRS passive-bioacoustic Quell-Entscheidung | nächster Schritt: Register-Eintrag + Compiler.

## Abschluss

Commit-Wort (`/commit`) steht aus.
