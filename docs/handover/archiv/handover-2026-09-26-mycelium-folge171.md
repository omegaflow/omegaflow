<!--
  title: Handover — Mycelium-Folge 171 (2026-09-26)
  session: Mycelium-Folge 171
  class: handover
  date: 2026-09-26
  sha256: e7fbbd75b0713a4811a45c6d3c62ed8dc4c26851d105a5299baadae77113c378
  status: live
-->
# Handover — Mycelium-Folge 171 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge. Sortierung: erst Akteur (**Linie** | **Rat** | **Operator** |
**Dritter**), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
**Trigger** / **Lage** / **Blockade** / **Braucht**. Status-Tag: `autonom` |
`operator-gebunden` | `blockiert` | `wartend` | `termin` | `LOCK`.

Diese Session konsumierte `handover-2026-09-26-mycelium-folge170.md`.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### CDN-Workflows hamqsl/ogimet/nohrsc/eri — Lauf-Stand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Neu-Dispatch `gh workflow run nohrsc_snowfall-cdn.yml` / `eri-cdn.yml`.
- **Lage:** (gemessen 2026-09-26 via `ci_manage list`/`log`) `hamqsl-cdn` `36233766455` **success** (Port verifiziert); `ogimet-cdn` `36233768757` **failure** auf `97b75797a` — `ogimet_compiler: 72594: no latitude/longitude in the header — the bin stays unwritten` (Log Z.531; korrekter Absent-Abbruch, kein Codefehler); `nohrsc_snowfall-cdn` `36233772273` **cancelled** (attempt 2); `eri-cdn` `36232783435` **failure**.
- **Blockade:** OGIMET-Antwort trägt für Station 72594 keine Koordinaten (Quellen-Messung).
- **Braucht:** nohrsc/eri neu dispatchen (`gh workflow run nohrsc_snowfall-cdn.yml` / `eri-cdn.yml`); OGIMET-`gsynres`-Rohantwort messen, ob je `lat`/`lon` im Header steht — sonst den Arm in `src/archivar/geo.rs` (`MAGIC_OGM`) als `parser-gap` registrieren.

#### CI am HEAD + Artefakt-Frische
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Lauf am HEAD `0fa25000e`.
- **Lage:** (gemessen 2026-09-26 via `ci_manage list`) `tools-build` `36233776812` **success** (09:51Z); `kernel-flatten` `36233774828` **pending**; Watchdog-Snapshot 11:48 nennt `te-gate`/`gll-rss-atdf-cdn`/`ci-check`/`meteo-cdn` in Arbeit; `tools-latest`-Manifest gegen HEAD ungemessen.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36233774828`; `sread target/release/.tools_manifest --limit 1` vs `git rev-parse HEAD`.

#### Zustand-due (Mycelium-Klasse) — fällige CDN-Assets
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Fälligkeit (due) im Zustands-Ledger `docs/zustand/external-state.md`.
- **Lage:** (gemessen 2026-09-26 via `register_lookup --open`) `docs/zustand/external-state.md:29` `dr3_stars.bin` DUE; `:30` `voyager_odr` Shards DUE; `:31` LIRA/RPW-BIA E-Feld DUE; `:33` Rosetta ODF PENDING.
- **Blockade:** CI-Lauf.
- **Braucht:** `gh workflow run gaia-cdn.yml` (dr3_stars); `ci_manage view 35143340703` (voyager_odr); `archive_search --sniff https://github.com/omegaflow/sources/releases/download/amda.irap.omp.eu/rpw_efield.bin` (LIRA/RPW); `ci_manage view 35881738785` (Rosetta ODF).

#### kernel-flatten — `--retry` + de441-Carrier
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** kernel-flatten `36233774828` (pending).
- **Lage:** (gemessen 2026-09-26 via `ci_manage list`) Lauf nach `--retry 3`-Push dispatcht, Status pending; Alt-Rot `36224127426` (`de441 base absent`); `phi/sources_index.φ` + `phi/pipeline/frame_registry.φ` sind gitignored (Crawl-Output, `.gitignore:73`).
- **Blockade:** CI-Lauf + fehlendes de441-Asset.
- **Braucht:** `ci_manage view 36233774828`; nach grünem Flatten de441-Bins + `url/format/origin`-Zeilen nach de440-Muster in `phi/sources.φ`.

#### GOES-19 ABI GSICS — Wurzel gemessen, Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `goes-cdn.yml`-Lauf.
- **Lage:** (gemessen 2026-09-26 via research-max) die L1b-Granule trägt `a_h_NRTH`/`b_h_NRTH` (GSICS Harmonization Offset/Slope; long_name in-granule); `src/archivar/goes_abi.rs:336-340` liest sie, `:444-456` setzt `CALIB_GSICS`; Quelle `GSICS_DEFAULT_URL` (`goes_abi_compiler.rs:8`, `…/GSICS_Harmonization_release_May2025_current.txt`) HTTP 200 (859 B); GOES-19 Band 1 A=0.0000, B=1.0230. Record-Note `goes-cdn.yml:26` auf `2 = GSICS-harmonized` gezogen.
- **Blockade:** keiner — der Asset-Lauf predatiert den In-Granule-Arm.
- **Braucht:** `gh workflow run goes-cdn.yml`; danach `phi/sources.φ:848`-Asset `calib=2` per `--sniff` gegenprüfen.

#### Postfach-Ledger riss — Arm gemessen, Fix gesetzt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Fix-Commit; `mail_digest --last 5` gegenprüfen.
- **Lage:** (gemessen 2026-09-26) `mail_digest.rs:8` defaultete auf den relativen Literal-Pfad `state/mail/mail_ledger.φ` ohne die `OMEGAFLOW_STATE`-aware `state_dir()`-Indirektion, die `mail_watchdog.rs:19` und `smail_recv.rs:392` tragen; anderer cwd/`OMEGAFLOW_STATE` → „ledger absent", obwohl die Datei (297 Z.) existiert. Fix gesetzt (`state_dir().join("mail/mail_ledger.φ")`); `cargo check -p omegaflow-service` grün, 0 Warnungen.
- **Blockade:** keine.
- **Braucht:** Fix mitcommitten; danach `mail_digest --last 5` live gegenprüfen.

#### gap-Orphans 4× TAP — Parser-Arm (mountain)
- **Status:** wartend | **Bindung:** eigen→mountain
- **Trigger:** Parser-Arm `unit-auto-detect`.
- **Lage:** (gemessen 2026-09-26 via grind-flash) die vier TAP-`pending` (`phi/blocked_sources.φ:146,150,154,158`) sind jetzt **gehalten** (0 Mycelium-Orphans; `--orphans` listet nur 6 `[future]`); Twin-URLs in `phi/sources.φ` (`gavo:7458`, `padc:7469`, `voparis:7480`, `skvo:9526`).
- **Blockade:** kein Parser-Arm (unit-auto-detect) für die vier TAP-JSON-Formen.
- **Braucht:** Arm bauen (mountain) oder Twin-Einträge mitziehen.

#### AllWISE — async-UWS Arm fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Arm `phi/blocked_sources.φ:223`.
- **Lage:** (gemessen 2026-09-26, Register-note `phi/blocked_sources.φ:223`) `allsky_4band_p3as_psd` sync-Stall; async-UWS Job 23542882 COMPLETED (1 Zeile, VOTable 1.3); Arm fehlt.
- **Blockade:** keine.
- **Braucht:** async-UWS-Arm für AllWISE bauen/messen.

#### GOSAT-GW — pending
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Datenverfügbarkeit `https://www.gosat-gw.nies.go.jp`.
- **Lage:** (gemessen 2026-09-26, Register-note `phi/blocked_sources.φ:324`) Homepage absent (nur Wayback 2022-09-06), kein Daten-Endpoint; `GOSAT_GW_*`-Credential vorhanden.
- **Blockade:** kein offener Endpoint.
- **Braucht:** `archive_search --verdict https://www.gosat-gw.nies.go.jp` beim nächsten Pass; bis dahin `pending`.

#### Lasair-LSST Broker
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Backend-Erholung `api.lasair.lsst.ac.uk` (502).
- **Lage:** (gemessen 2026-09-20, Register-note `phi/blocked_sources.φ:26`) 502 über Proton (direct 000); Frontend 200; ZTF-Zwilling 401; `LASAIR_LSST_TOKEN` vorhanden.
- **Blockade:** Upstream-Backend.
- **Braucht:** `archive_search --verdict https://api.lasair.lsst.ac.uk/api/` beim nächsten Pass.

#### Voyager 1/2 — closed-loop Doppler (`phi/blocked_sources.φ:49`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/blocked_sources.φ:49`.
- **Lage:** (gemessen 2026-09-26 via `register_lookup --open`) `[mycelium] pending`; V2 `radio_science_rss=2` dirs (saturn_encounter_data, saturn_occultation_medium_band), kein Cruise-ODF; TRK-2-34/ODF/ATDF request-only.
- **Blockade:** kein offener ODF-Endpunkt.
- **Braucht:** `archive_search --verdict` auf die V2-`radio_science_rss`-Verzeichnisse; ODF-Weg oder `request-only`-Verdikt.

#### SSDC / Limadou (`phi/pipeline/ledger.φ:14`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/pipeline/ledger.φ:14`.
- **Lage:** (gemessen 2026-09-26 via `register_lookup --open`) `[mycelium] ausstehend`; SSDC-Portal + CAS-Login funktionieren mit `SSDC_USER`/`SSDC_PASS` (query.php 200), aber „Permission Denied" für den omegaflow-Account.
- **Blockade:** SSDC-Berechtigung.
- **Braucht:** Berechtigungs-Akt / Account-Klärung mit SSDC (dritter) oder Descope mit Messung.

#### SuperDARN Ernte + Mirror-Transfer FAILED
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-26 via `state/mail/mail_ledger.φ:288`) Globus-Transfer `af68c4f1-b601-11f1-b9a2-0affd5e180af` (FITACF, 4994 Dateien / 34 126 266 995 B) **FAILED** 2026-09-25 10:23Z; MAP 6.561 Dateien/21,93 GB → `data/superdarn/map`; FITACF `phi/sources.φ:9465`; RAWACF `phi/sources.φ:8032`.
- **Blockade:** keine.
- **Braucht:** Globus-Mirror-Transfer neu anstoßen; Ziel-Asset per `--sniff`/`--verdict` messen + in `phi/sources.φ` registrieren; MAP-Ernte abschließen.

#### DEMETER Order 18387 — Product-GET 500
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Backend `regards.cnes.fr`.
- **Lage:** (gemessen 2026-09-25, Register-note `phi/blocked_sources.φ:75`) `DONE_WITH_WARNING`; `filesInErrorCount 96978`; `availableFilesCount 0`; product GET 500/0 B.
- **Blockade:** CNES-Backend (WAF).
- **Braucht:** `archive_search --verdict https://regards.cnes.fr/api/v1/rs-order` beim nächsten Pass.

#### DECaPS2 Dataverse — Ernte offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Harvest-Pass `phi/blocked_sources.φ:79`.
- **Lage:** (gemessen 2026-09-26, Register-note `phi/blocked_sources.φ:79`) `doi:10.7910/DVN/K88GFI` 309865090866 B / 100 `fits.gz`; Parser `phi/sources.φ:9628`; anonymer TAP-Weg `decaps_dr2.object`.
- **Blockade:** keine.
- **Braucht:** anonymen TAP-Weg ernten + in `phi/sources.φ` registrieren.

#### ivo://src.pas — 500
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Backend-Erholung `src.pas` (`phi/pipeline/ledger.φ:10`).
- **Lage:** (gemessen 2026-09-21, Register-note `phi/pipeline/ledger.φ:10`) `/tap` 200; `/tap/tables` 500 (PostgreSQL localhost:5…).
- **Blockade:** Backend-Fehler.
- **Braucht:** `archive_search --verdict` auf `/tap/tables` beim nächsten Pass.

#### api.sensor.community — CI-Puls
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `source-census`-Lauf.
- **Lage:** (gemessen 2026-09-26 via `archive_search --verdict`) stage 1 direct HTTP 403 (229 B), stage 2 socks5h HTTP 403 (229 B), stage 3 Wayback ohne Snapshot — ip-blocked; kein Exit-Wechsel (Operator-Wort 2026-09-25: Terms/§ 95a).
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>`.

#### Pre-CDN Lost-Blocks — Host-Verdikte nicht integriert
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/pipeline/index.φ:37/39`.
- **Lage:** (gemessen 2026-09-26 `stage/pre-cdn_join_report.txt`) 729 richest-URLs ohne Register, 37 params unregistriert, 4877 pool-lose Blöcke; `stage/pre_cdn_host_verdict_2026-09-25.txt` (75 Z.) trägt Host-Verdikte (1 pending, 8 externe GitHub-Repos, 4 pre-CDN-Asset, 12 declined, 2 blocked, 12 disponiert).
- **Blockade:** keine.
- **Braucht:** Host-Verdikte in `phi/sources.φ`/`declined_sources.φ`/`blocked_sources.φ` integrieren; 4877 pool-lose Blöcke (eigene Extraktion, blockade).

#### Pre-CDN params — 41 Direktiven + 15 Riss
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/pipeline/index.φ:39`.
- **Lage:** (gemessen 2026-09-26 `stage/pre-cdn_join_report.txt`) 41 params-Direktiven fehlen; 15 params-URLs mit konfliktierender Quelle (Riss: `imag-data.bgs.ac.uk` HAPI source drift).
- **Blockade:** HAPI-Source-Drift (Riss).
- **Braucht:** 41 Direktiven nachziehen; den 15er-Riss als Riss führen, nie glätten.

#### b2find intermagnet — Fanout-Stationsliste fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Harvest-Pass `stage/b2find_intermagnet_candidates.φ`.
- **Lage:** (gemessen 2026-09-26) `stage/b2find_intermagnet_candidates.φ` = 1 Kandidat (`…/GIN_V1/hapi/catalog`).
- **Blockade:** keine.
- **Braucht:** HAPI-Catalog nach Stationen (lat/lon) abfragen, Fanout-Liste bauen.

#### Arbeitsbaum-Formatierung — Autorschaft ungemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Pass `git diff`.
- **Lage:** (gemessen 2026-09-26 via `git diff`) reine `cargo fmt`-Umbauten in `skydirection.rs`/`kbo_residue_probe.rs`/`rixs_cuprate_probe.rs`/`suprastrom_form_probe.rs` (fremd, unangetastet); Zuordnung „eigene Ports vs. fremd" ungemessen.
- **Blockade:** keine.
- **Braucht:** beim nächsten Pass zuordnen (eigene behalten, fremde unangetastet).

#### EMODNET HFRADAR NADR — Termin-Re-Messung (`phi/sources.φ:1830`)
- **Status:** termin | **Bindung:** eigen
- **Trigger:** 2026-10-19.
- **Lage:** (gemessen 2026-09-24, Zustands-Ledger `docs/zustand/external-state.md:43`) Asset `emodnet_hfr_nadr.bin` registriert (Compiler `emodnet_hfr_compiler.rs`, Origin `erddap.emodnet-physics.eu/…/EUHFR_NRTcurrent_HFR-NAdr-Total`); Re-Messung offen.
- **Blockade:** Fälligkeit.
- **Braucht:** `archive_search --sniff https://github.com/omegaflow/sources/releases/download/erddap.emodnet-physics.eu/emodnet_hfr_nadr.bin` bei Fälligkeit; Ernte/Route prüfen.

#### pipeline/index.φ verifiziert-Pools
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/pipeline/index.φ:75-97`.
- **Lage:** (gemessen 2026-09-26 via `register_lookup --open`) acht `[mycelium] verifiziert`-Pools: `oai_arxiv`, `b2find_intermagnet_catalog`, `terrapulse_catalog`, `esa_geomagnetic_catalog`, `archeology_gaps_index`, `copernicus_catalog` (je 0), `grind_vires_catalog` (8), `grind_arcgis_index` (17).
- **Blockade:** keine.
- **Braucht:** Pools in den Harvest aufnehmen oder als leer/disponiert führen.

### Operator

#### Fremdmodell-Benchmark — vorbereitet bis zur Kante
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (per Akt).
- **Lage:** (gemessen 2026-09-26 via grind-flash) Entwurf `state/benchmark/fremdmodell-benchmark-2026-09-26.md` liegt bis zur Kante (Prompt, Antwortschlüssel, Metrik, Ziel chat.z.ai GLM-5.3, Sekundär claude.ai).
- **Blockade:** fehlender per-Akt-Consent.
- **Braucht:** Operator-Wort: `Fremdmodell-Benchmark Akt 1 (z.ai GLM-5.3) — ausführen.`

### Dritter

#### BepiColombo bc_mpo_more — Freigabe-Anfrage
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Antwort `psahelp@cosmos.esa.int`.
- **Lage:** (gemessen 2026-09-18, Register-note `phi/blocked_sources.φ:44`) `release_date 2099-01-01` (89434/89517 proprietär), `data?PRODUCT` 403; Anfrage raus; `bc_mpo_mag` anonym offen.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Wiedervorlage Antwort.

#### NSSDC-Anfragen — Wiedervorlage
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** NSSDCA-Antwort `state/mail/mail_ledger.φ`.
- **Lage:** (gemessen 2026-09-26 via `sread state/mail/mail_ledger.φ:74-78` + `mail_digest --last 2`) vier Anfragen 2026-09-16 raus (PSNO-00007, PSCM-00009, PSPG-00011/00457, Juno/Cassini); letzte Eingänge 2026-09-26 (NED, OpenAlex) — keine NSSDCA-Antwort; `PSPA-00605` ungesendet.
- **Blockade:** Antwort des NSSDCA.
- **Braucht:** Wiedervorlage-Frist setzen; `PSPA-00605` senden (Send = Operator-Hand).

#### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-09-28 (DEMETER) / 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-26 via grind-flash, `--verdict`) keine Erholung bei regards.cnes.fr / pithia.cbk.waw.pl / lpf.esac.esa.int / api.lasair.lsst.ac.uk; `psa.esa.int/psa-tap/tap/`, `superdarn.ca/data-download`, `limadou.ssdc.asi.it/` 200 offen.
- **Blockade:** WAF/Backend bzw. offene Produktfreigabe.
- **Braucht:** `archive_search --verdict <url>`; bei Erholung den jeweiligen `*-cdn.yml`-Lauf dispatchen.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | nächster Schritt: `gh workflow run mariner-occlt-cdn.yml`.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | nächster Schritt: `state/mail/mail_ledger.φ` auf MPI-FKF/TRISP-Antwort (`smail`).
- `docs/surveys/survey-2026-09-26-secrets-inventar.md` | Dispositionen committet; Namens-Disposition trägt die Future-Übergabe.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | ausstehend nur GOES GSICS (eigener Punkt) + Wiedervorlage 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | offen: Re-Check 3 Force-Kanal + 4 pending + `arvo-registry.sci.am` | nächster Schritt: `archive_search --verdict` je Host.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offen: Disposition der 55 undocumented `stale_pending` | nächster Schritt: `docs/specs/cdn_orphan_verdicts.json` je Netloc disponieren.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | offen: Ziel-Layout `knowledge/`+`backups/` | nächster Schritt: Operator-Wort zum Layout.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen: NOAA-NRS passive-bioacoustic Quell-Entscheidung | nächster Schritt: Register-Eintrag + Compiler.

## Abschluss

Commit-Wort (`/commit`) steht aus.
